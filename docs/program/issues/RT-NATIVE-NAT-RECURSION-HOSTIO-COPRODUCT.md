---
id: RT-NATIVE-NAT-RECURSION-HOSTIO-COPRODUCT
title: "A [Console] proc that recurses structurally on Nat, called from a let in main, checks and runs, but native-build refuses it with E0079 host_coproduct_identity: the decoded HostIO operation's outer constructor is neither in_l nor in_r. Its Rose-recursive twin builds and matches. Locate the cause and make the Nat-recursive shape build with exec equal to run"
status: active
owner: verify
size: M
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

The native lowering repair, with s23 and s21 as named pins, in one
candidate.

## Acceptance

- **AC-0.** The Architect specifies a D0 that locates which constructor
  reaches the decode for s23 and why s21's does not. The Verify
  implementer runs it on scratch and reports; the Architect rules the
  repair site from the report.
- **AC-1.** s23 native-builds and its exec stdout equals run stdout.
  s21 still does.
- **AC-2.** A mutation that restores the refusal on the repaired path
  reddens s23's pin, and s21's pin stays green under it.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to the decode's refusal for a genuinely foreign constructor.

Checks: 4 fired: read the refusal at `erasure.rs:4497` on `6544ac6ff`.
8 fired: a repair that skips the identity check also builds s23, so
AC-1 compares exec bytes with run bytes, not build success; s21 differs
only in the recursion's carrier and is the control.
