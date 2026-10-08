# MeowScript site

The playground and docs at [meowscript.vercel.app](https://meowscript.vercel.app). Next.js 16, React 19, Tailwind 4, CodeMirror 6. The interpreter runs in the browser as WebAssembly, inside a Web Worker so a runaway loop can be stopped.

```bash
pnpm install
pnpm dev          # http://localhost:3000
```

Three things are generated and committed, so the site builds anywhere without Rust:

| Command            | Produces                     | From                                         |
| ------------------ | ---------------------------- | -------------------------------------------- |
| `pnpm build:wasm`  | `public/wasm/*`              | `crates/meowscript-wasm` via wasm-pack       |
| `pnpm sync`        | `src/generated/examples.ts`  | `../examples/*.meow`                         |
| `pnpm sync`        | `src/generated/stdlib.ts`    | `meowscript packages --json` (needs cargo)   |

`pnpm build` runs `pnpm sync` first. When the sources are not around (on Vercel, say), the sync scripts leave the committed files alone. After changing the Rust code, run `pnpm build:wasm` and `pnpm sync` and commit the results.
