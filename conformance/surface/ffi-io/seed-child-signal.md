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

These rows cover only `SignalTerminate` at the live, exited-but-unwaited, and
cached-waited lifecycle points. They do not cover descriptor-limit or SIGCHLD
mint-failure repair, revocation, other signal constructors, auto-reap, or
post-settlement use.
