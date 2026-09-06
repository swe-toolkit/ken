---
id: LANG-MATCH-OR-PATTERN
title: "or-patterns `p | q` -- slice 4 of 34 §3's six absent pattern forms: an ALTERNATION over the same occurrence (not a projecting form, no new carrier), where every alternative binds the SAME name set at definitionally-equal types in the common pre-branch context (32 §4:345, 34 §3.1:372-382), coverage is the UNION of the alternatives and the arm is reachable if any alternative has a non-empty residual; and -- the composition discriminator this slice owes -- a fully-subsumed or-arm gets the subsumption cause never a false NoInhabitants, and an or-pattern carrying a top-level wildcard/variable ALTERNATIVE stays refused as the catch-all it is"
status: active
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS, LANG-MATCH-MATRIX-OCCURRENCE-THREADING]
blocks: []
github: null
origin: "Steward cut 2026-09-06 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn (the prerequisite-ordered six-slice cut). Slice 1 (as-patterns, LANG-MATCH-AS-PATTERN) landed e6645d7c2; slice 2 (tuple/pair, LANG-MATCH-TUPLE-PATTERN) landed af2b36dc8/closed a8ee55b2f; slice 3 (record, LANG-MATCH-RECORD-PATTERN) landed a42b90454 (blob-verified). This is slice 4, or-patterns, next in the enclave's order. SPEC-MATCH-PATTERN-PINS merged (34fd01c1) discharged the pin column, including the or-pattern grammar (32-grammar.md §4:345 `pattern \"|\" pattern` same-binders + the precedence prose :354-358). The value-binding predecessor LANG-MATCH-MATRIX-OCCURRENCE-THREADING (9ef5c3c19) threads a per-position occurrence to the arm-body leaf; an or-pattern alternates over the SAME occurrence and consumes it unchanged. The umbrella stays draft and flips its or-pattern row when this lands. Anchors measured by the Steward at origin/main a42b90454 before framing."
---

> # RELEASED 2026-09-06 to the language ring (lane-2), slice 4 of the enclave's
> # six-slice cut (evt_12qrtnp7237dn). Both depends_on satisfied:
> # SPEC-MATCH-PATTERN-PINS merged (34fd01c1) landed the or-pattern grammar pin
> # (same-binders `p | q`, precedence `as` tighter than `|`, flat alternative
> # list); the occurrence-threading capability
> # LANG-MATCH-MATRIX-OCCURRENCE-THREADING merged (9ef5c3c19) supplies the
> # per-position occurrence the alternation reuses. Immediate predecessor slice 3
> # (LANG-MATCH-RECORD-PATTERN) is merged (a42b90454) -- or-patterns share the
> # match-matrix spine but add NO projection: an or-pattern is an alternation over
> # the SAME column/occurrence, not a new carrier. Base = current origin/main
> # a42b90454 (RE-MEASURE every anchor at your cut -- the record landing advanced
> # the elab.rs NoInhabitants and top-level-refusal lines once already; escalate a
> # false fixed input rather than building around it). The Architect is the
> # required reviewer alongside language-qa + CI: binder-set identity, common-
> # pre-branch-context binder-type equality (with the spec's deliberate
> # conservatism), union coverage, and the composition discriminator are a
> # correctness argument, not a mechanical edit.

## What this is

The **or-pattern** form, `p | q` -- slice 4 of the six absent forms in
`spec/30-surface/34-data-match.md §3` (form list at `:339-340`; the rule at
`§3.1:372-382`). Unlike the tuple and record slices it is **NOT a projecting
form and adds NO new carrier**: an or-pattern is an **alternation over the same
scrutinee occurrence** -- *"`p | q` duplicate the residual arm under both
alternatives"* (`34 §3.1:372`). It matches iff any alternative matches, against
the identical column the arm was already scrutinizing.

It differs from every prior slice on the axis that is the whole content of this
node: **the alternatives must agree**. Three coupled constraints, all from
`34 §3.1:373-382`:

1. **Same binder-name set.** *"Each alternative binds every name exactly once,
   and all alternatives have the same binder-name set."* `(Inl x | Inr x)` is
   well-formed; `(Inl x | Inr y)` is an error (binder sets `{x}` vs `{y}`
   differ); a name bound twice within one alternative is an error.
