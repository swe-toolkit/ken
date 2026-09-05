---
id: LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE
title: "Roots-loader / module-surface completeness for synthesized instance dictionaries: a synthesized dictionary C_instance_T (the WIRE-mechanism real global an InstanceDecl/DeriveDecl produces) is a first-class NAMED declaration for ALL of roots/strict name resolution — (i) bind_local'd in its own module's per-unit scope, (ii) on its owning module's export surface by its canonical name, and (iii) selectively importable by that name. ONE predicate, three faces (§1b closure); NOT three point-fixes and NOT new pub-instance syntax. Completeness, not soundness; zero trusted-base delta (surface name resolution, not kernel; no instance bodies, no proof re-authoring). NOTE: the id says LOCAL but the ruled scope is the full three-face closure — see the widened-scope banner."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [CAT-MIGRATE-EC-FUNCTOR-IMPORT]
github: null
origin: "Steward, 2026-09-03. Minted on the Architect's classification evt_5hnv374ev2a80 (EC's renewed hard-stop = a reachable elaborator predecessor) and WIDENED by the Architect's D0 ruling evt_y9cn3eqdxakn to the FULL closure. The Architect named ONE predicate — the roots loader / module system does not model a synthesized instance dictionary (the WIRE-mechanism real global) as a first-class NAMED declaration under roots/strict — with THREE faces (§1b: at the 3rd entry, stop ruling entries, name the closure). GENERAL: unblocks EC and every instance-declaring / instance-consuming Tier C/D/E module, not EC alone. Language lane (crates/ken-elaborator/src/modules.rs = the module/import roots loader). Architect required soundness/design reviewer. Cross-lane note for the operator: a language-lane fix is now a partial gate on the foundation critical path (Tier C instance modules); surfaced, not acted on against the roster — default sequencing queues it behind the lane-2 FO priority (released as interim work while FO is held)."
---

> # ACTIVE — RELEASED to the language ring (Steward, 2026-09-05). Operator
> # direction "do not starve the other lanes; keep the fleet moving" — brought up
> # NOW ahead of the verify seam (the lane-2 verify node CI-NATIVE-PARITY-DURATION
> # is Architect-blocked and CI is "acceptable atm"), since this is the idle
> # language ring's ready deliverable and the L3 unblock. On landing, foundation's
> # CAT-MIGRATE-EC-FUNCTOR-IMPORT (and every instance-declaring/consuming Tier
> # C/D/E module) becomes releasable. base = current main.
>
> # SCOPE WIDENED (Architect D0 ruling evt_y9cn3eqdxakn). ONE predicate, THREE
> # faces — a synthesized instance dictionary C_instance_T is a first-class NAMED
> # declaration for roots/strict name resolution:
> #  Face 2 — bind_local'd in its OWN module's per-unit scope (same-module later
> #    ref resolves).
> #  Face 3a — on its OWNING module's export surface by its canonical name.
> #  Face 3b — selectively IMPORTABLE by that canonical name into a consumer.
> # Export/import key off the WIRE convention (a synthesized dict of a PUBLIC
> # class/instance is exported by canonical name), NOT new pub-instance syntax —
> # the dict is auto-synthesized, so export/import completeness is the
> # roots-loader's job, the same completeness class as Face 2. Face 1 (finding (A):
> # EC-local *_instance_* excluded from EC's own import block, resolved by
> # declaration order) is correct AUTHORING and stays in the consumer WP; it is
> # named here only to place this node as the roots-loader-side closure of the
> # same predicate. Completeness, zero trusted-base delta (surface name
> # resolution, not kernel; no instance bodies, no proof re-authoring).

## Why one predicate, three faces (Architect evt_5hnv374ev2a80 + evt_y9cn3eqdxakn)

The WIRE mechanism (every `instance C T { .. }` registers a real global
`C_instance_T`) makes the synthesized dictionary a genuine value. Under the
whole-catalog build it resolved ambiently via the flat `cx.globals` `Ok(name)`
fallback (`resolve_ref` ~366), so all three faces were masked. Standalone/roots
elaboration has no ambient fallback, and the roots loader does not model the
synthesized dict as a first-class named declaration — surfacing the SAME gap in
three places:

- **Face 2 — same-module scope.** `prebind_scope_declarations`
  (`crates/ken-elaborator/src/modules.rs` ~1876-1930) `bind_local`s each decl's
  top-level name and DataDecl/ExplicitDataDecl CONSTRUCTORS (~1912-1927), but
  `decl_namespace_effect` (~1816) classifies `InstanceDecl`/`DeriveDecl` as
  `ReferenceOnly` with an empty prebind arm, so `C_instance_T` is never
  `bind_local`'d. A later bare same-module ref is `UnresolvedCon` under roots.
- **Face 3 — cross-module export/import.** A synthesized dict cannot be exported
  by name from its owner nor selectively imported into a consumer — no
  visibility-only edit puts a synthesized name on the export surface, and
  `pub instance` is parser-ineligible (`parser.rs:2788`). So a consumer's bare
  cross-module ref to an owner's synthesized dict fails under roots. (Measured on
  EC: it references `Functor_instance_List/Option`, `Foldable_instance_List/Option`
  BY BARE NAME as first-class values — explicit dictionary arg
  `functor_map_of Option Functor_instance_Option ...` (EC :354/:362/:370, List
  :588) and superclass field wiring `functor = Functor_instance_Option`
  (EC :378/:1036/:1301-1302/:1325-1326); LF owns all four, EC declares none.
  Class_env registration services class-DIRECTED lookup only — it does not make a
  bare NAME resolve, so registry availability cannot discharge these sites.)

