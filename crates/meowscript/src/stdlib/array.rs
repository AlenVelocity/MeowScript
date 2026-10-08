//! `push`, `pounce`, and `shuffle` change the furrball they are given. Everything else returns a
//! new one.

use super::{Package, array, callable, number, slice_bound, wrong_type};
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value, format_number};
use std::cmp::Ordering;

pub const INCLUDES: Builtin = Builtin {
    name: "includes",
    arity: Arity::Exact(2),
    func: includes,
    doc: "Whether a furrball has an item, or whiskers contain a substring. Same as `~`.",
};

pub const INDEX_OF: Builtin = Builtin {
    name: "index_of",
    arity: Arity::Exact(2),
    func: index_of,
    doc: "Where an item (or substring) first appears, or -1.",
};

pub const REVERSE: Builtin = Builtin {
    name: "reverse",
    arity: Arity::Exact(1),
    func: reverse,
    doc: "A reversed copy of a furrball or whiskers.",
};

pub const SLICE: Builtin = Builtin {
    name: "slice",
    arity: Arity::Between(2, 3),
    func: slice,
    doc: "A copy of part of a furrball or whiskers, from `start` up to (not including) `end`. Negative counts from the end.",
};

pub static PACKAGE: Package = Package {
    name: "nya:furrball",
    doc: "Everything for furrballs (arrays).",
    builtins: &[
        Builtin {
            name: "push",
            arity: Arity::AtLeast(2),
            func: push,
            doc: "Add items to the end of a furrball. Changes it in place and returns it.",
        },
        Builtin {
            name: "pounce",
            arity: Arity::Exact(1),
            func: pounce,
            doc: "Remove and return the last item, or mew if the furrball is empty.",
        },
        Builtin {
            name: "top",
            arity: Arity::Exact(1),
            func: top,
            doc: "The first item, or mew.",
        },
        Builtin {
            name: "last",
            arity: Arity::Exact(1),
            func: last,
            doc: "The last item, or mew.",
        },
        Builtin {
            name: "bottom",
            arity: Arity::Exact(1),
            func: bottom,
            doc: "A new furrball with everything but the first item.",
        },
        INCLUDES,
        INDEX_OF,
        Builtin {
            name: "map",
            arity: Arity::Exact(2),
            func: map,
            doc: "A new furrball made by calling the pawction on each item.",
        },
        Builtin {
            name: "filter",
            arity: Arity::Exact(2),
            func: filter,
            doc: "A new furrball of the items the pawction says purrfect to.",
        },
        Builtin {
            name: "reduce",
            arity: Arity::Between(2, 3),
            func: reduce,
            doc: "Fold a furrball into one value with `pawction(total, item)`, starting from the optional initial value.",
        },
        Builtin {
            name: "each",
            arity: Arity::Exact(2),
            func: each,
            doc: "Call the pawction on every item. Returns mew.",
        },
        Builtin {
            name: "find",
            arity: Arity::Exact(2),
            func: find,
            doc: "The first item the pawction says purrfect to, or mew.",
        },
        Builtin {
            name: "range",
            arity: Arity::Between(1, 3),
            func: range,
            doc: "Numbers from `start` (default 0) up to but not including `end`, in `step`s (default 1).",
        },
        REVERSE,
        Builtin {
            name: "sort",
            arity: Arity::Between(1, 2),
            func: sort,
            doc: "A sorted copy. Give a `pawction(a, b)` returning a number to decide the order.",
        },
        Builtin {
            name: "join",
            arity: Arity::Between(1, 2),
            func: join,
            doc: "Glue the items into whiskers with a separator (default none).",
        },
        SLICE,
        Builtin {
            name: "concat",
            arity: Arity::AtLeast(1),
            func: concat,
            doc: "A new furrball with the items of all the given furrballs.",
        },
    ],
    constants: &[],
};

fn push(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("push", &args, 0)?;
    arr.borrow_mut().extend(args[1..].iter().cloned());
    Ok(args[0].clone())
}

fn pounce(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("pounce", &args, 0)?;
    let popped = arr.borrow_mut().pop();
    Ok(popped.unwrap_or(Value::Null))
}

fn top(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("top", &args, 0)?;
    let first = arr.borrow().first().cloned();
    Ok(first.unwrap_or(Value::Null))
}

fn last(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("last", &args, 0)?;
    let last = arr.borrow().last().cloned();
    Ok(last.unwrap_or(Value::Null))
}

fn bottom(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("bottom", &args, 0)?;
    let rest = arr.borrow().iter().skip(1).cloned().collect();
    Ok(Value::array(rest))
}

fn includes(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(match (&args[0], &args[1]) {
        (Value::Array(a), item) => Value::Bool(a.borrow().contains(item)),
        (Value::Str(s), Value::Str(needle)) => Value::Bool(s.contains(&**needle)),
        (Value::Str(_), other) => return Err(wrong_type("includes", 1, "whiskers", other)),
        (other, _) => return Err(wrong_type("includes", 0, "a furrball or whiskers", other)),
    })
}

fn index_of(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let found = match (&args[0], &args[1]) {
        (Value::Array(a), item) => a.borrow().iter().position(|v| v == item),
        (Value::Str(s), Value::Str(needle)) => {
            s.find(&**needle).map(|byte| s[..byte].chars().count())
        }
        (Value::Str(_), other) => return Err(wrong_type("index_of", 1, "whiskers", other)),
        (other, _) => return Err(wrong_type("index_of", 0, "a furrball or whiskers", other)),
    };
    Ok(Value::Number(found.map_or(-1.0, |i| i as f64)))
}

