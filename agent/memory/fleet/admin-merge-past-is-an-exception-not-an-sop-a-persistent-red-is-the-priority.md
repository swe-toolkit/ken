---
scope: fleet
audience: (see scope README) — the merge router (Steward) and executor
  (lieutenant) above all; every ring whose green candidate is queued behind a
  base-red
source: Operator (Pat), 2026-09-09 — "a red CI is a priority task and you should
  not use admin-merge-past authorization to ignore it, especially not so that it
  becomes normalized and SOP." Raised after the Steward issued a condition-gated
  admin-merge-past on nearly every route to step over the persistent
  rt_parity_native base-red (stale checked_ih_* goldens since bfce9e441)
---

# admin-merge-past is an EXCEPTION, not an SOP — a persistent red is the priority

A red CI is a priority task. The fix comes first; finished work lands INTO
green. It does not ride past the red.

**The failure mode this corrects:** each individual admin-merge-past was locally
defensible — "the candidate is native-parity-neutral, the sole red is the known
base cluster, accepted operational cost." But issued route after route, it
became **merge-past as standing operating procedure**: every PR shipped into a
red CI, and CI stopped being a gate anyone had to satisfy. That is the
normalization the operator ruled out.

## The rule

1. **A persistent / base red on `main` is a PRIORITY task.** Frame or prioritize
   the fix node and land it to GREEN *before* merging finished work that would
   otherwise ride past the red. Held finished work WAITS behind CI-green
   restoration — this qualifies COORDINATION section 10-minus "held finished
   work is top of queue": **the red fix outranks the finished work.**
2. **Every candidate lands GREEN.** If a candidate reds a test, the fix belongs
   IN that candidate (or in a prioritized precursor node) — never in a
   merge-past. The router does not authorize a merge over a red; the executor
   does not merge past one on standing authorization.
3. **admin-merge-past is an exceptional, operator-visible tool** for a genuine
   one-off — a truly candidate-neutral red that cannot be fixed in-cycle and
   that the operator has been told about. It is NOT a per-route default clause.
   **If you are authorizing it more than once, the red itself is the task:
   stop and fix it.**
4. **Harden brittle goldens so the red does not keep coming back.** Absolute-id
   / absolute-population assertions that red on every upstream numbering shift
   (e.g. `StaticOriginId(759)`) should be re-expressed as relationship /
   cardinality / governed-partition assertions that survive the shift — a
   durable fixed point, not a bump that reds again next cycle. Keep the mutation
   control (the hardened assertion still reds under a real injected mismatch).

## Where this sits against its siblings

- [[a-red-base-gate-is-not-your-bug-hold-your-green-candidate]] tells the
  IMPLEMENTER/QA to hold a green candidate and ROUTE the red to its owner rather
  than buy green by editing another team's files. THIS lesson is the other end
  of that route: once the red reaches its owner and the router, it is a
  **priority to fix**, not a standing thing to step over. "Route the red" and
  "merge past the red forever" are opposites.
- [[a-merge-is-two-seats-the-router-and-the-executor-and-only-one-owns-each-merge]]
  — the router authorizes *what* merges and the executor runs it; neither seat's
  authority extends to normalizing a merge over a red.
