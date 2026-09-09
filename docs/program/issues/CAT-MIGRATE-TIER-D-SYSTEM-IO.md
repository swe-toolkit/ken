---
id: CAT-MIGRATE-TIER-D-SYSTEM-IO
title: "Tier-D scaffold-retirement leaf: rename the Capability.System.IO theorem `write_all_all_success` to a non-shadowing canonical identifier so it stops colliding with its own subject function (the native-prelude `write_all_all_success`), retaining the subject function and preserving proof body + trust. The Tier-D singleton tail, after Posix and before Tier E."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-D-POSIX]
blocks: []
github: null
origin: "Operator scaffold-retirement campaign (2026-09-02, [[CAT-SCAFFOLD-RETIREMENT]]); foundation-leader Tier-D decomposition (evt_2e0pee5jxzv07) + this leaf named at evt_2t9vr32nj4age (thr_61xfbthx1xkx8): the sole remaining Tier-D slice, an independent one-module theorem-name erratum sequenced in the established Tier-D singleton tail after CAT-MIGRATE-TIER-D-POSIX and before Tier E. Steward-framed at main 55c211ce3."
---

> # RELEASED 2026-09-09 (Steward) — Tier-D final leaf of the scaffold-retirement campaign.
>
> Released to the foundation ring as lane 3's next deliverable after Posix
> landed (55c211ce3). Part of the [[CAT-SCAFFOLD-RETIREMENT]] 5-tier DAG; this is
> the Tier-D (Capability) singleton tail. Independent — no dependency on the
> Tier-D internal Cursor->Decoder->Parsing chain beyond lane sequencing. On the
> candidate: fresh Foundation QA + CV on the exact SHA (Architect required only
> if D0 surfaces a live theorem importer or a canonical-name ambiguity — see the
> D0 stop rule), then Steward M1-M4.

## The constraint (grounded, not aesthetic — §4c)

The scaffold-retirement end state is that catalog modules stand on the real
prelude + module/import, not fixture scaffolding. Under that surface a catalog
declaration must not share an identifier with a prelude function it depends on:
`Capability.System.IO` declares `theorem write_all_all_success`, and its subject
function `write_all_all_success` is a native-prelude function of the identical
name. A theorem shadowing its own subject function is a name collision on the
production surface the campaign migrates every module onto — a real hygiene
defect in the module's export/reference space, not a style preference. The
source is the operator's scaffold-retirement mission via [[CAT-SCAFFOLD-RETIREMENT]].

## The defect (measured at 55c211ce3)

`catalog/packages/Capability/System/IO.ken.md:37-38`:

```
theorem write_all_all_success (fuel : Nat) : Equal Bool (write_all_all_success fuel) True =
  proof all_success for write_all_all_success fuel
```

The theorem identifier `write_all_all_success` equals the subject function it is
a theorem about. The five sibling theorems in the same block use descriptive
predicate names that do NOT shadow their subjects (`:18` `write_all_terminates`,
`:21` `write_all_preserves_exact_prefix`, `:26` `write_all_success_is_complete`,
`:29` `write_all_preserves_first_error`) — only this one collides.

## D0 reference census (Steward, at 55c211ce3) — carry, verify, do not re-derive blind

Full-tree references to `write_all_all_success`:
- `catalog/packages/Capability/System/IO.ken.md:37,38` — the theorem declaration
  + its proof (the site to change).
- `crates/ken-elaborator/src/prelude.rs:2686,2688,2690` — the native-prelude
  SUBJECT FUNCTION definition and its own separate proof term (`fn
  write_all_all_success ...` + `proof all_success for write_all_all_success ...`).
  This is the shadowed function; it is RETAINED unchanged.
- `crates/ken-elaborator/tests/px8f_buffer_io_surface.rs:173` — the string
  `"write_all_all_success"` in a test surface inventory (acceptance evidence).

There is ZERO external catalog importer of `Capability.System.IO` (no other
catalog module imports it by path or name). So no by-name theorem consumer exists
to break. Confirm this census at the candidate base before renaming.

## Deliverable

Rename ONLY the theorem at IO.ken.md:37-38 to a non-shadowing canonical theorem
identifier. Retain the `write_all_all_success` subject function (prelude,
unchanged). Update every reference to the theorem by its new name: the proof
head, any loader-visible inventory, and the acceptance evidence (the
px8f_buffer_io_surface.rs surface inventory string if it names the theorem rather
than the function — D0 distinguishes which). Preserve the proof body and trust
(no new axiom/postulate/primitive; the statement is unchanged).

CANONICAL NAME — front-loaded, grounded against the landed sibling precedent, D0
finalizes: follow the sibling descriptive-predicate convention
(`write_all_success_is_complete` is the nearest precedent — a subject function
plus a semantic predicate, statement `Equal Bool (...) True`). RECOMMENDED:
`write_all_all_success_holds` (the all-success predicate holds for all fuel). D0
MUST confirm the chosen identifier collides with no existing prelude/module/catalog
identifier and adopt it, or a sibling-consistent alternative if a collision or
ambiguity is found. Do not invent a spelling that departs from the sibling
convention.

## D0 + stop rule

D0 measures: (a) the canonical replacement spelling (confirm no collision), and
(b) the client surface (confirm the census above — no live theorem importer). If
D0 finds a live theorem importer OR a canonical-name ambiguity, HARD STOP to the
Architect (foundation-leader evt_2t9vr32nj4age) — that is the only condition that
pulls an Architect decomposition refresh; otherwise this is a mechanically
completable erratum.

## Acceptance

- `Capability.System.IO` elaborates (`ken check`) and formats (`ken fmt --check`)
  green with the theorem referenced by its new non-shadowing name; the subject
  function `write_all_all_success` is retained and unchanged.
- The theorem STATEMENT is unchanged — still `Equal Bool (write_all_all_success
  fuel) True` about the subject function — and its proof still checks. No new
  axiom/postulate/primitive; `trusted_base()` unaffected.
- The theorem identifier no longer equals any prelude/module/catalog function
  identifier (the collision is gone). No other symbol renamed.
- Loader-visible inventory / acceptance evidence updated to the new theorem name;
  the IO acceptance test(s) green.
- Cross-cutting: no other catalog module or test perturbed (nothing imports
  Capability.System.IO by name; confirm the whole-catalog evidence-frontier
  census stays green — the affected-closure requirement).

## Sequencing

The Tier-D (Capability) singleton tail: after Posix (merged 55c211ce3), before
Tier E begins (Serialization.Json -> Application.Input.Schema -> sibling
consumers ArgParse / Configuration.Decoder; independent Tier-E algorithm leaves
InsertionSort / OrderedSearch / Gcd / Property). Independent leaf — buildable now.
