---
id: LANG-ACTIVE-PREMISE-KERNEL-VIEW
title: "Elaborator contextual kernel-query boundary for active result-refinement premises: a live premise binder can occur in an elaborated term/type while absent from the Context of the immediate kernel judgment, so a well-typed dependent-match program is KernelRejected (TypeMismatch/VarOutOfScope). Fix = a validated ActivePremiseKernelView boundary that makes the active-frame premise domains present, in a disposable validation shadow, at every kernel-query site reachable under a live result refinement (the active-frame-reachable subset of the 20 check + 44 infer sites). Structural completeness capability; elaborator-only, kernel/TCB untouched."
status: merged
owner: language
size: L
gate: none
tier: T1
depends_on: []
blocks: [LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS]
github: null
origin: "Split by the Steward 2026-09-12 from LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS on the operator's Route-B ruling (2026-09-12, this session; Steward decision-request evt_8bxk6htpmt9k, recommendation B). LANG-TRANSPORT-SIGMA was released as a size-S route-(a) reuse (synthesize transport's premise as the result-index Sigma, feed the existing walker); a three-stop Architect respin chain proved route (a) alone does NOT close the defect. Architect §1b: stops #2/#3 and the intervening review blocks share ONE predicate — a live premise binder occurs in an elaborated term/type while absent from the Context of the immediate kernel judgment. Research (evt_4pzqt85p2zkyf) confirmed the only sound closure is STRUCTURAL — at the typing context, not per-consumer. The Architect ruled a validated ActivePremiseKernelView contextual kernel-query boundary governing the active-frame-reachable subset of 20 check + 44 infer kernel-query sites (design authority evt_5yb47cjdfyrvj / evt_7p4yf0944r59j; scope correction evt_7a42k68zw35cc). That is a T1 structural capability, not the S reuse the parent was framed and sized for — so it is its own precursor node and route (a) becomes a thin dependent S consumer. Re-measure the elab.rs anchors at pickup — they drift."
---

> # ROUTE-B PRECURSOR (operator ruling 2026-09-12, this session). READ FIRST.
> # This node OWNS the ActivePremiseKernelView contextual kernel-query boundary.
> # It absorbs the in-flight structural work: WIP 6ee74ac1c (built under the
> # parent node before the split) re-homes HERE. LANG-TRANSPORT-SIGMA-PREMISE-
> # SYNTHESIS becomes a thin dependent S consumer that lands AFTER this.
> #
> # RULING LANDED (Architect evt_7jgq7apjw7jcy, 2026-09-12) — the repair spec is
> # the "## Architect implementation ruling" section below (READ IT; it is the
> # authoritative, durable spec, superseding the in-thread post). The precursor
> # is UNBLOCKED. The make_if_elim VarOutOfScope is classified as an independent
> # CONSUMER of the already-researched hard stop #3, correctly re-housed by Route
> # B — NOT a new defect and NOT a new hard-stop count. WIP 6ee74ac1c is evidence
> # to RE-HOME (preserve its accumulated work + controls), NOT a merge candidate.
> # The language-leader owes the explicit T1 precursor implementation kick from
> # this landed node.
> #
> # SEAT TIER: T1 (structural elaborator capability, soundness-adjacent). Verify
> # the language-implementer stands on a T1 seat before the kick.
> #
> # Reviewers: Architect (required soundness reviewer for this class) + Language
> # QA; standing Adversary hunt independent -> Steward M1-M4 -> lieutenant. This
> # is a CODE merge (M8/M8a Adversary applies).

## Architect implementation ruling (evt_7jgq7apjw7jcy) — READ FIRST, repair spec

Grounded by the Architect at origin/main `e88e81097` (node blob `58c8303a`) and
the exact held WIP `6ee74ac1` (`elab.rs` blob `f857d166`). The WIP has 20 textual
`kernel_check` and 44 textual `kernel_infer` calls (incl. tests); its last commit
correctly makes `elaborate_if_condition` validate the original condition through a
disposable check shadow but gates that path on `result_refinements.is_empty()`,
while `make_if_elim` still calls raw `kernel_infer(cx.env, &cx.ctx, result_ty)`.
The `IfBox` witness is independent of condition validation: its condition is
closed `True`; inference substitutes the refined `cell` core into
`IfBox local <Cast>`, so `result_ty` contains sentinel `1099512676352`, and raw
inference at depth 7 refuses `VarOutOfScope`.

Classification: an independent CONSUMER, not an independent defect and NOT a new
hard-stop count — the already-researched hard stop #3, now correctly housed by
Route B. The shared predicate stands: **a logically live premise binder occurs in
an elaborated term or type while absent from the `Context` of the immediate
kernel judgment.** The earlier technical ruling stands; only its old
same-parent-WP packaging was withdrawn. Do NOT add a make_if_elim-specific
shadow, guess/thread a classifier, or infer a lambda/Pi wrapper.

### Ruled component — one compatibility adapter over the sentinel representation

