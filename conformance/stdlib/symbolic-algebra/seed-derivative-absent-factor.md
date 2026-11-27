# A syntactically absent factor passes through differentiation

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §25.

The public law applies when the differentiated variable is syntactically
absent from the left factor. The open client fixes `v = 0` and
`c = EVar 1`, leaving the expression `e` open. A closed accepted row
shows that the corresponding factoring claim is false when the factor is
`EVar 0` and `e = EConst 1`.

## Accepted client

The open theorem applies the public law with its absence premise closed
by computation. It does not close the derivative equality by computation
alone. The accepted closed row gives a concrete counterexample for the
present factor.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EMul, deriv, poly_eq, deriv_absent_factor)

theorem factor_absent_open (e : Expr) :
  IsTrue (poly_eq (deriv Zero (EMul (EVar (Suc Zero)) e))
                  (EMul (EVar (Suc Zero)) (deriv Zero e))) =
  deriv_absent_factor Zero (EVar (Suc Zero)) e Proved

theorem factor_present_decides_false :
  Equal Bool
    (poly_eq
      (deriv Zero (EMul (EVar Zero) (EConst (Suc Zero))))
      (EMul (EVar Zero) (deriv Zero (EConst (Suc Zero)))))
    False = Proved
```

### stdlib/symbolic-algebra/derivative-absent-factor-open

- spec: §25's public law and syntactic-absence premise.
- promise class: durable invariant.
- given: the accepted open theorem above, with `v = Zero`,
  `c = EVar (Suc Zero)`, and arbitrary `e : Expr`.
- expect: `factor_absent_open` is accepted.
- measured: an open-in-`e` client applies the public theorem; its
  occurrence premise reduces to false.
- claimed: the public law is applicable to this factor for every `e`.
- gap: this fixed `v` and `c` application does not establish the law's
  full quantification or its implementation.
- why: the open expression prevents the conclusion from being closed by
  computation alone; the proof uses the public law.

### stdlib/symbolic-algebra/derivative-present-factor-is-not-equal

- spec: §25's absence-only premise and `poly_eq` conclusion.
- promise class: durable invariant.
- given: `v = Zero`, `c = EVar Zero`, and `e = EConst (Suc Zero)`.
- expect: the equality decision is `False`, as proved by the accepted
  `factor_present_decides_false` row.
- measured: one closed `poly_eq` result for the present factor.
- claimed: this instance of the proposed factoring equation is false.
- gap: this counterexample does not characterize every present factor
  or every expression.
- why: the row shows the refused twin's conclusion is false for a
  concrete expression, rather than merely unproved by the absent-factor
  law.

## Refused present-factor application

Append only the declaration below to a fresh full copy of the accepted
client. The imports and both accepted declarations must check first. The
refusal must occur inside `factor_present_refused`, not in its prefix.

```ken
-- Refuse: the factor contains the differentiated variable.
theorem factor_present_refused (e : Expr) :
  IsTrue (poly_eq (deriv Zero (EMul (EVar Zero) e))
                  (EMul (EVar Zero) (deriv Zero e))) =
  deriv_absent_factor Zero (EVar Zero) e Proved
```

### stdlib/symbolic-algebra/reject-present-factor-as-absent

- spec: §25's syntactic-absence premise.
- promise class: durable invariant.
- given: a fresh copy of the complete accepted client, followed by
  `factor_present_refused` with `c = EVar Zero`.
- expect: the new declaration is rejected inside its own body with
  `KernelRejected(TypeMismatch)`. Do not pin a span or message wording.
- measured: the accepted prefix succeeds and the same-form theorem
  application fails when only the factor changes to `EVar Zero`.
- claimed: the absent-factor law rejects this application when its
  premise reduces to `True = False`.
- gap: this row does not claim every present factor makes the factoring
  equation false; the separate closed row witnesses one false instance.
- why: the accepted and refused applications share their result form,
  open expression, differentiated variable, and proof; only the factor
  changes from absent to present.

## Evidence boundary

These are authored expectations grounded in Architect AC-0
(`evt_35sf9zt19dpsm`) and the §25 contract. The official conformance
runner has not executed this seed. A targeted package test that consumes
these fences would be implementation evidence, not official seed
execution. No CI, publication, landing, or trusted-base measurement is
claimed.

## Clean-room provenance

No external reference implementation or source was consulted. These
cases are stated in Ken's terms from the AC-0 ruling and the §25
behavioral contract.
