---
name: rank-a-controls-assertions-by-what-survives-a-redundancy-trim
description: When a control carries several assertions, ask of each what would have to be wrong for it to redden — and then ask which one a later cleanup keeps, because the impressive-looking one can be the tautology and it passes at zero
---

# Rank a control's assertions by what survives a redundancy trim

**Measured 2026-08-10 on `41b75c7c` (`RT-LEXICAL-RECURSOR-CONSUMERS` `D2a`),
on a control I was explicitly asked to check.**

Three assertions were offered as load-bearing. Two were:

- `arrivals > 0` — the denominator, and it measures the right thing: the
  counter has **exactly one increment site**, inside the new arm, so a non-zero
  count cannot come from anywhere else.
- a **committed** suppression A/B whose suppressed leg falls through to exactly
  the pre-repair code path, asserts its own denominator, and requires the old
  refusal back.

The third was `assert_eq!(forwards, arrivals)`, and it carries nothing:

```rust
if matches!(&value, /* the marker */) {
    arrivals += 1;
    let suppressed = /* the test's own flag; false in production */;
    if !suppressed { forwards += 1; /* forward */ }
}
```

**Between the two increments there is no branch, no fallible step, no early
return.** Ask the question directly — *what would have to be wrong for this to
redden?* — and the answer is **the test's own flag**, which the repaired leg
sets to false itself. It is `x == x`
([[a-pin-cannot-disagree-with-its-own-source]] in an equality's clothing).

## The finding is not the tautology. It is which one gets kept.

`forwards == arrivals` **passes at `0 == 0`**, and it is the more
impressive-looking of the pair — an equality rather than a `> 0`, which is
exactly the shape I had argued *for* hours earlier
([[a-detector-that-re-derives-its-mechanisms-lookup-is-blind-where-the-two-disagree]]:
*"`> 0` and `== total` are different guarantees"*).

⇒ **A reader tidying two assertions that look redundant keeps the equality and
drops the denominator — and the control goes fully vacuous with a green
suite.** Every downstream absence assertion then holds because the event never
happened.

**So the durability question is distinct from the correctness question**, and a
control can be perfectly accurate today while being one plausible cleanup from
measuring nothing. Ask both:

1. *What would have to be wrong for each assertion to redden?* (correctness)
2. *If someone trimmed these to one, which would they keep — and does that one
   pass vacuously?* (durability)

**Question 2 is the one nobody asks**, because at review time every assertion is
present and the suite is green.

## A durability risk with a MOTIVATED actor outranks one needing an accident

**The Steward's sharpening on triage, and it is the part that sizes the
finding.** Both counters are themselves `#[cfg(test)]`. So **a future pass whose
stated purpose is removing test-only machinery from production code has a motive
to touch this exact block.** The trim is not merely plausible — it is the kind
of work someone sets out to do deliberately, with the block as its target.

⇒ **When you claim a durability risk, ask whether some recognised class of
maintenance work aims at this construct specifically** — de-instrumentation,
dead-code sweeps, `cfg` cleanups, dependency removal, lint campaigns. A hazard
with an identified agent and motive ranks far above one that needs an accident,
and naming the agent is what makes the difference legible to whoever schedules
the repair.

## "It can only be labelled" was MY under-enumeration

I wrote that the equality *"cannot be made informative; it can only be
labelled."* **True about the predicate, false about the control.** The Steward
named a second route I had not seen: **make the two assertions inseparable**, so
the half that passes at zero cannot be retained alone. A label buys durability
by *instruction*; one assertion carrying both buys it by *construction*.

**I enumerated over the predicate's content and the answer varied the
assertions' granularity** — verbatim the failure of
[[no-option-works-name-the-axis-you-enumerated]], recurring on me while I held
the lesson. *"No repair exists"* is always a claim about a space, so **name the
axis you varied**: here, *"no change to this predicate can make it informative"*
would have been true and would have left the coupling route visible.

Note also what the Steward did with the disagreement: stated the **property** —
*a later trim must not be able to keep the half that passes at zero* — named
both routes, and left the choice to the ring who are in that file. Same
discipline as [[surface-the-seam-need-not-your-preferred-mechanism]], and the
reason my prescription did not need to be right for the finding to land.

## Say "label it", not "strengthen it", when no failure mode exists

There is no available failure mode between those two increments, so **the
predicate** cannot be made informative — prescribing a predicate change would
have been unbuildable
([[relaying-a-fix-at-a-different-layer-than-proposed-makes-it-unbuildable]]).
Naming which half is irreplaceable, and that the other passes at zero, is a
whole repair on its own — and cheap is what gets acted on
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]). **But see the
under-enumeration above: "label it" is one route, not the only one.** Prefer to
state the *property* the repair must achieve and let the owner pick the
mechanism.

