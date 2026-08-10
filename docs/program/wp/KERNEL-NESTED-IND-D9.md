# KERNEL-NESTED-IND D9 — ungate `nested-size-uses-lift`, on the repaired fixture

Owner: kernel. Size: S. Node: [[KERNEL-NESTED-IND]] (`active`).

**RE-RELEASED 2026-08-10. The `D10` hold is discharged** —
[[KERNEL-NESTED-IND-D10]] merged as `main` `b2662be4` (candidate `cfc86a83`,
PR #1838, both paths blob-verified). **The row's named witnesses now execute.**

`D10` discharged **both** required properties, and the second is the one that
matters here: not merely that the interpreter stopped panicking, but that the
`Former` arm's coordinate-origin repair makes the full-pipeline NC14 witness
compute **`Nat 3`** rather than falling back to `Neutral`. A selection-removal
control reds it to `Neutral`, and a guard-removal control restores the panic.
**Take that as a fixed input and do not re-derive it.**

**The fixture is already landed and is NOT yours to carry.** `D10` brought in
the `Join` migration (`Join : Bag a -> Bag a -> Bag a`, `One` unchanged, every
reaching operand migrated) together with the repair. **Do not re-apply it, do
not re-derive it, and do not edit it** — a `D9` candidate that touches
`nc14_data_match_lowering.rs` for the migration is re-landing work.

**What remains `D9` when `D10` lands:** the row binding, the gate-marker and
`blocked on` removal at both summary sites, `AC-4`'s discriminating fold
mutation, and the sort-boundary reporting below. **A `conformance/` path still
pulls a Spec vote on this merge Decision** — `D10` carries none and does not.

**Originally released 2026-08-10; the Language hold below is discharged and
stays discharged** —
[[LANG-SELECTOR-SORT-SPLIT-ELAB]] merged as `main` `c0757335`, the crates now
parse `recursive result for`, and the old spelling is at **zero** under
`crates/`. Re-derive your merge-base; do not reuse a SHA from this frame.

**Your fixed inputs are unchanged and I checked rather than assumed it.** The
Language merge touched ten `crates/ken-elaborator` paths and
`tests/nc14_data_match_lowering.rs` **is not among them** — the fixture this
frame quotes is byte-identical. `D2c` and the Runtime `D2d`/`D2e` merges touch
only `crates/ken-runtime` and `docs/`.

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

**AC-5 — the un-gating is EIGHT sites, not one, and I under-counted it.**
Corrected 2026-08-10 by measuring the file rather than trusting the frame.

**My earlier wording named "the gate marker" plus a `line-541` residual list.
Both were wrong.** There is no single marker, and the residual list is at
**557**, not 541 — a stale line citation repoints silently at real content
rather than failing.

**Enumerate by content, not by line number**, because these numbers move the
moment anyone edits the file. In `conformance/kernel/inductive/seed-nested.md`,
for `nested-size-uses-lift` **only**:

| site | what it is |
|---|---|
| the row's `Status:` paragraph | `blocked on KERNEL-RECURSIVE-RESULT-SURFACE`, ending `the marker remains gated` |
| `- given (future binding, gated)` | one of four qualifiers on the row's clauses |
| `- expect (future binding, gated)` | |
| `- fail-closed boundary (future binding, gated)` | |
| `- sort boundary (future binding, gated)` | |
| the file's opening summary | near line 21, naming both rows as gated |
| the residual list | near line 557, naming both rows as gated |

The two summary sites name **both** rows in one sentence, so each needs an edit
that drops `nested-size-uses-lift` and **leaves
`nested-dependent-motive-uses-lift` gated**.

> ### THE SIBLING ROW IS A NEAR-IDENTICAL DECOY. A phrase-keyed edit hits both.
>
> `nested-dependent-motive-uses-lift` carries its **own** `Status: blocked on
> KERNEL-RECURSIVE-RESULT-SURFACE` paragraph ending `the marker remains
> gated`, and its **own** four `(future binding, gated)` qualifiers on
> `given`, `expect`, `fail-closed boundary` and `sort boundary`. **Its
> paragraph even says it is the *same* blocker for the *same* reason.**
>
> ⇒ **A `sed` or global replace on `remains gated`, on `(future binding,
> gated)`, or on the blocker name silently un-gates a row that Kernel measured
> as unestablished.** Scope every edit to the target row's span and **report
> the sibling's site count as unchanged** — that count is the control, and a
> claim that the target row is clean says nothing about the decoy.

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

**The sequencing constraint is DISCHARGED.** `SPEC-SELECTOR-SORT-SPLIT`
respelled this row and [[LANG-SELECTOR-SORT-SPLIT-ELAB]] landed the elaborator
half at `c0757335`. Take the spelling from the row in the file, not from this
frame — measured there today, the `join` branch combines **`recursive result for
xs`** with **`recursive result for ys`**.

## The row's sort boundary — read this before you write the witness

The respell gave the row a clause `D8` never saw, and **it has two halves that
must be treated differently.** Both sit under "sort boundary" in
`seed-nested.md`.

**The half you must bind.** The selected hidden `Nat` result is classified by
`Type`, so **`induction hypothesis for xs` rejects with
`RecursiveResultSortMismatch`, naming `recursive result for xs` as the exact
required spelling.** That is implemented and reachable — bind it.

**The half you must NOT fabricate.** The clause continues: *"If metavariables
leave the selected result ambiguous between `Type` and `Omega`,
`RecursiveResultSortAmbiguous` rejects without a guessed or default selector."*

⇒ **That state is unreachable in the current representation, and this is
settled — do not spend a turn rediscovering it.** `MetaCtx` holds only
`Vec<Option<Level>>`, core `Term` has no term or sort metavariable, `zonk_term`
preserves the `Term::Type` versus `Term::Omega` constructor, and kernel
`classify` matches that constructor. A level solution moves the payload, never
the constructor. `RecursiveResultSortAmbiguous` is landed **defined and
Display-pinned with zero production construction sites**, reserved to
[[LANG-SORT-META-CAPABILITY]].

**The clause is conditional and its antecedent cannot arise, so it is satisfied,
not breached.** Do not build a witness for it, do not construct a malformed term
to trigger it, and **do not escalate it** — this paragraph is the ruling. It
cost Language a full turn and an Architect rejection; it should cost you
nothing.

**Report it as excluded with its reason**, so a reader does not read the row's
silence on that half as an oversight.

## Validation

Targeted only. `-p <crate>` or `--test <name>`, **never `--workspace`**. If any
enum variant is added or changed, the floor is a full `-p <crate>` test build,
because a suite-scoped run cannot observe an exhaustive `match` in a sibling
target. "No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. Both are good
outcomes.
