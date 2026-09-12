# CAT-4 — Maps / Sets / Relations laws (Layer 2) — conformance seed

Format: `../../README.md`. These pin the **CAT-4 deliverable** against the
current contract in `spec/50-stdlib/58-maps-sets-relations.md`. CAT-4 **extends
the landed Map capstone** (`catalog/packages/Data/Collections/Map.ken.md`,
`spec/50-stdlib/54-map-verified-laws.md` — laws 1/2/3/5 + `to_list`-ordered
proved) with `delete`, the keyed-merge ops
(`union`/`intersection`/`difference`), `keys`/`values` coherence, the
Set-algebra laws (Set = Map-Unit), and the `Relation` frontier (properties
landed; closure computation next and general laws later). **Outer-ring,
kernel-untouched, zero `Axiom`, zero `trusted_base()` delta** — every landed law
is a real proof over the landed carriers, `subsume`/`reuse`-don't-re-derive.

**Grounded against `origin/main@1bd3a5667`** (`Map.ken.md`, `54`/`52`,
`16 §1.1`–`§1.4`/`§6`, `LawfulClasses.ken.md`, and `prelude.rs`; historical
fork rulings are provenance, not substitutes for the current producers):

- **Reuse corpus (`catalog/packages/Data/Collections/Map.ken.md`):**
  `Tree k v = Leaf | Node`; `insert`/`lookup`/`member`/`to_list`/`fold`/
  `from_list`/`from_list_acc`; `Ordered`/`all_keys` (`Ω` via
  `IsTrue b := Equal Bool b True` + derived `And`; the `Leaf` goal
  `Equal Bool True True` K7-collapses to `Top` and closes with `Proved`); the
  convoy idiom (`bool_dichotomy b : Or (Equal Bool b True)
  (Equal Bool b False)`, with `Or` a `Type`-sum built directly against
  `declare_inductive`); laws 1/2/3/5 + `to_list`-ordered
  (`preserves_ordered`, `lookup_found_after_insert`, `lookup_locality`,
  `lookup_assoc_agree` at the cited producer's line 5576,
  `lookup key m ≡ assoc key (to_list m)` under `Ordered`+`Distinct`, and
  `to_list_ordered`) + `all_keys_trans_below`/`all_keys_trans_above`,
  `is_sorted_append`, and the `trans`/`cong` "stop-one-step-short" transport
  bridges. **Zero `Axiom`, zero `trusted_base` delta throughout.** Permutation
  is the one Map law still deferred (proof-relevant, C5).
- **The D0–D4 producer has a split evidence boundary.** The current Map package
  contains `leq_nat`, `delete`, `union`, `intersection`, `difference`, `keys`,
  `values`, `compose`, `converse`, and the relation-predicate definitions. It
  contains the D0 order results, D1–D2 general proof corpus, and D3
  projection/ascending proofs. The D4 region contains transparent operation and
  predicate definitions, but no general
  compose/converse membership-law proof or concrete property-predicate proof
  witness. Those are retained below as conformance obligations, not inferred
  from declaration presence.
- **The named closure computation remains deferred:** `size`, `dom`,
  `reachable_within`, and `reachable_plus` are not part of the landed D0–D4
  producer. The structured closure rows below are explicitly blocked on
  `CAT-REL-TRANSITIVE-CLOSURE`; the faithfulness/saturation proof is a separate
  later follow-on. Neither deferral is evidence against those specifically
  inventoried landed definitions and proofs.

**Architect fork rulings (source of truth, `evt_55htg0ss8y1v6` +
`evt_3z7c592g37rtr`):**

- **Fork A — `union` = combining fn**
  `(V → V → V) → Map K V → Map K V → Map K V` (subsumes left/right bias; `f`
  takes `(from-a, from-b)`, Haskell `unionWith` orientation). `f` touches **only
  values** — `Ordered`-preservation is `f`-independent. **Map union is NOT
  commutative** in general — maps get only the **lookup characterization** +
  `Ordered`-preservation, **never** a map-commutativity law. Impl
  `union f a b := fold (\k v acc. insert_with f k v acc) b a`.
- **Fork B — transitive closure = BOUNDED/DECIDABLE positive reachability**,
  `Ω`-native `IsTrue`, not `‖·‖` truncation. `reachable_within Zero = False`;
  `reachable_within (Suc n) x y r` is the direct edge OR the Boolean fold of
  `reachable_within n z y r` over `z ∈ succ x r`. `reachable_plus` wraps that
  result at fuel `N := size (dom r)`. The equations remain total on raw trees.
  Under one lawful shared order and `Ordered` outer and inner trees, fuel is
  maximum positive path length and the result denotes `R⁺`, not reflexive `R*`.
  A shortest positive path then has at most `N` edges: its nonterminal vertices
  are distinct outer keys, while its terminal may be a target-only sink. **No
  raw multi-constructor `data … : Ω`** (proof-relevant and inadmissible,
  `16 §1.4`+§1.1). The decidable form reduces to a concrete `Bool` that the
  deferred cases below can make flip; a truncated closure cannot provide that
  execution boundary.
- **Fork C-rep — `Map K (Set K)` (adjacency), NOT `Set (Pair K K)`.** Rides
  `Ord K` only (`Set (Pair K K)` would force a total lexicographic
  pair-comparator + 4 pair-order-laws; the landed `pair_leq` compares first
  components only, non-total on pairs). Just `Tree K (Tree K Unit)` — a
  landed-`Tree` instantiation at `v := Set K`, zero new machinery. **C-scope:**
  compose/converse/`succ`/membership plus the property predicates are landed.
  The exact four-function closure computation is the next fast-follow; its
  faithfulness/saturation laws follow separately.
- **Fork D — `delete` = REBUILD-via-`from_list`.**
  `delete key m := from_list leq (drop_key key (to_list m))`, `drop_key` =
  **filter** (removes **ALL** matching entries, not drop-first). Non-recursive
  (zero own SCT). `Ordered`-preservation reuses landed `preserves_ordered`
  wholesale via one `from_list_preserves_ordered`. **None-law
  `lookup key (delete key m) ≡ None` is UNCONDITIONAL** (nothing with `key`
  survives to `from_list` — filter semantics); the **other-key** law threads
  `Ordered`+`Distinct` through the law-5 roundtrip.
- **Sub-ruling (1) — set laws MEMBERSHIP-EXTENSIONAL:**
  `(∀k. set_member k lhs ≡ set_member k rhs)`, **NEVER** `Equal (Set K) lhs rhs`.
  Tree-`Equal` set laws are **false** (fold+insert produces shape-different
  trees with the same key-set) — extensional is the **only sound** formulation,
  and is what makes the set laws corollaries of the map lookup-characterization
  + `bool_or`/`bool_and` algebra.
