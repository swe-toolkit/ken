---
id: CAT-MIGRATE-EC-CLOSURE-PROVIDERS
title: "EC standalone-cleanness predecessor (off the DecEq critical path): widen the LF / Derived / Transport PUBLISHED surfaces to the exact free-symbol closure set EffectfulClasses names across package boundaries, so EC can import that closure and elaborate standalone. Mechanical export publication (mark pub + extend each provider's loader-visible inventory), INCLUDING the currently-private attached proofs EC composes with; NO proof re-authoring, NO body change, NO relocation. Reuse-not-reimplement: EC composes with the existing proofs, it does not re-prove them."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: [CAT-MIGRATE-EC-FUNCTOR-IMPORT]
github: null
origin: "Steward, 2026-09-03, minted on the Architect's EC structural-closure ruling evt_7zdb106gd707s finding (B). The exact-four LF set in CAT-MIGRATE-EC-FUNCTOR-IMPORT is MEASURABLY INSUFFICIENT; the correct answer is a CLOSURE PREDICATE (EC's import set = every symbol EC's bodies reference MINUS every symbol EC declares locally), enumerated by grep and PROVEN closed by EC checking standalone with exactly that set — never discovered one surface at a time. The closure spans THREE packages, and widening LF/Derived/Transport surfaces is OUTSIDE EC's own scope, so it is this predecessor. EC-Functor-Import re-pointed to depend on it. The Architect's finding (A) (Functor_instance_Identity et al. are EC-LOCAL generated globals, not an LF surface — drop locally-generated *_instance_* from EC's import block) is a LOCAL EC fix, folded into EC-Functor-Import, NOT part of this predecessor."
---

> # EC import-closure provider predecessor. Off the DecEq critical path.
> # Publish the exact free-symbol closure set EC names from LF / Derived /
> # Transport (mark pub + extend loader inventory), including attached proofs
> # EC composes with. NO proof re-authoring, NO body change, NO relocation.
>
> EC (EffectfulClasses) references a closure of provider symbols across three
> packages and cannot import them standalone because they are private. This node
> publishes exactly that closure set on its owning providers so EC's successor
> WP imports it and elaborates standalone. It gates only EC-Functor-Import.

## The closure predicate (Architect finding (B), evt_7zdb106gd707s)

EC's import set = { every symbol EC's bodies reference } MINUS { every symbol EC
declares locally }. The set is enumerated by grep of EC against each provider and
PROVEN closed by EC checking standalone with exactly that set — it is a census,
not a sample, and it is NOT discovered by re-running the checker one surface at a
time (that is the surface-ladder this predicate is designed to stop).

## Closure set — CENSUS COMPLETE (foundation-leader evt_29adqgskp9zs7)

The finding-(B) closure census is done; source is byte-clean. Publish EXACTLY
this set (D0 confirms which members are already `pub` and drops those from D1
with a note). EC-LOCAL dictionaries are explicitly EXCLUDED from publication and
from EC's import inventory (finding (A) — they resolve locally); the
`*_instance_List`/`*_instance_Option` below are LF-OWNED instances EC imports,
which is a different thing from EC's own `*_instance_Identity`.

- **LF — `catalog/packages/Core/Classes/LawfulFunctors.ken.md`.** Direct EC
  external set: `Functor`, `Foldable`, `Functor_instance_List`,
  `Functor_instance_Option`, `Foldable_instance_List`, `Foldable_instance_Option`,
  `comp`, `idf`, `list_map`, `list_map::id`, `list_map::fusion`. PLUS the
  provider-internal SIGNATURE closure these pull into visibility:
  `Monoid`, `fold_map_step`, `monoid_mempty`. (The signature closure is
  mandatory — a published symbol whose signature names a private symbol is not
  loader-resolvable; publish the whole signature-closed set.)
- **Derived — `catalog/packages/Data/Collections/Derived.ken.md`:**
  `concat_map` (EC already imports this today — D0 confirms it is already `pub`
  and, if so, it is unchanged, not a new publication), `list_append`,
  `list_append::assoc`, `list_append::right_unit`.
- **Transport — `catalog/packages/Core/Logic/Transport`:** `cong`, `sym`,
  `trans`.

