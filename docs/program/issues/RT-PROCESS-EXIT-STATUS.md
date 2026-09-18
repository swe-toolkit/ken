---
id: RT-PROCESS-EXIT-STATUS
title: "rt_escape r2_cross_buffer_freeze_fails_closed_with_invalid_bounds refuses at the boundary-transfer admissibility walk with StaticResponseDeferred -- a deferred host response reaches a cross-owner transfer. The id names a ProcessExitStatus/Persistent-over-NoReferent refusal the row no longer produces; the id is a citation key and is NOT renamed, the title is. Decide whether the Deferred is a correct fail-closed or a classify leak, and repair whichever it is."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by the RT-SRCBODY-BIND-ORDER D12 complete no-fail-fast enumeration (evt_2n9wq8xyj0aa1); re-measured by RT-IGNORED-FAILING-ROWS-INVENTORY's ledger (docs/program/evidence/rt-ignored-failing-rows-ledger.md) row 13, which records a DIFFERENT refusal from the one this node is named for and label agrees? NO. Fails at frozen base 21fd46dc as well as at the candidate, so it is pre-existing base debt and not a regression. Framed by the Steward 2026-09-18 under the operator L1 directive 2026-09-17 ('Is L1 still working on clearing the ignored tests? That is the top priority until it is done.'). depends_on was [RT-SRCBODY-BIND-ORDER]; that node is merged and the edge was a READ, not a dependency -- the provenance is recorded here instead. Steward-filed per COORDINATION section 2."
---

> # TREAT EVERY ANCHOR IN THIS FRAME AS PERISHABLE
>
> If a fixed input below turns out false against the landed code, **say so and
> escalate — do not quietly build around it.** Every signature here was measured
> at a named tree and this node's whole history is of signatures moving
> underneath their records. The phrases are given so you can re-find the sites;
> **re-measure at your own base before you build.**

# The measurement

`crates/ken-cli/tests/rt_escape_second_resource_native.rs`,
`r2_cross_buffer_freeze_fails_closed_with_invalid_bounds`.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`** by the failing-rows
ledger, row 13:

```text
StaticResponseDeferred: a deferred host response is compiler control and
can only enter its exact response owner
```

**Superseded, kept so nobody re-derives it as new:**

```text
ProcessExitStatus: child 0 is held with a Persistent referent lifetime and
can be owned by PersistentStore, which its own producer occurrence's
ownership record did not plan for that position (planned Persistent over
[NoReferent])
```

The ledger records `label agrees? NO` for this row: the label predicts a
**Persistent-child / NoReferent ownership failure**, and that is not what the
row does.

> ### THE `id` IS NOT RENAMED. THE `title` IS.
>
> The id is cited from other artifacts and renaming it breaks those citations
> for a cosmetic gain. **A title is not a citation key** — it is the line a
> reader consults *instead of* the body, so a title asserting a superseded
> signature out-argues a corrected one two screens down. **Fix the title, keep
> the id, and read this section as the aim.**

## The row is DIFFERENTIAL, and any single-engine framing is wrong at step one

The body calls `assert_native_matches_interpreter`, then asserts
`buffer_freeze_outcome` equals `Resource(InvalidBounds)` **on both** `diff.native`
and `diff.interpreted`. The ledger caught this **by body** after a name-pattern
pass had excluded it: the name advertises none of the four differential
patterns.

⇒ The row asserts native/interpreted **agreement** plus an exact outcome on each
engine. A repair that makes one engine compile is not a pass.

# THE IN-TREE ANNOTATION IS STALE IN BOTH PLACES

The comment block above the row asserts the superseded signature under the
heading *"Observed signature, exactly"*, and the `#[ignore]` reason string
independently repeats it:

```text
"RT-PROCESS-EXIT-STATUS: a Persistent child is held against an ownership
 record that planned NoReferent for that position; fails at base 21fd46dc"
```

