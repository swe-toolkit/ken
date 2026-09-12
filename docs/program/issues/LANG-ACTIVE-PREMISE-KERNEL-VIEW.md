---
id: LANG-ACTIVE-PREMISE-KERNEL-VIEW
title: "Elaborator contextual kernel-query boundary for active result-refinement premises: a live premise binder can occur in an elaborated term/type while absent from the Context of the immediate kernel judgment, so a well-typed dependent-match program is KernelRejected (TypeMismatch/VarOutOfScope). Fix = a validated ActivePremiseKernelView boundary that makes the active-frame premise domains present, in a disposable validation shadow, at every kernel-query site reachable under a live result refinement (the active-frame-reachable subset of the 20 check + 44 infer sites). Structural completeness capability; elaborator-only, kernel/TCB untouched."
status: active
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
> # CURRENT BLOCK (NOT a Steward-release gap): the language-implementer is
> # blocked on an Architect inference-shadow ruling. The TERM-provenance census
> # (the last Architect block: add the active-result-refinement if-condition
> # surface control + repair elaborate_if_condition, then redo the census at TERM
> # provenance) found an INDEPENDENT make_if_elim classifier sentinel leak after
> # the if-condition fix. WIP 6ee74ac1c is clean. The immediate unblock is that
> # Architect ruling; this node's re-kick re-homes the work and continues under
> # it, it does not restart from zero.
> #
> # SEAT TIER: T1 (structural elaborator capability, soundness-adjacent). Verify
> # the language-implementer stands on a T1 seat before the kick.
> #
> # Reviewers: Architect (required soundness reviewer for this class) + Language
> # QA; standing Adversary hunt independent -> Steward M1-M4 -> lieutenant. This
> # is a CODE merge (M8/M8a Adversary applies).

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
