---
id: LANG-DENOTATION-OPERATION-HEAD-DECODE
title: "The checked-target denotation's operation decoder fully normalizes every Vis operation subterm, payload included, but reads only head constructor identities, so a recursive transparent declaration in a discarded payload with a non-concrete argument unfolds without bound and the export overflows its stack on both the rooted and the rootless route. Reduce each read head to weak-head normal form where it is read, so the identities are unchanged and discarded payloads are never normalized"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION]
github: null
origin: "Architect evt_8asgjmh2jhzn on runtime-implementer's U2 stop evt_5r6448w8zt3ns (RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION Increment 2): a producer defect in the shared denotation decoder, routed as a scoped prerequisite owned by the elaborator's ring. Steward-filed per COORDINATION section 2."
---

# The denotation decoder reads operation heads by weak-head form

## Objective

`compile_checked_target_denotation` exports a source whose Vis operation
payload reaches a recursive transparent declaration applied to a
non-concrete argument, on both the rooted (Strict) and the rootless
branch, at a stated 16 MiB stack. The decoder reads every identity it
reads today, and the same identities.

## Settled inputs (main `d42f7ad28`; `compiler_driver.rs` unchanged since `be58665f9`)

- **The defect.** `decode_operation`
  (`crates/ken-elaborator/src/compiler_driver.rs:1080`-`:1158`) runs
  `ken_kernel::normalize` on each full-arity Vis node (`:1084`-`:1086`).
  `normalize` is full normal form (`ken-kernel` `conv.rs:501`-`:523`),
  under binders and inside every Elim method (`:539`). The decoder then
  reads only the operation's head, the InL leaf's head, and the InR
  ambient and leaf heads, and discards the rest (`let _ = args;`,
  `:1155`).
- **The same hazard, already named.** `visit_decl`'s comment
  (`:936`-`:939`) says whole-body normalization unfolds a structurally
  recursive declaration without a concrete scrutinee forever. The
  decoder re-enters that hazard one level down.
- **The measurement.** D1 (Legacy) shows a 375-repetition cycle of Elim
  method, Lam body, App argument and `unfold_const` (`conv.rs:42`). D2
  (Strict, rooted) overflows too, so the defect is route-independent
  (runtime-implementer `evt_5r6448w8zt3ns`). The native build completes
  on the same source because it normalizes under recursion barriers
  (`:2747`, `:2814`).
- **The design.** Architect `evt_8asgjmh2jhzn`: a `head_normal` closure
  calling `ken_kernel::whnf` (public, `ken-kernel` `lib.rs:77`) at each
  place an identity is read, in four hunks of `decode_operation` only.
  Its checked patch (+16 -7, one file) is
  `/workspaces/ken/tmp/arch-rsw-u2-decode-whnf.patch` (sha256
  `4b76ef7235fdcb17...`). No test pins the decoder's error strings.
- **The fixture.** `lang_process_child_pipe_write_home.ken` (sha256
  `b2ca0ccc...aba8`). Its write operation is `InL
  (PrivateResourceStreamWrite ...)` (`prelude/process.rs:272`-`:275`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`decode_operation` in `compiler_driver.rs`, and one new ken-elaborator
test. No change to error texts, to the recursion-barrier environment,
to the export format or to any caller.

## Deliverable

One candidate on the Language ring.

## Acceptance

- **AC-1.** A ken-elaborator test exports, through
  `compile_checked_target_denotation` with explicit roots, a closed
  source whose operation payload reaches a recursive transparent
  declaration with a non-concrete argument: the fixture above, or a
  minimal source with the same property. It runs at a stated 16 MiB
  stack and asserts the exact `perform_nodes` set, enumerated by hand
  from the source's operations, not copied from a run. It also runs the
  same source rootless.
- **AC-2.** A disposable mutant that changes the one call inside
  `head_normal` back to `ken_kernel::normalize` makes AC-1 fail by stack
  overflow on at least the Strict branch. Name the observed outcome as
  measured, and restore byte-identically.
- **AC-3.** Every existing caller of `compile_checked_target_denotation`
  passes with its pinned inventories and obligations byte-identical:
  `b1_acceptance`, `b1_exact_denotation_alphabet`, `b2_acceptance`,
  `lang_delete_rooted_driver_entries`, `px7f_resource_lifetime_export`,
  `px8p_checked_buffer_producer`, `px8x_static_export_projection` and
  `v2_acceptance` (ken-elaborator), `b3_acceptance` (ken-interp) and
  `px10_spawn_plan_wait` (ken-cli). Full-workspace CI on the exact
  candidate is the gate.

## Stop conditions

- The repair changes an identity the decoder reads, or needs a kernel,
  trust or export-format change.
- AC-1 still overflows under the repair, on either branch.

Checks: 2 fired. The Architect traced the export path's normalization
fan-in: the decoder is its sole payload normalization. 8 fired. AC-1
completes with a hand-enumerated inventory only under weak-head
decoding, and AC-2's mutant restores the measured overflow, so the pair
fails exactly when the payload is normalized.
