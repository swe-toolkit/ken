---
id: LANG-MATCH-PATTERN-FORMS-ABSENT
title: "spec 34 §3 normatively lists nine pattern forms and the elaborator's PatKind has three -- literals, tuple/record patterns, as-patterns, or-patterns and guards are all absent from the AST, with no deferral statement anywhere in a chapter marked impl-ready and high-priority, and no tracker row for any of them"
status: draft
owner: language
size: unsized
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS]
blocks: []
github: null
origin: "Steward sweep 2026-08-14 at main 6da108b6, reached from the Adversary hunt evt_4d10j8tmjsbhj -- which measured, as a side observation on a diagnostic-prose node, that 34 §4.2's two reachability caveats are BOTH unreachable because MatchArm has no guard field and PatKind has no literal kind. That is a symptom; this node is the cause. The census below is the Steward's, measured after the hunt. Filed draft and unsized because the CUT is the deliverable and it is not yet made. Steward-filed per COORDINATION §2. AMENDED 2026-08-14 on the spec-enclave disposition evt_12qrtnp7237dn, which ruled 34 §3 an impl-ready present-tense obligation whose six absent forms are implementation debt rather than an aspirational menu, gave a prerequisite-ordered cut, and CORRECTED the Steward's caveat claim -- see the box below."
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

## THE CUT IS THE DELIVERABLE, AND THE ENCLAVE HAS NOW GIVEN IT

**Six forms is not one WP and this file is not a frame.** That much was clear at
filing; what this node could not supply was the order and the prerequisites.
**Both are now ruled -- see the disposition table below, which supersedes the
Steward's guesses in this section.**

Recorded because it is the reasoning the ruling had to answer, and because two
of the three guesses were wrong:

- **Literals depend on decidable equality.** ⇒ **Understated.** The enclave
  ruled `DecEq Char` alone insufficient; `Float`/`Float32` and `Decimal`
  distinguish runtime value equality from lawful proof `DecEq`, and numeric
  literals need expected-type checking besides. The blocker is larger than the
  open operator TCB question gating [[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]].
- **Guards change the exhaustiveness contract**, not just the pattern grammar --
  `§3.3` is explicit that a guarded arm does not discharge its constructor. ⇒
  **Stands**, and the enclave made it sharper: guards are **atomic** with their
  conditional lowering and all `§3.3`/`§4` behaviour, and **do not depend on
  literals.**
- **Or-patterns change binder checking.** ⇒ **Stands**, and the rule is bigger
  than "identical binder sets": see row 4 below.
- **"As-patterns and tuple/record patterns look like the two contained
  slices."** ⇒ **Half wrong.** As-patterns are the smallest first delivery, but
  **record patterns are not to be bundled with tuples merely because both
  project.** They are separate slices with separate pins.

## THE CAVEAT CLAIM, CORRECTED BY THE ENCLAVE

> **What I wrote was wrong and the correction is finer-grained.** I wrote that
> adding **either** guards or literal patterns makes **both** of `34 §4.2`'s
> caveats live at once. **Spec enclave, `evt_12qrtnp7237dn`: guards and literals
> each activate their OWN feature caveat, not both caveats merely because either
> lands.** What *is* true of either slice is that **both the coverage and the
> reachability obligations become live within it.**
>
> The underlying measurement stands and is unaffected: `MatchArm` has no guard
> field (`ast.rs:86`), `PatKind` has no literal kind (`ast.rs:167`), and the
> reachability checker `arm_used` (`elab.rs:1737`, `:2427`, `:8446`) **has no
> `§3.3` guard exception**, because there has never been a guard.
>
> ⇒ **A guards slice that adds the syntax and the elaboration but not the
> `§3.3` coverage exception ships a checker that calls correct programs
> redundant.** The reachability prose landed by `LANG-MATCH-DIAGNOSTIC-PROSE` is
> accurate **contingent on the absence** and goes stale with its own slice.
>
> **The enclave adds one rule the cut must obey:** each **wrapper** form can
> start over the current baseline, but **every later inner pattern form must add
> a composition discriminator.**
>
> Original measurement: Adversary `evt_4d10j8tmjsbhj`, re-checked by the Steward
> at `6da108b6`. The mirror of this paragraph is in
> [[LANG-REACHABILITY-SUBSUMING-ARMS]].

## THE ENCLAVE DISPOSITION AND ITS CUT ORDER

**`evt_12qrtnp7237dn`. `34 §3` is an impl-ready, present-tense obligation, and
the six absent forms are implementation debt, not an aspirational menu.** They
may be staged **only as explicit tracked slices**, and **every remainder stays
fail-closed until its slice lands** -- so a partial delivery must reject what it
has not implemented, never silently accept it.

**The umbrella node stays `draft`. It is not a six-form frame and must never
become one.**

