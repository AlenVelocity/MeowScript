# Contributing to MeowScript

How the project fits together, and how to change it without breaking anything.

## Setup

You need a Rust toolchain (stable, 1.88 or newer) and, for the site, Node 22 with pnpm. On Windows the GNU toolchain works without Visual Studio: `rustup-init --default-host x86_64-pc-windows-gnu`.

```bash
git clone https://github.com/AlenVelocity/MeowScript
cd MeowScript
cargo test --workspace
```

## Where things live

| Path | What |
| --- | --- |
| `crates/meowscript/src/lexer.rs` | text to tokens, with byte spans |
| `crates/meowscript/src/parser.rs` | tokens to AST (`ast.rs`); Pratt parser for operators |
| `crates/meowscript/src/interpreter.rs` | evaluates the AST; control flow travels as `Interrupt` |
| `crates/meowscript/src/value.rs` | runtime values and how they print |
| `crates/meowscript/src/stdlib/` | the prelude and the `nya:` pawckages, one file each |
| `crates/meowscript/src/host.rs` | the `Host` trait: printing, files, sleeping, randomness |
| `crates/meowscript-cli` | the `meowscript` binary |
| `crates/meowscript-wasm` | the browser entry point (`run`, `version`, `keywords`) |
| `examples/` | programs run by the test suite; a `.out` file is a snapshot |
| `site/` | the Next.js playground and docs |

## Making changes

**Keywords and syntax.** Add the token in `token.rs` (both `keyword()` and `KEYWORDS`), lex it, parse it, evaluate it, and add a test next to the others in each file. Then add it to the cheat sheet in `site/src/components/CheatSheet.tsx`, the reference in `site/content/docs.md`, and the highlighter's keyword list in `site/src/lib/meowscript-language.ts`.

**Built-in functions.** Add a row to the right pawckage's table in `crates/meowscript/src/stdlib/`. The `arity` is checked before your function runs, so indexing `args` inside it is safe. The helpers in `stdlib/mod.rs` (`number`, `string`, `array`, `callable`) pull typed arguments with consistent error messages. Functions that take callbacks call `interp.call(f, args, None)`. The docs site generates its library reference from these tables, so the `doc` string is what people read.

**Examples.** Drop a `.meow` file in `examples/`. Run `UPDATE_SNAPSHOTS=1 cargo test -p meowscript --test examples` to write its `.out` file, then read the `.out` to check it is what you meant. Programs that sleep or read input get a `// test: skip` comment instead. To show it in the playground, add its name to `ORDER` in `site/scripts/sync-examples.mjs`.

**Error messages.** They start with a cat exclamation chosen by `ErrorKind`, then say what went wrong and, where possible, what to do instead.

**Comments.** Only where the code can't say why. A comment that repeats the function name or the next line gets removed in review.

## Before you push

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p meowscript-wasm --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace
```

If you touched the interpreter or the standard library, also refresh what the site ships:

```bash
cd site
pnpm build:wasm     # needs wasm-pack
pnpm sync           # regenerates src/generated/*
pnpm typecheck && pnpm lint && pnpm build
```

and commit the results in `site/public/wasm` and `site/src/generated`. CI checks that the generated tables match the code.

## Pull requests

Say what changed and why. If it changes the language, show a before and after.
