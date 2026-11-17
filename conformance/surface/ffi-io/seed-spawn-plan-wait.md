# Process spawn and wait conformance — PX10 seed cases

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.11,
child `f5e714239eb27a3ffd12ba31207fbfc4f0900b3d`.
Frame: `docs/program/issues/PX10-SPAWN-PLAN-WAIT.md`.

Section 1.11 specifies interpreter behavior and the native Process
availability boundary. These are acceptance cases, not execution evidence.
The seed has not been Ken-checked or run by the official conformance runner.
No CI, publication, landing, or trust measurement is claimed.
The cases describe surface clients using only the public forms in §1.11 and
related existing buffer contract; they do not freeze host syscall mechanics.
This update adds only Strict import-home cases for the three existing pipe
names; the existing §1.11.2 rows continue to cover their behavior. These
clients use the exact signatures, including the Auth/type parameters,
`withChild`'s `Cap a`, plan, and child-resource body, and each operation's
specified typed resource arguments. No second Process capability argument or
raw descriptor form is introduced.

Source clients use `ChildInherit ConsoleStream` and the qualified
`ConsoleStream.Stdin`, `ConsoleStream.Stdout`, and `ConsoleStream.Stderr`
values. They select the existing checked stream family. The prelude retains
its internal `ChildInherit Stream` declaration; no second family or
source-visible `Stream` alias is introduced.

## Test fixtures

The harness supplies deterministic child executables by absolute path. They
are test inputs, not Ken operations. Except where a case varies authority,
spawn clients declare `Process` and `FS AFull`. Spawn and wait use
`visits [Process]`; pipe clients that use `withBuffer` use `[FS, Process]`.

- `px10-argv-env` writes its received `argv[0]`, `argv[1]`, and selected
  environment values as the record below, then exits 0. It writes `<unset>` for
  an absent key. Record order is fixture-defined, not environment iteration
  order.

  ```text
  argv0=caller-zero
  argv1=payload
  selected=chosen
  replaced=<unset>
  parent-only=<unset>
  ```

- `px10-self-sigterm` sends SIGTERM to itself.
- `px10-exit-23` exits with status 23 and writes no output.
- `px10-default-probe` reads stdin to EOF, checks whether
  `PX10_PARENT_ONLY` is absent, writes a marker to stdout and stderr, and exits
  0 iff stdin was empty and that key was absent.
- `px10-read-until-eof` copies all stdin bytes to stdout, writes a fixed
  marker to stderr, and exits 0 at EOF.
- `px10-close-stdin` closes its stdin, announces readiness on stdout, then
  stays alive until released by the harness.
- `px10-stop-then-exit` stops itself and exits 0 after the harness resumes it.
- `px10-held` writes a fixed readiness record to stdout, then blocks reading
  stdin. It opens no other descriptors while blocked; the harness releases it
  after inspecting the live child.
- `px10-exit0-silent` exits 0 without writing to stdout or stderr.
- `px10-mark-then-exit` writes `started\n` to a marker path in `argv[1]`,
  writes `pipe-ready\n` to stdout, and exits 0. The harness observes the
  child-side marker independently of the Ken body.

## Plan construction and initialization

### surface/ffi-io/process-spawn-plan-builders-are-pure

- promise class: durable invariant.
- spec: §1.11.1.
- given: a pure `fn` builds a plan with `spawn_plan` and transforms it with
  `with_environment`, `with_stdin`, `with_stdout`, and `with_stderr`. It does
  not call `withChild`.
- expect: the client is accepted without a Process effect row or process
  capability. The builders produce an inert immutable `SpawnPlan`; they do not
  spawn a child or create a host resource. Spawning uses the separate
  `withChild` acquisition route.
- why: plan construction is pure and does not itself exercise process
  authority. The test does not invent a constructor or projection for the
  private plan representation.

### surface/ffi-io/process-spawn-plan-is-not-a-public-host-resource

- promise class: durable invariant.
- spec: §1.11.1–§1.11.2.
- given: a consumer attempts to construct `SpawnPlan` from positional plan
  fields or to project a host plan resource from a value returned by a public
  builder. The control constructs and transforms the plan with the named public
  builders.
- expect: source has no direct plan constructor or host-resource projection;
  the direct attempts fail, while the builder control succeeds. Do not pin a
  diagnostic or invent a name for the private constructor.
