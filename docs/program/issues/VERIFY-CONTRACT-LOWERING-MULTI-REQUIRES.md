---
id: VERIFY-CONTRACT-LOWERING-MULTI-REQUIRES
title: "A declaration with two requires clauses is rejected by the kernel, because the body's proof lambdas are not shifted under the earlier ones, and an ensures goal is closed without the requires premises. Lower contracts as spec 21 elabView states: λ(Δ). λ(p̄). b, with ensures checked under the requires"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: []
blocks: [LANG-REFINED-PARAM-REQUIRES-DESUGAR, VERIFY-CALL-SITE-PRECONDITION-DISCHARGE]
github: null
origin: "Adversary evt_433905phzqcxw (hunt of a9e16c3a1), fail-closed; the ensures-premise item is the Architect carry in KERNEL-INT-DIV-MOD-NATIVE's Not-this-WP list. Since a9e16c3a1, requires is the only discharge for a / or % divisor, and SPEC-REFINED-PARAM-REQUIRES-DESUGAR makes every refined parameter a requires. Steward-filed per COORDINATION section 2."
---

# Contracts with several requires clauses lower soundly

## Objective

A non-recursive declaration with any number of `requires` clauses
elaborates and kernel-checks. Each `ensures` goal and each obligation
raised while elaborating an `ensures` is closed under the `requires`
premises.

Not this WP: a recursive declaration with `requires`
(`fn recurse (n : Int) : Int requires Equal Int n n = recurse n` gives
`NotAFunction`, Verify QA `evt_25tbd7ab2ytxt`). The name is staged at
`carrier_ty`, and no call site supplies premise proofs. That failure
predates this WP and fails closed. It is VERIFY-CALL-SITE-PRECONDITION-DISCHARGE
(Architect `evt_4qb4fh6bbnyhg`).

## Settled inputs (Adversary `evt_433905phzqcxw`, read on `775d721c3`)

- `elaborate_view_with_spec` (`elab.rs:15034`) elaborates every requires
  core at parameter depth (`install_requires_assumptions`, `:10232`).
- **Body.** Phase 4 (`:15181`) wraps `full_ty` as `Π(req, weaken(full_ty,
  1))`, which is correct. It wraps `full_body` as `Term::lam(req.clone(),
  full_body)` with no shift, so the i-th proof lambda's domain sits under i
  earlier proof binders while still indexed at parameter depth.
  - Repro: `fn f (n : Int) (d : Int) : Int requires Equal Int n 1 requires
    Equal Int d 2 = n` gives KernelRejected `TypeMismatch { expected: Int,
    found: Eq Int @2 1 }`.
  - Of four two-clause declarations (M1-M4 in the finding), all four are
    rejected. No suite, catalog package or conformance row declares two
    clauses.
  - The Adversary's mutation `Term::lam(weaken(req, i), full_body)` over
    `enumerate().rev()` admits all four with correct binder types.
- **Ensures.** Phase 2 (`:15132`) builds `ens_ctx` as the parameters plus
  `result`, with no `requires`. Its goal is `close_goal(&param_ctx, …)`.
  - Spec 21 §1's example `requires Not (Equal Int d 0) ensures Equal Int
    (result * d + n % d) n = n / d` raises an ensures-side PartialPrim
    goal `Π n d result. NonZeroDivisor d`. That goal is false at `d = 0`,
    so it can never be discharged.
- **Spec.**
  - 21 §1: "Multiple requires/ensures clauses conjoin."
  - 21 elabView: `Γ := extend(Γ, φᵢ')` for each clause, and `coreTm :=
    λ(Δ). λ(p̄). b : Π(Δ). Π(φ̄). B`.
  - The ensures clauses are elaborated in that extended Γ.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. Phase 4 shifts each requires domain under the earlier proof binders.
2. Ensures clauses and their goals are elaborated and closed in the
   parameters plus the requires premises plus `result`, with the body
   weakened to match.
3. Every same-kind site gets the same treatment (check 7): grep
   `req_cores`, `close_goal(&param_ctx`, and the proc, instance-method and
   other declaration paths that wrap requires lambdas or close contract
   goals.

## Acceptance

- **AC-1.** The Adversary's M1-M4 elaborate and kernel-check.
  - M1 (`requires Not (Equal Int d 0) requires Not (Equal Int n 0) = n /
    d`) has 0 obligations, and its second domain is `Not (Eq Int n 0)` read
    under `(n, d, h1)`.
  - The twin `requires Not (Equal Int n 0) requires Equal Int d d = n / d`
    still raises exactly one divisor obligation at `d`.
- **AC-2 (Architect `evt_4nz89dqvempe1`).** On spec 21 §1's `divide`
  example, take T0 as `trusted_base()` just before `elaborate_file` and T1
  just after.
  - `T1 \ T0` is exactly the set of reported obligation `hole_id`s.
  - That set is one `Ensures` hole with goal `Π n d (h : Not (Eq Int d
    0)). …`, and no `PartialPrim` hole.
  - Assert membership and kinds, not a count. A fresh `ElabEnv`'s base is
    unchanged (the `lang_prelude_collections` pin stays green).
- **AC-3 (falsifiers; each must redden).**
  - F1: revert the Phase 4 shift; M1-M4 are KernelRejected.
  - F2: drop the premises from the ensures context; the AC-2 example raises
    the PartialPrim obligation again.
  - F3: shift by one too many; the AC-1 twin loses its obligation, or is
    rejected.

## Stop conditions

- The repair needs a kernel change, or `trusted_base()` moves: stop to the
  Architect. Moving means the fresh-env base changes, or a postulate
  outside the reported obligation holes appears, or the hole set changes
  outside this population. A PartialPrim hole leaving an ensures goal
  that now sits under its requires is the intended decrease.
- An existing pin changes verdict outside the multi-clause and ensures
  population: stop with the row.

## Closeout

Merged `958121efa` from exact `d32935726` (FULL CI green, run
`37121725445`). `elab.rs` is a three-way merge with LEAF-PARITY; the main
tree equals the routed merge tree, and the test file's blob matches.
Gates: Verify QA `evt_7ekazehv14sts`, Architect `evt_6xsth0x21cr9d`,
Decision `dec_72m6ps4nhezhc`. VERIFY-CALL-SITE-PRECONDITION-DISCHARGE
builds on it.
