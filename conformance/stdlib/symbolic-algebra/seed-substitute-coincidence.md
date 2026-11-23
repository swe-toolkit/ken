# Substitution agrees on the variables an expression contains

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §22 (co-turn).
Frame criterion: AC-2. Seed base:
`9716dd74f592368c2233465d2234f8ba0fd477a2`.

These cases are authored but unexecuted. They exercise the public
`substitute_coincidence` application and the occurrence-restricted agreement
premise. They do not test its generic proof body, general runtime behavior, or
trust beyond Architect's D0. The D0 measured the following exact law type:

```ken
pub theorem substitute_coincidence
      (e : Expr) (s : Nat → Expr) (t : Nat → Expr)
    : ((w : Nat) → Equal Bool (occurs w e) True → Equal Expr (s w) (t w))
      → Equal Expr (substitute s e) (substitute t e)
```

The accepted substitutions differ at variables 1 and 2, neither of which
occurs in the fixture. The refused prefix changes only the expression so that
variable 1 occurs; the same agreement body then fails at its index-1 premise.
The checked result controls independently show that the substitutions yield
different expressions there.

## Accepted client

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, occurs, substitute,
    substitute_coincidence)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seven : Nat = Suc (Suc (Suc (Suc (Suc two))))
const nine : Nat = Suc (Suc seven)

const fixture : Expr = EAdd (EVar Zero) (EConst two)

fn moved (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc a ↦
      match a {
        Zero ↦ EConst seven;
        Suc b ↦
          match b {
            Zero ↦ EConst nine;
            Suc c ↦ EVar (Suc (Suc (Suc c)))
          }
      }
  }

theorem agree_on_fixture (w : Nat)
    : Equal Bool (occurs w fixture) True
      → Equal Expr (moved w) (EVar w) =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. absurd h;
        Suc b ↦
          match b {
            Zero ↦ λh. absurd h;
            Suc c ↦ λh. Refl
          }
      }
  }

theorem moved_value :
    Equal Expr (substitute moved fixture) fixture = Refl

theorem identity_value :
    Equal Expr (substitute EVar fixture) fixture = Refl

theorem coincidence_application :
    Equal Expr (substitute moved fixture) (substitute EVar fixture) =
  substitute_coincidence fixture moved EVar agree_on_fixture
```

### symbolic-algebra/substitute-coincidence-off-occurrence-agreement

- spec: §22's occurrence-restricted structural substitution law.
- promise class: durable invariant.
- given: `fixture = EAdd (EVar Zero) (EConst two)`, `moved` sends
  variable 1 to `EConst seven`, variable 2 to `EConst nine`, and every
  other variable to its corresponding `EVar`; the second substitution
  is `EVar`.
- expect: the agreement function and law application check at structural
  result type `Equal Expr`. Both substitutions produce `fixture`.
- measured: the law application uses agreement only under
  `Equal Bool (occurs w fixture) True`. The independent closed result
  equations use `Refl`.
- claimed: substitutions may differ away from the variables occurring
  in an expression while agreeing on every occurring variable; the law
  concludes structural equality of the substituted expressions.
- gap: this closed application does not establish the generic theorem
  or its proof body.
- controls: the two independent substitution equations check both sides
  against `fixture`; a single `substitute_absent` application cannot
  account for differences at both unused variables.
- why: the fixture contains only variable zero, where both substitutions
  are `EVar Zero`; changes at variables one and two are irrelevant to
  this expression.

## Refused agreement at an occurring variable

Append the following declarations to a fresh complete copy of the accepted
client. The accepted prefix and both value controls must check before the
agreement declaration is checked.

```ken
const refused_fixture : Expr = EAdd (EVar one) (EConst two)

theorem moved_refused_value :
    Equal Expr (substitute moved refused_fixture)
      (EAdd (EConst seven) (EConst two)) = Refl

theorem identity_refused_value :
    Equal Expr (substitute EVar refused_fixture) refused_fixture = Refl

theorem agree_on_refused_fixture (w : Nat)
    : Equal Bool (occurs w refused_fixture) True
      → Equal Expr (moved w) (EVar w) =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. absurd h;
        Suc b ↦
          match b {
            Zero ↦ λh. absurd h;
            Suc c ↦ λh. Refl
          }
      }
  }
```

### symbolic-algebra/reject-substitute-coincidence-agreement-at-occurrence

- spec: §22's agreement premise requires equality at each occurring
  variable.
- promise class: durable invariant.
- given: a fresh accepted prefix and the same agreement body, now over
  `refused_fixture = EAdd (EVar one) (EConst two)`.
- expect: the agreement declaration rejects at its index-1 `absurd h`
  with the D0-measured `KernelRejected(TypeMismatch)`. Do not require a
  finer diagnostic span or wording.
- measured: both preceding value controls check by `Refl`; they establish
  `substitute moved refused_fixture` as
  `EAdd (EConst seven) (EConst two)` and
  `substitute EVar refused_fixture` as `refused_fixture`.
- claimed: the law's agreement premise cannot be supplied when the
  substitutions disagree at a variable that occurs in the expression.
- gap: this refusal is at the agreement premise, not an execution of a
  refused law application or a characterization of all invalid premises.
- controls: the accepted law application and its agreement function are
  the positive control. The two independent refused-fixture value
  equations show the disagreement is observable and not a coincidental
  refusal.
- why: relative to the accepted fixture, only the variable occurrence
  changes: variable one is now present. At that index `moved` returns
  `EConst seven` while `EVar` returns `EVar one`; the agreement body
  must reject rather than hide that difference.

## Evidence boundary

Architect's D0 on the exact seed base `9716dd74f592368c2233465d2234f8ba0fd477a2`
measured the accepted law application, both value controls, and the refusal
inside the fresh refused agreement declaration. D0 measured `Proved` for the
agreement's `Zero` arm and `Refl` for its open `Suc c` arm. Closed `Expr`
equations use `Refl`; `Proved` was refused for those equations. These are D0
observations, not execution of this seed. No Ken check, formatter check,
official conformance-runner result, CI, publication, landing, or measured
landed-trust result is claimed. Any runner form not measured by D0 remains a
GAP; no such form has been invented here.
