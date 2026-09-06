---
id: LANG-MATCH-RECORD-PATTERN
title: "record patterns `{ label = p, … }` -- slice 3 of 34 §3's six absent pattern forms: the second projecting form, but keyed by LABEL against the scrutinee's record declaration and OPEN (an omitted field is an implicit wildcard), checked in declaration order so dependent later fields see earlier projections; no elim_D (records are negative, 14 §4); and -- the composition discriminator this slice owes -- a redundant record arm must get the subsumption cause, never a false NoInhabitants"
status: active
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS, LANG-MATCH-MATRIX-OCCURRENCE-THREADING]
blocks: []
github: null
origin: "Steward cut 2026-09-06 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn (the prerequisite-ordered six-slice cut). Slice 1 (as-patterns, LANG-MATCH-AS-PATTERN) landed e6645d7c2; slice 2 (tuple/pair, LANG-MATCH-TUPLE-PATTERN) landed af2b36dc8/closed a8ee55b2f. This is slice 3, record patterns, next in the enclave's order. The enclave was explicit that records are NOT bundled with tuples merely because both project: separate node, separate pins (the field_pat spelling column). SPEC-MATCH-PATTERN-PINS merged (34fd01c1) discharged the prerequisite pin column, including the record field_pat pins now landed in 32-grammar.md §4 (:343-373). The value-binding predecessor LANG-MATCH-MATRIX-OCCURRENCE-THREADING (9ef5c3c19) threads a per-position occurrence to the arm-body leaf, which a componentwise record projection consumes. The umbrella stays draft and flips its record row when this lands. Anchors re-measured by the Steward at origin/main a8ee55b2f before framing."
---

> # RELEASED 2026-09-06 to the language ring (lane-2), slice 3 of the enclave's
> # six-slice cut (evt_12qrtnp7237dn). Both depends_on satisfied:
> # SPEC-MATCH-PATTERN-PINS merged (34fd01c1) landed the record field_pat pins;
> # the occurrence-threading capability LANG-MATCH-MATRIX-OCCURRENCE-THREADING
> # merged (9ef5c3c19) supplies the per-position occurrence a componentwise
> # record projection consumes. Immediate predecessor slice 2
> # (LANG-MATCH-TUPLE-PATTERN) is merged (a8ee55b2f) — records reuse its
> # projecting spine, not its positional keying. Base = current origin/main
> # a8ee55b2f (RE-MEASURE every anchor at your cut — the tuple landing advanced
> # these elab.rs lines once already; escalate a false fixed input rather than
> # building around it). The Architect is the required reviewer alongside
> # language-qa + CI: label-keyed open-record projection with declaration-order
> # dependent-field typing and the composition discriminator is a correctness
> # argument, not a mechanical edit.

## What this is

The **record** pattern form, `{ label = p, … }` -- slice 3 of the six absent
forms in `spec/30-surface/34-data-match.md §3` (form list at `:339`; the rule at
`:366-367`). Like the tuple slice it is a **projecting** form: `34 §3.1` states
*"Tuple / record patterns project the (negative) `Σ`/record components (`13 §3`,
`33 §2`) and match componentwise -- no `elim_D` (records are negative, matched by
projection, `14 §4`)."* So a record pattern lowers to **named projection**, never
to a data-eliminator.

It is a **separate** slice from tuples, and the enclave was explicit that records
are not bundled with tuples merely because both project. Records differ on three
axes tuples do not have, and those axes are the whole content of this node:

1. **Label-keyed, not positional.** A tuple projects by position (`Proj1`/
   `Proj2`); a record projects by **field label**, resolved against the
   scrutinee's record declaration.
2. **Open, not exhaustive-by-shape.** A record pattern lists a subset of fields;
   an **omitted field is matched by an implicit wildcard** (`32 §4:368`). A tuple
   pattern names every component.
3. **Declaration-order dependent typing.** Fields are checked in the record's
   **declaration order** (not source order) so a dependent later field's type
   sees the earlier projections.

## The design judgment, front-loaded

**1. What a record pattern means, and how it lowers.** `{ l₁ = p₁, …, lₖ = pₖ }`
matches a value of the (non-dependent or dependent) negative record type by, for
each listed field `lᵢ`, projecting the field named `lᵢ` from the scrutinee's
occurrence and matching `pᵢ` against that projection. Records are the **negative**
carrier: eliminated by named projection, never by a positive `elim_D`. The model
already in the tree is the record EXPRESSION path (`RExpr::RRecord { base, fields
}`, `check_pair_or_record` at `elab.rs:1210`, field projection at the Σ-record
projection site) -- a record pattern is the destructuring dual of that
construction. Match each `pᵢ` against the occurrence `Proj_{lᵢ}(<scrutinee
occurrence>)`, threaded through the landed occurrence capability.

