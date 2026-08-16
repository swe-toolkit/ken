---
id: RT-RECURSOR-TRANSPORT
title: "Retire the two live recursor residual classes — MatchScrutineeRecursor and LexicalCallArgumentRecursor — off the RecursiveDescent lane"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-CONTSPEC-WITNESS, RT-MATCH-RECURSOR-CONSUMERS, RT-LEXICAL-RECURSOR-CONSUMERS, RT-LEXICAL-ROW2-MISSING-MINT, RT-LEXICAL-R3-FUSION-EMITTER]
blocks: [RT-DESCENT-RETIRE]
github: null
origin: Operator directive 2026-07-29 — prioritize replacement of RecursiveDescent, migrate the remaining residual classes, do not linger half-migrated. Campaign docs/program/16-recursive-descent-retirement.md. Steward-filed (agents cannot create tracked work per COORDINATION §2). Recut 2026-08-08 per Architect ruling evt_237tbdsacqbk4.
---

> # RELEASED 2026-08-15 — `D0`-`D2` ONLY. `D3` IS STILL GATED. Read both halves.
>
> **Released to the runtime ring 2026-08-15, anchor `evt_1wz8jc8y38xtv`**, base
> `46a8ba199`. Status is `active`. **`D3` is NOT in that release.**
>
> > **This banner has now been wrong twice in one day, in opposite directions,
> > and the shape is the lesson.** It first announced `draft` while the
> > frontmatter said `ready`; corrected to `ready`, it went stale again within
> > hours when the release flipped the node `active`. **A banner that names a
> > status is a claim that ages, and nothing reds when it does.** A seat that
> > finds the tracked artifact contradicting what it was told is right to refuse
> > to move — that is exactly what cost the verify ring a lawful start the same
> > morning. **State the release and the gate; let the frontmatter carry the
> > status.**
>
> **Why the node sat at `ready` while its dependency was `active`, since the
> schema warns about it:** playbook §4e requires every successor of an in-flight
> node to be `ready` with a shovel-ready frame, so the frontier advances with no
> Steward pass between a merge and the next start. `check-issue-schema.sh
> --strict` permits it — a `ready` node depending on an `active` one **warns**,
> and only a `draft`/`ready` dependency fails. **That warning is the check
> working, not a defect to repair.**
>
> **The gate, unchanged: `D3` cannot start yet.**
> Four of the five `depends_on` are `merged`. The single remaining one is
> [[RT-LEXICAL-RECURSOR-CONSUMERS]], which is **`active`** — it is a
> multi-increment node on the operator's priority lane and it stays `active`
> across its own increments, so **do not wait for it to flip `merged` as a
> proxy for readiness.** `RT-LEXICAL-R3-FUSION-EMITTER` is done; do not go
> looking at it.
>
> ⇒ **`D0`-`D2` are startable now. `D3` is gated on the consumers node's
> transport half actually landing** — check the tree, not the node status.
>
> **Measured, not inferred.** Runtime's `RT-DESCENT-RETIRE` `D1` census at
> `c1b9a1e8` found 89 intact residual rows — 74 `LexicalCallArgumentRecursor`
> and 15 `MatchScrutineeRecursor`, which are precisely this node's two classes.
> The transport has not happened.
>
> **Confirm the count yourself at pickup, against the enum and not this line.**
> `enum RecursiveDescentResidual` at
> `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:1979` carries
> exactly the two live variants named above; three classes are retired. The
> campaign doc's own entries stop at 2026-08-09 and are not the instrument.

> # CORRECTION 2026-08-16 — THE `D3` GATE ABOVE CAN NEVER FIRE AS WRITTEN.
> # `D3` IS GATED ON `D0`-`D2`, WHICH ARE YOURS. Steward.
>
> **[[RT-LEXICAL-RECURSOR-CONSUMERS]] is `merged`** (PR #2440 closed it via
> [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]]). **All five `depends_on` are now
> closed.**
>
> **The gate above told you to check the tree rather than the node status. Do —
> and then read this, because the tree will tell you the opposite of what that
> instruction intended.** The consumers node closed by **disposition**, not by
> transport: its four remaining expressions are recorded as **compiler
> asserts/invariants**, unreachable from any admitted Ken source program. **No
> residual row was ever going to move**, so *"the transport half actually
> landing"* is a condition that cannot be satisfied by anything, and a seat
> applying it literally concludes `D3` is permanently blocked.
>
> **A gate keyed on one closure route cannot fire for a node that closed by
> another.** The banner was written when a repair was expected; the ratified
> criterion the node actually closed on was *"every expression carries a
> recorded disposition."*
>
> **Measured at `148f805f9`, and the coordinate above is stale:** the enum is at
> `core.rs:`**`2002`**, not `:1979`. Both variants still live; 30
> `LexicalCallArgumentRecursor` + 27 `MatchScrutineeRecursor` references in
> `crates/ken-runtime/src`; `RecursiveDescent` 117 across `crates/`. **Those
> rows are `D3`'s own work to delete, not evidence that a precondition is
> unmet.**
>
> ⇒ **`D3`'s only remaining condition is the frame's own item 3** — *"only after
> both executable positions are green may the two variants and their test-only
> selector hooks retire"* — **and that is `D0`/`D1`'s outcome. The gate is on
> this ring, not on another node.** Resumption posted at `evt_6pkf5hwqwv21k`.

