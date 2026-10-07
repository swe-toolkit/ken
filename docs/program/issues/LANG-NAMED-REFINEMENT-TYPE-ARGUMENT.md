---
id: LANG-NAMED-REFINEMENT-TYPE-ARGUMENT
title: "A conversion that holds only by unfolding a named refinement to its carrier is accepted with zero obligations: List Five admits Cons Int six and List Char admits 55296. Keep the ruled polarity guard, and decide whether alignment needs a refinement unfold by kernel conversion in which predicate-owning refinement roots never unfold, after substituting let-bound values"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION, LANG-MATCH-RESULT-REFINEMENT-IDENTITY]
blocks: []
github: null
origin: "F-E: separate finding the Architect's TYPE-POSITION D0 ruling evt_5am0p8wy7vc9j asked to be measured; measured on c49297983 by the language implementer (evt_2tgrvgpr76kwm), boundary confirmed by the Architect (evt_19x6v93jytdy4). Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A conversion that unfolds a named refinement where nothing introduces it is refused

## Objective

A conversion that holds only by unfolding a named refinement, at a position
where nothing introduces its predicate, is refused. A value that really has
the refined type, such as the prelude's `List Char`, stays legal, and the
corpus keeps its base results.

## Settled inputs (Architect S-c ruling `evt_7vpddt8z52k0j`)

- **The shared predicate** (recut `evt_5rpfj17mr3n9n`, research
  `evt_629xjmwz1wd5z`). All three stops read refinement presence off the
  spelling of a delta-transparent `Const` head. The guard's algorithm is
  right; its input was the wrong representation.
- **S-a is withdrawn.** D0 measured that at `elab.rs:1897` neither the
  expected nor the inferred refined type exists; `cong` and `J` results come
  only from `infer` as core terms.
- **S-c, probed by the Architect** on `b76c4f278`
  (`/workspaces/ken/tmp/arch-rigid-probe/probe.diff`): regressions 23 to 1,
  the ten NAMED tests 10/10, the rigid-set-empty mutant fails 9/10, and ζ
  only in the fallback admits a let-bound alias forgery.
- **Retained as proved.** The walk, the safety rule, the Char instance
  transports, LANG-MATCH's checked leaves, the residual arm, the ten
  integration tests and the WIP pins. WIP `b76c4f278` is the base of the
  repair.
- **The remaining row.** `Application/CommandLine/ArgParse.ken.md`
  62912..63209, the `cong` in the `Cons` arm of `positional_schema_fields_map`:
  the expected side carries `Cons Int 60 (...)` where the source wrote `Char`.
  Its producer is unknown.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **S-c exactly as ruled in `evt_7vpddt8z52k0j`.**
   - Kernel: `GlobalEnv.rigid_consts` with its `PartialEq` entry,
     `with_rigid_consts` and `is_rigid` in `ken-kernel/src/env.rs`, and the
     `is_rigid` early return at the top of `unfold_const` in `conv.rs`. The
     set is empty in every checking environment.
   - Elaborator: `ElabCtx.let_values`, populated at both let-binder pushes
     (`check_let` and `infer`'s `RLet` arm). `refuse_unsafe_nested_introduction`
     applies ζ to both sides before the walk, and on a would-be refusal
     accepts when conversion with every predicate-owning root rigid succeeds.
2. **D1 (logs only, after S-c builds).** Trace where the expected type at the
   ArgParse span acquires `Cons Int 60`. The outcomes R1 (an elaborator
   producer unfolds `Char`), R2 (a source value elaborates to the carrier) and
   R3 (the kernel builds it) are each a stop to the Architect.

## Acceptance

Base is current `origin/main`; compare every row against it.

- **AC-1 (refused; base accepts with 0 open).** The forgery rows of the ten
  NAMED tests, plus the let-bound alias forgery `fn forge (u : Int) : List
  Five = let T = Five; ys : List T = Cons Int six (Nil Int) in ys`, refused
  at the inner `Cons`, and the let-bound value forgery `xs = Cons Int six
  (Nil Int); ys : List Five = xs`.
- **AC-2 (controls; accepted, obligation counts unchanged).**
  - The admitted rows of the ten NAMED tests, and one minimal admitted row
    per D0 shape: a let variable against its inlined value inside an
    `Equal` endpoint over `length Char`, and the `J`-motive shape. Each is
    red at `b76c4f278`.
  - **Corpus parity is the gate.** The 81 roots and the 55 packages match
    base file by file with zero newly refused files, including the 23 files
    of `evt_15fxy29zfx7n`, once the ArgParse row is ruled.
  - A kernel unit test: the view blocks δ of exactly the listed ids, and an
    empty view converts like the base environment.
- **AC-3 (mutation, QA).** M-rigid-empty admits the forgery rows.
  M-zeta-off refuses the let-endpoint row. M-zeta-fallback-only admits the
  let-alias forgery.
- **AC-4 (review).** Language QA reviews the candidate. Kernel QA reviews the
  `ken-kernel` hunk on the same exact SHA, and the Architect reviews both.

## Stop conditions

- Any newly refused corpus, catalog, prelude or example program: stop with
  the row. Do not respell.
- Any D1 outcome (R1, R2 or R3).
- Any kernel change beyond the ruled hunk, any change to checking results,
  or any `trusted_base()` or spec change. The core-subset-Σ alternative is
  the open decision `OQ-refinement-representation`, outside this WP.

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. position-keyed refusal of a named refinement (type argument, expression, codomain) refused the prelude's `Char` under `List`/`Option`; keyed on syntactic position
2. nested-introduction guard keyed on elaborator check routes; match compilation checks leaves against an inferred or δ-simplified substitute, so the refinement never reaches `check` — keyed on the type a leaf is checked against
3. after leaves are checked against the as-written result, the conversion guard sees an expected-side `Char` at Arg with no aligned value-side refinement. Keyed on which side of a conversion still spells the refinement-rooted `Const`.
```
