pub mod ast;
pub mod evaluation;
pub mod lexer;
pub mod parser;
pub mod std_library;
pub mod repl;
use std::{rc::Rc, cell::RefCell};

use evaluation::{object::*, store::*, *};
use lexer::Lexer;
use parser::Parser;

pub fn interpret(content: &str) {
    let store = Store::new();
    let mut evaluator = Eval {
        store: Rc::new(RefCell::new(store)),
    };
    let lexer = Lexer::new(content.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        for e in parser.errors.iter() {
            println!("\t{}", e);
        }
        return;
    }
    let res = evaluator.eval(program);

    if let Some(o) = res {
        match o {
            Object::Null => (),
            _ => println!("{}", o),
        }
    }
}