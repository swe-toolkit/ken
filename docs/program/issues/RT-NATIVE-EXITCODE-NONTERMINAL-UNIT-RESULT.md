---
id: RT-NATIVE-EXITCODE-NONTERMINAL-UNIT-RESULT
title: "A generated native unit whose result is an ExitCode constructor projects it into the process exit status at its own return, because transfer_unit_result_into_carrier keys the projection on process_object alone. A recursive helper returning ExitCode to a caller that inspects it therefore traps natively, exit 1, where the interpreter exits with the caller's chosen status. Project the exit status only at the root result boundary, as the carried join seat already does"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Measured native/interpreter divergence on main fa83c81f1 (post-merge hunt of RT-NATIVE-EXITCODE-FAILURE-OUT-OF-RANGE, evt_3a6hs2ky4tz9p); pre-existing before that merge, which moved the trap into the helper. Violates the root-boundary rule of RT-ROOT-EXIT-PROJECTION-KEYED-ON-JOIN. L1 Linux ABI program, native parity. Steward-filed per COORDINATION section 2."
---

# Only the root result becomes the exit status

## Objective

A native program in which a generated unit returns an `ExitCode` value to
a caller exits as the interpreter does. The process exit status is decoded
once, at the root result boundary, and a non-root unit result stays an
ordinary carried constructor.

## Settled inputs (main `fa83c81f1`; anchors re-checked at release)

> **Corrected by AC-0** (Architect `evt_1cvv78w07fk1v`, on `0a53d387d`). The
> first two inputs below are false. The seam is reached by 0 of 21 measured
> rows. The projector is the join seat, whose gate reads
> `terminal_result_origins`, a set selected for every unit, root or not.
> `transfer_constructor_operands` (`core.rs` ~`:14904`) projects a carried
> Failure code with no gate. The ruled fix selects that set only for the
> root, gates the Failure arm on it, and decodes a non-immediate root code
> through the existing `transfer_carried_failure_exit_status` mapping. That
> last step changes the root decode. The Steward authorized it
> (`evt_28ggwxv0sjmrn`), relaxing the first stop condition for that step
> alone.

- **The seam.** `transfer_unit_result_into_carrier`
  (`lowering/calls.rs:1851`) sets `process_exit` from `self.process_object`
  and the constructor being `exit_success` or `exit_failure` (`:1861-1867`),
  then emits `emit_process_exit_status` and an `ImmediateExitStatus` carrier
  (`:1875-1877`). Nothing asks whether this unit's result is the root's.
- **The sibling seat already gates.** The carried join seat
  (`lowering/joins.rs:343`) also requires `required_kind ==
  Some(ScalarMergeKind::ExitCode)` or `terminal_result_origins.contains(origin)`.
- **What the last merge changed.** `emit_process_exit_status` now ends in
  `return_exit_sentinel` (`calls.rs:3053`), which on a negative status
  returns Abort from the function being lowered. At this seam that is the
  helper.
- **The measured rows** (at `befb00089` on main `fa83c81f1`). The root is the
  program of `crates/ken-cli/tests/rt_native_exitcode_failure_out_of_range.rs`,
  with a recursive helper `pick (fuel : Nat) (argv0 : Bytes) : ExitCode`
  that returns `ExitCode.Failure (int_to_uint8_raw P)` at `Zero`. Main's
  Cons arm matches `pick (Suc (Suc Zero)) argv0` and returns `Failure 7` on
  Success, `Failure 9` on Failure.
  - P out of range (`0 - |argv0|`, `2^64 + |argv0|`, `256`): native exit 1,
    "malformed ExitCode::Failure payload". Interpreter exit 9.
  - P = 3: native exit 1, "malformed borrowed process input". Interpreter
    exit 9.
  - Agreeing controls: the helper result returned directly by main (exit 1,
    MalformedHostAbiField both sides); a non-recursive inlined helper, or a
    local match on the constructor (exit 9 both sides).
- **Pre-existing.** With `fa83c81f1`'s ken-runtime diff reverted, the
  out-of-range row still exits 1 natively against the interpreter's 9; only
  the trap line differs.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The exit-status gate at the unit-result seam, and whatever the Architect
rules the gate reads. Retained as proved: the root projection of
RT-ROOT-EXIT-PROJECTION-KEYED-ON-JOIN, `emit_process_exit_status`,
`return_exit_sentinel` and the four root-terminal rows of
`rt_native_exitcode_failure_out_of_range.rs`.

## Acceptance

AC-1 and AC-2 are replaced by the Architect's recut in `evt_1cvv78w07fk1v`:
its 21 paired rows, and the mutations M-F2, M-F3 and M-D. AC-0 is done.

- **AC-0 (Architect, before code).** Rule what makes a unit result the root
  result at this seam, and whether it shares the join seat's predicate.
  Sweep the other generated-unit sites that call `emit_process_exit_status`
  and name any that need the same gate.
- **AC-1, behavior.** Paired native and interpreter rows on the helper
  program above, for P = 3 and for P = `0 - |argv0|`. The two lanes agree on
  each, at exit 9. A root-terminal control, the helper result returned
  directly, keeps its current agreed exit 1 and MalformedHostAbiField.
- **AC-2, mutation.** Restore the `process_object`-only gate. Both AC-1
  helper rows redden natively. Restore byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The fix needs the root exit projection, the sentinel or the decoder to
  change.
- A non-root `ExitCode` result needs a representation other than the
  ordinary carried constructor.

Checks: 4 fired. I read the seam at `calls.rs:1851-1885`, the join gate at
`joins.rs:343-357` and `return_exit_sentinel` at `:3053` on `fa83c81f1`; the
unit-result gate reads no root or terminal fact. 8 fired. The landed test's
rows are all root-terminal, so a decoder that projects at every unit result
passes them; AC-1's helper rows separate the two. 6 fired. The measured
population is one recursive helper with an inspecting caller; AC-0 sweeps
the other call sites rather than assuming they are clean.