- why: `SpawnPlan` is an immutable pure value, not a resource token. The
  negative is stated at the specified privacy boundary without guessing a
  surface form that the contract does not name.

### surface/ffi-io/process-spawn-plan-defaults-are-empty-and-null

- promise class: durable invariant.
- spec: §1.11.1.
- given: run `px10-default-probe` from a `spawn_plan` with no environment or
  stdio setters. The parent environment contains `PX10_PARENT_ONLY=parent`;
  parent stdin is nonempty, and parent stdout and stderr are captured.
- expect: the child sees empty stdin and no parent-only environment key,
  returns `Exited 0`, and neither marker reaches the parent's captured streams.
  The plan defaults to an empty environment and `ChildNull` for all three
  child streams.
- why: the parent values make inheritance observable. The AC-1 stdout-pipe
  case below is a positive control that the fixture can emit bytes and the
  parent can receive them.

### surface/ffi-io/process-spawn-uses-the-given-path

- promise class: durable invariant.
- spec: §1.11.1 and §1.11.3.
- given: the harness places `px10-argv-env` only in a directory named by the
  plan's `PATH` entry; the working directory has no file with that name. Use
  the bare name as `program`. The `withChild` body would write a marker if
  called. The control uses the same executable by its absolute path.
- expect: the bare-name spawn does not search `PATH`; it returns
  `ResourceHostIO IOError.NotFound`, creates no child, and skips the body. The
  absolute-path control succeeds and reaches the body marker.
- why: the same fixture and arguments isolate direct path execution from PATH
  lookup. The refusal is pinned at the typed error identity, not diagnostic
  text.

### surface/ffi-io/process-spawn-invalid-plan-values-refuse

- promise class: durable invariant.
- spec: §1.11.1 and §1.11.3.
- given: start from a valid absolute-path plan with nonempty argv and one valid
  environment entry. In separate runs, change exactly one field: make argv
  empty; insert NUL into the program path, an argument, an environment key, or
  an environment value; make the key empty; or make it contain `=`. Pair each
  malformed value with the corresponding valid neighbor and a child that
  records whether it ran.
- expect: every malformed plan returns `ResourceHostIO IOError.InvalidInput`
  before the child starts. Empty argv is rejected at spawn. Each valid neighbor
  reaches the child. Do not pin where NUL/key validation occurs before spawn.
- why: each negative changes one field, and its neighbor proves the same
  program, capability, and fixture work. The public builders do not expose a
  raw descriptor map, so host-side duplicate/invalid descriptor entries are
  not manufactured by a source client.

## AC-1: child output and typed wait results

### surface/ffi-io/process-spawn-passes-argv-and-environment

- promise class: durable invariant.
- spec: §1.11.1–§1.11.3; frame AC-1.
- given: the parent environment contains `PX10_PARENT_ONLY=parent`. Build a
  plan for the absolute `px10-argv-env` path with arguments
  `[b"caller-zero", b"payload"]`. Set the plan environment to
  `PX10_REPLACED=old`, then replace it with only `PX10_SELECTED=chosen`.
  Select `ChildPipe` for stdout. In the `withChild` body, take stdout once and
  read it through `read_pipe` using a `BufferHandle` and admitted
  `BufferWindow`, collecting each `ReadSome` span until `ReadEof`. Wait with
  `wait_child`. The program declares Process and `FS AFull`; the pipe client
  has the combined `[FS, Process]` row.
- expect: the collected bytes equal the fixture record above; wait returns
  `Exited 0`. The caller-supplied first argument is `argv[0]`. The final
  environment contains the selected key and neither the replaced key nor the
  parent's sentinel. The checked interpreter yields the expected bytes and
  `Exited 0`; native behavior is addressed separately below.
- why: one accepted child pins argv, exact environment replacement, no parent
  environment overlay, a stdout `PipeRead`, bounded pipe reads, and normal
  exit. It does not pin read chunk sizes or environment iteration order.

### surface/ffi-io/process-spawn-reports-signal-exit

- promise class: normative compatibility vector.
- spec: §1.11.1–§1.11.2; frame AC-1.
- given: run `px10-self-sigterm` through `withChild` and wait. Use
  `px10-argv-env` and `px10-exit-23` as normal-exit controls. Call
  `wait_child` twice on each still-live child resource.
