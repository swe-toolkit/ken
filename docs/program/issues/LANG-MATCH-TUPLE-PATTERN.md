---
id: LANG-MATCH-TUPLE-PATTERN
title: "tuple/pair patterns `(p₁, …, pₙ)` -- slice 2 of 34 §3's six absent pattern forms and the first SPLITTING form: it projects the negative Σ componentwise (no elim_D, matched by Proj1/Proj2), right-nests for arity >2, carries componentwise coverage/reachability, and -- the composition discriminator this slice owes -- a redundant tuple arm must get the subsumption cause, never a false NoInhabitants"
status: merged
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS, LANG-MATCH-MATRIX-OCCURRENCE-THREADING]
blocks: []
github: null
origin: "Steward cut 2026-09-06 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn, which gave the prerequisite-ordered six-slice cut. Slice 1 (as-patterns, LANG-MATCH-AS-PATTERN) landed e6645d7c2/closed 4cbb0c088; this is slice 2, tuple/pair patterns, in the enclave's order. SPEC-MATCH-PATTERN-PINS merged at 34fd01c1 discharged the whole prerequisite pin column, so the tuple grouping/comma/arity pins (32-grammar.md §4: `(p)` is grouping, a tuple contains at least two patterns, arity >2 right-nests, no zero- or one-tuple) are landed and this slice is cuttable. Its value-binding predecessor LANG-MATCH-MATRIX-OCCURRENCE-THREADING (9ef5c3c19) landed -- it threads a per-position occurrence to the arm-body leaf and subsumes every value-binding 34 §3 form, which is the capability a componentwise tuple projection consumes. The umbrella stays draft and flips its tuple row when this lands. Anchors re-measured by the Steward at origin/main 072dc688d before framing."
---

> # RELEASED 2026-09-06 to the language ring (lane-2), slice 2 of the enclave's
> # six-slice cut (evt_12qrtnp7237dn). Both depends_on satisfied:
> # SPEC-MATCH-PATTERN-PINS merged (34fd01c1) landed the tuple pins; the
> # occurrence-threading capability LANG-MATCH-MATRIX-OCCURRENCE-THREADING merged
> # (9ef5c3c19) supplies the per-position occurrence a componentwise tuple pattern
> # consumes. Base = current origin/main 072dc688d (RE-MEASURE every anchor at
> # your cut -- as-pattern already advanced these lines once; escalate a false
> # fixed input rather than building around it). The Architect is the required
> # reviewer alongside language-qa + CI: this is the FIRST splitting form and the
> # composition-discriminator (NoInhabitants honesty for a redundant tuple arm) is
> # a correctness argument, not a mechanical edit.

## What this is

The **tuple/pair** pattern form, `(p₁, …, pₙ)` -- slice 2 of the six absent
forms in `spec/30-surface/34-data-match.md §3` (form list at `:339`; the rule at
`:366-368`). It is the **first SPLITTING form**: unlike the as-pattern wrapper
(slice 1), a tuple pattern destructures the scrutinee and matches its components,
so it owns componentwise coverage and reachability.

`34 §3.1` states the elaboration exactly (`:366-368`): *"Tuple / record patterns
project the (negative) Σ/record components (`13 §3`, `33 §2`) and match
componentwise -- no `elim_D` (records are negative, matched by projection,
`14 §4`)."* So a tuple pattern lowers to **projection**, not to a data-eliminator.

This is a standalone slice, not the umbrella. Per the enclave disposition, the
umbrella `LANG-MATCH-PATTERN-FORMS-ABSENT` stays `draft` and **must never become
a six-form frame**; it flips its tuple row when this node lands. Record patterns
(slice 3) are a SEPARATE slice with separate pins -- the enclave was explicit
that record patterns are **not** bundled with tuples merely because both project.

## The design judgment, front-loaded

**1. What a tuple pattern means, and how it lowers.** `(p₁, …, pₙ)` matches a
value of the (non-dependent or dependent) `Σ`/pair type by projecting each
component and matching `pᵢ` against it. Pairs are the **negative** Σ: they are
eliminated by `Proj1`/`Proj2`, never by a positive `elim_D`. The expected
lowering mirrors how pair EXPRESSIONS already lower (`LANG-SURFACE-PAIR`, merged:
`EPair` right-nests into binary kernel `Pair`, `EProj`/`EPosProj` lower to
`Proj1`/`Proj2` -- `ast.rs:641/:652/:654`, `check_pair_or_record` at
`elab.rs:1210`). A tuple pattern therefore projects the scrutinee's occurrence
into per-component occurrences and matches each `pᵢ` against its projection.

