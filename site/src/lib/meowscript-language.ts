// One tokenizer feeds both the CodeMirror editor and the static code blocks in the docs, so
// they always highlight the same way.

import { HighlightStyle, LanguageSupport, StreamLanguage, syntaxHighlighting } from "@codemirror/language";
import { EditorView } from "@codemirror/view";
import { tags as t } from "@lezer/highlight";

export const KEYWORDS = [
  "scratch",
  "amew",
  "pawction",
  "purrhaps",
  "meowtually",
  "tail",
  "pawckage",
  "purrfect",
  "clawful",
  "mew",
  "furreal",
  "furrever",
  "fur",
  "hiss",
  "continue",
] as const;

const KEYWORD_SET = new Set<string>(KEYWORDS);
const ATOMS = new Set(["purrfect", "clawful", "mew"]);

const OPERATORS = ["<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "=", "<", ">", "+", "-", "*", "/", "%", "!", "&", "|", "^", "~"];

export type TokenKind =
  | "keyword"
  | "atom"
  | "string"
  | "number"
  | "comment"
  | "operator"
  | "punctuation"
  | "function"
  | "property"
  | "variable"
  | "invalid"
  | "text";

export interface Token {
  kind: TokenKind;
  text: string;
}

export interface TokenizerState {
  inBlockComment: boolean;
  inString: boolean;
  afterPossessive: boolean;
}

export function initialState(): TokenizerState {
  return { inBlockComment: false, inString: false, afterPossessive: false };
}

const IDENT_START = /[\p{L}_]/u;
const IDENT = /[\p{L}\p{N}_]/u;
const NUMBER = /^\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?/;

function scanString(line: string, pos: number): [number, boolean] {
  let i = pos;
  while (i < line.length) {
    const c = line[i];
    if (c === "\\") {
      i += 2;
      continue;
    }
    if (c === '"') return [i + 1, true];
    i++;
  }
  return [line.length, false];
}

export function nextToken(line: string, pos: number, state: TokenizerState): [TokenKind, number] {
  if (state.inBlockComment) {
    const close = line.indexOf("*/", pos);
    if (close === -1) return ["comment", line.length];
    state.inBlockComment = false;
    return ["comment", close + 2];
  }
  if (state.inString) {
    const [end, closed] = scanString(line, pos);
    state.inString = !closed;
    return ["string", end];
  }

  const c = line[pos];

  if (/\s/.test(c)) {
    let i = pos + 1;
    while (i < line.length && /\s/.test(line[i])) i++;
    return ["text", i];
  }

  if (c === "/" && line[pos + 1] === "/") return ["comment", line.length];
  if (c === "/" && line[pos + 1] === "*") {
    const close = line.indexOf("*/", pos + 2);
    if (close === -1) {
      state.inBlockComment = true;
      return ["comment", line.length];
    }
    return ["comment", close + 2];
  }

  state.afterPossessive = false;

  if (c === '"') {
    const [end, closed] = scanString(line, pos + 1);
    state.inString = !closed;
    return ["string", end];
  }

  if (/\d/.test(c)) {
    const m = NUMBER.exec(line.slice(pos));
    return ["number", pos + (m ? m[0].length : 1)];
  }

  if (c === "'") {
    const next = line[pos + 2];
    if (line[pos + 1] === "s" && (next === undefined || !IDENT.test(next))) {
      state.afterPossessive = true;
      return ["punctuation", pos + 2];
    }
    return ["invalid", pos + 1];
  }

  if (IDENT_START.test(c)) {
    let i = pos + 1;
    while (i < line.length && IDENT.test(line[i])) i++;
    const word = line.slice(pos, i);
    // The name after `'s` is a property, never a keyword.
    const wasProperty = line.slice(Math.max(0, pos - 2), pos) === "'s";
    if (wasProperty) return ["property", i];
    if (ATOMS.has(word)) return ["atom", i];
    if (KEYWORD_SET.has(word)) return ["keyword", i];
    let j = i;
    while (j < line.length && line[j] === " ") j++;
    return [line[j] === "(" ? "function" : "variable", i];
  }

  for (const op of OPERATORS) {
    if (line.startsWith(op, pos)) return ["operator", pos + op.length];
  }

  if ("()[]{},:;".includes(c)) return ["punctuation", pos + 1];

  return ["invalid", pos + 1];
}

export function tokenizeLine(line: string, state: TokenizerState): Token[] {
  const tokens: Token[] = [];
  let pos = 0;
  while (pos < line.length) {
    const [kind, end] = nextToken(line, pos, state);
    const safeEnd = end > pos ? end : pos + 1;
    tokens.push({ kind, text: line.slice(pos, safeEnd) });
    pos = safeEnd;
  }
  return tokens;
}

const STYLE_FOR_KIND: Record<TokenKind, string | null> = {
  keyword: "keyword",
  atom: "atom",
  string: "string",
  number: "number",
  comment: "comment",
  operator: "operator",
  punctuation: "punctuation",
  function: "variableName.function",
  property: "propertyName",
  variable: "variableName",
  invalid: "invalid",
  text: null,
};

const meowStream = StreamLanguage.define<TokenizerState>({
  name: "meowscript",
  startState: initialState,
  copyState: (s) => ({ ...s }),
  token(stream, state) {
    const [kind, end] = nextToken(stream.string, stream.pos, state);
    stream.pos = end > stream.pos ? end : stream.pos + 1;
    return STYLE_FOR_KIND[kind];
  },
  languageData: {
    commentTokens: { line: "//", block: { open: "/*", close: "*/" } },
    closeBrackets: { brackets: ["(", "[", "{", '"'] },
  },
});

export const meowHighlight = HighlightStyle.define([
  { tag: t.keyword, color: "var(--color-accent)" },
  { tag: t.atom, color: "var(--color-catnip)" },
  { tag: t.number, color: "var(--color-catnip)" },
  { tag: t.string, color: "var(--color-paw)" },
  { tag: t.comment, color: "var(--color-muted)", fontStyle: "italic" },
  { tag: t.operator, color: "var(--color-text-soft)" },
  { tag: t.punctuation, color: "var(--color-text-soft)" },
  { tag: t.function(t.variableName), color: "var(--color-text)" },
  { tag: t.propertyName, color: "var(--color-text)" },
  { tag: t.variableName, color: "var(--color-text)" },
  { tag: t.invalid, color: "var(--color-danger)", textDecoration: "underline wavy" },
]);

export const meowTheme = EditorView.theme(
  {
    "&": {
      backgroundColor: "var(--color-surface)",
      color: "var(--color-text)",
      fontSize: "14px",
      height: "100%",
    },
    ".cm-scroller": {
      fontFamily: "var(--font-mono)",
      lineHeight: "1.6",
    },
    ".cm-content": {
      padding: "12px 0",
      caretColor: "var(--color-accent)",
    },
    ".cm-line": {
      padding: "0 16px 0 8px",
    },
    ".cm-cursor, .cm-dropCursor": {
      borderLeftColor: "var(--color-accent)",
      borderLeftWidth: "2px",
    },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection": {
      backgroundColor: "color-mix(in oklab, var(--color-accent) 28%, transparent)",
    },
    ".cm-activeLine": {
      backgroundColor: "color-mix(in oklab, var(--color-text) 4%, transparent)",
    },
    ".cm-gutters": {
      backgroundColor: "var(--color-surface)",
      color: "var(--color-muted)",
      border: "none",
      paddingLeft: "8px",
    },
    ".cm-activeLineGutter": {
      backgroundColor: "transparent",
      color: "var(--color-text-soft)",
    },
    ".cm-matchingBracket": {
      backgroundColor: "color-mix(in oklab, var(--color-accent) 22%, transparent)",
      outline: "none",
    },
    "&.cm-focused": {
      outline: "none",
    },
  },
  { dark: true },
);

export function meowscript(): LanguageSupport {
  return new LanguageSupport(meowStream, [syntaxHighlighting(meowHighlight)]);
}
