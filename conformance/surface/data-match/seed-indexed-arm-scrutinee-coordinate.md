# Indexed arm scrutinee coordinate

Format: `../../README.md`.
Spec: `spec/30-surface/34-data-match.md` §3.3.
Frame: `docs/program/issues/SPEC-INDEXED-ARM-SCRUTINEE-COORDINATE.md`.

This seed pins the constructor-coordinate reading inside an indexed match arm.
It preserves the three D0 rows and adds a derived row for solved-index-name
substitution, which the D0 rows do not isolate. The rule is elaboration-time
name substitution only: it binds no scrutinee equation and adds no kernel
conversion. The packed-equation fork remains open in
`OQ-indexed-scrutinee-evidence`.

The accepted rule row is **RED-UNTIL**
`LANG-SOLVED-INDEX-SUBSTITUTION` AC-1b. Current main refuses it; this seed does
not present it as passing against current main.

## Shared fixture

The fixtures use these indexed families and lookup function:

```ken
data DualFin : Nat -> Type where {
  DualFZ : (n : Nat) -> DualFin (Suc n);
  DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n)
}
data DualEnv (a : Type) : Nat -> Type where {
  DualNil : DualEnv a Zero;
  DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n)
}
fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a =
  match i {
    DualFZ m ↦ match xs { DualCons k x rest ↦ x };
    DualFS m j ↦ match xs {
      DualCons k x rest ↦ dual_lookup a m rest j
    }
  }
```

For `match j` with `j : DualFin k`, the constructor-coordinate reading in the
`DualFZ q` arm is `j = DualFZ q` and `k = Suc q`; in the `DualFS q j2` arm it
is `j = DualFS q j2` and `k = Suc q`.

## surface/data-match/indexed-arm-scrutinee-coordinate-u11-mixed

- spec: `spec/30-surface/34-data-match.md` §3.3.
- promise class: durable invariant, pending the named implementation gate.
- given:

  ```ken
  fn u11 (a : Type) (k : Nat) (j : DualFin k)
    (rest : DualEnv a k) : a =
    match j {
      DualFZ q ↦ dual_lookup a (Suc q) rest j;
      DualFS q j2 ↦ dual_lookup a (Suc q) rest j
    }
  ```

- expect under the rule: accepts. In each arm, substitution gives the
  corresponding constructor for `j` and `Suc q` for `k`; the body is then the
  same as `u11_new` below.
- current-main baseline: refuses with a `DualFin (Suc q)` versus `DualFin k`
  type mismatch because `j` retains its pre-match meaning.
- status: **RED-UNTIL** `LANG-SOLVED-INDEX-SUBSTITUTION` AC-1b; do not count
  current-main refusal as conformance to the specified behavior.
- why: the arm body uses the variable scrutinee name `j`, so the outcome
  distinguishes constructor-coordinate substitution from the current
  pre-match-coordinate reading.

## surface/data-match/indexed-arm-scrutinee-coordinate-u11-new

- spec: `spec/30-surface/34-data-match.md` §3.3.
- promise class: durable invariant.
- given:

  ```ken
  fn u11n (a : Type) (k : Nat) (j : DualFin k)
    (rest : DualEnv a k) : a =
    match j {
      DualFZ q ↦ dual_lookup a (Suc q) rest (DualFZ q);
      DualFS q j2 ↦ dual_lookup a (Suc q) rest (DualFS q j2)
    }
  ```

- expect: accepts both before and after the rule. The body already spells the
  constructor and target index in each arm.
- measured in Architect's D0 report: accepted on the stated base in the
  scratch `arch_coord_probe`.
- claimed: this is an accepted control for the fixture, not evidence for the
  name-substitution rule.
- the gap: because it does not use `j` or `k` in its body, it cannot distinguish
  the old and new readings.

## surface/data-match/indexed-arm-scrutinee-coordinate-solved-index-k

- spec: `spec/30-surface/34-data-match.md` §3.3.
- promise class: durable invariant, pending the named implementation gate.
- given:

  ```ken
  fn index_coordinate_k (a : Type) (k : Nat) (j : DualFin k)
    (rest : DualEnv a k) : DualEnv a (Suc k) =
    match j {
      DualFZ q ↦
        DualCons k (dual_lookup a (Suc q) rest (DualFZ q)) rest;
      DualFS q j2 ↦
        DualCons k (dual_lookup a (Suc q) rest (DualFS q j2)) rest
    }
  ```

- expect under the rule: accepts. In each arm, `k` denotes the constructor's
  target index `Suc q`; `DualCons` then consumes `rest : DualEnv a (Suc q)` and
  constructs the branch result at `DualEnv a (Suc (Suc q))`.
- expected before AC-1b: rejects because `k` retains its pre-match meaning, so
  `DualCons k` requires `rest : DualEnv a k`. This additional row is derived
  from the fixture signatures; it was not measured by Architect's D0 probe or
  independently executed by CV.
- claimed: the solved-index-name substitution works independently of the
  `u11_mixed` scrutinee-name substitution.
- the gap: this closed instance does not establish the rule for every indexed
  family or solved index.

## surface/data-match/indexed-arm-scrutinee-coordinate-r1

- spec: `spec/30-surface/34-data-match.md` §3.3.
- promise class: durable invariant.
- given:

  ```ken
  fn r1 (a : Type) (k : Nat) (j : DualFin k)
    (rest : DualEnv a k) : a =
    match j {
      DualFZ q ↦ dual_lookup a q rest (DualFZ q);
      DualFS q j2 ↦ dual_lookup a (Suc q) rest (DualFS q j2)
    }
  ```

- expect: rejects before and after the rule. In the `DualFZ q` arm, the target
  index is `Suc q` and `rest` has type `DualEnv a (Suc q)`, but this body asks
  `dual_lookup` to use index `q` and therefore expects `DualEnv a q`.
- measured in Architect's D0 report: refused on the stated base with that
  `DualEnv` index mismatch in the scratch `arch_coord_probe`.
- claimed: substitution does not turn a non-coordinate `q` into the constructor
  target index.
- the gap: this one refusal does not characterize all mistyped coordinates.

## Evidence and limits

Architect's D0 report records that the three D0 rows were elaborated and
kernel-checked by a scratch `arch_coord_probe` on base
`71a6dfb6ceb1c42572f54f699e5bb2af91a99901`: `u11_mixed` refused,
`u11_new` accepted, and `r1` refused. These are reported D0 scratch results,
not execution of this conformance seed as a shipped test and not execution of
the implementation after AC-1b. No CI, landing, or publication is claimed.

Provenance is limited to the Ken-owned Spec, conformance conventions and WP
frame, plus the Architect's first-party D0 ruling. No external references,
`local/refs/`, or copyleft material were consulted.
