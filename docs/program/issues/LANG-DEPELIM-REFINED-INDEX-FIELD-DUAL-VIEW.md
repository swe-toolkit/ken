---
id: LANG-DEPELIM-REFINED-INDEX-FIELD-DUAL-VIEW
title: "D2b predecessor #3 (elaborator dependent-elim capability): give the eliminator method construction a DUAL VIEW of a branch field whose declared type depends on the refined (whole) record index. `install_index_refinements` capability-1 (`try_reindex_cast`) retypes such a field to the OUTER refined-index view for branch-source consumption, and a local helper that consumes the field at its constructor-local type then sees `KernelRejected(TypeMismatch)` (Expected the constructor-local record-index type, Found the outer refined-index type). The field must be available in BOTH views — the outer-refined-index view (goal/source alignment) and its original constructor-local view (local-helper consumption) — with the reindex cast (which already transports one direction) carrying between them at the helper boundary. Close the CLASS (any constructor-local field dependent on the refined record index), not the two FoKripke Forall constructors. Architect classification evt_anzrvvzaccep, CORRECTED evt_14xp6j8dqt9ky (extension predicate withdrawn; location/lane/reviewers stand)."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-03. D2b design hard-stop #4 for V3-FO-EMBEDDING-ADEQUACY. Classified by the Architect (evt_anzrvvzaccep) as an elaborator-capability predecessor in crates/ken-elaborator/src/elab.rs — NOT a kernel change (the kernel correctly refuses an ill-typed term; the fix is in the elaborator, which must PRODUCE a well-typed term that typechecks against the unmodified kernel), NOT source-compensable (the HS3 source-compensation ban holds; FoKripke consumes the fixed elaborator unchanged), NOT an architecture-B refutation. The Architect's ORIGINAL causal predicate (`IH transport across an eigenparameter-driven context extension`) was FALSIFIED by the language ring's D0 generic 2x2 grid (WIP d43fb633a, 4/4, no production edit) and WITHDRAWN by the Architect (evt_14xp6j8dqt9ky): context extension WITHOUT a dependent guard already elaborates (row 2), and a constructor-local field dependent on the refined record index rejects even WITHOUT extension (row 3). Extension was a confound (the Forall arms both extend the context AND carry a whole-conclusion-dependent field; Imp does neither). What STANDS from the original ruling: the location (elaborator, install_index_refinements capability-1 / try_reindex_cast), the lane (2, language), and the reviewers (Architect + CV). §1b (Architect): still DISTINCT from the landed LANG-RECORD-INDEX-SIGMA-CLOSURE — SIGMA-CLOSURE closes an index VALUE's type-structure into leaf equalities; THIS preserves a branch FIELD's type across the two index views (related family, not bundled). §1a: HS4 discharged self-ruled, no research advisory owed (next §1a trigger = HS6); the D0 correction is a re-classification WITHIN HS4, not a new stop. Elaborator coordinates measured at origin/main f40179963 (below); re-measure at your build SHA (D0)."
---

> # D2b predecessor #3. Elaborator dependent-elimination capability, crates/
> # ken-elaborator/src/elab.rs ONLY. A completeness fix (the kernel is
> # unmodified and correctly rejects the current ill-typed term); the elaborator
> # must give a refined-index-dependent branch field a DUAL VIEW so the
> # local-helper consumption typechecks. Close the CLASS.
> #
> # CORRECTED FRAME (Steward, 2026-09-03, from Architect evt_14xp6j8dqt9ky). The
> # original id LANG-DEPELIM-IH-EIGENPARAM-TRANSPORT and its "eigenparameter /
> # context-extension" causal predicate were FALSIFIED by D0 (WIP d43fb633a) and
> # withdrawn. Do NOT implement the IH-across-level transport — it is both
> # unimplementable (no equality relates arbitrary `levels` to `level_set levels
> # eigen`) and unnecessary (extension alone already elaborates). The true cause
> # and the corrected ACs are below.
>
> D2b (V3-FO-EMBEDDING-ADEQUACY) is HELD at eac970705 (WIP 97477903e preserved)
> until this predecessor lands. On landing the language ring rebuilds
> architecture B's Forall arms against the fixed elaborator, produces the D2b
> candidate, and the Architect is the required soundness reviewer on it.

## One-line objective

Extend the dependent-eliminator method construction so a branch field whose
declared type depends on the refined (whole) record index is available in BOTH
the outer-refined-index view (for goal/branch-source alignment) and its original
constructor-local view (for local-helper consumption) — the reindex cast that
already retypes the field one direction transports it back at the helper
boundary — converting the current `KernelRejected(TypeMismatch)` into a
well-typed term the (unchanged) method body consumes. Close the whole capability
class, not the two FoKripke Forall constructors. This unblocks D2b past hard
stop #4.

## The corrected cause and the discriminator that pins it

