---
id: LANG-MOD-CATALOG-REALIZATION
title: "WP-4 — catalog realization: mark pub on Arithmetic/Order, add the imports, and make Gcd import add/mul + leq_nat/sub instead of reimplementing them; each checks standalone through the real loader (campaign SUCCESS)"
status: draft
owner: language
size: M
gate: none
depends_on: [LANG-MOD-LOADER-ENTRY, LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-STRICT-RESOLUTION]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-4), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. FRAMED; release HELD (see campaign root release gate + the Or/Inl/Inr fork)."
---

> # FRAMED — HELD FOR RELEASE. Last in ring order; TWO extra gates.
>
> Gated on WP-1..3 ([[LANG-MOD-LOADER-ENTRY]], [[LANG-MOD-PUB-ELIGIBILITY]],
> [[LANG-MOD-STRICT-RESOLUTION]]) AND the Or/Inl/Inr fork (escalated to operator,
> evt_6b9wrt1kwswcp — under strict there is no global-fallback escape hatch, so
> any file using Or/Inl/Inr, e.g. Order, cannot be strict-clean until the fork
> gives them a public home or refactors them away). Also held under the campaign
> finish-then-switch gate. This WP is the campaign's success criterion.

# Objective

Realize catalog reuse: a catalog entry imports canonical operations instead of
reimplementing them, and every touched module checks standalone through the real
loader.

# Deliverable (Architect evt_hpnhqy1ex286)

- Mark `pub` on Arithmetic (`add`, `mul`) + Order (`leq_nat`, `sub`) and their
  required provider names public.
- Add the imports: Arithmetic imports Core.Logic.Transport (`cong`, `sym`,
  `trans`); Order imports Transport + Classes.LawfulClasses.
- Gcd selectively imports `add`/`mul` + `leq_nat`/`sub` and REMOVES its four
  local reimplementations.

# The census (Architect evt_xtscdw8r3q3k)

WP-4's migration set is CENSUS-DRIVEN, not the Gcd trio alone: measure EVERY
catalog file that the strict flip breaks (every file that currently resolves a
non-floor name through the global passthrough) and migrate all of them, so WP-2's
strict flip and WP-4's migration co-gate to CI-green together.

# Acceptance criteria

- AC-1. Arithmetic, Order, Gcd each check STANDALONE through the real loader.
- AC-2. Gcd's imports resolve to the exact provider IDs with no Gcd-owned
  competing identity — establish no-reimplementation by IDENTITY, not repo text.
- AC-3. The full census set migrates and the whole catalog is strict-green in CI
  (the co-gate with [[LANG-MOD-STRICT-RESOLUTION]]).
- AC-4 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Architect (component fit; no invented identity) + conformance-validator
(identity-preserving import resolution).

# Capability tier

T2 for the mechanical import/pub edits + census execution; the SOUNDNESS is
carried by WP-2/WP-3. Size M (may grow with census breadth). The Or/Inl/Inr fork
must resolve before Order is dependency-closed.
