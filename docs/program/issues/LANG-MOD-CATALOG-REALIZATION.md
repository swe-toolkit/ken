---
id: LANG-MOD-CATALOG-REALIZATION
title: "WP-4 (Component A) — module-graph/roots loader realization: the uniform local-scope prebind fix (decouple local-declaration binding from ResolutionMode) so canonical Or binds under legacy, the self-contained green set checks standalone-strict, and Arithmetic add/mul publish genuine legacy loader-resolved identity. Order + OrdResult + consumer migration + whole-catalog strict-green are Component B."
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

Realize the module-graph/roots loader so it conforms to spec §3.3 — a unit's own
local declarations bind independent of `ResolutionMode` (the uniform local-scope
prebind fix), so canonical `Core.Logic.Or` binds its `Inl`/`Inr` under legacy;
the self-contained green set checks standalone-strict; and the Arithmetic
providers (`add`/`mul`) are made public and publish genuine legacy loader-resolved
identity. Order + `OrdResult` + all consumer migration are Component B.

# Deliverable (Component A)

- THE LOADER FIX (primary — Architect mechanism ruling evt_41tpyzwkc14ez). The
  uniform local-scope rule: every declaration a unit itself makes enters that
  unit's local module scope INDEPENDENT of `ResolutionMode`. This conforms the
  loader to spec 33-declarations.md §3.3 ("Per-unit dependency closure": a unit
  resolves its body in its own module scope = its LOCAL declarations, its
  explicit imports, kernel/built-in vocabulary, and the closed prelude floor —
  mode is nowhere in that clause). Concretely, remove BOTH mode-gates in
  `prebind_scope_declarations`: the `mode == Strict` conjunct on
  `strict_unqualified_local` (modules.rs:1653, class locals) and the
  `if scope.mode != Strict { continue }` guard before the ctor match
  (modules.rs:1674, DataDecl + ExplicitDataDecl constructors). Mode continues to
  govern ONLY `resolve_ref`'s EXTERNAL-fallback (legacy leaves an unresolved
  external name bare; strict fails closed) — that path is byte-UNTOUCHED. This is
  a loader capability defect, NOT a spec change, NOT option 3 (external ambient
  in resolve_ref, untouched), NOT ambient restoration (a module's OWN ctors are
  not ambient authority), zero `trusted_base()` delta (surface diagnostics only;
  the flattened Σ the kernel receives is unchanged). Structural over ALL local
  declaration kinds — ordinary-data ctors (DataDecl), explicit-data ctors
  (ExplicitDataDecl), and class locals (ClassDecl) — NOT an `Inl`/`Inr`
  point-fix (a ctor-only repair just surfaces the next declaration-kind refusal).
- ADD Arithmetic's provider-internal import (its ONLY closure dependency —
  Architect HS#5 ruling evt_613d9fm7j45qj scoped A to Arithmetic): Arithmetic (no
  import line at base) -> `import Core.Logic.Transport (cong, sym, trans)`, and
  make `add`/`mul` + Transport's `cong`/`sym`/`trans` `pub`. Arithmetic has ZERO
  OrdResult/LawfulClasses references; its provider-internal closure is
  Transport-only, which elaborates FULLY under legacy. Identity-preserving, zero
  `trusted_base()` delta.
- ORDER IS NOT TOUCHED IN A. Order's provider surface (pub `leq_nat`/`sub` +
  Transport/LawfulClasses imports) and Order's `leq_nat`/`sub` identity move to
  Component B, because Order's closure transitively needs
  `Core.Classes.LawfulClasses` -> the HOMELESS `OrdResult` (HS#5), so it is NOT
  self-measurable in A.
  A performs no Order/LawfulClasses source edits — no unverifiable-in-A changes.
- With the loader fix + Arithmetic's Transport import applied, the
  module-graph/roots loader fully elaborates canonical `Core.Logic.Or` (its own
  `Inl`/`Inr` now bind locally under legacy) and Arithmetic (`Nat` resolves via
  legacy external fallback to the native prelude inductive `prelude.rs:440 data
  Nat = Zero | Suc Nat`; `cong`/`sym`/`trans` via the added Transport import), and
  Arithmetic's genuine `add`/`mul` `GlobalId`s publish — measurable under the
  LEGACY real-caller resolution the loader ships (`elaborate_module_from_roots =>
  ResolutionMode::Legacy`). A's entire closure (loader fix + Or + Transport +
  ProofErasureBoundaryChecker + Arithmetic) is HOMELESS-FREE — no further hard
  stop in A. Moving the real caller to STRICT is Component B.

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
  checks STANDALONE through the real loader via the strict-roots check that is
  runnable at base (these three pass at f0e0b92fa) — a regression guard that A's
  provider-import and pub edits do not break the green set. This is the
  strict-roots check on the green set specifically, NOT a move of the real caller
  to strict over the whole catalog (that is Component B).