**2. Arity >2 right-nests, and the pins are landed.** `SPEC-MATCH-PATTERN-PINS`
(merged, `34fd01c1`, `32-grammar.md §4`) settled: `(p)` is **grouping**, not a
1-tuple; a tuple contains **at least two** patterns; arity >2 **right-nests**
(`(p₁, p₂, p₃)` is `(p₁, (p₂, p₃))`); there is no zero- or one-tuple. Implement
that grouping/nesting exactly, so the shape matches the pair-expression carrier
it destructures. Re-verify the pins against `32-grammar.md §4` at your cut.

**3. The per-component occurrence comes from the LANDED threading capability,
not new matrix plumbing.** `LANG-MATCH-MATRIX-OCCURRENCE-THREADING` (9ef5c3c19)
threads a per-position occurrence to the leaf; slice 1 consumed it for the alias
value. Here each component `pᵢ` is matched against the occurrence
`Proj_i(<scrutinee occurrence>)`. **The design below stands on that capability;
do not invent parallel matrix plumbing.** If a componentwise projection occurrence
cannot be carried through the general match matrix without new descent plumbing,
that is the hard-stop finding (see Sizing), not something to build around.

**4. A new AST / RPatKind variant is correct here.** Tuple patterns are a
genuinely new form; add `PatKind::Tuple(Vec<Pattern>)` and the resolved
`RPatKind::Tuple(Vec<RPattern>)` (right-nested to binary at lowering, matching
the kernel `Pair` carrier). This is the same reasoning as slice 1's `As` variant,
and the opposite of `LANG-BYTES-HEX-LIST-LITERAL`, where a new token would have
signalled a wrong reading. Keep the surface arity in the AST and right-nest at
lowering, so a >2 arity error is reported at the surface, not after nesting.

**5. Top level: a tuple pattern ELABORATES (it is a matcher, not a catch-all),
while wildcard/var STAY refused.** The top-level non-constructor refusal
(`elab.rs:12829`, guard on `RPatKind::Wild | RPatKind::Var(_)`) exists because a
top-level wildcard/variable is a **catch-all** that breaks `NoInhabitants`
honesty (umbrella, "the consequence"). A tuple pattern is **not** a catch-all: it
is a positive destructuring of the pair, and a single tuple arm over a pair type
is **exhaustive** because the pair has exactly one introduction. So this slice
lets a top-level tuple pattern elaborate (routing it to the general matrix path,
which handles nested sub-patterns), and **leaves the wildcard/var refusal exactly
as it is**. The load-bearing interaction is point 6.

**6. The composition discriminator this slice OWES (enclave rule).** The enclave
required that *"every later inner pattern form must add a composition
discriminator."* For tuples it is the `NoInhabitants` honesty guard. The
reachability classifier reads `None => ArmDeadCause::NoInhabitants`
(`elab.rs:3831`, `:5782`, `:12882`), and `subsumed_by` is populated only for an
arm whose pattern resolves to a **claiming constructor**. A tuple pattern does
not resolve to a named data constructor, so a **redundant** tuple arm (e.g. a
second `(a, b)` after a first that already matches everything) risks landing in
`None` and being told it has **no inhabitants** -- a false diagnostic, the same
hazard slice 1 guarded for as-patterns. **This slice must give a redundant tuple
arm the subsumption/redundancy cause, never `NoInhabitants`.** That is AC-2 and
it is asserted, not assumed.

## Why this is NOT the wildcard/variable slice (hard scope line)

Accepting a top-level tuple pattern (point 5) is **not** accepting a top-level
catch-all. The `ArmDeadCause` third-cause obligation the umbrella carries -- a new
cause for "dead because earlier arms cover it, with no single claiming
constructor", and specifically **not** by widening `Subsumed` to an empty winner
set -- attaches to whichever slice lands **top-level wildcard/variable**. This
slice does **not** land that form and does **not** reopen
`LANG-REACHABILITY-SUBSUMING-ARMS`. The tuple redundancy in point 6 is discharged
by giving the redundant tuple arm the correct existing subsumption cause, within
the componentwise reachability sweep -- not by inventing the third cause.

