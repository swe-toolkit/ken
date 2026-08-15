---
id: LANG-MEMBERSHIP-OPERATOR-SURFACE
title: "membership has no parser arm in either spelling, and ASCII `in` -- which `31 §1b` requires to be the same token as `∈` -- is consumed by the `let … in` keyword, so the spec's accepted-forever ASCII guarantee fails for exactly this operator"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "CONF-BLOCKER-MARKER-RECONCILE's D3, answered by the spec enclave with the citation its AC-4 demanded and corroborated independently by the conformance-validator (31-lexical.md:33-35, :79, :101-113). Steward ruling evt_bgat447r9s6w: this is an unowned surface gap, not a keyword-role decision -- the endpoint-(b) reading is refuted by citation. Steward-filed per COORDINATION §2. Supplies the blocker for seed-canonical-format.md:387 and FMT1's aggregate at :52."
---

> # THE ENDPOINT READING IS REFUTED BY CITATION, NOT BY PREFERENCE.
>
> The natural conclusion from the tree alone is that ASCII `in` is simply
> committed to the keyword role and membership is glyph-only — a settled
> endpoint, like the level token in [[CONF-FMT8-LEVELTOK]]. **The spec says
> otherwise, in the general rule and in the specific table row**, and the
> enclave produced both citations rather than an opinion. Do not re-litigate
> the endpoint; measure against the cited requirement.

## The citations, which are the premise

`spec/30-surface/31-lexical.md:79` — the notation table:

| glyph | ASCII | role |
|---|---|---|
| `∈` | `in` | membership |

`31-lexical.md:105-112` — the general rule, stated as a **lexer capability**:

> *"A curated Unicode glyph and its ASCII transliteration ... lex to the
> **identical** token ... So the glyph carries **zero** extra information and
> **ASCII spellings remain accepted forever** (no program ever requires a
> special keyboard). This is genuinely a **lexer** capability, not only a
> convention."*

## The measurement

| fact | site |
|---|---|
| ASCII `in` maps to the keyword token | `lexer.rs:997` — `"in" => Token::KwIn` |
| source `∈` maps to `Member` | the glyph arm |
| **membership expression arm in the parser** | **absent — in either spelling** |

⇒ **Two distinct absences, and the second is the larger one.** Even the glyph
spelling does not parse as membership: `Member` is lexed and nothing consumes
it. So this is not merely an ASCII-alias gap; **the operator has no parse at
all.**

## Why this one is genuinely harder than its siblings, and is `M` not `S`

[[LANG-BYTES-HEX-LIST-LITERAL]] is a second spelling of an existing token. **This
is not**, because the ASCII bytes are already spoken for:

> **`let x = value in body`.** The same three characters are the `let`-binder's
> keyword. `seed-canonical-format.md:387`'s `why` states the bind exactly:
> *"the same input bytes occupy opposite token roles. Replacing every `in`
> either corrupts the keyword or fails to canonicalize membership."*

**So the lexer cannot decide this on the bytes alone**, and `31 §1b`'s rule —
one token for both spellings — collides with `let … in` as literally stated.
**That collision is the node.** It is why this is not a one-line lexer arm and
why it is filed `M`.

## Deliverables

**`D1` — the parser arm, glyph spelling first.** Make `∈` parse as a membership
expression. **This half has no keyword collision and is the part that is
unambiguously owed** — do it first and report it separately, so that a hard stop
on `D2` still lands a real increment.

**`D2` — determine what `31 §1b` actually requires of ASCII `in` here, and
report before you implement.** The candidates, and none is to be chosen by the
ring:

1. **Context discriminates.** `in` after a `let` binder is the keyword;
   elsewhere in expression position it is membership. Report whether the
   grammar makes that decidable at the point the lexer or parser must decide,
   **with the ambiguous case named if one exists.**
2. **`31 §1b` has an unstated exception** for bytes already bound to a keyword,
   and the table row at `:79` is in error or under-qualified. **Then the spec is
   what changes**, and this is a finding for the enclave, not a repair here.
3. **A different ASCII spelling** is intended for membership. Report whether
   anything in `31` supports one; **do not invent one.**

**`D3` — report which of the three holds, with the evidence, and stop there if
it is (2) or (3).** Those outcomes are spec changes and the Architect rules
them. Only (1) is implementable on this node's authority.

## Acceptance criteria

**`AC-1`.** `∈` parses as membership and elaborates. **Control:** a fixture
using the glyph, asserted on the elaborated form — not "it compiles".

**`AC-2`.** `let x = e in b` is unchanged. **Control:** the existing `let`
tests are named individually in the handback and stay green. **This is the
regression the whole node risks, and a green suite reported as a total is not
evidence for it.**

**`AC-3`.** `D2`'s verdict is reported with citations before any ASCII handling
is implemented. **A candidate that implements option 1 without first reporting
that option 1 holds has skipped the deliverable**, even if the code is right.

**`AC-4`.** If the outcome is (2) or (3), **no ASCII handling is implemented
at all** and the node stops with `D1` landed. That is a good outcome, not a
partial failure.

**`AC-5`.** Direction stated for every behaviour change. This **adds** accepted
programs; nothing currently accepted becomes rejected. If anything does, stop.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## What this unblocks, and the follow-through that is NOT yours

`seed-canonical-format.md:387` carries
`BLOCKED-ON-MEMBERSHIP-ASCII-ROLE (no blocker node exists)`, and FMT1's
aggregate at `:52` names the same surface. **This node is that blocker.**
[[CONF-BLOCKER-MARKER-RECONCILE]] will name it.

**Do not edit `conformance/`.** That seed is the spec enclave's and is in flight.
Say in the handback that it is now owned.

## Not this node

- **Not the IFC lattice operators.** `31-lexical.md:81`'s `⊑`/`<:` row is a
  different table entry with a different consumer.
- **Not a general notation-table audit.** If you find other glyph/ASCII pairs
  with no parse, **report them and stop** — that is a Steward re-cut, and it is
  a good finding.