```rust
struct ActivePremiseKernelView {
    context: Context,
    embedding: ActivePremiseEmbedding,
}

struct ActivePremiseEmbedding {
    original_len: usize,
    expanded_len: usize,
    original_to_expanded: Vec<usize>,
    expanded_sources: Vec<ExpandedBindingSource>,
    premise_to_expanded: HashMap<(usize, usize), usize>,
}

enum ExpandedBindingSource {
    Original(usize),
    Premise { sentinel_region: usize, premise_slot: usize },
}

fn active_premise_kernel_view(
    cx: &ElabCtx<'_>,
) -> Result<Option<ActivePremiseKernelView>, ElabError>;

fn kernel_check_current(
    cx: &ElabCtx<'_>, checked: &Term, expected: &Term,
) -> Result<(), ElabError>;

fn kernel_infer_current(
    cx: &ElabCtx<'_>, inferred: &Term,
) -> Result<Term, ElabError>;
```

Names may vary; responsibilities may not.

1. The ambient fast path is lawful exactly when
   `active_index_premise_frames.is_empty()`, NEVER from
   `result_refinements.is_empty()`. Active-frame current-context queries use the
   shared view.
2. Validate the complete frame plan before translating: unique regions;
   `install_depth <= original_len`; nondecreasing outer-to-inner install depths,
   retaining stack order at equal depths; every `ResultRefinement` names exactly
   one active frame and an in-range premise slot. Duplicate, nonnested, escaped,
   unknown, or overflowed authority refuses.
3. Construct one total bottom-relative embedding first. Insert each complete
   premise telescope at its `install_depth`, outer frames first and slots in
   declaration order: `Γ0,P0,Δ0,P1,Δ1,...`. Record both directions. Never
   recover a pair from proximity, counts, sentinel-range membership, or later
   subtraction.
4. Rebuild `Context` bottom-up. Relocate each premise domain against the exact
   expanded prefix preceding its slot and each original context-entry type
   against the prefix preceding that original binding. Context entries created
   after frame installation can contain sentinels and are part of the authority
   surface. A same/later-slot, later-frame, or outside-prefix dependency refuses.
   Do not bulk-weaken a finished context.
5. Use one binder-aware exhaustive traversal over every `Term` variant. At binder
   depth `d`, indices `< d` remain bound. For any free index, first canonicalize
   relative to `d`: an exact known `(sentinel_region, premise_slot)` maps only
   through `premise_to_expanded`; an ordinary original index maps by its
   bottom-relative binding identity. Unknown sentinels and ordinary out-of-scope
   indices refuse. Translate query term and expected type together after zonking.
6. `kernel_check_current` validates translated operands and returns only success.
   `kernel_infer_current` infers the translated original term, then
   inverse-translates its type through `expanded_sources`: original bindings
   return to original de Bruijn coordinates; premise bindings return to exact
   owner-local sentinels. Unowned/noninvertible output refuses. No contextual
   artifact becomes production core.

### make_if_elim disposition

Replace ONLY the raw classifier judgment with the shared
`kernel_infer_current(cx, result_ty)` path. Classify the result using the exact
inferred `Type(level)` vs `Omega(level)` result, preserving both kind and level.
`condition`, both branches, `result_ty`, the constructed motive, and returned
`Term::Elim` remain the original owner-local production terms. No all-active
lambda/Pi, translated `result_ty`, or contextual local escapes. The exact `IfBox`
witness must therefore differ only because its logically live premise is present
during the classifier judgment.

### Gateway closure

Replace `active_index_premise_validation_shadow`; do NOT keep parallel consumer
shadows. Branch-body validation, large-convoy base, completed generated-All
`Elim`, and `elaborate_if_condition` check original terms through
`kernel_check_current`; `make_if_elim` uses `kernel_infer_current`. Census all 20
`kernel_check` / 44 `kernel_infer` textual sites by call graph and TERM provenance
(active-frame reachability, context, every operand and reachable context-entry
type), then `gateway` or a structural proof of detachment. A fixed expected
type/local head does NOT exclude a site when another operand/context entry is
source-derived. Make raw imports visibly exceptional.

### Acceptance and causality

- Keep WIP 6ee74ac1's `IfCell` condition case. A natural `BypassCheck` once
  restores its condition-side failure; restore exact green.
- Add the exact `IfBox` case. `BypassInfer` once at the shared make_if gateway
  restores `VarOutOfScope { index: 1099512676352, depth: 7 }` at the inner-if
  span; restore exact green.
- On that same case, translate the query operand but omit relocation of the
  sentinel-bearing `boxed` lexical context-entry type; the later query must red.
  This proves whole-context relocation rather than named-term repair.
- Retain the two-frame nested surface case. Independently omit the outer frame
  and reverse frame insertion; each reds for its own reason and exact restoration
  greens.
- A private pair sends the same sentinel-bearing type shape through inference and
  distinguishes exact `Type(level)` from exact `Omega(level)`. A Pi/lambda
  surrogate or classifier guess must fail the pair.
- Negative/positive pairs cover duplicate region, nonnested depth, out-of-range
  slot, unknown sentinel in operand or context-entry type, forward premise
  dependency, ordinary out-of-scope variable, and noninvertible inferred output.
