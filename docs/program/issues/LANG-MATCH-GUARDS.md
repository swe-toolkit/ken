---
id: LANG-MATCH-GUARDS
title: "match guards `p if g` -- slice 5 of 34 §3's six absent pattern forms: an ARM-SELECTION refinement, not a pattern form and not a type refinement (34 §3.3:623-626), atomic with the coverage exception (a guarded arm does NOT discharge its constructor for exhaustiveness, 34 §4.1:882) and the reachability exception (a guarded arm does NOT cover, so a later unguarded arm for the same constructor stays reachable, 34 §4.2:902-904); the composition discriminator this slice owes is TWO-SIDED -- guards loosen coverage (a guard that would otherwise complete exhaustiveness makes the match non-exhaustive) and loosen reachability (a guard on an earlier arm keeps a later same-constructor unguarded arm reachable), the guard load-bearing to each verdict"
status: active
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS]
blocks: []
github: null
origin: "Steward cut 2026-09-06 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn (the prerequisite-ordered six-slice cut). Slices 1-4 landed: as-patterns (LANG-MATCH-AS-PATTERN, e6645d7c2), tuple/pair (LANG-MATCH-TUPLE-PATTERN, af2b36dc8/a8ee55b2f), record (LANG-MATCH-RECORD-PATTERN, a42b90454/760e12fd0), or-patterns (LANG-MATCH-OR-PATTERN, 7f80228c2/closed 440b16dfc, blob-verified 7/7). This is slice 5, guards, next in the enclave's order; slice 6 (literals) stays BLOCKED on the enclave's DecEq/expected-type finding. The enclave named NO prerequisite pin for guards and none is owed (SPEC-MATCH-PATTERN-PINS, 34fd01c1, discharged the pin column and 34 §3.3/§4 already carry the full guard semantics). Guards do NOT depend on literals (enclave, explicit) and are semantically orthogonal to the four landed pattern forms -- they attach to any already-elaborating arm; the dependency on slice 4 is contention (one serial language seat, shared elab.rs/parser.rs), not semantics. The umbrella stays draft and flips its guards row when this lands. Anchors measured by the Steward at origin/main 440b16dfc before framing."
---

> # RELEASED 2026-09-06 to the language ring (lane-2), slice 5 of the enclave's
> # six-slice cut (evt_12qrtnp7237dn). depends_on satisfied: SPEC-MATCH-PATTERN-
> # PINS merged (34fd01c1). No guard pin was named or owed -- 34 §3.3/§4 carry
> # the whole guard contract normatively. Immediate predecessor slice 4
> # (LANG-MATCH-OR-PATTERN) is merged (7f80228c2 / closed 440b16dfc); this slice
> # shares the match spine but adds NO pattern form -- a guard is an arm-level
> # `if g` on an already-elaborating pattern. Base = current origin/main
> # 440b16dfc (RE-MEASURE every anchor at your cut -- the or-pattern landing
> # advanced the elab.rs arm_used / NoInhabitants / top-level-refusal lines once
> # already; escalate a false fixed input rather than building around it). The
> # Architect is the required reviewer alongside language-qa + CI: the coverage
> # exception, the reachability exception, and the two-sided composition
> # discriminator are a correctness argument about the exhaustiveness/reachability
> # checker, not a mechanical edit.

## What this is

The **guard** form, `p if g` -- slice 5 of the six absent forms in
`spec/30-surface/34-data-match.md §3` (form list at `:339-340`, "... and
optional **guards**"; the contract at `§3.3:623-626`, `§4.1:882`,
`§4.2:902-904`). A guard is **not a pattern form and adds no carrier**: it is an
**arm-selection refinement** -- a boolean expression evaluated *after* the arm's
pattern matches, gating whether this arm's body is taken or control falls
through to the next arm.

It differs from every prior slice on the axis that is the whole content of this
node: **a guard loosens the exhaustiveness and reachability contracts.** Three
coupled rules, all normative in `34`:

1. **Guards do not cover (`§4.1:882`, `§3.3:625-626`).** *"A guarded arm
   `Cₖ p̄ if g => e` [...] the guard `g` may fail, [so] the arm does not by
   itself discharge the `cₖ` constructor for exhaustiveness."* Exhaustiveness is
   computed over **unguarded arms only** -- `every value ... matches some
   (unguarded, §3.3) arm`. ⇒ A match whose only arm for some constructor is
   guarded is **non-exhaustive** and must be diagnosed as a missing case.
