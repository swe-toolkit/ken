---
id: RT-PLANNER-GRAPH-FOUNDATION-SPLIT
title: "Move the planner's shared substrate out of the static-transition monolith — the root plan type and the identity vocabulary that all six planner domain slices quote"
status: ready
owner: runtime
size: TBD
gate: none
depends_on: [RT-BACKEND-SPLIT-CENSUS]
blocks: []
github: null
origin: Cut item 3 of RT-BACKEND-MODULE-SPLIT, filed 2026-08-17 once RT-BACKEND-SPLIT-CENSUS merged and supplied the evidence the campaign deliberately withheld filing ahead of. Framing constraints binding per RT-BACKEND-MODULE-SPLIT:330-359 (operator, 2026-08-08). Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # `ready` 2026-08-17 — CUT ITEM 3, THE FIRST PLANNER SLICE
>
> **Frame: `docs/program/wp/RT-PLANNER-GRAPH-FOUNDATION-SPLIT.md`.** It is
> shovel-ready for `D0` and deliberately unsized past it.
>
> ### WHY IT IS FILED NOW AND WAS NOT BEFORE
>
> [[RT-BACKEND-MODULE-SPLIT]] filed only cut items 1 and 2 on purpose: *"filing
> all sixteen now would create work ahead of the evidence that sizes it."* The
> evidence was the census, and **[[RT-BACKEND-SPLIT-CENSUS]] merged** — its
> type-ownership inventory records all 76 planner-owned type declarations with
> visibility and full external consumer sets. **The constraint that deferred this
> node has lapsed; the node is not new work.**
>
> ### THE CENSUS IS CURRENT FOR THE PLANNER, AND THAT IS MEASURED NOT ASSUMED
>
> The inventories pin measurement SHA `4de48651`. Between it and `c03331ad8`,
> `cranelift_backend/planning/` has **zero commits and an empty diff.** ⇒ The
> planner rows are current, so `D0` starts from the census rather than re-taking
> it. **Re-confirm with one command at pickup** — the frame's §1 carries it, and
> a non-empty diff means the fixed inputs are stale.
>
> ### `size: TBD` IS DELIBERATE. Do not fill it in from a line count.
>
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] landed cleanly because the census's
> `D6` had already returned a **bounded ownership proof** before anything moved.
> **No such proof exists for the planner foundation.** `D0` produces it, and the
> Steward cuts `D1` onward against what it returns.
>
> **`RT-BACKEND-MODULE-SPLIT:330-359` bars carrying today's line counts into this
> frame**, and the landed research reports independently warn against optimizing
> for equal-sized files. The frame omits the counts for that reason, not by
> oversight.

## What it owns

**The smallest set that unblocks cut items 4-9** — the six planner domain slices
(units/ABI, occurrences, continuations, aggregates, effects, joins/traps). They
are separable from each other and **not** separable from the root plan type and
the identity vocabulary they all quote.

The starting hypothesis, read off the census and stated so `D0` can refute it:
`StaticTransitionPlan<'src>` (`planning/static_transition.rs:2638`) plus the
shared identity and coordinate types. **A type only one domain references belongs
to that domain's slice, not here.**

## What it is not

- **Not a planner mega-diff.** `RT-BACKEND-MODULE-SPLIT:89-93` — a census merge
  permits one frame with independently reviewable commits and nothing more.
- **Not `#8` closure.** One accepted phase partial among eighteen.
- **Not a venue for the IR architecture.** The frame cites both landed research
  reports for the Architect; **reference is not adoption**, and this stays a
  behaviour-preserving split unless the Architect rules otherwise.

## The known trap

`planning.rs` carries `#[cfg(test)]` and `#[cfg(any(test, feature = ...))]`
gated re-exports, and warns three times in its own doc that **an ungated use of a
`cfg(test)`-gated re-export is an unresolved import in the production build that
the test profile cannot see.** A targeted test run is precisely the instrument
that cannot catch it. Same class as the `cfg`-context and path-relative
resolution gap the Adversary named on the primitive-lowering split.

## The carve pattern already exists in this file

`abi.rs` and `semantic_ir.rs` were carved out of the same monolith by
[[RT-NATIVE-FNSPLIT]]'s `B1`/`B1R` recuts, declared at
`planning/static_transition.rs:8-9`. **Copy that shape; do not invent another.**
