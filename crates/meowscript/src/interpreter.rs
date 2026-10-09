//! Tree-walking evaluator. `tail`, `hiss`, `continue`, `yowl`, and errors travel up through the
//! `Err` side of [`Flow`], so the happy path only deals with values.

use crate::ast::*;
use crate::env::Env;
use crate::error::{Error, ErrorKind, Result};
use crate::host::Host;
use crate::parser::parse;
use crate::rng::Rng;
use crate::span::Span;
use crate::stdlib;
use crate::value::{Closure, Value, format_number};
use indexmap::IndexMap;
use std::rc::Rc;

pub const MAX_CALL_DEPTH: usize = 1000;
const MAX_IMPORT_DEPTH: usize = 32;

enum Interrupt {
    Return(Value),
    Break(Span),
    Continue(Span),
    Throw(Value, Span),
    Error(Error),
}

impl From<Error> for Interrupt {
    fn from(e: Error) -> Self {
        Interrupt::Error(e)
    }
}

type Flow<T> = std::result::Result<T, Interrupt>;

pub struct Interpreter {
    host: Box<dyn Host>,
    prelude: Env,
    env: Env,
    rng: Rng,
    call_depth: usize,
    import_depth: usize,
    base_dir: Option<String>,
    // The text being run, for the line numbers `caught` reports.
    source: String,
    // A `yowl` inside a callback crosses the built-in that called it (`map`, say) as an `Error`.
    // Its value waits here and becomes a `Throw` again when the built-in returns.
    in_flight: Option<(Value, Span)>,
}

impl Interpreter {
    pub fn new(host: impl Host + 'static) -> Self {
        Self::with_boxed_host(Box::new(host))
    }

    pub fn with_boxed_host(mut host: Box<dyn Host>) -> Self {
        let seed = host.random_seed();
        let prelude = Env::new();
        stdlib::install_prelude(&prelude);
        let env = prelude.child();
        Self {
            host,
            prelude,
            env,
            rng: Rng::seeded(seed),
            call_depth: 0,
            import_depth: 0,
            base_dir: None,
            source: String::new(),
            in_flight: None,
        }
    }

    pub fn host(&mut self) -> &mut dyn Host {
        &mut *self.host
    }

    pub fn rng(&mut self) -> &mut Rng {
        &mut self.rng
    }

    pub fn reseed(&mut self, seed: u64) {
        self.rng = Rng::seeded(seed);
    }

    /// Where relative `pawckage "./file.meow"` paths resolve from.
    pub fn set_base_dir(&mut self, dir: Option<String>) {
        self.base_dir = dir;
    }

    /// Runs in the current global scope, so a REPL can call it repeatedly. Returns the value of
    /// the last expression statement.
    pub fn run(&mut self, source: &str) -> Result<Value> {
        let program = parse(source)?;
        let saved = std::mem::replace(&mut self.source, source.to_string());
        let result = self.exec_program(&program);
        self.source = saved;
        result
    }

    pub fn call(&mut self, callee: Value, args: Vec<Value>, span: Option<Span>) -> Result<Value> {
        match self.call_flow(callee, args, span) {
            Ok(v) => Ok(v),
            Err(Interrupt::Throw(value, at)) => {
                let error = uncaught(&value, at);
                self.in_flight = Some((value, at));
                Err(error)
            }
            Err(Interrupt::Error(e)) => Err(e),
            Err(Interrupt::Return(_) | Interrupt::Break(_) | Interrupt::Continue(_)) => {
                unreachable!("call_flow settles these itself")
            }
        }
    }

    fn call_flow(&mut self, callee: Value, args: Vec<Value>, span: Option<Span>) -> Flow<Value> {
        let attach = |e: Error| match span {
            Some(s) => e.or_at(s),
            None => e,
        };
        match callee {
            Value::Builtin(b) => {
                if !b.arity.accepts(args.len()) {
                    return Err(attach(Error::arity(format!(
                        "`{}` takes {}, but got {}",
                        b.name,
                        b.arity.describe(),
                        args.len()
                    )))
                    .into());
                }
                (b.func)(self, args).map_err(|e| match self.in_flight.take() {
                    Some((value, at)) if e.kind == ErrorKind::Thrown => Interrupt::Throw(value, at),
                    _ => attach(e).into(),
                })
            }
            Value::Function(closure) => {
                let def = &closure.def;
                let name = def
                    .name
                    .as_deref()
                    .map_or("this pawction".to_string(), |n| format!("`{n}`"));
                if def.params.len() != args.len() {
                    let n = def.params.len();
                    return Err(attach(Error::arity(format!(
                        "{name} takes {n} argument{}, but got {}",
                        if n == 1 { "" } else { "s" },
                        args.len()
                    )))
                    .into());
                }
                if self.call_depth >= MAX_CALL_DEPTH {
                    return Err(attach(Error::runtime(format!(
                        "pawctions are nested more than {MAX_CALL_DEPTH} calls deep; is there a recursion with no way out?"
                    )))
                    .into());
                }
                let env = closure.env.child();
                for (param, arg) in def.params.iter().zip(args) {
                    env.define(param.name.clone(), arg);
                }
                self.call_depth += 1;
                let saved = std::mem::replace(&mut self.env, env);
                let result = self.exec_stmts(&def.body.stmts);
                self.env = saved;
                self.call_depth -= 1;
                match result {
                    Ok(v) | Err(Interrupt::Return(v)) => Ok(v),
                    Err(Interrupt::Break(s)) => {
                        Err(Error::runtime("`hiss` only works inside a loop")
                            .at(s)
                            .into())
                    }
                    Err(Interrupt::Continue(s)) => {
                        Err(Error::runtime("`continue` only works inside a loop")
                            .at(s)
                            .into())
                    }
                    Err(other) => Err(other),
                }
            }
            other => Err(attach(Error::type_(format!(
                "only pawctions can be called, and this is a {}",
                other.type_name()
            )))
            .into()),
        }
    }

