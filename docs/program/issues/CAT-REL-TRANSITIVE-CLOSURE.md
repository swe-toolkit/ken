---
id: CAT-REL-TRANSITIVE-CLOSURE
title: "the binary-relations frontier (spec 58 / CAT-4, Fork B) names transitive closure as R+ x y := IsTrue (reachableWithin N x y), N := size (dom R), but the computational realization is design-pinned PROSE only -- Map.ken.md §4.7.12 lands succ/compose/converse and the reflexive/symmetric/transitive predicates, while size, bounded reachableWithin, and R+ itself do NOT exist on main; this node lands the kernel-untouched, Axiom-free computational closure (the faithfulness/saturation laws are the named deferred fast-follow)"
status: ready
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward cut 2026-09-12, the first node of the resumed catalog campaign's Band A frontier-harvest tranche, on the operator ruling of 2026-09-12 (Pat: 'develop a work program for L3 ... the catalog is the long arc ... what does the frontier enable'; concurred with the Steward's Band-A-first recommendation, 'concur with rec'). Grounded against the current working tree: relation algebra confirmed at catalog/packages/Data/Collections/Map.ken.md §4.7.12 (succ :15122, rel_member :15128, compose :15153, converse :15173, is_reflexive/symmetric/transitive/equivalence :15182-15194); transitive closure is design-pinned prose only (Map.ken.md:15045-15048, :15219 -- 'intentionally design-now/defer-build'); size, reachableWithin, R+ absent on main. Spec contract in spec/50-stdlib/58-maps-sets-relations.md §7 (D4, Fork C/B): R+ form :362, faithfulness :368-370, deferred laws :378-379/:402-403, AC1 size kernel-untouched :414-417, AC5 Omega-soundness :428-431, AC7 Axiom-free :435-437. Spec 58 is DRAFT v0; the .ken realization is the enclave-intended fast-follow. No capability blocker (ordinary total Ken; nested inductives merged, deceq carried as a leq value parameter). No existing tracker node covers relation closure."
---

> # BAND A frontier-harvest node 1 (resumed catalog campaign, operator ruling
> # 2026-09-12). Computational-first: this lands the closure REALIZATION; the
> # faithfulness/saturation LAWS are the named deferred fast-follow, not this
> # node. Owner foundation; Architect required reviewer (relations-frontier
> # design authority + decomposition) + foundation-QA + CV + standing Adversary
> # -> Steward M1-M4 -> lieutenant. Kernel-untouched, Axiom-free -- no TCB
> # growth, no operator touch.

## What this is

`spec/50-stdlib/58-maps-sets-relations.md §7` (CAT-4, the relations frontier)
pins transitive closure as **Fork B**: `R+ x y := IsTrue (reachableWithin N x y)`
with `N := size (dom R)` (58:362), a Π-into-Ω predicate wrapping a decidable
bounded `Bool` -- deliberately NOT a raw `data ... : Ω` closure inductive
(58:35-38, :428-430). The relation algebra it builds on is already landed in
`catalog/packages/Data/Collections/Map.ken.md §4.7.12`: a relation is the
adjacency representation `Tree k (Tree k Unit)` (key to successor-set;
`Set = Map Unit`), with `succ` (:15122), `rel_member` (:15128), `compose`
(:15153), `converse` (:15173), and the property predicates `is_reflexive`/
`is_symmetric`/`is_transitive`/`is_equivalence` (:15182-15194).

**What is missing on main, and what this node lands:** the closure realization
is prose-only (Map.ken.md:15045-15048, :15219). Concretely absent: `size`,
bounded `reachableWithin`, and `R+` itself. This node lands the
**computational** closure, kernel-untouched and `Axiom`-free (58:414-417,
:435-437). The **faithfulness/saturation laws** (any walk shortens to a simple
path of length <= N-1, so bounded reachability at bound >= N-1 equals full
closure, monotone and saturating -- 58:368-370) are the enclave's stated
deferred fast-follow (58:378-379, :402-403) and are a NAMED follow-on, not this
node.

