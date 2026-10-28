# Derivative evaluation after substitution

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §8.
Frame:
`docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-SUBSTITUTION.md`.

At a fixed environment, the law requires the replacement for the differentiated
variable to have derivative one, and every replacement for a distinct variable
to have derivative zero. The accepted `shift_off` fixture meets both premises
at `env23`. The refused `merge_off` fixture keeps the first premise but sends a
distinct variable to `EVar Zero`, so its derivative is not zero. In each case
the conclusion is checked on the same `xy = x₀ · x₁` input. These closed
observations do not prove the generic theorem.

## Accepted client and independent values

The full client must check before the refused claim is appended. Its premise
proof establishes the zero-elsewhere condition by splitting the source index;
the distinguished index is impossible under the premise's inequality. Direct
`Proved` controls independently compute both sides of the accepted and refused
fixtures. The `shift_off` same-shape control proves the exact equality shape
that the refused `merge_off` claim later attempts.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, deriv, substitute,
   compose_env, deriv_substitute)

import Core.Classes.LawfulClasses (leq_nat)

fn env23 (v : Nat) : Nat = Suc (Suc v)

const two : Nat = Suc (Suc Zero)
const four : Nat = Suc (Suc (Suc (Suc Zero)))
const xy : Expr = EMul (EVar Zero) (EVar (Suc Zero))

fn shift_off (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc k ↦ EAdd (EVar (Suc k)) (EConst (Suc Zero))
  }

fn merge_off (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc k ↦ EVar Zero
  }

theorem shift_off_zero_elsewhere
      (w : Nat)
    : (Equal Nat Zero w → Bottom)
      → Equal Nat (eval env23 (deriv Zero (shift_off w))) Zero =
  match w {
    Zero ↦ λhn. absurd (hn Proved);
    Suc k ↦ λhn. Proved
  }

theorem shift_off_derivative_at_zero :
  Equal Nat (eval env23 (deriv Zero (shift_off Zero))) (Suc Zero) = Proved

theorem shift_off_law_instance
    : Equal Nat
        (eval env23 (deriv Zero (substitute shift_off xy)))
        (eval (compose_env env23 shift_off) (deriv Zero xy)) =
  deriv_substitute Zero shift_off xy env23 Proved shift_off_zero_elsewhere

theorem shift_off_left_value :
  Equal Nat (eval env23 (deriv Zero (substitute shift_off xy))) four = Proved

theorem shift_off_right_value :
  Equal Nat
    (eval (compose_env env23 shift_off) (deriv Zero xy)) four = Proved

theorem shift_off_same_shape_control
    : Equal Nat
        (eval env23 (deriv Zero (substitute shift_off xy)))
        (eval (compose_env env23 shift_off) (deriv Zero xy)) =
  Proved

theorem merge_off_derivative_at_zero :
  Equal Nat (eval env23 (deriv Zero (merge_off Zero))) (Suc Zero) = Proved

theorem merge_off_derivative_at_one :
  Equal Nat
    (eval env23 (deriv Zero (merge_off (Suc Zero)))) (Suc Zero) = Proved

theorem merge_off_left_value :
  Equal Nat (eval env23 (deriv Zero (substitute merge_off xy))) four = Proved

theorem merge_off_right_value :
  Equal Nat
    (eval (compose_env env23 merge_off) (deriv Zero xy)) two = Proved

theorem merge_off_sides_order_false
    : Equal Bool
        (leq_nat
          (eval env23 (deriv Zero (substitute merge_off xy)))
          (eval (compose_env env23 merge_off) (deriv Zero xy)))
        False =
  Proved
```

### stdlib/symbolic-algebra/derivative-after-identity-slope-substitution

- spec: §8.1 and §8.2.
- promise class: durable invariant.
- given: `xy = EVar Zero · EVar (Suc Zero)`, `env23 v = v + 2`,
  `shift_off Zero = EVar Zero`, and each successor image is its own variable
  plus one.
- expect: `shift_off` has derivative one at `Zero` and derivative zero at
  every successor. `deriv_substitute` accepts at `env23`; the direct left and
  right values are both four.
- measured: one accepted generic-law application, its required zero-elsewhere
  premise proof, independent endpoint values, and a same-shape accepted
  `Proved` control.
- claimed: under the two pointwise derivative premises, differentiating a
  substituted expression evaluates as evaluating the original derivative in
  the composed environment.
- gap: this closed instance does not discharge the universally quantified
  theorem or establish a trust-base measurement.
- why: the shift leaves the differentiated variable unchanged and adds a
  constant to each other variable, so both premises hold and both sides
  independently reduce to four.

The expected values follow from the defining equations. Under `env23`, variables
zero and one evaluate to two and three. Substitution yields `x₀ · (x₁ + 1)`,
whose derivative in zero evaluates to four. The original derivative is `x₁`,
which evaluates to four under `compose_env env23 shift_off` as well.

## Refused merged-variable conclusion

Append only the following theorem to a fresh full copy of the accepted client.
The refusal must occur at this declaration's `Proved`, not in an import,
fixture, premise proof, or accepted prefix. `merge_off` still has derivative
one at zero, but its successor image is also `EVar Zero`, whose derivative is
one rather than zero. The accepted same-shape control uses `shift_off` on the
same expression and environment. The direct values and Boolean comparison
above independently establish that this `merge_off` conclusion is false.

### stdlib/symbolic-algebra/reject-derivative-after-merged-substitution

- spec: §8.1 and §8.2.
- promise class: durable invariant.
- given: the complete accepted client and the same `xy` and `env23`.
- expect: the following false `Equal Nat` proposition is rejected at its own
  `Proved`; `merge_off` violates the zero-elsewhere premise at successor one.
- measured: the accepted controls compute the left side as four, the right
  side as two, and `4 ≤ 2` as `False`.
- claimed: the two premises are necessary for this derivative/substitution
  equality; the generic theorem does not license an arbitrary substitution.
- gap: one closed counterexample does not classify every substitution that
  violates a premise or prove the conditional theorem's quantifiers.
- why: the accepted `shift_off` equality and the same-shaped refused claim
  differ only in the substitution; independent endpoint and False controls
  attribute refusal to the false conclusion, not to the harness.

```ken
theorem merge_off_commutation_rejected
    : Equal Nat
        (eval env23 (deriv Zero (substitute merge_off xy)))
        (eval (compose_env env23 merge_off) (deriv Zero xy)) =
  Proved
```

## Evidence boundary

Architect's AC-0 D0 on landed base `97caadd38871b23c7796b219681d0c054b092dec`
checked the generic law, the `shift_off` accepted client, the `merge_off`
refusal, and the premise counterexamples with no new `Axiom` and equal scratch
`trusted_base()` identity sets (114 → 114). The AC-0 catalog patch SHA-256 is
`de8de0a3fb9ffd1a6f585949f3304db8146ee25056462c6eb7eed14233f4edab`. These are
isolated scratch checks, not execution of this seed against
a shipped catalog export, official conformance execution, CI, publication,
landing, or landed trust. The §8 catalog export is absent on this branch.
