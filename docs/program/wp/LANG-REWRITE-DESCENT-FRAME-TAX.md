# `LANG-REWRITE-DESCENT-FRAME-TAX` — frame

**Owner:** language. **Size:** M. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `1be846b2d062e23be5cdf6ebd789837feeb637ab`.

> ## MEASURE FIRST. THE REPAIR'S PREMISE IS INFERRED, NOT MEASURED.
>
> `D0` is a measurement and it comes before `D1`. The claim that frame size
> dominates the descent's stack cost has never been measured — it is an
> inference from a small fixture depth. **If `D0` refutes it, that outranks the
> repair and the node stops there with a result.** A candidate whose roster
> contains `D1` but not a `D0` measurement at the base has not advanced this
> node.

## 1. Fixed inputs

    crates/ken-elaborator/src/modules.rs
      rewrite_rexpr_inner        the taxed frame        (~:1290)
      rewrite_rexpr              the wrapper            (~:1280)

    crates/ken-elaborator/tests/map_build_acceptance.rs
      D1_LEGACY_MAP_STACK_BYTES              2 MiB      (~:31)
      D1_LEGACY_MAP_STACK_RESERVATION_BYTES  512 B      (~:37)
      local_prebinding_preserves_legacy_map_union_stack_budget   (~:1487)

    catalog/packages/Data/Collections/Map.ken.md         the D1 fixture

Re-derive by **symbol**, not by line — these coordinates were measured on this
tree at the ground SHA and coordinates are perishable.

**Two comparison points**, both needed:

    BASE     origin/main  1be846b2d062e23be5cdf6ebd789837feeb637ab
    A1 TIP   ed47f3ec9328859ff80a66d10ab9ab28c991cdba
             (LANG-STANDARD-INFIX-CALL-COMPLETION; adds the RStandardOp and
              RInfixSpine arms. DO NOT BUILD ON IT -- read it, measure it,
              never branch from it.)

## 2. Deliverables

- **`D0` — THE MEASUREMENT, at the BASE and at the A1 TIP.** Three numbers at
  each point, by one stated method:
  1. `rewrite_rexpr_inner`'s stack frame size (`-Z print-type-sizes`, or a
     stack-pointer probe at function entry — state which and why).
  2. The maximum descent depth reached elaborating `Map.ken.md`.
  3. The descent's total stack consumption.
  **State the method once and use the identical method at both points.** A
  before/after pair from two methods measures the methods.
- **`D1` — THE REPAIR.** Stop `rewrite_rexpr_inner`'s frame carrying the whole
  language surface. Candidate approaches, implementer's call with the reason
  stated: move large/cold arms behind `#[inline(never)]` helpers so their
  temporaries leave the hot frame; box the large arm bodies; or convert the
  descent to an explicit worklist so depth stops being stack depth. Pick on the
  `D0` evidence, not on this list's order.
- **`D2` — REMEASURE.** `D0`'s three numbers again, same method, after `D1`.
  Report as a split, never a sum: which number moved and by how much.
- **`D3` — THE CONSEQUENCE FOR A1, REPORTED NOT ACTED ON.** With `D1` landed,
  does A1's arm still trip
  `local_prebinding_preserves_legacy_map_union_stack_budget`? Answer it and
  stop. **The re-baseline decision is the operator's and is not in this node.**

## 3. Acceptance criteria

- **`AC-1` — A BEFORE/AFTER PAIR FROM ONE METHOD.** `D0` at the base is
  recorded **before** `D1` is written, and `D2` re-runs the identical method.
  *(Control: state the method in one sentence such that a second person could
  re-run it. A measurement whose method is described only as "measured" is not
  reproducible and does not satisfy this.)*
- **`AC-2` — BEHAVIOUR PRESERVED ACROSS EVERY `RExpr` VARIANT, ARGUED WITHOUT
  THE COMPILER.** `rewrite_rexpr_inner` is a rewrite over every node: a variant
  that stops being remapped **does not fail to compile** (Architect,
  `evt_2y0a3j5yjznn2`), it silently stops having its operands remapped, and the
  divergence is only visible across a module boundary.
  *(Control: enumerate the `RExpr` variants at the base and at the candidate and
  show the two sets are equal AND that each variant's operands are still
  rewritten — a diff of the arm list, not an assertion that the match is
  exhaustive. `match` exhaustiveness proves every variant is HANDLED; it proves
  nothing about whether it is REWRITTEN.)*
- **`AC-3` — THE DETECTOR IS NOT EDITED IN THIS NODE.** No change to
  `D1_LEGACY_MAP_STACK_BYTES`, `D1_LEGACY_MAP_STACK_RESERVATION_BYTES`, or
  `local_prebinding_preserves_legacy_map_union_stack_budget`.
  *(Rationale, not ceremony: the detector is this node's INSTRUMENT. Editing an
  instrument in the same candidate that changes what it measures destroys the
  before/after pair `AC-1` exists to produce. If the repair cannot be shown
  without moving the detector, that is a `§4` hard stop.)*
- **`AC-4` — NO REGRESSION.** Green in CI, never a local `--workspace` run
  (`COORDINATION §12`). Local work is `scripts/ken-cargo -p ken-elaborator`,
  and `--test map_build_acceptance` for the instrument. Nothing wider.
- **`AC-5` — NO NEW `#[ignore]` ANYWHERE.**

## 4. Hard stops — report, do not work around

- **`D0` shows frame size is NOT the dominant term.** That refutes the premise
  of option (g) and **outranks the repair**. Stop and report; the options fork
  returns to the operator with a measurement it did not have.
- The repair requires changing `RExpr`'s shape, its variant set, or the
  elaborator's public surface.
- `D1` lands and `D2` shows the frame did not shrink — a finding that the
  mechanism statement is incomplete, and it outranks finishing the node.
