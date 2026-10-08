---
id: LANG-NATIVE-SIGMA-ERASED-FIELD
title: "Native lowering of a relevant Σ lowers its first field unconditionally, so a proof-first relevant Σ (Σ(p : Eq Int 7 7).Int) reaches the ErasedOmegaSubterm arm and refuses natively, although the plan keeps the pair and the interpreter returns Pair { fst: Neutral, snd: Int(8) }. Give an erased field in a retained Σ slot a native representation at interpreter parity"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-PROOF-ERASURE]
blocks: []
github: null
origin: "Architect design answer evt_55gdejjrzap0p on the LANG-REFINEMENT-PROOF-ERASURE AC-4 stop (language-implementer evt_qa27v5yaf6bx, language-leader evt_5wyvpxshfckeh). The gap is fail-closed and in the predecessor's own new StructuralPair arm, not a regression. Steward-filed per COORDINATION section 2."
---

# An erased field in a relevant Σ runs natively

## Objective

A relevant Σ whose first component is an erased Ω subterm lowers natively
and agrees with the interpreter. The predecessor's proof-first native
refusal pin becomes a native positive.

## Settled inputs (Architect `evt_55gdejjrzap0p`, on WIP `aab5ef48f`)

- **The refusal.** `lower_body_term_inner` lowers `StructuralPair { first,
  second }` (`erasure.rs:5226`) by lowering `first` unconditionally into a
  `"first"` record field. The `ErasedOmegaSubterm` arm (`erasure.rs:5212`)
  is a hard `erased_omega_subterm_reached_computation` refusal.
- **The witness.** `Σ(p : Eq Int 7 7).Int` with a postulated proof and
  second value `8`: plan `collapsed_sigmas = {}`, `erased_subterms = {1}`;
  interpreter `Pair { fst: Neutral, snd: Int(8) }`; native refusal.
- **The open fork** for the retained slot's native representation:
  - (i) a `Unit.MkUnit` placeholder, which needs the Unit family in the
    package (the only `MkUnit` in `erasure.rs` is the test host-spine table
    at `:8579`);
  - (ii) an empty `RuntimeValue::Record { fields: vec![] }`, whose backend
    support is unmeasured;
  - (iii) dropping the slot, which changes pair arity and `Project`
    lowering.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measurement only).** For each of (i)-(iii), write the native
   value the witness would produce and measure whether the backend lowers,
   stores and projects it today: every `Project` on a relevant Σ, every
   record consumer of the pair layout, and the package's family set. The
   Architect rules the representation from the D0.
2. **The ruled representation** in relevant-Σ native lowering, for an
   erased field in either slot.

## Acceptance

- **AC-1.** The predecessor's proof-first fixture
  (`relevant_sigma_pair_keeps_both_components`) gets a native positive in
  place of its refusal pin, agreeing with the interpreter on the second
  component, and a projection of that component agrees on both paths.
- **AC-2 (control).** `Σ Int Int` and the subset-Σ carrier rows keep their
  native results, and the predecessor's plan bytes and `core_semantic_hash`
  pins are unchanged.
- **AC-3 (mutation, QA).** Lowering the erased field unconditionally again
  returns AC-1 to the exact `erased_omega_subterm_reached_computation`
  refusal.

## Stop conditions

- The ruled representation needs a kernel, `trusted_base()` or spec
  change, or changes a package's plan bytes or semantic hash.
- No option lowers without a new runtime value kind: stop to the Steward
  with the D0.
