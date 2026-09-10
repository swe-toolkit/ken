---
id: LANG-ELAB-NESTED-FORMER-RECURSION
title: "Extend the elaborator's surface structural-recursion + totality machinery to CONSUME a recursive occurrence nested one positive-former deeper than a direct occurrence (List (Pair String Self)), so a total fold over such a value elaborates — surfacing the nested induction hypothesis the kernel former-lift now builds, and/or accepting the projection-reached nested occurrence as strictly-decreasing. Preserves totality exactly (positive/structural positions only, never a manufactured decrease). Layer-2 of the one nested-former-recursion capability whose layer-1 (kernel former-lift admission) has landed."
status: active
owner: language
size: M
gate: language
tier: T1
depends_on: []
blocks: [DS-9]
github: null
origin: "Steward-filed 2026-09-10 on the Architect ruling evt_71pwctbj3rax8 (DS-9 D3 D0 hard stop #2, thr_7scvvcxfn3cq2), grounded on foundation-implementer's DS-9 D3 D0 negative finding (post-kernel-fix, base 052286c8). The Architect ruled it a genuine remaining Language/elaborator structural-recursion gap with NO already-lawful source form on the three probed shapes, DS-9 blocked with no candidate, gated on this elaborator prerequisite (plus the standing show_int P2 floor). It is the direct surface counterpart to KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE (merged 486e9f33): the kernel now BUILDS the nested All-IH over List (Pair String Json); the elaborator surface cannot yet CONSUME it for a former-nested occurrence. Soundness-sensitive (totality is trust-critical) -> Architect required review, like the kernel fix. The Architect ruled the lane re-decision bounded/grounded and NOT needing the operator's return."
---

> # A fail-closed elaborator INCOMPLETENESS at the totality layer. The kernel
> # former-lift now admits and BUILDS the nested induction hypothesis for
> # List (Pair String Self) (layer-1, landed). The elaborator's surface
> # structural-recursion machinery cannot yet CONSUME a recursive occurrence
> # reached one positive-former deeper than direct, so a total fold over such a
> # value is rejected as non-terminating / out-of-scope. The fix completes the
> # capability; it does NOT relax totality. The soundness GATE (totality
> # preserved, positive-nesting only) is the load-bearing constraint and the
> # Architect's review criterion.

## The measured defect (Architect evt_71pwctbj3rax8; DS-9 D3 D0 HS#2)