- **Sub-ruling (2) — carrier:** use the landed Axiom-free `leq_nat` plus its
  four order results as the D0 basis; proved discriminators key on
  **`Map Nat`/`Set Nat`/`Relation Nat`**, never `Map Int` (`Ord Int`/`Ord Char`
  are Axiom-holed, so the accept arm goes vacuous and the flip degenerates to
  reject-vs-reject).
  `Ord Bool` is Axiom-free but 2-key — too degenerate for a ≥3-key relation
  discriminator (a→b→c ⊬ a→c). The carrier-vacuity guard binds (the CAT-3
  `List Bool` lesson, one carrier up).

**Status — landed definitions and proofs are distinct from executing
conformance.** On exact candidate `af4e5eb726eae1a73d56e8390dd7985ee5133f99`,
with test blob `581dd65c494cbecb147fa563284c80f948bfab65` and Map blob
`60949c3d4b6cc98046789249a4fb51cc385d8165`, the targeted command
`scripts/ken-cargo test -p ken-elaborator --test map_build_acceptance` returned
`29 passed; 0 failed; 0 ignored`. This prose-only respin keeps both blobs
unchanged.

The suite contains five `cat4_*` rows. One checks the named D0–D4 globals for
transparent declarations and zero trusted-base delta. Four execute values:
delete removes the requested key; union observes the left-biased collision
orientation while intersection and difference observe one result each;
`keys`/`values` produce aligned lists; and relation smoke observes the present
composed edge `1 → 3` and present converse edge `2 → 1`. That relation smoke
does **not** execute either absent-edge control below and does not prove a
general membership law or inhabit a property predicate. The corresponding D4
rows therefore remain explicit unexecuted obligations. The closure rows retain
the separate computation-then-laws boundary and do not claim that their four
functions already execute.

---

Named `Pair` uses below use the landed compiler-origin transparent-Σ floor
family without a package import. The relation representation's negative
`Set (Pair K K)` discussion is comparative and grants no unrelated ambient
name.

## Scope — canonical shapes

This is a signature synopsis, not a compilable package: bodies are omitted.
Declaration keywords and public identifiers match the current producer; the
four closure signatures are the deferred contract.

```
-- D0 carrier prerequisite (Axiom-free, Nat inductive):
fn leq_nat (m : Nat) (n : Nat) : Bool
proof refl for leq_nat (x : Nat) : Equal Bool (leq_nat x x) True
proof trans for leq_nat (x : Nat) (y : Nat) (z : Nat)
  : Equal Bool (leq_nat x y) True
    -> Equal Bool (leq_nat y z) True -> Equal Bool (leq_nat x z) True
fn total_leq_nat (x : Nat) (y : Nat)
  : Or (Equal Bool (leq_nat x y) True) (Equal Bool (leq_nat y x) True)
proof antisym for leq_nat (x : Nat) (y : Nat)
  : Equal Bool (leq_nat x y) True
    -> Equal Bool (leq_nat y x) True -> Equal Nat x y

-- D1 delete (Fork D — rebuild, drop_key = filter):
fn drop_key (k : Type) (v : Type) (leq : k -> k -> Bool) (key : k)
            (xs : List (Pair k v)) : List (Pair k v)
fn delete (k : Type) (v : Type) (leq : k -> k -> Bool) (key : k)
          (m : Tree k v) : Tree k v
  = from_list k v leq (drop_key k v leq key (to_list k v m))

-- D2 map operations; union's f receives (from-a, from-b):
fn union (k : Type) (v : Type) (leq : k -> k -> Bool) (f : v -> v -> v)
         (a b : Tree k v) : Tree k v
fn intersection (k : Type) (v : Type) (leq : k -> k -> Bool)
                (a b : Tree k v) : Tree k v
fn difference (k : Type) (v : Type) (leq : k -> k -> Bool)
              (a b : Tree k v) : Tree k v
-- Maps get no unconditional commutativity law.

-- Set algebra is membership-extensional, never Equal (Tree k Unit):
fn set_union (k : Type) (leq : k -> k -> Bool)
             (s : Tree k Unit) (t : Tree k Unit) : Tree k Unit

-- D3 projections:
fn keys (k : Type) (v : Type) (m : Tree k v) : List k
fn values (k : Type) (v : Type) (m : Tree k v) : List v

-- D4 relations use Tree k (Tree k Unit); Relation is schematic shorthand only.
fn succ (k : Type) (leq : k -> k -> Bool) (x : k)
        (r : Tree k (Tree k Unit)) : Tree k Unit
fn rel_member (k : Type) (leq : k -> k -> Bool) (x : k) (y : k)
              (r : Tree k (Tree k Unit)) : Prop
  = IsTrue (set_member k leq y (succ k leq x r))
fn compose (k : Type) (leq : k -> k -> Bool)
           (r : Tree k (Tree k Unit)) (s : Tree k (Tree k Unit))
         : Tree k (Tree k Unit)
fn converse (k : Type) (leq : k -> k -> Bool)
            (r : Tree k (Tree k Unit)) : Tree k (Tree k Unit)
fn is_transitive (k : Type) (leq : k -> k -> Bool)
                 (r : Tree k (Tree k Unit)) : Prop
-- rel_member is already IsTrue(...), so premises are not re-wrapped.

-- Closure computation, BLOCKED-ON-CAT-REL-TRANSITIVE-CLOSURE:
fn size (k : Type) (v : Type) (t : Tree k v) : Nat
fn dom (k : Type) (v : Type) (t : Tree k v) : Tree k Unit
fn reachable_within (k : Type) (leq : k -> k -> Bool) (fuel : Nat)
                    (x : k) (y : k) (r : Tree k (Tree k Unit)) : Bool
fn reachable_plus (k : Type) (leq : k -> k -> Bool)
                  (x : k) (y : k) (r : Tree k (Tree k Unit)) : Prop
-- reachable_within Zero x y r = False
-- reachable_within (Suc n) x y r = direct edge OR
--   fold cat4_bool_or False (reachable_within n z y r for z in succ x r)
-- reachable_plus k leq x y r =
--   IsTrue (reachable_within k leq
--     (size k Unit (dom k (Tree k Unit) r)) x y r)
-- Positive R+, not reflexive R*, and not data TransClosure : ... : Omega.
```

---

## AC1 — kernel-untouched, outer-ring

