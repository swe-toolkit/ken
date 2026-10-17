# Canonical equality for natural-coefficient polynomials

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §2.

This seed supplies closed operation oracles for the prospective §2 extension
of `Algorithm.SymbolicAlgebra.NatPolynomial`. It does not replace or extend
`seed-nat-polynomial.md`, whose cases remain about §1's ordered distributed
monomials. The assertions below are derived from the §2 contract and the
checked `Nat` arithmetic interface. They do not claim that §2 catalog code or
this seed has shipped or run.

Every case has promise class **durable invariant**.

## Shared fixtures and direct operation oracles

`env1` maps variable zero to two and every successor variable to three. The
expressions `x0_sum` and `two_times_x0` are structurally different: the first
adds two variable expressions, and the second multiplies a constant by one
variable expression. Section 2 requires both to canonicalize to the same
Horner form and `poly_eq` to return `True`. The value `x1_horner` retains a
zero second child because its first child is nonconstant; it represents
variable one, not variable zero.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, Poly, PConst, PHorner,
   eval, eval_poly, canonicalize, poly_eq,
   canonicalize_sound, canonicalize_complete,
   poly_eq_sound, poly_eq_complete, poly_eq_false_refutes)
import Core.Logic.Transport (cong, sym, trans)
import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_l)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc (Suc two)

fn env1 (v : Nat) : Nat =
  match v { Zero ↦ two; Suc w ↦ three }

const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const x0_sum : Expr = EAdd x0 x0
const two_times_x0 : Expr = EMul (EConst two) x0
const canonical_two_times_x0 : Poly = PHorner (PConst Zero) (PConst two)
const x1_horner : Poly =
  PHorner (PHorner (PConst Zero) (PConst one)) (PConst Zero)

