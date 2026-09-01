---
id: LANG-RECORD-INDEX-REFINEMENT
title: "Elaborator predecessor for D2b: make generated dependent-elimination branch refinement handle a constructor-headed record index (e.g. FokMkSequent gamma delta) by forming the three transport constructors' J under an abstract index type and abstract endpoints, then applying that checked generic helper at the concrete index — the core analogue of the already-green generic fok_cong pattern. No FokDerivation re-index, no kernel/spec/trust change."
status: ready
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-01, framing the Architect's D2b hard-stop disposition (ruling evt_68t4wwrs274nh, on the language-leader hard-stop evt_6m6bbz03qye6f). D2b (V3-FO-EMBEDDING-ADEQUACY) hard-stopped at the derivation inversion: adequacy needs to dependently eliminate FokDerivation over its compound record index FokMkSequent gamma delta, and the current match compiler cannot. The Architect independently reproduced the exact minimal failure at 70a291a96 (tree 740c5e7f4) and classified it as an ELABORATOR-ONLY predecessor — NOT a relation re-index (b) and NOT a kernel/TCB widening (c), so NO operator authorization is needed (the three-option list conflated elaborator repair with kernel enlargement; different trust layers). Steward owns the frame, sequencing, and D2b re-release. Base current origin/main 5cd4411b6; the elaborator coordinates below were located at current main and are stable vs the Architect's 740c5e7f4 measurement (the only intervening commits are doc-only). Symptom inventory folded from architect commit 21b1c477d onto the consumer node."
---

> # ELABORATOR-ONLY PREDECESSOR of D2b. The mechanism contract is the
> # Architect's, verbatim in scope: ruling `evt_68t4wwrs274nh`. Read it before
> # cutting anything; do NOT re-derive it here and do NOT reopen the D2b theorem.
> #
> # This node MERGES (unlike the D2b held evidence). It lands the elaborator fix,
> # then the Steward explicitly re-releases D2b. It touches the elaborator crate
> # ONLY — `FokDerivation`, `FoKripke.ken`, `/spec`, the kernel, the prover, the
> # trusted base, and the FO verdict are all byte-unchanged.

## The constraint (grounded, not aesthetic)

D2b must prove `embedding_adequacy` by structural induction on `FokIForm`, which
requires inverting `FokDerivation (FokMkSequent gamma delta)` — a dependent
elimination over a constructor-headed record index. Adequacy is the second of
the two `23 §4.4` theorems route FO needs before it may return `proved`; it
cannot be proved without this inversion, and the Architect ruled the inversion
cannot be expressed on the current match compiler. The constraint is the spec's
theorem obligation, not a convenience.

## Symptom inventory (Architect)

A derivation family over a constructor-headed record index can be built but not
dependently eliminated: generated refinement treats observational record
equality as a primitive `Eq`/`J` witness after it has reduced to a Sigma of
field equalities — keyed on the index equality's representation rather than the
derivation relation.

## Grounded defect (Architect, measured at `70a291a96` / tree `740c5e7f4`)

The top-premise path is ALREADY general and is NOT the defect:
`finish_dependent_elim` (`elab.rs:2445`) calls `synth_generated_index_evidence`
(`:1357`), and `synth_refl_proof` (`:1327`) recursively constructs Sigma-shaped
reflexivity. Do NOT replace that path or add a `FokSequent` special case;
`synth_generated_index_evidence` remains the SOLE top-premise reflexivity
producer.

The failure is later, in generated branch refinement:

1. `method_index_premises` (`elab.rs:3189`) correctly issues the raw premise
   `Equal FokSequent constructor_conclusion scrutinee_index`.
2. `install_index_refinements` (`elab.rs:3558`) WHNFs that premise. For a Nat
   constructor equality it obtains another primitive `Eq` after constructor
   peeling; for `FokMkSequent`, observational equality instead becomes the Sigma
   of the two `List` equalities, and the current fallback retains the whole
   record endpoints.
3. `build_sym` (`:3232`), `build_index_type_cong` (`:3268`), and
   `build_index_omega_transport` (`:3315`) then construct `J` ALREADY specialized
   to concrete `idx_ty = FokSequent`; `build_sym` also constructs a literal
   specialized `Refl` base. The concrete equality has already reduced to Sigma,
   so the direct specialized `J`/`Refl` path cannot recover the original `Eq`
   head — which is why even a constant `Nat` result fails before its arm can use
   `left`.

**Discriminating positive (proves this is an elaborator, not a logical/relation,
gap).** On the SAME exact file, a theorem applying the existing generic
`fok_cong` to `Equal FokSequent (FokMkSequent g1 d1) (FokMkSequent g2 d2)` and
projecting `fok_seq_gamma` checks GREEN: its `J` is checked while its
type/index are ABSTRACT, then instantiated at `FokSequent`. The current
dependent-match producer instead specializes first and asks the primitive `J`
path to rediscover an `Eq` after observational reduction.

## Authorized mechanism (Architect — do not re-derive)

