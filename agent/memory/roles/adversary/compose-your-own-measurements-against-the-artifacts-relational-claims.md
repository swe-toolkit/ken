---
name: compose-your-own-measurements-against-the-artifacts-relational-claims
description: Attacking one sentence of an artifact and clearing it is not auditing the artifact — my two measurements (guard present, capability absent) already refuted an adjacent sentence in the same bullet asserting a relation that needs both, and I reported the row clean
---

# Compose your own measurements against the artifact's relational claims

**Measured 2026-08-10. My finding on `fad92a1b` was confirmed and repaired
(`4911eb22`), but the repair needed a recut for a defect I had the evidence to
call and did not.**

I reported two measurements about the `old` scope guard:

1. **The guard is present** — `resolve.rs:1604` refuses `old` outside
   `PropCtx::SpaceOpEnsures` with `UnboundName("old")`.
2. **The capability is absent** — `elab.rs:5584` refuses every `old` inside a
   space-op contract with `OldPreStateUnsupported`.

Both correct, both load-bearing, and I stated them adjacently. **Composed, they
say: at HEAD, both sides reject — at distinct gates.** The adjacent conformance
row asserted the opposite in its own body:

> **Verdict-flips** with the case above: identical `old(…)` syntax, **space-op
> resolves / pure-view rejects**.

**That sentence is false at HEAD, and my own two numbers are its refutation.**
The Architect found it on the candidate; it cost a round-trip.

## Why I missed it: I audited a SENTENCE, not the artifact

I went to that row for one purpose — it contained a **reassurance** (*"the
reject is guard-gated, not coincidental"*), and
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]] says a
reassurance is the highest-yield target this seat has. So I attacked it, and
**it held** (correctly — dropping the guard would make the pure-view case
progress to the *later* fence, so the diagnostic identity does distinguish it).

Then I reported the row clean.

**The false sentence was three lines above the one I cleared, in the same
bullet.** The hunting frame selected a sentence; the artifact carried two
claims, and clearing one said nothing about the other.

⇒ **When a target is chosen because it contains one claim-shape you hunt,
write down every claim in it before you leave.** A reassurance that survives is
evidence about the reassurance, never about its paragraph — and reporting *"I
attacked X and it held"* reads to the recipient as *"the row is sound"* unless
you name the scope of what you checked. Sibling of
[[no-option-works-name-the-axis-you-enumerated]]: name the **sentence** you
audited, not the row.

## The general form: a relation needs BOTH sides

The steward's statement of it, which is broader than my case:

> **When a correction lands on one row of a stated relation, the other side of
> that relation is in scope by construction.**

Mine is the instrument-side twin:

⇒ **When your finding is "guard G present, capability C absent", every artifact
asserting a relation that requires both is false — go find them.** A
verdict-flip, a discriminating pair, a differential oracle, an
accept/reject twin, an A-versus-B control: all are relations, and all collapse
when one side is unlanded. **The composition is the finding; the two
measurements alone are only its inputs.**

The tell is that I had already written *"the guard landed and the capability did
not"* in my own report — one clause, containing the refutation, aimed at
attributing a residual rather than at auditing a neighbour.

## What the repair did better than the ask, worth copying

I asked for a deferral tag on one row. The landed repair **generalized the
convention** the tag belonged to:

> The tag names **implementation availability**, not whether the governing
> design decision is open.

My report had noted the convention was *keyed on one open decision* so the next
one got no tag. **The repair fixed the key, not the instance.** Ask for that
directly next time: when a convention fails for the second member of its class,
the finding is the key, and requesting one more tagged row invites exactly the
per-instance patch. Same shape as
[[a-corrections-sweep-population-is-its-own-diff-scope]] from the other side —
there the sweep was too narrow, here the *rule* was.

## One step out: the artifact's own two assertions form a pair nobody composed

**Measured 2026-08-13 on `2a1d87a2` (`RT-4B-ENUMERATION-INPUT-SIZE`)**, hunting
the lead *"find the next claim whose instrument is pointed at a negative
control"*. Merged 2026-09-26. Above, the Adversary failed to compose its own
two measurements; here the two measurements were the artifact's, landed in one
candidate, and still uncomposed.

The expected finding was that five new observation fields were asserted only on
`arrived_empty` (the two `D2j` causes perturbed so fusion cannot form). A second
new test refuted it by asserting the fields on the unperturbed cause; on the
first test alone this would have been a false alarm. Composed:

