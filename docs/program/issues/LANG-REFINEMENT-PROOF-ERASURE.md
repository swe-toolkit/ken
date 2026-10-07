---
id: LANG-REFINEMENT-PROOF-ERASURE
title: "The interpreter is strict in Unknown on pairs, the checked-core pair view rejects dependent Σ, and the native decoder has no refl and decodes a λ over an equation as relevant, so neither a subset-Σ proof nor a path-condition convoy can run. Erase every Ω-classified position (binder, argument, refinement pair forms), keyed on the classification, in lowering and in the interpreter"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-PATH-CONDITION-EVIDENCE, LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Language ring, so L1 keeps the runtime ring (operator 2026-09-17: ignored tests are top priority). Sequenced before LANG-PATH-CONDITION-EVIDENCE and widened to every Ω position (Architect evt_31p5m7pnr13fv: the convoy does not erase on the native path). Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W3."
---

# A refinement value runs as its carrier

## Objective

Nothing classified at Ω is evaluated or represented at runtime. A pair
at a refinement type runs as its carrier value, an Ω-classified λ binder
or application argument is erased, and a relevant Σ is unchanged.

## Settled inputs (Architect, measured at `1153a9fc6`)

- `eval.rs` Pair arm: an Unknown component makes the pair Unknown, and a
  postulate evaluates to Unknown.
- `checked_core.rs:3578` `type_has_dependent_sigma` rejects dependent Σ in
  the checked-core pair view.
- Refinement-ness is the kernel classification `whnf(T) = Sigma(A,φ)`
  with `φ` classified at Ω, never a spelling.
- (Architect, measured at `c8e59b77c`, `evt_31p5m7pnr13fv`.)
  `decode_supported_body_term_after_tag` (checked_core.rs ~3086) accepts
  `var int_lit const constructor_ref elim lam app let absurd pair proj1`;
  `refl` falls to `UnsupportedTermShape`. A `λ (e : Eq ..)` decodes as a
  runtime `Lambda`. The decoder's `app` arm has no expected type, so an
  argument's classification must come from the function's Π domain.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One mechanism, type-directed erasure keyed on the Ω classification, on
both the checked-core lowering path and the interpreter path:

- `Pair(v,π)` at a refinement becomes `v`, `Proj1` at a refinement
  becomes the identity, and `Proj2` is erased.
- A λ binder whose domain is classified at Ω is erased, and so is an
  application argument at an Ω-classified Π domain. A motive premise
  becomes a method binder, so a convoy (`elim … (refl s)` with methods
  `λ f̄. λ e. body`) runs as the plain `elim`.
- `refl` in an erased position needs no runtime form.

## Acceptance

- **AC-0 (D0, measure only).** For each evaluator, name the plane that
  carries the sort at each point erasure must decide: the Σ codomain, a
  λ domain, and an application argument (from the function's Π domain).
  The interpreter is untyped, so this decides its mechanism (CHECK 5).
- **AC-1.** A hand-built core row with a pair at `Σ Int φ` whose proof is
  an open hole lowers and evaluates to the carrier value on every path.
- **AC-2.** A hand-built relevant Σ (second component not at Ω) stays a
  pair on every path.
- **AC-2b.** A hand-built convoyed `if` and a convoyed Nat `match`
  (motive `λ y. Eq S s y → T`, applied to `refl s`) lower and run at
  parity with their unconvoyed forms, on both paths.
- **AC-3.** Runtime parity is unchanged.
- **AC-4 (mutation).** Keying erasure on the first component alone
  reddens AC-2.
- **AC-4b (mutation).** Keying argument erasure on the `refl` spelling
  instead of the classification reddens an AC-2b-shaped row whose
  evidence argument is a variable.

## Stop conditions

- An evaluator with no plane carrying the codomain sort: stop to the
  Architect.
- Any kernel, `trusted_base()` or spec change.
