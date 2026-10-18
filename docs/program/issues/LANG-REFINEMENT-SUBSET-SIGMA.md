---
id: LANG-REFINEMENT-SUBSET-SIGMA
title: "Refinements are carrier types, so List Int is accepted as List Five, a lambda checked at a named refinement over a Pi carrier emits no obligation, and a refined binder converts with its carrier. Elaborate refinements to the core subset Σ, introduce by pair plus obligation, forget by Proj1, and retire the carrier guard machinery"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [SPEC-REFINEMENT-SUBSET-SIGMA, LANG-REFINEMENT-PROOF-ERASURE, VERIFY-TRANSITIVE-HONESTY]
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

All three dependencies have landed. AC-0 re-verifies every anchor above
on the current base before measuring, and reports any that moved.

- **Evidence at the consumer (Architect `evt_41fxwysr833vv`).** No term
  applies a refinement hole on base (elab.rs ~:11666, ~:11779), so every
  program without a subset-Σ introduction elaborates byte-identically.
  The introduction requests evidence only for the conditions whose
  binders are free in its **evidence term** (amendment 1 below), from
  each condition's frame record:
  - `let`: `refl rhs` at the introduction's argument position; its type
    checks there after kernel `Let` substitution (check.rs ~:398-401). No
    inner Ω `Let`.
  - `if`, literal, or non-recursive non-indexed constructor branch: a
    convoy on the enclosing split, only when a requesting introduction is
    beneath it, gated by `row_leaves_fields_unsplit` /
    `split_convoy_allowed` and `no_pending_path_condition` (proved in WIP
    `59eaccce2`, which never lands; reuse its components).
  - Indexed family: a typed refusal.
  - Recursive family: `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW`.
  - **Demand mechanism (amendment 1, Architect `evt_6tq4e1qs4zx5w` on
    D0 `evt_7y18marcqgv6d`): (A), deferred re-elaboration.** A split
    keyed by an identity minted at split entry catches a typed demand,
    rolls back its branches and re-elaborates them with the convoy on, at
    most once per split. (B), always-bind, is out: M4 = 1 and 33 kernel
    rejections. (A) passed M1, M2 and the retry bound.
  - **Demand criterion (amendment 1).** A condition is demanded only if
    its binder is free in the introduction's evidence term. A
    certificate found at elaboration demands only the conditions it
    mentions, so `isScalar 32` demands nothing. An open hole is applied
    to its conditions and demands them all. Closing the hole over a
    condition is not a demand: base closes every hole over every
    admissible condition, which made ArgParse demand the recursive `Doc`
    split and admit 0/7.

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
- **Conformance flip** (Architect `evt_3t2jfn2bt8n3e`). In the same
  candidate, flip each test that claims a current-behaviour row W1 kept,
  and retire that row: `v1_acceptance.rs:97`
  (`ensures-emits-obligation-not-sigma`), `:134`
  (`refinement-lowers-to-carrier`), and `v2_acceptance.rs:262`
  (`refined-param-desugars-to-requires`), `:672`
  (`decoupled-from-sigma-sort`). Each flipped test claims its successor
  row. Update the `AC-transport` comment at
  `es4_classes_acceptance.rs:683` to `AC-char-ord-law-lifting`.

## Acceptance

- **AC-0b (census, scratch, first; amendment 1).** On prototype (A)
  with the corrected criterion, for each of the 6 catalog closures with
  introductions: count the introductions under a split, and classify
  each as certificate with no condition used, certificate using a
  condition (name the split family), or open hole (name the enclosing
  split families). Report admit or refuse per closure, with ArgParse's 7
  runs timed against AC-2's 10% bound if it admits.
  - **Zero open holes under a non-convoyable split** (recursive, or
    indexed without heterogeneous equations): the criterion change is in
    this WP and the build proceeds on (A).
  - **Any such hole:** stop to the Steward with the count. The scope
    call is between dropping those conditions from the hole's closed type
    (strengthening, with a typed refusal if then unprovable) and ordering
    `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW` first.
- **AC-0 (D0, measure only; closed `evt_1j5wf5kfe127q`).**
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
- **AC-E (evidence).** Every landed package's emitted core is
  byte-identical to base. An introduction under a `let`, an `if`, a
  literal and a non-recursive constructor branch each checks; one under an
  indexed split is a typed refusal. A Minimisation-shaped `let` whose RHS
  is a fixed point checks within base time when an introduction consumes
  it; emit `Refl(rhs)` unascribed whenever `rhs` infers.
- **AC-3.** Kernel QA reviews the K-a lines.

## Stop conditions

- Any kernel line beyond K-a, any conversion change, or any
  `trusted_base()` change.
- The AC-E fixed-point row needs a kernel conversion-order change
  (Refl's ascription defeating the `a == b` fast path, conv.rs ~:1129):
  stop with the measurement. It is a trust-root change for the operator.
