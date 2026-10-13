---
id: RT-INTERP-J-ERASED-ENDPOINT-COST
title: "The interpreter's J arm evaluates both endpoints of the erased equation before the base, so a J that transports a recursive result along a lemma about that result costs 3^n where the source and the kernel are linear. Reduce J without computing erased endpoints the result does not need"
status: active
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-INTERP-J-COMPUTATIONAL-REDUCTION]
blocks: []
github: null
origin: "Adversary M8 finding evt_4zewgpbcv6dhg on 44443145c (RT-INTERP-J-COMPUTATIONAL-REDUCTION): correctness, practical non-termination; every value matches the kernel. Violates spec 42 (40-runtime/42-evaluation.md:88-113: erased components have no runtime computation). Check 8: a value-only acceptance passes on the exponential configuration. Steward-filed per COORDINATION section 2."
---

# J does not compute its erased endpoints

## Objective

A J whose evidence is an erased Ω proof evaluates in time proportional to
the source's own computation whenever its transport is statically regular.
Values stay identical to the kernel's. A motive that mentions its index at
endpoints that differ as terms keeps spec 42's cast check, which needs both
endpoints. That case is not an erased computation (D0 `evt_6tcebrtrvk1a9`).

## Settled inputs (Adversary `evt_4zewgpbcv6dhg`, on `44443145c`)

- **The locus.** `eval_j` (`crates/ken-interp/src/eval.rs:1456`) matches
  the erased evidence `Term::Eq(_, lhs, rhs)` and evaluates both endpoints
  first. It returns the base when they are equal, and otherwise
  cast-reduces through C5. `erased_j_equation`
  (`crates/ken-elaborator/src/omega_erasure.rs:362`) supplies the
  endpoints from the evidence's Eq type.
- **The cost.** The repro is `theorem rk (k : Nat) : Equal Nat k k = Refl`
  and `fn f (n : Nat) : Nat = match n { Zero ↦ Zero; Suc k ↦ J (λp _. Nat)
  (Suc (f k)) (rk (f k)) }`. Each level evaluates `f k` three times: in
  the base and in each endpoint.
  - `eval_checked` of `f n` takes 6.8 ms at n=4, 370 ms at n=8 and 28.9 s
    at n=12, about 3x per step.
  - The kernel proves `Equal Nat (f n) n` in 3.5 to 7.7 ms for n from 8
    to 16. `obs::j_reduce` whnfs the evidence to `Refl` and never
    evaluates the endpoints.
  - **Controls.** The plain `Suc (f k)` form runs n=10 in 1.2 ms. The
    let-bound `rk r` form runs it in 1.6 ms.
- **Siblings.** A dependent motive evaluates the same endpoints. The
  kernel-side analogue is `KERNEL-J-NONREFL-ENDPOINT-SHARING`, which does
  not cover the interpreter.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect).** Rule how the arm avoids the erased computation.
   Candidates include: skipping the endpoints when the motive does not use
   the index, which C5 regularity already allows; sharing the base's and
   the endpoints' evaluation; and deciding the transport without evaluating
   the endpoints. The D0 covers the dependent-motive sibling. It also says
   whether the endpoints' evaluation is ever needed for a correct value.
2. **The ruled repair** (D0 `evt_6tcebrtrvk1a9`). `eval_j` returns
   `eval(base)` without evaluating the endpoints, the motive or `tt` when
   `j_transport_is_statically_regular` holds: the endpoints are the same
   term, or the motive's body, after peeling `Ascript`, never mentions its
   index. Otherwise it calls `eval_j_by_endpoints`, which is the existing
   path unchanged. `erased_j_equation` does not change.

## Acceptance

- **AC-1.** Rows f1 (the repro), f2 (dependent motive `λp _. K Nat p`,
  the same endpoints) and f3 (constant motive, `lemi` with distinct
  endpoints) each, at n = 20 under `eval_checked`:
  - return `Nat 20`;
  - finish in under 5 s in a debug build, on a thread with
    `recv_timeout`, so that a mutant fails instead of hanging;
  - are kernel-paired at n = 16, with the n−1 twin refused;
  - keep `trusted_base()` unchanged.
- **AC-2 (control).** Every `rt_interp_j_computational_reduction` row and
  the four `cat_graphs_dependency` cycles keep their values and kernel
  pairings. `cc7_argparse_acceptance` stays green. The four
  `computational_j_tests` units stay green. `eval_j`'s frame
  (`sub $N,%rsp`, measured at base and candidate) is not above base.
- **AC-3 (mutation, QA).** Three mutations, and no control reddens under
  any of them:
  - calling `eval_j_by_endpoints` directly reddens f1, f2 and f3;
  - dropping the motive half reddens f3 only;
  - dropping the `lhs == rhs` half reddens f2 only.

## Stop conditions

- The repair needs a kernel, `trusted_base()` or spec change.
- The interpreter's value differs from the kernel's on any row.
- To the Architect: any control's value changes, an AC-1 row misses the
  bound with the ruled code, or `eval_j`'s frame exceeds base.
- The f4 residual (a dependent motive with distinct endpoints) stays on
  the endpoint path. Removing it changes spec 42's cast rule, which is out
  of scope.