D0 reconciles this set against the tree at the build SHA (confirm each member's
owner + current visibility). If a member is already `pub`, it is unchanged and
dropped from D1 with a note; the publication set is exactly the currently-private
members of the above.

## Deliverables

- **D0 — closure census reconciliation at the build SHA.** Take the
  foundation-leader's finding-(B) closure census (EC free cross-package symbols
  MINUS EC-local declarations) and finalize the exact publish set per provider.
  Confirm each candidate above resolves to exactly one owning provider and is
  currently private (or already public — then it needs no change and is dropped
  from the deliverable with a note).
- **D1 — publish the closure set per provider.** Mark each census symbol `pub`
  in its owning module (LF / Derived / Transport); extend each provider's
  loader-visible inventory-equality control by EXACTLY those names. The attached
  proofs (`list_map::id`, `list_map::fusion`, `list_append::right_unit`,
  `list_append::assoc`) are published IN PLACE — a visibility flip, no proof
  body touched.

## Acceptance criteria, each with its control

- **AC-EXPORTED (positive, per symbol).** Each published symbol is
  LOADER-VISIBLE from its owning provider — a selective import resolves it to
  that provider's `GlobalId`, measured by the loader, not a `^pub` grep. Control:
  the probe resolves; a still-private sibling name in the same provider still
  rejects `UnboundName`.
- **AC-EXACT-INVENTORY (per provider module).** Each provider's loader-visible
  inventory equality extends by EXACTLY the published names (nothing else changes
  visibility); population from the module's own definitions, verdict from the
  loader, a per-symbol reddening mutation each reds distinctly.
- **AC-VISIBILITY-ONLY.** Every `pub` added changes visibility only: a
  differential shows each body (surface fn AND attached proof) BYTE-UNCHANGED;
  publishing mints no new class/instance and re-proves nothing; every existing
  consumer still resolves to the single existing owner. ZERO trusted-base delta —
  no new axiom/postulate/Opaque; the attached proofs pre-exist and are merely
  made visible.
- **AC-CLOSURE-SUFFICIENT (the downstream confirmation).** The published set is
  SUFFICIENT for EC: verified when EC-Functor-Import imports exactly this set and
  checks standalone-clean (the Architect's acceptance — "the provider
  publications are the predecessor's deliverable, verified by EC's standalone
  check passing"). This node lands on its own per-symbol export ACs; EC's
  standalone-green is the joint closure proof and is the successor's AC, not a
  gate on this node's own merge.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading LF, Derived, Transport, or a module whose closure this changes), scoped
  by changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green
  in CI is the workspace verdict).

## Contention check

Production touch: `Core/Classes/LawfulFunctors.ken.md`,
`Data/Collections/Derived.ken.md`, `Core/Logic/Transport` (pub markers +
inventory per module), plus each module's loader-inventory fixture. All
`catalog/` (lane 3, foundation). NO relocation, NO proof authoring, NO EC change
(EC's own (A) import-scope fix + closure import land in
[[CAT-MIGRATE-EC-FUNCTOR-IMPORT]]). Off the DecEq critical path — disjoint from
the Tier A..E relocation chain and from [[CAT-MIGRATE-TIER-C-DATA-VALUE]]; runs in
parallel with zero contention on those files.

## Capability tier: T2

Mechanical export publication (mark pub, extend inventory) across three
already-standalone-clean providers — no relocation, no proof authoring, no body
change. The one judgment is D0 census completeness (is the closure set exactly
the free-symbol census, and is every member LF/Derived/Transport-owned?), which
the foundation-leader's grep supplies and D0 reconciles. The Architect is the
required reviewer on surface correctness (exactly the closure set published,
nothing over-published; visibility-only; zero trusted-base delta).

## Gate, reviewer, sequencing

`gate: none` (no TCB touch — publishing pre-existing proofs/surfaces is
visibility-only). On the candidate: **Architect** (required — closure-set
correctness + visibility-only + zero trusted-base delta) + **Foundation QA + CV**
on the exact SHA, then Steward M1-M4 -> lieutenant. `blocks:
CAT-MIGRATE-EC-FUNCTOR-IMPORT` — build and land this first, then EC imports the
closure set and resolves its local `*_instance_*` dictionaries per finding (A).
Off the [[CAT-SCAFFOLD-RETIREMENT]] critical path; does NOT gate Tier C or the
DecEq chain.