    pub fn print(&mut self, line: &str) {
        self.host.print(line);
    }

    fn exec_program(&mut self, program: &Program) -> Result<Value> {
        match self.exec_stmts(&program.stmts) {
            Ok(v) | Err(Interrupt::Return(v)) => Ok(v),
            Err(Interrupt::Error(e)) => Err(e),
            Err(Interrupt::Throw(value, span)) => Err(uncaught(&value, span)),
            Err(Interrupt::Break(s)) => {
                Err(Error::runtime("`hiss` only works inside a loop").at(s))
            }
            Err(Interrupt::Continue(s)) => {
                Err(Error::runtime("`continue` only works inside a loop").at(s))
            }
        }
    }

    // A block's value is its last statement's, which is how `purrhaps` works as an expression
    // and how a pawction returns without `tail`.
    fn exec_stmts(&mut self, stmts: &[Stmt]) -> Flow<Value> {
        let mut last = Value::Null;
        for stmt in stmts {
            last = self.exec_stmt(stmt)?;
        }
        Ok(last)
    }

    fn exec_block(&mut self, block: &Block) -> Flow<Value> {
        let child = self.env.child();
        let saved = std::mem::replace(&mut self.env, child);
        let result = self.exec_stmts(&block.stmts);
        self.env = saved;
        result
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Flow<Value> {
        match &stmt.kind {
            StmtKind::Let { name, value } => {
                let v = self.eval(value)?;
                self.env.define(name.name.clone(), v);
                Ok(Value::Null)
            }
            StmtKind::Assign { target, value } => {
                let v = self.eval(value)?;
                self.assign(target, v)?;
                Ok(Value::Null)
            }
            StmtKind::Return { value } => {
                let v = match value {
                    Some(e) => self.eval(e)?,
                    None => Value::Null,
                };
                Err(Interrupt::Return(v))
            }
            StmtKind::Import { path, span } => {
                self.import(path, *span)?;
                Ok(Value::Null)
            }
            StmtKind::Break => Err(Interrupt::Break(stmt.span)),
            StmtKind::Continue => Err(Interrupt::Continue(stmt.span)),
            StmtKind::FnDecl { name, func } => {
                let closure = Closure {
                    def: func.clone(),
                    env: self.env.clone(),
                };
                self.env
                    .define(name.name.clone(), Value::Function(Rc::new(closure)));
                Ok(Value::Null)
            }
            StmtKind::Loop { cond, body } => {
                loop {
                    if let Some(cond) = cond
                        && !self.eval(cond)?.is_truthy()
                    {
                        break;
                    }
                    match self.exec_block(body) {
                        Ok(_) | Err(Interrupt::Continue(_)) => {}
                        Err(Interrupt::Break(_)) => break,
                        Err(other) => return Err(other),
                    }
                }
                Ok(Value::Null)
            }
            StmtKind::ForIn { var, iter, body } => {
                let iterable = self.eval(iter)?;
                let items = self.iterate(&iterable).map_err(|e| e.at(iter.span))?;
                for item in items {
                    let scope = self.env.child();
                    scope.define(var.name.clone(), item);
                    let saved = std::mem::replace(&mut self.env, scope);
                    let result = self.exec_block(body);
                    self.env = saved;
                    match result {
                        Ok(_) | Err(Interrupt::Continue(_)) => {}
                        Err(Interrupt::Break(_)) => break,
                        Err(other) => return Err(other),
                    }
                }
                Ok(Value::Null)
            }
            StmtKind::Try {
                body,
                binding,
                handler,
            } => {
                let caught = match self.exec_block(body) {
                    Ok(_) => return Ok(Value::Null),
                    Err(Interrupt::Throw(value, _)) => value,
                    Err(Interrupt::Error(e)) => self.error_value(&e),
                    Err(other) => return Err(other),
                };
                let scope = self.env.child();
                if let Some(name) = binding {
                    scope.define(name.name.clone(), caught);
                }
                let saved = std::mem::replace(&mut self.env, scope);
                let result = self.exec_block(handler);
                self.env = saved;
                result.map(|_| Value::Null)
            }
            StmtKind::Throw { value } => {
                let v = self.eval(value)?;
                Err(Interrupt::Throw(v, stmt.span))
            }
            StmtKind::Expr(expr) => self.eval(expr),
        }
    }

    fn error_value(&self, e: &Error) -> Value {
        let mut map = IndexMap::new();
        map.insert("kind".into(), Value::str(e.kind.word()));
        map.insert("message".into(), Value::str(e.message.as_str()));
        let line = e.line(&self.source).map_or(Value::Null, Value::from);
        map.insert("line".into(), line);
        Value::object(map)
    }

    fn iterate(&self, value: &Value) -> Result<Vec<Value>> {
        Ok(match value {
            Value::Array(items) => items.borrow().clone(),
            Value::Str(s) => s.chars().map(|c| Value::str(c.to_string())).collect(),
            Value::Object(map) => map.borrow().keys().map(|k| Value::Str(k.clone())).collect(),
            Value::Number(n) if *n >= 0.0 && n.fract() == 0.0 => {
                (0..*n as u64).map(|i| Value::Number(i as f64)).collect()
            }
            Value::Number(n) => {
                return Err(Error::type_(format!(
                    "`fur` over a number counts from 0 up to it, so it needs a whole number of at least 0, not {}",
                    format_number(*n)
                )));
            }
            other => {
                return Err(Error::type_(format!(
                    "`fur` can walk over a furrball, whiskers, object, or number, not a {}",
                    other.type_name()
                )));
            }
        })
    }

    fn assign(&mut self, target: &Expr, value: Value) -> Flow<()> {
        match &target.kind {
            ExprKind::Ident(name) => {
                if self.env.assign(name, value) {
                    Ok(())
                } else {
                    Err(Error::name(format!(
                        "can't `amew {name}` because `{name}` was never `scratch`ed{}",
                        self.suggestion(name)
                    ))
                    .at(target.span)
                    .into())
                }
            }
            ExprKind::Index { object, index } => {
                let obj = self.eval(object)?;
                let idx = self.eval(index)?;
                self.set_index(&obj, &idx, value)
                    .map_err(|e| e.or_at(target.span).into())
            }
            ExprKind::Member { object, name } => {
                let obj = self.eval(object)?;
                let key = Value::Str(name.name.clone());
                self.set_index(&obj, &key, value)
                    .map_err(|e| e.or_at(target.span).into())
            }
            _ => Err(Error::syntax("this can't be assigned to", target.span).into()),
        }
    }

    fn set_index(&mut self, obj: &Value, index: &Value, value: Value) -> Result<()> {
        match (obj, index) {
            (Value::Array(items), Value::Number(n)) => {
                let mut items = items.borrow_mut();
                let len = items.len();
                let i = array_index(*n, len).ok_or_else(|| out_of_bounds(*n, len))?;
                items[i] = value;
                Ok(())
            }
            (Value::Array(_), other) => Err(Error::type_(format!(
                "furrball indexes are numbers, not {}",
                other.type_name()
            ))),
            (Value::Object(map), key) => {
                let key = object_key(key)?;
                map.borrow_mut().insert(key, value);
                Ok(())
            }
            (Value::Str(_), _) => Err(Error::type_(
                "whiskers can't be changed in place; build a new string with `+` or `replace` instead",
            )),
            (other, _) => Err(Error::type_(format!(
                "can't assign into a {}",
                other.type_name()
            ))),
        }
    }

    // Errors with no position yet get this expression's span, so the innermost culprit wins.
    fn eval(&mut self, expr: &Expr) -> Flow<Value> {
        self.eval_inner(expr).map_err(|i| match i {
            Interrupt::Error(e) => Interrupt::Error(e.or_at(expr.span)),
            other => other,
        })
    }

    fn eval_inner(&mut self, expr: &Expr) -> Flow<Value> {
        Ok(match &expr.kind {
            ExprKind::Null => Value::Null,
            ExprKind::Bool(b) => Value::Bool(*b),
            ExprKind::Number(n) => Value::Number(*n),
            ExprKind::Str(s) => Value::Str(s.clone()),
            ExprKind::Ident(name) => match self.env.get(name) {
                Some(v) => v,
                None => {
                    return Err(Error::name(format!(
                        "`{name}` hasn't been `scratch`ed yet{}",
                        self.suggestion(name)
                    ))
                    .into());
                }
            },
            ExprKind::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.eval(item)?);
                }
                Value::array(out)
            }
            ExprKind::Object(entries) => {
                let mut map = IndexMap::with_capacity(entries.len());
                for (key, value) in entries {
                    let k = self.eval(key)?;
                    let k = object_key(&k).map_err(|e| e.at(key.span))?;
                    let v = self.eval(value)?;
                    map.insert(k, v);
                }
                Value::object(map)
            }
            ExprKind::Unary { op, operand } => {
                let v = self.eval(operand)?;
                match op {
                    UnaryOp::Not => Value::Bool(!v.is_truthy()),
                    UnaryOp::Typeof => Value::str(v.type_name()),
                    UnaryOp::Neg => match v {
                        Value::Number(n) => Value::Number(-n),
                        other => {
                            return Err(Error::type_(format!(
                                "`-` needs a number, not a {}",
                                other.type_name()
                            ))
                            .into());
                        }
                    },
                }
            }
            ExprKind::Binary {
                op: BinaryOp::And,
                left,
                right,
            } => {
                let l = self.eval(left)?;
                if l.is_truthy() { self.eval(right)? } else { l }
            }
            ExprKind::Binary {
                op: BinaryOp::Or,
                left,
                right,
            } => {
                let l = self.eval(left)?;
                if l.is_truthy() { l } else { self.eval(right)? }
            }
            ExprKind::Binary { op, left, right } => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                binary(*op, &l, &r)?
            }
            ExprKind::If {
                cond,
                then,
                otherwise,
            } => {
                if self.eval(cond)?.is_truthy() {
                    self.exec_block(then)?
                } else {
                    match otherwise.as_deref() {
                        Some(Else::If(e)) => self.eval(e)?,
                        Some(Else::Block(b)) => self.exec_block(b)?,
                        None => Value::Null,
                    }
                }
            }
            ExprKind::Function(def) => Value::Function(Rc::new(Closure {
                def: def.clone(),
                env: self.env.clone(),
            })),
            ExprKind::Call { callee, args } => {
                let f = self.eval(callee)?;
                let mut values = Vec::with_capacity(args.len());
                for arg in args {
                    values.push(self.eval(arg)?);
                }
                self.call_flow(f, values, Some(expr.span))?
            }
            ExprKind::Index { object, index } => {
                let obj = self.eval(object)?;
                let idx = self.eval(index)?;
                get_index(&obj, &idx)?
            }
            ExprKind::Member { object, name } => {
                let obj = self.eval(object)?;
                get_index(&obj, &Value::Str(name.name.clone()))?
            }
        })
    }

    fn import(&mut self, path: &str, span: Span) -> Result<()> {
        if let Some(name) = path.strip_prefix("nya:") {
            let Some(pkg) = stdlib::package(name) else {
                let names: Vec<_> = stdlib::PACKAGES
                    .iter()
                    .map(|p| format!("`{}`", p.name))
                    .collect();
                return Err(Error::import(format!(
                    "there is no built-in pawckage called `nya:{name}`; the ones I know are {}",
                    names.join(", ")
                ))
                .at(span));
            };
            pkg.install(&self.env);
            return Ok(());
        }

        if self.import_depth >= MAX_IMPORT_DEPTH {
            return Err(Error::import(
                "pawckages are importing each other in a circle (or nested absurdly deep)",
            )
            .at(span));
        }

        let resolved = self.resolve_path(path);
        let source = self.host.read_file(&resolved).map_err(|e| {
            Error::import(format!("couldn't load pawckage `{path}`: {}", e.message)).at(span)
        })?;
        let program = parse(&source).map_err(|e| {
            Error::import(format!(
                "inside pawckage `{path}`:\n{}",
                e.render(&source, Some(&resolved))
            ))
            .at(span)
        })?;

        let module_env = self.prelude.child();
        let saved_env = std::mem::replace(&mut self.env, module_env.clone());
        let saved_dir = std::mem::replace(&mut self.base_dir, parent_dir(&resolved));
        let saved_source = std::mem::replace(&mut self.source, source.clone());
        self.import_depth += 1;
        let result = self.exec_program(&program);
        self.import_depth -= 1;
        self.source = saved_source;
        self.base_dir = saved_dir;
        self.env = saved_env;
        result.map_err(|e| {
            Error::import(format!(
                "inside pawckage `{path}`:\n{}",
                e.render(&source, Some(&resolved))
            ))
            .at(span)
        })?;

        // A module's own `nya:` imports stay private to it.
        for (name, value) in module_env.local_bindings() {
            if !stdlib::is_package_provided(&name, &value) {
                self.env.define(name, value);
            }
        }
        Ok(())
    }

    fn resolve_path(&self, path: &str) -> String {
        let mut p = std::path::PathBuf::from(path);
        if p.extension().is_none() {
            p.set_extension("meow");
        }
        if p.is_absolute() {
            return p.to_string_lossy().into_owned();
        }
        match &self.base_dir {
            Some(dir) if !dir.is_empty() => std::path::Path::new(dir)
                .join(p)
                .to_string_lossy()
                .into_owned(),
            _ => p.to_string_lossy().into_owned(),
        }
    }

    fn suggestion(&self, name: &str) -> String {
        let mut candidates: Vec<Rc<str>> = self.env.visible_names();
        candidates.sort();
        candidates.dedup();
        let best = candidates
            .iter()
            .map(|c| (edit_distance(name, c), c))
            .filter(|(d, c)| *d <= 2 && c.len() > 1)
            .min_by_key(|(d, _)| *d);
        match best {
            Some((_, c)) => format!("; did you mean `{c}`?"),
            None => String::new(),
        }
    }
}

