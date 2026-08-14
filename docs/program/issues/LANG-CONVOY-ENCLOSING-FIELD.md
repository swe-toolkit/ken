---
id: LANG-CONVOY-ENCLOSING-FIELD
title: "spec 34 §3.2's Boundary paragraph names the two-vector `zip` recursive step a known gap and a follow-on -- the sibling-convoy re-typing cannot distinguish a genuine outer parameter from a field the enclosing match already bound, because `outer_scope_depth` is a raw context-depth subtraction that includes both -- and the follow-on was never filed"
status: draft
owner: language
size: unsized
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward sweep 2026-08-14 at main 96c95586, grepping spec/30-surface/ for deferral phrasing rather than auditing the tracker for gaps. This is the third obligation found that way -- the first two were spec 37's `filter` (produced LANG-PRELUDE-COLLECTIONS) and its `DecEq Char` transport (produced LANG-DECEQ-CHAR-LAWFUL-INSTANCES). Filed `draft` and unsized because the remedy is a design call, not because the gap is uncertain."
---

## What this is

`spec/30-surface/34-data-match.md §3.2`, the **Boundary** paragraph, states it
as normative text and calls it a follow-on:

> *"These re-typings compose through **one** level of nesting... A branch that
> both destructures a sibling through its **own** nested match *and* re-uses a
> field from the **enclosing** match's own destructuring in the same expression
> (e.g. a two-vector `zip`'s recursive step, which nests a match on the second
> vector while also passing the first vector's own tail into a recursive call)
> is a known gap -- the sibling-convoy re-typing does not yet distinguish a
> genuine outer *parameter* from a field the **enclosing** match already bound,
> and can substitute the wrong (though never unsound -- always kernel-proved)
> index there... the full two-vector `zip` recursive step is a follow-on."*

**A grep of `docs/program/issues/` finds no node for it** -- zero hits for
`convoy` or `two-vector` across the whole tracker.

## The mechanism, measured at `96c95586`

The spec says the re-typing "does not yet distinguish" the two. **The reason is
one line.** `elab.rs:2204`, in `check_match_dependent`:

```rust
let outer_scope_depth = cx.ctx.len() - n;
```

where `n` is the current branch's own field count. `install_index_refinements`
then runs capability 2 over `for abs_pos in 0..outer_scope_depth`
(`elab.rs:3000`), re-typing **every** binder in that range that mentions the
scrutinee's index.

⇒ **"Outer" is defined as raw context depth below this branch's fields.** In a
nested match, the enclosing match's bound fields sit exactly there, so they are
indistinguishable from genuine function parameters and are re-typed as though
they were. That is the spec's stated gap, and the discriminator it needs --
provenance for which context entries came from an enclosing match's
destructuring -- **does not exist in `cx.ctx` today.**

**Severity is completeness, not soundness, and the spec says so:** the wrong
substitution is "never unsound -- always kernel-proved". The failure surfaces
as a rejected program that should typecheck, not as a bad program admitted.

## Why this is `draft` and unsized

**The gap is certain; the remedy is a design call.** Making the distinction
requires threading provenance about context entries through
`check_match_dependent`, and the shape of that -- a parallel provenance vector,
a richer context entry, or a narrower `outer_scope_depth` computed from the
match's own entry depth -- is a component-design question. Per
`ken-steward §3` that is the Architect's, not the Steward's, and framing
deliverables now would produce a frame the ruling may invert.

**Filing it anyway is the point.** `34 §3.2` is normative surface spec carrying
a named gap with no tracker row; a sentence in a spec section is not
discoverable, and the two prior instances of this exact shape each sat unfiled
long enough that one of them had its stated blocking reason go false with
nobody noticing.

## What is already pinned, and must not be broken

`crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs` is
the boundary the spec calls "pinned, tested":
`sibling_convoy_retypes_outer_binder_through_nested_match` is the single-level
convoy, alongside the injectivity and goal-refinement cases and an AC8 that a
genuinely ill-typed program stays kernel-rejected. **Whatever shape the remedy
takes, those stay green** -- the single-level case is the behaviour the wider
one must generalize, not replace.

## Flip condition

**Flip to `ready` and frame it when the discriminator's shape is ruled.** The
question to route, in one sentence: *does the elaborator's local context need
per-entry provenance distinguishing an enclosing match's bound fields from
genuine outer binders, or can `outer_scope_depth` be computed from the match's
own context depth at entry instead?* The second is far cheaper if it is
sufficient, and whether it is sufficient is exactly the ruling.

**A failing two-vector `zip` fixture is the cheapest thing that would make this
concrete** and does not require the ruling -- if a later sweep wants one
increment of progress here before the design call, that is it.

## The pattern this instance confirms

Three obligations in `spec/30-surface/` have now been found by **grepping the
chapter for deferral phrasing** -- "tracked follow-on", "is a follow-on",
"deferred to a later", "not delivered here" -- and none by auditing the tracker.
**A tracker audit cannot see an obligation that was never entered into it.**

One sweep at `96c95586` also cleared a false positive worth recording so it is
not re-investigated: `33-declarations.md:751` defers the `export` declaration
and public re-export propagation build to "the named Language follow-on", but
that build has substantially landed -- `modules.rs` carries the export tables,
abstract export (`§4.2`) and the re-export identity rules, and `error.rs:612`
carries the re-export collision error. **That deferral is discharged, not a
gap.**
