import type { Metadata } from "next";
import Link from "next/link";
import { CodeBlock } from "@/components/CodeBlock";
import { SiteFooter } from "@/components/SiteFooter";
import { SiteHeader } from "@/components/SiteHeader";
import { stdlib } from "@/generated/stdlib";

export const metadata: Metadata = {
  title: "Download",
  description: "Prebuilt MeowScript binaries for Windows, macOS, and Linux, and how to build from source.",
};

const RELEASES = "https://github.com/AlenVelocity/MeowScript/releases";
const LATEST = `${RELEASES}/latest/download`;

const builds = [
  { os: "Windows", chip: "x64", file: "meowscript-x86_64-pc-windows-msvc.zip" },
  { os: "macOS", chip: "Apple silicon", file: "meowscript-aarch64-apple-darwin.tar.gz" },
  { os: "macOS", chip: "Intel", file: "meowscript-x86_64-apple-darwin.tar.gz" },
  { os: "Linux", chip: "x64", file: "meowscript-x86_64-unknown-linux-gnu.tar.gz" },
  { os: "Linux", chip: "arm64", file: "meowscript-aarch64-unknown-linux-gnu.tar.gz" },
];

export default function DownloadPage() {
  const version = `v${stdlib.version}`;
  return (
    <>
      <SiteHeader />
      <main className="mx-auto w-full max-w-[1120px] flex-1 px-4 pt-6 pb-16 sm:px-6 sm:pt-10">
        <div className="max-w-[72ch]">
          <h1 className="font-display text-[36px] font-extrabold sm:text-[44px]">Download</h1>
          <p className="mt-3 text-text-soft">
            The command line build runs <code>.meow</code> files, has a REPL, and can read and write files, which the
            playground can&apos;t. The current release is {version}. Every release, with checksums, is on the{" "}
            <a href={RELEASES} className="text-text underline decoration-line underline-offset-4 hover:text-accent">
              releases page
            </a>
            .
          </p>

          <ul className="mt-8 divide-y divide-line border-y border-line">
            {builds.map((b) => (
              <li key={b.file}>
                <a
                  href={`${LATEST}/${b.file}`}
                  className="group flex min-h-14 items-center justify-between gap-4 py-3 hover:text-accent"
                >
                  <span>
                    <span className="font-display text-[18px] font-bold">{b.os}</span>
                    <span className="ml-2 text-muted">{b.chip}</span>
                  </span>
                  <span className="shrink-0 text-[13px] text-muted group-hover:text-accent">{b.file}</span>
                </a>
              </li>
            ))}
          </ul>

          <h2 className="mt-12 font-display text-[24px] font-bold">After downloading</h2>
          <p className="mt-2 text-text-soft">
            The archive holds the <code>meowscript</code> program, the README, and the example programs. Unpack it
            anywhere and either run it from that folder or put it on your PATH.
          </p>
          <CodeBlock
            plain
            code={`meowscript examples/hello.meow
meowscript                      # the REPL; type scram to leave`}
          />
          <ul className="mt-4 space-y-3 text-text-soft">
            <li>
              <strong className="text-text">Windows</strong> may say the publisher is unknown, because the binary is
              not signed. Choose <em>More info</em>, then <em>Run anyway</em>.
            </li>
            <li>
              <strong className="text-text">macOS</strong> blocks unsigned downloads until you clear the quarantine
              flag: run <code>xattr -d com.apple.quarantine ./meowscript</code> once, or right-click the file and
              choose Open.
            </li>
            <li>
              <strong className="text-text">Linux</strong> needs nothing extra. The tarball keeps the executable
              bit.
            </li>
          </ul>

          <h2 className="mt-12 font-display text-[24px] font-bold">Build from source</h2>
          <p className="mt-2 text-text-soft">With a Rust toolchain (1.88 or newer), cargo can build and install it:</p>
          <CodeBlock
            plain
            code={`cargo install --git https://github.com/AlenVelocity/MeowScript --tag ${version} meowscript-cli`}
          />
          <p className="mt-4 text-text-soft">
            The commands and the REPL are described at the end of the{" "}
            <Link href="/docs#the-command-line" className="text-text underline decoration-line underline-offset-4 hover:text-accent">
              reference
            </Link>
            .
          </p>
        </div>
      </main>
      <SiteFooter />
    </>
  );
}
