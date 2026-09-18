---
id: RT-CARRIER-PRODUCER-OCCURRENCE
title: "a source aggregate reaches the carrier with no planner-issued producer occurrence, so the C2 edge refuses to emit and the nested-payload selection row never exercises its property"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER]
blocks: []
github: null
origin: Measured by the RT-SRCBODY-BIND-ORDER all-eight-package two-ended census (evt_ksrhrv82t5ae), after CI failed this row at candidate fb99d0fc. Fails identically at frozen base 21fd46dc, so it is pre-existing base debt and not a regression from D1. Fits no released owner; the ring stopped and reported rather than assigning a nearest fit. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## FRAMED — `ready`, size M
>
> Frame:
> [`RT-CARRIER-PRODUCER-OCCURRENCE`](../wp/RT-CARRIER-PRODUCER-OCCURRENCE.md)
>
> **The frame governs; this file is the origin record.** Where the two differ,
> the frame is later and was ground against `origin/main` `d18da5c6`.
>
> **Two things below are sharpened by the frame and should not be built on as
> written here:**
>
> 1. **"This is one observed occurrence" understates it — the row has at least
>    TWO independent refusals.** It compiles three edges; the panic reports only
>    the first, because the preflight walker refuses whole-graph before any
>    allocation (`lowering/mod.rs:4895-4980`). Edge 1's aggregate comes from
>    `synthesized_constructor`, edge 2 hand-writes `occurrence: None` at
>    `constructors.rs:2730` and `:2734`. **Repairing the first only moves the
>    panic**, so a repair sized on the observed signature is sized on a sample of
>    one. Frame `D1` and `AC-2`.
> 2. **The rig-versus-real question now has a derived answer, and the frame
>    front-loads it rather than leaving it open.** `synthesized_constructor`
>    (`mod.rs:11064`) returns `occurrence: None` on a deliberate branch taken
>    when `defining_emission_owner` is `None`, and the c2 rig's compiler
>    (`bare_carrier_test_lowering`, `constructors.rs:1884`) sets exactly that at
>    `:1926`. The chain reproduces the observed signature including its
>    `construct` field. **It was derived by reading and not executed** — frame
>    `D0` exists to kill it cheaply.
>
> `ready` does not mean released. The fleet is single-threaded and this node
> shares `constructors.rs` with [[RT-WORKER-FIXTURE-DECODE]] and its crate with
> the active `RT-CARRIER-BYTESPAN-OBSERVE`.

## Exact signature

```text
Unsupported(UnsupportedLowering {
    construct: "Constructor",
    reason: "a source aggregate reached the carrier with no planner-issued
             producer occurrence, so it would name no ownership record and
             could only be given the authority of wherever it happened to be
             transferred",
})
```

Panics at `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/
constructors.rs`, in
`c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload`, at
`.expect("the C2 carrier edge emits")` — the `emit(...)` call itself refuses.

## Provenance, measured at both ends

`scripts/ken-cargo test -p ken-runtime --lib --no-fail-fast`:

| ref | result |
|---|---|
| base `21fd46dc` | fails, at `:2509:14` |
| candidate `fb99d0fc` | fails, at `:2511:14`, **identical signature** |
| same, `--features px8-ds-test-support` | identical at both ends |

Base-fail and candidate-fail. The two-line offset is the candidate's added
lines above the row, not a different failure.

## The refusal is upstream of the property under test

The row exists to prove that a runtime host result **selects a separately
generated nested payload**. It never gets to select anything: the carrier edge
declines to emit at all, so the assertion the test was written for is never
evaluated. As with [[RT-WORKER-FIXTURE-DECODE]], this is a **refusal to build
the fixture**, not a wrong answer — which is why neither row could have been a
`D1` capture-order regression, and why both fail at a base that predates `D1`.

**The refusal text is a design statement, and it should be read as one before
it is read as a bug.** It says an aggregate arriving without a planner-issued
producer occurrence would name no ownership record, and so could only inherit
the authority of wherever it happened to be transferred. That is the lowering
refusing to fabricate provenance. The open question is whether the **test rig**
is failing to issue the producer occurrence the planner would issue in
production, or whether a real path can reach the carrier in that state.

## What the frame owes

- **Answer the rig-versus-real question first, because it decides everything
  else.** If the rig simply skips a planner step, the repair is in the fixture.
  If a production path can reach the carrier with no producer occurrence, the
  refusal is protecting a real hole and the row is evidence, not debt. Do not
  fold these two outcomes into one deliverable.
- **Do not repair it by relaxing the carrier's refusal.** Making the emit
  succeed by accepting an aggregate with no ownership record is the cheapest
  available false fix, and it would grant exactly the unearned authority the
  refusal text names. If that is the proposed direction, it is a mechanism
  question and returns to the Architect.
- **Prove the row measures its property once restored.** A green row after the
  emit succeeds does not establish that the nested payload was *separately
  generated* and correctly selected; that is a distinct claim needing its own
  discriminating pair.
