---
name: an-enumeration-can-test-every-member-in-its-loud-configuration
description: A prefix-relations enumeration ran every collision case at EOF, where the answer is always "unterminated" — the one input that never measures where a comment ENDS — so the fail-open configuration, a later closer swallowing a span of code silently, is untested for every member
metadata:
  type: feedback
---

# An enumeration can test every member in its loud configuration

**Measured 2026-08-13 on `7baa5eb2` (`LANG-SURFACE-BLOCK-COMMENTS`).**

A test named `ac5_prefix_relations_enumerated` covers the openers exhaustively —
`--`, `---`, `----`, `{-`, `{--`, `{---` — and one collision case, `{-}`,
asserted as `unterminated`.

**`unterminated` is the benign half of that case.** The same input with a plain
closer **later in the file** lexes **successfully** and silently eats everything
between. Executed by the Steward:

```
"{-} 1 -}"   ->  ACCEPT [Eof]     the `1` is gone, no error
```

**The discriminator is that the later closer must be a plain `-}`, not the tail
of a `--}`.** A `{--` inside a `{- -}` body partially matches the `{-` check and
increments depth (deliberate and sound: once inside a plain block only balance
matters). So `"{-}\n1\n{-- d --}\n2\n"` goes to depth 2, returns to 1 at the
single `-}`, and is rejected as "unterminated block comment": the `--}` converts
the fail-open case into the loud one. (The first version of this finding shipped
that second input as the witness, hand-traced; it was false.)

⇒ ***For any "where does X end" property, an at-EOF test measures that it does
not end*** — which is the single input that never exercises the property. **The
EOF configuration is where the defect announces itself; the terminating
configuration is where it is silent.** An enumeration that runs every member at
EOF has characterised the loud half of each and none of the quiet half.

**The tell is an enumeration whose rows all assert the same error.** Rows that
differ in input and agree in outcome are testing the *shared* consequence, not
the distinction the enumeration is named for
([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]] — and
here the rows are all negative checks). ⇒ **Ask what each row asserts, and if it
is the same string every time, the discriminating configuration is missing.**

**Assert on the RESULTING TOKEN STREAM, not on the error.** A silently
consumed span shows up as *missing tokens*; it cannot show up in a test that
only checks that an error occurred and what it said.

## ATTACK THE INPUTS THAT ARE COMPLETE IN THE NEIGHBOURING LANGUAGE

Two members were absent, and both are **openers that look closed**: `{--}` and
`{---}`. Dispatch is `{--`-before-`{-`, so `{--}` opens a **doc block** and
scans for the first literal `--}`.

**`{--}` is exactly what a Haskell-literate author writes for an empty
comment** — in Haskell it *is* empty (`{-` opens, `-}` closes). Adding `{--` as a
longer opener silently reclassifies it, and nothing in the tests or the doc
comments says so.

⇒ ***When a language adds a delimiter that is a PREFIX-EXTENSION of an existing
one, the inputs to attack are the ones that are COMPLETE in the old language and
INCOMPLETE in the new.*** That set is small, enumerable, and it is exactly where
a user's muscle memory produces a valid-looking token sequence with new meaning.
It is also invisible to an author reasoning forward from the new grammar,
because forward reasoning never generates the old language's spellings.

**Derived, not executed** (labelled as such at the time, and right in every
particular): the shortest empty doc block is `{----}` — `{--}` and `{---}` both
run on, because the residue after `{--` must be exactly `--}`
([[ask-the-tool-do-not-model-its-input-syntax]] — the tool is the oracle).

## REFUTE THE OBVIOUS ONES FIRST, AND SAY SO

Both scanners **fail closed** at EOF with distinct messages and spans anchored
at the **opener** rather than the EOF point; and the `{--`-inside-`{- -}` depth
case is deliberate with the reason at the site. ⇒ Two hypotheses dead on
reading, and reporting them is what distinguishes a thorough pass from a lucky
one. **But a refuted-hypotheses section is an input to the finding section**:
the depth fact above is exactly what falsified the first witness, and it was
written two paragraphs earlier in the same report. How to apply that, and why
a caveat chosen by confidence is worse than none, is in
[[compose-your-own-measurements-against-the-artifacts-relational-claims]].

**A witness in a filed finding becomes an AC**
([[a-pin-built-from-your-finding-inherits-your-enumeration]]), so a false
witness is a wrong specification: the implementer would have asserted silent
consumption, watched it error, and either "fixed" a non-defect or found the
frame wrong mid-turn. When someone corrects your witness, take the corrected one
as the finding; a replacement witness delivers the report that "your example is
wrong" would retire.

## A FINDING CAN SUPPLY THE RED AN ALREADY-OWNED NODE LACKS

A separate node already owns *"the two scanners' agreement is held by discipline
and tests, not by construction — a `{-`-before-`{--` reorder in one of them
diverges silently."*

⇒ **The missing rows would give that node a red it does not currently have**: a
reorder turns `{--}` from doc-opener into plain-block-opener, and the resulting
**token streams differ**. **Say this when it is true** — a coverage finding
that hands an existing durability node an executable witness is worth more than
the coverage, and it stops the two being triaged as competing asks.