- expect: the signaled child returns `Signaled 15` on both waits; the controls
  return `Exited 0` and `Exited 23` on both waits. Each successful result is
  cached. The closed `ChildExit` sum has only `Exited UInt8` and
  `Signaled UInt8`; stops
  and continues are not wait results.
- why: the pair observes both specified constructors and verifies repeat wait
  does not change the cached result. No exit status is inferred from a signal
  number.

### surface/ffi-io/process-spawn-waits-through-stop-and-continue

- promise class: durable invariant.
- spec: §1.11.2.
- given: run `px10-stop-then-exit` through `withChild` and call `wait_child`.
  The harness observes the child stopped in `/proc` before sending SIGCONT.
- expect: `wait_child` does not return a stopped or continued state; it blocks
  until termination and returns `Exited 0`.
- why: the input reaches the intermediate states that are explicitly excluded
  from `ChildExit`, while the final result has an ordinary-exit control.

## Child stdio, pipes, and process authority

### surface/ffi-io/process-spawn-stdio-targets-are-closed

- promise class: normative compatibility vector.
- spec: §1.11.1.
- given: a client exhaustively matches `ChildStdio` using `ChildPipe`,
  `ChildNull`, and `ChildInherit ConsoleStream`, with no catch-all. A separate
  negative client supplies an integer descriptor to `ChildInherit`; its
  positive control supplies a parent `ConsoleStream` value. The routing run
  selects `ConsoleStream.Stdin`, `ConsoleStream.Stdout`, and
  `ConsoleStream.Stderr` with distinct input and captured output on each
  parent stream.
- expect: the exhaustive client is complete for the three specified choices;
  the integer target is rejected by the `ConsoleStream` type and the positive
  control is accepted. Inherited child stdin reads the parent's input;
  inherited child stdout and stderr reach their corresponding parent streams.
  `ChildNull` connects only the selected child stream to the null device. No
  arbitrary descriptor value is accepted as a `ChildInherit` target.
- why: the three routing choices are tested against distinct parent streams;
  a swapped or raw-descriptor route differs observably. The case is about the
  public type and behavior, not a source-text inventory.

### surface/ffi-io/process-spawn-child-is-not-parent-fs-sandboxed

- promise class: durable invariant.
- spec: §1.11.2.
- given: the program declares Process and `FS AFull`. The `Cap a` passed to
  `withChild` is attenuated so its root excludes a readable sentinel file but
  permits the absolute child-fixture path. The parent tries its ordinary FS
  read of the sentinel as a denied control. It then spawns the fixture and
  passes the sentinel's absolute path in argv; the host OS user can read it.
- expect: the parent's FS operation remains denied, while the child reads and
  returns the sentinel bytes. The Process grant authorizes the spawn; the
  parent's FS root is not installed as a child sandbox.
- why: the same host file and OS identity distinguish Ken's parent capability
  confinement from the child process's OS authority.

### surface/ffi-io/process-spawn-pipe-ends-are-typed-and-taken-once

- promise class: durable invariant.
- spec: §1.11.1–§1.11.2.
- given: select `ChildPipe` for stdin, stdout, and stderr, then use
  `take_stdin`, `take_stdout`, and `take_stderr` in the `withChild` body. In
  separate clients, pass a `PipeWrite` to `read_pipe` and a `PipeRead` to
  `write_pipe`. For each `take_*` operation, test a selected end twice and
  test `ChildNull` and `ChildInherit` as unselected targets.
- expect: `take_stdin` yields `Resource ResourceKind.PipeWrite`;
  `take_stdout` and `take_stderr` yield `Resource ResourceKind.PipeRead`. The
  wrong read/write types fail type checking. A repeated or unselected take
  returns
  `ResourceHostIO IOError.InvalidInput`; the first take of each selected pipe
  is the accepted control.
- why: direction and single-use behavior are isolated independently. The
  typed API prevents raw fd selection; the accepted first take prevents a
  broken resource setup from masquerading as the specified refusal.

### surface/ffi-io/process-child-pipe-operations-have-explicit-home

