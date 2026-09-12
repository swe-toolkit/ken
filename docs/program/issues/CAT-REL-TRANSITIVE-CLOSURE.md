---
id: CAT-REL-TRANSITIVE-CLOSURE
title: "the binary-relations frontier (spec 58 / CAT-4, Fork B) names transitive closure as R+ x y := IsTrue (reachableWithin N x y), N := size (dom R), but the computational realization is design-pinned PROSE only -- Map.ken.md §4.7.12 lands succ/compose/converse and the property predicates, while size, bounded reachable_within, and reachable_plus do NOT exist on main; this node lands the kernel-untouched, Axiom-free computational closure (ONE node, no size/dom split) once the fuel-recurrence spec correction lands (the faithfulness/saturation laws are the named deferred fast-follow)"
status: ready
owner: foundation
size: M
gate: none
depends_on: [SPEC-REL-CLOSURE-RECURRENCE]
blocks: []
github: null
origin: "Steward cut 2026-09-12, RE-RELEASED 2026-09-12 after SPEC-REL-CLOSURE-RECURRENCE landed (4ccc932fe): the fuel-recurrence correction is on main and the inline contract below is verified byte-aligned to landed spec 58 §7 (recurrence 58:436-455, public predicate/bound 58:465-477, Omega-soundness 58:479-482, AC1 kernel-untouched 58:540-543). Original cut note follows. Steward cut 2026-09-12, the first node of the resumed catalog campaign's Band A frontier-harvest tranche, on the operator ruling of 2026-09-12 (Pat: 'develop a work program for L3 ... the catalog is the long arc ... what does the frontier enable'; 'concur with rec'). RECUT 2026-09-12 under Architect HS1 evt_16w35x7zxj442 (BOUNDED_CLOSURE_FUEL_CONVENTION_UNSPECIFIED_AND_BOUND_PROSE_OFF_BY_ONE, grounded at exact main b7c829c9f): keep ONE computational node (no size/dom split -- the composition size(dom r) must be gate-inspected in one candidate), but HOLD until the fuel-recurrence spec correction lands. Grounded against the tree: relation algebra at catalog/packages/Data/Collections/Map.ken.md §4.7.12 (succ :15122, rel_member :15128, compose :15153, converse :15173, predicates :15182-15194); closure design-pinned prose only (:15045-48, :15219); size/reachable_within/reachable_plus absent on main. Spec contract spec/50-stdlib/58-maps-sets-relations.md §7 (R+ :362, bound-prose :368-370, laws :378-79/:402-03, Omega-soundness AC5 :428-31, size kernel-untouched AC1 :414-17, Axiom-free AC7 :435-37). Spec 58 DRAFT v0. No capability blocker (ordinary total Ken; nested inductives merged; deceq carried as leq). No pre-existing tracker node for relation closure."
---

> # RE-RELEASED 2026-09-12 (was RECUT/HELD, Architect HS1 `evt_16w35x7zxj442`). READ.
>
> **Decomposition ruling: KEEP ONE computational node -- do NOT split
> `size`/`dom` from `reachable_within`/`reachable_plus`.** They are small
> structural prerequisites whose only new frontier consumer here is the bound
> `size (dom r)` appearing literally in `reachable_plus`; one candidate lets the
> gate inspect the actual composition rather than two independently-green
> artifacts. A split buys no isolation and adds a WP edge for a dependency
> internal to this closure.
>
> **The [[SPEC-REL-CLOSURE-RECURRENCE]] hold is LIFTED.** The correction landed
> on main as `4ccc932fe`: it states the exact fuel recurrence (zero recognizes no
> path; `Suc n` is `edge_R OR fold`), fixes the bound `N - 1 -> N :=
> size (dom R)`, and settles `R+` (not `R*`) semantics. The inline contract in
> "The contract" and "Deliverables" below is verified **byte-aligned to the
> landed spec** on all four points. Authoritative landed anchors (the draft-era
> `58:XXX` citations elsewhere in this frame predate the +486 reorganization and
> are historical -- chase these): four signatures 58:405-413; fuel recurrence
> 58:436-455; `R+`-not-`R*` semantics 58:457-463; public predicate + `size (dom
> r)` bound 58:465-477; `Perm`-move / no proof-relevant inductive 58:479-482;
> AC1 kernel-untouched / no `Axiom` 58:540-543; AC5 relation `Omega`-soundness
> 58:555.
>
> Kernel-untouched, `Axiom`-free -- no TCB growth, no operator touch. Owner
> foundation; Architect required reviewer (relations-frontier design authority) +
> foundation-QA + CV + standing Adversary -> Steward M1-M4 -> lieutenant.
> Re-measure every `Map.ken.md` anchor at the cut (current main is downstream of
> the byte-clean branch base `2d35fd3b6`).