2. **Definitionally-equal binder types in the COMMON PRE-BRANCH context.**
   *"Corresponding bindings must have definitionally equal types in the common
   context before the branch."* The check is **deliberately conservative** (spec
   `:379-382`): it may reject a dependent or-pattern whose binder types would
   coincide only after distinct alternative-specific refinements -- *"Such a
   pattern must use separate arms unless a later rule supplies a sound
   refined-context join."* This slice implements the conservative check and does
   NOT supply that later join.
3. **Canonical binder order by name.** *"The first alternative's left-to-right
   binder order is the canonical branch order; later alternatives map their
   bindings into those slots by name."* The arm body sees one binding per name;
   both alternatives feed it, mapped by name into the first alternative's slots.

Coverage and reachability (`:377-379`): the source arm is reachable iff **at
least one alternative has a non-empty residual**, and its coverage is the
**union** of the alternatives' coverage.

## The design judgment, front-loaded

**1. What an or-pattern means, and how it lowers.** `p₁ | … | pₙ` (a flat list
of `n >= 2` alternatives -- `|` is a flat chain whose grouping has no semantic
effect, `32 §4:357`) matches the scrutinee occurrence iff some `pᵢ` matches it.
In the pattern-matrix algorithm (`34 §3.1`) it **duplicates the residual arm
under each alternative** (`:372`): a matrix row whose head cell is an or-pattern
expands to `n` rows, one per alternative, each feeding the SAME arm body, with
the alternative's binders mapped by name into the canonical slots. It is **not a
new kernel primitive and adds no projection** -- it reuses the existing
column/occurrence and the existing `elim_D` split the alternatives resolve to.
Do NOT invent a new carrier or a parallel matrix.

**2. Binder-set identity is the load-bearing well-formedness check.** After
resolving each alternative, compute its binder-name set. Reject, at the surface
span, any or-pattern whose alternatives do not all carry the identical set
(`34 §3.1:373`), and any alternative that binds a name more than once. This is
resolve-time structural, independent of typing.

**3. Binder-type equality is checked in the common pre-branch context, and is
conservative by design.** For each shared binder name, the type it receives
under each alternative must be **definitionally equal in the context that holds
BEFORE the branch splits** (`34 §3.1:375`). Do NOT check equality after
alternative-specific refinement -- the spec is explicit (`:379-382`) that the
conservative pre-branch check is correct and that a dependent or-pattern needing
alternative-specific refinement **must use separate arms**. Rejecting such a
pattern is CORRECT behavior, not a gap; report it as the "use separate arms"
diagnostic, not as an internal error. This conservatism is the reason an
or-pattern is sound without a refined-context join.

**4. Canonical-order-by-name binder mapping.** The first alternative fixes the
slot order (its left-to-right binder order). Each later alternative maps its
same-named bindings into those slots (`34 §3.1:376-377`). The arm body binds
each name once, at the canonical slot; both alternatives' residuals feed it.

**5. The per-alternative occurrence is the LANDED threading occurrence,
unchanged.** `LANG-MATCH-MATRIX-OCCURRENCE-THREADING` (9ef5c3c19) threads a
per-position occurrence to the leaf. An or-pattern alternates over the SAME
occurrence the arm was already at -- each alternative matches that identical
occurrence; there is no new projection and no new descent. **If duplicating the
residual arm under the alternatives cannot be expressed over the general match
matrix without new matrix-descent plumbing, that is the hard-stop finding (see
Sizing), not something to build around.**

**6. A new AST / RPatKind variant is correct here.** Add
`PatKind::Or(Vec<Pattern>)` (a flat list of `n >= 2` alternatives; the parser
flattens the `|` chain -- `32 §4:357`) and the resolved `RPatKind::Or(...)` with
the binder-set-identity check applied and the canonical-slot mapping computed.
This mirrors slice 1's `As`, slice 2's `Tuple`, and slice 3's `Record` variant
decisions. Parse precedence: `as` binds tighter than `|` (`32 §4:354-356`), so
`p as x | q` is `(p as x) | q`; an or-pattern used as a constructor argument
must be parenthesized (`:358`). Keep the surface spans for the well-formedness
diagnostics (mismatched binder sets, duplicate name, unequal binder types).

