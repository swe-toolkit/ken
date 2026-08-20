---
id: RT-CONTROL-INTEGRATION-TESTS-SPLIT
title: "Cut control.rs's residual integration-test root below 10k by integration-test OWNERSHIP seam -- the named successor RT-BACKEND-SPLIT-CLOSURE left open, a fresh test-ownership axis, not a missed production owner"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-BACKEND-SPLIT-CLOSURE]
blocks: []
github: null
origin: "Steward, 2026-08-20, on the merge of [[RT-BACKEND-SPLIT-CLOSURE]] (item 18). That node's AC-6 phase closure reported OPEN with a named successor: control.rs (lowering/core/tests/control.rs) measured 30099 lines, over the operator's 10k ceiling, and its residual is integration-test end-to-end controls, not a missed production owner. Architect evt_3y64s8k4y9e27 named this successor as 'a fresh integration-test ownership cut by seam -- a real reduction path off this campaign's owner-extraction axis'. Steward-filed per COORDINATION section 2. The core.rs/mod.rs residual (~13k/~12k, the no-fourteenth-owner limit) is a SEPARATE, larger design question -- the Lowering hub struct's own state decomposition -- NOT in this node's scope; see Sequencing."
---

> # THIS IS A DIFFERENT AXIS FROM THE MODULE SPLIT. Do not read it as a missed owner.
>
> The phase [[RT-BACKEND-MODULE-SPLIT]] decomposed the backend by PRODUCTION
> semantic lifecycle (13 owners). Its closure remeasure ([[RT-BACKEND-SPLIT-CLOSURE]]
> strand 3) classified control.rs's 218 tests four ways and found that what
> remains over 10k is **class-4 end-to-end controls crossing planning through
> execution** — tests the frame's own guidance says are SUPPOSED to stay in the
> residual integration module, not domain tests that were mislaid.
>
> ⇒ **Cutting them is a fresh integration-test OWNERSHIP cut, by seam, NOT a
> production-owner extraction and NOT a size operation.** Grouping by coherent
> integration-test scope is the axis; hitting a line number is not.

## The operator's constraint, and it is the only one

**2026-08-18: "Files over 10k lines are decomposed into architecturally sound
smaller files. That is the whole constraint."** control.rs at 30099 violates it.
The factorization and sequencing are the Steward's and Architect's. **Nothing
else in this frame is an operator constraint** beyond that sentence — re-derive
each at use.

## The input, measured and landed -- do not re-run the classification

**[[RT-BACKEND-SPLIT-CLOSURE]] strand 3 already classified control.rs's 218
tests**, and its closure ledger is on `main`. Read it; do not re-derive it. The
landed partition:

| class | count (approx) | disposition |
|---|---|---|
| class-1 domain test | ~1 | already relocated to boundary.rs by the closure slice |
| class-3 mutation control at an already-moved domain's injection point | ~40-55 | candidates to re-home WITH their domain, if a clean single-domain home exists |
| **class-4 end-to-end control (planning -> execution)** | **~150-165** | **the residual this node cuts, by integration-test seam** |
| the file's own designated subject (`oriented_*`, `px8j_*`, root-authority) | remainder | stays; control.rs's own lifecycle |

**The class-4 bloc is the load-bearing population.** ~150-165 end-to-end
controls is what holds control.rs over 10k after the module split; it is a real
integration-test corpus with no production-domain home, and it must not be
converted to domain tests or moved by size to shrink the file.

## THE SEAM IS DESIGN JUDGMENT -- D0 proposes it, the Architect votes it

The exact partition of the class-4 bloc into architecturally-sound sub-10k
integration-test modules is **not pre-enumerated here** — that would duplicate D0
and go stale. What is frozen is the shape:

- **Group by integration-test SCOPE, not by size.** A coherent seam is a family
  of end-to-end controls that exercise the same pipeline span or scenario axis
  (e.g. plan-construction-through-lowering, effect/aggregate end-to-end,
  trap/terminal end-to-end, boundary-transfer end-to-end) — a name a reader
  recognizes as a test-ownership boundary, never "control_part_2".
