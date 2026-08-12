# LANG-SURFACE-LITERAL-ESCAPES — escapes, Char, bytes, and raw strings

Owner: language. Size: M. Node: [[LANG-SURFACE-LITERAL-ESCAPES]].

**Fixed inputs measured at `origin/main` = `762a9b44`**, every coordinate read
from the git object at that SHA. Line numbers are anchors to re-find at your own
base. Re-derive your merge-base from `origin/main`; **do not take a SHA from
this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if your
diff stays in `crates/` — and it must, see excluded scope.

## 1. The starting point is barer than "add escapes"

**The lexer performs no escape processing at all.** Its single string scan
(`crates/ken-elaborator/src/lexer.rs`, around `:214-230`) is:

```rust
Some('"')  => { self.advance(); break; }        // closing quote
Some(c)    => { self.advance(); s.push(c); }    // EVERYTHING else, verbatim
None | Some('\n') => { /* "unterminated string literal", with a span */ }
```

A backslash is just a character. `"\n"` is backslash-then-`n`. There is no
`InvalidEscape` in the file, and **`Char` literals, byte literals, byte strings
and raw triple strings do not exist.**

**The semantic targets are already built** — `String`, `Char` and `Bytes` all
resolve in the prelude (`prelude.rs:1292`, `:1302`, `:1502`). **This is scanner
and token work. If you find yourself adding a type, stop; you are in the wrong
node.**

## 2. Your specification is landed and your controls are already written

`SPEC-LITERAL-ESCAPE-PIN` merged at PR #1947. **Do not re-derive any of this and
do not amend `spec/`:**

- normative text — `spec/30-surface/31-lexical.md`
- **six discriminating conformance rows** —
  `conformance/surface/literals/seed-escapes.md`

**The seed is your acceptance surface, not a suggestion.** It pins common exact
decoding; the closed kind-selected repertoire; Unicode shape, scalar domain and
Char cardinality; ASCII byte bodies with exhaustive `\xHH` and the fixed-width
`b"\x41BC"` case; raw-triple non-processing; and `InvalidEscape` span and
precedence against ordinary unterminated controls.

> ### The pin's first candidate was GREEN under the wrong implementation
>
> **Row 6 is why it isn't any more.**
>
> Its pending-escape fixtures were String-stopped-by-delimiter,
> String-stopped-by-line-boundary and byte-string-stopped-by-EOF, with **Char
> appearing only in the paired no-pending-escape controls.** So nothing forced
> an incomplete **Char** escape to choose `InvalidEscape` over the unterminated
> path — an implementation that keeps every completed and malformed Char escape
> correct while routing a Char ending mid-escape to its unterminated-literal
> path passed every row.
>
> The recut replaced the redundant String/line fixture with a **Char/line**
> fixture, completing the matrix as String/delimiter, Char/line, Bytes/EOF.
> **The three legs are not redundant with each other. Do not collapse them.**

## 3. The design judgment, front-loaded

**The repertoire is CLOSED and SELECTED BY KIND.** Every unlisted sequence
rejects **by construction** — because the kind's table has no entry — and not by
an enumerated deny-list. **If you write a list of rejected escapes, you have
built the wrong thing**: a deny-list is silently incomplete the moment anyone
adds a character, and the pin chose closure precisely to make that unrepresentable.

**`\xHH` is fixed-width at exactly two hex digits, and byte-only.**
`b"\x41BC"` is bytes `0x41 0x42 0x43`. **No greedy lookahead** — this is the
opposite of the Unicode escape's one-to-six-digit rule, and getting one rule to
serve both is the tempting error.

**Precedence is lane-owned and it inverts an existing error.** Once the
backslash commits the lexer to an escape production, a literal ending before
that production completes raises **`InvalidEscape`, not** the existing
unterminated-literal error. **Every other unterminated-literal behaviour is
unchanged.** You are narrowing one path out of an existing error, not replacing
that error.

**Spans exclude the boundary.** The span begins at the backslash and ends
immediately after the last offending character; an interrupting delimiter, line
boundary or EOF is **outside** it.

## 4. Deliverables

**`D0` — `Token::Str`'s existing consumer, decided before anything else.**
`Token::Str` currently carries the symbol and library names in `foreign`
declarations (`lexer.rs:88`). Turning on escape processing **changes what a
`foreign` name containing a backslash means.** Measure whether any such name
exists in the tree and in the prelude, then decide: escapes apply there too, or
`foreign` names take a distinct non-escaping path. **State which and why in the
handback.** Do it first — it can invalidate the shape of `D1`.