**7. Top level: an or-pattern of positive alternatives ELABORATES, but a
top-level wildcard/variable ALTERNATIVE stays refused.** The top-level
non-constructor refusal (guard on `RPatKind::Wild | RPatKind::Var(_, _)`) exists
because a top-level catch-all breaks `NoInhabitants` honesty. An or-pattern all
of whose alternatives are positive (constructor / tuple / record / their
as-forms) is **not** a catch-all -- it is a positive alternation, and its
coverage is the union of its alternatives. But an or-pattern that carries a
`Wild` or `Var` **alternative** at top level (`_ | C`, `x | C y`) IS a catch-all
through that alternative, and must stay refused exactly as a bare top-level
`Wild`/`Var` is. So the refusal is lifted to see **through** an or-pattern: it
fires if ANY alternative is a top-level `Wild`/`Var`. The load-bearing
interaction is point 8.

**8. The composition discriminator this slice OWES (enclave rule).** The enclave
required that *"every later inner pattern form must add a composition
discriminator."* For or-patterns it has two coupled parts, both about keeping
`NoInhabitants` honest when the arm has no single claiming constructor:
   - **A fully-subsumed or-arm gets the subsumption cause, never
     `NoInhabitants`.** The reachability classifier reads
     `None => ArmDeadCause::NoInhabitants`, and `subsumed_by` is populated only
     for an arm whose pattern resolves to a single **claiming constructor**. An
     or-pattern resolves to the UNION of its alternatives' constructors, not one
     -- so an or-arm ALL of whose alternatives are subsumed by earlier arms (or
     a redundant inner alternative, e.g. `(A | A)`, or `(A | B)` after `A` and
     `B` are each covered) risks landing in `None` and being told it has **no
     inhabitants** -- a false diagnostic. This slice must give a fully-subsumed
     or-arm the subsumption/redundancy cause.
   - **A partially-dead or-pattern stays REACHABLE.** Because coverage is the
     union and the arm is reachable if ANY alternative has a non-empty residual
     (`34 §3.1:377-378`), one dead alternative alongside a live one does NOT
     kill the arm. The redundancy sweep must not report the whole arm dead when
     only some alternatives are subsumed; it reports at most the individual
     redundant alternative. This is AC-2 and it is asserted, not assumed.

## Why this is NOT the wildcard/variable slice (hard scope line)

Accepting a top-level or-pattern of positive alternatives (point 7) is **not**
accepting a top-level catch-all, and lifting the top-level refusal to see
through an or-pattern's `Wild`/`Var` alternatives KEEPS the catch-all refused.
The `ArmDeadCause` third-cause obligation the umbrella carries -- a new cause for
"dead because earlier arms cover it, with no single claiming constructor", and
specifically **not** by widening `Subsumed` to an empty winner set -- attaches to
whichever slice lands **top-level wildcard/variable acceptance**. This slice does
**not** land that form and does **not** reopen `LANG-REACHABILITY-SUBSUMING-ARMS`.
The or-arm redundancy in point 8 is discharged by giving a fully-subsumed or-arm
the correct existing subsumption cause within the componentwise reachability
sweep -- not by inventing the third cause. A `Wild`/`Var` alternative is not a
way to sneak a catch-all in: it stays refused.

## Fixed inputs (measured at origin/main a42b90454; RE-MEASURE at your cut -- the record landing drifted the elab.rs lines once)

- `PatKind`: `ast.rs:176`, six arms `{Wild:178, Var:180, Ctor:182, Tuple:184,
  Record:186, As:188}` -- **no `Or`**.
- `RPatKind`: `resolve.rs:61`, six arms `{Wild:62, Var:65, Ctor:66, Tuple:67,
  Record:68, As:69}` -- **no `Or`**. (`Var` carries `Option<usize>` occurrence
  slot; `As` carries a `usize` slot -- follow the record slice's occurrence-
  backed convention for any or-pattern binder.)
- Top-level non-constructor refusal: guard on `RPatKind::Wild | RPatKind::Var(_,
  _)` at `elab.rs:12593` and `:12806` (plus the matrix-descent occurrences at
  `:13034`, `:13202`, `:13281`, `:13368`, `:13493`, `:13618` -- re-measure).
  **This is the guard point 7 lifts to see through an or-pattern; a positive
  or-pattern is NOT this guard's case, a `Wild`/`Var`-bearing one IS.**