fn uncaught(value: &Value, span: Span) -> Error {
    Error::new(ErrorKind::Thrown, value.to_string()).at(span)
}

fn binary(op: BinaryOp, l: &Value, r: &Value) -> Result<Value> {
    use BinaryOp::*;
    use Value::*;
    let v = match (op, l, r) {
        (Add, Number(a), Number(b)) => Number(a + b),
        (Add, Str(a), _) => Value::str(format!("{a}{r}")),
        (Add, _, Str(b)) => Value::str(format!("{l}{b}")),
        (Add, Array(a), Array(b)) => {
            let mut items = a.borrow().clone();
            items.extend(b.borrow().iter().cloned());
            Value::array(items)
        }
        (Sub, Number(a), Number(b)) => Number(a - b),
        (Mul, Number(a), Number(b)) => Number(a * b),
        (Mul, Str(s), Number(n)) | (Mul, Number(n), Str(s)) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(Error::type_(format!(
                    "whiskers can be repeated a whole number of times, not {}",
                    format_number(*n)
                )));
            }
            Value::str(s.repeat(*n as usize))
        }
        (Div, Number(a), Number(b)) => {
            if *b == 0.0 {
                return Err(Error::runtime("can't divide by zero"));
            }
            Number(a / b)
        }
        (Rem, Number(a), Number(b)) => {
            if *b == 0.0 {
                return Err(Error::runtime(
                    "can't take the remainder of dividing by zero",
                ));
            }
            Number(a % b)
        }
        (Eq, _, _) => Bool(l == r),
        (NotEq, _, _) => Bool(l != r),
        (Lt, Number(a), Number(b)) => Bool(a < b),
        (Gt, Number(a), Number(b)) => Bool(a > b),
        (LtEq, Number(a), Number(b)) => Bool(a <= b),
        (GtEq, Number(a), Number(b)) => Bool(a >= b),
        (Lt, Str(a), Str(b)) => Bool(a < b),
        (Gt, Str(a), Str(b)) => Bool(a > b),
        (LtEq, Str(a), Str(b)) => Bool(a <= b),
        (GtEq, Str(a), Str(b)) => Bool(a >= b),
        (BitAnd, Number(a), Number(b)) => Number((*a as i64 & *b as i64) as f64),
        (BitOr, Number(a), Number(b)) => Number((*a as i64 | *b as i64) as f64),
        (BitXor, Number(a), Number(b)) => Number((*a as i64 ^ *b as i64) as f64),
        (Shl, Number(a), Number(b)) => Number((*a as i64).wrapping_shl(*b as u32) as f64),
        (Shr, Number(a), Number(b)) => Number((*a as i64).wrapping_shr(*b as u32) as f64),
        (In, _, Array(items)) => Bool(items.borrow().contains(l)),
        (In, _, Object(map)) => Bool(map.borrow().contains_key(&*object_key(l)?)),
        (In, Str(needle), Str(hay)) => Bool(hay.contains(&**needle)),
        (And | Or, _, _) => unreachable!("short-circuit operators are handled by the evaluator"),
        _ => {
            return Err(Error::type_(format!(
                "`{op}` doesn't work between {} and {}",
                article(l.type_name()),
                article(r.type_name())
            )));
        }
    };
    Ok(v)
}

