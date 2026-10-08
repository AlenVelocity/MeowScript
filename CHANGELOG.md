# Changelog

Each release has a section here. The release workflow uses the matching section as the GitHub release notes.

## 0.1.0 (2026-10-08)

The first release of the rewritten interpreter. The previous code, kept in git history, was a learning project that had stopped compiling; this is a new implementation with the same keywords.

### Language

- `scratch`, `amew`, `pawction`, `purrhaps`/`meowtually`, `tail`, `furrever`, `hiss`, `continue`, `furreal`, `pawckage`, `purrfect`, `clawful`, and the `'s` possessive, as before.
- New: `mew` (null), `&&` and `||`, `fur item ~ things` loops, `furrever` with a condition, named `pawction` declarations, `amew` into object keys and furrball elements, string escapes, block comments, and `purrhaps` as an expression.
- Furrballs and objects are reference types. Blocks, loops, and calls each open a scope.
- Operators use Python's precedence. Only `mew` and `clawful` are falsy. Dividing by zero is an error.
- Errors report a line and column with a caret, and unknown names get a "did you mean" hint.

### Standard library

- Always available: `meow`, `purr`, `log`, `length`.
- `nya:clawtility`, `nya:furrball`, `nya:whiskers`, `nya:catculator`, `nya:rundamn`, and `nya:scratchpad`, about 70 functions in all, every one with an arity check.
- `pawckage "./file.meow"` imports another file, resolved relative to the importing file.

### Command line

- `meowscript FILE`, `meowscript eval CODE`, `meowscript packages`, and a REPL that keeps reading while a block is open.
- Prebuilt binaries for Windows x64, macOS (Apple silicon and Intel), and Linux (x64 and arm64). They are not code-signed, so Windows and macOS show a warning on first run.

### Playground

- The interpreter runs in the browser as WebAssembly inside a Web Worker, with a Stop button, streamed output, and a docs page whose library reference comes from the interpreter itself.

### Known limits

- `kibble` (input) and file pawckages don't work in the browser.
- Reference cycles between closures and their scopes are never freed; long REPL sessions grow.
