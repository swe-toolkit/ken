---
id: LANG-MEMBERSHIP-OPERATOR-SURFACE
title: "membership has no parser arm in either spelling, and ASCII `in` -- which `31 §1b` requires to be the same token as `∈` -- is consumed by the `let … in` keyword, so the spec's accepted-forever ASCII guarantee fails for exactly this operator"
status: draft
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "CONF-BLOCKER-MARKER-RECONCILE's D3, answered by the spec enclave with the citation its AC-4 demanded and corroborated independently by the conformance-validator (31-lexical.md:33-35, :79, :101-113). Steward ruling evt_bgat447r9s6w: this is an unowned surface gap, not a keyword-role decision -- the endpoint-(b) reading is refuted by citation. Steward-filed per COORDINATION §2. Supplies the blocker for seed-canonical-format.md:387 and FMT1's aggregate at :52. RECUT 2026-09-06 (Steward) on the Architect hard-stop ruling evt_356e6vfg2hrs6: the node is NOT buildable as framed -- AC-1's parse+elaborate has no honest semantic target -- so it is split, deferred, and returned to draft. See the recut banner."
---

> # RECUT 2026-09-06 -- NOT BUILDABLE AS FRAMED (Architect evt_356e6vfg2hrs6).
> # Hard stop #1 on this WP. The node is split, deferred, and returned to draft.
> # This banner supersedes the Deliverables and Acceptance criteria below; the
> # original body is retained for history.
> #
> # WHY IT IS NOT BUILDABLE. The Architect grounded this at origin/main 072dc688d
> # (the WP base) and verified: (1) the infix-operator surface is CLOSED -- BinOp
> # = {Add, WrappingAdd, Sub, Mul, EqEq} lowered through a closed numeric/structural
> # registry, with NO open, user-extensible typeclass method-dispatch for operators;
> # (2) membership is NOT one operation -- it is four distinct NAMED functions each
> # needing an explicit witness (elem needs Ord a; set_member needs leq; member;
> # rel_member), and there is NO unifying `class Membership` in the catalog;
> # (3) `∈` has ZERO uses in the entire .ken corpus -- a reserved dead token with no
> # consumer and no demand; (4) ASCII `in` is UNCONDITIONALLY the let-separator
> # keyword. So AC-1 ("`∈` parses AND elaborates") has nothing honest to elaborate
> # TO: a fixed lowering fabricates a witness out of nowhere (dishonest), and a
> # correct general `x ∈ s` requires type-directed class-method dispatch + witness
> # synthesis -- a MAJOR new language capability that does not exist. A parse-only
> # arm that always errors at elaboration is strictly worse than the current clean
> # dead-token state. Do NOT build it (honesty-about-the-boundary, docs/PRINCIPLES).
> #
> # THE SPLIT (Architect route §4; recut is the Steward's):
> #
> # (A) THE REAL DRIVER IS THE FORMATTER, and it is SPEC's, not this node's. The
> #     only actual demand is seed-canonical-format.md:387 BLOCKED-ON-MEMBERSHIP-
> #     ASCII-ROLE + FMT1:52 -- the formatter needs a COHERENT §1b notation row,
> #     NOT a working value operator. D2 resolved to frame option (2): the §1b:79
> #     row (`| ∈ | in | membership |`) is internally incoherent -- §1c/BL3's
> #     "ASCII accepted forever, identical token" guarantee cannot be satisfied for
> #     a glyph whose only natural ASCII form is a reserved keyword word, and the
> #     landed lexer already chose Token::Member DISTINCT from KwIn. The Architect
> #     recommends (spec-leader authors, evt_356e6vfg2hrs6): declare `∈` GLYPH-ONLY
> #     with a narrow documented exception to the total-ASCII-transliteration
> #     guarantee. That matches the landed lexer and §1a P4 exactly and discharges
> #     the formatter blocker with ZERO parser/elaborator work. This is now a SPEC
> #     obligation (spec-leader, in flight per the Architect's @mention) plus the
> #     already-planned CONF-BLOCKER-MARKER-RECONCILE marker reconciliation. It is
> #     NOT owned by this language node.
> #
> # (B) THE MEMBERSHIP VALUE OPERATOR (parse `∈` + elaborate) is what THIS node is
> #     recut to, and it is DEFERRED: 0 corpus demand, no semantic target, GATED on
> #     a membership/typeclass-dispatch capability that does not exist. No such
> #     capability node exists yet and none is minted speculatively for zero-demand
> #     work (ken-steward §4c) -- this node stays `draft` as the parking place for
> #     the obligation. When a typeclass-method-dispatch capability is cut for real
> #     demand, this node is re-cut against it; language-leader remains owner and
> #     the Architect remains required reviewer. Do NOT release it before then.
> #
> # LANGUAGE STOOD DOWN CORRECTLY (language-leader evt_wyj2aqmqs316): no source
> # edit, no implementer dispatch, nothing landed on the language side. The honest
> # increment is Spec's row amendment, then -- if demand ever appears -- a future
> # dispatch-capability node ahead of a re-cut (B).
> #
> # --- original body retained below for history (superseded by this banner) ---
>
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
