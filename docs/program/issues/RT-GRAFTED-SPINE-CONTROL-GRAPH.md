---
id: RT-GRAFTED-SPINE-CONTROL-GRAPH
title: "Represent the emitted grafted spine as a typed, well-nested interprocedural control graph, produced from actual lowering rather than re-derived from a D0 projection. Behavior-inert structural precursor to RT-PLANNER-KRET-GRAFTED-SPINE's one final D0+D1+D2 attempt: it clears no ignored row and carries no behavior repair. Component boundary GraftedSpineControlGraph."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: [RT-PLANNER-KRET-GRAFTED-SPINE]
github: null
origin: "Architect ruling evt_7bff93f1zg2jh (RT-PLANNER-KRET-GRAFTED-SPINE hard stop 15), which rejects another direct-edge respin, adopts the Research result, and orders this precursor. The Architect confirmed at evt_1demsw13b66rm that this is a NEW narrow node and NOT the existing XL RT-GRAFTED-SPINE-IR-REPRESENTATION, whose subject is emission ownership: leave that node's draft status, dependency, frame and ownership subject untouched. The Steward chose this id, filed the node, and priced it. Counts remain hard stops 15 / entries 16."
---

# `RT-GRAFTED-SPINE-CONTROL-GRAPH` — frame

**Owner:** runtime. **Size:** L. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `532e5f04e4c4147924c61ed8c477c59b2458a440`.

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## 1. Objective

One component, `GraftedSpineControlGraph`, under
`crates/ken-runtime/src/cranelift_backend/`, that represents the emitted grafted
spine explicitly: a typed, well-nested interprocedural control graph built from
what lowering actually emits, together with an independent query over it.

**It is behavior-inert.** It clears no ignored row, changes no placement
decision, and carries no `D0`/`D1`/`D2` repair. Its product value is that
`RT-PLANNER-KRET-GRAFTED-SPINE`'s one final atomic attempt becomes expressible
against a real object instead of against a projection — and that attempt is
blocked until this lands.

## 2. Fixed inputs

- Architect ruling `evt_7bff93f1zg2jh`. Its six numbered provisions are §4 below
  and are not restated anywhere else.
- Research's adopted result: the faithful form is a typed well-nested
  interprocedural control graph plus the **existing** `AbiSlotKind::Control`
  word as an affine member token. Do not invent a second token.
- `RT-PLANNER-KRET-GRAFTED-SPINE` §9, which records the park and this
  dependency.

### The ruling's boundary evidence is CANDIDATE-SIDE. Most of it is not on `main`.

**Measured by the Steward at the ground SHA. Each of these names occurs ZERO
times under `crates/`:**

    PendingReleaseContextEdge     ReleasePlacementLedger
    ReleaseEmissionSite           record_finished_function
    claim_word

They exist only on `7b03f258c`, the rejected direct-edge candidate the ruling
quotes to *show the boundary*. That candidate is `+5667/-1869` over its
merge-base and **is not a base you build on** — it is the mechanism the ruling
removes. Nineteen commits have named the predecessor WP and none has landed a
line under `crates/ken-runtime/src`.

⇒ **Read provision 1's "`ReleaseEmissionSite` *stays* a function/body identity"
as a constraint on the scoping type this node INTRODUCES**, not as a reference
to an existing one. That is the Steward's reading, forced by the measurement,
and it loses nothing the ruling was asserting: the requirement is that the
typed scope is function/body-identified and never becomes a terminal identity.

**What IS on `main` and is real:** `StaticOriginId`
(`planning/static_transition/occurrences.rs`), `lower_computational_producer_call`
(`lowering/core.rs`), `AbiSlotKind::Control`
(`planning/static_transition/abi.rs`), and the five `UnitBundle` families
(`lowering/units.rs`).

