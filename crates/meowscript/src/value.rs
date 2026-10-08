//! Strings are immutable and shared. Furrballs and objects are reference types, like in
//! JavaScript: `push` on one variable is seen through every other variable holding it.

use crate::ast::FuncDef;
use crate::env::Env;
use crate::error::Result;
use crate::interpreter::Interpreter;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub type Array = Rc<RefCell<Vec<Value>>>;
pub type Object = Rc<RefCell<IndexMap<Rc<str>, Value>>>;

#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    Str(Rc<str>),
    Array(Array),
    Object(Object),
    Function(Rc<Closure>),
    Builtin(&'static Builtin),
}

pub struct Closure {
    pub def: Rc<FuncDef>,
    pub env: Env,
}

pub struct Builtin {
    pub name: &'static str,
    pub arity: Arity,
    pub func: BuiltinFn,
    pub doc: &'static str,
}

pub type BuiltinFn = fn(&mut Interpreter, Vec<Value>) -> Result<Value>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arity {
    Exact(usize),
    AtLeast(usize),
    Between(usize, usize),
}

impl Arity {
    pub fn accepts(self, n: usize) -> bool {
        match self {
            Arity::Exact(k) => n == k,
            Arity::AtLeast(k) => n >= k,
            Arity::Between(lo, hi) => (lo..=hi).contains(&n),
        }
    }

    pub fn describe(self) -> String {
        let plural = |n: usize| if n == 1 { "argument" } else { "arguments" };
        match self {
            Arity::Exact(k) => format!("exactly {k} {}", plural(k)),
            Arity::AtLeast(k) => format!("at least {k} {}", plural(k)),
            Arity::Between(lo, hi) => format!("between {lo} and {hi} arguments"),
        }
    }
}

impl Value {
    pub fn array(items: Vec<Value>) -> Value {
        Value::Array(Rc::new(RefCell::new(items)))
    }

    pub fn object(map: IndexMap<Rc<str>, Value>) -> Value {
        Value::Object(Rc::new(RefCell::new(map)))
    }

    pub fn str(s: impl Into<Rc<str>>) -> Value {
        Value::Str(s.into())
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "mew",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::Str(_) => "whiskers",
            Value::Array(_) => "furrball",
            Value::Object(_) => "object",
            Value::Function(_) | Value::Builtin(_) => "pawction",
        }
    }

    /// Zero and the empty string are truthy, so `purrhaps (count)` does not skip zero.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Null | Value::Bool(false))
    }

    /// Like `Display`, but strings are quoted.
    pub fn repr(&self) -> String {
        let mut out = String::new();
        self.repr_into(&mut out, 0);
        out
    }

    fn repr_into(&self, out: &mut String, depth: usize) {
        if depth > 24 {
            out.push_str("...");
            return;
        }
        match self {
            Value::Str(s) => {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\t' => out.push_str("\\t"),
                        '\r' => out.push_str("\\r"),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            Value::Array(items) => {
                out.push('[');
                for (i, item) in items.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    item.repr_into(out, depth + 1);
                }
                out.push(']');
            }
            Value::Object(map) => {
                out.push('{');
                for (i, (k, v)) in map.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    if is_plain_key(k) {
                        out.push_str(k);
                    } else {
                        Value::Str(k.clone()).repr_into(out, depth + 1);
                    }
                    out.push_str(": ");
                    v.repr_into(out, depth + 1);
                }
                out.push('}');
            }
            other => out.push_str(&other.to_string()),
        }
    }
}

fn is_plain_key(k: &str) -> bool {
    let mut chars = k.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_alphabetic() => chars.all(|c| c == '_' || c.is_alphanumeric()),
        _ => false,
    }
}

pub fn format_number(n: f64) -> String {
    if n.is_nan() {
        "NaN".to_string()
    } else if n.is_infinite() {
        if n > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else if n == 0.0 {
        "0".to_string()
    } else {
        n.to_string()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => f.write_str("mew"),
            Value::Bool(true) => f.write_str("purrfect"),
            Value::Bool(false) => f.write_str("clawful"),
            Value::Number(n) => f.write_str(&format_number(*n)),
            Value::Str(s) => f.write_str(s),
            Value::Array(_) | Value::Object(_) => f.write_str(&self.repr()),
            Value::Function(c) => match &c.def.name {
                Some(name) => write!(f, "<pawction {name}>"),
                None => f.write_str("<pawction>"),
            },
            Value::Builtin(b) => write!(f, "<built-in pawction {}>", b.name),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.repr())
    }
}

/// Structural for data, identity for functions.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Object(a), Value::Object(b)) => {
                Rc::ptr_eq(a, b) || {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.len() == b.len() && a.iter().all(|(k, v)| b.get(k) == Some(v))
                }
            }
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::Builtin(a), Value::Builtin(b)) => std::ptr::eq(*a, *b),
            _ => false,
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<usize> for Value {
    fn from(n: usize) -> Self {
        Value::Number(n as f64)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.into())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s.into())
    }
}

impl From<Vec<Value>> for Value {
    fn from(items: Vec<Value>) -> Self {
        Value::array(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_print_like_a_human_would_write_them() {
        assert_eq!(format_number(3.0), "3");
        assert_eq!(format_number(3.5), "3.5");
        assert_eq!(format_number(-0.0), "0");
        assert_eq!(format_number(1e21), "1000000000000000000000");
        assert_eq!(format_number(f64::INFINITY), "Infinity");
    }

    #[test]
    fn repr_quotes_nested_strings_only() {
        let v = Value::array(vec![Value::from("a"), Value::from(1.0), Value::Null]);
        assert_eq!(v.to_string(), r#"["a", 1, mew]"#);
        assert_eq!(Value::from("a").to_string(), "a");
        assert_eq!(Value::from("a\"b").repr(), r#""a\"b""#);
    }

    #[test]
    fn objects_print_in_insertion_order() {
        let mut map = IndexMap::new();
        map.insert("name".into(), Value::from("Tom"));
        map.insert("has space".into(), Value::from(true));
        assert_eq!(
            Value::object(map).to_string(),
            r#"{name: "Tom", "has space": purrfect}"#
        );
    }

    #[test]
    fn equality_is_structural_for_data() {
        assert_eq!(
            Value::array(vec![Value::from(1.0)]),
            Value::array(vec![Value::from(1.0)])
        );
        assert_ne!(Value::from(1.0), Value::from("1"));
        assert_ne!(Value::Null, Value::Bool(false));
    }
}