- **Every resulting module — and the residual control.rs — must land below
  10k.** Projected child sizes are part of the D0 proposal, not discovered after
  the move.
- **A class-4 control stays a class-4 control.** Relocating it into an
  integration-test module by scope is permitted; rewriting it into a
  single-domain mutation test to give it a domain home is the conversion the
  closure frame forbids.

## Deliverables

**`D0` — the seam proposal and the projected population.** From the landed
strand-3 classification, propose the integration-test ownership seam(s): each
candidate module named by its test-ownership scope, the class-4 controls it
takes, the projected line count of each new module AND of the residual
control.rs, all below 10k. Include the ~40-55 class-3 mutation controls'
disposition (re-home with domain where a clean single-domain home exists;
otherwise they stay). **This is the design judgment; hand back for the
Architect's vote before any move.** No code moves in D0.

**`D1..Dn` — execute the cuts the Architect votes**, one reviewable move per seam
(or grouped only as the Architect's D0 vote permits), **remeasuring the tree
after each**. Each move is byte-faithful (the test body character-identical from
the `///`/`#[test]` through the closing brace; only relocation-provenance
comments and the needed `use` lines added) with before/after discovery parity.

## `D0` — the seam proposal, re-measured at pickup `3e0a5e6a8`

**Method.** All 217 `#[test]` functions in `control.rs` extracted with exact
line spans (each test's span runs to just before the next `#[test]`, so a
helper/fixture function physically between two tests travels with the
preceding test -- matching the same "moves with its dominant use" precedent
`RT-EMITTER-AGGREGATES-SPLIT` established). Classification corroborated
against evidence, not name alone: every test's own `use crate::
cranelift_backend::lowering::{...}` import traced to its **declaring
file** (the `mod.rs` hub vs. an already-extracted domain file), plus full
body reads of ~25-30 representative tests spread across every proposed
bucket. One test was reclassified after a body spot-check found its name
pattern misleading. **Every one of the 217 tests classifies; nothing is
left over**, and every proposed bucket's line sum is a direct read of real
line spans, not an estimate -- the six numbers below (five new modules
plus the residual) sum to exactly 30,099, the file's own current line
count.

### Proposed modules -- named by integration-test scope, not size

| # | module | tests | lines | scope |
|---|---|---:|---:|---|
| 1 | `recursor_fusion.rs` | 25 | 3,836 | Recursor fusion-identity plane and continuation-key routing end-to-end (`d2f_*`, `r3_fused_*`/`r3_4b_*`, `d0_r3_fusion_gate_*`, `contkey_*`, `required_consumer_*`, `planned_closure_preexistence_*`, `missing_call_input_*`) -- all compile through `px8j_capture_source_trace`/`plan_static_transition_graph` and assert on the production fusion-plane/continuation-key counters. |
| 2 | `host_call_carrier.rs` | 47 | 4,714 | Host-call boundary, carrier production and trap-identity end-to-end (`typed_trap_exit_*`, `rtfp_*`, `b2f_*`, `d4_*`, `governed_nested_brackets_*`, first-wave `d6a_*`/`d6b_*`, `rt_scale_b_*`, the host-result-carrier `d8_*` five, `d7_*`, `ac1b_*`, `rt_d2_*`, `d2_ac6_*`, `d5_c2_*`/`d5_c4_*`/`d5_the_*`, and siblings). |
| 3 | `specialization_binding.rs` | 48 | 5,826 | Specialization marker / generated-context creation through composed-call binding resolution, end-to-end (`d5a_*` (16), `continuation_case_binder_run_*`, `d4b_*`, first-wave `d3b_*`, `contsrc_*`, first-wave `ced_d2_*`, second-wave `d6a_*`, `erasing_a_seat_key_axis_*`, `d4a_*`, second-wave `d3b_*`/`d3c_*`, `d7a_*`, `d8a_*`/`d8b_*`/`d8d_*`/`d8e_*`/`d8h_*`/`d8i_*`/`d8j_*`/`d8k_*`). |
| 4 | `source_frame_bridge.rs` | 22 | 5,909 | Source-frame bridge, checked-frame consumption and functionized-shared-emitter end-to-end (`d8l2_*`, `d8m_*` (7), `d8n_*`, `d8o_*`, `d8p_*`, `d8f_*`, `d8g_*`, second-wave `d6b_*`/`d6c_*`). |
| 5 | `positional_candidate_settlement.rs` | 29 | 4,204 | Positional/de-Bruijn binding order and continuation-candidate settlement/fusion-local ledger end-to-end (`d9b_*`, the positional `d3_*` cluster, `ccr_d3_*`, `coc_d3_*`, `sar_d3_*`, second-wave `ced_d2_*`/`ced_d1_*`/`ced_d3_*` (the `m1..m5` mutation-row cluster), `call_edge_executability_axis_*`, `d2b_*`, `d2k_*`, `r3_the_base_uncomposed_*`, `dp_composition_time_*`, `ac_d3_self_*`, `d3_the_fusion_local_composition_ledger_*`). |

**Residual `control.rs`: 46 tests, 5,610 lines** -- `oriented_*`, `px8j_*`,
`row2_*`, `distinguished_*`, `unmarked_*`, `nested_computational_*`,
`checked_*`, `self_consistent_*`, `valid_root_*`, the structural-census
pins (`correspondence_*`/`the_*`, which scan production *source text*
directly, a third kind of test distinct from class-3/class-4 that still
belongs to the file's own remit), and
`rt_escape_within_path_duplicate_frame_consume_still_rejects` (a direct-API
`Lowering` fixture, no pipeline entry). This is the file's own native
lifecycle per its own header, unchanged. **Every heavily-shared
fixture-generator physically embedded in these native spans stays put**
(`px8j_capture_source_trace`, `px8j_scope_chain_observation_result`,
`oriented_test_*`, `self_consistent_*_join_site`, `root_authority_test_
lowering`) -- the new modules reach it via qualified path
(`super::control::px8j_capture_source_trace(...)`), the exact mechanism
`core/tests/effects.rs` already uses today to call `control::
recursive_port_process_compiles`, established at `RT-EMITTER-EFFECTS-
SPLIT` `D2`.

**Reconciliation:** 5,610 + 3,836 + 4,714 + 5,826 + 5,909 + 4,204 =
**30,099** exactly. All five new modules and the residual land comfortably
under the 10k ceiling; the largest (`source_frame_bridge.rs`, 5,909) has
the least headroom.

### Two shared helpers need explicit promotion, not a design change

`recursive_port_process_compiles` (33 call sites across the proposed
modules) and `d8f_compile`/`d8f_compile_with` (11 call sites) are each
currently embedded in one module's own span (`host_call_carrier.rs` and
`source_frame_bridge.rs` respectively) but used from several others.
Both already carry sufficient visibility (`pub(in ...lowering::core::
tests)` / `pub(in ...lowering)`) for a cross-file qualified-path call to
work without any widening -- but leaving a heavily-shared helper
anchored in whichever module happens to textually contain it is an
accident of history, not a design choice. **Proposed: promote both into
`core/tests/mod.rs`** (1,707 lines at pickup, ample headroom) as
declared shared test support, matching the file's own existing role for
`recursive_port_process_compiles`'s own sibling shared fixtures.

### Class-3 disposition -- re-derived from the actual declaration sites,
### not assumed from the prior classification's own framing

Each of the five class-3 clusters the prior closure named was traced to
its mutation type's own **declaring file**, checked against whether that
file has a pre-existing `mod tests` to re-home into:

- **`d3b_*`** -- flips `D3bConsumerMutation`, declared in `lowering/
  mod.rs` (the hub, not a domain). **Stays** (lands in
  `specialization_binding.rs` above, per its own pipeline behavior).
- **`d3c_*`** -- flips `D3cPositionSelection`, also `mod.rs`. **Stays**
  (same module).
- **`d5a_*`** -- flips `D5aMarkerMutation`, also `mod.rs`. **Stays**
  (same module).
- **`typed_trap_exit_*`** -- flips a triple of mutations from **three
  different already-extracted files** (`TrapIdentityMutation` in
  `joins.rs`, `TrapCallerProtocolMutation` in `calls.rs`,
  `TrapFrameBindingMutation` in `mod.rs`), and its own body calls
  `plan_static_transition_graph_with_symbols` directly. Genuinely
  cross-domain and pipeline-driven, not single-domain by any measure.
  **Lands in `host_call_carrier.rs`**, not re-homed to any one domain.
- **`ced_d3_*`** (and its `ced_d2_*`/`ced_d1_*`/`sar_d3_*`/`coc_d3_*`/
  `ccr_d3_*` neighbors) -- the *one* cluster whose enum family
  (`D3Mutation`/`D3Seat`, independently confirmed declared at
  `units.rs:4064`/`4107`) is genuinely single-domain-owned. But **`units.
  rs` has no `mod tests` of its own** (independently confirmed --
  `boundary.rs`/`aggregates.rs`/`calls.rs`/`source.rs` each have one,
  `units.rs` and `joins.rs` do not), so there is no existing home to
  move into, and the cluster's own fixture chain
  (`d3_binding_dependent_arm` -> `d8f_compile`) drives the whole compile
  pipeline and reads hub-owned counters from `mod.rs` -- cross-domain by
  construction regardless. **Lands in
  `positional_candidate_settlement.rs`**, not re-homed.

**Net finding, a genuine refinement of the prior closure's own framing,
not a re-derivation of the same number:** zero of the five named class-3
clusters have a clean single-domain re-home. The prior closure's own
"~40-55 class-3, candidates to re-home with domain" framing implied
several would qualify; tracing each to its actual declaration site (not
assumed from the mutation-enum naming pattern alone) finds every one is
either hub-owned or cross-domain by its own fixture chain. This does not
change AC-5 (every one of these stays class-4-shaped in spirit and lands
in an integration module, never converted into a domain test) -- it
changes only which document states the disposition and why.

### Flagged for the Architect's own judgment -- genuine ambiguity, not
### papered over

- **Module 3's own internal seam is the most judgment-laden call.**
  `specialization_binding.rs` (5,826 lines) merges two adjacent but
  distinguishable concerns -- upstream marker/generated-context creation
  vs. downstream composed-call binding/causal-edge resolution on the
  same composed call. Kept as one module for comfortable size margin;
  splitting into two (roughly 2,130/3,696) is equally defensible if a
  finer separation is wanted, though `source_frame_bridge.rs` (5,909)
  could not absorb any overflow from a split without approaching the
  ceiling itself.
- **The `d5_c2_*`/`d5_c4_*`/`d5_the_*` cluster (placed in
  `host_call_carrier.rs`) could plausibly sit in `recursor_fusion.rs`
  instead** -- these exercise checked-call closeout / mutual-recursion-
  group axis behavior, and their fixtures build call-seam "frame
  carriers," which is why they landed with the host-call bucket, but
  the boundary between the two buckets here is soft, not sharp.
- **`source_frame_bridge.rs` (5,909 lines) has the least headroom** of
  the five proposed modules -- worth a second look at D1-drafting time
  if any of its tests are later found misclassified.
- **Not individually body-read: all ~150+ of the class-4-shaped
  population.** Classification rests on a complete import-provenance
  census (every test, not sampled) corroborated by ~25-30 spot body-
  reads distributed across every bucket, one of which caught and fixed
  a real misclassification. A full per-test re-audit at `D1`-drafting
  time is warranted, the same limitation the prior closure's own strand
  3 stated for its own classification pass.

**This is the design judgment; no code moves are in this deliverable.**
Handing back for the Architect's vote on the module boundaries above
(both the five-module structure and the two flagged soft boundaries)
before any `D1` cut executes.

## Acceptance criteria -- the phase gates bind here, restated

- **`AC-1` — control.rs and every module this node creates land below 10k**,
  each recorded. This is where the operator's constraint is discharged for
  control.rs. **No move may CREATE OR ENLARGE any file past 10k** (`AC-4b` of the
  phase): a move that would is a finding to route, not a transfer to complete.
- **`AC-2` — test identity and DISCOVERY parity, before the mutation proof.** A
  before/after test-identity-and-discovery ledger for each affected build
  profile; a nonzero selected-test count executed directly; each relocated test
  discovered under the same cfg/profile at its new path. A test that silently
  stops being collected passes every mutation check that remains.
- **`AC-3` — byte-faithful transport, not a line-pairing aid.** For every moved
  test record old path, new path, and body/attributes/cfg/visibility identity.
  Permitted normalization: module declarations, imports/path qualification, and
  the relocation-provenance comment. **Any semantic hunk hard-stops the move.**
- **`AC-4` — the affected library and targeted test configurations compile**
  (`scripts/ken-cargo -p ken-runtime`, scoped; the workspace gate is CI's, never
  a local run — `COORDINATION §12`).
- **`AC-5` — class-4 controls stay class-4.** No relocated end-to-end control is
  rewritten into a single-domain mutation/domain test. The ledger states each
  moved test's class, unchanged across the move.
- **`AC-6` — no-regression, in CI.** Targeted local validation only.

## Banned scope

- **No semantic change of any kind**, and **no production-code change** — this is
  a test-tree cut. An exposed behavioural dependency stops the move and returns
  for a ruling.
- **No line-count-driven extraction.** The seam is integration-test ownership
  scope with a 10k ceiling, not equal-sized files. A "control_part_2" split by
  size fails the frame's intent even if every child lands under 10k.
- **No converting a class-4 end-to-end control into a domain test** to give it a
  home. If a control has no coherent integration-test scope, it stays in the
  residual control.rs.
- **No widened production visibility to make a test move compile.** If a symbol
  the test reaches must widen, that is a finding to route, not a move to
  complete.
- **The core.rs/mod.rs hub-struct-state question is NOT in scope** (see below).

## Contention

**Bound file: `cranelift_backend/lowering/core/tests/control.rs`** and the new
integration-test modules it spawns under `lowering/core/tests/`. Re-derive every
test by name at pickup, never by line offset (control.rs was 30099 at the item-18
merge and moves under this node itself). A concurrent lowering/emitter semantic
candidate touching `control.rs` holds this slice — check the intersection at
pickup, per the phase's durable contention rule.

## Sequencing

**`active` at filing**, released to the runtime ring as the phase's named
successor. `depends_on: [RT-BACKEND-SPLIT-CLOSURE]` (merged). **Runtime-leader
owns within-lane ordering** against the `docs/program/16` residuals and PX8 — this
node is operator-constraint work (a live 30k > 10k violation) and the phase's
open successor, but its priority against other runtime residuals is the ring's
call, not a forced kick.

> ### THE core.rs / mod.rs RESIDUAL IS A SEPARATE, LARGER QUESTION. Not here.
>
> [[RT-BACKEND-SPLIT-CLOSURE]] left core.rs (~13k) and mod.rs (~12k) over 10k as
> the **honestly-bounded no-fourteenth-owner limit** — the Architect ruled
> (evt_3y64s8k4y9e27) that reducing them further is a DIFFERENT axis: splitting
> the Lowering hub struct's own state per-concern, "a separate, larger design
> question this slice correctly declines to open unilaterally." **That axis is
> NOT this node** — this node closes control.rs only. Whether core.rs/mod.rs get
> a hub-struct-state decomposition or are accepted as a permanent residual is a
> design fork for the Architect and the operator; the Steward has parked it for
> operator input and it does not gate this node. RT-BACKEND-MODULE-SPLIT's phase
> record stays open until that fork is resolved AND control.rs lands below 10k.
