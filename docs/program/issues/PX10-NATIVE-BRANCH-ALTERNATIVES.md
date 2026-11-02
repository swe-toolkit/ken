---
id: PX10-NATIVE-BRANCH-ALTERNATIVES
title: "The native static-transition plan assumes every continuation result has exactly one producer construct, so a result that joins two Vis alternatives chosen by a runtime scrutinee refuses at the detached-result seat, the caller cut and the invocation transport. A producer call whose transport sources mix a predeclared and a specialization owner refuses as a mixed-owner relay. PX10's withChild fixtures hit one or the other. Lift both refusals for these shapes without weakening any guard for static results, and bring the three fixtures to native build with interpreter parity"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-NATIVE-SITE-IDENTITY]
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect D0 for PX10-NATIVE-SITE-IDENTITY (evt_5dkq1wcbfvdxm): predicate P2 lies outside that node's objective, since no owner key discriminates which branch ran. Steward recut evt_2teptrm99sf03. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A branch-joined result keeps each alternative

## Objective

A continuation result whose producer has N statically known
alternatives, selected at run time, is discharged and transported for
whichever alternative ran. A producer call whose continuation sources
mix owners is transported as well. The three PX10 slice-1 fixtures build
natively and match the interpreter. Every guard that requires exactly
one producer still holds for a static result.

## Settled inputs (Architect D0 on `86b7cac5e`, evt_5dkq1wcbfvdxm)

- **The predicate (P2).** Each of the detached-result seat, the caller
  cut and the invocation transport is a function of the result. At a
  branch join over a runtime scrutinee, each therefore refuses. The
  refusal is right for a plan with no runtime selection.
- **The guard population** has six one-producer guards:
  - `core.rs` ~12804 (seat);
  - `responses.rs` ~5566, ~5612 and ~5643 (consumer, boundary, cut);
  - `aggregates.rs` ~4770 and ~4842 (environment substitution).
- **The measured walls** (Architect evt_2vjqrxcmee6ra, on SITE-IDENTITY
  without the W3 key):
  - smoke: P2, two causal calls on one result. The edges share the
    result and the producer alternative and differ only in the producer
    construct.
  - argv and live: W3, `responses.rs:3825`, a relay source in a
    mixed-owner transport stage (sources 1724 and 1228). Each stage
    mixes predeclared and specialization owners at one producer call.
    `rt_performing_mixed_owner` pins that refusal on a fixture of the
    same shape.
  - W7 (argv) and `aggregates.rs` ~4770 (live) appeared only after an
    over-admitting W3 key, so they are unverified as real walls.
- **A guard bypass is wrong code.** Suppressing only the seat guard
  built smoke natively, and the native run then trapped
  (`UnclassifiedRuntimeTrap`) where the interpreter succeeds.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Ruled design (Architect AC-0 evt_5efd2tp6dqsj3)

P2 and W3 are two mechanisms. Both framed P2 options are rejected on
measurement: native lowering has no generic indirect path, and an arm has
nothing to eliminate, since each alternative is lowered Carried at its own
origin and the selection is read in another function.

- **W3, an instance key.** The stage map (`transport_producer_owners`)
  and the exclusively-predeclared test in
  `derive_pre_schema_relay_promotions` key on `(producer_call_origin,
  k_identity.producer_owner())`. One provenance owner whose sources at
  one call mix predeclared and specialization emission owners still
  refuses.
- **P2, a labelled alternative transport** (ADR 0023's closed flow set,
  applied to continuation results):
  - R1, the planner record `ProducerAlternativeSet` keyed on (emission
    owner, result), with members sorted by target and the label as the
    index. The admission predicate and record are verbatim in the ruling.
  - R2 (ruled evt_4r4n10cjeeqgf on the advisory evt_2jvqarpm2hb6y), one
    planned tag-1 Record `[Child, Label]` per issued set, keyed on the
    set's (emission owner, result) and governed through
    `emit_checked_aggregate_alloc`. Every member writes that one record
    in its own arm. Sets issue from `pre_schema_transport_sources`
    before any ownership build reads them.
  - R3, one resolver returns `Single` or `Alternatives`. A reader handed
    `Alternatives` emits the closed label switch, and a reader with no
    such arm refuses with a typed PlannerInvariant.
  - R4, the seat returns the labelled word for exactly one issued set.
    Every other multi-edge residual still refuses.

## Deliverable

Two increments of this WP, each its own candidate, both after
SITE-IDENTITY lands.

1. **Increment 1, the W3 key**, plus the re-anchored
   `rt_performing_mixed_owner` pin (Steward evt_49kzxbpbm16tb). The pin
   keeps its promise: the fixture refuses before native execution with a
   typed planner refusal, now the computational-occurrence
   PlannerInvariant (`aggregates.rs:465`), and the interpreter still
   reaches both FsReadFile operations.
