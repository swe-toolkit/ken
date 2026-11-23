---
id: RT-NATIVE-BORROWED-LANE-CHECKED-TYPE
title: "Native lowering admits a match onto the borrowed process-input lane from its case set's constructor names alone, so a list of any element type gets a borrowed leaf, and a nested match on an element constructor outside the four-row borrowed table refuses at native-build although provenance makes that leaf unreachable. Key the admission on the scrutinee's checked type, carried as an explicit field on the runtime Match, with today's rule kept wherever the type is not rigidly visible"
status: draft
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_7dcsn0qdafqmb on ABI-S2's A-d stop (language-implementer evt_pkyezk8nayxm; hold evt_7qcp68q6b803v; research advisory evt_2by4w115x1k81). B-a evt_74gjjrbrv0jnh: the wall predates ABI-S2. Steward scope evt_wrp06ygjgj1: its own node on the Language ring, ahead of ABI-S2's consuming fixture. Closes the Architect carry 'borrowed-leaf admission' and the predicate of RT-NATIVE-BYTES-TO-LIST inventory entry 1 and ABI-S2 entry 4. L1b Linux ABI program, Steward order."
---

# Borrowed-lane admission by checked type

Withdrawn to draft as a carry (Steward `evt_611zz5ccb9z9w`). On main
`67128ba0f` the landed RT-NATIVE-BYTES-TO-LIST backstop aborts in the dead
borrowed leaf, so the B program builds and runs (Architect
`evt_3jvnz3jc5g36p`), and ABI-S2's wall was elsewhere (I-probe
`evt_3va3hpc27g78q`). What remains is structural. Re-release when a consumer
of a borrowed-leaf binder that the backstop does not cover refuses, or when
a third abort site is proposed. The design, `evt_7dcsn0qdafqmb` §§1-2 with
the recorder AC of `evt_3jvnz3jc5g36p`, stands.

## Objective

A natively built program that matches on the fields of a list element
whose checked type cannot be borrowed builds and runs. It agrees with the
interpreter. Programs over `argv`, `env` and generic lists keep the
borrowed lane they use today.

## Settled inputs (main `5bc385c1c`; the ruling's anchors)

- **The counterexample (B-a).** `data P = MkP Int`, a `total` over
  `List P` with a nested `match p`, called from main on a constant
  two-element list. `ken check` and `ken run` give 0. The native build
  refuses at `lower_borrowed_match`: "has no borrowed constructor
  identity".
- **The two admission readers.** `source.rs` ~`:4059` (`admits_borrowed`)
  and `joins.rs` ~`:729` (`admits_borrowed_input`). Both test only that
  every case names a constructor in `borrowed_constructor_identity`
  (`mod.rs` ~`:15051`, four rows).
- **The fact's home.** Erasure's ordinary checked-match lowering has the
  checked type in hand (`CheckedCoreMatchView`). `RuntimeMatchCase`
  (`ir.rs` ~`:806`) carries only a constructor and binders.

The ruling (`evt_7dcsn0qdafqmb` §§1-2) is the design:
- `BorrowedLaneAdmission { ByCaseFamily, ExcludedByCheckedType }` on
  `RuntimeExpr::Match`;
- `checked_type_rigid_family_head`, true only on an `ind_former` head;
- the List and Prod exclusion rule;
- both readers conjoin `ByCaseFamily`.

Build it as written. The borrowed table stays four rows, and the frame
fingerprints do not change.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The new field and its construction sites (all `ByCaseFamily` except
erasure's ruled case), the helper in `checked_core.rs`, the two readers,
and the rows below. No change to the borrowed table, the process-input
binding, the prelude, or any fixture written to avoid the nested match.

## Acceptance

- **AC-1.** The ruling's rows (a) to (f): the B-probe program native-runs
  to exit 0; the argv, env and generic-list controls keep the borrowed
  lane and agree; an erasure pin on the emitted field; site attribution
  for both readers, with GAP and its reason if one is unreachable.
- **AC-2.** The ruling's mutations m1 to m4, each restored byte-identically.
  m3, which excludes every List, must redden the argv control.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

The ruling's pre-registered C-b and C-c go to the Architect:
- an element head arrives tagged `const`;
- a control reddens with every mutation restored.

Do not widen the rule to `const`. Do not add a table row.

Checks: 7 fired. The counterexample is the measured B-a program, re-run
as written. 8 fired. Over-exclusion still makes the counterexample green,
so m3 against the argv control is the discriminator. 2 fired. The fan-in
is the two named readers. The ruling's sweep found no third reader of
the borrowed table outside `lower_borrowed_match`.