fn get_index(obj: &Value, index: &Value) -> Result<Value> {
    match (obj, index) {
        (Value::Array(items), Value::Number(n)) => {
            let items = items.borrow();
            array_index(*n, items.len())
                .map(|i| items[i].clone())
                .ok_or_else(|| out_of_bounds(*n, items.len()))
        }
        (Value::Str(s), Value::Number(n)) => {
            let len = s.chars().count();
            array_index(*n, len)
                .and_then(|i| s.chars().nth(i))
                .map(|c| Value::str(c.to_string()))
                .ok_or_else(|| out_of_bounds(*n, len))
        }
        (Value::Object(map), key) => {
            let key = object_key(key)?;
            Ok(map.borrow().get(&*key).cloned().unwrap_or(Value::Null))
        }
        (Value::Array(_) | Value::Str(_), other) => Err(Error::type_(format!(
            "{} indexes are numbers, not {}",
            obj.type_name(),
            article(other.type_name())
        ))),
        (other, _) => Err(Error::type_(format!(
            "can't index into {}",
            article(other.type_name())
        ))),
    }
}

/// Negative indexes count from the end.
pub(crate) fn array_index(n: f64, len: usize) -> Option<usize> {
    if n.fract() != 0.0 || n.is_nan() {
        return None;
    }
    let i = n as i64;
    let i = if i < 0 { i + len as i64 } else { i };
    (0..len as i64).contains(&i).then_some(i as usize)
}

