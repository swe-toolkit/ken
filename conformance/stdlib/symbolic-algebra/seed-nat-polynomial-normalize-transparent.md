# Normalization is transparent to canonical polynomial equality

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §9.

This seed supplies closed clients for the three §9 laws. The exact
spec commit is `7a9ab1a3a697accff7dccd53f002ecd0d1f48508`; the
§9 catalog exports and package-owned test are not present on this
seed's base. The spec records Architect D0 scratch checks of the
generic laws and closed fixture. This seed does not claim execution
against a shipped catalog export, official conformance execution, CI,
publication, landing, or landed trust.

Every case has promise class **durable invariant**. The fixed
normal-form output and closed values are contract oracles, not a
snapshot of repository state.

## Accepted client and independent controls

Let `e = (x₀ + 1) · x₁`. The expected normal form is built from the
public constructors, not by calling `normalize`. The accepted client
applies each new public law, separately checks the corresponding
`poly_eq` answer with `Proved`, checks both derivative indices, and
uses a substitution that swaps variables zero and one.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, normalize, poly_eq,
   deriv, substitute, normalize_poly_eq, deriv_normalize,
   substitute_normalize)

const one : Nat = Suc Zero
const eight : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))

const e : Expr =
  EMul (EAdd (EVar Zero) (EConst one)) (EVar one)

const expected_normal_form : Expr =
  EAdd
    (EMul (EVar Zero) (EMul (EVar one) (EConst one)))
    (EAdd (EMul (EVar one) (EConst one)) (EConst Zero))

const expected_swapped_expression : Expr =
  EMul (EAdd (EVar one) (EConst one)) (EVar Zero)

const expected_swapped_normal_form : Expr =
  EAdd
    (EMul (EVar one) (EMul (EVar Zero) (EConst one)))
    (EAdd (EMul (EVar Zero) (EConst one)) (EConst Zero))

fn zero_env (v : Nat) : Nat = Zero

fn env23 (v : Nat) : Nat = Suc (Suc v)

fn swap_variables (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦
      match w {
        Zero ↦ EVar Zero;
        Suc k ↦ EVar (Suc (Suc k))
      }
  }

theorem explicit_normal_form :
  Equal Expr (normalize e) expected_normal_form = Refl

theorem normalize_poly_eq_instance :
  IsTrue (poly_eq (normalize e) e) = normalize_poly_eq e

theorem normalize_poly_eq_oracle :
  IsTrue (poly_eq (normalize e) e) = Proved

theorem deriv_zero_normalize_instance :
  IsTrue (poly_eq (deriv Zero (normalize e)) (deriv Zero e)) =
  deriv_normalize Zero e

theorem deriv_zero_poly_eq_oracle :
  IsTrue (poly_eq (deriv Zero (normalize e)) (deriv Zero e)) = Proved

theorem deriv_one_normalize_instance :
  IsTrue (poly_eq (deriv one (normalize e)) (deriv one e)) =
  deriv_normalize one e

theorem deriv_one_poly_eq_oracle :
  IsTrue (poly_eq (deriv one (normalize e)) (deriv one e)) = Proved

theorem substitute_normalize_instance :
  IsTrue
    (poly_eq
      (substitute swap_variables (normalize e))
      (substitute swap_variables e)) =
  substitute_normalize swap_variables e

theorem substitute_normalize_poly_eq_oracle :
  IsTrue
    (poly_eq
      (substitute swap_variables (normalize e))
      (substitute swap_variables e)) = Proved

theorem swapped_expression_shape :
  Equal Expr (substitute swap_variables e)
    expected_swapped_expression = Refl

theorem swapped_normal_form_shape :
  Equal Expr (substitute swap_variables (normalize e))
    expected_swapped_normal_form = Refl

theorem swapped_expression_value :
  Equal Nat (eval env23 (substitute swap_variables e)) eight = Proved

theorem swapped_normal_form_value :
  Equal Nat
    (eval env23 (substitute swap_variables (normalize e))) eight = Proved

