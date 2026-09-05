---
id: LANG-MATCH-AS-PATTERN
title: "as-patterns `p as x` -- the first contained slice of 34 §3's six absent pattern forms: the alias binds the value matched by p at its position, the inner p obeys the current position rules so the top-level non-constructor refusal stays fail-closed (keeping NoInhabitants honest), and the P1 association/precedence pin is honored forward-compatibly against the not-yet-existing or-pattern"
status: active
owner: language
size: S
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS]
blocks: []
github: null
origin: "Steward cut 2026-09-05 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn ('release a first contained as-pattern slice rather than a six-form frame'). SPEC-MATCH-PATTERN-PINS merged at 34fd01c1 discharged the whole prerequisite pin column, so P1 (`p as x` association/precedence) is landed and this slice is cuttable. First of six slices; the umbrella stays draft and flips its as-pattern row when this lands. Anchors re-measured by the Steward at main 64e77119c before framing."
---

> # RELEASED 2026-09-05 to the language ring (lane-2) — the next lane-2 WP.
> # LANG-BYTES-HEX-LIST-LITERAL merged, and LANG-MEMBERSHIP-OPERATOR-SURFACE
> # hard-stopped and is HELD pending an Architect decomposition, so this
> # contained slice is the live lane-2 deliverable. Base = current main
> # 76c87a74f. SPEC-MATCH-PATTERN-PINS is merged (P1 as-association/precedence
> # landed), so the slice is cuttable. The elaborator pattern anchors are
> # unchanged since the frame's measurement (PatKind ast.rs:167, top-level
> # non-constructor refusal elab.rs:12325, RPatKind resolve.rs:54) — re-measure
> # at cut and escalate a false fixed input rather than building around it. The
> # load-bearing design line: the inner p keeps the current position rules so
> # the top-level non-constructor refusal STAYS fail-closed (keeps NoInhabitants
> # honest, does not reopen LANG-REACHABILITY-SUBSUMING-ARMS).

## What this is

The **as-pattern** form, `p as x` -- slice 1 of the six absent forms in
`spec/30-surface/34-data-match.md §3` (`§3.1` bullet 5). It is a **wrapper**
form: it matches with `p` and additionally binds `x` to the value matched by
`p`. It does not split the scrutinee and adds no coverage of its own.

This is a standalone slice, not the umbrella. Per the enclave disposition, the
umbrella `LANG-MATCH-PATTERN-FORMS-ABSENT` stays `draft` and **must never become
a six-form frame**; it flips its as-pattern row when this node lands. The other
five forms (tuple, record, or, guards, literals) are later slices, each its own
node, **each fail-closed until it lands**.

## The design judgment, front-loaded

**1. What an as-pattern means.** `p as x` matches exactly what `p` matches and
binds `x`, in the arm body's context, to the value that `p` matched at its
position. Coverage and reachability of `p as x` are **exactly `p`'s** -- the
alias is a binding, not a matcher. The expected elaboration is: strip the alias,
recurse on `p` for the actual match, and add the binding `x = <value at p's
position>` to the body context.

**2. The as-pattern does NOT widen where a non-constructor pattern is
accepted (WP-cut, load-bearing -- see the next section).** The inner `p` obeys
the SAME position rules the elaborator enforces today. At top level that means
`p` must be a constructor pattern, so `(C p̄) as x` at top level elaborates,
while `_ as x` and `y as x` **as a top-level arm stay refused**, exactly as bare
`_` / `y` are refused today. Under a constructor, `p` follows the current
sub-pattern rules. This keeps the slice contained and keeps the reachability
classifier honest.

**3. An arm containing an as-pattern uses the general match-compilation path.**
An as-pattern is neither a flat `Var` nor `Wild`, so it does not fit the
single-shape "checked" fast path (which permits only flat `Var`/`Wild`
sub-patterns -- see the doc at `elab.rs:12446`). Arms carrying an as-pattern
route to the general matrix path, which already handles nested sub-patterns.
That is correctness over the fast path and is expected, not a defect.

**4. A new AST / RPatKind variant is correct here, and is NOT the red flag it
was for the lexer slice.** As-patterns are a genuinely new pattern form; add
`PatKind::As` and `RPatKind::As` (the shape the tree prefers, e.g.
`As(Box<Pattern>, String)`). Contrast `LANG-BYTES-HEX-LIST-LITERAL`, where a new
token would have signaled a wrong reading -- there the value already existed;
here the form does not.