**`1092`, `328`, `609`, `598` are RUNTIME-OBSERVED values, not source
constants** — grep finds them in no `.rs` file on `main` or on the candidate.
Do not spend a turn looking for them; they appear when the target fixture is
lowered.

## 3. Scope

Expected to change: a new module under
`crates/ken-runtime/src/cranelift_backend/`, plus the minimum wiring in
`lowering/units.rs`, `lowering/calls.rs`, `lowering/core.rs` and
`lowering/effects.rs` needed to observe emission and to seal the producers.

**Two boundaries that must not move.** No existing placement decision changes.
The two ignored rows `:314` and `:348` keep their current status, assertions and
`#[ignore]` attributes; a candidate that moves either has left this node.

## 4. Deliverable — the six provisions, verbatim in substance

1. **Nominal node kinds:** function entry, caller-local call, that call's return
   site, function exit, continuation entry, and terminal dispatch. The
   function/body scoping identity stays a function/body identity; it is not a
   terminal identity.
2. **Typed edges:** intrafunction CFG flow, call to callee entry, and callee
   exit to *that exact call's* return. Validate every Cranelift `Inst` only in
   its own `Function`; translate it before global storage into an opaque local
   node id scoped by its typed function/body identity. **No raw `Inst`,
   function number, emission order, or symbol becomes placement authority.**
3. **Well-nested summaries:** derive realizable call/return reachability to a
   fixpoint. A call returns only to its own return node; ordinary graph
   reachability is insufficient, and transitive closure over function-level
   adjacency would admit unmatched-return paths.
4. **Independent terminal observation:** when lowering actually encounters a
   governed member origin, record current typed site, member/body origin, and
   local dispatch node **before** consulting a site-selected Direct claim. For
   Direct members, `D0` supplies member identity and expected producer source;
   it no longer pre-installs the member only at `expectation.site` and then
   validates the site it caused. Non-Direct claimant paths are unchanged.
5. **Independent query:** `(member, expected_source) -> {actual seed edges, one
   terminal claim, realizable summaries}`. One source may yield several seed
   alternatives across specializations and contexts. **Do not require one raw
   path.** Require every seed to be real, every realizable member-carrying
   terminal to be the same member terminal, and no finite member-carrying exit
   to escape the bracket unconsumed. A terminal may serve multiple members only
   under injective claim-word discrimination.
6. **Closed producers:** every one of the five `UnitBundle` families contributes
   calls, returns, entries, exits and terminal encounters through **exhaustive
   sealed variants**. Unknown endpoints fail closed.

Ordered increments on one branch are fine; they land together as one green
whole. Selecting a context by number is the forbidden projection again and is a
hard stop, not an option.

## 5. Acceptance

**`AC-1` — four independent reds.** Each of these mutations reds, and each reds
at its own layer rather than all at one:

    deleted intermediate call/return
    mismatched return
    terminal moved to an entry, a body, or another member
    duplicate terminal

**`AC-2` — the green that the cheap implementation fails.** A branch and
reconvergence fixture with **two realizable paths to one terminal** stays green.
An implementation that requires a unique path passes `AC-1` and fails here; that
is the point of the pair.

**`AC-3` — behavior inertness, shown by a differential, not asserted.** Changing
`D0`'s expected source, or any former Direct `site` value, may change validation
only — never graph topology and never the observed terminal. Demonstrate it by
changing one and showing the graph and terminal identical.

## 6. Stop condition

Hand back, do not work around, if any of these is observed:

- a node kind, edge, or producer family in §4 cannot be built without a
  placement decision changing;
- the sealed producer enumeration cannot be closed over the five `UnitBundle`
  families, so an endpoint would have to be admitted open;
- `AC-2`'s two-paths-to-one-terminal fixture cannot be made green without
  weakening `AC-1`.

**The one-attempt bound on `RT-PLANNER-KRET-GRAFTED-SPINE` is not spent here.**
This node has its own counter. A hard stop on this frame is a stop on this
node.