> # `D0`-`D2` LANDS; `D3` IS GATED — 2026-08-08, hard stop 4
>
> **The node is split across a merge boundary and the two halves are in
> different states.** Read both before acting on either.
>
> - **`D0`-`D2`'s production mechanism is sound and lands** — but **not at
>   `8efdfdb3`**. That object's record claims *"position A closes"* and *"both
>   lanes now agree on position A"*, and the census supplies an A-only
>   counterexample (`d8d`) reachable **at that same object**. Architect approval
>   was withdrawn mid-publish (`evt_38bz22cqd7e48`); **`dec_6nsrbyw1wjpb` is
>   void**, PR #1609 was closed before merge and `main` was never modified. What
>   lands is a **bounded child** over `8efdfdb3` narrowing every class-wide claim
>   to the exact `D1` witness — no production or test-logic change, fresh SHA,
>   fresh QA, fresh Architect, fresh Decision.
> - **`D3` is blocked on FOUR successor nodes** — which is why this node carries
>   `depends_on` edges onto nodes filed *after* it. **It was two when this
>   line was written; rows 1-5 have since been split three ways, so the count
>   in your memory is stale.** [[RT-MATCH-RECURSOR-CONSUMERS]] (row 6, Position
>   A completion); [[RT-LEXICAL-RECURSOR-CONSUMERS]] (six of the eight
>   expressions in rows 1-5); [[RT-LEXICAL-ROW2-MISSING-MINT]] (row 2); and
>   [[RT-LEXICAL-R3-FUSION-EMITTER]] (row 5's before-hole expression).
>   The `D3` retirement exposed six previously-green semantic controls failing
>   closed across five refusal boundaries; the census partitioned them into
>   **two populations with two distinct activation seams**, not one — that
>   partition is by *seam* and is unchanged; the three-way split above is by
>   *owner* within the B-only seam.
> - **`10369776252861e8b15e613576256a3682c70066` is held evidence only** — not a
>   candidate, not the repair base, not to be continued.
> - **No new `#[ignore]` on this node, ever.** The Steward ruled the six
>   quarantinable at `evt_7vhjcstd37a50`; that ruling is **withdrawn** (Architect
>   `evt_5w09dcwbf7k70`).
>
> Full sequencing and the census the repair node is held on: see
> [[RT-LEXICAL-RECURSOR-CONSUMERS]] and the `D3` gate banner in the frame.

> # RECUT 2026-08-08 — THE WHOLE PRIOR CONTRACT IS WITHDRAWN, NOT AMENDED
>
> **Authority: Architect ruling `evt_237tbdsacqbk4`**, answering the Steward's
> re-derivation request `evt_4hr31qp6ab5xg`.
>
> Everything this node said before today was written against a world in which
> `RT-DECL-CLOSURE-PORT` `D7` had not landed and the ContinuationSpecialization
> seams did not exist. Both are now false. **Do not read the old contract for
> context; it is superseded, and it is wrong in the direction that costs the
> most — it describes work that no longer needs doing and a base that would
> destroy landed architecture.**
>
> Three specific withdrawals:
>
> **1. The global population-authority obligation is withdrawn.** The old text
> said this node owes *"one exact `BoundaryUse` record per static lowering
> event"*, replacing `D7`'s population authority in place. That sentence is
> **superseded, not an unfulfilled deliverable**. `BoundaryUse` has **zero hits
> in `crates/`**; the surviving references are historical docs. `D7`'s actual
> landed authority is `PlannedEffectSeat`, and it is **discharged for its own
> domain** — host-effect occurrences, with an intentionally effect-specific key,
> Need/Avail vocabulary and choke point.
>
> It does not extend to either residual class, and **this node must not widen it
> into a universal lowering-event record.** That is the exact domain conflation
> ruled out in `evt_1v9m7t4m9dmj7`.
>
> **Nor is there a missing universal authority to build.** Lowering deliberately
> uses separate exact authorities for separate semantic populations: host-effect
> seats, aggregate allocation occurrences, continuation source slots,
> continuation specializations and call identities, join plans, typed declared-
> unit calls. Security comes from the exact domain-specific producer plus its
> checked consumption boundary — **not from one global token vocabulary.**
>
> **2. The ordering rule is withdrawn.** *"Population authority FIRST, cell-level
> repair after"* is an artifact of the pre-`D7`, pre-CONTSPEC world. It is
> replaced by the three-step bounded order in the frame.
>
> **3. `07ce6ef1` is NOT the repair base.** The old text said it *"SURVIVES AND
> IS THE REPAIR BASE — do not reset it"*. It is **not an ancestor of `d9b2eb38`**
> and exists only on preserved and old `D7` branches. Its `StaticRecursorWorker`
> prototype has 36 crate hits there and **zero on current `main`**, and the four
> core files have diverged by **`+58,582/-17,365`** (measured at `837f9296`
> across `lowering/core.rs`, `core/tests/control.rs`, `lowering/mod.rs`,
> `planning/static_transition.rs`). Continuing or
> cherry-picking it **would overwrite the landed continuation-specialization,
> ownership, ABI and ledger architecture.** Cite it as historical refusal and
> design evidence only; re-derive every mechanism claim on the new base.
>
> **Size withdrawn from `L` to a provisional `M`.** The hard internal mechanism
> the `L` assumed this node must invent has since landed.

> # THIS NODE DELIVERS DIRECTLY. IT DOES NOT MERELY CLOSE.
>
> **A prior banner said *"THIS NODE NO LONGER DELIVERS DIRECTLY — it closes when
> the terminal seam merges."* That is withdrawn and it did measurable harm.**
>
> On 2026-08-08 an implementer, correctly suspicious that closing an
> unimplemented node would be wrong, checked that suspicion **against this
> banner** and retired it. The banner answers *when* the node closes; it was
> read as evidence that closing it is *sound*. Both residual classes were live
> in production at the time, and the closure would have unblocked
> `RT-DESCENT-RETIRE` — the lane deletion — while two classes could still select
> the lane.
>
> ⇒ **An instruction to close is not evidence that the work behind it is done**,
> and a node asserting its own closure timing is the least independent source
> available for whether that closure is sound. The check that settles it: **are
> the classes this node owns still live in production?**

## What this node owns

Two `RecursiveDescentResidual` variants, both still live and both still
selecting the `RecursiveDescent` lane. Find them by name in
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs` — do not trust a
line number in this file or the frame, including one written today.

**`MatchScrutineeRecursor`** — an ordinary `Match` whose scrutinee is a
`ComputationalMatch` carrying recursive positions.

**`LexicalCallArgumentRecursor`** — a `Call` whose callee is a `LexicalClosure`
and whose **argument** is such a recursor.

Three sibling classes are already retired: `TransparentDeclarationClosure`,
`SeedClosureCall`, `ProducerMatchCall`. These two are the remainder.

## They were folded as "one mechanism" — that claim is now conditional

The prior text folded them on the grounds that both fire on an active
computational recursor and *"differ only in the syntactic position it
occupies"*, so retiring one without the other would build the same transport
twice.

**That remains the working hypothesis and it is no longer an assumption you may
carry.** Per the ruling: if `D1` shows the two positions require materially
different transports, **hard stop and re-size or re-fold** — do not preserve the
"same mechanism" claim merely because both variants mention an active recursor.

## What remains owed

The frame carries the full contract. In outline, and in this order:

1. **`D0`/`D1` — re-census and activation probe on the post-WITNESS base.**
   Under a test-only per-variant selector exclusion, run one discriminating
   executable witness per position and record the first real functionized
   outcome. **This determines whether the landed continuation machinery already
   closes either class for free.**
2. **Only for a class that does not close for free, add the narrow consumer-port
   authority its failure proves necessary** — domain-specific, planner-owned,
   over the existing continuation machinery.
3. **Only after both executable positions are green** may the two variants and
   their test-only selector hooks retire.

The surviving invariant is outcome **(b)**: invocation-local activation, resume
and return-hole state never enters ABI data. Only ordinary typed values cross;
static continuation and callee identity stay planner- and compiler-owned; any
open, escaping or ambiguous case refuses **before** allocation or call emission.

## Base

**Branch after [[RT-CONTSPEC-WITNESS]] actually merges, from that then-current
`main`, and pin the new base at pickup.** Not `07ce6ef1`, not any preserved
freeze ref.

## Sequencing

Last migration before the capstone [[RT-DESCENT-RETIRE]], which owns the lane
itself and must not be closed by this node. By the time this runs, three classes
are retired, so program shapes that have **never** reached `FunctionizedUnits`
will reach it here for the first time — campaign Trap 2. **Expect a hard stop
and route it; that is the fail-closed machinery working, not a defect.**
[[RT-FNUNIT-RESULT-TOKEN]] is one such stop already routed.

## The frame is written

`docs/program/wp/RT-RECURSOR-TRANSPORT.md`. Campaign context, the traps binding
every node in this arc, and the schedule:
`docs/program/16-recursive-descent-retirement.md` — read it before the frame.
