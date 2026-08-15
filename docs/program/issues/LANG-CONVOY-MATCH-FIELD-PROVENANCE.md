---
id: LANG-CONVOY-MATCH-FIELD-PROVENANCE
title: "capability 2's sibling-convoy candidate range is a positional proxy for a provenance property -- carry the match-field regions explicitly on the elaboration context and skip them, closing spec 34 section 3.2's Boundary gap without the incompleteness a positional floor would introduce"
status: merged
owner: language
size: S
gate: none
depends_on: [LANG-CONVOY-ENCLOSING-FIELD]
blocks: [LANG-INTERVENING-LET-FRAME-WEAKENING]
github: null
origin: "Architect remedy ruling evt_1vyfpyv0t6dgj (2026-08-15), delivered with the approval of LANG-CONVOY-ENCLOSING-FIELD's measurement candidate db399d12d and attached to Decision dec_4vw75zd71kj2g. The predecessor measured the RANGE hypothesis with raw operands; this node implements the remedy the Architect ruled from that measurement. He named the spelling, refused the positional floor, and withheld composition. Steward-filed per COORDINATION section 2."
---

> # THE MECHANISM IS RULED. DO NOT RE-DERIVE IT, AND DO NOT SUBSTITUTE A FLOOR.
>
> The predecessor node measured the defect and the Architect confirmed it at the
> loop. **The remedy below is his ruling, not a proposal** — the region stack,
> the push/pop siting, and the skip-rather-than-compose choice are all fixed
> inputs. What is left is building it and proving it with controls that a wrong
> implementation would fail.
>
> **The one thing he explicitly did not measure is `D2`**, and he said so
> plainly. That is the deliverable most likely to produce a finding.

## Treat every anchor here as perishable

If a fixed input below turns out false against the landed code, **say so and
escalate — do not quietly build around it.** Line numbers are measured at
`e6d2716cf`; `crates/` has not changed since `6275bbc35`, so they should hold,
but re-measure at your own base and report what you actually find.

## The defect, confirmed at the loop

`elab.rs:2217`, in the arm elaborator:

```rust
let outer_scope_depth = cx.ctx.len() - n;
```

`n` is **this** match's own field count, so the subtraction removes only this
match's fields. Capability 2 (`install_index_refinements`, `:2931`; the guard
at `:3007`, the loop at `:3021`) then ranges over `0..outer_scope_depth` and
treats every binder in it as *"an OUTER (pre-existing) binder"*.

**An enclosing match's bound fields sit exactly in that range**, and they are
neither outer nor pre-existing. Refining one re-points it at the inner match's
peeled index while a sibling reference in the same recursive call keeps the
enclosing one — the same head, one de Bruijn index apart, which is precisely
the measured `expected ((Dg574 Dg67) @9) / found ((Dg574 Dg67) @4)`.

The predecessor's operands, independently re-derived by QA: `bottom_pos=5`,
`already_present=false`, enclosing entry depth `3`, so `5 >= 3`. **Absent plus
above selects RANGE and rules out overwrite.**

## Why NOT a positional floor — this is the ruling's load-bearing half

A floor at the enclosing match's entry depth encodes the right property **only
while the enclosing match's fields are the topmost thing below the inner
match.** They need not be.

`RLam` (`elab.rs:1128`) and `RLet` (`:1141`) both push onto `cx.ctx` and
elaborate the body inside that push:

```rust
RExpr::RLet(_name, ty_opt, rhs, body, span) => {
    let (rhs_core, rhs_ty) = prepare_let_rhs(cx, ty_opt, rhs, span)?;
    cx.ctx.push(rhs_ty.clone());          // elab.rs:1144
    let body_result = check(cx, body, &weaken(expected, 1), span);
```

So `match v { VCons m a xs => let k = … in match w { … } }` yields
`[outer | m,a,xs | k | inner fields]`. **`k` is a genuine outer binder sitting
ABOVE the enclosing boundary**, and a floor silently makes it ineligible for
sibling convoy.

⇒ **A floor trades a wrong-index substitution for a new incompleteness — the
same failure class this work exists to remove.** It is the shape
[[LANG-FOREIGN-CTOR-ARM-REJECT]] already hit once: a predicate that tests
**position** where the property is **membership**.

**The binding requirement is that the predicate be provenance, not position.**
The region stack is the authorized spelling of that. If you find that `Context`
already threads per-entry metadata, a per-slot provenance tag is equally
correct — **report that before switching**, since it changes a kernel-adjacent
type and the region stack deliberately does not.

## The authorized shape

Beside `var_refinements` on the elaboration context (`elab.rs:346`):

```rust
/// Bottom-relative `[start, end)` ranges of `cx.ctx` positions bound as
/// constructor fields by a match arm currently under elaboration,
/// innermost last. Capability 2 (sibling convoy) must skip these: a field
/// bound by an ENCLOSING match is not a genuine outer binder, and refining
/// it re-points it at the inner match's peeled index while its sibling
/// references keep the enclosing one.
match_field_regions: Vec<std::ops::Range<usize>>,
```

In the arm elaborator, pushed once the `n` fields are installed and popped with
the existing cleanup at `:2319`:

```rust
let field_region = (cx.ctx.len() - n)..cx.ctx.len();
cx.match_field_regions.push(field_region);
// … install_index_refinements, check the body …
cx.match_field_regions.pop();
```

and as the first statement of capability 2's loop body (`:3021`):

```rust
for abs_pos in 0..outer_scope_depth {
    if cx.match_field_regions.iter().any(|r| r.contains(&abs_pos)) {
        continue;
    }
```

**Pushing before the `install_index_refinements` call is safe and the reason
was checked rather than assumed:** the current match's own region is
`outer_scope_depth..cx.ctx.len()`, disjoint from the loop's
`0..outer_scope_depth`, so its presence cannot affect its own installation.

## Deliverables

**`D1` — tighten the landed `zip` fixture's assertion FIRST, while it is still
red.** It currently accepts any kernel `TypeMismatch` anywhere in the program.
Pin the discriminating fact that was measured and then not asserted: the two
sides are **the same head differing only in the de Bruijn index**.

> **Do this before the remedy, not after.** Without it you cannot tell *"the
> remedy closed the gap"* from *"the remedy perturbed the program onto a
> different error path"*. A regression that broke the `VNil` arm instead would
> keep the loose assertion green while reading as "the known gap is unchanged."

### `D1` HAS TWO HALVES, AND THE SECOND ONE CANNOT BE DONE THE OBVIOUS WAY

**Adversary hunt on `00efd1f41` (`evt_1e17vjqkdsg13`), triaged and folded in
here 2026-08-15.** The finding is independent of the residual above, and it
lands on this deliverable specifically.

**The fixture's doc comment says the recursive call's `xs` argument fails the
`TypeMismatch`. Nothing observed supports that.** The error's span is
`Span { start: 0, end: 167 }`, and the source string is **167 characters** —
measured, not inferred. ⇒ **The span covers the entire declaration.** It names
no argument, no sub-expression, and no position.

⇒ **The `xs` localisation is an authoring analysis, not an observation.** It is
unpinned by the assertion *and* unrepresented in the error itself.

**So the discriminating fact has two halves — WHICH ERROR SHAPE, and WHICH
OPERAND — and pinning the first does not give you the second.** Tightening the
`matches!` to *"same head, differing only in the index"* would still not
establish that the mismatch is on `xs` rather than on `ys`, on `m`, or on the
result type.

**Why this matters at exactly this node:** if the remedy lands and the fixture
goes green, *"the gap closed"* and *"the mismatch moved to a different
operand"* remain indistinguishable. **That is the same question `D1`'s ordering
exists to answer, one level down.**

**What to do:** pin the error shape as above, and for the operand half either
obtain a narrower span from the kernel or make an elaborator-side observation
of which binder was re-typed. **If neither is available at acceptable cost, say
so and state the limit explicitly** — an unpinnable operand is a legitimate
outcome, but it must be recorded rather than left implied by a green test.

**Two smaller carries from the same hunt:**

- **The fixture is a conjunction, not a single property.** The inner
  `match w { VCons _ b ys => … }` has one arm against `Vec`'s two constructors
  and elaborates only because `VNil` is index-impossible at `Vec Nat (S m)`
  (`34 §4.3`). ⇒ **It reaches the convoy gap only while index-impossibility
  holds.** The direction is safe — a regression there yields
  `ExhaustivenessError` and reds the assertion — but **"which property moved?"
  has two candidates and only one is currently named.** Name both.
- **The measured indices exist only in a notification.** `@9` and `@4` are in
  the handbacks; the doc comment says only *"differing only in the de Bruijn
  index"* with no numbers, and the test records neither. **Carry them into the
  fixture's doc at zero cost** rather than re-measuring them.

**Confirmed green for the right reason today, which the residual did not
establish:** the Adversary printed the live error and it is byte-for-byte the
one reported, so the committed fixture has not drifted and the loose `matches!`
is not currently being satisfied by some other mismatch. *"The assertion cannot
distinguish"* and *"the assertion is currently satisfied by the wrong thing"*
are different claims; only the first was carried, and the second is now
measured false.

**`D2` — the `let`-interleaved discriminator fixture. THE ARCHITECT DID NOT
BUILD THIS AND SAID SO.** A `zip`-shaped program with a `let` between the outer
match arm and the inner match, so a genuine outer binder sits above the
enclosing field region.

He established from `:1143`/`:1132` that the interleaving is **expressible and
reaches this path**; he did **not** confirm it elaborates far enough to reach
capability 2. **Measure that.**

> **This is the only fixture that separates the ruled remedy from a floor — a
> floor passes every other test now in the tree.** If it turns out the shape
> cannot reach capability 2, **report that and say what you measured**; it does
> not block `D3`, but it does mean the region set is unwitnessed and the node
> must say so rather than implying coverage it does not have.

**`D3` — implement the region stack exactly as spelled above.**

**`D4` — flip both fixtures to positive assertions** per `AC-1`.

## Acceptance criteria