## Fixed inputs (measured at origin/main 072dc688d; RE-MEASURE at release)

- `PatKind`: `ast.rs:167`, four arms `{Wild, Var, Ctor, As}` -- **no `Tuple`**.
  `As` is at `:175`. `MatchArm`: `ast.rs:86` (no guard field). Pair-expression
  carriers: `EPair` `ast.rs:641` (right-nests to binary kernel `Pair`), `EProj`
  `:652`, `EPosProj` `:654`.
- `RPatKind`: `resolve.rs:54`, four arms `{Wild, Var, Ctor, As(Box<RPattern>,
  String, usize)}` -- **no `Tuple`**; `RMatchArm` immediately below.
- Top-level non-constructor refusal: message at `elab.rs:12829`, guard on
  `RPatKind::Wild | RPatKind::Var(_)`. **Preserve this for top-level wildcard/var;
  a top-level tuple pattern is NOT this guard's case.**
- Reachability classifier `None => ArmDeadCause::NoInhabitants`: `elab.rs:3831`,
  `:5782`, `:12882` (the general-matrix site); `ArmDeadCause` enum at
  `error.rs`. `mark_shared_ctor_subsumption` is the subsumption populator.
- Pair-expression lowering path (the model for projection): `RExpr::RPair` ->
  `check_pair_or_record` at `elab.rs:1210`; kernel `Term::{Sigma, Pair, Proj1,
  Proj2}` present. No pair `elim` exists and none is to be added -- projection
  only (`34 §3.1:367`).
- No tuple/pair PATTERN handling exists anywhere in parser, resolver, or
  elaborator today (only pair EXPRESSIONS via `LANG-SURFACE-PAIR`). Confirm with
  `git grep` at your cut.

## Deliverables

**`D1` -- surface + AST.** The parser accepts `(p₁, …, pₙ)` (n ≥ 2) with the
landed pins: `(p)` is grouping (not a 1-tuple), a tuple needs at least one comma,
arity >2 right-nests, no zero-tuple. Add `PatKind::Tuple(Vec<Pattern>)`; resolve
threads it to `RPatKind::Tuple(Vec<RPattern>)`. Report a >2 arity or malformed
tuple at the surface span.

**`D2` -- elaboration by projection.** `(p₁, …, pₙ)` matches by projecting the
scrutinee's occurrence componentwise (`Proj1`/`Proj2`, right-nested for n >2) and
matching each `pᵢ` against its component occurrence, threaded through the landed
occurrence capability. No pair `elim`. Report the discriminating elaboration
point at `file:line`, including the dependent-`Σ` component-typing point (the
second component's type may depend on the first).

**`D3` -- coverage/reachability, componentwise.** Exhaustiveness and reachability
are computed componentwise: a single tuple arm over a pair type is exhaustive; a
tuple arm is redundant iff its componentwise cover is subsumed. The
exhaustiveness/reachability sweeps descend into the components.

**`D4` -- the composition discriminator (NoInhabitants honesty).** A redundant
tuple arm is flagged with the subsumption/redundancy cause, **never**
`NoInhabitants`. This is the enclave-required discriminator for this slice.

**`D5` -- fail-closed for the unlanded forms.** Record patterns, or-patterns,
guards, and literals remain absent and refused. A wildcard/variable **at top
level** stays refused exactly as today (the tuple form does not relax it).

## Acceptance criteria

**`AC-1`.** A top-level `(p₁, …, pₙ)` elaborates: the components match by
projection and their binders are in scope in the arm body. **Control:** assert
structurally that the body sees each component's binding (and, for a dependent
`Σ`, that the second component checks at the type refined by the first) -- not
"it compiles". Include an arity-3 case to exercise right-nesting.

