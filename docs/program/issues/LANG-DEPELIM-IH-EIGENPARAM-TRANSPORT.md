---
id: LANG-DEPELIM-IH-EIGENPARAM-TRANSPORT
title: "D2b predecessor #3 (elaborator dependent-elim capability): transport a recursive induction hypothesis across an EIGENPARAMETER-DRIVEN index/context extension. The eliminator method construction reconciles the SCRUTINEE-index refinement but cannot type the RECURSIVE IH when the premise's target index is a constructor-field-derived value that introduces a fresh eigenparameter extending the motive-threaded context (a de Bruijn level / forcing context). Result: `recur ... child` at the extended context has no generated transport back to the base-context conclusion across the record-index match, so the kernel correctly rejects the ill-typed term (KernelRejected TypeMismatch). Extend the method construction to build that transport; close the CLASS, not the two FoKripke constructors. Architect classification evt_anzrvvzaccep."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-03. D2b design hard-stop #4 for V3-FO-EMBEDDING-ADEQUACY, classified by the Architect (evt_anzrvvzaccep) as an elaborator-capability predecessor in crates/ken-elaborator/src/elab.rs — NOT a kernel change (the kernel correctly refuses an ill-typed term; the fix is where soundness converts, the elaborator, which must PRODUCE a well-typed transport that typechecks against the unmodified kernel), NOT source-compensable (all four constructor-local case helpers already elaborate standalone; the failure is in the eliminator-generated `recur ... child` application, which no spelling inside the helper body reaches — the HS3 source-compensation ban holds, FoKripke consumes the fixed elaborator unchanged), NOT an architecture-B refutation. §1b (Architect): HS4 is entry #1 of D2b's elaborator-capability symptom class (HS1-HS3 were FoKripke design/semantics stops, so no 3rd-entry structural-closure trigger fires yet); RELATED to the landed LANG-RECORD-INDEX-SIGMA-CLOSURE family (both dependent-elim-over-record-indexed gaps) but a DISTINCT predicate — SIGMA-CLOSURE keys on the index VALUE's type-structure (project a record/sigma equality into leaves), HS4 keys on a recursive IH crossing an eigenparameter-driven CONTEXT EXTENSION; SIGMA-CLOSURE is necessary-but-not-sufficient (its leaf projection is on-path for the record-valued FokSequent but does not transport the IH across the extension). §1a: HS4 discharged as a self-ruled stop, no research advisory owed (next §1a trigger = HS6). Elaborator coordinates measured at origin/main 0d96ecc40 (the Architect's cited line numbers were from the D2b hold base eac970705, which predates the landed SIGMA-CLOSURE work and drifts ~900 lines); re-measure at your build SHA (D0)."
---

> # D2b predecessor #3. Elaborator dependent-elimination capability, crates/
> # ken-elaborator/src/elab.rs ONLY. A completeness fix (the kernel is
> # unmodified and correctly rejects the current ill-typed term); the elaborator
> # must generate a well-typed recursive-IH transport. Close the CLASS.
>
> D2b (V3-FO-EMBEDDING-ADEQUACY) is HELD at eac970705 (WIP 97477903e preserved)
> until this predecessor lands. On landing the language ring rebuilds
> architecture B's Forall arms against the fixed elaborator, produces the D2b
> candidate, and the Architect is the required soundness reviewer on it.

## One-line objective

Extend the dependent-eliminator method construction so a recursive premise whose
target index is a constructor-field-derived value that introduces a FRESH
eigenparameter (e.g. `subst0 body (Param eigen)`) yields an induction hypothesis
whose type is transported/aligned with the refined motive index at the
conclusion — converting the current `KernelRejected(TypeMismatch)` into a
well-typed `recur ... child` term the (unchanged) method body consumes. Close
the whole capability class, not the two FoKripke Forall constructors. This
unblocks D2b past hard stop #4.

## The capability gap and the discriminator that pins it (Architect evt_anzrvvzaccep)

