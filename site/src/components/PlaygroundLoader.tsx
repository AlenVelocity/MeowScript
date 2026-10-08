"use client";

import dynamic from "next/dynamic";
import type { Example } from "@/generated/examples";

function PlaygroundSkeleton() {
  return (
    <section id="playground" aria-label="Playground" aria-busy="true" className="scroll-mt-20">
      <div className="flex items-center justify-between pb-3">
        <div className="h-9 w-40 rounded-md border border-line bg-surface" />
        <div className="h-9 w-20 rounded-md bg-accent/40" />
      </div>
      <div className="grid gap-3 lg:grid-cols-[3fr_2fr] lg:gap-4">
        <div className="h-[22rem] rounded-lg border border-line bg-surface sm:h-[26rem] lg:h-[30rem]" />
        <div className="flex h-[18rem] items-center justify-center rounded-lg border border-line bg-surface text-[14px] text-muted sm:h-[20rem] lg:h-[30rem]">
          Loading the playground…
        </div>
      </div>
    </section>
  );
}

const Playground = dynamic(() => import("./Playground").then((m) => m.Playground), {
  ssr: false,
  loading: PlaygroundSkeleton,
});

export function PlaygroundLoader({ examples }: { examples: Example[] }) {
  return <Playground examples={examples} />;
}