- Reachability classifier `None => ArmDeadCause::NoInhabitants`: `elab.rs:3835`,
  `:5786`, `:13343`, `:13424`, `:13549` (drifted after the record landing --
  re-measure); `ArmDeadCause` enum in `error.rs`; `mark_shared_ctor_subsumption`
  is the subsumption populator (the point 8 discriminator sits here).
- Or-pattern grammar: `32-grammar.md §4:345` (`pattern "|" pattern -- or-pattern
  (same binders)`), precedence `:354-358` (`as` tighter than `|`; flat
  alternative list; constructor-argument parenthesization). Semantics rule:
  `34-data-match.md §3.1:372-382`.
- No or-pattern handling exists anywhere in parser, resolver, or elaborator
  today. Confirm with `git grep` at your cut.

## Deliverables

**`D1` -- surface + AST.** The parser accepts a `|` chain as one flat
`PatKind::Or(Vec<Pattern>)` (`n >= 2`), with `as` binding tighter than `|` and an
or-pattern parenthesized when used as a constructor argument. Resolve threads it
to `RPatKind::Or(...)`, applying the **binder-set-identity** check: every
alternative binds the identical name set, each name once; a mismatch or an
intra-alternative duplicate is a surface error at its span.

**`D2` -- elaboration by alternation over the same occurrence.** `p₁ | … | pₙ`
matches the arm's existing scrutinee occurrence iff some `pᵢ` matches; it
duplicates the residual arm under each alternative, mapping each alternative's
binders by name into the first alternative's canonical slots, threaded through
the landed occurrence capability. No new carrier, no new projection. Report the
discriminating elaboration point at `file:line`.

**`D3` -- binder-type equality in the common pre-branch context.** For each
shared binder name, its type under each alternative is checked **definitionally
equal in the pre-branch context**. The check is conservative: a dependent
or-pattern whose binder types coincide only after alternative-specific
refinement is REJECTED with a "use separate arms" diagnostic (`34 §3.1:379-382`)
-- that rejection is correct behavior, not an error to work around.

**`D4` -- coverage/reachability, union and per-alternative.** An or-arm's
coverage is the union of its alternatives' coverage; the arm is reachable iff at
least one alternative has a non-empty residual. The exhaustiveness/reachability
sweeps descend into the alternatives and combine by union.

**`D5` -- the composition discriminator (NoInhabitants honesty), both parts.**
(i) A fully-subsumed or-arm is flagged with the subsumption/redundancy cause,
**never** `NoInhabitants`. (ii) A partially-dead or-pattern (some but not all
alternatives subsumed) stays **reachable**; only the individual redundant
alternative may be reported, never the whole arm as dead. This is the
enclave-required discriminator for this slice.

**`D6` -- fail-closed for the unlanded forms.** Guards and literals remain absent
and refused. A wildcard/variable **at top level** stays refused -- including when
it appears as an **alternative** of a top-level or-pattern (point 7).

## Acceptance criteria

**`AC-1`.** A top-level or-pattern of positive alternatives (e.g. `Inl x | Inr
x`) elaborates: it matches iff some alternative matches, and the shared binder is
in scope in the arm body at the canonical slot. **Control:** assert structurally
that the body sees the shared binding regardless of which alternative matched
(both alternatives feed the same slot) -- not "it compiles". Include a
three-alternative case to exercise the flat chain.

**`AC-2` -- the composition discriminator, both parts, asserted.** (i) A
fully-subsumed or-arm (all alternatives covered by earlier arms, e.g. `A | A`, or
`A | B` after `A` and `B` are each covered) is flagged redundant with the
**subsumption** `ArmDeadCause`, and the test asserts the cause is subsumption,
**NOT** `NoInhabitants`. (ii) A partially-dead or-pattern (one alternative
subsumed, one live) is asserted **reachable** -- the whole arm is not reported
dead. A test that lets a fully-subsumed or-arm read as `NoInhabitants`, or a
partially-dead one read as dead, is a defect against this slice.