## Deliverables

1. `size : (k v : Type) -> Tree k v -> Nat` -- node count by structural
   recursion (spec AC1, 58:414-417: ordinary total Ken, kernel-untouched). If
   `dom : Tree k v -> Tree k Unit` (the key set, for `size (dom R)`) is not
   already present in Map, land it here too; re-measure at the cut.
2. `reachableWithin` -- bounded-iteration reachability on the adjacency
   representation, threading the explicit `leq : k -> k -> Bool` comparator
   exactly as the §4.7.12 relation ops do: `reachableWithin` of bound `0` is the
   base step (or reflexive+step per the spec's exact form -- confirm against
   58:362 at the cut), and each successive round unions the successors reachable
   in one more step. Ordinary structural recursion on the `Nat` bound.
3. `R+` (the transitive-closure predicate): `reachablePlus x y :=
   IsTrue (reachableWithin (size (dom r)) x y)`, matching 58:362 verbatim.
4. Reaching tests: a small concrete relation (over `Nat` with `leq_nat`, the
   spec's pinned carrier, 58:395) exercising `size`, a positive `reachableWithin`
   at a bound that reaches a multi-step target, a negative for an unreachable
   pair, and `R+` agreeing with a hand-computed closure on the fixture. A
   control that the existing §4.7.12 ops (`succ`/`compose`/`converse`/predicates)
   stay green.

## Acceptance criteria

- `size`, `reachableWithin`, and `R+` compile in the Map package and are
  `Axiom`-free and kernel-untouched (no `Decl::Opaque`, no primitive, no TCB
  entry; 58:435-437). Verify with the catalog build, not by assertion.
- `R+ x y` reduces to `IsTrue (reachableWithin (size (dom r)) x y)` -- the exact
  58:362 form, `N` bound as `size (dom R)`, not an arbitrary constant.
- A genuine differential: the fixture has a pair reachable only in >= 2 steps
  that `R+` accepts and `succ`/`rel_member` (one step) rejects, and an
  unreachable pair `R+` rejects; neither is vacuous green-vs-green.
- The Ω-encoding is sound (58:428-431, AC5): `R+` is a Π-into-Ω predicate over a
  decidable bounded `Bool`, never a raw proof-relevant `Ω` closure.
- The landed §4.7.12 relation ops stay green.

## Not this node

- **The faithfulness and saturation LAWS** (58:368-370, :378-379, :402-403) --
  the enclave-stated deferred fast-follow; a named follow-on
  (`CAT-REL-CLOSURE-LAWS`), filed when this lands. This node is computational
  realization only, per the campaign's computational-first trust level.
- **Any raw `data ... : Ω` closure inductive** -- explicitly forbidden by the
  Fork-B design (58:35-38, :428-430); the bounded-`Bool` encoding is the point.
- **A general bounded-iteration interface beyond what closure needs**
  (Map.ken.md:15219 defers that) -- land only the `reachableWithin` closure
  needs.

## Contention / spec currency

Foundation ring, `Map.ken.md` (Data/Collections). No cross-lane contention (L1
runtime on `crates/ken-lowering`; L2 language on `crates/ken-elaborator`).
**Spec 58 is DRAFT v0** (58:3): the closure FORM and signatures are pinned
(58:362, :414-437), so the computational realization builds against a stable
contract, but confirm the exact `reachableWithin` base-case shape against 58:362
at the cut and raise a hard stop to Architect + Steward if the draft is
underspecified where the build needs a decision. The law statements (the
follow-on) may need enclave confirmation before that node is cut.

## Sizing / tier

**Size M, tier T1.** The code is ordinary total Ken (structural recursion on a
`Nat` bound over the adjacency representation), but the review turns on a design
argument: the bounded-`Bool` Ω-encoding faithfully realizing the spec's closure
form without a proof-relevant inductive, and the `N := size (dom R)` bound being
the right one. Architect is the required reviewer as the relations-frontier
design authority; the decomposition (whether `size`/`dom` split from
`reachableWithin`/`R+`) is the Architect's to refine.