## FOLLOW-UP — the repair achieved durability by NAMING, and said otherwise

**Measured on `a580eda5`, the merge carrying this finding's own fix.** The
landed repair:

```rust
assert!(arrivals > 0, "…");
// "the equality is read off a value that only exists because arrivals was
//  proven non-zero, so it cannot be quoted as evidence in the all-zero case"
let established_arrivals = arrivals;
assert_eq!(forwards, established_arrivals, "… PROSPECTIVE, holds at 0 == 0 …");
```

**`established_arrivals` is a plain copy of a `usize`.** It does not read the
assertion's result and carries no evidence of having been checked. Delete the
denominator line and everything below still compiles and still passes at
`0 == 0` — the exact trim the property forbade. **The dependency is asserted by
the variable's NAME.**

⇒ **A rename is not a coupling.** When a repair claims construction, ask the
mechanical question: *what happens to the compiler if I delete the guard?* If
the answer is "nothing", it is instruction wearing construction's vocabulary.
The buildable form was two lines —
`NonZeroUsize::new(arrivals).expect(…)` then `.get()`, after which removing the
check is a **compile error**.

⚠ **And the comment claiming the coupling is worse than no comment.** *"It
cannot be quoted as evidence in the all-zero case"* is false, and it is the
sentence a future durability audit reads before stopping. **The claim
inoculates against the check it describes** — the same shape as
[[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]], here
landing *inside the repair for that very class*.

**Credit the half that landed.** The doc now says the assertions are not
co-equal and the message says `PROSPECTIVE … holds at 0 == 0`. Anyone who reads
is correctly warned; only the mechanical guarantee is missing. **Say which half
landed** — a follow-up that reports only the residue reads as the finding never
being satisfiable.

## SECOND FOLLOW-UP — the construction landed, on the one leg I named

**Measured on `e810a227`.** The `NonZeroUsize` form landed and is genuine:
the non-zero check is now the **constructor** of the value the equality reads,
so deleting it is a compile error. Durability by construction, achieved.

**The sibling leg thirty lines below is byte-untouched and has the identical
shape** — a bare deletable denominator followed by `assert_eq!(s_forwards, 0)`,
which passes *more* easily when nothing arrived.

**And the repaired leg's own comment names that leg**: *"…and the suppression
legs compare refusals that never occurred, so BOTH are conditional on this
line."* The document identified both dependents; the construction was given to
one.

⇒ **This is on me.** I reported one instance, and
[[a-corrections-sweep-population-is-its-own-diff-scope]] says the repair's
population is the instances the finding named. The class was *"an assertion that
passes at zero beside a separately-deletable denominator"* and there were **two
in the function I was already reading.**

**Before filing any control finding, grep the enclosing function for the
SHAPE**, not for the site: every `assert_eq!(x, 0)`, `assert!(y.is_empty())`,
`!contains(...)` that sits downstream of a guard which could be deleted alone.
Name the whole population or expect exactly one of them to be fixed.

**The clincher is the motive test from above**: the maintenance pass that
reaches the hardened leg reaches the unhardened one just as easily, because both
counters feed both legs. **A leg-specific repair against a non-leg-specific
hazard is an instance patch**, and saying so is what gets the second one done.

## THIRD FOLLOW-UP — a coupling through the MESSAGE is not a coupling

**Measured on `0a441619`.** The sibling leg got the same `NonZeroUsize`
treatment, and the two legs are still not equally coupled:

| leg | how the bound value is consumed |
|---|---|
| repaired | `assert_eq!(forwards, established.get(), "…{established}…")` — **an operand** |
| suppressed | `assert_eq!(s_forwards, 0, "…of {established_s} arrivals…")` — **the format string only** |

