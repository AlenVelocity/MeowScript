// wasm-pack -> public/wasm. Removes the .gitignore wasm-pack drops there, since the output is
// committed.

import { spawnSync } from "node:child_process";
import { rmSync } from "node:fs";
import { fileURLToPath } from "node:url";

const repoRoot = fileURLToPath(new URL("../../", import.meta.url));
const outDir = fileURLToPath(new URL("../public/wasm/", import.meta.url));

const result = spawnSync(
  "wasm-pack",
  [
    "build",
    "crates/meowscript-wasm",
    "--release",
    "--target",
    "web",
    "--out-dir",
    outDir,
    "--out-name",
    "meowscript",
    "--no-pack",
    "--no-typescript",
  ],
  { cwd: repoRoot, stdio: "inherit" },
);

if (result.status !== 0) {
  console.error("build-wasm: wasm-pack failed. Is it installed? https://github.com/wasm-bindgen/wasm-pack");
  process.exit(result.status ?? 1);
}

rmSync(new URL(".gitignore", `file://${outDir}`), { force: true });
console.log("build-wasm: wrote public/wasm/meowscript.js and meowscript_bg.wasm");
