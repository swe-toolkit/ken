# Sum types, pattern matching, and refinements

> Status: **impl-ready (L2)**. Normative and high-priority for the feature.
> Sum types, real constructors and eliminators, `match` with exhaustiveness +
> reachability, `Result`/`Option`, indexed (GADT-like) families, and
> refinement types are first-class and fully checked from day one — each `data`
> declaration lowers to a genuine inductive type with real constructors and a
> real eliminator, never an opaque base.
>
> **No new kernel rule for the landed L2 surface.** Its direct and Π-bound forms
> lower to the **landed** kernel: a `data` decl elaborates to a kernel inductive
> family + its generated `elim_D` (`../10-kernel/14`, K1 and **K1.5**); `match`
> elaborates to `elim_D`
> (`39 §2.6`); a refinement `{x:A|φ}` elaborates to its **carrier `A` plus an
> emitted obligation** (`../20-verification/21 §2`, `22 §2.1`), never a kernel
> type former. The elaborator and the exhaustiveness/reachability checker are
> **untrusted** (`39 §1`): a bug yields a rejected valid program or a poor
> diagnostic, **never** an unsound acceptance — the kernel re-checks the emitted
> `elim_D` (`§4.4`). The nested-positive work named below is **partially
> landed**. The current kernel admits declaration-recorded strictly-positive
> parameter paths and checked-transparent Sigma aliases, including fresh
> `Bag Rose` and two-former `Bag (Wrap Deep)` paths. Type-classified surface
> recursive results execute through generated lifts and nested ι. Independently
> marked generator, topology, sort, and dependent-motive residuals remain
> staged; this status does not claim those cases.
>
> **Perishable — pin against the landed kernel, not this prose.** K1.5 has
> landed: the K1-era blanket rejection of Π-bound recursion
> (`check_no_pi_bound_recursive`) is **retired**, and `check_positivity` is the
> sole structural admission gate (`../10-kernel/14 §8.4`;
> `ken-kernel/src/inductive.rs`). W-style `(b:B) → D` recursive constructor
> arguments are **admitted**. Nested-positive admission is no longer blanket-
> gated: the current D6 corpus executes the fresh-carrier, composed-carrier,
> transparent-head, and controlled-negative paths. `KERNEL-NESTED-IND` retains
> only the cases independently marked as residual. The narrower live surface
> boundary is the Ω-classified dependent-motive selector, gated on
> `KERNEL-RECURSIVE-RESULT-SURFACE`; the Type-classified selector is landed.
> Verify against the on-`main` `14`/kernel before building.

## Canonical non-dependent pair package

The named surface family `Pair` is an ordinary standard-package definition over
the kernel's dependent pair. It is not the kernel term constructor called
`Pair` in core syntax, and it is not a member of the closed prelude floor. A
source unit uses the named family only after importing it from its one defining
public interface. Merely having an implementation-global definition with the
same spelling does not provide that interface under Strict (`30 §4`/`§5`, `33
§3.3`, `39 §2.0`).

The public behavioral contract has four checked transparent declarations:

```ken
Pair     : Type -> Type -> Type
mk_pair  : (A : Type) -> (B : Type) -> A -> B -> Pair A B
pair_fst : (A : Type) -> (B : Type) -> Pair A B -> A
pair_snd : (A : Type) -> (B : Type) -> Pair A B -> B
```

Their meanings are fixed by conversion. For `A B : Type`, `a : A`, `b : B`,
and `p : Pair A B`:

```text
Pair A B                                      ≡  (x : A) × B
mk_pair A B a b                               ≡  (a, b)
pair_fst A B (mk_pair A B a b)                ≡  a
pair_snd A B (mk_pair A B a b)                ≡  b
mk_pair A B (pair_fst A B p) (pair_snd A B p) ≡  p
```

The right sides are semantic notation for the existing kernel
`Sigma`/introduction/projection forms, not new surface syntax. The first four
lines follow by transparent unfolding and Σ-β; the last is the existing
judgmental Σ-η (`../10-kernel/13 §6`). Each displayed equation is
**definitional** and requires no theorem, postulate, or runtime rule.

When one of these judgments is restated as a surface equality, its proof
terminal is determined by the normalized proposition rather than by one
uniform “reflexivity” token. A concrete Bool fst or snd β instance reduces the
whole equality to `Top`, so it closes with `Proved`; `Refl` rejects because the
goal is no longer `Eq`-shaped. Reconstruction η at a neutral
`p : Pair Bool Bool` remains `Eq`-shaped and closes with `Refl`; `Proved`
rejects there. Both terminals rely only on conversion, but they are not
interchangeable.

Each public name has exactly one defining module and one canonical `GlobalId`.
A direct import and any re-export preserve those identities and allocate no
replacement declaration. A separately authored definition with the same name,
type, and transparent body has a different identity; definitional equality does
not turn it into the provider declaration or rewrite an existing identity-keyed
reference. All four definitions are ordinary kernel-rechecked Ken and add
nothing to `trusted_base()` or the flat `Σ` model beyond their checked package
declarations.

Until the canonical interface is realized, positive package vectors that name
these declarations are **RED-UNTIL the canonical Pair package**. Strict bare use
continues to reject. The eventual package does not add `Pair` to the floor and
does not retain an ambient, Legacy, compatibility-alias, or mixed-resolution
route to an implementation convenience. The concrete package path and the
transition mechanism are separately designed; this section fixes only the
observable one-interface, one-identity, transparent-Σ contract.

## 1. Sum types (real, not stubbed)

```
data Option a = None | Some a
data Result e a = Err e | Ok a
data Color = Red | Green | Blue
data Tree a = Leaf | Node (Tree a) a (Tree a)
data Expr = Lit Int | Add Expr Expr | Neg Expr
```

A `data` declaration elaborates to an **inductive family** (`../10-kernel/14
§1`): the kernel admits the **type former** `D`, its **constructors** `cₖ` (real
introduction forms), and the **generated dependent eliminator** `elim_D` (`14
§3`) — the *only* primitive way to consume a value of `D`. Constructors are real
intro forms and `elim_D` is a real eliminator **with computation**: `elim_D …
(Some x) ≡` the `Some` method applied to `x` (`14 §3` ι-reduction). Values can
be built **and** taken apart, and the eliminator reduces.

- **Constructor arguments** are positional or named-record style (`32 §1`); the
  surface `C A B` form and the record form `C { f : A, g : B }` both elaborate
  to a constructor telescope `(Δₖ) → D Δ_p t̄ₖ` (`14 §1`).
- **Recursive constructors** are admitted **subject to strict positivity** (`14
  §2`, `§8`): a recursive occurrence of `D` may appear only at positive
  polarisation—directly, as a function target, or through declared
  strictly-positive parameter positions—never to the left of an arrow or
  through an unknown/non-positive parameter. The elaborator emits the
  declaration; the **kernel** runs `check_positivity`. The elaborator does
  **not** re-implement positivity — it is a kernel admission gate (`§4.4`, the
  trust boundary).
