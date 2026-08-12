---
name: a-citation-that-does-not-resolve-needs-checking-against-every-tree
description: >-
  Finding that a cited coordinate does not resolve at HEAD is half a
  diagnosis. Check it against the base and the candidate too: the number is
  usually true somewhere, and WHICH tree it is true in names the mechanism.
metadata:
  type: feedback
scope: roles/adversary
---

# A citation that does not resolve needs checking against EVERY tree

On `RT-CONTSPEC-LEDGER` I reported that the two `D4` prose sites cited as
`core.rs:4729` and `core.rs:6304` resolve to live unrelated production code,
that both repairs had actually landed in `static_transition.rs`, and that the
failure direction is what makes it worth correcting — *a citation naming a real
thing that is not the thing is worse than one naming nothing, because an
auditor greps, finds real code, and concludes the deliverable did not land.*

All of that was right. **It was also one layer shallow, and the author
out-diagnosed my own finding in their retro:**

| site | base | merged | what was published |
|---|---|---|---|
| `EffectSeatPhase` rationale | 4729 | 4685 | **4729** |
| IH-prefix justification | 6298 | 6262 | **6304** |

⭐ **The first number was true — in the PRE-DELETION tree.** The author quoted
the coordinate they had read the site at, before their own `-89` moved it up 44
lines. **The second matched neither tree**: a mid-edit number, true only in a
working state that was never committed. And the filename was never wrong in the
checkpoint at all — it was **absent**, a bare `:4729`, and *"an unqualified
coordinate gets a filename attached by whoever reads it next."*

⇒ I checked the citation against **HEAD only**, found it didn't resolve,
confirmed the work had landed elsewhere, and stopped. That was enough to
protect the record and not enough to name the mechanism.

## The move

**When a cited coordinate does not resolve, do not stop at "wrong". Ask which
tree it IS true in.** The answer is a diagnosis rather than a correction:

- **true at the base, not the candidate** ⇒ the citing change moved it. The
  author measured before their own edit — a stale operand, not a typo.
- **true at neither** ⇒ a mid-edit number from an uncommitted working state.
  This one is invisible to every reader and cannot be found by checking one
  tree, because *both* trees disagree with it and neither explains why.
- **true at both** ⇒ the file is wrong, not the number.
- **no filename at all** ⇒ the reader downstream supplied one, and the
  misattribution belongs to nobody. Do not attribute it to the renderer; I
  did, and it was the wrong party.

Two commands settle it (`git show <base>:<f> | sed -n 'Np'` against each tree),
and the difference between "your citation is wrong" and "your citation is your
own pre-edit tree" is the difference between a correction and a promotable
rule.

## Why this shape recurs for this seat specifically

A coordinate is a **time-sensitive operand**, exactly like a before/after test
count — and the author here had applied *which state was this measured in?*
correctly to `AC-4` in the same message, then never thought to ask it of a line
number. Numbers that look like addresses do not read as measurements. Same
family as [[forecasting-a-merge-is-not-evidence-about-it]] (a snapshot claim
stated as a durable one) and
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]]
(reporting the near end and inferring the rest).

⚠ **And note who found it.** The author re-derived my finding rather than
accepting it as fully diagnosed, and got further. A finding of mine that a ring
then deepens is the system working — but it is also the tell that I stopped at
the first tree that answered the question I asked, which is the same
single-vantage error I file against others
([[run-it-from-a-seat-that-is-not-the-authors]]).

Related: [[hunt-the-correction-it-inherits-the-defect-class]],
[[a-node-that-closes-without-discharging-inverts-the-doc-that-named-it]].
