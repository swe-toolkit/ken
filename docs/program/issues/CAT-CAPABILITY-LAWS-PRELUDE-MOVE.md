---
id: CAT-CAPABILITY-LAWS-PRELUDE-MOVE
title: "Move the prelude's unkeyed capability laws into their single catalog consumers: transfer_count_request_budget with its bounded proof into Capability.System.Buffer (retargeted over Nat add), and write_all_complete, write_all_call_bound, write_all_first_error and write_all_all_success with their attached proofs into Capability.System.IO; second L3 slice of the minimal-prelude program"
status: merged
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: the prelude is the minimal set required; convenience names not required by the prelude rules are technical debt and move to packages. Move units ruled by Architect evt_5mb5qr5exmrke (L3-2, laws slice) on the program design evt_6e5mg39d37bad. Steward-filed per COORDINATION section 2."
---

# Move the unkeyed capability laws into their consumers

## Objective

`Capability.System.Buffer` and `Capability.System.IO` declare the laws they
use, and no longer reach them through the prelude fall-through.

## Settled inputs -- Architect `evt_5mb5qr5exmrke`, measured at `ee22122af`

- **Movable:** no internal reader, no native reader, and exactly one catalog
  consumer.
  - Into `Capability.System.Buffer`: `transfer_count_request_budget` with
    `proof bounded`.
  - Into `Capability.System.IO`: `write_all_complete` with
    `success_complete`, `write_all_call_bound` with `termination`,
    `write_all_first_error` with `first_error`, and `write_all_all_success`
    with `all_success`.
- **Retiring:** `buffer_nat_add` has the same recursion as
  `Data.Numeric.Nat.Arithmetic.add`, so a verbatim copy would reimplement a
  shared component. `buffer_suc_cong`, used only by `termination`, gives way
  to `Core.Logic.Transport.cong`, which changes a proof body, not a
  statement.
- **Staying (L2-3's companion closure, not this slice).** A name stays if its
  body matches `PrivateBufferSpan`/`PrivateTransferCount`, or if a staying
  internal body reads it (`writeAt`, `spanBytes`, `write_all_advance_span`,
  `private_write_all_fuel`, `writeAll`, `write_all_exact_prefix_prop`,
  `prelude.rs:2745-2838`). That keeps `buffer_span_*`, `buffer_nat_to_int`,
  `transfer_count_nat`/`_int`/`_remaining`, `transfer_count_positive_prop`,
  `transfer_count_positive` and `write_all_exact_prefix_prop` with its
  `exact_prefix` proof.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- The moved functions and attached proofs are declared in their consumers,
  with the prelude's statements.
- **One sanctioned statement change:** `transfer_count_request_budget` is
  defined over `add`, so `bounded` stays `Refl`. Buffer's `bounded` statement
  (`Buffer.ken.md:31-32`) reads `add (transfer_count_nat count)
  (transfer_count_remaining count)`.
- The prelude registrations stay until the L2 flip. No `crates/` change except
  test expectations.

## Acceptance

- **AC-1 (import proof, not a kernel verdict).** A kernel-verdict control
  cannot catch a fall-through here: the prelude copies are transparent and stay
  registered, so they check green.
  - With the five moved functions and the two retired names
    (`buffer_nat_add`, `buffer_suc_cong`) removed from `globals` before
    loading, Buffer and IO still check, and every attached-proof selector
    (`proof bounded for ...`, `proof success_complete for ...`, and the rest)
    still resolves. Attached-proof globals need no removal: a local selector
    resolves to the module-qualified key (Architect `evt_6y1zf4fje6rgt`).
  - **Base half (Architect `evt_7eenk75kws3r9`).** On base, with the same
    seven names removed, roots-load Buffer and IO in separate loads:
    - Buffer fails `UnresolvedCon { name: "transfer_count_request_budget" }`
      (a type-position head), and the span's source slice equals that name;
    - IO is predicted to fail `UnresolvedCon { name: "write_all_call_bound" }`
      the same way. A different removed name, or `UnboundName` naming a
      removed name, is recorded as a measurement and pinned.
    - Any variant outside `{UnboundName, UnresolvedCon}`, or any name outside
      the removed set, is a STOP.
- **AC-2 (statements).** Every consumer theorem statement is byte-identical
  except the sanctioned `bounded` retarget. Attached proofs keep their
  statements.
- **AC-3 (census, Architect `evt_24084xfdemttg`).** A row is the ambient set
  of the entry's whole load closure, imports and moved proof bodies included.
  Exact sets:
  - Buffer goes from 15 to 13: it loses `buffer_nat_add`,
    `transfer_count_request_budget` and
    `transfer_count_request_budget::bounded`, and gains `Proved` through the
    new `Data.Numeric.Nat.Arithmetic` import (Arithmetic's row is `[Equal,
    Proved]`). Row: `BufferSpan`, `BufferWindow`, `Equal`, `MkBufferWindow`,
    `Proved`, `TransferCount`, `buffer_span_budget`, `buffer_span_length`,
    `transfer_count_int`, `transfer_count_nat`, `transfer_count_positive`,
    `transfer_count_positive_prop`, `transfer_count_remaining`.
  - IO goes from 15 to 8: it loses the four `write_all_*` functions and their
    four `::` proofs, and gains `Proved` from the moved proof bodies
    (`termination`, `all_success`, `success_complete`). Row: `BufferSpan`,
    `Equal`, `Proved`, `ResourceError`, `TransferCount`, `Unit`,
    `write_all_exact_prefix_prop`,
    `write_all_exact_prefix_prop::exact_prefix`.
  - `Proved` is a floor type already ambient in 37 rows. Any other addition,
    any change to another row, or `Suc` or `False` as a leaf is a stop.
  - If `LANG-PRELUDE-FLOOR-FIFTEEN` lands first, both rows also lose `Equal`
    and `Proved` (Buffer 11, IO 6); whichever lands second re-predicts before
    rebaselining.
  - No `trusted_base()` change. Targeted builds only, through
    `scripts/ken-cargo`; no-regression means green in CI.
- **AC-4 (non-catalog consumers, Architect `evt_1kkygg9f7psn8`).** Enumerate
  every non-catalog source (`crates/*/tests`, `r_layer_tests`, `ken-cli`
  fixtures, `examples/`, `conformance/`) that loads or exposes
  `Capability.System.Buffer` or `Capability.System.IO`, transitive importers
  included. Classify each loose source that names a moved or retired name
  bare as (a) consumes the moved law, so it exposes the consumer module;
  (b) independent; or (c) a labelled transition sentinel. Post the list with
  the candidate. The moved names are transparent, so a missed site stays
  kernel-green, and this sweep is its only guard.

## Stop conditions

- Any other catalog module consumes Buffer's `bounded` theorem.
- A statement other than `bounded` changes, or a predicted import name is not
  already ambient.
- Any `trusted_base()` growth.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## After landing

The staying accessors are L2-3's: if D0 puts `BufferSpan` or `TransferCount`
in B, its companion closure enters with it, atomically, and the `30 §4`
companion-closure rule is extended from `Pair` to keyed types with private
constructors on that branch (Architect `evt_5mb5qr5exmrke`).
