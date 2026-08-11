# LANG-SURFACE-RECORD-DECL — the `record` declaration and `.field` on it

Owner: language. Size: M. Node: [[LANG-SURFACE-RECORD-DECL]].
Fixed inputs measured at `origin/main` = **`24933da4`**. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**
[[LANG-LEX-NUMERIC-FORMS]] lands on top of that measurement and touches only
numeric scanning in `lex_numeric`, so nothing below moves under it.

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if
your diff stays in `crates/`.

## What this deliverable is

`record Point { x : Int, y : Int }` parses, elaborates to a right-nested Σ, and
`p.x` type-checks against it. That is the whole cut.

**No new kernel term, and `trusted_base()` does not move.** `Term::Sigma`,
`Pair`, `Proj1`, `Proj2` already exist and already carry the definitional η
that `33 §2`'s field access depends on. You are adding a surface declaration
form for a thing the elaborator already builds.

## The design judgment, front-loaded

**`class` already does the elaboration you need.** `ast.rs:342` records that
`ClassDecl` elaborates to a record type — a Σ-chain whose kernel sort is
decided by `sort_sigma` (`check.rs:198`). Read `elab_class_decl` before you
write anything. **The question this node answers is not "how do I build a
Σ-chain from fields" — that code exists — but "what does a record register as,
so that projection finds it."**

**`p.x` already parses and resolves.** `parse_atom_expr` builds
`Expr::EProj(base, field, span)`, and `resolve.rs:1693` forwards the field name
to `RProj` with no class-specific handling. **Do not add parser work for field
access.** If you find yourself editing the projection parse loop, stop — the
adjacency guard there is `LANG-LEX-PROJECTION-ADJACENCY`'s and `p.1.2` must
keep working.

**The refusal is at one lookup.** `infer_proj` (`elab.rs:3490`) scans
`class_env.classes` for a `type_id` match and fails with *"projection base's
type is not a known class dictionary"*. Extending that lookup is the node.

> ### ONE LOOKUP, NOT TWO. This is a stop if you find you need two.
>
> `RProj` is a single elaboration node. Giving it two independent resolution
> branches — one for classes, one for records — is exactly where one branch
> silently captures the other's input, and the failure is a wrong answer rather
> than an error. **Whatever you choose in Deliverable 2, the field lookup stays
> single-entry.** If records genuinely cannot share the lookup, that is a
> design fork and it comes back to me.

**The `whnf` hazard is real and it is documented in the code.** `infer_proj`
deliberately inspects the base type **as elaborated, never `whnf`'d**, because
a class type is `Decl::Transparent` and unfolding it goes straight through into
the raw Σ-chain, losing the "which type is this" information the lookup needs.
**A record type has the same hazard.** If you make record types transparent,
you inherit it; the comment at `:3497-3504` tells you what breaks and why.

## Deliverables

**D1 — the declaration parses.** A `KwRecord` token and a `record Name { f :
T, … }` declaration form. `record` is already reserved (`31-lexical.md:41,525`)
and the lexer emits nothing for it, so this is a keyword that exists in the
spec and in no code.

**D2 — it elaborates to a right-nested Σ, and registers.** Model on
`elab_class_decl`. **State in one sentence which registry you chose and why** —
reusing `ClassInfo` (which already carries `field_names` and `field_types`)
makes the lookup generalize for free but puts non-classes in a structure named
for classes; a parallel registry keeps the concepts apart at the cost of a
two-source lookup. Either is defensible. **Check what else scans
`class_env.classes` before you reuse it** — if instance resolution iterates it,
a record in there is a candidate instance, and that is a wrong answer.

**D3 — `p.x` type-checks on a record value.** Through the extended lookup, with
the field's declared type.

**D4 — dependent fields.** A later field's type may mention an earlier field
(`33 §2`). Right-nested Σ gives this structurally; the deliverable is that the
declaration's scope handling admits it, or an explicit refusal with a span if
you determine it does not and say why.

