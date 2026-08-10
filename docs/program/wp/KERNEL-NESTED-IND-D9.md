# KERNEL-NESTED-IND D9 — ungate `nested-size-uses-lift`, on the repaired fixture

Owner: kernel. Size: S. Node: [[KERNEL-NESTED-IND]] (`active`).
**Held pending [[LANG-SELECTOR-SORT-SPLIT-ELAB]] — see Contention.**
Fixed inputs measured at `main` `c380ab32`. `D2c` lands between framing and
release and touches only `crates/ken-runtime` and `docs/`, so every input this
frame names is unchanged by it.

## This is D8 again, with the blocker removed

`D8` reached outcome (b) and attributed a refusal to the elaborator. **That
attribution was false and is withdrawn** — see
[[LANG-NESTED-MATCH-LIFT-ALIGNMENT]], now `closed`. There is no elaborator
defect and no Language work in front of this.

**The refusal was D8's own fixture.** `D8` changed the `Join` constructor in
`crates/ken-elaborator/tests/nc14_data_match_lowering.rs`
(`NESTED_LIFT_NAT_THREE_SOURCE`) from `Join : a -> a -> Bag a` to `Join : Bag a
-> Bag a -> Bag a`, and left the reaching constant supplying bare `LiftRose`
operands where `Bag LiftRose` had become required. `TypeMismatch { expected:
(Dg570 Dg582), found: Dg582 }` is `Bag LiftRose` against `LiftRose`, and nothing
more.

## The repair, already measured — do not re-derive it

Language verified at exact `02070073` (`evt_4zdhnrdpf0dw`) that with `Join`
recursive **and** the reaching operands migrated, the outer-`Suc` selector form
elaborates and completes final kernel checking:

```
const liftSizeResult : Nat = liftSize \
  (LiftNode (Join LiftRose (One LiftRose LiftLeaf) (One LiftRose LiftLeaf)))
```

**`One : a -> Bag a` stays unchanged.** Only `Join` becomes recursive, and that
asymmetry is load-bearing — migrating `One` too would change the topology the
row specifies.

`scripts/ken-cargo test -p ken-elaborator --test nc14_data_match_lowering
nested_inductive_elaboration_preserves_trusted_base_set -- --exact` is 1/0 with
the trusted-base comparator holding.

**Take that as a fixed input.** You are not re-measuring whether the pipeline
works; you are binding the row to witnesses.

## Deliverable

`D8`'s outcome (a): bind
`conformance/kernel/inductive/nested-size-uses-lift` to executing witnesses
named by the row itself, and remove its gate marker and `blocked on
KERNEL-RECURSIVE-RESULT-SURFACE` status line.

## Acceptance criteria

These are `D8`'s, carried forward unchanged except where the fixture repair
makes one of them cheaper.

**AC-1.** The row's two named executing witnesses exist and pass:
`nested_recursive_bag_rose_elaborates_checks_erases_and_interprets_at_nat_three`
and `nested_recursive_bag_join_residual_folds_all_leaves_at_nat_three`. Both
drive the selector through the **full** pipeline — surface elaboration, kernel
checking, erasure, interpretation — the first for the Nat-3 `Bag`/`Rose`
computation, the second for a deeper residual `Bag.join` topology.

**AC-2.** The row's three named degenerate shapes are excluded **by
construction, not by intent**: a finite unroll, a depth-three snapshot that
never consumes a residual recursive result, and a header naming only one side of
the `join`. State for each which property of the witness rules it out.

**AC-3.** The kernel structured-IH/iota witness
`production_nested_lift_is_consumed_and_iota_computes` is named in the binding,
per the row's executing-binding requirement.

**AC-4 — the discriminating control, and it is the point of the case.** With the
correct lifted IH the result is `3`; an implementation that supplies a lift but
drops or ignores its leaves computes `1`; a guard-deletion-only implementation
cannot type-check the definition at all. **A witness that only asserts `3` does
not discriminate.** Show the `1` outcome is reachable under a mutation of the
fold and that the witness reds on it. One mutation, reported with its result.

**AC-5.** The gate marker and the `blocked on` status are removed from **this
row only**, and the file's summary paragraph no longer claims it is gated.
**The line-21 summary and the line-541 residual list are separate sites and both
say so.**

**AC-6 — new, and it is why D8 cost two rings a turn.** Any fixture edit that
changes a constructor's type **migrates every reaching term in the same
commit**, and the candidate contains no source that only exists in a restored
working tree. **A probe you restore without committing is an attribution nobody
can re-read** — D8's failing source survived only in one seat's scrollback, and
recovering it is what unblocked this. If a measurement is worth reporting, its
input is worth committing.

## Excluded scope

- **`nested-dependent-motive-uses-lift` is OUT.** Kernel measured it as
  unestablished — the test file has no `Omega`, `Proof`, `motive`, or
  dependent-motive residual-`Bag` witness. It keeps its gate. **The rows share a
  blocker sentence, not a witness**, so do not update its status on the strength
  of this row's outcome.
- **`crates/ken-runtime` planner and lowering stay OUT.** `AC-K12`'s native
  stage remains Runtime-owned.
- **No new capability.** If the pipeline cannot express the row, that is a hard
  stop and it comes back to me. Do not invent the missing surface, and do not
  re-attribute to Language — that route was measured and closed.
- `wp/KERNEL-NESTED-IND-D5` and `-D6` are squash-merge leftovers, verified by
  blob. Do not reopen either.

## Contention

Paths are `conformance/kernel/inductive/seed-nested.md` and the
`crates/ken-kernel` / `crates/ken-elaborator` test targets. Runtime is on
`crates/ken-runtime`. **Language may be on `crates/ken-elaborator/src` under
[[LANG-SELECTOR-SORT-SPLIT-ELAB]] once the spec lands — your paths are test
targets and the conformance row, so the intersection is empty, but check before
you write.** A `conformance/` path pulls a Spec vote on the merge Decision.

**A sequencing constraint, and it is why this frame is not released yet.**
`SPEC-SELECTOR-SORT-SPLIT` respells this row: `nested-size-uses-lift` now
requires **`recursive result for xs`**. The crates still parse only `structural
result of`, and the elaborator respell is
[[LANG-SELECTOR-SORT-SPLIT-ELAB]].

⇒ **Until that lands, no executing witness can spell the selector the way the
row it binds specifies.** Writing the witness in the old spelling would bind the
row to a source that contradicts it, which is worse than leaving it gated.

**Do not start this deliverable until `LANG-SELECTOR-SORT-SPLIT-ELAB` has
merged.** I will release it then; if you receive this frame before that, the
kickoff is the error and it comes back to me. When you do start, take the
spelling from the row in the file rather than from this frame.

## Validation

Targeted only. `-p <crate>` or `--test <name>`, **never `--workspace`**. If any
enum variant is added or changed, the floor is a full `-p <crate>` test build,
because a suite-scoped run cannot observe an exhaustive `match` in a sibling
target. "No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes.