- **Check whether the same refusal reaches other rows.** This is one observed
  occurrence; the enumeration that found it was scoped to the failing set, not
  to this signature. Name the population before sizing the repair.

## Symptom inventory

Hard stops taken on this node, and what each one was actually a symptom of.
Seeded on the Architect's suggestion (`evt_24jpn6zngv87y`); the point of the
list is that a second entry of the same kind is a finding about how frames are
written, not about this node.

1. **Frame anchors de-aimed by a behaviour-preserving file split, keyed on
   source coordinates.** 2026-09-18, hard stop 1. Raised by runtime-implementer
   (`evt_2n3wx07120cq6`), confirmed by the Architect, re-measured and amended by
   the Steward. `c7f071bcb` moved every production site the frame named from
   `lowering/mod.rs` into `lowering/aggregates.rs` and `boundary.rs`; the three
   `§4` bans then resolved to real, unrelated code rather than erroring. **The
   general lesson, now written into the frame: a line-anchored prohibition in a
   moving file fails by RE-AIMING onto a same-shaped neighbour, and the natural
   check returns yes.** The remedy adopted frame-wide is to anchor bans on
   `<symbol> :: <property>` with `file:line` marked as a hint. The Architect's
   §1a hard-stop counter does not advance for this one — the ruling was the
   Steward's, not theirs.

2. **A peeling/forcing ladder measures a program Ken never runs, keyed on
   DEPTH.** 2026-09-18, hard stop 2. Values behind a deferred guard are the
   values the guard exists to reject, so each run buys one layer with no bound
   on layers remaining and no guarantee the next is even a new predicate.
   Architect withdrew the ladder and specified a caller closure in its place.
   **That replacement was itself defective — see entry 3.**

3. **An instrument whose COUNTEREXAMPLE CONDITION IS SATISFIED BY CORRECT CODE,
   keyed on CALL-GRAPH POSITION.** 2026-09-18, hard stop 3. Architect specified
   *"a branch reaches an entry point with no bind anywhere above it"*.
   Production reaches that state on EVERY compile, by design
   (`core.rs:1758-1760`), so the search could not terminate in the negative:
   only a residue or a false alarm was reachable. It produced 13 apparent
   counterexamples, none real. A second independent fault in its caller pattern
   changed only WHICH correct paths were listed. **Neither fault was visible
   from the output.** The instrument was the Architect's and was withdrawn
   rather than debugged; `D2` then settled from the write side, and the answer
   proved to be in a comment block directly above the declining read
   (`aggregates.rs:3468-3476`).

   Both entries 2 and 3 are the Architect's and both advance their `§1a`
   counter: **`(RT-CARRIER-PRODUCER-OCCURRENCE, D2) = 2`, next research pull at
   3.**

### The three entries share ONE predicate, and it is the finding

The list was seeded on the premise that a second entry of the same kind is a
finding about how frames are written. **Three entries arrived and the Architect
named what they share** (`evt_4xarw5w7crtd5`):

    1  frame anchors de-aimed by a behaviour-preserving file split
         keyed on SOURCE COORDINATES                          (Steward's)
    2  the deferral/peeling ladder: force a guard, observe the next layer
         keyed on DEPTH -- what is behind layer N             (Architect's)
    3  the caller closure: "a branch reaches an entry with no bind above it"
         keyed on CALL-GRAPH POSITION                         (Architect's)

> **EACH WAS ANCHORED ON A POSITION WHERE THE QUESTION WAS ABOUT A PROPERTY.**

**The failure mode is uniform and quiet: a position-anchored instrument does not
error, it RESOLVES ONTO SOMETHING.** The re-aimed ban resolved onto real
unrelated code and the natural check returned yes. The ladder returned a layer.
The closure returned 13 roots. **Three confident, well-formed, wrong answers,
and not one of them looked like a malfunction.**

Two further instances, neither an inventory entry, both from this node: the
frame's `D5` cites `:2677-2681` for a note the tree has since moved, and an
*"is an inventory armed for this WP"* check was run as a search of
`docs/program/wp/` — a LOCATION — when the section sits in
`docs/program/issues/`. **Five instances, one predicate.**

**The remedy already adopted for `§4` bans — anchor on `<symbol> :: <property>`,
`file:line` as a hint — generalizes to three more artifact kinds:**

    instrument termination conditions   state the PROPERTY that ends the
                                        search, never the POSITION reached
    currency citations                  cite by CONTENT; coordinates are hints
    cross-artifact searches             search by the property's NAME across
                                        the tree, not by the path you expect

**No recut follows from this.** Each of the three entries' repairs was
individually correct and is already made — the ban form was fixed, the ladder
was withdrawn, the closure was withdrawn. The predicate buys the rule
extension, not a rescope of this node; the Architect declined to manufacture
work to make the trigger look productive, and that judgment is recorded here
rather than quietly dropped.
