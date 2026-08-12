---
name: a-carried-obligation-gated-on-a-merge-event-fires-on-an-accepted-partial
description: >-
  A deferred control gated on "when node X lands" assumes land implies
  capability. The uncovered branch is the ACCEPTED PARTIAL: the named event
  fires, the capability does not arrive, and the gate reads as open.
metadata:
  type: feedback
---

# A carried obligation gated on a MERGE EVENT fires on an accepted partial

`RT-BODY-OCCURRENCE-PROVENANCE` landed an `#[ignore]`d control carrying a
deferred obligation, correctly built so it could not pass vacuously: the body
is a bare `panic!`, so removing the attribute without supplying the witness
reds. That defence is sound and it is not the one that mattered.

Three days of work later `KERNEL-NESTED-IND` merged as an **accepted partial** —
node not closed, `D5` undischarged, native consumability not claimed. The
control's gate read **`Release condition: KERNEL-NESTED-IND merged`**. That
predicate went true. The capability did not arrive.

> **The general rule, as the Steward recorded it:** *gate a carried control on
> the capability, never on a merge event.* They had shipped **four accepted
> partials** in that window, so the branch is not exotic — on this fleet it is
> the common case.

## The sharpest part: coextensive predicates diverge, and the LABEL lands on the weakest

The doc carried **three** phrasings, all true and interchangeable when written:

| phrasing | after the partial merge |
|---|---|
| "nested-inductive admission is on `main`" — the capability | not established |
| "`KERNEL-NESTED-IND` merged" / "on main" (three occurrences) | **TRUE** |
| owned by the first "post-Kernel **closure** candidate" | FALSE |

The line explicitly labelled **`Release condition:`** was the *merged* one — and
a reader hunting the gate reads the labelled line first. So divergence
**promoted the weakest predicate to the operative one**, and the two phrasings
naming the real capability sat in the same comment being ignored.

⇒ When several wordings of a condition are coextensive at authoring time, the
label attaches to whichever is easiest to state, not to the one that will still
be right. **Ask which of them are separable events, and gate on the one that is
false longest.** Fixing only the labelled line reproduces the defect one line
down — the Steward's `AC-D7-2` requires all five occurrences to agree for
exactly that reason, and called it the third same-shape instance that day.

## The hunting move — this finding was NOT in the cut I was asked to hunt

I only had it because I had hunted the **previous** merge and still held what it
carried. Neither notification referenced the other; the Steward wrote both and
said they did not see it.

⇒ **On every merge notification, ask what PREVIOUSLY-LANDED carried obligations
name this node.** One `git grep <NODE-ID> -- crates/` is the whole check. A
carried control is authored in one ring and released by an event in another, and
**no single ring's vantage spans that** — which is precisely why it is this
seat's. Sibling of
[[a-node-that-closes-without-discharging-inverts-the-doc-that-named-it]]: same
family (an anticipatory clause's uncovered branch), different trigger — that one
is closure-without-discharge inverting prose, this one is partial-merge opening
a gate.

## The technique that made it actionable rather than arguable

The doc named its own dependency, a **held** commit. Do not test that with
ancestry:

- `git merge-base --is-ancestor <held> main` said **not an ancestor**. Under
  squash-merge that is worth **nothing** — a squash-merged SHA never becomes an
  ancestor (`COORDINATION` §14). I discarded it.
- `git diff --numstat <held> main -- <each path>` settles it by **content**. The
  kernel path came back `+31/-0` (a superset: landed). The four elaborator and
  interpreter paths came back as the **exact numeric inverse** of the held
  commit's own diff — `+33/-37` against its `+37/-33`, and so on.

**An exact numstat inversion proves the target sits at the PRE-change state.**
That one reading converted "the venue may not be reconstructible" into a
measurement, and the Steward said it was what made the finding actionable.
Related: [[a-citation-that-does-not-resolve-needs-checking-against-every-tree]]
(ask which tree it IS true in), and `fleet/` — a squash drops pieces silently.

## The self-directed half

I had verified this same control **one merge earlier** and cleared it — reading
the panic body, confirming it could not read as discharged, and recording it as
swept. That verdict was correct at that anchor and said nothing about the gate,
because I audited the control's **body** and never its **release condition**.
A fail-closed body and a wrong gate are independent surfaces; checking the one
that is easy to check is not coverage of the other
([[rank-subclaims-by-load-bearing-not-by-checkability]]).
