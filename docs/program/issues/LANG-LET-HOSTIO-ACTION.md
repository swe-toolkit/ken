---
id: LANG-LET-HOSTIO-ACTION
title: "Spec 36 §2.4 denotes `let x = e1 in e2` as `bind ⟦e1⟧ (λ x. ⟦e2⟧)`, monadic sequencing, but as built a `let` of a HostIO value is an ordinary `let`: the bound action is dropped without being performed, and the program type-checks. A Ken program can lose an effect silently. Rule which reading is normative, then make the build and the spec agree, with a run-output control"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect D0 evt_4t0zk96zkg1jg §7 (LANG-STRICT-ISOLATED-ROUTES), carried as an L2 defect by the Steward (evt_77etwxedc3ydm): framed from a Spec ruling on which side is normative. Released to the Verify ring on L2 (operator 2026-09-25 one-resolution-mode objective; it is the only carried L2 defect that fails silently). Steward-filed per COORDINATION section 2."
---

# A let of a HostIO action is not silently dropped

## Objective

A `let` whose right-hand side is a HostIO action either performs it in
source order, or is defined not to and cannot drop an effect silently,
as the Spec ruling says. The build and spec 36 then agree.

## Settled inputs (main `b46d344f0`)

- **The spec.** 36 §2.4 (`36-effects.md:607`) gives
  `⟦ let x = e1 in e2 ⟧ = bind ⟦e1⟧ (λ x. ⟦e2⟧)`, with the grouped form
  at `:609` binding each right-hand side once, left to right.
- **The build** (Architect D0 evt_4t0zk96zkg1jg §7). A `let` of a
  HostIO value elaborates as an ordinary `let`. The bound action is
  dropped and the program type-checks, so a check-only test cannot see
  it. LANG-STRICT-ISOLATED-ROUTES AC-2c compares run output for this
  reason.
- **The entry contract.** 33 §3.2.1: `main : ... → HostIO a ExitCode`
  (`33-declarations.md:245`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate. Either the elaborator sequences the action as §2.4
states, or spec 36 is amended to the ruled reading and the build
refuses or flags the silent drop, whichever AC-0 rules.

## Acceptance

- **AC-0 (Spec ruling, then D0).** The Spec Leader rules whether 36
  §2.4 governs a `let` whose right-hand side is a HostIO value: it
  either sequences the action, or binds the action as a value. If it
  binds the value, the ruling says what stops the effect from being lost
  silently. The Architect then rules the repair site and its fan-in:
  every `let` form, including the grouped one and `let` in a `proc`
  body.
- **AC-1 (run-output control).** A `ken run` program
  `let x = <print "a"> in <print "b">`, spelled with the catalog's
  console action, is pinned on its stdout: `ab` under the sequencing
  ruling, or the ruled refusal or diagnostic otherwise. AC-0 measures
  the base row first. The D0 predicts `b` alone with exit 0, the silent
  drop; if the base prints `ab`, stop and report.
- **AC-2 (no regression).** The named targets the D0 lists pass. A
  `let` of a pure value is byte-unchanged in elaborated core.

## Stop conditions

- A kernel or trust change.
- The ruling needs a surface form that does not exist.

Checks: 4 fired: §2.4's equation was read at its line, and the as-built
behaviour is taken from the Architect's D0, not assumed. 8 fired: AC-1's
base row prints `b` alone, and the repaired row differs in stdout, so
the control can fail.
