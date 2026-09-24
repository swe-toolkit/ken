---
id: LANG-IMPORT-LOAD-ORDER-INDEPENDENCE
title: "A unit's import N must resolve the same way whatever its callers loaded first: a same-unit inline module declared later is UnboundName at the import, an earlier one resolves only through this unit's ordered declaration edge, and a file import follows N.ken even when an unrelated inline N exists"
status: ready
owner: language
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-24 ~04:25Z: 'concur with rec on load order bug. fix it.' Defects: Adversary F2 on f597a0cf8 (evt_7twk6f8zns6zw) and the LOW F1/F2 seam residual from the 0e2fc7bd0 hunt. Frame basis: Architect evt_57xafyyc22b2f. Steward-filed per COORDINATION section 2."
---

# Import resolution depends on load order

## Objective

A unit's meaning no longer depends on which files its callers loaded first
(spec `33 §3.3`: no unit may borrow its caller's imports; `§3.2`: a
file-backed `import N` has the `N.ken` identity).

## Fixed inputs -- Architect `evt_57xafyyc22b2f` at `1a4495376`

In `crates/ken-elaborator/src/modules.rs`:

- `apply_import` still ends in `.unwrap_or_else(|| module.to_string())`, so
  an unavailable inline name falls through to ambient `exports["N"]`.
- `load_unit`'s pre-scan (`declared_inline_import`) still skips a sibling
  declared later.
- `lexical_inline_import` reads the global `inline_children` plus the unit's
  all-declarations set, not a per-unit ordered provenance set. The F1 repair
  bounded the walk at `file_root` but did not change these two.

Nothing in the catalog relies on either behavior. The Architect has not
rerun the reproducers on this SHA.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

One resolver rule, applied the same way in `load_unit`'s pre-scan,
`expand_scope`'s textual import, `apply_import`/`lexical_inline_import`, and
prebinding/instance-synthesis replay. Classify each `import N` as:

1. **same-unit child declared and expanded before this import:** resolves
   only through this unit's ordered declaration edge and that child's public
   exports;
2. **same-unit child declared later:** hard `UnboundName` at the import,
   never ambient `exports["N"]`, whatever a caller or an earlier
   `elaborate_file` loaded;
3. **file-backed absolute import:** the catalog-root `N.ken` / `.ken.md`
   identity, even when an unrelated global export named `N` exists.

Keep the lexical walk bounded by the file root and dotted-import absolute
semantics.

## Acceptance

- **AC-1 (fresh base reproduction first).** On exact main, reproduce both
  cases red before changing code:
  - **Case A (CLI-reachable).** `N.ken` exports `x = Suc (Suc Zero)`.
    `A.ken` has `module P { import N; pub const premature : Nat = N.x }`
    before `module N { pub const x : Nat = Zero }`. `C1` imports file `N`
    then `A`; `C2` imports `A` then `N`. Main admits `C1` with file `N.x`,
    rejects `A` alone and `C2`.
  - **Case B (embedding API).** An unrelated inline `A.N` (`decoy`) is
    elaborated first; then `A.ken`'s `module P { import N; … N.decoy }`
    precedes its own later `module N`.
- **AC-2 (candidate).** `A`, `C1` and `C2` all reject `UnboundName N` at
  `A`'s premature import, in fresh environments, in any caller order. Case B
  rejects at the same import cold and preloaded. Positives: a `module N`
  declared before its sibling `import N` selects that inline `A.N.x` even
  when `N.ken` exists; `Data/Foo.ken`'s bare `import N` binds file `N.x` cold
  and with an unrelated inline `Data.N` preloaded. Existing F1
  qualified/aliased/selective/private/grandchild and same-identity-clash
  pins stay green.
- **AC-3 (mutation controls).** Treat pre-scan membership alone as
  availability: Case B reddens. Route the unavailable-inline branch to
  `exports["N"]`: `C1` reddens while cold `C2` still rejects.
- **AC-4.** No new trust. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

Stop if Case A or B is not red on exact main; either candidate accepts a
premature sibling under any preload; a file import captures an inline
module; a valid earlier same-unit child breaks; the diagnostic comes from a
later unrelated error; or the fix deletes an existing F1 privacy or
qualified-access pin.
