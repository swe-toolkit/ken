# Natural-coefficient polynomial expressions

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.

These are **authored closed behavioral oracles** for the prospective
`Algorithm.SymbolicAlgebra.NatPolynomial` module. Architect's full
D0 source (SHA256
`6eba5226846c340539834c849a093b730bcf553e702aa5bc6a58de094782f940`)
checked, rc=0. A scratch copy with **only** the five explicit
constructor exports required by `33 §4.1` also checked, rc=0
(SHA256
`70696428dcf0be5832cb528529f3ad5841daf428ed9dca214e07190400b40642`).
Against that scratch module, the positive block below was extracted
verbatim to `SeedPositive.ken` (SHA256
`b9f1fce33dc1f4394376b8d4f7dd5ef9ca1c2d1c96c716c55d96e8c9d9da9980`):
all nineteen closed assertions checked, rc=0. Appending only the
dropped-monomial negative yielded `SeedBadDrop.ken` (SHA256
`e46a09b3d1e374d47d274c0ea29e356ff445598392e1ff336b8eac0fb6a12727`),
rc=1 `TypeMismatch`; appending only the false canonical converse
yielded `SeedBadCanonical.ken` (SHA256
`5f55e57cacc47b79c08f3e174c6d80773e74ca4a55d80e6656068e760f1ec51a`),
rc=1 `TypeMismatch`. Replacing either negative's `Refl` by `Proved`
also rejected with `KernelRejected TypeMismatch`; the refusal is not
just a choice of proof terminal. A separate private scratch proof
of the noncanonical pair's evaluation equality for **every**
environment checked rc=0 (SHA256
`10c1c3152fe71e5709a04a31b58b1a52bf3e5d5dede1e89f8e3bdfbed70cb099`).
All source-shaped checks used the freshly built local `ken check`
from base `bc092bdd5`; they do not publish the module, execute a
shipped seed, measure trusted-base delta, or establish CI results.
Every case has promise class **durable invariant**. `Eq` over
inductive structures is not a semantic-equality decision: list
order and repeated variables are observable here.

## Shared fixtures and positive assertions

`env1` maps variable zero to two and every successor variable to
three. `e1 = (x₀ + 1)·(x₁ + 2)` evaluates to fifteen. Its four
ordered monomials are `[x₀x₁, 2x₀, x₁, 2]`; the explicit list and
reified expression are constructed from public constructors rather
than from the operation under test. The `expected_expr`'s final
`+ 0` is part of `reify`'s checked representation, not a claim of
canonical simplification.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, Monomial, Mono,
   eval, eval_monomial, eval_monomials, monomials, reify, normalize,
   normalize_sound, normalize_idempotent, monomials_sound,
   reify_sound, monomials_reify)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const six : Nat = Suc (Suc four)
const fifteen : Nat =
  Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc
    (Suc (Suc (Suc (Suc (Suc Zero))))))))))))))

fn env1 (v : Nat) : Nat =
  match v { Zero ↦ two; Suc w ↦ three }

const e1 : Expr =
  EMul (EAdd (EVar Zero) (EConst one))
       (EAdd (EVar one) (EConst two))

const m01 : Monomial = Mono Zero (Cons Nat Zero (Cons Nat one (Nil Nat)))
const m02 : Monomial = Mono one (Cons Nat Zero (Nil Nat))
const m1 : Monomial = Mono Zero (Cons Nat one (Nil Nat))
const m2 : Monomial = Mono one (Nil Nat)
const expected_ms : List Monomial =
  Cons Monomial m01 (Cons Monomial m02
    (Cons Monomial m1 (Cons Monomial m2 (Nil Monomial))))
const expected_expr : Expr =
  EAdd (EMul (EVar Zero) (EMul (EVar one) (EConst one)))
    (EAdd (EMul (EVar Zero) (EConst two))
      (EAdd (EMul (EVar one) (EConst one))
        (EAdd (EConst two) (EConst Zero))))

theorem eval_oracle : Equal Nat (eval env1 e1) fifteen = Proved

theorem eval_monomial_oracle : Equal Nat (eval_monomial env1 m01) six = Proved

theorem eval_monomial_coefficient_oracle :
  Equal Nat (eval_monomial env1 m02) four = Proved

theorem eval_monomials_empty_oracle :
  Equal Nat (eval_monomials env1 (Nil Monomial)) Zero = Proved

theorem eval_monomials_oracle :
  Equal Nat (eval_monomials env1 expected_ms) fifteen = Proved