The language ring's D0 (generic 2x2 grid, WIP `d43fb633a`, 4/4, no production
edit) isolates the rejection to ONE axis, refuting the original extension
predicate:

1. Non-extending recursive arm, no dependent guard: elaborates.
2. Recursive arm whose call changes the motive-threaded context (`eigen_extend
   levels Zero`), no dependent guard: **elaborates BEFORE any fix** — so context
   extension is NOT the cause.
3. Recursive arm at the SAME level context carrying `guard : EigenGuard
   (EigenMkSequent gamma [right])` passed to the constructor-local helper:
   deterministically `KernelRejected(TypeMismatch)` — so the dependent field on
   the refined record index IS the cause.
4. Dependent guard PLUS changed context: byte-identical mismatch to row 3.

The rejection is readable: Expected `EigenGuard (EigenMkSequent gamma [right])`,
Found `EigenGuard outer_sequent`. `install_index_refinements` capability-1 has
retyped the constructor field to the OUTER refined record index for branch-source
consumption; the local helper needs the SAME field at its constructor-local
declared type. The local view was replaced, not preserved/transported back. Rows
3/4 being byte-identical is the proof that extension is inert and the dependent
field is the whole cause.

This maps onto FoKripke's unique Forall-only field. `FokDerivForallWorldRight` /
`FokDerivForallObjRight` carry `h_fresh : Equal Bool (fok_sequent_mentions_parameter
(FokMkSequent gamma delta) (FokQParameter eigen)) False` — a field whose type
mentions the WHOLE conclusion sequent `(FokMkSequent gamma delta)`, i.e. the
refined record index. `FokDerivImpRight`'s equalities are all projections
(`fok_nth_form delta right ...`), never the whole sequent — so Imp lacks exactly
the whole-conclusion-dependent field, which is why Forall-vs-Imp could not
separate the confound. (Grounded by the Architect at `97477903e:catalog/packages/
Tooling/Verification/FoKripke.ken`, ~1092-1145.)

## Corrected causal predicate (Architect evt_14xp6j8dqt9ky)

**Dependent elimination loses the CONSTRUCTOR-LOCAL type of a branch field whose
declared type depends on the refined (whole) record index.**
`install_index_refinements` capability-1 (`try_reindex_cast`) retypes such a field
to the OUTER refined-index view for branch-source consumption, but a local helper
that consumes the field at its constructor-local type then sees a `TypeMismatch`.
The **missing capability is DUAL-VIEW**: the field must be available in BOTH the
outer-refined-index view (for goal/source alignment) and its original
constructor-local view (for local-helper consumption), with the reindex cast —
which already exists in one direction — transporting between them at the helper
boundary.

## Mechanism location (elab.rs coordinates at f40179963 — re-measure at D0)

The Architect's cited line numbers (`install_index_refinements` :2953,
capability-1 :3001-3024) were from an older base and DRIFT; measured at
origin/main `f40179963`:
- `install_index_refinements` (:4065) — capability-1 is the branch-field retyping
  that calls `try_reindex_cast`.
- `try_reindex_cast` (:3805) — retypes a branch field via `var_refinements` to a
  `new_ty`; this is the one-direction cast the dual-view fix generalizes.
- `build_index_type_cong` (:3727), `motive_index_premises` (:3580),
  `method_index_premise_pairs` (:3624), `finalize_refined_body` (:4184) — the
  surrounding convoy/refinement machinery.
This is NOT the multi-index `index_domain_mentions_prior_index` bail — that
cannot fire on the single-index `FokDerivation`. Re-measure all coordinates at
your D0 build SHA; the WIP `d43fb633a` grid was cut at `0d96ecc40`.

## Completeness gap (false-reject), NOT a soundness hole

`KernelRejected(TypeMismatch)` is completeness-side: the kernel correctly refuses
an ill-typed term. The elaborator only CONSTRUCTS the elaborated dependent-match
term; the generated-elim path re-runs `kernel_infer` on the zonked assembled term
(the soundness net, unchanged). Giving the field a correct dual view lets the
elaborator produce a term the kernel already accepts; a wrong cast is still
rejected by the kernel. Kernel gate untouched -> elaborator-only, `gate: none`,
no operator authorization, no TCB touch. This is NOT a kernel seat — do not
release kernel.

## Deliverables (Architect design direction — component-level, not prescriptive)

Extend the eliminator method construction so a branch field whose declared type
depends on the refined record index keeps BOTH views:
- PRESERVE the field's constructor-local-typed view alongside the
  outer-refined-index view that capability-1 already installs — do not replace
  one with the other.
- At the local-helper boundary, transport between the two views using the
  existing reindex cast (`try_reindex_cast` already carries the outer direction;
  the missing piece is making the constructor-local view available/reachable when
  the helper consumes the field at its declared type).
- Keep `finalize_refined_body`'s subtraction arithmetic underflow-safe as premise
  counts change (the direct-driver overflow the ring saw at the sentinel rebase
  was a symptom of that count, not a second defect).

