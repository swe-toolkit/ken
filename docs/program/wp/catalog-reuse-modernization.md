# Catalog-reuse modernization campaign — charter

Operator direction 2026-08-26 (after the three-lane feasibility trial passed and
the foundation lane was told to continue). Steward-owned; lane-3 (foundation),
concurrent and contention-free with the runtime priority lane (touches `catalog/`
and `docs/`, not the runtime crates).

## Why

The catalog packages were authored before two capabilities landed: the expanded
prelude (canonical `Nat` floor and friends) and working module imports with
`pub` exports. As a result many packages carry avoidable debt:

- definitions that duplicate what the prelude now provides;
- tools reimplemented locally that a sibling catalog module now `pub`-exports
  (import-reuse was not available when they were written);
- bottom-up file arrangement (fundamentals first) rather than the top-down
  arrangement the catalog implementation standard now prescribes.

None of this is a soundness defect — the packages are verified. It is factoring,
redundancy, and arrangement, which no soundness gate checks. The catalog
implementation standard (landed `90e31409e`) is the yardstick.

## Shape (operator-concurred 2026-08-26): census-first

1. **Pilot** — `CAT-ORDER-PUB-EXPORT` then `CAT-GCD-REFACTOR`. Gcd/Order is the
   first package worked end to end; it proves the per-package recipe (import
   canonical tools, drop reimplementations, arrange top-down) and shakes out the
   prerequisite pattern (a consumed module must `pub`-export its tools first).
2. **Census** — `CAT-REUSE-CENSUS`. A catalog-wide inventory that sizes the
   campaign: per package, which defs are prelude-redundant, which are
   sibling-reimplemented, and whether arrangement is bottom-up. Each item is
   **risk-tagged** so depth is decided per-item, not globally.
3. **Scoped rework** — from the census, a bounded set of per-package rework WPs
   (or a small number of grouped ones), sequenced by the Steward. Not framed
   blind; framed from the census.

## Depth (operator default: conservative, risk-tagged)

- Low risk: replace a local computational-tool duplicate with an import of the
  canonical `pub` tool; pure top-down rearrangement.
- Higher risk (soundness-sensitive): removing local proof scaffolding in favour of
  imported laws, or anything that changes a package's proof obligations. The
  census tags these; they are worked only on an explicit per-item decision, and a
  genuine mechanism gap HARD-STOPS to spec/Architect.

## Guardrails

- Reuse, do not reimplement; subsume, do not proliferate (`docs/PRINCIPLES.md`).
- No computational meaning changes without the package oracle staying green.
- Reviewers: foundation-qa + conformance-validator (catalog standard). A design/
  spec gap (pub eligibility, attached-proof ownership) is routed to spec/Architect,
  not forced — a gap finding is a payoff of the campaign, exactly as in the trial.
- One package's changes per WP where practical (keeps diffs single-package and
  review differential); a shared prerequisite (like a module's pub-export) is its
  own small WP.
