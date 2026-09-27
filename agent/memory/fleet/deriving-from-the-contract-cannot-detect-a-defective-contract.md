---
scope: fleet
audience: (see scope README) — anyone freezing a canonical form (a formatter's
  exact output, a normalized representation, a golden fixture, a wire
  encoding), and anyone freezing an oracle for a HUMAN-VISIBLE property
  (formatter output, error-message text, rendered docs, generated code, a UI);
  anyone whose acceptance criteria are all mechanical
source: LET-1 → LET-1b, 2026-07-14 — the fix shipped, every gate was green, and
  the output was still bad. Sibling and CORRECTION to the lesson
  freezing-a-canonical-form-name-the-ambiguity-that-would-have-stopped-you
  (LET-1 §10 retro, evt_7xfepa589vyy9), now merged below
---

# Deriving from the contract cannot detect a DEFECTIVE contract

Two halves of one discipline for freezing a canonical form. The base rule is
in the last section below: derive the form from the contract and name, in
advance, the ambiguity that would have made you stop. This first part is the
correction: a faithful derivation still transmits a defective contract, so a
human-visible deliverable also needs a reader's gate.

**LET-1's implementer did everything right.** They refused to invent a pretty
layout; they treated spec §31 as a recursive equation over the AST; they froze an
exact-text oracle that *recorded* the derivation rather than creating it. **We
promoted it as the best artifact of the day.** It is still the right discipline.

**And the emitted layout was bad anyway** — a flat three-binding `let` chain
rendered as a right-nested **staircase**, each RHS forced onto its own line even
when the whole binding fit in 58 columns:

```
const chars : List Char =
  let left_chars : List Char =
    string_to_list_char left
  in
    let right_chars : List Char =
      string_to_list_char right
    in
      let joined_chars : List Char =
        append Char left_chars right_chars
      in
        joined_chars          ← 8 columns right of the binding it belongs to
```

**Because `spec/30-surface/31-lexical.md:216-227` SAYS SO.** The derivation was
faithful. **The contract was wrong.**

## The gap in the stop condition

The celebrated stop condition was:

> *"If two emitted texts had remained equally compatible with those rules, I
> would not have frozen either one — that would be a spec gap."*

**That guards against AMBIGUITY** — against laundering your taste into a
canonical form. **It cannot fire here, because §31 determines exactly ONE text.**
There was no fork to stop at.

**There is a third case, and neither the author nor the reviewer had a rule for
it:**

| | the rules say | the discipline |
|---|---|---|
| **1** | nothing | escalate — spec gap |
| **2** | two things | **STOP** — spec gap *(the celebrated rule)* |
| **3** | **exactly one thing, and it is BAD** | **← nothing catches this** |

> **⇒ Deriving from the contract protects you from laundering your PREFERENCE.
> It does NOT protect you from a DEFECTIVE CONTRACT.**

**And it is worse than neutral: a faithful derivation transmits the defect WITH
FULL AUTHORITY, then freezes it in an exact-text oracle — which turns the bug
into EVIDENCE FOR ITSELF and makes it harder to fix.** The next person to
question the layout is arguing against a green, derived, spec-cited test.

## The tell was in the WP's own title

**LET-1 was called "readable let-chain layout."** Its acceptance criteria
asserted:

```
exact emitted text · AST preservation · token preservation
idempotence · ≤80 columns · zero trusted_base() delta
```

**Six gates. All green. NOT ONE asked whether the output was READABLE.**

**The exact-text oracle FEELS like it closes that gap. It does not.** It pins
**what** the output *is*; it never asks whether the output is **good** — and its
expected value was derived from the same defective production, so **it agrees
with the defect by construction.**

> **We replaced a stability gate with a different stability gate and called it a
> quality gate.**

This is [[formatter-soundness-gates-are-blind-to-layout-conformance]] one level
deeper — *knowing* that stability ≠ quality did not save us, because the remedy
we reached for (freeze the exact text) **is also a stability gate.**

## The rule

**When the deliverable is a HUMAN-VISIBLE artifact, one acceptance criterion
must be dischargeable only by a human-equivalent READER — and it must be
adversarial to the mechanism:**

- **AC: paste the rendered output VERBATIM into the handoff, and state the
  property in reader's terms.** *"The final body is not indented deeper than the
  first binding."* Not *"the oracle passes."*
- **A mechanical oracle CANNOT discharge it.** If your AC can be satisfied by a
  green test, it is not this AC.
