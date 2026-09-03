---
id: CAT-MIGRATE-EC-FUNCTOR-IMPORT
title: "Scaffold-retirement, orthogonal Core.Classes node (off the DecEq critical path): make EffectfulClasses (EC) standalone-green by publishing the four LawfulFunctors (LF) provider surfaces it consumes ambiently — LF marks class Functor + fn comp + fn idf + fn list_map pub; EC replaces ambient resolution with a real selective import from LF. Publication + one import block + loader-inventory extension; NO relocation, NO proof authoring. Not a regression fix — EC already elaborates in the full-catalog build via ambient class-install; this node removes its dependence on that scaffolding so it elaborates standalone."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-03. Spun out of the Tier-B recut as a census-error correction: the Tier-B D0 (foundation-implementer) found EC fails standalone with UnresolvedCon Functor — EC was wrongly folded in as clean. Architect ruling evt_21c0cdvnmv3f3 CONFIRM 2: (b) LF-Functor + EC clean-ification is ORTHOGONAL to the DecEq chain (Functor is not DecEq; the recut relocation touches LC/BytesKeys/StringKeys/EmptyDec/StringBijection, never EC), so it is its OWN independent Core.Classes node, runnable in parallel whenever, and must NOT sit on the relocation's critical path. Architect CONFIRM 1 blessed the surface: publishing LF class Functor + comp/idf/list_map is the sound and intended consequence of the class-owner model + scaffold-retirement mandate — pub changes VISIBILITY only, class-uniformity preserved by construction (still exactly one class Functor in LF). Coordinates re-measured by Steward at origin/main 21f87f5b5 (below); re-measure at your build SHA (D0)."
---

> # Orthogonal EC standalone-cleanness node. Off the DecEq critical path.
> # Tier-A publication shape (mark pub / add one import block / extend the
> # loader-visible inventory), NO relocation, NO proof authoring.
>
> EffectfulClasses consumes four LawfulFunctors symbols ambiently and fails
> standalone. This node publishes exactly those four LF surfaces and gives EC a
> real import block for them, so EC elaborates standalone. It gates nothing and
> nothing gates it — LF is already standalone-clean on main; run it whenever
> foundation has a seat free of the priority (P + the recut relocation).

## Not a regression fix (read before treating standalone-red as a bug)

EC elaborates today in the FULL catalog build via ambient class-install (the
operator's class-uniformity ruling, 2026-09-02 — type classes install
wholesale across a compilation by design). Nothing is on fire. This node is a
standalone-CLEANNESS quality node: it brings EC to the scaffold-retirement end
state (`zero catalog dependence on fixture scaffolding / ambient resolution`,
[[CAT-SCAFFOLD-RETIREMENT]]) so EC elaborates on its own. "EC fails standalone
at UnresolvedCon Functor" is the STARTING condition this node closes, not a
defect on `main`.

## The provider + consumer surfaces (measured at `21f87f5b5`; re-measure — D0)

**Provider — LF = `catalog/packages/Core/Classes/LawfulFunctors.ken.md`.** All
four are currently PRIVATE (bare, no `pub`):
- `fn idf` (:193)
- `fn comp` (:195)
- `class Functor` (:197)
- `fn list_map` (:210)

LF is itself standalone-clean on `main` (`CAT-LAWFULFUNCTORS-STANDALONE-IMPORT`,
squash 4a088d8aa) — so publishing these four is the ONLY provider change; no LF
proof or body is touched.

**Consumer — EC = `catalog/packages/Core/Classes/EffectfulClasses.ken.md`.** EC
imports only `Data.Collections.Derived (concat_map)` (:53) and resolves all four
LF symbols ambiently: `Functor` (13 occurrences, incl. the `functor` field :65,
`Functor_instance_*` dictionaries), `idf` (27), `comp` (call-sites :1375, :2327,
:2339, :2377, :2389, :3406, :5345, :5472, :5544, …), `list_map` (80). Standalone
elaboration fails `UnresolvedCon Functor`. Add a real selective import from LF
for exactly the four symbols D0 confirms EC uses; retire the ambient reach.

## Deliverables

