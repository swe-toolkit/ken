---
id: LANG-MATCH-MATRIX-OCCURRENCE-THREADING
title: "the match compiler drops each column's matched value before the arm-body leaf, so no value-binding pattern form (as-pattern, and later literal/nested-binder positions) can bind the value it matched -- thread a per-position OCCURRENCE (the core term for that column's value, re-indexed at each descent depth) through RowState and the matrix descent, reusing the existing elim_D field binders and the proven under()/scrut_occurs weakening; no new core construct"
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [LANG-MATCH-AS-PATTERN, LANG-MATCH-PATTERN-FORMS-ABSENT]
github: null
origin: "Architect component-design ruling evt_4e73y2bawf4gz (2026-09-05), option (a), on the LANG-MATCH-AS-PATTERN hard stop (language-implementer evt_kybs4q6dm6z4, framed by language-leader evt_7qgfbd4vz3thb): the general match compiler carries only real_pats + arm_idx to the leaf and loses each column's matched value at the constructor split, so an alias RVar (or any value-binding position) has no value term to bind. The Architect ruled this ONE capability -- per-position occurrence terms threaded through the match matrix -- subsumes EVERY value-binding form in spec 34 §3 (as-patterns first, then literal positions and nested binders), so it is the predecessor for the whole LANG-MATCH-PATTERN-FORMS-ABSENT program, not one slice. Steward-cut per COORDINATION §2."
---

> # RELEASED 2026-09-05 to the language ring (lane-2). The predecessor capability
> # the Architect's as-pattern ruling (evt_4e73y2bawf4gz) named. Base = current
> # main e44f4ac21 (re-measure at cut; main may advance a commit or two as the EC
> # catalog respin + the BYTES-HEX-CONTIGUOUS spec pin land -- NEITHER touches
> # crates/ken-elaborator/src, so no overlap with this WP; the lieutenant rebases
> # at merge). The as-pattern consumer slice (LANG-MATCH-AS-PATTERN) is HELD on
> # this node and re-releases as its consumer once this lands.

## What this is, and why it is one capability, not a per-form fix

The as-pattern slice hard-stopped on a real structural gap the Architect
confirmed by grounding from main: the emitted match compilation is nested
`elim_D` (34 §3.1, normative), and the general compiler's `RowState`
(`elab.rs:11923`) carries only `real_pats` + `arm_idx`. At the constructor
split, `build_ctor_buckets` (`elab.rs:12189`) forwards only subpatterns +
`arm_idx` (`:12216-17` / `:12231-32`); the matched column value is gone before
the sole arm-body leaf (`compile_match_matrix`, `elab.rs:12013`), which sees
only flat `Var`/`Wild` columns. So a value-binding position -- an as-pattern's
alias, and later a literal position or a nested binder -- has no value term to
bind.

**The value is NOT actually absent from the core.** Each `elim_D` method already
binds the constructor's freshly-bound fields as core de Bruijn binders -- the
"split column woven in via weaken-then-wrap, never a real push" at
`elab.rs:12136-38`. **"Never a real push" is a SURFACE-de-Bruijn bookkeeping
fact** (`RowState`/`resolve` never counted the field), **not a claim that the
core lacks the value.** The value at any position is a field binder the
eliminator introduces; what is missing is a handle to it at the leaf.

**The codebase already has the occurrence concept and its correctness
discipline:** `scrut_core` + `scrut_occurs` with per-binder weakening
`under(scrut_core)` (`elab.rs:1715-1749`) drive dependent-match motive rebasing
-- exactly "the core term for the matched value, re-indexed as descent goes
under binders." Today it is used for the TOP-LEVEL scrutinee only. **This
capability generalizes it to a per-position occurrence threaded through the
matrix.** Reuse the proven machinery; do not invent a parallel one.

## The mechanism the Architect ruled: option (a), NOT (b), NOT the shortcut

**Thread a per-position OCCURRENCE -- the core term denoting that column's value
at the current descent depth -- through `RowState` and the matrix descent.** The
occurrence MUST be re-indexed (weakened) at each nested depth.

- **A top-level-only `scrut_core` shortcut is WRONG, not merely incomplete.** It
  is a silent partial (nested positions -- the normatively-required nested
  as-patterns of 34 §3.1 -- unsupported) AND de-Bruijn-UNSOUND at depth: an
  un-rebased outer term corrupts the goal, the known convoy/telescope de Bruijn
  hazard. So the mechanism must construct per-position occurrences with correct
  weakening.
