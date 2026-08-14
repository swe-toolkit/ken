---
id: LANG-REFINED-FALLBACK-COLDNESS-CLAIM
title: "The doc comment justifying LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT's -3120 saving says the pow10 cascade's arms are all bare literals, which the generator refutes -- only the True arms are, every False arm is the next nested match, and the innermost is an application; the conclusion survives, the recorded reason does not, and it misleads in both directions"
status: ready
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_28m4mz6wy8fgn on squash 51b3a75c, triaged by the Steward. Findings 1-3 accepted; Finding 1 re-verified against origin/main before filing (decimal_char.rs:62-75 generator read directly, elab.rs:1868 comment read directly). Filed as a node rather than carried because the defect is a false sentence sitting on main in the artifact written to carry the justification."
---

## What this is

**A false load-bearing sentence, not a wording slip.**
`LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` landed a `-3120` bytes/call saving
whose entire safety argument is that the
extracted capability-3 refined fallback is **cold on this cascade**. The doc
comment recording that argument states something the generator refutes.

## Fixed inputs, measured at `main` `51b3a75c`

**The landed comment, `crates/ken-elaborator/src/elab.rs:1868`:**

> *"...and every one of that cascade's arms is **a bare literal** that the cheap
> unrefined attempt always resolves..."*

**The generator, `crates/ken-elaborator/src/decimal_char.rs:62-75`:**

```rust
if k > max_shift {
    format!("{unbounded_name} k")
} else {
    format!("match (eq_int k {k}) {{ True |-> {lit} ; False |-> {rest} }}",
            lit = pow10_literal(k), rest = rec(k + 1, max_shift, unbounded_name))
}
```

⇒ **At each of the 31 levels the `True` arm is a bare literal and the `False`
arm is the next nested `match`. At the innermost level the `False` arm is
`{unbounded_name} k`, an application.** Only half the arms are bare literals,
and no `False` arm is one.

**The correct form already exists in the Steward's M8 notification
(`evt_6a4988rt9vw2m`):** *"every arm is a bare literal **or a two-way `eq_int`
match** the unrefined attempt always resolves."* The second disjunct is exactly
the `False` arm, and it is what the landed comment dropped.

## Why this is worth a node rather than a passing fix

**It misleads in both directions**, which is the property that makes a false
justification worse than a missing one. A future author reading *"safe because
the arms are bare literals"* concludes that **a cascade with non-literal arms is
where this inverts** -- when the current cascade already has 31 non-literal arms
and is fine. The reader is steered away from the real discriminator.

**The conclusion is not in dispute.** The saving was measured, and the numeric
agreement (`3120 x 31 = 94.5 KiB`, `x 32 = 97.5 KiB`, bracketing the measured
~96 KiB cliff movement) stands independently. **This node repairs the reason,
and must not be read as reopening the change.**

## Deliverables

**`D1` — repair the clause at `elab.rs:1868`.** State the real discriminator:
the unrefined attempt resolves both a bare literal and a two-way `eq_int`
match, so the refined fallback is not entered on this cascade. **The repair is
one clause and it is already written in the M8 notification** -- do not
re-derive it, and do not weaken it to "mostly literals".

**`D2` — `elab.rs:1002`'s `REQUIRED MINIMUM` block, the same edit's natural
neighbour.** That block (from `LANG-PRELUDE-ELABORATION-DEPTH`) says its 4 MiB
figure rests on a peak *"measured only on the shallowest possible input ... a
floor beneath a deeper program's real cost, not a sufficient bound on it."*
**That stated gap was closed 850 lines below it** by this candidate's deep-source
measurement, and a reader of the block still learns only the shallowest input
was ever measured.

**Do not silently replace its number.** The block's *"worst observed 1,982,464
bytes"* is a **peak-usage** measurement; the new `1,998,848` is a **process-level
minimum-viable stack**. They are different quantities. `D2` records that the gap
is closed and cites the new measurement; **whether the headline figure itself
needs revising requires a peak-usage run on the deep source, which nobody has
done.** Say that rather than reconciling two numbers that do not mean the same
thing.

**`D3` — the sentence that belongs beside the coldness claim.** The claim is
about *which programs*, the change was measured on **one** program, and **the
inversion mode is a SIGSEGV on the guard page, not a red test.** A recursion
that does enter the fallback at every level pays the extracted frame at every
level. Nothing in the suite would announce it.

**`D4` — replace the Adversary's stated assumption with a number.** `objdump`
the new `check_match_dependent_refined_fallback` prologue. If its frame is
comparable to the `+6472` measured on the mirror extraction, a
fallback-entering recursion pays roughly `30232 + 6472 = 36704` against the old
`33352` -- about `3352` per level, **~104 KiB over 31 levels, which exceeds the
entire ~96 KiB this change bought.** That is one read and it converts `D3` from
a caveat into a bound.

## Acceptance criteria

**`AC-1` — the repaired clause is checked against the generator, not against
the old comment.** Cite `decimal_char.rs:62-75` in the review. The defect
arose from restating a justification without re-reading its source.

**`AC-2` — `D4`'s number is reported even if it is reassuring.** A small
prologue is the outcome that retires `D3`'s concern, and it is only worth
anything if it was measured rather than assumed.

**`AC-3` — no behaviour change.** Comments and doc text only, plus whatever
`D4` measures. `check_match_dependent` and its helper keep their current bodies;
this node does not re-tune the extraction.

**`AC-4` — the landed saving is not reopened.**
`LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` is merged and correct. If `D4` shows
the inversion is large, that is a **new**
node about which arm to extract, not a revert of this one.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`.** `D1`-`D3` are comment edits in one file; `D4` is a single `objdump`.
The one-hour target applies comfortably. **If `D4` turns into a stack-behaviour
investigation, stop and report** -- that is the successor `AC-4` names.

## Not this node

- **Not a revert or re-tune of the extraction.** See `AC-4`.
- **Not a revision of `elab.rs:1002`'s 4 MiB figure**, which needs a peak-usage
  run on the deep source that nobody has done. `D2` records the gap's closure,
  not a new bound.
- **Not tuning `MAX_SHIFT`** or touching `decimal_char.rs`.
