# PX10 held-child signalling conformance — Increment 2

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md §1.11.4`.
Approved Spec child: `7280ea37009b5bdfff569107ab7f6e24f7b2902f`.

These are acceptance cases, not execution evidence. The seed has not been
Ken-checked or run by the official conformance runner. No CI, publication,
landing, or trust measurement is claimed. The cases exercise the public
`Capability.Process.Child` surface and do not assert Ken's syscall path,
resource representation, or process identifier.

The witness fixture uses a supported Linux host with pidfd support and a
non-auto-reaping `SIGCHLD` disposition (`SIG_DFL`, not `SIG_IGN` or
`SA_NOCLDWAIT`). The harness also verifies a process-wide `SIGTERM`
disposition of `SIG_DFL` and that `SIGTERM` is unblocked in the spawning
thread. The fixture's pidfd is only an external exit witness; it is not a Ken
interface or an assertion about Ken's implementation.

The six mapping cases also require default dispositions for `SIGHUP`, `SIGINT`,
`SIGQUIT`, `SIGUSR1`, and `SIGUSR2`, and that each is unblocked in the spawning
thread. The target installs no handlers for them. `SIGKILL` is uncatchable and
unmaskable. The harness sets the target's core-size limit to zero for the
`SIGQUIT` case.

## Fixture and common client

The harness supplies the absolute-path executable `px10-child-exit-witness`.
The target waits for EOF on stdin and then exits 0; its default `SIGTERM`
action terminates it. A helper opens a pidfd for the target while it is alive,
then writes `watcher-ready\n` to stdout. After the target exits, the helper's
pidfd becomes readable; the helper then writes `target-exited\n` and exits.
The helper observes the target but never waits for or reaps it. It writes no
other stdout data. `watcher-ready` is emitted only after the exit witness is
armed, and the target cannot exit normally until the Ken client closes stdin.

Each client declares `Process` and `FS AFull`, imports the required names from
`Capability.Process.Child`, and uses `spawn_plan`, `with_stdin`, and
`with_stdout` to build a plan for the fixture's absolute path with nonempty argv
`[b"px10-child-exit-witness"]`. It selects `ChildPipe` for stdin and stdout,
then uses `withChild`. The body takes those ends and reads
the fixture markers through `read_pipe` with a `withBuffer`-owned
`BufferHandle` and admitted `BufferWindow`, collecting returned spans with
`spanBytes`. Pipe clients use `visits [FS, Process]`. Every listed operation
occurs inside the `withChild` body, before it returns or bracket settlement
begins.

The harness bounds each run. A timeout is failure; cleanup after a timeout is
not an accepted result. No case uses a sleep or elapsed time to establish the
target's exit state.

### surface/ffi-io/process-signal-live-before-wait

- promise class: normative compatibility vector.
- spec: §1.11.4; AC-2b (i).
- given: Read `watcher-ready` while the target is blocked on its still-open
  stdin pipe. Call `signal_child ChildSignal.SignalTerminate`, then read
  `target-exited`, then call `wait_child`, all before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 15)`.
- measured: the live-ready marker, both public operation results, and the
  post-signal exit marker.
- claimed: signalling a live held child succeeds and the wait reports
  `Signaled 15`.
- gap: the exit marker relies on the harness fixture's stated pidfd witness;
  this case does not observe Ken's host syscall or assert how it delivers the
  signal.
- why: the held stdin pipe keeps the target live until signalling. A
  `signal_child` no-op leaves it blocked, so the expected exit marker and
  `Signaled 15` are not produced.

### surface/ffi-io/process-signal-exited-before-wait

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Read `watcher-ready`, close the child's stdin with
  `close_pipe_write`, and read `target-exited`. Before any `wait_child`, call
  `signal_child ChildSignal.SignalTerminate`; then call `wait_child`. Keep the
  `withChild` body open through both calls.
- expect: the close returns `Ok Unit`; after the exit marker,
  `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Exited 0)`.
- measured: readiness precedes the EOF release; the exit marker follows
  pidfd readability; the signal and wait results are observed before bracket
  settlement.
- claimed: a naturally exited but not yet waited child still accepts
  `signal_child`; its actual exit remains `Exited 0`.
- gap: the exit-state witness is the harness fixture protocol, not a Ken
  syscall trace. The helper does not reap the target; default `SIGCHLD` keeps
  it waitable by the Ken parent.
