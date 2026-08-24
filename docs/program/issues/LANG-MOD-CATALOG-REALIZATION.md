---
id: LANG-MOD-CATALOG-REALIZATION
title: "WP-4 (Component A) — module-graph/roots loader realization + provider public surface: the self-contained green set checks standalone-strict through the real loader, and Arithmetic add/mul + Order leq_nat/sub are made public/import-eligible. Whole-catalog strict-green + consumer migration are re-homed to Component B."
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-MOD-LOADER-ENTRY, LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-OR-CANONICAL-HOME]
blocks: [LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-4), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. RE-FRAMED 2026-08-24 to Component A after a structural hard stop (below); the strict whole-catalog co-gate + consumer migration are re-homed to [[LANG-MOD-CATALOG-COMPLETENESS]] (Component B)."
---

> # RE-FRAMED 2026-08-24 — Component A; migration + strict co-gate re-homed to Component B.
>
> WP-4 as originally released hit a GENUINE STRUCTURAL HARD STOP at base
> `f0e0b92fa` (language-implementer evt_3j60e77n0ahsy, confirmed by
> language-leader evt_7r5cx3fkvfwxn; branch held byte-clean, no code change
> attempted). Arithmetic, Order, and Gcd all reference the TYPE `Nat`, which no
> catalog module declares or exports and which is NOT on the fixed surface floor
> `{Bool, Char, List}` (spec/30-surface/30-taxonomy.md §4; 33-declarations.md
> §3.3). Under the reviewed strict-resolution contract there is no ambient
> fallback, so strict-roots correctly rejects `UnboundName{Nat}` for those three
> and for the 42 baseline-red leaves. Adding only the authorized provider imports
> (Transport, LawfulClasses, add/mul, leq_nat/sub) cannot make them load — the
> whole-catalog strict co-gate requires a deliverable OUTSIDE WP-4's authorized
> surface (a canonical home for `Nat`). The census also drifted: current behavior
> census is 34 baseline-red residuals, not the frame's original 32.
>
> ARCHITECT RULING (evt_214z6r6qnwme0) — the lawful component boundary:
> - OPTION 3 (widen the strict floor/vocabulary to admit `Nat` as ambient) is
>   RULED OUT on design grounds: it reintroduces the ambient fallback the strict
>   contract removed — a contract/soundness regression, not a legal future. No
>   operator floor-change escalation is owed.
> - The strict whole-catalog co-gate is MIS-LOCATED on WP-4: it gates a
>   deliverable WP-4 is not authorized to produce (a `Nat` home). Unbundle into
>   two components.
> - COMPONENT A (this node): the module-graph/roots loader realization delivered
>   against the AUTHORIZED provider surface. Acceptance is loader behavior +
>   strict-satisfiability for the units whose providers already exist (the
>   self-contained green set) — NOT whole-catalog strict-green.
> - COMPONENT B ([[LANG-MOD-CATALOG-COMPLETENESS]]): release ONE canonical `Nat`
>   (and the other required convenience homes) via defining public interfaces,
>   migrate the consuming units to import them, and satisfy the re-homed
>   whole-catalog strict-green co-gate. The gate is not abandoned — it is
>   re-homed onto B, where the deliverable it gates actually lives.
> - END-STATE INVARIANT: the strict-resolution contract is CORRECT and stands.
>   Deferring the co-gate off A is sequencing, not weakening. Component A must NOT
>   reintroduce ambient, invent identities, or restore fallback to appear green —
>   that is option 3 in disguise. The implementer's three refusals were correct.
>
> Predecessors landed: [[LANG-MOD-LOADER-ENTRY]] merged,
> [[LANG-MOD-PUB-ELIGIBILITY]] merged, [[LANG-MOD-OR-CANONICAL-HOME]] merged
> (NODE B respin e1509b88d, closed cf8dc2724). Component A is buildable now on
> base `f0e0b92fa` and is RE-RELEASED to the language ring. Closing Component A
> unblocks Component B; the campaign's catalog-reuse success criterion (the Gcd
> reuse + whole-catalog strict-green) now lands in Component B, which unblocks
> foundation [[CAT-GCD-REFACTOR]] when the campaign root closes.
>
> STRICT-RESOLUTION CO-GATE MOVES TO B. [[LANG-MOD-STRICT-RESOLUTION]]'s remaining
> whole-catalog strict enforcement / CI closure co-closes with Component B's
> migration (not this node) — B CO-DELIVERS that closure. Its consumed D1 strict
> machinery is landed (5a74301f4, ancestor of main); STRICT-RESOLUTION stays
> `ready`/open until B's co-gated strict-green lands.

# Objective

Realize the module-graph/roots loader against the authorized provider surface:
the self-contained green set checks standalone through the real loader under
strict, and the authorized providers (Arithmetic `add`/`mul`, Order
`leq_nat`/`sub`) are made public and import-eligible for Component B to consume.

# Deliverable (Component A)

- Mark `pub` on Arithmetic (`add`, `mul`) + Order (`leq_nat`, `sub`) and their
  required provider names public / import-eligible — the provider SURFACE that
  Component B's migration will consume. (Marking `pub` does not require `Nat` to
  resolve; validating Arithmetic/Order standalone-strict does, and that is B.)
- Deliver the module-graph/roots loader realization so the self-contained green
  set checks standalone through the REAL loader under strict.

# The self-contained green set (measured at base f0e0b92fa)

The behavior-side strict-roots probe over the closed 45-leaf catalog population
yields 3 strict-green: `Core.Logic.Or`, `Core.Logic.Transport`, and
`Tooling.Verification.ProofErasureBoundaryChecker` (language-implementer
evt_3j60e77n0ahsy). These are the units whose providers exist on the authorized
surface. The remaining 42 red leaves need `Nat` / other convenience homes and
are Component B's population (the 34-baseline-red residual triage moves to B).

# Acceptance criteria

- AC-A1. Each unit in the self-contained green set (`Core.Logic.Or`,
  `Core.Logic.Transport`, `Tooling.Verification.ProofErasureBoundaryChecker`)
  checks STANDALONE through the real loader under strict.
- AC-A2. Arithmetic (`add`, `mul`) and Order (`leq_nat`, `sub`) plus their
  required provider names are `pub` / import-eligible through the real loader's
  resolution — established by IDENTITY (the exact provider IDs are reachable),
  NOT by repo text and NOT by Arithmetic/Order standalone-strict-green (which is
  Component B).
- AC-A3 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-A-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.
- NON-GOAL (explicit, re-homed to Component B): Arithmetic/Order/Gcd standalone
  strict-green, whole-catalog strict-green, the Gcd import-reuse, and the
  34-residual triage are NOT in Component A. Component A MUST NOT reintroduce
  ambient resolution, invent competing identities, or restore prelude fallback to
  make any red unit appear green — that is the ruled-out option 3 in disguise.

# Reviewers

Architect (component fit; loader realization against the authorized surface only;
no invented identity) + conformance-validator (identity-preserving pub/import
resolution).

# Capability tier

T2 — loader realization against a fixed authorized surface + mechanical pub
markings; the soundness is carried by WP-2/WP-3 and the strict contract, whose
whole-catalog enforcement is Component B. Size M.
