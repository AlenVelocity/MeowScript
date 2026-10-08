// Runs the interpreter off the main thread, so an endless loop never freezes the page and Stop
// can simply terminate the worker. Output is flushed on a time budget: a tight loop sends about
// 30 messages a second however fast it prints, while nap()-paced output still shows up as it
// happens.

import type { WorkerRequest, WorkerResponse } from "./protocol";
import { WASM_BINARY_URL, WASM_GLUE_URL, type WasmModule } from "./wasm-types";

const ctx = self as unknown as {
  postMessage(message: WorkerResponse): void;
  onmessage: ((event: MessageEvent<WorkerRequest>) => void) | null;
};

const FLUSH_EVERY_MS = 30;
const FLUSH_EVERY_LINES = 20_000;

let wasm: WasmModule | null = null;

async function load(): Promise<void> {
  // The glue is served from /public, not bundled; a variable keeps the bundler from resolving it.
  const glueUrl = WASM_GLUE_URL;
  const mod = (await import(/* webpackIgnore: true */ /* turbopackIgnore: true */ glueUrl)) as WasmModule;
  await mod.default({ module_or_path: WASM_BINARY_URL });
  wasm = mod;
  ctx.postMessage({ type: "ready", version: mod.version() });
}

load().catch((error: unknown) => {
  ctx.postMessage({ type: "fatal", message: error instanceof Error ? error.message : String(error) });
});

ctx.onmessage = (event) => {
  const request = event.data;
  if (request.type !== "run") return;
  if (!wasm) {
    ctx.postMessage({ type: "error", id: request.id, message: "The interpreter is still loading." });
    return;
  }

  let batch: string[] = [];
  let lastFlush = Date.now();
  const flush = () => {
    if (batch.length === 0) return;
    ctx.postMessage({ type: "lines", id: request.id, lines: batch });
    batch = [];
    lastFlush = Date.now();
  };
  const onOutput = (line: string) => {
    batch.push(line);
    if (batch.length >= FLUSH_EVERY_LINES || Date.now() - lastFlush >= FLUSH_EVERY_MS) flush();
  };

  const started = Date.now();
  try {
    wasm.run(request.source, onOutput);
    flush();
    ctx.postMessage({ type: "done", id: request.id, millis: Date.now() - started });
  } catch (error: unknown) {
    flush();
    const message = typeof error === "string" ? error : error instanceof Error ? error.message : String(error);
    ctx.postMessage({ type: "error", id: request.id, message });
  }
};
