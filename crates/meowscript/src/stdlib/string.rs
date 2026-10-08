use super::array::{INCLUDES, INDEX_OF, REVERSE, SLICE};
use super::{Package, integer, string};
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value};

pub static PACKAGE: Package = Package {
    name: "nya:whiskers",
    doc: "Everything for whiskers (strings).",
    builtins: &[
        Builtin {
            name: "in_whiskers",
            arity: Arity::Exact(1),
            func: in_whiskers,
            doc: "Turn anything into whiskers, the way `meow` would print it.",
        },
        Builtin {
            name: "pawrse",
            arity: Arity::Exact(1),
            func: pawrse,
            doc: "Read a number out of whiskers, or mew if there isn't one.",
        },
        Builtin {
            name: "replace",
            arity: Arity::Exact(3),
            func: replace,
            doc: "Replace every occurrence of `from` with `to`.",
        },
        Builtin {
            name: "split",
            arity: Arity::Between(1, 2),
            func: split,
            doc: "Split into a furrball on a separator. With no separator, splits on whitespace.",
        },
        Builtin {
            name: "upper",
            arity: Arity::Exact(1),
            func: upper,
            doc: "SHOUTING.",
        },
        Builtin {
            name: "lower",
            arity: Arity::Exact(1),
            func: lower,
            doc: "whispering.",
        },
        Builtin {
            name: "trim",
            arity: Arity::Exact(1),
            func: trim,
            doc: "Strip whitespace from both ends.",
        },
        Builtin {
            name: "starts_with",
            arity: Arity::Exact(2),
            func: starts_with,
            doc: "Whether the whiskers begin with the given prefix.",
        },
        Builtin {
            name: "ends_with",
            arity: Arity::Exact(2),
            func: ends_with,
            doc: "Whether the whiskers end with the given suffix.",
        },
        INCLUDES,
        INDEX_OF,
        Builtin {
            name: "chars",
            arity: Arity::Exact(1),
            func: chars,
            doc: "A furrball of the individual characters.",
        },
        Builtin {
            name: "repeat",
            arity: Arity::Exact(2),
            func: repeat,
            doc: "The whiskers repeated that many times. Same as `*`.",
        },
        Builtin {
            name: "pad_start",
            arity: Arity::Between(2, 3),
            func: pad_start,
            doc: "Pad on the left to a width, with spaces or the given filler.",
        },
        Builtin {
            name: "pad_end",
            arity: Arity::Between(2, 3),
            func: pad_end,
            doc: "Pad on the right to a width, with spaces or the given filler.",
        },
        REVERSE,
        SLICE,
    ],
    constants: &[],
};

fn in_whiskers(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(Value::from(args[0].to_string()))
}

fn pawrse(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(match &args[0] {
        Value::Number(n) => Value::Number(*n),
        Value::Str(s) => s
            .trim()
            .parse::<f64>()
            .map(Value::Number)
            .unwrap_or(Value::Null),
        _ => Value::Null,
    })
}

fn replace(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("replace", &args, 0)?;
    let from = string("replace", &args, 1)?;
    let to = string("replace", &args, 2)?;
    if from.is_empty() {
        return Err(Error::runtime(
            "`replace` can't replace an empty string; there's nothing to find",
        ));
    }
    Ok(Value::from(s.replace(&*from, &to)))
}

fn split(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("split", &args, 0)?;
    let parts: Vec<Value> = match args.get(1) {
        None => s.split_whitespace().map(Value::from).collect(),
        Some(_) => {
            let sep = string("split", &args, 1)?;
            if sep.is_empty() {
                s.chars().map(|c| Value::str(c.to_string())).collect()
            } else {
                s.split(&*sep).map(Value::from).collect()
            }
        }
    };
    Ok(Value::array(parts))
}

fn upper(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(Value::from(string("upper", &args, 0)?.to_uppercase()))
}

fn lower(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(Value::from(string("lower", &args, 0)?.to_lowercase()))
}

fn trim(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(Value::from(string("trim", &args, 0)?.trim()))
}

fn starts_with(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("starts_with", &args, 0)?;
    let prefix = string("starts_with", &args, 1)?;
    Ok(Value::Bool(s.starts_with(&*prefix)))
}

fn ends_with(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("ends_with", &args, 0)?;
    let suffix = string("ends_with", &args, 1)?;
    Ok(Value::Bool(s.ends_with(&*suffix)))
}

fn chars(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("chars", &args, 0)?;
    Ok(Value::array(
        s.chars().map(|c| Value::str(c.to_string())).collect(),
    ))
}

fn repeat(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let s = string("repeat", &args, 0)?;
    let n = integer("repeat", &args, 1)?;
    if n < 0 {
        return Err(Error::type_(
            "`repeat` can't repeat something a negative number of times",
        ));
    }
    Ok(Value::from(s.repeat(n as usize)))
}

fn pad(fname: &str, args: &[Value], at_start: bool) -> Result<Value> {
    let s = string(fname, args, 0)?;
    let width = integer(fname, args, 1)?.max(0) as usize;
    let fill = match args.get(2) {
        Some(_) => string(fname, args, 2)?,
        None => " ".into(),
    };
    let len = s.chars().count();
    if fill.is_empty() || len >= width {
        return Ok(Value::Str(s));
    }
    let padding: String = fill.chars().cycle().take(width - len).collect();
    Ok(Value::from(if at_start {
        format!("{padding}{s}")
    } else {
        format!("{s}{padding}")
    }))
}

fn pad_start(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    pad("pad_start", &args, true)
}

fn pad_end(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    pad("pad_end", &args, false)
}
