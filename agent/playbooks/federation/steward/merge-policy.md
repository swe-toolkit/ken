---
name: steward-merge-policy
description: Steward merge policy — what may land on main and when, as distinct from how to publish it. Covers the accepted-base rule, partial-WP merging, semantic atomicity, and finding the green cut boundary.
scope: federation
---

# Merge policy: what lands on `main`, and when

Steward task procedure. Read at the point of use. Governing playbook:
`../steward.md`. The mechanics of publishing — the nine-step M1-M9 gate — are
`merge-procedure.md`; this file is the decision that precedes it.

Ask both questions at every release and every review vote.

## THE STEWARD OWNS MERGE TIMING. THE ARCHITECT OWNS WORK QUALITY.

Operator, 2026-08-13, verbatim: *"Architect's domain covers the quality of the
work done, not the timing of when it hits main. You own git, and my
instructions to you are not to have long-running branches. There is some nuance
around recuts, but in general, we should recut at a seam with green CI and merge
that to main, whether or not it comprises a complete work package. Keep in mind
that the entire project is incomplete, and the incompleteness of a work package
is immaterial to whether the code is in main."*

**This outranks everything below it in this file.** Where the sections below
make *ring acceptance* the bar, this makes **a seam with green CI** the bar.

> **The default is: recut at a seam with green CI and merge. Completeness of the
> work package is IMMATERIAL. The whole project is incomplete.**

**The failure this exists to stop, measured 2026-08-13.** A ring declared its
object *"non-candidate, unrouted, no partial to QA"*, and the Architect had
ruled the node an **atomic object**. The Steward read both as a merge
prohibition and held. Neither is one:

| the input | what it actually governs | what it does NOT govern |
|---|---|---|
| ring says "non-candidate" | whether the unit is a complete deliverable ready for review | whether the tree may sit on `main` |
| Architect rules "atomic object" | what the **node** must contain before it **closes** | where intermediate commits live |
| QA has not reviewed it | whether the WP is **done** | whether inert code may land |

⇒ **"Not accepted" and "not fit for `main`" are different properties, and only
the first is the ring's to declare.** The cost of conflating them was a
25-commit branch across six hours and three rebases onto moving bases — three
chances to lose work, for no benefit, on an object that was **inert on `main`**
(its gate flag false and its population empty, so it could not execute).

**Do not ask the Architect whether something may merge.** Ask it whether the
work is right. Timing is not its call and offering it the question invites an
answer that reads as authoritative and is out of its lane.

**The nuance the operator left open is recuts, not holds.** Finding the seam is
judgment — see *Prefer a cut that is a straight ancestor* below, and
*Semantic atomicity is the one genuine bar*, which survives because it is about
a cut that would put `main` in a **self-inconsistent** state, never about a WP
being unfinished.

## A team's accepted base belongs on `main`

Operator, 2026-08-06: *"if a team is working from a committed, accepted base,
that base should be merged to main."*

**Standing policy, and it is the structural fix for the long-branch pathology.**
The moment a ring is building *on top of* a base that is committed and accepted,
that base has stopped being a candidate and started being the floor everyone
stands on. **Leaving it unmerged does not keep it under review — it keeps `main`
behind reality and makes every later cut larger.**

Ask it at every release and every review vote: *what base is this ring building
on, and is that base on `main`?* If the answer is no and the base is accepted,
**land the base**, then let the ring continue on top of it.

**This subsumes the "unmerged accumulation" failure and is cheaper than
detecting it.** A 212-commit branch, and a backlog of eight open PRs nobody was
triaging, were the same defect seen from two directions: work that was finished
and accepted but never landed, because attention stayed on the unit in front of
the Steward. **Neither was caught by watching the branch; both are prevented by
watching the base.**

## The CI gate is not a reason to hold an accepted base

**Corollary to the rule above.** Operator, same ruling: *"Adjust tests as
necessary to clear CI gates (mark skip, add comment — it's light technical
debt that enables a more sane git history)."*
A failing test on an accepted base is **skipped with a comment naming its exact
signature and an owning node**, not a reason to leave the base off `main`.
**Every skip needs an owner**; a skipped row measures nothing, so the node that
owns it owns un-skipping it.

