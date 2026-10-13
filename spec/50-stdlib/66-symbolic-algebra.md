# Polynomial expressions over natural coefficients

> **Status: DRAFT v0 (SPEC-SYMBOLIC-ALGEBRA-CONTRACT).** Section 1
> specifies the `Nat` polynomial expressions, ordered distributed
> monomials, and five generic laws checked in Architect D0. This is
> not a claim that `Algorithm.SymbolicAlgebra.NatPolynomial` has
> landed, that the authored seed ran as a delivered suite, or that
> the trusted-base delta was measured.

## 1. Polynomial expressions over natural coefficients

### 1.1 Carrier and interpretation

`Algorithm.SymbolicAlgebra.NatPolynomial` exports two ordinary
inductive carriers. `Expr` has natural constants, variables, binary
addition and multiplication. Variables are `Nat` indices, and an
environment is a **total** `Nat → Nat` function: evaluation cannot
fail on an absent variable. This section does not require a `Fin n`
variable or a `Vec Nat n` environment.

```ken
pub data Expr = EConst Nat | EVar Nat | EAdd Expr Expr | EMul Expr Expr
export EConst, EVar, EAdd, EMul

pub data Monomial = Mono Nat (List Nat)
export Mono

pub fn eval (env : Nat → Nat) (e : Expr) : Nat
pub fn eval_monomial (env : Nat → Nat) (m : Monomial) : Nat
pub fn eval_monomials (env : Nat → Nat) (ms : List Monomial) : Nat
```

`eval env (EConst c) = c`, `eval env (EVar v) = env v`, and the
`EAdd` and `EMul` branches apply the existing checked `Nat` `add`
and `mul` to the evaluated operands. `Mono k vs` has a **positive
coefficient** `Suc k` and an ordered, possibly empty list of variable
indices `vs`. `eval_monomial` multiplies that coefficient by the
product of `env v` over `vs`, in list order; the empty product is
`Suc Zero`. A positive coefficient does not guarantee a nonzero
value under an environment assigning zero to a variable.
`eval_monomials` sums these values in list order, starting at `Zero`.
Repetitions and unsorted variable indices are allowed: `vs` is not
a set or a map. The explicit constructor exports are required for
clients to build closed expressions and monomials: publishing a
`data` type alone leaves its constructors private (`33 §4.1`).
Architect's checked D0 used the constructors inside its defining
module; adding only those five exports and loading a client was
separately checked without changing a proof body.

### 1.2 Distribution, reification, and normalization

```ken
pub fn monomials (e : Expr) : List Monomial
pub fn reify (ms : List Monomial) : Expr
pub fn normalize (e : Expr) : Expr
```

`monomials` maps `EConst Zero` to the empty list, a positive constant
`EConst (Suc k)` to `[Mono k []]`, and `EVar v` to
`[Mono Zero [v]]`. Addition appends the two monomial lists in source
order. Multiplication distributes: for each left monomial in order,
produce its products with **every** right monomial in order, then
append those batches. A product concatenates the variable lists in
left-then-right order. For predecessor coefficients `a` and `b`,
the product coefficient is `add b (mul (Suc b) a)`, which denotes
`(Suc a)·(Suc b)` after adding one. The checked D0 uses that
spelling to make multiplication by a unit-variable monomial reduce
to the original predecessor and to prove `monomials_reify` exactly.
All of the product, scaling, and append helpers may remain private;
they are not additional public operations.

`reify []` is `EConst Zero`. For `Mono k vs` followed by a tail,
`reify` builds an `EAdd` of the monomial expression and the reified
tail. The monomial expression multiplies the ordered variables by
`EConst (Suc k)` and has only the constant at empty `vs`.
`normalize e = reify (monomials e)`. Thus `monomials e` is an
**ordered distributed list**, and `normalize e` is its reified
`Expr`. This form distributes products without sorting variables,
sorting summands, or merging equal terms. It is idempotent, but
**not canonical for semantic equality**.

### 1.3 Five checked-law obligations

The module must export **checked proof bodies**, not assumptions or
open obligations, for exactly the following five laws. These are
generic in the expression or monomial list and, where present, the
environment; a closed example does not discharge their quantifiers.
`Equal Nat` states evaluation equality; `Eq` over `Expr` or
`List Monomial` states structural equality of these inductive
carriers. In particular, `monomials_reify` preserves the exact list,
not merely its evaluation or a sorted quotient.

```ken
pub theorem normalize_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval env (normalize e)) (eval env e)

pub theorem normalize_idempotent (e : Expr) :
  Eq Expr (normalize (normalize e)) (normalize e)

pub theorem monomials_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval_monomials env (monomials e)) (eval env e)

pub theorem reify_sound (env : Nat → Nat) (ms : List Monomial) :
  Equal Nat (eval env (reify ms)) (eval_monomials env ms)

pub theorem monomials_reify (ms : List Monomial) :
  Eq (List Monomial) (monomials (reify ms)) ms
```

The monomial-evaluation laws use existing checked natural addition,
multiplication, distributivity, and `list_append`, with private
structural proofs for append, variable products, and multiplication.
`normalize_sound` composes `reify_sound` with `monomials_sound`;
`normalize_idempotent` follows from `monomials_reify`. These are
checked *proofs*, not numerical tests. They do not imply that
semantic equality is decided by comparing normal forms: `x₀+x₀`
and `2·x₀` evaluate equally under every environment by natural
arithmetic but have different ordered monomial lists. A separate
private scratch lemma checked the evaluation equality for an
arbitrary environment using the existing `mul_add_distrib_l` and
unit-multiplication law; it is not a sixth catalog law. Equivalence
of evaluations follows from identical lists; its converse is
**not** claimed. The conformance seed includes a positive semantic
fixture and the unequal-list refusal on the same pair.

### 1.4 Trust and deferrals

The D0 development at base `bc092bdd5` checked all five generic
proofs together, rc=0, without `Axiom`, postulate, or admit in the
authored source. It did **not** measure `trusted_base()` before and
after loading; no quantified zero-delta trust claim follows from a
source scan. The section requires no new kernel form, syntax, or
primitive, and the future catalog implementation must not replace
a proved law with an assumption. A scratch check and a source-shaped
seed check are separate from catalog publication, execution of a
shipped seed, Cargo tests, and CI.

Subtraction and signed coefficients, division, factoring, and any
field-dependent operation are deferred. Canonical sorted/merged
normal forms and a decision procedure for semantic equality are
also deferred, as are dimension-indexed environments, substitution,
composition, and derivatives. None is silently supplied by the
`Nat` distributive normalizer in §1.
