---
name: mutation-prove-a-pin
description: How to prove a mechanical pin actually guards its property, by mutating the thing it watches and confirming it reddens. Load when running a mutation campaign against a pin, a control, or a detector. Covers mutation-provenance in both directions, detector-side versus population-side mutation, reset ordering, and reproduction recipes.
scope: tools
---

# Prove a pin by mutation

Authoring a pin is `pin-a-property` (`agent/playbooks/tools/pin-a-property.md`).
This file is the other half: **demonstrating that the pin you authored bites.**
They are different tasks at different moments — you author before the change
lands and you prove after — so they are separate skills. Section numbers below
continue that file's, and the `§` references point back into it.

## 10. Mutation hygiene

**A verdict from a mutation that did not apply is not evidence — and it looks
exactly like evidence.** The run compiles, the test executes, the output is
well-formed. **"The mutation ran" and "the mutation changed the subject, and only
the subject" are different claims, and only the second licenses a verdict.**

**One campaign of eight produced four distinct invalid kinds (2026-07-25) —
report each, never discard it:**

| kind | why its outcome is not evidence |
|---|---|
| **never applied** (bad anchor) | the test ran against pristine source |
| **broke the build** | nothing was measured |
| **inserted a comment** | the pin had nothing to catch, so GREEN is vacuous |
| **edited the DETECTOR along with its subject** | the oracle was rewritten to match its rewritten subject and reported success **on a sound pin** |

That last one is the dangerous shape: it **would have filed a spurious finding
against a correct detector.** The mutation respaced *every* occurrence of its
needle — including the copy inside the test's own string literal. Re-run against
declaration lines only, it reddened.

⇒ **Every mutation needs a provenance check before its outcome is recorded:**
assert the occurrence count **before** mutating; and where the result is **green**,
prove the change reached the compiled artifact (symbol present, binary mtime after
the edit). **Reporting the invalid rows is what makes the valid ones
trustworthy** — a campaign that silently drops them is indistinguishable from one
that had no failures.

## 10a. The provenance check itself fails in BOTH directions

**Measured 2026-07-25, on the very next candidate after the rows above.** A
provenance check reported `applied=False` for two mutations that **had** landed:
it re-counted the anchor **after** the replacement, and **the replacement string
contained the anchor.** The campaign would have discarded **two sound results as
unproven.**

