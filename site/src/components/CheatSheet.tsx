import { CodeBlock } from "./CodeBlock";

const rows: { cat: string; human: string; example: string }[] = [
  { cat: "scratch", human: "declare a variable", example: "scratch lives = 9;" },
  { cat: "amew", human: "change it", example: "amew lives = lives - 1;" },
  { cat: "pawction", human: "function", example: 'pawction greet(name) { meow("Hi", name); }' },
  { cat: "tail", human: "return", example: "tail lives * 2;" },
  { cat: "purrhaps / meowtually", human: "if / else", example: 'purrhaps hungry { meow("feed me") } meowtually { nap(10) }' },
  { cat: "furrever", human: "loop, with an optional condition", example: "furrever lives > 0 { amew lives = lives - 1; }" },
  { cat: "fur ~", human: "for each", example: "fur toy ~ toys { meow(toy); }" },
  { cat: "hiss / continue", human: "break / continue", example: "purrhaps tired { hiss; }" },
  { cat: "purrfect / clawful", human: "true / false", example: "scratch asleep = purrfect;" },
  { cat: "mew", human: "null", example: "scratch owner = mew;" },
  { cat: "furreal", human: "typeof", example: 'furreal "whiskers"  // "whiskers"' },
  { cat: "pawckage", human: "import", example: 'pawckage "nya:furrball";' },
  { cat: "'s", human: "property access", example: "cat's name" },
  { cat: "~", human: "is in", example: '"Tom" ~ cats' },
  { cat: "whiskers", human: "a string", example: '"meow"' },
  { cat: "furrball", human: "an array", example: '["yarn", "box", "sunbeam"]' },
];

export function CheatSheet() {
  return (
    <section id="cheat-sheet" aria-labelledby="cheat-sheet-title" className="scroll-mt-20">
      <h2 id="cheat-sheet-title" className="font-display text-[24px] font-bold sm:text-[28px]">
        Cat to human
      </h2>
      <p className="mt-2 max-w-[62ch] text-muted">
        Each keyword is a pun for one ordinary thing. This is the whole vocabulary.
      </p>
      <div className="mt-6 overflow-x-auto">
        <table className="w-full min-w-[560px] border-collapse text-[14px]">
          <thead>
            <tr className="text-left text-[13px] text-muted">
              <th scope="col" className="border-b border-line pb-2 pr-4 font-normal">
                Cat
              </th>
              <th scope="col" className="border-b border-line pb-2 pr-4 font-normal">
                Human
              </th>
              <th scope="col" className="border-b border-line pb-2 font-normal">
                Looks like
              </th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.cat} className="align-top">
                <th scope="row" className="border-b border-line py-2.5 pr-4 text-left font-display text-[17px] font-bold text-accent whitespace-nowrap">
                  {row.cat}
                </th>
                <td className="border-b border-line py-2.5 pr-4 text-text-soft">{row.human}</td>
                <td className="border-b border-line py-2.5">
                  <code className="text-text">{row.example}</code>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="mt-8 max-w-[720px]">
        <p className="text-muted">Put together, it reads like this:</p>
        <CodeBlock
          tryIt
          code={`pawckage "nya:furrball";

scratch cats = ["Tom", "Luna", "Salem"];

pawction introduce(cat) {
    purrhaps cat == "Salem" {
        tail cat + " (a witch's familiar)";
    }
    tail cat;
}

fur cat ~ cats {
    meow("This is", introduce(cat));
}

meow("That's", length(cats), "cats, and", "Luna" ~ cats, "that Luna is one of them.");`}
        />
      </div>
    </section>
  );
}
