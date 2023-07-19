use std::collections::HashMap;
use rand::Rng;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("random"), Object::Inbuilt(random));
    globals.insert(String::from("round"), Object::Inbuilt(round));
    globals.insert(String::from("ceil"), Object::Inbuilt(ceil));
    globals.insert(String::from("floor"), Object::Inbuilt(floor));
    globals.insert(String::from("abs"), Object::Inbuilt(abs));
    globals.insert(String::from("sqrt"), Object::Inbuilt(sqrt));
    globals.insert(String::from("sin"), Object::Inbuilt(sin));
    globals.insert(String::from("cos"), Object::Inbuilt(cos));
    globals.insert(String::from("tan"), Object::Inbuilt(tan));
    globals.insert(String::from("pow"), Object::Inbuilt(pow));
    globals.insert(String::from("log2"), Object::Inbuilt(log2));
    globals.insert(String::from("log10"), Object::Inbuilt(log10));
    globals.insert(String::from("modulo"), Object::Inbuilt(modulo));
    globals.insert(String::from("Math.PI"), Object::Number(std::f64::consts::PI));
    globals.insert(String::from("Math.E"), Object::Number(std::f64::consts::E));
    globals.insert(String::from("MAX_INT"), Object::Number(std::f64::MAX));
    globals.insert(String::from("MIN_INT"), Object::Number(std::f64::MIN));
    Res {
        globals,
        raw: None,
    }
}

pub fn random(args: Vec<Object>) -> Object {
    let min = match &args[0] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let max = match &args[1] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(min..max);
    Object::Number(random_number)
}

pub fn round(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.round()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn log2(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.log2()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn log10(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.log10()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn sin(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.sin()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn cos(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.cos()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn tan(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.tan()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn pow(args: Vec<Object>) -> Object {
    let base = match &args[0] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let to = match &args[1] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let result = f64::powf(base, to);
    Object::Number(result)
}

pub fn modulo(args: Vec<Object>) -> Object {
    let a = match &args[0] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let b = match &args[1] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let mut result = a % b;
    result = result + b;
    result = result % b;

    Object::Number(result)
}


pub fn floor(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.floor()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn ceil(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.ceil()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn abs(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.abs()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn sqrt(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.sqrt()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}
