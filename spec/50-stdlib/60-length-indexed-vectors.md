# Length-indexed vectors — `Vec`

> Status: **normative** for the `Vec` family, its constructors, and the total
> `head` showcase — the family-declaration + single-scrutinee dependent-`match`
> mechanism they rest on is **landed** on the surface elaborator (the
> acceptance suite `crates/ken-elaborator/tests/explicit_data_elaboration.rs`,
> 14 tests, exercises the identical indexed-family + total-`head` mechanism).
> `tail` and the `Fin` bounded-index family's **declaration** are also **landed**
> (§3, §6): **`DS-5b`** delivered the dependent-`match` refinement (single-
> scrutinee constructor-injectivity + single-level sibling convoy + goal
> refinement, `34 §3.2.1`), so `tail` now builds. `zip` (the full two-vector
> recursive step) and `lookup` (`Fin`-indexed) check in the catalog
> Vector module from CAT-VEC (`665059cfd`). The former `DS-5c` gate
> is retired because that capability is present, not because a DS-5c
> package landed (§6). `Vec` is an ordinary inductive family with a
> real eliminator; `Vector.ken.md` contains no `Axiom`, and its named
> operations and laws have checked bodies (§6, §7). DS-5.

Length-indexed vectors are the canonical dependent-types showcase: the length
lives *in the type* as an **index**, so an operation that is partial on lists —
`head` on the empty list — becomes **total** on `Vec`, because the type rules
the empty case out before the term is ever written. This chapter specifies the
family and the operations that make that showcase concrete. The
catalog Vector module now checks `zip` and `lookup` as well as `tail`;
§6 records the former DS-5c gate as retired by checked catalog
capability, not by a DS-5c delivery.

## 1. The family — `Vec (A : Type) : Nat → Type`

`Vec` is an **indexed inductive family**: `A` is a **parameter** (one fixed type
of elements, uniform across constructors) and the length is a **`Nat` index**
that the constructors **refine** (each constructor targets a *specific* length,
not a uniform one). It is `14-inductive.md §2`'s canonical indexed example, and
`Nat` is the landed Peano inductive `data Nat = Zero | Suc Nat` (`§2` here uses
`Zero` / `Suc` throughout).

```
data Vec (A : Type) : Nat → Type where
  vnil  :                       Vec A Zero
  vcons : (n : Nat) → A → Vec A n → Vec A (Suc n)
```

- **`A` is a parameter, `Nat` is the index.** The distinction is load-bearing
  (`14 §2.1`): a parameter is fixed in every constructor's result (`Vec A _`),
  while the index *varies* per constructor — `vnil` targets `Vec A Zero`,
  `vcons` targets `Vec A (Suc n)`. A constructor that changed the parameter, or
  whose result head was not `Vec A …`, or that carried the wrong index arity, is
  **rejected** at declaration (`data.rs::validate_ctor_result_target`; the
  acceptance suite pins each rejection — wrong family head, changed parameter,
  too-few / too-many indices).
- **Strict positivity holds** (`14 §2`/`§8.2`): `Vec` occurs only in strictly
  positive position in `vcons`'s recursive argument `Vec A n`, so the family is
  admitted with a real dependent eliminator `elim_Vec` (`14 §3`).
- **Level.** For `A : Type ℓ`, `Vec A n : Type ℓ` — the family sits at the
  element type's level; the `Nat` index contributes no level (it is a value
  index, not a type parameter). Predicative, non-cumulative (`12`).

**Landed.** The result-index-refining declaration form elaborates today: the
acceptance suite declares the identical mechanism under the illustrative names
`data Vector (A : Type) : Nat → Type where { EmptyVector : Vector A Zero ;
ConsVector : (n : Nat) → A → Vector A n → Vector A (Suc n) }` and checks that the
kernel admits it as an inductive family with the refined constructor targets.
The choice of surface name `Vec` / `vnil` / `vcons` here is the canonical one
(matching `14 §2`); the mechanism is name-agnostic.

## 2. The showcase — total `head` (landed)

The empty vector has length `Zero`; a vector of length `Suc n` is provably
non-empty. So `head` restricted to `Vec A (Suc n)` is **total** — no `Option`,
no partiality, no run-time emptiness check:

```
head : (A : Type) → (n : Nat) → Vec A (Suc n) → A
head A n v = match v { vcons m x xs ⇒ x }
```

The single-arm `match` writes **only** the `vcons` arm and **omits** `vnil` —
and this is exhaustive, not partial, by `34-data-match §4.3`:

- At the scrutinee's index `Suc n`, `vcons` is **type-possible** (its target
  `Vec A (Suc _)` unifies) ⇒ its arm is **required**.