theorem monomials_oracle :
  Eq (List Monomial) (monomials e1) expected_ms = Refl

theorem monomials_zero_oracle :
  Eq (List Monomial) (monomials (EConst Zero)) (Nil Monomial) = Proved

theorem monomials_var_oracle :
  Eq (List Monomial) (monomials (EVar one))
    (Cons Monomial m1 (Nil Monomial)) = Refl

theorem reify_oracle : Eq Expr (reify expected_ms) expected_expr = Refl

theorem reify_empty_oracle : Eq Expr (reify (Nil Monomial)) (EConst Zero) = Proved

theorem normalize_oracle : Eq Expr (normalize e1) expected_expr = Refl

theorem normalize_eval_oracle :
  Equal Nat (eval env1 (normalize e1)) fifteen = Proved

theorem idempotent_oracle :
  Eq Expr (normalize (normalize e1)) (normalize e1) = Refl

theorem normalize_sound_instance :
  Equal Nat (eval env1 (normalize e1)) (eval env1 e1) =
  normalize_sound env1 e1

theorem normalize_idempotent_instance :
  Eq Expr (normalize (normalize e1)) (normalize e1) =
  normalize_idempotent e1

theorem monomials_sound_instance :
  Equal Nat (eval_monomials env1 (monomials e1)) (eval env1 e1) =
  monomials_sound env1 e1

theorem reify_sound_instance :
  Equal Nat (eval env1 (reify expected_ms))
    (eval_monomials env1 expected_ms) =
  reify_sound env1 expected_ms

theorem monomials_reify_instance :
  Eq (List Monomial) (monomials (reify expected_ms)) expected_ms =
  monomials_reify expected_ms

theorem semantic_pair :
  Equal Nat
    (eval env1 (EAdd (EVar Zero) (EVar Zero)))
    (eval env1 (EMul (EConst two) (EVar Zero))) = Proved
