---
id: LANG-SURFACE-RECORD-DECL
title: "`33 §2` specifies `record Point { x : Int, y : Int }` and `record` is already a reserved keyword, but the lexer emits no token for it and the parser has no declaration form -- while the elaboration target is complete and already exercised, since `class` elaborates to exactly the right-nested Sigma chain a record needs and `p.x` already parses and resolves, refusing only at `infer_proj` because that lookup scans the class registry"
status: active
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Steward measurement 2026-08-11, taken while re-deriving the Language lane's next node after LANG-LEX-NUMERIC-FORMS. The research surface-gap sweep at 8898c426 (evt_3dsd7j9t4r33a) ranked records among the next candidates; this node is that candidate re-measured at current main, and the sizing below supersedes the sweep's. The companion finding that killed the sweep's other candidate is recorded in "Why this and not implicit binders".
---

## The gap

`spec/30-surface/33-declarations.md §2` specifies records directly:

```
record Point { x : Int, y : Int }
record User  { name : String, age : { n : Int | n ≥ 0 } }   -- refined field
```

Measured at `origin/main`:

| layer | state |
|---|---|
| `31-lexical.md:41,525` | `record` is a **reserved keyword** |
| `lexer.rs` | **zero occurrences.** No `KwRecord`, no `"record"` string |
| `parser.rs` | no record declaration form |
| `elab.rs` | no record type, no record registry |

So `record Point { x : Int, y : Int }` is a parse error today, and the keyword
is reserved against a form nothing implements.

## Why this is a small node and not a large one

**The elaboration target already exists and is already exercised.** `33 §2`
says a record "elaborates to right-nested Σ with definitional η" — and that is
precisely what `class` already does. `ast.rs:342` describes `ClassDecl` as
elaborating "to a record type (Σ-chain) whose kernel sort determines property
vs structure via `sort_sigma`", and `check.rs:198` is that function.

⇒ **A record is the declaration form for a thing the elaborator already
builds.** The kernel side is complete: `Term::Sigma`, `Pair`, `Proj1`, `Proj2`
are all in `crates/ken-kernel/src/term.rs` and carry the definitional η the
spec's field access depends on.

**And the projection surface is already there, generically.** `p.x` parses
today — `parse_atom_expr` produces `Expr::EProj(base, field, span)` — and
`resolve.rs:1693` passes the field name straight through to `RProj` with no
class-specific handling at all.

## Where it actually refuses, and why that is the whole node

`infer_proj` (`elab.rs:3490`) is the one class-specific layer. It takes the
base's type **as elaborated**, extracts a `Const` id, and then:

```rust
let (field_names, field_types) = class_env
    .classes
    .values()
    .find(|ci| ci.type_id == class_type_id)
```

failing with *"projection base's type is not a known class dictionary"*.

⇒ **`p.x` fails at exactly one lookup, and that lookup wants exactly the
metadata a record declaration produces.** `ClassInfo` already carries
`field_names` and `field_types` — the two things `record Point { x, y }`
declares. The node's real work is making that lookup find records too.

**This is an extension, not a second projection path.** `RProj` is one node;
giving it two independent resolution branches is where one branch silently
captures the other's input. Whatever registry records land in, the field lookup
must remain single-entry.

## Why this and not implicit binders

The same research sweep ranked "ordinary implicit binders with call-site
insertion" alongside records. **Measured at current main, that candidate is not
an M and this frame records why, so it is not re-proposed at the wrong size.**

`MetaCtx` is `Vec<Option<Level>>` (`elab.rs:114`). **There are no term
metavariables in the elaborator at all.** `unify_types` (`:262`) is misnamed: it
walks two terms structurally and solves *level* metas as it goes, has no occurs
check, and its final arm is `_ => {}` — a mismatch is a silent no-op, not a
failure.

⇒ Implicit-argument insertion needs a term meta, a unifier that can fail, and an
occurs check, none of which exist. That is a foundational node, not a surface
one. **Do not size it from the presence of the word `unify`.**

## What is not yet known

- **Whether records register in `class_env` or a sibling registry.** Reusing
  `ClassInfo` makes the `infer_proj` lookup generalize for free but puts
  non-classes in a structure named for classes, which affects instance
  resolution if anything else scans it. A parallel registry keeps the concepts
  apart at the cost of a two-source lookup. **This is a real design choice and
  the frame should not pre-empt it** — but the single-entry constraint above
  binds either way.
