---
id: LANG-LIFT-DISPATCH-SELF-GUARD
title: "`check_match_with_lift`'s family-membership protection is transitive -- it holds only because the dispatch has exactly one caller, while its sibling in the same file already has two -- so make the dispatch self-guarding instead of documenting the hazard"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-FOREIGN-CTOR-ARM-REJECT]
blocks: []
github: null
origin: "Adversary hunt on the landed squash 0c6c1747 (evt_12eh2n45gyjn9), confirmed and extended by the Architect at evt_dqfq4m16vv1n with the one-line vehicle below. Nothing is live and the merge is not reopened -- both agreed this is successor work. Steward-filed per COORDINATION §2. The Architect asked explicitly that it NOT be discharged with a comment, and said why; that reasoning is the node."
---

> # THE FIX IS ONE LINE. THE NODE IS ABOUT WHY IT IS A LINE AND NOT A COMMENT.
>
> **Nothing here is live.** `LANG-FOREIGN-CTOR-ARM-REJECT` is `merged` and
> correct; every path into the reachability sweep is guarded **today**. This
> node is about what a future ordinary-looking edit can break without any local
> signal that it did.

## The measurement

Caller counts re-derived on `main` after the merge:

| dispatch | callers | guarded |
|---|---|---|
| `check_match_dependent` `:1969` | — | **directly**, `:2013` |
| `infer_match` `:8460` | **two** — `:1222`, `:3580` | **directly**, `:8492` |
| `check_match_with_lift` `:1491` | **one** — `:2042` | **transitively, by its caller** |

`check_match_with_lift` has its own `missing_pattern_witness` and
`ExhaustivenessError` sites. It looks like an independent entry point, and its
protection comes entirely from the fact that its single caller sits inside
`check_match_dependent` after that function's guard at `:2013`.

⇒ **Completeness rests on that dispatch having exactly one caller — while its
sibling in the same file already has two.** A second caller from `check`, which
is precisely the shape `infer_match` has *today*, opens a foreign-constructor
path straight into the witness and subsumption machinery with no family guard.

## Why a comment is the wrong instrument, stated by the Architect

> *"A clause saying 'guarded by my caller' documents a module-level
> reachability property that no local edit can see it is breaking — the same
> failure mode as the guard-shape risk we just closed."*

**A comment describes the hazard to someone who is already reading the right
function.** The edit that breaks this is adding a caller *somewhere else*, and
that author has no reason to open `check_match_with_lift` at all.

⇒ **Making the dispatch self-guarding converts one module-level reachability
property into three local ones**, and the *"do not add an unguarded caller"*
hazard stops existing rather than being written down.

## The vehicle, and it needs no plumbing

The signature already carries everything the guard needs — `arms: &[RMatchArm]`
and `host: &InductiveDecl`. At the top of the function body, **before the
`binding.support` unwrap**:

```rust
ensure_arm_ctors_belong_to_family(cx, arms, host, host.id)?;
```

`host.id` is already used at `:1517`. **Cost: one redundant `O(arms × ctors)`
pass on the lifted path, where the guard is a pure read.**

## Deliverables

**`D1`** — add the self-guard at the top of `check_match_with_lift`, before the
`binding.support` unwrap. Re-derive the three caller counts at your base first
and **report them**; if they have moved, that is a finding before it is a
repair.

**`D2`** — a control that reds if the guard is removed. The existing
foreign-arm tests reach this dispatch through `check_match_dependent`, so they
pass with or without it. **A test that cannot distinguish the two states does
not cover this node.** The control must exercise the lifted path specifically.

## Acceptance criteria

**`AC-1`.** The guard is in the function body, not at its call site. A repair
that guards `:2042` instead has re-implemented the transitive property this
node exists to remove.

**`AC-2`.** `D2`'s control reds with the guard bypassed — demonstrate it by
mutation, the way `LANG-FOREIGN-CTOR-ARM-REJECT`'s own control was demonstrated
(`return Ok(())` in the helper, watch the specific diagnostic change). **Report
the diagnostic it reds with**, not merely that it failed.

**`AC-3`.** Direction stated: this **adds** a rejection on a path that is
currently unreachable. No existing accept becomes a reject. If any landed test
changes behaviour, stop — that means the path was reachable and this node's
premise was wrong, **which is a bigger finding than the repair.**

**`AC-4`.** No comment is added in place of the guard. A comment *alongside* it
is fine.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## Not in scope

- **The slice-indexing residual.** `:8182`/`:8183`'s carried-`arm_idx`
  invariant folds into [[LANG-MATCH-PATTERN-FORMS-ABSENT]], not here.
- **Nested foreign sub-patterns.** Same node.
- **Do not re-open `0c6c1747`.** It is correct as landed.

## The general lesson, recorded because it has now happened twice in this lane

**The Architect stated a local ordering where the property needed was
reachability**, and asked that it be on the record rather than smoothed over:

> *"I offered function-local ordering ... and that establishes **when** the
> guards run, not **what they see**."*

The Adversary's addition is the cheap remedy: *"that is a cheap check to add to
the same review, not a different kind of review."* **When a guard's sufficiency
is argued from ordering, ask the second question — what does the guarded code
see, and from how many entry points.**
