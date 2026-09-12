# Maps, sets, and relations laws (CAT-4, Layer 2)

> Status: **DRAFT v0** (CAT-4). This chapter is the **contract** for Layer-2
> keyed-collection laws (`delete`, `union`/`intersection`/`difference`,
> `keys`/`values`), the **set algebra** that rides `Set = Map Unit`, and the
> **relations frontier** (composition, converse, the property predicates, and
> transitive closure). It **builds on the landed Map capstone** (`54`/`52`,
> `catalog/packages/Data/Collections/Map.ken.md`) — laws 1/2/3/5 +
> `to_list`-ordered are proved there and are **reused, not re-derived** (frame
> §2 pin 2). It inherits `55`/`57`'s lawful-class template: landed proof
> artifacts are `Ω` propositions with zero `Axiom` and zero `trusted_base()`
> delta, built by the convoy induction + `trans`/`cong` route-around grammar
> (`54 §2`/`§3`) with the **sharpened per-branch `Proved`-vs-`Refl`** endpoint
> rule (`57 §1 pt 3`). Deferred proof obligations are marked explicitly below.
> **CAT-2/CAT-3-independent** — value-level, no `Monad`/collection-view needed.
> **Kernel-untouched, outer-ring.** Four design forks are resolved here
> (Architect, `main@7169300f`): **A** `union` takes a **combining function**;
> **B** transitive closure is **bounded-reachability `IsTrue`** (`Ω`-native, the
> `Perm` move — never a raw multi-ctor `data … : Ω`); **C** a relation is
> `Map K (Set K)` (adjacency), with an explicit **landed / deferred** scope
> split; **D** `delete` is **rebuild-via-`from_list`**. Two enclave sub-rulings:
> set laws are stated **membership-extensionally** (never `Equal (Set K)`), and
> **`leq_nat` + its four order results are the D0 carrier prerequisite**
> (`Nat`, not the `Axiom`-holed `Ord Int`/`Ord Char`). The D0 order results,
> D1–D2 operations and general proofs, and D3 projection/ascending operations
> and proofs are implemented in `catalog/packages/Data/Collections/Map.ken.md`.
> D4 currently lands transparent `compose`/`converse`, property-predicate
> definitions, and the public `size`/`dom`/`reachable_within`/`reachable_plus`
> closure computation. All four closure functions execute; the general
> compose/converse membership proofs, the concrete predicate discriminators,
> and closure faithfulness/saturation laws remain separately deferred.

## 1. What CAT-4 inherits (`54`/`52` and `55`/`57`)

Every law below is authored to five inherited points, so nothing is
re-litigated:

1. **`Ω` law fields, no proof-relevant inductive** (`55 §4`, `16 §6`). A lookup
   or membership equation is `Equal T u v : Ω` (or `IsTrue b := Equal Bool b
   True`), a direct value equation. The one construct that is *proof-relevant as
   a relation* — the **transitive closure** — is **not** a raw multi-ctor
   `data TC : … : Ω` (that is the inadmissible proof-relevant inductive, `16
   §1.4`+§1.1, the exact `Perm` hazard); it is pushed into a **decidable
   bounded-reachability `Bool`** and wrapped in `IsTrue` (`§7`, Fork B) — the
   same move CAT-3 made for `Perm`.
2. **Proved by the convoy idiom + the Gap-A route-around** (`54 §2`/`§3`): a law
   over the `Tree` carrier uses ordinary recursive `fn`, `proof`, or `theorem`
   declarations and reflects each stuck `leq k k'` through `bool_dichotomy` (a
   Gap-B dependent match on a `Bool` **variable**)
   and transports once with `J` (`53 §3`). The IH is an **ordinary
   self-recursive call on the subtree** (`54 §2.1` — the kernel's IH-slot binder
   is dead/surface-unreferenceable), never a synthesized `ih_l`/`ih_r`.
