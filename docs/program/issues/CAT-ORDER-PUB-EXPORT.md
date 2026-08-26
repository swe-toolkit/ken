---
id: CAT-ORDER-PUB-EXPORT
title: "Bring catalog Data/Numeric/Nat/Order.ken.md to the pub-export standard — mark its declared exported operations (leq_nat, sub, min, max, compare) pub so packages can selectively import them, matching the already-compliant Arithmetic.ken.md; verify standalone elaboration and cross-package import resolution. The catalog-reuse prerequisite that unblocks CAT-GCD-REFACTOR."
status: merged
owner: foundation
size: S
gate: none
depends_on: [LANG-MOD-PUB-ELIGIBILITY, CAT-ORD-NAT-CANONICAL-OWNER]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Steward, 2026-08-26, on operator direction to continue the foundation lane after the three-lane feasibility trial passed. Measured prerequisite for [[CAT-GCD-REFACTOR]] (held since 2026-08-22 on a falsified import-surface premise): Arithmetic.ken.md now pub-exports add/mul and imports its transport lemmas (standalone-load failure resolved), but Order.ken.md still has ZERO pub exports (leq_nat/sub — the two tools Gcd needs — are plain fn). The reuse mechanism is demonstrated (Arithmetic pub-exports fns WITH attached proofs and elaborates), so this is execution, not design. Steward-filed per COORDINATION section 2."
---