- AC-A2 (Arithmetic only — Order's identity is Component B, HS#5). With the
  loader fix applied, Arithmetic (`add`, `mul`) FULLY elaborates through the real
  loader under the LEGACY real-caller mode A ships, and its genuine `GlobalId`s
  are observed by IDENTITY (Architect ruling evt_41tpyzwkc14ez /
  evt_613d9fm7j45qj) — reachability to the GENUINE provider (not repo text, not a
  frozen numeric id, no invented/competing identity), NOT Arithmetic
  standalone-STRICT-green (which is Component B). Component B must preserve
  Arithmetic's provider identities when it re-homes `Nat` and moves the caller to
  strict (B's forward-compat constraint).
- AC-A4 (identity envelope). Idempotent same-identity, fail-closed
  distinct-identity: the existing prelude-collision `AmbiguousReference` check
  (modules.rs:1667-1673) must still bite — two distinct declarations of one
  spelling still collide; a same-`GlobalId` reachable twice must NOT error. Pin
  both directions.
- AC-A5 (no restored ambient authority). The fix touches `prebind` ONLY;
  `resolve_ref`'s strict fail-closed on EXTERNAL names is byte-unchanged — verify
  strict still rejects external `Nat`/conveniences exactly as before, so AC-A1's
  strict green set stays 3-green and does not silently widen.
- AC-A6 (legacy stack budget — the real subtlety). The strict-only gating was a
  DELIBERATE choice (modules.rs:1646-1652) to keep constructor/class temporaries
  out of `expand_scope`'s long-lived frame and retain the legacy stack budget for
  recursive application-spine elaboration. Binding local decls under legacy must
  go into the PERSISTENT scope (`scope.bind_local`), NOT grow the recursive-spine
  frame. PIN a legacy recursive-spine stack-budget property AT THE SITE with an
  explicit `RUST_MIN_STACK` — a pin that rides the ambient stack is vacuous.
  Measure it, do not assert it.
- AC-A7 (declaration-kind coverage). A `DataDecl` ctor and a `ClassDecl` local
  under legacy must ALSO bind after the fix, not only `ExplicitDataDecl` — pin
  one of each, or the closure is not closed.
- AC-A3 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-A-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.
- MUTATION THAT BITES (natural, compile-preserving). A legacy roots-load of
  canonical `Or` must now bind `Inl`/`Inr` and publish Or's interface (was
  `UnboundName{Inl, span 493-512}`); reverting the fix to strict-gated must red
  it. Restore byte-identically.
- NON-GOAL (explicit, re-homed to Component B): Arithmetic/Order/Gcd standalone
  strict-green, whole-catalog strict-green, the Gcd import-reuse, and the
  34-residual triage are NOT in Component A. Component A MUST NOT reintroduce
  ambient resolution, invent competing identities, or restore prelude fallback to
  make any red unit appear green — that is the ruled-out option 3 in disguise.

# The hard-stop chain — one defect, resolved (Architect §1b + §1a research pull)

Component A took THREE boundary hard stops, each a ruled mechanism refuted by the
build. The Architect's symptom inventory (evt_1f4za019khrs1), keyed on what A's
AC required that the boundary withheld:
1. HS#1 — strict whole-catalog co-gate placed in A: A's AC required a NAME
   (`Nat`) only B's catalog authoring can resolve.
2. HS#2 — AC-A2 needs the provider-internal Transport imports (a Steward boundary
   call cut them to B): A's AC required the providers' transitive IMPORT EDGES.
3. HS#3 — legacy Order -> Or `Inl`/`Inr` unbindable (ctor prebind was
   strict-only): A's AC required the providers' transitive CLOSURE to elaborate
   under ONE coherent mode.
4. HS#4 — legacy Order -> `UnresolvedCon{bool_or}` (Order's unqualified
   `IsTrue`/`bool_or`/`Ord` resolve to `Core.Classes.LawfulClasses`, an edge a
   spelling-scoped premise missed): A's AC required a provider-internal import
   edge not yet authorized. Same class as HS#2.

