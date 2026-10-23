# Polynomial expressions over natural coefficients

> **Status: DRAFT v0 (SPEC-SYMBOLIC-ALGEBRA-CONTRACT §1;
> SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY §2;
> SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION §3;
> SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION §4;
> SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE §5).** The §1–3 catalog operations,
> checked laws, and seeds are present on main. Section 4's catalog
> operations and checked laws are delivered alongside its authored seed.
> Section 5 is specified from a checked Architect D0; its catalog exports
> and conformance seed are separate deliveries. D0 alone establishes
> neither shipped seed-harness execution nor CI.

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
field-dependent operation are deferred. Section 1's distributed list
is not a canonical form or an equality decision: §2 supplies a
separate Horner representation and decision, now landed, while
sorted-list canonicalization remains deferred. Dimension-indexed
environments remain outside §§1–5. Formal differentiation is specified
in §5; it does not make this ordered-list normalizer canonical.
Substitution and its composition are specified separately in §§3–4;
neither is silently supplied by the `Nat` distributive normalizer in §1.

## 2. Canonical polynomial form and semantic equality

> **Section 2 status: DRAFT v0 (SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY).**
> Architect D0 checked five additional generic laws. The §2 catalog
> code and authored seed have since landed; this historical D0 alone
> does not establish shipped seed-harness execution or CI.

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
Boolean answer. The §2 seed is present on main; these observations
do not establish its shipped harness execution or CI.

### 2.3 Evidence, trust, and remaining boundary

On base `bc711b1e0`, Architect D0 reported `ken check` rc 0 for the
in-module Horner development and all five generic laws. Architect
also measured five new checked transparent theorem bodies, zero
`Axiom`/postulate/admit in that development, and a scratch
`trusted_base()` comparison of **114 before and 114 after** loading
the module on its five providers. Closed positive examples checked;
a false equality claim on `x₀` and `x₁` was kernel-rejected. Those
are **Architect-reported scratch measurements**. The §2 catalog
extension and seed are now present on main; those D0 measurements
do not establish shipped seed-harness execution, CI, or a measured
landed trust delta.

The completeness proof in D0 distinguishes canonical heads using
positivity of `Nat` coefficients and cancellation, then recurses on
the Horner children. It does not establish a sorted-monomial-list
route, which was not attempted. No new kernel form, surface form,
primitive, or funext assumption is required. §1's ordered-list
`normalize` remains only a sound, idempotent distributor; this §2
carrier supplies a separate canonical representation and equality
decision. Sorted-list canonicalization, subtraction and signed
coefficients, division, factoring, field-dependent operations,
dimension-indexed environments remain outside §§1–5. Section 5
specifies formal differentiation, but `canonicalize_complete` alone
does not prove that differentiation respects semantic equality.
Sections 3–4 specify substitution and its composition without
changing §2's canonical carrier or decision.

## 3. Substitution and evaluation under a composed environment

An expression substitution is a total function `s : Nat → Expr`.
`substitute s e` replaces each variable of the **original** `Expr`
by its image under `s`; it preserves the constant, addition, and
multiplication constructors and their operand order. An inserted
`s v` is not recursively substituted a second time. This is a
single simultaneous substitution, not composition of two
substitutions. Both operations below are public:

```ken
pub fn compose_env (env : Nat → Nat) (s : Nat → Expr) (v : Nat) : Nat =
  eval env (s v)

pub fn substitute (s : Nat → Expr) (e : Expr) : Expr =
  match e {
    EConst c ↦ EConst c;
    EVar v ↦ s v;
    EAdd a b ↦ EAdd (substitute s a) (substitute s b);
    EMul a b ↦ EMul (substitute s a) (substitute s b)
  }
```

`compose_env env s` is a total **natural-valued environment**:
it evaluates each replacement expression under `env` before the
original expression looks up that variable. It is not an
`Expr`-valued composition of substitutions. Ken's theorem types
cannot use an inline lambda as a function argument here, so the
named public builder is required by the evaluation law's type.

The module must provide checked proof bodies for exactly two new
public laws. Evaluation under the substitution is evaluation of
the original expression under the composed environment, for every
environment, substitution and expression:

```ken
pub theorem eval_substitute
  (env : Nat → Nat) (s : Nat → Expr) (e : Expr) :
  Equal Nat (eval env (substitute s e)) (eval (compose_env env s) e)
```

A `True` answer from §2's canonical `poly_eq` remains `True` after
the same substitution is applied to both expressions. This law
uses `IsTrue` in the **forward direction only**:

```ken
pub theorem poly_eq_substitute
  (s : Nat → Expr) (e1 : Expr) (e2 : Expr)
  (h : IsTrue (poly_eq e1 e2)) :
  IsTrue (poly_eq (substitute s e1) (substitute s e2))
```

