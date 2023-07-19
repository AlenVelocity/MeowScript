use std::{format, collections::HashMap};
use crate::evaluation::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("length"), Object::Inbuilt(length));
    globals.insert(String::from("kibble"), Object::Inbuilt(kibble));
    globals.insert(String::from("nap"), Object::Inbuilt(nap));
    Res { globals, raw: None }
}

/// Function to get the length of an array or string.
/// # Arguments
/// * `args` - The array or string to get the length of.
/// # Returns
/// `Object` - The length of the array or string.
pub fn length(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
            
        ));
    }
    match &args[0] {
        Object::String(s) => Object::Number(s.len() as f64),
        Object::Array(a) => Object::Number(a.len() as f64),
        o => Object::Error(format!("Argument must be a string or array. Got {}", o)),
    }
}

pub fn kibble(args: Vec<Object>) -> Object {
    println!("{}", &args[0]);
    if let Object::String(s) = &args[0] {
        print!("{}", s);
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    Object::String(input)
}

pub fn nap(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    if let Object::Number(n) = &args[0] {
        std::thread::sleep(std::time::Duration::from_millis(*n as u64));
    }
    Object::Null
}