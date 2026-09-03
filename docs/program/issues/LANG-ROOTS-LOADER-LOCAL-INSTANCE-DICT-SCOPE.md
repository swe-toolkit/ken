---
id: LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE
title: "Roots-loader completeness: bind a locally-synthesized instance dictionary (C_instance_T, the derived global an InstanceDecl/DeriveDecl produces) into the per-unit local scope under strict/roots resolution, mirroring the DataDecl -> constructors treatment. Under roots, a bare later reference to a module's own generated dictionary is currently UnresolvedCon because the prebind arm for InstanceDecl/DeriveDecl is empty; the whole-catalog build masked it via the flat cx.globals fallback, and standalone/roots elaboration (scaffold retirement) exposes it. Completeness, not soundness; zero trusted-base delta (surface name resolution, not kernel)."
status: ready
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: [CAT-MIGRATE-EC-FUNCTOR-IMPORT]
github: null
origin: "Steward, 2026-09-03, minted on the Architect's classification evt_5hnv374ev2a80 of EC's renewed hard-stop (foundation-leader routed it; the barrier is reachable from Ken source via standalone/roots elaboration). Classified a DISTINCT reachable ELABORATOR predecessor — not a catalog fix, not kernel. GENERAL: every module that declares a local instance and references its synthesized dictionary later in the same module hits it under roots, so this unblocks EC AND the instance-declaring Tier C/D/E modules (Validation Functor_instance_Validation, NonEmpty Semigroup_instance_NonEmpty, ...), not EC alone. Language lane (crates/ken-elaborator/src/modules.rs = the module/import roots loader). Architect required soundness/design reviewer. Cross-lane note for the operator: a language-lane fix is now a partial gate on the foundation critical path (Tier C's instance-declaring increments); surfaced, not acted on against the roster — default sequencing queues it behind the lane-2 FO priority."
---

> # Roots-loader completeness: a locally-synthesized instance dictionary is a
> # first-class local declaration under roots resolution. Bind C_instance_T into
> # the per-unit local scope, to the EXACT qualified canonical the synthesis
> # emits. Completeness, not soundness; no kernel, no instance body, no import
> # surface change.

## Mechanism (Architect evt_5hnv374ev2a80, grounded on modules.rs at c6484045d)

- The roots-loader per-unit local scope is populated by
  `prebind_scope_declarations` (`crates/ken-elaborator/src/modules.rs`
  ~1876-1930): it `bind_local`s each decl's top-level name and, additionally,
  DataDecl/ExplicitDataDecl CONSTRUCTORS (~1912-1927) — a parent decl binding its
  DERIVED names.
- But `decl_namespace_effect` (~1816) classifies `Decl::InstanceDecl { .. } |
  Decl::DeriveDecl { .. }` as `ReferenceOnly`, and the prebind match arm for them
  is the empty `_ => {}`. So the SYNTHESIZED dictionary `C_instance_T` — the
  derived global an instance PRODUCES — is never `bind_local`'d into the per-unit
  scope.
- Consequence: the synthesized dict IS registered in `cx.globals` (the
  legacy/flat path resolves a bare name via the `Ok(name)` fallback in
  `resolve_ref` ~366, so legacy-flat is GREEN), but under STRICT/roots resolution
  a bare `C_instance_Local` is not in `scope.bindings`, so it never resolves to
  the qualified canonical the global is under -> `UnresolvedCon`. The class
  reference the instance carries is correctly `ReferenceOnly` (resolved via
  `resolve_class_ref` ~1397); it is the PRODUCED dictionary that is wrongly
  unbound.

## Why it surfaces now

The umbrella expected these generated dicts (EC `Functor_instance_Identity`,
Validation `Functor_instance_Validation`, NonEmpty `Semigroup_instance_NonEmpty`)
to "resolve ambiently in the whole-catalog build" — that ambient resolution is
exactly the flat `cx.globals` fallback. Standalone/roots elaboration has no
ambient fallback, so the scaffold-retirement migration to standalone EXPOSES a
pre-existing roots-loader gap the whole-catalog build masked. Not a regression on
`main`; a completeness gap the migration reaches.

## Fix direction (completeness, not soundness)

Give `InstanceDecl`/`DeriveDecl` a namespace-effect that BINDS the synthesized
dictionary name into the per-unit local scope — mirror the DataDecl -> constructors
treatment (a parent decl binding a derived name), binding `C_instance_T` to the
EXACT qualified canonical the synthesis emits. Extend
`reject_decl_prelude_bindings` to cover the new binding so it is
prelude-collision-checked like every other local. Leave the instance's class
reference `ReferenceOnly` (that part is correct).

