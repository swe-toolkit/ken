---
id: RT-NATIVE-NAT-RECURSION-HOSTIO-COPRODUCT
title: "A [Console] proc that recurses structurally on Nat, called from a let in main, checks and runs, but native-build refuses it with E0079 host_coproduct_identity: the decoded HostIO operation's outer constructor is neither in_l nor in_r. Its Rose-recursive twin builds and matches. Locate the cause and make the Nat-recursive shape build with exec equal to run"
status: active
owner: verify
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect SLOTS AC-0 ruling evt_214sk9hkje5k (s23 classified outside LANG-PROC-TREE-ARGUMENT-AND-SCRUTINEE-SLOTS; D0 evt_vvtjn0gjph49). Steward-filed per COORDINATION section 2."
---

# Nat recursion builds natively

## Objective

The SLOTS D0 row s23 (`cntn : Nat -> Unit visits [Console]`, structural
recursion on Nat, bound by a let in `main`) native-builds, and its
native exec output equals its `ken run` output, `p\nb\n`.

## Settled inputs (measured at `6544ac6ff`)

- **The refusal.** `decode_checked_host_operation` refuses with
  `host_coproduct_identity`, "HostIO coproduct constructor identity
  changed" (`crates/ken-elaborator/src/erasure.rs:4497`), when the outer
  constructor is neither `in_l` nor `in_r`. It is fail-closed: E0079,
  no output.
- **The discriminator.** s21, the same program recursing on Rose,
  builds and matches (D0 `evt_vvtjn0gjph49`, matrix
  `/workspaces/ken/tmp/slots-d0/matrix.tsv`, SHA256 `65bf7a44`). That
  Nat recursion is the trigger is inferred, not located (Architect
  `evt_214sk9hkje5k`).
- **Scope.** The shape is one main already accepts after FSRA. It is
  not a SLOTS slot shape and carries no closure.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate, two increments (Steward `evt_534h1jzsgnjkx`,
`evt_112a9rhdaazg9`, `evt_m1tpqnr44meb`):

- **Inc 1:** the native host-operation layout derived from the checked
  Vis type (`054d1ef38`), which clears E0079.
- **Inc 2:** the emitted-member scrutinee lowering. A performing
  member's call is lowered as a call to its one emission, never as an
  inlined first unfolding, and a match over a performed call becomes a
  `Let`. This is B1a WIP2's `erasure.rs` and
  `erasure/performed_call_scrutinee_tests.rs`, byte-identical, carried
  here with credit to its author. RNSPK's rebase drops them. Nothing
  else from WIP2 is carried.

The rows are s23 (IO-typed), `cnth` (the HostIO-typed twin from D1),
and the s21 Rose control. All five `reject_carried_residual_arguments`
sites (`core.rs:3581`, `:6873`, `:18421`; `lowering/source.rs:5382`,
`:5549`) stay byte-unweakened.

## Acceptance

- **AC-0.** The Architect specifies a D0 that locates which constructor
  reaches the decode for s23 and why s21's does not. The Verify
  implementer runs it on scratch and reports; the Architect rules the
  repair site from the report.
- **AC-1.** s23 and `cnth` each native-build with exec stdout equal to
  run stdout, `p\nb\n`. s21 still does.
- **AC-2.** A mutation restoring each repair (the layout, and the
  emitted-member scrutinee lowering) reddens s23 and `cnth`, and s21
  stays green under it.
- **AC-3.** Before handoff, the committed native suites that lower
  performed calls run to completion by exact name: the px10 and px8ta
  smoke and green-three rows, checked_ih parity, and the SLOTS and FSRA
  native rows. A red committed row stops the WP, which then stacks
  behind B1a.

## Inventory (node §1a)

1. N0001 fires in `main`, not in `cntn`. Main's Vis continuation
   (`CheckedHostVisContinuation`, body `Call Var(5) [Var(0)]`) applies
   the host result to a Carried recursor residual whose
   `recursive_unit_body` is None. Keyed on: the producer of the
   eliminated tree is a performing-declaration call that is not a
   direct ComputationalMatch scrutinee (Architect
   `evt_7k5hxq098ea7g`; D2 and D3 `evt_1rz5sk3959mxa`,
   `evt_6h68wkaw348q3`).

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to the decode's refusal for a genuinely foreign constructor.

Checks: 4 fired: read the refusal at `erasure.rs:4497` on `6544ac6ff`.
8 fired: a repair that skips the identity check also builds s23, so
AC-1 compares exec bytes with run bytes, not build success; s21 differs
only in the recursion's carrier and is the control.
