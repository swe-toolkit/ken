---
id: RT-NATIVE-CLOSURE-INDEX-COST
title: "build_target_closure builds two dependency indexes over every package declaration, not over what the target reaches: one validates the whole checked package again for each declaration, and the other scans each declaration's bytes for every symbol's spelling. So every native build pays a cost set by the size of the prelude and imports, and PX11-NATIVE Inc2's exchange fixture cannot reach execution inside its bound. Validate once, by a type only validation can construct, and compute dependencies only for the symbols the closure walk pops, with closure identity unchanged"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling V1 (evt_1vazgmcjmv3yz) on PX11-NATIVE Inc2's U5 exchange measurement (runtime-implementer evt_4mefzekyx8j11), measured on main source with no Net operation. Steward decision: a separate prerequisite on the Runtime ring, beside RT-NATIVE-BYTES-TO-LIST on the Language ring. L1 Linux ABI program, Steward order."
---

# Native closure indexes over the reachable set

## Objective

A native build computes its target closure in time set by what the target
reaches, not by the size of the package. The closure it computes is the same
closure it computes today.

## Settled inputs (main `9716dd74f`)

- **The two indexes.** `build_target_closure`
  (`crates/ken-elaborator/src/compiler_driver.rs` ~`:5577`) builds both
  indexes before it walks, and the walk reads only `index.get(popped)`
  (~`:5590` to ~`:5629`). `build_target_closures` (~`:5566`) has already
  validated the package once, and it rebuilds both indexes for each target.
  - **C-a.** `declaration_dependency_index` (~`:5655`) builds a selection
    for every declaration. Each selection clones every declaration key into
    `reachable_declarations`. It then calls
    `checked_core_declaration_body_view` (`checked_core.rs` ~`:1526`),
    which re-validates the whole package through
    `validate_body_view_selection` (~`:1749`) and
    `validate_checked_core_package` (~`:1446`). On `Err`, it falls back to
    a spelling-substring scan.
  - **C-b.** `declaration_semantic_dependency_index` (~`:5714`) scans each
    declaration's bytes for each symbol's encoded bytes with
    `contains_subslice` (~`:6114`).
- **Measured cost.** These timings are from the Architect's runs on main:
  - a trivial program (p3) takes 42 s natively;
  - with the `Capability.Net.Stream` import (p2), it takes 80 s;
  - Inc2's exchange fixture runs past 360 s.

  The samples put the busy worker in both indexes (Architect log
  `/workspaces/ken/tmp/arch-subst-coinc-wt/zz-arch-u8probe/l40.log`;
  runtime-implementer samples at +60 s).
- **Unattributed.** A `Term::clone` phase at +300 s has no caller in the
  sample. No design is ruled for it here.
- **Callers.** `checked_core_declaration_body_view` has one non-test
  caller, `declaration_dependency_index`. Its tests (`checked_core.rs`
  ~`:6661` to ~`:7097`) keep the validating entry.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`build_target_closure`, both indexes and a new internal body-view entry in
`checked_core`. Every dependency criterion stays byte-identical, including
the `Err` fallback and C-b's spelling-substring test. A change to what
counts as a dependency changes closure identity and is a different WP.

Out of scope:
- no timeout or bound change;
- no change to the public validating entry;
- no PX11 Inc2 code.

## Acceptance

- **AC-0 (Architect, before code).** Step 0 measures first:
  - timestamps around `build_target_closures` and the phases after it,
    on p3, p2 and the exchange fixture;
  - a deep sample, 60 or more frames, of the exchange at +300 s to
    attribute `Term::clone`.

  The ruling then fixes:
  - **Validate once, by type.** An internal entry takes a borrow that only
    `validate_checked_core_package` can construct (outline:
    `ValidatedCheckedCorePackage<'a>`). `build_target_closures` passes it,
    so skipped validation cannot be represented.
  - **Reachability-driven.** Dependencies are computed on demand for the
    popped symbol and memoized by symbol. The selection's key set is built
    once per package.
  - **The `Term::clone` phase,** if the step-0 sample attributes it to
    this closure.
- **AC-1.** `TargetClosure.closure_identity` and the report fingerprints
  are equal before and after over p3, p2, px4b's `PROCESS_BYTES_PROGRAM`
  and the exchange fixture. The before and after timings are reported as
  evidence. No test asserts a timing.
- **AC-2.** A mutation that computes dependencies for one fewer reachable
  symbol turns AC-1's identity pin red. A second mutation empties the
  `Err` fallback's dependency set and also turns the pin red. If no AC-1
  fixture reaches a declaration through that fallback, AC-0 names one
  that does and adds it to the set.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- Identity equality needs a change to a dependency criterion.
- The validation token needs a change to the public body-view entry or to
  its tests.
- With the closure phase fixed, the exchange still runs past its bound.
  Report the step-0 sample of the remaining phase to the Architect.

Checks: 4 fired. I read `build_target_closures`, both index builders,
the walk's `index.get` reads and the `Err` fallback on `9716dd74f`. Both
indexes span `semantic.declarations`, and both are rebuilt for each
target. 8 fired. A faster build with a different closure would also pass
a timing check, so AC-1 pins identity and the timing is evidence only.
9 fired. C-a has two arms, the body view and the `Err` substring
fallback, so AC-2's second mutation shows the fallback arm is still
reached.
