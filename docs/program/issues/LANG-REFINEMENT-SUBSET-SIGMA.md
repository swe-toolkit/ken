---
id: LANG-REFINEMENT-SUBSET-SIGMA
title: "Refinements are carrier types, so List Int is accepted as List Five, a lambda checked at a named refinement over a Pi carrier emits no obligation, and a refined binder converts with its carrier. Elaborate refinements to the core subset Σ, introduce by pair plus obligation, forget by Proj1, and retire the carrier guard machinery"
status: draft
owner: language
size: L
tier: T1
gate: architect
depends_on: [SPEC-REFINEMENT-SUBSET-SIGMA, LANG-PATH-CONDITION-EVIDENCE, LANG-REFINEMENT-PROOF-ERASURE, VERIFY-TRANSITIVE-HONESTY]
blocks: []
github: null
origin: "Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W5, kernel hunk K-a. Supersedes LANG-NAMED-REFINEMENT-TYPE-ARGUMENT, LANG-LAMBDA-PI-CARRIER-REFINEMENT-INTRODUCTION and LANG-NAMED-REFINED-BINDER-FIRST-CLASS."
---

# Refinements are subset Σ types

## Objective

A refinement is `Σ x:A. φ` with `φ` at Ω, in the kernel. Forging a
refined value is a kernel type mismatch, not a missed obligation.

## Settled inputs

- The Architect's design note `evt_30frdrmj45ehg`, DESIGN 1-5, and the
  spec as amended by `SPEC-REFINEMENT-SUBSET-SIGMA`.
- No conversion change is needed (Architect probe on `1153a9fc6`).
- `Char` assumes the carrier at three kernel sites:
  `register_checked_char_carrier` (`check.rs:1919`), `checked_char_literal`
  (`check.rs:2033`), and the literal String view (`conv.rs` ~221).

This frame is `draft` until its four dependencies land. It is re-anchored
then.

## Deliverable

- Designs 1-4: representation, introduction, evidence, and one `coerce`
  at every check-mode mismatch. Forget by `Proj1` at elimination
  positions. No coercion under a type former or binder.
- **Kernel hunk K-a**, authorized: the three Char sites move to
  `Σ Int isScalar` with a closed witness. Each literal-view element's
  typing is established by `check` at checked-String-literal admission.
- Retire `walk_nested_introduction`, the same-root reuse exemption,
  RefinementFacts as a guard input, and the root abstraction in
  `simplify_branch_goal_keeping_refinements`.

## Acceptance

- **AC-0 (D0, measure only).**
  - (a) Introductions by closure class.
  - (b) Every elaborator site that whnf's an inferred type to inspect its
    head.
  - (c) Corpus sites relying on forget-under-former, swept by mechanism
    across catalog/, crates/*/tests, examples/ and conformance/.
  - (d) Every kernel or prim reduction that produces or consumes a value
    at a refinement type, beyond the three Char sites.
  - The Architect rules recursive-call hypotheses from (a). If (c) is
    large, the Steward splits the catalog migration.
- **AC-1 (forgery rows, kernel-refused or one obligation each).**
  - `List Five` admitting `Cons Int six`, and `List Char` admitting
    55296.
  - The let-alias, let-value and alias-branch forgeries, and the ArgParse
    shape. These are from WIP `b76c4f278` and its successor.
  - `const c : Fn5 = \y. 6` gives 1 obligation, with goal
    `Equal Int ((λ. 6) 0) 5`.
  - `apply take5` is refused, and `take5 six` keeps 1 obligation at the
    caller.
- **AC-2.** Population delta 0 (76/32 roots, 55/20 packages), apart from
  explicit-conversion migrations AC-0(c) lists. Runtime parity is
  unchanged. Derived and ArgParse timing stay within 10%.
- **AC-3.** Kernel QA reviews the K-a lines.

## Stop conditions

- Any kernel line beyond K-a, any conversion change, or any
  `trusted_base()` change.
