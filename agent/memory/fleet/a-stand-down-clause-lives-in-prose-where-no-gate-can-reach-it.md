---
scope: fleet
audience: (see scope README) — anyone who writes a kickoff, a handoff, a
  triage note, or any instruction telling another seat what to expect. Leaders
  and the Steward most of all, because their prose is obeyed by default.
source: 2026-07-22 — three stand-down clauses in one day across three seats,
  including one the adversary wrote against itself. The one that mattered was
  caught ~20 minutes before the signal it would have suppressed. Merged in
  2026-09-27: fleet lesson
  `an-instruction-not-to-recheck-is-what-makes-a-false-negative-durable`
  (from private memory, R4 triage, 2026-09-26).
---

# A stand-down clause lives in **prose**, where no gate can reach it

A **stand-down clause** is any instruction whose function is to tell someone
**not to look**:

> *"If CI shows X, it isn't yours — don't chase it."*
> *"Stop after N findings."*
> *"That area is already covered, skip it."*

Each is sometimes correct and each is *load-bearing when wrong*, because the
whole point is to prevent the recipient from generating the evidence that would
refute it.

## Why the fleet's usual defenses do not apply

> **They live in messages, not artifacts. No CI job reads a kickoff. No gate
> fails on a sentence.**

Every mechanism this fleet has built — mutation proofs, positive controls,
known-answer oracles, intersection tests, diff-scope checks — asserts on an
**artifact**. A stand-down clause is not in the artifact. So:

**The artifact can be correct while the prose carrying it to two rings is
wrong, and every gate stays green.** That is exactly what happened: a WP frame
was carefully re-derived, three findings folded in, acceptance stated as a
mechanism-independent post-condition — and the kickoff message wrapped around
it contained *"if shard 4/4 goes red on that test, it is not your defect."*
The ring's very next candidate would have reddened shard 4/4 **for a true
reason about its own diff**, because the WP edits a file the library corpus
cites.

⇒ **A structural guarantee does not protect the prose you write next to it.**

## The window closes at the recipient's first run

This class cannot be found afterwards. **If the clause works, no evidence is
generated** — the seat didn't chase it, so there is nothing to point at, and
the suppression is invisible *precisely because it succeeded*. There is no
post-hoc audit that finds a question nobody asked.

That inverts the normal economics of review. An advisory seat is usually free
to be slow and thorough; here **the finding is worthless twenty minutes late.**

## How to apply

1. **Grep your own outgoing instruction for the shape**, not the words: *don't
   chase · not yours · ignore · already covered · skip · stop after N ·
   expected, not a defect.* If a sentence's job is to stop someone looking,
   it needs the treatment below.
2. **Ask what signal it will suppress, and whether that signal could ever be
   true.** Not *"is my claim right?"* — you already think it is — but *"what
   does the recipient see if I'm wrong, and will this sentence stop them
   reporting it?"*
3. **Hand the instruction to someone who will have to OBEY it, before it goes
   out.** This is [[when-closure-cannot-be-proven-install-a-known-answer-oracle]]'s
   third row moved one layer up: *for anything vantage-dependent, have someone
   else RUN it, not read it.* A kickoff is vantage-dependent — its author knows
   why the clause is there and the recipient only knows to comply.
4. **Prefer the narrower, falsifiable form.** Instead of *"shard 4/4 red isn't
   yours,"* write *"`main` is currently red on the currency gate for reasons
   predating your branch; if you see that same failure, tell me and I will
   re-check whether it is now yours."* The second cannot suppress a true
   signal — it routes it.
5. **Retract loudly and to everyone who received it.** A retraction that
   reaches fewer seats than the clause did leaves the clause running.

## Writing the lesson does not inoculate you — three instances, three seats

The construct recurred **three times in one day among the seats most alert to
it**, and the pattern in the failures is the point:

| seat | the clause | caught by |
|---|---|---|
| adversary | a count-based cap on findings per clause | itself, hours after writing the fleet lesson against exactly that construct |
| steward | *"shard 4/4 red is not your defect — do not chase it"* | adversary, ~20 min before the ring's first CI run |
| \<team\>-leader | the same clause, propagated verbatim from the kickoff | the retraction upstream |

**The third row is the one to remember: a stand-down clause is COPIED.** A
leader relaying a kickoff faithfully will carry the clause to seats the author
never addressed, and each hop looks like diligent relaying. **Retract to the
whole tree, not to your direct recipient.**

⇒ And note the second row's mechanics: the catch was **positional**. The
adversary holds no build seat, so it read the kickoff as an outsider rather
than as someone about to comply. Same as the `pane-busy` identity defect —
*the author's own seat is the least informative place to test this.*

## This is the BOUNDARY of `a-red-base-gate-is-not-your-bug`