- **D0 — census at the build SHA.** Re-measure LF's four private decls and EC's
  ambient consumption set. Confirm the EXACT set of LF symbols EC needs (the
  four above are the measured candidates; the loader's `UnresolvedCon` set at
  standalone is the authority). If EC's standalone D0 surfaces a fifth ambient
  provider NOT owned by LF (a symbol from another still-scaffolded module), that
  is a HIDDEN dep — cite it and split it out rather than dragging an unpublished
  provider in (the same DAG axis this node respects). If it is LF-owned but not
  in the four, publish it here and say so.
- **D1(LF) — publish the four.** Mark `class Functor` (:197), `fn comp` (:195),
  `fn idf` (:193), `fn list_map` (:210) `pub`; extend LF's loader-visible
  inventory-equality control to exactly these names.
- **D1(EC) — import block + standalone.** Add a selective import from LF for the
  exact D0-confirmed set; retire the ambient reach; extend EC's loader-visible
  inventory to reflect the imports (imported names are not new EC exports —
  extend the control the way its existing import of `concat_map` is reflected);
  EC elaborates standalone (exit 0).

## Acceptance criteria, each with its control (Tier-A proven shape)

- **AC-EXPORTED (positive, per symbol).** `class Functor`, `fn comp`, `fn idf`,
  `fn list_map` are each LOADER-VISIBLE from LF — a selective import resolves
  each to LF's `GlobalId`, measured by the loader, not a `^pub` grep. Control:
  the probe resolves; a still-private sibling name in LF still rejects
  `UnboundName`.
- **AC-EXACT-INVENTORY (per module).** LF's loader-visible inventory equality
  extends by EXACTLY the four names (nothing else changes visibility);
  population from the module's own definitions, verdict from the loader, a
  per-symbol reddening mutation each reds distinctly. EC's import-visible set
  extends by exactly the D0-confirmed imported names.
- **AC-STANDALONE-GREEN.** EC elaborates standalone (exit 0) after the import
  block — no ambient/scaffolding fallback. The prior standalone failure was
  `UnresolvedCon Functor`; the control is that removing the new import line
  restores that exact failure.
- **AC-VISIBILITY-ONLY (class-uniformity, the Architect's CONFIRM 1).** `pub` on
  `class Functor` — which LF already defines — changes visibility only: a
  differential shows the `class Functor` body BYTE-UNCHANGED; still exactly one
  `class Functor` catalog-wide, publishing mints no second class; every
  `Functor f` and `Functor_instance_*` in EC resolves to LF's single class. No
  computational change.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every
  target loading LF or EC or a module whose closure this changes), scoped by
  changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green in
  CI is the workspace verdict).

## Contention check

Production touch: `Core/Classes/LawfulFunctors.ken.md` (four `pub` markers +
inventory) and `Core/Classes/EffectfulClasses.ken.md` (one import block +
inventory), plus the two modules' loader-inventory fixtures. All `catalog/`
(lane 3, foundation). NO relocation, NO DecEq consumer repoint.

This node and the DecEq chain are disjoint at the file level: the DecEq chain
(P = [[CAT-MIGRATE-TIER-B-PROVIDERS]] and the successor
[[CAT-MIGRATE-TIER-B-CLASSES]]) touches LC / BytesKeys / StringKeys / EmptyDec /
StringBijection — never LF or EC. So this runs in parallel with the DecEq chain
with zero contention. It does NOT gate P or the relocation and must not preempt
them: P is the foundation priority; pick this up when a seat is free of it (or
when the leader parallelizes a second seat), never by pulling the P seat.

## Capability tier: T2

Mechanical export publication + one bounded import-block clean-ification (mark
pub, add imports, extend inventory) — no relocation, no proof authoring. The one
judgment is EC's D0 completeness (are all its ambient deps LF-owned and
published here, or is there a hidden non-LF provider?), which either folds in or
splits out with a cited reason. The Architect is the required reviewer on
surface correctness (exactly the four intended LF surfaces published, nothing
over-published; class-uniformity by construction), which is a gate, not
implementer cognitive load.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator already ruled the class-owner model).
On the candidate: **Architect** (required — surface correctness +
class-uniformity) + **Foundation QA + CV** on the exact SHA, then Steward
M1-M4. Off the [[CAT-SCAFFOLD-RETIREMENT]] critical path — `depends_on: []`,
runnable whenever a foundation seat is free of the DecEq priority.