**An instrument that certifies evidence can fail toward throwing good evidence
away, and that is exactly as corrupting as passing bad evidence through** — it is
merely quieter, because a discarded row leaves no trace to audit. The rule above,
read only in the false-positive direction (*"did it change the subject, and only
the subject"*), does not catch this.

⇒ **Count the anchor BEFORE the edit and compare against a PREDICTED
post-count** — do not re-match a needle the replacement may still contain. A
provenance check is itself a pin, so §6's positive control and §6a's
which-arm-fired question apply to it: **feed it a mutation you know landed and a
mutation you know did not, and confirm it distinguishes them.**

## 10b. Detector-side and population-side mutations are not interchangeable

**This is NOT the "edited the DETECTOR along with its subject" row above.**
That row is about mutating **both**. This is about mutating **the detector
instead of the population** — and it passes every check on this page, including
the provenance check, because the mutation genuinely applied and the intended
named test genuinely reddened.

Every control has **two** operands: the **detector**, and the **population the
detector is claimed to reach**. They answer different questions:

| you mutated | a redden proves | a redden does NOT prove |
|---|---|---|
| the **detector** (narrow its predicate, neuter an arm) | the detector is wired to **something** | that it reaches the population the AC names |
| the **population** (add a real instance to a real input) | the detector **reaches that population** | which arm of the detector fired (§6a) |

**An AC whose property is REACH can only be discharged population-side.** A
detector-side mutation on such an AC can redden for the entire life of a
detector that reaches nothing — which is precisely the defect the AC exists to
prevent, so the control has been made blind to its own subject.

**Measured 2026-07-26, `KW-ORACLE-CLOSURE`, and the report was TRUE.** The AC
row read *"widen one corpus file's occurrence set beyond a declaration head
(e.g. add `lemmas` in prose) — must redden."* The build ran a **detector-side**
mutation (*"head-only occurrence scan"*), it reddened, and the handoff correctly
said *"each control reddened its intended named test."* QA then ran the
**population-side** mutation the row actually specified — one line of prose added
to a real corpus file — and the suite came back **exit 0, 1 passed, 0 failed**.
**The occurrence predicate was still not reaching the corpus**, the exact
defect the WP existed to fix, with a green control sitting on top of it.

⇒ **Three obligations, all mechanical:**

1. **When you author the row, name the operand.** "Must redden" is
   under-specified; *"adding an instance to `<real input file>` must redden"* is
   not. **Necessary, and measured NOT sufficient — see the next block.**
2. **When you discharge it, quote the row and diff it against what you ran.**
   Do not repair a population-side failure by hunting for a detector-side
   mutation that reddens — the post-condition is the row's mutation, verbatim.
   And pair it with §6's **positive control**: without an arm showing a real
   instance being *found*, "found nothing" and "never looked" read identically,
   which is exactly what a reach failure looks like from the outside.
3. **Report WHICH OPERAND MOVED as its own field of the handoff** — not as
   something a reader can infer from the mutation you describe. See below for
   why this is the load-bearing one.

## 10c. The code seam names an operand too, and it wins

**The three `KW-ORACLE-CLOSURE` retros (2026-07-26) refuted the natural reading
of obligation 1.** I wrote that row, so I asked the ring the one question I could
not answer myself: **was the row ambiguous about which operand moves, or clear
and skipped?** — because those need **opposite** repairs, and a softened answer
would have had me fix the wrong one.

**QA answered: clear and skipped.** The row *did* name the corpus-side operand.
So "author it more precisely" is **not** the repair, and reaching for it would
have hardened prose that was already correct.

**The implementer supplied the mechanism, and it is the reusable part:** the
implementation seam in front of them was a `declaration_lines` helper. Mutating
*that* was **cheap, isolated, compile-preserving, and it reddened the correctly
named test** — four properties that each independently read as *"good control."*
In their words, they *varied the operand named by the code seam rather than the
operand named by the reach claim.* Note what is **not** in that sentence:
carelessness, haste, or a misread row. The seam you are standing in front of
**supplies a default operand**, that default is **not** the AC's, and it is
selected by locally sound reasoning.

⇒ **A property named at authoring time competes with an operand named by the
code, at the point of work — and the code is closer.** Sibling of *"a rule far
from the point of work does not fire"*: the row was correct **and** it was not
where the choice got made.

**And the leader seat cannot close this gap by reviewing harder.** Asked what was
visible at their seat that could have distinguished the first candidate from a
correct discharge, the leader answered **"nothing"** — they bound branch, tree,
scope and diff hygiene, and confirmed a named test reddened, *and none of those
facts say which operand the AC requires to move.* Do not install a
leader-review step here; it provably cannot work. That plain "nothing" is worth
more than a hedge would have been — it is what rules the wrong repair out.

⇒ **Which is why obligation 3 is a REPORTED FIELD.** Every AC→control handoff
carries **(the property · the operand that moved · the observed boundary)**
together, stated, not inferable. That makes the distinction visible at a seat
that otherwise has no instrument for it — and it is the only one of the three
obligations that changes what a reviewer can *see*.

## 10d. Mechanics: site, restore, reset order

- Apply each mutation at its **natural production site**, not at a convenient
  one; a mutation the real code path never reaches proves nothing.
- **Restore byte-identically** and verify with `git diff --quiet`.
  `git diff --stat` **always exits 0** and is not an emptiness test.
- **Commit the real fix before any mutation-proof reset.**
- When a resource cliff (stack, RSS, timeout) fires, **measure the base's
  MARGIN**, not just pass/fail — attribution needs the margin. And **fixing a
  cliff by raising a limit spends a detector**: name which one, and where its
  replacement belongs.

## 11. Reproduction recipes

If a pin rests on captured constants, a re-capture after the change would
produce byte-identical values — **so nothing distinguishes a genuine baseline
from a re-recording.** Record the base SHA, the probe names, and the exact
worktree + invocation, and **specify the sanctioned invocation verbatim**
(`scripts/ken-cargo`, targeted) or the recipe will document a procedure the
fleet is not allowed to run. **Demonstrate the binding; do not testify to it.**

## 12. Running a mutation campaign at the review gate

Promoted from `agent/playbooks/build/qa-test-design.md`'s Causality gate, which
is where this is applied: before Approve, demonstrate that breaking the claimed
mechanism at its seam makes the unchanged test fail with the expected
opposite.

- **Enumerate your probes by the STATE each one builds, then look for the
  missing cell (promoted ORACLE-VIS-*; four instances, three seats, one day).**
  The probes that **follow the diff** — the happy path and the error path —
  get written for free, because the change itself puts that state in front of
  you. The probe that has to **manufacture a violation** is *orthogonal to the
  change*, so it is the one nobody writes — **and its absence is invisible,
  because the suite still reads as complete.** A publisher gate shipped with
  three probes, two green:
  `green→PERMIT ✅ · red→PERMIT ⛔ · conflict→CANNOT-EVALUATE ✅`.
  Two of three passed **and the two that passed were exactly the ones
  exercising the code that had just changed**; the red probe — the only one
  that had to *construct* a violation — was the only discriminator. So don't
  ask *"did I test it?"*; **tabulate the states (satisfied / violated /
  unevaluable) and name which probe builds each.** An empty cell is the
  finding. Same shape as a `compile_fail` block that passes for any reason at
  all, and as a detector arm whose real job is *rejecting* a signal.
## 12a. Mutate to the property's nearest legal neighbour

- **Mutate to the property's NEAREST legal neighbour, not to an obvious
  break.** A mutation only proves what it varies, so the question is *which
  variation the check is blind to*. The invariant, which is not about
  programming-language syntax:

  > **The property is semantic; the check operates on a REPRESENTATION. The
  > nearest legal neighbour is where two representations denote the SAME
  > thing but differ in the part the check inspects.**

  That is where a check goes vacuous, and it applies to every substrate we
  gate — a TOML key that may be quoted or re-nested, a manifest defeated by
  an equivalent array spelling, a shell command line admitting different
  quoting, a JSON payload with reordered or aliased fields. Enumerate the
  spellings the **substrate** admits and mutate to the **worst legal one**.
  Then prefer **asking the substrate's own parser over running a matcher
  against its text** — the compiler is the Rust instance of that rule, not
  the rule itself. A text matcher's blind spots are exactly the forms you did
  not imagine, which is why they cannot be enumerated from the armchair.

  Worked example (promoted ORACLE-VIS-PACKAGING; **caught by runtime-qa**,
  who constructed it rather than inheriting it): the obvious widening
  `pub fn f(` went red correctly, while the **legal line-split** form —
  `#[cfg(test)]` / `pub` / `fn build_process_starter_executable_artifact(` —
  compiled clean and **passed green, 13/13** over a genuine widening, because
  the text pin matched visibility against the *same line's* prefix, which is
  empty on the `fn` line. Same denotation, different representation, and the
  difference sat precisely in the part the check read.
## 12b. Build breaks, stale inputs, and inherited mutations

- **A mutation that breaks the BUILD proves nothing** — the checks then
  "pass" against rubble. Confirm the crate still compiles under the mutation;
  use a compile-preserving stand-in (e.g. a `#[cfg(not(test))]` sibling) when
  the direct edit would collide with a gated declaration.
- **When a mutation passes where it should FAIL, suspect a STALE INPUT
  before you doubt the mutation.** Freshness is a **third axis**, independent
  of correctness and of the positive control: a control proves the harness
  *works*, never that it read **current** code. A probe selecting among 15
  accumulated rlibs by filename hash reported on hours-old source **with every
  signal healthy, the positive control included**. Check *which artifact the
  probe actually compiled against* before concluding the property holds.
- **Construct your OWN mutation before you run theirs — and if you can only
  re-run theirs, SAY SO IN THE VERDICT.** **Re-running is not re-deriving.**
  A QA that re-runs the
  implementer's mutation inherits the implementer's *vantage* — including the
  forms they did not imagine, which for a representation-matching mechanism is
  the **entire failure surface**. That is the one place a mutation proof
  degrades silently into agreement, and it leaves no trace: the verdict reads
  identical either way.
  ⇒ **Agreement counts as corroboration only when neither seat inherited the
  other's premise.** So derive the violation independently from the *property*,
  not from their test. (ORACLE-VIS-PACKAGING: QA mutation-proved across three
  axes with its own construction, and the finding that blocked the WP was a
  form the implementer's own probes could not have suggested.)
  The second clause is the load-bearing one — **an inherited mutation is not a
  defect, but an inherited mutation reported as an independent one is.**
  Naming the limitation makes the degraded case visible instead of
  indistinguishable from the real thing.