### stdlib/collections/cat4-kernel-untouched-outer-ring (soundness)
- spec: `58 §9` AC1, `52 §1.1`/§9 (the Map-container AC1 precedent), `16 §1`.
- given: the landed D0–D2 proof corpus, D3 projection/ascending proofs, the D4
  definition producer, and, when built, the
  `size`/`dom`/`reachable_within`/`reachable_plus` fast-follow.
- expect: the relevant build diff under `crates/ken-kernel/` is **empty**; zero
  `trusted_base()` delta; no new `Term`/`Decl`. Every operation is an ordinary
  `fn`; every landed or later law is an ordinary `proof`/`theorem` over the
  existing `Tree` carrier and kernel machinery. Assert structurally from
  declaration forms, the empty kernel diff, and `trusted_base()` delta, not from
  prose.
- why: AC1's derived-not-primitive core — CAT-4 rides the outer ring entirely
  ([[enriching-opaque-former-kind-is-kernel-clean]] sibling: even the `Relation`
  former is a plain `Tree K (Set K)` instantiation, no kind extension).
  **`(soundness)`** — a kernel touch or a new `Decl` for any CAT-4 op would grow
  the TCB. Structural (empty kernel diff + admission-kind), not a value.
  (structural; kernel-untouched.)

---

## AC2 — reuse the landed capstone, zero re-derivation

### stdlib/collections/delete-union-reuse-landed-no-rederivation (soundness)
- spec: `58 §9` AC2 / `58 §1 pt 4`, `54 §5` (laws 1–5),
  `catalog/packages/Data/Collections/Map.ken.md`
  (`preserves_ordered`/`lookup_assoc_agree`/`fold`/`insert`).
- given: the landed `delete`/`union`/`intersection`/`difference` proof terms.
- expect: `Ordered`-preservation for **all four** routes through **one**
  `from_list_preserves_ordered`-shaped lemma reusing the landed
  **`preserves_ordered`** per-insert step (base `ordered_empty`), and the
  lookup-side delete/merge laws route through the landed **law 5
  (`lookup_assoc_agree`)** + its `from_list` dual — **no fresh two-tree
  simultaneous-descent induction, no re-proof of laws 1–5**. Grep-clean of
  `Axiom`/`declare_postulate` for every CAT-4 law. **The flip:** a build that
  re-derives a preservation lemma from scratch **and** `Axiom`-stubs a step is
  caught by the zero-delta cone walk (below); a faithful reuse adds zero delta.
- why: `§2 pin 2` (`subsume`/`reuse`-don't-re-derive), the WP's central
  directive. **`(soundness)`** — a re-derivation that smuggles an `Axiom` reads
  `proved`-by-default ([[untrusted-layer-backstop-hole-for-omissions]]); the
  delta net is the backstop. Structural (proof-term shape + delta membership).
  (soundness; structural reuse + zero-delta.)

### stdlib/collections/leqnat-d0-real-axiom-free-carrier (soundness)
- spec: `58 §2` sub-ruling 2, `51 §6` (Axiom-free order carriers),
  `../classes/seed-lawful-classes.md` (`Ord Bool` the only landed Axiom-free
  order carrier; `Ord Int`/`Ord Char` Axiom-holed).
- given: the landed D0 basis `leq_nat` plus `proof refl for leq_nat`,
  `proof trans for leq_nat`, `proof antisym for leq_nat`, and
  `total_leq_nat`.
- expect: all four order results are **real and kernel-checked** over inductive
  `Nat`: the three attached proofs recurse structurally and `total_leq_nat`
  returns a proof-relevant `Or` value. Each has zero `Axiom` and zero
  trusted-base delta. Unlike primitive `Int`, `Nat` supplies the eliminator that
  makes these results derivable. **The flip:** stub any result with `Axiom` →
  the cone walk reports a non-empty delta → rejected; the real result leaves
  the delta empty → accepted.
- why: the carrier prerequisite that makes the proved CAT-4 discriminators
  **non-vacuous** — see the standing carrier discriminator below.
  **`(soundness)`** — an `Axiom`-holed `leq_nat` would make every proved
  `Map Nat` law's accept-arm vacuous. Structural delta-flip. (soundness;
  structural zero-delta.)

---

## AC3 — invariant preservation (the convoy idiom over `Tree`)

### stdlib/collections/delete-preserves-ordered (soundness)
- spec: `58 §3.1` D1, `54 §5.1` (`preserves_ordered`), Fork D
  (`evt_3z7c592g37rtr`).
- given: `Ordered m → Ordered (delete key m)` as a proof term, over `Map Nat`
  (real `leq_nat`).
- expect: **accepts** — `delete = from_list ∘ drop_key ∘ to_list`, so preservation
  is `from_list_preserves_ordered` (plain `List` induction: each step = the landed
  per-insert `preserves_ordered`, base = `ordered_empty` `Leaf → Proved`; the
  `from_list_acc Nil acc → acc` base is a **passthrough of `acc`'s `Ordered`,
  not `Proved`** — per the sharpened `55 §3.2`/`57 §1 pt 3` endpoint rule,
  build-pinned). **Zero `Axiom`, zero delta.** **The flip:** a `delete` that
  emitted a non-`Ordered` tree (e.g. a glue that mis-orders the promoted node)
  could not discharge preservation → `Axiom`-stub → non-empty delta →
  **rejected**.
- why: AC3's invariant face, reusing the landed machinery wholesale (the reason
  Fork D chose rebuild). **`(soundness)`** — structural zero-delta + the
  per-branch base-witness pin (`Proved` on the
  `Leaf`/`Equal Bool True True` collapse, passthrough on `from_list_acc Nil`,
  `Refl`/`cong` on neutral steps — never uniform
  [[tt-vs-refl-endpoint-rule-for-inductive-equal-law-bases]]).
  (soundness; structural; endpoint tokens landed.)

### stdlib/collections/union-preserves-ordered-f-independent (soundness)
- spec: `58 §4` D2, Fork A (`f` touches only values).
- given: `Ordered a → Ordered b → Ordered (union f a b)`, over `Map Nat`.
- expect: **accepts, and the proof does NOT mention `f`** —
  `union f a b = fold (\k v acc. insert_with f k v acc) b a`, and `insert_with`'s
  **key** placement is `insert`'s, so `Ordered`-preservation is `f`-independent
  (`f` only combines **values** at a collision, never moves a key). One shared
  shared fold-preserves-`Ordered` lemma covers
  `union`/`intersection`/`difference`. **Mechanism-consistency check (my V2
  carry):** the three merge ops must agree on the shape of this shared lemma.
  **The flip:** a `union` whose merge mis-placed a key by `Ordered` fails
  preservation → `Axiom` → delta → reject.