- promise class: durable invariant.
- spec: §1.11.1–§1.11.2; §1.12.
- given: in a fresh pre-source environment, snapshot the existing
  `GlobalId` for each name before loading any catalog source. Run the
  type-correct pipe clients from `process-spawn-pipe-progress-and-early-close`
  in Strict isolated source. In the full-import control, import
  `write_pipe`, `close_pipe_write`, and `close_pipe_read` from
  `Capability.Process.Child` and use each at its specified type. Keep this
  complete non-target import set fixed in that control:
  `ChildPipe`, `spawn_plan`, `with_stdin`, `with_stdout`, `with_stderr`,
  `withChild`, `wait_child`, `take_stdin`, `take_stdout`, `take_stderr`, and
  `read_pipe` from `Capability.Process.Child`; `withBuffer` from
  `Capability.System.Resource`; `BufferHandle`, `BufferWindow`, and
  `MkBufferWindow` from `Capability.System.Buffer`; and `readAt` and
  `spanBytes` from `Capability.System.IO`. `take_stdin` supplies `PipeWrite`,
  while `take_stdout` and `take_stderr` supply `PipeRead`.
- expect: the client typechecks and each of the three target names resolves
  to its pre-source identity: `write_pipe` and `close_pipe_write` take
  `Resource ResourceKind.PipeWrite`, while `close_pipe_read` takes
  `Resource ResourceKind.PipeRead`.
- measured: the early-close client's uses and the three resolved
  `GlobalId`s, compared with the independent pre-source snapshot.
- claimed: the home adds an explicit import path, not a new declaration
  or operation.
- gap: this case does not measure the operations' runtime behavior.
- why: an accepted source alone would also pass if each package export
  were copied into a new declaration; checking identity distinguishes
  that from the specified home.

### surface/ffi-io/process-child-pipe-operations-require-explicit-import

- promise class: durable invariant.
- spec: §1.11.1–§1.11.2; §1.12.
- given: use the same type-correct pipe clients from
  `process-spawn-pipe-progress-and-early-close`. The full-import control
  imports `write_pipe`, `close_pipe_write`, and `close_pipe_read` from
  `Capability.Process.Child`, plus the same
  complete non-target set in every run: `ChildPipe`, `spawn_plan`,
  `with_stdin`, `with_stdout`, `with_stderr`, `withChild`, `wait_child`,
  `take_stdin`, `take_stdout`, `take_stderr`, and `read_pipe` from
  `Capability.Process.Child`; `withBuffer` from `Capability.System.Resource`;
  `BufferHandle`, `BufferWindow`, and `MkBufferWindow` from
  `Capability.System.Buffer`; and `readAt` and `spanBytes` from
  `Capability.System.IO`. In separate runs, remove only the `write_pipe`,
  `close_pipe_write`, or `close_pipe_read` import while leaving its use and
  every other import unchanged.
- expect: each run refuses with `UnboundName` naming only the omitted
  name. Its matching full-import client is the accepting control.
- measured: three individually omitted-import refusals and three
  corresponding accepting imports of those names.
- claimed: each homed pipe operation requires its explicit package
  import in Strict isolated source.
- gap: this does not pin Legacy-root resolution or CLI routing.
- why: every refusal changes only its own import; the paired accepted
  source establishes the name and its use are otherwise valid.

### surface/ffi-io/process-spawn-pipe-progress-and-early-close

- promise class: durable invariant.
- spec: §1.11.2 and `38 §1.7.1–§1.7.2`.
- given: `px10-argv-env` writes the fixed record above to stdout and exits.
  The parent takes stdout and uses `read_pipe` with a `withBuffer`-owned
  `BufferHandle` and two-byte `BufferWindow`s, converting each returned span
  with `spanBytes`. In a second run, `readAt` reads a nonempty prefix of
  `b"payload"` into a `BufferSpan` from the same buffer acquisition;
  `spanBytes` confirms and saves that prefix. The parent takes the child's
  stdin with `take_stdin`, writes the span, takes stdout and stderr, and calls
  `close_pipe_write` on the write end. It reads both output pipes to EOF before
  waiting. `px10-read-until-eof` reads stdin until EOF, echoes to stdout, and
  writes a fixed marker to stderr. For `px10-exit0-silent`, the parent takes
  stdout and calls `close_pipe_read`.
