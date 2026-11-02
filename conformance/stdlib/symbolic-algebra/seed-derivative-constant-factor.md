# Constant factors pass through differentiation — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §12.

These closed clients exercise the public `deriv_const_factor` law. Their
finite applications do not establish its generic quantifiers.

## Accepted client

Use `two = 2`. The law applies to `EVar Zero` and to the product
`EMul (EVar Zero) (EVar (Suc Zero))`. The same-form `Proved` control
checks the derivative-preserving right side of the refusal below.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EMul, deriv, poly_eq, deriv_const_factor)

const two : Nat = Suc (Suc Zero)

theorem factor_law_at_variable_zero :
  IsTrue (poly_eq
    (deriv Zero (EMul (EConst two) (EVar Zero)))
    (EMul (EConst two) (deriv Zero (EVar Zero)))) =
  deriv_const_factor Zero two (EVar Zero)

theorem factor_law_at_variable_product :
  IsTrue (poly_eq
    (deriv Zero (EMul (EConst two)
      (EMul (EVar Zero) (EVar (Suc Zero)))))
    (EMul (EConst two)
      (deriv Zero (EMul (EVar Zero) (EVar (Suc Zero)))))) =
  deriv_const_factor Zero two (EMul (EVar Zero) (EVar (Suc Zero)))

theorem same_form_proved_control :
  IsTrue (poly_eq
    (deriv Zero (EMul (EConst two) (EVar Zero)))
    (EMul (EConst two) (deriv Zero (EVar Zero)))) = Proved
```

### stdlib/symbolic-algebra/deriv-constant-factor

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §12.
- promise class: durable invariant.
- given: `k = 2`, `v = 0`, and `e = EVar Zero` or the product of
  variables zero and one.
- expect: both public law applications and the same-form `Proved`
  control accept.
- measured: the two law instances and the `Proved` control, on these
  closed expressions only.
- claimed: the public law applies for every `v`, `k`, and expression.
- gap: these closed applications do not establish the generic theorem
  or its proof body.
- why: the law applications exercise the variable and product cases;
  the control establishes that this `IsTrue (poly_eq ...)` form accepts
  `Proved` when the derivative is retained.

## Refused dropped-derivative claim

Use a fresh copy of the complete accepted client above. Append only this
declaration. It keeps the accepted control's left side and proposition
form but drops the derivative from the right side.

```ken
-- Refuse: the right side omits the derivative of EVar Zero.
theorem refuse_dropping_derivative :
  IsTrue (poly_eq
    (deriv Zero (EMul (EConst two) (EVar Zero)))
    (EMul (EConst two) (EVar Zero))) = Proved
```

### stdlib/symbolic-algebra/reject-dropped-derivative

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §12.
- promise class: durable invariant.
- given: a fresh copy of the accepted client and the false claim above.
- expect: reject `refuse_dropping_derivative` because `Proved` cannot
  inhabit the false `IsTrue (poly_eq ...)` claim; the accepted prefix
  and same-form control pass.
- measured: Architect AC-0 reports the accepted prefix and control pass
  and the refused declaration fail with `KernelRejected TypeMismatch`.
- claimed: the derivative of `2 * EVar Zero` is not canonically equal
  to `2 * EVar Zero` under `poly_eq`.
- gap: this one closed refusal does not establish the generic law or
  characterize all expressions.
- why: the control and refusal share the same left side, proposition
  form, and `Proved` proof; only the right side differs. The refusal
  changes that side from `2 * deriv (EVar Zero)` to `2 * EVar Zero`.

## Evidence boundary

Architect AC-0 on base `bddc92ad3` reports both public law applications
and the same-form `Proved` control accepted, and the dropped-derivative
claim refused with `KernelRejected TypeMismatch`. AC-0 also reports no
new `Axiom` and equal `trusted_base()` GlobalId identity sets, 114 to
114. These are Architect scratch-development results, separate from
Foundation's later working-tree package test. Foundation reports its
named `cat_symbolic_algebra_deriv_constant_factor` package test passed
2/2 on the actual CV seed fences and the full preloaded-provider trusted
GlobalId identity sets remained 114→114. This is Foundation's reported
working-tree evidence, not an exact-tip gate, official conformance-runner
execution, CI, publication, landing, or landed trust evidence. CV ran no
tests. The `Equal Expr`/`Refl` probe is excluded: AC-0 measured refusal
at the proof form, so it does not discriminate the dropped-derivative
claim.

## Clean-room provenance

This seed was authored from the landed specification's §2 and §5
behavior and Architect's AC-0 ruling for proposed §12. No external
reference implementation or source was consulted or copied; the cases
are original closed Ken clients.
