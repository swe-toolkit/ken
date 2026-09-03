---
id: LANG-RECORD-INDEX-SIGMA-CLOSURE
title: "D2b predecessor #2 (elaborator structural closure): record-index-equality CONSUMERS in the elaborator handle the Eq head but not its observational Sigma-decomposition. install_index_refinements peels only Term::Eq and falls through on the Sigma-shaped record-index equality, installing no per-component refinement (gamma->g0), so a dependent-inversion body needs fok_nth_form @14 (outer gamma) where the constructor-declared type says @10 (local g0). Fix the consumer to recursively project the Sigma into per-component refinements, AUDIT every such consumer for the same Eq-only gap, and close the class with a per-site regression fixture. Section-1b predicate named by the Architect (evt_2ptgr3f2ef7c4)."
status: merged
owner: language
size: M
gate: none
tier: T1
depends_on: [LANG-GENERATED-INDEX-EVIDENCE-CLOSURE]
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-02. D2b hard-stop #2 for V3-FO-EMBEDDING-ADEQUACY, the CONSUMPTION-side sibling of the just-built synthesis-side predecessor LANG-GENERATED-INDEX-EVIDENCE-CLOSURE. After the Top-closure fix (synthesis) let D2b advance past HS1, the fold reached fok_invert_atomlike and false-rejected. Probe (language-implementer evt_4cjm6qrwccjvc, language-leader evt_5597m0qf9rzhn) confirmed case (a): elaborator-owned dependent-index rebasing, NOT a source scoping error and NOT a bare weakening omission. Architect ruling evt_5w65hdk73zp9b (provisional case-(a) route) then evt_2ptgr3f2ef7c4 (Section-1b predicate NAMED + structural-closure disposition). Elaborator-only, no FoKripke workaround (D2b's banned-source-compensation constraint), no operator authorization (kernel_infer backstop unchanged). Coordinates measured at origin/main e485a696c; they DRIFT on the held D2b branch acef50612. Architect count: D2b design hard-stop #2; §1a Research advisory fires at the 3rd, not triggered."
---

> # MERGED (Steward, 2026-09-03) — squash `4af6e16f4` on `origin/main`, full CI
> # green (PR #3255, exact `2462be787`). Blob-verified: the atomic
> # `project_generated_index_equality_leaves` {Eq,Sigma,Top} walker present in
> # `crates/ken-elaborator/src/elab.rs`. Architect soundness APPROVE
> # (`evt_44gxn8bk0rd9e`, sole crates/-only domain reviewer: completeness fix,
> # kernel_infer backstop intact) + Language QA APPROVE (`evt_5xrt562agm95e`,
> # reproduced the 6 closure fixtures + all 12 mutations); Decision
> # `dec_2pjd6e8mhaw0r`. This DISCHARGES the D2b HS2 predecessor — D2b
> # (V3-FO-EMBEDDING-ADEQUACY, held `acef50612`) re-releases to the language ring
> # for the Architect's resumed-proof re-review (Steward move).
>
> The consumption-side mirror of the synthesis-side {Eq,Sigma,Top} closure
> (LANG-GENERATED-INDEX-EVIDENCE-CLOSURE). One atomic leaf-plan walker projects
> the single retained Eq sentinel into per-component leaves at all three
> refinement consumers + the hidden-finish synthesis; producers byte-unchanged.

## One-line objective

Close the Section-1b class: elaborator CONSUMERS of a dependent-match
record-index equality must recursively handle its observational
Sigma-decomposition, not only the `Eq` head. Fix `install_index_refinements`
(the consumer that installs per-component index refinements), AUDIT every such
consumer for the same Eq-only gap, and pin each with a regression fixture. This
unblocks D2b (`V3-FO-EMBEDDING-ADEQUACY`) past hard stop #2.

## The Section-1b predicate (Architect evt_2ptgr3f2ef7c4)

Consumers of a dependent-match record-index equality in the elaborator each
handle the `Eq` head but fail to recursively handle its observational
SIGMA-DECOMPOSITION. Two entries, same structural gap, two different consumers of
the same record-index equality:

- Entry 1 (LANDED, [[LANG-GENERATED-INDEX-EVIDENCE-CLOSURE]]): `synth_refl_proof`
  descended a `Sigma` but its terminal vocabulary was stratified (`Top` not
  handled inside the Sigma recursion) — a false-reject on evidence SYNTHESIS.
- Entry 2 (this WP): `install_index_refinements` does not decompose the `Sigma`
  at all (`Eq`-only) — installs no per-component index REFINEMENT.

Each was locally correct, which is why the shared predicate was invisible until
the second entry. The synthesis side is closed; this WP closes the
consumption/refinement side and forecloses the class.

## Grounded cause (probe: language-implementer evt_4cjm6qrwccjvc; Architect-confirmed)

For method 0 of the `fok_invert_atomlike` dependent inversion,
`install_index_refinements` receives the generated whole-index equality
`Eq FokSequent (FokMkSequent g0 d0) (FokMkSequent gamma [goal])`, whose WHNF is
`Sigma (Equal (List FokForm) g0 gamma) (Equal (List FokForm) d0 [goal])`. Its
peel accepts only `Term::Eq`; the `Sigma` case falls through to the unreduced
whole-record triple. No constructor field type mentions that whole record, so
`try_reindex_cast` installs no `gamma -> g0` (or `[goal] -> d0`, or other
component) refinement.

Consequence at the kernel check: author-named `h_g` resolves correctly at `@4`,
but its constructor-declared type is `fok_nth_form @10 @8` (`@10` = local `g0`)
while the body needs `fok_nth_form @14 @8` (`@14` = outer `gamma`). `14 - 10 = 4`
is exactly the four outer declaration binders — BOTH de Bruijn references are
well-weakened; what is absent is the component substitution/refinement through
the Sigma-shaped observational equality. The same local-vs-outer mismatch appears
in all three right-intro methods (the compound record-index class, not an
Init-only typo).

## This is a COMPLETENESS gap (false-reject), NOT a soundness hole

As with entry 1: the elaborator only CONSTRUCTS the elaborated dependent-match
term; the generated-elim path re-runs `kernel_infer` on the zonked assembled term
(the soundness net documented at `elab.rs:4785`, unchanged). Installing a correct
component refinement lets the elaborator build a term the kernel already accepts;
a wrong term is still rejected by the kernel. Kernel gate untouched — hence
elaborator-only, `gate: none`, no operator authorization, no TCB touch.

## Deliverables (structural closure, Architect disposition evt_2ptgr3f2ef7c4)

1. **Implement ONE shared Sigma-projection walker and apply it at BOTH confirmed
   record-index consumers.** The walker is the consumption-side mirror of the
   predecessor's uniform `{Eq, Sigma, Top}` descent: `Eq` -> project the leaf
   refinement; `Sigma` -> recursively project both components via `Proj1`/`Proj2`;
   `Top` -> no leaves; **any other shape (including `Pi` under `Sigma`) -> atomic
   fail-closed rejection** (never a silent fall-through to the unreduced
   whole-record triple). Project from the producer's preserved raw-`Eq` premise
   sentinel (the producer keeps one raw `Eq` premise binder per declared index),
   NOT by flattening method arguments. The two confirmed consumers that need it
   (language-leader pre-stage evt_2qaczkndnpzm4, grounded at landed `569712a4`):
   **`install_index_refinements`** (the HS2 site) and **`refine_branch_goal`** —
   both peel `Term::Eq` and fall through on the `Sigma`.