This is COMPLETENESS: legacy-flat green proves the term is valid and the dict
correctly generated; the roots loader merely drops one class of local binding.
Zero trusted-base delta — surface name resolution, not kernel; no instance body,
no import surface change.

## Soundness guards (Architect §7b — the fix must NOT become a blanket pass)

1. Bind to the SAME canonical the synthesis produces — no shadow global.
2. The naming must match the synthesis convention EXACTLY.
3. NON-DEGENERATE acceptance pair (the AC): the minimal repro turns GREEN AND the
   byte-equivalent legacy-flat stays green, WHILE a control with a genuinely
   colliding/duplicate dictionary name still REJECTS — so the new binding
   DISCRIMINATES, it does not blanket-admit.

## Deliverables

- **D1 — bind the synthesized dictionary under roots.** Change
  `decl_namespace_effect` / `prebind_scope_declarations` so InstanceDecl/DeriveDecl
  bind their synthesized `C_instance_T` into the per-unit local scope at the exact
  synthesis canonical; extend `reject_decl_prelude_bindings` to include it. No
  change to the class-reference path (stays ReferenceOnly), no instance-body
  change, no import-surface change.

## Acceptance criteria, each with its control

- **AC-ROOTS-RESOLVES (positive).** The minimal repro — imported class `C`, local
  head `Local`, local `instance C Local`, a later local bare reference to
  `C_instance_Local` — elaborates GREEN under strict/roots resolution (previously
  `UnresolvedCon`). Control: reverting the new binding restores the exact
  `UnresolvedCon`.
- **AC-LEGACY-UNCHANGED.** The byte-equivalent legacy-flat elaboration of the same
  repro stays green (the fix adds a roots binding, it does not alter the flat
  path).
- **AC-DISCRIMINATES (non-degenerate, §7b guard 3).** A control with a genuinely
  colliding/duplicate synthesized dictionary name still REJECTS — the new binding
  is not a blanket admit. This AC is the whole reason the tier is T1: prove the
  binding discriminates a real collision from a valid local dict.
- **AC-CANONICAL-EXACT.** A differential shows the bound name is the EXACT
  qualified canonical the synthesis emits (no shadow global, no second
  registration); the dict resolved under roots is the same `cx.globals` entry the
  flat path resolves.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure for the
  elaborator roots-loader path (module/import + strict-resolution suites), scoped
  by changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green
  in CI is the workspace verdict).

## §1b watch (Architect)

Entry 2 of an emerging predicate: the roots loader does not model a
locally-synthesized instance dictionary as a first-class local declaration under
roots. The Architect's EC finding (A) (exclude local dicts from the IMPORT set)
and this node (BIND local dicts into scope) are the TWO FACES of that one gap;
(A) remains correct authoring regardless, and this is the roots-loader-side
structural closure. If a 3rd same-family entry appears, close it structurally as
"synthesized instance dictionaries are first-class local decls under roots," NOT
a 3rd point-fix.

## Capability tier: T1

The fix site and direction are fully specified, but the deliverable is the
soundness-guard judgment the parent stop turned on — the binding must discriminate
a real duplicate/collision from a valid local dictionary (AC-DISCRIMINATES),
bind to the exact synthesis canonical (AC-CANONICAL-EXACT), and not degrade to a
blanket admit. That discrimination is reasoning-dense name-resolution-under-roots
work, not a mechanical arm addition.

## Gate, reviewer, sequencing

`gate: none` (zero trusted-base delta — surface name resolution, not kernel). On
the candidate: **Architect** (required — soundness/design; the §7b
non-blanket-pass guard) + **CV** (if conformance applies to strict resolution) +
**Language QA** on the exact SHA, then Steward M1-M4 -> lieutenant. Language lane;
QUEUED behind the lane-2 FO priority (V3-FO-EMBEDDING-ADEQUACY coherent-frame
closure) — it does NOT preempt it. `blocks:
[[CAT-MIGRATE-EC-FUNCTOR-IMPORT]]` and, generally, the instance-declaring Tier C/D/E
modules ([[CAT-MIGRATE-TIER-C-DATA-VALUE]] Validation/NonEmpty and the like); those
migrate standalone-green only once this lands.
