export type WorkerRequest = { type: "run"; id: number; source: string };

export type WorkerResponse =
  | { type: "ready"; version: string }
  | { type: "fatal"; message: string }
  | { type: "lines"; id: number; lines: string[] }
  | { type: "done"; id: number; millis: number }
  | { type: "error"; id: number; message: string };
