---
id: SPEC-PRIORITY-QUEUE-CONTRACT
title: "no priority-queue/heap ADT is specified anywhere in spec/ (every 'heap' hit is the runtime value-store); the Band-A frontier-harvest build CAT-PRIORITY-QUEUE cannot proceed without a normative contract, so the spec enclave authors a short dedicated priority-queue contract: the six-name abstract interface (carrier + empty/insert/find_min/pop_min/merge over an explicit Ord k, payload separate), lawful-order + multiplicity (entries not sets, duplicates preserved), optional extraction, persistence, equal-priority tie NON-stability, recursive representation validity (without freezing the private leftist layout), an honest structural cost model, and the proof-status split (computational now, general laws deferred), with a reaching conformance seed"
status: ready
owner: spec
size: S
gate: none
depends_on: []
blocks: [CAT-PRIORITY-QUEUE]
github: null
origin: "Steward cut 2026-09-12 from the Architect priority-queue design ruling (Parts A/B/C/D: evt_3vfd1feghmm2c, evt_15etn5hm102n2), grounded at main 1691160dd. It is the first node of the Band-A frontier-harvest priority-queue tranche (operator 2026-09-12 named priority-queue/heap as the next L3 node after CAT-REL). The Architect ruled spec-contract-FIRST, then one complete computational node: 'Yes: SPEC-PRIORITY-QUEUE-CONTRACT before CAT-PRIORITY-QUEUE.' New normative library ADT, NOT a clarification of Map. IN-LANE SCOPE CALL (Steward, on the Architect's explicit deferral 'Steward retains the Band-A scope call' + operator confirmation 2026-09-12 that planner/library surface of this kind is fleet-owned, not operator-gated): bounded standard-library surface, no new language/kernel mechanism, no new trust-root, no TCB -- fenced, spec-enclave-owned, no operator sign-off. Analogous to SPEC-REL-CLOSURE-RECURRENCE -> CAT-REL-TRANSITIVE-CLOSURE."
---

> # SPEC CONTRACT owed to the enclave (Architect design ruling, Parts A-D).
>
> The Architect designed the priority-queue ADT and ruled the contract must land
> BEFORE the build node `CAT-PRIORITY-QUEUE` (which is `draft`, `depends_on` this,
> and flips `ready` when this merges). This node states WHAT the contract must
> settle; the enclave authors the normative text and the reaching conformance
> seed. Grounded at main `1691160dd`. Re-measure spec anchors at the cut.

## What this is

There is no priority-queue or heap ADT specified anywhere in `spec/` (measured at
`1691160dd`: every "heap" occurrence is the runtime content-addressed value store;
every "priority" is prose). The Band-A frontier-harvest build needs a normative
contract to build against, exactly as `CAT-REL-TRANSITIVE-CLOSURE` needed
`SPEC-REL-CLOSURE-RECURRENCE`. The Architect designed the ADT (a persistent
leftist priority queue) and stated the contract's obligations; the enclave writes
the spec.

## Placement (enclave's call, with one constraint)

Spec owns placement: a short dedicated priority-queue contract, or an explicitly
delimited collection section. Do NOT append it beneath chapter 57's inherited
"every law proved" template -- current 57 has a materially different proof posture
and stale spelling/status claims, and this ADT ships computational-first with
general laws deferred. No whole-chapter rewrite or broad cleanup rides along.
Detailed leftist machinery belongs in the component design (`CAT-PRIORITY-QUEUE`),
NOT the normative contract: the public behavior must not freeze every private
tie/layout choice.

## What the contract must settle

1. **The six-name abstract interface.** An abstract carrier and exactly five
   public operations; `Q` abbreviates the queue type, `E` the existing
   `Pair k v` (abbreviations, not extra declarations):
   - `empty  k v d : Q`
   - `insert k v d : k -> v -> Q -> Q`
   - `find_min k v d : Q -> Option E`
   - `pop_min k v d : Q -> Option (Pair E Q)`
   - `merge  k v d : Q -> Q -> Q`

   Publish precisely the carrier plus these five. No partial extraction,
   fabricated default, delete-only API, public mutable handle, arbitrary-key
   deletion, decrease-key, stable-ordering promise, or bulk-construction family.

2. **Lawful order, payload separate.** Require `Ord k` (the priority), NOT
   `Ord v` and NOT `Ord (Pair k v)` -- comparing only the first component of a
   pair dictionary violates that dictionary's antisymmetry when distinct payloads
   share a priority. Queues are bound to their comparator through the ordinary
   function-valued type parameter: operations take an explicit `d : Ord k` and
   operate on `Q = PriorityQueue k v (ord_leq_at k d)`. This makes merging
   opposite-order queues a type error while keeping definitionally-equal
   comparators usable without demanding identical proof records. No runtime
   comparator-identity token or registry.

