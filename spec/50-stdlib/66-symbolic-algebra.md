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

## 2. Canonical polynomial form and semantic equality

> **Section 2 status: DRAFT v0 (SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY).**
> Architect D0 checked five additional generic laws. This section
> specifies an extension of the existing module, not a claim that its
> §2 code or seed has landed.

### 2.1 Recursive canonical carrier

`Algorithm.SymbolicAlgebra.NatPolynomial` extends its §1 `Expr` and
`Monomial` interface with the following public carrier and operations:

```ken
pub data Poly = PConst Nat | PHorner Poly Poly
export PConst, PHorner

pub fn eval_poly (env : Nat → Nat) (p : Poly) : Nat
pub fn canonicalize (e : Expr) : Poly
pub fn poly_eq (e1 : Expr) (e2 : Expr) : Bool
```

`PConst c` denotes the constant `c`. `PHorner p q` denotes the value
of `p` with every variable index shifted up by one, plus the value of
`q` multiplied by variable `Zero`. Equivalently, the head `p` is
interpreted under `v ↦ env (Suc v)`, while `q` and variable `Zero`
use `env` itself. Thus the head cannot depend on variable `Zero`.
This interpretation is over the same **total** `Nat → Nat`
environments and natural-number arithmetic as §1.

A **canonical** `Poly` has canonical children and contains no node
`PHorner (PConst c) (PConst Zero)`: that node represents merely the
constant `c` and is collapsed to `PConst c`. This prohibition does
**not** eliminate `PHorner (PConst Zero) q`, which can represent
variable `Zero` multiplied by `q`, or a zero second child when the
first child is itself a `PHorner`, which shifts a nonconstant
polynomial to higher variable indices. The public constructors can
also build noncanonical `Poly` values; it is `canonicalize` that
produces the canonical shape.

Monomial order in this carrier is **structural**, not a sorted list
comparison. Variable `Zero` is outermost; the repeated second-child
Horner spine represents its powers in ascending order; coefficients
are recursively polynomials in higher-index variables. Like terms
occupy the same coefficient position and merge there by natural
addition. With `Nat` coefficients there is no cancellation requiring
signed arithmetic. `canonicalize` starts from §1's distributed
`monomials e`, collects their coefficients into this Horner shape,
and collapses the redundant constant-headed zero tails. It does
not change §1's `normalize : Expr → Expr`, whose ordered distributed
lists remain noncanonical for semantic equality.

`poly_eq` compares the **canonical** results for two expressions,
returning `True` for equal forms and `False` for unequal forms.
Coefficient equality is decidable using both directions of the
existing natural-number `leq_nat`, not a new kernel equality rule.
Structural comparison and canonicity helpers are private; no
`DecEq Expr` or public theorem about arbitrary noncanonical `Poly`
values is added.

### 2.2 Exactly five checked-law obligations

The module must export checked proof bodies, not postulates or open
goals, for exactly these five new laws. `Equal Poly` compares the
resulting canonical forms; `IsTrue` is the proposition that the
Boolean answer is `True`. Equal evaluation is quantified over
**every** total environment, not merely a finite test set.

```ken
pub theorem canonicalize_sound (env : Nat → Nat) (e : Expr)
  : Equal Nat (eval_poly env (canonicalize e)) (eval env e)

pub theorem canonicalize_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
  : Equal Poly (canonicalize e1) (canonicalize e2)

pub theorem poly_eq_sound (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2)) (env : Nat → Nat)
  : Equal Nat (eval env e1) (eval env e2)

pub theorem poly_eq_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
  : IsTrue (poly_eq e1 e2)

pub theorem poly_eq_false_refutes (e1 : Expr) (e2 : Expr)
    (hf : Equal Bool (poly_eq e1 e2) False)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
  : Bottom
```

`canonicalize_sound` preserves evaluation. The converse here is
about **two expressions**: if their evaluations agree at every
environment, `canonicalize_complete` identifies their canonical
`Poly` values. The decision's `True` answer therefore entails
agreement at every environment (`poly_eq_sound`), and universal
agreement forces `True` (`poly_eq_complete`). A `False` answer is
incompatible with such a universal proof
(`poly_eq_false_refutes`); this law does not supply a particular
environment witnessing inequality. The stronger intermediate fact
used in D0 about positive environments is **private** and is not a
sixth public law.

For example, `x₀+x₀` and `2·x₀` have distinct §1 ordered monomial
lists, but both yield `PHorner (PConst Zero) (PConst 2)` and
`poly_eq` returns `True`. In contrast, `x₀` and
`x₁` differ when variable `Zero` is assigned `1` and variable `1`
is assigned `0`, so `poly_eq` returns `False`; an `IsTrue` claim for
that pair must be refused. These distinct observations rule out
both syntactic-expression comparison as the decision and a constant
Boolean answer. They describe the intended conformance cases, not
a claim that the separately owned §2 seed has shipped or run.

### 2.3 Evidence, trust, and remaining boundary

On base `bc711b1e0`, Architect D0 reported `ken check` rc 0 for the
in-module Horner development and all five generic laws. Architect
also measured five new checked transparent theorem bodies, zero
`Axiom`/postulate/admit in that development, and a scratch
`trusted_base()` comparison of **114 before and 114 after** loading
the module on its five providers. Closed positive examples checked;
a false equality claim on `x₀` and `x₁` was kernel-rejected. Those
are **Architect-reported scratch measurements**, not execution of a
committed §2 catalog extension, a delivered §2 conformance seed,
CI, or an author-measured trust delta.

The completeness proof in D0 distinguishes canonical heads using
positivity of `Nat` coefficients and cancellation, then recurses on
the Horner children. It does not establish a sorted-monomial-list
route, which was not attempted. No new kernel form, surface form,
primitive, or funext assumption is required. §1's ordered-list
`normalize` remains only a sound, idempotent distributor; this §2
carrier supplies a separate canonical representation and equality
decision. Sorted-list canonicalization, subtraction and signed
coefficients, division, factoring, field-dependent operations,
dimension-indexed environments, substitution, composition, and
derivatives remain outside this contract.