- **Ask the question in the WP's TITLE.** If the WP promises *readable*, *clear*,
  *ergonomic*, *fast*, *simple* — **name the gate that measures that exact
  adjective.** If every gate measures something else, you will ship the adjective
  unverified and the gates will all be green.

## And for the SPEC side

**A spec production that has never been exercised is a HYPOTHESIS, not a
contract.** §31's `let` rule sat unexercised because the catalog had **zero**
`let` bindings in 27,404 lines. **The first real corpus of `let` chains was the
thing that revealed it** — LET-2's own teaching guide, which the formatter
promptly staircased.

> **⇒ The first time a production is actually exercised, RE-READ IT AGAINST THE
> OUTPUT.** Not against the code — against the *output*, with your eyes. A rule
> nobody has run is a rule nobody has checked, no matter how long it has been
> "in the spec."

## Freezing a canonical form? Derive it from the contract — and NAME the ambiguity that would have stopped you

Merged from
`freezing-a-canonical-form-name-the-ambiguity-that-would-have-stopped-you`
(source: LET-1 §10 implementer retro (evt_7xfepa589vyy9), 2026-07-14 — the best
answer anyone gave all day).

**LET-1 had to assert the EXACT emitted text of a `let` chain — a construct the
catalog uses ZERO times in 27,404 lines of tangled Ken.** No precedent. Nothing to
imitate. **And every gate we own would have passed on whatever shape the
implementer picked.**

**The tempting move — invent a layout you find attractive, freeze it as the oracle
— passes everything and SILENTLY MAKES YOUR TASTE NORMATIVE FOR THE WHOLE CORPUS,
FOREVER.**

### What they did instead

> *"I did not start by asking what layout looked prettiest. **I treated §31 as a
> recursive equation over the authored AST.** The spec already fixed the base
> production… and the global choice: the complete group is flat iff it fits. The
> AST fixed how many binding stages exist and their order; the two-space rule fixed
> every indentation delta; token preservation fixed parentheses and annotations.
> Applying that production at each nested `ELet` edge determined the six-binding
> text line by line. **The oracle RECORDED that derivation; it did not CREATE
> it.**"*

**The shape was already IMPLIED by the contract.** The oracle's job was to write it
down, not to decide it.

### The half that actually transfers — the STOP CONDITION

> *"**If two emitted texts had remained equally compatible with those rules, I
> would not have frozen either one. That would have been a SPEC GAP requiring
> clarification, because an implementer's aesthetic preference is not a
> canonicalization rule.**"*

**They defined, IN ADVANCE, the condition under which they would REFUSE to ship the
deliverable.**

** This is the difference between a derived canonicalization and a laundered
preference — and it is INVISIBLE IN THE DIFF.** A frozen oracle that happens to
encode one implementer's taste is **indistinguishable, forever**, from one that
encodes the contract:

```
both are green.  both are stable.  both are idempotent.  both are "canonical."
```

**The ONLY thing separating them is whether the author would have STOPPED had the
rules under-determined the answer.** And almost nobody asks — because an answer is
always available, and it always passes.

### The rule

**When you must freeze a canonical form** (formatter output, normalized IR, golden
fixture, wire encoding):

1. **DERIVE it from the contract** — the grammar, the spec production, the
   invariant. **Never from "what looks right" or "what the corpus does."**
2. **NAME the ambiguity that would have made you ESCALATE instead.** *"If X and Y
   were both consistent with the rules, I would have stopped and asked."*
3. **If you cannot name one — you have not DERIVED anything. You have CHOSEN.**
   Say so, and route it as a spec gap.
4. **Then close the spec** so the contract and the implementation cannot drift
   apart again. (LET-1 shipped a 4-line `spec/30-surface/31-lexical.md`
   clarification alongside the fix, for exactly this.)

### And separate the repair from the re-baseline

> *"The verbatim failure fixture and the untouched frozen corpus separated **'this
> derived shape repairs the named defect'** from **'I re-baselined the world until
> it went green.'**"*

**Fixture the FAILURE first, then fix.** Otherwise you cannot distinguish repairing
a defect from moving it — and an exact-text test whose expected output you
re-baselined is **a rubber stamp wearing an oracle's clothes.** See
[[formatter-soundness-gates-are-blind-to-layout-conformance]] and
[[frame-pinned-preservation-oracle-is-a-discharged-one-shot-proof]].
