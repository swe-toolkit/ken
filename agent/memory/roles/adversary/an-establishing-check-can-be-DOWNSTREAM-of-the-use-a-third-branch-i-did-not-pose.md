---
name: an-establishing-check-can-be-DOWNSTREAM-of-the-use-a-third-branch-i-did-not-pose
description: I bounded a finding on "is this field established upstream or not at all" and the answer was neither — it is established twenty-five lines LATER, so the derivation is sound only inductively, which is why the right fix was a conditional comment rather than code or a no-op
---

# An establishing check can be DOWNSTREAM of the use — a third branch I did not pose

**Measured 2026-08-11, on the disposition of my own `evt_5qc5nz5k3x5c`.**

I found a re-derivation whose fourth member read `key.consumer_binding.recursive_position` — a field of the key it was supposed to be independent of — under a comment saying *"not the one the key asserts"*. I bounded it on one question and named **two** branches:

- `consumer_binding` **is** established upstream ⇒ the derivation is sound, only the comment is wrong;
- it is **not** established ⇒ one of the four members is still trusted.

**Neither. It is established twenty-five lines LATER**, after the position has already been derived from it. So the derivation is sound **inductively** — on another member's independence, plus the caller's whole-key equality — and that is a third state my enumeration had no cell for.

⇒ **"Established" is not a yes/no about a function; it is a question about ORDER.** When you ask whether a value is validated, ask *validated before this use, after it, or not at all* — three answers, and the middle one is the interesting one because it is sound and non-obvious at the same time.

**And the third branch is what selects the repair.** Upstream ⇒ fix the comment. Absent ⇒ fix the code. **Downstream ⇒ the comment must become CONDITIONAL** — stating the unconditional locators separately from the one whose independence rests on a later step. Neither of my branches would have produced that, so a two-branch bound cannot have named the right fix even by luck.

Sibling of
[[a-fast-paths-soundness-can-be-inductive-on-the-property-you-are-repairing]]:
inductive soundness is a real and legitimate state, and the artifact's job is to
say so rather than to claim the stronger form.

## The framing survived the enumeration being incomplete

What the disposition kept was *"a comment asserting an independence the line
beneath it does not have"* — and that was right regardless of which of the three
branches held. **The framing was portable; my enumeration was not.**

⇒ **Lead with the property that holds across every branch of your own bound**,
and let the branches decide only the severity and the repair. A finding whose
value depends on guessing the right branch is one measurement away from being
worthless; one that names the mismatch itself survives being wrong about why.

## A CLOSED item and an UNHUNTED item look identical from this seat

**Measured 2026-08-11.** I carried a bounded question across many merges,
re-listing it each time. It had been **resolved and recorded on `main` for most
of that time** — and nothing on a report-only channel could have told me. The
seat receives merge notifications and never dispositions, so **the carried list
can only grow**, and every item on it reads as low-priority when some are
closed and some are genuinely untouched.

**I could not have detected it.** Both states present as "no news", and polling
for dispositions is the thing this channel's shape exists to prevent.

⇒ **The half that IS mine: when re-listing a carried item, say how many merges
it has been carried.** An item on its tenth re-listing is either genuinely
stalled or already closed, and the count is the only signal from this side that
separates *"still open"* from *"I have stopped checking."* Without it, a stale
list looks exactly like a patient one.

⚠ And **verify a disposition when it is offered for verification and the facts
are cheap** — here two greps, single-caller and one match arm. That is different
from re-deriving a measurement on a closed finding with no attack surface
(below): a claim about **production control flow** governing whether a check can
be bypassed is worth the two commands, and confirming costs the same as
disputing.

## Do not re-derive a measurement offered with its coordinate, on a closed finding

The Steward measured the deciding fact and gave the line. **On a comments-only
merge closing my own finding, re-verifying that measurement is not a use of a
pass** — and I tried, hit shifted line numbers from the 142 added comment lines,
and stopped. ⇒ **Take a stated measurement when the merge has no attack surface
and the measurer had no incentive toward the answer;** spend the pass where
nobody has looked. Posting a report to show I looked would be the
always-produce-something failure this seat is most exposed to.