fn map(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("map", &args, 0)?;
    let f = callable("map", &args, 1)?;
    let items = arr.borrow().clone();
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(interp.call(f.clone(), vec![item], None)?);
    }
    Ok(Value::array(out))
}

fn filter(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("filter", &args, 0)?;
    let f = callable("filter", &args, 1)?;
    let items = arr.borrow().clone();
    let mut out = Vec::new();
    for item in items {
        if interp
            .call(f.clone(), vec![item.clone()], None)?
            .is_truthy()
        {
            out.push(item);
        }
    }
    Ok(Value::array(out))
}

fn reduce(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("reduce", &args, 0)?;
    let f = callable("reduce", &args, 1)?;
    let items = arr.borrow().clone();
    let mut iter = items.into_iter();
    let mut total = match args.get(2) {
        Some(initial) => initial.clone(),
        None => iter.next().ok_or_else(|| {
            Error::runtime("`reduce` of an empty furrball needs an initial value")
        })?,
    };
    for item in iter {
        total = interp.call(f.clone(), vec![total, item], None)?;
    }
    Ok(total)
}

fn each(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("each", &args, 0)?;
    let f = callable("each", &args, 1)?;
    let items = arr.borrow().clone();
    for item in items {
        interp.call(f.clone(), vec![item], None)?;
    }
    Ok(Value::Null)
}

fn find(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("find", &args, 0)?;
    let f = callable("find", &args, 1)?;
    let items = arr.borrow().clone();
    for item in items {
        if interp
            .call(f.clone(), vec![item.clone()], None)?
            .is_truthy()
        {
            return Ok(item);
        }
    }
    Ok(Value::Null)
}

fn range(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let (start, end, step) = match args.len() {
        1 => (0.0, number("range", &args, 0)?, 1.0),
        2 => (number("range", &args, 0)?, number("range", &args, 1)?, 1.0),
        _ => (
            number("range", &args, 0)?,
            number("range", &args, 1)?,
            number("range", &args, 2)?,
        ),
    };
    if step == 0.0 {
        return Err(Error::runtime(
            "`range` can't step by 0; it would never arrive",
        ));
    }
    let count = ((end - start) / step).ceil().max(0.0);
    if count > 10_000_000.0 {
        return Err(Error::runtime(format!(
            "`range` would make {} numbers, which is more than I'm willing to hold",
            format_number(count)
        )));
    }
    let items = (0..count as usize)
        .map(|i| Value::Number(start + step * i as f64))
        .collect();
    Ok(Value::array(items))
}

fn reverse(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(match &args[0] {
        Value::Array(a) => Value::array(a.borrow().iter().rev().cloned().collect()),
        Value::Str(s) => Value::str(s.chars().rev().collect::<String>()),
        other => return Err(wrong_type("reverse", 0, "a furrball or whiskers", other)),
    })
}

fn sort(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("sort", &args, 0)?;
    let mut items = arr.borrow().clone();
    if args.len() > 1 {
        let cmp = callable("sort", &args, 1)?;
        let mut failure = None;
        items.sort_by(|a, b| {
            if failure.is_some() {
                return Ordering::Equal;
            }
            match interp.call(cmp.clone(), vec![a.clone(), b.clone()], None) {
                Ok(Value::Number(n)) => n.partial_cmp(&0.0).unwrap_or(Ordering::Equal),
                Ok(other) => {
                    failure = Some(Error::type_(format!(
                        "the `sort` pawction must return a number (negative, zero, or positive), not {}",
                        other.type_name()
                    )));
                    Ordering::Equal
                }
                Err(e) => {
                    failure = Some(e);
                    Ordering::Equal
                }
            }
        });
        if let Some(e) = failure {
            return Err(e);
        }
        return Ok(Value::array(items));
    }
    let all_numbers = items.iter().all(|v| matches!(v, Value::Number(_)));
    let all_strings = items.iter().all(|v| matches!(v, Value::Str(_)));
    if all_numbers {
        items.sort_by(|a, b| match (a, b) {
            (Value::Number(x), Value::Number(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
            _ => Ordering::Equal,
        });
    } else if all_strings {
        items.sort_by(|a, b| match (a, b) {
            (Value::Str(x), Value::Str(y)) => x.cmp(y),
            _ => Ordering::Equal,
        });
    } else {
        return Err(Error::type_(
            "`sort` can order a furrball of only numbers or only whiskers; for anything else give it a pawction(a, b)",
        ));
    }
    Ok(Value::array(items))
}

fn join(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("join", &args, 0)?;
    let sep = match args.get(1) {
        Some(v) => v.to_string(),
        None => String::new(),
    };
    let joined = arr
        .borrow()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(&sep);
    Ok(Value::from(joined))
}

fn slice(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let start = number("slice", &args, 1)?;
    let end = if args.len() > 2 {
        Some(number("slice", &args, 2)?)
    } else {
        None
    };
    Ok(match &args[0] {
        Value::Array(a) => {
            let a = a.borrow();
            let lo = slice_bound(start, a.len());
            let hi = end.map_or(a.len(), |e| slice_bound(e, a.len())).max(lo);
            Value::array(a[lo..hi].to_vec())
        }
        Value::Str(s) => {
            let chars: Vec<char> = s.chars().collect();
            let lo = slice_bound(start, chars.len());
            let hi = end
                .map_or(chars.len(), |e| slice_bound(e, chars.len()))
                .max(lo);
            Value::str(chars[lo..hi].iter().collect::<String>())
        }
        other => return Err(wrong_type("slice", 0, "a furrball or whiskers", other)),
    })
}

fn concat(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let mut out = Vec::new();
    for i in 0..args.len() {
        let a = array("concat", &args, i)?;
        out.extend(a.borrow().iter().cloned());
    }
    Ok(Value::array(out))
}
