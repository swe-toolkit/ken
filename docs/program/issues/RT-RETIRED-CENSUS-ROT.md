---
id: RT-RETIRED-CENSUS-ROT
title: "Censuses retired by #[cfg(any())] are preserved as a readable record of a property, but cfg-stripping means nothing name-resolves them -- 3 of 3 are dead on revival, and one names a function deleted 19 days after its retirement"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_6npaybf8cznp8 (2026-08-18) on the RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING D3 landing b430d73e0. Steward-filed per COORDINATION section 2. The finding measured all three retired censuses by flipping cfg(any()) to #[test]; every figure below is the Adversary's, reproduced from its report."
---

> # RELEASED 2026-09-14 BY THE STEWARD. STARTABLE NOW, ON A CLEAN BASE.
>
> **The hold is lifted and the reason it was written no longer holds.** It said
> *"held behind the single runtime lane"* — the constraint was the runtime ring's
> attention, not anything about this work. That constraint is inverted today: the
> ABI-S6 / D5b stack is blocked behind a red CI at an unpushed tip, nothing there
> is dispatchable to an implementer, and the runtime ring is idle. **Re-sequencing
> is the Steward's call (`steward.md §3`); nothing in the node's content changed.**
>
> **Why this node and not another of the sixteen ungated `ready` runtime nodes:
> it is the one with a genuinely CLEAN BASE.** It is `size: S`,
> `depends_on: []`, and it touches `crates/ken-runtime/src/control.rs` — clear of
> the 24-commit `wp/ABI-S6-d5b-file-backed` stack that every other runtime item is
> entangled with. It can be cut from `origin/main` and reviewed on its own.
>
> **It remains true that this is not urgent** — no behaviour is wrong and nothing
> is unsound; the rot is in commentary rather than in compiled code. It is being
> released because it is *available*, not because it became important. **If
> anything in the D5b arc becomes dispatchable, that outranks this.**
>
> **`D0` is a component-design call and is the Architect's, not the
> implementer's.** Take it there before building either arm; the Architect is
> seated and light. Note `AC-2`'s shape in particular: the control must be shown
> to **red on today's `main`** before any repair, and that red is the acceptance
> evidence. A green run proves nothing here, because the tree already fails 3 of 3.

## The defect

`control.rs` retires censuses by putting `#[cfg(any())]` on them rather than
deleting them. **The stated reason to prefer that over deletion is that the
retired body stays readable as a record of the property it pinned.** That
presumes the body keeps describing the tree.

**It does not, and the mechanism that would tell you was removed by the same act
that retired it.** `cfg`-stripping precedes name resolution, so a retired body is
neither type-checked nor name-resolved. The tree drifts underneath it silently
and permanently.

## Measured: 3 of 3 retired censuses fail on revival

The Adversary flipped `#[cfg(any())]` to `#[test]` on every retired census in
`control.rs` and ran them under `-p ken-runtime --lib`. **All three fail.**

| retired census | line | first failure | measured vs asserted |
|---|---|---|---|
| `d8_join_helpers_have_the_closed_typed_caller_population` | 12891 | `control.rs:12899` | `fn merge_branch_value(` defs **0** vs 1 |
| `exactly_one_plan_origin_to_expression_lookup_exists` | 8241 | `control.rs:8255` | pinned planner surface list has 32 entries; the current one is many times that |
| `the_lower_expr_call_population_is_dispositioned_by_owner_not_by_site` | 9234 | `control.rs:9270` | `lower_expr` tokens **70** vs 65 |

libtest stops at the first assertion, so the D8 census was re-run with its
asserts replaced by prints, letting the test's own instrument report every value.
**Four of its seven assertions are false**, and the independent greps and the
test's instrument agree exactly:

| the test's own measurement | asserts | actual |
|---|---|---|
| `fn merge_branch_value(` in `mod.rs` | 1 | **0** |
| `fn merge_scalar_branch(` in `mod.rs` | 1 | 1 |
| `fn merge_planned_scalar_branch(` in `mod.rs` | 1 | 1 |
| `.merge_branch_value(` in `core.rs` | 4 | **0** |
| `.merge_scalar_branch(` in `core.rs` | 10 | **4** |
| `.merge_planned_scalar_branch(` in `core.rs` | 1 | 1 |
| `plan: &JoinPlanToken` in `mod.rs` | 3 | **2** |

