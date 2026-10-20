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
    `evt_5zp1649t8qn9n`; amendment 3, `evt_6g7tvrp8r02s4`).** The
    ruling's text:
    1. An introduction with a selected certificate demands exactly the
       enclosing splits whose condition binders are free in the peeled
       certificate evidence (`lower_binders` of the actual term). A
       closed certificate demands none and is emitted directly as the
       subset-Σ pair.
    2. An introduction with no certificate is an open hole. It demands
       every admissible condition it is closed over. Convoyable splits
       (`let`, `if`, literal, non-recursive non-indexed constructor
       branch) are discharged as the frame's evidence rules state.
    3. An open hole whose demand reaches an indexed split without
       heterogeneous equations is a typed refusal, never `Internal`. A
       recursive unindexed constructor split has no equation evidence
       until `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW`. In this candidate it
       contributes no condition and is never demanded, and the hole
       closes over the evidenced Γ only.
  - **Route (C) record (Steward `evt_489y4gdxhx0sc`,
    `evt_5zg993ve8hrjd`).** Spec 22 §3 and 34 §3.3 stay normative and
    require a checked equation in every unindexed arm, recursive ones
    included. The Architect retracted the basis of amendment 3's
    recursive clause (`evt_6jd4e9y5cvrca`). This WP lands as a disclosed
    partial: the gap is recorded in SPEC-PROGRESS and pinned by the
    recursive-split test, and `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW`
    closes it and flips that pin.

## Deliverable

- Designs 1-4: representation, introduction, evidence, and one `coerce`
  at every check-mode mismatch. Forget by `Proj1` at elimination
  positions. No coercion under a type former or binder.
- **Kernel hunk K-a**, authorized: the three Char sites move to
  `Σ Int isScalar` with a closed witness. Each literal-view element's
  typing is established by `check` at checked-String-literal admission.