From the language ring's four-arm localization: the whole-sequent-motive driver
elaborates Init and Imp but deterministically `KernelRejected(TypeMismatch)` on
BOTH ForallWorldRight and ForallObjRight, identically. Imp is ALSO recursive at a
changed, field-derived index (`append gamma p` / `set_nth delta right q`) — yet
it PASSES. So the gap is NOT "recursive premise at a changed index" (that would
red Imp too). It is specifically:

**The elaborator cannot transport a recursive IH whose motive is instantiated at
an EIGENPARAMETER-DRIVEN extension of the stateful context the whole-sequent
motive threads (the de Bruijn level / forcing context).** Imp's child needs no
new parameter -> same level context -> the identity refinement carries it.
Forall's child lives under an extended level context (`fok_level_set levels
eigen`) -> the IH result at the extended context has no generated transport back
to the base-context conclusion across the `FokSequent` record-index match.

## Grounded cause (Architect, fresh grounding at the held object + elab.rs)

`FokDerivation` (read from the held object `97477903e:catalog/packages/Tooling/
Verification/FoKripke.ken`, ~lines 1092-1145): `data FokDerivation : FokSequent
-> Type`, a SINGLE record-valued index (`FokMkSequent gamma delta`). FokDerivInit
has no recursive premise; FokDerivImpRight one recursive premise at a
field-derived sequent (append/set_nth); FokDerivForallWorldRight /
FokDerivForallObjRight one recursive premise at `FokMkSequent gamma (set_nth
delta right (fok_subst0_form body (FokQParameter eigen)))` — the child index
substitutes a FRESH eigenparameter and carries the freshness guard
`fok_sequent_mentions_parameter ... eigen = False`. All conclude at
`FokMkSequent gamma delta`.

Dependent-elim machinery in `crates/ken-elaborator/src/elab.rs` (line numbers
verified at origin/main 0d96ecc40 — they DRIFT from the Architect's eac970705
citation; re-measure at D0):
- `motive_index_premises` (:3580)
- `method_index_premise_pairs` (:3624)
- `build_index_type_cong` (:3727)
- `refine_branch_goal` (:3861)
- `install_index_refinements` (:4065)
- `finalize_refined_body` (:4184) — sentinel rebasing `depth + premise_count - 1
  - slot` inside it.
This machinery builds transport for the SCRUTINEE-index refinement (abstract
motive index <-> conclusion index). The gap is in the recursive-IH typing of the
eliminator methods, at the boundary where `recur ... child` is passed to the
(already-elaborating) Forall helper.

This is NOT the multi-index `index_domain_mentions_prior_index` bail (:4274,
called at :3592/:3636) — that cannot fire on the single-index `FokDerivation`.
Exact construction pin is the language ring's localization (source bytes
152693-154670 of the held FoKripke); the classification does not depend on the
line.

## Completeness gap (false-reject), NOT a soundness hole

`KernelRejected(TypeMismatch)` is completeness-side: the kernel correctly refuses
an ill-typed term. The elaborator only CONSTRUCTS the elaborated dependent-match
term; the generated-elim path re-runs `kernel_infer` on the zonked assembled term
(the soundness net, unchanged). Building a correct recursive-IH transport lets
the elaborator produce a term the kernel already accepts; a wrong transport is
still rejected by the kernel. Kernel gate untouched -> elaborator-only, `gate:
none`, no operator authorization, no TCB touch. This is NOT a kernel seat — do
not release kernel.

## Deliverables (Architect design direction — component-level, not prescriptive)

Extend the eliminator method construction so a recursive premise whose target
index is a constructor-field-derived value that introduces a fresh eigenparameter
yields an IH transported/aligned with the refined motive index at the conclusion.
The convoy already reconciles the scrutinee-index refinement; the missing piece
is the RECURSIVE IH, typed at the premise's own field-derived index. Build the
transport carrying `M child_seq` to what the method body consumes:
- REUSE `build_index_type_cong` and the landed SIGMA-CLOSURE leaf-projection
  (`project_generated_index_equality_leaves`) — both on-path for the
  record-valued `FokSequent`.
- ADD the eigenparameter / context-extension transport the discriminator proves
  is missing (the piece SIGMA-CLOSURE's value-structure projection does not
  supply).
- Keep `finalize_refined_body`'s subtraction arithmetic underflow-safe as premise
  counts change (the direct-driver overflow the ring saw at the sentinel rebase
  was a symptom of that count, not a second defect).

The exact carrier is the implementer's design (T1); the frame does not prescribe
the term. No FoKripke source workaround (D2b's banned-source-compensation
constraint); the `fok_target_soundness_at` Forall arms stay on the held D2b
branch and rebuild against the fixed elaborator after this lands.

## Acceptance criteria (Architect's five; predicate form, controls named)

1. **AC-NON-REGRESSION (non-extending recursive arms).** Init and Imp continue
   to elaborate — the fix does not disturb the recursive arm that needs no new
   parameter. Control: the existing Init/Imp elaboration stays green.
2. **AC-FORALL-ELABORATES (zero trusted-base delta).** The Forall arms elaborate;
   `fok_target_soundness_at` completes with ZERO trusted-base delta — no axiom,
   postulate, primitive, kernel, relation, or statement change (trusted-base
   census before == after).
3. **AC-DISCRIMINATING-CONFORMANCE (generic, not FoKripke-specific; verdict must
   FLIP).** A SMALL inductive family — a record/derived index plus a recursive
   constructor introducing a fresh eigenparameter that extends a motive-threaded
   context, plus a dependent eliminator that must transport the IH across it —
   (a) elaborates AFTER the fix and (b) REDS BEFORE it (a per-capability reddening
   control: the conformance verdict FLIPS). Paired with a non-extending sibling
   arm that already passed BEFORE the fix (a non-degenerate pair, so the fix is
   not a blanket-accept). This is a conformance case, not a FoKripke fixture.
4. **AC-CLASS-CLOSED (not two-constructor-scoped).** The fix handles ANY recursive
   IH crossing a field-derived eigenparameter / context extension — not a special
   case for `FokDerivForall*`. A fix that closes the named counterexample without
   closing the class is a defect: the AC-3 generic family is the evidence the
   class (not the instance) is closed.
5. **AC-NO-SOURCE-COMPENSATION.** FoKripke consumes the fixed elaborator
   UNCHANGED — the D2b rebuild adds no source workaround to `FoKripke.ken` to
   compensate for the elaborator; the held statement is unchanged.

## Contention check

Production touch: `crates/ken-elaborator/src/elab.rs` (the eliminator method
construction — `method_index_premise_pairs` / the recursive-IH typing +
`build_index_type_cong` reuse + `finalize_refined_body` arithmetic) plus per-case
test fixtures including the AC-3 generic conformance family. Lane 2 (language).
No other active lane touches `elab.rs` (lane 1 runtime = ken-runtime + rt_parity
fixtures; lane 3 foundation = catalog/). The held D2b branch carries only
`FoKripke.ken` edits (WIP 97477903e) — no collision; D2b rebases onto this after
it lands. The landed SIGMA-CLOSURE machinery it reuses is already on main
(2462be787), so `depends_on: []` — immediately buildable on current main.

## Capability tier: T1

The core change may be compact, but the review turns on a soundness argument at
the kernel-adjacent index-refinement / recursive-IH-typing path (does the
generated transport type-align correctly, or does it over-accept?) AND on the
class-closure judgment (does the fix close the capability class, or only the two
FoKripke constructors?). That reasoning — plus the Architect's required
pseudocode-level soundness read of the IH-typing/convoy change — is the
deliverable, not the diff.

## Gate, reviewers, sequencing

`gate: none` (no TCB touch — the kernel is unmodified and independently checks the
generated term; no operator authorization). Required reviewers (Architect ruling
evt_anzrvvzaccep): **Architect** (soundness — the T1 read of the IH-typing/convoy
change; the elaborator generates a kernel-checked transport) + **CV**
(conformance — the AC-3 discriminating case). crates/-only -> Architect's domain,
no Librarian. Language QA does the lane's standard mechanics/build review. Steward
routes M1-M4; lieutenant M5-M9. D2b stays HELD at eac970705 (WIP 97477903e) until
this lands; the Steward EXPLICITLY re-releases D2b only after — the held Forall
arms rebuild against the landed elaborator fix and continue the unchanged
adequacy theorem.
