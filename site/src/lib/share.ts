// A shared program travels in the URL fragment, which browsers never send to the server.
// `#z=` holds it deflated; `#code=` holds plain UTF-8 for browsers without CompressionStream.
// Both are base64url, so nothing in them needs percent-encoding.

const DEFLATED = "z";
const PLAIN = "code";

function params(hash: string): URLSearchParams {
  return new URLSearchParams(hash.replace(/^#/, ""));
}

export function hasSharedProgram(hash: string): boolean {
  const p = params(hash);
  return p.has(DEFLATED) || p.has(PLAIN);
}

// Returns the fragment without its `#`.
export async function encodeProgram(code: string): Promise<string> {
  const bytes = new TextEncoder().encode(code);
  if (typeof CompressionStream === "undefined") return `${PLAIN}=${toBase64Url(bytes)}`;
  const deflated = await pipe(bytes, new CompressionStream("deflate-raw"));
  return `${DEFLATED}=${toBase64Url(deflated)}`;
}

// Null when the fragment carries no program, or one that won't decode, usually because the link
// was cut short somewhere along the way.
export async function decodeProgram(hash: string): Promise<string | null> {
  const p = params(hash);
  const deflated = p.get(DEFLATED);
  const plain = p.get(PLAIN);
  try {
    if (deflated !== null) {
      if (typeof DecompressionStream === "undefined") return null;
      return utf8(await pipe(fromBase64Url(deflated), new DecompressionStream("deflate-raw")));
    }
    if (plain !== null) return utf8(fromBase64Url(plain));
  } catch {
    return null;
  }
  return null;
}

async function pipe(bytes: BufferSource, stream: CompressionStream | DecompressionStream): Promise<Uint8Array> {
  const output = new Blob([bytes]).stream().pipeThrough(stream);
  return new Uint8Array(await new Response(output).arrayBuffer());
}

function utf8(bytes: Uint8Array): string {
  return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
}

function toBase64Url(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function fromBase64Url(text: string): Uint8Array<ArrayBuffer> {
  const binary = atob(text.replace(/-/g, "+").replace(/_/g, "/"));
  return Uint8Array.from(binary, (c) => c.charCodeAt(0));
}