- expect: each `ReadSome span count` has a positive count bounded by the
  admitted window and a span of that count; repeated reads reconstruct the
  child's exact bytes and end in `ReadEof`. `write_pipe` returns `Wrote count`
  with `0 < count ≤ span length` for the actual bytes written, not a promise to
  complete the span. The parent reads the child's stdout and stderr to EOF;
  stdout equals the prefix of the saved `spanBytes` value with that count,
  and stderr
  equals the fixed marker. After
  `close_pipe_write`, the child observes EOF before `wait_child` returns; any
  remaining suffix is the caller's work. Closing the same write end again
  through a copied resource returns `Closed`; the first close is the accepted
  control.
  On `px10-exit0-silent`, `close_pipe_read` succeeds once, the repeat through
  a copied end returns `Closed`, and wait returns `Exited 0`.
- why: counts and spans are checked against the actual transfer, not a fixed
  chunk size. Explicit write-end close makes the stdin-to-EOF behavior
  expressible before wait and detects a wait-before-close deadlock.

### surface/ffi-io/process-spawn-write-closed-pipe-is-broken-pipe

- promise class: durable invariant.
- spec: §1.11.2–§1.11.3.
- given: the parent obtains a `PipeWrite` for child stdin and selects
  `ChildPipe` for stdout. Take stdout and use `read_pipe` to receive the
  `px10-close-stdin` readiness marker, sent after it closes its read end. Then
  write a valid same-buffer `BufferSpan`. The positive control uses
  `px10-read-until-eof`, which remains a reader.
- expect: the closed-reader case returns
  `ResourceHostIO IOError.BrokenPipe`; the Ken edge remains alive rather than
  terminating by SIGPIPE. The reading control accepts a write and returns
  `Wrote` with its actual count.
- why: the controlled pair changes only whether the child still reads. It
  distinguishes typed BrokenPipe handling from a process-level SIGPIPE death.

### surface/ffi-io/process-spawn-requires-process-and-execute

- promise class: durable invariant.
- spec: §1.11.2–§1.11.3; `36 §1.4`.
- given: the accepted program declares `program capabilities Process` and
  `program capabilities FS AFull`; a spawn/wait `proc` uses
  `visits [Process]`. Controls use the same call with only `[FS]` in the row,
  with only the FS AFull program grant, and with the Process grant but a
  `Cap a` attenuated to omit EXECUTE. A fourth program declares Process
  without `FS AFull`.
- expect: the accepted program starts its child. The `[FS]`-only row fails the
  existing effect-escape check. FS AFull without the Process grant, and an
  attenuated cap without EXECUTE, return
  `ResourceHostIO IOError.CapabilityDenied` before body or child start.
  Admission rejects
  Process without `FS AFull`, without pinning an unspecified diagnostic. Spawn,
  wait, and pipe operations use `[Process]`, not `[FS]`; a pipe client that
  uses `withBuffer` has `[FS, Process]`.
- why: the full-grant client controls the capability and host route. Separate
  controls isolate effect-row admission, program grant, and attenuated EXECUTE.

## Bracket settlement and error classification

### surface/ffi-io/process-spawn-bracket-releases-before-wait

- promise class: durable invariant.
- spec: §1.11.2.
- given: select `ChildPipe` for stdin and run `px10-read-until-eof`, which
  blocks until stdin reaches EOF. The `withChild` body returns without taking
  or closing the stdin end and without calling `wait_child`.
- expect: bracket settlement releases taken and untaken pipe ends before
  reaping the child. The child observes EOF, exits, and is reaped; no child
  remains after bracket completion.
- why: if settlement waits before releasing the parent's write end, the child
  cannot observe EOF and the bracket cannot complete. The public case pins the
  observable close-before-reap behavior, not the internal release ordering.

### surface/ffi-io/process-spawn-terminal-error-kills-and-reaps

- promise class: durable invariant.
- spec: §1.11.2.
- given: select child stdin and stdout pipes for `px10-held`; its body reads
  the ready marker, then propagates a terminal error before waiting.
- expect: terminal-error finalization kills and reaps the unreaped child; no
  zombie or orphan remains. The bracket result preserves the body error rather
  than replacing it with successful cleanup.
- why: the live-child precondition makes cleanup observable. Normal settlement
  is covered separately by the stdin-to-EOF case.

### surface/ffi-io/process-spawn-bracket-keeps-four-outcomes-distinct

- promise class: durable invariant.
- spec: §1.11.2.
- given: a client consumes `withChild` as
  `Result ResourceError (ResourceBracketResult e r)` and distinguishes body
  success/error from release success/error. Include a pre-body spawn failure,
  a successful body, and a terminal body error.
