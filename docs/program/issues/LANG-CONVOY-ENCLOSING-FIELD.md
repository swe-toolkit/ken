---
id: LANG-CONVOY-ENCLOSING-FIELD
title: "spec 34 §3.2's Boundary paragraph names the two-vector `zip` recursive step a known gap and a follow-on -- the sibling-convoy re-typing cannot distinguish a genuine outer parameter from a field the enclosing match already bound, because `outer_scope_depth` is a raw context-depth subtraction that includes both -- and the follow-on was never filed"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: [LANG-CONVOY-MATCH-FIELD-PROVENANCE]
github: null
origin: "Steward sweep 2026-08-14 at main 96c95586, grepping spec/30-surface/ for deferral phrasing rather than auditing the tracker for gaps. This is the third obligation found that way -- the first two were spec 37's `filter` (produced LANG-PRELUDE-COLLECTIONS) and its `DecEq Char` transport (produced LANG-DECEQ-CHAR-LAWFUL-INSTANCES). Filed `draft` and unsized because the remedy is a design call, not because the gap is uncertain. FLIPPED ready and narrowed to the discriminating fixture ALONE on the Architect ruling evt_1rk8wyak0z7sr (2026-08-14, grounded at main c932e7b4), which refused both options the Steward offered, surfaced a third candidate whose carrier already exists, and authorized cutting the fixture with no ruling needed because it discriminates between remedies that prescribe different fixes."
---

> # MERGED 2026-08-15 as squash `00efd1f41`, from base `6275bbc35`.
>
> Candidate `db399d12d`, PR #2268. One commit, sole path
> `crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs`,
> `+35/-0`. QA `evt_7x34pndbg6kjv`, Architect `evt_1vyfpyv0t6dgj`, Decision
> `dec_4vw75zd71kj2g`.
>
> **`AC-3` — no remedy implemented — is verified on `main`, not merely on the
> candidate:** `elab.rs` is blob `dc1797a5a` at the declared base and blob
> `dc1797a5a` on `main` after the merge.
>
> **The verdict is the RANGE hypothesis.** `bottom_pos=5`,
> `already_present=false`, enclosing entry depth `3`, so `5 >= 3` — absent plus
> above. The Architect confirmed it at the loop: `outer_scope_depth =
> cx.ctx.len() - n` (`elab.rs:2217`) subtracts only *this* match's fields, so
> the enclosing match's fields sit inside capability 2's candidate range and
> are treated as genuine outer binders.
>
> **The third outcome — the one that would have refuted the Architect — did not
> occur, and it was reachable.** The frame required it to be sayable and the
> ring did not have to say it.
>
> **Residual, carried not hidden: the fixture's assertion is weaker than the
> measurement that produced it.** It accepts any kernel `TypeMismatch` anywhere
> in the program, while the discriminating fact — same head, differing only in
> the de Bruijn index — was measured, independently re-derived by QA, and then
> not pinned. Non-blocking here because the node is fixture-only by `AC-3` and
> the operands are pinned in two handbacks. **Tightening it is `D1` of
> [[LANG-CONVOY-MATCH-FIELD-PROVENANCE]]**, deliberately ordered before the
> remedy so that "the gap closed" can be told apart from "the program moved
> onto a different error path".

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

## THE RULING: NEITHER OPTION I OFFERED. And the fixture comes first.

**Architect `evt_1rk8wyak0z7sr`, grounded by reading `elab.rs` at `main`
`c932e7b4`.** He refused both halves of the question and named a third
candidate. **This node is now the fixture alone; the remedy is not yet ruled.**

**The entry-depth option is insufficient, and the reason is worth carrying.**
*"Computed from the match's own context depth at entry"* does not help, because
**when the INNER match is entered, the enclosing match's bound fields are
already in `cx.ctx`** -- its entry depth *includes* them. Excluding them needs
the depth at the **enclosing** match's entry, threaded down. That is a different
quantity and nothing currently carries it.

**Depth is not disqualified, though, and the pinned test says why in its own
words.** `firstIsSecond`'s retyped binder is described in the test as *"an
outer, independently-bound function parameter, never destructured by the outer
match"* -- so it sits **below** any enclosing-match floor, while the `zip`
offender is an enclosing match's field, **above** it. A threaded floor separates
exactly those two. **But a floor is coarse**: it excludes `let`s and every other
binder introduced between the two matches, which are eligible today. ⇒ **It
trades a wrong-index substitution for a possible new incompleteness -- the same
failure class this node exists to remove.**

### The third candidate, whose carrier already exists

**Capability 2's selection is already content-keyed, not position-keyed.**
`try_reindex_cast` returns `Option` and fires only when the binder's type
actually mentions `b2`; `outer_scope_depth` merely bounds the *candidates*. ⇒
**The defect is not that the range is wide -- it is that mentioning `b2` is true
without entailing that this branch's equation licenses re-indexing here.**

And the per-entry fact is already recorded:

- `cx.var_refinements: HashMap<usize, (Term, Term, usize)>` is keyed by
  `bottom_pos`, and `bottom_pos = cx.ctx.len() - 1 - outer_idx` where
  `outer_idx = cx.ctx.len() - 1 - abs_pos` ⇒ **`bottom_pos == abs_pos`, an
  absolute index from the bottom of the context, stable across nesting depth.**
- ⇒ an enclosing match's refinement for a binder and an inner match's refinement
  for that same binder **land on the same key**.
- **Capability 2 (`:3000`-`:3027`) never consults `var_refinements` before
  inserting.** It `insert`s, so the inner refinement **silently overwrites** the
  enclosing one.

**That is a per-entry discriminator needing no new provenance field:** *has an
enclosing match already refined this entry, and may I replace or must I
compose?*

## THIS NODE IS THE FIXTURE. THE REMEDY IS NOT RULED.

**The Architect was explicit about the bound on his own reading:** he
established that `bottom_pos` is absolute and collides across nesting levels,
that capability 2 does not check before inserting, and therefore that an inner
match *can* overwrite. **He has NOT established that this overwrite produces the
`zip` failure**, and declined to price a remedy from a mechanism found by
reading -- *"I did that in this arc already and it cost a turn."*

## Deliverables

**`D1` -- a failing two-vector `zip` recursive-step fixture.** The smallest
program that both destructures a sibling through its own nested match and
re-uses a field the enclosing match bound, in the same expression. **It must
fail today**, and the failure must be the wrong-index substitution the spec
describes, not an unrelated rejection.

**`D2` -- instrumentation that discriminates, reporting the LEVEL of each
operand.** When the wrong index arrives, report for the binder that received it:

1. its `bottom_pos`;
2. whether `cx.var_refinements` **already held an entry at that `bottom_pos`**,
   installed by the **enclosing** match, at the moment the **inner** match's
   capability 2 inserted;
3. whether that binder is **below** the enclosing match's entry depth (a genuine
   outer binder) or **above** it (an enclosing-match field).

**`D3` -- report the raw values and the verdict separately.** Not "the overwrite
hypothesis holds" -- the three measurements, then what you read from them. A
verdict without its operands is what cost this program a turn on the `D2k`
probe.

## Acceptance criteria

**`AC-1` -- the fixture fails at this base, with the failure text quoted
verbatim.** A fixture that passes means the gap is not where the spec says it
is; **that is the finding** and it stops here.

**`AC-2` -- all three measurements in `D2` are reported, with raw values.**

**`AC-3` -- no remedy is implemented.** No floor, no provenance field, no
compose-or-skip. **The Architect rules the remedy on this output.** A candidate
containing a fix has exceeded the node.

**`AC-4` -- instrumentation is removed before handback**, or is `#[cfg(test)]`
and named as such. The fixture stays; the probe does not.

**`AC-5` -- no-regression, in CI.** `COORDINATION §12`; build and test targeted,
`-p ken-elaborator`. **The pinned acceptance suite below stays green** -- if the
new fixture cannot be added without reddening it, stop and report.

## What the three outcomes mean, including the one that refutes the Architect

**Already-present at (2)** ⇒ the **overwrite** hypothesis; the fix is a
per-entry compose-or-skip on a carrier that already exists -- no provenance
field, no threading.

**Absent at (2), above the floor at (3)** ⇒ the **range** hypothesis; the fix is
the threaded floor, with the coarseness accepted and its residual stated.

**Absent at (2), below the floor at (3)** ⇒ **both hypotheses are wrong and the
ruling reopens on new evidence.** The Architect named this outcome himself and
called it legitimate. **It must be sayable** -- report it plainly and stop; do
not reach for a third mechanism.

## What is already pinned, and must not be broken

`crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs` is
the boundary the spec calls "pinned, tested":
`sibling_convoy_retypes_outer_binder_through_nested_match` is the single-level
convoy, alongside the injectivity and goal-refinement cases and an AC8 that a
genuinely ill-typed program stays kernel-rejected. **Whatever shape the remedy
takes, those stay green** -- the single-level case is the behaviour the wider
one must generalize, not replace.

## Sizing

**`S`.** One fixture and three measurements. **If the fixture cannot be made to
fail, that is the hard stop and it is a good outcome** -- it means the spec's
stated gap is not reachable from the surface as it stands, which is a bigger
finding than the remedy would have been.

## The flip condition is DISCHARGED -- recorded so it is not re-asked

It read: *"flip when the discriminator's shape is ruled"*, and offered the
provenance-versus-entry-depth fork. **The ruling refused the fork.** The
sentence *"a failing two-vector `zip` fixture is the cheapest thing that would
make this concrete, and does not require the ruling"* was written as a fallback
and turned out to be the answer -- the Architect adopted it in terms.

**The transferable form: when you route a design question, offer the cheap
increment that needs no ruling alongside it.** Here it converted a blocked node
into a released one in a single exchange.

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