- **`Result`, `Option`** are ordinary prelude `data` decls (`../50-stdlib/`):
  fallibility and absence are **honest sum types**, not sentinel values. There
  is no `null` and no error code — `None`/`Err` are constructors the
  exhaustiveness checker (`§4`) forces every consumer to handle. **`Either a b
  = Left a | Right b` is a distinct declared value coproduct** — a **catalog
  package** (`50-stdlib/README.md §"Package listing"`: core data are packages,
  Ken `data`/defs over the built-ins, not prelude primitives), NOT a prelude
  `data` decl: an ordinary non-dependent sum needs zero kernel/elaborator/
  effects support, so it is declared at the user level
  (`catalog/packages/Data/Sums/Combinators.ken.md`), matching the spec's own model.
  `Result` remains the distinct, error-biased sum wired into the effect system
  (`fs_resp : … = Result IOError Bytes`); `Either` is the neutral,
  non-error-biased sibling — the two coexist, neither subsumes the other
  (judgment call L5, 2026-07-10; an earlier erratum subsumed `Either` into
  `Result` while `Either` had no declaration or user — that condition no
  longer holds now that `Either` is landed). **A third, structurally
  isomorphic neutral sum is also prelude-declared and user-reachable:**
  `Coproduct a b = InL a | InR b` (`crates/ken-elaborator/src/effects/
  state.rs`'s `declare_coproduct`, hand-built rather than a surface `data`
  decl, `elab.globals.insert`-registered like any other prelude type — an
  ordinary surface reference such as `InL a b x` elaborates). It is the
  effect-signature composition coproduct (`ITree`'s `resp_coproduct`/
  `inject_l`/`inject_r`, effect-composition `D2`) — kept hand-built as a
  deliberate risk-reduction for effect-row plumbing, not deprecated or
  hidden. `Either` is the catalog-level neutral sum for ordinary user
  code; `Coproduct` is internal effect-signature plumbing most code never
  names directly. The two are not reconciled into one declaration here —
  that is a reflect-don't-extend opportunity the implementation's own
  comment leaves explicitly open for the Architect to take up, not a
  decision this WP makes.

**What the elaborator builds vs. what the kernel admits (the staged line).**
The elaborator lowers a `data` decl to a kernel `InductiveDecl` and relies on
the kernel's admission gates. Non-recursive and **direct-recursive**
(`A → List A → List A`) constructors are K1. **W-style** (Π-bound) recursive
arguments — `(b:B) → D`, the branching shape of `W` and L5's `ITree` — are
**K1.5** and now admitted (`14 §2.1`, landed). **Nested strictly-positive**
admission is partially landed: the current kernel accepts fresh
recorded-positive carrier paths such as `Bag Rose` and the composed
`Bag (Wrap Deep)` path, while unknown and non-positive paths still reject
fail-closed. Type-classified nested surface recursion also executes through its
generated lift and ι. Only the individually marked residual cases remain
implementation-gated; in this chapter the live surface-lowering residual is
the Ω-classified dependent-motive selector under
`KERNEL-RECURSIVE-RESULT-SURFACE`. **Mutual** families remain a separately
deferred extension (`14 §8.6`).

## 2. Indexed families and dependent constructors (GADT-like)

Constructors may target different **indices** (`../10-kernel/14 §1`), giving
length-indexed and well-typed-by-construction data. The surface form is the
ordinary inductive-family shape, with parameters before the colon and an index
telescope after it:

```ken
data D (Δ_p) : (Δ_i) -> Type where {
  C1 : (Δ_1) -> D Δ_p t̄_1;
  ...
}
```

The legacy simple form `data D a = C A | ...` remains sugar for the non-indexed
case whose constructor result is the default `D a`. The explicit `where` form is
required when a constructor writes its full dependent signature.

### 2.1 Data heads: parameters vs. indices

The binders before the colon are **parameters**. They are fixed across every
constructor target. The type after the colon is the family result: a telescope of
**indices** ending in `Type ℓ`. A non-indexed family writes `: Type`; an indexed
family writes, for example, `: Nat -> Type` or `(n : Nat) -> Type`. Universe
levels are inferred or checked by the existing `Type` rules (`12 §4`); no
datatype-specific level calculus is added.

```
data Vec (A : Type) : Nat -> Type where {
  VNil  : Vec A 0;
  VCons : (n : Nat) -> A -> Vec A n -> Vec A (n+1)
}
```