**Two records, one drift, and both are audited for presence and consumed for
content.** A reader who trusts either one goes looking for an ownership-record
defect that this row does not currently exhibit.

**One clause in that block is probably still true and must not be deleted
reflexively:** *"It refuses at object emission, so the program never executes."*
The boundary-admissibility walk is pre-execution, so the clause likely survives
the signature change — **check it, do not assume it**, and note that the
sibling row in [[RT-SUBCONTINUATION-LIFO-RELEASE-ORDER]] carries the identical
sentence where it is now flatly false.

# Where the refusal is produced

Both sites are in `crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs`.
**Cited by phrase because a line number is destroyed by the edit this node
performs:**

    grep -n "can only enter its exact response owner" \
      crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs

- the `Lowered::StaticResponseDeferred` arm of
  `boundary_transfer_admissibility`, which returns `unsupported(...)`;
- the `LoweredVariant::StaticResponseDeferred` arm that yields
  `FailClosedForbidden { why: ... }`.

Both carry the same `why` text, so the ledger's signature alone does not say
which one fired. **Establishing which is `D1`.**

# THE MEASURED SIGNATURE HAS A LANDED OWNER FAMILY. THE OLD ONE DID NOT.

This is the part of the node that changed and the reason it is now startable.

The superseded text says the row *"fits none of the five released owners"* and
that the ring **said so explicitly rather than forcing it into the nearest
one** — the right call at the time. **That judgment was made against the
ProcessExitStatus signature.** Against the measured one it is wrong:
`StaticResponseDeferred` is exactly the mechanism
[[RT-COMPOSED-RETURN-SSA-SPECIALIZATION]] recut and landed.

That node is **`merged`** (squash `ad9905a7e`, 2026-09-03), and the ledger row
was measured on 2026-09-17 — **two weeks after it landed.** So this is not a
pre-recut signature that the SSA work has already fixed; it is a Deferred
refusal that survives the recut.

Its governing contract, from that node's own recut and amendment:

    Deferred = P1 UNION P2
      P1   absent-residual
      P2   present-but-unconsumed placeholder  (the HS3-b leak)
    discriminator = CALLER-CONSUMPTION
      Specialized  IFF  specializable AND the caller consumed it

⇒ **That discriminator is the fork this node turns on.** Do not re-derive it —
fold and cite it.

# `D0` — re-establish the premise at YOUR base, and be able to report it absent

**Run before anything else. Name your own base; do not measure against a SHA
pinned in this frame.**

Un-ignore the row locally and run it targeted (`scripts/ken-cargo`, `--test
rt_escape_second_resource_native`; never `--workspace`). Record the verbatim
refusal.

    (a) the StaticResponseDeferred refusal reproduces
          => proceed to D1.
    (b) a DIFFERENT refusal appears
          => that is the result. Record it, correct the annotation, STOP and
             report. Do not repair forward into a signature nobody has framed.
    (c) the row PASSES
          => the premise is gone. Report that and propose un-ignoring. This is
             a REPORTABLE OUTCOME, not a failure to reproduce -- do not look
             harder for a way to make it fail.

**`(c)` is live, not a formality.** The SSA specialization landed after the
`#[ignore]` was written and directly reworked this mechanism; six base failures
were already fixed incidentally by one earlier candidate on this chain.

# `D1` — which arm, and which side of the discriminator

1. **Which of the two sites fired.** They share a `why` string; distinguish
   them (the admissibility walk recurses per child, the variant arm classifies
   a transfer). Record the enclosing value and the transfer being attempted.
2. **Apply the caller-consumption discriminator to this row's response.** Does
   the caller consume it?

    (i)  CONSUMED, and the value is specializable
           => it should have classified Specialized and never reached the
              boundary as a placeholder. This is the P2 leak. The repair is in
              classify, NOT at the boundary. THE BEST GUESS, and the one to
              attempt first.
    (ii) NOT consumed
           => the Deferred is correct and the fail-closed is doing its job.
              The row is then asserting a transfer the design forbids, and the
              defect is in the FIXTURE or in the program it compiles. Repair
              the row, and say plainly that the compiler was right.

