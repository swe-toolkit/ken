---
id: RT-SUBCONTINUATION-LIFO-RELEASE-ORDER
title: "px8ta public_two_three_level_brackets_finish_and_release_lifo is the ONLY row in the failing-fifteen that executes and returns a WRONG ANSWER rather than refusing to build -- depth-2 releases come back [Id(1), Id(2)], which is ACQUISITION order, where strict LIFO requires the reverse [Id(2), Id(1)]. Its in-tree annotation still says it refuses at object emission so the program never executes and no binding order is observable, which is the exact opposite of what was measured. Establish whether the ordering is a real defect and what owns it."
status: merged
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, from RT-IGNORED-FAILING-ROWS-INVENTORY's ledger (docs/program/evidence/rt-ignored-failing-rows-ledger.md), row 9 and AC-5: the row's owning node RT-CLOSURE-BOUNDARY-LANE is merged, carries no frame, and the ledger records label agrees? NO -- 'label predicts a closure-lane refusal; this is an ordering assertion inside one engine'. Last of the six unroutable rows to get a node. Operator directive 2026-09-15: 'The other tests should be fixed.' Steward-filed per COORDINATION section 2."
---

> # MERGED 2026-09-18 — AND THE ROW IS STILL `#[ignore]`d. THAT IS THE CORRECT OUTCOME.
>
> Closed at squash **`4eb3dc4c61f556c6426016c4dc9ff9a018e83550`** (PR #3930),
> which landed the whole of this node: `§5` frames exactly two deliverables and
> both are discharged.
>
>     D0   fork resolved to arm (i) -- a STABLE PRODUCT DEFECT in release
>          ordering, escalated rather than repaired, exactly as the frame
>          required. Adversary corroborated the defect at the squash SHA.
>     D1   annotation re-measured at main and corrected in the same change.
>
> **Do not read this closure as the row clearing.** Arm (i) is the branch where
> finishing the node and clearing the row are *different events*, and this node
> only ever owned the first. The row is one of the failing fifteen and it stays
> in the count.
>
> **It is held by TWO blockers under one `#[ignore]`, and either fix alone
> leaves it red** — the annotation says so in the file:
>
>     the ordering product defect    RT-BRACKET-RELEASE-ORDER-PARITY
>     the depth-3 refusal at object  RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED
>       emission, a SEPARATE finding   (reported with it, never owned by this node)
>
> **Neither successor had landed when this closed**, so the sequencing lives
> here rather than in either of them. A reader who finds this node `merged` and
> the row still ignored is looking at the intended state, not at dropped work.
>
> ⇒ **The general shape, which is the reusable part:** a node that escalates
> rather than repairs completes by *filing* the defect, so its closure moves the
> node count and not the objective count. Anything measuring the fifteen must
> count the row, not the node.

> # THIS ROW IS NOT LIKE THE OTHER FOURTEEN, AND THAT IS THE REASON TO READ IT FIRST
>
> Every other row in the failing fifteen is a **refusal**: the compiler declines
> to build something and the property under test is never evaluated. **This one
> builds, runs, and returns the wrong answer.**
>
>     fourteen rows   the program never executes; a refusal is the observation
>     THIS ROW        the program executes; the ORDER of its releases is wrong
>
> ⇒ **It is the only row in the set where a behavioural defect is even possible**,
> and the only one whose green would mean the product changed rather than the
> fixture. Treating it as one more refusal to clear is the mistake this banner
> exists to prevent.

# The measurement

`crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:286`,
`public_two_three_level_brackets_finish_and_release_lifo`, `#[ignore]`d at
`:285`.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`** by the failing-rows
ledger, row 9:

```text
assertion `left == right` failed: depth 2 releases must be strict LIFO
  left:  [Id(1), Id(2)]
  right: [Id(2), Id(1)]
```

**Which side is which, resolved from the assertion rather than from the
message** (`:482-485`):

    assert_eq!(
        releases,                                  <- LEFT is OBSERVED
        opens.into_iter().rev().collect::<Vec<_>>(),  <- RIGHT is EXPECTED
        "depth {depth} releases must be strict LIFO"
    );

⇒ **Observed releases are `[Id(1), Id(2)]` — the SAME order as acquisition.
Strict LIFO expects the reverse.** The resources are being released in the
order they were opened.

> **STATE THE DIRECTION, BECAUSE THE TWO READINGS SEND THE WORK TO DIFFERENT
> PLACES.** A panic message names `left` and `right`, not *observed* and
> *expected*, and which is which depends entirely on the argument order at the
> call site. **Read the `assert_eq!`, never the message.** Getting this backwards
> turns "releases are FIFO" into "releases are reversed", and those are not the
> same defect.
>
> **This also bears on `(iii)` below:** releasing in acquisition order is a
> *systematic* behaviour with an obvious mechanism — a missing reversal, or an
> iteration in the wrong direction — not the signature of a race. The observed
> pair is still a transposition, so nondeterminism is not excluded, but it is
> now the less likely reading rather than a live rival.

Ledger signature class **G**, *"releases must be strict LIFO (in-engine ordering
assertion)"* — **population 1**. It is the only member of its class.

**The ledger also establishes this row is NOT differential.** It names
`ken_runtime::EffectObservation` only as a helper's return type, so it is not
asserting native/interpreted agreement. **One engine, one ordering, wrong.**

# THE IN-TREE ANNOTATION SAYS THE OPPOSITE OF WHAT WAS MEASURED

The comment block above the row, verbatim (`:274-275`):

> **It refuses at object emission, so the program never executes and no binding
> order is observable in it.**

And the `#[ignore]` reason (`:285`):

> `"RT-CLOSURE-BOUNDARY-LANE: a runtime-local closure has no durable lane across
> the boundary; fails at base 21fd46dc"`

**Both describe a refusal. The measured failure is a comparison of two actual
release sequences, which the program had to execute to produce.**

> ### THIS IS NOT MERELY STALE. IT INVERTS.
>
> An ordinary stale annotation names an old cause and leaves you to find the new
> one. **This one asserts that the very thing the ledger observed — an
> observable binding order — cannot be observed.** A reader who trusts it
> concludes the ordering assertion is unreachable and stops.
>
> ⇒ **The row advanced from a REFUSAL to a WRONG ANSWER, and the annotation did
> not move with it.** That is a different and worse failure than the one
> `RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL` records, where the labels merely
> stopped matching. Here the label actively argues against looking.
>
> **The annotation is not wrong about its own moment.** It was written when the
> row refused at object emission, and it was accurate then. **Nothing re-reads
> an annotation when the thing it describes changes underneath it.**

# `D0` — is the ordering defect REAL, or is the harness observing it wrong?

**Ask this before anything else, and do not assume the assertion is right.**

    (i)  releases really do come back in ACQUISITION order
         => a genuine behavioural defect in subcontinuation release ordering,
            with an obvious shape: a missing reversal or an iteration running
            the wrong way. This is a PRODUCT bug and the only one the
            failing-fifteen contains. STOP and escalate; do not size a repair
            here.
    (ii) the order is correct and the ASSERTION's expectation is wrong
         => the fixture encodes the wrong notion of "strict LIFO" for depth 2,
            or compares the wrong two identities. A fixture repair.
    (iii) the order is nondeterministic
         => the row is flaky and the ledger caught one sample. The two
            sequences are a permutation of each other, which is consistent
            with a race and MUST be excluded before either (i) or (ii).

**`(iii)` is checked first even though it is least likely**, because it is
cheap, because the observed pair is exactly a transposition, and because **this
row already has a documented history of harness-level interference**: the same
annotation block records a stack-size bisection (the row needs more than the
2 MiB libtest worker gives it and runs under a 256 MiB wrapper) and notes the
refusal surfaces on a **helper thread** `px8ta-nested-brackets`, with the test
thread failing only through a wrapper that *"carries no signature of its own."*

> **A row that already needed a thread and a stack wrapper to observe anything
> is a row whose observation mechanism is in scope.** Run it enough times to
> distinguish a stable wrong order from an unstable one before believing either.

# `D1` — re-measure at current `main`, and re-read the annotation against it

The ledger's signature is a correct record of `04d4dd38a`, **not of `main`**.
Re-run the row and record what it does now. **Whatever the outcome, correct the
annotation block in the same change** — it is the thing that will mislead the
next reader, and it costs nothing once the row has been run.

**Correct it to what is observed, not to this node's name.** If the row has
advanced again, the annotation should say what it does now.

# What must not happen

- **Do not repair the assertion to match the observed order.** Under `(i)` that
  edits the test until it agrees with a defect. **The expectation is the thing
  under test here**, which is exactly backwards from the other fourteen rows,
  and it is why `D0` is a fork rather than a formality.
- **Do not un-ignore it before `D0`.** Under `(iii)` a flaky row in CI is worse
  than a parked one.
- **Do not attribute it to `RT-CLOSURE-BOUNDARY-LANE`.** That node is merged,
  carries no frame, never names this row, and the ledger refutes its label for
  it. **Cited, not reopened** — a merged node's record of what it believed stays
  as it was.
- **Do not fold this into the refusal-clearing program.** Its green means
  something different from every other row's green. If it closes under `(i)`,
  that is a product fix and belongs in its own accounting.

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that measured this row.
  **Read `docs/program/evidence/rt-ignored-failing-rows-ledger.md`, not the
  node**; the node is the commissioning document and the ledger is the result.
- [[RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL]] — the sibling filed the same
  day for rows 1, 2 and 10. **Same shape, milder form:** there the labels
  stopped matching; here the annotation inverts. Both come from the same
  mechanism — a row's recorded signature is its only routing key, and nothing
  re-derives it when the row moves.
- [[RT-CLOSURE-BOUNDARY-LANE]] — the merged node whose label this row still
  carries. Cited for provenance only.
