# Kernel conformance — seed cases

Format: `../README.md`. These pin the kernel's load-bearing soundness
commitments (`../../spec/10-kernel/README.md §5`) and the three prototype-gap
non-reproductions (`../../spec/10-kernel/README.md §6`).

**Phase tags.** Cases tagged [K2], [K2c], or [K-api] are out of K1 scope -- they
are acceptance criteria for later WPs. The K1 subset is extracted separately in
`seed-k1.md`. Quotient-formation cases stay here; the observational reductions
are detailed in the K2 seed file `observational/seed-observational.md`.

## kernel/universes/type-in-type-rejected (soundness)
- spec: `spec/10-kernel/12-universes.md §1`
- given: a derivation asserting `Type ℓ : Type ℓ` (same level)
- expect: **rejects** (universe inconsistency / level mismatch)
- why: no `Type:Type` (Girard's paradox); the prototype's unchecked universes
  are not reproduced. *(G1, G5.)*

## kernel/universes/predicative-pi (soundness)
- spec: `spec/10-kernel/12-universes.md §2`, `13 §1`
- given: `A : Type 0`, `B : Type 1`, form `(x : A) → B`
- expect: **accepts** at `Type (max 0 1) = Type 1` (predicative `max`)
- why: predicative formation; no universe drop.

## kernel/pi-sigma/dependent-second-projection
- spec: `spec/10-kernel/13-pi-sigma.md §2`
- given: `p : (n : Nat) × Vec A n`; the term `p.2`
- expect: **accepts** with type `Vec A p.1` (dependent second projection)
- why: Σ is genuinely dependent — the prototype's non-dependent Σ is not
  reproduced.

## kernel/pi-sigma/eta (Π and Σ)
- spec: `spec/10-kernel/13-pi-sigma.md §1–2`, `17 §2`
- given: `f : (x:A) → B`; `p : (x:A) × B`
- expect: `f ≡ λ x. f x` and `p ≡ (p.1, p.2)` hold **definitionally**
- why: type-directed η in conversion.

## kernel/identity/j-on-refl [K2]
- spec: `spec/10-kernel/15-identity.md §4`
- given: `J A a P d a (refl a)`
- expect: **reduces-to** `d` (J-β)
- why: the base computation rule.

## kernel/observational/j-on-nonrefl (soundness, the headline non-reproduction) [K2]
- spec: `spec/10-kernel/15-identity.md §4`
- given: `J` applied to a **non-`refl`** canonical equality (e.g. produced by
  `subst`/`cast`/a constructor congruence)
- expect: **reduces** (to a constructor form), does **not** get stuck
- why: closes the prototype's `J`-only-on-`refl` gap, via `cast` (ADR 0005).
  **Fails on any kernel that only reduces `J` on `refl`.**

## kernel/observational/cast-refl (soundness, canonicity/regularity) [K2]
- spec: `spec/10-kernel/16-observational.md §3`
- given: `cast A A refl a`
- expect: **reduces-to** `a` (regularity)
- why: `cast` on a reflexive type-equality is the identity — the clean
  equational theory OTT achieves (and De Morgan cubical does not).

## kernel/observational/funext-definitional (soundness) [K2]
- spec: `spec/10-kernel/16-observational.md §2`
- given: `Eq ((x:A)→B) f g`
- expect: **reduces-to** `(x:A) → Eq B (f x) (g x)` (funext is definitional)
- why: observational `Eq` at a Π-type *is* pointwise equality.

## Quotient terms (K2)

Unless a case tests failed formation, `A / R / e` is formed with
`R : A → A → Ω_l` and a checked `e : IsEquiv A R` (`16 §5`).

## kernel/formation/quotient-requires-checked-equivalence (soundness) [K2]
- spec: `16 §5`
- given: `A := Nat`, `R := λ _ _. Eq Nat 0 1 : A → A → Ω_0`, and an
  `e` whose symmetry and transitivity clauses return their `Bottom` premises,
  but whose reflexivity clause supplies an inhabitant of `Top`; attempt
  `Q := A / R / e` in the empty context.
- expect: quotient formation **rejects** and `trusted_base()` is unchanged.
- why: `Eq Nat 0 1` reduces to `Bottom`, so the candidate does not inhabit
  `IsEquiv A R`. The old zero-one probe is refused at formation, not by a
  neutral class equality. A kernel that skips checking `e` accepts this query.

## kernel/formation/quotient-rejects-first-domain-mismatch (soundness) [K2]
- spec: `16 §5`
- given: `A := Bool`, `R : Nat → Bool → Ω_0`, and a core `Quot` term with
  `e := Top` as its well-scoped third field.
- expect: quotient formation **rejects**; no quotient type is formed. No
  particular error variant or check order is part of the expectation.
- why: the first domain of `R` is not `A`; both relation domains must match
  the carrier.

## kernel/formation/quotient-rejects-second-domain-mismatch (soundness) [K2]
- spec: `16 §5`
- given: `A := Bool`, `R : Bool → Nat → Ω_0`, and a core `Quot` term with
  `e := Top` as its well-scoped third field.
- expect: quotient formation **rejects**; no quotient type is formed. No
  particular error variant or check order is part of the expectation.
- why: the second domain of `R` is not `A`; matching only the first domain
  cannot admit the quotient.

## kernel/formation/quotient-rejects-wrong-relation-level (soundness) [K2]
- spec: `16 §5`; `12 §2`
- given: `A := Nat : Type 0`, `P := Eq (Type 0) Nat Nat : Ω_1`, with
  `refl Nat : P`; `R := λ _ _. P`, and an `e : IsEquiv A R` built from
  that inhabitant in all three clauses.
- expect: quotient formation **rejects** because `R` returns `Ω_1`, not
  the carrier's `Ω_0`.
- why: `e` is valid at `Ω_1`, so this isolates the exact-level premise. A
  level check that accepts any `Ω` would form this quotient.

## kernel/formation/quotient-is-equivalence-clauses (soundness) [K2]
- spec: `16 §5`
- given: context `A : Type 0`, `P : A → Ω_0`; `R x y :=
  (P x → P y) × (P y → P x)`; `e` contains identity reflexivity,
  swapped symmetry, and composed transitivity proofs.
- expect: `Q := A / R / e` **accepts** at `Type 0`; `trusted_base()` is
  unchanged.
- why: the three checked clauses witness the specified equivalence, without
  postulates.

## kernel/formation/quotient-rejects-unswapped-symmetry (soundness) [K2]
- spec: `16 §5`
- given: the same open `A`, `P`, and `R`; `e` uses an identity witness for
  reflexivity, but its symmetry clause returns `h : R x y` unchanged where
  `R y x` is required; transitivity is correct.
- expect: quotient formation **rejects**.
- why: with open `P`, `R x y` and `R y x` are distinct propositions; the
  wrong symmetry direction cannot check.

## kernel/formation/quotient-rejects-wrong-transitivity-premise (soundness) [K2]
- spec: `16 §5`
- given: the same open `A`, `P`, and `R`; `e` has correct reflexivity and
  symmetry, but its transitivity clause returns the first premise `R x y`
  instead of a proof of `R x z` from `R x y` and `R y z`.
- expect: quotient formation **rejects**.
- why: the two endpoints are distinct under open `P`; the wrong transitivity
  result cannot inhabit the required clause.

## kernel/observational/quotient-eq [K2]
- spec: `16 §2.2`, `§5`
- given: a formed `Q := A / R / e`, `R : A → A → Ω_l`, and canonical
  classes `[a]`, `[b]`; query `whnf(Eq Q [a] [b])`.
- expect: **reduces to** `R a b : Ω_l`. With `R := λ _ _. Top` and a
  checked total-equivalence witness, the result is `Top`; an opaque `R`
  may leave its own application neutral, but not the outer `Eq`.
- why: the class-pair rule exposes the checked relation. Other endpoint
  shapes are covered by `observational/quotient-eq-neutral-endpoint`.

## kernel/inductive/elim-computes
- spec: `spec/10-kernel/14-inductive.md §3`
- given: `elim_Nat M z s (suc n)`
- expect: **reduces-to** `s n (elim_Nat M z s n)` (ι)
- why: eliminators compute structurally (the prototype's stubbed sum types do
  not).

## kernel/conversion/sct-accepts-lexicographic [K2c]
- spec: `spec/10-kernel/17-conversion.md §4`
- given: a definition recursing with lexicographic / permuted descent (e.g.
  Ackermann-shaped but well-founded)
- expect: **accepts** as transparent (SCT certifies termination)
- why: SCT is more permissive than single-argument structural recursion.

## kernel/conversion/sct-rejects-nonterminating (soundness) [K2c]
- spec: `spec/10-kernel/17-conversion.md §4`
- given: a non-terminating definition `loop x = loop x`
- expect: **rejects** transparent admission (totality error); δ-unfolding stays
  terminating
- why: keeps conversion (hence type-checking) decidable.

## kernel/judgments/certificate-recheck (soundness) [K-api]
- spec: `spec/10-kernel/18-judgments.md §4`
- given: a prover-produced certificate term `p` and a goal `φ`
- expect: `check_proof p φ` **accepts** iff `p` genuinely has type `φ`; a
  tampered certificate **rejects**
- why: the de Bruijn criterion — the prover is never trusted.
