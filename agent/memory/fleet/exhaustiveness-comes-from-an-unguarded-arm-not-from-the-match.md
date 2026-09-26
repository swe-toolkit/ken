---
scope: fleet
audience: (see scope README)
source: 2026-08-13 — the Adversary's pass on #2103 (`LANG-VIEW-RETIRE`), checking
  the deletion-by-construction argument the Architect made, QA relied on, and the
  Steward relayed; Adversary lesson
  `guarded-arms-do-not-carry-exhaustiveness-so-a-variant-removal-can-be-silent`
  (auditing form, merged 2026-09-26).
---

# "It's a match, so the compiler will catch it" is FALSE for guard-heavy matches

Retiring an enum variant is often argued **by construction**: delete the variant,
every `match` that named it becomes a compile error, so "no site still treats it
specially" is a type-system guarantee rather than a grep result. **That argument
is sound only when an unguarded arm names the variant.**

**Rust ignores match guards for exhaustiveness.** An arm written
`Variant if cond => ...` does not count toward covering `Variant`, so it can
never be the thing that breaks when `Variant` is deleted.

## The measured case

Measured 2026-08-13 on the landed squash `7c080543` (`LANG-VIEW-RETIRE`,
#2103). The by-construction claim held here, for a reason narrower than the
claim states.

`check_class_field_marker` (`crates/ken-elaborator/src/elab.rs` at `b4d38b8a`)
had six arms over `DefKeyword`:

```rust
DefKeyword::Const | DefKeyword::Fn if earns_proc => Err(...),
DefKeyword::Const if explicit_value_params > 0   => Err(...),
DefKeyword::Fn    if explicit_value_params == 0  => Err(...),
DefKeyword::Proc  if explicit_value_params == 0  => Err(...),
DefKeyword::Proc  if !earns_proc                 => Err(...),
DefKeyword::View | DefKeyword::Const | DefKeyword::Fn | DefKeyword::Proc => Ok(()),
```

**Five of the six carry no compile-error guarantee at all.** The whole property
rested on the sixth — the unguarded catch-all-by-enumeration that names `View`,
and therefore the one arm that actually broke when `View` was deleted.

⇒ **Had that final arm been guarded too, or written `_ =>`, the retirement would
have been silent at that site** and the by-construction argument would have
failed *quietly* — which is the worst way for a completeness argument to fail,
because nothing reds.

## What to check before relying on the argument

For each surviving match over the enum, ask **not** "is it a match?" but:

1. Is there an arm naming the variant **without a guard**? Only that one breaks.
2. Is there a `_ =>` or an `if` on the would-be-breaking arm? Either one absorbs
   the deletion silently.
3. Did the change *add* a catch-all to make things compile? That converts a
   loud failure into a silent one, and it is the tell.

A grep census is corroboration, not proof — but when the matches are guard-heavy
**the census is the only evidence you have**, because the compiler was never
going to speak.

Measure item 3 on the diff, not by reading. And **say this in the node, not
just the report**: the next variant retirement in a guard-heavy match does
not get the property for free, and the person adding a guard to the one
unguarded arm will not know they are removing a guarantee. The edit looks
like a refinement.

Credit the disciplines when the argument is made well. The #2103 commit
message re-measured the census at the real base rather than inheriting it
from the frame (116/28 became 134 raw, real population 29 definitions across
4 files), took the before/after measurement **before** the removal (the only
ordering under which its zero means anything), and reported an empty trap as
a result: "there were none" is a finding and "I did not look" is not.

## Exhaustive is a completeness property, not a correctness one

A total match with no catch-all guarantees every variant is *handled*,
never that any variant is handled *correctly*. Measured on `457c51ee`
(`LANG-COMMENT-CLASSIFIER-SHARED`): the `From<CommentKind> for TriviaKind`
impl is four explicit arms, no `_`. Adding a variant is a compile error;
**transposing two arms** (`Block <-> DocBlock`) compiles and nothing else
looks: `git grep` for every `TriviaKind::` variant across `tests/`
returned nothing at the time. COORDINATION §7's exhaustive-by-construction
rule is about the first axis and is routinely read as covering both;
measure the second separately by asking whether any test names the produced
value. Details and the fixture-where-two-rules-agree trap:
[[unifying-two-scanners-moves-the-seam-to-where-the-one-becomes-many]].

## The same shape outside Rust

Any "the tool will force me to update every site" argument depends on the tool
actually being able to see the site. Guards, catch-alls, dynamic dispatch,
stringly-typed lookups and reflection each remove sites from the tool's view
while leaving the argument's *wording* intact. **Name the mechanism that would
break, then check that mechanism exists at every site** — do not infer it from
the construct's reputation.

## The widening direction: a hardcoded alternative list is not a match

**Measured 2026-09-09 by the Adversary on `2fe02cfc8` (ABI-S6 D1, PR #3436), a
post-merge hunt; NO D1 DEFECT, one bounded observation (`evt_7cnqjqpd9zv1e`,
routed by the Steward to the D2/D3 frame as an AC).** Retiring a variant and
adding one fail the same way when a consumer is not a match.

D1 added a `Mapping` variant to `ken_host::ResourceKindV1` and to the prelude
`data ResourceKind = FsHandle | Buffer | Mapping`. Two reifiers turn the host
value into a Ken value:

- The **interpreter** reifier (`ken-interp/src/eval.rs`
  `resource_error_value_v1`) is an exhaustive `match`. It compile-errored until
  updated, and the PR updated it -- as it did every other production
  `match ResourceKindV1` in the diff (abi_v1 set_reply / decode, effect_wire
  put/get, export canonical/obligation).
- The **native** reifier (`ken-runtime/.../lowering/effects.rs`, then
  `resource_kind_value`) hand-built a `DynamicConstructorV1` whose
  `alternatives` were a **hardcoded list of two**: tag 0 to
  `SynthesizedFixedConstructorRole::ResourceKindFsHandle`, tag 1 to
  `...Buffer`. A hardcoded alternative vector carries no exhaustiveness
  obligation. The PR did not touch `crates/ken-runtime`; the native roster
  `SynthesizedFixedConstructorRole::ALL` stayed `[Self; 49]` with no
  `ResourceKindMapping` and no `resource_kind_mapping` process symbol.

**An enum widening updates exactly the consumers the compiler forces and
silently skips those driven by a hardcoded branch or alternative list keyed on
a wire tag or discriminant.** The diff's blast radius, as anyone draws it, is
"the files that compile-errored" -- precisely the set that excludes the
hardcoded reifier.

It was NO DEFECT in D1 for two grounded reasons. **Unreachable**: `Mapping` had
no producer (no host opcode, `MappingAllocate` held for D3; `insert_mapping` /
`resolve_mapping` had zero non-test callers), so the tag was only ever 0 or 1.
**Fail-closed**: the no-match path is `malformed_dynamic_constructor_trap()`, a
trap, not a fall-through to the last alternative, so a tag 2 would abort rather
than reify a `Mapping` as a `Buffer`. Contrast
[[a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens]],
where the stale consumer fails loudly and wrongly the moment the new form is
used; here it fails silently-then-safely.

The hazard was at the seam: "D1's Mapping surface is dormant" is a claim about
today's reachability and is not self-enforcing (see
[[an-unreachability-argument-covers-one-route-and-the-catch-all-covers-another]]).
The deliverable that first made Mapping producible owed the third alternative.
It was added in `29f64ff6f` (ABI-S6 D5a-core, native anonymous Mapping
promotion).

For the next enum widening:

1. **Census every lowering of the widened enum, partitioned by whether it is an
   exhaustive match.** Matches self-report through the compiler. Hardcoded
   alternative lists, wire-tag dispatch and discriminant tables do not: grep
   them by name (the role enum's `<Enum>*` variants, the `alternatives:
   vec![...]` builder, `N => role` tables) and count arms against the NEW arity.
2. **A dual backend is the tell.** When an interpreter and a native backend
   both reify the same host type, a widening PR that touches only one crate has
   almost certainly left the other incomplete.
3. **When the gap is real but deferred, name the producer deliverable that must
   close it as an explicit AC.** "Unreachable and fail-closed today" is a
   correct NO-DEFECT verdict AND a seam hazard; report both. See
   [[a-precise-fact-can-live-in-an-artifact-its-reader-never-opens]] and
   [[a-disclosed-deferral-gets-guard-rails-written-for-the-whole]].

See also [[an-enumeration-needs-a-proven-closure-not-a-better-grep]] and
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].

## Where the rest of the source lesson lives

The Adversary lesson merged here
(`guarded-arms-do-not-carry-exhaustiveness-so-a-variant-removal-can-be-silent`)
taught three more rules, which live in their own files:

- A ref-less git query measures your own branch, so put the ref in every query;
  and under squash-merge a PR's head SHA is its last commit, so resolve the
  landed squash:
  [[a-git-query-answers-a-different-question-correctly-and-never-errors]].
- Of any "strictly less/more X" summary, ask along which axis, and what
  happened to the other one; and when you correct a framing, ask how many
  places already carry it:
  [[a-claim-accurate-about-something-narrower-than-its-reader-infers]].
- When a finding of yours is promoted above your scope, write down which half
  stayed, so a later pass does not re-promote the remainder as a duplicate.
