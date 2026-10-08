import type { Metadata } from "next";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { CodeBlock } from "@/components/CodeBlock";
import { SiteFooter } from "@/components/SiteFooter";
import { SiteHeader } from "@/components/SiteHeader";
import { StdlibReference } from "@/components/StdlibReference";

export const metadata: Metadata = {
  title: "Docs",
  description: "The MeowScript language reference: every keyword, operator, and nya: pawckage.",
};

function slug(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-");
}

function textOf(node: React.ReactNode): string {
  if (typeof node === "string") return node;
  if (Array.isArray(node)) return node.map(textOf).join("");
  if (node && typeof node === "object" && "props" in node) {
    return textOf((node as { props: { children?: React.ReactNode } }).props.children);
  }
  return "";
}

export default function DocsPage() {
  const source = readFileSync(join(process.cwd(), "content", "docs.md"), "utf8");

  return (
    <>
      <SiteHeader />
      <main className="mx-auto w-full max-w-[1120px] flex-1 px-4 pt-6 pb-16 sm:px-6 sm:pt-10">
        <article className="docs">
          <Markdown
            remarkPlugins={[remarkGfm]}
            components={{
              h2: ({ children }) => <h2 id={slug(textOf(children))}>{children}</h2>,
              h3: ({ children }) => <h3 id={slug(textOf(children))}>{children}</h3>,
              pre: ({ children }) => <>{children}</>,
              code: ({ className, children }) => {
                const text = textOf(children);
                const isBlock = text.includes("\n") || (className ?? "").startsWith("language-");
                if (!isBlock) return <code>{children}</code>;
                const lang = (className ?? "").replace("language-", "");
                return <CodeBlock code={text} plain={lang !== "meow" && lang !== ""} tryIt={lang === "meow"} />;
              },
            }}
          >
            {source}
          </Markdown>
          <StdlibReference />
        </article>
      </main>
      <SiteFooter />
    </>
  );
}
