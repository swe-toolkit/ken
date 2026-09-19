---
id: CAT-NAT-ORDER-LAWS
title: "Proof-backfill for Data/Numeric/Nat/Order.ken.md: land the inductive sub/leq fragment the package explicitly defers (self-subtraction, saturation, strict decrease), then the min/max/compare theory. Dependency-bottom node of the proof-completeness survey's 17 follow-ons: its sub/leq lemmas are what the downstream *-LAWS packages reduce their own bounds reasoning to."
status: merged
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-PROOF-COMPLETENESS-SURVEY]
blocks: []
github: null
origin: "Operator ruling 2026-09-13 (Pat): 'A catalog package is not finished until its proofs are complete... computational tests are not part of the package, so a reader cannot trust them as intrinsics.' Recorded as docs/PRINCIPLES.md #16. That ruling directed the catalog proof-completeness survey and said its gaps become proof-completion nodes. The survey LANDED 2026-09-19 as ec23d4ab6 (report docs/CATALOG-PROOF-COMPLETENESS-SURVEY.md, +162) naming 18 genuine proof-backfill packages, 17 of them needing nodes framed. This is the first of those 17, selected by the Steward as the dependency bottom (steward.md §3: sequencing is the Steward's call) -- see the frame §1a for the measurement that established the ordering."
---

# Proof-backfill: `Data/Numeric/Nat/Order.ken.md`

The survey's row for this package, verbatim:

> Provider order laws and three local zero laws are intrinsic. The source
> explicitly names `sub n n = Zero` as unproved; broader min/max/sub/compare
> algebra is absent.
>
> `CAT-NAT-ORDER-LAWS`: prove the complete min/max/sub/compare theory,
> including self-subtraction, over the current structural definitions and
> canonical `leq_nat` with no new trust.

The package does not merely lack these proofs — it **names the gap in its own
prose** and carries the failing attempt in a ` ```ken reject ` fence
(`catalog/packages/Data/Numeric/Nat/Order.ken.md:146`). That is the honest
posture PRINCIPLES #16 now supersedes: the package is not finished until the
proof lands.

## Why this node is first of the seventeen

Not a priority judgment about `Nat` — a measured dependency. See the frame's
`§1a`. In short: the downstream `*-LAWS` packages discharge their bounds
obligations by reducing them to `sub`/`leq` facts that **do not exist yet**, so
framing any of them first hands an implementer a node whose first move is to
invent `Nat` lemmas belonging to a different package.

## Scope

Two deliverables, `D1` releasable on its own (`steward.md` §4a: accepted work
merges as soon as it is done, even a partial WP).

- **`D1` — the inductive `sub`/`leq` fragment.** The four lemmas the deferral
  names or the downstream consumers need. This is the increment being released.
- **`D2` — the min/max/compare theory.** Held; framed when `D1` lands.

## Not this node

- No change to any definition in the package. The proofs are over the current
  structural `min`/`max`/`sub`/`compare` and the canonical `leq_nat`.
- No new trust: no `Axiom`, postulate, primitive, `Omega` path carrier, TCB
  change, or kernel change. The survey states this constraint for all 17 rows
  and it is an acceptance criterion here.
- Not machine-checked complexity. The survey excludes it explicitly.
- Not the other sixteen survey follow-ons. They are named in
  `docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md` and stay unframed until
  released one at a time.