- **Whether the record type is `Decl::Transparent` like a class type.**
  `infer_proj` deliberately does not `whnf` the base type because a class type
  unfolds straight through into the raw Σ-chain, losing the identity the lookup
  needs. A record has the same hazard and probably the same answer, but it has
  not been measured.
- **How dependent fields order.** `33 §2` permits a later field's type to
  mention an earlier one. Right-nested Σ gives this for free structurally; what
  is unmeasured is whether the declaration's scope handling already admits it.

## Not this node

**Record literals, punning, and functional update.** `33 §2` also specifies
`{ x = 1, y = 2 }`, `{ x, y }`, and `{ p | y = 3 }`. These are deferred to a
sibling because they turn on a grammar fork this node does not need to open:
braces already carry refinement types (`{ n : Int | n ≥ 0 }`) and class and
instance bodies, and `31-lexical.md:194` names "record/refinement braces"
together. **A record value is constructible in this node's scope without any of
it** — a record is a right-nested Σ, and `LANG-SURFACE-PAIR` landed tuple
introduction, so `(1, 2)` checked against `Point` builds one.

Record patterns in `match` are `34 §3`'s and are not this node's either.

## `S1a` accepted partial — the transitional read seam, MERGED 2026-08-11

Exact `0ae99a0067cdaf67fbf0676429da75460085908c` from declared base `17f68eb1`;
three commits, nine `crates/ken-elaborator` paths, `+88/-87`. Decision
`dec_2pe1cesy87ne4` resolved APPROVE — Architect `evt_5j10r2p5a21q0`, QA
`evt_66brv66dhegrm`. No `spec/` or `conformance/` path, so no Spec vote.

**What landed.** A public transitional read-only `ClassEnv::classes()` borrowing
the sole existing map, plus the migration of **every executable read** from the
`.classes` field to the accessor. The sole production writer and the
`init_class_env` construction stay direct; `pub classes` remains for this slice.
Storage- and semantic-neutral — no second store, no registry flip, no
instance-resolution or projection change, no records.

**This closes none of the node's record-declaration surface.** It is slice one of
a staged migration and buys exactly one thing: the aliasing point is now a
**function** rather than a field.

### Why that distinction was the whole problem, recorded because it cost three turns

**The Steward authorized an infeasible slice.** The instruction was *"`.classes`
becomes a thin alias over the registry, not a parallel store."* Measured:

```
crates/ken-elaborator/src/classes.rs:106:    pub classes: HashMap<String, ClassInfo>,
```

**It is a public struct field, and a field is storage.** Rust cannot make a
public field a view over a different backing store, so the instruction had only
two readings: materialise a second `HashMap` (a parallel store, forbidden by
`evt_4qvpthrt47e8z`), or convert every consumer at once — which is the entire
migration the slice existed to avoid.

**The implementer refused three times and was right three times.** The Steward
had told the leader that a third refusal would indicate the seat rather than the
scope. **That was wrong**, and the correction belongs here rather than only in a
thread: the seat was producing correct refusals against unbuildable
instructions.

⇒ **Before authorizing a slice, check the TYPE of the thing you are asking to
change.** A field is not a function, and no amount of staging makes a field
behave like one.

**The Architect's staged order** (`evt_49qybm9pasz24`) supplies what the Steward's
did not: `S1a` introduces the callable seam over the **existing** map keeping
`pub classes`; `S1b` closes mutation and construction, then privatizes; `S2..Sn`
introduce storage-independent views and migrate by call shape; the storage flip
comes last and changes no call site. **`classes()` is a deletion gate, not a
compatibility API** — its required doc comment says so, and a candidate omitting
that sentence was blocked (`dec_5cgh7x06txz16`, rejected) precisely so the broad
raw-map view cannot fossilize.

## `S1b` accepted partial — the storage is private, MERGED 2026-08-11

Exact `d073c9475bfb4c49242fe7b4fd0f02c3b6c24d02` from declared merge-base
`3be31f105ebf76f412a00dfb9a81e0f49e1e6aac`; two non-merge commits, exactly
`crates/ken-elaborator/src/classes.rs` and `crates/ken-elaborator/src/elab.rs`,
`+22/-16`. Decision `dec_7jxs6fj6hyf2s` resolved APPROVE — Architect
`evt_573a4w9fwgx8g`, QA `evt_5sg7smy0s1gt7`. No `spec/` or `conformance/` path,
so no Spec vote. `origin/main` is `61c034e2`.

