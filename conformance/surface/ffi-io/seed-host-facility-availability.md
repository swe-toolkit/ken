# Host-facility availability through a Ken process

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.8 property (3), as routed in
ABI-M2 AC-0 (`evt_3as7r55jkc0at`). These Linux host-boundary cases observe
`/proc/self/fd` through a checked Ken client. They do not claim that Ken source
can change host facilities.

## surface/ffi-io/unix-listen-reports-missing-proc-facility

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run the same checked client in otherwise equivalent Linux user and
  mount namespaces. It declares `Net Unix`, receives an `FS AFull` grant
  rooted at `/`, and calls the public `with_socket_listener` path for a Unix
  address at a fresh absent socket leaf under a pre-created writable
  directory beneath that root. Before the call, the client uses
  `withDirectory` on `/proc/self/fd` under the same root. The procfs control
  must enter the body; the empty-tmpfs test must observe `IOError.NotFound`.
  The control namespace mounts procfs at `/proc`; the test namespace mounts an
  empty tmpfs there instead. Do not substitute a host-side path check.
- expect: the procfs control returns success, enters the listener body, and
  creates the socket path. In the test namespace,
  `with_socket_listener` returns exactly
  `Err (ResourceHostIO IOError.Unsupported)`, does not enter the body, and
  leaves the socket path absent. It does not return `CapabilityDenied`,
  `PermissionDenied`, or a successful listener.
- measured: the in-process `withDirectory` result, exact checked Ken result,
  body-entry marker, and socket-path existence in both profiles.
- claimed: absence of the named facility reaches the public Unix-listener
  operation as `Unsupported` before its socket effect.
- gap: this case covers `/proc/self/fd` only. It does not cover statx,
  accept4, or any stream-socket family, and does not assert those facilities
  are absent on the test host.
- controls: the procfs-mounted run uses the identical client, grant, address,
  and runner configuration except for the `/proc` mount. A fresh path per run
  prevents a prior socket from masking whether this operation created one.
- why: the in-process `withDirectory` pair establishes the facility state
  seen by the Ken process. The supported twin proves the public route is
  reachable; the typed refusal and absent path distinguish it from an
  unconditional refusal or a post-bind refusal.

## surface/ffi-io/unix-listen-caches-first-procfd-observation

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run one checked client in a Linux user and mount namespace managed
  by a supervisor that can mount within that same namespace. It declares
  `Net Unix`, receives an `FS AFull` grant rooted at `/`, and uses the same
  public `with_socket_listener` route. Use fresh absent socket leaves under
  a pre-created writable directory. Start with an empty tmpfs at `/proc`.
  In the Ken process, `withDirectory` on `/proc/self/fd` observes
  `IOError.NotFound`; its first listener request uses a fresh path. After
  that result, the supervisor mounts procfs over `/proc` in the same mount
  namespace. The same Ken process uses `withDirectory` to confirm that
  `/proc/self/fd` is now available, then requests a listener at a second
  fresh path. Keep the process, grant, client, and namespace unchanged.
- expect: the first request returns exactly
  `Err (ResourceHostIO IOError.Unsupported)`, does not enter the body, and
  leaves its path absent. The second request in that process returns the
  same result with no body entry or path creation, despite the now-available
  procfs; the first decision is cached for the process.
- measured: both in-process path checks, both exact listener results and body
  markers, and both socket-path states. The supervisor records the mount
  transition in the target process's mount namespace.
- claimed: a facility absence found on first need remains the process's
  availability decision; a later need does not re-probe and reverse it.
- controls: a fresh process starting with procfs mounted uses the same client
  for both fresh paths; both listener bodies are entered and both paths are
  created. The earlier absent-facility row also requires this public route to
  succeed with procfs present.
- why: if the implementation re-probes after absence, the second request
  succeeds after the supervisor makes procfs available. The procfs-start
  control rejects an unconditional `Unsupported` implementation. Repeated
  `Unsupported` results without the mount transition are not evidence of a
  per-process cache.

A runner unable to change `/proc` inside the target process's mount namespace
has not executed this row. A local `unshare` supervisor can mount tmpfs and
procfs in one user and mount namespace; a same-namespace process observed
`/proc/self/fd` unavailable before the transition and available after it.
The final bwrap process used for the basic row has no effective mount
capability, so this state-transition row requires a supervisor with mount
capability in the namespace.

## surface/ffi-io/unix-listen-retries-after-undecided-probe-error

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2; §1.13 Unix listener acquisition.
- given: Run one checked client in a supervisor-managed Linux user and mount
  namespace with procfs mounted. It declares `Net Unix`, receives an
  `FS AFull` grant rooted at `/`, and uses the public `with_socket_listener`
  route. In the same Ken process, first use `withDirectory` on
  `/proc/self/fd` to confirm availability. Then arm a one-shot host syscall
  fault injector for the first `/proc/self/fd` facility-probe lookup made by
  the listener request; it returns `EAGAIN` for that probe only and allows
  later calls. The runner records that the fault hit the probe and not the
  preceding in-process check. The first listener path has a missing parent.
  After the result, the Ken client creates that parent with its same
  `FS AFull` grant. The supervisor then mounts an empty tmpfs over `/proc`
  in the target's mount namespace. In that same Ken process, `withDirectory`
  now observes `IOError.NotFound`; the client makes a second listener
  request at a fresh path under the created parent.
- expect: the first listener call stops at the facility check and returns
  exactly `Err (ResourceHostIO (IOError.Other EAGAIN))`; on Linux, `EAGAIN`
  is errno 11. It does not continue to the missing-parent check, enter the
  body, or create a socket path. This is neither `NotFound` nor `Unsupported`.
  On the next need, a fresh probe finds `/proc/self/fd` absent. The listener
  returns exactly `Err (ResourceHostIO IOError.Unsupported)` without entering
  the body or creating the second path. Neither faulted call enters the body
  or creates a socket path.
- measured: the in-process path checks, the one-shot `EAGAIN` probe
  interception, the exact `Other(EAGAIN)` and `Unsupported` listener results,
  both body markers, the mount transition, and both path states.
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
- why: the runner's one-shot fault is applied only after the checked Ken
  availability control, and the trace must witness its actual probe target.
  In the faulted run the facility check stops before the parent walk; only the
  unfaulted control reaches the missing-parent result. Changing the mount
  before the next need makes failure to re-probe observable: it cannot produce
  the specified `Unsupported`.

This case requires a Linux runner that can inject one transient error at the
facility probe and change `/proc` within the target's mount namespace. A local
`strace` 6.13 path-targeted `EAGAIN` injection on `/proc/self/fd` failed the
first open and allowed the second; a local `unshare` supervisor changed the
same namespace from procfs to tmpfs. These are harness-feasibility checks,
not Ken operation results. No Ken operation was executed for these rows; no
official conformance execution, CI, publication, landing, or trust
measurement is claimed.
