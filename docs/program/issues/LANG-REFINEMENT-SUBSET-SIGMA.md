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
  - **Demand criterion (amendments 1 and 2, Architect
    `evt_5zp1649t8qn9n`).** The ruling's text:
    1. An introduction with a selected certificate demands exactly the
       enclosing splits whose condition binders are free in the peeled
       certificate evidence (`lower_binders` of the actual term). A
       closed certificate demands none and is emitted directly as the
       subset-Σ pair.
    2. An introduction with no certificate is an open hole. It demands
       every admissible condition it is closed over. Convoyable splits
       (`let`, `if`, literal, non-recursive non-indexed constructor
       branch) are discharged as the frame's evidence rules state.
    3. An open hole whose demand reaches a non-convoyable split
       (recursive, or indexed without heterogeneous equations) is a
       typed refusal, never `Internal`. Recursive families stay with
       `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW`.

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

- **AC-0b (census; closed, build-go, Architect `evt_5zp1649t8qn9n` on
  `evt_19xr1vssp79k7`).** 78 scanned, 6 closures with introductions, 16
  introductions, all closed certificates, 0 open holes under a
  non-convoyable split. The build resumes from checkpoint `569f52f19`.
  The certificate-with-condition and open-hole classes have no catalog
  population, so amendment 2 pins them with synthetic rows:
  - **AC-0b-1.** A durable classifier row with three nested-`Bool` cases:
    an outer-used certificate demands only its split, a closed
    certificate demands neither, and an open hole demands both. Paired
    failing configuration: demand read off the hole telescope makes the
    closed-certificate case demand both.
  - **AC-0b-2.** One open hole under a recursive split gives the typed
    refusal, not `Internal`. Paired failing configuration: removing the
    gate admits it or panics.
  - **AC-0b-3.** At the candidate, the six closures stay admitted with 16
    pairs kernel-checked, and the certificate-discard mutation refuses
    the five `Doc` closures while Console stays admitted.
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
  unchanged. Derived and ArgParse timing stay within 10%, re-measured
  against the candidate's own base (the census's +0.05% is scratch
  evidence only).
- **AC-E (evidence).** Every landed package's emitted core is
  byte-identical to base. An introduction under a `let`, an `if`, a
  literal and a non-recursive constructor branch each checks; one under an
  indexed split is a typed refusal. A Minimisation-shaped `let` whose RHS
  is a fixed point checks within base time when an introduction consumes
  it; emit `Refl(rhs)` unascribed whenever `rhs` infers.
- **AC-3.** Kernel QA reviews the K-a lines.
- **AC-S (carried, Architect `evt_1pm2qvb169bfm`).** The type-position
  sort walk `unify_sorts_walk` has no `Sigma` arm, so under W5 an alias
  nested in a refined domain's carrier links no level. If this build
  reaches it, add a `Sigma` arm with the Pi arm's domain push and pop,
  with the row `(F : {x : Small | φ} -> Type) (a : Type) (o : F a)`.
  Otherwise report it for a separate node.

## Stop conditions

- Any kernel line beyond K-a, any conversion change, or any
  `trusted_base()` change.
- The AC-E fixed-point row needs a kernel conversion-order change
  (Refl's ascription defeating the `a == b` fast path, conv.rs ~:1129):
  stop with the measurement. It is a trust-root change for the operator.

## Hard-stop inventory (§1b)

§1a: 1 (Architect `evt_4b6ktd7c5xqxx`, from `evt_290erhrm8bnaf`).

1. The two-pass provisional family was taken from the raw carrier
   lowering (`data::elab_*_data_decl`). It was keyed on that lowering
   being well-typed, and W5 makes it ill-typed for any field or index
   that applies a refined-parameter former (`Box five`: expected
   `Σ Int Eq5`, found `Int`). Corrected to a family head with zero
   constructors (the `family_probe` idiom, but in the same env), with
   every field, parameter, index and result going through the shared
   elaborator.
