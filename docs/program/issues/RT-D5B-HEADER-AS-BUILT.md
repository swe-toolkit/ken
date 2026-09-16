---
id: RT-D5B-HEADER-AS-BUILT
title: "Correct immediate_bridge.rs's module header to the as-built state. Slice 3 falsified two of its sentences: it calls the live wiring a deliberately absent successor while that wiring is on main at construction.rs:1460, and it explains the never-used warnings by 'nothing calls in from production' when production now calls in. Comment-only, one file. The durable form is a PREDICATE plus a dated roster: an item warns never-used exactly when it has no live root, and the seven diagnostics measured at 67684fa5d are a snapshot, not a promise."
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-D5B-LIVE-WIRING]
blocks: []
github: null
tier: T2
origin: "Steward, 2026-09-16. Filed at runtime-leader's request (shape 2, standalone rather than folded into RT-D5B-POSTCALL-REFUSAL-MECHANISM) after the Architect observed that folding it there lands the correction on the same commit that falsifies it. Defect found by the Architect at immediate_bridge.rs:27; second false sentence found by the Steward at :18-22. Dead-set roster measured by the Steward on origin/main 67684fa5d with scripts/ken-cargo build -p ken-runtime, corroborated by the runtime-implementer's own (lib) build. Steward-filed per COORDINATION section 2."
---

> ## MERGED 2026-09-16 at `7ebbc34fafb12bd9f0460106cd14077137ad9104`
>
> **Verified by blob, not by ancestry** — the publisher squashes, so a routed
> commit is an ancestor of nothing. `immediate_bridge.rs` is byte-identical
> between the approved candidate `a799ddca61cb0a3eebea5c48aafd984be5928e50`
> and `main`, landed via PR #3754 (a respin of `dc6b1369d`, which went red on
> a doctest compile failure the header's own unfenced roster caused). The
> AC-5 control that missed that failure was repaired separately at
> `cc7a2d77d0cc43bf1adb507b15a787696d7c0cf1` (squash `7f8fa2497`).
>
> **Comment-only, one file, zero compiled change.** `AC-5` is the control —
> **and it was repaired on 2026-09-16 after the first candidate went red.**
> "Comment-only" does not imply "compiles nothing": an indented block inside
> `//!` is a doctest. Read `AC-5` before writing the roster.

## The defect

`immediate_bridge.rs:5-31` on `main` contains two false sentences. Both were
true when written in slice 2, and **the header dated itself to the slice that
would end them**.

**FALSE 1.** *"The live wiring — the assignment of
`publish_immediate_bridge_realization_plan`'s result into the plan before
response phase B — is a successor slice and is deliberately absent."* It is
present, at `construction.rs:1460`. The header calls its own successor absent
while that successor is on `main`.

**FALSE 2.** *"Because nothing calls in from production, a production build
reports `never used` here… the warnings clear themselves when the successor
wires the plane in."* Production calls in now. **And the promise is the more
damaging half: the successor wired the plane and the warnings did not all
clear.** A reader who trusts that sentence concludes the wiring is broken.

**STILL TRUE — do not touch.** The `#[allow(dead_code)]` prohibition and the
zero-hit-grep control are both sound as *intent*. They belong to
`RT-D5B-POSTCALL-REFUSAL-MECHANISM`.

**FALSE 3, and it is already false on `main` rather than introduced by any
candidate.** The control's two forbidden names are `InlineBridgeNoCall` — the
deferred-response sub-case variant — and `owns_seat`. The sentence declaring
they are *"not spelled anywhere in this file"* **spells `owns_seat` while
saying so.** Measured at `67684fa5d`:

    immediate_bridge.rs   owns_seat          1 hit   <- the control's own sentence
    immediate_bridge.rs   InlineBridgeNoCall 0 hits
    crates/               InlineBridgeNoCall 0 hits

⇒ **A grep for `owns_seat` in this file returns 1, and the hit is the
prohibition.** The control has been self-refuting since slice 2 wrote it, which
is why `AC-4` below asks for the count rather than for the sentence.

Note `owns_seat` is **not** forbidden crate-wide — `static_transition.rs`
carries 2 legitimate hits. The control is scoped to this file only, and an
`AC` that greps `crates/` measures the wrong population.

## Why this is standalone rather than a fold-in

The runtime leader's first call was to ride it on
`RT-D5B-POSTCALL-REFUSAL-MECHANISM`. The Architect's objection is decisive:
**that node wires the consumer, so its own commit falsifies the roster the fix
would be landing.** A correction whose expiry is the commit that carries it is
not a correction.

## The durable form: a predicate, with the roster subordinate to it

A flat list of names decays the moment an item is wired, and it can only be
falsified by rebuilding — which is exactly what let the original sentence rot
unnoticed. State the rule a reader can check from the call graph, then the
measurement, explicitly scoped:

    THE INVARIANT.  An item in this module warns `never used` exactly when it
    has no live root. Production reaches publish_ -> derive_ ->
    relation_from_rows and the Stratum A chain, and nothing else in this file.
    Read the diagnostic's NAMED ITEMS, never its count — a merged diagnostic
    that loses one subject leaves the tally unmoved.

    THE ROSTER.  A measurement of one commit, true of no other.