§1b STRUCTURAL CLOSURE (Architect evt_3pfr8hgp29m69, ending the per-edge chain):
HS#2 and HS#4 are the same defect — authorizing provider-internal edges one at a
time, where an `UnresolvedCon` names only the FIRST missing name, so each
authorization guarantees the next stop. The ruling pre-authorizes the ENTIRE
provider-internal transitive closure to legacy-elaborability as Component A work
(see the Deliverable), with the A/B discriminator and the homeless-convenience
seam as the only remaining genuine stop. The HS#3 loader fix is build-CONFIRMED
(canonical Or loads under legacy; Arithmetic proceeds).

HS#5 — the homeless-convenience seam fired exactly as designed (Architect ruling
evt_613d9fm7j45qj): Order's closure via LawfulClasses reaches the HOMELESS
`OrdResult` (two private competing decls, no defining public interface). The
capability-vs-dependency discriminator classifies it on the DEPENDENCY side
(unlike Or, whose ctors A already had and the loader just failed to bind):
`OrdResult` must be CREATED as a canonical home = catalog completeness = B's
charter. RESOLUTION — the chain CONVERGES: `OrdResult`'s canonical home -> B;
AC-A2 SPLITS by self-measurability (Arithmetic add/mul identity stays in A, its
closure being Transport-only; Order's leq_nat/sub identity + Order's provider
surface move to B); Component A is reduced to its HOMELESS-FREE, self-measurable
closure (loader fix + Or pin + Arithmetic + green set) with no further hard stop.
The recurring lesson holds and is now discharged: before re-homing an AC, ask
whether the closure member the component cannot supply is its OWN latent
capability defect (Or -> the loader fix, kept in A) or genuinely the OTHER
component's deliverable (Nat, OrdResult -> created in B).

SHARED PREDICATE (confirmed): a `GlobalId` is signature-gated (elab.rs:7702-7733)
and exists only after the provider FULLY elaborates; full elaboration pulls the
provider's ENTIRE transitive dependency closure (Transport; Or + its ctors; the
`Nat` signatures), which must resolve under ONE loader mode (mixed forbidden,
modules.rs:854). So genuine-identity measurement is INSEPARABLE from full-closure
elaboration under a unified mode — there is no thin identity-only AC-A2 and no
improvised signature phase (research advisory evt_560tmsey7pwy0 +
evt_51csyb25926kf; Ken has no signature artifact).

THE RESOLUTION (Architect §1a research pull, then mechanism ruling
evt_41tpyzwkc14ez): the closure member A "could not supply" (Or's own `Inl`/`Inr`)
is NOT B's catalog deliverable — it is A's OWN loader failing to bind a
declaration that ALREADY EXISTS in the catalog. That is a capability defect in
the loader A charters, not a decomposition defect. The two axes are separate:
external fallback (`Nat`) lives in `resolve_ref` and stays mode-governed; local
declaration scope (Or's ctors) lives in `prebind_scope_declarations` and, per
§3.3, is unconditional. Unify the closure under ONE legacy run by fixing the
loader (the Deliverable's uniform local-scope rule) — not by moving AC-A2 to B.
The prior ee9ddcfde amend's legacy-mode AC-A2 claim WITHOUT the loader fix is
superseded by this ruling; the fix makes the legacy measurement genuinely
achievable. B still owns strict completeness.

The recurring lesson, folded here per §1: a decomposition boundary drawn through
an un-cuttable dependency closure makes each component discover, one at a time, a
closure member it cannot supply — but before re-homing the AC, check whether the
unsuppliable member is genuinely the OTHER component's deliverable or the SAME
component's own latent capability defect. Here it was the latter three times over.

# Reviewers

Architect (component fit; loader realization against the authorized surface only;
no invented identity) + conformance-validator (identity-preserving pub/import
resolution).

# Capability tier

T1 — RAISED from T2 by the Architect mechanism ruling (evt_41tpyzwkc14ez). A now
carries a bounded loader-soundness fix (the uniform local-scope rule, decoupling
local-declaration binding from `ResolutionMode` in `prebind_scope_declarations`,
structural over three declaration kinds) with a genuine subtlety — the legacy
recursive-spine STACK BUDGET (AC-A6) the coupling deliberately protected must be
preserved and pinned at the site, not asserted. The mechanical pub/import edits
are T2, but the loader fix + stack-budget pin + declaration-kind-coverage
reasoning demand T1. Kick-time seat check: confirm the assigned language seat is
provisioned for the loader-soundness reasoning (high effort), not just the
mechanical edits. Size M.
