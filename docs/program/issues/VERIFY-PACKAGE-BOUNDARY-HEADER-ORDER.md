---
id: VERIFY-PACKAGE-BOUNDARY-HEADER-ORDER
title: "In a multi-source package each headed source overwrites boundary_header, so the program's authority is whichever header elaborated last. Measure, then make two headers in one package a refusal or an order-independent rule"
status: merged
owner: verify
size: S
tier: T1
gate: architect
depends_on: [VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION]
blocks: []
github: null
origin: "Architect residual R2 evt_gvm1bttrssfw from the VERIFY-EXAMPLE-BOUNDARY-HEADER-ISOLATION D0 (evt_2k1ctycw0444f), traced on 7832e4e16. A program's capability authority depends on source order. Steward-filed per COORDINATION section 2."
---

# One package, one boundary header

## Objective

A program's boundary header, and so its `ProgramCaps`, does not depend on
the order in which its sources elaborate.

## Settled inputs (Architect `evt_gvm1bttrssfw`, at `7832e4e16`)

- `compile_ken_package_sources` (`compiler_driver.rs` ~:1266-1276) and
  `prepare_native_program_sources` (~:2333-2346) elaborate each source as
  its own direct call.
- Each headed source writes `module_state.boundary_header`
  (`modules.rs` ~:5457-5458), so the last headed source wins.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. The Architect rules from it and from spec whether two headed
sources in one package are refused, or resolved by a rule that does not
depend on order; then the ruled rule on both package routes.

## Acceptance

- **AC-0 (measure).** Two sources with headers `FS ANone` and `FS AFull`,
  in both orders, on the package and native routes: the header and
  `ProgramCaps` each order produces, and every catalog or corpus package
  with more than one headed source.
- **AC-1.** Both orders give the same result on both routes.
- **AC-2 (control).** A package with one headed source is unchanged.
- **AC-3 (mutation, QA).** Restoring the unguarded overwrite reddens AC-1.

## Stop conditions

- A catalog or corpus package has two headed sources that disagree: stop
  with the list.
- Any kernel, trust or spec change.

## Closeout

Merged `52b4fe001` from exact `b11c5bef2` (PR #4670). Verify QA
`evt_36b1ks7g7zhw6`, Architect `evt_18t4v858sbvs6` (AC-0 ruling
`evt_2h8k6y0ytt2m4`), Decision `dec_2b8xcx8e25nnp`. A second headed source
in one package is now a typed `DuplicateBoundaryHeader` refusal on the
package and native routes, in either order. No catalog or conformance
package has more than one headed source. §1a: 0.
