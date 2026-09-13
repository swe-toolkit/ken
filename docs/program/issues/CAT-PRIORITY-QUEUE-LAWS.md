---
id: CAT-PRIORITY-QUEUE-LAWS
title: "the deferred proof follow-on to CAT-PRIORITY-QUEUE: discharge the general kernel proofs the computational build ships as tested-only -- meld/insert/pop_min representation validity, entry multiplicity conservation (count_by p), and extract-min (global-minimum + nondecreasing drain) under the same fixed lawful order; introduce NO second queue, no Omega path carrier/postulate/new primitive; operator-released proof completion"
status: merged
owner: foundation
size: L
gate: none
depends_on: [CAT-PRIORITY-QUEUE]
blocks: []
github: null
tier: T1
origin: "Steward filed 2026-09-12 from the Architect priority-queue decomposition ruling (evt_15etn5hm102n2, Part C), grounded at main 1691160dd. The Architect ruled the chain SPEC-PRIORITY-QUEUE-CONTRACT -> CAT-PRIORITY-QUEUE -> a separately named CAT-PRIORITY-QUEUE-LAWS proof follow-on: the build ships a TESTED computational implementation with the semantic requirements load-bearing and covered but their GENERAL kernel proof deferred here. Filed now so the named follow-on is not lost. DEFERRED, not released: gated on an operator/Architect laws-tranche ruling (like CAT-REL-CLOSURE-LAWS behind the Band-A frontier-harvest sequence). Re-measure the CAT-PRIORITY-QUEUE anchors at release."
---

# The priority-queue proof follow-on

`CAT-PRIORITY-QUEUE` (the Band-A build node) ships `Data.Collections.PriorityQueue`
as a **tested computational implementation**: the semantic requirements (empty,
minimum, occurrence preservation, exactly-one removal, remainder validity,
persistence) are load-bearing and covered by acceptance tests, but their GENERAL
kernel proof is deferred to this node. The Architect ruled it a separately named
follow-on, not part of the build (evt_15etn5hm102n2, Part C). Filed so the
obligation is not lost. The operator released this tranche on 2026-09-13;
see Gating below.

## What this discharges (Architect Part C)

Over the SAME fixed lawful order the build uses (do NOT introduce a second queue):

1. **Representation validity, general.** `meld`, `insert`, and `pop_min` preserve
   the recursive leftist validity invariant (ordered children, root priority `<=`
   each nonempty child root, left rank `>=` right rank, cached rank `= Suc (right
   rank)`) -- proved, not just observed on fixtures.
2. **Entry multiplicity conservation.** Stated entry-wise via `count_by p` for
   arbitrary `p : k -> v -> Bool` (payloads need no equality instance): `merge`
   adds counts; `insert` adds exactly the predicate's 0/1 contribution; a
   successful `pop_min` decomposes the original count into the removed entry's
   contribution plus the remainder's. Do NOT replace entry conservation with
   key-set membership.
3. **Extract-min, general.** The root/`find_min` entry is a global minimum over
   the stored priorities under the lawful order (root's global-minimum property
   from comparator transitivity + the invariant); repeated `pop_min` drains in
   nondecreasing priority. State the exact propositions and their premises.

## Component design and acceptance

The Architect's [proof-strategy ruling](../CAT-PRIORITY-QUEUE-LAWS-design.md)
is the implementation design. Its exact general propositions, premises,
induction decomposition, shared proof dependencies, private finite-pop
observation and acceptance requirements are load-bearing here.

Keep the same six-name public queue surface. Queue predicates, measures,
observers and checked laws remain private; complete private proofs are intrinsic
package proofs, not an importable client proof API. No existing queue operation
may call a proof observer, validity traversal, size function, or fuel wrapper.
The design's small proof-only spillover to `Data.Numeric.Nat.Arithmetic` and
`Core.Classes.LawfulClasses` exposes/reuses shared laws rather than duplicating
them in the queue package.

Completion requires every stated general law, arbitrary-predicate conservation,
nonvacuous validity witnesses/refutations, and total drain at the structural
entry count, not just sortedness conditional on a completed trace. Verify that
representative defects in actual workers invalidate unchanged general proofs.
Preserve public privacy, same-order typing, persistence and cost evidence;
complexity proofs remain deferred. Local tests are targeted; whole-repo gates
run only in CI. The design's admission probes establish the named techniques,
not the full theorem suite, which Foundation must author and have checked.

## Trust posture and constraints

Proof-bearing data in `Type`, Boolean decidable predicates reflected into
`Omega`; no `Omega` path carrier, postulate, or new primitive; no `Axiom`, no TCB
entry, kernel-untouched. Machine-checked complexity bounds are a SEPARATE deferral
(not this node). Existing/imported assumption dependencies stay visible.

## Gating

RELEASED 2026-09-13 (Steward) on the operator ruling (Pat, this session): "A
catalog package is not finished until its proofs are complete ... computational
tests are not part of the package ... this is inherently inferior and weakens
the promise ken strives to make." That is the operator laws-tranche ruling this
node was gated on, and it makes the proof follow-on a completion REQUIREMENT for
the package, not an optional deferral. Both preconditions are met: (a)
`CAT-PRIORITY-QUEUE` merged (main 5d1347aaf) and (b) the operator ruling above.
The Architect remains the required reviewer and design authority for the proof
strategy. Re-measure every `CAT-PRIORITY-QUEUE` anchor at the cut.

## Not this node

- The computational implementation and its acceptance coverage -- that is
  `CAT-PRIORITY-QUEUE`.
- A second/alternate queue representation.
- Machine-checked complexity bounds.

## Sizing / tier

**Size L, tier T1.** General inductive proofs (validity preservation,
multiplicity conservation, extract-min) over the leftist carrier -- a genuine
proof-authoring tranche, not a drain.
