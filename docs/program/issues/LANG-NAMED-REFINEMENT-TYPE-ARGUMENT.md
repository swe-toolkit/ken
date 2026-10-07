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

## Settled inputs (Architect S-c `evt_7vpddt8z52k0j`, R1 `evt_1sh6f2ycy40yk`)

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
- **D1 returned R1** (`evt_4df4xkfec6gwg`). `simplify_branch_goal_keeping_refinements`
  makes a refinement rigid only when it appears by name in the goal, so a
  root exposed by δ is unfolded by the per-child `whnf`. That yields
  ArgParse's `Cons Int 60` and admits an alias-branch forgery. Running the
  simplifier refinement-rigid gives population delta 0 on the Architect's
  probe. A cloned environment per call costs 3-4x, so the mechanism is a
  scoped δ block.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

S-c with R1, as ruled in `evt_7vpddt8z52k0j` and `evt_1sh6f2ycy40yk`:

- **Kernel (`ken-kernel/src/conv.rs`).** A thread-local rigid set read at
  the top of `unfold_const`, the sole δ site; `with_rigid_consts`, which
  scopes it and restores the outer scope on exit and unwind; and
  `rigid_scope_active`. `with_rigid_consts` is exported. Each public
  `check.rs` entry that writes a declaration into `GlobalEnv` (the
  `declare_*`, `admit_*` and `register_*` functions) asserts no active scope
  under `debug_assert!`. Outside a scope nothing is blocked, so checking results
  are unchanged.
- **Elaborator.** `ElabCtx.rigid_refinements`, built once from the
  predicate-owning roots. `simplify_branch_goal_keeping_refinements` runs
  inside the scope at every caller, and `collect_refinement_consts` is
  deleted if nothing else uses it. The guard keeps ζ before the walk and
  the let side table (`check_let` and `infer`'s `RLet` arm). On a would-be
  refusal it accepts when conversion inside the scope succeeds.

## Acceptance

Base is current `origin/main`; compare every row against it.

- **AC-1 (refused; base accepts with 0 open).** The forgery rows of the ten
  NAMED tests, plus the let-bound alias forgery `fn forge (u : Int) : List
  Five = let T = Five; ys : List T = Cons Int six (Nil Int) in ys`, refused
  at the inner `Cons`; the let-bound value forgery `xs = Cons Int six
  (Nil Int); ys : List Five = xs`; and the alias-branch forgery `def L =
  List Five; fn forge (b : Bool) : L = match b { True ↦ Cons Int six (Nil
  Int); False ↦ Nil Int }`, admitted on S-c without R1.
- **AC-2 (controls; accepted, obligation counts unchanged).**
  - The admitted rows of the ten NAMED tests, and one minimal admitted row
    per D0 shape: a let variable against its inlined value inside an
    `Equal` endpoint over `length Char`, the `J`-motive shape, and the
    ArgParse shape (a dependent match whose goal δ-exposes
    `string_to_list_char "<"` inside an `Equal` at a record). Each is red at
    `b76c4f278`.
  - **Corpus parity is the gate.** The 81 roots and the 55 packages match
    base file by file with zero newly refused files, including the 23 files
    of `evt_15fxy29zfx7n`.
  - A kernel unit test: δ is blocked only for listed ids, a nested scope
    restores the outer one, and the scope is restored after `catch_unwind`.
  - **Timing.** `ken check` wall time for Derived and ArgParse stays within
    10% of S-c without R1 (2.5s and 8s on the Architect's probe). State the
    population's wall time.
- **AC-3 (mutation, QA).** M-rigid-empty admits the forgery rows.
  M-zeta-off refuses the let-endpoint row. M-zeta-fallback-only admits the
  let-alias forgery. M-simplifier-unscoped admits the alias-branch row and
  refuses the ArgParse-shape row. Each mutant is first reported in a
  discrimination table (reach, rows reddened, observation).
- **AC-4 (review).** Language QA reviews the candidate. Kernel QA reviews the
  `ken-kernel` hunk on the same exact SHA, and the Architect reviews both.

## Stop conditions

- Any newly refused corpus, catalog, prelude or example program: stop with
  the row. Do not respell.
- Timing over 10% of S-c without R1.
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
