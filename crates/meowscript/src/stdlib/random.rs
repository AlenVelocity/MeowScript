use super::math::RANDOM;
use super::{Package, array, integer, number};
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value};

pub static PACKAGE: Package = Package {
    name: "nya:rundamn",
    doc: "Random numbers, choices, and shuffles.",
    builtins: &[
        Builtin {
            name: "seed",
            arity: Arity::Exact(1),
            func: seed,
            doc: "Restart the random sequence from a number, so runs repeat exactly.",
        },
        RANDOM,
        Builtin {
            name: "uniform",
            arity: Arity::Exact(2),
            func: uniform,
            doc: "A random float in [a, b).",
        },
        Builtin {
            name: "randint",
            arity: Arity::Exact(2),
            func: randint,
            doc: "A random whole number from a to b, including both ends.",
        },
        Builtin {
            name: "choice",
            arity: Arity::Exact(1),
            func: choice,
            doc: "A random item from a furrball.",
        },
        Builtin {
            name: "shuffle",
            arity: Arity::Exact(1),
            func: shuffle,
            doc: "Shuffle a furrball in place and return it.",
        },
    ],
    constants: &[],
};

fn seed(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let n = number("seed", &args, 0)?;
    interp.reseed(n.to_bits());
    Ok(Value::Null)
}

fn uniform(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let a = number("uniform", &args, 0)?;
    let b = number("uniform", &args, 1)?;
    Ok(Value::Number(interp.rng().range_f64(a, b)))
}

fn randint(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let a = integer("randint", &args, 0)?;
    let b = integer("randint", &args, 1)?;
    Ok(Value::Number(interp.rng().range_i64(a, b) as f64))
}

fn choice(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("choice", &args, 0)?;
    let items = arr.borrow();
    if items.is_empty() {
        return Err(Error::runtime("`choice` can't pick from an empty furrball"));
    }
    let i = interp.rng().index(items.len());
    Ok(items[i].clone())
}

fn shuffle(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let arr = array("shuffle", &args, 0)?;
    interp.rng().shuffle(&mut arr.borrow_mut());
    Ok(args[0].clone())
}