theorem equal_x0_sum_and_two_times_x0 (env : Nat → Nat)
    : Equal Nat (eval env x0_sum) (eval env two_times_x0) =
  trans
    Nat
    (add (env Zero) (env Zero))
    (add (mul one (env Zero)) (mul one (env Zero)))
    (mul two (env Zero))
    (cong
      Nat
      Nat
      (env Zero)
      (mul one (env Zero))
      (λn. add n n)
      (sym Nat
        (env Zero)
        (mul one (env Zero))
        ((proof one_l for mul) (env Zero)))
    (sym
      Nat
      (mul two (env Zero))
      (add (mul one (env Zero)) (mul one (env Zero)))
      (mul_add_distrib_l one one (env Zero)))

theorem eval_x0_sum_oracle :
  Equal Nat (eval env1 x0_sum) four = Proved

theorem eval_two_times_x0_oracle :
  Equal Nat (eval env1 two_times_x0) four = Proved

theorem eval_horner_x0_double_oracle :
  Equal Nat (eval_poly env1 canonical_two_times_x0) four = Proved

theorem eval_horner_x1_oracle :
  Equal Nat (eval_poly env1 x1_horner) three = Proved

theorem eval_noncanonical_constant_horner_oracle :
  Equal Nat (eval_poly env1 (PHorner (PConst three) (PConst Zero))) three = Proved

theorem canonicalize_constant_oracle :
  Equal Poly (canonicalize (EConst two)) (PConst two) = Proved

theorem canonicalize_x0_sum_oracle :
  Equal Poly (canonicalize x0_sum) canonical_two_times_x0 = Refl

theorem canonicalize_two_times_x0_oracle :
  Equal Poly (canonicalize two_times_x0) canonical_two_times_x0 = Refl

theorem poly_eq_equal_pair_oracle :
  IsTrue (poly_eq x0_sum two_times_x0) = Proved

theorem poly_eq_unequal_variables_oracle :
  Equal Bool (poly_eq x0 x1) False = Proved
```

### `stdlib/symbolic-algebra/canonical-equality-closed-operations`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §§2.1–2.2.
- given: the structurally different `x0_sum` and `two_times_x0`, plus
  closed `Poly` values under `env1`.
- expect: both expressions canonicalize to
  `PHorner (PConst Zero) (PConst two)`; their decision is `True`. The
  decision for `x0` and `x1` is `False`.
- measured: `canonicalize`, `poly_eq`, and `eval_poly`; the evaluator
  cases exercise a zero second child below a nonconstant head and the
  specified shift of variable indices.
- claimed: canonical Horner comparison decides the equal pair, while
  the evaluator follows the recursive `PHorner` interpretation.
- the gap: these closed inputs do not establish canonicity or semantic
  equality for all expressions and environments.
- why: an ordered-monomial-list comparison returns `False` for
  `x0_sum` and `two_times_x0`; a constant-`True` comparator returns
  the wrong result for `x0` and `x1`.

## Closed instances of the five generic laws

The proof `equal_x0_sum_and_two_times_x0` is for every total environment.
It derives `x + x = 2·x` from the checked multiplication unit and left
distributivity, rather than assuming the law being tested. It supplies the
nontrivial semantic premise for `canonicalize_complete` and
`poly_eq_complete`.

```ken
theorem canonicalize_sound_instance :
  Equal Nat (eval_poly env1 (canonicalize x0_sum)) (eval env1 x0_sum) =
  canonicalize_sound env1 x0_sum

theorem canonicalize_complete_instance :
  Equal Poly (canonicalize x0_sum) (canonicalize two_times_x0) =
  canonicalize_complete
    x0_sum two_times_x0 equal_x0_sum_and_two_times_x0

theorem poly_eq_sound_instance :
  Equal Nat (eval env1 x0_sum) (eval env1 two_times_x0) =
  poly_eq_sound x0_sum two_times_x0 Proved env1

theorem poly_eq_complete_instance :
  IsTrue (poly_eq x0_sum two_times_x0) =
  poly_eq_complete x0_sum two_times_x0 equal_x0_sum_and_two_times_x0

theorem poly_eq_false_refutes_instance
    (hf : Equal Bool (poly_eq x0_sum two_times_x0) False)
  : Bottom =
  poly_eq_false_refutes
    x0_sum two_times_x0 hf equal_x0_sum_and_two_times_x0
```

### `stdlib/symbolic-algebra/canonical-equality-law-instances`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §2.2.
- given: the equal pair and its independently derived equality under
  every environment.
- expect: each of the five exported theorem applications has its
  specified closed expression inputs and result type. The
  `poly_eq_false_refutes` application is conditional on `hf`: a false
  answer for this universally equal pair is intentionally impossible to
  construct when `poly_eq` conforms.
- measured: one closed instance of `canonicalize_sound`,
  `canonicalize_complete`, `poly_eq_sound`, `poly_eq_complete`, and
  `poly_eq_false_refutes`.
- claimed: the five generic laws have the exact §2.2 contracts; no
  closed instance is treated as a proof of a generic law.
- the gap: these instances do not verify the future catalog proof
  bodies for arbitrary inputs.
- why: the equal pair makes the completeness and soundness premises
  substantive; the false-refutation application keeps its necessary
  false-answer premise explicit instead of asserting a false closed
  proof.

## Refuse a false equality claim

The preceding positive control establishes that `poly_eq x0 x1` is
`False`. In a separate checker input, assert the opposite proposition:

```ken
theorem reject_unequal_variables_as_equal :
  IsTrue (poly_eq x0 x1) = Proved
```

### `stdlib/symbolic-algebra/reject-unequal-variables-as-equal`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §2.2.
- promise class: durable invariant.
- given: the same closed `x0`/`x1` pair as
  `poly_eq_unequal_variables_oracle`.
- expect: the positive control accepts `False`; this separate false
  `IsTrue` claim is refused at its own theorem.
- measured: the decision boundary on the same pair, not an unrelated
  parse or name-resolution error.
- claimed: `x0` and `x1` are not universally equal, and their decision
  is not `True`.
- the gap: this case does not produce a concrete environment witness
  as a consequence of `poly_eq_false_refutes`.
- why: the spec's `False` result is incompatible with the asserted
  `IsTrue` proposition.

## Coverage boundary

The direct block covers all three §2 operations, the nontrivial equal
pair, an unequal pair, the evaluator's shifted-head interpretation, and
both a retained zero second child and a noncanonical constant Horner
input. The law block instantiates all five §2.2 theorems, including the
conditional false-refutation theorem. The separate negative input refuses
a false `True` claim with a same-pair positive control. None of these cases
proves a generic law, measures a trusted-base delta, executes a shipped
catalog seed, or establishes CI or landing.