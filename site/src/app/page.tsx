import Link from "next/link";
import { CheatSheet } from "@/components/CheatSheet";
import { CodeBlock } from "@/components/CodeBlock";
import { Paw } from "@/components/Paw";
import { PlaygroundLoader } from "@/components/PlaygroundLoader";
import { SiteFooter } from "@/components/SiteFooter";
import { SiteHeader } from "@/components/SiteHeader";
import { examples } from "@/generated/examples";
import { stdlib } from "@/generated/stdlib";

const trail = [
  { left: "58%", top: "6%", rotate: -32 },
  { left: "64%", top: "22%", rotate: -18 },
  { left: "71%", top: "36%", rotate: -32 },
  { left: "77%", top: "52%", rotate: -18 },
  { left: "84%", top: "66%", rotate: -32 },
  { left: "90%", top: "82%", rotate: -18 },
];

function PawTrail() {
  return (
    <div aria-hidden="true" className="pointer-events-none absolute inset-0 hidden md:block">
      {trail.map((paw, i) => (
        <Paw
          key={i}
          className="paw-step absolute size-7 text-text"
          style={{
            left: paw.left,
            top: paw.top,
            transform: `rotate(${paw.rotate}deg) scaleX(${i % 2 ? -1 : 1})`,
            opacity: 0.05 + i * 0.015,
            animationDelay: `${300 + i * 140}ms`,
          }}
        />
      ))}
    </div>
  );
}

export default function Home() {
  return (
    <>
      <SiteHeader />
      <main className="mx-auto w-full max-w-[1120px] flex-1 px-4 sm:px-6">
        <section className="relative pt-6 pb-8 sm:pt-10 sm:pb-10">
          <PawTrail />
          <h1 className="relative max-w-[18ch] font-display text-[40px] font-extrabold sm:text-[52px]">
            The <span className="squiggle">purrfect</span> programming language.
          </h1>
          <p className="relative mt-4 max-w-[60ch] text-[15px] text-text-soft sm:text-[17px]">
            MeowScript is a small scripting language where <code className="text-accent">purrhaps</code> means if,{" "}
            <code className="text-accent">pawction</code> means function, and every program announces itself with a{" "}
            <code className="text-accent">meow</code>. The interpreter is written in Rust and runs right here, in your
            browser.
          </p>
        </section>

        <PlaygroundLoader examples={examples} />

        <div className="mt-20">
          <CheatSheet />
        </div>

        <section id="packages" aria-labelledby="packages-title" className="mt-20 scroll-mt-20">
          <h2 id="packages-title" className="font-display text-[24px] font-bold sm:text-[28px]">
            Batteries, in a bag of nya
          </h2>
          <p className="mt-2 max-w-[62ch] text-muted">
            <code>meow</code>, <code>purr</code>, and <code>length</code> are always there. Everything else is a{" "}
            <code>pawckage</code> away.
          </p>
          <ul className="mt-6 grid gap-x-8 gap-y-4 sm:grid-cols-2">
            {stdlib.packages.map((pkg) => (
              <li key={pkg.name} className="border-t border-line pt-3">
                <Link
                  href={`/docs#${pkg.name.replace(":", "-")}`}
                  className="font-display text-[18px] font-bold hover:text-accent"
                >
                  {pkg.name}
                </Link>
                <p className="mt-1 text-[14px] text-muted">{pkg.doc}</p>
                <p className="mt-1 text-[13px] text-text-soft">
                  {pkg.functions.map((f) => f.name).join(", ")}
                  {pkg.constants.length > 0 && `, ${pkg.constants.map((c) => c.name).join(", ")}`}
                </p>
              </li>
            ))}
          </ul>
        </section>

        <section id="install" aria-labelledby="install-title" className="mt-20 scroll-mt-20">
          <h2 id="install-title" className="font-display text-[24px] font-bold sm:text-[28px]">
            Run it on your own machine
          </h2>
          <p className="mt-2 max-w-[62ch] text-muted">
            The command line build adds a REPL and file access. It needs a Rust toolchain.
          </p>
          <div className="max-w-[720px]">
            <CodeBlock
              plain
              code={`cargo install --git https://github.com/AlenVelocity/MeowScript meowscript-cli

meowscript                 # the REPL
meowscript hello.meow      # run a file
meowscript packages        # what's in the nya: pawckages`}
            />
          </div>
          <p className="text-muted">
            The full language reference is in the{" "}
            <Link href="/docs" className="text-text underline decoration-line underline-offset-4 hover:text-accent">
              docs
            </Link>
            .
          </p>
        </section>
      </main>
      <SiteFooter />
    </>
  );
}