```

The five `*_instance` proofs apply the separately checked **generic**
laws to closed inputs; they are not independent checks of their own
universal premises. The direct value, list, and expression oracles
supply independent expectations on the same operations. `Proved`
closes reducible natural goals and the empty-list/empty-expression
equalities; `Refl` closes the checked structural `Eq` goals on
nonempty inductive carriers. Neither a finite instance nor its proof term establishes
a new generic law by itself.

## Expression evaluation uses the total environment

### stdlib/symbolic-algebra/eval-distributed-expression

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.1.
- promise class: durable invariant.
- given: `e1 = (x₀+1)·(x₁+2)` and `env1(0)=2`, `env1(1)=3`.
- expect: `eval env1 e1 = fifteen` and the closed `Equal Nat`
  proposition checks.
- measured: both branches of each `EAdd`, the `EMul` result, and two
  distinct variable indices on one input.
- claimed: expression evaluation uses each constructor's specified
  natural operation and a total environment lookup.
- the gap: this concrete expression does not prove correctness for
  all expressions or environments.
- why: dropping an addend or using the same index for both variables
  changes fifteen.

## A monomial multiplies its positive coefficient and ordered factors

### stdlib/symbolic-algebra/eval-monomial-nontrivial

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.1.
- promise class: durable invariant.
- given: `m01 = Mono 0 [0,1]` and `m02 = Mono 1 [0]` under `env1`.
- expect: `eval_monomial env1 m01 = six` (`1·2·3`) and
  `eval_monomial env1 m02 = four` (`2·2`).
- measured: two variable occurrences and both unit and non-unit
  positive coefficients.
- claimed: evaluation multiplies `Suc k` by each environment value.
- the gap: multiplication is commutative in `Nat`, so the first
  value does not distinguish permutations of its variables.
- why: omitting a variable or ignoring the predecessor coefficient
  changes one of these independently fixed numbers.

## Monomial-list evaluation sums every summand

### stdlib/symbolic-algebra/eval-monomials-four-summands

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.1.
- promise class: durable invariant.
- given: independently constructed `expected_ms` representing
  `[x₀x₁, 2x₀, x₁, 2]` under `env1`.
- expect: `eval_monomials env1 expected_ms = fifteen`:
  `6 + 4 + 3 + 2 = 15`; on `Nil Monomial`, the result is `Zero`.
- measured: four nonzero summands, including middle terms and the
  constant tail, and the exact empty-list base.
- claimed: the list evaluator starts at zero and adds every monomial.
- the gap: fixed lists do not prove the arbitrary-list fold.
- why: dropping any one of the four positive contributions changes
  the expected natural value.

## Distribution emits four ordered monomials

### stdlib/symbolic-algebra/monomials-ordered-cross-product

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.2.
- promise class: durable invariant.
- given: `e1` with two sums of two terms, plus the independent
  constructor-built `expected_ms`.
- expect: `monomials e1` equals exactly `[Mono 0 [0,1],
  Mono 1 [0], Mono 0 [1], Mono 1 []]` by `Eq (List Monomial)`.
- measured: four products, left-major/right-minor order, the
  predecessor coefficient for two, and two different variable lists.
- claimed: distribution retains every cross-term without sorting or
  merging.
- the gap: equal lists in one fixture do not prove generic
  distribution soundness or completeness.
- why: deleting `Mono 0 [1]`, changing the coefficient, or swapping
  ordered monomials refuses this structural equality.

## Zero constants vanish and variables use a unit coefficient

### stdlib/symbolic-algebra/monomials-zero-and-var

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.2.
- promise class: durable invariant.
- given: `EConst Zero` and `EVar one`.
- expect: `monomials (EConst Zero)` is `Nil Monomial`, and
  `monomials (EVar one)` is `[Mono Zero [one]]`.
- measured: two distinct base branches, including the absence of
  a zero monomial and the successor variable's correct index.
- claimed: constants and variables use the specified base list.
- the gap: a positive constant's singleton is exercised inside
  `e1`, not as a separate isolated input.
- why: treating zero as an explicit coefficient or ignoring the
  variable index changes at least one expected list.

## Reification preserves list order and the trailing zero

### stdlib/symbolic-algebra/reify-explicit-expression

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.2.
- promise class: durable invariant.
- given: `expected_ms` and independent `expected_expr` with all
  four corresponding summands and terminal `EConst Zero`.
- expect: `reify expected_ms` is structurally `expected_expr`, and
  `reify (Nil Monomial)` is `EConst Zero`.
- measured: expression constructor shape, left-to-right monomial
  order, and terminal zero in both the final tail and empty base.
- claimed: `reify` constructs the stated unsimplified expression.
- the gap: structural equality on one list is not generic
  `reify_sound`; the law is checked separately.
- why: swapping a term or omitting the terminal zero changes `Eq Expr`
  even if evaluation under `env1` stays the same.

## Normalization distributes without claiming canonicity

### stdlib/symbolic-algebra/normalize-explicit-expression

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.2.
- promise class: durable invariant.
- given: `e1` and independently built `expected_expr`.
- expect: `normalize e1` is structurally `expected_expr`, and
  `eval env1 (normalize e1) = fifteen`.
- measured: the public `normalize` call and both its expression
  shape and numeric denotation on a nontrivial product of sums.
- claimed: normalization produces the specified distributed expression
  and preserves its value on this fixture.
- the gap: a finite input is not generic soundness, and its output
  need not be unique among semantically equivalent expressions.
- why: a drop, rearrangement, or fixed-identity normalizer changes the
  independently constructed shape even if it passes an idempotence test.

## Normalization soundness is instantiated separately

### stdlib/symbolic-algebra/normalize-sound-instance

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `e1` and `env1` with the direct `eval` and `normalize` oracles.
- expect: `normalize_sound env1 e1` inhabits the stated equality of
  `eval env1 (normalize e1)` and `eval env1 e1`; each is fifteen.
- measured: the exported theorem applies to a well-typed, nontrivial
  closed input while independent numeric oracles pin both endpoints.
- claimed: the catalog must supply the separately D0-checked generic
  soundness theorem for every environment and expression.
- the gap: this use-site checks one instance, not the theorem's generic
  proof body in a future catalog module.
- why: a normalizer dropping the `x₁` monomial does not retain the
  stated evaluation on this input.

## Idempotence keeps the reified expression fixed

### stdlib/symbolic-algebra/normalize-idempotent-instance

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `e1` and its distributed normalized expression.
- expect: `normalize_idempotent e1` inhabits the structural `Eq Expr`
  between two normalizations and one normalization; the independent
  closed `idempotent_oracle` checks with `Refl`.
- measured: application of the generic D0 theorem and a separate
  constructor-level comparison on one expression.
- claimed: repeated normalization changes no expression structure.
- the gap: one closed application cannot substitute for a generic
  proof, and idempotence does not decide semantic equality.
- why: a normalizer that keeps rewriting a normalized expression
  fails this structural proposition.

## Monomial soundness equates distributed and direct evaluation

### stdlib/symbolic-algebra/monomials-sound-instance

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `e1`, `env1`, and an explicit four-term expected list.
- expect: `monomials_sound env1 e1` inhabits equality of the
  monomial-list and direct-expression evaluations, both fifteen.
- measured: public theorem application and independently pinned
  values on the two sides.
- claimed: every expression's distributed monomial sum denotes its
  direct evaluation under every environment.
- the gap: this instance does not discharge the theorem's universal
  premises; D0 checked the generic proof separately.
- why: a dropped monomial changes the four-term sum independently
  of the direct expression's value.

## Reification soundness equates expression and list evaluation

### stdlib/symbolic-algebra/reify-sound-instance

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `expected_ms` and `env1`, with independent `reify` and
  monomial-list value oracles.
- expect: `reify_sound env1 expected_ms` inhabits equality of
  `eval env1 (reify expected_ms)` and
  `eval_monomials env1 expected_ms`.
- measured: public theorem application on a four-term list with a
  numeric total fixed as fifteen.
- claimed: reification preserves evaluation for arbitrary lists.
- the gap: a closed application is not the generic proof body.
- why: an implementation omitting any term changes the denotation
  while the independently pinned list still sums to fifteen.

## Reification round-trips the exact monomial list

### stdlib/symbolic-algebra/monomials-reify-instance

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `expected_ms` with four differently shaped entries.
- expect: `monomials_reify expected_ms` inhabits `Eq (List Monomial)`
  for `monomials (reify expected_ms)` and the **same ordered list**.
- measured: an exact structural list theorem, not a numerical value
  or unordered multiset comparison.
- claimed: any list round-trips through `reify` at its original order.
- the gap: the closed use-site does not prove generic list induction;
  Architect D0 supplies a checked proof, not a catalog export.
- why: sorting, merging, or discarding a list element changes the
  structural result even if evaluation is unaffected.

## A dropped monomial is rejected

### stdlib/symbolic-algebra/reject-dropped-x1-term

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §§1.2–1.3.
- promise class: durable invariant.
- given: the same `e1` and four-entry `expected_ms` that the positive
  `monomials_oracle` accepts. The following **separate** checker input
  drops only its third element, `m1 = Mono Zero [one]`:

```ken
const missing_m1 : List Monomial =
  Cons Monomial m01 (Cons Monomial m02
    (Cons Monomial m2 (Nil Monomial)))