This elaborates to the kernel family `Vec (A : Type ℓ) : Nat -> Type ℓ` with
constructors at distinct **index instances** — `VNil` at `0`, `VCons` at
`n+1` (`14 §1`, the `vnil`/`vcons` canonical form). The index varies per
constructor; the **parameter** `A` is fixed across the family. The same power
refinement types give (`§5`) is expressed *in the data declaration*: a function
whose argument has type `Vec A (n+1)` **cannot** be applied to an empty vector.
The non-emptiness is in the type, and the impossible application is a kernel
type error (the index `n+1` cannot unify with `VNil`'s `0`).

### 2.2 Constructor signatures and telescope scope

A constructor with an explicit signature is written:

```
C : (x1 : A1) -> ... -> (xm : Am) -> D Δ_p t̄
```

The elaborator peels the leading function binders as the constructor telescope
`Δ_k` and checks the final codomain as the constructor result. Telescope scoping
is left-to-right:

- each earlier constructor binder is in scope for later argument types and for
  the result-index expressions;
- data-head parameters are in scope throughout every constructor signature;
- result-index expressions may mention data parameters and constructor
  telescope binders, but bind no new variables;
- an anonymous arrow `A -> ...` is a non-dependent telescope entry with a
  generated inaccessible binder name.

Implicit constructor binders may be written with the existing implicit-binder
spelling, `{x : A} -> ...`; they are ordinary telescope entries whose arguments
may be inserted at constructor uses by the full elaborator (`39`). This initial
feature does **not** add named-argument application or record-field labels inside
an explicit dependent constructor signature. The old record-style shorthand
`C { f : A, g : B }` stays sugar for the simple default-result form; dependent
record-field constructors are a later surface refinement.

### 2.3 Allowed constructor-result targets

The constructor result MUST be the declared family at the declared parameters
and some well-typed index expressions. Operationally, after ordinary
elaboration and transparent/WHNF exposure, the codomain must be definitionally
equal to:

```
D p̄ ī
```

where `D` is the family being declared, `p̄` are the data-head parameters in
their declared order, and `ī` has exactly the family's index arity. The
definitionally-equal class permits harmless aliases or reducible notation, but
it does **not** permit a different family head, changed parameters, an
undersaturated/oversaturated target, or a non-family result. Such a declaration
is rejected as a bad constructor result target before or while forming the
kernel inductive declaration. Strict positivity, nested structural admission,
mutual rejection, and W-style admission remain exactly the kernel rules of
`14`; the surface does not re-open them.

Examples:

```ken
data Ty = TInt | TBool

data Tm : Ty -> Type where {
  LitInt  : Int -> Tm TInt;
  LitBool : Bool -> Tm TBool;
  If      : (t : Ty) -> Tm TBool -> Tm t -> Tm t -> Tm t
}
```

and a proof-carrying constructor shape:

```ken
data CheckedSource : Type where {
  MkCheckedSource :
    (sid : SourceId) ->
    (bs : Bytes) ->
    (len : Nat) ->
    UnitByteLength bs ->
    IsUtf8 bs ->
    SourceLength bs len ->
    CheckedSource
}
```

The second example is only an expressibility example for proof-carrying
constructor telescopes. It does **not** change the CAT-5 `Source` contract and
does not unblock CAT-5 D3; CAT-5 D3 remains routed through the separate
`KM-sigma-projection-execution` mechanism.

**Impossible constructors at an index** (the load-bearing reconcile, resolved in
`§4.3`). Matching a scrutinee of type `Vec A (n+1)` **need not** write the
`VNil` arm: `VNil : Vec A 0`, and `0 ≢ n+1`, so `VNil` is
**type-impossible** at this index. This is sound and does **not** weaken the
kernel's requirement that `elim_Vec` receive a method for *every* constructor
(`14 §3`): the elaborator **synthesizes** the `VNil` method by **absurdity**
from the unsatisfiable index equation (`§4.3`), so the kernel still receives a
*total* `elim_Vec`. The surface omission is a convenience the elaborator
discharges, never a hole in the eliminator.

## 3. Pattern matching → `elim_D`

```
view area (s : Shape) : Decimal = match s {
  Circle r       => 3.14159d * r * r
  Rect   w h     => w * h
  Tri    b h     => 0.5d * b * h
}
```

`match` scrutinizes one or more expressions and selects the **first** arm whose
pattern matches. Patterns are as in `32 §4`: constructors `C p̄`, variable
binders, the wildcard `_`, literals, tuples/pairs, record patterns, as-patterns
(`p as x`), or-patterns (`p | q`, same binders), and optional **guards**. It is
**not** a new kernel primitive — it **elaborates to `elim_D`** (`../10-kernel/14
§3`, `39 §2.6`).

### 3.1 Compilation (pattern matrix → nested eliminators)

The elaborator compiles a `match` by the standard pattern-matrix algorithm
(column-by-column): pick a scrutinee column, split on its head constructors into
one `elim_D` per matched type, and recurse on the residual matrix under each
constructor's freshly-bound fields. The result is a tree of nested `elim_D`
applications — one eliminator per scrutinized inductive, nested for nested
patterns. Specifically:

- **Constructor patterns** drive the `elim_D` split: arm `Cₖ p̄ => e` becomes the
  `cₖ` method, with `p̄` matched against `cₖ`'s fields in the residual matrix.
- **Variable / wildcard** patterns bind (or discard) the scrutinee in a method
  that does not split further; a column of all-variables needs no eliminator.
- **Literal** patterns are checked against the scrutinee type (`31 §3`, `35
  §4`) and compile to the selected value-comparator test below (a guard-like
  `if` chain, `39 §2` item 7); a literal column is **not** closed, so a literal
  `match` is exhaustive **only** with a final variable/wildcard arm (`§4.1`).
  The lexical Boolean literals `true` and `false` are the exception in pattern
  position: before comparator selection, they elaborate as the ordinary `Bool`
  constructor patterns from `14 §3`. They therefore contribute constructor
  coverage: `true` and `false` together exhaust `Bool`, and a following
  variable/wildcard arm is unreachable (`§4.2`).
- **Tuple / record** patterns project the (negative) `Σ`/record components (`13
  §3`, `33 §2`) and match componentwise — no `elim_D` (records are negative,
  matched by projection, `14 §4`).
- **As-patterns** `p as x` bind `x` to the whole value matched by `p`, at the
  original scrutinee type. Their match set, coverage, and reachability are
  exactly those of `p`. The alias must not duplicate a name bound within `p`.
- **Or-patterns** `p | q` duplicate the residual arm under both alternatives.
  Each alternative binds every name exactly once, and all alternatives have
  the same binder-name set. Corresponding bindings must have definitionally
  equal types in the common context before the branch. The first alternative's
  left-to-right binder order is the canonical branch order; later alternatives
  map their bindings into those slots by name. The source arm is reachable if
  at least one alternative has a non-empty residual, and its coverage is the
  union of the alternatives' coverage. Checking the binder types in the common
  pre-branch context is deliberately conservative: it may reject a dependent
  or-pattern whose binder types would coincide only after distinct
  alternative-specific refinements. Such a pattern must use separate arms
  unless a later rule supplies a sound refined-context join.

#### Literal-pattern comparator selection

A literal pattern always has the scrutinee's type as its expected type. Numeric
defaulting therefore does not choose a match comparator: the literal is first
checked at that expected type by `35 §4`, and comparator selection uses the
resulting carrier. A literal form that the expected carrier does not admit is a
type error.

The comparator is a **value-level branch-selection operation**. It returns a
`Bool`; it neither constructs an `Equal` proof nor adds an equality hypothesis
to the branch. A lawful `DecEq` certificate may license a comparator, but is not
required when the type's normative value semantics already fixes a total
comparison. This distinction is necessary for IEEE floating point and
non-canonical `Decimal`, whose value comparators are deliberately not lawful
`DecEq` operations.

| Literal after expected-type checking | Match comparator | Equality authority and boundary |
|---|---|---|
| numeric at `Int` | `eq_int` | Exact integer value equality and the registered `Int` equality certificate (`18a §5.4`). |
| numeric at `Int8`/`Int16`/`Int32`/`Int64` or `UInt8`/`UInt16`/`UInt32`/`UInt64` | exact fixed-width integer value equality | The mathematical fixed-width value fixed by `35 §2.2`; comparison is exact and does not perform arithmetic, wrap, widen, or narrow. |
| numeric at `Float` | `eq_float` | IEEE-754 binary64 `==`, including NaN unequal to every value and `+0.0` equal to `-0.0` (`35 §2.4`, `18a §5.4`); explicitly not proof equality. |
| numeric at `Float32` | `eq_float32` | IEEE-754 binary32 `==` with the same boundary (`35 §2.4`, `18a §5.4`); explicitly not proof equality. |
| numeric at `Decimal` | `decimalEq` | Exact base-10 value equality by exponent alignment (`18a §5.6.1(4)`), not definitional equality of the non-canonical pair carrier and therefore not `DecEq Decimal`. Literal-pattern use additionally requires a total `Bool` result; while `18a §5.6.1(2)` permits the unbounded-alignment case to remain stuck, `Decimal` literal patterns reject rather than emit a non-selecting match. |
| string, ordinary or raw | codepoint-wise `String` `eq` | Equality of the canonical `String` value (`37 §2.1`, `§2.5`); value-level comparison, independent of whether the lawful `DecEq String` instance has landed. |
| character | `eqChar` | Unicode-scalar/codepoint equality, derived through the `Int` projection (`18a §5.9.1(3)`); value-level comparison, independent of the separately staged lawful instance. |
| byte string or bracketed hexadecimal bytes | byte-sequence content equality | Equality of the immutable, length-determined byte sequence (`38 §1.1`); representation and equality acceleration are unobservable. This row assigns no new surface operation name. |

The absence of a law-carrying `DecEq` instance does not by itself block a row
licensed by normative value equality: literal matching consumes only the
comparator's `Bool`. It blocks a row only when that certificate is the row's
named equality authority. In particular, the separately staged `DecEq Char`
and `DecEq String` instances do not replace or weaken the value comparators
named above.

A numeric literal at a user-defined carrier remains outside this table while
the `35 §4.2` user-type literal mechanism is staged. Any literal/carrier pair
without a row or without the row's total comparator rejects at elaboration; the
elaborator never guesses a comparator from spelling, representation, or an
unrelated `Eq` proof.

**Nested recursive fields.** When a constructor field contains recursive values
through a declared strictly-positive parameter path, its method type includes
the lifted IH of `14 §3.2`. The source arm still binds the field written by the
program. When a nested match deconstructs that field, its branch context
deconstructs the lift in lockstep: every pattern-bound recursive child receives
the corresponding motive instance, and every pattern-bound enclosing child
retains the residual lift for its contained occurrences. A recursive theorem or
computation over the field consumes those generated hypotheses. Re-emitting an
unrestricted self-call, discarding the lift, or admitting the declaration
without this branch context is not a valid lowering. The Type-classified
surface path is landed and executes through the generated lift and nested ι.
The narrower Ω-classified dependent-motive selector remains gated on
`KERNEL-RECURSIVE-RESULT-SURFACE` until the surface can expose its exact
associated proof result; kernel admission is not blanket-gated on that
residual.

#### 3.1.1 Recursive results and induction hypotheses for nested fields

The two contextual surface forms are:

```ken
recursive result for xs
induction hypothesis for xs
```

Each selects the kernel-supplied recursive method result associated with the
resolved surface binding `xs`. They are the `recursive_result` and
`induction_hypothesis` primary expressions of `32 §3`. Neither is function
application, an invocation of the enclosing definition, a generated identifier,
or a general-recursion construct.

The validity rule is positive and exhaustive. One of the two forms is valid if
and only if its operand resolves to a surface variable whose current branch
environment carries exactly one validated structural-result association and the
form agrees with the selected result's classifier. Such an association exists
precisely while compiling a nested strictly-positive match method in which:

1. the source pattern binds an enclosing recursive field;
2. the checked method telescope supplies the structural support evidence and
   recursive method result for that same field and structural occurrence; and
3. those three components form a one-to-one, in-range association with the same
   support provenance.

The association begins in the branch body where the field is bound and remains
available in lexically nested expressions until that binding is shadowed or the
branch ends. It is attached to the resolved binding identity, not its spelling:
copying the field into a `let`, projecting from it, or introducing another
variable with the same text does not copy the association. This is why the
selectors say **`for`**, not possessive `of`: the hidden result is associated
with this binding in this branch, not an intrinsic projection from the field
value. The rule is structural and does not recognize a carrier, constructor,
field name, or motive by name.

After validating the association, elaboration classifies the type assigned to
its hidden result by the checked method telescope:

- if that result type is classified by `Type l`, only
  `recursive result for xs` is valid;
- if that result type is classified by `Omega l`, only
  `induction hypothesis for xs` is valid.

The classification is of the **selected result**, never its structural support
evidence. In particular, a topology-carrying `All^Omega` support application may
itself inhabit `Type` while the associated recursive result is an `Omega`-valued
proof; that result still requires `induction hypothesis for xs`. Conversely, a
proof-relevant witness whose type is classified by `Type` requires
`recursive result for xs`, regardless of the programmer's proof intent. The
split follows the result sort, not terminology inferred from its use.

The names use the settled vocabulary for those two roles rather than one
Ken-local umbrella term: a `Type`-classified recursive method result is a
**recursive result**, while an `Omega`-classified one is an **induction
hypothesis**. They are not aliases, and no sort-agnostic selector remains. If
the written form disagrees with the classifier, elaboration rejects with
`RecursiveResultSortMismatch` and names the exact form required at that
occurrence. If unsolved metavariables leave the classifier ambiguous between
`Type` and `Omega`, elaboration rejects with `RecursiveResultSortAmbiguous`; it
does not guess or default either spelling.

Consequently, for an arbitrary branch binding `u` and an otherwise unnamed
validated association `u ↦ r`, `recursive result for u` selects `r` when the
type of `r` is `Type`-classified, and `induction hypothesis for u` selects `r`
when it is `Omega`-classified. Removing that association makes the corresponding
form invalid without any new case in this specification.

Elaboration replaces either selector with the exact hidden recursive method
result from the association. No generated support identifier or hidden method
binder enters source scope, and constructor-pattern arity is unchanged. The
ordinary field expression `xs` retains its declared source type; neither it nor
an owner self-call is reinterpreted as the recursive result. Missing, duplicate,
swapped, and foreign associations reject fail-closed under `39 §2.3`/`§4`.

Schematic nested support methods may therefore consume whole-field results
without a finite source unroll. Under a `Type`-classified motive:

```ken
Join xs ys ↦ combine
  (recursive result for xs)
  (recursive result for ys)
```

Under an `Omega`-classified motive, the same constructor branch instead uses:

```ken
Join xs ys ↦ combineProof
  (induction hypothesis for xs)
  (induction hypothesis for ys)
```

This addition does not alter ordinary direct or W-style matching. A direct
recursive field continues to use the existing exact-motive self-call rewrite,
and a W-style field continues to use its existing Pi-abstracted induction
hypothesis. Neither form receives a structural-result association merely by
being recursive. In a match that contains neither selector, pattern arity, bound
variables, branch types, termination routing, diagnostics, and emitted core
behaviour for direct and W-style cases are normatively unchanged.

### 3.2 Dependent-motive recovery

`elim_D` takes a **motive** `M : (Δ_i) → D Δ_p Δ_i → Type ℓ'` (`14 §3`).
The elaborator **recovers** `M` from the `match`'s expected result type by
abstracting it over the scrutinee and its indices:

- **Non-dependent `match`** (result type independent of the scrutinee) → the
  motive is **constant**, `M = λ ī x. T` for the expected `T`; this is the
  ordinary recursor.
- **Dependent `match`** (result type mentions the scrutinee or its indices —
  indexed families, and the body-as-motive obligations of
  `../20-verification/22 §4`) → `M` is the expected type **generalized** over
  the scrutinee `x : D Δ_p ī` and the indices `ī`. Recovering this dependency is
  what lets a branch refine the result type (essential for `§2` indexed families
  and for `22 §4`'s inductive postconditions). The elaborator solves the motive
  by higher-order pattern unification against the expected type (`39 §2` item
  3);
  genuine ambiguity is a surface error, never a guess (`39 §3`).

#### 3.2.1 Peeled-field injectivity, sibling convoy, and goal refinement

`§3.2`'s motive recovery refines the result type over the **scrutinee's own**
index only. For an indexed family (`§2`), a branch's constructor equation —
`target_ī ≡ scrut_ī` at the family's index type, e.g. `Suc m ≡ Suc n` for
`vcons`'s branch of `Vec A (Suc n)` — additionally licenses two further
re-typings, carried into the branch via the kernel's own `Eq`/`J`/`Cast`
(`16`), **never postulated**:

- **Constructor injectivity for a peeled recursive field.** When the
  constructor equation itself reduces further by the kernel's own
  same-constructor `Eq`-at-inductive case (`16 §2.2`) — e.g. `Suc m ≡ Suc n`
  reduces to `m ≡ n` — a recursive field typed at the **un-peeled** local
  variable (`xs : Vec A m` in `vcons`'s branch) may be re-typed to the goal's
  index (`Vec A n`) by `cast`ing along the type-level congruence the reduced
  equation licenses. This is what makes `tail : Vec A (Suc n) → Vec A n`
  well-typed: the branch peels `Suc m ≡ Suc n` to `m ≡ n` and re-types the
  tail field accordingly.
- **Sibling convoy.** An **outer** binder already in scope — typically
  another function parameter of the same indexed family sharing the
  scrutinee's index (`w : Vec B n` alongside `v : Vec A n`) — is re-typed
  the other direction (via the equation's symmetric form) so that a nested
  match on it stays exhaustive without an impossible arm. Without this, an
  omitted-constructor case on the sibling (`§2.4`) cannot discharge by
  absurdity, because its own index premise is not yet known to be
  constructor-headed.
- **Goal refinement.** A branch whose body **constructs** a fresh value of
  the family (rather than re-using an existing, now re-typed binding) has no
  context variable for the above to redirect — its natural type uses the
  constructor's own target index directly. Such a branch's checking goal is
  refined the same way (substituting the scrutinee's un-refined index for the
  constructor's own), and the checked result is `cast` back up to the
  original goal.

**Boundary.** These re-typings compose through **one** level of nesting (a
convoy sibling, or a single peeled recursive field, re-used directly). A
branch that both destructures a sibling through its **own** nested match
*and* re-uses a field from the **enclosing** match's own destructuring in the
same expression (e.g. a two-vector `zip`'s recursive step, which nests a
match on the second vector while also passing the first vector's own tail
into a recursive call) is a known gap — the sibling-convoy re-typing does not
yet distinguish a genuine outer *parameter* from a field the **enclosing**
match already bound, and can substitute the wrong (though never
unsound — always kernel-proved) index there. `tail` and a single-level
convoy (one sibling, one nested match, no further reuse of an enclosing
field) are the pinned, tested boundary of this WP; the full two-vector `zip`
recursive step is a follow-on.

### 3.3 Per-branch definitional refinement (the hypothesis)

In the `cₖ` arm, after the `elim_D` split, the scrutinee is **definitionally**
the matched constructor: `s ≡ cₖ field̄` holds by the ι-rule the branch sits
under (`14 §3`). The verification layer turns this into a **hypothesis** — the
scrutinee equation `(_ : Eq A s (cₖ field̄))` added to the local context `Γ`
(`../20-verification/22 §3`) — so inside the `Circle r` arm one may *assume*
`s ≡ Circle r`, and a dependent motive (`§3.2`) refines the **result type** of
that arm accordingly. This per-branch refinement (a fact about the value **and**
a refinement of the type) is what AC6 pins — the surface origin of `22 §3`'s
path-sensitive `Γ`.

**Guards do not refine and do not cover.** A guarded arm `Cₖ p̄ if g => e`
elaborates to a conditional *inside* the `cₖ` method (`39 §2` item 7); because the
guard `g` may fail, the arm does **not** by itself discharge the `cₖ`
constructor for exhaustiveness (`§4.1`). Guards are an arm-selection refinement,
not a coverage contribution.

### 3.4 Transport by a propositional equality — the `J` former

Per-branch refinement (`§3.3`) rewrites the goal by an equality that holds
**definitionally** — the scrutinee equation `s ≡ cₖ field̄` that the `elim_D`
split makes true by the ι-rule. It does **nothing** for an equality that holds
only **propositionally**: a proved `p : Eq A a b` that is *not* a definitional
convertibility (e.g. an order hypothesis `IsTrue (leq k k') = Eq Bool (leq k k')
True` over an **abstract** key `k`, where `leq k k'` is a *stuck* redex no match
can fire). To rewrite a goal mentioning `a` into one mentioning `b` along such a
`p`, the surface provides one term-former, the identity eliminator **`J`** — the
explicit, proof-carrying transport every dependent theory supplies (Agda
`subst`, Lean `▸`, Coq `eq_rect`).

`J` is **not** a new kernel construct. It elaborates directly to the kernel's
existing `Term::J` (`../10-kernel/15 §4`), which the kernel derives to `cast`
and **reduces on any equality** (`../10-kernel/16 §3`); both `J` and `cast` are
already in `trusted_base()`, so this former adds **nothing** to the trust
surface — it only makes an already-trusted eliminator reachable from `.ken`.

**Surface syntax and typing rule (the pin).** `J` is written applied to three
arguments, `J motive base eq`, and is elaborated in **infer mode** (like the
kernel eliminators, unlike the checked-mode `Refl`/`absurd`/`Proved` sugar whose
motive comes from the goal): the equality's type `A` and endpoints `a`, `b` are
recovered from `eq`'s inferred type, and the result type is synthesized. The
rule is verbatim the kernel's `J`-formation (`../10-kernel/15 §4`):

```
  Γ ⊢ eq : Eq A a b
  Γ ⊢ motive : (b' : A) → Eq A a b' → s     (first domain ≡ A;  s any sort)
  Γ ⊢ base : motive a (refl a)
  ─────────────────────────────────────────────────────────────  (J)
  Γ ⊢ J motive base eq  :  motive b eq
```

The motive's codomain **sort `s` is unconstrained** — it may be `Type ℓ`
**or `Ω`**. This is load-bearing, not incidental: an `Ω`-valued motive is what
lets `cong` conclude a proof-irrelevant type-equality and what lets a Branch-B
proof obligation living in `Ω` (`../50-stdlib/52 §5`) be discharged by transport
at all. `J` derives its computation from `cast`, whose rule deliberately does
**not** require the endpoints to be convertible (that non-requirement *is*
transport):

```
  Γ ⊢ e : Eq Type A B      Γ ⊢ t : A
  ─────────────────────────────────────  (cast)   -- ../10-kernel/15 §3, 16 §3.1
  Γ ⊢ cast A B e t  :  B
```

Because the motive's binder types are fixed by `eq`, the user writes the motive
as an unannotated lambda `\b' _. G[b']` — its domains need no ascription.

**The motive is user-written (the Agda-`subst` posture).** The user abstracts
the rewritten occurrence explicitly, naming `G[·]` in the motive. Inferring
*which* occurrences of `a` to generalize — `rewrite` / with-abstraction /
auto-motive spelling — is a **separate, non-soundness ergonomic sugar, out of
scope here**; it is deferred to a later surface-syntax WP.

**Why this is sound (and what it is not).** `J` **asserts nothing**: the motive,
the base, and the user-supplied witness `eq` are all kernel obligations on the
emitted `Term::J`, kernel-re-checked in full — an ill-typed transport (wrong
motive, wrong endpoints, or a proof of the *wrong* equation) is **rejected by
the kernel**, never silently accepted. Every rewrite is witnessed by a proof the
user supplied; the elaborator never manufactures an equality. This is
deliberately **not** an implicit congruence: the unsound cross-wise
`Eq`-congruence that would identify `bool_eq x y` with `bool_eq y x` (smuggling
propositional symmetry into definitional equality) stays a hard **NO**
(`../10-kernel/16`; `../50-stdlib/51` K6), and if a realistic transport goal
fails to *compute*, the remedy is a sound kernel completeness fix, never an
elaborator that routes around the kernel. There is **no conversion change**
here — transport discharges through the `J`/`cast`
*typing* rule (the equation obligation lands on the user's `eq`), so this former
implies **no `../10-kernel` conversion-completeness note**.

The five everyday combinators built on `J` — `subst`, `cong`, `cast`, `sym`,
`trans` — are ordinary non-recursive library `view`s, **not** formers; they are
listed in `../50-stdlib/53-transport.md`.

### 3.5 Proof-returning dependent motives

A `match` may be checked against an expected **proof target** whose type is an
`Ω` proposition and whose statement depends on the scrutinee. This is not a new
surface construct and not a CAT-4 special case: it is the ordinary dependent
eliminator of `§3.2` with an `Ω`-codomain motive (`../10-kernel/14 §3`).

The canonical shape is:

```
match s {
  C0        => p0
  C1 x ...  => p1
}
```

checked at a target `P[s] : Ω`, where `P` may be a direct equality
(`Equal A lhs[s] rhs[s]` / `Eq A lhs[s] rhs[s]`) or any proof expression that
mentions `s` after the same transparent unfolding and WHNF exposure used by
ordinary elaboration. The elaborator recovers the motive
`M = λx. P[x]` (and, for indexed families, `λī x. P[ī, x]`) and emits
`elim_D M ... s`. Each branch is then checked against the **specialized**
target:

- the `C0` method must inhabit `P[C0]`;
- the `C1 x ...` method must inhabit `P[C1 x ...]`;
- a nested proof-returning `match` repeats the same rule at its own scrutinee.

`Equal` has no separate rule here. The prelude spelling is a transparent alias
for the kernel's computing `Eq` (`../50-stdlib/53 §1`), so branch refinement is
ordinary substitution into the equality operands followed by the existing
`Eq`/`Top`/`Bottom` reductions (`../10-kernel/16 §1.4`, §2). A branch may close
with `Proved`, `Refl`, `J`, or a library combinator only if that term checks against
the branch-specialized proof target. In particular, proof irrelevance does **not
erase the branch obligation**: it equates proofs only after both proofs have
already been checked at the same proposition (`../10-kernel/16 §1.2`).

**Acceptance boundary.** A proof-returning dependent `match` accepts when:

- the scrutinee elaborates to an inductive value and the target classifies as a
  sort (`Ω_l` for proof targets, `Type l` for ordinary large elimination);
- the motive can be recovered by generalizing the expected target over the
  scrutinee and any indices it mentions;
- every type-possible constructor has a method whose body checks against the
  target after the constructor fields and branch refinement have been applied.

**Rejection boundary.** The same surface rejects when any branch supplies a proof
for the wrong specialized target, when the target does not classify as a sort,
when motive recovery would require guessing which occurrences to generalize, or
when the ordinary exhaustiveness/reachability checks of `§4` fail. The
elaborator may report an explicit "unsupported dependent motive" for a
not-yet-implemented recovery shape, but it must not silently fall back to a
constant motive that ignores the dependency, and it must not accept all proof
branches by proof irrelevance.

### 3.6 The `eqn:` modifier — dependent case-analysis on a stuck scrutinee

A `match e { … }` on a **stuck/neutral** scrutinee — a comparison like
`da.eq x y` that does not reduce under abstract `x`/`y` — discards the
connection between `e` and the constructor each branch matched. A proof that
needs "`e` **is** `True` in the `True` branch" then cannot be written directly,
and authors hand-roll a `bool_dichotomy` + named dispatcher + explicit `J`-motive
(the `§3.4` transport, spelled out by hand: ~15–30 lines, re-paid at every
`DecEq`/`Ord` proof). The **`eqn:` modifier** is the sugar that collapses that
idiom. It is **`match` plus one modifier — not a new keyword** (`32 §3`):

```ken
match e eqn: h { C1 ↦ … ; … ; CN ↦ … }
```

In the branch for constructor `Cₖ` it **binds** `h : Equal T e Cₖ` — the proof
that the scrutinee *is* the matched constructor — and **generalizes `e`'s
occurrences in the expected goal**, so the body is checked at the
constructor-specialized goal and transported back (below). The semantics are
uniform whether `e` is a stuck application or a bound variable — no special case.

**Token — `eqn: h`.** From Coq's `destruct … eqn:` lineage, familiar to the
dependent-types audience, and self-documenting: `eqn` names exactly what `h` is,
the scrutinee=constructor **eq**uatio**n**. It is a **contextual modifier
keyword** in the scrutinee slot — the same shape as `visits`/`requires`/`where`
(`32`), not a new declaration or `match` keyword. *Rejected:* `as h` — `as`
already binds a **value** (import-aliasing `import M as N`, `33 §3`; as-patterns,
`§3.1`), so `e as h` misreads as binding the scrutinee *value* rather than the
equation; and `with h` — no Ken precedent and not self-documenting. The modifier
applies to a **single-scrutinee** `match` (as shipped); a branch **must** bind
the named `h` (always available even if unused), and the modifier generalizes
**all** occurrences of `e` (no partial-occurrence control). Per-scrutinee `eqn:`
on a *multi*-scrutinee `match` is not shipped — the pre-existing multi-scrutinee
`match e1, e2 { … }` grammar-vs-parser gap (`§3.1`) is not this clause's to
resolve.

**Scope — finite nullary-constructor enumerations only.** The scrutinee type `T`
must be an inductive **all of whose constructors are nullary**: `Bool`;
`OrdResult = Lt | Eq | Gt` (`../50-stdlib/`); and any future all-nullary enum.
This is one deliberate step past `Bool` — the near-term `compare`/`Ord` path
scrutinizes the 3-way `OrdResult`, which a `Bool`-only form would **not**
subsume (re-framing for `OrdResult` a brick later is exactly the proliferation
this construct exists to kill). It does **not** cover general dependent matching
over inductives **with fields** — constructor fields introduce fresh existentials
in the branch equation plus index unification, a separate and larger capability
that neither the current sites nor the `Ord` path needs. The elaboration
machinery is identical for two versus N nullary constructors, so `all-nullary-
ctor`, not `Bool`, is the natural boundary.

**Elaboration — pure sugar over existing primitives (`§3.4`), empty
`trusted_base()` delta.** For scrutinee `e : T`:

1. **Generalize** the occurrences of `e` in the **expected goal** `G` to a fresh
   `v : T` (higher-order pattern abstraction, `§3.2`, `39 §2` item 3), and form the
   scrutinee's own eliminator with a motive that **returns a function over the
   branch equation**: `M := λ (v : T). Equal T e v → G[e := v]`.
2. **Check each branch** for constructor `Cₖ` as a `λ` binding the `eqn:` name
   `h : Equal T e Cₖ`, its body the branch expression at the specialized goal
   `G[e := Cₖ]`.
3. **Recover the goal** by applying the completed eliminator — at the scrutinee
   its type is `M e = Equal T e e → G` — to **`Refl : Equal T e e`**, yielding
   `G`. The equation-returning motive plus the `Refl` application **is** the
   `§3.4` `J`-style transport, realized directly through `T`'s own eliminator; no
   separate dichotomy term or materialized `J` former is built.

The construct is assembled from **only** `T`'s eliminator (`Term::Elim`), `Equal`
(`Term::Eq`), and `Refl` (`Term::Refl`) — the landed `bool_dichotomy : (b : Bool)
→ Or (Equal Bool b True) (Equal Bool b False)` is the **precedent** that such a
per-branch-`Refl` equation construction is *derived, never a postulate* (the
modifier builds the tighter eliminator-with-equation-motive form rather than
materializing a dichotomy). It adds no new `Decl::Opaque`, no `postulate`;
`../10-kernel` is untouched and the **`trusted_base()` delta is empty**. Pure
elaboration sugar (`39 §1`, untrusted).

**Fail-closed — the soundness backstop.** The elaborator is **untrusted**: it
assembles the transported term and submits the **whole** term to the kernel
(`kernel_infer`, the same discipline as `39`'s instance resolution). A wrong or
under-general motive can only make the assembled transport **ill-typed** →
**kernel-rejected**; the output's type is pinned to the author-declared goal `G`
and re-checked, so the construct can **never** admit a wrong-but-accepted proof.
If a **well-typed** motive cannot be formed (occurrences ill-typed to abstract),
the elaborator **errors explicitly** (fail-closed) — it must not silently pick a
partial abstraction.

**The reverted-hypothesis pattern (where the token payoff is largest).** The
common sound-direction sites case-split to transport a **hypothesis** whose type
mentions `e` (`ph : IsTrue (list_eq … (Cons …) (Cons …))`), while the **goal**
itself does not mention `e`. The clean subsumption: the author **π-abstracts the
hypothesis into the goal** — writes the helper to *return a function*
`IsTrue (e …) → Equal …` — so `e`'s occurrences live in the goal's **domain** and
the modifier transports them automatically:

```ken
-- e = `list_eq a da.eq (Cons a x xs) (Cons a y ys)`, which reduces to
-- `da.eq x y && list_eq a da.eq xs ys`; the scrutinee is the head test `da.eq x y`.
fn list_deceq_sound_cons (a : Type) (da : DecEq a) (x y : a) (xs ys : List a)
  :   IsTrue (list_eq a da.eq (Cons a x xs) (Cons a y ys))
    → Equal (List a) (Cons a x xs) (Cons a y ys) =
  match (da.eq x y) eqn: h {
    True  ↦ λ ph. … da.sound x y h : Equal a x y, the IH on xs/ys, and Cons-cong … ;
    False ↦ λ ph. absurd ph   -- domain reduced to `Bottom` (list_eq → False, K7)
  }
```

In the `True` arm the domain reduces to `IsTrue (list_eq a da.eq xs ys) → …` (the
recursive comparison, discharged by the induction hypothesis) and `h : Equal
Bool (da.eq x y) True` feeds `da.sound x y h`. In the `False` arm it reduces to
`Bottom → …`, i.e. `λ ph. absurd ph` — a **genuinely-transported** `Bottom`
(`list_eq` computes to `False`, closed by `16`'s `K7`), **never** a papered
`Refl`. This is what collapses the hand-rolled `§3.4` dispatch to a direct
`match`; the normative payoff is the reverted-hypothesis form, not the
goal-side-only case.

## 4. Exhaustiveness and reachability (required — the headline safety)

Ken requires this from day one. The checker is a **surface algorithm** (not a
kernel rule, `§4.4`) run during `match` compilation (`39 §2.6`).

### 4.1 Exhaustiveness

The arms MUST cover every **type-possible** constructor of the scrutinee type:
every value of the scrutinee type matches some (unguarded, `§3.3`) arm. The
algorithm walks the pattern matrix and, per scrutinee column, computes the set
of constructors the arms cover; a constructor of the scrutinee's `data` left
uncovered makes the `match` a **compile error** that **names the unmatched
pattern** (`§4.4` explains why naming it is the *surface* checker's job and not
the kernel's). A wildcard/variable column covers all remaining constructors. For
a **closed** `data` an exhaustive `match` needs **no** `default` arm, and the
compiler **proves totality** of the case analysis (`§4.4`).

> The unmatched-pattern **witness** is a most-general pattern not covered by any
> arm (e.g. `Blue`, or `VCons _ _ _` for a partial `Vec` match) — the
> constructive evidence of the gap, reported in the diagnostic (`24`).

### 4.2 Reachability

Every arm MUST be **reachable**: under the first-match semantics (`§3`), an arm
whose patterns are entirely subsumed by the union of the earlier arms matches no
value and is a **redundant-arm** warning/error. The same matrix walk detects it
(an arm reaching an empty residual matrix is unreachable). Two subtleties:

- A **guarded** arm is *not* counted as covering its constructor (`§3.3`), so a
  later unguarded arm for the same constructor is **reachable** (it catches the
  guard-failure cases) — guards loosen reachability exactly as they loosen
  coverage.
- A **literal** column never closes (`§3.1`), so a final variable/wildcard arm
  after literal arms is reachable (and *required* for exhaustiveness).

### 4.3 Indexed families — type-possible vs. index-impossible

For an indexed scrutinee (`§2`) the coverage obligation is over the constructors
**type-possible at the scrutinee's index**, not all constructors of the family.
A constructor whose target index cannot unify with the scrutinee's index is
**index-impossible** and need not be written. This splits cleanly, and the split
is the **soundness rule** the whole feature rests on:

- **Type-possible at the index** ⇒ the arm is **required**; omitting it is a
  non-exhaustiveness compile error (`§4.1`). The elaborator MUST NOT fabricate a
  method for a type-possible constructor (that would silently admit a partial
  function — the unsound failure mode).
- **Index-impossible** ⇒ the elaborator **synthesizes** the constructor's
  `elim_D` method by **absurdity**: the constructor's index equation (its target
  index ≐ the scrutinee's index) is refuted by constructor disjointness /
  index discrimination (e.g. `0 ≢ n+1` via `15`/`16`), yielding a proof of the
  empty index constraint from which the method body is `Empty`-eliminated. The
  kernel still receives a **total** `elim_D` (`14 §3`), so the omission is sound
  by construction.

The discriminator between "required" and "omittable" is therefore exactly **"is
this constructor type-possible at the scrutinee's index?"** — `§4.1` (closed,
non-indexed: every constructor type-possible ⇒ all required) and `§2`/this
section (indexed: only the index-satisfiable ones required) are the **same
rule** at two index regimes. The conformance corpus pins both faces and their
agreement (`§8`).

### 4.4 The trust boundary (why this is safe though untrusted)

The exhaustiveness/reachability checker is **untrusted surface** (`39 §1`), yet
the safety it enforces is **kernel-backed**, double-netting the headline:

- **The kernel proves the *eliminator* sound.** `elim_D` is total by strict
  positivity + structural ι (`14 §2`, `§9`); it **requires a method for every
  constructor** of the family.
- **The surface proves the *match covers* it.** A non-exhaustive `match` over a
  type-possible constructor cannot elaborate to a complete `elim_D` — the
  elaborator has no method body for the missing arm and (per `§4.3`) MUST NOT
  fabricate one, so the only honest outputs are **(a)** the surface
  exhaustiveness error naming the unmatched pattern (the good diagnostic), or
  **(b)** an under-applied `elim_D` the **kernel rejects** as ill-typed (the
  backstop). Either way a non-exhaustive `match` is **never** silently accepted
  as a partial function.

So the *safety* (totality of case analysis) is guaranteed even against a buggy
checker — the kernel cannot type an `elim_D` missing a method. What the surface
checker uniquely provides is the **quality** of the failure: a precise
"non-exhaustive, unmatched pattern `Blue`" rather than the kernel's bare
"eliminator under-applied." This is the crisp boundary: **kernel = the
eliminator is sound and total; surface = the match covers it, with a precise
witness.** (It is the `match` analogue of `39 §1`'s split — cleverness outside,
certainty inside.)

## 5. Refinement types

`{ x : A | φ x }` — the type of `x : A` for which the proposition `φ x : Ω`
holds (the comprehension subobject, `../20-verification/21 §2`; the predicate's
universe `Ω` is `../10-kernel/12 §5`, `16 §1`). At the surface:

```
def Nat        = { n : Int | n ≥ 0 }
def NonEmpty a = { xs : List a | xs ≠ Nil }
view head {a} (xs : NonEmpty a) : a = match xs { Cons x _ => x }
```

**The encoding — carrier plus obligation (normative, `21 §2`).** A refinement
`{x:A|φ}` is **not** a kernel type former. It elaborates to its **carrier `A`**;
the predicate `φ` is tracked by the (untrusted) elaborator and **every
introduction** of a value at the refinement — using an `A` where `{x:A|φ}` is
expected — **emits the obligation** `φ a` (`22 §2.1`), discharged by the prover
or surfaced as a typed hole (`24 §2`). Consequences, each grounded:

- **No implicit subset coercion past `φ`.** `A ≤ {x:A|φ}` (the introduction
  direction) costs the obligation `φ`; it is never a silent coercion. The
  **forgetful** direction `{x:A|φ} ≤ A` is **free** — in the carrier encoding it
  is the identity on `A` — and emits **no** obligation (`22 §2.1`/§2.5).
- **No runtime payload.** The proof component is a **mere proposition** (`16
  §1.2`) — proof-irrelevant and computationally irrelevant — so a refined value
  behaves as a bare `A` at runtime; refinements are **zero-cost** and pure
  compile-time enforcement.
- **Not a core `Σ` over Ω.** The naive reification `{x:A|φ}=Σ(A,φ)` is **not**
  used — it is collapsed by Ω-proof-irrelevance when the carrier is relevant
  (the landed `sort_pi_sigma` Σ-sort caveat, `21 §2`, `13 §4`/§5). The
  carrier-plus-obligation form is **independent** of that kernel erratum (it
  never forms a core `Σ` over an Ω predicate), so L2 builds on it as-is.

Refinements compose with `data`, records, and function arguments/results, and
are how `requires`/`ensures` desugar (`21 §1`/§2). Pushing a property into a
type makes the checker enforce it at **every** use — the surface route to L2
verification.

## 6. Smart constructors & views (optional sugar)

Pattern synonyms / view patterns (matching through an abstraction) and smart
constructors (a `view` that enforces an invariant and returns a refined type,
`§5`) are ergonomic sugar over `§1`–`§5`; whether to include them is
**OQ-syntax**. The semantic core — constructors + `elim_D` + exhaustiveness +
refinements — does not depend on them.

## 7. Level-discipline reconcile

Per the standing directive, every formation rule here is given its explicit
level computation and reconciled against `../10-kernel/12`; no rule adds a
universe computation — each is an instance of a landed kernel rule.

- **`data D (Δ_p) : (Δ_i) → Type ℓ`.** Constructor-argument types live at `ℓ`
  or below (predicativity, `14 §1` → `12 §2`); the family lands at the
  declared `ℓ`. **No new rule** — the kernel's inductive formation (`14 §1`)
  computes it.
- **W-style argument `(b:B) → D Δ_p t̄`.** Lives at `max(level B, ℓ)`; `14
  §1`'s
  rule forces `level B ≤ ℓ` (the domain is absorbed into the family level, `14
  §2.1`). Existing rule, **no new formation**.
- **`match` → `elim_D`.** The motive `M` may land in any `Type ℓ'` — **large
  elimination** is permitted under the predicative universe checks (`14 §3`), so
  a `match` may compute a *type* (e.g. an indexed result). No level restriction
  beyond the kernel's `Univ` checks (`12 §1`).
- **Proof-returning `match` → `elim_D`.** A proof target such as
  `Equal A lhs rhs` lands in `Ω_l` (`15 §2`, `16 §2`), so the recovered motive
  may have codomain `Ω_l` rather than `Type ℓ'` (`§3.5`, `14 §3`). This is the
  same predicative Π-into-Ω rule as other propositions (`16 §1.1`); it adds no
  universe coercion and does not turn the proof target into a `Type`.
- **Refinement `{x:A|φ}`.** `φ : Ω` (proof-irrelevant, `12 §5`/`16 §1`); the
  refinement's **core image is its carrier `A`** (`§5`), so it sits at `A`'s
  level `l` (`A : Type l`) — a *subtype at the same level*, predicative, **no
  universe bump** (`12 §2`/§3, non-cumulative). The obligation `φ a` is an Ω
  proposition discharged in V3 (`22`); it introduces no new universe (`22 §7`).

## 8. What WS-L must deliver here

Real sum types with constructors + a computing eliminator (no opaque lowering);
indexed/GADT-like families with dependent constructor signatures (`§2`) and
index-aware coverage; `match` → `elim_D` with dependent-motive recovery and
per-branch definitional refinement; proof-returning dependent motives whose
`Equal`/`Ω` target mentions the scrutinee, with wrong-specialized-branch
negatives; **exhaustiveness + reachability** checking with a named
unmatched-pattern witness; `Result`/`Option` in the prelude; and
refinement types with free forgetful coercion + obligation emission on
introduction. Acceptance is part of **G6** (real sum types end-to-end). The
whole layer is **untrusted**; the kernel re-checks every emitted `elim_D`
(`§4.4`).

The dependent-constructor feature should be built in separate WPs:

- **SURF-gadt-parser-ast** — accept the `32 §1` explicit `data ... : ... where`
  form, constructor `C : telescope -> D params indices` signatures, spans, and
  default-result sugar without changing current simple sums.
- **SURF-gadt-elaboration** — lower data-head parameters/indices and
  constructor telescopes to kernel inductive-family declarations, including the
  bad-result-target diagnostics of `§2.3` and kernel re-check of positivity.
- **SURF-gadt-coverage-diagnostics** — implement index-aware coverage for these
  explicit signatures, including named unmatched-pattern witnesses for
  type-possible constructors and absurd method synthesis for index-impossible
  constructors.
- **SURF-gadt-field-sugar** — later, add named-argument/record-field constructor
  ergonomics for dependent signatures. This is deliberately not in the initial
  parser/elaboration slice.

No build WP in this sequence implements CAT-5 D3. CAT-5 D3 remains on
`KM-sigma-projection-execution` and resumes unchanged after that mechanism
lands.

Conformance: `../../conformance/surface/data-match/` — AC1 (`data` is real:
construct-then-eliminate **reduces**), AC2 (`match`→`elim_D` computes; nested→
nested), AC3 (exhaustiveness — the headline: a missing case **rejects naming the
unmatched pattern**, the exhaustive version accepts — verdict **and** the named
witness flip), AC4 (reachability — a redundant arm flagged, verdict flips), AC5
(indexed family — the impossible application **rejects** *while* the impossible
arm may be **omitted**: a non-degenerate pair on the same `§4.3` rule), AC6
(branch refinement = `22 §3` hypothesis — a *dependent* motive, asserted
structurally), AC7 (refinement type — the obligation `φ` is **emitted** on
introduction (observe the VC structurally), the forgetful direction free, no
silent coercion), AC8 (proof-returning dependent motive into `Ω`), and AC9
(dependent-constructor syntax: positive `Vec`/proof-carrying declarations, bad
result target, positivity rejection through the kernel gate, and omitted
possible-vs-impossible coverage). Per-case verdict/structural-flip + the
**cross-case sweep**: the exhaustiveness/coverage class (`§4.1`/§4.3`) agrees:
"type-possible at the index ⇒ required; index-impossible ⇒
omittable-and-absurd-filled."
