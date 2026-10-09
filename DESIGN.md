# Design notes for the MeowScript site

The site at `site/` is a playground first and a landing page second. These notes record the choices so later changes stay consistent. Tokens live in `site/src/app/globals.css`.

## Brief

**Subject.** A toy programming language whose keywords are cat puns, with a real tree-walking interpreter written in Rust and compiled to WebAssembly. Its world is terminals, warm fur, yarn, and late-night tinkering. Playful, and still a working tool: the interpreter runs and the errors are specific.

**Audience.** Developers arriving from GitHub or a tweet with about thirty seconds of curiosity. Mostly on a desktop, sometimes on a phone. They want to see a program run and understand the puns.

**Job.** Run a MeowScript program and watch the cat answer. Then learn what the keywords mean.

**Surface.** A working tool with a thin marketing wrapper. The playground sits at the top of the page, not below a hero and feature cards.

## Plan

**Color.** Dark only, because it is a code playground and the content is syntax-highlighted code. The neutrals are warm, tinted toward amber like a tabby, rather than blue-grey.

| Token | Value | Role |
| --- | --- | --- |
| `bg` | `#171310` | page |
| `surface` | `#201a15` | editor, output, code listings |
| `surface-2` | `#2a231c` | inline code, hover rows |
| `line` | `#3b3128` | borders and table rules |
| `text` | `#f1e8d8` | body (15:1 on `bg`) |
| `text-soft` | `#cdbfab` | secondary prose, operators in code |
| `muted` | `#a09078` | labels, hints, comments (5.3:1 on `bg`) |
| `accent` | `#f0a048` | tabby orange: the Run button, the paw marks, keywords, link hover |
| `paw` | `#e9a9bd` | strings in code only |
| `catnip` | `#a7d79c` | numbers and `purrfect`/`clawful`/`mew` in code only |
| `danger` | `#f0846f` | errors in output, the Stop button, the squiggle |

The accent appears in four places: the primary button, the paw marks, keywords in code, and link hover. Headings and borders stay neutral so it keeps meaning something.

**Type.** Baloo 2 (700, 800) for headings: rounded and a little chunky, which reads as soft and cat-like without becoming a children's book. Fira Code (400) for body and code, self-hosted. Setting prose in the code font says "everything here is code" and carries the old site's identity forward; ligatures are turned off outside code so prose doesn't get `=>` arrows. Scale: 13 / 14 / 15 / 18 / 24 / 40 (52 on wide screens).

**Layout.** One left-aligned column, 1120px wide at most, 72 characters for docs prose. Narrow layout first: the editor stacks above the output, and on large screens they sit side by side at 3:2.

```
[paw meowscript                      Docs  GitHub]

The purrfect programming language.        . .
One paragraph.                          .  . .
                                           .  .
Example [yarn.meow v]           Clear  [paw Run]
+------------------------------+  +-----------------+
| editor                       |  | output          |
|                              |  |                 |
+------------------------------+  | Ready   v1.0.0  |
                                  +-----------------+
Cat to human (table)
Batteries, in a bag of nya (package list)
Run it on your own machine
footer
```

**Signature.** The output panel's voice. Every line a program prints with `meow` starts with `Meow!`, and the site sets that word in the accent with a small paw in front of it, so the program's output is visibly the cat talking. The status bar speaks the same way: "Playing…" while a program runs, "Purring" when it finishes, "Hissed" when it fails, and an error's cat exclamation ("Hiss!", "Meowch!") is set in the display face.

**Small jokes, placed once each.** A trail of faint paw prints walks from the headline toward the Run button on wide screens. "purrfect" in the headline carries a spellchecker squiggle, because it is misspelled on purpose. The paw on the logo and on the Run button tilts on hover. The 404 page is headed "Meow-sterious." None of these repeat elsewhere.

## Checked against the defaults

No gradients, no centered hero with three cards, no emoji as icons (the paw is one inline SVG, reused by reference), no uppercase letter-spaced labels, no numbered markers, no scroll animations. The one near-default is "dark background with one warm accent"; it stays because the accent is tied to the subject (an orange tabby) and the surfaces are tinted toward it rather than neutral.

## States

- Interpreter loading: Run reads "Loading…" and is disabled; the output panel says so.
- Empty output: a faint paw and "Press Run, or Ctrl+Enter in the editor, to hear from the cat."
- Running: Run becomes Stop (outlined, `danger`). Stop terminates the worker and starts a fresh one.
- Error: the interpreter's rendered error, with its caret line, in `danger` and preserved whitespace.
- Worker failed to start: an explanation and a link to the issue tracker.
- Output over 3000 lines: the oldest are trimmed with a note.
- Sharing: Share writes the program into the URL fragment and copies the link. Its label reads "Link copied" for two seconds, or "Link is in the address bar" when the clipboard refuses. Opening a link loads its program with the picker set to "from a link"; a link that won't decode loads the default example with a note under the playground.

## Motion

The paw trail fades in once on load, staggered, 240ms per print. Hover tilts on the two paws, 150ms. Button presses nudge one pixel. `scroll-behavior: smooth` for in-page links. All of it is off under `prefers-reduced-motion`.
