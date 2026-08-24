---
id: LANG-MOD-CATALOG-COMPLETENESS
title: "WP-4 Component B — catalog completeness: give Nat one canonical public home (and the other required convenience homes), migrate the consuming units to import them (Gcd imports add/mul + leq_nat/sub + Nat and drops its reimplementations), and satisfy whole-catalog strict-green. This is the module/import campaign's catalog-reuse success step."
status: draft
owner: language
size: L
gate: none
depends_on: [LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Architect ruling evt_214z6r6qnwme0 (2026-08-24), unbundling the WP-4 strict whole-catalog co-gate off Component A. The co-gate gates a deliverable outside WP-4's authorized surface (a canonical home for the type Nat), so it is re-homed here. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # DRAFT — held on Component A; released when A lands.
>
> Component B carries the substance the WP-4 hard stop exposed
> (language-implementer evt_3j60e77n0ahsy, Architect ruling evt_214z6r6qnwme0):
> the catalog cannot go whole-catalog strict-green while the type `Nat` (and
> other prelude conveniences the 42 red leaves consume) has no canonical catalog
> home. Component A delivers the loader realization + the provider public surface;
> B releases the missing homes and migrates the consumers. It depends on A (the
> providers must be public before consumers migrate) and is released once A lands.
>
> This is the module/import campaign's catalog-reuse SUCCESS step: when B lands
> its whole-catalog strict-green, the campaign root [[LANG-MODULE-IMPORT-SYSTEM]]
> can close and foundation [[CAT-GCD-REFACTOR]] unblocks.

# Objective

Catalog completeness under strict resolution: every convenience the catalog uses
resolves from a defining public interface (no ambient), and the whole catalog is
strict-green through the real loader.

# Deliverable

- ONE canonical `Nat` — `data Nat` with `Zero`/`Suc`, exported from a SINGLE
  defining public interface at the right catalog location, imported by
  Arithmetic/Order/Gcd and every other consumer. Subsume-don't-proliferate: one
  canonical home, the NODE B (`Core.Logic.Or`) canonical-home pattern — NOT
  per-consumer copies, NOT ambient, NOT an invented identity (Architect design
  constraint, ruling point 5).
- Canonical public homes for the OTHER conveniences the 42 red leaves consume
  (census-driven — see the triage AC below).
- Migrate the consuming units to import the canonical `Nat` home so they resolve
  under strict: Arithmetic/Order/Gcd import the canonical `Nat` (the
  provider-INTERNAL Transport imports on Arithmetic/Order are already delivered by
  Component A and carry forward — B does NOT re-add them). Gcd selectively
  imports `add`/`mul` (Arithmetic) + `leq_nat`/`sub` (Order) + `Nat`, and REMOVES
  its four local reimplementations.
- Move the real caller to STRICT (`elaborate_module_from_roots` strict mode)
  after the dependency census has migrated — this is the flag-day the legacy A
  loader defers.
- FORWARD-COMPAT identity preservation (Architect ruling evt_47t9dwz0chstv):
  strict excludes the native prelude `Nat`, so B re-homes `Nat` to a canonical
  catalog interface and migrates the providers to import it — but B MUST PRESERVE
  the provider identities (`add`/`mul`/`leq_nat`/`sub`) that Component A's pub
  surface already exposed and measured under AC-A2. No competing provider
  identity is minted by the `Nat` re-home. This is the NODE B canonical-home
  pattern (as `Core.Logic.Or` replaced the prelude `Or`).

# The census (re-homed from WP-4; drift corrected)

The behavior census at base `f0e0b92fa` is 45 leaves = 3 strict-green (the
self-contained green set, delivered by Component A) + 42 red. WP-2 D0's recorded
premise of 32 baseline-red residuals is STALE: the current behavior census is 34
baseline-red residuals (language-implementer evt_3j60e77n0ahsy). Completion
ranges over the real population, not the stale count.

# Acceptance criteria

- AC-B1 (the re-homed co-gate). The whole catalog is strict-green in CI — the
  co-gate with [[LANG-MOD-STRICT-RESOLUTION]], whose remaining whole-catalog
  strict enforcement / CI closure co-closes here. Local targeted `-p` only;
  whole-catalog strict-green is a CI gate, never a local `--workspace` run.
- AC-B2. Arithmetic, Order, and Gcd each check STANDALONE through the real loader
  under strict (the AC-1 re-homed from WP-4 — now satisfiable because `Nat`
  resolves through its canonical home).
- AC-B3 (residual triage). Completion ranges over the 34-residual population:
  each residual is either migrated to strict-green OR explicitly excluded with a
  stated reason. "Every census vector empty" is NOT sufficient — enumerate the
  disposition of all 34.
- AC-B4. Gcd's four imports resolve to the exact provider IDs with no Gcd-owned
  competing identity — establish no-reimplementation by IDENTITY, not repo text.
- AC-B5 (canonical-home identity). Exactly one `data Nat` exists in the catalog
  (one defining interface); every consumer resolves `Nat` to it. No second
  `Nat` identity, no ambient/floor `Nat`.
- AC-B6 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-B-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.
- END-STATE INVARIANT (Architect ruling point 4). The strict contract is
  correct: every name resolves from a defining public interface, no ambient. B
  delivers that end state; it does NOT weaken strict to reach green.

# Reviewers

Architect (canonical-home component fit; one-canonical-Nat; no invented identity)
+ conformance-validator (identity-preserving import resolution; strict-green
census disposition).

# Capability tier

T1 for the canonical `Nat`-home design (the defining interface + its catalog
location, one-canonical-Nat) and the strict-green closure argument; T2 for the
mechanical convenience-home authoring + consumer migration + census execution.
Size L (may grow with the convenience-home breadth the 34 residuals require).