**What landed.** Slice two of the Architect's staged order: a narrow
`register_class` owning the sole production insertion, a narrow `initialized`
owning the real registry construction, and the `classes` field made **private**.
The `S1a` read seam is unchanged — `classes()` remains the documented
transitional view. Storage- and semantic-neutral: no second map, no
`classes_mut`, no registry flip, no record syntax, no instance-resolution or
projection change.

**This closes none of the node's record-declaration surface either.** What it
buys is that the compiler now holds the boundary the previous slice could only
document.

### The one hunk that was blocked, and why the block was cheap

The Architect rejected `b00a29a7` for exactly two unformatted hunks and named
both. The ring returned a formatting-only recut, re-ran QA, and re-approved
inside seven minutes. **A block that names the hunk costs a ring minutes; a
block that names a concern costs it a turn.** This is the second time on this
node that the naming was what made the block cheap.

## `S2` accepted partial — the keyed class-view migration, MERGED 2026-08-11

**Exact `a87068beb0b9aaa3784e05253be8510c85d937a1`, PR #1918.** Decision
`dec_3w9j2qr75h21b` resolved: Architect `evt_3ptrqb4thb4s4`, QA
`evt_1r8he0akbwez0`. Two non-merge commits from declared merge-base `ee67040f`,
exactly 11 `crates/ken-elaborator/**` paths, `+260/-65`, clean `diff --check`.
M6 blob identity **11/11 MATCH**. Current-main path intersection empty.

**The node stays `active`** — `classes()` still exists, and its deletion is the
closure gate below, not this slice.

### What it moved, measured on both trees

Direct `.classes()` **call sites** across `crates/ken-elaborator` — comment
matches excluded, which is the correction below:

| tree | real call sites | in `src/` |
|---|---|---|
| base `ee67040f` | 29 | 9 |
| merged `a87068be` | **2** | **1** |

**27 of 29 consumers migrated** onto the borrowed exact-key views. Re-measured
on `main` after the merge, not carried from the handback.

> **CORRECTED. This first read "31 → 3, 28 of 31", and a bare `.classes()` grep
> is not a call-site census** — it matches the token inside comments too.
>
> At base, 31 raw matches were **29 calls plus 2 comments**
> (`tests/lc_acceptance.rs:178` and the `//!` block at
> `tests/seal2_support/mod.rs:12`). On `main`, 3 raw matches are **2 calls plus
> 1 comment** (that same `//!` line). The `src/` column was unaffected — no
> comment matches there — which is why the 9 → 1 figure stood while both totals
> were wrong.
>
> **The adversary caught the numerator and inherited the denominator**
> (`evt_5s0j5633597ah`): it corrected `3` to `2` by naming the doc comment, then
> stated the result as "31 → 2", carrying my uncontested base figure. So the
> corrected count was still wrong until the base was re-measured the same way.
> **A correction inherits every premise it does not itself re-derive.** That is
> the same defect the correction was fixing, one layer down and inside the fix.

### The two survivors, named — and one is not residual progress

| site | what it is |
|---|---|
| `src/elab.rs:3524` | the **sole production survivor**; `S3` migrates exactly this one via `infer_proj` |
| `tests/seal2_support/mod.rs:149` | `for (class_name, info) in class_env.classes().iter()` — **whole-map iteration** |

**The second is unmigratable by the keyed view, by construction.** You cannot
key into a map you are enumerating, so it is neither deferred nor missed — it
is outside what the replacement can express, and it will still be there after
the storage flip.

⇒ **This bears directly on the closure gate below.** `classes()` cannot be
deleted while a consumer needs whole-map iteration: either that consumer
changes, or the view grows an iteration form. **A residual count cannot show
this and an enumeration can**, which is why the census is now named rather than
counted (adversary, `evt_5s0j5633597ah`).

The Architect approved the borrowed exact-key view boundary, the complete
authorized keyed-reader migration, the residual census, and the pre-sentinel
three-axis controls.

### The first S2 candidate, and why it is recorded here

`1ce8b424` was released as an S2 handback claiming it had *"migrated production
keyed lookups"*. It was `classes.rs` `+6/-0`, with all 31 occurrences intact and
no `elab.rs`, `lib.rs`, or test path touched. The leader caught it by
**independently re-deriving the diff** rather than reading the claim, and it was
withdrawn unpublished. It is local-only and must never be merged or resumed; its
base `f8f8bfbc` is long superseded.

**The cause was not implementer judgment.** The seat was running two model tiers
below its configuration — `gpt-5.6-luna low` against a configured
`gpt-5.6-sol medium` — and produced that attempt in 84 seconds. It was re-seated
before this turn. **A downgraded seat does not report being out of its depth; it
reports success**, so the leader's independent re-derivation is the only thing
that caught it, and it caught it on the first pass.

