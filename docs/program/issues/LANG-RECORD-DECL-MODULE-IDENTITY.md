---
id: LANG-RECORD-DECL-MODULE-IDENTITY
title: "A module's record declaration enters globals under its bare spelling and is never scope-bound, so the Legacy roots loader and the direct API admit an unimported use of another module's record. Qualify and scope-bind records exactly as data types are, so an unselected record is refused in every mode and a public record is importable"
status: ready
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-CLASS-IDENTITY-BY-CHECKED-ID]
blocks: []
github: null
origin: "LANG-CLASS-IDENTITY-BY-CHECKED-ID AC-0b row (iv), report-only (language-implementer evt_561f7wsc6nrbj, language-leader evt_205fgpfjk4x1n), carried to the Steward under Architect Rule B (evt_5810nqw1819w8). Operator L2 objective 2026-09-25 (one resolution mode). Steward-filed per COORDINATION section 2."
---

# Records resolve by module identity

## Objective

A record declared in a module resolves exactly as a data type declared
there does, in every loader mode: by its qualified identity, reached only
through the declaring unit's scope or an import. An unselected record is
refused.

## Settled inputs (AC-0b on `0812a8d3d`, evt_561f7wsc6nrbj)

- **The leak.** Scratch module A declares `record Packet { field : Int }`.
  In all modes it enters `globals["Packet"]` bare, with no
  `globals["A.Packet"]`, is not in `class_env`, and is not scope-bound.
  A client `fn record_probe (d : Packet) : Packet = d` with no import is
  accepted by Legacy roots and the direct API, and refused by Strict roots
  (`UnboundName(Packet)`).
- **Import.** `import A (Packet)` fails as `UnboundName(A.Packet)` in all
  modes. The fixture's record was not `pub`, so whether a `pub record` is
  importable today is unmeasured.
- **The control.** A `pub data Tag` in the same A is qualified
  (`globals["A.Tag"]`), refused unselected in all three modes, and
  admitted when imported.
- **The mechanism.** RecordDecl hits neither `is_qualifiable`
  (`modules.rs:3392`) nor the class exemption, so it is never prebound in
  scope (Architect `evt_5810nqw1819w8`). Spec 33 §3 states a module program
  as one fully-qualified namespace, and 33 §1 puts every top-level name,
  records included, in that one namespace.

Re-measure on the landed base after `LANG-CLASS-IDENTITY-BY-CHECKED-ID`.
If a settled input is false, stop and report.

## Deliverable

RecordDecl is qualified and scope-bound like DataDecl, so its record type,
its constructor and its field projections are reached only through scope.
A `pub record` exports and imports like a `pub data`. Consumers that read a
record's bare `globals` key migrate to its qualified key or a checked export
id, keeping each assertion's property.

## Acceptance

- **AC-0 (Architect).** Before building: measure `pub record` export and
  import in all three modes, and census every consumer of record bare
  `globals` keys and field-projection lookups across `crates/*/src`,
  `crates/*/tests`, `r_layer_tests`, `catalog/`, `conformance/` and
  `examples/`. The Architect rules the field-projection keying.
- **AC-1.** Under Strict roots, Legacy roots and the direct API: an
  unselected module record is refused with the data control's error class;
  an imported `pub record` is admitted and its fields project; two modules
  each declaring `record Packet` both elaborate, and each client gets the
  one it imported. Restoring the bare record key turns the unselected row
  red.
- **AC-2.** Catalog, conformance and record suites stay green (Full CI).

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog, conformance or test source that fails only because it uses a
  record it does not select: return the list to the Architect.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

Checks: 4 fired: the bare record key, absent qualified key, unselected
Legacy and direct acceptance, and the qualified refused `Tag` control are
all measured on `0812a8d3d` (evt_561f7wsc6nrbj). 9 fired: loader mode is
a dispatch axis, so every AC-1 row runs under all three modes.
