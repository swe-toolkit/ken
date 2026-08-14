---
id: LANG-MATCH-PATTERN-FORMS-ABSENT
title: "spec 34 §3 normatively lists nine pattern forms and the elaborator's PatKind has three -- literals, tuple/record patterns, as-patterns, or-patterns and guards are all absent from the AST, with no deferral statement anywhere in a chapter marked impl-ready and high-priority, and no tracker row for any of them"
status: draft
owner: language
size: unsized
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward sweep 2026-08-14 at main 6da108b6, reached from the Adversary hunt evt_4d10j8tmjsbhj -- which measured, as a side observation on a diagnostic-prose node, that 34 §4.2's two reachability caveats are BOTH unreachable because MatchArm has no guard field and PatKind has no literal kind. That is a symptom; this node is the cause. The census below is the Steward's, measured after the hunt. Filed draft and unsized because the CUT is the deliverable and it is not yet made. Steward-filed per COORDINATION §2."
---

## What this is

**`spec/30-surface/34-data-match.md §3` lists the pattern forms normatively, in
one sentence (line 249-251). Six of the nine are absent from the AST.**

Measured at `main` `6da108b6`, `crates/ken-elaborator/src/ast.rs`:

```rust
pub struct MatchArm {          // :86
    pub pat: Pattern,
    pub body: Expr,
    pub span: Span,
}

pub enum PatKind {             // :167
    Wild,
    Var(String),
    Ctor(String, Vec<Pattern>),
}
```

| `34 §3` form | spec | AST |
|---|---|---|
| constructor `C p̄` | `§3.1` bullet 1 | `PatKind::Ctor` |
| variable binder | `§3.1` bullet 2 | `PatKind::Var` |
| wildcard `_` | `§3.1` bullet 2 | `PatKind::Wild` |
| **literals** | `§3.1` bullet 3, compiles to a decidable-equality chain (`35`, `39 §2.7`) | **absent** |
| **tuple / pair patterns** | `§3.1` bullet 4, projects the negative `Σ` | **absent** |
| **record patterns** | `§3.1` bullet 4, matched by projection (`14 §4`) | **absent** |
| **as-patterns `p as x`** | `§3.1` bullet 5 | **absent** |
| **or-patterns `p \| q`** | `§3.1` bullet 5, identical binder sets (`32 §4`) | **absent** |
| **guards `if g`** | `§3` line 251 and `§3.3` line 472, elaborate to a conditional inside the `cₖ` method | **absent** (`MatchArm` has no field) |

## Why this is a gap and not a staged deferral

**Interrogating the constraint before filing, per `ken-steward §4c`:**

- **The chapter's own status line (`34:3`) is `impl-ready (L2). Normative and
  high-priority for the feature.`** Not draft, not staged.
- **A deferral-phrase grep of the chapter finds three deferrals and none of them
  is a pattern form.** `:458` is the two-vector `zip` convoy step (filed as
  [[LANG-CONVOY-ENCLOSING-FIELD]]); `:533` is auto-motive spelling for `J`;
  `:173` is record-field constructors, which is narrower than record patterns.
  **The six rows above are stated as what the elaborator does, in the present
  tense, with their compilation rules given.**
- **The tracker has no row for any of them.** [[LANG-SURFACE-PAIR]] (merged)
  delivered pair *expressions*, positional projections and the `Σ` type
  production -- **expression syntax, not pattern syntax**; it is the nearest
  neighbour and it does not overlap.

⇒ **Normative text, an impl-ready chapter, a measured absence, and no deferral
and no node. That is the strongest grounding available for a new node** and it
is the same shape as the three obligations the deferral sweep already produced.

## THE DELIVERABLE OF THIS NODE IS THE CUT

**Six forms is not one WP and this file is not a frame.** The forms do not
decompose evenly and at least three carry a dependency that is not theirs:

- **Literals depend on decidable equality**, whose widening is the open operator
  TCB question gating [[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]]. **Do not cut a
  literal-patterns WP before that is answered.**
- **Guards change the exhaustiveness contract**, not just the pattern grammar.
  `§3.3` is explicit: a guarded arm does **not** discharge its constructor, so
  `§4.2` coverage and reachability both change the day guards land.
- **Or-patterns change binder checking** -- identical binder sets across
  alternatives is a new well-formedness rule, not a new `PatKind` row.

**As-patterns and tuple/record patterns look like the two contained slices**,
and that is a Steward reading of the spec text, not a measurement. **Whoever
cuts this measures it first.**

## What must ride along with GUARDS specifically, whenever that slice is cut

> **`34 §4.2`'s two reachability caveats are both vacuous today, and adding
> either guards or literal patterns makes BOTH live at once.** The reachability
> checker is `arm_used` -- an arm that never won at any leaf -- at `elab.rs:1737`,
> `:2427` and `:8446`. **It has no `§3.3` guard exception**, because there has
> never been a guard.
>
> ⇒ **A guards slice that adds the syntax and the elaboration but not the
> `§4.2`/`§3.3` coverage exception ships a checker that calls correct programs
> redundant.** The reachability prose landed by `LANG-MATCH-DIAGNOSTIC-PROSE` is
> accurate **contingent on this absence** and goes stale on the same day.
>
> Measured by enumerating the AST variants, not by a grep that found nothing
> (Adversary `evt_4d10j8tmjsbhj`; re-checked by the Steward at `6da108b6`). The
> mirror of this paragraph is in [[LANG-REACHABILITY-SUBSUMING-ARMS]].

## Flip condition

**Flip to `ready` per slice, not as a whole.** The first slice to be cut needs
two things this node deliberately does not decide: **which forms are one WP**,
and **whether any of them is gated on a kernel or spec question** the way
literals are gated on decidable equality.

**The route is the Spec enclave, not the Architect alone** -- the question is
what `34 §3` obliges and in what order, which is spec-shaped. **Raise it as
"what does the chapter oblige and what is genuinely stageable", never as "which
node should this be"**, which presumes the answer.

**One thing that is already decided and does not need the ruling:** this node
does **not** amend `34`. If the conclusion is that some form should be dropped
from the surface, that is a spec-erratum question raised as one.

## Not this node

- **Not [[LANG-REACHABILITY-SUBSUMING-ARMS]]**, which is the reachability
  *diagnostic payload* and explicitly implements neither feature.
- **Not [[LANG-CONVOY-ENCLOSING-FIELD]]**, which is nested-match re-typing
  completeness at `34 §3.2` and is orthogonal to which pattern forms exist.
- **Not [[LANG-SURFACE-PAIR]]**, merged, which delivered pair expressions and
  projections rather than pair patterns.