## What this is

`spec/50-stdlib/58-maps-sets-relations.md §7` (CAT-4, the relations frontier)
pins transitive closure as **Fork B**: `reachable_plus x y := IsTrue
(reachable_within (size (dom r)) x y)` -- a Π-into-Ω predicate over a decidable
bounded `Bool`, deliberately NOT a raw `data ... : Ω` closure inductive
(58:35-38, :428-430). The relation algebra it builds on is landed in
`catalog/packages/Data/Collections/Map.ken.md §4.7.12`: a relation is the
adjacency representation `Tree k (Tree k Unit)` (key to successor-set;
`Set = Map Unit`), with `succ` (:15122), `rel_member` (:15128), `compose`
(:15153), `converse` (:15173), and the property predicates (:15182-15194).

**Missing on main, and what this ONE node lands:** the closure realization is
prose-only (Map.ken.md:15045-15048, :15219). Absent: `size`, bounded
`reachable_within`, and `reachable_plus`. This node lands all three as the
**computational** closure, kernel-untouched and `Axiom`-free (58:414-417,
:435-437). The **faithfulness/saturation laws** (58:368-370 as corrected) are
the enclave's stated deferred fast-follow (58:378-379, :402-403) and a NAMED
follow-on, not this node.

## The contract (from [[SPEC-REL-CLOSURE-RECURRENCE]], Architect ruling)

Signatures (existing snake-case Map API):

```text
size            : (k v : Type) -> Tree k v -> Nat
dom             : (k v : Type) -> Tree k v -> Tree k Unit
reachable_within: (k : Type) -> (leq : k -> k -> Bool) -> (fuel : Nat)
                  -> (x y : k) -> (r : Tree k (Tree k Unit)) -> Bool
reachable_plus  : (k : Type) -> (leq : k -> k -> Bool)
                  -> (x y : k) -> (r : Tree k (Tree k Unit)) -> Prop
```

- `dom` = the outer key set: structurally replace every node value by `MkUnit`,
  preserving `Leaf`/`Node`, key, subtree shape. It does NOT union targets into
  the domain and needs no comparator.
- `size` counts raw `Tree` nodes: `Leaf ↦ Zero`;
  `Node l _ _ r ↦ Suc (add (size l) (size r))`, reusing canonical
  `Data.Numeric.Nat.Arithmetic.add`.
- Fuel recurrence, `edge_R x y = set_member k leq y (succ k leq x r)`:

  ```text
  reachable_within 0       x y r = False
  reachable_within (Suc n) x y r =
    edge_R x y
    OR fold OR False { reachable_within n z y r | z ∈ succ k leq x r }
  ```

  Every recursive call decreases exactly `n`, including in the fold step.
- **Fuel is the maximum positive path length** (this is `R+`, not `R*`): fuel 0
  recognizes no path; fuel 1 exactly a direct edge; fuel 2 additionally a
  two-edge path; `x = y` is accepted only via a positive self-loop/cycle, never
  by reflexivity.
- `reachable_plus k leq x y r = IsTrue (reachable_within k leq
  (size k Unit (dom k (Tree k Unit) r)) x y r)`. `IsTrue` may unfold to the
  existing `Equal Bool _ True`; no path witness, no proof-relevant inductive.

## Deliverables

1. `size` and `dom` per the signatures/definitions above.
2. `reachable_within` per the exact fuel recurrence, and `reachable_plus` per the
   exact public predicate.