> # MERGED AT `f08388396`. `AC-1`'s HAZARD CLAUSE IS **WITHDRAWN, NOT SATISFIED**.
>
> Exact `dac4d16af7584b68adbcb0ed45109dbd146cf3ba`, declared base `43bd0d597`,
> three paths, `+284/-21`, PR #2279. Decision `dec_63bdyk827f5gf` `resolved`;
> QA `evt_307nnw77csbmc`, Architect `evt_5b3c38r3xrqm6`.
>
> **`AC-1`'s hazard-discriminating pair claim is withdrawn**, and `AC-3`'s
> control is what was measured inert. QA replaced capability 2's
> membership guard with the
> prohibited positional floor `if abs_pos >= 3` and ran the **whole**
> `ds5b_dependent_match_refinement_acceptance` file: exit 0, 7 passed / 1
> ignored — **identical to the region-guard run.** The file-wide discriminator
> population is empty. The implementer reproduced both directions independently.
>
> **Two claims, and only one is unwitnessed. Do not collapse them:**
>
> | claim | status |
> |---|---|
> | the remedy was **necessary** | **witnessed** — `D1`'s pre-remedy red, same head, differing only `@9` vs `@4` |
> | the **region set beats a positional floor** | **behaviourally unwitnessed.** Both repairs fix the witnessed bug; they diverge only on a genuine outer binder pushed above an enclosing arm's field region, and no program in the suite reaches that |
>
> ⇒ **This node proves the fix. It does not prove the choice of fix.** The region
> set stays on provenance grounds — a field bound by an enclosing match is not a
> genuine outer binder, which is a statement about what the region set *means* —
> and that is design-justified, not measured.
>
> **The bounded attempt at a discriminating witness was authorized to fail and
> did.** It reached `install_index_refinements` with a fresh `let k : Vec Nat n`
> and died in `refine_branch_goal`. **That is
> [[LANG-INTERVENING-LET-FRAME-WEAKENING]], and its `D1` is a regression check on
> this node** — the Architect approved with that measurement outstanding.
>
> **Why the failure of an AC did not block the merge:** the shipped code is
> byte-identical to a head already reviewed sound, the suite is green with no
> behaviour changed, and **this node's own banned scope forbids the repair the
> block would demand.** Blocking would have spun the ring against its own
> guardrail.

**`AC-1` — the `zip` fixture elaborates AND evaluates to the correct result.**
**Control:** assert on the evaluated value, not on the absence of an error.

> **"The `TypeMismatch` is gone" FAILS this criterion.** An over-wide skip also
> makes the error go away — by refusing to refine something it should have
> refined — and a wrong-but-successful elaboration passes a vanishing-error
> test. This is the Architect's explicit carry and it is the single most likely
> way to get this node wrong.

**`AC-2` — `sibling_convoy_retypes_outer_binder_through_nested_match` stays
green.** **Control:** named individually in the handback with its own run, not
folded into a suite total. **This is the under-refinement guard** — it is the
single-level convoy the wider behaviour must generalize rather than replace,
and it is what catches a skip that is too wide.

**`AC-3` — the `D2` binder is shown to be a genuine outer binder that is NOT
skipped.** **Control:** report its measured `abs_pos` and the contents of
`match_field_regions` at that moment, showing the position lies above the
enclosing field region and outside every active range. **This is what
demonstrates provenance rather than position**; without it the implementation
is indistinguishable from a floor. If `D2` could not be made to reach
capability 2, say so here explicitly instead.

**`AC-4` — skip, never compose.** No composition of an inner refinement with an
enclosing one. **A candidate that composes has exceeded the node**, even if it
is correct.

**`AC-5` — direction stated for every behaviour change.** This **adds** accepted
programs; nothing currently accepted becomes rejected. **If anything does,
stop** — that is `AC-2`'s failure arriving by another route.

**`AC-6` — no-regression, in CI** (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## The residual, accepted and recorded so it is not re-filed

**Skipping is incomplete, not unsound.** Some programs that could type-check
will not, surfacing as a type error rather than a bad term. The Architect
considered composition — the richer fix — and **withheld it deliberately**: no
measured case needs it, and reflect-don't-extend applies.

⇒ **If you find a case that genuinely needs composition, that is a good finding
and it routes to the Architect. It is not this node's to build.**

## Banned scope

- **Do not compose refinements.** `AC-4`. Withheld by ruling.
- **Do not implement a positional floor**, threaded or otherwise, including as
  a "simpler first step". The floor is the refuted option.
- **Do not touch the kernel.** The region stack is additive on the elaboration
  context and touches no kernel-facing type; keep it that way.
- **Do not widen to capability 1 or to other `install_index_refinements`
  consumers.** If you find the same positional-proxy shape elsewhere, **report
  it and stop** — that is a Steward re-cut and a good finding.

## Sizing

**`S`.** One context field, one push/pop pair, one `continue`, two fixtures and
an assertion tightening. Cost at runtime is one push/pop per arm and a scan
over match-nesting depth per candidate.

**The hard stop worth naming:** if `D2`'s shape cannot reach capability 2, the
node still lands `D1`/`D3`/`D4` and reports the gap in coverage. That is a
complete outcome, not a partial failure.