- why: the body calls the signal operation only after the helper reports
  target exit, with no prior wait or settlement and no timing-only delay.

### surface/ffi-io/process-signal-after-cached-wait

- promise class: normative compatibility vector.
- spec: §1.11.2 and §1.11.4; AC-2b (iii).
- given: Use the same fixture, plan, pipes, and permissions as the
  exited-before-wait case. Read `watcher-ready`, close stdin, read
  `target-exited`, call `wait_child`, then call
  `signal_child ChildSignal.SignalTerminate` before the body returns.
- expect: `wait_child` returns `Ok (ChildExit.Exited 0)`; the subsequent
  `signal_child` returns
  `Err (ResourceHostIO (IOError.Other 3))`, not `Closed`.
- measured: the same exit witness, followed by the cached wait result and
  the later public signal result while the bracket is still open.
- claimed: a successful cached wait changes the later signal result to
  `Other 3` while the resource remains unsettled.
- gap: this row does not cover auto-reaped children or use after bracket
  settlement; those are separate §1.11.4 outcomes.
- why: paired with the preceding row, fixture, plan, child exit and authority
  are fixed; only whether `wait_child` precedes `signal_child` changes. The
  pair distinguishes exited-but-unwaited `Ok Unit` from cached-waited
  `Other 3`.

### surface/ffi-io/process-signal-map-hangup

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalHangup`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 1)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalHangup` maps to SIGHUP, signal 1.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

### surface/ffi-io/process-signal-map-interrupt

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalInterrupt`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 2)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalInterrupt` maps to SIGINT, signal 2.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

### surface/ffi-io/process-signal-map-quit

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalQuit`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 3)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalQuit` maps to SIGQUIT, signal 3.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

### surface/ffi-io/process-signal-map-kill

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalKill`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 9)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalKill` maps to SIGKILL, signal 9.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

### surface/ffi-io/process-signal-map-user1

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalUser1`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 10)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalUser1` maps to SIGUSR1, signal 10.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

### surface/ffi-io/process-signal-map-user2

- promise class: normative compatibility vector.
- spec: §1.11.4.
- given: Use a fresh common-fixture client. At `watcher-ready`, call
  `signal_child ChildSignal.SignalUser2`; read `target-exited`, then call
  `wait_child` before the body returns.
- expect: `signal_child` returns `Ok Unit`; `wait_child` returns
  `Ok (ChildExit.Signaled 12)`.
- measured: the signal result, exit marker, and wait result.
- claimed: `SignalUser2` maps to SIGUSR2, signal 12.
- gap: the pidfd marker witnesses target exit only; no syscall path is asserted.
- why: a wrong mapping gives a different exit signal or no marker; a no-op
  leaves the target blocked and fails the bounded run.

For repeatability, the harness supplies a separate absolute-path executable,
`px10-child-signal-counter`. It installs a `SIGUSR1` handler and unblocks that
signal before emitting `counter-ready`. Each handled delivery emits one marker,
`signal-one` or `signal-two`; after the second marker it waits for stdin EOF,
then exits 0. It writes no other stdout data.

### surface/ffi-io/process-signal-delivery-is-repeat-observable

- promise class: durable invariant.
- spec: §1.11.4.
- given: Use the counter fixture with the common pipe and buffer setup. Read
  `counter-ready`; call `signal_child ChildSignal.SignalUser1`, read
  `signal-one`, call it again, and read `signal-two`. Close stdin and call
  `wait_child`, all before the body returns.
- expect: both signal calls return `Ok Unit`; the markers arrive in order; the
  wait returns `Ok (ChildExit.Exited 0)`.
- measured: two public signal results, two fixture acknowledgments, and the
  final wait result while the child resource remains held.
- claimed: two `SignalUser1` deliveries are separately observable, so repeating
  `SignalChild` can repeat an external effect.
- gap: the markers are fixture behavior, not a syscall trace or assertion about
  Ken's implementation.
- why: the first marker is consumed before the second send, so signal
  coalescing is not a confound. A suppressed or deduplicated second delivery
  cannot produce `signal-two`.

The first three rows cover `SignalTerminate` at the live, exited-but-unwaited,
and cached-waited lifecycle points. The six mapping rows cover the other
constructors, and the counter row observes repeated `SignalUser1` delivery.
These cases do not cover descriptor-limit or SIGCHLD mint-failure repair,
revocation, auto-reap, or post-settlement use.