- why: AC3 for the merge ops; pins `f`-independence structurally (guards a
  conformance over-claim that threads `f` into the invariant proof).
  **`(soundness)`** — structural (proof does not cite `f`) + zero-delta.
  (soundness; structural.)

---

## D1 — `delete` laws

### stdlib/collections/delete-none-law-unconditional
- spec: `58 §3.2` D1, Fork D (filter `drop_key`, unconditional None-law).
- given: `lookup key (delete key m) ≡ None`, over a **real inserted** `Map Nat`
  (computes) — driven through real `delete`/`lookup`, not hand-fed.
- expect: **reduces-to `None`** with **no `Ordered`/`Distinct` hypothesis** —
  `drop_key` filters out every entry matching `key`, so nothing with `key`
  survives to `from_list`, and `lookup` finds nothing. A `delete` that fails to
  remove the key (or removes a different key) yields `Some v` — the value-flip.
- why: D1's headline delete law, made executable (seed-map
  `insert-lookup-roundtrip` twin, delete side). **Drive the real producer:**
  build via real `insert`, delete via real `delete`, read via real `lookup`.
  (reduces-to value-flip.)

### stdlib/collections/dropkey-drop-first-fails-none-law-dup (soundness)
- spec: `58 §3` D1, Fork D **build-pin** (`drop_key` = filter, NOT drop-first;
  `evt_3z7c592g37rtr` + spec-leader `evt_51q1f8jcshgcd`).
- given: a **hand-built duplicate-key** tree
  `Node (Node Leaf k v1 Leaf) k v2 Leaf` (a non-`Distinct` input, the law-5
  counterexample shape) over `Nat`, and `lookup k (delete k ·)`.
- expect: **filter-`drop_key` reduces-to `None`** (removes **both** `k` entries →
  nothing survives) — the None-law holds **unconditionally**. A **drop-first**
  `drop_key` (removes only the first match) leaves the second `k` entry →
  `from_list` keeps it → `lookup k` reduces-to `Some v` → **the None-law FAILS**.
  The two `drop_key` semantics land on **opposite** verdicts on this input; over
  a `Distinct` map they coincide (both `None`) — the dup input is what
  discriminates.
- why: pins the exact soundness-adjacent build-pin — **filter is what makes the
  None-law unconditional**; drop-first silently reintroduces a `Distinct`
  dependency. **`(soundness)`** — a drop-first build passes every
  `Distinct`-input case (green-vs-green) and only this dup-input case catches
  it; keyed on the **structural** discriminator (duplicate-key survivor). Honest
  note: the input is a non-`Distinct` hand-built tree (not reachable by real
  `insert`), asserting the **robustness** the filter semantics guarantee.
  (soundness; verdict-flip on `drop_key` semantics.)

### stdlib/collections/delete-other-key-law-threads-ordered-distinct
- spec: `58 §3.2` D1, `54 §5` law 5 (`lookup_assoc_agree`,
  `catalog/packages/Data/Collections/Map.ken.md`), Fork D
  (other-key threads `Ordered`+`Distinct`).
- given: when `k` is **order-distinct** from `key` (i.e.
  `IsTrue (order_equiv_key leq k key)` is **false**, with
  `order_equiv_key leq a b := bool_and (leq a b) (leq b a) : Bool` — Bool-valued
  per fold-in 1, `58 §3`), the other-key law
  `lookup k (delete key m) ≡ lookup k m`, over a **real inserted** `Map Nat` (so
  `Ordered`+`Distinct` hold), with `k` an **untouched** key present in `m`.
- expect: **`lookup k (delete key m)` reduces-to the SAME `Some v` as
  `lookup k m`** — deleting `key` leaves `k`'s entry untouched. Non-degenerate:
  probe an **untouched present** key (probing only the **deleted** key is
  green-vs-green — both `None`, `delete-none-law` territory). A `delete` that
  removes the wrong subtree / too much yields `None` or a wrong value at `k` —
  the flip.
- why: D1's locality face; the law-5-roundtrip law (reuses landed
  `lookup_assoc_agree` + `from_list` dual + a `drop_key`/`assoc` lemma). The
  discriminator MUST probe an untouched key — the
  [[assert-specific-error-variant-not-is-err]] / non-degenerate-pair rule.
  (reduces-to value-flip.)

---

## D2 — `union`/`intersection`/`difference` + set algebra

### stdlib/collections/union-lookup-characterization-four-case
- spec: `58 §4` D2, Fork A (combining fn `f (from-a, from-b)`, 4-case
  orientation).
- given: `lookup k (union f a b)` at each of the 4 cases, over `Map Nat`, with a
  **non-commutative `f`** (e.g. `f x y = x`, left-projection) and a key present
  in **both** `a` and `b` with distinct values.
- expect: **reduces-to** — both-`None`→`None`; `(Some x, None)`→`Some x`;
  `(None, Some y)`→`Some y`; `(Some x, Some y)`→**`Some (f x y)`** (orientation
  `f x y`, the `a`-value first). The collision case with a non-commutative `f`
  **pins the orientation**: `Some (f x y)`, NOT `Some (f y x)`. A `union` that
  drops `b`'s exclusive keys, or applies `f` reversed, flips the observed value.
- why: D2's headline map law (`§5.2`-analog for merge). The **non-commutative
  `f` + both-present key** is the discriminator: a commutative `f` or a
  one-side-only key would be green-vs-green on the orientation. (reduces-to
  value-flip; orientation-pinned.)

### stdlib/collections/map-union-not-commutative-no-false-law (soundness)
- spec: `58 §4` D2, Fork A (**maps get NO commutativity law**).
- given: over `Map Nat` with a **non-commutative `f`** and a key `k` present in
  both with `x ≠ y`: `lookup k (union f a b)` vs `lookup k (union f b a)`.
- expect: **they reduce to DIFFERENT values** — `Some (f x y)` vs
  `Some (f y x)`, and `f x y ≠ f y x`. So **`union f a b ≡ union f b a` is
  FALSE** for maps — a conformance case (or a spec law) asserting map-union
  commutativity **unconditionally** is **unsound** (it asserts a false
  equation). Only the **lookup characterization** (above) and
  `Ordered`-preservation are map laws; commutativity is **Set-only** (below, and
  only because `V = Unit` makes `f` trivial). Assert the **dis-equality**
  structurally.