3. **Per-branch `Proved`-vs-`Refl`**, never uniform (`55 §3.2`/`57 §1 pt 3`,
   sharpened by CAT-3): a branch closes with `Proved` when both endpoints reduce to
   the **same fully-collapsing** head — a **nullary** ctor or one whose
   components all collapse — going to `Top` (K7); and with `Refl` when they
   reduce to a **neutral**, *including a non-nullary head with any neutral
   component* (it stays `Eq`-shaped, and `Proved : Top` would be ill-typed there).
   The landed capstone bases are the template: `ordered_empty`/`lookup_empty_is_none
   → Proved` (operation reduced into a collapsing `Equal Bool True True`); a
   `from_list_acc Nil acc` base is a **passthrough** (the accumulator's own
   `Ordered`, not `Proved`).
4. **Reuse, don't re-derive, the Map capstone** (frame §2 pin 2). `delete`/
   `union`/`intersection`/`difference` build **on** the landed
   `insert`/`lookup`/
   `to_list`/`fold`/`from_list`/`member` + `preserves_ordered` (law 1) +
   `lookup_assoc_agree` (law 5) + `assoc`. **`Set a = Map a Unit`** (landed
   `set_insert`/`set_member`/`set_to_list`, spelled `Tree a Unit` directly — no
   `Set` type alias is landed); the set-algebra laws ride that identity.
5. **Surface spelling.** The SURF-1 declaration migration is landed. Current
   value operations in the Map package use `fn`; proof artifacts use the
   declaration form selected by their result and proof style (`proof` or
   `theorem`). The retired `view` keyword is not accepted source syntax. Each
   example below follows its corresponding landed declaration rather than
   applying one keyword indiscriminately. (`Nat`'s successor is `Suc`, not
   `Succ` — `prelude.rs`, `data Nat = Zero | Suc Nat`.)

## 2. D0 — the `leq_nat` carrier prerequisite (the vacuity guard)

CAT-4 uses the landed `Nat` carrier with **≥ 3 distinct keys under an
`Axiom`-free order**. This is the CAT-3 `List Bool` lesson, sharpened: a proved
accept-arm is only load-bearing if the carrier's order laws are **genuinely
inhabited**.

- `Ord Int`/`Ord Char` on `main` are **`Axiom`-holed** — their order laws are
  postulated, so a proof that *uses* them is vacuous and the intended flip
  degenerates to reject-vs-reject (green-vs-green — no discrimination).
- `Ord Bool` is `Axiom`-free but has **only 2 keys** — it **cannot** exhibit a
  three-node transitivity witness (`a → b → c`: is `a → c` in the closure?),
  which the relation/closure discriminators require.
- **`Nat` is the carrier.** The Map package now defines `leq_nat` and its four
  order results as ordinary total Ken with zero `Axiom` and no kernel change,
  by structural recursion on `Nat` (`data Nat = Zero | Suc Nat`, prelude).
  This landed D0 basis remains the carrier-vacuity guard for every relation
  discriminator.

```
fn leq_nat (m : Nat) (n : Nat) : Bool =
  match m {
    Zero  => True ;
    Suc m2 => match n { Zero => False ; Suc n2 => leq_nat m2 n2 }
  }
```

The four order results are the **unbundled bare-parameter dictionary** the
capstone already threads (`52 §2` Architect-ruled encoding). Their exact public
identities are three `proof` selectors for `leq_nat` plus the ordinary function
`total_leq_nat`; callers supply them directly as separate parameters. Proof
shapes are all `Nat`-structural and comparison-driven only through `leq_nat`'s
own recursion:

- **`proof refl for leq_nat`** has type
  `(x : Nat) → Equal Bool (leq_nat x x) True`. It inducts on `x`: the `Zero`
  base reduces to `Equal Bool True True`, which K7 collapses to `Top` and closes
  with `Proved`; the `Suc m` step reduces to the predecessor goal and closes
  with `proof refl for leq_nat m`.
- **`proof trans for leq_nat`** has type
  `(x : Nat) → (y : Nat) → (z : Nat) →
  Equal Bool (leq_nat x y) True →
  Equal Bool (leq_nat y z) True → Equal Bool (leq_nat x z) True`. It inducts
  on all three arguments. The `x = Zero` base is live and closes by `Proved`;
  the `Suc/Suc/Suc` arm calls `proof trans for leq_nat` on the predecessors;
  every remaining `Zero`/`Suc` mismatch discharges a false premise with
  `absurd`.
