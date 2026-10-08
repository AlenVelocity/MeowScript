// sessionStorage rather than a URL parameter, so a refresh doesn't resurrect the snippet.

const KEY = "meowscript.try-it";

export function handOverCode(code: string): void {
  try {
    window.sessionStorage.setItem(KEY, code);
  } catch {
    // Storage unavailable (private mode, say). The playground shows its default.
  }
}

export function takeHandedCode(): string | null {
  try {
    const code = window.sessionStorage.getItem(KEY);
    if (code !== null) window.sessionStorage.removeItem(KEY);
    return code;
  } catch {
    return null;
  }
}