**`AC-3` -- binder-set identity, asserted.** Mismatched binder sets across
alternatives (`Inl x | Inr y`) and an intra-alternative duplicate name are each
rejected at the surface with the specific diagnostic (control: assert the
diagnostic, not `is_err()`; assert neither is silently accepted with one binder
dropped).

**`AC-4` -- binder-type equality and its conservatism, asserted.** Corresponding
binders with definitionally-equal types in the pre-branch context are accepted
(control: a `(Inl x | Inr x)` over a type where both payloads share a type
checks, and the body uses `x` at that type). A dependent or-pattern whose binder
types would agree only after alternative-specific refinement is **rejected** with
the "use separate arms" diagnostic (control: assert that specific diagnostic --
the conservative rejection is the specified behavior).

**`AC-5` -- alternation, not a new carrier; precedence.** The elaborated form
duplicates the residual over the same occurrence (no new projection/carrier
introduced); assert on the elaborated term shape. Parse precedence is asserted:
`p as x | q` parses as `(p as x) | q`; an or-pattern as a constructor argument
requires parentheses.

**`AC-6` -- scope integrity.** `PatKind::Or` / `RPatKind::Or` added; the
top-level `Wild`/`Var` refusal is lifted to see through an or-pattern's
alternatives (a `Wild`/`Var` alternative at top level stays refused) but is
otherwise **unchanged**; NO change to guard or literal forms -- they remain
absent and fail-closed; the landed as/tuple/record forms are unchanged. If
landing or-patterns turns out to require touching guard/literal handling, that is
the AC-stop finding, not a widening.

**`AC-7` -- no-regression in CI (`COORDINATION §12`).** Targeted locally
`-p ken-elaborator`; never `--workspace`.

## Not this slice

- **Not guards / literals** -- slices 5, 6, each its own node, each fail-closed
  until it lands.
- **Not top-level wildcard/variable acceptance, and not the `ArmDeadCause` third
  cause.** Those attach to whichever slice lands a top-level non-constructor
  catch-all; this slice does not, and it keeps `NoInhabitants` honest by refusing
  a `Wild`/`Var` alternative rather than accepting it.
- **Not the refined-context join** for dependent or-patterns whose binder types
  agree only after alternative-specific refinement. The spec (`34 §3.1:381-382`)
  defers that to "a later rule"; this slice implements the conservative
  pre-branch check and rejects such patterns with the "use separate arms"
  diagnostic. Do not build the join here.
- **Not the reachability slice-indexing hygiene residual** the umbrella carries
  (the `get().expect(<invariant>)` convention at the `mark_shared_ctor_subsumption`
  / matrix-descent sites). Fold it in ONLY if this implementation genuinely
  touches those sites; otherwise leave it and it stays on the umbrella.
- **Not an amendment to `34 §3`.** The enclave ruled the chapter's obligations
  real and present-tense.

## Contention

Shares `crates/ken-elaborator` with lane-2 work. Slices 1-3
(`LANG-MATCH-AS-PATTERN`, `LANG-MATCH-TUPLE-PATTERN`, `LANG-MATCH-RECORD-PATTERN`)
are merged; no live lane-2 `elab.rs` contention at release. Foundation's Tier-C
P3 (`CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION`) also builds under `-p
ken-elaborator` but touches `catalog/` + the roots-loader inventory, disjoint
from this slice's pattern AST / resolve / match-elaboration source. The
lieutenant publishes serially; the second candidate rebases. Re-check at release.

## Sizing and tier

**`M`.** A new `PatKind`/`RPatKind` variant with a flat alternative list, the
resolve-time binder-set-identity check, common-pre-branch-context binder-type
equality with the spec's deliberate conservatism, residual-duplication over the
landed occurrence capability, union coverage/reachability, and the two-part
`NoInhabitants` composition discriminator. Comparable to the record slice, with
alternation and binder-agreement replacing label-keyed projection. **Hard stop
and finding:** if duplicating the residual arm under the alternatives cannot be
expressed over the general match matrix without new matrix-descent plumbing
(rather than reusing the landed occurrence threading), surface that -- do not
build around it.

**Tier `T1`.** Binder-set identity, common-context binder-type equality with the
conservative refinement rule, union coverage, and the `NoInhabitants` honesty
discriminator are soundness-adjacent correctness arguments, not mechanical edits.
The T1 language seat fits; no reseat.
