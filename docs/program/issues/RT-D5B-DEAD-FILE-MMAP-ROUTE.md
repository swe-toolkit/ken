---
id: RT-D5B-DEAD-FILE-MMAP-ROUTE
title: "After the file-backed mapping flip, native file acquisition copies into owned bytes, and the file-mmap route (resource_map_file, try_new_mapped_file, map_file_v1) has no live caller while carrying an unsafe block whose SAFETY precondition is false. Delete the route or rule why it stays, correct the comments that now misdescribe the copy-in path, and rule whether ABI-S6's acceptance is met"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-D5B-MAPPING-AVAILABILITY-FLIP]
blocks: []
github: null
origin: "Residuals of RT-D5B-MAPPING-AVAILABILITY-FLIP routed to the Steward: Runtime QA evt_3bhx30w0hjj72 (dead mmap route with a false SAFETY precondition; the mmap-of-fd trait comment) and Architect evt_1rwfscpssx3cr (stale seat-contract and D1 comments, replacement text given). L1 objective: the Linux ABI program, ABI-S6. Steward-filed per COORDINATION section 2."
---

# The dead file-mmap route leaves the native host

## Objective

The native host contains no unreachable file-mmap acquisition route. Its
unsafe block's SAFETY precondition is false, and no caller reaches it. The
comments describe the copy-in path that native file acquisition actually
takes. The Architect states whether ABI-S6's acceptance is met on the
landed base. If it is met, ABI-S6 closes with this WP. If it is not, the
ruling names the unmet criterion, and that criterion is the L1 successor.

## Settled inputs (flip candidate `76c592194`, tree `a8f193e11`)

- **The route has no caller.** `ProcessHost::resource_map_file`
  (`abi_v1.rs:749`) calls `MappingRegionV1::try_new_mapped_file`
  (`effect_v1.rs:1174`), which calls `mapping_v1::map_file_v1`
  (`mapping_v1.rs:94`). The trait default is at `effect_v1.rs:2432`. The
  Architect's sweep (`evt_1rwfscpssx3cr`) found `resource_map_file(` only at
  its two definitions, and each of the other two called only inside the
  route.
- **The live path.** The MAF dispatcher copies the file prefix into
  `MappingStorageV1::InProcess`. Release dispatches on storage
  (`release_native`, HS-1 `evt_7wme5eytyz2bk`).
- **Stale comments, with the Architect's replacement text in
  `evt_1rwfscpssx3cr`:**
  - `planning/static_transition/effects.rs` near `:380`, `:387` and `:543`
    (seat-contract count and carried-seat claim, plus the glyph line);
  - `effect_v1.rs` near `:6696` (the D1 "represented-unavailable"
    sentence);
  - the trait comment at `effect_v1.rs:2431` ("Native ProcessHost overrides
    this with mmap-of-fd").
- **Out of scope.** Spec 38's post-acquisition external-write question
  belongs to the Spec Leader. A zero-copy mmap route is not part of this
  WP; reintroducing one is a new design.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the landed base).**
   - Rule whether the route is deleted whole or any part stays, with the
     reason. The `mincore` test helper and the anonymous-map path are
     live and stay.
   - Rule whether each ABI-S6 acceptance criterion (`ABI-S6.md`
     "Acceptance criteria") is met on main, naming the test that
     witnesses it.
2. **One crates branch.** It removes what AC-0 rules dead and applies the
   comment corrections. Production behavior is unchanged.

## Acceptance

- **AC-1.** On the candidate tree, `git grep` over `crates/` finds no
  definition or call of each deleted function. The anonymous-map and
  `mincore` call sites are unchanged.
- **AC-2.** The flip's named targets stay green: the ken-verify MAF
  scenario, `abi_s6_mapping_file_backed_native`, the ken-host mapping
  rows, and the D1 default-unmap pin.
- **AC-3.** Every `unsafe` block left in `mapping_v1.rs` has a SAFETY
  comment whose precondition a live caller establishes. QA names that
  caller for each block.

## Stop conditions

- AC-0 finds a live caller of the route, or a test that needs it.
- Deleting the route changes any wire layout, catalog row or
  availability.

Checks: 3 fired: the route's three functions were grepped over `crates/`
on `76c592194`, and every hit is a definition or a call inside the route.
The anonymous-map path is a separate function, so it is not in that
population. 8 fired: AC-3 asks for the caller that establishes each
remaining SAFETY precondition. A tree that kept the dead route would
fail it, because no caller establishes the route's precondition.