- expect: a spawn failure is the outer `ResourceError` and skips the body.
  After acquisition, the existing four-way bracket result keeps body and
  release outcomes distinct; no process-specific bracket error type replaces
  it. The success and body-error paths are observed in the adjacent cases.
- why: this pins the specified result boundary without inventing names for
  existing bracket constructors or release-failure triggers.

### surface/ffi-io/process-spawn-revocation-does-not-signal-child

- promise class: durable invariant.
- spec: §1.11.2–§1.11.4; `62` authority.
- given: Run paired clients with `ChildPipe` selected for stdin, stdout, and
  stderr, using the same `px10-held` fixture and plan. The harness verifies
  default, unblocked `SIGTERM`. Both clients take stdout and read the readiness
  record while the child is live. In the unrevoked control,
  `signal_child ChildSignal.SignalTerminate` succeeds and `wait_child` returns
  `Ok (ChildExit.Signaled 15)`. In the revoked run, leave stderr
  untaken. Before the Ken client calls `signal_child`, the harness applies
  the existing §62 host-management revocation action to the originating
  grant; the Ken client does not call `revoke`. Then call
  `signal_child ChildSignal.SignalTerminate` on the first
  child. Also
  attempt another `withChild` spawn using that grant,
  `wait_child`, `take_stderr`, and `read_pipe` on held stdout. The harness
  observes the first child before releasing it.
- expect: the control signal returns `Ok Unit` and wait returns
  `Ok (ChildExit.Signaled 15)`. In the revoked run, `signal_child` returns
  `Err (ResourceHostIO IOError.Revoked)`; each other later plan, child, and
  pipe operation returns `ResourceHostIO IOError.Revoked`. The first child
  remains live until controlled release. After bracket
  settlement, use of an escaped resource copy returns `Closed`.
- measured: default and unblocked `SIGTERM`, the control signal and wait
  results, all revoked operation results, and the revoked child's liveness
  before release.
- claimed: revocation blocks `SignalChild` without delivering a signal; the
  same fixture and operation deliver `SignalTerminate` before revocation.
- gap: child liveness is the no-delivery witness for this SIGTERM-sensitive
  fixture; no host syscall or resource-admission implementation is asserted.
- why: the paired `signal_child` runs differ only by grant revocation. A path
  that signals before returning `Revoked` fails the live-child observation.

### surface/ffi-io/process-errors-use-existing-identities

- promise class: durable invariant.
- spec: §1.8 and §1.11.2–§1.11.4.
- given: separately induce ENOENT with a missing absolute path; EACCES or EPERM
  with a non-executable file and a valid executable control; ENOEXEC with an
  executable invalid image and a valid executable control; an empty-argv or
  malformed-byte plan; a request without EXECUTE; a write to a closed child
  pipe; and ECHILD under the stated SIGCHLD residual. For E2BIG, compare a
  well-formed oversized argv/environment with an under-limit control. For
  ETXTBSY, compare an executable held open for writing with the same file
  after the writer closes. Also apply `process_error_to_system` to an
  exhaustive `ProcessOperation` match with one arm and no catch-all for each
  of `SpawnChild`, `TakePipe`, `ReadPipe`, `WritePipe`, `ClosePipe`,
  `WaitChild`, and `SignalChild`, with both a known program value and `None`,
  using `IOError.Other errno` as one cause. Query
  `operation_idempotence (ProcessOp WaitChild)` and
  `operation_idempotence (ProcessOp SignalChild)`, then pass both results to
  `retry_guidance` with the same `Transient` input.
- expect: host causes remain nested as `ResourceHostIO IOError`: ENOENT is
  `NotFound`, EACCES/EPERM is `PermissionDenied`, ENOEXEC is `Other` carrying
  that errno, malformed input is `InvalidInput`, missing EXECUTE is
  `CapabilityDenied`, EPIPE is `BrokenPipe`, revoked resources retain
  `Revoked`, and ECHILD is `Other 10`.
  Other host failures remain `Other errno`, including E2BIG and ETXTBSY when
  the OS reports them. The catalog lift preserves the selected `ProcessOp`,
  known `ProcessResource (Some programBytes)`, original cause, and
  `NoSafeContext`; `None` remains `None`. `Closed`, `ResourceKindMismatch`,
  and `InvalidBounds` remain resource-discipline errors, not SystemError
  causes; the lift accepts only `IOError`. No new
  `SpawnError` or `Revoked` identity, raw pid, or fd is exposed.
  `operation_idempotence (ProcessOp WaitChild) = Idempotent` and
  `operation_idempotence (ProcessOp SignalChild) = NonIdempotent`. For the
  shared `Transient` input, their retry guidance is respectively
  `RetryAdvised` and `RetryUnsafeNonIdempotent`.
