import type { CSSProperties } from "react";

export function Paw({ className = "", style }: { className?: string; style?: CSSProperties }) {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" className={className} style={style}>
      <ellipse cx="5.6" cy="10.2" rx="2.1" ry="2.7" />
      <ellipse cx="18.4" cy="10.2" rx="2.1" ry="2.7" />
      <ellipse cx="9.4" cy="6" rx="2.1" ry="2.8" />
      <ellipse cx="14.6" cy="6" rx="2.1" ry="2.8" />
      <path d="M12 10.8c3.9 0 6.9 3 6.9 6 0 2.3-1.6 3.7-3.6 3.7-1.3 0-2.2-.6-3.3-.6s-2 .6-3.3.6c-2 0-3.6-1.4-3.6-3.7 0-3 3-6 6.9-6z" />
    </svg>
  );
}