- `D3` shows A1's arm still trips the detector. **Report it. Do not re-baseline
  and do not re-scope A1** — that decision is the operator's.

## 5. Symptom inventory — ARMED AT FILING

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...
(empty at filing)
```

**Hard-stop count on this WP: 0.** The research pull fires at 3.

## 6. Contention

`crates/ken-elaborator/src/modules.rs` is the contended file. **A1 edits it and
A1 is HELD behind this node** (`blocks:` set at filing), so the ring owns the
file for the duration — but A1's candidate at `ed47f3ec9` will need rebasing
onto this node's landing, and the bigger that diff the worse that rebase. Keep
`D1` as small as the measurement allows; this is not the node in which to tidy
`modules.rs`.

**Do not branch from `ed47f3ec9`.** Read it and measure it; build from
`origin/main`.

No contention with L1 (`RT-PLANNER-KRET-GRAFTED-SPINE`, `ken-runtime` planning)
or L3.

## 7. What is settled and is not this node's work

The option fork is CLOSED by operator ruling: **(g), then remeasure.** Options
(a) re-baseline the constant, (b) retire or re-scope the detector, and (c) A1
does not land with the arm are **not live** and are not to be re-argued inside
this node. If `D0` refutes (g)'s premise, that reopens the fork **at the
operator**, not at the ring.

## 8. D0 measurement — recorded before D1

**Method, applied identically at both points:** build the exact
`map_build_acceptance` test in the debug test profile, use `nm -C` plus
`objdump -Cd` to read `rewrite_rexpr_inner`'s uninstrumented prologue stack
allocation, then run a disposable eight-MiB clone of the D1 fixture with an
entry `rsp` probe and a thread-local active-depth guard. Correct the probe's
outermost-to-deepest entry span by subtracting its measured per-frame overhead
for the intervening frames, then add one uninstrumented frame:
`span - (depth - 1) * (probe_frame - frame) + frame`. The probe and cloned test
were measurement-only and are absent from the candidate.

| point | frame | max depth | corrected total stack | frame share |
|---|---:|---:|---:|---:|
| base `1be846b2d` | 41,544 B | 46 | 1,922,648 B | 99.40% |
| A1 `ed47f3ec9` | 44,136 B | 46 | 2,041,880 B | 99.43% |

The base probe frame was 41,608 B and reported a 1,883,984 B entry span; the
A1 probe frame was 44,216 B and reported 2,001,344 B. At both points the
non-`rewrite_rexpr_inner` remainder is 11,624 B. A1 therefore adds 2,592 B per
frame and 119,232 B at the unchanged depth. The frame is the dominant term, so
D0 confirms rather than refutes D1's premise.

## 9. D2 remeasurement and D3 consequence

D2 used the identical §8 method after D1. The uninstrumented frame is
19,080 B, the maximum descent depth remains 46, and the corrected total stack
is 1,019,624 B. The disposable probe frame was 19,160 B and reported a
1,004,144 B entry span.

The split against the base is: frame **down 22,464 B** (54.1%), depth
**unchanged**, and corrected total stack **down 903,024 B** (47.0%). The
per-arm non-inlined calls move variant-specific work out of the dispatcher, so
the non-dispatcher part grows from 11,624 B to 141,944 B while the recursive
whole-surface frame falls by more than half.

For D3, A1 tip `ed47f3ec9` plus D1 was assembled temporarily at `08c0e1c07`.
`local_prebinding_preserves_legacy_map_union_stack_budget` passed unchanged at
its stated two-MiB stack. D1 therefore prevents A1's arm from tripping the
existing detector; no re-baseline or detector edit is proposed here.

## 10. AC-2 arm and operand audit

The base and candidate each have the same 29 ordered match arms over 28 unique
variants; `RCon` deliberately has a kernel-head arm followed by its resolving
arm. The exact sequence at both points is:

```text
RCon RCon RVar RPatternAlias RRecursiveResult RUniv RApp RLam RLet RAsc
ROld RCell RBecomes RNumLit RStr RCharLit RByteStr RBinOp RInfixSpine
RMatch RIf RPair RRecord RPosProj RProj RPi RArrow RAttachedProofRef RTrunc
```

The operand audit is:

| variant group | preserved rewrite |
|---|---|
| `RCon` | kernel head stays canonical; other names use `resolve_ref` |
| `RApp` | function uses `rewrite_rexpr_inner`; argument uses `rewrite_rexpr` |
| `RLam`, `ROld`, `RBecomes`, `RTrunc` | body, value, or inner expression uses `rewrite_rexpr` |
| `RLet` | optional type uses `rewrite_rtype`; RHS and body use `rewrite_rexpr` |
| `RAsc` | expression uses `rewrite_rexpr`; type uses `rewrite_rtype` |
| `RBinOp`, `RArrow` | both expression operands use `rewrite_rexpr` |
| `RInfixSpine` | every operand rewrites; user operator names use `resolve_ref` |
| `RMatch` | scrutinee, patterns, guards, and bodies all rewrite |
| `RIf` | condition and both branches use `rewrite_rexpr` |
| `RPair` | every component uses `rewrite_rexpr` |
| `RRecord` | optional base and every field value use `rewrite_rexpr` |
| `RPosProj`, `RProj` | projected expression uses `rewrite_rexpr` |
| `RPi` | domain uses `rewrite_rtype`; codomain uses `rewrite_rexpr` |
| `RAttachedProofRef` | subject and proof name use `resolve_attached_ref` |
| all remaining variants | no recursive expression or reference operand |

D1 changes only where those existing operations execute: each recursive arm's
body is now a distinct non-inlined helper monomorph, leaving the dispatcher's
arm list and operand transformations intact.