fn out_of_bounds(n: f64, len: usize) -> Error {
    if n.fract() != 0.0 {
        Error::type_(format!(
            "indexes are whole numbers, not {}",
            format_number(n)
        ))
    } else if len == 0 {
        Error::runtime(format!(
            "index {} is out of bounds; this one is empty",
            format_number(n)
        ))
    } else {
        Error::runtime(format!(
            "index {} is out of bounds; valid indexes are 0 to {} (or -1 to -{} from the end)",
            format_number(n),
            len - 1,
            len
        ))
    }
}

/// Object keys are always strings; numbers and booleans are converted.
pub(crate) fn object_key(v: &Value) -> Result<Rc<str>> {
    match v {
        Value::Str(s) => Ok(s.clone()),
        Value::Number(_) | Value::Bool(_) => Ok(v.to_string().into()),
        other => Err(Error::type_(format!(
            "object keys are whiskers, not {}",
            article(other.type_name())
        ))),
    }
}

fn article(type_name: &str) -> String {
    match type_name {
        "mew" => "mew".to_string(),
        "whiskers" => "whiskers".to_string(),
        "object" => "an object".to_string(),
        other => format!("a {other}"),
    }
}

fn parent_dir(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
}

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::BufferHost;

    fn run(src: &str) -> (Result<Value>, Vec<String>) {
        let host = BufferHost::new();
        let mut interp = Interpreter::new(host.clone());
        let result = interp.run(src);
        (result, host.take())
    }

    fn eval(src: &str) -> Value {
        run(src).0.unwrap()
    }

    fn output(src: &str) -> Vec<String> {
        let (r, lines) = run(src);
        r.unwrap();
        lines
    }

    fn error(src: &str) -> Error {
        run(src).0.unwrap_err()
    }

    fn field(object: &Value, key: &str) -> Value {
        get_index(object, &Value::from(key)).unwrap()
    }

    #[test]
    fn arithmetic_and_precedence() {
        assert_eq!(eval("1 + 2 * 3"), Value::from(7.0));
        assert_eq!(eval("(1 + 2) * 3"), Value::from(9.0));
        assert_eq!(eval("10 % 3"), Value::from(1.0));
        assert_eq!(eval("-2 * -3"), Value::from(6.0));
        assert_eq!(eval("1 + 2 << 1"), Value::from(6.0));
        assert_eq!(eval("6 & 3 == 2"), Value::from(true));
    }

    #[test]
    fn strings_concatenate_with_anything() {
        assert_eq!(
            eval(r#""a" + 1 + purrfect + mew"#),
            Value::from("a1purrfectmew")
        );
        assert_eq!(eval(r#""ab" * 3"#), Value::from("ababab"));
        assert_eq!(eval(r#""b" > "a""#), Value::from(true));
    }

    #[test]
    fn variables_and_scopes() {
        assert_eq!(eval("scratch x = 1; amew x = x + 1; x"), Value::from(2.0));
        assert_eq!(
            eval("scratch x = 1; purrhaps purrfect { amew x = 5; } x"),
            Value::from(5.0)
        );
        assert_eq!(
            eval("scratch x = 1; purrhaps purrfect { scratch x = 5; } x"),
            Value::from(1.0)
        );
        let e = error("amew y = 1;");
        assert_eq!(e.kind, crate::error::ErrorKind::Name);
        let e = error("scratch count = 1; meow(cuont);");
        assert!(e.message.contains("did you mean `count`"), "{}", e.message);
    }

    #[test]
    fn functions_closures_and_recursion() {
        assert_eq!(
            eval("pawction add(a, b) { tail a + b; } add(2, 3)"),
            Value::from(5.0)
        );
        assert_eq!(
            eval("scratch f = pawction(x) { x * 2 }; f(4)"),
            Value::from(8.0)
        );
        assert_eq!(
            eval(
                "pawction fib(n) { purrhaps n < 2 { tail n; } tail fib(n - 1) + fib(n - 2); } fib(15)"
            ),
            Value::from(610.0)
        );
        let src = "pawction counter() { scratch n = 0; tail pawction() { amew n = n + 1; tail n; }; } \
                   scratch c = counter(); c(); c(); c()";
        assert_eq!(eval(src), Value::from(3.0));
        let e = error("pawction f(a) {} f(1, 2)");
        assert!(
            e.message.contains("`f` takes 1 argument, but got 2"),
            "{}",
            e.message
        );
    }

    #[test]
    fn deep_recursion_is_an_error_not_a_crash() {
        let handle = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(|| error("pawction down(n) { tail down(n + 1); } down(0)"))
            .unwrap();
        let e = handle.join().unwrap();
        assert!(e.message.contains("recursion"), "{}", e.message);
    }

    #[test]
    fn if_is_an_expression() {
        assert_eq!(
            eval("purrhaps clawful { 1 } meowtually purrhaps purrfect { 2 } meowtually { 3 }"),
            Value::from(2.0)
        );
        assert_eq!(
            eval("scratch x = purrhaps (1 > 2) { \"big\" } meowtually { \"small\" }; x"),
            Value::from("small")
        );
        assert_eq!(eval("purrhaps clawful { 1 }"), Value::Null);
    }

    #[test]
    fn truthiness() {
        assert_eq!(eval("!mew"), Value::from(true));
        assert_eq!(eval("!0"), Value::from(false));
        assert_eq!(eval(r#"!"""#), Value::from(false));
        assert_eq!(eval("mew || 3"), Value::from(3.0));
        assert_eq!(eval("1 && 2"), Value::from(2.0));
        assert_eq!(eval("clawful && undefined_name"), Value::from(false));
    }

    #[test]
    fn loops() {
        assert_eq!(
            output(
                "scratch i = 0; furrever { purrhaps i == 3 { hiss; } purr(i); amew i = i + 1; }"
            ),
            vec!["0", "1", "2"]
        );
        assert_eq!(
            output("scratch i = 0; furrever i < 2 { purr(i); amew i = i + 1; }"),
            vec!["0", "1"]
        );
        assert_eq!(
            output("fur x ~ [1, 2, 3] { purrhaps x == 2 { continue; } purr(x); }"),
            vec!["1", "3"]
        );
        assert_eq!(output("fur c ~ \"ab\" { purr(c); }"), vec!["a", "b"]);
        assert_eq!(
            output("fur k ~ { a: 1, b: 2 } { purr(k); }"),
            vec!["a", "b"]
        );
        assert_eq!(output("fur i ~ 3 { purr(i); }"), vec!["0", "1", "2"]);
        assert!(error("hiss;").message.contains("inside a loop"));
        assert!(
            error("pawction f() { hiss; } furrever { f(); }")
                .message
                .contains("inside a loop")
        );
    }

    #[test]
    fn furrballs_are_shared_and_indexable() {
        assert_eq!(eval("scratch a = [1, 2, 3]; a[-1]"), Value::from(3.0));
        assert_eq!(
            eval("scratch a = [1, 2, 3]; scratch b = a; amew b[0] = 9; a[0]"),
            Value::from(9.0)
        );
        assert_eq!(
            eval("[1, 2] + [3]"),
            Value::array(vec![1.0.into(), 2.0.into(), 3.0.into()])
        );
        assert_eq!(eval("2 ~ [1, 2]"), Value::from(true));
        assert_eq!(eval("\"b\"[0]"), Value::from("b"));
        let e = error("[1, 2][5]");
        assert!(e.message.contains("out of bounds"), "{}", e.message);
        assert!(error("[1][0.5]").message.contains("whole"));
    }

    #[test]
    fn objects_and_possessives() {
        let src = r#"scratch cat = { name: "Tom", "age": 3 }; amew cat's age = 4; cat's age + cat["age"]"#;
        assert_eq!(eval(src), Value::from(8.0));
        assert_eq!(eval("scratch o = {}; amew o's x = 1; o"), eval("{ x: 1 }"));
        assert_eq!(eval("{ a: 1 }'s missing"), Value::Null);
        assert_eq!(eval("\"a\" ~ { a: 1 }"), Value::from(true));
        assert_eq!(eval("scratch n = \"k\"; { [n]: 1 }'s k"), Value::from(1.0));
        assert_eq!(eval("scratch k = 2; { k }"), eval("{ k: 2 }"));
        assert_eq!(eval("furreal { }"), Value::from("object"));
    }

    #[test]
    fn typeof_names() {
        assert_eq!(eval("furreal mew"), Value::from("mew"));
        assert_eq!(eval("furreal 1"), Value::from("number"));
        assert_eq!(eval("furreal \"\""), Value::from("whiskers"));
        assert_eq!(eval("furreal []"), Value::from("furrball"));
        assert_eq!(eval("furreal purrfect"), Value::from("boolean"));
        assert_eq!(eval("furreal pawction() {}"), Value::from("pawction"));
        assert_eq!(eval("furreal meow"), Value::from("pawction"));
    }

    #[test]
    fn meow_prefixes_and_purr_does_not() {
        assert_eq!(
            output("meow(\"hi\", 1); purr(\"hi\", 1); meow();"),
            vec!["Meow! hi 1", "hi 1", "Meow!"]
        );
    }

    #[test]
    fn errors_carry_positions() {
        let src = "scratch x = 1;\nscratch y = x / 0;";
        let e = error(src);
        assert_eq!(
            e.span.map(|s| crate::span::position(src, s.start).line),
            Some(2)
        );
        let e = error("meow(nope)");
        assert_eq!(e.span, Some(Span::new(5, 9)));
    }

    #[test]
    fn packages() {
        assert_eq!(
            eval("pawckage \"nya:catculator\"; floor(PI)"),
            Value::from(3.0)
        );
        let e = error("pawckage \"nya:dogs\";");
        assert!(e.message.contains("nya:furrball"), "{}", e.message);
        assert!(error("floor(1)").message.contains("hasn't been"));
    }

    #[test]
    fn curious_catches_interpreter_errors_as_objects() {
        let src = "scratch got = mew;\ncurious {\n    scratch n = \"nine\" - 1;\n} caught err {\n    amew got = err;\n}\ngot";
        let got = eval(src);
        assert_eq!(field(&got, "kind"), Value::from("type"));
        assert_eq!(
            field(&got, "message"),
            Value::from("`-` doesn't work between whiskers and a number")
        );
        assert_eq!(field(&got, "line"), Value::from(3.0));
        assert_eq!(
            output(
                "pawckage \"nya:clawtility\"; curious { assert(clawful, \"no\"); } caught e { purr(e's kind, e's message); }"
            ),
            vec!["runtime assertion failed: no"]
        );
        assert_eq!(
            output("curious { nope(); } caught { purr(\"caught\"); } purr(\"after\");"),
            vec!["caught", "after"]
        );
    }

    #[test]
    fn curious_catches_a_yowled_value_as_it_was() {
        let src = "scratch thing = { code: 7 }; scratch got = mew;\
                   curious { yowl thing; } caught e { amew got = e; }\
                   amew got's seen = purrfect;\
                   [got's code, thing's seen]";
        assert_eq!(eval(src), Value::array(vec![7.0.into(), true.into()]));
        assert_eq!(
            output(
                "pawction deep(n) { purrhaps n == 0 { yowl \"bottom\"; } deep(n - 1); } curious { deep(5); } caught e { purr(e); }"
            ),
            vec!["bottom"]
        );
    }

    #[test]
    fn yowl_survives_a_trip_through_a_builtin() {
        let src = "pawckage \"nya:furrball\";\
                   curious { map([1, 2], pawction(x) { yowl { n: x }; }); } caught e { purr(e's n); }";
        assert_eq!(output(src), vec!["1"]);
        let e = error(
            "pawckage \"nya:furrball\"; each([1], pawction(x) { yowl \"from a callback\"; });",
        );
        assert_eq!(e.kind, ErrorKind::Thrown);
        assert_eq!(e.message, "from a callback");
    }

    #[test]
    fn control_flow_passes_through_curious() {
        assert_eq!(
            eval("pawction f() { curious { tail 1; } caught { tail 2; } tail 3; } f()"),
            Value::from(1.0)
        );
        assert_eq!(
            output(
                "fur i ~ 5 { curious { purrhaps i == 1 { continue; } purrhaps i == 3 { hiss; } purr(i); } caught { purr(\"no\"); } }"
            ),
            vec!["0", "2"]
        );
    }

    #[test]
    fn errors_in_caught_propagate_and_nesting_works() {
        let e = error("curious { yowl 1; } caught e { nope(); }");
        assert_eq!(e.kind, ErrorKind::Name);
        assert_eq!(
            output(
                "curious { curious { yowl 1; } caught e { yowl e + 1; } } caught e { purr(e); }"
            ),
            vec!["2"]
        );
    }

    #[test]
    fn uncaught_yowl_renders_like_any_other_error() {
        let src = "meow(\"start\");\nyowl \"out of kibble\";";
        let e = error(src);
        assert_eq!(e.kind, ErrorKind::Thrown);
        let text = e.render(src, Some("bowl.meow"));
        assert!(
            text.starts_with("Yowl! uncaught yowl: out of kibble"),
            "{text}"
        );
        assert!(text.contains("bowl.meow:2:1"), "{text}");
        assert_eq!(error("yowl { code: 1 };").message, "{code: 1}");
    }

    #[test]
    fn return_value_of_a_program_is_its_last_expression() {
        assert_eq!(eval("1; 2; scratch x = 3;"), Value::Null);
        assert_eq!(eval("1; 2"), Value::from(2.0));
        assert_eq!(eval("tail 7; 8"), Value::from(7.0));
    }
}
