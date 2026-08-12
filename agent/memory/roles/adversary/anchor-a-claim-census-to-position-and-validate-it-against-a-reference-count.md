---
name: anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count
description: A substring census of a claim class silently mixes in the same vocabulary embedded in other constructs — anchor the pattern to the claim's POSITION, validate the corrected count against a known reference, and resolve ids by prefix with a positive control
scope: roles/adversary
---

# Anchor a claim census to position, and validate it against a reference count

**Measured 2026-08-10 on `53c09f9b` (`CI-L1-EXECUTING-COVER`). The finding was
real; my first measurement of it was not.**

Hunting for conformance-row claims the checker could not see, I grepped
`crates/` for `surface/[a-z0-9]` and got **157** mentions — of which **29 were
`//!`**, the module-level file-header form. That form is exactly what caused the
WP's three rejections, so it read as a strong finding: an entire uncounted
certifying population.

**All 29 were file-path citations** — `spec/30-surface/35-numbers.md`,
`conformance/surface/numbers/seed-numbers.md`. Not claims at all. **The claim
vocabulary is a substring of the path vocabulary**, so the census mixed two
populations and I nearly filed the union as a gap.

⇒ **Anchor the pattern to the claim's POSITION, not its substring.** The real
form is *doc content beginning with the id*, so the probe is
`^\s*/// surface/` — not `surface/` anywhere on the line. Re-measured that way:
`///` = 29, `//!` = **0**, `//` = 3. The `//!` hypothesis died and the residue
was the actual finding — three claims on `#[test]` functions written with `//`
instead of `///`, two of which resolve to **zero** headings anywhere.

This is the very shape I already carried from `CI-IGNORED-SWEEP` — *"a
comparison between two independently-correct numbers drawn from different
populations"* — reproduced by me, in a hunt whose subject was that same
mechanism. **Having the lesson did not prevent it; looking at the raw matched
lines did.** Print the matches, do not just count them.

## Validate the corrected count against a reference

The repaired `///` census came out at **exactly 29 — the count the checker
itself reported.** That agreement is what confirms my population is the same one
the instrument governs; without a reference number I would have had no check on
my own probe at all. **When auditing an instrument, look for a count it already
publishes and reconcile to it** — agreement validates the probe, and a
disagreement is either your bug or the finding, both worth having.

## Resolve ids by PREFIX, with a positive control

I first tested resolution with `^### <id>$` and got zero for all three
candidates. Real headings carry trailing markers — `(soundness)`, `[NODE-ID]` —
so an exact match under-resolves and manufactures false positives. Prefix
matching plus a **known-good id as positive control** (one that must resolve,
and did) corrected it: one of the three resolves fine and only two are false.

⇒ **Any id-resolution probe needs a positive control**, or a systematically
wrong matcher reports the whole population as broken and the report is
maximally confident and maximally wrong. Sibling of
[[construct-the-positive-control-yourself-and-calibrate-on-the-excluded-example]],
applied to the probe rather than to the artifact.

## CORRECTION — the reference-count rule above misled me one merge later

**A reference count validates your probe ONLY on axes where the two were
derived independently.** I reconciled my census to the checker's published 30
and called the agreement a validation. **My probe hardcoded `surface/` — the
same token the checker hardcodes.** Sharing that premise, the two could only
ever agree, so the reconciliation carried **zero** information about either.

The true population was ~74 claim lines across four namespaces; the checker
governs about 40%. I reported that its stated boundary "matches the real
population" when I had measured one namespace of it.

⇒ **Before citing agreement with an instrument as evidence, name the premise
you might share with it** — pattern, namespace, path glob, file-extension
filter. If it is shared, the agreement is a tautology dressed as corroboration
([[differential-oracle-is-blind-to-a-shared-premise]], where the shared premise
is *which population was searched*).

⚠ **The trap is that this reads as rigour.** Reconciling to a published number
*feels* like the disciplined move, and it is — but only across an axis the two
derived separately. **Derive your population from the artifact's grammar (any
`<ns>/<area>/<id>` on a `#[test]`), never from the instrument's vocabulary.**

## THE CORRECTION APPLIED — derive the vocabulary from the FILE, then check the word's other senses

**Measured 2026-08-10 on `33a0cef5`, verifying an un-gating count.** I was handed
three tokens to check (`remains gated`, `(future binding, gated)`, a blocker
name). **Searching those would have shared a premise with the verification I was
auditing and agreed for free** — the tautology this file already records.

Instead I grepped the file for the *concept* and read every hit, which produced
the real vocabulary: `remains gated`, `(future binding, gated)`,
`Status: blocked on`, `executes un-gated`, `implementation-gated`,
`Independently gated`. **No fourth status spelling had escaped** — and *that* is
the finding a token-keyed count cannot produce about itself.

**Then the residue was the yield.** Eleven lines from the qualifiers sat:

> `gated erasure admitting the generated support Elim …`

`gated` in the **mechanism** sense, not the status sense. ⇒ **The count was
correct because the qualifier has a distinctive parenthesised form, not because
the word is unambiguous.** Both the original verification and every future one
rest on that, and nothing in the artifact says so.

⇒ **When a census is safe only because of a form, name the form in the
artifact.** Otherwise the next person re-derives it from the bare word, merges
two senses, and — for a gating claim — errs in the direction nothing reds on.
**Read the other senses of your keyword before reporting a count as clean**; the
near-miss is usually inside the same region you just counted.

## Report the correction

Stating the bad first measurement in the finding costs three sentences and is
what lets the reader weigh the corrected number. A report that silently presents
only the fixed census asks to be trusted on exactly the step that failed.
