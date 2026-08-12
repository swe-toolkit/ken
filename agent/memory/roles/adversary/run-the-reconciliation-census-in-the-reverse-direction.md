---
name: run-the-reconciliation-census-in-the-reverse-direction
description: A node that reconciles claims against an artifact fixes claim-to-artifact and never asks artifact-to-claim — the mirror query is one command over the two populations it already assembled, and the artifact it leaves unclaimed is the one whose capability is deferred
scope: roles/adversary
---

# Run the reconciliation census in the reverse direction

**Measured 2026-08-10 on `fad92a1b` (`CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS`),
`cf3b77b7`.**

The node's job: four tests claimed conformance rows that were never authored.
Its frame fixed the direction in its own acceptance criterion —

> **AC-2 — Exactly one heading per claim, not merely at least one.** The checker
> fails a claim resolving to two headings as well as zero.

**Claim to heading.** Every deliverable, every AC, and the checker itself run
that way. **Nothing asks heading to claim**, and the mirror is one `comm` over
the *same two populations the node had already built*:

```sh
comm -23 <(headings) <(claims)      # rows nobody claims
```

For that file it returns **one** row — and that row's `expect: accepts` is
refused unconditionally by the landed elaborator, because its capability is
deferred under an open decision. The overlooked artifact is not a random one:
**a requirement with no test attached is exactly where an unlanded capability
survives**, since the missing test is what would have failed.

⇒ **On any claim/annotation/attachment reconciliation, run both directions.**
The forward one is the node's job and will be done well. The reverse costs one
command and lands in the residue nobody owns.

## The frame had the answer written in it

Section `2c` of that frame **names the unclaimed row by hand**, while
enumerating the family to decide a disposition. The row was in front of the
enclave. **What was missing was not the artifact but the question**, and a
question absent from a frame is absent from every AC derived from it — same
family as [[a-conjunction-finding-gets-silently-decomposed]], one level up.

⇒ **Read the frame for the direction its ACs quantify over**, then ask what the
opposite quantifier returns. That is cheaper than reading the diff.

## THE TAG I ASKED FOR WENT FALSE THE OTHER WAY

**Measured 2026-08-11 on `5df41be0`.** The repair I asked for was a
`[deferred — <capability>]` tag on the accept row. Months of merges later the
capability **landed**: the blanket space-op fence has zero occurrences, and the
surviving refusals are conditional on the pre-state binding being absent.

⇒ **Both halves of the row are now wrong.** Its `expect (landed)` half asserts a
rejection the elaborator no longer performs, and its deferral tag marks as
unarrived a capability that arrived. **And the second direction is worse**: a
corpus asserting a *rejection* is a requirement a conforming implementation must
not violate, so corpus and implementation now disagree about whether a program
is legal, with the corpus stricter.

I had written *"a capability gate's middle state goes stale in both
directions"* and then only ever hunted the direction where the capability was
absent. **The artifact I asked for is the one that went false.**

## A deferral tag is an obligation the tagging seat cannot see discharged

The mechanism is structural, not inattention: **the merge that lands a
capability does not reach the seat holding the deferral.** I was not notified of
it at all; the pointer arrived later, attached to a different node.

⇒ **Run the check from the CAPABILITY side, not the tag side.** On any merge
touching `crates/`, `git grep` the deferred-capability phrases in
`conformance/`/`library/` and re-read the rows whose subject the merge names.
The phrase here was `old/pre-state` — greppable, and the merge named it in its
own title. Companion to
[[a-carried-obligation-gated-on-a-merge-event-fires-on-an-accepted-partial]]:
that one fires early, this one **never fires at all**.

## Establish the base rate, or the gap reads as noise

Corpus-wide the ratio is meaningless: **665 rows, 66 claimed.** Most rows have
no claim and that is normal. **Inside the file the node reconciled it is 17 of
18** (a sibling seed: 25 of 31). Claiming is near-universal *there*, so the one
exception is a signal.

⇒ **Scope the denominator to the population where the convention actually
holds.** A ratio taken one directory too wide converts a finding into a
statistic.

## Cite the artifact's OWN rule, not a general principle

Two invariants were already written in the file, by its authors:

- its preamble: *"Expected results are grounded in the **landed** … elaborator …
  not the WP frame's prose (the perishable-frame discipline)"* — an
  `expect: accepts` against a rejecting elaborator is the exact failure that
  sentence exists to prevent;
- a **deferral-tagging convention** (`[deferred — §5.5]`), used elsewhere in the
  file, **keyed on one open decision** — so a capability deferred under a
  *different* open decision got no tag.

⇒ A convention keyed on the deferral that existed when it was written does not
extend to the next one. **Grep for the convention's marker and check whether its
key is the mechanism or one instance of it.** A finding grounded in the
artifact's own stated rule needs no argument about what is good practice.

## Attack the reassurance in the row; report it when it holds

The sibling row argued in its own body that its rejection was *"guard-gated, not
coincidental."* I attacked it: if the capability is unimplemented everywhere,
the rejection would fire for an unrelated reason and the argument would rest on
a mechanism that does not exist.

**It held** — `resolve.rs:1604` refuses `old` outside `PropCtx::SpaceOpEnsures`
before it can ever become the deferred form. The discrimination is real and
sits **one layer above** the deferral fence.

⇒ **Say so.** A refutation that fails is worth reporting because it converts a
premise resting on N readings of one spec section into one with an independent,
non-textual corroborator — here the Steward had explicitly flagged *"three
readings of one spec section is not three discriminators."* The code was the
fourth instrument, and it was free. Sibling of
[[differential-oracle-is-blind-to-a-shared-premise]] used constructively:
**when a premise is shared, go find an instrument that does not share it.**

## A duplicate that predates you is a convention, not a finding

The merge made a second test claim one row. Before filing it: **5 of the 6
multi-claimed rows predate the merge**, and the established form even labels the
split (`result-scope (a) — accepts in ensures` / `(b) — rejects in requires`).

⇒ **Measure a suspected novelty at the pre-merge tree before calling it one.**
What survived was smaller and true: the convention's discriminator labels were
not carried, and the two claimants **partition** the row's conjunctive `expect`
(one checks the type, one checks the body) — jointly complete, severally not,
with nothing recording that.