DISCARD the withdrawn IH-across-level-transport direction entirely — there is no
equality between arbitrary `levels` and `level_set levels eigen`, and the generic
recursive call at the changed context already elaborates when the dependent guard
is absent, so such a transport would be both unjustified and unnecessary.

The exact carrier is the implementer's design (T1); the frame does not prescribe
the term. No FoKripke source workaround (D2b's banned-source-compensation
constraint); the `fok_target_soundness_at` Forall arms stay on the held D2b
branch and rebuild against the fixed elaborator after this lands. This is a
DISTINCT capability from the landed SIGMA-CLOSURE (which closes an index VALUE's
type-structure into leaf equalities) — do not assume its leaf-projection is the
mechanism here; this preserves a branch FIELD's type across the two index views.

## Acceptance criteria (Architect's five, AC-3 corrected to the guard axis)

1. **AC-NON-REGRESSION (non-guard arms).** Init and Imp continue to elaborate —
   the fix does not disturb an arm that carries no refined-index-dependent field.
   Control: the existing Init/Imp elaboration stays green.
2. **AC-FORALL-ELABORATES (zero trusted-base delta).** The Forall arms elaborate;
   `fok_target_soundness_at` completes with ZERO trusted-base delta — no axiom,
   postulate, primitive, kernel, relation, or statement change (trusted-base
   census before == after).
3. **AC-DISCRIMINATING-CONFORMANCE (generic; varies the GUARD axis; verdict must
   FLIP).** The discriminating conformance case must vary the DEPENDENT-GUARD axis
   — a constructor-local field whose type mentions the refined whole record index
   — INDEPENDENTLY of context extension. **Adopt the D0 grid `d43fb633a`
   (`crates/ken-elaborator/tests/dependent_match_eigen_guard_localization.rs`) AS
   the AC-3 fixture** (generic, not FoKripke-specific). Minimum bar: a
   non-degenerate pair on a shared shape — {guard present -> REDs BEFORE the fix,
   elaborates AFTER} paired with {guard absent, context changed -> already
   elaborates both before and after}. The "verdict must FLIP" binds the
   guard-present row; the guard-absent/extension-only row is the confound control
   proving the fix keys on the true axis, not on extension. This is a conformance
   case, not a FoKripke fixture.
4. **AC-CLASS-CLOSED (not two-constructor-scoped).** The fix handles ANY
   constructor-local field whose declared type depends on the refined record index
   — not a special case for `FokDerivForall*` by name. A fix that closes the named
   counterexample without closing the class is a defect: the AC-3 generic grid is
   the evidence the class (not the instance) is closed.
5. **AC-NO-SOURCE-COMPENSATION.** FoKripke consumes the fixed elaborator
   UNCHANGED — the D2b rebuild adds no source workaround to `FoKripke.ken` to
   compensate for the elaborator; the held statement is unchanged.

## Contention check

Production touch: `crates/ken-elaborator/src/elab.rs` (the eliminator method
construction — `install_index_refinements` capability-1 / `try_reindex_cast`
dual-view + `finalize_refined_body` arithmetic) plus per-case test fixtures
including the AC-3 generic conformance grid (`d43fb633a` already authored the
grid as `dependent_match_eigen_guard_localization.rs`). Lane 2 (language). No
other active lane touches `elab.rs` (lane 1 runtime = ken-runtime + rt_parity
fixtures; lane 3 foundation = catalog/). The held D2b branch carries only
`FoKripke.ken` edits (WIP 97477903e) — no collision; D2b rebases onto this after
it lands. `depends_on: []` — immediately buildable on current main (the D0 grid
was cut at `0d96ecc40`).

## Capability tier: T1

The core change may be compact, but the review turns on a soundness argument at
the kernel-adjacent index-refinement path (does the dual-view cast type-align
correctly at the helper boundary, or does it over-accept?) AND on the
class-closure judgment (does the fix close the capability class, or only the two
FoKripke constructors?). That reasoning — plus the Architect's required
pseudocode-level soundness read of the dual-view/reindex-cast change — is the
deliverable, not the diff.

## Gate, reviewers, sequencing

`gate: none` (no TCB touch — the kernel is unmodified and independently checks the
generated term; no operator authorization). Required reviewers (Architect ruling
evt_anzrvvzaccep, unchanged in the correction evt_14xp6j8dqt9ky): **Architect**
(soundness — the T1 read of the dual-view/reindex-cast change; the elaborator
generates a kernel-checked term) + **CV** (conformance — the AC-3 discriminating
grid). crates/-only -> Architect's domain, no Librarian. Language QA does the
lane's standard mechanics/build review. Steward routes M1-M4; lieutenant M5-M9.
D2b stays HELD at eac970705 (WIP 97477903e) until this lands; the Steward
EXPLICITLY re-releases D2b only after — the held Forall arms rebuild against the
landed elaborator fix and continue the unchanged adequacy theorem.
