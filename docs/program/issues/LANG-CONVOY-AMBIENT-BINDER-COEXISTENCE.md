---
id: LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE
title: "A true dependent-match proof fails when one more ambient binder is in scope: an unused binder typed at the index hits an internal assertion that the index-equation convoy overlaps an ambient context convoy, and an unused hypothesis carrying the goal's embedded eliminator makes the elaborator emit ill-typed core. Let both convoys coexist"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY]
blocks: []
github: null
origin: "Adversary M8 finding evt_2qd35nyer850 on 66cb5e9b7 (LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY): two sibling shapes of the class, both failing on fd3b9262b too, fail-closed. Check 7 (a fix must hold on its siblings). Steward-filed per COORDINATION section 2."
---

# An ambient binder does not break a convoyed match

## Objective

A true dependent-match proof checks whether or not an unrelated ambient
binder or hypothesis is in scope. Well-formed source never reaches an
internal assertion, and the elaborator never emits core the kernel
rejects.

## Settled inputs (Adversary `evt_2qd35nyer850`, at `66cb5e9b7`)

The fixture is `env()` from
`crates/ken-elaborator/tests/lang_embedded_context_convoy.rs`. The rows'
source is in the finding.

- **Shape 1, internal error.** E8, an `Equal a (g ...) (g ...)` goal
  under an outer `match c` with an inner `match i`, is Ok. E9 adds an
  unused `(xs : DualEnv a n)` and fails with `Internal("index-equation
  convoy unexpectedly overlaps an ambient context convoy")`. E11, with
  `xs` after `i`, fails the same way. E10, which is E9 without the outer
  match, is Ok.
- **Its locus.** In `plan_coherent_frame_motive`, the `equation_convoy`
  branch (elab.rs:4486-4490) returns that error when
  `probe_context_convoy` is non-empty. The merged repair changed only the
  embedded-method gate below it.
- **Shape 2, ill-typed core.** H9 is AC-1 row R1 plus an unused
  hypothesis `(h : Equal a (dual_lookup ...) y)`. It fails with
  `KernelRejected TypeMismatch` at the `match xs`, the signature R1 had
  before the fix. H5 (leaf `h`) fails identically, and H1, the nested
  `match i` form using `h`, fails with a `J`-cast mismatch. The controls
  H6, H7 and H8 are Ok. In these rows the convoyed binder's own type
  carries the goal's embedded eliminator.
- **Not a regression.** All the failing rows fail identically on
  `fd3b9262b`. No row has a recursive step.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first. The Architect rules how the index-equation convoy and an
ambient context convoy compose, and whether one rule also convoys a
hypothesis whose type mentions the goal's embedded eliminator. Then the
repair.

## Acceptance

- **AC-1.** E9, E11, H9, H5 and H1 check, with no Axiom and no trust
  change.
- **AC-2 (control).** E8, E10, H6, H7 and H8 stay Ok. The
  `lang_embedded_context_convoy` suite, including its false twin, is
  unchanged, and so is the catalog census.
- **AC-3.** A false twin of H9, whose hypothesis disagrees with the goal,
  is still refused.
- **AC-4 (mutation, QA).** Restoring the overlap assertion reddens E9.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- The repair overlaps `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`'s gates
  or `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE`'s wall: stop to the
  Architect to rule the order.
