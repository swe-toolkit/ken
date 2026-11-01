# Canonical polynomial equality is an equivalence

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §11.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-POLY-EQ-EQUIVALENCE.md`.

This seed exercises the three public §11 laws on a closed client. Its
accepted control and refusal use the same `IsTrue` goal shape and
`Proved` terminal. The finite instances exercise the law interfaces;
they do not prove the generic theorem bodies. This seed is authored,
not Ken-checked or executed by the official conformance runner.

Every case has promise class **durable invariant**.

## Accepted client

The client uses `x0_sum = x0 + x0` and
`two_times_x0 = 2 * x0`, which have the same canonical polynomial.
`x1` is the next variable. The accepted `same_form_control` supplies
the forward premise; symmetry returns to `x0_sum`, and transitivity
closes both round trips. `reflexivity_by_public_law` exercises the
public reflexivity theorem. This uses the named theorem
`poly_eq_refl`, not the kernel terminal `Refl`.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq,
   poly_eq_refl, poly_eq_sym, poly_eq_trans)

const one : Nat = Suc Zero
const two : Nat = Suc one

const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const x0_sum : Expr = EAdd x0 x0
const two_times_x0 : Expr = EMul (EConst two) x0

theorem same_form_control :
  IsTrue (poly_eq x0_sum two_times_x0) = Proved

theorem reverse_by_symmetry :
  IsTrue (poly_eq two_times_x0 x0_sum) =
  poly_eq_sym x0_sum two_times_x0 same_form_control

theorem round_trip_by_transitivity :
  IsTrue (poly_eq x0_sum x0_sum) =
  poly_eq_trans x0_sum two_times_x0 x0_sum
    same_form_control reverse_by_symmetry

theorem reverse_round_trip_by_transitivity :
  IsTrue (poly_eq two_times_x0 two_times_x0) =
  poly_eq_trans two_times_x0 x0_sum two_times_x0
    reverse_by_symmetry same_form_control

theorem reflexivity_by_public_law :
  IsTrue (poly_eq x1 x1) = poly_eq_refl x1
```

### stdlib/symbolic-algebra/poly-eq-equivalence-closed-client

- spec: §11 and §2.1–§2.2.
- promise class: durable invariant.
- given: `x0_sum`, `two_times_x0`, and `x1` from the accepted client.
- expect: the same-form `Proved` control accepts; symmetry and
  transitivity close both round trips; the public reflexivity theorem
  closes the `x1` instance.
- measured: source authored only; this seed has not been executed.
- claimed: the three public relational laws are usable on this closed
  equal pair and reflexive instance.
- gap: these finite applications do not prove the laws for every
  expression; the generic bodies are a separate catalog artifact.
- maintenance: adding expression constructors while preserving the
  §2 decision and §11 laws must leave this client green. Contract
  changes require spec and seed review.
- why: the two equal expressions are structurally different, and the
  proof chain exercises symmetry and transitivity in both directions.

## Isolated refused unequal-expression claim

On a fresh complete copy of the accepted client above, append only the
following theorem. The `same_form_control` remains an accepted
same-form `Proved` control. At the environment that returns `one` for
every variable, `x0_sum` evaluates to `one + one = two`, while `x1`
evaluates to `one`; the expressions are not equal as polynomials. The
refusal must be at this claim. Do not use kernel `Refl` for either
row or a transitivity application with a false premise.

```ken
theorem reject_unequal_expressions_as_equal :
  IsTrue (poly_eq x0_sum x1) = Proved
```

### stdlib/symbolic-algebra/reject-unequal-expressions-as-equal

- spec: §11 and §2.1–§2.2.
- promise class: durable invariant.
- given: a fresh accepted prefix with the same-form control, followed
  by only `reject_unequal_expressions_as_equal`.
- expect: the prefix accepts and the false `IsTrue` claim is refused
  at its own theorem. No diagnostic variant or message is pinned.
- measured: source authored only; this seed has not been executed.
- claimed: `poly_eq` does not report universal equality for
  `x0_sum` and `x1`.
- gap: this counterexample does not prove the generic laws or
  characterize every pair of unequal expressions.
- maintenance: preserving the §2 canonical decision and §11
  equivalence laws keeps this counterexample refused; a contract
  change requires spec and seed review.
- why: the false claim and accepted control share the left operand,
  goal shape, and proof terminal; only the right expression changes.
  The all-ones witness gives distinct natural values.

## Evidence boundary

Architect's AC-0 checked the generic law bodies in scratch. Foundation's
exact-tip package test separately reported 2/2 for the named target and
trusted identity sets 114 to 114; it covers the catalog client, not this
seed. This seed is authored and unexecuted. No official conformance
execution, CI, publication, landing, or landed-trust result is claimed.
