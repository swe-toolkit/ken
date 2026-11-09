---
id: SPEC-GRAPHS-SUCCESSOR-LIST-ADAPTOR
title: "Spec 62 builds a Graph only from a Boolean edge function (MkGraph) and names a successor-list adaptor as outside §1 (62-graphs.md:348), so a client holding adjacency lists must write the decision function and re-prove its membership meaning by hand. Rule from a checked D0 at zero trust a public adaptor from a DecEq-keyed successor function to Graph, with sound and complete laws tying its edge to list membership, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# Graphs from successor lists

## Objective

`Algorithm.Graphs.Dependency` exports an operation that builds a
`Graph q` from `DecEq q` and a successor function `q → List q`, and
two public checked laws. **Sound:** an edge of the built graph means
the target is in the source's successor list (`list_elem`).
**Complete:** the converse. Spec 62 states the operation and both laws
by their public types.

## Settled inputs (main `48cda6399`)

- **The graph.** `Graph q` has the single constructor
  `MkGraph : (q → q → Bool) → Graph q` (`Dependency.ken.md:64`), and
  `edge` (`:70`) applies its function.
- **The membership laws.** The public `contains` (`:478`),
  `contains_sound` (`:725`, `contains q d x xs = True → list_elem q x
  xs`) and `contains_complete` (`:748`, the converse).
- **The gap.** Spec 62 §1.4 lists "a successor-list adaptor" among the
  constructions outside §1 (`62-graphs.md:348`). No catalog package
  builds a `Graph` from successors.
- **The neighbouring relation.** `Data.Collections.Map` closes with a
  binary-relation library over `Tree k (Tree k Unit)`, an `Ord`-keyed
  adjacency map (`Map.ken.md:15008`). Its public face is
  `reachable_plus` (`:15423`); `succ` (`:15346`), `rel_member`
  (`:15352`) and `add_edge` (`:19538`) are private. It is not
  connected to `Graph`, which is keyed by `DecEq` and `Finite`. AC-0
  rules whether the adaptor takes `q → List q`, this adjacency map, or
  both, so the catalog does not grow a second successor representation
  by accident.
- **Why it holds.** Defining the adaptor's edge as `contains q d t
  (succ s)` makes each law the edge reduction composed with
  `contains_sound` or `contains_complete`. No new helper, import or
  Nat lemma is needed.
- **The carried inventory omission.** Dependency's §7 API inventory
  (`Dependency.ken.md:2725` onward) omits `topo_order_edge_antitone`,
  `edge` and `ordered_vertices` (Adversary `evt_3sjejdcgpkn12`). This
  candidate edits that inventory anyway, so it corrects them.
- **Seeds and tests.** Seeds go under `conformance/stdlib/graphs/`;
  tests are named `cat_graphs_<slug>.rs`. Spec 62 ends at §13
  (`:1250`), so the new section is §14.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:
- the adaptor and both laws, with names and placement as AC-0 rules
  them;
- the corrected §7 inventory;
- a §14 in spec 62;
- a closed conformance seed;
- a package test;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the names and placement, and confirms that no §1 to §13
  operation or law type changes.
- **AC-1.** §14 states the operation and both laws by their public
  types. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client builds a closed graph from
  successor lists on a three-vertex type, and checks one listed edge
  through the sound law and one through the complete law. Its refused
  block pins, in the same form as an accepted control, an edge claimed
  for a target absent from the source's list. A kernel that accepts it
  would show the law was not checked.
- **AC-3.** Every Graphs package test that checked before still checks.
  The new package test fails if the adaptor's edge is redefined so
  that it ignores the list.

## Stop conditions

- A law needs an Axiom, or a kernel or surface change.
- A change to an existing §1 to §13 operation or law type.

Checks: 4 fired: `MkGraph`, `edge`, `contains`, `contains_sound` and
`contains_complete` were read at the cited lines on `48cda6399`, and the
composition type-checks on paper in that order. 3 fired: `successor`
and `adjacency` were grepped over `catalog/`. Map's adjacency relation
(`Map.ken.md:15008`) is the one existing successor representation and is
named above; the Graphs hit is the unrelated Nat lemma
`successor_not_self_leq` (`:2319`).
