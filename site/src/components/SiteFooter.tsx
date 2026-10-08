export function SiteFooter() {
  return (
    <footer className="mx-auto w-full max-w-[1120px] px-4 py-10 text-[13px] text-muted sm:px-6">
      <p>
        Made by{" "}
        <a href="https://twitter.com/Alenvelocity" className="underline decoration-line underline-offset-4 hover:text-text">
          Alen
        </a>
        , with help from a cat. Interpreter in Rust, running in your browser as WebAssembly.
      </p>
    </footer>
  );
}
