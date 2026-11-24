---
id: RT-NATIVE-INDEXED-FAMILY-LOWERING
title: "Native lowering refuses every runtime value of an indexed family: a dependent motive over an unindexed head is refused at the body view, and a match on a Vec that carries a runtime index is refused by both the body view and erasure, with only the generated All support family exempt. So a native program cannot build an address from a computed Vec UInt8 16, and PX11's IPv6 exchange row refuses with E0075 at loopback_tail. Rule from an AC-0 how an indexed-family value is represented natively and what a match on it lowers to once its index equations are erased, then lift both planes"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect scope post evt_7v56x4vmh5nc1 on PX11 Inc2's E0075 refusal at loopback_tail: not a one-motive gap, a main capability. Steward scope (1) evt_263egtazn0v7e: its own node, ahead of PX11 Inc2's single promotion. Main's transition sentinel rt_c5_primitive_type_native_gate.rs:38 names this WP. L1 Linux ABI program, Steward order."
---

# Native indexed-family lowering

## Objective

A natively built program that computes a value of an indexed family, and
matches on it, builds and runs, and agrees with the interpreter. That
covers `Vec` convoys and the prelude's `socket_ipv6` over a runtime
`Vec UInt8 16`.

## Settled inputs (main `4a4ce82f2`; Architect `evt_7v56x4vmh5nc1`)

- **The witness.** In `px11_stream_exchange.ken`, `address6` (`:54`) is
  `socket_ipv6 (loopback_tail <15>) 0`. `loopback_tail` (`:48`) is
  recursive, so its `Vec UInt8 16` result stays a runtime value, and
  `socket_ipv6` with its three helpers (`Stream.ken.md:46`) matches on it.
- **Plane (D), observed.** `loopback_tail`'s motive `λn. Vec UInt8 (Suc n)`
  is dependent over the unindexed head `Nat`.
  `validate_supported_match_motive` (`checked_core.rs` ~`:4722`-`:4735`)
  refuses it.
- **Plane (I), predicted.** A match on a `Vec` with a runtime index is
  refused by the body view (`checked_core.rs` ~`:4619`) and by erasure
  (`erasure.rs` ~`:3762`, "match over Vec carries runtime indices").
- **The precedent.** The generated All support family is the only exemption
  today.
- **The pins to restate.** These are the rt_c5 sentinel
  (`rt_c5_primitive_type_native_gate.rs:38`),
  `body_view_rejects_unsupported_dependent_motive`, and nc14's
  `unsupported_dependent_motive` row.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The native representation and match lowering for indexed families, as
AC-0 rules them, the restated pins, and the rows below. No kernel, surface
or trust change, no prelude change, and no fixture rewritten to avoid the
runtime `Vec`.

## Acceptance

- **AC-0 (Architect, before code).** Rule the following:
  - how a native value of an indexed family is represented;
  - what a match on it lowers to once its index equations are erased;
  - which motives plane (D) admits.

  Both planes are lifted together, so no admission of (D) alone. Prior art
  for the ruling is Brady, McBride and McKinna (2003), with the All-support
  exemption as the in-tree precedent. AC-0 may split the node into
  increments.
- **AC-1.** Native rows compared with `ken_cli::run_program` or the
  interpreter:
  - the rt_c5 sentinel's replacement, native Int and String `Vec` convoys;
  - `loopback_tail` plus `socket_ipv6` over a runtime `Vec`, measuring
    plane (I);
  - the three pinned refusals, restated to what the ruling still refuses.
- **AC-2.** Revert the repair: the convoy and `socket_ipv6` rows refuse
  again at their named planes. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The representation needs a kernel, surface or trust change.
- AC-0 finds that a match on an erased index needs an equation the native
  lane cannot discharge. Report that form; do not weaken the row.
- A row's fixture trips the provider safety flag while it is being
  written. Stop to the Steward. No seat retries it in other wording.

Checks: 4 fired. The obligation is to lower `match octets` with a runtime
`octets : Vec UInt8 16`, and both refusal sites exist on `4a4ce82f2`.
8 fired. "loopback_tail refuses" fits both a missing motive and missing
indexed families, so the `socket_ipv6` row measures the second plane.
3 fired. The Architect swept `crates/`, `catalog/`, `docs/program`,
`spec/` and `agent/`: there is no existing node, and three pins restate.
