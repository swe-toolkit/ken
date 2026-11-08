# Normalization congruence — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §15; §2.1–§2.2.

This seed pins the §15 law with the AC-2 commuted-sum acceptance and a
fresh-prefix failed-premise refusal. It is an authored expectation, not a run
of the shipped seed harness. No Ken check, official conformance run, CI,
publication, landing, or trust measurement is claimed.

Use a fresh Legacy-roots provider context matching the existing §2
canonical-equality seed, with `NatPolynomial` and its
`Core.Classes.LawfulClasses` dependency loaded. The client imports only the
selected `NatPolynomial` exports below; it does not add a selective `IsTrue`
import.

## Shared source prefix

In a fresh input, elaborate the complete source below:

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EAdd, EVar, normalize, poly_eq, poly_eq_normalize)

theorem generic_poly_eq_normalize
      (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2))
    : IsTrue (poly_eq (normalize e1) (normalize e2)) =
  poly_eq_normalize e1 e2 h

theorem commuted_sum_premise :
  IsTrue
    (poly_eq
      (EAdd (EVar Zero) (EVar (Suc Zero)))
      (EAdd (EVar (Suc Zero)) (EVar Zero))) = Proved

theorem commuted_sum_normalized :
  IsTrue
    (poly_eq
      (normalize (EAdd (EVar Zero) (EVar (Suc Zero))))
      (normalize (EAdd (EVar (Suc Zero)) (EVar Zero)))) =
  poly_eq_normalize
    (EAdd (EVar Zero) (EVar (Suc Zero)))
    (EAdd (EVar (Suc Zero)) (EVar Zero))
    commuted_sum_premise

theorem distinct_variables_poly_eq_false :
  Equal Bool (poly_eq (EVar Zero) (EVar (Suc Zero))) False = Proved
```

### stdlib/symbolic-algebra/poly-eq-normalize-commuted-sum

- promise class: **normative compatibility vector** — the public law accepts
  the commuted-sum pair with its true premise
- spec: `66 §15`; `66 §2.1–§2.2`
- given: the shared prefix in a fresh roots client with the `NatPolynomial`
  dependency closure
- measured: the generic law application and the closed commuted-sum premise
  and conclusion
- claimed: `poly_eq_normalize` preserves a `True` canonical-equality decision
  under `normalize` for this accepted instance; this finite client does not
  prove the generic law
- the gap: one commuted-sum instance does not establish the theorem for all
  expressions
- expect: all four theorem declarations in the prefix accept.
  `commuted_sum_premise` is the `Proved` witness for the same `poly_eq` pair
  used by `commuted_sum_normalized`; the direct false oracle confirms that the
  distinct-variable pair has no `True` premise.
- controls: the generic client reaches the public law; the concrete
  commuted-sum application is the accepted same-form control.
- why: the operands are structurally different but denote the same polynomial,
  so the positive premise and law application are nontrivial.

### stdlib/symbolic-algebra/poly-eq-normalize-rejects-false-premise

- promise class: **discriminating** — the failed premise, not an earlier parse,
  import, or prefix error, causes the refusal
- spec: `66 §15`; the `poly_eq` result also uses §2.1
- given: in a second fresh environment, repeat the entire shared prefix above
  and append only this law application:

  ```ken
  theorem normalize_congruence_with_false_premise :
    IsTrue
      (poly_eq
        (normalize (EVar Zero))
        (normalize (EVar (Suc Zero)))) =
    poly_eq_normalize (EVar Zero) (EVar (Suc Zero)) Proved
  ```

- measured: acceptance of every declaration in the repeated prefix and the
  failure location of the final law application
- claimed: `Proved` cannot supply the law premise because the distinct
  variables have `poly_eq = False`
- the gap: a rejection before the final law application would not establish the
  failed-premise boundary. The repeated prefix and accepted false-result oracle
  make the refusal attributable.
- expect: the repeated prefix accepts. The final theorem refuses at its own
  law application because `Proved` does not have the required
  `IsTrue (poly_eq (EVar Zero) (EVar (Suc Zero)))` type. A parse error,
  unbound name, or failure in an earlier declaration does not satisfy this
  case.
- controls: `commuted_sum_normalized` is the accepted application of the same
  law with a valid `Proved` premise; `distinct_variables_poly_eq_false` is the
  accepted control for the false-premise input.
- why: the two applications use the same law and differ at the premise's
  expression pair. No `Equal Expr` refusal is used; §9.2 excludes that
  shortcut.
- evidence boundary: these are specified source expectations, not execution
  results. The scratch D0 result is separate evidence, not a shipped seed,
  official conformance run, CI result, publication, landing, or measured trust.
