---
id: RT-NATIVE-PRODUCER-ALTERNATIVE-ANSWER-READER
title: "A checked-IH transport's answer is written as a labelled producer-alternative record, but nothing in the native backend reads that label and unwraps its Child: the seat's assertion validates the word and returns it still labelled, and the word travels as an ordinary Carried operand, so an ordinary Result match consumes it and traps at run time (B-closure stop 11, trap 46). AC-0 measured that no labelled answer is both minted and read, so delete the label: the writer returns the producer's word and the seat passes it through"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect closure form evt_7xv7216msypg (B-closure D0 round 24, stop 11, inventory entry 15), recommending a separate closure WP; Steward recut. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# The producer-alternative label is deleted

## Objective

No native answer is labelled. A checked-IH transport's writer returns
the producer's own word for every member of an issued set, and the
seat's Alternatives arm passes the carried word through. Merged at
`5dfe23688` (PR #5212).

## Settled inputs (main `7877abf22`; measured at `7a1158c5d`, evt_7xv7216msypg)

- **The writer.** `call_checked_ih_environment_transport` writes the
  labelled record (`lowering/core.rs:9430` to `:9459`). It decides
  labelled-ness from the planner's set membership
  (`producer_alternative_set_of(identity)`, `:9430`).
- **No reader.** The only non-test consumers are that writer, the
  detached seat's Alternatives arm (`core.rs:12926` to `:12938`) and the
  containment count (`compiled.rs:123`). The seat calls
  `assert_producer_alternative_record` (`:13114`), which checks class,
  tag, field count and label range, then returns the word still
  labelled. `select_checked_ih_transport_alternative` (`calls.rs:984`)
  and the four `residual.rs` label readers (`:111`, `:151`, `:209`,
  `:281`) read the slot residual, the transport's input side.
- **The operand plane.** The labelled word travels as
  `LoweringOperand::Carried`, the same kind as any value, so the match at
  (Spec13, origin 641) cannot classify it and nothing refuses it at
  compile time.
- **The keys.** `ProducerAlternativeSet` carries `emission_owner` and
  `result` (`alternatives.rs:15` to `:19`), and the transport carries
  `destination_owner` (`core.rs:9372`). `CarriedResidualWord` is the
  precedent for an operand that carries its own key.
- **The counterexample.** On compose, terminal
  `-8121433039591964927` decodes to class 5, 2 fields, label 0, written
  at `core.rs:9455:36` in funcid82/Spec13 (occurrence 322).
- **Not established.** Which set the Spec13 member belongs to. The only
  live set observed is (Spec2, result 1208). Whether the raw Child
  satisfies the match at 641: it is class 4 `ITree::Ret`, not a
  `Result`.
- **Held.** PX10 B-closure is parked at `5cd38e8b5` on this WP. Its
  AC-1 reruns after this WP lands.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

As ruled at AC-0 (Architect D0-2 `evt_6wbcf81pb62p8`; Steward scope
`evt_1qd1scg5w545h`, replacing `evt_7mftn8rw9srj6`). On the four
committed builds, 24 records were labelled and none was consumed; the
12 seats each received an unlabelled word yet asserted a labelled
record. So the label was a property two planes disagreed about, with no
consumer.

- **R1.** `call_checked_ih_environment_transport` returns `returned` for
  every member.
- **R2.** The seat's Alternatives arm passes `Carried` through and
  refuses any other operand with `PlannerInvariant`
  `ProducerAlternativeSet`. `assert_producer_alternative_record` is
  deleted.
- **R3.** `producer_alternative_record` and its planned aggregate are
  deleted. `ProducerAlternativeSet` stays for membership and reader
  resolution.
- **Not built.** `CausalEdgeDischarge`, a labelled operand kind
  (`LoweringOperand::LabelledAlternative` is reserved) and an issued
  reader. A measured labelled population would reopen them.
- **Carried to PX10 entry 15.** The `compiled.rs:119` containment
  (B-closure deletes it), the dead planned transports, and the routing
  predicates at `core.rs:3686` and `:3748`.

## Acceptance

- **AC-0 (D0).** Two rounds, both on scratch: the Spec13 member's set
  (`evt_7deaft1cqphfv`), then the consumer census (D0-2).
- **AC-1, no labelled mint.** Over argv and live, a writer observation
  records every answer raw, including the six foreign members. Restoring
  identity-membership labelling reddens it.
- **AC-2, seat pass-through.** Over argv, live and smoke, every
  issued-set seat passes its word through and emits no record
  assertion. Restoring the assertion reddens it.
  `px10_issued_set_reader_mismatch_refuses_at_planning` is unchanged.
- **AC-3.** Full-workspace CI green on `843fccca5` (run 41870493245).
  No parity is claimed; B-closure's next fact is trap 44.

## Stop conditions

- A kernel or trust change.
- A closure or containment check that still requires the deleted
  aggregate to be emitted: stop to the Architect.
- A pin that can be met only by running B-closure's parity rows.

Checks: 6 fired: the labelled population was counted at both seams on
the committed builds, 24 minted and 0 consumed, so the record, operand
kind and reader were not built and the label was deleted. 5 fired:
labelled-ness was decided on the planner's identity plane and needed on
lowering's discharge plane; after deletion no plane carries it. 8 fired:
each pin's failing configuration (identity-membership labelling, the
restored seat assertion) produces a different observation from the fix.
