---
id: CAT-MIGRATE-TIER-B-PROVIDERS
title: "Scaffold-retirement Tier B, provider-publication predecessor P (Tier-A shape, NO relocation): publish the two provider surfaces the DecEq class-owner relocation actually consumes — (a) LC (LawfulClasses) marks its OWN class DecEq + fn bool_eq pub (already listed public in LC's Public-API prose; markers just missing), confirming instance DecEq Bool loader-visible; and (c) StringBijection (a Tier-A-missed Data.Text provider) migrates off scaffolding onto real Transport imports and publishes string_to_list_char_injective. Publishing only; the relocation is the successor CAT-MIGRATE-TIER-B-CLASSES, gated on this."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-A-PROVIDERS]
blocks: [CAT-MIGRATE-TIER-B-CLASSES]
github: null
origin: "Steward, 2026-09-03. The provider-publication predecessor split out of Tier B after the foundation D0 hard-stop (foundation-implementer evt_5bt85erc05fa4) found the CAT-MIGRATE-TIER-B-CLASSES relocation consuming two UNPUBLISHED provider surfaces — violating the Architect's DAG axis 'no consumer-only WP references an unpublished provider'. Architect ruling evt_21c0cdvnmv3f3 (converging with Steward disposition evt_7bzffq1q90rr8): adopt the RECUT (not rescope-in-place); bundle (a) LC-class-publication + (c) StringBijection-clean-ification+cert-publication as ONE provider-publication predecessor P; the recut relocation is gated on P and is then WHOLE (UInt8/Bytes/String, String no longer deferred). Architect CONFIRM 1 (evt_21c0cdvnmv3f3): publishing the CLASS surfaces is the sound + intended consequence of the operator Flag-2 class-owner ruling + scaffold-retirement mandate — pub changes VISIBILITY only, class-uniformity preserved by construction (still exactly one class DecEq in LC), corroborated by LC's Public-API prose :2281-2285. Coordinates measured at origin/main 4823e40a4 / d983f4c6d; re-measure at your build SHA (D0)."
---

> # Provider-publication predecessor P for the Tier-B DecEq relocation. Tier-A
> # export-publication shape (mark pub / migrate to real imports / extend the
> # loader-visible inventory), NO relocation, NO consumer repoint of DecEq.
>
> The DecEq class-owner relocation (successor CAT-MIGRATE-TIER-B-CLASSES) consumes
> two provider surfaces that are private/scaffolded today. Per the Architect's DAG
> axis, provider publication is its own WP ahead of the consumer relocation. This
> node publishes exactly those two surfaces; the relocation gates on it.

## The two surfaces (measured at `4823e40a4`; re-measure at your build SHA — D0)

**(a) LC = `catalog/packages/Core/Classes/LawfulClasses.ken.md` publishes its own
class.** `class DecEq` (:75) and `fn bool_eq` (:329) are currently PRIVATE (bare,
no `pub`); a roots-loader selective import of each fails `UnboundName`. LC's own
"Public API" prose (:2281-2285) ALREADY lists `class DecEq` + `instance DecEq
Bool` as public — the `pub` markers were simply omitted, which is precisely why
EmptyDec had to duplicate them (the scaffolding this tier retires). `instance
DecEq Bool` (:420) must be loader-visible after.

**(c) StringBijection = `catalog/packages/Data/Text/StringBijection.ken.md`
clean-ification + cert publication.** `theorem string_to_list_char_injective`
(:16) is PRIVATE and the module is itself SCAFFOLDED — raw standalone fails
`UnresolvedCon sym`; adding the real Transport import `(cong, sym, trans)` takes
it to exit 0 (measured in the ring's disposable D0 worktree). This is a
Tier-A-MISSED Data.Text provider (Tier A published Derived, not StringBijection).
Publish the cert `pub` and give the module its real import block so it is
standalone-green.

## Deliverables

- **D0 — census at the build SHA.** Re-measure the two surfaces; confirm LC's
  `class DecEq`/`bool_eq` privacy + the Public-API prose, and StringBijection's
  full ambient-symbol set + that every provider it needs is already published. If
  StringBijection's own D0 surfaces HIDDEN dependencies beyond Transport (a
  provider not yet published), split (c) into its own node and cite it — do not
  drag an unpublished provider in (the same axis this node exists to honor).