## The ordering is what makes this a claim about the mechanism

- retirement `6a451b456`, 2026-07-29, put `#[cfg(any())]` on the D8 census
- deletion `1aec3e3e1`, 2026-08-17 (`RT-DESCENT-RETIRE` `D3`-`D6`/`D8`), removed
  `fn merge_branch_value` from `lowering/mod.rs`
- `git merge-base --is-ancestor 6a451b456 1aec3e3e1` returns true

**The subject was deleted 19 days after its census was retired, and nothing could
notice.** `merge_branch_value` now exists nowhere under `crates/` except as two
string literals inside the retired body. A reader consulting `control.rs:12891`
for the D8 join-helper family gets a three-member family whose first member does
not exist.

## Already refuted, so nobody respends it

**The `plan: &JoinPlanToken == 3` assertion falling to 2 is NOT a token-gate
hole.** It was chased as one and it is closed by construction: the live family is
`merge_scalar_branch` (`mod.rs:17919`) and `merge_planned_scalar_branch`
(`mod.rs:18271`), both taking `join_plan: &JoinPlanToken`, both rejecting a
non-`NativeScalarPair` representation before delegating to `merge_scalar_operand`
(`mod.rs:17942`), which takes no token and has **exactly two call sites in all of
`crates/`** — those two wrappers. The count fell 3 to 2 because a helper was
deleted, not because one lost its token.

**What the retired assertion did guard is the family's closure** — a third caller
of `merge_scalar_operand` could be added inside `lowering/mod.rs` with no token
and nothing would object — and the successor the retirement names does not carry
that. **That is a missing guard, not a defect, and it is deliberately not filed
as one.** If the fork below picks deletion, decide separately whether that
closure property is worth a live control.

## `D0` — decide what a retired body IS. The fork is the whole node.

The two arms are not equivalent and neither is obviously right.

1. **A retired body is a record.** Then it needs one cheap live control keeping
   it honest: a test that `include_str!`s `control.rs` and asserts every
   identifier named inside a `#[cfg(any())]` block still resolves somewhere under
   `crates/`. **That control would have reddened on `1aec3e3e1`.**
2. **A retired body is not a record.** Then deleting these three is more honest
   than preserving text that reads as one, and git history remains the record.

**Do not split the difference by fixing the three current failures.** Re-deriving
them restores the appearance of currency without restoring the mechanism, and the
next deletion re-rots them with nobody watching. That is the failure this node
exists to stop, not an instance of it to clean up.

## Acceptance criteria

- **`AC-1`.** `D0` is decided and `control.rs` says which reading it took, at the
  retirement convention's own site rather than only in this node.
- **`AC-2` — if arm 1: the control is demonstrated by the mutation it catches.**
  Delete or rename an identifier named inside a retired body and show the control
  reds; restore it and show green. **A green run on the current tree is not
  evidence** — the current tree already fails 3 of 3, so the control must be
  shown to red on today's `main` before any repair, and that red is the
  acceptance evidence, not a regression.
- **`AC-3` — the population is stated.** Say how many `#[cfg(any())]` regions
  exist in `control.rs` and how the control enumerates them. An identifier
  extractor that silently skips a spelling reintroduces exactly this class one
  level down.
- **`AC-4` (no-regression).** Workspace green **in CI**, never a local
  `--workspace` run (`COORDINATION §12`).

## Banned scope

- **Reviving any retired census as a live test.** All three fail; turning them on
  is a different and much larger piece of work, and `D3` of
  [[RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING]] already ruled that reviving a
  census in this file is not a cheap option.
- **Repairing the four false D8 assertions.** See the fork — that is the
  split-the-difference move.
- **Filing the `merge_scalar_operand` closure gap as a defect.** It is a missing
  guard on a gate that is closed by construction today.
- **Claiming a regression or reclassifying `D3`'s landing.** `D3` is correct and
  this finding strengthens its choice: it rejected revival because the census is
  compiled out, and revival turns out not to be available for this family at all.

## Sequencing

Blocks nothing. The operator's run order stands. **It is worth doing before the
next large deletion campaign under `lowering/`**, because that is when a retired
body rots, and the campaign in front of it is the backend module split.