3. **Entries, not sets; multiplicities preserved.** Every key/payload occurrence
   is preserved, including duplicates and distinct equal-priority payloads.
   State the conservation law in a payload-equality-free form: `count_by p` for
   arbitrary `p : k -> v -> Bool` -- `insert` adds exactly the predicate's 0/1
   contribution; `merge` adds counts; a successful `pop_min` decomposes the
   original count into the removed entry's contribution plus the remainder's.
   Do NOT quietly replace entry conservation with key-set membership.

4. **Minimum + optional extraction.** `find_min`/`pop_min` on empty are `None`.
   `find_min` agrees with the entry a successful `pop_min` returns. `pop_min`
   returns the removed entry AND the remainder together (never partial). A
   minimum law quantifies over the stored priorities under the supplied lawful
   order.

5. **Tie non-stability.** Equal-priority extraction is explicitly NOT
   stable/FIFO; a merge tie bias may pick the first root without creating a
   stability promise. State this so no client depends on equal-priority order.

6. **Recursive representation validity + persistence.** Operations produce and
   return valid queues (the remainder of a `pop_min` is valid); the structure is
   persistent (originals survive their use in `merge`/`pop_min`). State validity
   as an abstract property of a produced queue -- do NOT freeze the concrete
   leftist layout here (that is the component design).

7. **Honest cost model.** Logarithmically many heap-node steps / priority
   comparisons for valid operands, with `merge` bounded by the two right spines.
   Do NOT treat unary `Nat` rank comparisons or arbitrary priority comparison as
   constant-time, and do NOT claim measured native/wall-clock O(log n). Structural
   account only.

8. **Proof status.** The build ships a TESTED COMPUTATIONAL implementation: the
   semantic requirements (empty, minimum, occurrence preservation, exactly-one
   removal, remainder validity, persistence) are load-bearing and covered by
   acceptance tests, but their GENERAL kernel proof is deferred to a named
   follow-on (`CAT-PRIORITY-QUEUE-LAWS`). State that a checked abstract data
   declaration does NOT by itself prove the algorithms preserve their invariant;
   an `Ord` dictionary certifies the order laws, not the queue algorithms. No
   `Omega` path carrier, postulate, or new primitive; Boolean decidable
   predicates reflect into `Omega`, proof-bearing data stays in `Type`.

## Deliverables

1. The normative contract text (placement per the enclave), settling items 1-8.
2. A reaching conformance seed that a discriminating implementation passes and a
   non-conforming one reds -- shaped by the `CAT-PRIORITY-QUEUE` discriminators
   (Architect Part D: public construct/drain through real exported identities;
   empty/singleton/removal; order-not-insertion-position with two lawful orders +
   a mismatched-order merge refusal; entries-not-sets with `merge q q` doubling;
   meld + persistence with an independent reference multiset; rank/shape validity
   independent of sorted output). The seed states expected observations without
   computing them from the implementation under test.

## Acceptance criteria

- The contract settles all eight items above, is placed without a chapter-57
  append or whole-chapter rewrite, and freezes no private leftist layout choice.
- It requires `Ord k` with payload separate, and the comparator-parameter binding
  that makes an opposite-order merge a type error.
- Conservation is stated entry-wise (`count_by p`), never as key-set membership;
  tie non-stability is explicit; the cost model is structural and honest; the
  proof-status split (computational now, general laws deferred) is explicit.
- A reaching conformance seed exists whose expected values are independent of the
  implementation under test.

## Not this node

- The implementation (carrier inductive, `make_node`, meld, the five ops) -- that
  is `CAT-PRIORITY-QUEUE` (`draft`, flips `ready` on this merge).
- The general validity/conservation/extract-min PROOFS -- that is
  `CAT-PRIORITY-QUEUE-LAWS` (`draft`, deferred).
- Any new language/kernel mechanism, TCB entry, or trust-root: this is bounded
  normative library surface.

## Sizing / tier

**Size S, tier T1.** A short contract, but T1: it fixes the ADT's semantics
(payload-separate lawful order, entry-wise conservation, tie non-stability, the
computational/deferred proof split) that the build and its laws depend on.
Spec-enclave-owned; the Architect is the design authority and a reviewer.

## Contention

Spec enclave, `spec/50-stdlib/`. No cross-lane contention (L1 runtime on
`crates/`; L2 language on `crates/ken-elaborator`; this is `spec/` only).
Re-measure any cited `spec/` anchor at the cut.