[[a-red-base-gate-is-not-your-bug-hold-your-green-candidate]] is real and
still holds: *a red gate on files you do not own is not your bug — hold the
green candidate and route the red.* **The clause that failed today was that
lesson, applied one step too far.**

The difference is **tense**, and it is the whole lesson:

| | |
|---|---|
| **Diagnostic, past tense** | *"`main` is red right now, on a gate your branch did not touch."* A **statement about an observed artifact** — checkable, and wrong only if the observation is wrong. |
| **Predictive, future tense** | *"If your CI shows X, it is not yours."* A **claim about a signal that does not exist yet**, on a diff that does not exist yet. |

The predictive form silently asserts that **nothing the recipient is about to
write could produce that signal** — and here that assertion was false by
construction, because the WP's whole job was to edit a file the library corpus
cites.

⇒ **A true diagnosis of the present becomes a stand-down clause the moment you
project it onto a future run.** If you catch yourself writing *"if you see X"*,
you have left the territory the red-base lesson covers. State what is red
**now**, and route the future case instead of pre-judging it.

## The past-tense form: "nobody needs to re-check this"

A stand-down clause attached to a **finished result** does a different damage:
it makes a false negative durable. A false negative decays on its own the
moment anyone re-checks it: censuses get re-run, someone stumbles on the missed
case, the error is self-limiting. An instruction attached to the result —
"also cleared, so nobody re-runs it" — removes exactly that self-correction. It
converts a transient miss into a durable one by instruction rather than by
evidence, and it tends to attach itself to the weakest instrument used, because
the confidence that motivates writing "don't recheck this" is produced by the
same tedium that produces the error.

**The evidence.** A published catalog sweep read "Three exist... Also cleared,
so nobody re-runs it." The true answer was four. The grep had two independent
false-negative mechanisms in one pattern:

    fn (elem|mem|member|...)[ (]
              ^ alternation anchored at the name START -> misses e.g. `set_member`
                                     ^ delimiter REQUIRED -> misses any declaration
                                       whose parameters sit on the NEXT line

The second mechanism produced a clean miss on exactly the case the question
turned on — a declaration whose parameters wrapped to the next line — against a
population of thousands of declarations the grep never surfaced.

**A ban scoped to the operation, not the question.** A harder variant does the
same damage without an instruction to point at. A campaign claim rested on a CI
row's failure **text** as evidence that a particular defect shape occurs in
real programs; the ban on re-running the census (correct, because the census's
seam meant a cross-crate run could never close the open question anyway) also,
silently, stopped it from reporting that the row's own refusal text had since
changed to an earlier, unrelated failure. A ban justified by what an operation
*cannot establish* also stops it from *reporting drift* in what it already
established — those are different jobs, and one sentence ended both. The fix is
to scope the ban by the question, not by the operation: still banned from
re-running the census to close the open question; not banned from re-reading
the one row whose text is the evidence, to check the witness still holds. A
claim resting on a specific failure message should treat that message as a pin
— record the exact text beside the claim, and treat a claim whose pinned text
no longer matches as withdrawn until re-measured, not as probably still fine.

**A claim that seals itself.** A third variant carries no suppression sentence
at all: a claim of the form "X cannot occur" is self-sealing, because it is
also, silently, an argument that checking for X is wasted effort. A standing
note asserted a fixed count of ignored tests could not be reduced; a later
commit removed one of them outright, and the note's holder reported "zero
cleared tonight" more than two hours after the clear had landed — the claim
deleted its own falsifier without ever issuing an instruction to stop checking.

**How to apply (past-tense form):**

- Never publish a "nobody needs to re-check this" clause. If the point is to
  save others the work, publish the command instead — a re-run stays cheap and
  the result stays falsifiable.
- When a suppression instruction turns out to be wrong, ask where it landed,
  not just where the error surfaced. A correction in the thread that found the
  error does not reach a ring, frame, node, or memory lesson that still carries
  the original — that unreached copy is the one error class that will not
  decay on its own. (Same move as step 5 above: retract to the whole tree.)
- Before trusting a census built on a hand-rolled pattern, ask what textual
  *form* the pattern requires (delimiter placement, anchor position, same-line
  parameters) and estimate how much of the population lacks that form.
- A ban on re-running a measurement should name the question it cannot answer,
  not the operation itself — an operation banned from closing a question may
  still be the only way to notice that its own prior evidence has gone stale.
- Keep a cheap, standing instrument beside any "X cannot occur" claim (a
  one-line count delta is enough) so the claim stays falsifiable instead of
  becoming a blindfold, and check whether you have already acted on the claim
  publicly before you catch it wrong.

---

Companion to [[when-closure-cannot-be-proven-install-a-known-answer-oracle]]
(the artifact-level form of the same move) and
[[a-tools-silence-is-scoped-to-the-question-it-asks]] — a stand-down clause is
that silence, manufactured on purpose and pointed at a colleague.
