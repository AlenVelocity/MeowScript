use super::{Package, string};
use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::value::{Arity, Builtin, Value};

pub static PACKAGE: Package = Package {
    name: "nya:scratchpad",
    doc: "Reading and writing files. Not available in the browser.",
    builtins: &[
        Builtin {
            name: "read_file",
            arity: Arity::Exact(1),
            func: read_file,
            doc: "The whole contents of a file as whiskers.",
        },
        Builtin {
            name: "write_file",
            arity: Arity::Exact(2),
            func: write_file,
            doc: "Write whiskers to a file, replacing what was there.",
        },
        Builtin {
            name: "append_file",
            arity: Arity::Exact(2),
            func: append_file,
            doc: "Add whiskers to the end of a file.",
        },
        Builtin {
            name: "exists",
            arity: Arity::Exact(1),
            func: exists,
            doc: "Whether a file or directory is there.",
        },
    ],
    constants: &[],
};

fn read_file(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let path = string("read_file", &args, 0)?;
    interp.host().read_file(&path).map(Value::from)
}

fn write_file(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let path = string("write_file", &args, 0)?;
    let contents = args[1].to_string();
    interp.host().write_file(&path, &contents, false)?;
    Ok(Value::Null)
}

fn append_file(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let path = string("append_file", &args, 0)?;
    let contents = args[1].to_string();
    interp.host().write_file(&path, &contents, true)?;
    Ok(Value::Null)
}

fn exists(interp: &mut Interpreter, args: Vec<Value>) -> Result<Value> {
    let path = string("exists", &args, 0)?;
    Ok(Value::Bool(interp.host().file_exists(&path)))
}