- **Interpreter hunk, folded in** (Steward `evt_502fjf2x12crm` on
  Architect `evt_1kwpqy4wgbenh`, after CI red on `ee5dfde13`). Add a
  sufficient-only `SigmaTy` arm to `ken-interp` `eq_type_eq`, and have
  `rt_c5_producer_type_slots.rs` evaluate its indices through `Const`
  references, not raw `transparent_body`. Runtime Char values stay the
  bare `Int` carrier (spec 42 §3.2, §3.5), and no Char consumer changes.
  Runtime QA reviews this hunk. Its controls: A+B passes; reverting
  either alone gives `Unknown`; the AC-2a census is byte-identical; and
  a sweep of raw `transparent_body` evaluation of numeric `Char` sites.
  The `PiTy` gap is a carry.
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
  - **AC-0b-2 (amendment 3).** One open hole under an indexed split
    without heterogeneous equations gives the typed refusal, not
    `Internal`. Paired failing configuration: removing the gate admits it
    or panics. Under a recursive unindexed split (`sort`'s Nil leaf), the
    hole's `goal_closed` holds no `Eq` on `List` and the `xs` match keeps
    the plain Σ motive; closing over the equation, or demanding the
    split, each refuses. This row is the route (C) known-gap pin, not
    conformance. The consumer sweep over every source root
    reports each such hole's outcome against main, and any change from
    proved to not proved is a stop.
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
- **AC-2a (ambient census exception "SUBSET-SIGMA K-a Char projection",
  Steward `evt_61fmagt1tw8g9` on Architect `evt_ra1z83nr0nm`).** The
  strict-resolution census may change only by roots that gain
  `charToInt`, and each such root must have `Core.Classes.LawfulClasses`
  in its loader closure and no prior `charToInt`. Every changed row
  carries the exception's name. Measured on `137d23a54`: 47 rows gain it,
  none lose it. The 78-row attribution TSV is evidence, not a gate; the
  census vector is the enforced check. `register_checked_char_carrier`
  refuses a polymorphic or non-`Type` `Char` before unfolding it, with a
  regression whose guard deletion reddens.
- **AC-E (evidence).** Every landed package's owned declarations emit
  raw kernel core identical to base modulo global-id renaming, except the
  introduction, refinement-binder and K-a Char classes, each difference
  attributed (Architect `evt_cq3bk2mfr7gn`, which gives the procedure: the
  introduction class totals the census's 16 sites in 6 closures, and any
  unattributed difference is a stop). The Minimisation row below asserts
  its timing. An introduction under a `let`, an `if`, a
  literal and a non-recursive constructor branch each checks; one under an
  indexed split is a typed refusal. A Minimisation-shaped `let` whose RHS
  is a fixed point checks when an introduction consumes it; emit
  `Refl(rhs)` unascribed whenever `rhs` infers. Its timing criterion
  (Architect `evt_7cc79te990727`, replacing the 1.10× bound of
  `evt_6yhfdb723hhy0`, which no lawful configuration meets). Three kernel
  queries are mandatory, each once: `admit_bodies` on the obligation, the
  site pair `kernel_check_current`, and the final declaration admission.
  Reusing a kernel verdict to skip one is out of scope. The rows use
  `blow n = match n { Zero ↦ Zero; Suc m ↦ plus (blow m) (blow m) }` with
  `pad` unused leading binders `(u0 : Nat) …`: LET is `fn carried (u0 ..)
  : { x : Nat | Equal Nat x (blow K) } = let value = blow K in value`, and
  OPEN is the same signature with body `u0`. Debug build, median of 3,
  base `7ac7a5cd1` measured in the same run. The repair is in
  `emit_subset_evidence`:
  - **S1.** The certificate chain keeps its order and first-found
    selection, because the certificate feeds
    `demanded_refinement_splits`.
  - **S2.** Each candidate is decided in order before `kernel_check_raw`.
    Accept a `Var(i)` without probing when `recorded_eq_endpoints` of its
    type equals that of the goal after zonk. Reject without probing only
    by one exhaustive match, with no `_` arm, over the (candidate whnf
    head, goal whnf head) pair, grounded in conv.rs `convert_type`, with
    the goal's whnf computed lazily at most once per call. Never reject on
    an unreduced Eq head. Otherwise probe.
  - **S3.** Delete the explicit pre-admission `kernel_check_raw` on the
    closed body; `admit_bodies` checks the same body at the same type.
  - **G1 (S3, paired).** Restoring the pre-check makes LET K=12 at least
    0.5 s slower.
  - **G2 (S2).** For LET and OPEN at K=12, pad8 takes at most 1.10× pad0.
    Pristine `9e8959da7` measures 1.35× and 4.98×; base is flat.
  - **G3.** LET K=12 pad0 takes at most 2.25× its own base. Pristine is
    2.75×, and each single repair is 2.34× or 2.35×.
  - **G4 (S1).** `fn f (h : Equal Nat Zero Zero) : { x : Nat | Equal Nat
    Zero Zero } = Zero` emits `tt` as the pair's proof. Paired: an
    identity-first mutant selects `h` and turns the row red. The 78-root
    raw parity shows no new difference against `9e8959da7`.
  - AC-S 5/5, nested_domain 9/9, up_to_whnf 3/3, the landed suite and
    the earlier Eq-head ACs stay green on the final bytes.
- **AC-H (Architect `evt_14f63z3dtk1s3`).** The terminal arm of
  `coerce_refinement` asks the kernel through `kernel_check_current` in
  place of its bare `TypeMismatch`; the Let and direct-Eq arms stay above
  it. No landed test is weakened: `cat5_parsing_package.rs` is byte
  identical to main, and any other landed-test edit that tolerates an
  elaborator `TypeMismatch` is reverted or justified in the handoff.
  - **H1.** The landed hull and cat5 tests pass byte-identical to main.
  - **H2 (paired).** Restoring the bare-reason arm turns H1 red.
  - **H3.** `fn f (n : Nat) : { x : Bool | Equal Bool x True } = n`
    refuses with `KernelRejected(TypeMismatch)` with Nat and Bool heads.
    Paired: the bare arm gives `ElabError::TypeMismatch`.
- **AC-3.** Kernel QA reviews the K-a lines.
- **AC-S (Architect `evt_2399h0mk7asd9`, replacing `evt_1pm2qvb169bfm`).**
  `unify_sorts_walk` gains a `Sigma/Sigma` arm with the Pi arm's domain
  push and pop, and a Σ-carrier arm `(Term::Sigma(carrier, _), _)` that
  walks the carrier against the actual. Both sit inside the walk; the
  ruling's text is the code. Rows, with `const Big : Type 2 = Type 1` and
  `F : ({x : Big | Top}) -> Type`:
  - b1 `(a : Type)` and b2 `(a : {x : Type | Top})` are admitted.
  - b3 `(a : Type 1)` is admitted and b5 `(a : Type 0)` refused
    (controls).
  - Paired: deleting the Σ-carrier arm refuses b1, and dropping the
    `Sigma/Sigma` pattern refuses b2.
  - The `Small` twins pass by the level's Zero default and are not
    evidence.

## Stop conditions

- Any kernel line beyond K-a, any conversion change, or any
  `trusted_base()` change.
- Meeting G1 to G4 needs a kernel change, such as a kernel-side memo of
  checked judgements or a conversion change: stop with the measurements.
  It is a trust-root change for the operator.

## Hard-stop inventory (§1b)

§1a: ctor-field staging 1 (Architect `evt_4b6ktd7c5xqxx`, from
`evt_290erhrm8bnaf`); AC-S sort link 1 (`evt_2399h0mk7asd9`);
ordinary-mismatch deferral boundary 1 (`evt_14f63z3dtk1s3`).

1. The two-pass provisional family was taken from the raw carrier
   lowering (`data::elab_*_data_decl`). It was keyed on that lowering
   being well-typed, and W5 makes it ill-typed for any field or index
   that applies a refined-parameter former (`Box five`: expected
   `Σ Int Eq5`, found `Int`). Corrected to a family head with zero
   constructors (the `family_probe` idiom, but in the same env), with
   every field, parameter, index and result going through the shared
   elaborator.
2. The type-position sort link was keyed on the domain being the carrier
   type. W5 makes a refined domain Σ, which neither the carrier compare
   nor the walk matched. Fix: a Σ/Σ arm plus a Σ-carrier arm in
   `unify_sorts_walk`. Keyed on the refinement being represented by its
   carrier (Architect `evt_25zg89vfk8ksw`).
3. `coerce_refinement` refused an ordinary (non-refinement) mismatch on
   its own judgment, where main defers to the kernel. The refusal's
   carve-outs were keyed on the proposition's spelling (direct Eq head,
   Architect `evt_7rxrymmzb344v`); `IsTrue`, a transparent alias of Eq,
   fell outside (landed `cat_numeric_interval_hull`). Fix: the failure
   path asks the kernel in the current context; no elaborator verdict
   (Architect `evt_14f63z3dtk1s3`). It shares no predicate with entries
   1 and 2.
