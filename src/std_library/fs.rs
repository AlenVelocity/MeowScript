use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::evaluation::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("readFile"), Object::Inbuilt(read_file));
    globals.insert(String::from("writeFile"), Object::Inbuilt(write_file));
    globals.insert(String::from("exists"), Object::Inbuilt(file_exists));
    Res {
        globals,
        raw: None
    }
        
}

pub fn read_file(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => {
            let path = Path::new(s);
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => {
                    return Object::Error(format!("Couldn't open {}: {}", display, why))
                }
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => {
                    return Object::Error(format!("Couldn't read {}: {}", display, why))
                }
                Ok(_) => {
                    Object::String(s)
                }
            }
        }
        _ => Object::Error(format!("Argument must be a string. Got {}", args[0]))
    }
}

pub fn write_file(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2.",
            args.len()
        ));
    }
    match &args[0] {
        Object::String(s) => {
            let path = Path::new(s);
            let display = path.display();
            let mut file = match File::create(&path) {
                Err(why) => {
                    return Object::Error(format!("Couldn't create {}: {}", display, why))
                }
                Ok(file) => file,
            };
            match file.write_all(args[1].to_string().as_bytes()) {
                Err(why) => {
                    return Object::Error(format!("Couldn't write to {}: {}", display, why))
                }
                Ok(_) => {
                    Object::Null
                }
            }
        }
        _ => Object::Error(format!("Argument must be a string. Got {}", args[0]))
    }
}

pub fn file_exists(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
        ));
    }
    match &args[0] {
        Object::String(s) => {
            let path = Path::new(s);
            if path.exists() {
                Object::Bool(true)
            } else {
                Object::Bool(false)
            }
        }
        _ => Object::Error(format!("Argument must be a string. Got {}", args[0]))
    }
}

