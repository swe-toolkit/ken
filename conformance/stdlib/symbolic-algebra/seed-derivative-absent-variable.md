# A derivative in a syntactically absent variable is zero

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §14.

The same expression contains variables zero and one. Variable one occurs
inside a product by zero, so its value does not depend on that variable,
even though syntactic absence fails. The §14 law is stricter: it applies
only when the differentiated variable does not occur syntactically. The
closed rows below exercise the same public theorem form; they do not
prove its generic quantifiers.

## Accepted client

The open theorem `scaled_absent` is generic in the expression `x`,
variable `v`, coefficient `k` and environment `env`. Its premise is
syntactic absence from `x`; the constant factor adds no occurrence, so
the public law proves the scaled expression's derivative evaluates to
zero. Let `e1` be the §7 expression. The accepted closed control applies
`deriv_absent` to an index absent from `e1`; its `occurs` premise
reduces to false. The two refused claims use that same theorem
application and proof form, changing only the index. In both refused
rows the proposed `Proved` witness is for a false premise because the
index occurs. The second index is semantically unused, distinguishing
syntactic absence from §7's semantic independence.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, occurs, deriv, deriv_absent)

theorem scaled_absent
      (v : Nat) (k : Nat) (x : Expr) (env : Nat → Nat)
    : Equal Bool (occurs v x) False → Equal Nat (eval env (deriv v (EMul (EConst k) x))) Zero =
  λabsent. deriv_absent v (EMul (EConst k) x) env absent

const e1 : Expr = EAdd (EVar Zero) (EMul (EVar (Suc Zero)) (EConst Zero))

fn env23 (v : Nat) : Nat = Suc (Suc v)

theorem absent_variable_zero : Equal Nat (eval env23 (deriv (Suc (Suc Zero)) e1)) Zero =
  deriv_absent (Suc (Suc Zero)) e1 env23 Proved
```

### stdlib/symbolic-algebra/derivative-absent-variable-generic

- spec: §14's generic syntactic-absence law and §5's structural
  derivative.
- promise class: durable invariant.
- given: arbitrary `x : Expr`, `v : Nat`, `k : Nat` and
  `env : Nat → Nat`, with `Equal Bool (occurs v x) False`.
- expect: the generic client is accepted for the derivative of the
  scaled expression.
- measured: `scaled_absent` retains open binders and applies the public
  theorem to `EMul (EConst k) x` without specializing the inputs.
- claimed: absence from `x` suffices for the derivative of a constant
  multiple of `x` to evaluate to zero under every environment.
- gap: this client uses the public theorem; it does not prove its
  generic implementation or establish official runner execution.
- why: an open expression and explicit absence premise ensure this
  seed itself exercises the generic interface, rather than relying on
  a Rust-side client.

### stdlib/symbolic-algebra/derivative-absent-variable

- spec: §14's syntactic-absence law and §5's structural derivative.
- promise class: durable invariant.
- given: `e1`, which contains variables zero and one, and the closed
environment `env23`.
- expect: the public theorem application accepts for the absent index
  `Suc (Suc Zero)`.
- measured: one closed application of `deriv_absent`, with its
  `Equal Bool (occurs ...) False` premise discharged by `Proved`.
- claimed: the law accepts when the selected variable is syntactically
  absent from the expression.
- gap: this closed instance does not prove the theorem's generic
  quantifiers or establish that absence is necessary for every
  zero-valued derivative.
- why: the accepted control fixes the expression, environment, result
  type and proof form used by both refused rows.

## Refused occurring-variable zero claim

Append only the following declaration to a **fresh full copy** of the
accepted client. Its theorem application has the same form as the
accepted control. The selected index changes from absent `Suc (Suc
Zero)` to occurring `Zero`; therefore the `Proved` term cannot establish
the required false `occurs` premise. The refusal must be at this new
declaration, not in an import or the accepted prefix.

### stdlib/symbolic-algebra/reject-occurring-variable-absent-law

- spec: §14's syntactic-absence premise.
- promise class: durable invariant.
- given: the complete accepted client and the same `e1` and `env23`.
- expect: `deriv_absent` refuses the claimed zero result for occurring
  variable `Zero` at the supplied `Proved` premise.
- measured: the accepted absent-index application and this same-form
  refusal differ only in the selected variable.
- claimed: the absence law cannot be invoked with a false occurrence
  premise.
- gap: this row does not characterize all occurring variables or
  their derivative values.
- why: the identical theorem application shape makes the premise's
  truth, rather than a changed conclusion or proof form, determine the
  verdict.

```ken
theorem refuse_occurring_zero : Equal Nat (eval env23 (deriv Zero e1)) Zero =
  deriv_absent Zero e1 env23 Proved
```

## Refused occurring-but-unused variable

Append only this declaration to another **fresh full copy** of the
accepted client. Variable `Suc Zero` occurs in the factor multiplied
by zero, so its derivative evaluates to zero, as §7's
`deriv_independent` case establishes. Nonetheless, it fails the
syntactic-absence premise of §14. The refusal must be at this new
declaration, specifically the `Proved` premise, not in the accepted
prefix.

### stdlib/symbolic-algebra/reject-occurring-unused-absent-law

- spec: §14's syntactic-absence premise, contrasted with §7's
  semantic-independence law.
- promise class: durable invariant.
- given: the complete accepted client and the same `e1` and `env23`.
- expect: `deriv_absent` refuses for occurring-but-unused variable
  `Suc Zero`, despite the derivative's zero value.
- measured: the accepted control and this row share the theorem
  application form; the index alone changes.
- claimed: semantic independence does not satisfy §14's syntactic
  premise when the variable occurs.
- gap: this one expression does not characterize the relationship
  between syntactic occurrence and semantic dependence in general.
- why: the same proposition and proof form as the accepted control
  isolate occurrence; the row separates §14 from §7.

```ken
theorem refuse_occurring_unused : Equal Nat (eval env23 (deriv (Suc Zero) e1)) Zero =
  deriv_absent (Suc Zero) e1 env23 Proved
```

## Evidence boundary

Architect's AC-2 ruling `evt_3qykdakw6g7gj` specifies these three rows
and explicitly excludes `Refl`, which refuses even the true control.
The ruling reports that `accept_client_v2` imports `IsTrue` from
`Core.Classes.LawfulClasses`; these fences do not name `IsTrue`, so no
such import is needed here. No external or `local/refs` source was
consulted. CV has not run Ken checks, package tests or the official
conformance runner on this seed. The rows specify expected behavior;
they are not evidence of candidate execution, CI, publication, landing
or landed trust.
