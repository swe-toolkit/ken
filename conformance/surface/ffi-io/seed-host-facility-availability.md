# Host-facility availability through a Ken process

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.8 property (3), as routed in
ABI-M2 AC-0 (`evt_3as7r55jkc0at`). These Linux host-boundary cases exercise
the public Unix-listener operation through checked Ken clients. A supervisor
helper checks facility state in the target process's mount and PID namespaces,
not through the Ken filesystem path API or the host namespace. They do not
claim that Ken source can change host facilities.

## surface/ffi-io/unix-listen-reports-missing-proc-facility

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run the same checked client in otherwise equivalent Linux user,
  mount, and PID namespaces. It declares `Net Unix`, receives an `FS AFull`
  grant rooted at `/`, and calls the public `with_socket_listener` path for a
  Unix address at a fresh absent socket leaf under a pre-created writable
  directory beneath that root. Before the listener request, a supervisor
  helper runs in the target's mount and PID namespaces, opens `/` with
  `O_PATH`, and uses `readlinkat` on `/proc/self/fd/<fd>`. The
  control namespace mounts procfs at `/proc`; the test namespace mounts an
  empty tmpfs there. Do not use `withDirectory` or inspect the path from
  outside the target namespaces.
- expect: the helper's `readlinkat` returns the descriptor target in the
  procfs control and `ENOENT` in the empty-tmpfs profile. The procfs listener
  returns success, enters the body, and creates its socket path. In the test
  namespace,
  `with_socket_listener` returns exactly
  `Err (ResourceHostIO IOError.Unsupported)`, does not enter the body, and
  leaves the socket path absent. It does not return `CapabilityDenied`,
  `PermissionDenied`, or a successful listener.
- measured: the supervisor helper's exact `readlinkat` result, the checked
  Ken listener result, body-entry marker, and socket-path existence in both
  profiles.
- claimed: absence of the named facility reaches the public Unix-listener
  operation as `Unsupported` before its socket effect.
- gap: this case covers `/proc/self/fd` only. It does not cover statx,
  accept4, or any stream-socket family, and does not assert those facilities
  are absent on the test host.
- controls: the procfs-mounted run uses the identical client, grant, address,
  and runner configuration except for the `/proc` mount. A fresh path per run
  prevents a prior socket from masking whether this operation created one.
- why: the helper measures `/proc/self/fd` in the target namespaces without
  using Ken's no-follow filesystem path API. The successful procfs listener
  proves that the public Ken route can use the facility; the typed refusal
  and absent path distinguish it from an unconditional refusal or a post-bind
  refusal.

## surface/ffi-io/unix-listen-caches-first-procfd-observation

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run one checked client in a supervisor-managed Linux user, mount,
  and PID namespace. It declares `Net Unix`, receives an `FS AFull` grant
  rooted at `/`, and uses the public `with_socket_listener` route. Use fresh
  absent socket leaves under a pre-created writable directory. Start with an
  empty tmpfs at `/proc`. Before the first listener request, a supervisor
  helper in the target's mount and PID namespaces opens `/` with `O_PATH` and
  uses `readlinkat` on `/proc/self/fd/<fd>`. The first
  listener request uses a fresh path. After that result, the supervisor mounts
  procfs over `/proc` in the same mount namespace. A helper in the target
  namespaces repeats the `O_PATH`/`readlinkat` check and records the descriptor
  target. The same Ken process then requests a listener at a second fresh path.
  Keep the process, grant, client, and namespace unchanged.
- expect: the initial helper `readlinkat` returns `ENOENT`; after the mount
  transition, the helper reads the descriptor target. The first listener
  request returns exactly `Err (ResourceHostIO IOError.Unsupported)`, does
  not enter the body, and leaves its path absent. The second request in that
  process returns the same result with no body entry or path creation,
  despite the now-available procfs; the first decision is cached for the
  process.
- measured: both supervisor-helper `readlinkat` results, both exact listener
  results and body markers, both socket-path states, and the mount transition
  in the target process's mount namespace.
- claimed: a facility absence found on first need remains the process's
  availability decision; a later need does not re-probe and reverse it.
- controls: a fresh process starting with procfs mounted uses the same client
  for both fresh paths; both listener bodies are entered and both paths are
  created. The namespace helper also records a successful `readlinkat` check.
  The earlier absent-facility row requires this public route to succeed with
  procfs present.
- why: if the implementation re-probes after absence, the second request
  succeeds after the supervisor makes procfs available. The helper records the
  mount transition independently of the listener result. The procfs-start
  control rejects an unconditional `Unsupported` implementation. Repeated
  `Unsupported` results without the mount transition are not evidence of a
  per-process cache.