There is no converse. Before substitution, `poly_eq (EVar Zero)
(EVar (Suc Zero))` is `False`; if `s v = EConst Zero` for every
variable, the substituted pair is equal and `poly_eq` is `True`.
Thus substitution may identify expressions that were distinct under
some environments. The forward proof uses `poly_eq_sound` under
the named composed environment, the evaluation law in both
directions, and `poly_eq_complete`. It requires no function
extensionality or assumption about a particular environment.
The existing `add_cong` and new `mul_cong` are private proof steps,
not further public laws or trusted premises.

For a closed illustration, let `env v = Suc v`, let `s Zero` be
`EAdd (EVar Zero) (EConst (Suc Zero))`, and let `s (Suc v)` be
`EMul (EVar (Suc Zero)) (EVar Zero)`. Evaluating the substituted
`EAdd (EVar Zero) (EVar (Suc Zero))` gives `4`; evaluating that
original expression under the uncomposed `env` gives `3`. The
latter is **not** a valid replacement for `compose_env env s`.
These finite values illustrate the law and its refusal boundary;
they are not a proof of either universally quantified theorem.

The Architect's D0 on base `de3dc65b8` checked both generic law
bodies, generic clients, and the closed discrimination above, with
no `Axiom` and identical `trusted_base()` sets of 114 entries before
and after loading the edited package. The §3 catalog extension and
seed have since landed; that earlier scratch measurement does not
establish shipped seed-harness execution, CI, or a measured landed
trust delta. No subtraction, signed coefficient, division, new
primitive, kernel rule, surface feature, or expansion of the
catalog's ambient-name census is required. Section 4 specifies
composition of substitutions without changing §3's one-pass rule.

## 4. Composition and identity of expression substitutions

Substitutions remain total maps `Nat → Expr` with the single-pass
meaning fixed in §3. To apply `t` first and then `s`, compose their
images by substituting `s` into each `t v`. The builder is public:

```ken
pub fn compose_subst (s : Nat → Expr) (t : Nat → Expr) (v : Nat) : Expr =
  substitute s (t v)
```

`compose_subst` is `Expr`-valued, unlike §3's `Nat`-valued
`compose_env`. Naming it lets the following law state its function
argument without an inline lambda. Applying `s` to each inserted
`t v` is intentional: `substitute t e` itself is still one pass,
and the **second** substitution then visits the result. The module
must export checked proof bodies, not postulates or open goals, for
exactly two new public laws:

```ken
pub theorem substitute_compose
  (s : Nat → Expr) (t : Nat → Expr) (e : Expr) :
  Equal Expr (substitute s (substitute t e))
    (substitute (compose_subst s t) e)

pub theorem substitute_var (e : Expr) :
  Equal Expr (substitute EVar e) e
```

The first equality is **structural `Equal Expr`** for all `s`, `t`,
and `e`, not merely equal evaluation under a chosen environment.
The second uses the exported variable constructor `EVar` as the
identity map and needs no named identity builder. A client-local
named `identity_sub` can pass closed identity controls, but for an
open `e` it does not generically convert to `EVar`; the public
`substitute_var` law applies to the constructor, not every
extensionally equivalent function. Pointwise, `compose_subst s EVar v`
reduces to `s v`, and `compose_subst EVar t v` equals `t v` by
`substitute_var (t v)`. These are consequences, not two more exports
or an equality between functions requiring function extensionality.

The composition proof is by induction on `e`. Constants and
variables reduce directly; sums and products use the respective
induction hypotheses through private `expr_add_cong` and
`expr_mul_cong`. There is no `DecEq Expr`, normalization, new
primitive, kernel form, surface form, or new ambient import. Applying
`eval_substitute` and congruence to the composition law also gives
pointwise evaluation under `compose_env env (compose_subst s t)`;
applying `eval_substitute` twice gives evaluation under
`compose_env (compose_env env s) t`. Neither evaluation consequence
is an additional public law.

A closed order discriminator can substitute a variable into an
expression and then replace that inserted variable: with
`e = EAdd (EVar Zero) (EConst one)`, `t Zero = EVar one`, and
`s one = EConst two`, choose `t (Suc v) = EVar (Suc v)` and
`s Zero = EVar Zero` while `s (Suc v) = EConst two`. Both
`substitute s (substitute t e)` and
`substitute (compose_subst s t) e` are
`EAdd (EConst two) (EConst one)`. Reversing the composition yields
`EAdd (EVar one) (EConst one)`, which is structurally different.
This illustrates why the law fixes an order; the closed fixture is
not a proof of the generic theorem.

On landed base `fcd05e7eb`, Architect D0 checked the generic law
bodies, closed clients and reversed-order refusal in scratch: rc 0,
no `Axiom`, and identical `trusted_base()` identity sets of 114
entries before and after loading its edited package. The §4 catalog
extension and authored seed are now present; the D0 measurements alone
do not establish shipped seed-harness execution, CI, publication, or a
measured landed trust delta. Section 5 specifies formal derivatives;
it does not add a differentiation/substitution commutation law.
Sorted-list canonicalization, dimension-indexed environments,
subtraction and signed/field operations remain deferred.

