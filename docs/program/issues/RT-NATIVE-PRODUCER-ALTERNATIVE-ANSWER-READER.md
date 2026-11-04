---
id: RT-NATIVE-PRODUCER-ALTERNATIVE-ANSWER-READER
title: "A checked-IH transport's answer is written as a labelled producer-alternative record, but nothing in the native backend reads that label and unwraps its Child: the seat's assertion validates the word and returns it still labelled, and the word travels as an ordinary Carried operand, so an ordinary Result match consumes it and traps at run time (B-closure stop 11, trap 46). Give each causal edge one planner discharge that records whether its answer is labelled, carry a labelled answer as its own operand kind that ordinary consumers refuse at compile time, and issue the one reader that switches on the label"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect closure form evt_7xv7216msypg (B-closure D0 round 24, stop 11, inventory entry 15), recommending a separate closure WP; Steward recut. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A labelled answer has one reader

## Objective

The native backend reads a checked-IH transport's labelled answer only
through one issued reader. That reader switches on the label, unwraps
each member's Child, and joins them. Any other consumer handed a
labelled answer refuses at compile time.

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

C1 to C3 of the Architect's closure form (evt_7xv7216msypg), in one
runtime candidate:

- **C1.** For each causal edge the planner records one discharge,
  either the pending-Vis row or the checked-IH transport, never both.
  It also records whether that transport's answer is labelled. An
  answer is labelled only if it reaches the issued set's reader,
  (`set.emission_owner`, `set.result`). The writer, the seat and both
  Construct arms read that record and decide nothing locally.
- **C2.** A labelled answer is its own operand kind, holding the word
  and its set's key. Every exhaustive operand match decides on it. An
  ordinary consumer refuses it with `PlannerInvariant` naming
  `ProducerAlternativeSet`.
- **C3.** One issued reader switches on the label, unwraps each
  member's Child, and joins them on the shape of the value join at
  `calls.rs:1059` to `:1082`. `assert_producer_alternative_record` is
  that reader's precondition.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded timeout.
  The runtime implementer runs it on scratch and reports. The Architect
  rules from that report, and does not build or test. It measures:
  - which set the Spec13 member at `core.rs:9455` belongs to, which
    tells C1's case (a label written for another owner's reader) from
    C3's (a missing reader);
  - a census of every consumer of a labelled word, enumerated from the
    operand matches, not from the call path in front of the D0.

  It then rules the operand kind's name, the discharge record's home,
  and the reader's site.
- **AC-1, refusal pin.** A named ken-runtime unit test hands an ordinary
  consumer a labelled answer, and it refuses at compile time with the
  ruled reason. Deleting the refusal, or giving the ordinary consumer a
  fallback that treats the word as `Carried`, reddens it.
- **AC-2, reader pin.** A named test drives the issued reader on a
  labelled answer of each member and reads back that member's Child. A
  mutant that swaps two members' labels at the writer reddens it, and
  so does a mutant that writes the bare payload without its label.
- **AC-3.** Full-workspace CI on the exact candidate SHA is green. No
  native parity row is claimed; B-closure's AC-1 owns parity.

## Stop conditions

- A kernel or trust change.
- A causal edge that needs both discharges.
- A pin that can be met only by running B-closure's parity rows.

Checks: 2 fired: the reader census is the Architect's measured one (the
writer, the seat's assertion and the containment count), and AC-0
re-enumerates it from the operand matches, because the `Carried` plane
reaches every consumer. 5 fired: labelled-ness moves onto the operand
plane that consumers read, keyed on the set's (`emission_owner`,
`result`), not on the writer's local set membership. 8 fired: AC-1's
fallback mutant is the `Carried` flow that reached the match at 641, and
AC-2's bare-payload mutant is checked by a reader that unwraps, not by
the seat's assertion.