- **`proof antisym for leq_nat`** has type
  `(x : Nat) → (y : Nat) → Equal Bool (leq_nat x y) True →
  Equal Bool (leq_nat y x) True → Equal Nat x y`. It inducts on both arguments:
  `Zero/Zero` closes with `Proved`; `Suc/Suc` applies `cong` to the predecessor
  proof; each mixed arm discharges a false premise with `absurd`.
- **`total_leq_nat`** has type
  `(x y : Nat) → Or (Equal Bool (leq_nat x y) True)
  (Equal Bool (leq_nat y x) True)`. It inducts on both arguments: `Zero` on
  either side selects the corresponding `Inl`/`Inr` with `Proved`, and
  `Suc/Suc` preserves the predecessor result's `Or` tag.

`proof antisym for leq_nat` is needed only for the `Distinct`-discharge
boundary (`54 §4`, ADR 0010-gated, out of scope). `delete`/`union`
invariant-preservation uses `proof trans for leq_nat` and `total_leq_nat`,
matching the landed law-1 parameters. Implementations may bind those parameters
to local names such as `reflLeq` and `transLeq`; such binders are not second
public globals.

The Boolean prerequisites are landed too. Canonical `bool_and` and its algebra
come from `Core.Classes.LawfulClasses`; the Map package defines its local
match-based `bool_not` and `cat4_bool_or` helpers. D1 uses `bool_and` for
`drop_key`'s decision (§3), and the set algebra uses `bool_and`/`bool_not` (§5).

Every named `Pair`/`pair_fst`/`pair_snd` use below refers to the landed canonical
compiler-origin transparent-Σ floor family specified by
`../30-surface/34 §"Canonical non-dependent pair floor family"`. The current
Map package uses those identities directly; no package import or ambient
fallback supplies them.

## 3. D1 — `delete` (rebuild-via-`from_list`, Fork D)

`delete`'s equal-key case has **no analog in `insert`**: `insert` overwrites in
place (one path, structure-preserving), but `delete` must **remove** the node
and merge its two subtrees. Fork D rules the **rebuild** route (reuse the
capstone wholesale; the structural-`glue` alternative re-derives a whole new
invariant apparatus — hypothetical local `glue`/`delete_min` helpers plus a
cross-subtree-bound transport with no analog in the landed corpus):

```
fn drop_key (k : Type) (v : Type) (leq : k -> k -> Bool) (key : k) (xs : List (Pair k v)) : List (Pair k v) =
  match xs {
    Nil => Nil (Pair k v) ;
    Cons e xs2 =>
      match order_equiv_key k leq key (pair_fst k v e) {
        True  => drop_key k v leq key xs2 ;                       -- drop ALL matches (filter)
        False => Cons (Pair k v) e (drop_key k v leq key xs2)
      }
  }

fn delete (k : Type) (v : Type) (leq : k -> k -> Bool) (key : k) (m : Tree k v) : Tree k v =
  from_list k v leq (drop_key k v leq key (to_list k v m))
```

- **`drop_key` is FILTER (remove **all** order-equivalent entries), not
  drop-first** (Fork D build-pin). The order-equivalence **decision** is
  **Bool-valued** —
  `order_equiv_key leq a b : Bool = bool_and (leq a b) (leq b a)`
  (§2's `bool_and`) — so `drop_key`'s `match … { True => … ; False => … }` has a
  `Bool` scrutinee, exactly as `insert`/`lookup` branch on `leq key k2 : Bool`.
  (This is the *decision*; the landed **Prop**-valued `order_equiv` in
  `catalog/packages/Data/Collections/Map.ken.md` is its `Ω` counterpart — used
  in the *laws*, never as an
  executable-`match` scrutinee.) `drop_key` is a plain `List` recursion
  (Gap-B-free, like `pair_keys`/`assoc`).
- **`delete` is NON-recursive** — a pipeline of landed structural ops
  (`to_list → drop_key → from_list`), so it carries **zero SCT obligation of its
  own** (one less thing to check than a self-recursive `glue`-`delete`).

### 3.1 `Ordered`-preservation

`delete` produces `from_list … (…)`, and `from_list` of **any** list is `Ordered`
by construction, so preservation needs one new lemma and no `delete`-specific
induction:

```
theorem from_list_preserves_ordered
  (k : Type) (v : Type) (leq : k -> k -> Bool)
  (transLeq : …) (total : …)
  (xs : List (Pair k v))
  : Ordered k v leq (from_list k v leq xs) = …          -- List-induction; each step = landed preserves_ordered
```

- **List-induction over `xs`** (the `from_list_acc` accumulator): base `Nil` is a
  **passthrough** to the accumulator's `Ordered` (the initial `Leaf`, i.e.
  `ordered_empty → Proved`); step inserts one entry, closed by the **landed**
  `preserves_ordered` (law 1) applied to the accumulator's `Ordered`. Nothing
  new about `insert` is proved — it is reused wholesale.
