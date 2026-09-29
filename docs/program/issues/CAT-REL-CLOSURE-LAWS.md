---
id: CAT-REL-CLOSURE-LAWS
title: "Prove the relation laws spec 58 §7 defers from the landed closure computation: the compose/converse membership characterizations, the relation-predicate proof-flip discriminators, and reachable_plus faithfulness and saturation under a lawful key order; clusters 1-2 as separate increments first, cluster 3 behind an AC-0 design stop the Architect rules"
status: ready
owner: foundation
size: L
gate: architect
tier: T1
depends_on: [CAT-REL-TRANSITIVE-CLOSURE]
blocks: []
github: null
origin: "Architect L3 runway ruling 2026-09-29 (evt_5f1ewknxv3m6h): opens this laws tranche, second in the L3 proof backfill after CAT-ORD-INT-AXIOM-NAMING. Operator 2026-09-13: 'A catalog package is not finished until its proofs are complete' and 'schedule the proof backfill before extending the catalog.' Spec 50-stdlib/58-maps-sets-relations.md §7 C-scope. Filed 2026-09-12 when CAT-REL-TRANSITIVE-CLOSURE landed (1691160dd); reframed by the Steward per COORDINATION section 2."
---

# Relation closure laws

## Objective

The relation surface in `Map.ken.md` proves the laws spec 58 §7 names as
the deferred follow-on to the landed closure computation (`size`, `dom`,
`reachable_within`, `reachable_plus`; merged `1691160dd`).

## Fixed inputs (read at `e5cd36c54`)

- **The residuals** are in spec 58 §7 C-scope (`:378-411`), with
  faithfulness at `:498`:
  1. **Membership.** `succ x (compose R S) = ⋃ { succ y S : y ∈ succ x R }`
     and `y ∈ succ x R ⇔ x ∈ succ y (converse R)`. No general proof exists.
     The smoke tests cover no absent composed edge and not the unreversed
     direction of converse.
  2. **Predicate proof-flips.** A non-transitive `Nat` relation (`a → b`,
     `b → c`, no `a → c`) fails `is_transitive`, and its completion inhabits
     it. Neither arm is constructed.
  3. **Faithfulness and saturation.** Under one lawful key order throughout,
     `reachable_plus` at fuel `size (dom R)` equals the full transitive
     closure, monotone in fuel. This is stated in prose only.
- **Encoding.** The laws are proved over the bounded-`Bool` `IsTrue`
  encoding. No raw `data … : Ω` closure inductive (Fork B). The
  well-formedness premises are law premises, not function parameters.
- The package is `catalog/packages/Data/Collections/Map.ken.md`, which is
  high-contention; re-measure its relation section at the cut.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Checked theorems in the relation section for all three clusters, delivered
as increments.
- Clusters 1 and 2 first, as one or two increments, each landable alone.
- Cluster 3 only after its AC-0 ruling.
- No computation, kernel or trust change.

## Acceptance

- **AC-0 (design stop for cluster 3; no proof built first).** Propose the
  faithfulness route: the convoy idiom plus the Gap-A route-around (spec 58
  §1 point 2) over the bounded-`Bool` Ω encoding, under the lawful-order
  premise. The Architect rules before any cluster-3 proof. Clusters 1 and 2
  do not wait on it.
- **AC-1.** Each cluster's theorems check, with zero `trusted_base()`
  growth. The cluster-2 pair holds both arms: `is_transitive` is refuted on
  the three-edge relation and inhabited on its completion.
- **AC-2 (controls).**
  - Cluster 1: an absent composed edge and the unreversed converse
    direction are each refuted.
  - Cluster 3: a fuel of `size (dom R) − 1` fails the saturation theorem on
    a witness chain.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A cluster needs a computation change to be provable. Stop to the
  Architect with the site.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
