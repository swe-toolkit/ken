---
id: CONF-GEOMETRY-BOX-SEED-ISTRUE-IMPORT
title: "The box monotonicity seed's accepted client states IsTrue but imports only Algorithm.Geometry.NatBox, so ken check refuses it with UnboundName under Strict. It was accepted only through the Legacy passthrough, and no Rust test loads the seed, so CI never saw the refusal. Import IsTrue from Core.Classes.LawfulClasses and re-check the client and its refusals"
status: active
owner: spec
size: S
tier: T2
gate: spec-leader
depends_on: []
blocks: []
github: null
origin: "Architect evt_1c4w34tgp0y3a on the SPEC-GEOMETRY-NAT-ORIENTATION D0 (Foundation evidence evt_1earz2j7wmjq0): refusal identical on 8e7fd2987, inherited, routed as its own node to conformance-validator. Steward-filed per COORDINATION section 2."
---

# The box seed names what it uses

**Widened to four seeds** (Steward `evt_5j3wwjdrmrwfd`, on Architect
`evt_6vbm5mwm3mq74` §5). The orientation D0 measured three more accepted
fences refusing on base with UnboundName, by the same mechanism, each
re-rooted under a separate base catalog:

- `conformance/stdlib/geometry/seed-nat-box.md`: `And`, 2403..2406;
- `conformance/stdlib/optimization/seed-bounded-search.md`: `And`,
  854..857;
- `conformance/stdlib/optimization/seed-least-k.md`: `IsTrue`,
  5366..5372.

AC-1 to AC-3 apply to each of the four seeds. Import each missing name
from the module that publishes it, and name that module in the handoff.
The scope is the four seed files.

## Objective

`conformance/stdlib/geometry/seed-box-intersect-hull-monotone.md` checks
as written under `ken check`. Its accepted client imports every name it
uses.

## Settled inputs (main `8e7fd2987`)

- **The import.** The accepted client (`:23`-`:25`) imports only
  `Algorithm.Geometry.NatBox (Box, MkPoint, MkBox, inside, intersect,
  hull, intersect_monotone, hull_monotone)`.
- **The use.** It states `IsTrue` bare at `:44`-`:47` and in the
  theorems from `:78`.
- **The owner.** `IsTrue` is published by `Core.Classes.LawfulClasses`
  (`crates/ken-elaborator/src/r_layer_tests/cat_bool_pub_export.rs:489`),
  not by NatBox.
- **The refusal.** Extracted and run with `ken check` on `8e7fd2987`, the
  client gives rc 1 with UnboundName `IsTrue`. The orientation D0 measured
  the identical result on its scratch tree.
- **No consumer.** A grep of `crates/` for the seed's filename finds 0
  hits, so no CI run executes it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The four seed files only. For this seed, add `IsTrue` to the accepted
client's imports from `Core.Classes.LawfulClasses`. No catalog, spec or
crate change. NatBox is unchanged.

## Acceptance

- **AC-1.** The extracted accepted client checks rc 0 under `ken check`
  on the candidate.
- **AC-2.** Each of the seed's refusals, run from a fresh copy of the
  complete accepted client as the seed prescribes, refuses at its own
  claim, not at UnboundName `IsTrue`.
- **AC-3.** Removing the new import line restores UnboundName `IsTrue` on
  the client.

## Stop conditions

- A refusal changes its reason for any cause other than the unbound name.
- Any change outside the four seed files is needed.

Checks: 3 fired: the seed's consumers were grepped across `crates/`, with
0 hits. 8 fired: AC-3 pairs the import's presence with its absence on the
same client.