**5. The P1 pin is landed and must be honored forward-compatibly.**
`SPEC-MATCH-PATTERN-PINS` (merged, `34fd01c1`) settled `p as x` association and
precedence in `32 §4`: constructor application binds tighter than `as`; `as`
binds tighter than `|`; `as` is non-associative; and an `as`-pattern (or
or-pattern) used as a constructor argument must be parenthesized. Implement that
precedence NOW, even though `|` (or-patterns, slice 4) does not exist yet, so the
or-pattern slice needs no re-parse. This is what "each wrapper form may start
over the current baseline" buys only if the wrapper reserves its precedence.

## Why the top-level refusal STAYS -- the NoInhabitants dependency

This is the reason point 2 is a hard scope line and not a convenience.

The reachability classifier reads `None => ArmDeadCause::NoInhabitants` (at
`elab.rs:3798`, `:5749`, `:12375`). `subsumed_by` is populated only for an arm
whose pattern resolves to a claiming constructor, so `None` -- and therefore
`NoInhabitants` -- covers every arm that is not such a pattern. **`NoInhabitants`
is honest today only BECAUSE top-level wildcard/variable do not elaborate**
(the refusal at `elab.rs:12323`). The moment a slice accepts a top-level
non-constructor catch-all, the most ordinary redundant program there is -- a
trailing catch-all after exhaustive arms -- lands in `None` and is told it has
no inhabitants, which is a **false diagnostic**, strictly worse than the gap it
would close.

An as-pattern over a top-level non-constructor (`_ as x`, `y as x`) is exactly
such a catch-all. **This slice refuses it (fail-closed).** The top-level
wildcard/variable form and its `ArmDeadCause` third-cause obligation (a new cause
for "dead because earlier arms cover it, with no single claiming constructor" --
and NOT by widening `Subsumed` to an empty winner set) belong to their own
future slice, per the umbrella. This slice does **not** reopen
`LANG-REACHABILITY-SUBSUMING-ARMS`.

## Fixed inputs (measured by the Steward at main 64e77119c; re-measure at release)

- `PatKind`: `ast.rs:167`, three arms `{Wild, Var, Ctor}`, no `As`. `MatchArm`:
  `ast.rs:86` (no guard field). `Pattern`: `ast.rs:160`.
- `RPatKind`: `resolve.rs:54`, three arms `{Wild, Var, Ctor}`, no `As`;
  `RMatchArm` immediately below it.
- Top-level non-constructor refusal: `elab.rs:12319-12326`, guard at `:12323`
  (`if let RPatKind::Wild | RPatKind::Var(_) = arm.pat.kind`), message
  "non-constructor pattern in match (wildcard/var not yet supported at top
  level; use constructor patterns)". **Preserve this for top-level
  non-constructor inners.**
- Reachability classifier `None => ArmDeadCause::NoInhabitants`: `elab.rs:3798`,
  `:5749`, `:12375`; `mark_shared_ctor_subsumption` at `:6928`; `ArmDeadCause`
  enum at `error.rs:64`.
- No `as` / as-pattern handling exists anywhere in the elaborator, parser, or
  resolver today (`git grep` for `As`/`as_pattern` is empty of pattern uses).

## Deliverables

**`D1` -- surface + AST.** The parser accepts `p as x` with the P1 precedence
(looser than constructor application, tighter than `|`, non-associative,
parenthesize-when-used-as-a-constructor-argument). Add `PatKind::As`; `resolve`
threads it to `RPatKind::As`.

**`D2` -- elaboration.** `p as x` matches via `p` and binds `x` to the value
matched at `p`'s position, in the arm body's context. Report the discriminating
elaboration point at `file:line`.

**`D3` -- coverage/reachability by delegation.** `p as x` has exactly `p`'s
coverage and reachability; the alias is stripped for the exhaustiveness and
reachability sweeps.

**`D4` -- binder-collision rejection.** An as-pattern whose alias `x` collides
with a binder inside its own `p` (or otherwise already bound in the arm) is
rejected with a message naming the collision.

**`D5` -- fail-closed top-level.** An as-pattern whose inner is a top-level
non-constructor (`_ as x`, `y as x` as a top-level arm) is refused with a
diagnostic (the existing top-level-non-constructor message, or one naming the
as-pattern), never silently accepted.

## Acceptance criteria