**`D1` — escapes in ordinary strings**, closed and kind-selected, with Unicode
escapes at one-to-six hex digits denoting non-surrogate scalars.

**`D2` — `Char` literals**, including the cardinality rule.

**`D3` — byte literals and byte strings**, with fixed-width `\xHH` and
**ASCII-only** unescaped content.

**`D4` — raw triple strings**, which perform **no** escape processing.

**`D5` — `InvalidEscape`**, with the exact span rule and the precedence over
unterminated, emitting **no literal token**.

## 5. Acceptance criteria

**AC-1 — the six seed rows pass, as the seed writes them.** *Control:* the
conformance suite. **Not a paraphrase of the rows in your own test file** — the
seed is the artifact under agreement and a restatement can drift from it.

**AC-2 — the Char mid-escape leg is exercised independently.** *Control:* the
Char/line-boundary fixture fails on an implementation that routes an incomplete
Char escape to the unterminated path, and passes after. **Section 2 explains why
this specific one; do not credit it from the String or Bytes legs.**

**AC-3 — closure is structural, not enumerated.** *Control:* an escape nobody
anticipated — pick one absent from every table and every test — rejects with
`InvalidEscape`, and the rejection is reached because the kind's table has no
entry rather than because a deny-list names it.

**AC-4 — `\xHH` does not consume a third digit.** *Control:* `b"\x41BC"`
decodes to exactly `0x41 0x42 0x43`. **`AC-3` and `AC-4` are the two an
otherwise-correct implementation is most likely to fail.**

**AC-5 — spans exclude the boundary**, asserted as spans and not merely as
"an error occurred."

**AC-6 — ordinary unterminated behaviour is unchanged.** *Control:* a committed
case with **no** pending escape still raises the existing unterminated-literal
error with its existing span. **A negative check passes for any reason, so this
needs its positive twin** — the same shape *with* a pending escape raising
`InvalidEscape` instead.

**AC-7 — `D0`'s decision is committed as a control**, whichever way it went.

**AC-8 — no `spec/` and no `conformance/` edit, no new kernel term,
`trusted_base()` unchanged.** *Control:* `git diff --name-only` touches neither
directory. **If you believe the seed is wrong, that is a stop, not an edit** —
it is a merged agreement between two T1 Spec seats and the Architect.

**AC-9 — CI green.** Not a local `--workspace` run (`COORDINATION §12`).

## 6. Excluded scope

- **Editing the pin or the seed.** See `AC-8`.
- **Formatter rendering beyond round-trip preservation** of the new forms.
- **Numeric literals and `0x[…]` carrier ownership** — the pin disclaims both.
- **Char-cardinality, non-ASCII-byte-body and ordinary unterminated
  diagnostics** beyond what the seed names; the pin deliberately leaves their
  wording alone.
- **Block and doc comments** — [[LANG-SURFACE-BLOCK-COMMENTS]].

## 7. Stop conditions — return to me, do not decide

- **`D0` finds `foreign` names that escape processing would change**, and both
  answers have real costs. That is a compatibility call, not an implementation
  one.
- **A seed row appears wrong or unsatisfiable.** Do not amend it and do not work
  around it.
- **Closure cannot be made structural** without a deny-list.
- **The precedence rule cannot be expressed** without weakening the existing
  unterminated-literal error for cases the pin says are unchanged.

## 8. Contention and sizing

`crates/ken-elaborator/src/lexer.rs`, plus token-kind consumers.

**[[LANG-SURFACE-BLOCK-COMMENTS]] also opens `lexer.rs`** and is `ready`. Both
touch the scanner, so **the lane runs them one at a time**; sequence is mine and
either order works — they are disjoint in what they scan (`{-` and `--` versus
quote-delimited bodies), so whichever runs second rebases cheaply.
**Re-derive the intersection at candidate time** — a merge-base goes stale
without your branch moving.

`scripts/ken-cargo test -p ken-elaborator` plus the conformance suite for the
seed rows. **Never `--workspace`**; that is CI's gate.

**Sizing note.** The research sweep scoped character, byte and string-escape
literals as **three** M nodes. They are one node here because they are one
scanner, one error and one span rule — the node header explains the cut. If it
proves to be genuinely three units of work rather than three deliverables,
**that is a stop and the re-cut is mine.**