- **Option (b) ("keep consumed values available in the split context") is
  rejected.** It is the same underlying need but leaves the per-depth rebasing
  implicit and invites exactly that top-level-only partial. Threading the
  occurrence explicitly through `RowState` (a) makes the rebasing REUSE the
  proven `under()`/`scrut_occurs` discipline at each split.
- **No new core `Term`/IR variant, and no value crosses a function-return.** The
  occurrence is an existing `elim_D` field binder (or `scrut_core`), constructed
  in-elaborator; the emitted term is still nested `elim_D`, kernel-re-checked,
  `trusted_base` unchanged. This is WITHIN-LANE / option-(a). A new
  projection/accessor core term would be option-(b)/TCB and is unnecessary --
  **reject it, and see the hard stop below.**

## Deliverables (Architect D-decomposition, verbatim in substance)

**`D1` -- `RowState` gains a per-column occurrence.** Per column, the core term
for that position's value at the current descent depth.

**`D2` -- `infer_match` seeds the top-level occurrences** from the scrutinee it
already holds (`scrut_core`, `elab.rs:12284+`).

**`D3` -- `build_ctor_buckets` threads occurrences across the split.** At each
constructor split, set each field subpattern's occurrence to the fresh `elim_D`
field binder that method already introduces (`elab.rs:12136-38`), and WEAKEN
every still-live tail/IH occurrence in lockstep as descent goes under those
binders -- reusing the exact `under()`/`scrut_occurs` weakening already proven
for motive rebasing (`elab.rs:1715-1749`).

**`D4` -- the leaf supplies occurrences to binding positions.** At
`compile_match_matrix`'s leaf (`elab.rs:12013`), each binding position is given
its column's occurrence. Plain `Var` columns bind the same way (behavior-
identical); the occurrences are now available for the as-pattern/literal/nested
consumers that follow.

## Acceptance criteria

**`AC-OCCURRENCE-CORRECT`** (white-box). At the leaf, each column's occurrence is
the core term denoting that position's value at the current depth, asserted
structurally -- INCLUDING at a nested constructor position, where the occurrence
is that depth's `elim_D` field binder correctly weakened. Not "it compiles".

**`AC-WEAKEN-AT-DEPTH`** (the soundness control). A test exercising a two-level
constructor split shows tail/IH occurrences re-indexed correctly under the
intervening field binders. **This is the convoy/telescope de Bruijn hazard the
Architect named:** an un-rebased occurrence corrupts the goal, so a control that
does not go under at least two levels of binder cannot show the weakening is
right.

**`AC-VAR-PRESERVED`** (no-regression, behavior-identical). Existing `Var`/`Wild`
binding, nested `match`, and dependent-match motive rebasing (via `scrut_occurs`)
bind the SAME values through the new occurrence path. The existing
ken-elaborator match and dependent-match suites are named individually in the
handback and stay green. A green suite reported as a total is not evidence.

**`AC-GATE-0`** (within-lane / option-(a)). At build: the kernel tree is
unchanged, the emitted term is still nested `elim_D`, and there is NO new core
`Term`/IR variant and no projection/accessor core term. Confirm, do not assume.

**`AC-NO-REGRESSION`** -- green in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator` (and any consumer whose closure this match-compiler edit
changes -- cover every target that loads a module whose closure the increment
changes, diff-touched or not). Never `--workspace`.

## What is NOT credited here, and what lands with the consumer

The BLACK-BOX proof that a value-binding form binds the right value -- the
mandatory NESTED as-pattern test (an alias at a nested constructor position, not
only top-level) -- lands with the consumer slice `LANG-MATCH-AS-PATTERN`. **This
capability is credited on `AC-OCCURRENCE-CORRECT` + `AC-WEAKEN-AT-DEPTH` +
`AC-VAR-PRESERVED` + `AC-GATE-0`, NOT on regression-green alone** -- an inert
capability that preserves existing behavior while threading wrong occurrences
would pass a pure regression suite, which is why the white-box occurrence +
depth-weakening controls are required.

## Capability tier

**T1.** The deliverable is soundness-bearing de-Bruijn rebasing through the match
compiler -- the Architect explicitly flagged the convoy/telescope hazard, and
`AC-WEAKEN-AT-DEPTH` is the judgment the work turns on. The deliberate language
T1 seat fits; no reseat.

## Hard stop (GATE-0, carried)

Inability to thread the occurrence without a new core `Term`/IR variant, a
projection/accessor core term, or a value crossing a function-return is a HARD
STOP to the Architect -- it would mean the option-(a) premise is false. Do NOT
work around it. The Architect is the required reviewer of record on this
capability WP and on the as-pattern consumer.
