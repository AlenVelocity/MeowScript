use clap::{Parser, Subcommand};
use meowscript::host::NativeHost;
use meowscript::{Interpreter, Value};
use std::path::PathBuf;
use std::process::ExitCode;

// The evaluator recurses on the Rust stack, so run it on a thread with room to spare.
const STACK_SIZE: usize = 256 * 1024 * 1024;

#[derive(Parser)]
#[command(
    name = "meowscript",
    version,
    about = "The purrfect programming language 🐾"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// A .meow file to run (same as `meowscript run FILE`).
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Run a .meow file.
    Run { file: PathBuf },
    /// Evaluate a snippet of MeowScript and print its value.
    Eval { code: String },
    /// Start the interactive prompt (the default with no arguments).
    Repl,
    /// List the built-in `nya:` pawckages and what they contain.
    Packages {
        /// Print as JSON, for tooling such as the docs site.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let command = match (cli.command, cli.file) {
        (Some(c), _) => c,
        (None, Some(file)) => Command::Run { file },
        (None, None) => Command::Repl,
    };

    let worker = std::thread::Builder::new()
        .name("meowscript".into())
        .stack_size(STACK_SIZE)
        .spawn(move || match command {
            Command::Run { file } => run_file(&file),
            Command::Eval { code } => eval(&code),
            Command::Repl => repl::start(),
            Command::Packages { json } => {
                if json {
                    packages_json();
                } else {
                    packages();
                }
                ExitCode::SUCCESS
            }
        })
        .expect("spawn interpreter thread");

    worker.join().unwrap_or(ExitCode::FAILURE)
}

fn run_file(path: &PathBuf) -> ExitCode {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Scratched! could not read `{}`: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let mut interp = Interpreter::new(NativeHost);
    interp.set_base_dir(path.parent().map(|p| p.to_string_lossy().into_owned()));
    match interp.run(&source) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.render(&source, Some(&path.display().to_string())));
            ExitCode::FAILURE
        }
    }
}

fn eval(code: &str) -> ExitCode {
    let mut interp = Interpreter::new(NativeHost);
    match interp.run(code) {
        Ok(Value::Null) => ExitCode::SUCCESS,
        Ok(value) => {
            println!("{}", value.repr());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e.render(code, None));
            ExitCode::FAILURE
        }
    }
}

fn packages() {
    println!("Always available:");
    for b in meowscript::stdlib::PRELUDE {
        println!("  {:<14} {}", b.name, b.doc);
    }
    for pkg in meowscript::stdlib::PACKAGES {
        println!("\npawckage \"{}\";  {}", pkg.name, pkg.doc);
        for b in pkg.builtins {
            println!("  {:<14} {}", b.name, b.doc);
        }
        for (name, _) in pkg.constants {
            println!("  {:<14} constant", name);
        }
    }
}

// Hand-rolled so the binary doesn't need a JSON crate; the shape is small and stable.
fn packages_json() {
    fn quote(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }
    fn builtin(b: &meowscript::value::Builtin) -> String {
        format!(
            "{{\"name\":{},\"arity\":{},\"doc\":{}}}",
            quote(b.name),
            quote(&b.arity.describe()),
            quote(b.doc)
        )
    }

    let prelude: Vec<String> = meowscript::stdlib::PRELUDE.iter().map(builtin).collect();
    let packages: Vec<String> = meowscript::stdlib::PACKAGES
        .iter()
        .map(|pkg| {
            let functions: Vec<String> = pkg.builtins.iter().map(builtin).collect();
            let constants: Vec<String> = pkg
                .constants
                .iter()
                .map(|(name, value)| {
                    // JSON has no infinity.
                    let value = if value.is_finite() {
                        value.to_string()
                    } else {
                        quote(&meowscript::value::format_number(*value))
                    };
                    format!("{{\"name\":{},\"value\":{value}}}", quote(name))
                })
                .collect();
            format!(
                "{{\"name\":{},\"doc\":{},\"functions\":[{}],\"constants\":[{}]}}",
                quote(pkg.name),
                quote(pkg.doc),
                functions.join(","),
                constants.join(",")
            )
        })
        .collect();
    println!(
        "{{\"version\":{},\"prelude\":[{}],\"packages\":[{}]}}",
        quote(meowscript::VERSION),
        prelude.join(","),
        packages.join(",")
    );
}

// Unfinished input (an open `{`, say) carries over to the next line.
mod repl {
    use super::*;
    use std::io::{BufRead, Write};

    pub fn start() -> ExitCode {
        println!(
            "MeowScript {}. Type a line to run it. An empty line clears unfinished input; `scram` leaves.",
            meowscript::VERSION
        );
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut interp = Interpreter::new(NativeHost);
        let mut pending = String::new();

        loop {
            let prompt = if pending.is_empty() { ">> " } else { ".. " };
            let _ = write!(stdout, "{prompt}");
            let _ = stdout.flush();

            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) => {
                    println!("\nBye! 🐾");
                    return ExitCode::SUCCESS;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("could not read input: {e}");
                    return ExitCode::FAILURE;
                }
            }
            let line = line.trim_end_matches(['\n', '\r']);

            if line.trim().is_empty() {
                pending.clear();
                continue;
            }
            if pending.is_empty() && line.trim() == "scram" {
                println!("Bye! 🐾");
                return ExitCode::SUCCESS;
            }
            if !pending.is_empty() {
                pending.push('\n');
            }
            pending.push_str(line);

            match interp.run(&pending) {
                Ok(Value::Null) => {}
                Ok(value) => println!("{}", value.repr()),
                Err(e) if e.incomplete => continue,
                Err(e) => eprintln!("{}", e.render(&pending, None)),
            }
            pending.clear();
        }
    }
}