## 5. Formal derivatives preserve polynomial functions

A formal partial derivative of an `Expr` with respect to a `Nat`
variable is another `Expr`. It is a structural transformation,
not a finite-difference operator or a new primitive. The existing
private `nat_same v w` compares variable indices using the checked
natural order. The private Boolean helper `var_deriv` returns
`EConst (Suc Zero)` for a matching variable and `EConst Zero`
otherwise. The operation has these defining equations:

```ken
fn var_deriv (b : Bool) : Expr =
  match b {
    True ↦ EConst (Suc Zero);
    False ↦ EConst Zero
  }

pub fn deriv (v : Nat) (e : Expr) : Expr =
  match e {
    EConst c ↦ EConst Zero;
    EVar w ↦ var_deriv (nat_same v w);
    EAdd a b ↦ EAdd (deriv v a) (deriv v b);
    EMul a b ↦ EAdd (EMul (deriv v a) b) (EMul a (deriv v b))
  }
```

The product rule is the `EAdd` of two `EMul` terms in that order:
first differentiate the left operand, then the right. It is not
an `Expr` quotient law or a claim that arbitrary expressions with
the same evaluation have identical derivative syntax. The four
structural equations are definitional consequences of `deriv`,
not separate exported theorem names.

### 5.1 Checked semantic laws

The package must export checked proof bodies, not postulates or
open goals, for these two laws. The headline premise quantifies
over every total `Nat → Nat` environment. The result is pointwise
`Equal Nat` under every such environment, not structural equality
of derivative expressions or function extensionality.

```ken
pub theorem deriv_respects_eval
  (v : Nat) (e1 : Expr) (e2 : Expr)
  (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2))
  (env : Nat → Nat) :
  Equal Nat (eval env (deriv v e1)) (eval env (deriv v e2))

pub theorem poly_eq_deriv
  (v : Nat) (e1 : Expr) (e2 : Expr)
  (h : IsTrue (poly_eq e1 e2)) :
  IsTrue (poly_eq (deriv v e1) (deriv v e2))
```

`poly_eq_deriv` preserves a `True` answer in the forward direction
only. Its proof applies §2's `poly_eq_sound`, the headline law, and
`poly_eq_complete`. It does not preserve `False`: distinct constants
both differentiate to `EConst Zero`.

### 5.2 Derivation and extraction boundary

The checked D0 derives the headline without assuming that §2's
`canonicalize` commutes with differentiation. It therefore does
**not** use `canonicalize_complete` to obtain a derivative theorem.
A private `bump_env v t env` raises only variable `v` by `t`.
Structural induction on `e` proves the private Taylor identity

```ken
theorem deriv_taylor
  (v : Nat) (t : Nat) (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval (bump_env v t env) e)
    (add (eval env e)
      (mul t (add (eval env (deriv v e))
        (mul t (taylor_rem v t env e)))))
```

where `taylor_rem` is a private natural-valued structural remainder.
The sum and product branches use checked arithmetic identities
via §2's `poly_eq_sound` on arithmetic expression pairs interpreted
under a private seven-slot `cons_env` environment. The variable branch
splits on the Boolean comparison that both `var_deriv` and `bump_env` use;
it does not rely on a match of a delta-unfolded neutral expression.

For any `e1` and `e2` satisfying the headline premise, apply that
premise at `env` and at the bumped environment. Cancel the equal
base values with private `add_cancel_left`, then cancel the positive
bump factor with private `mul_cancel_positive`. Choose
`t = Suc (add d1 d2)`, where `d1` and `d2` are the two derivative
values at `env`; both are strictly below `t`. Private
`remainder_unique` then extracts `Equal Nat d1 d2`, despite the
unknown higher-order remainders. The route reuses §2's private
cancellation and remainder-uniqueness helpers and the checked
natural-order bounds; none is a new public law, decision procedure,
primitive or trusted assumption. No subtraction or signed coefficient is needed.

### 5.3 Evidence and limits

On landed base `80682b3f7`, Architect D0 checked the base package
and the derivative extension separately with `ken check` rc 0.
The D0 trust test passed with identical five-provider
`trusted_base()` identity sets, 114 before and 114 after loading.
There was no new `Axiom`, postulate, admit or import. Removing the
right-hand product-rule term made the D0 checker reject the proof
with K0004; restoring it checked again. The base-only trust harness
reported the same 114 → 114 measurement but failed its subsequent
new-export lookup, as those exports were not on the base; it was not
a passing base-harness run.

These are scratch development measurements, not catalog delivery,
shipped conformance-seed execution, CI, publication, landing or a
landed trust measurement. The §5 catalog extension and separate
seed remain to be delivered. No `DecEq Expr`, function equality,
derivative/substitution commutation, differentiation of signed or
field coefficients, or analytic limit is claimed.