- `vnil` is **index-impossible**: its target index `Zero` cannot unify with the
  scrutinee's `Suc n` (`Zero ≢ Suc n` by constructor disjointness / index
  discrimination, `15` / `16`). So the elaborator **synthesizes** `vnil`'s
  eliminator method by **absurdity** — the refuted index equation yields a proof
  of the empty index constraint, from which the method body is `Empty`-eliminated
  (`elab.rs`: the synthesized method is a `Term::Absurd` over the index premises
  computed by `method_index_premises`). The kernel still receives a **total**
  `elim_Vec` (`14 §3`), so omitting `vnil` is sound **by construction**.

The refinement is genuinely load-bearing, not decoration: `head` on an
**un-refined** scrutinee `v : Vec A n` (arbitrary length) correctly **rejects**
as non-exhaustive — there, `vnil` *is* type-possible, so its arm is required and
its omission is a real error. Totality is bought by the index, and only by the
index.

**Landed.** The acceptance suite elaborates exactly this showcase
(`fn vectorHead (A : Type) (n : Nat) (v : Vector A (Suc n)) : A = match v
{ ConsVector m x xs ⇒ x }`) and confirms both directions — the refined `head`
accepts, the un-refined one rejects non-exhaustive. This is the landed anchor
for the whole chapter: single-scrutinee dependent `match` with automatic motive
recovery (`34 §3.2`) + index-impossible auto-discharge (`34 §4.3`) is the
mechanism, and it is real.

## 3. Bounds — the `Fin` index family

A safe positional accessor needs an index that is **provably in range**. Ken
states this by construction with a bounded-index family `Fin` — `Fin n` is the
type of naturals strictly below `n`, so a `Fin n` **is** a witnessed in-bounds
index and no side-proof accompanies it:

```
data Fin : Nat → Type where
  fzero : (n : Nat) →         Fin (Suc n)
  fsuc  : (n : Nat) → Fin n → Fin (Suc n)
```

- `Fin Zero` is **uninhabited** (both constructors target `Fin (Suc _)`, never
  `Fin Zero`) — exactly the statement "there is no index into an empty vector,"
  captured at the type level.
- `Fin` is the *same indexed-family mechanism* as `Vec` (§1), so its
  **declaration elaborates today** on the landed surface (the result-index
  refinement is the landed capability). Prefer this one clean bounded-index
  story over proliferating an ad-hoc `Nat` index carrying a separate
  `IsTrue (lt i n)` proof (`reflect-don't-extend`, `subsume-don't-proliferate`):
  `Fin` states totality *by construction* — `lookup : Vec A n → Fin n → A` is
  total with **no** side-proof — which is the cleaner spec.

Choosing `Fin` over `Nat + proof` did not itself solve the former
§6 elaboration gap: `lookup` still recurses into the vector's tail.
That dependent-`match` enhancement has since landed, and `lookup`
checks in the catalog Vector module. The discipline choice remains
about the cleanest totality *statement*, not bypassing a checker.

## 4. The total API

