# Dimension-safe exact linear algebra

> **Status: DRAFT v0 (SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT).**
> Section 1 is normative for the seven `Nat`-valued operations and five
> proved laws below. Left matrix identity and matrix-product
> associativity are named but **gated**, not part of this section's
> deliverable. Specifying a catalog interface does not claim it has
> landed or that its conformance seed has run.

## 1. Vectors and matrices over natural scalars

### 1.1 Carrier, dimensions, and imports

`Algorithm.LinearAlgebra.NatMatrix` operates on the **existing**
`Data.Vector.Vector` carrier. A length-`n` vector of scalars is
`Vec Nat n`; an `m`-row, `n`-column matrix is `Vec (Vec Nat n) m`,
with each inner vector a row in order. A matrix has no second indexed
carrier, unchecked shape field, truncating zip, or implicit resizing.
A vector of length zero and a matrix with either dimension zero are
valid. `Vec`, `VNil`, `VCons`, `map`, `zip_with`, and the checked
`vec_zero_vnil` are existing declarations that the catalog node must
publish from `Data.Vector.Vector`; the latter supplies the empty-index
transport needed by generic proofs. The chapter neither duplicates
the family nor changes its constructors or law bodies.

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

The catalog node must provide a **checked proof body** for each
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

### 1.4 Gated laws, trust boundary, and evidence limits

`matrix_identity_left` (identity on the **left** of a matrix product)
and `matrix_product_assoc` (associativity across three composable
matrices) are **gated on
`LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`**. Neither is declared as
a proved theorem in §1; neither is owed by its first catalog node.
The left-identity proof's empty and successor branches each checked
separately, but their assembled nested dependent match is rejected
by the current elaborator. Associativity's generic proposition
type-forms, but its proof remains unmeasured. Closed left-identity
fixtures do not promote either gated universal property to proved.
A later section may add these laws only after its Language
prerequisite lands and the generic proofs check.

The ordinary `Vec` eliminator, natural recursion, `Equal`, and the
existing Nat laws account for the specified work. This section adds
no kernel form, syntax, primitive, `Float`, or `trusted_base()` entry.
A source-shaped D0 development checked the seven operations, the
generic transpose, vector identity, right matrix identity and dot
commutativity proofs, and the fixed product/identity/transpose
fixtures without `Axiom`; the Architect separately checked generic
dot distributivity. Scratch checks demonstrate feasibility of these
**five** promises but neither deliver catalog exports nor execute
the authored conformance seed. The rejected assembled left-identity
proof remains a named elaboration gate, not a trust workaround.

Signed scalars, a relation-parametric semiring abstraction and a
Rational-valued matrix tier, determinants, inverses, solving and
decompositions, and every `Float` or approximate/error-bound result
are outside §1. No requirement is discharged by a comment, an
unchecked assumption, or a numerical example alone.