2. **AUDIT + CLASSIFY every consumer** of a record-index / dependent-match index
   equality in the elaborator for the same `Eq`-only-no-`Sigma`-decomposition
   shape (grep the index-refinement + motive-rebasing paths for `Term::Eq`
   peeling that lacks a `Sigma` arm). Each consumer resolves to exactly one of:
   already Sigma-decomposing, needs the walker (fix), or demonstrably out of
   scope (cited). **`install_hidden_result_variable_refinements` is an explicit
   must-classify target** — it consumes a SEPARATE hidden whole-scrutinee
   equation (not `method_index_premise_pairs`) outside the indexed FokDerivation
   producer path and may carry a parallel record-`Sigma` limitation. The audit
   either fixes it with the shared walker (if it has the gap on a reachable
   record-shaped scrutinee equation) or records it EXCLUDED with a cited reason
   and its current private test home — NOT silently implied (Steward disposition,
   answering the leader's frame question). All other surface `Refl`/`J` sites are
   author-facing or leaf-only and are out of scope.
3. **The regression net** (language-leader pre-stage, ready): first-component
   install red, second-component goal-refinement red, whole-record `J` red, and
   direct recursive-IH `Sigma`-discharge green — one fixture per closed consumer,
   the acceptance shape the just-landed predecessor established.

No FoKripke source workaround (D2b's banned-source-compensation constraint). The
`fok_invert_atomlike: theorem -> fn` correction stays on the held D2b branch.

## Acceptance criteria (predicate form; fixtures illustrate, they are not the roster)

- **AC-COMPONENT-REFINEMENT (positive).** A dependent inversion whose record
  index equality WHNFs to a `Sigma` of component equalities installs the
  per-component refinements and elaborates. Two-sided control: reverting the
  Sigma-projection in `install_index_refinements` re-reds the fixture with the
  outer-vs-local binder mismatch (`fok_nth_form @<outer>` expected vs
  `@<local>` found).
- **AC-CLASS-CLOSED (the audit, predicate not roster).** Every elaborator
  consumer of a record-index / dependent-match index equality that peels the
  `Eq` head handles the `Sigma`-decomposition uniformly — no consumer falls
  through on the Sigma to an unreduced whole-record triple. The two confirmed
  consumers (`install_index_refinements`, `refine_branch_goal`) are fixed via the
  shared walker; the audit returns an explicit verdict for EACH remaining
  consumer (Sigma-decomposing / fixed / cited-excluded), with
  `install_hidden_result_variable_refinements` named among them. A surviving
  unclassified Eq-only consumer is a defect, not a deferral; a cited exclusion
  (out-of-class or unreachable, with its test home) is a valid audit outcome, a
  silent omission is not.
- **AC-NO-OVERACCEPT (soundness on the record).** The generated-elim assembled
  term remains `kernel_infer`-re-derived (`elab.rs:4785` net unchanged); an
  installed refinement that does not correspond to a real component equality is
  rejected by the kernel. No refinement path yields a term the kernel would not
  independently classify.

## Contention check

Production touch is `crates/ken-elaborator/src/elab.rs` (`install_index_refinements`
/ `try_reindex_cast` + the audited consumers) plus per-site test fixtures. Same
crate as the synthesis-side predecessor LANG-GENERATED-INDEX-EVIDENCE-CLOSURE
(landing/landed) — sequence this AFTER that lands (depends_on), as the Sigma
evidence this consumer projects is what that predecessor synthesizes. No other
active lane touches `elab.rs` (lane 1 runtime = ken-runtime + rt_parity fixture;
lane 3 foundation = catalog/). The held D2b branch carries only the
`fok_invert_atomlike` edit in `FoKripke.ken` — no collision; D2b rebases onto
this after it lands.

## Capability tier: T1

Two-line-ish core fix, but the review turns on a soundness argument on the
kernel-adjacent index-refinement path (does projecting the Sigma over-accept?)
AND on the audit judgment (is the class actually closed, or is a fourth consumer
lurking?). That judgment, plus the Architect's required soundness review, is the
deliverable — not the diff.

## Gate on landing

`gate: none` (no TCB touch, no operator authorization). Reviewed by the
**Architect (required soundness reviewer; will re-review D2b after this lands)** +
**language QA**, standing Adversary hunt independent. Steward routes M1-M4;
lieutenant M5-M9. D2b stays HELD at acef50612 until this lands; the Steward
EXPLICITLY re-releases D2b only after — the held D2b evidence rebases onto the
landed fix and continues the unchanged adequacy theorem.
