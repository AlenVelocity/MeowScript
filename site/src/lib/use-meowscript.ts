"use client";

import { useCallback, useEffect, useReducer, useRef } from "react";
import type { WorkerRequest, WorkerResponse } from "./protocol";

export type OutputLine = { kind: "out" | "error" | "note"; text: string };

export type Status = "loading" | "ready" | "running" | "failed";

export interface MeowState {
  status: Status;
  version: string | null;
  lines: OutputLine[];
  lastRunMillis: number | null;
  fatal: string | null;
}

const MAX_LINES = 3000;

type Action =
  | { type: "ready"; version: string }
  | { type: "fatal"; message: string }
  | { type: "start" }
  | { type: "lines"; lines: string[] }
  | { type: "done"; millis: number }
  | { type: "error"; message: string }
  | { type: "stopped" }
  | { type: "clear" };

const initial: MeowState = { status: "loading", version: null, lines: [], lastRunMillis: null, fatal: null };

function append(lines: OutputLine[], extra: OutputLine[]): OutputLine[] {
  const next = lines.concat(extra);
  if (next.length <= MAX_LINES) return next;
  const kept = next.slice(next.length - MAX_LINES);
  return [{ kind: "note", text: `… earlier lines trimmed; only the last ${MAX_LINES} are kept` }, ...kept];
}

function reducer(state: MeowState, action: Action): MeowState {
  switch (action.type) {
    case "ready":
      return { ...state, status: "ready", version: action.version, fatal: null };
    case "fatal":
      return { ...state, status: "failed", fatal: action.message };
    case "start":
      return { ...state, status: "running", lines: [], lastRunMillis: null };
    case "lines":
      return { ...state, lines: append(state.lines, action.lines.map((text) => ({ kind: "out", text }))) };
    case "done":
      return { ...state, status: "ready", lastRunMillis: action.millis };
    case "error":
      return { ...state, status: "ready", lines: append(state.lines, [{ kind: "error", text: action.message }]) };
    case "stopped":
      return { ...state, status: "loading", lines: append(state.lines, [{ kind: "note", text: "Stopped." }]) };
    case "clear":
      return { ...state, lines: [], lastRunMillis: null };
  }
}

export function useMeowScript() {
  const [state, dispatch] = useReducer(reducer, initial);
  const workerRef = useRef<Worker | null>(null);
  const runIdRef = useRef(0);
  // Incoming lines are gathered here and handed to React once per frame, so a fast program
  // can't starve the page.
  const pendingRef = useRef<string[]>([]);
  const frameRef = useRef<number | null>(null);

  const flushPending = useCallback(() => {
    frameRef.current = null;
    if (pendingRef.current.length === 0) return;
    const lines = pendingRef.current;
    pendingRef.current = [];
    dispatch({ type: "lines", lines: lines.length > MAX_LINES ? lines.slice(-MAX_LINES) : lines });
  }, []);

  const discardPending = useCallback(() => {
    pendingRef.current = [];
    if (frameRef.current !== null) {
      cancelAnimationFrame(frameRef.current);
      frameRef.current = null;
    }
  }, []);

  const spawn = useCallback(() => {
    workerRef.current?.terminate();
    const worker = new Worker(new URL("./meow.worker.ts", import.meta.url), { type: "module" });
    worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
      const msg = event.data;
      switch (msg.type) {
        case "ready":
          dispatch({ type: "ready", version: msg.version });
          break;
        case "fatal":
          dispatch({ type: "fatal", message: msg.message });
          break;
        case "lines":
          if (msg.id !== runIdRef.current) break;
          pendingRef.current = pendingRef.current.concat(msg.lines);
          if (pendingRef.current.length > MAX_LINES) pendingRef.current = pendingRef.current.slice(-MAX_LINES);
          if (frameRef.current === null) frameRef.current = requestAnimationFrame(flushPending);
          break;
        case "done":
          if (msg.id !== runIdRef.current) break;
          flushPending();
          dispatch({ type: "done", millis: msg.millis });
          break;
        case "error":
          if (msg.id !== runIdRef.current) break;
          flushPending();
          dispatch({ type: "error", message: msg.message });
          break;
      }
    };
    worker.onerror = (event) => {
      dispatch({ type: "fatal", message: event.message || "The interpreter worker crashed." });
    };
    workerRef.current = worker;
  }, [flushPending]);

  useEffect(() => {
    spawn();
    return () => {
      discardPending();
      workerRef.current?.terminate();
    };
  }, [spawn, discardPending]);

  const run = useCallback(
    (source: string) => {
      const worker = workerRef.current;
      if (!worker || state.status !== "ready") return;
      runIdRef.current += 1;
      discardPending();
      dispatch({ type: "start" });
      const request: WorkerRequest = { type: "run", id: runIdRef.current, source };
      worker.postMessage(request);
    },
    [state.status, discardPending],
  );

  const stop = useCallback(() => {
    runIdRef.current += 1;
    flushPending();
    dispatch({ type: "stopped" });
    spawn();
  }, [spawn, flushPending]);

  const clear = useCallback(() => {
    discardPending();
    dispatch({ type: "clear" });
  }, [discardPending]);

  return { ...state, run, stop, clear };
}
