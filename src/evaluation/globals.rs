use super::object::*;
use std::collections::HashMap;

/// Adds the built-in functions to the global environment.
/// This is called once at the start of the program.
/// The global environment is a HashMap of names to objects.
pub fn new_globals() -> HashMap<String, Object> {
    let mut globals = HashMap::new();
    globals.insert(String::from("meow"), Object::Inbuilt(meow));
    globals.insert(String::from("log"), Object::Inbuilt(log));
    globals
}

/// The built-in function `log`.
/// It takes an unlimited number of arguments,
/// and logs them to the console.
/// It returns `null`.
fn log(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Error(String::from("Wrong number of arguments"));
    } else {
        for arg in args {
            print!("{} ", arg);
        }
        println!();
    }
    Object::Null
}

fn meow(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::Error(String::from("Wrong number of arguments"));
    } else {
        print!("Meow!");
        for arg in args {
            print!(" {}", arg);
        }
        println!();
    }
    Object::Null
}