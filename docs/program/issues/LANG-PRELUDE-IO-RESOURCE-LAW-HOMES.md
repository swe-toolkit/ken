---
id: LANG-PRELUDE-IO-RESOURCE-LAW-HOMES
title: "Capability.System.IO and Capability.System.Resource present 23 checked loop and bracket equations as client-usable package laws, but 21 of them name internal helpers, so no Strict client can state them, and once I4 deletes Legacy the packages' prose promises a route that does not exist. Home the two IO laws a client can state, record the 21 as internal implementation equations in the intrinsic ledger, and correct both packages' prose"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-NONHOME-FAMILIES-HOME]
blocks: [LANG-DELETE-LEGACY-RESOLUTION]
github: null
origin: "Architect evt_5r4z1rhn7e414 (G11, Steward S2) on LANG-PRELUDE-ENTROPY-SURFACE-ROUTE AC-0; recut on this node's AC-0, Architect evt_6981f57xbz42x. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# The IO and Resource package laws say what a client can use

## Objective

The two IO laws a Strict client can state are homed at their pre-source
GlobalIds: importing one from `Capability.System.IO` is accepted, and the
same client without the import refuses `UnboundName`. The other 21 are
recorded as internal implementation equations, and neither package's prose
presents them as client-usable. After this node, I4 can delete Legacy
without either package advertising an unreachable law.

## Recut (AC-0, Architect `evt_6981f57xbz42x`, base `2e3dbee36`)

The connective question passed: the pre-source `And` and `Not` convert to
`Core.Logic.Conjunction` and `Core.Logic.Not` by equal bodies, with a
negative control. The G11 premise did not. Probing each global in each
law's statement under Strict, one client per name:

- 17 laws name internal helpers (`private_write_all_*`,
  `private_with_*_after_*`, `release_if_live`, `resource_settle_result_for`,
  `AmbientOp`, `FSOp` and others) and cannot be written. The Architect's G9
  and G10 rulings keep those helpers internal.
- `resource_settle_body_ok_release_error` and
  `resource_settle_body_error_release_error` are writable only at a closed
  release error, where `Proved` or `Refl` already proves them.
- `resource_settle_body_ok_clean` and `resource_settle_body_error_clean`
  are writable but reflexive in client vocabulary.
- `write_all_advance_start_prop` and `write_all_advance_start` are
  writable with content, in the form of the homed precedent
  `write_all_exact_prefix_prop` (`modules.rs:428-429`).

The rule: home a law if and only if its generic statement can be written in
Strict-reachable vocabulary and is not reflexive there.

## Settled inputs (main `2e3dbee36`)

- **The precedent rows.** `("Capability.System.IO",
  "write_all_exact_prefix_prop")` and its `::exact_prefix` proof
  (`crates/ken-elaborator/src/modules.rs:428-429`). The two new rows take
  the same form.
- **The prose.** `IO.ken.md:10-27` presents the write loop's checked laws
  and a clause table naming all eight. `Resource.ken.md:11-33` presents the
  fifteen bracket equations as the package's checked equations. The
  `ResourceError` constructors resolve only qualified (`ResourceError.Closed`).
- **The ledger.** `spec/30-surface/30-intrinsic-ledger.md:371`, "Checked
  laws and helpers are not intrinsic candidates", is the table where G9
  recorded the buffer helpers.
- **The evidence for the 21.** The kernel-term conversions in
  `crates/ken-elaborator/tests/cat_system_resource_laws.rs` and the
  `write_all` reads in `px8f_buffer_io_surface.rs`. They stay as they are.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Two `PRELUDE_PACKAGE_HOMES` rows and the IO export line; the two packages'
prose; spec 30 H rows for the two homed laws and ledger rows for the 21
(Spec enclave); one package test. No new law, identity, kernel change or
prelude registration change, and no helper homed.

## Acceptance

- **AC-1.** For `write_all_advance_start_prop` and `write_all_advance_start`,
  a Strict client importing it from `Capability.System.IO` is accepted,
  keyed on the pre-source GlobalId read from the live env, and the same
  client without the import refuses `UnboundName`. The client states the
  law's use, not only its name: it declares a theorem whose type is the
  law's generic statement written in imported vocabulary, proved by the
  imported law.
- **AC-2.** `IO.ken.md` and `Resource.ken.md` stop presenting the 21 as
  client-usable. They say the checked prelude equations name internal
  helpers, are verified as implementation equations by the two tests above,
  and cannot be stated by a client. IO's clause table keeps its clauses and
  names only `write_all_advance_start` and
  `write_all_exact_prefix_prop::exact_prefix` as usable laws. Resource's
  Using section gives the qualified `ResourceError.Closed` spelling. Every
  `ken` block in both files still checks.
- **AC-3.** Spec 30 gives the two homed laws H rows naming
  `Capability.System.IO`, and the ledger table records the 21 as internal
  checked implementation equations with no H row. The Spec Leader and CV
  review it, with a Decision.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Carried to I4

The two I4-owned Legacy `Not` clients in `cat_system_resource_laws.rs`
(`assert_legacy_not_client`) lose their source route when Legacy goes. I4
rules them as kernel-term rows, like the eleven effect laws.

## Stop conditions

- A home needs a new identity, a new law, a helper export or a kernel
  change.
- A homed law's checked type differs from its pre-source declaration.
- A `ken` block in either package stops checking after the prose change.

Checks: 8 fired: the Architect showed that import-accepts plus
no-import-refuses passes for an unstatable law, so the recut AC-1 also
requires the client to apply the law, which a name-only home cannot
satisfy. 6 fired: the 21-versus-2 partition is the Architect's per-name
Strict probe over every global in each statement, not a reading of shapes;
I confirmed the precedent rows at `modules.rs:428-429` and the ledger table
at `30-intrinsic-ledger.md:371` on `2e3dbee36`.