- why: the "don't over-claim" boundary — pins that map-union commutativity is
  **false, not merely unproved**. **`(soundness)`** — a build (or seed) that
  shipped a map-commutativity law would either `Axiom`-postulate a false
  equation or fail to elaborate; the discriminator is the concrete
  counterexample. Keyed on the non-commutative `f` (a commutative `f` hides it).
  (soundness; verdict-flip on a false-law claim.)

### stdlib/collections/set-union-comm-extensional-not-tree-equal (soundness)
- spec: `58 §5`/`§9 AC6` sub-ruling 1, `52 §4.4` (Set = Map-Unit), `16 §1.3`
  (`Ω` props).
- given: two formulations of set-union commutativity over `Set Nat`
  (`Tree Nat Unit`): **(A) membership-extensional**
  `(∀k. set_member k (set_union a b) ≡ set_member k (set_union b a))`; **(B)
  Tree-`Equal`** `Equal (Set Nat) (set_union a b) (set_union b a)`.
- expect: **(A) accepts, (B) rejects** — the two formulations **flip**. (A) is a
  real proof: both sides reduce (pointwise) to
  `bool_or (set_member k a) (set_member k b)` vs the swap, closed by `bool_or`
  commutativity (finite `2×2` via landed `bool_dichotomy`), **no `Tree`
  induction**. (B) is **false, not merely unprovable**: `set_union a b` and
  `set_union b a` are built by `fold`+`insert` and produce **shape-different
  trees with the same key-set**, so `Equal (Set Nat) …` between them is
  uninhabited (would need `Node`-injectivity on distinct shapes). A build
  proving (B) would inhabit `Bottom` (or `Axiom`-postulate a false `Equal`).
- why: the load-bearing sub-ruling-(1) soundness pin — **extensional is the ONLY
  sound set-law formulation**. **`(soundness)`** — the non-degenerate PAIR
  ([[differential-verify-which-mechanism-is-the-net]] discipline: same inputs,
  two formulations, opposite verdicts), keyed on the **structural**
  discriminator (extensional-membership vs Tree-`Equal`). A single (A)-accept
  case is green-vs-green under a build that never tries (B). (soundness;
  verdict-flip pair on formulation.)

---

## D3 — `keys` / `values` coherence

### stdlib/collections/keys-coherence-mem-iff-issome-lookup
- spec: `58 §6` D3, `52 §5.3` (`to_list`-ordered),
  `catalog/packages/Data/Collections/Map.ken.md`
  (`pair_keys`/`all_keys`/`to_list_ordered`).
- given: `mem k (keys m)` vs `is_some (lookup k m)`, over `Map Nat`, for **k
  present** and **k absent** (the non-degenerate pair).
- expect: **the two `Bool`s reduce to the SAME value** — `True`/`True` for a
  present key, `False`/`False` for an absent one. `keys m = pair_keys (to_list m)`
  (landed pieces), so membership in `keys` tracks `lookup`-someness exactly. A
  `keys` that drops an entry (→ `mem` `False` while `lookup` `Some`) or
  duplicates/fabricates one flips the agreement.
