---
id: RT-NATIVE-CARRIED-RESPONSE-CONSTRUCTOR-MATCH
title: "Native lowering refuses, with N0001 'a closure cannot cross the boundary', a bracket continuation that passes a carried host-response field into a function which builds a constructor from it and matches that constructor by nested patterns, while the interpreter runs the same program; the same program with a literal in place of the field builds. Lower the form natively with interpreter-equal exit status, or, if AC-0 rules it outside the native fragment, refuse it by a typed cause that names the form"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect carry evt_1r4k1r197qv1m (b), measured on the PX11 Inc2 r5 CI red: the refusal reproduces on main with an FS-only bracket and no Net, so it is not specific to sockets, and any interpreter-valid bracket program reaches it. Lanes L1 carry: the Runtime WP after RT-NATIVE-EXITCODE-NONTERMINAL-UNIT-RESULT. Steward-filed per COORDINATION section 2."
---

# A carried response field may feed a matched constructor natively

## Objective

A Ken program whose bracket continuation receives `Err
(ResourceError.ResourceHostIO identity)` and passes `identity` to a function
that builds a `SystemError` from it and matches that value by nested
patterns builds natively and exits with the interpreter's status. If AC-0
rules that the form cannot enter the native fragment, the build refuses
with a typed cause that names the carried-field conversion, not the
generic closure-boundary refusal.

## Settled inputs (main `833ea912b`; anchors re-checked at release)

- **The reproducer.** The Architect's FS-only program is
  `/workspaces/ken/.scratch/architect-rulings/inc2-r5ci-probes/p6_fs_carried_identity_program.ken`
  (66 lines). Its continuation matches `outcome`, and on
  `Err (ResourceError.ResourceHostIO identity)` it calls
  `typed_denied_program first identity`. That function matches
  `process_error_to_system SpawnChild (Some Bytes prog) identity`
  (`Capability/System/Error.ken.md:240`) against a nested `MkSystemError`
  pattern. Native-build refuses with N0001 on main.
- **The control.** `p1_fs_typed_in_cont.ken` in the same directory differs
  only in passing the literal `IOError.CapabilityDenied`. It builds. The
  Architect's discriminators: classifying `outcome` directly by pattern
  builds, and capturing the bracket's own operand builds. The refusal needs
  the carried field to flow into a constructor that is then matched.
- **The refusal path.** `lower_carried_bool_match` (`joins.rs:1029`), then
  `lower_carried_constructor_match` (`joins.rs:1143`), then
  `transfer_represented_boundary_value` (`aggregates.rs:1193`), then
  `boundary_transfer_admissibility` (`boundary.rs:1032`), which refuses
  `Lowered::Closure` at `:1040`.
- **The scratch is perishable.** Copy both programs into a test fixture in
  the candidate. Do not cite `.scratch` from a test.
- **Consumers.** PX11 Inc2's typed-error fixtures avoid the form by
  matching the identity literally (`px11_wrong_grant.ken`,
  `px11_missing_listener.ken`), and the doc comments in
  `crates/ken-cli/tests/px11_stream_socket.rs` name this limit as a separate
  follow-up.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Native lowering of this form, or its typed refusal; the two fixtures; and
the paired test rows. Restoring the PX11 fixtures to the carried form is in
scope only if AC-0 says the rows then test what they claim.

Out of scope: any change to the interpreter, to `process_error_to_system`,
to the boundary rule for genuine closures, or to any other refusal in
`boundary_transfer_admissibility`.

## Acceptance

- **AC-0 (Architect, before code).** Name the value that lowers as a
  `Lowered::Closure` here and why. Then rule one of two outcomes:
  - **lower:** the representation that lets the carried field cross, and
    why genuine closures still refuse;
  - **refuse:** the typed cause and its seat.
- **AC-1, behavior.** Under the lower ruling, the reproducer builds natively
  and its exit status equals the interpreter's for a denied path and for a
  permitted one. Under the refuse ruling, the native build fails with the
  typed cause, and the interpreter row still runs.
- **AC-2, control.** The literal-identity control builds and runs natively,
  unchanged. A program that sends a genuine closure across the boundary
  still refuses with the existing N0001.
- **AC-3, mutation.** Revert the lowering or refusal change. The AC-1 row
  reddens, and the AC-2 rows do not. Restore byte-identically.
- **AC-4.** Every touched test target passes on the exact candidate with no
  name filter, and QA re-runs it there. Full-workspace CI is green on the
  exact candidate.

## Stop conditions

- Admission needs a change to the interpreter, a kernel rule, or a trusted
  postulate.
- The fix would admit a genuine closure across the boundary.
- AC-0 finds the refusal caused by something other than the carried
  response field. Report the mechanism; do not widen this node.

Checks: 4 fired. I read the Architect's measured ruling, both probe
programs, and the four functions on the refusal path on `833ea912b`; the
programs differ only in the carried field. 8 fired. The literal control and
the genuine-closure row separate a targeted repair from one that weakens the
boundary rule. 3 fired. The PX11 typed-error fixtures are the existing
consumers that work around this limit.
