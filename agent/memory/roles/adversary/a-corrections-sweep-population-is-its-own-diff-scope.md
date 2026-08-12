---
name: a-corrections-sweep-population-is-its-own-diff-scope
description: An "all N occurrences agree" acceptance criterion is satisfied against the file the fix was scoped to, so the authority document — which is not in that diff — keeps the defective predicate; and the fix's own AC can offer the option its own diagnosis table calls FALSE
scope: roles/adversary
---

# A correction's sweep population is its own diff scope, so the authority document is never in it

**Measured 2026-08-09 on `e1e93ee6` (RT-BODY-OCCURRENCE-PROVENANCE `D7`), the
fix for a finding I had filed one merge earlier.**

`D7` corrected a carried control's release gate from a **merge event** to the
**capability**, and its `AC-D7-2` said *"all five occurrences agree."* They did.
The candidate was one path, `crates/.../planning/static_transition.rs`, and all
five surfaces in that file were swept clean.

**The node that governs the control kept the defective phrasing.** Two sites —
`docs/program/issues/RT-BODY-OCCURRENCE-PROVENANCE.md:277`, the `AC-5`
acceptance row, and `:52-53` — still assigned the runnable form to *"the first
post-Kernel closure candidate"*, the phrase `D7` removed from `crates/` as
keyed on node closure rather than capability.

⇒ **"All N occurrences" is evaluated against the population the fix could
touch.** A comment-only, one-path candidate has a diff scope of one file, so the
sweep is bounded by it before anyone counts. The authority artifact sits in
another directory and is structurally outside — and it is the one a reader
consults first, because it says so itself (`:57`, `:323`: *"The node's
acceptance table is the authority."*)

## The document diagnosed the predicate and then kept it

The finding is self-evidencing rather than an argument about vocabulary. The
node's own `D7` diagnosis table, `:405`:

```
| 16510 | owned by the first post-Kernel closure candidate | FALSE — the node is not closed |
```

**The document recording that predicate as FALSE carries it in its own
authoritative acceptance row.** When a correction names the bad predicate in a
table, `git grep` that predicate's *words* against the correcting document
itself — the diagnosis and the operative rows are written in different passes
and only the diagnosis is proofread as a claim.

## The AC can sanction the option its own table refutes

`AC-D7-1` required the labelled line to state the capability, *"either
'nested-inductive admission is on `main`' or 'post-Kernel **closure**' …
Choosing between them is the owner's call"* — offering as acceptable the exact
phrasing `:405` classifies FALSE. **The implementer chose correctly, so nothing
reddened and the AC's defect left no trace.** An acceptance criterion that
enumerates permitted answers must be checked against the finding's own verdict
table; a free choice between a sound and an unsound option is a coin-flip gate
that passes whenever the author happens to be careful. See
[[a-conjunction-finding-gets-silently-decomposed]] — same failure family: the
finding survives operationalization only where the AC's shape can hold it.

## Bound the severity honestly, and say which way

The `crates/` control was capability-keyed and `panic!`-bodied, so a candidate
walking to the code is stopped: **not** an un-ignore or vacuous-green hazard.
What is exposed is an owner assignment that fires early plus the authority
disagreeing with the code it governs. Say that, or the report reads as a
soundness hole — and see
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]]
for why the direction needs the far end measured, not inferred.

## Why the window is narrow

The node was set `status: merged` **two minutes after** `D7` landed. A closed
node is re-read by no gate, so the divergence freezes at the moment it is
cheapest to fix and most invisible. Companion to
[[a-node-that-closes-without-discharging-inverts-the-doc-that-named-it]]: there
closure inverted a doc's meaning; here closure merely **preserves** a defect the
same commit stream had just corrected elsewhere.

⇒ **On any correction landing as a one-path candidate, hunt the artifact that
declares itself the authority for the corrected thing, before its node closes.**
And re-measure the capability at current `main` rather than citing the fix's own
measurement — [[a-carried-obligation-gated-on-a-merge-event-fires-on-an-accepted-partial]]
is what makes the event-vs-capability gap live.

## THE AUTHORITY SITE AND THE COINING SITE ARE DIFFERENT TARGETS

**My own sweep missed one, and the Steward found it.** I took *"the artifact
that declares itself the authority"* as the target and stopped at the node.
The fix that landed (`c18c2df1`) corrected **three** sites, not my two: the
third was `docs/program/wp/RT-JOIN-ORIGIN-ATTRIBUTION.md` — **the frame where
the phrase was coined** — with **both** its cells still event-keyed.

A defective phrase has a **provenance**, and the artifact that minted it is
usually neither the code nor the authority table. It is also the site that will
re-seed the phrase into the *next* node, because a frame is what the next
framing is copied from — the same mechanism as
[[a-retired-rule-survives-in-the-boilerplate-that-gets-copied-into-new-artifacts]].

⇒ **Enumerate by asking where the wording came FROM, not only where it is
BINDING.** Concretely: `git log -S'<the exact phrase>' -- docs/` to find the
commit that introduced it, and sweep that artifact too. "Authority" is a
property of one document; **provenance is a chain**, and a sweep scoped to the
authority terminates one hop too early.

**Do not report a partial sweep as a clean catch.** I stated the two sites I
found without saying which population I had enumerated over — the exact defect
[[no-option-works-name-the-axis-you-enumerated]] already names, recurring in a
finding rather than in a ruling. Name the axis: *"I swept the authority
document; I did not sweep for the phrase's origin."*
