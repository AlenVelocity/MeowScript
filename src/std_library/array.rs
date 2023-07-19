use std::collections::HashMap;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals: HashMap<String, Object> = HashMap::new();
    globals.insert(String::from("pounce"), Object::Inbuilt(pop));
    globals.insert(String::from("top"), Object::Inbuilt(head));
    globals.insert(String::from("bottom"), Object::Inbuilt(tail));
    globals.insert(String::from("push"), Object::Inbuilt(push));
    globals.insert(String::from("includes"), Object::Inbuilt(includes));
    Res {
        globals,
        raw: Some("
            include \"std:util\";

            set map = fun (arr, f) {
                scratch res = [];
                scratch iter = fun (array) {
                    purrhaps (length(array) == 0) {
                        tail;
                    }  {
                        amew res = push(res, f(array[0]));
                        iter(bottom(array));
                    }
                };
            iter(arr)
            tail res;
            }; 
        ".to_string())
    }
}

/// The std:array-built-in function `push`.
/// Pushes an object onto the end of an array.
/// # Arguments
/// * `args` - The array to push onto.
pub fn push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2.",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.push(args[1].clone());
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn pop(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.pop();
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn head (args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.pop();
            Object::Array(array)
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn tail (args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => Object::Array(a[1..].to_vec()),
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}

pub fn includes (args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2.",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(a) => {
            let array = a.clone();
            Object::Bool(array.contains(&args[1]))
        }
        o => Object::Error(format!("First argument must be an array. Got {}", o)),
    }
}