theorem deriv_zero_normalized_value_at_zero :
  Equal Nat (eval zero_env (deriv Zero (normalize e))) Zero = Proved

theorem deriv_zero_original_value_at_zero :
  Equal Nat (eval zero_env (deriv Zero e)) Zero = Proved

theorem deriv_one_normalized_value_at_zero :
  Equal Nat (eval zero_env (deriv one (normalize e))) one = Proved

theorem deriv_one_original_value_at_zero :
  Equal Nat (eval zero_env (deriv one e)) one = Proved

theorem moved_variable_poly_eq_is_false :
  Equal Bool
    (poly_eq (deriv Zero (normalize e)) (deriv one e)) False = Proved
```

### `stdlib/symbolic-algebra/normalize-transparent-law-instances`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §9 and §§1.2, 2.2,
  3, 5.1.
- promise class: durable invariant.
- given: `e = (x₀ + 1) · x₁`, its independently written normal form,
  both derivative indices, and a substitution swapping variables zero
  and one.
- expect: the explicit normal form, all three public law applications,
  and their independently reduced `poly_eq` answers are accepted. The
  swapped forms have value eight under `env23`; the two derivative
  indices have their specified closed values under `zero_env`.
- measured: constructor-level normalization output, four closed
  `poly_eq` answers, three generic-law applications, swapped outputs,
  and derivative values for indices zero and one.
- claimed: normalization preserves canonical semantic equality with
  the source, and derivative and substitution preserve that equality.
- the gap: these closed controls do not establish the generic law
  bodies or a shipped package export.
- why: the expected form is constructed independently; the swapped
  substitution is non-identity; and the distinct derivative values
  keep the two variable indices observable.
- maintenance: adding unrelated expression cases must leave these
  fixed inputs green. Changing §1's distribution contract or the
  §9 preservation laws requires deliberate spec and seed review,
  not refreshing these expected outputs.

## Isolated false moved-variable claim

The accepted client above establishes the same-index `deriv Zero`
control and separately computes the moved-variable decision as
`False`. At the all-zero environment,
`deriv Zero (normalize e)` evaluates to zero while
`deriv (Suc Zero) e` evaluates to one. Check the complete accepted
client first; then append only the following theorem to a fresh copy.
The refusal must occur at this theorem, not in an import, fixture, or
accepted control. The claim has the same `IsTrue (poly_eq ...)` shape
as the accepted `deriv_zero_poly_eq_oracle`; only the derivative index
on the right changes.

### `stdlib/symbolic-algebra/reject-moved-derivative-index-after-normalization`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §9.2 and §§2.2,
  5.1.
- promise class: durable invariant.
- given: the full accepted client, followed by the single false
  same-shape `IsTrue` claim below.
- expect: the accepted prefix succeeds; the appended claim is refused
  at `Proved`. No diagnostic variant or message is part of the oracle.
- measured: the same-index positive control, the independent `False`
  `poly_eq` result, and the zero-versus-one observations on the exact
  operands of the false claim.
- claimed: derivative transparency preserves the same differentiation
  index; it does not equate the normalized expression's derivative
  in variable zero with the original expression's derivative in
  variable one.
- the gap: this one fixture does not characterize every pair of
  derivative indices or prove the generic law.
- why: the accepted same-shape control and independent observations
  distinguish the intended false result from a missing import,
  malformed fixture, or earlier refusal.
- maintenance: future extensions preserving §9 and the derivative
  contract must keep this fixed false result refused. Changing those
  contracts requires deliberate spec and seed review; this is not a
  transition sentinel.

```ken
theorem refuse_moved_derivative_index_after_normalization :
  IsTrue (poly_eq (deriv Zero (normalize e)) (deriv one e)) = Proved
```

The seed intentionally does not assert
`Equal Expr (normalize e) e = Refl`: that is not a valid
structural-normalization oracle, and §9 records that the scratch probe
was refused before a kernel equality check. The closed cases do not
prove the universal theorems, run an official conformance runner,
measure a landed trust delta, or establish CI, publication, or landing.
