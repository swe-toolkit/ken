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
