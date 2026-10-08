import Link from "next/link";
import { Paw } from "./Paw";

export function SiteHeader() {
  return (
    <header className="mx-auto flex w-full max-w-[1120px] items-center justify-between px-4 py-4 sm:px-6">
      <Link
        href="/"
        className="group flex items-center gap-2 font-display text-[20px] font-extrabold text-text hover:text-accent"
      >
        <Paw className="size-6 text-accent transition-transform duration-150 group-hover:-rotate-12 motion-reduce:transition-none" />
        meowscript
      </Link>
      <nav aria-label="Site" className="flex items-center gap-5 text-[14px] text-muted">
        <Link href="/docs" className="py-2 hover:text-text">
          Docs
        </Link>
        <Link href="/download" className="py-2 hover:text-text">
          Download
        </Link>
        <a href="https://github.com/AlenVelocity/MeowScript" className="py-2 hover:text-text">
          GitHub
        </a>
      </nav>
    </header>
  );
}