## Merge accepted work as soon as it is done, even a partial WP

Operator, 2026-08-06: *"From now on, merge in accepted work once it is done,
even if it is only a partial WP."*

**WP closure and merge are separate events.** A WP closes when its ACs are met;
a completed portion merges as soon as it is accepted. Do not hold a finished
deliverable because its siblings are unfinished. The unit of merge is
**accepted work**, not the WP.

The bar is *accepted*, not *finished-looking*: reviewed and cleared on its own
terms. Its deliverables must be complete — landing a deliverable's first half
is not partial-WP merging, it is shipping a fragment.

> **AMENDED 2026-08-13 — read the top section first; this paragraph is the one
> that misled.** The two sentences above describe the bar for calling work
> **accepted**. They are **not** a gate on merging, and reading them as one is
> what produced the 25-commit branch. **A seam with green CI merges whether or
> not anything has been "accepted" and whether or not a deliverable is whole.**
> "Shipping a fragment" is a real caution about *claiming a deliverable is
> delivered* — it is not a reason to keep a green tree off `main`.

## "Reviewed" and "releasable" are different properties

**The cut needs both.** Checkpoint reviews bind exact SHAs *for a deliverable's
own claim* — they do not assert the tree is green at that commit, because
nobody ever asked them that. **A prefix every one of whose commits carries a
live approval can still be red.** Before publishing any cut, establish that the
target is green **and** that `main` is green, so a red is attributable to the
cut rather than inherited. CI is the instrument; the local `--workspace` run is
still banned (§12).

> **Measured 2026-08-06, one hour after the rule above was written.** I cut
> `RT-DECL-CLOSURE-PORT` at `fc758323` — 34 commits, `D1`-`D6a`, every
> deliverable complete and reviewed — and published it. **Eight CI checks
> failed** on a shared `ken native trap: malformed borrowed process input`
> signature. `main` was green at `3015aafd`, so the prefix introduced it. PR
> closed, branch deleted.
>
> ⇒ **A contiguous, complete, fully-reviewed prefix is not thereby a
> releasable one.** The node-id boundary I cut on is a *bookkeeping* boundary;
> greenness is a *semantic* one, and they do not have to coincide. Find the
> green boundary by measurement before you publish, not by inference from the
> deliverable table.

## Semantic atomicity is the one genuine bar

**The one genuine bar is semantic atomicity.** Where two deliverables are a
declared atomic pair — one regresses alone, or the other has no reaching
witness alone — they land together. That is a property of the work. Nothing
else qualifies: not "the branch is the evidence chain", not "a rebase would
cost re-anchoring", and above all not "a currently-working path would go red"
(operator, 2026-07-28 — Ken has no users).

## Prefer a cut that is a straight ancestor

**Prefer a cut that is a straight ancestor.** A contiguous prefix cut at an
existing commit preserves every exact SHA and every review verdict below it
with no rebase, so verdict transfer is paid only on what remains in flight.
Look for that boundary before concluding a split is expensive.

> **Measured 2026-08-06, and this is why the rule exists.**
> `RT-DECL-CLOSURE-PORT` reached **203 unmerged commits over three days, three
> node ids on one ref, with six nodes blocked behind it.** Its `D1`-`D6a`
> prefix — 34 commits, complete, cuttable at an ancestor with zero rebase —
> had been pushed to origin two days earlier and simply never had a PR opened
> against it.
>
> The branch grew because every split proposal was refused with *"a rebase
> destroys the preserved exact SHAs."* **That is false as a git claim.** Each
> refusal then produced a new deliverable label inside the same branch
> (`D3b`, `D6a/b/c`, `D8n/o/p`, `D9a/b`) instead of a mergeable node. **A recut
> whose output cannot merge independently is a subdivision, and subdividing a
> held branch makes it longer, not smaller.**
>
> ⇒ **The tell is a recut that produces labels rather than merges.** If you
> have recut a node more than twice and nothing has landed, the recuts are not
> working and the next one will not either.

