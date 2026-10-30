---
id: LANG-CLI-RESOLUTION-REPORT
title: "Spec 33 §3.3 promises that the standalone and non-catalog CLI routes resolve the five Console/Char pre-source names to their exact pre-source identities, but no ken check, run or native-build observation carries identity, so conformance cannot test the promise at those routes. Add an opt-in machine-readable resolution report to each of the three routes, produced by the route's own elaboration, and pin it with an accepted root and a type-compatible shadow control"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR]
blocks: []
github: null
origin: "Architect advisory evt_6dqnkthbb5vw4 on the Console/Char CLI exact-ID rows; Steward scope ruling evt_2m8pn6mjwnf47 (path (b): Console/Char lands with a declared CLI coverage gap naming this WP). L2 objective: one resolution mode (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# The CLI reports what each written name resolved to

## Objective

`ken check`, `ken run` and `ken native-build` can each emit, on request,
a machine-readable record of how the root unit's written free names
resolved. A conformance case can then tell a pre-source identity from a
type-compatible source re-definition at each CLI route.

## Settled inputs (main `05a15edee`, which includes Console/Char)

- **The promise.** Spec 33 §3.3 and 39 §2.0 promise the five exact
  pre-source identities through the standalone and non-catalog CLI
  routes. Console/Char's seed declares the CLI half a coverage gap that
  names this WP.
- **No existing observation discriminates** (Architect evt_6dqnkthbb5vw4):
  - `Refl` and type checks accept a compatible re-definition;
  - interpreter and native execution never read these five identities;
  - exit status and stderr are the same under the shadow;
  - `check` and `run` emit no artifact.
- **The reserved slot.** `spec/50-stdlib/70-diagnostic-codes.md` §1.4
  defers any CLI machine format. A future one must carry the same code
  strings and reconcile with `ken.verify/v1` (`20-verification/25`).
- **The routes.** `crates/ken-cli/src/main.rs` dispatches `run` (:118),
  `check` (:133) and `native-build` (:137) on `05a15edee`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect).** Rule:
   - the opt-in spelling and the output channel;
   - the record shape and its relation to §1.4 and `ken.verify/v1`;
   - where each route emits the report: after checking, before
     execution or emission, from that route's own elaboration.
2. **One candidate:**
   - the spec text in §70 and §33.3;
   - the CLI implementation;
   - a conformance case replacing Console/Char's declared gap;
   - named tests.

## Acceptance

- **AC-1 (accepted root).** On each of the three routes, a closed root
  that writes only the five names and floor names reports
  `pre-source` with the canonical name (30 §4) for each of the five.
- **AC-2 (shadow).** The same root with a type-compatible `fn write`
  reports `source-declared` for `write` on each route. A mutant that
  reports every name as `pre-source` reddens a named row.
- **AC-3 (identity vocabulary).** Records name identities as the
  pre-source canonical name or as `(module, declaration)`, never as a
  raw GlobalId number. Without the opt-in flag, each route's output is
  byte-identical to today's.

## Stop conditions

- The report needs a kernel or trust change.
- The report changes a route's resolution result rather than observing
  it.

Checks: 8 fired: the shadow control is the wrong-but-type-compatible
identity that every existing CLI observation fails to separate, so AC-2
reddens exactly where today's rows were blind. 9 fired: each route is
observed on its own elaboration, so one shared re-check cannot hide the
route axis.