⇒ **The `+6/-0` stub and this `+260/-65` candidate come from the same seat on
the same task.** That contrast is the evidence the re-seating was the fix, and
the reason the census above is stated as a measurement on both trees rather than
as a claim.

## `S3` accepted partial — the type-ID projection view, MERGED 2026-08-11

Exact `370b233d`, merge-base `548405c0`, `origin/main` now `10d5eda9`, PR #1923.
Three `ken-elaborator` paths, `+56/-7`: `src/classes.rs`, `src/elab.rs`,
`tests/class_field_purity.rs`. Blob identity 3/3 against the declared base.
Decision `dec_1j1b1pfjw1ezf`, QA `evt_35pk0afyybges`, Architect
`evt_3yrk4b6n9x5b3`.

A public `projection_by_type_id(GlobalId) -> Option<ProjectionView>` that scans
the sole private class registry and builds its result through the existing
private `ClassInfo::view` using the registry's actual stored key. It adds no
public `ClassInfo`, no raw or mutable map surface, no cloned telescope, and no
second index or cache. `infer_proj` now takes names, field types, and
substitution from that borrowed view.

**What was deliberately preserved is the interesting half.** `infer_proj` keeps
its pre-existing un-normalized base-type identity rule — it still inspects the
elaborated `base_ty` without `whnf` and accepts only the same `App(Const, head)`
or bare `Const` shapes — and both its span-bearing refusals, unknown-dictionary
`TypeMismatch` and unknown-field `UnresolvedCon`. Field position, earlier-field
self-projections, substitution order, and the separate positional-pair path are
unchanged. The only replacement is the raw-map scan and its cloned vectors.

QA forced the adapter predicate false, and separately redirected only
`infer_proj` to a foreign `GlobalId`; each unchanged control reddened at its own
boundary and was restored byte-for-byte. Two mutations at two distinct sites,
not one repeated.

### The last production raw-map reader is gone, and the residual is NOT progress

`src/elab.rs:3524`, the sole production survivor named in the `S2` record, is
the site this slice migrated. **What remains is entirely inside SEAL-2:** one
`//!` explanatory comment, and `tests/seal2_support/mod.rs:149` —
`for (class_name, info) in class_env.classes().iter()`.

⇒ **That last one is whole-map iteration, and a keyed exact-key view cannot
express it by construction** — you cannot key into a map you are enumerating.
So the count going to one understates the situation rather than overstating it:
this is not a deferred site awaiting its turn in the `S`-series. It is a shape
the new boundary does not cover, and it **gates the deletion of `classes()`**
recorded below. Closing it needs either an iteration form on the view or a
change to that consumer — a decision, not a migration.

**Read the count with that in mind.** The census across this node has gone
29 real call sites → 2 → 1, and every step of that sequence is real work; but
the remaining 1 is the one the migration mechanism was never able to reach, so
the trend line does not predict its closure.

## `S4` accepted partial — the raw accessor is deleted, MERGED 2026-08-11, PR #1930

Exact `411ca71f`, declared merge-base `8fd3a893`, `origin/main` now `413444ba`.
Four paths, `+51/-13`, **blob identity 4/4** enumerated from the declared base.
Decision `dec_1bdc50wespjd0` verified `resolved` at merge time by reading the
object, not by carrying the handoff's claim. Architect `evt_4hsqgm863bp6a`, QA
`evt_xn0d3xcwq591`. The Architect independently computed merge tree `c3ea648b`
and the publisher's landed tree was `c3ea648b`.

It delivered borrowed class-entry enumeration, migrated the isolated SEAL-2
whole-class-field census, deleted the raw `classes()` accessor with its callers
and comments, and committed a non-vacuous population control.

**Measured on `origin/main` after the merge:** `fn classes(` has **zero**
definitions and `.classes()` has **zero** callers anywhere in `ken-elaborator`.

### What closed here was the GATE, and the node's own deliverable is untouched

The `S3` record named `tests/seal2_support/mod.rs:149` as the single residual —
whole-map iteration that the keyed view could not serve — and said plainly that
it was not deferred progress, because the accessor could not be deleted while
any consumer needed whole-map iteration. `S4` is the slice that answered it, so
the census reads 29 → 2 → 1 → 0 and the deletion gate is discharged.

