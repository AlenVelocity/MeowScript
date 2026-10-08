use super::{Package, number};
use crate::error::{Error, Result};
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value};

pub const RANDOM: Builtin = Builtin {
    name: "random",
    arity: Arity::Between(0, 2),
    func: random,
    doc: "A random number: in [0, 1) with no arguments, [0, n) with one, or [min, max) with two.",
};

macro_rules! unary {
    ($name:ident, $f:expr) => {
        fn $name(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
            let n = number(stringify!($name), &args, 0)?;
            Ok(Value::Number($f(n)))
        }
    };
}

unary!(round, f64::round);
unary!(ceil, f64::ceil);
unary!(floor, f64::floor);
unary!(trunc, f64::trunc);
unary!(abs, f64::abs);
unary!(sqrt, f64::sqrt);
unary!(sin, f64::sin);
unary!(cos, f64::cos);
unary!(tan, f64::tan);
unary!(log2, f64::log2);
unary!(log10, f64::log10);
unary!(ln, f64::ln);
unary!(exp, f64::exp);

pub static PACKAGE: Package = Package {
    name: "nya:catculator",
    doc: "Maths. Angles are in radians.",
    builtins: &[
        RANDOM,
        Builtin {
            name: "round",
            arity: Arity::Exact(1),
            func: round,
            doc: "Round to the nearest whole number.",
        },
        Builtin {
            name: "ceil",
            arity: Arity::Exact(1),
            func: ceil,
            doc: "Round up.",
        },
        Builtin {
            name: "floor",
            arity: Arity::Exact(1),
            func: floor,
            doc: "Round down.",
        },
        Builtin {
            name: "trunc",
            arity: Arity::Exact(1),
            func: trunc,
            doc: "Drop the fractional part.",
        },
        Builtin {
            name: "abs",
            arity: Arity::Exact(1),
            func: abs,
            doc: "Distance from zero.",
        },
        Builtin {
            name: "sqrt",
            arity: Arity::Exact(1),
            func: sqrt,
            doc: "Square root.",
        },
        Builtin {
            name: "sin",
            arity: Arity::Exact(1),
            func: sin,
            doc: "Sine (radians).",
        },
        Builtin {
            name: "cos",
            arity: Arity::Exact(1),
            func: cos,
            doc: "Cosine (radians).",
        },
        Builtin {
            name: "tan",
            arity: Arity::Exact(1),
            func: tan,
            doc: "Tangent (radians).",
        },
        Builtin {
            name: "log2",
            arity: Arity::Exact(1),
            func: log2,
            doc: "Logarithm base 2.",
        },
        Builtin {
            name: "log10",
            arity: Arity::Exact(1),
            func: log10,
            doc: "Logarithm base 10.",
        },
        Builtin {
            name: "ln",
            arity: Arity::Exact(1),
            func: ln,
            doc: "Natural logarithm.",
        },
        Builtin {
            name: "exp",
            arity: Arity::Exact(1),
            func: exp,
            doc: "e raised to the power.",
        },
        Builtin {
            name: "pow",
            arity: Arity::Exact(2),
            func: pow,
            doc: "`base` raised to `exponent`.",
        },
        Builtin {
            name: "modulo",
            arity: Arity::Exact(2),
            func: modulo,
            doc: "Remainder that is never negative for a positive divisor, unlike `%`.",
        },
        Builtin {
            name: "min",
            arity: Arity::AtLeast(1),
            func: min,
            doc: "The smallest of the numbers, or of one furrball of numbers.",
        },
        Builtin {
            name: "max",
            arity: Arity::AtLeast(1),
            func: max,
            doc: "The largest of the numbers, or of one furrball of numbers.",
        },
        Builtin {
            name: "clamp",
            arity: Arity::Exact(3),
            func: clamp,
            doc: "Keep a number between `low` and `high`.",
        },
    ],
    constants: &[
        ("PI", std::f64::consts::PI),
        ("TAU", std::f64::consts::TAU),
        ("E", std::f64::consts::E),
        ("INFINITY", f64::INFINITY),
    ],
};

fn random(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let (lo, hi) = match args.len() {
        0 => (0.0, 1.0),
        1 => (0.0, number("random", &args, 0)?),
        _ => (number("random", &args, 0)?, number("random", &args, 1)?),
    };
    Ok(Value::Number(interp.rng().range_f64(lo, hi)))
}

fn pow(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    Ok(Value::Number(
        number("pow", &args, 0)?.powf(number("pow", &args, 1)?),
    ))
}

fn modulo(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let a = number("modulo", &args, 0)?;
    let b = number("modulo", &args, 1)?;
    if b == 0.0 {
        return Err(Error::runtime("can't take `modulo` by zero"));
    }
    Ok(Value::Number(a.rem_euclid(b)))
}

// Accepts either `f(1, 2, 3)` or `f([1, 2, 3])`.
fn numbers(fname: &str, args: &[Value]) -> Result<Vec<f64>> {
    if let [Value::Array(items)] = args {
        let items = items.borrow();
        return items
            .iter()
            .enumerate()
            .map(|(i, v)| match v {
                Value::Number(n) => Ok(*n),
                other => Err(Error::type_(format!(
                    "`{fname}` wants a furrball of numbers, but item {i} is {}",
                    other.type_name()
                ))),
            })
            .collect();
    }
    (0..args.len()).map(|i| number(fname, args, i)).collect()
}

fn min(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let ns = numbers("min", &args)?;
    Ok(ns
        .into_iter()
        .reduce(f64::min)
        .map_or(Value::Null, Value::Number))
}

fn max(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let ns = numbers("max", &args)?;
    Ok(ns
        .into_iter()
        .reduce(f64::max)
        .map_or(Value::Null, Value::Number))
}

fn clamp(_: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let x = number("clamp", &args, 0)?;
    let lo = number("clamp", &args, 1)?;
    let hi = number("clamp", &args, 2)?;
    if lo > hi {
        return Err(Error::runtime("`clamp` needs `low` to be at most `high`"));
    }
    Ok(Value::Number(x.clamp(lo, hi)))
}
