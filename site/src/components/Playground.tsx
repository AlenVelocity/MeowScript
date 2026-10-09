"use client";

import { useCallback, useEffect, useId, useState } from "react";
import type { Example } from "@/generated/examples";
import { decodeProgram, encodeProgram, hasSharedProgram } from "@/lib/share";
import { takeHandedCode } from "@/lib/try-it";
import { useMeowScript } from "@/lib/use-meowscript";
import { Editor } from "./Editor";
import { Output } from "./Output";
import { Paw } from "./Paw";

interface Props {
  examples: Example[];
  initialExampleId?: string;
}

// Picker values for programs that aren't one of the examples.
const FROM_DOCS = "";
const FROM_LINK = ":link";

const SHARE_NOTE_MILLIS = 2000;

// Client-only (see PlaygroundLoader), so sessionStorage and the URL can be read while picking the
// first program.
export function Playground({ examples, initialExampleId = "yarn" }: Props) {
  const fallback = examples.find((e) => e.id === initialExampleId) ?? examples[0];
  const [state, setState] = useState(() => {
    const handed = takeHandedCode();
    // The editor stays empty for the moment it takes the effect below to decode the link.
    if (hasSharedProgram(window.location.hash)) return { code: "", selected: FROM_LINK };
    return handed !== null ? { code: handed, selected: FROM_DOCS } : { code: fallback.source, selected: fallback.id };
  });
  const [brokenLink, setBrokenLink] = useState(false);
  // An object, so a second click restarts the timer even when the text is the same.
  const [shareNote, setShareNote] = useState<{ text: string } | null>(null);
  const meow = useMeowScript();
  const clearOutput = meow.clear;
  const selectId = useId();

  const { code, selected } = state;
  const setCode = useCallback((code: string) => setState((s) => ({ ...s, code })), []);
  const run = useCallback(() => meow.run(code), [meow, code]);

  // Runs again on hashchange, which is what happens when a second link is pasted into the same tab.
  useEffect(() => {
    let live = true;
    const load = async () => {
      const hash = window.location.hash;
      if (!hasSharedProgram(hash)) return;
      const linked = await decodeProgram(hash);
      if (!live || hash !== window.location.hash) return;
      setState(linked === null ? { code: fallback.source, selected: fallback.id } : { code: linked, selected: FROM_LINK });
      setBrokenLink(linked === null);
      clearOutput();
    };
    load();
    window.addEventListener("hashchange", load);
    return () => {
      live = false;
      window.removeEventListener("hashchange", load);
    };
  }, [fallback, clearOutput]);

  useEffect(() => {
    if (shareNote === null) return;
    const timer = window.setTimeout(() => setShareNote(null), SHARE_NOTE_MILLIS);
    return () => window.clearTimeout(timer);
  }, [shareNote]);

  const share = async () => {
    const url = new URL(window.location.href);
    url.hash = await encodeProgram(code);
    window.history.replaceState(null, "", url);
    try {
      await navigator.clipboard.writeText(url.href);
      setShareNote({ text: "Link copied" });
    } catch {
      setShareNote({ text: "Link is in the address bar" });
    }
  };

  const pickExample = (id: string) => {
    const example = examples.find((e) => e.id === id);
    if (!example) return;
    setState({ code: example.source, selected: id });
    setBrokenLink(false);
    meow.clear();
  };

  const busy = meow.status === "running";
  const canRun = meow.status === "ready";
  const current = examples.find((e) => e.id === selected);

  return (
    <section id="playground" aria-label="Playground" className="scroll-mt-20">
      <div className="flex flex-wrap items-center justify-between gap-3 pb-3">
        <div className="flex items-center gap-2">
          <label htmlFor={selectId} className="text-[13px] text-muted">
            Example
          </label>
          <select
            id={selectId}
            value={selected}
            onChange={(e) => pickExample(e.target.value)}
            className="h-9 rounded-md border border-line bg-surface px-2 text-[14px] text-text hover:border-muted focus-visible:border-accent"
          >
            {selected === FROM_DOCS && <option value={FROM_DOCS}>from the docs</option>}
            {selected === FROM_LINK && <option value={FROM_LINK}>from a link</option>}
            {examples.map((e) => (
              <option key={e.id} value={e.id}>
                {e.file}
              </option>
            ))}
          </select>
        </div>
        <div className="ml-auto flex items-center gap-2">
          <button
            type="button"
            onClick={share}
            className="h-9 rounded-md px-3 text-[14px] text-muted hover:text-text"
          >
            <span aria-live="polite">{shareNote?.text ?? "Share"}</span>
          </button>
          <button
            type="button"
            onClick={meow.clear}
            disabled={meow.lines.length === 0}
            className="h-9 rounded-md px-3 text-[14px] text-muted hover:text-text disabled:opacity-40 disabled:hover:text-muted"
          >
            Clear
          </button>
          {busy ? (
            <button
              type="button"
              onClick={meow.stop}
              className="h-9 rounded-md border border-danger px-4 text-[14px] font-medium text-danger hover:bg-danger/10 active:bg-danger/20"
            >
              Stop
            </button>
          ) : (
            <button
              type="button"
              onClick={run}
              disabled={!canRun}
              aria-keyshortcuts="Control+Enter"
              className="group inline-flex h-9 items-center gap-1.5 rounded-md bg-accent pr-4 pl-3 text-[14px] font-semibold text-bg hover:bg-accent-strong active:translate-y-px disabled:cursor-wait disabled:opacity-50 disabled:hover:bg-accent"
            >
              <Paw className="size-4 transition-transform duration-150 group-hover:-rotate-12 motion-reduce:transition-none" />
              {meow.status === "loading" ? "Loading…" : "Run"}
            </button>
          )}
        </div>
      </div>

      <div className="grid gap-3 lg:grid-cols-[3fr_2fr] lg:gap-4">
        <div className="h-[22rem] overflow-hidden rounded-lg border border-line bg-surface sm:h-[26rem] lg:h-[30rem]">
          <Editor value={code} onChange={setCode} onRun={run} />
        </div>
        <div className="h-[18rem] overflow-hidden rounded-lg border border-line bg-surface sm:h-[20rem] lg:h-[30rem]">
          <Output
            lines={meow.lines}
            status={meow.status}
            version={meow.version}
            lastRunMillis={meow.lastRunMillis}
            fatal={meow.fatal}
          />
        </div>
      </div>
      {brokenLink ? (
        <p role="status" className="pt-3 text-[13px] text-muted">
          The program in this link couldn&apos;t be read; the link may have been cut short. This is {fallback.file}{" "}
          instead.
        </p>
      ) : (
        current?.description && <p className="pt-3 text-[13px] text-muted">{current.description}</p>
      )}
    </section>
  );
}
