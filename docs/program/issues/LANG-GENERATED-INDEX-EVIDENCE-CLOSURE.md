---
id: LANG-GENERATED-INDEX-EVIDENCE-CLOSURE
title: "D2b predecessor (elaborator-only): close the generated-index reflexive-evidence terminal-vocabulary stratification — Top is admitted only at the outer entry (synth_generated_index_evidence) and not inside the Sigma recursion (synth_refl_proof), so a reflexive record index that reduces through a Sigma to a nested closed-equal field (WHNF Top) is false-rejected with `Refl expects an Eq-shaped goal`. Route the Sigma recursion through the Top-aware entry so {Eq, Sigma, Top} dispatch uniformly at every nesting depth."
status: merged
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-02. Second D2b hard-stop predecessor for V3-FO-EMBEDDING-ADEQUACY, distinct from the first (LANG-RECORD-INDEX-REFINEMENT, the eq_at_inductive kernel/TCB binder-hygiene fix, merged PR #3216). D2b resumed after that landed and, at acef50612, reached a NEW gap in a DIFFERENT function: the generated-index reflexive-evidence synthesizer stratifies its terminal vocabulary. Hard stop reported by language-leader evt_77f4tyydwv4ew; Architect ruling evt_27wm95g6gvtje (CAUSE confirmed from source, completeness gap not soundness hole, sanctioned generic repair, predecessor scope); carried to the Steward by language-leader evt_5wba1aypnbes3. Coordinates measured at origin/main e485a696c (they DRIFT on the held D2b branch acef50612, ~+74 lines below main per the Architect); the ring re-measures. Architect count: D2b design hard-stop #1 (the OOM was resource, the fok_invert_atomlike theorem->fn was a mechanical self-resolve); §1a Research advisory fires at the 3rd, not triggered."
---

## One-line objective

Teach the generated-index reflexive-evidence synthesizer to dispatch its
terminal vocabulary `{Eq, Sigma, Top}` UNIFORMLY at every nesting depth, so a
reflexive record index whose evidence goal reduces through a `Sigma` to a nested
closed-equal field (which WHNFs to `Top`) elaborates instead of being
false-rejected. This unblocks D2b (`V3-FO-EMBEDDING-ADEQUACY`).

## Grounded cause (Architect-confirmed, evt_27wm95g6gvtje; verified from source at origin/main e485a696c)

Read the algorithm, not the prose:

- `synth_generated_index_evidence` (`crates/ken-elaborator/src/elab.rs:1361`):
  `whnf(expected)` → `Top` ⇒ `tt`; `_` ⇒ delegate to `synth_refl_proof`
  (`:1372`). Top is handled ONLY at this outer entry.
- `synth_refl_proof` (`:1331`): `Eq(a,t,u)` ⇒ convert-check → `Refl(t)`;
  `Sigma(dom,cod)` ⇒ recurses into ITSELF on `dom` (`:1349`) then
  `subst0(cod,fst)` (`:1351`); `_` ⇒ `Err "Refl expects an `Eq`-shaped goal"`
  (`:1356`).

The Sigma arm re-enters `synth_refl_proof`, which has NO Top arm, so a field
that WHNFs to `Top` reached via Sigma descent hits the `_` reject. The D2b
`FokSequent` reflexive index reduces to a `Sigma`, whose nested
`Equal (List FokForm) Nil Nil` field is definitionally `Top` — exactly this
path. The terminal vocabulary is stratified into an outer set (`{Top}` at the
entry) and an inner set (`{Eq, Sigma}` in the recursion) keyed on nesting depth.

## This is a COMPLETENESS gap (false-reject), NOT a soundness hole (Architect-confirmed)

The goal is genuinely inhabited: `Top` has the unique inhabitant `tt`, and the
reduction `Equal (List A) Nil Nil ⇒ Top` is the kernel's own definitional
behaviour — `tt : Top` is precisely the evidence the generated Elim premise
expects. Decisive backstop: the generated-elim path re-runs `kernel_infer` on
the zonked assembled term — `synth_generated_index_evidence` is called at
`elab.rs:2724`, and the assembled elim term is kernel-re-derived at `:2740`
(the general soundness-net contract is documented at `:4785`, "final
`kernel_infer` re-derivation as the soundness net, never trusting" the
elaborator's own construction). So the elaborator cannot smuggle a bad proof
through this fix; a wrong term is rejected by the kernel. The kernel gate is
UNTOUCHED and stays the sole soundness authority. This is why the WP is
elaborator-only, `gate: none` (Architect + language QA + standing Adversary,
NOT operator) — it does not grow the TCB.

## Sanctioned mechanism (paste-ready, Architect evt_27wm95g6gvtje)

In `synth_refl_proof`'s `Sigma` arm, swap BOTH internal recursive calls from
`synth_refl_proof` to `synth_generated_index_evidence`:

```rust
Term::Sigma(dom, cod) => {
    let fst = synth_generated_index_evidence(env, ctx, &dom, span)?;
    let snd_ty = subst0(&cod, &fst);
    let snd = synth_generated_index_evidence(env, ctx, &snd_ty, span)?;
    Ok(Term::pair(fst, snd))
}
```

That is the whole production fix — two call-target swaps (origin/main `:1349`
and `:1351`; re-measure on the branch). Now `{Eq, Sigma, Top}` dispatch
identically at EVERY nesting depth: a component that WHNFs to `Top` yields `tt`,
`Eq` yields `Refl`, nested `Sigma` recurses again.

**Why this form, NOT a bare `Top` arm on `synth_refl_proof`.**
`synth_refl_proof` has a SECOND caller — the user-facing `Refl` sugar at
`elab.rs:1135`, whose equality-origin guard falls to the `_` reject at `:1139`
and MUST stay unchanged ("`Refl` is gated to equality-origin goals"). Routing
the fix through the Sigma recursion leaves `synth_refl_proof`'s top-level `_`
arm untouched, so the user-`Refl` contract is preserved; only nested components
inside an already-admitted `Sigma` gain Top-completeness. A bare `Top` arm on
`synth_refl_proof` would re-introduce the stratified-vocabulary defect (two
functions kept in sync by hand) AND weaken the user-`Refl` guard.

## Deliverables

1. The two call-target swaps in `synth_refl_proof`'s `Sigma` arm (above).
2. A completeness fixture (elaborator test): a reflexive generated
   dependent-match index whose evidence goal reduces THROUGH a `Sigma` to a
   nested closed-equal field that WHNFs to `Top`, which previously false-rejected
   with `Refl expects an `Eq`-shaped goal` and now elaborates.
3. A guard fixture: a bare user `Refl` on a top-level `Top` goal (not
   equality-origin) stays REJECTED, establishing the user-`Refl` contract is
   unchanged across the swap.

The `fok_invert_atomlike: theorem -> fn` correction (a proof-relevant
`Type`-returning definition cannot be `theorem`) is Architect-ruled valid and
IN-SCOPE FOR D2b — it stays on the held D2b branch, NOT on this node.

## Acceptance criteria (predicate form; the fixtures illustrate, they are not the roster)

- **AC-CLOSURE (positive completeness).** The terminal vocabulary of generated
  reflexive-index evidence synthesis is closed UNIFORMLY over nesting structure:
  any evidence goal built from `{Eq, Sigma, Top}` at any depth synthesizes a
  witness. The class-representative fixture (deliverable 2) elaborates.
  Two-sided control: restoring the pre-fix `synth_refl_proof` calls in the Sigma
  arm re-reds it with EXACTLY `Refl expects an `Eq`-shaped goal`. (The mutation
  proves the swap is the cause, not incidental.)
- **AC-USER-REFL-CONTRACT (guarded invariant).** User-facing `Refl` on a
  top-level `Top` goal stays REJECTED, unchanged before and after the swap —
  because the swap touches only the nested Sigma recursion, never
  `synth_refl_proof`'s top-level `_` arm nor the user-`Refl` caller at
  `elab.rs:1135`. The guard fixture (deliverable 3) holds both ways.
- **AC-NO-OVERACCEPT (soundness on the record).** The synthesizer's terminal
  vocabulary remains exactly `{Eq, Sigma, Top}` — a nested field whose WHNF is
  outside that set still hits the reject (no wildcard is opened) — and the
  assembled generated-elim term remains `kernel_infer`-re-derived at
  `elab.rs:2740`. No synthesis path yields a witness the kernel would not
  independently classify. This is the auditable form of the Architect's
  "completeness, not soundness" disposition.

## Contention check

Production touch is `crates/ken-elaborator/src/elab.rs` (`synth_refl_proof`
Sigma arm) plus an elaborator test fixture. No other active lane touches
`elab.rs`: lane 1 (runtime) is on `crates/ken-runtime` + the `rt_parity` fixture
(the HS7 fallback d2e039bb is byte-identical in `ken-elaborator`); lane 3
(foundation) is on `library/` catalog + a measurement. The held D2b branch
`acef50612` carries the `fok_invert_atomlike` edit in `FoKripke.ken` and its
adequacy proof — no file collision with this node; D2b rebases/folds onto this
after it lands.

## Capability tier: T1

The mechanism is two lines, but the review turns on a soundness ARGUMENT on the
kernel-adjacent evidence-synthesis path: whether closing the vocabulary
over-accepts, and whether the user-`Refl` contract survives. That judgment, not
the diff, is the deliverable — so a T1 seat and the Architect's required
soundness review on the exact SHA.

## Gate on landing

`gate: none` in the roadmap sense (no operator authorization — no TCB touch).
The candidate is reviewed by the **Architect (required soundness reviewer, will
re-review D2b after this lands)** + **language QA**, with the standing Adversary
code-merge hunt attacking it independently. Steward routes (M1-M4); lieutenant
executes (M5-M9). Only after it lands does the Steward EXPLICITLY re-release D2b
(`V3-FO-EMBEDDING-ADEQUACY`); the held D2b evidence at `acef50612` rebases onto
the landed fix and continues the unchanged adequacy theorem.

## On denial / no elaborator-only path

The Architect ruled a convoy/re-index workaround in `FoKripke.ken` is FORBIDDEN
(D2b's own banned-source-compensation constraint) precisely because the correct
fix is up in the elaborator, not in the proof. There is no in-D2b route; this
predecessor is the dependency.