> # SPLIT 2026-08-26 — Architect ruling (evt_6f4h4mhejp4bm): this node is now the NARROW half
>
> The verify-first D0 hard-stop was correct, and the Architect ruled the pub-only
> repair fork INCOMPLETE: neither an Order-local theorem nor a provider-owned
> `bool_or` bridge alone is lawful closure, because both leave the current
> `instance Ord Nat` ORPHANED in Order (reproduced as `OrphanInstance { class:
> "Ord", head_type: "Nat" }` on a scratch tree at `2afacd0c0`). The root predicate
> is one defect: Order was authored against ambient-provider scaffolding, so one
> file appears to own declarations that strict package ownership assigns to
> different defined-at homes.
>
> The node is SPLIT. The two-package ownership migration is the fresh prerequisite
> [[CAT-ORD-NAT-CANONICAL-OWNER]] (`ready`, kicked separately) — it moves `leq_nat`
> + its laws, `total_leq_nat`, the `bool_or::eq_true_of_or` bridge, and the sole
> `instance Ord Nat` to their defined-at home `Core.Classes.LawfulClasses`, and
> makes Order a facade. This node stays `draft` `depends_on` it and RESUMES NARROWLY
> after it lands: mark ONLY Order's remaining owned operations `min`/`max`/`sub`/
> `compare` public. [[CAT-GCD-REFACTOR]] stays blocked until BOTH successors land.
>
> The Deliverables and ACs below are REPLACED to the narrow resume scope: the prior
> D0/AC-3/AC-4 (which asserted `leq_nat`'s laws and `Ord Nat` stay Order-owned) and
> the prior D1 (which asserted both `sub` and `leq_nat` are Order definitions) are
> FALSE after the migration and are superseded, not annotated around. `leq_nat` is
> the re-exported LawfulClasses identity; `total_leq_nat` is no longer an Order
> export candidate. Do NOT start this node's source work before
> [[CAT-ORD-NAT-CANONICAL-OWNER]] lands. Systemic sizing of this defect class is in
> [[CAT-REUSE-CENSUS]].

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. Order's standalone boundary borrowed unimported canonical dependencies and
   declared both a foreign `bool_or` attachment and the orphan `Ord Nat`
   instance — keyed on ambient-provider scaffolding instead of defined-at
   package ownership.

## Objective

Bring `catalog/packages/Data/Numeric/Nat/Order.ken.md` into line with the catalog
implementation standard's pub-export requirement, matching the already-compliant
`Arithmetic.ken.md`: the module's declared exported operations must be `pub` so a
consuming package can selectively import them instead of reimplementing them.

## Measured state (PRE-migration snapshot; re-measure after the prerequisite lands)

This snapshot predates [[CAT-ORD-NAT-CANONICAL-OWNER]]. After that prerequisite,
`leq_nat` (and its laws), `total_leq_nat`, the `bool_or::eq_true_of_or` bridge, and
`instance Ord Nat` no longer live in Order — Order re-exports the LawfulClasses
`Ord`/`leq_nat` surface. Only `min`/`max`/`sub`/`compare` remain Order-owned here.

- `Arithmetic.ken.md` is the WORKING REFERENCE: `add` and `mul` are `pub fn` (2
  pub, 0 plain), it imports `Core.Logic.Transport (cong, sym, trans)`, and it
  elaborates. A `pub fn` WITH attached proofs (`proof zero_r for add`, etc.) is
  already exported successfully — the reuse mechanism is proven, not open.
- `Order.ken.md` has ZERO `pub fn` (6 plain `fn`: `leq_nat`, `total_leq_nat`,
  `min`, `max`, `sub`, `compare`). Its own header declares `min`/`max`/`sub`/
  `compare` (plus the `Ord Nat` instance and `leq_nat`) as the entry's exports, but
  none carry `pub`. It imports `Core.Logic.Or (Or, Inl, Inr)` (canonical home
  landed).
- Consuming package `Gcd.ken.md` needs `leq_nat` and `sub` from Order (and
  `add`/`mul` from Arithmetic) — see [[CAT-GCD-REFACTOR]].

## Deliverables (narrow resume — starts only after [[CAT-ORD-NAT-CANONICAL-OWNER]] lands)

- D0 (mark the remaining Order-owned operations public) — mark `min`, `max`, `sub`,
  and `compare` as `pub fn` in the facade Order module, and PROVE: (a) `Order.ken.md`
  elaborates standalone at Omega as the facade (importing and re-exporting the
  LawfulClasses `Ord`/`leq_nat` surface per the prerequisite); (b) each newly-`pub`
  fn passes the `LANG-MOD-PUB-ELIGIBILITY` gate (top-level, public-typed subject).
  `leq_nat` is NOT marked `pub` here — it is the re-exported LawfulClasses identity.
  `total_leq_nat` is no longer an Order export candidate (moved to LawfulClasses,
  provider-private). Any NEW gap HARD-STOPS to spec/Architect.
- D1 (import-resolution witness) — a minimal probe selectively imports `sub` and
  `leq_nat` from `Data.Numeric.Nat.Order` and resolves them: `sub` reaches the Order
  definition's `GlobalId`, and `leq_nat` reaches the re-exported LawfulClasses
  canonical `GlobalId` (NOT an Order definition). Re-verify the import surface at
  pickup, not inherited.

## Acceptance criteria

- AC-1 — Order's remaining owned operations `min`/`max`/`sub`/`compare` are
  `pub fn`; `Order.ken.md` elaborates standalone as the facade with no
  `UnresolvedCon` and no eligibility rejection.
- AC-2 — a selective import from `Data.Numeric.Nat.Order` resolves `sub` to the
  Order `GlobalId` and `leq_nat` to the re-exported LawfulClasses canonical
  `GlobalId`, showing both are reachable by a consuming package through the facade.
- AC-3 — Order registers no `Ord` instance and mints no `leq_nat`/bridge alias; a
  consumer's `where Ord Nat` through Order resolves to the LawfulClasses dictionary
  identity (the [[CAT-ORD-NAT-CANONICAL-OWNER]] ownership migration is intact).
- AC-4 — no computational content of the retained Order operations changes (`pub`
  is an export-visibility change only); Order's existing oracle / conformance rows
  stay green.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted only, never
  `--workspace`.

## Reviewers

foundation-qa (the `pub` markings pass eligibility, the module elaborates
standalone, import resolution reaches the Order `GlobalId`s, no computational
change) + conformance-validator (catalog implementation standard compliance). A
design/spec gap (attached-proof ownership, eligibility) HARD-STOPS to
spec/Architect.

## Capability tier

T2 — applying a landed capability (pub export, proven by Arithmetic) to one more
catalog module, reviewed on the export surface and the import-resolution witness,
not a novel design. Size S. If D0 surfaces a genuine mechanism gap it escalates to
T1 spec/Architect via hard stop.

## Sequencing

Lane-3 (foundation). Held `draft` behind the prerequisite
[[CAT-ORD-NAT-CANONICAL-OWNER]] (the two-package ownership migration); resumes
narrowly once that lands. Blocks [[CAT-GCD-REFACTOR]] — the Gcd-only reuse refactor
resumes only after BOTH this node and the prerequisite land.