> ## EVERY PIN IN THIS TABLE LANDED AT `34fd01c1`. The column heading used to
> ## read "which does not exist yet" and that is now false for all six rows.
>
> **Measured by the Steward at `main` `a12f37b7`.** [[SPEC-MATCH-PATTERN-PINS]]
> delivered the whole prerequisite column:
>
> - **as-pattern association/precedence** -- `32 §4`: constructor application
>   binds tighter than `as`, `as` binds tighter than `|`, `as` is
>   non-associative, and an `as`- or or-pattern used as a constructor argument
>   must be parenthesized.
> - **tuple grouping/comma** -- `32 §4`: `(p)` is grouping, a tuple contains at
>   least two patterns, arity >2 right-nests, no zero- or one-tuple.
> - **`field_pat` spelling** -- `32 §4`: label/value form, punning, omission,
>   open-vs-closed, duplicates, unknown fields, source order. All seven.
> - **or-pattern binder join** -- `34 §3.1`: same binder-name set, each name
>   bound exactly once per alternative, corresponding types definitionally equal
>   in the common pre-branch context, first alternative's order canonical. Its
>   association with `as` is in `32 §4`.
> - **guards** -- no pin was named and none is owed.
> - **literals** -- `34 §3.2` carries the literal-kind-to-comparator table with
>   an equality-authority column, and pins that a literal's expected type is the
>   scrutinee's.
>
> ⇒ **Read the rows below as slice DESCRIPTIONS with their pins discharged, not
> as blockers.** The order still stands and the umbrella still never becomes one
> frame.

| order | slice | prerequisite pin -- ALL LANDED at `34fd01c1` |
|---|---|---|
| 1 | **as-patterns** -- alias binds the whole matched scrutinee, no split, preserves `p`'s coverage/reachability, rejects collision with an inner binder | `p as x` association/precedence |
| 2 | **tuple/pair patterns** -- dependent `Σ` projection typing, right-nesting for arity >2, componentwise coverage/reachability, `Proj1`/`Proj2` lowering with no pair `elim` | `(p)` is grouping; a tuple requires a comma |
| 3 | **record patterns** -- **do not bundle with tuples merely because both project** | `field_pat` spelling: label/value form, punning, omission and open-vs-closed, duplicates and unknown fields, source order |
| 4 | **or-patterns** -- owns union coverage and residual duplication | full binder-join rule and association with `as`: identical name sets, exactly one binding per alternative, corresponding types definitionally equal in the common pre-branch context, canonical branch environment |
| 5 | **guards** -- atomic with their conditional lowering and all `§3.3`/`§4` coverage, reachability and non-refinement behaviour. **They do not depend on literals.** | none named |
| 6 | **literals** -- **blocked** | a literal-kind-to-value-comparator table and a corrected citation, plus the relevant equality authority |

**On literals, the enclave went further than this node did: `DecEq Char` alone
is insufficient.** `Float`/`Float32` and `Decimal` distinguish runtime value
equality from lawful proof `DecEq`, and numeric literals additionally require
expected-type checking. ⇒ **The literal slice is blocked on more than the open
operator TCB question**, and this node's earlier statement that it was gated on
that question alone understated it.

## Flip condition -- the spec half is DONE; the next node is LANGUAGE's

**The enclave's disposition asked for the small spec pins first and THEN a
first contained as-pattern slice** -- not a six-form frame. **The first half of
that instruction is discharged.**

⇒ **This section previously read *"the next node in this chain is
spec-enclave-owned, not Language-owned"*, and that is no longer true.** It was
correct when written and `34fd01c1` retired it. **Nothing spec-side stands
between this node and its first slice.**

**What actually gates the as-pattern slice now is Language's own queue, which
is a contention fact and not a premise.** Three nodes already share `elab.rs`
and run in a line -- [[LANG-REACHABILITY-SUBSUMING-ARMS]], then
[[LANG-WITNESS-DIAGNOSTIC-STRICTNESS]], then
[[LANG-FOREIGN-CTOR-ARM-REJECT]]. The as-pattern slice is the fourth, and the
Steward cuts it rather than the ring.

**This node flips per slice, never as a whole.**

**One thing already decided and needing no further ruling:** this node does
**not** amend `34`. The enclave ruled the chapter's obligations real, so the
"drop it from the surface" branch is closed -- the six forms are debt to be
paid, not text to be corrected.

## Not this node

- **Not [[LANG-REACHABILITY-SUBSUMING-ARMS]]**, which is the reachability
  *diagnostic payload* and explicitly implements neither feature.
- **Not [[LANG-CONVOY-ENCLOSING-FIELD]]**, which is nested-match re-typing
  completeness at `34 §3.2` and is orthogonal to which pattern forms exist.
- **Not [[LANG-SURFACE-PAIR]]**, merged, which delivered pair expressions and
  projections rather than pair patterns.