**2. The seven field_pat pins are landed -- implement them exactly.**
`SPEC-MATCH-PATTERN-PINS` (merged, `34fd01c1`) settled `32-grammar.md §4`
(`:343-373`). Re-verify each at your cut:
   - `field_pat ::= ident "=" pattern | ident` (`:346`): a field is explicit
     `label = p` or **punned** `label`.
   - A **punned** `label` means `label = label` (`:366`): the label selects the
     projection and the second occurrence is an ordinary **variable pattern**
     binding that name.
   - Records are **open** (`:368`): an **omitted field is matched by an implicit
     wildcard** -- it is not an error and it binds nothing.
   - Field labels are **resolved against the scrutinee's record declaration**
     (`:369`).
   - Patterns are checked in **declaration order** (`:370`) so dependent later
     fields see the earlier projections. **Source order is not significant**
     (`:371`) -- reorder to declaration order before typing.
   - A **duplicate or unknown** field label is a **surface error** (`:371-373`);
     it is never treated as a positional field or an extension field.

**3. The per-component occurrence comes from the LANDED threading capability.**
`LANG-MATCH-MATRIX-OCCURRENCE-THREADING` (9ef5c3c19) threads a per-position
occurrence to the leaf; the tuple slice consumed it for positional projections.
Here each listed field `pᵢ` is matched against the occurrence
`Proj_{lᵢ}(<scrutinee occurrence>)`, and an omitted field contributes an implicit
wildcard at its projection. **Do not invent parallel matrix plumbing.** If a
label-keyed projection occurrence cannot be carried through the general match
matrix without new descent plumbing, that is the hard-stop finding (see Sizing),
not something to build around.

**4. A new AST / RPatKind variant is correct here.** Record patterns are a
genuinely new form; add `PatKind::Record(Vec<FieldPat>)` (where a `FieldPat`
carries a label and an optional pattern, an absent pattern meaning the punned
`label = label` variable) and the resolved `RPatKind::Record(...)` with labels
resolved to record-declaration field identities and reordered to declaration
order. This mirrors slice 1's `As` and slice 2's `Tuple` variant decisions.
Keep the surface field set + source order available for diagnostics (duplicate/
unknown-field errors report at the surface span), and normalize to
declaration-order for typing.

**5. Top level: a record pattern ELABORATES (it is a matcher, not a catch-all),
while wildcard/var STAY refused.** The top-level non-constructor refusal (guard on
`RPatKind::Wild | RPatKind::Var(_)`) exists because a top-level wildcard/variable
is a **catch-all** that breaks `NoInhabitants` honesty. A record pattern is **not**
a catch-all: it is a positive destructuring of the record, and -- because a record
type has exactly one introduction -- a single record arm listing (a subset of)
its fields is **exhaustive** (the omitted fields are implicit wildcards, which is
coverage, not a gap). So this slice lets a top-level record pattern elaborate
(routing it to the general matrix path) and **leaves the wildcard/var refusal
exactly as it is**. The load-bearing interaction is point 6.

**6. The composition discriminator this slice OWES (enclave rule).** The enclave
required that *"every later inner pattern form must add a composition
discriminator."* For records it is the same `NoInhabitants` honesty guard the
tuple slice carried, now for label-keyed arms. The reachability classifier reads
`None => ArmDeadCause::NoInhabitants`, and `subsumed_by` is populated only for an
arm whose pattern resolves to a **claiming constructor**. A record pattern does
not resolve to a named data constructor, so a **redundant** record arm (e.g. a
second `{ x = a, y = b }` after a first that already covers everything, or a
subset-record arm subsumed by an earlier one) risks landing in `None` and being
told it has **no inhabitants** -- a false diagnostic. **This slice must give a
redundant record arm the subsumption/redundancy cause, never `NoInhabitants`.**
That is AC-2 and it is asserted, not assumed. Note the openness interaction: an
arm `{ x = a }` that omits `y` subsumes a later `{ x = a, y = _ }`; the
subsumption sweep must see the omitted field as an implicit wildcard, not as an
absent obligation.

## Why this is NOT the wildcard/variable slice (hard scope line)