2. **Guards do not shadow (`§4.2:902-904`).** *"A guarded arm is not counted as
   covering its constructor, so a later unguarded arm for the same constructor
   is reachable (it catches the guard-failure cases) -- guards loosen
   reachability exactly as they loosen coverage."* ⇒ A later unguarded arm for a
   constructor already handled by an **earlier guarded** arm is **reachable**,
   not subsumed.
3. **Guards do not refine (`§3.3:623`, "Guards do not refine ...").** The guard
   is typechecked at `Bool` in the arm's branch context -- the pattern's binders
   are in scope and the pattern's definitional refinement (`§3.3:611`) is in
   effect, so the guard may mention binders and use their refined types -- but
   the guard's success contributes **no additional definitional equality** to
   the body. The branch hypothesis comes from the pattern; the guard is a
   runtime gate layered on top of it.

## The composition discriminator this slice owes

The enclave's rule (`evt_12qrtnp7237dn`, mirrored on the umbrella): each wrapper
form may start over the baseline, but **every later form must add a composition
discriminator** -- a control proving the new form is load-bearing to a
reachability/coverage verdict that the prior baseline gets wrong. Slice 2
discharged the `NoInhabitants` split for a redundant tuple arm; slice 3 the
open-record subset case; slice 4 the two-part or-arm subsumption. **The guard
discriminator is two-sided, and it is the enclave's own "both the coverage and
the reachability obligations become live within it":**

- **Coverage side.** A guard that would otherwise complete exhaustiveness must
  make the match **non-exhaustive**. The verdict must flip on the guard's
  presence: `match b:Bool { True if g => x; False => y }` reports `True`
  missing; the identical match with the guard removed (`True => x; False => y`)
  is exhaustive. The guard is load-bearing to the missing-case diagnostic.
- **Reachability side.** A guard on an earlier arm must keep a later
  same-constructor unguarded arm **reachable**: `match b { True if g => x;
  True => y; False => z }` reports `True => y` reachable; the identical shape
  with the first arm's guard removed (`True => x; True => y; ...`) reports
  `True => y` **subsumed**. The guard flips the verdict the other way.

Both controls are two-sided (guard present vs absent flips the verdict), which
is what makes them a discriminator rather than a smoke test. Neither is
expressible today -- `MatchArm` has no guard field, so the reachability checker
`arm_used` has never had a `§3.3` exception to exercise.

## Deliverables

- **D1 -- AST.** Add `guard: Option<Expr>` to `MatchArm`
  (`crates/ken-elaborator/src/ast.rs:86`). `None` is exactly today's behaviour;
  the lossless/round-trip surface (`lossless.rs`) carries the guard where the
  parser produced one.
- **D2 -- Parser.** In the match-arm parse (`parser.rs`, `parse_match_expr` at
  `:2336`, arm assembly near `:2825-2845`) accept an optional `if <expr>`
  between the pattern and `=>`, populating `MatchArm.guard`. The guard is a full
  expression terminated by `=>`. Precedence per `34 §3` / `32 §4`: the guard
  sits at arm level, after the whole pattern (including any `as`/`|`); it does
  not bind into the pattern.
- **D3 -- Elaboration / lowering.** Elaborate the guard at `Bool` in the arm's
  refined branch context (binders in scope, pattern refinement in effect, no new
  refinement contributed -- rule 3). Lower it to a runtime conditional inside the
  compiled matcher (`§3.1`, the `cₖ` method): guard true -> this arm's body;
  guard false -> fall through to the next arm exactly as if the pattern had not
  matched. No change to unguarded arms' lowering.
- **D4 -- Coverage exception.** The exhaustiveness checker (`§4.1`) counts a
  guarded arm as **not discharging** its constructor. A construct that is
  exhaustive only by virtue of a guarded arm is reported non-exhaustive with the
  missing case named.
- **D5 -- Reachability exception.** `arm_used` (`elab.rs:3707`/`:3868`,
  `:5626`/`:5819`, and the matrix path `:12742`/`:12977` -- re-measure) treats a
  guarded arm as **not covering**, so a later unguarded arm for the same
  constructor is marked used/reachable. A guarded arm itself is reachable
  whenever its pattern is (the checker does not evaluate `g`).

