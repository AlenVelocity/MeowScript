"use client";

import { useEffect, useRef } from "react";
import type { OutputLine, Status } from "@/lib/use-meowscript";

interface Props {
  lines: OutputLine[];
  status: Status;
  version: string | null;
  lastRunMillis: number | null;
  fatal: string | null;
}

const MEOW = "Meow!";
const PAW_ID = "output-paw";

// Defined once; every meowed line references it, so thousands of lines stay cheap.
function PawSymbol() {
  return (
    <svg className="hidden" aria-hidden="true">
      <symbol id={PAW_ID} viewBox="0 0 24 24">
        <ellipse cx="5.6" cy="10.2" rx="2.1" ry="2.7" />
        <ellipse cx="18.4" cy="10.2" rx="2.1" ry="2.7" />
        <ellipse cx="9.4" cy="6" rx="2.1" ry="2.8" />
        <ellipse cx="14.6" cy="6" rx="2.1" ry="2.8" />
        <path d="M12 10.8c3.9 0 6.9 3 6.9 6 0 2.3-1.6 3.7-3.6 3.7-1.3 0-2.2-.6-3.3-.6s-2 .6-3.3.6c-2 0-3.6-1.4-3.6-3.7 0-3 3-6 6.9-6z" />
      </symbol>
    </svg>
  );
}

function PawMark({ className }: { className: string }) {
  return (
    <svg className={className} aria-hidden="true">
      <use href={`#${PAW_ID}`} />
    </svg>
  );
}

function ErrorLine({ text }: { text: string }) {
  const newline = text.indexOf("\n");
  const first = newline === -1 ? text : text.slice(0, newline);
  const rest = newline === -1 ? "" : text.slice(newline);
  const space = first.indexOf(" ");
  const exclamation = space === -1 ? first : first.slice(0, space);
  const message = space === -1 ? "" : first.slice(space);
  return (
    <pre className="whitespace-pre-wrap break-words text-danger" role="alert">
      <strong className="font-display text-[1.15em] font-bold">{exclamation}</strong>
      {message}
      {rest}
    </pre>
  );
}

function Line({ line }: { line: OutputLine }) {
  if (line.kind === "error") return <ErrorLine text={line.text} />;
  if (line.kind === "note") return <div className="text-muted">{line.text}</div>;
  if (line.text === MEOW || line.text.startsWith(`${MEOW} `)) {
    return (
      <div className="whitespace-pre-wrap break-words">
        <span className="inline-flex items-center gap-1 text-accent">
          <PawMark className="size-[0.85em] translate-y-px fill-current" />
          {MEOW}
        </span>
        {line.text.slice(MEOW.length)}
      </div>
    );
  }
  return <div className="whitespace-pre-wrap break-words">{line.text || " "}</div>;
}

function statusText({ status, lines, lastRunMillis, fatal }: Props): string {
  if (fatal) return "The interpreter could not start";
  if (status === "loading") return "Waking up the interpreter…";
  if (status === "running") return "Playing…";
  if (lines.at(-1)?.kind === "error") return "Hissed";
  if (lastRunMillis !== null) return `Purring · finished in ${lastRunMillis} ms`;
  return "Ready";
}

export function Output(props: Props) {
  const { lines, status, version, fatal } = props;
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = scrollRef.current;
    if (el) el.scrollTop = el.scrollHeight;
  }, [lines]);

  return (
    <div className="flex h-full min-h-0 flex-col">
      <PawSymbol />
      <div
        ref={scrollRef}
        className="min-h-0 flex-1 overflow-auto px-4 py-3 text-[14px] leading-[1.6]"
        aria-live="polite"
        aria-label="Program output"
      >
        {fatal ? (
          <div className="flex h-full flex-col items-center justify-center text-center" role="alert">
            <PawMark className="size-12 fill-danger/60" />
            <p className="mt-3 text-danger">The playground could not load the interpreter.</p>
            <p className="mt-2 max-w-[40ch] text-muted">{fatal}</p>
            <p className="mt-2 text-muted">
              Try a hard refresh. If it keeps happening,{" "}
              <a className="underline hover:text-accent" href="https://github.com/AlenVelocity/MeowScript/issues">
                open an issue
              </a>
              .
            </p>
          </div>
        ) : lines.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center text-center">
            <PawMark className="size-12 fill-text/15" />
            <p className="mt-3 max-w-[32ch] text-muted">
              {status === "loading" ? "Waking up…" : "Press Run, or Ctrl+Enter in the editor, to hear from the cat."}
            </p>
          </div>
        ) : (
          lines.map((line, i) => <Line key={i} line={line} />)
        )}
      </div>
      <div className="flex items-center justify-between gap-3 border-t border-line px-4 py-2 text-[13px] text-muted">
        <span className="truncate">{statusText(props)}</span>
        <span className="shrink-0">{version ? `MeowScript ${version} · WebAssembly` : ""}</span>
      </div>
    </div>
  );
}
