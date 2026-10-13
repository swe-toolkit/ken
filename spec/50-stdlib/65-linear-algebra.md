# Dimension-safe exact linear algebra

> **Status: DRAFT v1 (SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT;
> SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS).** Section 1 specifies the seven
> `Nat`-valued operations and five laws landed in the catalog. Section 2
> adds two generically checked product laws for a follow-on catalog
> delivery. An authored conformance seed is not itself evidence that it
> has run as a delivered suite.

## 1. Vectors and matrices over natural scalars

### 1.1 Carrier, dimensions, and imports

`Algorithm.LinearAlgebra.NatMatrix` operates on the **existing**
`Data.Vector.Vector` carrier. A length-`n` vector of scalars is
`Vec Nat n`; an `m`-row, `n`-column matrix is `Vec (Vec Nat n) m`,
with each inner vector a row in order. A matrix has no second indexed
carrier, unchecked shape field, truncating zip, or implicit resizing.
A vector of length zero and a matrix with either dimension zero are
valid. `Vec`, `VNil`, `VCons`, `map`, `zip_with`, and the checked
`vec_zero_vnil` are published by `Data.Vector.Vector`; the latter
supplies the empty-index transport needed by generic proofs. The
chapter neither duplicates the family nor changes its constructors
or law bodies.

`Nat` is the only scalar in this section. Its existing checked `add`
and `mul`, identities, commutativity, associativity, and distributivity
supply the arithmetic. A new semiring class and a `Rational` instance
are not implied: the landed Rational carrier identifies values under
`rational_equiv`, not structural `Equal Rational`, so substituting it
would change the very equality in the laws below. No floating-point
or signed calculation is silently represented by a natural.

### 1.2 Seven exact operations

All seven functions belong to `Algorithm.LinearAlgebra.NatMatrix`.
Their result indices are part of their checked types. `vector_add`
adds aligned components, and `vector_scale` multiplies every
component by the natural factor. Their empty inputs return an empty
vector. No zip discards a component.

```ken
pub fn vector_add
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n

pub fn vector_scale
  (n : Nat) (factor : Nat) (xs : Vec Nat n) : Vec Nat n

pub fn dot
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Nat
```

`dot Zero (VNil Nat) (VNil Nat)` is zero. On aligned successors
`VCons Nat k x xs` and `VCons Nat k y ys`, it is
`add (mul x y) (dot k xs ys)`. Thus a mismatch is refused by `Vec`'s
index before the product is evaluated, and every component
contributes exactly once. A natural result has no rounding or
subtraction. The arithmetic's checked laws, not a numeric primitive,
justify the generic equations in §1.3.

```ken
pub fn matrix_vector
  (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) (vector : Vec Nat n) : Vec Nat m

pub fn transpose
  (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) : Vec (Vec Nat m) n

pub fn matrix_product
  (m : Nat) (n : Nat) (p : Nat)
  (left : Vec (Vec Nat n) m) (right : Vec (Vec Nat p) n)
  : Vec (Vec Nat p) m

pub fn identity (n : Nat) : Vec (Vec Nat n) n
```

`matrix_vector` takes each row's dot product with the vector, in
row order. `transpose` takes the ordered columns as its rows: entry
at row `i`, column `j` becomes entry at row `j`, column `i`; it does
not reverse either order. `matrix_product` takes each row of the
`m × n` left input and dots it with every ordered column of the
`n × p` right input, producing an `m × p` matrix. The two inner
indices must be the **same `n`**; `m × n` by `p × q` with `n ≠ p`
does not type-check. The zero-dimensional result follows from those
same indexed operations, without an exceptional arithmetic value.

`identity n` has one on its diagonal and zero elsewhere, using
`Suc Zero` and `Zero` as the scalar values. `identity Zero` has zero
rows. At `Suc k`, its first row is one followed by `k` zeros, and
each remaining row prefixes zero to the corresponding row of
`identity k`. This construction returns exactly `Vec (Vec Nat n) n`;
there is no separately stored dimension witness. Private local
helpers for repeating zeros, prepending elements and computing row
products are implementation choices, not additional public
operations.

### 1.3 Five checked-law obligations

The landed catalog module provides a **checked proof body** for each
public theorem below, with no `Axiom`, postulate, assumed lemma, or
open obligation. The statements quantify over every length and
well-typed operand; a finite fixture is not a substitute for any
of them. `Equal Nat` and `Equal (Vec …)` are ordinary checked
propositions here, not floating comparisons or a new matrix
equality relation.

