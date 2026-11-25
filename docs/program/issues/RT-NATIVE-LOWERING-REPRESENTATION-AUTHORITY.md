---
id: RT-NATIVE-LOWERING-REPRESENTATION-AUTHORITY
title: "Native lowering decides per site which Lowered representations a consumer accepts: five match consumers hold four different hand-kept scrutinee subsets, and lower_primitive_call routes runtime operands symbol by symbol with a fall-through refusal. Each upstream repair that makes a value runtime exposes the next consumer (PX11 Inc2 entries 5, 11, 12). Give match scrutinees one exhaustive classification every consumer goes through, and primitives one complete per-symbol operand table"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect §1b answer at PX11 Inc2 entry 12 (evt_5116n6wy5wt40): entries 5, 11 and 12 share predicate R, and the fix is a structural closure, not a 13th per-site arm. L1 Linux ABI program, Steward order."
---

# One representation authority for native match scrutinees and primitive operands

## Objective

Every native match consumer classifies its scrutinee by one total function, and
every primitive reads its operand routes from one complete table. The PX11
exchange gets past the composed-match refusal at origin 3919. A representation
that a consumer cannot lower refuses at a written arm that names the consumer
and the representation.

## Settled inputs (Inc2 tip `74853120e`; Architect `evt_5116n6wy5wt40`)

- **Predicate R.** A native-lowering consumer enumerates, per site, which
  Lowered representations of one checked type it accepts. An upstream repair
  that moves a value from compile-time to runtime exposes a consumer that never
  enumerated the new representation. The members are PX11 Inc2 entries 5
  (`uint8_to_int`), 11 (`bytes_concat`) and 12 (the composed match on a
  BorrowedOption).
- **The match consumers** (`lowering/core.rs` at `74853120e`):
  - ordinary, `lower_runtime_match_expr` `:16980`;
  - composed, `lower_computational_match_value_composed_once` `:4639`, whose
    let-else at `:5205` is entry 12;
  - producer, `lower_computational_producer_match` `:7043`;
  - source-frame, in `lowering/source.rs` (`:2518`);
  - the Bool-only tree-producing If (`:4299`).
  The ruling's table records which representations each one accepts.
- **The primitive routes.** `lower_primitive_call`
  (`lowering/core/primitive.rs`) keeps carried routes per symbol. Everything
  else falls through to `specialized_operands_at`, which refuses a carried
  operand. The ruling names the symbols that have no route.
- **Already proved, kept.** The entry-11 fix is commit `74853120e` on
  `wp/PX11-NATIVE-STREAM-SOCKET-PARITY-inc2-bytes-concat`: a runtime
  `bytes_concat`, passing T1, T2, M1, M2 and the transfer rows 7/7 (Architect
  `evt_7ahjpbxbgvw09`). Its delta from `87ebb80ee` touches only `aggregates.rs`,
  `primitive.rs` and `core/tests/constructors.rs`, and it merges cleanly onto
  main without Inc2's other commits (tree `385c9aafb`).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

- The candidate's base is origin/main. Its first commit carries the entry-11
  delta unchanged. Inc2's other commits stay on Inc2.
- Change the five match consumers and `lower_primitive_call`, adding one
  classification module and one table, as items 1 to 4 of the ruling describe.
- Existing lowering bodies move behind the classification and keep their
  behaviour. `carrier_handle_disposition` stays the only tag/class authority.
- Out of scope: the exchange fixture or profile, the host, the ABI, and the
  boundary helper graph.

## Deliverable

One candidate that lands before PX11 Inc2 resumes.
- The match authority comes first: one `MatchScrutinee` classification with no
  wildcard, every consumer matching it exhaustively, and the borrowed-option
  selection shared in an ordinary-join mode and a composed-continuation mode.
- The primitive operand table follows in the same candidate.
- AC-0 (Architect design ruling, arms registered before the build) comes
  before any code.

## Acceptance

- **AC-1.** Measured, not committed: with the candidate applied to the Inc2
  replay, the unchanged exchange (fixture `c05724514`, profile `cdb0089c`,
  `RUST_MIN_STACK` unset) gets past origin 3919. A K-next-style refusal
  elsewhere is a pass arm and becomes the next inventory entry.
- **AC-2.** A pin that the classification is total over Lowered with no
  wildcard, and a pin that the primitive table is complete. Mutations that add
  a wildcard arm, drop one variant's arm in any consumer, or remove one symbol's
  entry must each fail to compile or turn red.
- **AC-3.** Composed-route rows for BorrowedOption: the `Some` case and the
  `None` case each continue through one pending eliminator. The 7/7 entry-11
  rows still pass in the same invocation.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A consumer's existing body changes behaviour when moved behind the
  classification.
- A representation that the table shows as accepted needs a new lowering, not
  a move.
- AC-1 shows the exchange still refusing at origin 3919.

Checks: 4 fired. I read the five consumers' anchors at `74853120e`
(`core.rs:4602`, `:4639`, `:5205`, `:7043`, `:7308`, `:16980`, `:17029`,
`:17041` and `:4299`, plus the `source.rs:2518` refusal text). `primitive.rs`
is at `lowering/core/primitive.rs`. 4 fired. `git merge-tree --merge-base
87ebb80ee 224272297 74853120e` is clean, so the entry-11 delta stands alone on
main. 8 fired. Adding one BorrowedOption arm to the composed consumer would
pass AC-1 alone, so AC-2's totality and completeness mutations are the controls
that only the closure passes.