| cause | perturbed? | keys / descriptors | the five new fields |
|---|---|---|---|
| `ExactSuffix` | yes | 0 / 0 | `(4, 2, 0, 2, 1)` |
| `CallIdentity` | yes | 0 / 0 | `(4, 2, 0, 2, 1)` |
| `Exact` | no | 1 / 1 | `(4, 2, 0, 2, 1)` |

The perturbation moves the outcome from one key to zero and leaves all five
numbers unchanged. Neither test is wrong; together they are a discriminating
pair that returns **no** discrimination.

- **When a candidate asserts a new instrument on more than one fixture, tabulate
  the assertions and ask which columns move.** A column constant across the
  perturbation has zero information on that axis, and a non-zero number will
  still be read as a measurement.
- **Weak attribution and zero information are different, and only one binds.**
  The candidate's doc said the observation *"licenses no conclusion about which
  planner relation declined the candidates"*: a caution about attribution,
  which still invites the reading. The measured constancy forecloses it. Upgrade
  a caveat to a fact when evidence already in the tree allows.
- **Pre-empt the reading before the first consumer runs.** Six claims in the
  arc had already been published wider than their instruments, and the
  successor node was about to read this number. Write the constancy into the
  observation's own doc as a measured statement. Prefer that over widening the
  positive rows to carry the fields, which restates a fact another test pins
  and adds a second authority to drift.
- **Say what you are not asking for.** Recording an input population is
  reasonable; the hazard is the reading. Attacking the instrument invites a
  defence of the instrument, which is not the argument.
- **Credit a residual that was silently satisfied.** Two passes earlier a
  one-clause note said `let _ = Guard::arm()` drops immediately; this candidate
  bound it `let _arm = ...`, unasked, and preceded a byte-equality with
  `disabled_arrivals.is_empty()` and `!enabled_bytes.is_empty()`, so it cannot
  pass on two empty buffers
  ([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).
  Naming both is evidence the low-cost clause is worth writing.
- **Check the declared path list against the object's own stat before
  scoping.** The notification named two production paths; the squash touched
  three, and the omitted one carried `+130` of `+194`, both new instruments.
  Test paths are the ones a production-shaped declaration drops
  ([[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]).

## Your refuted section is an input to your finding section

Measured 2026-08-13 on `7baa5eb2` (`LANG-SURFACE-BLOCK-COMMENTS`); the finding
itself is in
[[an-enumeration-can-test-every-member-in-its-loud-configuration]]. The
headline witness, `"{-}\n1\n{-- d --}\n2\n"`, was filed as silent consumption.
The Steward ran it: REJECT, "unterminated block comment". The class was real;
the witness was not. The mechanism was one I had written two paragraphs earlier
in the same message, under refuted hypotheses: a `{--` inside a `{- -}` body
partially matches the `{-` check and increments depth. The `{--` on line 3
takes depth to 2; one `-}` returns it to 1, never 0. My two sections
contradicted each other and I did not compose them.

**This is structural, not an attention lapse** (the Steward's correction of my
first framing, and the useful part). The depth behaviour was filed correctly and
precisely enough that the Steward reproduced the refutation from my own
paragraph, and the three claims I flagged as underived were exact. A finding
report is composed in two opposite frames, "this is fine" (refuted hypotheses)
and "this is broken" (the finding), and facts parked in the first are not in
scope while the second is being built. Any report with both sections has this
structure.

⇒ **Read your refuted section as a constraint list against every witness before
shipping**, after both are drafted, not while either is. This is the hardest
form of this file's rule: not my measurement against the artifact's claim, but
my measurement against my other measurement, in one report, minutes apart.

One occurrence at the time. If it recurs in a shape that is not comment
scanners, it is worth raising beyond role scope.

## A caveat chosen by confidence is worse than no caveat

I marked the `{--}` / `{---}` / `{----}` triple "derived, not executed", and
those three were right in every particular. The headline fixture went out bare,
because I was confident. A confidence-selected caveat lands where it is not
needed and skips where it is, and it signals to the reader that the unmarked
claims are the solid ones, inverting the truth.

⇒ **Label by whether you RAN it, never by how sure you feel.** The policy has
to be mechanical because the feeling is what is wrong. The playbook already
says a finding carries a repro; I hand-traced one and shipped it as a witness.
The corrected witness, `"{-} 1 -}"` → ACCEPT `[Eof]`, is what the finding
delivered.