**That is a prerequisite falling, not the node advancing toward its objective.**
Nothing in `S1a`, `S1b`, `S2`, `S3` or `S4` has touched the record declaration
form. Measured on `main` at the merge: `KwRecord` has **zero** occurrences in
`crates/ken-elaborator/src/lexer.rs`, and so does the string `"record"`. The
node still delivers exactly what `## The gap` says it does, and none of it is
built.

⇒ **Node stays `active`. `LANG-SURFACE-RECORD-LITERAL` stays blocked**, and its
`depends_on` edge is correct as written.

**The reading hazard this creates is recorded in the section below**, because
discharging a gate makes its prose read as a finished node.

## TRACKED REMAINING WORK — delete `ClassEnv::classes()` before records land

> ### DISCHARGED by `S4`. This was a GATE, never the node's deliverable.
>
> `classes()` is deleted and its caller census is zero — see the `S4` section
> below. **Everything in this section is now a record of a closed obligation.**
>
> **Do not read its discharge as the node closing.** The sentence below says
> the node *does not close while the accessor exists*, which is a necessary
> condition and reads, once satisfied, exactly like a sufficient one. It is not.
> `LANG-SURFACE-RECORD-DECL` delivers the `record Point { x : Int, y : Int }`
> declaration form, and at the merged `S4` tree the lexer still has **zero**
> `KwRecord` and **zero** `"record"` occurrences. **The headline deliverable is
> unbuilt.** The gap it closes is described at `## The gap`, some 300 lines
> above this section, which is the distance that makes this misreadable.

Recorded as live
remaining work rather than only as prose, on an adversary finding
(`evt_cfzgvy0512yw`) that named the structural reason:

> **The deletion obligation was recorded only in the doc comment of the item
> being deleted.** When `classes()` goes, its comment goes with it — so that
> sentence can never be what *reminds* anyone. It is readable only by someone
> who has already found the accessor.

⇒ **The risk is fossilization, and fossilization is exactly the case where
nobody goes looking.** A broad raw-map accessor outlives its slice precisely
because no one is reading its doc comment. **A gate whose only statement lives at
the point of work does not fire**; this one has to sit where a node status check
sees it.

**Closure condition:** `classes()` is deleted, and its caller census is zero. The
Architect's order puts that after `S2..Sn` migrate consumers to the
storage-independent views (`class`, `projection_by_type_id`, `class_entries`) and
after the storage flip — **it is a deletion gate, not a compatibility API to
preserve.**

**The census that was owed is now taken, and it clears.** It was recorded as
outstanding because the adversary's pass covered `src/` only, leaving the
test/support tree unmeasured. Measured at `61c034e2` across the whole crate —
`src/` and all 150 files under `tests/` — the direct uses of the field are
**exactly two, both inside `classes.rs`**:

| site | what it is |
|---|---|
| `classes.rs:144` — `&self.classes` | the accessor's own body |
| `classes.rs:147` — `self.classes.insert(...)` | inside `register_class` |

Nothing else touches the field anywhere, and no `ClassEnv` struct literal
survives outside `classes.rs`. ⇒ **The field's deletion is blocked by nothing,
and after `S1b` the compiler is what holds that** rather than a census that
decays. The remaining consumers are all reads through `classes()`. That census
was **31 call sites — 9 in `src/elab.rs`, 22 across seven test files** when it
was taken, and migrating them to the storage-independent views is the gate's
real content.

> **UPDATED after `S2` merged (`a87068be`).** The original "31" was a raw grep
> that counted 2 comment matches as call sites; the real base was **29 calls**.
> **Measured on `main`: 2 call sites remain, 1 of them in `src/`** — `S2` moved
> 27 of 29. The figure above is retained as the original scope, **not** as
> current state.
>
> **The live residual is an enumeration, not a number**, and it is in the `S2`
> section: `src/elab.rs:3524` (sole production survivor, `S3` migrates it) and
> `tests/seal2_support/mod.rs:149` (whole-map iteration, **unmigratable by the
> keyed view**). The second is why a count misleads here — it is not residual
> progress, and the accessor cannot be deleted while a consumer needs
> whole-map iteration.

**Two doc comments still name the private field as the access path**, and both
sit in test files that can no longer compile such an expression:

- `crates/ken-elaborator/tests/adversary_seal2_repros.rs:169`
- `crates/ken-elaborator/tests/seal2_producer_closure.rs:172`

Both read `` `class_env.classes[C].field_types` ``. This is prose debt, not a
build defect — fold it into whichever `S2..Sn` slice touches those files rather
than spending a cut on it. `tests/seal2_support/mod.rs:12` was checked and is
**already correct** (it names `classes()`), so this is two sites, not three.
