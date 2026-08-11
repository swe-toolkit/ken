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
