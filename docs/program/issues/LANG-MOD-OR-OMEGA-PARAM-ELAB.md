---
id: LANG-MOD-OR-OMEGA-PARAM-ELAB
title: "Or arm (b), NODE A (prerequisite) — teach explicit-data parameter/index elaboration to honor an Omega-sorted binder, so a catalog package can declare Or : Omega -> Omega -> Type; the result-sort admission is unchanged, preserving the ban on proof-relevant multi-constructor Omega-valued families"
status: ready
owner: language
size: S
gate: none
depends_on: []
blocks: [LANG-MOD-OR-CANONICAL-HOME]
github: null
origin: "Architect arm-(b) prerequisite finding evt_21gve67p385jh (Or as written is UNSPELLABLE in surface data syntax today); enclave sort-discipline GO spec-leader evt_3j02n0pkgze3a. Operator ruled the Or/Inl/Inr fork arm (b) (canonical package home, evt_6b9wrt1kwswcp). Steward-filed per COORDINATION section 2, 2026-08-23, under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # RELEASED 2026-08-23 — WP-2 D1 landed (5a74301f4); the release-order gate is
> # satisfied. Buildable now (enclave GO'd). This is the language ring's next
> # module/import unit.
>
> Architect sequence: NODE A -> [[LANG-MOD-STRICT-RESOLUTION]] D1 (landed) -> NODE
> B ([[LANG-MOD-OR-CANONICAL-HOME]]). NODE A and WP-2 D1 were mutually independent
> (this touches the data elaborator; D1 touched resolution); both precede NODE B,
> and D1 is done, so NODE A is next. Not on any other lane's critical path.
>
> Operator directive 2026-08-23: with RT-CHECKED tier-1 confirmed RUNTIME work
> (Architect evt_6h5ndf9hxf22f — NOT a language spillover), the language ring
> resumes the module/import campaign HERE, unblocking the foundation catalog
> trial ([[CAT-GCD-REFACTOR]]) in parallel with the runtime lane.

# Objective

`Or`/`Inl`/`Inr` are proof-relevant (`Or : Omega -> Omega -> Type`; the params are
Omega-sorted propositions, the result is `Type` so an `Inl`/`Inr` case-split is
informative and cannot live at Omega). Today they exist ONLY as a Rust prelude
registration (`crates/ken-elaborator/src/prelude.rs:1253/1262/1264`); no catalog
package can declare them, because an Omega-sorted parameter is unspellable in the
explicit-data surface path: `data.rs::rtype_to_kernel_checked` has no Omega arm
and maps `Omega` to `UnresolvedCon`. The type path already special-cases Omega
(`elab.rs:621`), but the data-declaration parameter/index telescope does not route
through it. This node closes exactly that gap.

# Deliverable

Add the Omega case to explicit-data parameter/index elaboration
(`data.rs::rtype_to_kernel_checked`), translating an Omega-sorted binder to
`Term::Omega(Level::Zero)`, mirroring the existing `elab.rs:621` type-path
handling. The result-sort admission is UNCHANGED — this teaches the elaborator to
accept an Omega-sorted PARAMETER, not to relax what result sorts a data family may
have.

# Enclave ruling (spec-leader evt_3j02n0pkgze3a — read for exact wording at review)

The Omega parameter/index translation, WITH the result-sort admission left
unchanged, accepts `Or` AND preserves the ban on proof-relevant / multi-
constructor Omega-VALUED families. No extra index- or constructor-argument guard
is needed: the existing telescope validation, the `Type`-family codomain check,
the constructor-target/positivity check, and the classify + universe-bound check
are the guards. The node's discrimination AC pins that the ban still fires.

# Acceptance criteria

- AC-A1 (accept). A data declaration with Omega-sorted parameters and a `Type`
  result — `data Or (a:Omega)(b:Omega):Type where {Inl : a -> Or a b; Inr : b ->
  Or a b}` — elaborates with no `UnresolvedCon` on the Omega parameters.
- AC-A2 (discrimination — the ban preserved; the enclave's pin). A data
  declaration whose RESULT SORT is Omega with two or more constructors is STILL
  rejected — the classify/universe-bound guard fires exactly as before. CONTROL:
  a mutation that disabled the guard would let this through; the new Omega
  parameter arm must not open that door. This is the axis the enclave GO turns on
  — a proof-relevant Omega-valued family stays banned.
- AC-A3 (zero trust — cross-cutting invariant). Zero `trusted_base()` delta; the
  elaborator change allocates no kernel declaration; the flat-Sigma pin
  (`module_elaborates_to_identical_flat_sigma`) stays green.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-elaborator`
  only.

# Reviewers

Architect (soundness of the sort admission) + conformance-validator
(accept/discrimination discriminator pair) + enclave sort-discipline confirm-as-
built (spec-leader/spec-author own the Omega proof-irrelevance boundary).

# Capability tier

T1 (soundness-adjacent sort-discipline change to data elaboration; small in lines,
reasoning-dense in review — the discrimination AC is the point). Size S.
