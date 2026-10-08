//! The prelude is always available; everything else arrives with `pawckage "nya:<name>";`.
//! Every function is a plain Rust `fn` in a static table, so adding one means adding a row.

mod array;
mod fs;
mod math;
mod random;
mod string;
mod util;

use crate::env::Env;
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Array, Builtin, Object, Value, format_number};
use std::rc::Rc;

pub struct Package {
    pub name: &'static str,
    pub doc: &'static str,
    pub builtins: &'static [Builtin],
    pub constants: &'static [(&'static str, f64)],
}

impl Package {
    pub fn install(&'static self, env: &Env) {
        for b in self.builtins {
            env.define(b.name, Value::Builtin(b));
        }
        for (name, value) in self.constants {
            env.define(*name, Value::Number(*value));
        }
    }
}

pub static PACKAGES: &[&Package] = &[
    &util::PACKAGE,
    &array::PACKAGE,
    &string::PACKAGE,
    &math::PACKAGE,
    &random::PACKAGE,
    &fs::PACKAGE,
];

pub fn package(short_name: &str) -> Option<&'static Package> {
    PACKAGES
        .iter()
        .copied()
        .find(|p| p.name.strip_prefix("nya:") == Some(short_name))
}

/// Whether a binding came from a `nya:` package, so a file `pawckage` doesn't re-export the
/// packages it imported for itself.
pub fn is_package_provided(name: &str, value: &Value) -> bool {
    match value {
        Value::Builtin(_) => true,
        Value::Number(n) => PACKAGES
            .iter()
            .flat_map(|p| p.constants)
            .any(|(cname, cvalue)| *cname == name && cvalue.to_bits() == n.to_bits()),
        _ => false,
    }
}

pub static PRELUDE: &[Builtin] = &[
    Builtin {
        name: "meow",
        arity: Arity::AtLeast(0),
        func: meow,
        doc: "Print the arguments on one line, announced with \"Meow!\".",
    },
    Builtin {
        name: "purr",
        arity: Arity::AtLeast(0),
        func: purr,
        doc: "Print the arguments on one line, separated by spaces.",
    },
    Builtin {
        name: "log",
        arity: Arity::AtLeast(0),
        func: purr,
        doc: "The same as `purr`, for people who miss JavaScript.",
    },
    util::LENGTH,
];

pub fn install_prelude(env: &Env) {
    for b in PRELUDE {
        env.define(b.name, Value::Builtin(b));
    }
}

fn join_display(args: &[Value]) -> String {
    args.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn meow(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let line = if args.is_empty() {
        "Meow!".to_string()
    } else {
        format!("Meow! {}", join_display(&args))
    };
    interp.print(&line);
    Ok(Value::Null)
}

fn purr(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    interp.print(&join_display(&args));
    Ok(Value::Null)
}

fn wrong_type(fname: &str, index: usize, expected: &str, got: &Value) -> Error {
    Error::type_(format!(
        "`{fname}` wants {expected} as argument {}, but got {}",
        index + 1,
        got.type_name()
    ))
}

fn number(fname: &str, args: &[Value], i: usize) -> Result<f64> {
    match args.get(i) {
        Some(Value::Number(n)) => Ok(*n),
        Some(other) => Err(wrong_type(fname, i, "a number", other)),
        None => Err(Error::arity(format!(
            "`{fname}` is missing argument {}",
            i + 1
        ))),
    }
}

fn integer(fname: &str, args: &[Value], i: usize) -> Result<i64> {
    let n = number(fname, args, i)?;
    if n.fract() != 0.0 || !n.is_finite() {
        return Err(Error::type_(format!(
            "`{fname}` wants a whole number as argument {}, but got {}",
            i + 1,
            format_number(n)
        )));
    }
    Ok(n as i64)
}

fn string(fname: &str, args: &[Value], i: usize) -> Result<Rc<str>> {
    match args.get(i) {
        Some(Value::Str(s)) => Ok(s.clone()),
        Some(other) => Err(wrong_type(fname, i, "whiskers", other)),
        None => Err(Error::arity(format!(
            "`{fname}` is missing argument {}",
            i + 1
        ))),
    }
}

fn array(fname: &str, args: &[Value], i: usize) -> Result<Array> {
    match args.get(i) {
        Some(Value::Array(a)) => Ok(a.clone()),
        Some(other) => Err(wrong_type(fname, i, "a furrball", other)),
        None => Err(Error::arity(format!(
            "`{fname}` is missing argument {}",
            i + 1
        ))),
    }
}

fn object(fname: &str, args: &[Value], i: usize) -> Result<Object> {
    match args.get(i) {
        Some(Value::Object(o)) => Ok(o.clone()),
        Some(other) => Err(wrong_type(fname, i, "an object", other)),
        None => Err(Error::arity(format!(
            "`{fname}` is missing argument {}",
            i + 1
        ))),
    }
}

fn callable(fname: &str, args: &[Value], i: usize) -> Result<Value> {
    match args.get(i) {
        Some(v @ (Value::Function(_) | Value::Builtin(_))) => Ok(v.clone()),
        Some(other) => Err(wrong_type(fname, i, "a pawction", other)),
        None => Err(Error::arity(format!(
            "`{fname}` is missing argument {}",
            i + 1
        ))),
    }
}

/// Clamps a possibly negative slice bound into `0..=len`.
fn slice_bound(n: f64, len: usize) -> usize {
    let i = n as i64;
    let i = if i < 0 { i + len as i64 } else { i };
    i.clamp(0, len as i64) as usize
}