- why: each cause has a controlled input; the valid-path and open-pipe cases
  above are accepted controls. The exact existing identities are pinned, not
  message text. The exhaustive operation match forces a new arm for any
  added variant. `WaitChild` is the positive control for the idempotence pair:
  a `SignalChild = Idempotent` misclassification would advise retry for both
  operations on the shared `Transient` input, so the expected verdict split
  would fail.

## AC-2: live-child descriptor and signal boundary

### surface/ffi-io/process-spawn-closes-unselected-parent-descriptors

- promise class: durable invariant.
- spec: §1.11.1 and §1.11.3; frame AC-2.
- given: before spawn, the harness verifies the spawning process holds
  non-CLOEXEC fd 9. The program starts `px10-held` with child stdin and stdout
  pipes. Its `withChild` body uses `take_stdout`, reads the readiness record
  with `read_pipe` using a `BufferHandle` and `BufferWindow`, leaves stdin
  open, then calls `wait_child`. The harness inspects `/proc/<child>/fd` while
  that wait blocks.
- expect: the child holds exactly descriptors `{0, 1, 2}`. Fd 9 and every
  other unselected parent descriptor are absent. Release the child only after
  measuring that live state.
- why: the verified parent-fd precondition makes close-on-exec alone
  insufficient. `/proc` observes the child, not a plan or a source assertion.

### surface/ffi-io/process-spawn-resets-sigpipe-and-inherits-mask

- promise class: durable invariant.
- spec: §1.10 and §1.11.3; frame AC-2.
- given: the harness establishes and verifies that the spawning thread ignores
  SIGPIPE, inherits ignored SIGHUP, has a caught SIGUSR2 action, and has the
  controlled mask `{SIGUSR1}`. Run the same `withChild` program and wait shape
  as the descriptor case; inspect `SigIgn`, `SigCgt`, and `SigBlk` in
  `/proc/<child>/status` while `wait_child` blocks.
- expect: SIGPIPE is absent from both `SigIgn` and `SigCgt` (`SIG_DFL`);
  inherited ignored SIGHUP remains in `SigIgn`; caught SIGUSR2 is absent from
  both fields after exec. The child's `SigBlk` equals the spawning thread's
  controlled mask `{SIGUSR1}`. Disposition and mask assertions are independent.
- why: the measured parent state is non-degenerate on both axes. Resetting
  SIGPIPE does not authorize clearing the mask; inheriting the mask does not
  excuse leaving SIGPIPE ignored.

### surface/ffi-io/process-spawn-ignored-sigchld-residual

- promise class: durable invariant.
- spec: §1.11.3.
- given: the launcher ignores SIGCHLD before a child is spawned.
- expect: Ken does not reset SIGCHLD or promise a successful wait. If the OS
  auto-reaps the child and `wait_child` receives ECHILD, the result is
  `ResourceHostIO (IOError.Other 10)`. A successful wait is also permitted.
- why: this records the specified environment-dependent residual; it does
  not require SIGCHLD repair or hide the existing errno.

### surface/ffi-io/process-spawn-unavailable-descriptor-refuses-before-child

- promise class: durable invariant.
- spec: §1.11.3.
- given: use one checked `withChild` client and `px10-mark-then-exit` in a
  supported Linux environment and an environment where the OS lacks the
  process-descriptor support required by §1.11.3. The harness independently
  verifies the unsupported environment. On that host, run the fixture directly,
  check its marker and stdout, then remove the marker. The body in each Ken run
  reads `pipe-ready\n` and returns a distinct value. The harness traces child
  creation and marker state.
- expect: the supported control reaches the body, reads `pipe-ready\n`, and
  completes with `ChildExit.Exited 0`; the child marker is present. Without
  descriptor support, `withChild` returns outer
  `Err (ResourceHostIO IOError.Unsupported)` before the body; there is no
  child creation or child marker. A different error does not satisfy the case.
