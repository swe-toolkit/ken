---
id: KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE
title: "Complete the intrinsic All former-lift guest-path resolution to descend through NESTED positive formers (List (Pair String X)), so the eliminator of a strictly-positive inductive whose recursive occurrence sits one former-nesting deeper than a direct application can be built — preserving strict positivity (positive former nestings only, never a negative-position path)."
status: draft
owner: kernel
size: M
gate: kernel
tier: T1
depends_on: []
blocks: [DS-9]
github: null
origin: "Steward-filed 2026-09-10 on the Architect ruling evt_6g6fgjb0qan40 (thr_5fw1hbtp3fses), grounded on foundation-implementer's DS-9 D3+ D0 hard stop evt_6ha6y4ghng65j (exact base ed3b57d6). A genuine, grounded kernel-completeness gap — NOT an unsoundness — that totally blocks DS-9 (no Json match method can be built). The Architect ruled it a Kernel WP with the soundness gate below, distinct from and independent of the pre-existing show_int floor and of CAT-MAP; the Architect stated it does NOT need the operator's 12:00 UTC return (bounded, grounded, clear soundness gate)."
---

> # KERNEL prerequisite for DS-9. Grounded on Architect ruling evt_6g6fgjb0qan40.
> #
> # A fail-closed kernel INCOMPLETENESS: the intrinsic All former-lift rejects a
> # well-formed strictly-positive type. The fix completes the capability; it does
> # NOT relax a soundness check. The soundness GATE (positivity preservation) is
> # the load-bearing constraint and the Architect's review criterion.

## The measured defect (Architect evt_6g6fgjb0qan40; D0 evt_6ha6y4ghng65j)

`crates/ken-kernel/src/inductive.rs:1455`, `intrinsic_former_lift_type`, resolves
the intrinsic All former-lift's guest path by scanning a former's arguments
(`guest_params_from_shape`; the `RecursiveShape::Former` arm at `:1409`). It finds
the guest when a former is applied to the recursive type **directly** — `List
Json` / `List ArrayJson` — but returns "no guest path" when the recursive
occurrence sits one former-nesting **deeper**: `List (Pair String Json)`, where
`Json` is buried inside `Pair` inside `List`. It fires while building the `Json`
match METHOD (the eliminator), upstream of any codec recursion, so it blocks even
a **non-recursive** match over `Json`.

D0 reproduced it exactly (foundation-implementer, base `ed3b57d6`): a six-arm
non-recursive `match value { … JsonObject members ↦ Zero }` over the public `Json`
fails at the `JsonObject` arm with `KernelRejected { PositivityViolation("intrinsic
All lift has no guest path") }`, while the paired `ArrayJson = ArrayNull |
ArrayValues (List ArrayJson)` (direct `List` application) elaborates. So the guest
path buried in `List (Pair String Json)` is the specific gap.

D0 re-measures the exact loci at pickup (line numbers drift). The `:1455` /
`:1409` / `guest_params_from_shape` anchors are the Architect's; confirm them.

## Soundness — this is INCOMPLETENESS, not a missing guard (the whole gate)

`Json` with `JsonObject (List (Pair String Json))` **IS strictly positive** —
`List` and `Pair` are covariant and `Json` occurs only in positive position. The
kernel is conservatively **incomplete**, not protecting against a real
unsoundness: `PositivityViolation` here means the kernel REJECTS a well-formed
type, never that it accepted an unsound one.

**The load-bearing constraint (Architect's review criterion).** The descent
completion must extend guest-path resolution through **POSITIVE former nestings
ONLY**. It must NOT open a path that would admit a recursive occurrence in a
**negative** position (a function domain) — the K1 hidden-negative classes. That
is the soundness gate the Architect will hold the review to. It is not a carrier
change and needs no forbidden re-encoding.

## Deliverables (D0-first)

- **D0 — reproduce + locate the descent boundary.** Reproduce the reject on the
  minimal `List (Pair String X)` guest and confirm the direct-application case
  passes. Locate exactly where `guest_params_from_shape` / the
  `RecursiveShape::Former` arm stops descending (it resolves one former level, not
  the nested former). Emit the exact positive-nesting descent the fix adds and the
  negative-position boundary it must NOT cross. Hard-stop to the Architect if the
  gap is not a pure positive-nesting descent (e.g. if the guest path would require
  touching a negative position).
- **D1 — complete the positive-nesting descent.** Extend the former-lift guest-path
  resolution to descend through nested positive formers so the eliminator builds
  for `List (Pair String X)` (and the general nested-positive-former shape). No
  relaxation of the negative-position check; strict positivity is preserved
  exactly as today for every previously-rejected unsound shape.

## Acceptance criteria (each with its control)

- **AC-NESTED-POSITIVE-ADMITTED (the capability).** A strictly-positive inductive
  whose recursive occurrence sits inside a nested positive former (`List (Pair
  String Self)`, and the general shape) builds its match method / eliminator and
  elaborates. Control: the exact D0 probe (six-arm `Json` match, incl. the
  `JsonObject` arm) that reds today goes green; a paired direct-application case
  stays green (no regression).
- **AC-STRICT-POSITIVITY-PRESERVED (the soundness gate — hard).** Every shape the
  kernel rejects today for a genuine positivity violation — a recursive occurrence
  in a **negative** position (function domain), the K1 hidden-negative classes —
  STILL reds `PositivityViolation` after the fix. Control: a negative-position
  probe (recursive occurrence in a function domain, and a former-nested negative
  occurrence) still rejects; a mutation that lets the descent cross into a negative
  position reddens this control. The fix that admits `List (Pair String Self)` must
  NOT admit any negative-position occurrence — this is the Architect's review
  criterion, and a control that only exercises positive shapes does not discharge
  it.
- **AC-NO-TCB-WIDENING.** The change completes an existing resolution path; it adds
  no axiom/postulate/primitive and no new trusted surface. Control: `trusted_base()`
  differential delta zero; the diff is confined to the guest-path descent in
  `inductive.rs` (and its direct callees), not a new kernel capability.
- **AC-AFFECTED-CLOSURE.** Re-run every kernel/elaborator target whose positivity
  or eliminator-construction path this touches (targeted via `scripts/ken-cargo`,
  never `--workspace`; CI is the workspace verdict). Retain every existing
  positivity test unweakened.

## Gate, reviewer, sequencing

`gate: kernel` — this is kernel/TCB-adjacent (the positivity checker), so the
**Architect owns the soundness gate** (positivity preservation is THE criterion,
AC-STRICT-POSITIVITY-PRESERVED) and Kernel QA owns the build. kernel-leader frames
the team-level WP decomposition within this objective. On the candidate: Architect
soundness review + Kernel QA on the exact SHA, then Steward M1-M4 -> lieutenant.
`blocks: DS-9` — on this node's landing the Steward re-releases DS-9's D3+ (the
show_int floor remains a separate, independent gate on the complete number leaf).

## Hard stop

Route to the Architect if D0 finds the gap is not a pure positive-nesting descent
— if admitting `List (Pair String Self)` cannot be done without also opening a
negative-position path, the shape is not what this frame assumes and the soundness
gate is at risk. Land nothing on that axis until the Architect rules.
