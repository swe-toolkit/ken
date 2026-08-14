---
id: SURF-gadt-field-sugar
title: "Tracker node for the landed dependent-constructor field-sugar slice (`34 §8`): points at `docs/program/wp/SURF-gadt-field-sugar.md`; the frame's own D0 audit bounded the slice to declaration-only labels, which is why AC3's unknown/missing/extra-field checks have no corresponding test -- there is no named-argument constructor expression or pattern syntax to check them against"
status: merged
owner: language
size: S
gate: none
depends_on: [SURF-gadt-coverage-diagnostics]
blocks: []
github: null
origin: "[[LANG-GADT-SEQUENCE-TRACKER-GAP]] D1 -- Steward release evt_6jb0p5w0zx69p, thread thr_11nhrpk58gkqt. Frame is `docs/program/wp/SURF-gadt-field-sugar.md`; landed at squash-merge `e026e721` (\"SURF-gadt-field-sugar: declaration labels\")."
---

## What this is

The tracker node for `SURF-gadt-field-sugar`, the fourth and last
dependent-constructor build slice `34 §8` names. Full design content lives in
`docs/program/wp/SURF-gadt-field-sugar.md` -- not restated here.

## Status, measured at `origin/main` `f78b486d`

**Landed.** Squash-merge `e026e721` is an ancestor of `f78b486d`. Tests are
split between `explicit_data_parser.rs` (parse/reject) and
`explicit_data_elaboration.rs` (lowering).

The frame's own D0 audit (item 3: "is declaration-only sugar the bounded
slice for this WP?") resolved to **declaration-only**: the landing commit's
own message states plainly that "constructor expressions, constructor
patterns, and record-style field lists inside explicit dependent constructor
signatures remain unchanged and out of scope." Confirmed independently by
`grep -rn field_labels crates/ken-elaborator/src/*.rs`: the field carried by
`CtorDecl`/`ExplicitDataCtor::Simple` flows through `ast.rs`, `parser.rs`,
`resolve.rs`, `modules.rs`, and into a diagnostic-naming use in `data.rs:543`
(`UniverseArgument.name`) -- never into any expression- or pattern-side
structure. This shapes how AC3 reads below.

Per-AC discharge, obligation-keyed:

- **AC1 (named constructor declaration sugar lowers positionally).** Search:
  a named-field declaration elaborating to the same kernel telescope shape as
  the positional equivalent, with positional constructor/eliminator/method
  arities. Found two forms:
  `legacy_named_constructor_field_sugar_lowers_to_positional_constructor`
  (`explicit_data_elaboration.rs:45`, legacy `data Point = MkPoint { x, y }`)
  and `explicit_where_named_constructor_field_sugar_lowers_to_positional_
  constructor` (`:58`, explicit-family `PairBox`/`PairBoxed { first, second
  }`). Both assert `ind.constructors[0].args.len()` matches the field count
  and construct through the ordinary positional path
  (`MkPoint 1 2`/`PairBoxed Int 1 2`).
- **AC2 (existing positional forms are unchanged).** Search: prior-slice
  positional tests still present and green. `non_indexed_explicit_family_
  elaborates_and_constructor_is_usable` (`:24`, from `SURF-gadt-elaboration`)
  and the full `indexed_*` test set (from `SURF-gadt-coverage-diagnostics`)
  remain in the same file, unmodified by this slice's diff (`git show --stat
  e026e721` touches `explicit_data_elaboration.rs` only with a `+34` line
  addition, consistent with pure test-addition rather than edits to existing
  cases).
- **AC3 (label checking is fail-closed).** Search: unknown/duplicate/missing/
  extra labels rejecting "in every syntax form this WP chooses to support."
  Since D0 chose declaration-only, the only applicable case is a
  **duplicate** label at declaration time (unknown/missing/extra apply to
  named-argument *use* sites, which don't exist in this slice's scope).
  Found: `named_constructor_field_sugar_rejects_duplicate_labels`
  (`explicit_data_parser.rs:230`) -- `data Point = MkPoint { x : Int, x :
  Int }` rejects, message contains `"duplicate field \`x\`"` and
  `"MkPoint"`. Confirmed no expression/pattern-side label-checking test
  exists, consistent with the bounded scope rather than an untested AC.
- **AC4 (indexed-family semantics are unchanged).** Search: whether this
  slice's diff touches coverage/motive/reachability code.
  `explicit_where_named_constructor_field_sugar_lowers_to_positional_
  constructor` (`:58`) uses a non-indexed family (`PairBox` has zero
  indices), so field sugar was exercised without touching the indexed path;
  `git show --stat e026e721` confirms `elab.rs` (where coverage/motive logic
  lives) is untouched -- the diff is confined to `ast.rs`, `lib.rs`,
  `modules.rs`, `parser.rs`, `resolve.rs`.
- **AC5 (scope is bounded).** Same `git show --stat e026e721`: no
  `ken-kernel`, `Cargo.lock`, `packages`, `spec`, or `conformance` movement.
- **AC6 (durable naming hygiene).** Same sweep as the other three slices: no
  `SURF-gadt-*` leakage in non-test `src/*.rs`; `field_labels` and
  `UniverseArgument` are domain names, not WP labels.

## Not this node

No repair, no reframing. In particular, this node does not reopen whether
named-argument constructor *expressions*/*patterns* should exist --
`SURF-gadt-field-sugar`'s frame explicitly deferred that to its own D0 and it
was decided out of scope, and this tracker node's origin,
`[[LANG-GADT-SEQUENCE-TRACKER-GAP]]`, separately rules that question not
this-node's-to-reopen ("Not `SURF-gadt-field-sugar`'s scope question").