**Attempt `(i)`.** One honest try plus a handback. If the attempt shows the
value is genuinely unconsumed, that is `(ii)` measured from the inside, which
is a better measurement than a standalone probe would have been.

# `D2` — correct the annotation, whatever `D0` and `D1` return

**In the same change.** Both places: the *"Observed signature, exactly"* block
and the `#[ignore]` reason string. **Correct it to what is observed, not to this
node's name**, and record the SHA it was measured at inside the annotation so
the next reader can tell staleness from disagreement.

# Acceptance criteria

- **AC-0.** `D0` is run at the implementer's own named base, the verbatim
  refusal is pasted into the handback, and the base SHA is stated. A `(b)` or
  `(c)` outcome discharges this node's measurement obligation and **stops** the
  work — it is a pass, not a failure.
- **AC-1.** The handback names **which of the two `boundary.rs` sites** fired,
  by enclosing function, and states how it was distinguished from the other.
- **AC-2.** The handback states the caller-consumption verdict for this row's
  response — consumed or not — and cites the evidence, not the expectation.
- **AC-3.** If `(i)`: the repair is in classify and the diff touches no
  fail-closed arm in `boundary.rs`. If it does touch one, that is a hard stop to
  the Architect under `D3` below, not a judgment call.
- **AC-4.** If `(ii)`: the fixture change is accompanied by one sentence stating
  that the compiler's refusal was correct, written where the row is, so the same
  shape is not re-filed as a compiler defect.
- **AC-5.** Both stale annotation records are corrected and carry the SHA they
  were measured at. A grep for `Persistent referent lifetime` and for
  `planned NoReferent` in the two rows' files returns **zero live hits** outside
  a block explicitly labelled superseded.
- **AC-6.** The row is differential: if it is un-ignored, `assert_native_matches
  _interpreter` **and** both per-engine `InvalidBounds` assertions pass. A
  native-only green does not satisfy this.
- **AC-7.** No-regression is **green in CI**, never a local `--workspace` run
  (`COORDINATION §12`).

# `D3` — the hard stop

**Widening what the boundary admits is not this node's to decide.** Both refusal
sites are fail-closed by design and their comments record that alternatives were
*"considered and refused"*. If the repair appears to require admitting
`StaticResponseDeferred` across a transfer, **stop and route to the Architect**
with the consumption verdict from `AC-2`. Do not relax a fail-closed arm to turn
a row green.

# What must not happen

- **Do not rename the `id`.** It is cited elsewhere. The title is the thing that
  was wrong and it is now fixed.
- **Do not repair at the boundary because that is where the message came from.**
  The message names where the value was *caught*, not where it was *made*. Under
  `(i)` the defect is upstream in classify.
- **Do not treat this as a single-engine row.** See the differential section.
- **Do not re-derive the Deferred contract.** It is landed and cited above; fold
  it.
- **Do not inherit the "fits no released owner" judgment.** It was true of the
  superseded signature and is false of this one.

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that re-measured this row.
  **Read `docs/program/evidence/rt-ignored-failing-rows-ledger.md`, not the
  node**; the node is the commissioning document and the ledger is the result.
- [[RT-COMPOSED-RETURN-SSA-SPECIALIZATION]] — merged; owns the Deferred residual
  and the caller-consumption discriminator this node turns on.
- [[RT-COMPMATCH-TREE-SCRUTINEE]] — the sibling row framed the same day, same
  ledger, same drift mechanism.
- [[RT-SUBCONTINUATION-LIFO-RELEASE-ORDER]] — carries the identical *"refuses at
  object emission"* sentence, where it is now false. Cited as the reason to
  check that clause here rather than assume it.
