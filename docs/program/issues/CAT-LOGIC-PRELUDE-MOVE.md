---
id: CAT-LOGIC-PRELUDE-MOVE
title: "Move the prelude's logic conveniences into catalog packages: And, and_intro, and_fst, and_snd and Not into a Core/Logic home, and Dec/Yes/No/Empty onto Core.Logic.EmptyDec's own definitions, with every consumer importing them; opened by the identity guard that proves no compiler code or other prelude definition holds each moved identity; first L3 slice of the minimal-prelude program"
status: active
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: the prelude is the minimal set required; 'any convenience names that are not required by the prelude rules above (or the kernel built-ins) should be considered technical debt and moved to packages'. Program design Architect evt_1ymf3p94jygx3 and evt_4s5he6tnf3xs4 (slice L3-1, arms C and D). Steward-filed per COORDINATION section 2."
---

# Move the logic conveniences out of the prelude

## Objective

Every catalog module that uses `And`, `and_intro`, `and_fst`, `and_snd`,
`Not`, `Dec`, `Yes`, `No` or `Empty` resolves it through an import of a
catalog package, never through the prelude fall-through.

## Settled inputs -- Architect, measured at `50966beb2`

- `spec/30-surface/30-taxonomy.md §4` keeps `And`/`Dec`/`Yes`/`No`/`Empty`
  out of the prelude: source-constructible, with a `Core/Logic` package home.
- The prelude registers them in `crates/ken-elaborator/src/prelude.rs`
  (`And` at `:946`, `and_intro`/`and_fst`/`and_snd` from `:948`, `Not` at
  `:1170`, `Dec` at `:1493`, `Empty` near `:1457`). `And`'s body is
  `Σ(_:A).B` at `Ω`. `and_id` is also stored in a struct field (`:354`,
  `:3060`), which the identity guard must resolve.
- `Core.Logic.EmptyDec` already defines `Empty` and `Dec` but its census row
  still lists `Dec`/`Empty`/`Yes`/`No`/`Not` as ambient. Explain this in AC-0.
- Census at `50966beb2`: `And` and `and_intro` in 32 rows, `and_fst` and
  `and_snd` in 31, `Dec`/`Yes`/`No`/`Empty`/`Not` in one each.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- A `Core/Logic` package home for `And` and its three helpers and `Not` (a new
  module or an existing sibling, the Architect's choice at AC-0), with the
  same statements.
- `Dec`/`Yes`/`No`/`Empty` resolve to `Core.Logic.EmptyDec`'s definitions.
- Every consumer imports them. The prelude registrations stay until the L2
  flip; nothing in `crates/` changes except test expectations.

## Acceptance

- **AC-0 (identity guard, post to the WP thread, then proceed).** For each
  name to move:
  - (i) no compiler code holds its identity: a producer reading from
    registration to every reader, not a grep. `and_id` at `prelude.rs:354`
    is the known holder to resolve.
  - (ii) no other prelude definition's body mentions it (its
    reverse-dependency closure inside the prelude).
  - (iii) the EmptyDec anomaly explained.

  A name that fails (i) or (ii) either moves with its closure, if the
  closure is itself movable, or it is keyed: STOP for routing.
- **AC-1.** Every catalog module that used a moved name now imports it. The
  catalog checks. Attached proofs that consumed the prelude identities are
  unchanged in statement.
- **AC-2.** The census diff is deletion-only. The deleted (row, name) pairs
  are predicted before the build and measured after; an unpredicted deletion
  is a finding. No `trusted_base()` change. Targeted builds only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- A moved name is keyed (AC-0 (i) or (ii) fails with a non-movable closure).
- A consumer's statement changes, or a proof needs the prelude identity.
- Any `trusted_base()` growth, or any new ambient name in the census.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
