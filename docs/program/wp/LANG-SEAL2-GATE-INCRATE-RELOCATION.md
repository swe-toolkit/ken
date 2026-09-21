# WP frame — `LANG-SEAL2-GATE-INCRATE-RELOCATION`

    owner   language       tier   T2        size   M
    depends none
    node    docs/program/issues/LANG-SEAL2-GATE-INCRATE-RELOCATION.md
    review  Architect NOT required -- the design is fully ruled at
            evt_6b39fyc17xzm1, evt_x1b90s36dtc2, evt_1se8wycskre80.
            Language QA exact-SHA. Code merge → full CI + M8 Adversary.

## 1. Objective

Build, for the first time, the property SEAL-2's producer walk has always
claimed: that no `ElabEnv` namespace can reach `enumerate_producer_types`
unclassified. The node holds the reasoning, the ruled design, and the D1/D2
fence. This frame carries only the re-grounded inputs and the controls.

## 2. Fixed inputs -- RE-MEASURED at `a17235098`; the node's are stale

The node measured at `7fadb164c` and every coordinate below has moved since.
Re-ground again at your own base; this node's coordinates move often.

- `ElabEnv` is `crates/ken-elaborator/src/lib.rs:137-218`: **fifteen `pub`
  fields and exactly ONE `pub(crate)` field**, `standard_operators` at `:173`.
  There are no fully private fields.
- That single field is what defeats the gate, and it is **permanent, not
  transitional**: `lib.rs:10` records `standard_operators::StandardOperatorRole`
  as crate-internal BY CONTRACT. Do not plan around it becoming `pub`.
- `tests/seal2_support/mod.rs` is **1251 lines** (node: 1198). The walk is
  `:130-225`, **96 lines** (node: `:104-172`, 69).
- The walk carries `..` at `:180` today, under a retraction notice at
  `:159-179`. The gate is defeated now, and the file says so in its own words.
- **The walk's consumers are still exactly one file**: `seal2_producer_closure.rs`
  at `:22`, `:181`, `:215`, plus internal `mod.rs:400`.
  `adversary_seal2_repros` imports the other three helpers and **not** the walk
  -- the Architect's correction at `evt_1se8wycskre80` holds at this base.
- `#[ignore]` count is **zero** in both `adversary_seal2_repros.rs` and
  `seal2_producer_closure.rs`. Nothing here is parked, ignored, or red.
- `LANG-R-LAYER-EXPORT-RETRACTION` landed at `8fd30c13a` and touched `lib.rs`
  but **neither** `seal2_support/` nor either seal2 test. It did not move this
  node's subject, and its relocation pattern is precedent, not contention.

## 3. Deliverable

Both D1 and D2 as the node specifies them. They are one node precisely so that
D1 cannot close it. D1 may land first; it may not be reported as satisfying
AC-1.

## 4. Acceptance -- three controls, each able to fail

**AC-1 -- D2's property is demonstrated by a mutation, not asserted.** In a
scratch tree add one field to `ElabEnv` and do NOT touch the walk. The build
must FAIL and the failure must name the relocated walk. Record the field added
and the exact error, then restore byte-exactly. **Showing the walk compiling is
not evidence**: that reads identical in the world where the property holds and
the world where `..` is still there.

**AC-2 -- D1 is shown NOT to be sufficient, by running AC-1's mutation against
it.** Apply the identical one-field mutation to a tree carrying D1 only. It must
break at D1. Then satisfy D1 alone -- type the identifier into D1's destructure
and change nothing else -- and show that the walk still compiles and
`seal2_producer_closure` is **GREEN**. Record that green as the observation it
is: it is the exact failure the fence was built for, not a pass.

**AC-3 -- the interim notice comes out with the `..`.** After D2, the retraction
notice at `:159-179` is deleted and the doc comment's three clauses -- "with no
`..`", "can never be a silent pass", "naming every field ... is the entire
point" -- are true as written. `#[test]` count across the crate unchanged or
higher; `#[ignore]` still zero in both seal2 test files; no `#[doc(hidden)]`
anywhere in the diff.

## 5. Stop condition

Hand back, rather than working around, on any of:

- A second inaccessible `ElabEnv` field, or `standard_operators` not being
  `pub(crate)` at your base. Either changes the input set and is the Steward's.
- The walk's caller being unable to follow it in-crate without weakening an
  assertion. Report which and why; do not keep a public seam to preserve it and
  do not delete the assertion.
- Any fixed input above measuring false at your base.

**Not a stop: a coordinate that moved.** Re-ground it, say so in the candidate,
and continue.