- **D1(a) — LC publishes its class.** Mark `class DecEq` (:75) and `fn bool_eq`
  (:329) `pub`; confirm `instance DecEq Bool` (:420) loader-visible; extend LC's
  loader-visible inventory-equality control to the published names.
- **D1(c) — StringBijection standalone + cert pub.** Add the real Transport import
  `(cong, sym, trans)` (retire any scaffolding reach-through); mark
  `string_to_list_char_injective` (:16) `pub`; extend StringBijection's
  loader-visible inventory; the module elaborates standalone (exit 0).

## Acceptance criteria, each with its control (Tier-A proven shape)

- **AC-EXPORTED (positive, per symbol).** `class DecEq`, `fn bool_eq`,
  `string_to_list_char_injective` are each LOADER-VISIBLE from their owner — a
  selective import resolves each to the owner's `GlobalId`, measured by the
  loader, not a `^pub` grep. Control: the probe resolves; a still-private sibling
  name in the same module still rejects `UnboundName`. Confirm `instance DecEq
  Bool` (:420) loader-visible.
- **AC-EXACT-INVENTORY (per module).** LC's loader-visible inventory equality
  extends by exactly `class DecEq` + `fn bool_eq` (and nothing else changes
  visibility); StringBijection's by exactly `string_to_list_char_injective`.
  EQUALITY, population from the module's own definitions, verdict from the loader,
  a per-symbol reddening mutation each reds distinctly.
- **AC-STANDALONE-GREEN.** StringBijection elaborates standalone (exit 0) after
  the real Transport import — no ambient/scaffolding fallback. LC still elaborates
  standalone after the pub markers.
- **AC-VISIBILITY-ONLY (class-uniformity, the Architect's CONFIRM 1).** `pub` on a
  class the owner already defines changes visibility only — a differential shows
  the `class DecEq` / `class`-body and `fn bool_eq` bodies BYTE-UNCHANGED; still
  exactly one `class DecEq` in LC, publishing mints no second class. No
  computational change.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading LC or StringBijection or a module whose closure this changes), scoped by
  changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace`.

## Contention check

Production touch: `Core/Classes/LawfulClasses.ken.md` (two `pub` markers +
inventory) and `Data/Text/StringBijection.ken.md` (import block + one `pub` +
inventory), plus the two modules' loader-inventory fixtures. All `catalog/` (lane
3). No relocation, no DecEq consumer repoint (that is the successor). No other
active lane touches these. The successor CAT-MIGRATE-TIER-B-CLASSES relocates the
DecEq instances into LC and consumes both published surfaces — sequence it AFTER
this lands (`blocks`).

## Capability tier: T2

Mechanical export publication + a bounded module clean-ification (add imports,
mark pub, extend inventory) — no relocation, no proof authoring. The one judgment
is StringBijection's D0 completeness (are all its ambient deps published?), which
either folds in or splits (c) out with a cited reason. The Architect is required
reviewer on the surface correctness (exactly the intended surfaces published,
nothing over-published; class-uniformity by construction), which is a gate, not
implementer cognitive load.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator already ruled the class-owner model). On
the candidate: **Architect** (required — surface correctness + class-uniformity)
+ **Foundation QA + CV** on the exact SHA, then Steward M1-M4. This is the
provider-publication half of Tier B ([[CAT-SCAFFOLD-RETIREMENT]]); its successor
[[CAT-MIGRATE-TIER-B-CLASSES]] (the whole DecEq relocation + EmptyDec
consolidation) gates on it. The orthogonal EC/LF-Functor clean-ification is a
SEPARATE Core.Classes node off this critical path.
