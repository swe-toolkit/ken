---
id: LANG-RESULT-TRANSPORT-SIGMA-SWEEP
title: "Sibling-consumer sweep of the record-index Sigma closure: transport_recursive_group_call_result (elab.rs ~:1934, UNCHANGED by LANG-RECORD-INDEX-SIGMA-CLOSURE) consumes the SAME generated record-index premise as the three consumers that WP made Sigma-aware, but still via the whole-Eq path (try_reindex_cast -> build_index_type_cong -> J over the raw sentinel), NOT the new project_generated_index_equality_leaves walker. A recursive-group fn whose result type is indexed by a COMPONENT of the record (not the whole record) reaches subst_term_generalize, finds no whole-record term, and silently installs no per-component refinement — the exact completeness gap the parent WP closed in the other consumers, left in the result-transport sibling. CONFIRM-OR-REFUTE reachability, then fix-with-the-walker or record EXCLUDED with a cited reason."
status: draft
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Adversary M8 hunt on LANG-RECORD-INDEX-SIGMA-CLOSURE (merged 4af6e16f4, PR #3255), verdict NO OBJECTION / clean plus ONE bounded observation (evt_625k42qvwjdqz), Steward triage 2026-09-03. The observation is grounded in the parent WP's OWN AC-CLASS-CLOSED (which required EVERY elaborator consumer of a record-index equality that peels the Eq head to handle the Sigma-decomposition uniformly): the audit's must-classify set named install_hidden_result_variable_refinements but NOT its sibling transport_recursive_group_call_result, which consumes the same premise. Direction is COMPLETENESS-ONLY: the generated-elim assembled term stays kernel_infer-re-derived, so a missing refinement is a silent no-op or a kernel-rejected term (false-reject), never an over-acceptance. NOT a soundness hole, NOT a hard stop, NOT a blocker. Reachability of a component-indexed recursive-group result is UNCONFIRMED (no executed repro — native/cargo blocked by COORDINATION section 12); the language ring knows whether component-indexed recursive-group results occur and can confirm or refute cheaply."
---

> # QUEUED bounded finding (Steward triage), NOT released. Completeness-only,
> # kernel-backstopped, reachability unconfirmed. It is the
> # fourth-consumer sibling of the just-closed {Eq,Sigma,Top}
> # class ([[LANG-RECORD-INDEX-SIGMA-CLOSURE]]).
>
> The parent WP closed the record-index Sigma-decomposition class at three
> consumers (refine_branch_goal, install_index_refinements,
> install_hidden_result_variable_refinements) via one shared walker and asserted
> the class closed. The Adversary's post-merge M8 found a consumer of the same
> premise that the audit did not classify. This node records that gap so it is
> not lost; it is non-blocking and its confirm/refute is naturally exercised by
> the live D2b work (V3-FO-EMBEDDING-ADEQUACY), the exact consumer the closure
> was built to unblock.

## The observation (Adversary evt_625k42qvwjdqz; coordinates DECAY — re-measure at your build SHA)

`transport_recursive_group_call_result` (elab.rs ~:1934 on `f01266423`; the
Adversary cited ~:1943) was NOT changed by the parent WP. It pushes a result
refinement with the same arguments as the hidden-result push
(`weaken(scrut_ty,n)`, concrete, `weaken(scrut_core,n)`, same premise slot) but
still routes through the whole-`Eq` path — `try_reindex_cast` ->
`build_index_type_cong` -> `J` over the raw sentinel — rather than the new
`project_generated_index_equality_leaves` walker (elab.rs ~:1394).
`hidden_group_result_refinement` (~:3005 per the Adversary) does not gate on an
indexed (non-record) family, so a record scrutinee reaches this path.

- **The tested case works:** the parent WP's
  `hidden_result_refinement_handles_a_reducible_record_scrutinee` proves a
  WHOLE-record-indexed result (`HiddenOut : HiddenIx -> Type`) — whole-record
  `subst_term_generalize` finds the whole record term, whole-`Eq` `J`
  typechecks.
- **The un-swept case:** a recursive-group fn whose result type is indexed by a
  COMPONENT of the record (e.g. `HiddenOut : Nat -> Type` at `proj1`). There
  whole-record `subst_term_generalize` finds nothing -> silent no-op -> no
  per-component refinement installed. Coverage today is whole-record-indexed
  results only.

## Direction and disposition

- **Completeness-only, no soundness risk.** The kernel backstop
  (`kernel_infer` re-derivation of the assembled generated-elim term, the
  `elab.rs:4785` net the parent WP left unchanged) means a missing or wrong
  component refinement is a false-reject, never an over-acceptance. So this is a
  QUEUED coverage/audit-completion item, not a gate on anything landed.
- **Confirm-or-refute first, do not fix blind.** The language ring — which is
  currently in this exact code on D2b — determines whether a component-indexed
  recursive-group result is REACHABLE (in FoKripke or the corpus). Two valid
  outcomes, mirroring the parent WP's AC-CLASS-CLOSED audit shape:
  - **Reachable:** apply the shared `project_generated_index_equality_leaves`
    walker to `transport_recursive_group_call_result` exactly as the parent WP
    applied it to the other three consumers, and pin it with the parent's
    fixture shape (component-install red, whole-record `J` still green, a
    genuine differential on the walker diagnostic).
  - **Unreachable / out-of-class:** record it EXCLUDED with a cited reason and
    its current test home — a valid audit outcome, a silent omission is not
    (the parent WP's own standard).

## Relationship to the live lane-2 work (why this is NOT released as its own WP now)

The parent closure exists to unblock D2b (`V3-FO-EMBEDDING-ADEQUACY`, active on
the language ring). If D2b's `embedding_adequacy` proof reaches a
component-indexed recursive-group result, this gap fires as a real false-reject
INSIDE the live work — at which point it folds into D2b (or hard-stops to the
Steward for release as the fix here). If D2b completes without reaching it, the
case is unreached and this node stays queued behind the lane. Either way the
finding is durable and the language ring watches for it during D2b; it is not a
separate release competing with the lane.

## Gate

`gate: none` (no TCB touch — elaborator completeness on a kernel-backstopped
path). If released: Architect (required soundness reviewer for this class, as on
the parent) + language QA, standing Adversary hunt independent. Steward routes
M1-M4.
