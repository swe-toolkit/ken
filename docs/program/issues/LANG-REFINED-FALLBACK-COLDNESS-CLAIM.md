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

## Adversary hunt `evt_6r8qxhyn3hcb6` on `294fceac` — ACCEPTED, and it adds two deliverables plus a number

**Triaged by the Steward; re-verified against `main` `7543ddcc` before filing.**
The hunt was aimed at `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA` and **cleared that
candidate**, but what it turned up belongs to this node's arc.

**The candidate consumed zero frame bytes, measured both ways.** `objdump` of
`register_prelude`'s prologue — a probe loop, so it states its frame exactly —
reads `sub $0x31000,%r11` **both with the bracket and with the bracket removed
and rebuilt.** The two `BTreeSet`s, the `difference`, and the `if` added
nothing.

### The number nobody in the stack arc has stated

**`register_prelude`'s own frame is 200,704 bytes (~196 KiB), and it is live
beneath the cascade** — it is the function that calls `register_decimal_char`
(`prelude.rs:1007` on current `main`; the footprint node's localization cited
`:984`, which has drifted).

| quantity | bytes |
|---|---|
| margin after the footprint fix (`D3` bisection) | ~98,304 (~96 KiB) |
| `register_prelude`'s single frame | **200,704 (~196 KiB)** |

⇒ **One function's frame is more than double the entire headroom that node
bought.** The footprint arc's numbers are all about the *recursion* — `check` at
14,744/call, `check_match_dependent` at 33,352 → 30,232, the cascade at ~1010 →
~915 KiB. **The ~196 KiB the caller contributes is a separate additive term and
it is stated nowhere.**

**This is not a defect and nothing regressed.** It has presumably been that size
for a long time, and it is precisely why *"the margin was already this thin"*
was the right diagnosis. **What it changes is the shape of the remaining
problem:** the next straw need not be in the cascade or in any recursive frame.
It can be one more local in `register_prelude`, which is the largest single
consumer on the path.

**Not measured, and do not assert it either way:** whether 200,704 is a *recent*
number. The delta for that candidate is zero; **the trend was not run.** One
`objdump` of the same symbol at an older SHA decides whether this needs a node
or just a recorded number.

### `D5` — the bracket's failure message names the wrong four

`prelude.rs:492` reads:

```
prelude List combinators (map/fold/zip/filter) must contribute nothing to the
trusted base: expected {}, got {GlobalId(N)}
```

**The one declaration the file explicitly names as trusted-base-growing has its
natural insertion point inside the bracket.** The comment at `:447-450` says
`sort` and `unfoldUpTo` are *"deliberately not here"* because `sort`'s
`is_sorted ∧ Perm` obligation would enter as an undischarged postulate — and
*"here"* is the bracketed range, which opens at `:460` and closes at `:485`.
Add `sort` where its own exclusion note sits and the guard fires as a hard
`ElabError::Internal` out of `ElabEnv::new()`, so **every** elaboration fails,
**naming four innocent declarations.**

**The mechanism is right and fail-closed; the diagnosis sends the author to the
wrong names.** One clause: name the bracketed range rather than the four
spellings.

### `D6` — the bracket's population is positional and the comment says it is nominal

*"Bracket exactly these four declarations"* is true today and pinned by nothing.
Inserting a declaration between the endpoints silently enrols it; moving one out
silently drops it. **One clause saying the bracket covers whatever lies between
the markers** — which is both what it does and what makes it robust.

### What the hunt cleared, recorded so it is not re-hunted

Between the two endpoints there is nothing but the four `elaborate_decl` calls
and their `?`s. An early error return skips the check but also fails
`register_prelude` entirely, **so there is no path where the delta is silently
unchecked while elaboration proceeds.** `register_prelude` runs once per
`ElabEnv::new()`, so no double entry. And the check is not vacuous:
`prover.rs:493`'s `emit_unknown_hole` registers under an auxiliary name during
elaboration, which is what makes an empty expected delta an assertion rather
than a theorem.

**`D4` is NOT discharged by this hunt.** The `objdump` run here was of
`register_prelude`, not of `check_match_dependent_refined_fallback`. `D4` still
needs its own read.

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

**`AC-6` — `D5`'s repaired message is checked by firing the guard, not by
reading it.** Add a trusted-base-growing declaration inside the bracketed range,
confirm the new message points at the range rather than at four names, and
restore. A diagnostic rewritten without being fired is not known to be an
improvement.

**`AC-7` — the ~196 KiB `register_prelude` figure is recorded, not acted on.**
It is context for the stack arc and this node does not shrink it. **Do not open
a stack investigation from it**; if the trend read is worth running, that is a
separate node.

## Sizing

**`S`.** `D1`-`D3`, `D5` and `D6` are comment and message-text edits in two
files; `D4` is a single `objdump`. The one-hour target applies comfortably.
**If `D4` turns into a stack-behaviour investigation, stop and report** -- that
is the successor `AC-4` names.

**Grew from four deliverables to six on Adversary hunt `evt_6r8qxhyn3hcb6`,
and stayed `S`** because every addition is a clause. If it stops feeling like
`S` once you are in it, that is a sizing report to the Steward, not something
to absorb.

## Not this node

- **Not a revert or re-tune of the extraction.** See `AC-4`.
- **Not a revision of `elab.rs:1002`'s 4 MiB figure**, which needs a peak-usage
  run on the deep source that nobody has done. `D2` records the gap's closure,
  not a new bound.
- **Not tuning `MAX_SHIFT`** or touching `decimal_char.rs`.
- **Not a stack investigation off the ~196 KiB `register_prelude` frame.** See
  `AC-7`. Recording the term is the deliverable; shrinking it is not, and the
  trend read that would decide whether it needs a node was not run.
- **Not a change to the bracket's mechanism or its population.** `D5` and `D6`
  repair a message and a comment. The guard is fail-closed and correct, and the
  four bracketed declarations stay exactly as they are.
