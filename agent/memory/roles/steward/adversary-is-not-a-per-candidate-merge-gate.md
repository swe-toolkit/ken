---
name: adversary-is-not-a-per-candidate-merge-gate
description: There is no "Adversary §10a" merge gate; dispatching the adversary to hunt a candidate/respin violates the report-only edge and is the token sink §10⁻a exists to prevent.
metadata:
  type: feedback
---

The merge gate set for a code candidate is **QA + CV + Architect**. The
adversary is NOT a fourth approval, and there is no `§10a` gate anywhere in the
corpus (`steward.md §6`, `merge-policy.md`, `COORDINATION.md`).

**Do not post `@adversary re-gate exact <SHA> ... hunt the delta ... return a
verdict`.** That request violates `COORDINATION §10⁻a` (the edge is report-only:
"do not ask the adversary to hunt something — a request for an attack is a
conversation the Steward does not make") and is the exact token sink the law was
written to prevent: the only sanctioned adversary compaction is the M8 merge
notification (`§15`), so a per-candidate / per-respin hunt has no compaction seam
and a rejected or respun candidate never reaches one — the hunts accumulate in
one context.

**Why:** measured 2026-08-24, this seat treated "Adversary §10a CLEAN" as a
required merge approval and dispatched it on every candidate and every respin
(three stacked hunts on one Component B partial) before the operator flagged the
cost and ruled: align to the law.

**How to apply:** the adversary hunts autonomously and reports on its one inbound
edge; the Steward's only outbound traffic is the M8 merge notification, which
compacts it first. Before posting anything to the adversary, confirm it is that
notification. Resolve merge Decisions on QA + CV + Architect only. The
enforcement guard lives in `steward.md §6`.
