# MeowScript

A small scripting language where the keywords are cat puns. `purrhaps` is if, `pawction` is function, `tail` is return, and every program announces itself with a `meow`. The interpreter is written in Rust and also runs in the browser as WebAssembly.

Try it at [meowscript.vercel.app](https://meowscript.vercel.app).

```meow
pawckage "nya:furrball";

scratch cats = ["Tom", "Luna", "Salem"];

pawction introduce(cat) {
    purrhaps cat == "Salem" {
        tail cat + " (a witch's familiar)";
    }
    tail cat;
}

fur cat ~ cats {
    meow("This is", introduce(cat));
}
```

```
Meow! This is Tom
Meow! This is Luna
Meow! This is Salem (a witch's familiar)
```

## The language in one table

| Cat | Human |
| --- | --- |
| `scratch x = 1;` | declare |
| `amew x = 2;` | assign (also `amew cat's age = 4;` and `amew arr[0] = 1;`) |
| `pawction f(a, b) { tail a + b; }` | function and return |
| `purrhaps … { } meowtually { }` | if / else (an expression) |
| `furrever { }`, `furrever cond { }` | loop forever, loop while |
| `fur item ~ things { }` | for each (furrballs, whiskers, object keys, or `0..n`) |
| `hiss;` / `continue;` | break / continue |
| `curious { } caught err { }` | try / catch (the name after `caught` is optional) |
| `yowl value;` | throw any value |
| `purrfect`, `clawful`, `mew` | true, false, null |
| `furreal x` | typeof: `"number"`, `"whiskers"`, `"furrball"`, … |
| `cat's name`, `cat["name"]` | property access |
| `x ~ things` | membership |
| `pawckage "nya:furrball";` | import a built-in library, or `"./file.meow"` |

Strings are *whiskers*, arrays are *furrballs*. Operators follow Python's precedence (`6 & 3 == 2` is `(6 & 3) == 2`), only `mew` and `clawful` are falsy, and dividing by zero is an error. The reference is in the [docs](https://meowscript.vercel.app/docs), or at [`site/content/docs.md`](site/content/docs.md).

Errors point at the problem:

```
Meow-sterious! name error: `cuont` hasn't been `scratch`ed yet; did you mean `count`?
  --> toys.meow:3:6
   |
 3 | meow(cuont);
   |      ^^^^^
```

## Install

Prebuilt binaries for Windows x64, macOS (Apple silicon and Intel), and Linux (x64 and arm64) are on the [releases page](https://github.com/AlenVelocity/MeowScript/releases/latest), with a `SHA256SUMS.txt`. The [download page](https://meowscript.vercel.app/download) picks the right file for each platform and explains the unsigned-binary warnings on Windows and macOS. Unpack the archive and put `meowscript` on your PATH.

With a Rust toolchain (1.88 or newer):

```bash
cargo install --git https://github.com/AlenVelocity/MeowScript --tag v0.1.0 meowscript-cli
```

```
meowscript                    # the REPL
meowscript hello.meow         # run a file
meowscript eval "1 + 2"       # evaluate a snippet
meowscript packages           # what the nya: pawckages contain
```

Changes between versions are in [CHANGELOG.md](CHANGELOG.md).

## Layout

```
crates/meowscript        the language: lexer, parser, interpreter, standard library
crates/meowscript-cli    the `meowscript` binary (runner and REPL)
crates/meowscript-wasm   the wasm-bindgen entry point for the browser
examples/                programs; those with a .out file are snapshot-tested
site/                    the playground and docs (Next.js)
```

The interpreter reaches the outside world only through a `Host` trait. The CLI gives it a terminal, the tests give it a buffer, and the browser build gives it a callback that posts lines to the page.

## Development

```bash
cargo test --workspace                       # unit tests, doctests, and every example
cargo run -p meowscript-cli -- examples/yarn.meow
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

For the site, see [site/README.md](site/README.md). After changing the Rust code, run `pnpm build:wasm && pnpm sync` there to refresh the WebAssembly and the generated tables.

The first version of this interpreter is in the git history before the rewrite. This one replaces it with a byte-indexed lexer that records spans, a Pratt parser that reports where it got stuck, an evaluator that returns `Result` instead of error values, and a browser build on stable Rust with wasm-bindgen instead of Emscripten.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Cat puns welcome.
