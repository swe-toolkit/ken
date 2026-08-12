---
name: disable-the-mutation-mechanism-to-find-vacuous-controls
description: >-
  To find committed mutation controls that are green by vacuity, force every
  mutation read to Exact and run the suite — any control test that still passes
  is measuring nothing. Reading for the fallback idiom only finds candidates.
metadata:
  type: feedback
scope: roles/adversary
---

# Disable the mutation mechanism to find vacuous controls

A `#[cfg(test)]` mutation control goes green-by-vacuity in two ways: its
fixture never reaches the seam, or the mutation **degenerates to the identity**.
The second hides in a population search that falls back to the exact value —
`find(|x| x != exact).unwrap_or(exact)` — so when the population holds one
member the control silently perturbs nothing.

**Grep finds candidates. It does not settle them.** A site can carry the idiom
and still be sound if the test asserts the mutation *was observed to perturb*
something. One block here had `unwrap_or(exact)` plus four arms guarded on
`is_none()`, and read as five vacuous rows; instrumenting it showed the fixture
compiles two call sites, one per regime, so every arm is identity at one and
live at the other, and the test's `assert!(mismatches > 0)` holds honestly.
That assertion is exactly what the vacuous control lacked.

**The instrument that settles it: force every mutation read to `Exact`, then
run the whole suite.** A control test that still passes with its mutation
switched off is measuring nothing. Here 13 enums neutralized reds 16 of 617,
and every one was a control test — no survivors, so the shape was clean. This
is cheap (one run), needs no per-site reasoning, and its negative result is
meaningful, unlike deleting a gate.

**What it does not prove.** It shows each *driver test* is live, not each
*variant*: a test covering four variants reds identically if three are dead.
Close that only where the variants can carry the shape, and say plainly which
variants you left un-probed rather than letting a clean run imply coverage.

**Do not probe every mutation against one fixture.** My first attempt built one
fixture under all variants and compared plans; it reported 18 as vacuous. Wrong
— a key-weakening mutation changes nothing when the fixture's units already
differ in other fields, which is why the matrix test builds minimal-difference
keys by hand. Each control must be measured against **its own driver's**
fixture. Related:
[[a-measured-property-can-be-true-and-not-entail-what-the-mechanism-needs]],
[[a-differential-over-an-aggregate-passes-while-one-of-n-contributors-defects]],
[[a-green-mutation-does-not-tell-you-which-blindness-let-it-through]].
