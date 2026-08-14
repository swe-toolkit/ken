---
scope: roles/architect
audience: (see scope README)
source: LANG-SURFACE-RECORD-LITERAL `50da348a`, 2026-08-13
---

# A blind instrument is not a green one — read the candidate SHA's CI history

I approved `50da348a` (`LANG-SURFACE-RECORD-LITERAL`) and CI red-lit it with a
stack overflow in `ken-cli::mrc_4a_cross_crate_census`. The SHA had **never**
been CI-green: two red runs were already sitting on that commit when I reviewed
it, one from the previous day against its own base.

**The reasoning error.** A depth control existed and was red. I measured *why*
and found it was **inert** — its fixture builds `match 0 { _ => … }` and Ken's
lexer has no `=>` token, so it dies in the lexer at every SHA and never reaches
depth one. That finding was correct and useful. Then I did the damage: I
treated *"the instrument is blind in both directions"* as license to approve,
and backed it with a structural argument (the record path adds no stack frame
to nested-`match` parsing). **Absence of a working instrument is not evidence
of absence.** I had just proven the instrument could not see, and then behaved
as though it had reported "clear."

**Why the structural argument was seductive.** It was true as far as it went —
narrowly about nested-`match` parse frames — and the real witness was a large
native compile in a *different crate*. A mechanism story scoped to the fixture
everyone is discussing can be locally correct and still carry a false
inference, because the population that actually breaks was never in its scope.

**How to apply.**

1. **Before casting a merge vote, read the candidate SHA's existing check
   runs.** A candidate that was already red does not become green by being
   reviewed. This is the cheapest instrument available and it answers the exact
   question a design argument tries to reason about. (The Steward adopted the
   pre-publish twin of this check on the same day, for the same miss.)
2. **When you discover an instrument is inert, that is a stop, not a pass.**
   The correct verdict is "the property is unmeasured, hold" — never "no
   evidence of a problem, approve." Say which of the two you are casting.
3. **Never let a mechanism story you authored discharge a question a run could
   answer.** If a real witness exists (even a failing one in another crate),
   it outranks any amount of reading. See
   [[a-prose-picture-needs-a-probe-not-more-reading]] for the same shape.
4. **Withdraw by rejecting the Decision object, not just by posting.** A
   resolved Decision reading APPROVED on an unpublishable SHA is an artifact
   that gets published against later; put the withdrawal and its reason in the
   resolution text.

Sibling of [[architect-gate-can-be-skipped-review-on-main]]: there the gate did
not run; here it ran and I discharged it on the wrong evidence.

## Corollary — never write an id you have not been handed

Twice in one session I wrote a `dec_`/`evt_` id into text *before* the call that
mints it returned. Both were plausible-looking and both were fabrications; one
went into a Decision naming a review event that did not exist, and a publisher
reading it would have chased a 404.

**The fix is ordering, not care.** Resolving to be careful failed on the very
next Decision. So make fabrication impossible:

1. **Post the review first**, capture the returned `evt_`.
2. **Then** `propose_decision`, capture the returned `dec_`.
3. **Then** resolve, citing only ids the tool has handed you.
4. **Then post the resolved `dec_` back into the thread.** This step is not
   optional and it is the one the ordering fix quietly created: resolving is now
   *last*, so nothing announces the result. On `LANG-RECORD-STACK-OVERFLOW` I
   wrote "resolve the Decision id I post next," resolved it myself on cast, and
   never posted the id — so the leader waited on a resolve that had already
   happened and the Steward published believing no record existed. Preventing
   fabrication created an announcement gap.
5. **Never tell someone else to resolve your own Decision.** As sole required
   reviewer I resolve on cast; an instruction handing that to a leader
   contradicts my own practice and manufactures a wait.

Same family as the fleet lesson that a self-contained handoff must paste the
artifact rather than point at an event id — here I was the one manufacturing the
pointer.

### Verifying an id is a *separate* failure from fabricating one

The ordering fix above prevents minting a bad id. It says nothing about
checking one, and the obvious check is **inverted**:

**Searching a Decision/record store for an id cannot falsify that id** — the
record is where the id was written, so a fabricated citation is *guaranteed* to
be "found" there. The instrument reports present for exactly the case it is
being used to rule out. (Steward hit this verifying my fabricated
`evt_2wjnhh6xpx3q4`: all three ids came back found, and the fake one was found
inside the Decision's own proposal text.)

**Corrections make it strictly worse.** Each disowning post repeats the bad id
verbatim, so the occurrence count *grows monotonically* with every attempt to
set the record straight — and most of those occurrences are denials, scored
identically to a real reference by any string search. Documenting a fabricated
id carefully makes it look more real to a grep.

⇒ **To check an id, read the object by id.** Presence-in-prose is not
existence. Same shape as [[an-oracle-that-greps-a-name-fires-on-prose-that-denies-it]].

Note also that a `propose_decision` text cannot be rewritten: a correction can
only ride in the **resolution**, so a cold reader meets the bad id first and the
disowning second. Put the verdict, the measurements, and the authorization in
the resolution, where they are read as current.
