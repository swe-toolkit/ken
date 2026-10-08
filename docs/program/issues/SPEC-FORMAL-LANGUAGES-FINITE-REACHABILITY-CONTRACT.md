---
id: SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT
title: "spec/50-stdlib/61-formal-languages.md section 2 is a deferred placeholder, so the catalog has no contract for finite-state evidence or for deciding reachability and emptiness of a Dfa. Specify the Finite certificate (an enumeration with an Omega membership proof, no DecEq, no dedup), fin_finite and pair_finite, and the certificate-returning reachability decision with its sound and complete laws"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [CAT-FORMAL-LANGUAGES-FINITE-REACHABILITY]
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". Architect L3 order evt_7z0tswfa1wd9 section 4 item 1; boundary ruling evt_5aspyx6byry (measured development, 99 declarations, rc=0, no Axiom). Steward-filed per COORDINATION section 2."
---

# A finite-state evidence and reachability contract

## Objective

Section 2 of `spec/50-stdlib/61-formal-languages.md` is the normative
contract that `CAT-FORMAL-LANGUAGES-FINITE-REACHABILITY` builds against.

## Settled inputs (Architect `evt_5aspyx6byry`)

- **The certificate.** `Finite q` is a record of `listed : List q` and a
  proof that every `x : q` satisfies `list_elem q x listed`, where
  `list_elem` is Ω-valued (`Bottom` at `Nil`, a truncated `Or` at `Cons`).
  No duplicate-free condition. `Finite q` yields no `DecEq q`: decidable
  state equality stays a separate requirement (§5).
- **A value, not a class.** `Fin n` receives the value `fin_finite n`, and
  `pair_finite : Finite q → Finite r → Finite (Pair q r)` lets the decision
  apply to §1's `product`, `intersection` and `union`.
- **The decision.** `find_word fq fa d target s : Option (List a)` is
  primary. `reachable` (`is_some` of it), `accepted_word` (from `start` to
  `final`) and `is_empty` (`bool_not` of reachability to `final`) are
  derived. Reachability is to a predicate `target : q → Bool` from any
  state. State-to-state reachability needs `DecEq q` and stays in §5.
- **Laws.** `find_word_sound`, `find_word_complete`,
  `accepted_word_accepts`, `is_empty_rejects`, with the types in the
  ruling. Completeness is in scope.
- **Packages.** The certificate is `Data.Finite.Finite`; the decision is
  `Algorithm.FormalLanguages.Reachability`. §1 is unchanged.
- **Not normative:** any complexity bound. A later table or BFS
  implementation must satisfy the same laws.

## Deliverable

Section 2, normative for:
- `list_elem`, `Finite`, `MkFinite`, `elements`, `covers`, `fin_finite`
  and `pair_finite`;
- `find_word`, `reachable`, `accepted_word` and `is_empty`, and the four
  laws, each stated as a proved theorem;
- the trust boundary: no Axiom, primitive, kernel form or `trusted_base()`
  entry;
- the statements that the certificate gives no `DecEq` and that no
  complexity bound is promised.

Whether the certificate gets its own chapter is the Spec enclave's call.
Update `spec/SPEC-PROGRESS.md` if the chapter set changes.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.

## Closeout

Merged `c7c05d4e6` from exact `d840592ae` (PR #4604). Conformance-validator
`evt_4yxcktk4cs60b`, Architect `evt_22k52472gn6r7`, Decision
`dec_2pxwjr54qn006`. Section 2 of `spec/50-stdlib/61-formal-languages.md`
specifies the `Finite q` certificate (Ω list membership, no `DecEq`, no
duplicate-free condition), `fin_finite` and `pair_finite`, and the
certificate-returning `find_word` with `reachable`, `accepted_word`,
`is_empty` and the four laws, at zero trust. No complexity bound is
promised.