**And the two dead causes are different, which a flat roster flattens:**
`build_`/`validate_` are an unreached validation pair, while the accessors are
a read surface with no consumer yet. The second shrinks when
`CheckedIhPostCallConsumer` lands; the first does not.

## The measurement

`scripts/ken-cargo build -p ken-runtime`, default features, at `67684fa5d`.
**Seven diagnostics in this file; 95 in the crate.**

    :58     field `default` is never read                 ImmediateBridgeConsumer::Ordinary
    :61-62  fields `cases` and `default` are never read   ::Computational
    :66-67  fields `cases` and `default` are never read   ::CheckedComputational
    :109    multiple methods are never used               impl ImmediateBridgeRealization
                                                          all 11 accessors, named individually
    :504    function `build_immediate_bridge_realization_plan` is never used
    :675    function `validate_immediate_bridge_realization_plan` is never used
    :690    methods `immediate_bridge_realization` and
            `immediate_bridge_realization_identities` are never used

**A four-item roster was proposed and is an undercount** — a correct subset
naming `build_`, `validate_` and the two plan accessors, omitting five
never-read fields and the eleven-accessor `impl`. Verified separately that the
`impl` diagnostic names **all eleven** declared accessors, so "every accessor"
is exact rather than approximate.

**The other half of the measurement, worth stating because it is the control
that passed:** `publish_`, `derive_` and `relation_from_rows` are **absent**
from the dead set. `RT-D5B-LIVE-WIRING`'s `AC-3` predicted exactly that
inversion.

## Acceptance

**AC-1. Both false sentences are gone**, and the correction records what was
wrong rather than silently swapping the text. A reader arriving later should be
able to see that the header once claimed the opposite and why.

**AC-2. The invariant is stated as a rule checkable from the call graph**, and
the roster is explicitly labelled a measurement at a named commit. A candidate
that lands only a corrected list has rebuilt the thing that rots.

**AC-3. The roster covers all seven diagnostics.** Control: re-take the
measurement by the method the header names and show the diagnostic set and the
roster agree. Name the count of items, and do not assert a universal over the
`impl` without checking the declaration count against the named set.

**AC-4. The zero-hit control is made TRUE, not merely restated.** Control, in
this file only:

    git grep -c 'owns_seat' <candidate> -- .../immediate_bridge.rs          -> 0
    git grep -c 'InlineBridgeNoCall' <candidate> -- .../immediate_bridge.rs -> 0

`owns_seat` reads **1** on `main` and the hit is the prohibition's own
sentence, so a candidate that preserves the sentence verbatim fails this. Say
the rule without naming the names — *"the deferred-response sub-case variant
for a bridge realized without a physical call, and the seat-ownership rewrite
that travels with it"* is a complete description that spells neither.

**Do not widen the grep to `crates/`.** `owns_seat` is legitimate elsewhere —
`static_transition.rs` carries 2 hits — and a crate-wide control measures the
wrong population and cannot be satisfied.

**AC-4b. The `#[allow(dead_code)]` prohibition is intact.**

**AC-5. Zero compiled change, AND THE DOC COMMENT COMPILES NOTHING.** Targeted
only, `COORDINATION §12`:

    scripts/ken-cargo test --doc -p ken-runtime    -> passes
    scripts/ken-cargo build -p ken-runtime         -> this file's diagnostic set
                                                      unchanged, compared BY NAME

**A `//!`-only diff is NOT evidence of zero compiled change, and the first
version of this AC said it was.** A four-space-indented block inside `//!` is a
Markdown code block, and rustdoc compiles an untagged code block **as Rust**.
The first candidate, `dc6b1369d`, was `//!`-only in one hunk and still went red
on `test shard 1/8` (PR #3754, run `35074374336`): the roster at `:68-74` was
handed to the compiler as source, and rustc tried to lex `` `cases` `` as a
token.

⇒ **Write the roster as a fenced block tagged `text`** — `text`, not `ignore`,
because `ignore` claims it is Rust that we choose not to run, and it is prose.

    //! ```text
    //!     build_immediate_bridge_realization_plan
    //!     ...
    //! ```

**The old control could not fail in the presence of the defect it existed to
catch**, because the diff *was* `//!` lines only. It encoded the premise *"a doc
comment is inert"* and then measured the premise instead of the property. A
control for "zero compiled change" has to **compile**.

**Note on the rest of the file:** two further indented runs sit in `///`
comments inside the `#[cfg(test)]` module that begins at `:760`. They do not
fire because **rustdoc does not build `cfg(test)` code**, so they are never
collected — that is a *condition they currently satisfy*, not a property of
them. Leave them alone; they are outside this node.

**And do not compare warning COUNTS.** The first version of this AC said *"the
crate's warning count is unchanged at 95"* without naming the build unit; the
implementer measured `94` on `(lib)` alone. Both numbers were right and the AC
was the thing that was wrong. **Name the build unit and compare the named items
for this file.**

**AC-6. No decorative glyphs.**

## Local build discipline

Targeted only, through `scripts/ken-cargo`, `-p ken-runtime`. **Never
`--workspace`.** `COORDINATION §12`.