```ken
pub theorem dot_comm
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) :
  Equal Nat (dot n xs ys) (dot n ys xs)

pub theorem dot_distrib
  (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n) :
  Equal Nat
    (dot n (vector_add n xs ys) zs)
    (add (dot n xs zs) (dot n ys zs))

pub theorem transpose_invol
  (m : Nat) (n : Nat) (rows : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (transpose n m (transpose m n rows)) rows

pub theorem vector_identity
  (n : Nat) (v : Vec Nat n) :
  Equal (Vec Nat n) (matrix_vector n n (identity n) v) v

pub theorem matrix_identity_right
  (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m)
    (matrix_product m n n a (identity n)) a
```

The dot laws are over natural sums: commutativity rearranges each
paired factor, and left distributivity expands a componentwise sum
before adding the resulting dot products. The transpose theorem
requires the original **row and column order**, not merely the right
shape. The vector identity law applies to any `Vec Nat n` including
an empty vector. The right matrix identity law composes that
behavior over arbitrary rows and the columns of `identity n`; it
is not a left-identity claim in disguise. The `vec_zero_vnil`
transport used in the checked proof of the dot and transpose empty
cases is a **proved Vec law**, not a fresh axiom.

### 1.4 Trust boundary and evidence limits

The ordinary `Vec` eliminator, natural recursion, `Equal`, and the
existing Nat laws account for §1's specified work. Its seven
operations and five generic laws are now landed in
`Algorithm.LinearAlgebra.NatMatrix`; neither the first catalog node
nor the §1 signatures promise the two additional theorems in §2.
The checked catalog module introduces no `Axiom`. Section 2's
additional generic proofs are checked in a scratch development but
are not yet exported by that module. No statement in this chapter
adds a kernel form, syntax, primitive, or `Float` support. A scratch
check or authored seed is not execution of the delivered seed.

Signed scalars, a relation-parametric semiring abstraction and a
Rational-valued matrix tier, determinants, inverses, solving and
decompositions, and every `Float` or approximate/error-bound result
are outside this contract. No requirement is discharged by a comment,
an unchecked assumption, or a numerical example alone.

## 2. Left identity and associativity of matrix products

The following two **generic laws have checked proof bodies** in the
product-laws D0 development. A follow-on catalog node must publish
them as theorems over every well-typed operand; the landed module
still exports only the §1 laws. Neither law is gated on
`LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`. Both use the existing
`Nat`, `Vec`, and matrix operations, without a new carrier or scalar
tier.

```ken
pub theorem matrix_identity_left
  (m : Nat) (n : Nat) (a : Vec (Vec Nat m) n) :
  Equal (Vec (Vec Nat m) n)
    (matrix_product n n m (identity n) a) a

pub theorem matrix_product_assoc
  (m : Nat) (n : Nat) (p : Nat) (q : Nat)
  (a : Vec (Vec Nat n) m)
  (b : Vec (Vec Nat p) n)
  (c : Vec (Vec Nat q) p) :
  Equal (Vec (Vec Nat q) m)
    (matrix_product m p q (matrix_product m n p a b) c)
    (matrix_product m n q a (matrix_product n p q b c))
```

Left identity picks each row of `a` with the corresponding unit row
and preserves its order, including when either dimension is zero.
This is not a rephrasing of §1's right identity: the unit matrix is
on the other side and its size is the **row** count `n`, rather than
the column count `m`. Product associativity compares two ways to
compose `m × n`, `n × p`, and `p × q` matrices; both sides are `m × q`
and the shared indices are statically equal. Its proof distributes
column products across vector addition and scaling, using the
existing checked Nat distributivity and associativity laws. Neither
a finite equality of products nor the type formation of these
propositions is the generic proof.

The D0 check on landed base `e940e481c` closed each theorem and both
together by structural recursion, using existing Nat proofs and
`Refl`, `Proved`, `J`, `cong`, `trans`, and `sym`. The added proof text
has **no `Axiom`**; the trusted-base delta was **not measured**, so
that observation is not a quantified zero-delta assertion. The
checked D0 uses a private named helper `dot_with n row` where the
landed package writes the argument to `map` as
`λcolumn. dot n row column`. With the λ spelling, the proof checks
failed during elaboration at `left_head` and `columns_cons` with an
ill-typed ambient-method error; the named spelling checked both
laws without changing any public type or operation. The λ failure
belongs to `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`; fixing
that elaborator defect is **not a prerequisite** for these theorems.
The catalog follow-on may use the named private helper while
retaining the exact public interface above. D0 proves feasibility,
not catalog publication, shipped-seed execution, or CI.
