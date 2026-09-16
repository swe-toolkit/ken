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
hold this one for a one-line fix"* is asking the executor to un-act on an
authorization it already holds.

**MEASURED, because the first draft of this section asserted a mechanism I had
not checked.** It said the publisher "is polling for `ROUTED:`". It is not:

    grep -rn  'ROUTED' scripts/      ->  0     ANY extension, not just .sh/.py
    grep -rni 'routed' scripts/      ->  0     case-insensitive
    POSITIVE CONTROL, same tool and tree: scripts/scripted-pr-automerge.sh and
    scripts/publisher-gate-probes.sh are hit by another key -- so the grep
    REACHES the files that would matter. The zero is a measurement, not a miss.

⇒ **There is no string matcher anywhere. Detection is a reading.** The
lieutenant is an agent that reads the post and judges it, so there is no regex
to fail open and no prefix to get right — **and the failure is fuzzier than a
parser bug, not tidier.** A qualifier only works if the reader weighs it, and
the reader is the same one that did not act on two plain-language holds.

**The strongest evidence is not the absence — it is the executor's own standing
instruction.** `moot.toml:562`, the lieutenant's `startup_prompt`:

> *"…await the Steward's ROUTED authorization."*

⇒ **The concept the executor is told to wait for is the UNQUALIFIED one.** That
is a mechanism for the conclusion rather than merely the lack of a contrary
one: a qualified `ROUTED, HOLD FOR …` is not a different token to the seat
waiting on it, it is the thing it was told to await.

**SIX documents carry instructions about `ROUTED`, not two** (measured at this
file's own tree; records such as diary entries, issue/WP nodes and index rows
are excluded, since they report rather than instruct):

    agent/playbooks/federation/lieutenant.md                  6 sites
    agent/COORDINATION.md §14b                          :1172, :1192
    agent/playbooks/federation/steward.md                      :298
    agent/playbooks/federation/steward/merge-procedure.md :15, :244
    moot.toml                                                  :562

Measured twice in one hour, both times the Architect, both times correct:

    review asked to hold 9312aeab3   -> landed uncorrected at 235b49683
    review asked to hold 577444432   -> landed as the +33/-0 shape at 2a0b1cf83

**Both defects were real, both were named before the landing, and both became
follow-up corrections to `main` instead** — including one the reviewer had
explicitly predicted would become *"a `-N` correction to a memory file."*

⇒ **The tell for a REVIEWER:** if your finding needs to stop a landing, address
the **router**, not the executor, and say *"withdraw the routing"* rather than
*"hold"*. The router is the only seat that can un-authorize.

⇒ **The tell for the ROUTER, and it is the load-bearing one: DO NOT POST
`ROUTED:` AT ALL until the review you expect has cleared.** The window between
your authorization and the merge is exactly when reviews arrive, and you cannot
recall it.

**A qualified routing — `ROUTED, HOLD FOR <reviewer>` — is NOT the fix, and
this file's first draft recommended it.** It rests on the executor reading a
qualifier and weighing it correctly, which is the same fallible channel that
swallowed both holds above. **Adding words to a message that was already not
acted on is not a mechanism.** Withholding the authorization is, because an
absent `ROUTED:` cannot be misread.

Use a qualified routing only to say *"this exists and is gated"* for
coordination; never treat it as protection. If it is not ready to merge,
**the safe state is that no authorization exists.**

**Do not read this as "the executor erred."** It executed exactly the
authorization it was given, which is the protocol working. The defect is that a
live authorization had no withdrawal mechanism, and that belongs to the router.
