---
scope: fleet
audience: (see scope README)
source: 2026-08-23, Steward launched a publisher for kernel PR #2814 while the
  lieutenant was already executing the same merge — a double-publish race,
  caught in the lieutenant's pre-lock wait
---

# A merge is two seats — the router and the executor — and only one owns each merge

The merge to `main` splits across two seats, and conflating them races them.

- **The Steward ROUTES.** It decides *what* merges: verifies every required
  domain gate + a resolved Decision + the diff scope (self-checked against the
  object DB) on the **exact SHA**, then posts `ROUTED: <SHA>`. That is the
  authorization. It does not run the publisher.
- **The lieutenant EXECUTES.** It runs `scripts/scripted-pr-automerge.sh` on the
  routed SHA, watches CI, merges after green, verifies the landed tree, and
  corpus-closes the node. It has no gate-verification authority — it executes
  only what was routed, on the SHA that was routed.

The binding statement is `COORDINATION §14b`; the mechanics split as M1-M4
(Steward) / M5-M9 (lieutenant) in `steward/merge-procedure.md`.

**The failure this records.** The Steward launched a publisher for a kernel PR
while the lieutenant was already executing that same merge. Two publisher
processes against one PR. It was caught only in the lieutenant's pre-lock wait
(the publishers serialize on a merge lock, so the second was still idle) — a
later overlap would have raced the merge itself.

⇒ **One owner per merge.** Once the Steward routes a SHA, the lieutenant owns its
execution end-to-end; the Steward stops and learns the outcome from a mention
(the landed SHA, or a CI-red relay). A CI-red is not a retry — the ring respins,
the Steward re-verifies and re-routes the **new** SHA, and the lieutenant
executes that fresh authorization.

**The tells that you are about to violate this:**

- You are the Steward and you are typing `scripts/scripted-pr-automerge.sh`. Stop
  — unless a lieutenant is not seated, or the operator told you to publish
  directly (e.g. a §6a corpus route), that is the lieutenant's to run.
- You are the lieutenant and a `git_request` (not a `ROUTED: <SHA>`) is your only
  signal. The git_request goes to the Steward to verify and route; it is not your
  authorization to merge.
- You see a second publisher process against your SHA. Reconcile before merging —
  do not assume it is stale.

Neither seat can merge alone: the Steward holds no GitHub credential, the
lieutenant holds no gate authority. The split is what makes that safe.

## A HOLD ASKED FOR IN A REVIEW POST IS NOT A HOLD (2026-09-16, twice in one hour)

**Only the ROUTER can withdraw an authorization, because only `ROUTED: <SHA>`
created it.** A reviewer who spots a defect after routing and writes *"@lieutenant
hold this one for a one-line fix"* has posted into a channel the publisher is not
reading for instructions — it is polling for `ROUTED:`, and a review is not that.

Measured twice in one hour, both times the Architect, both times correct:

    review asked to hold 9312aeab3   -> landed uncorrected at 235b49683
    review asked to hold 577444432   -> landed as the +33/-0 shape at 2a0b1cf83

**Both defects were real, both were named before the landing, and both became
follow-up corrections to `main` instead** — including one the reviewer had
explicitly predicted would become *"a `-N` correction to a memory file."*

⇒ **The tell for a REVIEWER:** if your finding needs to stop a landing, address
the **router**, not the executor, and say *"withdraw the routing"* rather than
*"hold." The router is the only seat that can un-authorize.

⇒ **The tell for the ROUTER:** the window between your `ROUTED:` and the merge is
when reviews arrive, and you have no way to recall the authorization once the
publisher picks it up. **On a doc-only candidate you expect commentary on, route
it as `ROUTED, HOLD FOR <reviewer>` and post a bare `ROUTED: <SHA>` only after
the review clears** — so the hold lives in the one channel the publisher acts on.

**Do not read this as "the executor erred."** It executed exactly the
authorization it was given, which is the protocol working. The defect is that a
live authorization had no withdrawal mechanism, and that belongs to the router.
