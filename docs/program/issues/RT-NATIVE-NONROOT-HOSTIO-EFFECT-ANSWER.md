---
id: RT-NATIVE-NONROOT-HOSTIO-EFFECT-ANSWER
title: "A native root whose non-root HostIO helper answers with an unstepped effect tree (the taken arm is a host_bind, not a bare host_pure) traps 'malformed borrowed process input' with exit 1 and never performs the effect, while the interpreter performs it and exits with the answered code; and a root host_bind that inspects a helper's ITree.Ret-wrapped ExitCode answer is outside the landed EXITCODE fragment. Drive a carried non-root HostIO answer natively, effects included, with interpreter-equal stdout and exit status, or refuse the shape at build time by a typed cause that names it"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_41cmwhj7afwpk on RT-NATIVE-EXITCODE-NONTERMINAL-UNIT-RESULT (44e7a1c06), measured: native exit 1 with the trap and empty stdout against interpreter exit 5 and stdout 'w'; pre-existing at 3e4fb00c1. The test header's GAP (rt_native_exitcode_nonterminal_unit_result.rs:38) names the host_bind-inspects-answer shape as a separate Runtime carry. Steward-filed per COORDINATION section 2."
---

# A non-root HostIO answer that is still an effect runs natively

## Objective

A native root that calls a non-root `HostIO` helper performs every effect
in the helper's answer and exits with the code the interpreter exits with,
whatever the answer's shape:

- a bare `host_pure` (landed);
- an `ITree.Ret`-wrapped answer that the root inspects with `host_bind`;
- an answer still holding an unexecuted effect bind.

If AC-0 rules a shape outside the native fragment, the build refuses it with
a typed cause naming that shape. A trap at run time with the effect dropped
is never the outcome.

## Settled inputs (main `44e7a1c06`; anchors re-checked at release)

- **The reproducer (Adversary `evt_41cmwhj7afwpk`).** The untracked probe is
  `/workspaces/ken/.worktrees/adversary/crates/ken-cli/tests/zz_adv_exitcode_wrapped.rs`.
  Copy it into a fixture; do not cite another seat's worktree from a test.

  ```ken
  proc walk (fuel : Nat) (argv0 : Bytes) : HostIO AFull ExitCode visits [Console] =
    match fuel {
      Zero |-> host_bind AFull (Result IOError Unit) ExitCode
                 (host_console AFull (Result IOError Unit)
                   (write ConsoleStream.Stdout (bytes_encode "w")))
                 (\r. match r { Err _e |-> host_pure AFull ExitCode (ExitCode.Failure 40);
                                Ok _u |-> host_pure AFull ExitCode (ExitCode.Failure (int_to_uint8_raw P)) });
      Suc smaller |-> walk smaller argv0 }
  ```

  `main` matches `Cons argv0 _rest` and calls `walk Zero argv0`.
  - The rows at P = 5, P = 3 (computed from `argv0`) and P = 0 all exit 1
    natively, with stderr `ken native trap: malformed borrowed process
    input` and empty stdout.
  - The interpreter prints `w` and exits 5, 3 and 1 respectively.
- **The controls are at parity.** Each prints `w` and exits with the
  interpreter's code:
  - the same helper at fuel `Suc Zero` and `Suc (Suc Zero)`;
  - a non-recursive effectful proc called directly;
  - the effect-free unstepped shape at computed payloads 0, 3, negative,
    2^64 and 256;
  - a root-level `host_bind` continuation.
- **The mechanism.**
  - `strip_carried_terminal_ret` (`calls.rs:2934`) strips a carried
    `ITree.Ret` and passes every other constructor through unchanged.
  - The root's carried-`ExitCode` decode (`units.rs:11672`) then requires an
    `ExitCode.Failure` tag (`units.rs:11741`), so a carried Vis/bind word
    refuses with the -1 sentinel.
  - Nothing drives the carried effect tree, which is why stdout is empty.
- **The landed carry.** The GAP at
  `crates/ken-cli/tests/rt_native_exitcode_nonterminal_unit_result.rs:38`
  covers one `ITree.Ret`-wrapped helper answer. It names a `host_bind` that
  inspects such an answer as a separate Runtime carry. The Architect's
  scratch probe for that shape is
  `/workspaces/ken/.scratch/architect-rulings/exitnt-ret-class-probe.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Native lowering of a carried non-root `HostIO` answer at the root, or its
typed refusal; the paired fixtures; the GAP sentence. Out of scope:
the interpreter, the `ExitCode` payload transfer already landed, and any
change to `ITree` or the effect ABI.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - whether driving the carried tree is one mechanism for both shapes, and
    where it sits;
  - which shapes enter the native fragment, and the typed refusal and seat
    for any that do not;
  - the consumer census of `strip_carried_terminal_ret` and the root
    decode.

  If the two shapes need separate mechanisms, AC-0 keeps the effectful
  answer here and names the `host_bind` shape for a successor frame.
- **AC-1, behavior.** Native and interpreter rows, compared on stdout and
  exit status:
  - each Adversary row at P = 5, 3 and 0;
  - the parity controls above, still at parity;
  - the `host_bind`-inspects-answer row, if AC-0 admits it.

  A refused shape fails at build time with its typed cause and leaves no
  artifact.
- **AC-2, mutation.** Restore the pass-through of non-`Ret` words in
  `strip_carried_terminal_ret`, or revert the driver AC-0 names. The P = 3
  row reddens and the controls do not. Restore byte-identically.
- **AC-3.** Every touched test target passes on the exact candidate with
  no name filter, and QA re-runs it there. Full-workspace CI is green on
  the exact candidate.

## Stop conditions

- The fix needs an `ITree`, effect ABI or kernel change.
- AC-0 finds a further non-root answer shape that traps and that the ruled
  mechanism does not close. Report it rather than widening this node.

Checks: 4 fired. I read `strip_carried_terminal_ret` and the root decode on
`44e7a1c06`; they match the Adversary's mechanism. 9 fired. The fuel axis
is the discriminator: only `Zero` traps, so the frame keeps the `Suc`
controls beside the failing rows. 8 fired. The empty stdout separates an
undriven effect from a wrong decode, and AC-2 reddens one row, not the
controls.