A runner unable to change `/proc` inside the target process's mount namespace
has not executed this row. A local `unshare` supervisor can mount tmpfs and
procfs in one user and mount namespace; this demonstrates the mount-transition
harness setup only. The required namespace-helper `readlinkat` checks and the
Ken listener rows remain unexecuted. The final bwrap process used for the
basic row has no effective mount capability, so this state-transition row
requires a supervisor with mount capability in the namespace.

## surface/ffi-io/unix-listen-retries-after-undecided-probe-error

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run one checked client in a supervisor-managed Linux user, mount,
  and PID namespace with procfs mounted. It declares `Net Unix`, receives an
  `FS AFull` grant rooted at `/`, and uses the public `with_socket_listener`
  route. Before the listener request, a supervisor helper in the target's
  mount and PID namespaces opens `/` with `O_PATH` and uses `readlinkat` on
  `/proc/self/fd/<fd>`. After this helper check, arm a one-shot syscall
  fault injector for the facility probe's link read of `/proc/self/fd/<n>`
  during the listener request. It returns `EAGAIN` for that call only and
  allows later calls. On x86_64 the call is `readlink(2)`; on architectures
  that use it, the call is `readlinkat(2)`. The runner records which syscall
  was faulted and confirms the helper check was not faulted. The
  first listener path has a missing parent. After the result, the Ken client
  creates that parent with its same `FS AFull` grant. The supervisor then
  mounts an empty tmpfs over `/proc` in the target's mount namespace. A helper
  in the target namespaces repeats the `O_PATH`/`readlinkat` check; the same
  Ken process makes a second listener request at a fresh
  path under the created parent.
- expect: the helper's initial `readlinkat` returns a descriptor target.
  The first listener call stops at the facility check and returns exactly
  `Err (ResourceHostIO (IOError.Other EAGAIN))`; on Linux, `EAGAIN`
  is errno 11. It does not continue to the missing-parent check, enter the
  body, or create a socket path. This is neither `NotFound` nor `Unsupported`.
  After the mount transition, the helper's `readlinkat` returns `ENOENT`. On
  the next need, a fresh probe finds `/proc/self/fd` absent. The listener
  returns exactly `Err (ResourceHostIO IOError.Unsupported)` without entering
  the body or creating the second path. Neither faulted call enters the body
  or creates a socket path.
- measured: both namespace-helper `readlinkat` results, the one-shot `EAGAIN`
  fault on the probe's link-read syscall and its recorded syscall name, both
  exact listener results and body markers, the mount transition, and both
  path states.
- claimed: an undecided probe failure stops the current operation at the
  facility check and is reported through the operation's existing error
  mapping. No availability verdict is cached, and the next need probes again.
- controls: with the same client, the fault disabled, and procfs kept mounted,
  the first missing-parent request returns exactly
  `Err (ResourceHostIO IOError.NotFound)`; after the client creates the parent,
  the second listener request succeeds and enters its body. This unfaulted
  control establishes the ordinary missing-parent result. The faulted run's
  first `Other(EAGAIN)` distinguishes the probe error from both continuation to
  `NotFound` and treating `EAGAIN` as absence. Its second `Unsupported`
  rejects replaying or caching the undecided failure and skipping the next
  probe.
- why: the helper checks are outside Ken's no-follow filesystem path API but
  run in the target's mount and PID namespaces. The fault injector must witness
  and alter the probe's link-read syscall, which is `readlink` on x86_64 and
  may be `readlinkat` on another architecture; it must not fault an `open` on
  the same spelling. The faulted run stops at the facility check before the
  parent walk; only the unfaulted control reaches the missing-parent result.
  A mount change before the next need makes failure to re-probe observable:
  it cannot produce the specified `Unsupported`.

This case requires a Linux runner that can inject one transient error into the
facility probe's link read of `/proc/self/fd/<n>` and change `/proc` within the
target's mount namespace. A local `strace` 6.13 path-targeted `EAGAIN`
injection on an `open` of `/proc/self/fd` failed the first open and allowed
the second, but did not target the probe's link read; it is not evidence that
the required fault injection is feasible. A local `unshare` supervisor
changed the same namespace from procfs to tmpfs. These are harness-feasibility
checks, not Ken operation results. The required namespace-helper observations,
faulted link-read, and Ken operations were not executed. No official
conformance execution, CI, publication, landing, or trust measurement is
claimed.