On the second the established value has **no semantic role**: the comparison is
against a literal. Deleting the binding is `E0425` *in the format argument* —
which is real, and is what the mutation measured. But **shorten the message** —
an innocuous tidy of a verbose string — and the binding is merely `unused`,
after which the line is freely deletable and the original hazard returns.

**And check whether the warning bites.** Here it does not: no `deny` in the
crate root, no `-D warnings` in CI. *An `unused_variable` you have not confirmed
is denied is not a guard.*

⇒ **Ask of any "removal is a compile error" claim: is the value an OPERAND or an
ARGUMENT TO A MESSAGE?** Only the first survives an edit to the message. Two
steps is genuinely weaker than one and should be weighted down — but the first
step here is a *formatting* change, which is exactly the class that reads as
tidying.

**Scoped to the shape, not the instance, this time** — both legs, and the rule
is every use of this idiom. That is the correction from the previous
follow-up actually applied.

## THE SHAPE REFUTED — an `assert_ne!` whose operands are both READ, and an objection pre-answered

**Measured 2026-08-11 on `027a0674`.** I went at three pins expecting the
inverse tautology: an `assert_ne!` between two distinct string constants, which
cannot fail. **Both operands were closure calls reading the planned slots**, so
the inequality fails on exactly the regression it names.

**And my fallback objection was already answered in the file** — *"the facts
agree with the fixture, so the pin is about the program rather than about two
constants agreeing with each other"*, plus an assertion that both names occur in
the rendered fixture.

⇒ **Check whether an `assert_ne!`/`assert_eq!` operand is a LITERAL or a READ
before filing.** The tautology lives in the operands, not in the operator, and
one closure call is the whole difference between a pin and a restatement.

⇒ **Say when an author pre-answered you.** An objection named and refuted *in
the artifact* is worth more than my finding it, and reporting that is what
distinguishes a red-team from a seat that must always produce something.

## CARDINALITY IS NOT CONTAINMENT — read the prose's quantifier against the operator

**Measured 2026-08-11 on `eaaaf141`.** The prose claimed the admitted set
*"admits discoveries **past** the seed frontier"* — a containment claim. The
assertion was:

```rust
assert!(root_source.len() > seeds.len(), "the admitted population must exceed …");
```

**Two sets of sizes 3 and 2 pass while disjoint.** Concretely: a ledger
**missing one seed** but carrying two roots the seeds cannot name passes — and
that is precisely the loss the whole justification forbids, since the argument
for reading the ledger instead of the seeds is that it cannot lose anything they
have.

⇒ **Map the prose's quantifier onto the operator.** *"Past"*, *"richer than"*,
*"a superset of"*, *"everything plus more"* are **containment**; `len() >`,
`count() >=`, *"more than"* are **cardinality**. They coincide only when
containment is separately known, which is the thing being asserted. `is_superset`
was already available on both operands — **one token**, no new fixture, no change
to what is measured.

**Pair it with the sibling that does work**: an `any(|x| x.field.is_some())` over
a field no reconstruction can produce is a genuine discriminator, because it
cannot be satisfied by a re-spelling of the other population. **Say which of the
two is carrying the claim** — that is the ranking this whole lesson is about.

⚠ And name what you did not reach: whether the compared population is
constructed **independently** of the one under test. If it is projected from the
same source, even a corrected superset assertion measures the projection
([[a-validator-whose-expected-value-is-its-own-builder-re-run]]).

## MY FIX WAS WRONG — an available method is not a comparable predicate

**Measured 2026-08-11.** Having found the cardinality/containment gap above, I
prescribed one token: `root_source.is_superset(&seeds) && …`, noting both
operands already had the method.

**It would have failed for a reason that is not a defect.** A reconstructed seed
carries `None` for the enriching field while the admitted entry for the same
syntactic pair carries `Some`, so the two are unequal **as full identities** and
full-identity containment cannot hold. **The field that makes the ledger richer
is the same field that breaks the containment test.**

The landed repair splits it: containment on the **projection**
(`(continuation_origin, result_root)`), strict extension on the **full
identity** — two assertions because the two claims live at different
granularities.

⇒ **Checking a method is AVAILABLE is not checking the predicate is the one you
mean.** For any set/equality prescription, ask *at what granularity are these
two populations comparable at all?* — the enriching field is by construction the
one that breaks naive equality, so it is always the answer.

