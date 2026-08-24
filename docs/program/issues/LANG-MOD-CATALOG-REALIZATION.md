---
id: LANG-MOD-CATALOG-REALIZATION
title: "WP-4 — catalog realization: mark pub on Arithmetic/Order, add the imports, and make Gcd import add/mul + leq_nat/sub instead of reimplementing them; each checks standalone through the real loader (campaign SUCCESS)"
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-MOD-LOADER-ENTRY, LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-OR-CANONICAL-HOME]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-4), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. FRAMED; release HELD (see campaign root release gate + the Or/Inl/Inr fork)."
---

> # RELEASED 2026-08-24 — module/import campaign SUCCESS step; kicked on the language ring.
>
> All predecessors are landed: [[LANG-MOD-LOADER-ENTRY]] merged,
> [[LANG-MOD-PUB-ELIGIBILITY]] merged, and
> [[LANG-MOD-OR-CANONICAL-HOME]] merged (NODE B respin e1509b88d,
> closed cf8dc2724) — so `Order` is dependency-closed. The
> finish-then-switch gate is satisfied: the language ring finished its
> then-current WP (NODE B) and this is the module/import completion it
> switches to.
>
> STRICT-RESOLUTION IS A CO-GATE, NOT A PREDECESSOR (Steward ruling
> evt_6adrfngmdq3b5, on the language-leader's grounded answer
> evt_ptbmn70tymsf). Its consumed D1 strict machinery is LANDED
> (5a74301f4, ancestor of main); [[LANG-MOD-STRICT-RESOLUTION]] stays
> `ready`/open only because its remaining whole-catalog strict enforcement /
> CI closure co-closes WITH this WP's catalog migration (AC-3). Keeping it
> in this node's hard `depends_on` would circularly block it
> (STRICT-RESOLUTION cannot close until WP-4 delivers), so it is dropped
> from `depends_on` — the code dependency is already satisfied — and WP-4
> CO-DELIVERS STRICT-RESOLUTION's remaining closure. STRICT-RESOLUTION
> closes when this WP lands its co-gated strict-green.
>
> This WP is the campaign's success criterion, and closing it unblocks foundation
> [[CAT-GCD-REFACTOR]].
>
> THE OR/INL/INR FORK IS RESOLVED (operator ruled arm (b), evt_6b9wrt1kwswcp):
> `Or`/`Inl`/`Inr` get a canonical package home, not a refactor-away. Under strict
> there is no global-fallback escape hatch, so any file using them (e.g. `Order`)
> cannot be strict-clean until they resolve through a legal import. The Architect
> found arm (b) is UNSPELLABLE in surface data syntax today (Omega-sorted params;
> the data elaborator has no Omega arm), so the realization is TWO prerequisite
> nodes, sequenced A -> WP-2 D1 -> B:
> - [[LANG-MOD-OR-OMEGA-PARAM-ELAB]] (NODE A) — teach explicit-data param/index
>   elaboration to honor an Omega-sorted binder (enclave sort-discipline GO
>   evt_3j02n0pkgze3a). Buildable now; release-ordered behind WP-2 D1.
> - [[LANG-MOD-OR-CANONICAL-HOME]] (NODE B) — author `Core.Logic.Or`, migrate the
>   six consumers, retire the prelude registration (one identity). WP-4 depends on
>   NODE B; once it lands, `Order` is dependency-closed.
>
> Also held under the campaign finish-then-switch gate.

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

# The census (Architect evt_xtscdw8r3q3k; measured by WP-2 D0)

WP-4's migration set is CENSUS-DRIVEN, not the Gcd trio alone: measure EVERY
catalog file that the strict flip breaks (every file that currently resolves a
non-floor name through the global passthrough) and migrate all of them, so WP-2's
strict flip and WP-4's migration co-gate to CI-green together.

WP-2 D0 measured the census (landed `c64c62190`, recorded on
[[LANG-MOD-STRICT-RESOLUTION]]): a 12-route ambient-name inventory
(`COMPLETE_AMBIENT_NAME_ROUTES`, keyed on representation + consumer, not surface
spelling), partitioned disjoint/exhaustive into 2 floor-clean units, 10
ambient-dependent units to migrate, and 32 baseline-red residuals. That 32-residual
bucket is the triage obligation below (AC-3a).

# Acceptance criteria

- AC-1. Arithmetic, Order, Gcd each check STANDALONE through the real loader.
- AC-2. Gcd's imports resolve to the exact provider IDs with no Gcd-owned
  competing identity — establish no-reimplementation by IDENTITY, not repo text.
- AC-3. The full census set migrates and the whole catalog is strict-green in CI
  (the co-gate with [[LANG-MOD-STRICT-RESOLUTION]]).
- AC-3a (residual triage — Adversary flag, D0 post-merge sweep evt_1e3tpt44qxjkm).
  Completion must range over the 32-residual bucket: each residual is either
  migrated to strict-green OR explicitly excluded with a stated reason. "Every
  census vector empty" is NOT sufficient as a completion signal — the D0 vectors
  cover only the 12-route ambient inventory, so an empty-vectors verdict would
  declare done while the 32 residuals (73% of the 44 discovered) stay unmeasured.
  Enumerate the disposition of all 32.
- AC-4 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Architect (component fit; no invented identity) + conformance-validator
(identity-preserving import resolution).

# Capability tier

T2 for the mechanical import/pub edits + census execution; the SOUNDNESS is
carried by WP-2/WP-3. Size M (may grow with census breadth). `Order` is
dependency-closed once [[LANG-MOD-OR-CANONICAL-HOME]] (NODE B) lands `Core.Logic.Or`
and the six-consumer migration; WP-4 depends on it.
