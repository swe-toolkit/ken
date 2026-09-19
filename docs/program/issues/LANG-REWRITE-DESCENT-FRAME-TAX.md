---
id: LANG-REWRITE-DESCENT-FRAME-TAX
title: "rewrite_rexpr_inner's stack frame is sized by the union of ALL RExpr arms, so every variant added to the language surface taxes every level of every descent. The function (crates/ken-elaborator/src/modules.rs:1290) is one match over every RExpr variant, each arm constructing a fresh RExpr by value in return position; in a debug build rustc does not coalesce per-arm stack slots, so the frame carries the whole surface whether or not the program contains any of it. A1's two new arms (RStandardOp, RInfixSpine) added 144 B and pushed the D1 legacy-map descent past local_prebinding_preserves_legacy_map_union_stack_budget's 2 MiB boundary -- but RStandardOp is not special: the NEXT variant anyone adds does the same thing and the detector goes red again under a different name. Stop paying the whole-surface frame tax, then remeasure. MEASURE BEFORE REPAIRING: the claim that frame size dominates is INFERRED, not measured, and D0 exists to test it."
status: ready
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [LANG-STANDARD-INFIX-CALL-COMPLETION]
github: null
origin: "Steward, 2026-09-19, at origin/main 1be846b2d062e23be5cdf6ebd789837feeb637ab. Filed on the operator's ruling this session -- verbatim: 'g then remeasure' -- selecting option (g) from the A1 re-baseline fork and directing that the measurement follow the repair. Option (g) was the Steward's, added after the enumeration (a)-(f) was already before the operator; it is the only option that does not re-place the detector's boundary. A1 (LANG-STANDARD-INFIX-CALL-COMPLETION) HOLDS behind this node: Steward sequencing call, on the grounds that landing A1 first FORCES the D1 re-baseline decision that (g) exists to avoid, and A1 was already gated on the operator so the hold costs no new time."
---

> ## FRAMED 2026-09-19 — `ready`, size M, tier T1
>
> Frame: [`wp/LANG-REWRITE-DESCENT-FRAME-TAX.md`](../wp/LANG-REWRITE-DESCENT-FRAME-TAX.md)

## The shape, which is what the operator ruled on

`rewrite_rexpr_inner` (`crates/ken-elaborator/src/modules.rs:1290`) is a single
`match` over every `RExpr` variant. Every arm constructs a fresh `RExpr` **by
value in return position**, and tests run in a debug build, where rustc does not
aggressively coalesce per-arm stack slots. So:

    stack cost  =  frame_increment  x  descent_depth
                        ^                    ^
              constant, set by the       property of the SOURCE
              size of the WHOLE             being elaborated
              language surface

⇒ **the frame carries the entire language surface at every level, through every
node type**, including nodes containing nothing A1 added.

## Why this is a requirement and not a tidy-up

Two reasons, and the first is the operator's ruling:

1. **Options (a)-(f) all re-place the boundary; none removes the tax.** Under
   this shape `RStandardOp` is not special — it is simply the increment that
   happened to cross a boundary calibrated with a 512-byte shim. Re-baselining
   is standing rent: the next `RExpr` variant does the same thing and
   `local_prebinding_preserves_legacy_map_union_stack_budget` goes red again
   under a different name, against a different node, with the same argument.
2. **The codebase has already paid for this once.** The `RMatch` arm carries the
   scar in its own comment: *"Keep recursive match descent free of per-arm
   iterator frames. The iterator/Result collection adds a chain of adapter
   frames for every nested source match; a finite checked program must not
   become unresolvable merely because the prelude gained declarations."* That is
   this defect, found earlier, fixed locally in one arm. The general form was
   never addressed.

## What is MEASURED and what is INFERRED — do not blur these

**MEASURED**, verified at A1's tip `ed47f3ec9328859ff80a66d10ab9ab28c991cdba`:

    D1_LEGACY_MAP_STACK_BYTES              2 MiB  the measured candidate-green /
                                                  inline-parent-SIGABRT BOUNDARY,
                                                  not a budget
    D1_LEGACY_MAP_STACK_RESERVATION_BYTES  512 B  bisection: 0 B both pass,
                                                  512 B candidate-green/parent
                                                  SIGABRT, 2048 B both abort

  (`crates/ken-elaborator/tests/map_build_acceptance.rs:31,37`) ⇒ the whole
  discriminating range is **512-2048 B**: a hair-trigger by construction.

    144 B    the increment A1's two arms added (0.28x the reservation)
    128 KiB  the RStandardOp descent's TOTAL stack cost

  `crates/ken-elaborator/src/modules.rs:2663`, in-tree: *"MEASURED: removing it
  does NOT fix the overflow that test reports. The overflow comes from the
  `RStandardOp` descent in `rewrite_rexpr_inner`, which is a DIFFERENT FRAME and
  which correctness requires."*

  `catalog/packages/Data/Collections/Map.ken.md` is 15,337 lines and its
  **maximum leading indentation across the whole file is 28 columns**;
  `union_from_list_acc` (`:7582`) is a short tail-recursive Ken function, not a
  nested literal. ⇒ expression nesting depth is capped near 10-15.

**INFERRED, and D0 exists to test it:** that with depth that small, **frame size
must be the dominant term** — 128 KiB over ~15 levels is order 8 KiB per frame.
Nothing has measured the frame. A Steward attempt to derive depth as
`128 KiB / 144 B ~= 910` was **refuted**: it divided a TOTAL by an INCREMENT and
produced a depth the fixture does not contain. See
`agent/memory/fleet/a-total-and-an-increment-are-two-quantities-and-the-ratio-between-them-is-not-a-measurement.md`.

⇒ **If D0 shows frame size is NOT dominant, that refutes the premise of (g) and
outranks the repair.** It is a hard stop, and it is a result.

## The hazard that makes this T1

`rewrite_rexpr_inner` is a **rewrite over every node**, and the Architect has
already ruled on what that means (`evt_2y0a3j5yjznn2`, quoted in the function):
a variant left out **does not fail to compile** — it silently stops having its
operands remapped, and an operator crossing an import then completes differently
from the same operator in its home module, *which no single-module fixture can
see*. Any restructuring of this `match` re-runs that risk across every arm at
once. The exhaustiveness argument cannot be delegated to the compiler.