- `delete_preserves_ordered` is then `from_list_preserves_ordered … (drop_key …
  (to_list
  … m))` — immediate, `drop_key`/`to_list` are irrelevant to the conclusion (any
  list input suffices).

### 3.2 The two lookup laws

- **None-law (UNCONDITIONAL):** `lookup key (delete key m) ≡ None`. Because
  `drop_key` is **filter**, no entry order-equivalent to `key` survives into the
  rebuilt tree, so `lookup` finds nothing — **no `Ordered`/`Distinct`
  hypothesis** (a drop-first `drop_key` would let a duplicate survive if the
  input weren't distinct; filter closes that off structurally). Routes through
  the `from_list`/`assoc` dual: `lookup key (from_list xs) ≡ assoc key xs`, and
  `assoc key (drop_key key ys) ≡ None` (a `List`-level lemma: filtering out
  `key` leaves nothing for `assoc` to match).
- **Other-key law:** `Not (order_equiv_key leq k key) → lookup k (delete key m) ≡
  lookup k m`. This one **threads `Ordered`+`Distinct`** through the landed
  **law 5** (`lookup_assoc_agree` in
  `catalog/packages/Data/Collections/Map.ken.md` —
  `lookup k m ≡ assoc k (to_list m)` under `Ordered`+`Distinct`) + its
  `from_list` dual + a `drop_key`/`assoc`
  lemma (`k ≠ key → assoc k (drop_key key ys) ≡ assoc k ys` — dropping a
  *different* key doesn't disturb `k`'s first-match). All `List`-level, reusing
  the landed `assoc`.

## 4. D2 — `union` / `intersection` / `difference` (Fork A)

**Fork A: `union` takes a combining function** (subsume-don't-proliferate — the
combining fn subsumes both biases: left `= union (\x _. x)`, right `= union (\_
y. y)`; a biased-only op forces a second op the first time anyone merges
values). Orientation `f (from-a) (from-b)`, matching `unionWith`:

```
fn insert_with (k : Type) (v : Type) (leq : k -> k -> Bool) (f : v -> v -> v) (key : k) (val : v) (m : Tree k v) : Tree k v =
  …                                                    -- like `insert`, but on key-collision store `f val old`

fn union (k : Type) (v : Type) (leq : k -> k -> Bool) (f : v -> v -> v) (a : Tree k v) (b : Tree k v) : Tree k v =
  fold k v (Tree k v) (\key val acc. insert_with k v leq f key val acc) b a
```

`intersection`/`difference` are the same fold-into-a-fresh-accumulator shape
with a **membership test** against the other map (no combining fn needed — they
select keys, they don't merge values):

```
fn intersection … (a : Tree k v) (b : Tree k v) : Tree k v =
  fold k v (Tree k v) (\key val acc. match member k v leq key b { True => insert k v leq key val acc ; False => acc }) (empty k v) a

fn difference … (a : Tree k v) (b : Tree k v) : Tree k v =
  fold k v (Tree k v) (\key val acc. match member k v leq key b { True => acc ; False => insert k v leq key val acc }) (empty k v) a
```

- **Lookup characterization (the D2 map law, Fork A):** `lookup k (union f a b)`
  is the 2×2 table — both-`None → None`; `(Some x, None) → Some x`; `(None, Some
  y) → Some y`; `(Some x, Some y) → Some (f x y)`. `intersection`: `Some` iff
  `k ∈ a ∧ k ∈ b` (value from `a`); `difference`: `Some` iff `k ∈ a ∧ k ∉ b`.
- **`Ordered`-preservation is `f`-independent** and rides **one** shared
  shared fold-preserves-`Ordered` lemma: each of the three ops is a fold whose
  step
  either `insert`s (landed `preserves_ordered`) into an `Ordered` accumulator or
  returns it unchanged; the fold therefore preserves `Ordered` (base = the
  initial `Ordered` accumulator, `b` or `empty`). `f` touches **only values**;
  `Ordered` is about **keys**, so it never enters this proof.
- **Map `union` is NOT commutative in general** (unless `f` is) — the trees
  differ in shape *and* in a collided value's argument order. So **maps get only
  the lookup characterization + `Ordered`-preservation, never a commutativity
  law.** Commutativity/associativity/idempotence are **Set-only** (`§5`). Do not
  over-claim.

## 5. Set algebra (`Set = Map Unit`)

Set ops are the map ops at `v := Unit` (`set_union s t := union … (\_ _. MkUnit)
s t`, etc. — at `Unit` every combining policy coincides, so Fork A's choice is a
**no-op** here). The set membership algebra uses the landed `bool_or`
(`catalog/packages/Core/Classes/LawfulClasses.ken.md`) together with the
`bool_and`/`bool_not` introduced as D0 prerequisites (`§2`).

**The set laws are stated MEMBERSHIP-EXTENSIONALLY** (enclave sub-ruling —
load-bearing soundness):

```
theorem set_union_comm_law
  (k : Type) (leq : k -> k -> Bool)
  (reflLeq : (x : k) -> Equal Bool (leq x x) True)
  (transLeq : …)
  (x : k) (s : Tree k Unit) (t : Tree k Unit)
  : Ordered k Unit leq s -> Distinct k Unit leq s
    -> Ordered k Unit leq t -> Distinct k Unit leq t
    -> Equal Bool
         (set_member k leq x (set_union k leq s t))
         (set_member k leq x (set_union k leq t s))
```

— **never** `Equal (Set K) (set_union s t) (set_union t s)`. **Tree-`Equal` set
laws are FALSE, not merely unprovable:** `union a b` and `union b a` are
built by
`fold`+`insert` and produce **shape-different trees with the same key-set**.
Extensional (`∀x. set_member x lhs ≡ set_member x rhs`) is the only sound
formulation — and it is exactly what makes the laws **corollaries** rather than
fresh `Tree` inductions:

- A **membership-homomorphism** lemma reduces each op to Bool algebra:
  `set_member x (set_union s t) ≡ bool_or (set_member x s) (set_member x t)`
  (from the D2 lookup characterization at `v = Unit`), and likewise
  `bool_and` for `∩`, `bool_and _ (bool_not _)` for `∖`.
- Then **commutativity / associativity / idempotence / identity** of `∪`/`∩`
  follow from the same properties of `bool_or`/`bool_and` — a **finite 2×2**
  discharged via the landed `bool_dichotomy` (no induction over the tree at all).
- **`difference` membership:**
  `set_member x (set_difference s t) ≡`
  `bool_and (set_member x s) (bool_not (set_member x t))`, the same shape.

## 6. D3 — `keys` / `values` coherence

- **`keys`** is landed and reuses `pair_keys` + `to_list` (it is literally
  `set_to_list` generalized off `Unit`): `keys k v m := pair_keys k v (to_list k
  v m)`.
- **`values`** uses the landed mirror projection of `pair_keys`, with `pair_snd`
  for `pair_fst`: `pair_vals k v xs := match xs { Nil => Nil v ;
  Cons p xs2 => Cons v (pair_snd k v p) (pair_vals k v xs2) }`, then `values k v m
  := pair_vals k v (to_list k v m)`.
- **Coherence with ordering:** `keys` are **ascending** under `Ordered m` —
  `is_sorted leq (keys m)` — off the landed **`to_list_ordered`** (which gives
  `is_sorted (pair_leq leq) (to_list m)`) + a small `pair_keys`-preserves-sortedness
  lemma (`pair_leq` compares first components, i.e. keys, so projecting keeps the
  order). `values` carry **no** ordering claim (values are unordered).
- **Coherence with `to_list`:** `keys`/`values` are the two **componentwise
  projections** of `to_list` — same length, positionally aligned (`keys` reads
  `pair_fst` where `values` reads `pair_snd` of the same entry). Stated
  structurally over the shared `to_list m` traversal (no `length`/`zip` primitive
  needed — those are CAT-3/List-level and out of this chapter).

## 7. D4 — Relations (the frontier; Fork C)

**Fork C-rep: a relation is `Map K (Set K)` (adjacency) — `Tree K (Tree K
Unit)`**, a plain instantiation of the landed `Tree` carrier at `v := Set K`, so
every landed parametric op works at that `v` with **zero new machinery**. Rides
**`Ord K` only** (outer map + inner set both keyed by `K`). *Not* `Set (Pair K
K)`: the landed `pair_leq` compares **first components only** (a partial,
non-total order) so it cannot even key a `Set (Pair K K)`, and a proper pair-set
would force a lexicographic pair-comparator + its four order laws as a whole
extra bundle; and composition over `Set (Pair K K)` is a quadratic nested scan.

```
fn succ (k : Type) (leq : k -> k -> Bool) (x : k) (r : Tree k (Tree k Unit)) : Tree k Unit =
  match lookup k (Tree k Unit) leq x r { None => empty k Unit ; Some s => s }

fn rel_member (k : Type) (leq : …) (x : k) (y : k) (r : Tree k (Tree k Unit)) : Prop =
  IsTrue (set_member k leq y (succ k leq x r))          -- Ω-native membership
```

**C-scope — the landed / deferred split, stated explicitly (no silent
truncation):**

**LANDED DEFINITIONS AND OBSERVED SMOKE:**

- **`compose`** computes `R∘S` by folding over `succ x R` and unioning the
  corresponding `S`-images. **`converse`** folds each stored edge into the
  reversed adjacency map. The current `map_build_acceptance` relation-test
  source contains one positive observation for each definition: composition
  contains `1 → 3` when given `1 → 2` and `2 → 3`, and converse contains
  `2 → 1` when given `1 → 2`. The paired seed records the exact passing run and
  its source blobs.
- The property predicates are transparent `Π`-into-`Ω` definitions (`16 §1.1`,
  fine — properties are not proof-relevant): `is_reflexive`, `is_symmetric`,
  `is_transitive`, and `is_equivalence`. For example, `is_transitive r` is
  `(x : k) → (y : k) → (z : k) → rel_member x y r → rel_member y z r →
  rel_member x z r`.

**RELATION LAW AND CONFORMANCE RESIDUAL:**

- The normative membership characterizations remain
  `succ x (compose R S) = ⋃ { succ y S : y ∈ succ x R }` and
  `y ∈ succ x R ⇔ x ∈ succ y (converse R)`. No corresponding general proof is
  present in the landed D4 producer. The existing runtime smoke does not test
  an absent composed edge or the unreversed direction of converse; the paired
  positive/negative membership cases remain unexecuted conformance obligations.
- The concrete predicate discriminators remain obligations too. In particular,
  a non-transitive `Nat` relation (`a → b`, `b → c`, no `a → c`) must fail
  `is_transitive`, while its completion must inhabit it. The transparent
  predicate declaration and the suite's zero-delta census do not construct
  either proof-flip arm.
- These residuals belong to the separate general-relation-law follow-on, not to
  the landed computation below. They are not silently credited to the
  four-function closure build and do not weaken its contract.

**CONTRACT AND PUBLIC COMPUTATION LANDED** (the four functions are public and
executing; their faithfulness proof follows separately):

- **Transitive closure (Fork B): `reachable_plus` decides positive
  reachability.** The four public functions have these signatures:

  ```text
  size : (k : Type) → (v : Type) → Tree k v → Nat
  dom : (k : Type) → (v : Type) → Tree k v → Tree k Unit
  reachable_within : (k : Type) → (leq : k → k → Bool) → (fuel : Nat)
                     → (x : k) → (y : k)
                     → (r : Tree k (Tree k Unit)) → Bool
  reachable_plus : (k : Type) → (leq : k → k → Bool)
                   → (x : k) → (y : k)
                   → (r : Tree k (Tree k Unit)) → Prop
  ```

  `size` counts raw `Tree` nodes: `size (Leaf) = Zero` and
  `size (Node l _ _ r) = Suc (add (size l) (size r))`, using the landed
  `Data.Numeric.Nat.Arithmetic.add`. `dom` is exactly the outer key set. It
  replaces each node value by `MkUnit` while preserving the `Leaf`/`Node`
  shape, key, and subtrees. It does **not** add keys that occur only in successor
  sets, and it needs no comparator.

  **Semantic scope.** The path interpretation and the faithfulness/saturation
  statements below assume one lawful (reflexive, transitive, antisymmetric, and
  total; §2) key order used throughout: the outer relation tree is
  `Ordered k (Tree k Unit) leq`, and every successor tree stored
  in it is `Ordered k Unit leq` under the same comparator. These are premises of
  the correspondence laws, not extra function parameters or runtime checks.
  `size` and `dom` remain total on every raw tree, as do the
  `reachable_within` and `reachable_plus` equations. On arbitrary raw trees,
  however, no correspondence to transitive closure of `edge_R` is promised:
  `fold` visits every stored entry while `set_member` uses comparator-directed
  `lookup`, and those observations can disagree when a successor tree is not
  ordered.

- **Fuel recurrence.** Let
  `edge_R x y := set_member k leq y (succ k leq x r)`. Then:

  ```text
  reachable_within k leq Zero x y r = False

  reachable_within k leq (Suc n) x y r =
    cat4_bool_or
      (edge_R x y)
      (fold k Unit Bool
        (λz. λu. λseen.
          cat4_bool_or (reachable_within k leq n z y r) seen)
        False
        (succ k leq x r))
  ```

  Every recursive call receives exactly the predecessor `n`, including calls
  made by the `fold` step. No general bounded-iteration interface or second
  Boolean disjunction is introduced: the definition reuses the landed `fold`,
  `succ`, `set_member`, and `cat4_bool_or`.

- **Fuel is the maximum positive path length.** Fuel zero recognizes no path;
  fuel one recognizes exactly a direct edge; fuel two additionally recognizes
  a two-edge path. Equality of endpoints is not a zero-step success: `x = y`
  succeeds only when the relation contains a positive self-loop or cycle within
  the bound. Thus this computes `R⁺`, not reflexive-transitive `R*`. In
  particular, making `Zero` recognize a direct edge would shift every bound by
  one, while making `Zero` recognize `x = y` would compute the wrong relation.

- **The public predicate fixes its own bound:**

  ```text
  reachable_plus k leq x y r =
    IsTrue
      (reachable_within
        k
        leq
        (size k Unit (dom k (Tree k Unit) r))
        x
        y
        r)
  ```

  This is the **`Perm` move**: a decidable bounded `Bool` computation is wrapped
  in `IsTrue`, an `Ω`-native value-equation. It carries no path witness and
  introduces no raw multi-constructor `data TC : … : Ω` (which would be an
  inadmissible proof-relevant inductive, `16 §1.4`+§1.1).

- **Under those well-formedness assumptions, faithful, not an approximation:**
  choose a shortest positive walk from `x` to `y`. Its nonterminal vertices are
  distinct; otherwise a repeated subwalk could be removed while preserving
  positive reachability. For the resulting
  path `v₀ → v₁ → … → vₘ`, the edge count is `m`, and all `m` nonterminal
  vertices `v₀ … vₘ₋₁` are distinct outer keys because each has an outgoing
  edge. Therefore `m ≤ N` for `N := size (dom R)`; the terminal `vₘ` may be a
  target-only vertex absent from `dom R`. Bounded reachability at fuel `N`
  consequently equals full transitive closure (it is monotone in the fuel and
  saturates by that bound).
  The old `N−1` claim was false for `{a ↦ {b}}` with target-only sink `b`:
  `size (dom R) = 1` and the valid edge `a → b` has length one.

  Why bounded beats truncation: (i) `‖ Σpath. path_connects path x y ‖`, with
  `path_connects` explicitly schematic here, is `Ω`-sound
  but **non-computational** — no verdict-flipping conformance case can run
  against it; the bounded form **evaluates**; (ii) the downstream consumer is
  L14 model-check interop, which literally computes bounded reachability — the
  decidable encoding is the right semantic fit; (iii) it rides the **landed**
  `Map`/`Set` operations named above; truncation needs a `Σ`-of-paths inductive
  we do not have.

- **Why the laws remain deferred:** the landed public computation supplies
  `size`, `dom`, `reachable_within`, and `reachable_plus`. A separate
  general-relation-law follow-on supplies the missing compose/converse
  membership proofs, concrete property-predicate discriminators, and the
  closure **faithfulness/saturation laws**. The last of these requires the
  simple-path shortening and `N`-fuel saturation proof. **The computation is
  pinned here; none of those proofs is credited to it or truncated away.**
- **Seam:** relations feed L14 + Lane B; the adjacency rep is one design with
  those consumers. Do not over-build here.

## 8. Derivation paths and build sequencing

Everything bottoms out in landed built-ins plus the Map capstone. The current
package contains `leq_nat` and its four order results; the Boolean helpers; the
D1–D2 delete, keyed-merge, and set-algebra operations with their named proofs;
the D3 projections and their projection/ascending proofs; and the D4
`succ`/`rel_member`/`compose`/`converse` plus
property-predicate definitions. These ordinary `fn`/`proof`/`theorem`
declarations reuse the landed `Tree`, Map, Set, `IsTrue`, and Boolean basis and
have zero `Axiom` or trusted-base delta. D4's general membership proofs and
concrete predicate proof-flips are not present merely because the definitions
are.

The landed public computation contains
`size`/`dom`/`reachable_within`/`reachable_plus` together, so
`reachable_plus` visibly fixes its bound as `size (dom r)`. The paired
conformance seed executes all four functions across its eight closure cases.
The separate relation-law follow-on then proves compose/converse membership,
executes the property discriminators, and establishes closure
faithfulness/saturation.
Conformance lives at `../../conformance/stdlib/collections/`: its executing
closure cases use the landed lawful `Nat` order and well-formed outer and inner
trees, never the `Axiom`-holed `Ord Int`/`Ord Char`.

## 9. Acceptance

- **AC1 — Kernel-untouched.** No `crates/ken-kernel/` change; no new `Term`/
  `Decl`; no `declare_primitive`/`declare_postulate`; **no `Axiom`** anywhere.
  The landed public closure functions (`size`/`dom`/`reachable_within`/
  `reachable_plus`) are ordinary total Ken, execute in the outer ring, and are
  kernel-untouched, like the already-landed `leq_nat` carrier basis.
- **AC2 — Reuse, not re-derive.** `delete`/`union`/`intersection`/`difference`
  build on the landed `insert`/`lookup`/`to_list`/`fold`/`from_list`+
  `preserves_ordered`+law 5; `leq_nat` plus its four order results are
  `Axiom`-free.
- **AC3 — `Ordered`-preservation** for `delete` (via
  `from_list_preserves_ordered`) and `union`/`intersection`/`difference` (via
  one shared fold-preserves-`Ordered` lemma, `f`-independent).
- **AC4 — The characterizations:** D2 lookup 2×2 table; D1 None-law
  (unconditional) + other-key law (via law-5 roundtrip); D3 keys-ascending +
  keys/values projection coherence.
- **AC5 — Relation `Ω`-soundness and semantic scope.** Property predicates are
  `Π`-into-`Ω`; transitive closure is **bounded-reachability `IsTrue`**, never a
  raw `data … : Ω`. Correspondence to `R⁺` requires one lawful shared order and
  `Ordered` outer and every inner tree; arbitrary raw trees receive only the
  total equations. The computation/law deferral split is explicit.
- **AC6 — Set laws are membership-EXTENSIONAL** (`∀x. set_member x lhs ≡
  set_member x rhs`), **never** `Equal (Set K)`; discharged as `bool_or`/
  `bool_and` corollaries via `bool_dichotomy`, not fresh `Tree` inductions.
- **AC7 — Carrier vacuity guard.** Discriminators run on the **`Nat`** carrier
  with the real `leq_nat` dictionary (≥ 3 distinct keys for transitivity/
  closure), **never** the `Axiom`-holed `Ord Int`/`Ord Char`.
- **AC8 — Evidence boundary.** Transparent D4 definitions and the current
  positive compose/converse smoke checks are landed evidence only for those
  facts. They do not discharge the absent-edge membership controls, the general
  compose/converse membership proofs, the concrete property-predicate
  proof-flips, or closure faithfulness/saturation; those remain in the separate
  relation-law follow-on.