Accepting a top-level record pattern (point 5) is **not** accepting a top-level
catch-all, and an omitted field's implicit wildcard is **not** a top-level
catch-all either -- it is coverage inside a positive destructuring. The
`ArmDeadCause` third-cause obligation the umbrella carries -- a new cause for
"dead because earlier arms cover it, with no single claiming constructor", and
specifically **not** by widening `Subsumed` to an empty winner set -- attaches to
whichever slice lands **top-level wildcard/variable**. This slice does **not**
land that form and does **not** reopen `LANG-REACHABILITY-SUBSUMING-ARMS`. The
record redundancy in point 6 is discharged by giving the redundant record arm the
correct existing subsumption cause, within the componentwise reachability sweep --
not by inventing the third cause.

## Fixed inputs (measured at origin/main a8ee55b2f; RE-MEASURE at release -- the tuple landing drifted these once)

- `PatKind`: `ast.rs:167`, five arms `{Wild, Var, Ctor, Tuple, As}` -- **no
  `Record`**. `Tuple` is at `:175` (slice 2's landed variant), `As` at `:177`.
- `RPatKind`: `resolve.rs`, five arms `{Wild, Var, Ctor, Tuple, As}` -- **no
  `Record`**.
- Record-EXPRESSION carriers (the model for projection): `RExpr::RRecord { base,
  fields }`; `check_pair_or_record` at `elab.rs:1210` (and the pair/record split
  at `:1057`); the negative-Σ / record field projection site; `infer_proj`. Kernel
  `Term::{Sigma, Pair, Proj1, Proj2}` present; named-field projection lowers
  through the record-declaration field order. No record `elim` exists and none is
  to be added -- projection only (`34 §3.1:367`).
- Top-level non-constructor refusal: guard on `RPatKind::Wild | RPatKind::Var(_)`
  near `elab.rs:12829`. **Preserve this for top-level wildcard/var; a top-level
  record pattern is NOT this guard's case.**
- Reachability classifier `None => ArmDeadCause::NoInhabitants`: `elab.rs:3831`,
  `:5782`, and the general-matrix site (drifted to ~`:13029`/`:13148` after the
  tuple landing -- re-measure); `ArmDeadCause` enum in `error.rs`;
  `mark_shared_ctor_subsumption` is the subsumption populator.
- Record field_pat grammar: `32-grammar.md §4:343` (open record pattern), `:346`
  (`field_pat`), `:365-373` (the pin prose). Semantics rule: `34 §3.1:366-367`.
- No record PATTERN handling exists anywhere in parser, resolver, or elaborator
  today (only record EXPRESSIONS). Confirm with `git grep` at your cut.

## Deliverables

**`D1` -- surface + AST.** The parser accepts `{ field_pat, … }` with the landed
pins: explicit `label = p` and punned `label`; open records (an omitted field is
implicit-wildcard, not an error); duplicate or unknown field label is a surface
error at its span. Add `PatKind::Record(Vec<FieldPat>)`; resolve threads it to
`RPatKind::Record(...)` with labels resolved against the record declaration and
normalized to declaration order.

**`D2` -- elaboration by named projection.** `{ l₁ = p₁, … }` matches by
projecting each listed field `lᵢ` from the scrutinee's occurrence
(`Proj_{lᵢ}`, resolved via the record declaration) and matching `pᵢ` against it,
threaded through the landed occurrence capability; omitted fields contribute
implicit wildcards. No record `elim`. Fields are typed in **declaration order** so
a dependent later field's type sees the earlier projections. Report the
discriminating elaboration point at `file:line`, including the dependent-field
typing point.

**`D3` -- coverage/reachability, componentwise and open.** A single record arm
over a record type is exhaustive (omitted fields are implicit wildcards, which is
coverage). A record arm is redundant iff its listed-field cover -- with omissions
read as wildcards -- is subsumed by an earlier arm. The exhaustiveness/
reachability sweeps descend into the listed fields.

**`D4` -- the composition discriminator (NoInhabitants honesty).** A redundant
record arm is flagged with the subsumption/redundancy cause, **never**
`NoInhabitants`, including the open-record subsumption case where an earlier
field-subset arm covers a later fuller arm. This is the enclave-required
discriminator for this slice.

**`D5` -- fail-closed for the unlanded forms.** Or-patterns, guards, and literals
remain absent and refused. A wildcard/variable **at top level** stays refused
exactly as today (the record form does not relax it).

## Acceptance criteria

**`AC-1`.** A top-level `{ l₁ = p₁, … }` elaborates: each listed field matches by
named projection and its binders are in scope in the arm body; a **punned**
`label` binds `label`. **Control:** assert structurally that the body sees each
listed field's binding (and, for a dependent record, that a later field checks at
the type refined by the earlier projections) -- not "it compiles". Include a
punned-field case and an omitted-field case.

