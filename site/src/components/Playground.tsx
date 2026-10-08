"use client";

import { useCallback, useId, useState } from "react";
import type { Example } from "@/generated/examples";
import { takeHandedCode } from "@/lib/try-it";
import { useMeowScript } from "@/lib/use-meowscript";
import { Editor } from "./Editor";
import { Output } from "./Output";
import { Paw } from "./Paw";

interface Props {
  examples: Example[];
  initialExampleId?: string;
}

// Client-only (see PlaygroundLoader), so sessionStorage can be read while picking the first program.
export function Playground({ examples, initialExampleId = "yarn" }: Props) {
  const fallback = examples.find((e) => e.id === initialExampleId) ?? examples[0];
  const [state, setState] = useState(() => {
    const handed = takeHandedCode();
    return handed !== null ? { code: handed, selected: "" } : { code: fallback.source, selected: fallback.id };
  });
  const meow = useMeowScript();
  const selectId = useId();

  const { code, selected } = state;
  const setCode = useCallback((code: string) => setState((s) => ({ ...s, code })), []);
  const run = useCallback(() => meow.run(code), [meow, code]);

  const pickExample = (id: string) => {
    const example = examples.find((e) => e.id === id);
    if (!example) return;
    setState({ code: example.source, selected: id });
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
            {selected === "" && <option value="">from the docs</option>}
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
      {current?.description && <p className="pt-3 text-[13px] text-muted">{current.description}</p>}
    </section>
  );
}
