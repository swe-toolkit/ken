# Zero sums have zero summands

Format: `../../README.md`.
Spec: `spec/50-stdlib/64-rational.md` §4.
Frame: `docs/program/issues/SPEC-NAT-ADD-ZERO-INVERSION-PUBLIC.md`.

Status: authored; not executed. These cases exercise the public attached
proofs for arbitrary operands and at the closed zero-sum boundary. The
closed cases do not establish the generic proof bodies.

## Accepted client

Import `add`; its attached proofs are used through that operation.

```ken
import Data.Numeric.Nat.Arithmetic (add)

theorem add_eq_zero_left_open
    (a : Nat) (b : Nat)
    (sum_zero : Equal Nat (add a b) Zero)
  : Equal Nat a Zero =
    (proof eq_zero_l for add) a b sum_zero

theorem add_eq_zero_right_open
    (a : Nat) (b : Nat)
    (sum_zero : Equal Nat (add a b) Zero)
  : Equal Nat b Zero =
    (proof eq_zero_r for add) a b sum_zero

theorem zero_sum : Equal Nat (add Zero Zero) Zero = Proved

theorem left_zero_summand : Equal Nat Zero Zero =
  (proof eq_zero_l for add) Zero Zero zero_sum

theorem right_zero_summand : Equal Nat Zero Zero =
  (proof eq_zero_r for add) Zero Zero zero_sum

theorem bumped_sum
    : Equal Nat (add (Suc Zero) Zero) (Suc Zero) = Proved
```

### stdlib/numeric/nat-add-zero-inversion-open

- spec: `spec/50-stdlib/64-rational.md` §4.
- promise class: durable invariant.
- given: arbitrary `a`, `b : Nat` and a proof of
  `Equal Nat (add a b) Zero`.
- expect: both attached proofs apply, returning `Equal Nat a Zero`
  and `Equal Nat b Zero`, respectively.
- claimed: each law is available to a generic client through `add`.
- gap: applying the laws does not establish their checked proof bodies.

### stdlib/numeric/nat-add-zero-inversion-closed

- spec: `spec/50-stdlib/64-rational.md` §4.
- promise class: durable invariant.
- given: the accepted client with `zero_sum` for `add Zero Zero`.
- expect: each law proves the corresponding zero summand equality;
  `bumped_sum` proves `add (Suc Zero) Zero = Suc Zero` with `Proved`.
- claimed: the laws accept a closed zero sum, and the same proof form
  accepts the nonzero value of the control sum.
- gap: these closed values do not establish either generic law.

## Isolated refusal

The refusal starts from a fresh copy of the entire accepted client and
appends only the stated declaration. `bumped_sum` is its accepted
same-sum control: it uses `Proved` on `add (Suc Zero) Zero` with the
correct value `Suc Zero`.

### stdlib/numeric/reject-nonzero-left-summand-from-zero-sum

- spec: `spec/50-stdlib/64-rational.md` §4.
- promise class: durable invariant.
- given: the accepted client and `add (Suc Zero) Zero = Suc Zero`.
- expect: reject the attempted left-law application because `Proved`
  cannot establish that this sum equals `Zero`.
- control: `bumped_sum` accepts the same sum and proof form with value
  `Suc Zero`.
- why: the left law cannot conclude that a nonzero first summand is
  zero from a false zero-sum premise.
- gap: this refusal does not establish the left law's generic proof
  body.

```ken
theorem nonzero_left_summand_is_zero
    : Equal Nat (Suc Zero) Zero =
      (proof eq_zero_l for add) (Suc Zero) Zero Proved
```

## Evidence boundary

The Architect's AC-0 ruling reports scratch measurements of the
corresponding generic clients, closed applications, refusal and
control. Those are attributed scratch evidence, not execution of this
seed or an assembled candidate. CV has not run `ken check`, semantic
tests or the official conformance runner. No CI, publication or landing
is claimed. No `local/refs/` or external reference implementation was
consulted.
