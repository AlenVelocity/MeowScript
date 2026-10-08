"use client";

import { useRouter } from "next/navigation";
import { handOverCode } from "@/lib/try-it";

export function TryItButton({ code }: { code: string }) {
  const router = useRouter();
  const tryIt = () => {
    handOverCode(`${code}\n`);
    router.push("/#playground");
  };
  return (
    <button
      type="button"
      onClick={tryIt}
      className="absolute top-2 right-2 h-8 rounded-md border border-line bg-bg px-2.5 text-[12px] text-muted hover:border-accent hover:text-accent focus-visible:border-accent focus-visible:text-accent"
    >
      Try it
    </button>
  );
}
