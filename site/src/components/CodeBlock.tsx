import { initialState, tokenizeLine } from "@/lib/meowscript-language";
import { TryItButton } from "./TryItButton";

interface Props {
  code: string;
  tryIt?: boolean;
  plain?: boolean;
}

export function CodeBlock({ code, tryIt = false, plain = false }: Props) {
  const text = code.replace(/\n$/, "");
  const lines = text.split("\n");
  const state = initialState();

  return (
    <div className="group relative my-4">
      <pre
        className={`overflow-x-auto rounded-lg border border-line bg-surface py-3 pl-4 text-[13.5px] leading-[1.6] ${tryIt ? "pr-20" : "pr-4"}`}
      >
        <code>
          {plain
            ? text
            : lines.map((line, i) => (
                <span key={i} className="block">
                  {tokenizeLine(line, state).map((tok, j) => (
                    <span key={j} className={tok.kind === "text" ? undefined : `tok-${tok.kind}`}>
                      {tok.text}
                    </span>
                  ))}
                  {line === "" ? " " : null}
                </span>
              ))}
        </code>
      </pre>
      {tryIt && <TryItButton code={text} />}
    </div>
  );
}
