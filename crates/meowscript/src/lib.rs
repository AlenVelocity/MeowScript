//! MeowScript, a small cat-themed scripting language.
//!
//! ```
//! use meowscript::{BufferHost, Interpreter};
//!
//! let host = BufferHost::new();
//! let mut interp = Interpreter::new(host.clone());
//! interp.run(r#"
//!     scratch cat = { name: "Whiskers", lives: 9 };
//!     meow(cat's name, "has", cat's lives, "lives");
//! "#).unwrap();
//! assert_eq!(host.output(), "Meow! Whiskers has 9 lives");
//! ```

pub mod ast;
pub mod env;
pub mod error;
pub mod host;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod rng;
pub mod span;
pub mod stdlib;
pub mod token;
pub mod value;

pub use error::{Error, ErrorKind, Result};
pub use host::{BufferHost, Host};
pub use interpreter::Interpreter;
pub use value::Value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Runs a program; the error, if any, comes back rendered with its source excerpt.
pub fn run_with_host(
    source: &str,
    host: impl Host + 'static,
) -> std::result::Result<Value, String> {
    let mut interp = Interpreter::new(host);
    interp.run(source).map_err(|e| e.render(source, None))
}

/// Runs a program and returns what it printed, plus the rendered error if it failed.
pub fn capture(source: &str) -> (Vec<String>, Option<String>) {
    let host = BufferHost::new();
    let mut interp = Interpreter::new(host.clone());
    let result = interp.run(source);
    (host.take(), result.err().map(|e| e.render(source, None)))
}
