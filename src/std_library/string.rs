use std::collections::HashMap;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("replace"), Object::Inbuilt(replace));
    globals.insert(String::from("in_whiskers"), Object::Inbuilt(to_string));
    Res {
        globals,
        raw: None,
    }
}

pub fn replace(args: Vec<Object>) -> Object {
    if args.len() != 3 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 3.",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => {
            let mut s = s.clone();
            s = s.replace(&args[1].to_string(), &args[2].to_string());
            Object::String(s)
        }
        o => Object::Error(format!("First argument must be a string. Got {}", o)),
    }
}

pub fn to_string(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => Object::String(s.clone()),
        Object::Array(a) => {
            let mut s = String::new();
            for o in a.iter() {
                s.push_str(&o.to_string());
            }
            Object::String(s)
        },
        Object::Number(n) => Object::String(n.to_string()),
        Object::Bool(b) => Object::String(b.to_string()),
        Object::Null => Object::String(String::from("null")),
        Object::Error(e) => Object::String(e.clone()),
        Object::Fn(..) => Object::String(String::from("[Function]")),
        Object::Inbuilt(..) => Object::String(String::from("[Inbuilt Function]")),
        o => Object::String(format!("{}", o))        
    }
}