## Acceptance criteria

- **AC-1 (syntax round-trips).** `p if g => e` parses, elaborates, and
  round-trips losslessly; an arm with no guard is byte-identical in behaviour to
  today (`guard: None`).
- **AC-2 (guard runs, non-refining).** A guarded constructor arm runs its body
  only when `g` is true and falls through to the next arm when false;
  `ken run` differential over a program that exercises both branches of `g`.
  The guard may reference pattern binders; it adds no definitional equality
  (a body term whose type would need the guard's truth does NOT typecheck).
- **AC-3 (coverage discriminator, TWO-SIDED).** `match b:Bool { True if g => x;
  False => y }` is reported **non-exhaustive** (missing `True`); the same match
  with the guard removed is **exhaustive**. The diagnostic names `True`. Removing
  D4 must red exactly this (control: with the coverage exception disabled the
  guarded match is wrongly accepted as exhaustive).
- **AC-4 (reachability discriminator, TWO-SIDED).** `match b { True if g => x;
  True => y; False => z }` reports `True => y` **reachable**; the same shape with
  the first arm unguarded reports `True => y` **subsumed** (`Subsumed`, with the
  earlier arm as claimant -- never `NoInhabitants`). Removing D5 must red exactly
  this (control: with the reachability exception disabled the later unguarded arm
  is wrongly flagged unreachable).
- **AC-5 (fully-guarded is non-exhaustive).** A match over every constructor of
  a sum where **every** arm is guarded is non-exhaustive (no arm discharges), and
  adding one unguarded fallback makes it exhaustive.
- **AC-6 (NoInhabitants stays honest).** No new path lets a guarded arm reach the
  `NoInhabitants` classifier: the coverage/reachability exceptions route through
  `Subsumed`/reachable, never through the empty-type cause. The umbrella's
  standing obligation -- do not widen `Subsumed` to an empty winner set, do not
  make `NoInhabitants` false -- holds unchanged.
- **AC-7 (remainder fail-closed).** Guard syntax on a position that does not
  elaborate today is rejected, not silently accepted (see Not-this-slice).

## Not this slice

- **Top-level `_`/`Var` refusal is UNCHANGED.** A guard does not lift it:
  `_ if g => e` and `x if g => e` at **top level** stay refused, because the
  pattern is still a top-level wildcard/variable. Lifting that refusal drags in
  the umbrella's two-obligation block (a third `ArmDeadCause` for "dead because
  earlier arms cover it, no single claiming constructor", plus the
  `NoInhabitants` dependency clause) and is a **separate** future obligation.
  Guards here attach to already-elaborating arms (constructor, tuple, record,
  as-pattern, or-pattern). This keeps `NoInhabitants` honest (AC-6) and the slice
  atomic. Rejecting a top-level guarded wildcard is correct fail-closed
  behaviour, not a gap this slice owes.
- **Literals (slice 6) are not unblocked by this.** Guards do not depend on
  literals and landing guards does not touch the enclave's DecEq/expected-type
  blocker.
- **No exhaustiveness "the guard is always true" analysis.** The checker does
  not evaluate or reason about `g`; a guarded arm never discharges, even if `g`
  is syntactically `True`. Treating a trivially-true guard as covering is out of
  scope (and would re-introduce the coverage unsoundness this slice exists to
  prevent).

## Contention

One serial language seat. This slice shares `elab.rs` (the `arm_used` /
exhaustiveness machinery), `parser.rs` (the match-arm parse) and `ast.rs`
(`MatchArm`) with the just-landed slice 4 -- which is why it is cut
one-release-ahead **after** slice 4, not concurrently. Re-measure all anchors at
the cut SHA. No other lane touches these files.

## Sizing / tier

Size **M**, tier **T1**. The edit surface is moderate (one AST field, one parser
branch, one lowering conditional, two checker exceptions), but the content is the
correctness argument that the coverage and reachability exceptions are *exactly*
`§4.1`/`§4.2` -- loosen in both directions, and only for guarded arms -- with the
two-sided discriminator proving the guard is load-bearing. That is reasoning
work, not a mechanical port. One releasable increment or a genuine hard stop is
reachable in a turn; a hard stop escalates to the Steward + Architect.