2. **Increment 2, R1 to R4**, with the per-guard disposition report and
   the boundary-environment checks of AC-2e.

## Acceptance

Increment 1:

- **AC-1a.** argv_env_pipe advances to W7 and live_child to the P2 seat
  at result 1104, each a verbatim census row.
- **AC-1b.** The re-anchored pin holds, and a row pins "one provenance
  owner at one call under two emission owners refuses", or records it
  unexercised with the reason.
- **AC-1c.** The four bracket suites and the return-fold equivalence
  stay green.

Increment 2:

- **AC-2a-build.** Smoke, argv_env_pipe and live_child build natively,
  each with its census row (issued set count, measured, as AC-2f). No row
  asserts a native run result for these three: the host's native-dispatch
  gate refuses the unpromoted Process operations, and the run's -1 does
  not name its site. Their native run parity is
  `PX10-NATIVE-SPAWN-PARITY` AC-1 (Architect evt_4qb22p4991jmm).
- **AC-2a-exec (parity).** A fixture whose host operations are all
  NativeTested issues a producer alternative set: census set count at
  least 1, and exactly one `SynthesizedUse { role: ProducerAlternative }`
  record for that set. It runs once per label, and the two alternatives
  give distinguishable (request, outcome) traces. Each native run reports
  the same pairs as the interpreter, under a bounded wait. The
  Architect's D0-12 measures the fixture before it is ruled. If no
  NativeTested-only shape issues a set, that is a stop to the Architect;
  do not widen admission to get one.
- **No admission route.** This WP adds no feature, init flag or shim that
  runs an unpromoted operation natively. Promotion is
  `PX10-NATIVE-SPAWN-PARITY` Increment B.
- **AC-2b (guards intact).** `DuplicateResidualEdge` is refused by the
  predicate, plus one named row at the seat. A guard no fixture reaches
  with an issued set keeps its single-producer refusal, and the build
  reports each guard's disposition and the row that reached it.
- **AC-2c (negative, QA).**
  - A reader whose expected member set differs from the issued set
    refuses at planning, with a named row.
  - A mutant that swaps two members' labels at the writer reddens the
    AC-2a-exec parity row.
  - A mutant that writes the bare payload without its label is caught by
    the reader assertion before any field projection.
- **AC-2d (no regression).** As AC-1c.
- **AC-2e (boundary environments; Adversary evt_1sk82gw1hvv76,
  Architect evt_5e2dn1gb0kvg8, evt_4qqmh8t0hgk81, evt_6epa2xgs5mkqf).**
  Increment 1 landed the W3 key alone, so these move here.
  - `core.rs:11212` takes the local capture-order check exactly as ruled
    in evt_6epa2xgs5mkqf, not the unrouted-environment refusal. In
    scratch, reversing the environment origins makes smoke's census run
    refuse at that check.
  - `mod.rs:13702` takes the ruled guard, through a `match` that returns
    `Result` outside `or_else`.
  - `source.rs:5521` takes `refuse_unrouted_boundary_environment` once
    its arrival count over the census and the four instrumented bracket
    suites reads 0. A nonzero count stops to the Architect.
- **AC-2f (record identity, evt_4r4n10cjeeqgf).** Each fixture's census
  reports its issued set count, measured. For smoke, exactly one
  `SynthesizedUse { role: ProducerAlternative }` record exists, keyed on
  its set with two members, and the ledger relates to that id as many
  events as `checked_ih_transport_emissions` holds for set members.

## Stop conditions

- A kernel or trust change, or a post-fork callback.
- An advancing stop the Architect does not rule within Increment 2.
  The Architect holds the §1a count (11 at evt_4qb22p4991jmm), and
  Research is called before the ruling on the twelfth.
- A stop named in the R2 ruling: `occurrence_authority` refuses for a
  set's result, a set member does not reach
  `call_checked_ih_environment_transport`, or a set's meet differs
  between the two ownership builds.

Checks: 7 fired: under the W3 key, `rt_performing_mixed_owner` no longer
reaches the mixed-owner guard and fails closed at a separate wall, so the
pin is re-anchored on its promise rather than its count. 2 fired: the
guard fan-in is eight (six framed, `responses.rs:511` reached behind a
bypass, `:6885` by sweep), so the rule is one resolver, not a list. 8
fired: native build is a proxy, since bypass builds trapped or hung;
AC-2a-exec compares traces with the interpreter on admitted operations,
since a native -1 on a Process fixture can come from any of 18 sites,
and AC-2c's label swap must redden that row.