Collecting the operations this chapter specifies, with each one's
buildability state (per the Architect's ground-truthed DS-5 ruling):

| Operation | Type | State |
|---|---|---|
| `vnil` / `vcons` | family constructors (§1) | **landed** |
| `head` | `(A : Type) → (n : Nat) → Vec A (Suc n) → A` | **landed** (§2) |
| `Fin` decl | `Nat → Type` (§3) | **landed** (decl) |
| `tail` | `(A : Type) → (n : Nat) → Vec A (Suc n) → Vec A n` | **landed** (§6; DS-5b) |
| `zip` | `(A B : Type) → (n : Nat) → Vec A n → Vec B n → Vec (Pair A B) n` | **checked in catalog Vector** (§6; CAT-VEC `665059cfd`; former DS-5c gate retired) |
| `lookup` | `(A : Type) → (n : Nat) → Vec A n → Fin n → A` | **checked in catalog Vector** (§6; CAT-VEC `665059cfd`; former DS-5c gate retired) |

`Pair` in `zip`'s codomain is the canonical non-dependent compiler-origin floor
family (`Pair : Type → Type → Type`, `../30-surface/34 §"Canonical
non-dependent pair floor family"`). The catalog Vector module now
checks the Pair-valued `zip` without a live Pair or DS-5c gate.
The length index `n` guarantees the two input vectors **align**, so
`zip` is total and loses no elements — unlike the `List` `zip`, which
must truncate. That guarantee is the second showcase; landing the
previous gates changed availability, not this indexed contract.

## 5. Laws (scoped to the totality showcase)

Per the frame, this first chapter is scoped to the **totality showcase**; the
equational theory is named and deferred explicitly rather than under-specified
silently.

- **In-chapter (definitional).** `head` computes by the eliminator's ι-rule:
  `head A n (vcons n x xs) ≡ x` (`14 §3`). This is not a postulated law but the
  eliminator's computation rule, holding definitionally; it is the observable
  content of §2's totality.
- **Deferred (named, to a follow-on / the package WP).** The following are real
  and worth pinning but are **not** in this chapter's scope; each is deferred
  explicitly, not omitted silently:
  - `tail` / `lookup` computation (`tail A n (vcons n x xs) ≡ xs`;
    `lookup … (fsuc …) ≡ lookup … `) — deferred by this chapter's
    scope; `tail`'s is provable now (`tail` landed, §6); `lookup` and
    its private computation proof have checked in the catalog Vector
    module. This chapter still defers adding those equations to its
    own law list.
  - `zip` / `map` **naturality** and the `zip`-`unzip` round-trip.
  - The **length / `to_list` bridge** (`Vec A n → List A` and `length ∘ to_list ≡ n`)
    relating the indexed family to the unindexed `List`.

## 6. Dependent-`match` refinement — checked catalog operations

`tail`, `zip`, and `lookup` check in the catalog Vector module.
**`DS-5b`** delivered enough dependent-`match` refinement for `tail`;
the two-vector recursive step and Fin-indexed lookup check in CAT-VEC
(`665059cfd`). Their former DS-5c gate is retired because these
capabilities are present, not because a DS-5c package landed.

**What `DS-5b` landed (`34 §3.2.1`).** A dependent `match` recovers its motive
automatically over the scrutinee's own index (`34 §3.2`); `DS-5b` extended that
automatic path to carry each branch's **constructor index equation** into the
local context, in three forms (acceptance suite
`crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs`, all
zero-`trusted_base()`-delta):

1. **Constructor injectivity for a peeled recursive field** — `tail`'s `vcons`
   arm yields `xs : Vec A m`, and `vcons`'s injective equation `Suc m ≡ Suc n`
   now re-types it to the required `Vec A n`. **`tail` builds.**
2. **Single-level sibling convoy** — matching one scrutinee refines an *outer*
   binder sharing the same index, so a nested match on that sibling is licensed.
3. **Base-case goal refinement** — a base-case arm's checking goal is re-typed
   by the branch equation.

The refinement is load-bearing, not permissive: the **AC8** over-refinement case
(a goal demanding an equation the branch does *not* license) **stays a genuine
kernel rejection**, never a silent accept.

**Why the former DS-5c gate is retired.** At the DS-5b stage,
two operations needed more than its single-level convoy:

- **`zip`** — the full **two-vector recursive step**. The recursive `zip xs ys`
  reuses *both* vectors' peeled tails; `DS-5b`'s convoy handled **one**
  sibling through **one** nested match with **no further reuse**
  (`34 §3.2.1`). The recursive reuse hit its then-unresolved distinction
  between a genuine outer parameter and a field the enclosing `match`
  already bound.
- **`lookup`** — the `Fin`-indexed recursion into the vector's tail
  needed the same follow-on capability.

CAT-VEC (`665059cfd`) supplies checked `zip` and `lookup` in the
catalog Vector module. The former DS-5c gate is retired because the
capability is present, not because a DS-5c package was delivered.
`Vector.ken.md` contains no `Axiom`. The former gap was elaboration
*completeness*, not soundness.
There remains **no manual-motive escape hatch** — `match` carries only
a scrutinee and arms (`ast.rs::Expr::EMatch` has no motive field;
`parser.rs::parse_match_expr` builds `EMatch { scrut, arms, span }`).
The automatic path now supports these two operations; that says
nothing about every more general nested proposition goal.

## 7. Trust, provenance, and clean-room

- **No new trusted form.** `Vec` and `Fin` are ordinary inductive
  families with real eliminators; `head`'s totality is **kernel-proved**
  by the total `elim_Vec` the index-impossible auto-discharge
  synthesizes (§2), not postulated. The catalog Vector module checks
  `zip` and `lookup` using dependent `match`; `Vector.ken.md` contains
  no `Axiom` and adds no `trusted_base()` entry. The former DS-5c gap
  was elaboration *completeness*, never a soundness hole papered over
  by a postulate. This is the `List` / `Nat` posture
  (`reflect-don't-extend`).
- **Landed evidence.** The family-declaration + total-`head` mechanism is the
  acceptance suite `crates/ken-elaborator/tests/explicit_data_elaboration.rs`
  (14 tests); the spec rules it rests on are `14 §2`/`§3` (indexed families +
  dependent eliminator), `34 §3.2` (automatic motive recovery), and `34 §4.3`
  (index-impossible auto-discharge — the totality mechanism).
- **Clean-room.** Length-indexed vectors and the total-`head` construction are
  standard dependent-type-theory material; this chapter is authored from the
  landed Ken surface + `14`/`34` + first principles, in Ken's own words, with no
  reference source copied.