- Preserve cumulative value and type transport, current-state fallback,
  selected-position J, scoped single-constructor/non-indexed projection,
  multi-constructor refusal, nested lifecycle, large-convoy non-escape,
  generated-All final-Elim, whole-record/index, and DS5b controls.

### Fences

Elaborator only; kernel/TCB/spec unchanged. NO stable-local migration, new kernel
primitive, consumer-specific inference wrapper, classifier threading, lambda/Pi
inference surrogate, numeric sentinel authority, term-only/context-only
relocation, caught kernel error, escaped validation term, or outer-frame
finalization by an inner owner. Route-(a) remains a later thin consumer and is
NOT implemented as part of this precursor.

## The capability (Architect design authority)

A dependent match under an active result refinement installs premise binders
(sentinel regions with declared-order premise domains) that live in the
elaborated term/type but are ABSENT from the `Context` used by the immediate
kernel judgments the elaborator issues while checking that frame. The kernel
then rejects a well-typed program (`TypeMismatch`, or `VarOutOfScope` for an
outer sentinel) because it cannot see the premise that makes the term
well-formed. This is a COMPLETENESS defect (an over-strict false-reject); nothing
ill-typed is ever accepted and the kernel/TCB is untouched.

The ruled closure is structural, at the typing context rather than per-consumer:
an `ActivePremiseKernelView` boundary that, for every kernel-query site reachable
under a live result refinement, builds a DISPOSABLE validation shadow presenting
the active-frame premise domains (inner-through-outer weakening by exact context
growth, the frame's `wrap_premise_lams/pis_finalized` wrapping, zonk, then
kernel-check), validates against it, and returns only the current owner's
ordinary term — never the full shadow, so outer sentinels remain for their owner
to finalize. One-frame behaviour is byte-for-term identical to the pre-boundary
path. The boundary governs the active-frame-reachable subset of the 20 check + 44
infer kernel-query sites, censused at TERM provenance (each term operand and
relevant context entry classified), routing only genuinely sentinel-capable
immediate validations through the shadow.

## Scope and fences

Elaborator-only. NO kernel/TCB/spec change and NO new capability beyond the
boundary the Architect ruled. Reuse the existing `kernel_check`/`kernel_infer`
machinery; the validation shadow is disposable (validation-only), NEVER escapes,
and every production-term return is the original unchanged term. Census at TERM
provenance, not expected-type labels. Fail closed on duplicate regions, context
shrink below the install depth, or an out-of-telescope sentinel slot. Do not
broaden `finalize_refined_body`, scan numeric sentinel ranges, skip/catch kernel
errors, or finalize another frame's premise.

## Acceptance

- The reaching nested-surface fixtures compile through `ElabEnv::elaborate_file`/
  `elaborate_decl` (the compound dependent-match programs that currently
  false-reject, incl. outer+inner distinct active refinements and a
  recursive-group sibling returning a multi-index result); whole-record and
  whole-index controls stay green.
- A complete TERM-provenance census of every raw `kernel_check`/`kernel_infer`
  reachable under nonempty result refinements, with the included/excluded
  classification reported; only sentinel-capable immediate validations route
  through the shadow, and each consumer returns its original production term.
- Fail-closed controls: duplicate region, context shrink below install depth,
  out-of-telescope sentinel slot each refuse.
- Mutation-backed per-site controls, each applied alone and restored
  byte-identically: omit exactly one outer validation-shadow frame -> restore the
  exact outer-sentinel `VarOutOfScope`; drop the first cumulative VALUE commit
  while retaining its type commit -> restore the former kernel-backed false
  rejection; a consumer-specific mutation per repaired site (large-convoy shadow
  return, final generated-All `elim` validation, `elaborate_if_condition`,
  make_if_elim classifier) reds distinctly while positive neighbours keep the
  validator live.
- No kernel/TCB/spec change; native/interpreter unaffected. Scoped elaborator
  tests via `scripts/ken-cargo` (never `--workspace`); CI runs the full gate.
- A new structural refusal that needs production code beyond this boundary is a
  HARD STOP to the Steward + Architect, not a widening of this contract.

## History (symptom inventory; Architect respin chain)

Route (a) reuse was released 2026-09-11 (parent node, operator L2 item 9) and hit
a three-stop chain plus intervening review blocks, all sharing the one §1b
predicate above. Progression of exact SHAs under the parent branch
`wp/LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS`: `3895d14a` (route-(a) core) ->
`f83aec14` (cumulative-refinement + fallback respin) -> `7d602994` (active
index-premise frame stack + validation shadow) -> `393948fb` (large-convoy
non-escape + conditional final-Elim validation) -> if-condition surface control +
TERM-provenance census (last Architect block) -> current WIP `6ee74ac1c`, blocked
on the Architect ruling for the independent make_if_elim classifier sentinel
leak. Design authority: evt_79xy16y7fv5sx (original route-(a) design),
evt_4pzqt85p2zkyf (Research: structural closure is the only sound fix),
evt_5yb47cjdfyrvj / evt_7p4yf0944r59j (ActivePremiseKernelView ruling),
evt_7a42k68zw35cc (scope correction). Operator Route-B split ruling: 2026-09-12,
this session (Steward decision-request evt_8bxk6htpmt9k).
