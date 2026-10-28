---
id: LANG-PRELUDE-FLOOR-KEYED-EFFECTS
title: "Admit Unit, Coproduct and Instant to the reserved strict prelude floor as the second keyed-additions slice, with Coproduct and Instant scoped to their checked constructor families and the spec roster, count and PRELUDE_FLOOR_NAMES changing together"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-KEYED-ADDITIONS]
blocks: []
github: null
origin: "Slice 2 of the keyed floor additions, from the same D0 table (LANG-PRELUDE-FLOOR-KEYED-ADDITIONS deliverable 2). Keying rows: Architect evt_a9zrvmkrazcr. Source need: Spec Leader evt_4abm4krcwzqf8. Steward-filed per COORDINATION section 2."
---

# The strict floor admits Unit, Coproduct and Instant

## Objective

Strict resolution admits `Unit`, `Coproduct` and `Instant` as reserved floor
types, each resolving to its exact registered id. The spec 30 §4 roster and
count, the ledger rows and `PRELUDE_FLOOR_NAMES` agree on one branch.

## Settled inputs (main `9d4ba1e83`)

These are the names that have both witness clauses (spec 30 §4).

| Type | Keying (Architect `evt_a9zrvmkrazcr`) | Source need (Spec `evt_4abm4krcwzqf8`) | Constructors (ledger) |
|---|---|---|---|
| `Unit` | env:unit_id; role:unit (`MkUnit`) | 36 §1.1, §2.1 | unscoped `MkUnit` |
| `Coproduct` | env:coproduct_id; `InL`/`InR` roles | 36 §2.3, §4.5.7 | scoped `Coproduct.InL`, `Coproduct.InR` |
| `Instant` | keyed through `MkInstant` | 36 §1.1 (`now`) | scoped `Instant.MkInstant` |

- **The mechanism is slice 1's.** That is `PRELUDE_FLOOR_NAMES`, floor-id
  admission and `scope_constructor_family` (LANG-PRELUDE-FLOOR-KEYED-ADDITIONS,
  routed at `295657a4a`). Build on a base with slice 1 landed.
- **Excluded from this slice, with the reason for each:**
  - **`ITree` (`Ret`/`Vis`).** It has both witnesses. Its scope is not yet
    ruled, and a scoped family would migrate about 1,900 bare lines in 126
    files, concentrated in the ken-cli native and planner tests that
    `ABI-S6-HS18-MAIN-BASED-CLOSURE` is respinning. It is its own successor
    slice, framed once Spec rules its scope and, if it is scoped, after
    increment A lands.
  - **`Console`.** Spec requires the effect name, but the D0 has no keyed
    identity spelled `Console`. The keyed family is `ConsoleOp`, which Spec
    rules compiler-internal with no rename. Not admitted.
  - **`Stream`/`Stdin`/`Stdout`/`Stderr`** are keyed but have no source need
    (37 §5). They stay internal.
  - **`get`, `put`, `run_state`, `bind`, `resp_coproduct`, `inject_l` and
    `inject_r`** stay package or internal, per Spec.
- **Migration size.** Scoping touches the bare `InL`/`InR` consumers (about
  137 lines in 42 files outside `crates/*/src`) and the bare `MkInstant`
  consumers (24 lines in 11 files). Migrate them token-aware, as slice 1 did.
  None of them is in increment A's candidate paths. The responses.rs hit is
  Rust, not Ken.
- **No stop.** Every CAT-*-PRELUDE-MOVE and LANG-PRELUDE-* row is merged
  except slice 1 itself (Architect `evt_a9zrvmkrazcr`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One spec+crates branch admits the three types. It changes the 30 §4 roster
and count, the ledger rows and `PRELUDE_FLOOR_NAMES` (20 to 23) together, and
migrates the consumers. The Spec co-turn on the same branch owns the
`spec/` text, and CV votes on the `spec/` and `conformance/` paths.

## Acceptance

- **AC-1.** In both strict and legacy resolution modes:
  - each admitted type resolves to its exact registered id;
  - `Coproduct.InL`, `Coproduct.InR` and `Instant.MkInstant` resolve to their
    checked constructor ids, and bare `InL`, `InR` and `MkInstant` are
    refused;
  - bare `MkUnit` resolves;
  - a same-name source declaration of any admitted type is refused.
- **AC-2.** The spec count equals `PRELUDE_FLOOR_NAMES.len()`, and the census
  diff is deletion-only with predicted pairs. The strict ambient row of a
  catalog package that used `Unit` ambiently no longer lists it.
- **AC-3.** `trusted_base()` is unchanged as an identity set.
- **AC-4 (mutation, QA).** Each mutant fails and is restored:
  - removing one admitted name from `PRELUDE_FLOOR_NAMES` reddens its AC-1
    row;
  - admitting `Coproduct` unscoped reddens the bare `InL` refusal row in both
    modes.

## Stop conditions

- An admission that the D0 table does not support with both witness clauses.
- A migration hunk in a path that increment A of
  `ABI-S6-HS18-MAIN-BASED-CLOSURE` also changes, if A has not landed. Report
  it; do not rebase across it.
- Any kernel or trust change, or the flip, which stays with L2-4.

Checks: 4 fired: every admitted name, producer and source clause is read from
the two rulings on `9d4ba1e83`; the mechanism is named in slice 1's terms. 3
fired: the scoped-constructor consumer population was measured by
`git grep -w` on `origin/main` outside `crates/*/src` (InL 24 files, InR 18,
MkInstant 11), and its overlap with increment A's paths was measured as none
in Ken source. 6 fired: the ITree population (Ret 126 files, Vis 96) is the
measured reason it is split out, not inferred.