**`AC-2` -- the composition discriminator, asserted.** A redundant tuple arm (a
second tuple arm subsumed by an earlier one) is flagged redundant with the
**subsumption** `ArmDeadCause`, and the test asserts the cause is subsumption,
**NOT** `NoInhabitants`. This is the honesty guard for the first splitting form;
a test that lets a redundant tuple arm read as `NoInhabitants` is a defect against
this slice.

**`AC-3` -- exhaustiveness is componentwise and honest.** A single tuple arm over
a pair type is accepted as exhaustive (control: no false "non-exhaustive"); and a
tuple match that is genuinely non-exhaustive in a component (once inner
constructor patterns are involved) reports the missing component case, not a whole
tuple. Assert on the diagnostic, not `is_err()`.

**`AC-4` -- arity/grouping pins.** `(p)` is grouping and binds `p`, not a 1-tuple;
a zero-tuple `()` and a malformed tuple are rejected at the surface; arity >2
right-nests such that `(a, b, c)` matches `(a, (b, c))`. Assert on the parse
result / message.

**`AC-5` -- projection, not elimination.** The elaborated form uses `Proj1`/
`Proj2` (mirroring pair expressions); no pair `elim`/`elim_D` is introduced.
Assert on the elaborated term shape.

**`AC-6` -- scope integrity.** `PatKind::Tuple` / `RPatKind::Tuple` added; NO
change to record, or-pattern, guard, or literal forms -- they remain absent and
fail-closed; the top-level wildcard/var refusal is unchanged. If landing tuples
turns out to require touching any of those, that is the AC-stop finding, not a
widening.

**`AC-7` -- no-regression in CI (`COORDINATION §12`).** Targeted locally
`-p ken-elaborator`; never `--workspace`.

## Not this slice

- **Not record patterns (slice 3).** Separate node, separate pins (`field_pat`
  spelling: label/value form, punning, omission, open-vs-closed, duplicates,
  unknown fields, source order). The enclave forbade bundling records with tuples
  merely because both project.
- **Not or-patterns / guards / literals** -- slices 4, 5, 6, each its own node,
  each fail-closed until it lands.
- **Not top-level wildcard/variable acceptance, and not the `ArmDeadCause` third
  cause.** Those attach to whichever slice lands a top-level non-constructor
  catch-all; this slice does not, keeping `NoInhabitants` honest.
- **Not the reachability slice-indexing hygiene residual** the umbrella carries
  (the `get().expect(<invariant>)` convention at the drifted
  `mark_shared_ctor_subsumption` / matrix-descent sites). Fold it in ONLY if this
  implementation genuinely touches those sites; otherwise leave it untouched and
  it stays on the umbrella.
- **Not an amendment to `34 §3`.** The enclave ruled the chapter's obligations
  real and present-tense; the "drop a form from the surface" branch is closed.

## Contention

Shares `crates/ken-elaborator` with lane-2 work. The `elab.rs` in-line queue the
umbrella named (`LANG-REACHABILITY-SUBSUMING-ARMS`,
`LANG-WITNESS-DIAGNOSTIC-STRICTNESS`, `LANG-FOREIGN-CTOR-ARM-REJECT`) and slice 1
(`LANG-MATCH-AS-PATTERN`) are all `merged`; no live lane-2 `elab.rs` contention at
release. Foundation's P3 (`CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION`) also builds
under `-p ken-elaborator` but touches `catalog/` + the roots-loader inventory,
disjoint from this slice's pattern AST / resolve / match-elaboration source. The
lieutenant publishes serially; the second candidate rebases. Re-check at release.

## Sizing and tier

**`M`.** The first splitting form: a new `PatKind`/`RPatKind` variant with
surface arity + right-nesting, projection-based elaboration over the landed
occurrence capability, componentwise coverage/reachability, and the
NoInhabitants composition discriminator. Larger than the as-pattern wrapper (`S`)
because it splits the scrutinee and owns coverage of its own. **Hard stop and
finding:** if a componentwise projection occurrence cannot be carried through the
general match matrix without new matrix-descent plumbing (rather than consuming
the landed occurrence threading), surface that -- do not build around it.

**Tier `T1`.** Componentwise coverage/reachability preservation, dependent-`Σ`
component typing, and the `NoInhabitants` honesty discriminator are
soundness-adjacent correctness arguments, not mechanical edits. The T1 language
seat fits; no reseat.