**`AC-2` -- the composition discriminator, asserted.** A redundant record arm
(subsumed by an earlier record arm, including the field-subset-subsumes-superset
open-record case) is flagged redundant with the **subsumption** `ArmDeadCause`,
and the test asserts the cause is subsumption, **NOT** `NoInhabitants`. A test
that lets a redundant record arm read as `NoInhabitants` is a defect against this
slice.

**`AC-3` -- openness and declaration order, asserted.** An **omitted** field is an
implicit wildcard (control: an arm omitting a field is accepted and binds nothing
for it, no false "non-exhaustive"); **source order is not significant** (control:
two arms differing only in field source order behave identically); fields are
typed in **declaration order** (control: a dependent record where a later field's
type depends on an earlier projection checks correctly regardless of source
order). Assert on the diagnostic/elaborated form, not `is_err()`.

**`AC-4` -- label pins.** A **duplicate** field label and an **unknown** field
label are each rejected at the surface as errors (control: assert the specific
diagnostic, and that neither is silently treated as positional or as an
extension field).

**`AC-5` -- projection, not elimination.** The elaborated form uses named record
projection (mirroring record expressions); no record `elim`/`elim_D` is
introduced. Assert on the elaborated term shape.

**`AC-6` -- scope integrity.** `PatKind::Record` / `RPatKind::Record` added; NO
change to or-pattern, guard, or literal forms -- they remain absent and
fail-closed; the top-level wildcard/var refusal and the landed tuple/as forms are
unchanged. If landing records turns out to require touching any of those, that is
the AC-stop finding, not a widening.

**`AC-7` -- no-regression in CI (`COORDINATION §12`).** Targeted locally
`-p ken-elaborator`; never `--workspace`.

## Not this slice

- **Not or-patterns / guards / literals** -- slices 4, 5, 6, each its own node,
  each fail-closed until it lands.
- **Not top-level wildcard/variable acceptance, and not the `ArmDeadCause` third
  cause.** Those attach to whichever slice lands a top-level non-constructor
  catch-all; this slice does not, keeping `NoInhabitants` honest.
- **Not named-argument/record-field constructor sugar** (`SURF-gadt-field-sugar`,
  `34:1063`) -- that is a later constructor-application surface refinement, a
  different feature from destructuring a negative record by projection.
- **Not the reachability slice-indexing hygiene residual** the umbrella carries
  (the `get().expect(<invariant>)` convention at the `mark_shared_ctor_subsumption`
  / matrix-descent sites). Fold it in ONLY if this implementation genuinely
  touches those sites; otherwise leave it and it stays on the umbrella.
- **Not an amendment to `34 §3`.** The enclave ruled the chapter's obligations
  real and present-tense.

## Contention

Shares `crates/ken-elaborator` with lane-2 work. Slices 1 (`LANG-MATCH-AS-PATTERN`)
and 2 (`LANG-MATCH-TUPLE-PATTERN`) are merged; no live lane-2 `elab.rs` contention
at release. Foundation's Tier-C P3 (`CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION`) also
builds under `-p ken-elaborator` but touches `catalog/` + the roots-loader
inventory, disjoint from this slice's pattern AST / resolve / match-elaboration
source. The lieutenant publishes serially; the second candidate rebases.
Re-check at release.

## Sizing and tier

**`M`.** A new `PatKind`/`RPatKind` variant with label resolution against the
record declaration, open-record implicit-wildcard coverage, declaration-order
dependent-field typing, named-projection elaboration over the landed occurrence
capability, componentwise coverage/reachability, and the NoInhabitants
composition discriminator. Comparable to the tuple slice, with label-keyed open
semantics replacing positional shape. **Hard stop and finding:** if a label-keyed
projection occurrence cannot be carried through the general match matrix without
new matrix-descent plumbing (rather than consuming the landed occurrence
threading), surface that -- do not build around it.

**Tier `T1`.** Declaration-order dependent-field typing, open-record coverage,
and the `NoInhabitants` honesty discriminator are soundness-adjacent correctness
arguments, not mechanical edits. The T1 language seat fits; no reseat.
