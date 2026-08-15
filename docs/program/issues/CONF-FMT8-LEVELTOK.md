---
id: CONF-FMT8-LEVELTOK
title: "FMT8's fixture is unproducible: the row demands a 'genuine level-token fixture' but the lexer has no Level/Label token kind and never will under endpoint (b)"
status: active
owner: spec-enclave
size: S
gate: none
depends_on: [SPEC-IDENT-BLESSED]
blocks: []
github: null
origin: "Raised by the conformance-validator's block on SPEC-IDENT-BLESSED successor b3468101 (2026-07-27); both carriers independently verified by the Steward at origin/main d6df571e. Ruled out of that WP's scope in evt_7egdvdf68p7a4 and filed here. Framed and flipped ready by the Steward 2026-08-14 at 88667c204, re-measuring every fixed input."
---

> # `ready` as of 2026-08-14. Frame: `docs/program/wp/CONF-FMT8-LEVELTOK.md`.
>
> The `depends_on` edge recorded a **scheduling** dependency — the endpoint (b)
> ruling this node rests on landed with `SPEC-IDENT-BLESSED`, which is
> `merged`. That dependency is discharged.
>
> **Re-measured at `88667c204`: every claim below still holds**, and the
> mechanism is now pinned at `file:line` in the frame rather than asserted.

## The measurement

`conformance/surface/formatting/seed-canonical-format.md:304`, row
`surface/formatting/l-identifier-is-not-a-level-token`:

- **given:** `fn keep_l (l : Nat) : Nat = l` beside *"a genuine level-token
  fixture using the canonical level role"*
- **expect:** `RED-UNTIL-BUILT (B2/B3/C)` — *"only the parsed level token
  prints `ℓ`"*

**The lexer has no Level/Label token kind.** A `Token`-scoped grep for either
in `crates/ken-elaborator/src/lexer.rs` returns nothing. All three of `ℓ`
(`lexer.rs:885`), `l` (`lexer.rs:1043`), and `level` produce
`Token::Ident("level")`; the formatter preserves whichever raw source lexeme
was written.

**The spec the row cites agrees.** `spec/30-surface/31-lexical.md:82` records
the level role as *"supplied by parser context"* — not as a token kind. So the
row's demand contradicts its own citation.

⇒ **The fixture the row requires cannot be constructed**, and under the ruled
endpoint (b) it never will be — the absence of a distinct level token is the
ruling. **FMT8 as written can never go green.**

## Why this is a defect class, not a stale line

**A `RED-UNTIL-BUILT` row whose fixture is unproducible is byte-identical, to
any reader, to a row that simply has not been built yet.** It sits red forever
and reads as pending. Nothing in the corpus distinguishes "waiting on work"
from "waiting on something that will never exist."

Same class as `SEC1-IFC-R3`'s synthetic `Disproved` verdicts: a row whose
evidence can never be real, sitting in a corpus that reports it as merely
outstanding.

⇒ **The valuable deliverable is not this one row — it is the sweep.** How many
other rows name a fixture the landed lexer and formatter cannot produce? The
frame scopes that census to the formatting seed's **20** `RED-UNTIL-BUILT`
rows and puts the other **7** explicitly out of scope rather than silently
skipping them.

## Scope notes

- **Not foldable into `SPEC-IDENT-BLESSED`** — that WP is spec-only and
  `conformance/**` is outside its edit authority. Ruled `evt_7egdvdf68p7a4`.
- The `SPEC-IDENT-BLESSED` successor carries a **forward note** naming this
  node, so the contradiction is recorded rather than silent. That note is not
  a fix and must not be read as one.
- The row's *intent* is sound and worth preserving: it discriminates a
  raw-byte over-firing canonicalizer from a correct one. The repair is to
  re-express that discrimination over operands the lexer **can** produce —
  source `ℓ` vs `l` vs `level`, all resolving to one binding while each
  round-trips to its own spelling. **Do not simply delete the row**; that
  discards a real discriminator.
- **The unproducible/pending distinction already has a landed spelling.**
  `conformance/behavioral/buffer-io/seed-buffer-io.md:727` carries
  `BLOCKED-ON-NATIVE-REACHABILITY ([[RT-NATIVE-FNSPLIT]])`. Reuse that shape;
  do not mint a vocabulary.