theorem drop_monomial_rejected :
  Eq (List Monomial) (monomials e1) missing_m1 = Refl
```

- expect: the positive source accepts all four monomials; the same
  source extended by this false theorem is rejected, not silently
  certified as a normal form or a proof of the original polynomial.
- measured: one monomial occurrence on an otherwise identical input.
- claimed: every distributed cross-term is preserved.
- the gap: this fixture catches the missing `x₁` term; other drops
  need their own witness, and a rejected proof in scratch is not a
  measured shipped-suite refusal.
- why: the deleted term evaluates to three under `env1`, making the
  remaining sum twelve rather than fifteen; a dropped-monomial
  normalizer violates `normalize_sound` as well as this shape oracle.

## Semantic equality does not decide list equality

### stdlib/symbolic-algebra/reject-canonical-converse

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §1.3.
- promise class: durable invariant.
- given: `x₀+x₀` and `2·x₀` in the same environment. The positive
  `semantic_pair` checks their evaluations agree at `env1`.
- expect: **do not accept** the following structural list equality
  even though the two expressions have equal denotation:

```ken
theorem canonical_converse_rejected :
  Eq (List Monomial)
    (monomials (EAdd (EVar Zero) (EVar Zero)))
    (monomials (EMul (EConst two) (EVar Zero))) = Refl
```

- measured: refusal of a specific noncanonical equality between
  `[x₀, x₀]` and `[2x₀]`, not a decision theorem.
- claimed: §1 specifies distribution, not merging of like terms.
- the gap: the positive check only establishes equality at `env1`;
  the **generic** semantic equivalence of these two expressions
  follows from natural distributivity, not from that fixture.
- why: equating their structural lists would silently assert the
  deferred canonicalization boundary as though it had landed.

## Coverage boundary

The positive oracles exercise all six public operations, both
carriers, all five generic laws at closed use sites, the zero and
successor expression cases, four distributed terms, reification
order, and a noncanonical pair. The two separate negative inputs
exercise a dropped term and the false canonical converse. Their
results do not publish the module, discharge the generic laws from
closed data, execute a shipped suite, measure trusted-base delta, or
establish a semantic-equality decision procedure.
