---
id: VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT
title: "Program admission reads main from globals, so when a program defines no main, a ken example fence that does may become the program's entry point. Measure, then keep example declarations out of entry-point selection"
status: active
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION]
blocks: []
github: null
origin: "Architect residual R3 evt_gvm1bttrssfw from the VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION D0 (evt_2k1ctycw0444f), traced on 7832e4e16, not measured. Same class as that WP: a checked, unshipped example changes what the program runs. Steward-filed per COORDINATION section 2."
---

# An example is never the entry point

## Objective

A `ken example` fence is checked, not shipped: a `main` it declares is
never selected as the program's entry point, on `ken run` or on native
admission.

## Settled inputs (Architect `evt_gvm1bttrssfw`, at `7832e4e16`)

- `admit_checked_main` (`crates/ken-elaborator/src/program_admission.rs`
  ~:51-55) reads `env.globals["main"]`.
- Example fences elaborate into the same environment as the program, so
  an example's `main` is in `globals` when the program has none. Traced
  from source, not measured.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. If the row is admitted, entry-point selection reads only the
program's own declarations; the Architect rules the mechanism, reusing
the seam `VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION` lands where it fits.

## Acceptance

- **AC-0 (measure).** A `.ken.md` program with a valid header, no `main`
  in its plain fences, and a `main` in a `ken example` fence, on `ken
  run` and native admission. If both refuse with `MissingMain`, record it
  and close the node without a code change.
- **AC-1.** That program is refused with `MissingMain` on both routes.
- **AC-2 (control).** A program whose own `main` is followed by an
  example is admitted unchanged.
- **AC-3 (mutation, QA).** Restoring the globals read reddens AC-1.

## Stop conditions

- A catalog or corpus program takes its `main` from an example: stop with
  the list.
- Any kernel, trust or spec change.
