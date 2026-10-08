use super::{Package, callable, number, object, string, wrong_type};
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value};

pub const LENGTH: Builtin = Builtin {
    name: "length",
    arity: Arity::Exact(1),
    func: length,
    doc: "How many characters, items, or keys something has.",
};

pub static PACKAGE: Package = Package {
    name: "nya:clawtility",
    doc: "Input, timing, objects, and other everyday tools.",
    builtins: &[
        LENGTH,
        Builtin {
            name: "kibble",
            arity: Arity::Between(0, 1),
            func: kibble,
            doc: "Read a line of input. Shows the prompt first if you give one.",
        },
        Builtin {
            name: "nap",
            arity: Arity::Exact(1),
            func: nap,
            doc: "Pause for that many milliseconds.",
        },
        Builtin {
            name: "now",
            arity: Arity::Exact(0),
            func: now,
            doc: "Milliseconds since the Unix epoch.",
        },
        Builtin {
            name: "keys",
            arity: Arity::Exact(1),
            func: keys,
            doc: "A furrball of an object's keys, in insertion order.",
        },
        Builtin {
            name: "values",
            arity: Arity::Exact(1),
            func: values,
            doc: "A furrball of an object's values, in insertion order.",
        },
        Builtin {
            name: "remove",
            arity: Arity::Exact(2),
            func: remove,
            doc: "Remove a key from an object. Returns the removed value, or mew.",
        },
        Builtin {
            name: "assert",
            arity: Arity::Between(1, 2),
            func: assert,
            doc: "Stop with an error unless the first argument is truthy.",
        },
        Builtin {
            name: "apply",
            arity: Arity::Exact(2),
            func: apply,
            doc: "Call a pawction with a furrball of arguments.",
        },
    ],
    constants: &[],
};

fn length(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(match &args[0] {
        Value::Str(s) => Value::from(s.chars().count()),
        Value::Array(a) => Value::from(a.borrow().len()),
        Value::Object(o) => Value::from(o.borrow().len()),
        other => {
            return Err(wrong_type(
                "length",
                0,
                "whiskers, a furrball, or an object",
                other,
            ));
        }
    })
}

fn kibble(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let prompt = match args.first() {
        Some(v) => v.to_string(),
        None => String::new(),
    };
    interp.host().read_line(&prompt).map(Value::from)
}

fn nap(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let ms = number("nap", &args, 0)?;
    if ms < 0.0 {
        return Err(Error::type_(
            "`nap` can't sleep for a negative amount of time",
        ));
    }
    interp.host().sleep(ms);
    Ok(Value::Null)
}

fn now(interp: &mut Interpreter, _: Vec<Value>) -> Result<Value> {
    Ok(Value::Number(interp.host().now_millis()))
}

fn keys(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let o = object("keys", &args, 0)?;
    let keys = o.borrow().keys().map(|k| Value::Str(k.clone())).collect();
    Ok(Value::array(keys))
}

fn values(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let o = object("values", &args, 0)?;
    let values = o.borrow().values().cloned().collect();
    Ok(Value::array(values))
}

fn remove(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let o = object("remove", &args, 0)?;
    let key = string("remove", &args, 1)?;
    let removed = o.borrow_mut().shift_remove(&*key);
    Ok(removed.unwrap_or(Value::Null))
}

fn assert(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    if args[0].is_truthy() {
        return Ok(Value::Null);
    }
    let message = match args.get(1) {
        Some(m) => format!("assertion failed: {m}"),
        None => "assertion failed".to_string(),
    };
    Err(Error::runtime(message))
}

fn apply(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let f = callable("apply", &args, 0)?;
    let call_args = super::array("apply", &args, 1)?;
    let call_args = call_args.borrow().clone();
    interp.call(f, call_args, None)
}
