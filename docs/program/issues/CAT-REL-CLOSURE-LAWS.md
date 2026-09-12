---
id: CAT-REL-CLOSURE-LAWS
title: "the deferred general-relation-law follow-on to CAT-REL-TRANSITIVE-CLOSURE: spec 58 §7 C-scope names three residual proof obligations left explicitly deferred by the landed four-function computation -- (1) the general compose/converse membership characterizations, (2) the concrete predicate proof-flip discriminators (is_transitive and friends inhabited on a completion, refuted on a non-transitive relation), (3) the closure faithfulness/saturation laws (reachable_plus faithful, not an approximation, under a lawful ordered key); the landed node deliberately excludes all three, and this node lands them once the operator/Architect authorize a laws tranche"
status: draft
owner: foundation
size: L
gate: none
depends_on: [CAT-REL-TRANSITIVE-CLOSURE]
blocks: []
github: null
origin: "Steward filed 2026-09-12 when CAT-REL-TRANSITIVE-CLOSURE landed (origin/main 1691160dd), honoring the named fast-follow commitment in that node (its 'Not this node' section: 'the faithfulness/saturation LAWS ... the deferred fast-follow (CAT-REL-CLOSURE-LAWS, filed when this lands)'). Spec-grounded, not aesthetic (steward §4c): spec/50-stdlib/58-maps-sets-relations.md §7 C-scope explicitly states the landed/deferred split and names these three residual clusters as 'the separate general-relation-law follow-on, not ... the landed computation' (58:394-396). DEFERRED, not released: it sits behind the Band-A frontier-harvest sequence (priority-queue/heap, keyed-record/association, Vec zip/lookup) and needs an operator/Architect ruling to open a laws tranche -- the faithfulness proof is a T1 design question (the convoy idiom + Gap-A route-around over the bounded-Bool Omega encoding), not a mechanical drain. Anchors re-measured 2026-09-12 at 1691160dd (post the +486 §7 reorg the CAT-REL frame warns about)."
---

# The general relation-law follow-on (CAT-4, Fork B residuals)

`CAT-REL-TRANSITIVE-CLOSURE` (merged `1691160dd`) landed the four public
closure functions -- `size`, `dom`, `reachable_within`, `reachable_plus` --
as the kernel-untouched, `Axiom`-free **computation**, and deliberately left
the **laws** deferred. `spec/50-stdlib/58-maps-sets-relations.md §7` states this
landed/deferred split explicitly (the "C-scope" block, 58:363-396); the three
residual clusters below are what "the separate general-relation-law follow-on"
(58:394) refers to. This node lands them when a laws tranche is authorized.

This is a NAMED, spec-grounded obligation, filed so it is not lost -- not a
release. It is deferred behind the Band-A frontier-harvest sequence and gated on
an operator/Architect ruling to open a laws tranche (see "Gating" below).

## The three deferred residual clusters (spec 58 §7 C-scope)

1. **General compose/converse membership characterizations** (58:383-388). The
   normative statements are
   `succ x (compose R S) = ⋃ { succ y S : y ∈ succ x R }` and
   `y ∈ succ x R ⇔ x ∈ succ y (converse R)`. No corresponding general proof is
   present in the landed D4 producer. The existing runtime smoke tests one
   positive observation each (composition contains `1 → 3` from `1 → 2`,
   `2 → 3`; converse contains `2 → 1` from `1 → 2`) but does **not** test an
   absent composed edge or the unreversed direction of converse. The paired
   positive/negative membership cases remain unexecuted conformance obligations.

2. **Concrete predicate proof-flip discriminators** (58:389-393). The property
   predicates (`is_reflexive`, `is_symmetric`, `is_transitive`,
   `is_equivalence`) are transparent `Π`-into-`Ω` definitions and landed, but
   their discriminating obligations are not constructed. In particular a
   non-transitive `Nat` relation (`a → b`, `b → c`, no `a → c`) must **fail**
   `is_transitive`, while its transitive completion must **inhabit** it. The
   transparent predicate declaration and the suite's zero-delta census construct
   neither proof-flip arm.

3. **Closure faithfulness / saturation laws** (58:422-427, :483-492). Under the
   well-formedness premise that one lawful (reflexive, transitive,
   antisymmetric, total; §2) key order is used throughout -- the outer relation
   tree `Ordered k (Tree k Unit) leq` and every stored successor tree
   `Ordered k Unit leq` under the same comparator -- `reachable_plus` is
   **faithful, not an approximation**: bounded reachability at fuel
   `N := size (dom R)` equals full transitive closure (monotone in the fuel,
   saturates by that bound). The landed node states this in prose and pins the
   `N` (not `N − 1`) bound computationally, but proves no faithfulness/saturation
   theorem. The well-formedness premises are premises of the correspondence
   laws, not function parameters -- on arbitrary raw trees no correspondence is
   promised (58:428-433).

## Why this was split out (not a weakening of the landed node)

Spec 58 §7 is explicit that these residuals "are not silently credited to the
four-function closure build and do not weaken its contract" (58:395-396). The
computation is sound and self-contained as landed: the abstract surface exposes
no public `Tree` constructor, so no client can launder a false reachability
claim, and the deferral is a soundness-safe direction (the Adversary confirmed
this at `evt_42dpj4cfwgj25` -- "faithfulness deferred + no public Tree ctor =>
nothing launderable"). Laws add trust where the computation composes; they do
not repair a hole.

## Sizing / tier

**Size L, tier T1.** Cluster 3 (faithfulness/saturation) is a genuine design
question: proving a bounded-`Bool` `Ω`-encoding faithfully realizes closure via
the convoy idiom + the Gap-A route-around (58 §1 point 2) over the landed
adjacency representation, under the lawful-order premise. Clusters 1-2 are
conformance-obligation discharges (membership proofs, proof-flip arms) that are
smaller but still law-authoring, not drains. The Architect is the required
reviewer and the relations-frontier design authority; a laws tranche likely
decomposes into per-cluster increments.

## Gating

DEFERRED. Do NOT release without:

- an operator/Architect ruling to open a **laws tranche** on the catalog
  campaign (the current Band-A tranche is computational-first: "lawful zero-delta
  where it composes" -- laws are the natural next depth, but their sequencing
  against Band A / Band B is an operator call); and
- the Architect's decomposition/design input for cluster 3's faithfulness proof
  (T1 design authority; this node carries no proof strategy yet).

Re-measure every `58:XXX` anchor and every `Map.ken.md` §4.7.12 anchor at the
cut before release; the §7 body has already been reorganized once (+486) since
the CAT-REL frame was written.

## Not this node

- The landed computation (`size`/`dom`/`reachable_within`/`reachable_plus`) --
  that is `CAT-REL-TRANSITIVE-CLOSURE`, merged.
- Any raw `data ... : Ω` closure inductive -- forbidden by Fork B; the laws are
  proved over the bounded-`Bool` `IsTrue` encoding, not a proof-relevant
  path-witness inductive.
