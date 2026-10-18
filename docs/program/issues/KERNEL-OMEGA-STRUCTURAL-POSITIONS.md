---
id: KERNEL-OMEGA-STRUCTURAL-POSITIONS
title: "Structural conversion compares a neutral Cast's proof argument syntactically, although Cast's formation fixes it at an Omega type, so two Casts that differ only in their equality proof do not convert. Spec 16 sections 1.2 and 8.2 already make Omega contents definitionally irrelevant. Skip the positions that are Omega by formation on the structural path"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-SOLVED-INDEX-SUBSTITUTION]
github: null
origin: "Architect ruling evt_2hy029razcfgq on LANG-SOLVED-INDEX-SUBSTITUTION's third stop (route A, after Research advisory evt_6q5n8x61jsyvw; S10 arms from Research sweep evt_7daqm6ydmbnsr). A kernel change: draft until the operator approves. Steward-filed per COORDINATION section 2."
---

# Omega-by-formation positions are skipped on the structural path

## Objective

Structural conversion treats a term position that is Ω by its formation
rule as definitionally irrelevant, as spec 16 already requires, starting
with a neutral Cast's equality proof.

## Settled inputs (Architect `evt_2hy029razcfgq`, on `02b64897f`)

- **The gap.** The neutral Cast arm (conv.rs ~:1413) compares all four
  fields structurally. Its comment says `e` is not skipped because the
  type-agnostic path has no trusted field type. Cast's formation
  (check.rs ~:422) checks `e` at `Eq (Type l) A B`, which is Ω, so the
  position carries the property and no inferred type is needed.
- **The spec.** 16 §1.2: the conversion checker "may skip the contents of
  propositional arguments entirely". 16 §8.2: "any sub-term at Omega type
  is definitionally trivial". This node is kernel conformance to that
  text, not a spec change.
- **Soundness.** The arm compares A and B first, and the structural
  path's precondition is that both terms are well typed. So `e1` and `e2`
  inhabit convertible Ω types, and Ω proof irrelevance makes them equal.
  Prior art decides the same skip by position on untyped conversion
  (Research `evt_6q5n8x61jsyvw`: Sirman et al. CC^obs, Agda relevance
  marks, Coq SProp).
- **Measured with an env-gated Cast skip** on WIP `fff3a5a8c`: sec and
  secrest go green; `ken-kernel --lib` goes from 121 to 119, and the two
  failures are pins of today's behaviour,
  `cast_congruence_compares_all_four_fields_directionally` (conv.rs
  ~:2868) and `nonrecursive_delta_retry_failed_spines_have_linear_structural_entries`
  (~:1623). k2c_conversion 59/59, obs_type_eq_structural 23/23 and
  acceptance 14/14 are unchanged.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Cast-proof skip in `crates/ken-kernel/src/conv.rs`. Research's S10
positions (the Absurd proof, Elim parameters that are proofs, and the
missing `(Refl, Refl)` arm) are each measured first. Admit only those
whose Ω-ness is fixed by formation, and name any other.

## Acceptance

- **AC-1.** The two pins are re-expressed under the new rule.
- **AC-2.** One negative row per admitted position: a Cast whose A, B or
  t differs still does not convert.
- **AC-3.** One conformance row per admitted position, under
  `conformance/`.
- **AC-4 (mutation, QA).** Restoring the structural `e` comparison returns
  sec to red on the SOLVED-INDEX WIP, and the AC-2 negatives stay
  refused.

## Stop conditions

- A position admitted without a formation rule that fixes it at Ω.
- Any change to what typechecks, as opposed to what converts, or any
  `trusted_base()` change.
