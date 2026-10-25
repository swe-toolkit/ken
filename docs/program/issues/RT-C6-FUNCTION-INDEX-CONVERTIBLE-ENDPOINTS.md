---
id: RT-C6-FUNCTION-INDEX-CONVERTIBLE-ENDPOINTS
title: "A closed, checked refl cast between data-family endpoints whose function index differs only by convertibility, such as CFn (λn.n) to CFn (λn. idN n), evaluates to Unknown, while the kernel reduces it. The Spec leader reads spec 42 C6 and §16.3.2's indexed-family case as requiring it to compute. Make the evaluator compute it without comparing closures by representation"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-C5-COMPOUND-HIGHER-ORDER-INDICES]
blocks: []
github: null
origin: "Architect RT-C5-COMPOUND D0 ruling evt_1p1wrct6dcx9 (rows 5 and 6, routed to the Spec enclave); Spec leader reading evt_3ww04vvp4k6v0: canonicity requires these casts to compute by C6. Steward-filed per COORDINATION section 2."
---

# A canonical cast at a convertible function index computes

## Objective

A closed, checked refl cast whose endpoints share an inductive head, and
whose function-valued indices are convertible but not identical terms,
evaluates to its payload in the interpreter. The kernel already reduces it.

## Settled inputs

- **The rows (Architect `evt_1p1wrct6dcx9`, measured at `1a84b2378` with the
  C5 identical-endpoint arm).** `fn idN (n : Nat) : Nat = n`.
  - Row 5: `CFn (λn.n)` to `CFn (λn. idN n)` by refl. The kernel check
    passes, the interpreter returns Unknown, and kernel whnf returns the
    value.
  - Row 6: `CPairFn (λn.n, 1)` to `CPairFn (λn. idN n, 1)` by refl. Same
    outcomes.
- **The contract (Spec leader `evt_3ww04vvp4k6v0`).** This is C6, not C5.
  §16.3.2's indexed-family case consumes the index equality and rewrites the
  constructor's index-determining argument. With `idN` transparent, δ/β
  makes the index equality canonical, so canonicity requires the cast to
  compute. This is not an evaluator η step, and no spec change is owed. If
  `idN` is opaque, or the function equality is not canonically
  decomposable, the §16.3.2 guard must be checked first.
- **Forbidden.** Comparing closures by representation or identity
  (spec 42 §1), and reading the proof value under Ω proof irrelevance.

- **D0 corrections (Architect `evt_3nqj0r8z6f497`, on `f13fdac80`).**
  - `f` is a parameter of `CFn` and `CPairFn`, not an index. The kernel
    reduces these casts by `cast_at_inductive`'s no-index-change branch,
    re-applying the constructor at the target's parameters, not by
    §16.3.2's index rewrite.
  - The kernel also computes a non-convertible parameter twin (via J and
    an axiom), so such a twin is not a refusal row. The stuck twin has a
    field whose type depends on the parameter (`CDep`).
  - Opacity does not gate this push: an opaque `idN` refl row is
    rejected by the kernel check. No spec change is owed (42 §3.3).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0's parameters-only C6 push, in `crates/ken-interp/src/eval.rs` only
(patch `c6-param-push-full.patch`, sha `71ec90ab`, verbatim in
`evt_3nqj0r8z6f497` §3):
- `cast_reduce_in` serves both production cast sites, the `Term::Cast` arm
  and `eval_j_by_endpoints`.
- `eq_type_eq` becomes a wrapper over `values_agree`, which does not refute
  a closure pair only in push mode.
- The one consumer migration is in
  `rt_c5_compound_higher_order_indices.rs` (§5).

## Acceptance

- **AC-0 (D0, Architect).** Done: `evt_3nqj0r8z6f497`.
- **AC-1.** Rows 5 and 6, closed and through `tcast`, evaluate to their
  constructor. The constructor's parameter, applied to `Zero` and
  `Suc Zero`, gives the ground identity. The captured row (`addN Zero`)
  computes too.
- **AC-1b (Architect `evt_4a0d1dbmpbdqt`, Adversary F1).**
  `tcast (CFn idf) (CFn idf) (Refl _) (MkCFn idf)` reaches
  `eval_j_by_endpoints`, gives `MkCFn`, and has the same head as kernel
  whnf.
- **AC-1c.**
  - (i) The dependent-field twin `CDep`, via J with an axiom, is Unknown
    while the kernel is stuck.
  - (ii) The parameter twin, via J with an axiom, computes `MkCFn` at the
    target parameter (applied to `Zero` it gives `Suc Zero`), matching the
    kernel.
  - (iii) The comparable-distinct String, Float and Float32 pins stay
    Unknown, unedited.
- **AC-1d (opaque).** The closed refl row with `λn. idNop n` is rejected by
  the kernel check. The J row with an axiom computes in both, with no
  opacity gate.
- **AC-2 (mutation, QA).** Each mutation reddens its named row and is
  restored byte-identically:
  - (a) both call sites back to `cast_reduce`: rows 5, 6 and 1b become
    Unknown, while the C5 rows stay values;
  - (b) `values_agree(p, q, true)` changed to `eq_type_eq`: rows 5, 6, 1b
    and C become Unknown;
  - (c) the parameter gate deleted: the three `primitive_type_cast_tests`
    redden;
  - (d) the field-type check deleted: row D(i) computes.
- **AC-3.** Full CI. Its consumers are `ken-interp --lib`, every
  `ken-interp` test binary, ken-cli `rt_c5_producer_type_slots` and the
  ken-cli lib `process_input_type_slot_tests`.
- **Carried, out of scope.** Two rows stay Unknown while the kernel
  computes them:
  - H, a field whose type contains the function;
  - I, a genuine function-valued index.

  Both need closure convertibility at the value layer, which is a separate
  design question and not a node.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A mechanism that compares closures by representation or identity.