**`AC-1`.** `(C p̄) as x` at top level elaborates: `x` is bound to the whole
scrutinee and `p̄` match as before. **Control:** assert structurally that the arm
body sees `x` bound to the matched value, not "it compiles".

**`AC-2` -- coverage/reachability preserved, and specifically not
`NoInhabitants`.** A match whose arms use as-patterns produces identical
exhaustiveness and reachability verdicts to the same match with the aliases
removed. **Control:** a redundant `(C ..) as x` after an arm already covering `C`
is flagged redundant with the SAME `ArmDeadCause` it gets without the alias --
and the test asserts that cause is the subsumption cause, **not**
`NoInhabitants`. This is the honesty guard, asserted, not assumed.

**`AC-3` -- binder collision rejected.** An alias colliding with an inner binder
(e.g. `(MkWrap y) as y`) is rejected; assert on the message text, not
`is_err()`.

**`AC-4` -- top-level non-constructor inner stays refused.** `_ as x` and
`y as x` as a top-level arm are refused (fail-closed); assert on the message. A
test that ACCEPTS either is a defect against this slice, not a feature -- it is
the NoInhabitants dependency above.

**`AC-5` -- P1 precedence, the part testable now.** `as` is non-associative
(`p as x as y` is a parse error or requires parens) and binds looser than
constructor application (`C a as x` parses as `(C a) as x` is wrong -- P1 says
constructor application binds tighter, so `C a as x` is `C (a) ... as x` only
with parens; assert the parenthesization rule that an as-pattern used as a
constructor argument requires parens). The `as`/`|` interaction is pinned by P1
and is exercised by the or-pattern slice (slice 4), since `|` does not lex yet;
state in the handback that the precedence is reserved so or-patterns need no
re-parse.

**`AC-6` -- scope integrity.** No new `Token` variant; `PatKind::As` /
`RPatKind::As` added; NO change to tuple, record, or, guard, or literal forms --
they remain absent and fail-closed. If landing as-patterns turns out to require
touching any of those, that is the AC-stop finding, not a widening.

**`AC-7` -- no-regression in CI (`COORDINATION §12`).** Targeted locally
`-p ken-elaborator`; never `--workspace`.

## Not this slice

- **Not top-level wildcard/variable acceptance, and not the `ArmDeadCause` third
  cause.** Those attach to whichever slice lands a top-level non-constructor
  pattern; this slice deliberately does not, keeping `NoInhabitants` honest
  (umbrella, "the consequence" section).
- **Not the reachability slice-indexing hygiene residual** -- the
  `mark_shared_ctor_subsumption` / matrix-descent `get().expect(<invariant>)`
  convention the umbrella carries (its old `:8182`/`:8183` sites have drifted to
  `~elab.rs:6941` and `~:12007`). That is a separate reachability-hygiene item on
  the umbrella. Fold it in ONLY if this implementation genuinely touches those
  sites; otherwise leave it untouched.
- **Not tuple / record / or-pattern / guards / literals** -- slices 2 through 6,
  each its own node, each fail-closed until it lands.
- **Not an amendment to `34 §3`.** The enclave ruled the chapter's obligations
  real and present-tense; the "drop a form from the surface" branch is closed.

## Contention

Shares `crates/ken-elaborator` with lane-2 work. The three nodes the umbrella
named as an in-line `elab.rs` queue (`LANG-REACHABILITY-SUBSUMING-ARMS`,
`LANG-WITNESS-DIAGNOSTIC-STRICTNESS`, `LANG-FOREIGN-CTOR-ARM-REJECT`) are all
`merged`. The only live lane-2 contention is the current WP
`LANG-BYTES-HEX-LIST-LITERAL` (landing), whose file region (the lexer) is
disjoint from this slice's (pattern AST / resolve / match elaboration), and it
lands before this releases. Re-check at release.

## Sizing and tier

**`S`.** One wrapper form: a new `PatKind`/`RPatKind` variant, alias-binding
elaboration, coverage/reachability by delegation to `p`, one collision check,
one preserved fail-closed guard. **Hard stop and finding, re-sizing to `M`:** if
the aliased value cannot be carried to the arm body without threading it across
the general match-matrix descent (rather than a straightforward arm-body
binding), surface that -- do not build around it.

**Tier `T1`.** Coverage/reachability preservation and binder scoping are
soundness-adjacent, and the fail-closed `NoInhabitants` dependency is a
correctness argument rather than a mechanical edit. The deliberate T1 language
seat fits; no reseat.