## Acceptance criteria

**AC-1 — `record Point { x : Int, y : Int }` declares, and `p.x` and `p.y`
have the declared types.** Assert the elaborated **type**, not merely that
elaboration succeeded. A success assertion passes for a record whose fields all
came out `Int` by accident.

**AC-2 — the Σ shape is right-nested and the projection agrees with it.** For
a three-field record, `r.z` must reach the third component. **A two-field
record cannot distinguish right-nesting from left**, so the control needs three
fields; with two, both nestings give the same answer and the test passes either
way.

**AC-3 — a dependent field works.** `record Sized { n : Int, v : Vec n }` or
the nearest form the surface admits. If D4 comes out as a refusal instead,
control the refusal and its span.

**AC-4 — the class path is unchanged.** `class`/`instance` declarations still
elaborate, and `d.leq`-style dictionary projection still resolves. **This is
the AC that catches a two-branch lookup**: name an existing class test and show
it still passes with a record declared in the same program.

**AC-5 — an unknown field is refused with a span.** `p.nosuchfield` names the
field and the record type. Not a panic, not a silent `Proj1`.

**AC-6 — the projection seam is unchanged.** `p.1` and `p.2` positional
projection, `p.1.2`, and `p. 1.2` behave as `LANG-SURFACE-PAIR` and
`LANG-LEX-PROJECTION-ADJACENCY` left them.

**AC-7 — the A/B.** Disable the record declaration branch and show
`record Point { … }` fails to parse; restore and the AC-1 program elaborates.

**AC-8 — no `spec/` edit, no new kernel term, `trusted_base()` unchanged.**
If you find yourself adding a `Term` variant, stop — a record is a Σ and the
kernel already has one.

## Excluded scope

- **Record literals, punning, and functional update.** `{ x = 1, y = 2 }`,
  `{ x, y }`, `{ p | y = 3 }` are a sibling's. They open a brace fork this node
  does not need: braces already carry refinement types (`{ n : Int | n ≥ 0 }`)
  and class/instance bodies, and `31-lexical.md:194` names "record/refinement
  braces" together. **You do not need them to build a record value** — a record
  is a right-nested Σ and [[LANG-SURFACE-PAIR]] landed tuple introduction, so
  `(1, 2)` checked against `Point` constructs one. Use that in your controls.
- **Record patterns in `match`.** `34 §3`'s.
- **No instance-resolution change.** If reusing `class_env` would alter which
  instances resolve, that is the signal to use a parallel registry, not to
  adjust resolution.
- No implicit binders, no unification work. See the node's "Why this and not
  implicit binders" — the elaborator has no term metavariables, and that is a
  foundational node, not this one.

## Stop conditions — return to me, do not decide

- **The field lookup cannot stay single-entry.**
- **Records cannot share `class_env` and a parallel registry duplicates the
  Σ-building code** rather than reusing it. That would mean the two forms are
  less alike than this frame claims, which changes the node.
- **You need a new kernel term or a `trusted_base()` change.**
- **Dependent fields turn out to need the scope handling rebuilt** rather than
  admitted.

## Contention

`crates/ken-elaborator/` — `lexer.rs` (one keyword), `parser.rs`, `elab.rs`
(`infer_proj` and the class-decl neighbourhood), `classes.rs`. **Language's own
lane only**; Runtime is in `crates/ken-runtime/`. The lexer touch is a keyword
token and does not meet `lex_numeric`, so it does not contend with
[[LANG-LEX-NUMERIC-FORMS]] or [[LANG-LEX-HEX-FLOAT]]. **Re-derive the
intersection at candidate time** — a merge-base goes stale without your branch
moving.

## Sizing and validation

`scripts/ken-cargo test -p ken-elaborator` plus your focused suite.
**Never `--workspace`**; that is CI's gate. Read `elab_class_decl` and
`infer_proj` in full before estimating — if the class path turns out to be less
reusable than this frame claims, say so early rather than reimplementing it.
