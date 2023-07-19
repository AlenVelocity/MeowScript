use super::store::Store;
use crate::ast::{BlockStatement, Ident};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub type InbuiltFunction = fn(Vec<Object>) -> Object;

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Error(String),
    Fn(Vec<Ident>, BlockStatement, Rc<RefCell<Store>>),
    Inbuilt(InbuiltFunction),
    Array(Vec<Object>),
    Object(HashMap<Object, Object>),
    Typeof(Box<Object>),
    Loop(Box<BlockStatement>),
    Break,
    Continue
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Return(a), Object::Return(b)) => a == b,
            (Object::Error(a), Object::Error(b)) => a == b,
            (Object::Fn(a, b, c), Object::Fn(d, e, f)) => a == d && b == e && c == f,
            (Object::Inbuilt(a), Object::Inbuilt(b)) => a == b,
            (Object::Array(a), Object::Array(b)) => a == b,
            (Object::Object(a), Object::Object(b)) => a == b,
            (Object::Typeof(a), Object::Typeof(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Object {}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Number(ref value) => write!(f, "{}", value),
            Object::String(ref value) => write!(f, "{}", value),
            Object::Bool(ref value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(ref value) => write!(f, "{}", value),
            Object::Error(ref value) => write!(f, "{}", value),
            Object::Fn(ref params, _, _) => {
                let mut result = String::new();
                for (i, Ident(ref s)) in params.iter().enumerate() {
                    if i < 1 {
                        result.push_str(s);
                    } else {
                        result.push_str(&format!(", {}", s));
                    }
                }
                write!(f, "fn({}) {{ ... }}", result)
            }
            Object::Inbuilt(_) => write!(f, "[inbuilt fn]"),
            Object::Array(ref val) => {
                let mut result = String::new();
                for (i, obj) in val.iter().enumerate() {
                    if i < 1 {
                        result.push_str(&format!("{}", obj));
                    } else {
                        result.push_str(&format!(", {}", obj));
                    }
                }
                write!(f, "[{}]", result)
            }
            Object::Object(ref hash) => {
                let mut res = String::new();
                for (i, (k, v)) in hash.iter().enumerate() {
                    if i < 1 {
                        res.push_str(&format!("{}: {}", k, v));
                    } else {
                        res.push_str(&format!(", {}: {}", k, v));
                    }
                }

                write!(f, "{{{}}}", res)
            }
            Object::Typeof(ref obj) => write!(f, "typeof({})", obj),
            Object::Loop(ref _block) => write!(f, "loop {{ ... }}"),
            Object::Break => write!(f, "break"),
            Object::Continue => write!(f, "continue"),
        }
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Object::Bool(ref b) => b.hash(state),
            Object::String(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}