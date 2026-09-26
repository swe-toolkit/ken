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

## A hit counter must count the change, not the failure

Merged 2026-09-26 from the Adversary lesson measured 2026-08-15 on
`ad47054a5...1cd9947cf`: the degenerate-to-identity mutation again, this time
hidden in the hit counter.

```rust
let position = if mutated { usize::MAX } else { 0 };
match plan.child_static_origin(entry, position) {
    Ok(body)          => Body(body),
    Err(_) if mutated => { note_hit(); MissingBodyChild { entry } }
    Err(_)            => Entry(entry),
}
```

At one route the unmutated lookup already fails (a sibling control in the same
candidate asserts that a declared-unit scheduling entry with no child zero must
state its level). Arming the mutation there changes no outcome and still counts
a hit. Redirecting an input to something impossible makes the operation fail;
that is not evidence its result moved. **Where the unmutated result is
computable (here literally `position = 0`), the counter must compare against
it.**

- **Diff the new instance against its older siblings.** Both neighbouring
  mutations in the file stated and enforced it: *"The hit is counted only when
  the coordinate actually CHANGES"*, and `assert_ne!(passed_in, used, "the
  substitution returned the coordinate it was given, so this row is a no-op
  wearing a hit count")`. A house style documented but not factored into a
  helper gets re-derived correctly twice and dropped the third time.
- **Say where it is masked and what unmasks it.** In its own control the weight
  was carried by a discriminating tag flip, so `assert!(hits > 0)` beside it
  was redundant and green for a good reason; but the counter is global across
  all six routes. The report: "this control does not depend on the broken part,
  and the next one written to the `hits > 0` shape will." A redundant
  assertion is where a defect hides.
- **Group by cause.** A second symptom, `MissingBodyChild` returned for an
  unmutated `Err` without a hit at the sibling resolver, conflates "the
  mutation removed it" with "the plan has none". Same root: neither resolver
  compares against its own unmutated outcome; one repair covers both.
- **Say when an exclusion is simply right.** Five production
  `child_static_origin(...)?` calls were excluded from the repair population
  and flagged as not independently controlled. A production `?` behaves
  identically in both profiles, so it cannot produce the test-profile-only
  divergence the node exists to close. When the criterion is the argument,
  asking for a control is asking for ceremony.
