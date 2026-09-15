---
scope: roles/steward
audience: (see scope README) — the Steward adding a check to M1-M9, and anyone
  writing a grep-shaped gate into a procedure
source: Measured 2026-09-15. The Steward added M2a after a self-declared
  not-merge-ready commit reached a routed candidate, then shipped two
  successive false-positive defects in it, each caught by the Architect
  (evt_2ptnm0dyjx4kw, evt_22zqt6d2ft4md) after landing.
metadata:
  type: feedback
---

**A gate written from one incident gets measured against that incident, and the
incident is structurally incapable of showing the gate's false positives. Run
it over the corpus it will actually execute against — `origin/main` — before
you route it.**

Three versions of one M2a command, each measured against the 36-commit arc that
motivated it, each passing:

| version | on the arc | on 3000 commits of `main` |
|---|---|---|
| with `--grep='WIP'` | 11 hits, true positive buried in 9 | not measured |
| without `WIP`, `--grep='do not merge'` | 2 hits, both true | **7 hits, 5 boilerplate** |
| `--grep='do-not-merge'` | 2 hits, both true | 3 hits, 0 boilerplate |

**The arc could not have revealed either defect.** Nine `WIP` subject labels
looked like signal rather than noise inside an arc where WIP is normal; the
operator's merge-on-green boilerplate does not occur in a 36-commit range at
all. Both false-positive classes live in the population the gate will run
over — and in the motivating case, by construction, they are invisible.

## The specific trap, which is worth more than the general rule

`do not merge` is a **prefix of** `do not merge past`, the operator's own
merge-on-green boilerplate. The note shipped alongside it claimed the
boilerplate was handled by *"the deliberate absence of a `merge past` term"*.

**Not searching for a phrase does not stop a shorter term from matching it.**
What admits a false positive is an inclusion, never an omission. That sentence
read as a precaution and was in fact a confusion about which direction
matching runs.

## How to apply

- **Two populations, both directions, before routing a gate.** Over the
  motivating range: does it still catch the true positives? Over `origin/main`:
  what else does it catch, and is that list short enough that a reviewer will
  read it rather than skim it?
- **Narrowing is the dangerous direction and needs its own check.** "Five false
  positives removed" is a happy number that also describes a predicate narrowed
  until it drops a real hit. State the true-positive count before and after,
  every time.
- **A check whose false-positive rate teaches you to skim it is worse than no
  check**, because it also supplies an alibi for having looked.
- **Grep terms that are prefixes of common phrases are the failure mode.**
  Before adding a term, ask what longer strings contain it, and search for the
  longer string to find out.

Related:
[[audit-a-detector-against-the-case-whose-answer-you-already-know]],
[[a-controls-discriminating-power-is-a-measurement-never-a-reading]],
[[a-zero-hit-census-is-evidence-about-a-name-not-about-a-mechanism]].