The kernel former-lift admission fix landed (base `052286c8` carries the
`guest_params_from_shape` WHNF alignment in `crates/ken-kernel/src/inductive.rs`),
so the six-arm `Json` match now BUILDS and the direct `List Json` array fold
EXECUTES. The new wall is a DIFFERENT layer: the elaborator's surface
structural-recursion + totality machinery in `crates/ken-elaborator/src/elab.rs`
(`RExpr::RRecursiveResult` / `ElabError::StructuralResultOutOfScope`, and the SCT
size-change gate). For `JsonObject`'s `List (Pair String Json)` the recursive
`Json` is nested inside a `Pair`, and every natural surface form fails for a
STRUCTURAL reason (Architect's three probes):

- `fold_chars` over the `pair_snd member` -> `NotTerminating("SCT: idempotent
  self-loop has no strictly-decreasing parameter")`. SCT does not track a
  recursive occurrence reached by PROJECTING a former-typed constructor argument
  as strictly-decreasing — even though `pair_snd member : Json` genuinely IS a
  strict subterm of the `JsonObject` value.
- `recursive result for member` -> `StructuralResultOutOfScope`. The
  `RRecursiveResult` binding is scoped to DIRECT recursive-occurrence constructor
  arguments; `member : Pair String Json` is not a direct recursive occurrence of
  `Json` (the `Json` is buried in the `Pair`), so no IH is surfaced under
  `member`.
- Nested tuple-pattern destructure of `member` — still does not surface the
  nested recursive result.

So the kernel now BUILDS the nested IH (the All-lift over `List (Pair String
Json)`), but the elaborator SURFACE cannot CONSUME it for a former-nested
occurrence: `RRecursiveResult` / SCT reach only direct recursive-occurrence
children. No already-lawful form exists on the three probed shapes plus this
structural reason; array-only is dishonest for an array/object increment, and
restructuring off `List (Pair String Json)` is a forbidden carrier change.

D0 RE-MEASURES the exact loci at pickup (line numbers drift; the Architect's
`:3357`/`:8978` are stale — as landed the anchors are `RRecursiveResult` at
`elab.rs:7108`, `StructuralResultOutOfScope` at `:7116`, and the SCT gate around
`:11427`–`:12175`; confirm at pickup).

## Soundness — this is INCOMPLETENESS at the totality layer, not a missing guard

SCT rejecting `pair_snd member` is CONSERVATIVE: it cannot prove the descent, so
it rejects — it rejects a well-formed TOTAL fold, never accepts a non-terminating
one. `pair_snd member : Json` genuinely IS a strict subterm of the `JsonObject`
value; the machinery simply does not recognise a subterm reached through a
positive former-nesting.

**The load-bearing constraint (Architect's review criterion).** The extension must
recognise a recursive occurrence reached through a POSITIVE former-nesting as
strictly-decreasing / IH-consumable, and ONLY through positive/structural
positions — never manufacturing a "decrease" that is not a genuine subterm, and
never through a negative position. Totality must NOT weaken: every shape the
elaborator rejects today for a genuine non-termination reason still reds after
the fix. This is the exact positivity-preservation discipline that gated the
kernel fix, now at the totality layer. It is not a carrier change and needs no
forbidden re-encoding.

## Deliverables (D0-first)

- **D0 — reproduce + locate the consumption boundary.** At the pickup SHA,
  reproduce the DS-9 `JsonObject` (`List (Pair String Json)`) total-fold reject on
  the three probed surface forms and confirm the direct `List Json` fold
  elaborates. Locate exactly where `RRecursiveResult` scoping stops at the direct
  recursive-occurrence child, and where the SCT size-change gate fails to treat
  the projection-reached nested occurrence as strictly-decreasing. Emit the exact
  positive-former-nesting descent the fix adds and the negative-position /
  non-subterm boundary it must NOT cross. Hard-stop to the Steward + Architect if
  the gap is not a pure positive-nesting consumption extension (e.g. if surfacing
  the nested IH would require admitting a non-genuine-subterm decrease, or
  touching a negative position, or a relation/kernel change).
- **D1 — complete the positive-nesting consumption.** Extend the surface
  structural-recursion machinery so a total fold over a recursive occurrence
  nested inside a positive former elaborates. The Architect names two coordinated
  sub-capabilities; the Language team + enclave pick the cleanest (either or
  both):
  - **(B, preferred) surface the nested IH.** Make `recursive result for` reach an
    occurrence nested inside a positive former, consuming the All-IH the kernel
    now builds — the direct surface counterpart to the landed kernel fix.
  - **(A) extend SCT / structural-descent.** Accept a projection-reached nested
    recursive occurrence (`pair_snd member`) as strictly-decreasing.
  No relaxation of totality; every previously-rejected genuine non-termination
  stays rejected exactly as today.

## Acceptance criteria (each with its control)

- **AC-NESTED-FORMER-FOLD-ADMITTED (the capability).** A total fold over a
  recursive occurrence nested inside a positive former (`List (Pair String Self)`,
  and the general shape) elaborates and type-checks. Control: the exact DS-9 D0
  `JsonObject` total fold that reds today (`StructuralResultOutOfScope` /
  `NotTerminating`) goes green; the paired direct-occurrence fold (`List Self`,
  the `JsonArray` case) stays green (no regression).
- **AC-EXECUTES-AT-RUNTIME (the layer-3 pre-empt — Architect §1b).** The admitted
  nested object fold does not merely ELABORATE — it EXECUTES: a runtime/codegen
  execution probe drives the nested `List (Pair String Json)` fold to a value
  (native and, where the harness runs it, interpreter), confirming the generated
  nested IH is actually consumed at runtime, not only admitted. Control: the probe
  exits successfully and returns the expected fold result; a mutation that drops
  the nested-IH consumption reds it. (Kernel QA's own residual flagged that the
  kernel fix exercised ADMISSION, not runtime use of the generated IH — this AC
  pre-empts a layer-3 HS#3.)
- **AC-TOTALITY-PRESERVED (the soundness gate — hard).** Every shape the
  elaborator rejects today for a genuine non-termination reason still reds after
  the fix. Control: a non-terminating / non-subterm probe (a self-loop with no
  strictly-decreasing parameter; a "decrease" that is not a genuine subterm; a
  negative-position occurrence) still rejects; a mutation that lets the descent
  treat a non-subterm as decreasing, or cross into a negative position, reddens
  this control. The fix that admits `List (Pair String Self)` must NOT admit any
  non-genuine-subterm decrease — this is the Architect's review criterion, and a
  control that only exercises the positive-nested shape does not discharge it.
- **AC-NO-TCB-WIDENING.** The change completes an existing elaborator
  resolution/totality path; it adds no axiom/postulate/primitive, no relation
  re-index, and no new trusted surface. Control: `trusted_base()` differential
  delta zero; the diff is confined to the structural-recursion / SCT machinery in
  `elab.rs` (and its direct callees), not a kernel or relation change.
- **AC-AFFECTED-CLOSURE.** Re-run every elaborator target whose structural-
  recursion / totality / SCT path this touches (targeted via `scripts/ken-cargo`,
  never `--workspace`; CI is the workspace verdict). Retain every existing
  totality/SCT test unweakened.

## Gate, reviewer, sequencing

`gate: language`, but SOUNDNESS-SENSITIVE (totality is trust-critical), so the
**Architect owns the soundness gate** (totality preservation, positive-nesting
only — AC-TOTALITY-PRESERVED is THE criterion, like the kernel fix) and Language
QA owns the build. On the candidate: Architect required review + Language QA +
Adversary on the exact SHA, then Steward M1-M4 -> lieutenant. `blocks: DS-9` — on
this node's landing the Steward re-releases DS-9's D3+ (the show_int floor remains
a separate, independent P2 gate on the complete number leaf).

## The one capability, layer by layer (Architect §1b)

DS-9's blocker is ONE capability gap manifesting LAYER BY LAYER, not a kernel bug
then a surface bug: **Ken does not yet support recursion over a recursive
occurrence nested one positive-former deeper than direct (`List (Pair String
Self)`).** Shared predicate: "a recursive occurrence one former-nesting deeper
than direct is unsupported at layer L."

- Layer-1 — kernel former-lift ADMISSION. LANDED (KERNEL-INTRINSIC-ALL-LIFT-
  NESTED-POSITIVE, 486e9f33).
- Layer-2 — elaborator structural-recursion CONSUMPTION + totality. THIS node.
- Layer-3 — runtime/codegen execution of the generated nested IH. Folded into
  this node's acceptance as AC-EXECUTES-AT-RUNTIME (the Architect's §1b
  recommendation to pre-empt an HS#3); if a genuine runtime/codegen gap surfaces
  under that probe, it becomes its own node and (per §1a) the 3rd hard-stop on the
  SAME nested-former question triggers a research pull.
- Layer-4 — conformance. Downstream, not enumerated here.

Do not enumerate one layer per hard-stop as a fresh capability; this is the
coherent scoping the Architect asked for.

## Hard stop

Route to the Steward + Architect if D0 finds the gap is not a pure positive-
nesting consumption extension — if surfacing the nested IH / accepting the
projection-reached occurrence cannot be done without admitting a non-genuine-
subterm decrease, touching a negative position, or requiring a relation/kernel
change, the shape is not what this frame assumes and the soundness gate is at
risk. Land nothing on that axis until the Architect rules.
