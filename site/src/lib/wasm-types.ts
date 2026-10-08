// Shape of the wasm-pack output in public/wasm, which the worker loads by URL at runtime.

export interface WasmInitOutput {
  memory: WebAssembly.Memory;
}

export interface WasmModule {
  /** Throws the rendered error text if the program fails. */
  run(source: string, onOutput: (line: string) => void): void;
  version(): string;
  keywords(): string[];
  default(input?: {
    module_or_path?: string | URL | Request | Response | BufferSource | WebAssembly.Module;
  }): Promise<WasmInitOutput>;
}

export const WASM_GLUE_URL = "/wasm/meowscript.js";
export const WASM_BINARY_URL = "/wasm/meowscript_bg.wasm";
