# The standard-package catalog

> Status: **DRAFT v0**. Reframed by ES1 from "the stdlib (L8)" into the
> **standard-package tier** of the surface taxonomy (`../30-surface/30 §5`).
> This is the third tier: **ordinary Ken, optional, explicitly imported, out of
> `trusted_base()`** — not the always-present prelude (`30 §4`) and not the
> built-in surface TCB (`30 §3`). The monolithic **L8 dissolves** into this
> catalog; `docs/program/wp/L8-stdlib-core.md` is superseded. Each entry is
> ordinary Ken **with its derivation path from the built-ins stated** — a
> catalog entry with no path is a **hidden built-in** (a spec bug caught with
> CV's derivation-path table, `../../conformance/surface/taxonomy/`). ES4 builds
> these as in-repo packages under `catalog/packages/`.

This file is the spec/index side of the catalog. The source-style and
refinement contract for realized packages lives in
`../../docs/program/07-catalog-style-guide.md`.

The standard packages are **ordinary Ken** (they self-host atop the kernel and
the built-ins), with one discipline that distinguishes them from a typical
standard library: **their core abstractions carry their laws as propositions**
(`../20-verification/`), **proved, not postulated**, so the verification layer
can use them. A `Monoid` is not just `(append, empty)` — it is that plus
*proofs* of associativity and the unit laws.

## 1. What is NOT in this catalog (the line to `30`)

The everyday minimum that is **always in scope** is not a package — it is the
two lower tiers of the surface taxonomy (`../30-surface/30`):

- **Built-in (the surface TCB, `30 §3`)** — primitive types + literals
  (`Int`/`Float`/`String`/`Bytes`, `../30-surface/35`, `37`), the audited
  primitive ops (`../10-kernel/14 §5`), the effect/FFI boundary
  (`../30-surface/38`), and the base elaborator syntax.
- **Prelude (Ken-defined, always-present, `30 §4`)** — the **closed** union of
  the checked type/signature arm (`Auth`, `Bool`, `List`, `Option`,
  `ResourceKind`, `Result`, `Utf8Error`), the checked transparent
  subset-Σ definition `Char` (`18a §5.9.1`), and the internal-provision
  arm (`Nat`, kernel origin; `Pair`, compiler-bootstrap origin; `Equal`, `Prop`,
  `Proved`, `Top`, `Bottom`, keyed by kernel machinery). Its exact fifteen-name
  floor is `{Auth, Bool, Bottom, Char, Equal, List, Nat, Option, Pair, Prop,
  Proved, ResourceKind, Result, Top, Utf8Error}`. `Proved : Top` is a checked
  proof term, not a type former. Constructors enter only through exact recorded
  parentage; Pair's separate exact companion inventory is
  `{mk_pair, pair_fst, pair_snd}`. `Ordering` is **not** prelude — no primitive
  returns it and it has no internal-provision witness, so it is a package
  (`30 §4`). Kernel `Ω` and native `Eq`/`J` remain kernel vocabulary, not
  duplicate prelude postulates. `Equal` is the checked prelude floor definition
  whose body is that native `Eq` (`30 §4`, `§6`).

The ordinary Ken entries below are **packages**: imported, derivable,
re-checked. The tooling-facing `ken` command's diagnostic-code contract
in §5 is the stated exception: a process interface, not an importable
standard package. Core data `Unit`/`Empty`/`Either` plus the core
combinators (`id`, `∘`, `const`, `flip`) are packages — Ken `data`/defs
over the built-ins, not prelude. `Nat`, `Option`,
`Pair`, and `Result` are deliberate exceptions among core data carriers: source
must reach their canonical compiler-installed identities because the
internal-provision contract or public primitive signatures name them; a fresh
source declaration cannot recreate those identities.

The compiler-origin floor `Pair` derives transparently from the existing
dependent Σ former: `Pair A B ≡ (x : A) × B`, with checked transparent
introduction and projection helpers and definitional fst/snd β plus
reconstruction η (`../30-surface/34 §"Canonical non-dependent pair floor
family"`). It needs no primitive, postulate, package identity, or trust entry.
The type floor contains `Pair`; its separate companion inventory contains
`mk_pair`, `pair_fst`, and `pair_snd`. The floor realization is landed: current
catalog contracts use those four compiler-installed identities under Strict.
No catalog import, alias, or fallback supplies them.

The sole canonical `instance Ord Nat` remains defined by the class-owning
`Core.Classes.LawfulClasses`; `Data.Numeric.Nat.Order` may import/re-export only
that dictionary. The former Pair-floor proof deferrals are also discharged:
`pair_compare_eq_sound` and `pair_compare_lt_asym` are ordinary theorems, and
`proof eq_true_of_or for bool_or` is the exact public selector. The prior Pair
floor gate on the `Order → LawfulClasses → Compare → Pair` dependency is
therefore closed; these are landed identities and proofs, not permissions for a
second package-owned Pair.

## 2. Lawful classes (the verification-aware core) — packages

A curated set of **classes with laws** (`../30-surface/33 §5`), each a package
whose derivation bottoms out in the built-ins (the class + instances are Ken;
the `Int` instance of `Num`/`Ord` wraps the audited primitive op, `30 §6` F2).
The **first tranche** — `Eq`/`DecEq`/`Ord`, the pattern-setter — is pinned in
**`51-lawful-classes.md`** (ES4-classes), with the `catalog/packages/` layout
established at `../../catalog/packages/README.md`:

| Class | Operations | Laws (propositions) |
|---|---|---|
| `Eq`/`DecEq` | `eq` | reflexive, symmetric, transitive (decidable equality) |
| `Ord` | `cmp`, `≤` | total order |
| `Semigroup`/`Monoid` | `<>`, `empty` | associativity, unit |
| `Functor`/`Applicative`/`Monad` | `map`, `pure`, `>>=` | functor/monad laws |
| `Foldable`/`Traversable` | `fold`, `traverse` | the fold/naturality laws |
| `Num`/`Integral`/`Fractional` | arithmetic | ring/field-ish laws (per type) |

Instances for the built-in/prelude types are provided **with their law proofs**,
so `fold` over a `Monoid` can be reasoned about, and a generic verified
algorithm may *assume the laws hold* (they are **proved, not postulated** — the
discipline that carries into every package build).

The **constructor-class tranche** — `Semigroup`/`Monoid` (value-level algebra)
and `Functor`/`Foldable` (the first classes over a type constructor
`f : Type → Type`) — is pinned in **`55-lawful-functors.md`** (CAT-1,
`catalog/packages/Core/Classes/`), the reusable template the **effectful
tranche** —
`Applicative`/`Monad`/`Traversable` — extends in **`56-effectful-classes.md`**
(CAT-2, same package): the deep chain **wires** superclass fields (`56 §2`), and
`Traversable.traverse` is the first effect-row-polymorphic `proc` (SURF-1,
`../30-surface/36 §1.5`).

## 3. Collections — packages over built-in/prelude carriers

`List`, `Option`, and `Result` are **prelude** types (named by primitive
signatures, `30 §4`); their combinators are packages. In particular,
`List`'s `map`/`filter`/`fold`/`range` operations remain imported package
content even though its canonical family and constructors are always present.
`Array` is a **built-in audited runtime type** (`../30-surface/37 §3.2`,
`30 §6`: `declare_primitive` OpaqueType, item-2 audited); its internal
persistent-tree, copying, and sharing strategy is private (`41 §2`).
**`Map`/`Set`, by contrast, are proved package trees over `Ord k` — out of
`trusted_base()`, not primitives** (`52-map.md`; OQ-A retires the earlier
opaque `Map`/`Set` primitive, `30 §6`). This catalog provides `Array`'s
combinators and hosts the proved `Map` module (`52-map.md`). The **verified
building blocks** — a `sort` returning
`{ ys | is_sorted ys ∧ Perm ys xs }`, the proved `Map` — are the canonical
demonstrations of the thesis; the sort's predicates
`is_sorted`/`Perm` are **definitions** the prover unfolds (`../30-surface/37 §6`,
ES1), never postulates.

The **collection laws** (length/membership/decomposition + the verified `sort`
above) and the agent-facing **view** abstraction — the Layer-1 unit for
looking at data a different way (projection/lens, refinement, representation,
indexed, quotient-respecting, obligation-producing) — are pinned in
**`57-collections-and-views.md`** (CAT-3). `Perm` is `Ω`-native
**count/multiset-equality**, not a raw inductive (the `Ord.total` soundness
move, `57 §3.1`); the view is a plain `Σ`-record (concrete flavors ship now,
the polymorphic family gated on a bounded multi-param-`class` extension). The
family is named **`view`** (operator's call) — SURF-1's retirement of the `view`
keyword frees the word, and `view` is the standard term for a read projection.

The **Layer-2 keyed-collection laws** — `delete`,
`union`/`intersection`/`difference` (a **combining-function** `union`,
subsuming both biases), `keys`/`values` coherence — plus the **set algebra**
(`∪`/`∩`/`∖`, stated **membership-extensionally**, never `Equal (Set K)`:
same-key-set trees
differ in shape) and the **relations frontier** (composition, converse, the
reflexive/symmetric/transitive predicates, transitive closure) are pinned in
**`58-maps-sets-relations.md`** (CAT-4). `delete` is **rebuild-via-`from_list`**
(reusing the landed `preserves_ordered`); a relation is `Map K (Set K)`
adjacency; and the transitive closure is **bounded-reachability `IsTrue`** —
`reachable_plus` wraps `reachable_within` at fuel `size (dom r)`. Under the
lawful-order and well-formed-adjacency premises in `58 §7`, it denotes positive
`R⁺`, never reflexive `R*` or a raw proof-relevant inductive. The `Axiom`-free
D0 `leq_nat` floor, D1–D2 general proofs, D3 projection/ascending proofs, and
D4 relation-operation and predicate definitions are landed. D4's general
membership proofs and concrete predicate discriminators are not. The public
`size`/`dom`/`reachable_within`/`reachable_plus` computation is landed and
executing; the general relation-law tranche, including compose/converse
membership, concrete predicate discriminators, and closure
faithfulness/saturation, remains separately deferred.

The **persistent priority queue** is pinned in the dedicated
**`58a-priority-queues.md`** contract. It exposes exactly one abstract,
comparator-bound carrier plus `empty`/`insert`/`find_min`/`pop_min`/`merge`,
with `Ord k` applying only to priorities and no order or equality requirement
on payloads. Entry multiplicity, optional minimum extraction, persistence,
non-stable equal-priority ties, abstract recursive validity, and a structural
right-spine cost account are normative. `CAT-PRIORITY-QUEUE` is the named
complete computational build; its general validity, conservation, and
extract-min proofs remain separately deferred to
`CAT-PRIORITY-QUEUE-LAWS`. This dedicated chapter deliberately does not inherit
chapter 57's all-laws-land-together posture and freezes no private leftist node
layout.

The **`Membership` provider class behind `∈`** is pinned in the dedicated
**`58b-membership.md`** contract. It is a unary structure class over the
container with the query type as an associated field, so it lands one universe
up at `Type ℓ → Type (suc ℓ)` — the class former of `33 §5.2` operating, not a
new kind of declaration. Because instance resolution is canonical per outermost
head, the standard providers are **nominal, witness-bound views** rather than
raw heads, and installing `Membership Tree` is forbidden: key, set and
relation-edge membership are three meanings over one head. The comparator
travels in the view value, so an implicit resolver choosing a fresh `Ord` at the
use site is non-conforming. `Bool` is primary and the `Ω` view is read off it,
never eliminated from. The shared observational relation `same_members` is a
**definition**, not a law — a standalone over a dictionary, not a class field,
carrying no obligation and discharged by nothing. The whole of
the nonvacuous content is the per-provider adapter-fidelity obligation, so the
chapter records that the class's shared law layer is **empty** rather than
manufacturing content for it. The standard `∈`
binding, its fixity and the use-site completion policy are fixed with the
standard operator bindings, not here.

The **Layer-3 parsing/syntax/diagnostics contract** — source artifacts as byte
identity, half-open byte spans, total parser result values, small package-owned
grammars, parser/printer and formatter laws, and diagnostic primary/secondary
span validity — is pinned in **`59-parsing-syntax-diagnostics.md`** (CAT-5).
It is an ordinary `catalog/packages/Capability/Parsing/` catalog package: no compiler parser
rewrite, no compiler-internal AST as public API, no full Ken syntax reflection,
and no `.ken.md` implementation work. A derived input such as a blanked
`.ken.md` compiled view is only an offset-preserving view of the original
source artifact; the original artifact owns diagnostic identity.

The **length-indexed vector** `Vec (A : Type) : Nat → Type` — a **fresh
dependent inductive family** (not a built-in/prelude carrier), the canonical
dependent-types totality showcase — is pinned in
**`60-length-indexed-vectors.md`** (DS-5). The length index rules the empty case
out *at the type level*, so `head : Vec A (Suc n) → A` is total with no
`Option`; the family, `vnil`/`vcons`, and `head` are landed (the acceptance
suite `explicit_data_elaboration.rs`). `tail`, `zip`, `lookup`, and
bounded `Fin` use now check in the catalog Vector module from CAT-VEC
(`665059cfd`). DS-5b landed; the former DS-5c gate is retired because
the catalog capability is present, not because a DS-5c package landed.
`Vector.ken.md` contains no `Axiom`; `Vec` is an ordinary inductive
with a real eliminator, not a new trusted-base entry.

The **formal-languages contract** is pinned in
**`61-formal-languages.md`**. Section 1 specifies an ordinary `Dfa q a`
with a total transition, a start state, and Boolean acceptance over any
state carrier or alphabet. Its `run`/`accepts`, complement, general
Boolean-combining product and intersection/union obey seven checked laws.
Section 2 specifies `Finite q` as an explicit Ω-membership certificate
and `Algorithm.FormalLanguages.Reachability` as a complete finite-state
decision with four checked laws. Section 3 specifies a relation-valued
`Nfa q a`, Ω path acceptance, finite-mask determinization with checked
language equality in both directions, and decidable emptiness through
section 2. The ordinary `Core.Logic.And` and `unit_finite`/`bool_finite`
values are generic catalog dependencies, not new trusted rules. Neither
`DecEq q` nor `DecEq a` is needed in sections 1–3; no complexity promise
is made. Section 4 defines `Regex a`, its independent Ω language predicate,
finite repetition evidence, a derivative matcher using an explicit
`DecEq a`, and six checked soundness/completeness laws. It depends on shared
list concatenation/all-elements tools and Bool-truth lemmas in their owning
packages, with no automaton conversion in this contract. Specifying §4
does not claim its catalog build has landed. Section 5 specifies Dfa
language equivalence via a finite disagreement product and minimisation
by canonical representatives **within `q`**, with eight checked laws.
Neither decision requires `DecEq q`; only the named-state reachability
and stronger state-count constructions remain deferred. Section 6 adds a
bounded cursor element list and a streaming DFA runner in
`Capability.Parsing.Lexer`. Its two bridge laws compare the runner with
`run` over `cursor_elements` for arbitrary `CursorOps`; unfolding a
successful peek needs `CursorLaws` and uses its advance-progress law.
The byte-argument cursor supplies a lawful instance, but no cross-instance
location relation or generic byte-concatenation theorem is claimed.
Specifying §§5–6 does not claim their catalog builds have landed.

The **Layer-3 graphs and dependency contract** is pinned in
**`62-graphs.md`** §1 (SPEC-GRAPHS-DEPENDENCY-CONTRACT), §2
(SPEC-GRAPHS-CYCLE-REFUTES-ORDER), §3
(SPEC-GRAPHS-WALK-COMPOSITION-ORDER), and §4
(SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC), and §5
(SPEC-GRAPHS-REACHABILITY-ALONG-WALK). A graph has Boolean directed adjacency
over any carrier; explicit `Finite q` permits decidable predicate
reachability through §61 §2's existing Dfa fixed point, with a checked
`Walk` rather than a stuttering raw word. `DecEq q` enters for a complete,
duplicate-free topological list or a genuine cycle; the §1 catalog
package is present on the base. §2 adds three checked generic laws:
`before` is irreflexive and transitive for any list, and a cycle refutes
every topological order by its forward-edge field alone. Section 3
specifies `walk_append` by recursion on the second walk and the checked
`ordered_walk_forward` law for a walk with a first edge; the private
`walk_before` remains unexported. The §2 seed's blocks were exercised
only as scratch clients. Section 3's catalog exports and package-owned
seed test are present, but this candidate does not execute them.
Section 4 specifies generic left-unit and associativity laws for
`walk_append`. Its catalog exports and package-owned seed test are on
main; the latter is not official conformance execution or landed
trust. Section 5 specifies a checked `graph_reachable_along_walk`
law: reachability at a walk's endpoint carries back to its start.
The Architect's isolated D0 checked the generic proof on `454adf947`
with no new `Axiom` and identical scratch trusted GlobalIds
(114 → 114). The §5 export is absent; the separately authored closed
seed does not establish shipped §5 execution, official conformance,
CI, publication, landing or landed trust. No second fixed point is
added.

The **exact finite-support statistics contract** is pinned in
**`63-statistics.md`** §§1–4 (SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT;
SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT;
SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT;
SPEC-STATISTICS-EVENT-MASS-INCLUSION-EXCLUSION). `Algorithm.Statistics.Exact`
stores natural-weight atoms with checked positive total, without
`Finite q` or `DecEq q`; its nine §1 laws govern natural numerators.
`Algorithm.Statistics.Empirical` reads an existing `NonEmpty q`, keeping
repeated observations as distinct unit-weight entries under seven §2
laws. Section 3 adds value-typed probability, expectation, and
empirical mean to those same modules, with twelve further checked-law
obligations and no §3 CAT delivery claim. Section 4 adds a public
intersection event and three proved natural-weight laws for overlapping
events, event implication, and ordered scores; no §4 seed or CI result
is claimed at the AC-1 stage. `pure`/`bind`, uniform enumeration,
approximate statistics, and `Float` results remain deferred.

The separate **non-negative exact rational contract** is pinned in
**`64-rational.md`** (SPEC-NUMERIC-RATIONAL-CONTRACT).
`Data.Numeric.Rational` stores a natural numerator and the predecessor
of a positive denominator, with cross-product value equivalence and
Boolean order. Its checked arithmetic, relation/respect laws, and
positive-factor Nat prerequisites add no quotient, gcd import,
primitive, or trusted-base entry. Equivalent fractions may have
different carrier spellings. The carrier's catalog package is
landed; its contract does **not** itself supply the §63 §3 adapters.

The **dimension-safe exact linear-algebra contract** is pinned in
**`65-linear-algebra.md`** §§1–6
(SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT;
SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS;
SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY;
SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY;
SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC;
SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY).
`Algorithm.LinearAlgebra.NatMatrix` reuses `Data.Vector.Vector.Vec`
with row-major `Vec (Vec Nat n) m` matrices. Its seven initial natural
operations and all ten §1–3 checked laws are landed; neither §2 law
awaits `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`. The existing seed
includes closed §3 compatibility cases, with a catalog acceptance
source target. Section 4 specifies row-wise public `matrix_add`,
left and right matrix-product distribution, matrix-vector distribution
in the matrix argument, and transposition of a sum. Its four generic laws and
operation are catalog exports with checked bodies and a package-owned
seed test; Architect's historical scratch D0 had no census verdict
and did not establish shipped execution. Section 5 specifies public
commutativity and associativity of both sums for every dimension.
Architect D0 checked four generic proofs, lifting the matrix laws
row-wise from vector laws, with no new `Axiom` and unchanged scratch
trust identities (114 → 114). The §5 exports and package-owned seed
test are now on main; that test is not official conformance execution
or landed trust. Section 6 adds premise-free vector-argument
additivity and homogeneity for `matrix_vector`, including zero
dimensions. Architect D0 checked both generic proofs with no new
`Axiom` and unchanged scratch trust identities (109 → 109). The §6
catalog exports are absent; its authored closed seed and D0 do not
establish shipped-seed execution, official conformance, CI,
publication, landing or landed trust. No rational or `Float` tier is
claimed.

The **natural-coefficient symbolic-algebra contract** is pinned in
**`66-symbolic-algebra.md`** §§1–7 (SPEC-SYMBOLIC-ALGEBRA-CONTRACT;
SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY;
SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION;
SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION;
SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE;
SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION;
SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-INDEPENDENT-VARIABLE). The §1
`Algorithm.SymbolicAlgebra.NatPolynomial` catalog package and seed
are landed: `Expr` has natural constants, `Nat`-indexed variables,
addition and multiplication under a total environment. Ordered
distributed `Monomial` lists support reification and normalization
with five checked laws but are not canonical for semantic equality.
§2 specifies a separate recursive Horner `Poly` with collapsed
constant-headed zero tails, `canonicalize`, and the Boolean `poly_eq`
decision. Five additional generic checked laws give canonicalization
soundness/completeness and both directions of the decision, including
refusal of a `False` answer under universally equal evaluation. The
§2 catalog extension and seed are present on main; D0 trust evidence
alone does not establish shipped seed-harness execution or CI. Section
3 specifies public `compose_env`, one-pass `substitute`, evaluation
under the composed environment, and forward-only `poly_eq`
preservation. Its catalog exports and seed are present on main; the
D0 alone does not establish shipped seed-harness execution or CI.
Section 4 specifies public `compose_subst` with the order `t` then
`s`, structural `Equal Expr` composition, and constructor-identity
substitution. Both laws checked in Architect scratch D0 with trust
identity sets 114→114; §4 catalog exports and seed are present on main.
That D0 alone does not establish shipped seed execution, CI or landed
trust. Section 5
specifies structural `deriv` and two generic checked laws: universal
evaluation equality preserves derivative evaluation, and a `True`
`poly_eq` result remains `True` after differentiation. The Architect's
§5 D0 checked the Taylor-bump and remainder-extraction proof route
with no new `Axiom` and trust identities 114→114. The §5 seed is at
`conformance/stdlib/symbolic-algebra/seed-nat-polynomial-derivative.md`;
§5 catalog exports and authored seed are now present on main;
earlier D0 and seed authorship alone do not establish their official
conformance execution, CI or landed trust. Section 6 specifies one
public Transparent checked law, `deriv_commute v w e env`, relating
the **evaluations** of the two derivative orders at every total
environment, not their `Expr` structure. `poly_eq_complete` gives a
`True` decision as a client consequence without a new exported
corollary. The only new proof helper remains private. Architect D0
checked the generic proof with no new `Axiom` and unchanged scratch
trusted identities 114 → 114. The §6 export and package-owned seed
test are on main; neither establishes official conformance or landed
trust. Section 7 specifies the public `deriv_independent` law: if
changing every variable except `v` leaves the value of `e` unchanged,
its derivative in `v` evaluates to zero at the given environment.
The premise is semantic, even when `v` occurs in a zero-valued product.
Architect D0 checked the generic proof on `9e1d6e9cd` with no new
`Axiom` and unchanged scratch trusted identities 114 → 114. This
candidate adds the checked catalog export and a same-input accepted
and refused seed; author-local checks do not establish official
conformance, CI, publication, landing or landed trust. A sorted-list
normal form and signed/field arithmetic remain deferred.

The **exact natural-geometry contract** is pinned in
**`67-geometry.md`** §§1–7 (SPEC-GEOMETRY-EXACT-CONTRACT;
SPEC-GEOMETRY-MANHATTAN-METRIC; SPEC-GEOMETRY-BOX-CONTAINMENT;
SPEC-GEOMETRY-BOX-AREA; SPEC-GEOMETRY-BOX-INSIDE-ORDER;
SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY;
SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY).
The §1 `Algorithm.Geometry.NatBox` module, closed seed and required
public Nat Order bounds have landed. Two-coordinate `Nat` points and closed
axis-aligned boxes (including inverted-corner empty boxes) have
point membership, intersection, hull and Manhattan distance with
six §1 checked laws. §2's two additional generic metric laws,
their catalog code and seed are present on main. The zero-distance
law combines with §1's `manhattan_self` without a further public
converse. §3 specifies a corner-wise Boolean `inside` test and seven
checked D0 laws: soundness, reflexivity, intersection and hull bounds,
and a converse requiring a point witness. The §3 conformance seed is
authored and present at
`conformance/stdlib/geometry/seed-box-containment.md`. The §3
`inside` and seven law exports are present in `NatBox` on main since
`e3e0cda6c`. Exact-seed shipped-harness execution and CI are
unmeasured. The 114 → 114 trust result and zero-residual census
are D0 scratch measurements; they do not establish a landed trust
measurement.
Section 4 specifies public `area` as the product of two inclusive
natural-coordinate spans, with zero area when either axis is inverted.
Three generic laws state containment monotonicity without an outer-
validity premise and zero area for each inverted axis. Intersection
area bounds follow from monotonicity and the public §3.2 corner
laws; no extra area-intersection theorem is exported. The §4 catalog
exports and seed are now present on main, with a package-owned test
executing its seed fences and measuring unchanged trust; Architect's
older scratch D0 was not itself shipped-seed execution. Section 5
adds unconditional public transitivity, greatest-intersection and
least-hull laws for corner-wise `inside`, including inverted boxes.
Architect checked them in scratch with unchanged 114 → 114 trusted
identities; §5 exports, seed and package-owned test are now on main.
The package test does not establish official conformance execution or
landed trust. The per-axis order helpers stay private. Section 6 adds
public `comm for min` and `comm for max` selectors in Nat Order and
structural `intersect_comm` and `hull_comm` in NatBox, including
inverted boxes. Unlike rational interval endpoint swaps, Nat corners
support structural `Equal Box`. Architect D0 checked all four generic
proofs without a new `Axiom`, at equal scratch trust identities
114 → 114. The §6 catalog exports and package-owned seed test are present on
main; the test does not establish official conformance or landed trust.
Section 7 specifies public `assoc for min` and `assoc for max` selectors
and unconditional structural `intersect_assoc` and `hull_assoc` laws.
Architect's isolated D0 checked the four generic proofs on `372a6a1a6`
with no new `Axiom` and unchanged scratch trust identities 114 → 114.
The §7 catalog exports are not landed on this base; its authored closed
seed and D0 do not establish shipped §7 execution, official conformance,
CI, publication, landing or landed trust. The existing box-congruence
helpers remain private.
Signed-coordinate geometry, Euclidean and floating-point distance,
other dimensions, canonical empty boxes and point-set equality remain
outside scope.

The **exact rational-interval contract** is pinned in
**`68-numerical.md`** §§1–5
(SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT;
SPEC-NUMERICAL-INTERVAL-HULL;
SPEC-NUMERICAL-INCLUSION-ISOTONICITY;
SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY;
SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER;
SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE §6). The §1–2
`Algorithm.Numeric.RationalInterval` operations, ten checked laws,
seeds and required Rational monotonicity/min-max exports are present
on main. Non-negative `Rational` endpoints admit inclusive membership
and inverted-bounds empty intervals; point, addition and multiplication
use corresponding endpoints, while hull/intersection use exact min/max
bounds. Section 3 specifies public endpoint-wise `interval_inside`,
soundness, reflexivity, addition/multiplication isotonicity, four
hull/intersection inclusions, and a converse only with a member
witness. Its nine generic laws and the Boolean operation are present in the
catalog package with checked bodies. The authored seed at
`conformance/stdlib/numerical/seed-interval-inclusion.md` runs in targeted
package acceptance, including its two refused clients. Exact trusted-base
identity sets remain equal at 114 → 114 on the candidate; the earlier D0
scratch result by itself did not establish execution, CI or a landed trust
delta. Inverted empty intervals need not
pass the endpoint test despite vacuous member-set inclusion. Section 4
specifies four checked inclusions: each side of the
left- and right-oriented distributivity pair lies inside the other
by endpoint value, without structural `Equal Interval` or interval
commutativity. The §4 exports and seed are now present on main,
with a targeted package test; this is not official conformance
execution or a landed trust measurement. Section 5 adds five
generic obligations: transitivity of endpoint-wise inclusion and
swapped-operand inclusion for addition, multiplication, hull and
intersection. Their converses follow by swapping the theorem's
arguments, without structural `Equal Interval` or extra public
laws. Architect checked five generic proofs in scratch with no
new `Axiom` and unchanged scratch trust identities (114 → 114).
The §5 catalog exports and a package-owned seed test are on main;
that test is not official conformance execution, CI or landed trust.
Section 6 specifies four premise-free laws for mutual endpoint-value
inclusion under both addition and multiplication regroupings, in
either nesting direction, even for inverted intervals. Closed
regroupings coincide structurally by `Refl`; open regroupings are
not definitionally convertible, and generic structural equality
would need an out-of-scope Rational export. Architect's isolated
D0 checked the four generic proofs without a new `Axiom` and with
scratch trusted identities unchanged (114 → 114). The §6 exports
are absent on this base; its authored closed seed is not shipped
execution, official conformance, CI, publication, landing or landed
trust. Signed arithmetic, division, `Float`, rounding, width and
structural hull/intersection algebra remain deferred.

The **finite event-trace contract** is pinned in
**`69-time-and-events.md`** §§1–6 (SPEC-TIME-EVENT-TRACE-CONTRACT;
SPEC-TIME-WINDOW-MEMBERSHIP; SPEC-TIME-WINDOW-MERGE-DISTRIBUTION;
SPEC-TIME-MERGE-MEMBERSHIP; SPEC-TIME-WINDOW-COMPOSITION;
SPEC-TIME-MERGE-ASSOCIATIVITY).
`Data.Time.EventTrace` uses ordinary `Event p` values in plain
`List (Event p)` traces, non-decreasing `Nat` times, half-open
windows and left-biased merging at ties. Its five §1–2 checked laws,
two generic filter prerequisites and two existing seeds are present
on main. Section 3 specifies the D0-checked `trace_window_merge`
structural list equality **when both inputs are time-ordered**; one
ordered input does not suffice. The count corollary under those same
premises adds no public law. The checked §3 catalog export is present in
`Data.Time.EventTrace`; package acceptance runs the authored seed at
`conformance/stdlib/time-and-events/seed-window-merge-distribution.md`.
CI and a landed trust measurement are not established by this status.
Section 4 specifies three checked, premise-free `list_elem` laws:
merge-member soundness and separate left/right member preservation.
Their catalog exports and conformance seed are now present on main;
a targeted package test checks the corrected seed, not official
conformance or a landed trust measurement. A private one-way
window-of-merge membership consequence adds no public law.
Section 5 specifies one unconditional structural equality for nested
windows at intersected `max`/`min` bounds and two empty-merge unit
laws. The catalog promotes §3's original private
`trace_merge_nil_right` proof without duplicating it. Seven new
window-composition proof tools stay private. Architect's isolated
D0 checked the three public proofs with unchanged scratch trusted
identities 114 → 114; §5 catalog exports and a targeted package-owned
seed test are now present on main. This does not establish official
conformance, CI or a landed trust measurement. Section 6
specifies premise-free structural associativity of `trace_merge` on
arbitrary lists, including unordered inputs. The candidate carries
the catalog law with seven private helpers and a closed seed that
separates an outer-operand swap by a cross-input time-4 tie. Architect
D0 checked the generic proof with no new `Axiom` and equal scratch
trusted identities (114 → 114); candidate-local checks do not claim
official conformance, CI, publication, landing or landed trust. Clock
effects, durations, causality, infinite streams, premise-free
structural window/merge distribution and k-way merge remain deferred.

The **finite candidate-selection contract** is pinned in
**`72-optimization.md`** §1 (SPEC-OPTIMIZATION-SEARCH-CONTRACT;
SPEC-OPTIMIZATION-ARGMIN-OPTION §1.4;
SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE §1.5) and §3
(SPEC-OPTIMIZATION-ARGMAX; SPEC-OPTIMIZATION-ARGMAX-OPTION §3.4).
The landed `Algorithm.Optimization.Argmin` selects the earliest minimum
or greatest `Nat`-cost member of `NonEmpty a` with no `DecEq a`; the
seven `argmin` and seven `argmax` laws and both seeds are present on
main. Section 1.4's `argmin_option` and four laws are also landed.
Section 1.5 specifies the public `argmin_cost_order_invariant`:
a transform preserving and reflecting every strict `Nat` cost
comparison leaves the selected candidate unchanged, ties included.
Architect's isolated D0 checked its generic proof with no new
`Axiom` and equal scratch trusted identities 114 → 114. This
spec-and-seed candidate does not deliver the catalog export;
local seed checks are not official conformance execution, CI,
publication, landing or landed trust. Section 3.4 specifies
`argmax_option` on a possibly empty `List a`:
`Nil` yields `None`, and `Cons` yields `Some` of `argmax` over
`nonempty_cons`. Four laws give the two equations and direct-to-`List`
membership and greatest-cost consequences for an observed `Some`.
Architect's D0 checked these laws at 114→114 trusted identities. The
§3.4 catalog exports, seed and package-owned seed test are now on
main; its checks are distinct from official conformance execution.
Earlier D0 alone established no CI or landed trust measurement. Continuous or
implicit search, heuristics, approximation, arbitrary-order or
multi-objective costs, top-k and complexity claims remain deferred.

The **finite bounded-search contract** is pinned in
**`72-optimization.md`** §2 (SPEC-OPTIMIZATION-BOUNDED-SEARCH;
SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY §2.4).
`Algorithm.Optimization.BoundedSearch.find_below` takes a Boolean
predicate over `Nat` and a finite bound, returning `Some` with the
least satisfying index below it or `None` when none exists. Three
Architect-D0-checked generic laws give soundness, leastness and
completeness. No search direction, predicate-call count, recursion
shape or complexity guarantee is normative. The §2 catalog module
and seed are present on main; its D0 scratch checks alone establish
neither shipped-seed execution, CI nor a landed trust measurement.
Section 2.4 adds one public `find_below_stable` law: if the result is
`Some n` and `leq_nat bound larger = True`, that **same** `Some n`
remains at the larger bound. `leq_suc_not_leq_same` stays private;
there is no `None`-stability law, since a larger range can contain a
first hit. Architect's D0 checked the generic proof with no new
`Axiom` and equal scratch trusted identities 114 → 114. The §2.4
catalog export, seed and package-owned test are now on main; the
test and earlier D0 do not establish official conformance or a
landed trust measurement. The `List`-input Option
selector is specified in §1.4, not §2.

## 4. I/O, effects, serialization

- Effect interfaces (`../30-surface/36`):
  `IO`/`FS`/`Net`/`Clock`/`Console`/`Rand` — the **boundary primitive** is
  built-in (`30 §3`); the typed effect surface over it is package Ken.
- Serialization (`../30-surface/38 §1`): **`71-serialization.md`** §1
  (SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT) pins a pure
  `Data.Serialization.JsonCodec` over the full existing six-constructor
  `Json` carrier. Opaque `String`/`Int` payload tokens and a structural
  decoder support four theorems checked in Architect D0: decode
  after encode, trailing-input refusal, empty-input refusal and
  stray-array-close refusal. Objects preserve member order and duplicate
  keys; duplicate-key rejection is deferred. The JSON character/byte
  and UTF-8 layer, generic `Encode`/`Decode` derivation and Merkle hashing
  remain separate. §1 specifies a prospective package, not landed
  catalog code, a shipped conformance seed, CI or a trusted-base delta.
- A `Stream`/`Iterator` type for lazy iteration (productivity per
  OQ-coinduction) — a package.

## 5. Tooling-facing (T-stream)

The **`ken` command's stable diagnostic-code contract** is pinned in
**`70-diagnostic-codes.md`** §1 (SPEC-DIAGNOSTIC-CODE-CONTRACT).
It governs controlled batch CLI failures rather than a Ken catalog package:
codes have exact ASCII grammar and carrier-selected C/E/K/X/N/H domains;
the one append-only ledger owns their assignments; §1.2 pins seven
native cause-family meanings. Each coded failure emits a code-bearing
line to stderr and exits 1; an admitted `ken run` program's outcome
has no tool code. Multiple one-code lines are possible. CLI JSON,
warnings, and elaboration-recovery ordering remain deferred; this
contract does not claim the renderer or black-box seed has landed.

The catalog also seeds the tooling Ken needs to be usable (strategy WS-T): a
test/property framework (`../30-surface/` doc-tests + property tests, T3), and
the pedagogy/reference corpus (T4) — important because a new language has
near-zero training priors, so honest, runnable docs *are* the seed corpus.

## 6. Optional / research packages (not core)

Off to the side, never required by the core language: the **Leech/Golay/Co₀**
facilities in their three separate roles (`../40-runtime/44 §4`, OQ-6); the
**coalgebraic** layer (Store-comonad cells, process coalgebras, profunctor wires
— strategy WS-R); linear/affine types; delimited continuations. These are
harvested back as ordinary packages only if they earn it.

## 7. What WS-L must deliver here — the dissolved L8, as packages

The standard-package catalog (this file) delivered as **in-repo packages under
`catalog/packages/`** (ES4): the lawful classes (with **proved** laws), the
collection combinators + verified building blocks, the effect/IO/serialization
interfaces,
and the test/doc tooling seed — **each with its built-in-derivation path
stated** (the ES1 discipline; a missing path is a hidden-built-in spec bug).
Acceptance contributes to **G6** (a realistic verified component uses the
packages) and **G7** (the agent loop has enough library to work with).
Conformance: `../../conformance/stdlib/` — law proofs for the instances and the
verified `sort`/`Map` building blocks; and the derivation-path table
(`../../conformance/surface/taxonomy/`, ES1) that proves each entry is genuinely
package, not a hidden built-in.