- measured: the host support probe, exact `withChild` result, body result,
  process-creation trace, and marker state on both environments.
- claimed: unavailable descriptor support refuses before child creation and
  skips the body, rather than falling back to a process number.
- gap: the process-creation trace must cover all children of the Ken process;
  marker absence alone would miss a spawned child that never execs. The
  supported twin and direct fixture run control the client and executable.
- why: the paired result and process trace distinguish a correct refusal from
  an unconditional `Unsupported`, a bad executable, or a post-spawn refusal.

### surface/ffi-io/process-spawn-auto-reaped-child-settles-unsignalled

- promise class: durable invariant.
- spec: §1.11.3.
- given: the launcher ignores SIGCHLD. Run a client using
  `px10-mark-then-exit` with stdout selected. Its body reads the buffered
  `pipe-ready\n` bytes after child exit, calls `wait_child`, reports the result
  to the harness, and waits at a harness-controlled barrier before returning
  a controlled terminal body error. A default-SIGCHLD twin with the same body
  observes the bytes and `ChildExit.Exited 0`. In the ignored-SIGCHLD run, the
  harness confirms auto-reap and `Other 10`; after the report, it reuses the
  child's former numeric process identifier for a live decoy before allowing
  terminal settlement. Reuse occurs only after body entry, outside the
  pre-acquisition residual. The decoy records catchable signals and remains
  held; a liveness monitor detects disappearance or stop. Separate controls
  verify signal logging and termination detection. The existing live-child
  terminal-error case controls cleanup of an unreaped child.
- expect: spawn succeeds, the body runs, the child marker is present, and the
  buffered pipe bytes remain readable. `wait_child` returns
  `Err (ResourceHostIO (IOError.Other 10))`. After terminal settlement, no
  signal is recorded and the same-numbered decoy remains running.
- measured: auto-reap, buffered bytes, `Other 10`, PID reuse after the body
  reports ECHILD, empty signal log, and decoy running state after settlement.
- claimed: the auto-reaped child remains observable through its body and pipes,
  while terminal settlement does not signal the reused process.
- gap: PID reuse must be confirmed after ECHILD and before settlement, and
  the matched decoy control must prove the liveness observer is active;
  otherwise the no-signal property is unmeasured. This does not pin the
  acknowledged pre-acquisition reuse residual.
- why: a stale-number signal would terminate the reused decoy. This case
  observes public results and process liveness, not the host's identity
  mechanism or a specific signal syscall.

## Native Process availability

### surface/ffi-io/process-native-build-refuses-before-object-emission

- promise class: transition sentinel.
- spec: §1.11 status and §§1.11.2–1.11.3.
- given: request a native build of the exact checked program in
  `surface/ffi-io/process-spawn-passes-argv-and-environment`. Use the same
  `px10-argv-env` fixture with no source changes and a fresh output
  destination with no preexisting artifact. Its interpreter case is the
  positive control. The program
  declares `Process` and `FS AFull`; its pipe client uses `[FS, Process]`.
- expect: the interpreter control accepts with the specified bytes and
  `Exited 0`. Native build stops before object emission with a typed
  availability refusal naming the operations that remain
  `RepresentedUnavailable`; no native artifact is emitted. An unrelated
  parse, type, or build failure does not satisfy this case.
- why: the accepted interpreter run controls source validity. Native Process
  remains unavailable in this slice; an emitted object would overclaim parity.
  The transition ends when `PX10-NATIVE-SPAWN-PARITY` lands. That successor
  must replace this refusal with native Process observations and update this
  seed's opening status paragraph. This row is a contract case, not executed
  by Ken or the official runner.

## Coverage boundary

These cases pin the public plan, Process grant, pipe, wait, error, descriptor,
signal, and native-availability behavior in §1.11, including AC-1 and AC-2.
The host-side invalid or duplicate descriptor-entry check has no public input
because builders expose no descriptor map. The exact reverse order among
internal release actions and the absence of a post-fork callback are not
source-observable; this seed pins close-before-reap behavior without claiming
to test those implementation properties. The added rows also pin the
no-descriptor pre-spawn refusal and the auto-reaped child's observable wait and
settlement behavior without exposing the host's identity mechanism. Explicit
pidfd signalling and the other successor attachments—cwd, credentials,
namespaces, cgroups, rlimits, process groups, and non-blocking wait—remain
outside this slice.
