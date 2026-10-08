import Link from "next/link";
import { Paw } from "@/components/Paw";
import { SiteFooter } from "@/components/SiteFooter";
import { SiteHeader } from "@/components/SiteHeader";

export default function NotFound() {
  return (
    <>
      <SiteHeader />
      <main className="mx-auto flex w-full max-w-[1120px] flex-1 flex-col items-center justify-center px-4 py-16 text-center sm:px-6">
        <Paw className="size-16 -rotate-12 text-text/20" />
        <h1 className="mt-6 font-display text-[32px] font-extrabold sm:text-[40px]">Meow-sterious.</h1>
        <p className="mt-2 max-w-[40ch] text-text-soft">There is no page at this address.</p>
        <Link
          href="/"
          className="mt-8 inline-flex h-10 items-center rounded-md bg-accent px-5 text-[14px] font-semibold text-bg hover:bg-accent-strong"
        >
          Back to the playground
        </Link>
      </main>
      <SiteFooter />
    </>
  );
}