- why: D3's coherence law; reuses `pair_keys`/`to_list`. Non-degenerate present +
  absent pair (a present-only test is green-vs-green under a "keys returns
  everything" bug). (reduces-to value-flip.)

### stdlib/collections/keys-ascending-off-tolistordered
- spec: `58 §6` D3 (keys-ascending coherence), `54 §5` (`to_list_ordered`),
  `catalog/packages/Data/Collections/Map.ken.md` (`pair_keys`/`pair_leq`).
- given: `is_sorted leq (keys m)` over an `Ordered` `Map Nat` built by inserting
  keys in **non-ascending** order (e.g. `insert 3 · insert 1 · insert 2`).
- expect: **reduces-to a proof / ascending list** —
  `keys m = pair_keys (to_list m)`, and the landed `to_list_ordered` gives
  `is_sorted (pair_leq leq) (to_list m)`; since `pair_leq` compares first components
  (the keys), projecting via `pair_keys` **preserves** the ascending order, so
  `is_sorted leq (keys m)` holds regardless of insertion order. A `keys` that
  emitted in insertion/tree order (not the `to_list` order) yields a
  non-ascending list — the flip is on the **list order**.
- why: D3's ordering-coherence face (`58 §6`), **distinct** from the membership
  coherence above — reuses the landed `to_list_ordered` (the reconcile-surfaced
  gated addition; `values` carry NO such claim, the adjacent case). Insertion
  out of order is the discriminator (a pre-sorted input is green-vs-green).
  (value-flip on list order.)

### stdlib/collections/values-no-ordering-coherence-claim
- spec: `58 §6` D3 (`values` = `pair_vals (to_list m)`; no ordering claim).
- given: the `values` op + the D3 law surface.
- expect: `values m` yields the values **in `to_list` (ascending-key) order**,
  but **no conformance case asserts `values` is itself sorted** — values carry
  no order (`pair_vals` is `pair_keys`'s mirror with `pair_snd`). The **only**
  ordering coherence is `keys`'s (above), tracking `to_list_ordered`. A case
  asserting `is_sorted (values m)` would false-fail a faithful build (values
  needn't be ordered).
- why: pins the `keys`/`values` **asymmetry** honestly — keys inherit the key
  ordering, values do not. Guards a conformance over-claim. (boundary; named
  scope.)

---

## D4 — `Relation` (landed half + the deferred-closure boundary)

### stdlib/collections/non-transitive-relation-fails-istransitive (soundness)
- spec: `58 §7` D4 / Fork C-scope (landed `is_transitive` Π-into-`Ω`),
  `16 §1.1`.
- given: a concrete relation over **≥3 distinct `Nat` keys** `{a, b, c}` as
  adjacency `Map Nat (Set Nat)`: `R = {a↦{b}, b↦{c}}` (edges a→b, b→c, **no**
  a→c), and its transitive-completion `R' = R ∪ {a↦{b,c}}`.
- expect: **proof-flip on the Π-into-`Ω` predicate `is_transitive` over `Nat`** —
  it is `(x y z) → rel_member x y r → rel_member y z r → rel_member x z r`
  (`rel_member` is **already** `IsTrue (set_member …)` : Prop, `58 §7` — the
  premises are NOT re-wrapped in `IsTrue`). For `R'` it is **provable** (each
  live triple's conclusion holds; every other triple is vacuous — a false
  premise `rel_member _ _ R'` discharges by `absurd`). For `R` the sole live
  triple `(a,b,c)` has conclusion `rel_member a c R`, which reduces to
  `IsTrue False` = **uninhabited** → **not provable** → **reject**. Accept `R'`
  / reject `R`: opposite verdicts, keyed on the closing edge a→c.
- why: D4's headline relation discriminator (`58 §7`, the chapter's own
  example). **`(soundness)`** — needs **≥3 distinct keys** (a→b→c ⊬ a→c), which
  `Bool` (2 keys) cannot exhibit; hence the `Nat` carrier prerequisite binds.
  Over `Nat` the Ω-predicate itself is the discriminator (no decidable `Bool`
  `is_transitive` is defined). Its accept arm needs the Axiom-free `leq_nat`, so
  a `Map Int` arm would be vacuous. The producer lands the transparent
  `is_transitive` definition, but neither this proof-flip nor a general theorem
  inhabiting it is in `map_build_acceptance`. This remains an **unexecuted
  relation-law conformance obligation**, with the closing edge as its
  non-degenerate discriminator. (soundness; unexecuted Ω-predicate proof-flip;
  ≥3-key carrier-gated.)

### stdlib/collections/relation-properties-are-pi-into-omega
- spec: `58 §7` D4 / Fork C-scope, `16 §1.1` (Π-into-`Ω` is sound — properties
  are not proof-relevant).
- given: `is_symmetric` over concrete `Nat` relations — a symmetric `R_s` (`a↔b`)
  and a non-symmetric `R_a` (`a→b`, no `b→a`).
- expect: **proof-flip** —
  `is_symmetric r := (x y) → rel_member x y r → rel_member y x r` is **provable**
  for `R_s` (each live pair's converse holds, the rest vacuous) and **not
  provable** for `R_a` (the `(a,b)` obligation `rel_member b a R_a` reduces to
  `IsTrue False`). `is_equivalence` = the `And` of refl/sym/trans. These are
  `Π`-into-`Ω` predicates (**fine** — proof-irrelevant, no path data), **not**
  proof-relevant inductives. Non-degenerate: the missing `b→a` edge.
- why: pins the `Ω`-sound shape of the landed predicate definitions and retains
  the missing concrete discriminator. **Reflexivity caveat (soundness — a
  self-catch):** `is_reflexive r := (x:k) → rel_member x x r` quantifies over
  all of `k = Nat`, so a finite adjacency relation can never inhabit it (most
  `x` have empty `succ`). A sound `is_reflexive` accept arm needs a
  reflexive-by-construction relation, not a finite one. Symmetry and
  transitivity avoid this because off-fixture premises are false. The producer
  lands these transparent definitions; `map_build_acceptance` constructs none
  of the stated proof-flip arms. This case therefore remains an **unexecuted
  relation-law conformance obligation**. (soundness; unexecuted Ω-predicate
  proof-flip.)

### stdlib/collections/converse-and-compose-membership
- spec: `58 §7` D4 (landed `compose`/`converse` definitions; general membership
  proofs and the full paired controls remain residual), `16 §1.1`.
- given: over `Nat` relations (adjacency `Map Nat (Set Nat)`), the required
  `converse` characterization
  `rel_member y x (converse R) ⇔ rel_member x y R`, and the required `compose`
  characterization in which `rel_member x z (compose R S)` reflects
  `succ x (compose R S) = ⋃ { succ y S : y ∈ succ x R }`.
- expect: **value-flip on relation membership.** With `R = {a→b}`:
  `rel_member b a (converse R)` holds while `rel_member a b (converse R)`
  reduces to `IsTrue False`. With `R = {a→b}`, `S = {b→c}`:
  `rel_member a c (compose R S)` holds while
  `rel_member a b (compose R S)` does not. A failed transpose or a compose that
  mis-unions the `S`-images flips the paired observation.
- why: the definitions reuse landed `fold`/`union`/`member`, but definition
  presence is not a general membership proof. The current reaching test
  `cat4_relations_compose_and_converse_over_adjacency_maps` observes only the
  present compose edge `1 → 3` and present converse edge `2 → 1`. It does not
  execute either absent-edge arm above. The two positives are **landed smoke
  evidence**; both negatives and both general characterizations remain
  **unexecuted relation-law conformance obligations**. (partially executed
  value-flip; full positive/negative pair and general proofs residual.)

### stdlib/collections/transitive-closure-decidable-not-raw-omega (soundness)
- spec: `58 §7`/`§1 pt 1`, `16 §1.4`+§1.1, and
  `../../challenge/C2-proof-relevant-omega`.
- given: the exact `reachable_plus` result type and definition over the
  Bool-valued `reachable_within` recurrence.
- expect: `reachable_plus` is the `Ω`-native value equation
  `IsTrue (reachable_within k leq (size k Unit (dom k (Tree k Unit) r)) x y r)`.
  It is not truncation and never a raw multi-constructor
  `data TransClosure … : Ω` carrying paths. The latter would make distinct paths
  proof-relevant at `Ω`, which is inadmissible (`16 §1.4`+§1.1). The Bool form
  has the executable boundary exercised by the cases below.
- why: pins AC5's representation and trust boundary independently of any one
  graph. (soundness; structural; four-function computation deferred.)

**Common premises for every deferred behavioral case below.** Each relation uses
`Nat` with the landed `leq_nat` and its Axiom-free lawful-order results. Outer
maps and every stored successor set are built with the landed constructors and
operations so they are `Ordered` under that same comparator. These cases lie
inside `58 §7`'s path-correspondence premises; none uses a malformed raw tree.
Each is **BLOCKED-ON-CAT-REL-TRANSITIVE-CLOSURE** until all four named functions
land. A blocked case is not reported green merely because the current 29-test
suite run is green.
The displayed `0`/`1`/`2`/`3` are compact labels for `Zero` and successive
`Suc` values; executable fixtures use those actual `Nat` constructors.
**Promise class:** every row is a durable invariant; numeric literals are fixed
fixture expectations, never a census of current repository state. When the gate
activates, each case must execute the named public function and its `why` clause
names the incorrect neighboring behavior that must flip.

### stdlib/collections/closure-size-counts-fixed-three-node-tree (deferred)
- spec: `58 §7` (`size`).
- given: an explicitly constructed well-formed `Tree Nat Unit` with exactly
  three `Node` constructors: one root, one left child, and one right child.
- expect: `size Nat Unit t` reduces to the fixed independent value
  `Suc (Suc (Suc Zero))`. The expected value is not computed through `size`,
  `to_list`, or another traversal of the subject tree.
- why: pins raw node count and both subtree contributions; empty-only or
  self-derived expectations would not discriminate an omitted branch.
  (value-flip; blocked on closure computation.)

### stdlib/collections/closure-dom-keeps-only-outer-keys (deferred)
- spec: `58 §7` (`dom`).
- given: a well-formed `r_dom` with outer keys `{0, 2}`, edge `0 → 1`, and
  edge `2 → 3`, where `1` and `3` occur only in successor sets.
- expect: `dom Nat (Tree Nat Unit) r_dom` has exactly the preserved outer-tree
  shape and keys `{0, 2}`, with each value replaced by `MkUnit`; membership is
  false for target-only
  keys `1` and `3`. No comparator is consumed by `dom`.
- why: catches a domain implementation that unions in successor targets rather
  than projecting the outer key set. (value/shape flip; blocked on closure
  computation.)

### stdlib/collections/closure-zero-one-direct-edge-boundary (deferred)
- spec: `58 §7` (fuel recurrence).
- given: the one-edge relation `r_edge = {0 → 1}`.
- expect: `reachable_within Nat leq_nat Zero 0 1 r_edge = False` and
  `reachable_within Nat leq_nat (Suc Zero) 0 1 r_edge = True` on the same
  relation.
- why: the paired boundary rejects direct-at-zero and catches an off-by-one fuel
  convention. (Bool flip; blocked on closure computation.)

### stdlib/collections/closure-positive-self-not-reflexive (deferred)
- spec: `58 §7` (positive `R⁺`, not reflexive `R*`).
- given: `r_acyclic = {0 → 1}` and, on the same endpoint `0`,
  `r_loop = {0 → 0}`.
- expect: `reachable_plus Nat leq_nat 0 0 r_acyclic` is uninhabited, while
  `reachable_plus Nat leq_nat 0 0 r_loop` is inhabited; the self-loop already
  makes `reachable_within Nat leq_nat (Suc Zero) 0 0 r_loop` true.
- why: a reflexive zero-step base makes both arms succeed and therefore fails
  this shared-endpoint pair. (proof/Bool flip; blocked on closure computation.)

### stdlib/collections/closure-positive-two-edge-cycle (deferred)
- spec: `58 §7` (positive-cycle treatment).
- given: the well-formed two-cycle `r_cycle = {0 → 1, 1 → 0}` with no
  self-loop.
- expect: `reachable_within Nat leq_nat (Suc Zero) 0 0 r_cycle` is false,
  `reachable_within Nat leq_nat (Suc (Suc Zero)) 0 0 r_cycle` is true, and
  `reachable_plus Nat leq_nat 0 0 r_cycle` is inhabited because the outer-domain size
  is two.
- why: distinguishes positive cyclic self-reachability from both direct
  self-loops and zero-step reflexivity. (Bool/proof flip; blocked on closure
  computation.)

### stdlib/collections/closure-one-two-fuel-two-edge-path (deferred)
- spec: `58 §7` (successor fold and public bound).
- given: `r_path` has outer edges `{0 → {1, 3}, 1 → {2}}`, with no direct
  edge `0 → 2`. The ordered successor tree for `0` has root `3` and left child
  `1`, so the required intermediate is not the inner-tree root.
- expect: `rel_member Nat leq_nat 0 2 r_path` is uninhabited and
  `reachable_within Nat leq_nat (Suc Zero) 0 2 r_path = False`, while
  `reachable_within Nat leq_nat (Suc (Suc Zero)) 0 2 r_path = True` and
  `reachable_plus Nat leq_nat 0 2 r_path` is inhabited at outer-domain size
  two.
- why: independently requires the successor fold rather than mistaking direct
  membership for closure. (Bool/proof flip; blocked on closure computation.)

### stdlib/collections/closure-reverse-direction-unreachable (deferred)
- spec: `58 §7` (directed positive reachability).
- given: the same `r_path` as the preceding case.
- expect: `reachable_within Nat leq_nat (Suc (Suc Zero)) 2 0 r_path = False`
  and `reachable_plus Nat leq_nat 2 0 r_path` is uninhabited.
- why: catches direction reversal or accidental undirected traversal; the
  forward case above is the positive control. (Bool/proof flip; blocked on
  closure computation.)

### stdlib/collections/closure-target-only-sink-uses-N-not-N-minus-one (deferred)
- spec: `58 §7` (outer-domain bound).
- given: `r_sink = {0 → 1}`, whose only outer key is `0`; `1` is a target-only
  sink.
- expect: `dom Nat (Tree Nat Unit) r_sink` contains only `0`,
  `size Nat Unit (dom Nat (Tree Nat Unit) r_sink) = Suc Zero`, and
  `reachable_plus Nat leq_nat 0 1 r_sink` is inhabited because fuel one recognizes
  the direct edge.
- why: an `N−1` bound supplies zero fuel and rejects this valid positive path.
  This is the independent off-by-one discriminator for the public wrapper, not
  merely another direct-call recurrence test. (proof flip; blocked on closure
  computation.)

### stdlib/collections/closure-computation-then-laws-deferred
- spec: `58 §7` (explicit fast-follow split).
- given: the four computational functions and the later general-relation-law
  tranche.
- expect: **two ordered deferrals.** `CAT-REL-TRANSITIVE-CLOSURE` first lands
  `size`, `dom`, `reachable_within`, and `reachable_plus`, activating the eight
  behavioral cases above. A separate follow-on supplies the compose/converse
  membership proofs, executes the concrete predicate discriminators, and proves
  closure correspondence by simple-path shortening and `N`-fuel saturation
  under the lawful-order/`Ordered` representation premises. This seed does not
  claim that any of those general results is already proved.
- why: names both boundaries so computation cannot be mistaken for a proof, and
  proof deferral cannot be mistaken for absent computation requirements.
  (boundary; computation and laws deferred separately.)

---

## Carrier — the standing vacuity guard

### stdlib/collections/proved-carrier-is-lawful-nat-not-int (soundness)
- spec: `58 §2`/`§9 AC7` sub-ruling 2, `52 §5.4` (Axiom-holed
  `Ord Int`/`Ord Char`), `../classes/seed-lawful-classes.md`, the CAT-3
  `List Bool` carrier lesson.
- given: the landed D0–D3 proof artifacts, the planned D4 proof-flips, and the
  choice of key carrier.
- expect: the proved accept-arms key on **`Map Nat`/`Set Nat`/`Relation Nat`**
  with the **real Axiom-free `leq_nat`** basis — **never `Map Int`**. Over `Int`
  (or `Char`), `Ord` is **Axiom-holed**, so a "proved" law's accept-arm cites an
  `Axiom` and the arm goes **vacuous**; the discriminator flip degenerates to
  **reject-vs-reject** (green-vs-green). And `Bool` (Axiom-free) has only 2 keys
  — **too few** for a ≥3-key relation-transitivity discriminator. So the sound
  proved carrier is **`Nat` + real `leq_nat`**. **The flip:** a build that
  shipped the proved arms over `Map Int` would have a **vacuous** accept-arm
  (its "proof" leans on the `Ord Int` `Axiom`) — the carrier-check catches it.
- why: the carrier-vacuity guard **promoted to a standing discriminator** so the
  build cannot silently pick an Axiom-holed carrier and ship a vacuous green
  (exactly CAT-3's `verified-sort-proved-carrier-is-lawful-bool`, one carrier up
  — Nat here because relations need ≥3 keys). The current
  `cat4_new_api_is_derived_and_axiom_free` row establishes that `leq_nat` and
  its four order results are transparent with zero trusted-base delta. It does
  not execute every proof-flip's carrier choice; the cross-case inventory below
  keeps that separate structural obligation explicit. **`(soundness)`** —
  verdict-independent structural (which carrier + whether its order basis cites
  an `Axiom`). (soundness; landed order-basis evidence; cross-case carrier-use
  obligation retained.)

---

## Coverage map (AC / deliverable → cases)

- **AC1 (kernel-untouched):** `cat4-kernel-untouched-outer-ring`.
- **AC2 (reuse, zero re-derivation):**
  `delete-union-reuse-landed-no-rederivation`,
  `leqnat-d0-real-axiom-free-carrier`.
- **AC3 (invariant preservation):** `delete-preserves-ordered`,
  `union-preserves-ordered-f-independent`.
- **D1 (`delete` laws):** `delete-none-law-unconditional`,
  `dropkey-drop-first-fails-none-law-dup`,
  `delete-other-key-law-threads-ordered-distinct`.
- **D2 (`union` + set algebra):** `union-lookup-characterization-four-case`,
  `map-union-not-commutative-no-false-law`,
  `set-union-comm-extensional-not-tree-equal`.
- **D3 (`keys`/`values`):** `keys-coherence-mem-iff-issome-lookup`,
  `keys-ascending-off-tolistordered`, `values-no-ordering-coherence-claim`.
- **D4 (landed definitions + relation-law and closure residuals):**
  `non-transitive-relation-fails-istransitive`,
  `relation-properties-are-pi-into-omega`, `converse-and-compose-membership`,
  `transitive-closure-decidable-not-raw-omega` (AC5 representation pin),
  `closure-size-counts-fixed-three-node-tree`,
  `closure-dom-keeps-only-outer-keys`,
  `closure-zero-one-direct-edge-boundary`,
  `closure-positive-self-not-reflexive`,
  `closure-positive-two-edge-cycle`,
  `closure-one-two-fuel-two-edge-path`,
  `closure-reverse-direction-unreachable`,
  `closure-target-only-sink-uses-N-not-N-minus-one`, and
  `closure-computation-then-laws-deferred`.
- **Carrier (standing vacuity guard):** `proved-carrier-is-lawful-nat-not-int`.

## Cross-case consistency sweep

- **Carrier discipline uniform:** every landed or planned soundness accept arm
  keys on `Nat` + real `leq_nat` (never `Int`/`Char` Axiom-holed, never `Bool`
  for the ≥3-key relation cases) — the standing guard binds all of them.
- **Commutativity, one story across cases:** `map-union-not-commutative` (maps —
  false) and `set-union-comm-membership-extensional` (sets — true, extensional)
  do **not** contradict: map union is non-commutative in general; set union is
  commutative **only** because `V = Unit` trivializes `f`, and **only** stated
  membership-extensionally (Tree-`Equal` is false for both). No case asserts map
  commutativity; no case asserts a Tree-`Equal` set law.
- **`delete` laws, hypothesis-consistent:** None-law **unconditional** (filter);
  other-key **threads `Ordered`+`Distinct`** (law-5 roundtrip);
  `dropkey-drop-first` fires **only** on a non-`Distinct` dup input — the three
  agree on when `Distinct` is / isn't needed (Fork D's exact split).
- **Endpoint tokens per-branch, NOT uniform:** `Leaf`/`Equal Bool True True`
  collapse → `Proved`; `from_list_acc Nil acc` → passthrough of `acc` (not
  `Proved`); neutral steps → `Refl`/`cong`; the delete lookup-side laws'
  non-nullary-head-with-neutral- component closers → `Refl` (the sharpened
  `57 §1 pt 3` rule) — build-pinned, reconciled at the build.
- **D4 evidence has four distinct classes.** `compose`/`converse` and the
  predicates are landed transparent definitions. The current suite supplies two
  positive relation smoke observations. The missing negative arms and concrete
  predicate proof-flips remain conformance obligations. General membership
  proofs remain later relation-law work. None is inferred from another.
- **Closure, one design and two stages:**
  `transitive-closure-decidable-not-raw-omega` pins the `IsTrue` boundary. The
  eight behavioral rows independently cover raw node count, outer-only domain,
  fuel, positive self-reachability, directed traversal, and the public `N`
  bound. All eight activate with the four-function computation. The general
  faithfulness/saturation proof remains a later, separately named follow-on.
- **Well-formedness is uniform:** every closure path case uses one lawful
  `leq_nat` order and `Ordered` outer and inner trees. No case generalizes its
  expected path semantics to malformed raw trees, even though the equations
  remain total there.

## Reconcile state

This seed is reconciled to the current `58 §7` contract and the producer
snapshot at `origin/main@1bd3a5667`. In particular:

- operative calls and proof identities use the producer's snake-case public
  names; `unionWith` is retained only as an explicitly named Haskell analogue,
  and local binders are not presented as globals;
- the exact closure names are `size`, `dom`, `reachable_within`, and
  `reachable_plus`; no camel-case closure spelling remains;
- the old `N−1` correspondence claim is removed and the target-only-sink case
  independently requires the corrected `N` bound;
- D0 order results, D1–D2 general proofs, and D3 projection/ascending proofs
  are landed; D4 operation/predicate definitions, the two positive smoke
  observations, unexecuted conformance arms, and general
  proof residuals are stated separately;
- the 29-test result is anchored to the exact test and Map blobs that were run,
  and no individual D4 obligation is credited from aggregate greenness; and
- the four closure functions and later general relation proofs remain distinct
  deferrals, while the closure representation, recurrence, behavioral cases, coverage
  map, and well-formedness assumptions tell one story.

Any independent vote binds both this seed's citations and its case coverage to
the exact candidate, not merely to the historical CAT-4 fork record.