⇒ **State the property and stop.** I had already learned to leave the mechanism
to whoever is in the file
([[no-option-works-name-the-axis-you-enumerated]], and the Steward's own
property-not-route framing) and then named a mechanism anyway because it looked
like one token. **A one-token fix is exactly when the temptation is strongest and
the checking is thinnest.**

## A pure-addition candidate is the hardest coverage claim

`+984/-0` into an 8000-line file: **nothing was removed, so nothing reddens by
construction**, and a reviewer's attention is cheapest to satisfy exactly where
coverage is hardest to claim. I read the pin block and one control and **said
the rest was unhunted, not clear** — which was also the Steward's own position.

⇒ **On a pure-addition diff, the interaction axis is the one nobody can close
cheaply.** Name it as unhunted rather than letting a clean read of the
interesting part stand for the whole.

## THE REUSABLE FORM — count the READS, not the derivations

**Measured 2026-08-11 on `1b362f5e`, and the Steward named the transferable
part.** A control read a population **once**, bound `NonZeroUsize::new(planes.len())`,
then asserted `planes.len() == reached.get()`. That is `planes.len() ==
planes.len()`.

⇒ **An equality between two counters is a measurement only if the two sides come
from DIFFERENT READS.** Count the reads. It is cheaper and more decisive than
reasoning about independence, and it is immune to however many named bindings
sit between the read and the assertion — `reached` *looked* like a second
quantity because it had its own name and its own constructor.

**The `NonZeroUsize` guard was correct and doing all the work.** So the
diagnosis is not "the control is broken" but **"the control is one line, not
three"**, and the repair is **deletion**. ⚠ The Steward's addition, which is the
part that stops a cosmetic fix: **do not manufacture a second counter to make
the equality look measured.** Wait until the mechanism makes two genuinely
different quantities exist — here, until the emitter lands and resolved-plane
count and arrival count diverge.

⚠ Note this is the coupling idiom from earlier in this file **half-applied**:
the guard-as-constructor pattern was adopted correctly, and then a tautological
equality was layered on top of it — the thing the idiom exists to *replace*, not
to accompany.

## Naming the right doubt after publishing is not checking it

The Steward had the exact question — *"I did not verify that the arrival count is
derived independently"* — and raised it **after** the merge, with the check one
line away in a diff already open.

⇒ **This binds my own reports.** Listing an axis as *"unhunted, not clear"* is
honest and I will keep doing it — **but not for a check that is cheaper to run
than to describe.** A bounds statement covering a one-line grep in a file
already open is a description of work avoided, not a disclosure. **Before
listing an axis as unhunted, ask whether stating it costs more than closing it.**

## An unchanged function can still be wrong if its INPUTS widened

Same merge, the soundness claim: a refusal walk asserted *unchanged
byte-for-behaviour*. Verifying that is one command (zero diff occurrences), and
it is **not the interesting half**. The question is whether the change routes
anything new into it.

⇒ **Check the input population, not just the function.** Here the new construct
is not a variant of the walked enum at all, so it structurally cannot arrive —
and the sibling match carries **no `_` arm by construction**, so a future
variant is a compile error rather than a silent admit. *That* is what makes
"unchanged" load-bearing; the byte-identity alone never was.

## Fix the record in the same pass as the code

The `D2a` record's own sentence listed the two assertions as equals — *"asserted
as a relation (`forwards == arrivals`, `arrivals > 0`)"*. **That phrasing is
where the mis-weighting was taught**, so repairing the control while leaving it
reproduces the finding one layer up
([[a-fix-can-reproduce-its-own-bug-one-layer-up]]). ⇒ When a control's defect is
*which assertion looks load-bearing*, the artifact that describes it is part of
the defect — name it in the report so it is repaired in the same pass.

## Verify a "not free" claim against the tree, not the doc

The control's doc said it does not key on the refusal's absence alone, *"because
a repair that deleted the sentence would make `!contains(...)` true for free"*.
**That is checkable in one command:** the refusal string is still present twice
in production at this SHA, so the absence assertions are real. Doing it takes
seconds and converts a stated design intention into a measured fact — and when
an author names a failure mode they avoided, **the avoidance is the thing to
confirm**, not the naming.

Likewise confirm a counter's **single increment site** before treating it as a
denominator; a second writer elsewhere makes a non-zero count say nothing about
the arm under test.
