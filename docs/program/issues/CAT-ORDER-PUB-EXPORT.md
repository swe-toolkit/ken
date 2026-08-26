---
id: CAT-ORDER-PUB-EXPORT
title: "Bring catalog Data/Numeric/Nat/Order.ken.md to the pub-export standard — mark its declared exported operations (leq_nat, sub, min, max, compare) pub so packages can selectively import them, matching the already-compliant Arithmetic.ken.md; verify standalone elaboration and cross-package import resolution. The catalog-reuse prerequisite that unblocks CAT-GCD-REFACTOR."
status: draft
owner: foundation
size: S
gate: none
depends_on: [LANG-MOD-PUB-ELIGIBILITY]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Steward, 2026-08-26, on operator direction to continue the foundation lane after the three-lane feasibility trial passed. Measured prerequisite for [[CAT-GCD-REFACTOR]] (held since 2026-08-22 on a falsified import-surface premise): Arithmetic.ken.md now pub-exports add/mul and imports its transport lemmas (standalone-load failure resolved), but Order.ken.md still has ZERO pub exports (leq_nat/sub — the two tools Gcd needs — are plain fn). The reuse mechanism is demonstrated (Arithmetic pub-exports fns WITH attached proofs and elaborates), so this is execution, not design. Steward-filed per COORDINATION section 2."
---

> # HELD 2026-08-26 — D0 HARD-STOP: pub-only frame cannot authorize the fix; design ruling routed to Architect
>
> The verify-first D0 fired as designed. Foundation-leader hard-stopped
> (evt_6s39mc9d7a5cp) at clean branch `2afacd0c0`: this is a FALSE D0(a) premise,
> not a pub-only implementation defect. `Order.ken.md` does NOT elaborate standalone
> PRE-EDIT — it fails at `leq_nat::antisym` — and resolving its actual dependencies
> exposes a nonlocal attached-proof ownership conflict at
> `bool_or::eq_true_of_or`.
> That trips D0 clause (c) (attached-proof ownership) and clause (a)
> (standalone elaboration): both were written as hard-stop-to-spec/Architect gates,
> and they caught the gap before any edit. This is the campaign's intended payoff —
> a gap finding, not a failed WP.
>
> The pub-only frame CANNOT authorize the required provider/attachment change.
> Foundation-leader routed the component-ownership design ruling to the Architect.
> This node HOLDS (`ready` -> `draft`) pending that ruling. Do NOT accept a
> visibility-only candidate or D1 as a deliverable — the D0 hard-stop stands, and
> marking `pub` without resolving the attached-proof ownership would move the subject
> below the boundary the node claims. On the Architect ruling, the Steward reframes:
> the fix likely needs a provider/attachment WP AHEAD of any pub-only step, so this
> node's shape depends on that ruling. [[CAT-GCD-REFACTOR]] stays blocked on this.
>
> Systemic sizing of this wall is now routed to [[CAT-REUSE-CENSUS]] (amended to
> record standalone-load + attached-proof ownership per module, not just pub status).
>
> ---
>
> Prior release banner (superseded by the hold above):
> The foundation lane's next step, extracted from [[CAT-GCD-REFACTOR]]'s falsified
> premise per that node's 2026-08-22 scope ruling (do NOT fold dependency-package
> repairs into the Gcd-only WP). Arithmetic is already compliant; Order is not.

## Objective

Bring `catalog/packages/Data/Numeric/Nat/Order.ken.md` into line with the catalog
implementation standard's pub-export requirement, matching the already-compliant
`Arithmetic.ken.md`: the module's declared exported operations must be `pub` so a
consuming package can selectively import them instead of reimplementing them.

## Measured state (ground at current origin/main; re-measure before building)

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

## Deliverables (verify FIRST — the 2026-08-22 hold's lesson was an unverified premise)

- D0 (verify the export surface) — mark Order's declared exported operations
  (`leq_nat`, `sub`, `min`, `max`, `compare`; keep `total_leq_nat` internal unless
  the module contract says otherwise) as `pub fn`, and PROVE: (a) `Order.ken.md`
  elaborates standalone at Omega; (b) each newly-`pub` fn passes the
  `LANG-MOD-PUB-ELIGIBILITY` gate (top-level, public-typed subject); (c) `leq_nat`'s
  attached recursive laws (`leq_nat::refl/trans/antisym`) remain owned by Order and
  do not reject at declaration under attached-proof ownership. If any of (a)-(c)
  fails, HARD-STOP and route the gap to spec/Architect — a gap finding is the
  trial's payoff; do NOT force it.
- D1 (import-resolution witness) — a minimal probe selectively imports `leq_nat`
  and `sub` from `Data.Numeric.Nat.Order` and resolves them; the exact `GlobalId`
  reached is the Order definition, not a reimplementation.

## Acceptance criteria

- AC-1 — Order's declared exported operations are `pub fn`; `Order.ken.md`
  elaborates standalone with no `UnresolvedCon` and no eligibility rejection.
- AC-2 — a selective import of `leq_nat` and `sub` from `Data.Numeric.Nat.Order`
  resolves to the Order `GlobalId`s (import-resolution witness), showing the tools
  are reachable by a consuming package.
- AC-3 — `leq_nat`'s attached laws stay owned by Order (no nonlocal
  attached-declaration rejection); the `Ord Nat` instance and its postulated laws
  are unchanged in meaning.
- AC-4 — no computational content of Order changes (`pub` is an export-visibility
  change only); Order's existing oracle / conformance rows stay green.
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

Lane-3 (foundation). Releasable now (dep `LANG-MOD-PUB-ELIGIBILITY` merged). Blocks
[[CAT-GCD-REFACTOR]] — the Gcd-only reuse refactor resumes after this lands.
