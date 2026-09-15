---
id: RESULT-NAMES-TWO-ARTIFACTS
title: "'Result' names two different declarations -- the ITree effect-monad's result and the user's Result type -- so every artifact in the ABI-S6 arc saying 'the demanded Result identity' is ambiguous between them, including the Architect's own rulings. The ambiguity already cost a hunt: three independent reasons the ruled repair could never have worked, and 'they are different types' was the cheapest to check and was checked last."
status: ready
owner: doc
size: M
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Named by the Architect at evt_7t8na2v88mv71 (2026-09-15) after runtime-implementer's provenance read (evt_5twg054svktvx) decoded the two demanded identities and found them to be ITree constructors rather than the user's Result. Architect proposed the disambiguating spelling and routed the node call to the Steward. Steward-filed per COORDINATION section 2."
---

## The defect, which is in the record rather than the code

Two distinct declarations are both spelled `Result`:

- **ITREE-RESULT** — the effect-tree monad's result. Its constructors are
  `ITree::Vis` and `ITree::Ret`. This is what the proof machinery's *demanded*
  identity refers to, and what HS18's contract work is about.
- **USER-RESULT** — the user's `Result` type, e.g. `Result ResourceError Unit`
  in `body_from_write`. Its constructors are `Ok` and `Err`. This is what the
  px8f dispatch traps on.

**They were never the same type.** Measured at `b0a7c2945`
(runtime-implementer, `evt_5twg054svktvx`), the two demanded identities at the
only two bodies that reach the proof path decode as:

    DenseRange { start: 3318, len: 37 }  ->  "ctor:px8f_write_partition::ITree::Vis"
    DenseRange { start: 4362, len: 37 }  ->  "ctor:px8f_write_partition::ITree::Ret"

Not `Result::Ok`, not `Result::Err`.

## What it cost, which is the reason this is a node and not a style note

Three independent reasons now say the ruled `(c)` repair could never have
fixed px8f: the population was wrong, the seat covers one of six emitters, and
**the types were different all along.** The third was by a wide margin the
cheapest to establish and it was established last.

The general form is worth more than the instance: **asking why A does not match
B presupposes A and B are the same kind of thing.** Before investigating a
disagreement, confirm the two things are comparable.

## Scope: what actually needs the sweep

- `docs/program/` ABI-S6 and HS18 material — 15 HS18 documents alone.
- Architect rulings that say "the demanded Result identity" or similar. The
  Architect has stated its own rulings are among the ambiguous artifacts.
- `docs/program/issues/` nodes from this arc, including
  `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` and `RT-TRAP-IDENTITY`.
- Any `spec/` occurrence, if the term reaches there.

**HS18 itself is NOT mis-scoped — checked before filing.** Its scope line reads
*"Keep the whole-function Ret contract; use independently certified infeasible
edges, not Vis relabeling"* (`ABI-S6-HS18-determination.md:20-21`). It names
**Vis and Ret**, which are exactly ITREE-RESULT's constructors, so HS18's sense
is determined by its own text. The hazard is a reader who meets the bare word
`Result` in that material without connecting it to Vis/Ret and reads USER-RESULT.
That is a disambiguation job, not a re-scoping job, and the distinction should
survive into the sweep so nobody "fixes" HS18's scope.

## Deliverables

- **D0.** A disambiguating spelling applied across the corpus: **ITREE-RESULT**
  versus **USER-RESULT**, per the Architect's proposal.
- **D1.** A census of occurrences with each one's resolved sense, so the
  denominator is checkable and a later reader can audit a call rather than
  re-deriving it.
- **D2.** Occurrences whose sense is **not** determined by their own context are
  **routed to the Architect, not guessed.** This is the deliverable most likely
  to be skipped, and guessing here reintroduces exactly the defect.

## Acceptance criteria

- **AC-1.** Every occurrence is either disambiguated or listed as routed to the
  Architect. No occurrence is left bare and unlisted.
- **AC-2.** The census states how it was enumerated, including the search terms,
  so a zero for any term is readable as evidence about that term rather than
  about the corpus.
- **AC-3.** HS18's scope is **unchanged in substance.** This node disambiguates
  wording; it does not re-scope a determination. If the sweep appears to require
  a scope change, stop and route it.
- **AC-4.** No code identifiers are renamed under this node. Source-level
  renaming is a separate decision with a different blast radius; if the sweep
  finds the ambiguity is load-bearing in code too, file that separately.

## OPEN SCOPE QUESTION: this node is `doc`, and the defect recurs in CODE

Raised by the Architect at `evt_53p7m3s2f6t8q`, recorded here rather than acted
on. **Four times in this arc a name has not told us its population:**

    the contract's two `demanded` artifacts, eighteen lines apart
    `Result` naming ITREE-RESULT and USER-RESULT          (this node)
    `generated_constructor_authorities` ranging over EVERY carried constructor
        (RT-AUTHORITY-CONTRACT-MISDESCRIBES-ITS-POPULATION)
    `PlannedTrapIdentity`, a dedup index keyed on trap value with no origin
        (RT-TRAP-IDENTITY)

**At least two of those are in code, which a doc sweep does not reach.** So this
node may be scoped wrong: it is `owner: doc`, and a corpus disambiguation pass
cannot catch a field whose name misdescribes its population in Rust.

**Deliberately not converted into a node tonight**, and the reason is the test
in `§4c`: a node needs an assignable deliverable, and *"names should tell their
populations"* is not one yet. The three code instances already have their own
nodes; the pattern across them is an observation, not work.

**The trigger to revisit:** a fifth instance, or any instance that the three
existing nodes do not already cover. At that point the right artifact is
probably a code-side convention or a check — not a widening of this doc sweep.
Do not widen this node's scope to absorb them.

## Contention

Doc-only, `docs/` and possibly `spec/`. **Contention-free with the build lane**
per the doc-track exception (`CLAUDE.md`). Does not touch `crates/`.

## Related

- `RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION` — filed the same hour; its D1
  requires tags decoded to names precisely because a raw word cannot separate
  an ITREE-RESULT constructor from a USER-RESULT one.
- `RT-TRAP-IDENTITY` — the adjacent non-discrimination, one level down:
  `PlannedTrapIdentity` is a dedup index keyed on trap value with no
  source-origin.
- `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` — the arc's other one-term-two-
  artifacts instance, at the struct-field level rather than the type name.