3. Reaching tests carrying ALL of these independent discriminators (over `Nat`
   with `leq_nat`, the spec's pinned carrier), each a genuine differential:
   - direct-edge boundary: `reachable_within 0 a b r = False`,
     `reachable_within 1 a b r = True` on the same `a -> b`;
   - positive-vs-reflexive: an acyclic no-self-edge relation rejects
     `reachable_plus a a`; a self-loop accepts it;
   - two-step: on `a -> b -> c` with no `a -> c`, fuel 1 rejects `a,c`, fuel 2
     accepts it, `rel_member a c` rejects it, `reachable_plus a c` accepts it;
   - unreachable: the same relation rejects `c,a`;
   - outer-domain off-by-one: on the single edge `a -> b` where `b` has NO outer
     map entry, `dom r` is `{a}`, `size (dom r) = 1`, and `reachable_plus a b`
     still accepts (catches both a false `N - 1` assumption and a base case
     shifted to direct-at-zero).
   Plus a `dom` successor-leakage pin: adding `b` merely as a target must NOT
   make `b` a domain member. The `size` oracle in a multi-node test is derived
   INDEPENDENTLY, not computed through `size` itself.

## Acceptance criteria

- `size`, `dom`, `reachable_within`, `reachable_plus` compile in the Map package,
  `Axiom`-free and kernel-untouched (no `Decl::Opaque`, no primitive, no TCB
  entry, no raw `data ... : Ω`; 58:435-437). Verified by the catalog build.
- `reachable_plus x y` reduces to the exact landed public-predicate form
  (58:465-477) with the bound `size (dom r)`.
- All five discriminators above hold (none vacuous), and the `dom`-leakage pin
  holds.
- The Ω-encoding is sound (58:479-482, AC5 58:555): `reachable_plus` is Π-into-Ω
  over a decidable bounded `Bool`.
- The landed §4.7.12 relation ops stay green.

## Implementation constraints

Reuse `Data.Numeric.Nat.Arithmetic.add`, `fold`, `succ`, `set_member`, and
`cat4_bool_or` -- no locally rederived `Nat` addition, `Bool` disjunction, set
traversal, or generic bounded-iteration interface. Add no `Axiom`, primitive,
`Decl::Opaque`, kernel change, or raw `data ... : Ω`. Lead the new literate
subsection with the headline `reachable_plus` contract and purpose;
dependency-ordered checked declarations follow beneath that lede.

## Not this node

- **The faithfulness/saturation LAWS** (58:368-370 corrected, :378-379,
  :402-403) -- the deferred fast-follow (`CAT-REL-CLOSURE-LAWS`, filed when this
  lands).
- **Any raw `data ... : Ω` closure inductive** -- forbidden by Fork B.
- **A general bounded-iteration interface** beyond what closure needs
  (Map.ken.md:15219 defers that).

## Symptom inventory

Append one entry per Architect hard stop; never rewrite history.

1. `R+` fixes its fuel as `size (dom R)` but does not define
   `reachableWithin`'s zero/successor convention, while the stated `N - 1`
   simple-path bound fails when `dom R` is the outer-key set and the endpoint is
   a target-only sink -- keyed on a named closure form without a closed fuel or
   vertex-population contract.

## Contention

Foundation ring, `Map.ken.md` (Data/Collections). No cross-lane contention (L1
runtime on `crates/ken-lowering`; L2 language on `crates/ken-elaborator`).
Re-measure every Map.ken.md anchor at the cut. The draft-v0 underspecification
that caused HS1 is resolved by the `depends_on`
[[SPEC-REL-CLOSURE-RECURRENCE]]; do not begin source edits until it lands and the
Steward re-releases.

## Sizing / tier

**Size M, tier T1.** Ordinary total Ken (structural recursion on a `Nat` bound
over the adjacency representation), but the review turns on a design argument:
the bounded-`Bool` Ω-encoding faithfully realizing closure without a
proof-relevant inductive, the fuel semantics being `R+` not `R*`, and the
`size (dom r)` bound. Architect is the required reviewer.