Legacy-flat green proves the dicts are valid and correctly generated; roots merely
drops their name resolution across all three faces. Completeness, not soundness.

## Fix direction (completeness; key off the WIRE convention, not new syntax)

Model `C_instance_T` as a first-class named declaration for roots/strict name
resolution, mirroring the DataDecl -> constructors treatment (a parent decl
binding derived names) and the module export/import machinery:

- **D1 (Face 2) — bind_local.** Give `InstanceDecl`/`DeriveDecl` a namespace
  effect that binds `C_instance_T` into the per-unit local scope at the EXACT
  synthesis canonical; extend `reject_decl_prelude_bindings` to cover it. Leave
  the instance's class reference `ReferenceOnly` (that part is correct).
- **D2 (Face 3a) — export.** Put a synthesized dict of a PUBLIC class/instance on
  its owning module's export surface by its canonical name, off the WIRE
  convention (auto-synthesized => auto-exported when its class/instance is
  public), with NO new `pub instance` syntax.
- **D3 (Face 3b) — selective import.** Make that canonical name selectively
  importable into a consumer module under roots, resolving to the owner's
  `cx.globals` entry.

All three bind/export/import to the SAME canonical the synthesis produces — no
shadow global, no second registration. Zero trusted-base delta.

## Acceptance criteria, each with its control

- **AC-SCOPE-RESOLVES (Face 2).** The minimal repro — imported class `C`, local
  head `Local`, local `instance C Local`, a later local bare ref to
  `C_instance_Local` — elaborates GREEN under strict/roots (previously
  `UnresolvedCon`). Control: reverting the binding restores the exact
  `UnresolvedCon`.
- **AC-EXPORT-IMPORT-RESOLVES (Face 3), non-degenerate — EC's two real shapes.**
  A consumer's bare cross-module ref to an owner's synthesized dict resolves under
  roots in BOTH measured shapes: an explicit dictionary argument
  (`functor_map_of Option Functor_instance_Option ...`) AND superclass field
  wiring (`functor = Functor_instance_Option`). Control: without the import, each
  is `UnresolvedCon`.
- **AC-LEGACY-UNCHANGED.** The byte-equivalent legacy-flat elaboration of the same
  repros stays green (the fix adds roots name resolution, it does not alter the
  flat path).
- **AC-DISCRIMINATES (§7b guard — the reason this is T1).** A control importing a
  genuinely-UNDECLARED dictionary name still REJECTS, and a control with a
  genuinely-colliding/duplicate synthesized dict name still REJECTS. The
  bind/export/import DISCRIMINATE a real declaration from an undeclared or
  colliding one — no blanket admit.
- **AC-CANONICAL-EXACT.** A differential shows every bound/exported/imported name
  is the EXACT qualified canonical the synthesis emits (no shadow global, no
  second registration); the dict resolved under roots is the same `cx.globals`
  entry the flat path resolves.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure for the
  elaborator roots-loader path (module/import + strict-resolution suites), scoped
  by changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace`.

## §1b — this IS the named closure (Architect, at the 3rd entry)

The predicate: the roots loader / module system does not model a synthesized
instance dictionary (the WIRE-mechanism real global) as a first-class NAMED
declaration under roots/strict. Three faces: (1) finding (A) exclude EC-local
dicts from imports [correct authoring, stays in the consumer WP]; (2) bind_local
into own scope; (3) export by name + selective import. Per §1b, at the 3rd entry
the Architect stops ruling entries and names the closure — this node IS that
closure (faces 2 + 3; face 1 is authoring). Do NOT re-open it as further
point-fixes.

## Capability tier: T1

The fix site and direction are specified, but the deliverable is the
soundness-guard judgment the stops turned on — bind/export/import must discriminate
a real synthesized declaration from an undeclared or colliding name
(AC-DISCRIMINATES), at the exact synthesis canonical (AC-CANONICAL-EXACT), without
degrading to a blanket admit. Reasoning-dense name-resolution-under-roots work
across the module surface, not a mechanical arm addition.

## Gate, reviewer, sequencing

`gate: none` (zero trusted-base delta — surface name resolution, not kernel). On
the candidate: **Architect** (required — soundness/design; the non-blanket-pass
guard) + **CV** (if conformance applies to strict resolution) + **Language QA** on
the exact SHA, then Steward M1-M4 -> lieutenant. Language lane; released as INTERIM
work while the lane-2 FO priority (V3-FO-EMBEDDING-ADEQUACY coherent-frame closure)
is held on its §1a advisory — no file contention (modules.rs vs the dependent-elim
files). FO resumes as priority when its ruling lands. `blocks:
[[CAT-MIGRATE-EC-FUNCTOR-IMPORT]]` and, generally, the instance-declaring /
instance-consuming Tier C/D/E modules ([[CAT-MIGRATE-TIER-C-DATA-VALUE]]
Validation/NonEmpty and the like). PAYOFF (Architect): because Functor + Foldable
are published in [[CAT-MIGRATE-EC-CLOSURE-PROVIDERS]]'s 10, once THIS node lands
its export/import completeness the four LF dicts (Functor_instance_List/Option,
Foldable_instance_List/Option) become importable with NO further foundation edit —
auto-synthesized, auto-exported off the now-public classes.
