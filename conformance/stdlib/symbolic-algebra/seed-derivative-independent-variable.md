# A derivative in a semantically unused variable evaluates to zero

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §7.
Frame:
`docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-INDEPENDENT-VARIABLE.md`.

The same expression contains variables zero and one. Variable one occurs
inside a product by zero, so its value does not depend on that variable,
even though a syntactic absence test would fail. Variable zero is used:
its derivative evaluates to one. Closed values do not prove the generic
law, whose premise quantifies over environments.

## Accepted client

The full client must check before the refused claim is appended. Its
public-name proof discharges the independence premise by reducing the
expression's value to its variable-zero value and using agreement at
that index. The closed `Proved` controls evaluate both derivatives on
the same expression and environment; the accepted zero-value control
uses the same proposition and proof form as the refused claim.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, deriv, deriv_independent)

import Core.Logic.Transport (cong)

import Core.Classes.LawfulClasses (leq_nat)

const e1 : Expr = EAdd (EVar Zero) (EMul (EVar (Suc Zero)) (EConst Zero))

fn env23 (v : Nat) : Nat = Suc (Suc v)

theorem one_ne_zero (h : Equal Nat (Suc Zero) Zero) : Bottom =
  cong Nat Bool (Suc Zero) Zero (λn. leq_nat n Zero) h

theorem e1_independent
      (env2 : Nat → Nat)
      (agree : (w : Nat) → (Equal Nat (Suc Zero) w → Bottom) → Equal Nat (env2 w) (env23 w))
    : Equal Nat (eval env2 e1) (eval env23 e1) =
  agree Zero one_ne_zero

theorem e1_deriv_unused : Equal Nat (eval env23 (deriv (Suc Zero) e1)) Zero =
  deriv_independent (Suc Zero) e1 env23 e1_independent

theorem e1_deriv_unused_every_env (env : Nat → Nat) :
  Equal Nat (eval env (deriv (Suc Zero) e1)) Zero =
  deriv_independent (Suc Zero) e1 env (λenv2. λagree. agree Zero one_ne_zero)

theorem e1_unused_value : Equal Nat (eval env23 (deriv (Suc Zero) e1)) Zero = Proved

theorem e1_used_value : Equal Nat (eval env23 (deriv Zero e1)) (Suc Zero) = Proved
```

### stdlib/symbolic-algebra/derivative-semantic-independence

- spec: §7 `deriv_independent` and §5's structural derivative.
- promise class: durable invariant.
- given: `e1` contains both variables but multiplies variable one by
  zero, and `env23` assigns concrete values to both.
- expect: a public-name proof of independence for variable one, a
  public theorem application at `env23` and at arbitrary `env`, and
  direct zero/one values for unused/used derivatives at `env23`.
- measured: a checked premise, two law applications and two separately
  computed `Proved` values on the same expression and environment.
- claimed: whenever evaluation is insensitive to changes of the
  specified variable alone, its derivative there evaluates to zero
  at the given environment.
- gap: the closed expression does not prove the generic statement;
  the catalog's checked proof discharges its quantifiers. The direct
  values only control this seed's selected environment.
- why: the zero-valued product distinguishes semantic independence
  from syntactic absence, while the used-variable value separates
  the true and false derivative conclusions on one input.

## Isolated refused used-variable zero claim

Append only this theorem to a **fresh full copy** of the accepted
client. Its `Equal Nat ... = Proved` form matches the accepted
`e1_unused_value`; its left side uses the same `e1` and `env23` as
`e1_used_value`. Only the differentiated index and the asserted value
change. The refused proposition must fail at this new declaration,
not at the import, independence witness or accepted prefix.

### stdlib/symbolic-algebra/reject-used-variable-zero-derivative

- spec: §7's semantic-independence premise and §5's derivative.
- promise class: durable invariant.
- given: the complete accepted client and the same `e1` and `env23`.
- expect: `eval env23 (deriv Zero e1) = Zero` refuses because the used
  variable's derivative evaluates to `Suc Zero`.
- measured: the accepted prefix checks the `Suc Zero` value on the
  identical operand; the appended zero claim fails at its own proof.
- claimed: the unused-variable theorem does not apply to the variable
  on which the expression's value actually depends.
- gap: this single closed counterexample does not classify all
  dependent expressions; it refutes the proposed zero consequence here.
- why: the same expression and environment carry both the accepted
  one-value control and the refused zero claim, so failure cannot be
  attributed to a missing import or unrelated fixture.

```ken
theorem seed_neg : Equal Nat (eval env23 (deriv Zero e1)) Zero = Proved
```

## Evidence boundary

Architect D0 on `9e1d6e9cd` checked the generic proof and three
private helpers, with `ken check` and `ken fmt --check` rc 0, zero
new `Axiom`, two probe tests passing, and identical scratch trusted
GlobalIds (114 → 114). The checked D0 patch SHA-256 is
`888d83660b97726dc67dc5fbfe966e648f21ba7adcc4db3b8b5991f3a7f5d99d`;
only its catalog hunk belongs to this candidate. Architect's
public-name client accepted, and its same-expression used-variable
zero claim was kernel-refused. Those scratch D0 results are not
shipped seed execution or landed trust.

On this candidate's checked catalog addition, `ken check` and
`ken fmt --check` for the literate NatPolynomial package returned
rc 0. The seed's full accepted Ken fence checked rc 0 as a separate
temporary client; formatting that client and then running
`ken fmt --check` also returned rc 0. A fresh copy of the complete
accepted fence plus **only** `seed_neg` failed rc 1/K0004
`KernelRejected TypeMismatch` at bytes 1154–1225, inside the new
theorem (the diagnostic reports `found: g1`). The positive fence's
direct `e1_used_value` control checks the same derivative at
`Suc Zero`; `e1_unused_value` checks a true
`Equal Nat ... Zero = Proved` in the refused claim's proof form.
The temporary files were removed after checking. Logs under
`/workspaces/ken/local/` are
`spec-author-deriv-independent-catalog-check.log`,
`spec-author-deriv-independent-catalog-fmt.log`,
`spec-author-deriv-independent-positive.log`,
`spec-author-deriv-independent-negative.log`,
`spec-author-deriv-independent-format.log` and
`spec-author-deriv-independent-fmt.log`. This used a preexisting
CLI binary, not one independently built at the exact detached base.
Neither a closed fixture nor a refused claim proves the generic
law or establishes official conformance-runner execution, CI,
publication, landing or landed trust.
