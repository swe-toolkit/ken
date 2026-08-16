---
id: RT-REFUSAL-CONSEQUENCE-RESTORE
title: "Restore the two clauses the D2 refusal rewrite dropped as collateral -- the consumption site and the runtime-representation consequence -- without reopening the transfer reading D2 closed"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-SECOND-RECOGNITION-ERASURE]
blocks: []
github: "https://github.com/swe-toolkit/ken/pull/2390"
origin: "Steward, 2026-08-16, dispositioning Adversary hunt evt_6ssfztvwhxjpz on the merged range ec2b4a1eb..e23a18aee (PR #2377). The hunt confirms D2's fix landed on its criterion and isolates two clauses removed by an edit scoped to the sentence rather than by the finding. Steward-filed per COORDINATION section 2."
---

## `D2`'s fix LANDED. This node does not touch it.

**`RT-SECOND-RECOGNITION-ERASURE` `D2` succeeded on its own criterion.** The
new *"**this recognition's own** transport **never** reaches a consumer"*
scopes the claim to the recognition and makes it absolute, **so a reader cannot
get from it to "a later consumption might discharge it"** — the reading `D1c`
refuted. The temporal phrasing that invited it, *"nor erased **before
construction**"*, is gone and **must stay gone.**

**Nothing here reopens that.** Adversary `evt_6ssfztvwhxjpz` verified the fix
before reporting what it cost.

## The defect: the refusal states its condition twice and its consequence never

**This message is user-facing.** `surface.rs:249` —
`impl fmt::Display for UnsupportedLowering` does
`write!(f, "{}: {}", self.construct, self.reason)`, **so `reason` is printed
verbatim.** It is what a programmer sees when their program is refused.

| clause | old | new |
|---|---|---|
| cause | no static elimination rebinds | *same* |
| disposition | neither consumed **at an exact-Var call** nor erased **before construction** | neither consumed nor erased |
| transport | — | this recognition's own transport never reaches a consumer |
| **consequence** | **denotes a value containing the callable and has no runtime representation** | **GONE** |

⇒ **Two of the three surviving clauses say "not consumed", and none says why
that is fatal.**

> ### ONLY ONE DROPPED PHRASE WAS IMPLICATED BY THE FINDING
>
> **`D1c` refuted `transfer`, so *"before construction"* had to go.** Neither
> *"at an exact-Var call"* nor the runtime-representation clause implies
> transfer — **the first names WHERE a consumer would have to be, the second is
> the actionable content.** Both were removed by **an edit scoped to the
> sentence that contained them, rather than by the finding that motivated it.**
>
> ⇒ **A refusal that describes only the state it is in tells the reader nothing
> to do.** *"Has no runtime representation"* is the sentence that turns this
> from a report into a diagnosis.

**This is grounded in `docs/PRINCIPLES.md`'s honesty-about-the-boundary
commitment, not in a preference for fuller prose.** A refusal is the compiler's
one chance to say what it could not do and why; a correct message that omits
the consequence spends that chance.

## `D0` — restore both clauses, keeping the fix

**The Adversary supplied wording that satisfies both constraints. Treat it as a
starting point, not as text to paste unread:**

> *"...that no static elimination rebinds, so this recognition's own transport
> never reaches a consumer at an exact-Var call and is not erased; a
> constructor carrying an unconsumed static worker denotes a value containing
> the callable and has no runtime representation."*

**It keeps *"this recognition's own"* and *"never"*, keeps *"before
construction"* deleted, and restores the site and the consequence.**

**Verify the claim before restoring it.** *"Denotes a value containing the
callable and has no runtime representation"* was true of the old code path.
**Confirm it is still an accurate description of what happens** — a restored
clause that is now wrong is worse than an absent one.

## `D1` — update the control in the same commit

**The four expected renderings in `core/tests/control.rs` agree with the
`format!` template byte-for-byte**, which is why the control pins the message
exactly and any drift reds. **That is working as designed: changing the message
means updating all four in the same commit**, and a candidate that changes one
without the others will red before review.

## Acceptance criteria

**`AC-1`. The transfer reading stays closed.** *"Before construction"* is not
restored in any form, and the message does not admit that a later consumption
might discharge an earlier recognition. **This is the whole point of the
predecessor and it is not relaxable.**

**`AC-2`. The consequence is stated.** A reader learns **why** the unconsumed
recognition is fatal, not only that it is.

**`AC-3`. String literals only.** No change to `close`'s law, the ledger's
states, the agreeing bijection, any consumer or producer, or the environment.
**Identical scope bar to the predecessor's `AC-1`.**

**`AC-4`. The control and the template still agree byte-for-byte**, all four
renderings updated together. **Do not weaken the control to accommodate the
text** — its exactness is the property that makes the message a pinned artifact.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Reopening `transfer`, void-at-supersession, or any ledger disposition.**
  The campaign's option space is closed; see
  [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]].
- **Changing what `close` refuses.** Only how the refusal reads.
- **Rewriting neighbouring refusal messages** because they are nearby. **This
  node exists because an edit scoped to a sentence took clauses the finding did
  not implicate — do not repeat that at a larger radius.**

## Sequencing

**Lane 1, queued behind `RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY` `D0`.** Text
only, no contention with that node's read, and **nothing blocks on it.**

**The defect it fixes is a message a user reads, not a soundness gap** — size
it accordingly and do not let it displace the lane's `D0`.