Keep the raw generated equality premise. Refactor the three generated transport
constructors so their `J` is formed under an ABSTRACT index type and ABSTRACT
endpoints, then apply that checked generic helper to the concrete `index_type`,
`old_index`, `new_index`, and branch evidence:

- symmetry used by the sibling convoy (`build_sym`);
- Type-classified index congruence used by `Cast` (`build_index_type_cong`);
- Omega-classified direct transport used by the landed Omega arm
  (`build_index_omega_transport`).

Generate the core analogue of the already-green generic `fok_cong` pattern — NOT
a `J` whose motive/domain has already substituted `FokSequent`. Preserve old/new
orientation EXACTLY. The kernel re-checks the completed generic applications as
it does today.

**Closure (do not special-case).** Do not special-case `FokSequent`,
`FokMkSequent`, two fields, or `List`. The rule is: ANY single index type whose
observational reflexive equality reduces structurally before branch refinement
must use the same generic transport producer. The previously retained
multi-index goal-restoration limitation stays UNSUPPORTED and is not silently
widened by this node.

## Acceptance criteria (Architect's required predecessor evidence, verbatim in scope)

- **AC-1 (the reported red→green).** The exact reported record-index
  constant-motive match (`FokDerivation (FokMkSequent gamma delta) -> Nat`,
  exhaustive) changes from red to green. This catches the current over-eager
  sibling-convoy construction even though the arm result is only `Nat`.
- **AC-2 (a real consumer, using fields).** A record-indexed dependent `Omega`
  consumer changes from red to green AND uses its constructor fields / recursive
  evidence. A read of a helper term is NOT enough.
- **AC-3 (the D2b probe reaches the arm body).** The held D2b inversion probe,
  over the UNCHANGED `FokDerivation` declaration, reaches the arm body. It need
  not finish adequacy inside this predecessor.
- **AC-4 (causal necessity, population-side).** A population-side mutation that
  restores the current specialized-at-concrete-index `J` producer makes BOTH
  real consumers (AC-1, AC-2) red again; the detector and fixture stay fixed.
- **AC-5 (orientation + field discrimination).** Wrong-direction old/new and a
  one-field substitution each RED independently while the other record field,
  the family, the motive, and cardinality stay fixed. A Sigma pair merely
  existing is not acceptance evidence.
- **AC-6 (no regression on the existing rows).** Existing Nat-index `Type` and
  `Omega` integration rows remain green with BYTE-IDENTICAL checked-core output
  where the new generic application is not required; no weakening of their
  constructor-injectivity controls.
- **AC-7 (the two-index boundary stays put).** The established two-index negative
  remains RED at its own exact unsupported boundary (not silently widened).
- **AC-8 (diff confinement + no-regression in CI).** The diff is confined to the
  elaborator crate plus its focused tests/frame. `FokDerivation`, `FoKripke.ken`,
  `/spec`, the kernel, the prover, the trusted base, and the FO verdict are
  byte-unchanged. Whole-suite green in CI (`COORDINATION §12`); targeted local
  validation only (`-p ken-elaborator`), never `--workspace`.

## Banned scope

- **Re-indexing `FokDerivation`** — no Nat code for sequents, no record split
  into `List` indices, no change to `fok_derives`/`fok_classically_valid`. That
  compensates in the relation for an elaborator defect (and `List`-index
  refinement is not established anyway).
- **Any kernel / spec / trusted-base / public-Ken-relation change**, and any FO
  `proved` flip. This is an elaborator-crate repair; the kernel re-checks the
  output unchanged.
- **Special-casing `FokSequent`/`FokMkSequent`/two-field/`List`**, or replacing
  the `synth_generated_index_evidence` top-premise path.
- **Widening the multi-index goal-restoration limitation** — it stays
  unsupported; AC-7 pins it.

## Required reviewers

- **Architect** — required reviewer for mechanism faithfulness: the fix forms the
  generic abstract-index `J` (the `fok_cong` analogue) and does not special-case
  the record, `synth_generated_index_evidence` stays the sole top-premise
  producer, and the two-index boundary is not widened.
- **language-QA** — the normal language-ring review path.
- **Adversary** — over-accept hunt: a generic transport producer that manufactures
  an unsound elimination, or an AC control that is manufactured rather than
  causal (AC-4/AC-5 must be caller-side and population-side, not detector-side).

## Sequencing

Framed and landed from current `main` (`5cd4411b6`), `depends_on: []`. On the
candidate: Architect + language-QA + Adversary on the exact SHA, then Steward
M1-M4, then the lieutenant. **Only after this node's exact consumer gate (AC-1..
AC-3) is green does the Steward EXPLICITLY re-release D2b** — the held D2b
evidence `70a291a96` (the 177-line strengthened ledger/inversion spine) is
reusable material, not a candidate; the language ring rebases/folds it onto the
landed elaborator fix and continues the EXACT unchanged adequacy theorem. No
checker narrowing, relation redesign, kernel seam, or FO `proved` flip is
authorized by this node.

## Capability tier: T1

A soundness-adjacent elaborator capability (dependent elimination over a compound
record index), reviewed on the argument that the generic transport is lawful and
does not over-accept — not a mechanical diff.
