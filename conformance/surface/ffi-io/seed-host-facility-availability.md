# Host-facility availability through a Ken process

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.8 property (3), as routed in
ABI-M2 AC-0 (`evt_3as7r55jkc0at`). This is a Linux host-boundary case; it
uses an isolated mount namespace to make `/proc/self/fd` unavailable to the
Ken process. It does not claim that Ken source can change host facilities.

## surface/ffi-io/unix-listen-reports-missing-proc-facility

- promise class: durable invariant.
- spec: §1.8 property (3), ABI-M2.
- given: run the same checked Ken client twice. It requests a Unix
  `NetListen` at a fresh path beneath its granted filesystem root, with the
  same `Net` and `FS AFull` authority in both runs. The harness launches each
  Ken process in an otherwise identical Linux user and mount namespace. In
  the control namespace, mount procfs at `/proc`; in the test namespace,
  mount an empty tmpfs at `/proc` instead. The harness verifies inside the
  Ken process that `/proc/self/fd` is unavailable in the test namespace and
  available in the control. Do not use a host-side check as a substitute for
  the in-process check.
- expect: in the control, `NetListen` succeeds and the socket path is
  created. With `/proc/self/fd` unavailable, it returns the existing
  `ResourceHostIO IOError.Unsupported` result before creating the socket
  path. It does not return `CapabilityDenied`, `PermissionDenied`, or a
  successful listener.
- measured: the namespace's `/proc/self/fd` availability, the checked Ken
  result, and socket-path existence after each run.
- claimed: the named facility absence reaches the Ken operation as
  `IOError.Unsupported` without performing the operation's socket effect.
- gap: this case covers the `/proc/self/fd` facility only. It does not cover
  statx, accept4, or any stream-socket family, and it does not assert that
  those facilities are absent on the test host.
- controls: the procfs-mounted run uses the identical client, authority,
  address, and runner configuration except for the `/proc` mount. A fresh
  path per run prevents a prior socket from masking whether the operation
  created one.
- why: hiding `/proc/self/fd` from the Ken process makes the host probe
  decisive. The supported twin proves the operation is reachable; the
  result and absent path distinguish the specified refusal from an
  unconditional refusal or a post-socket refusal.

This row requires a Linux test environment that can create the stated user and
mount namespaces. A runner unable to provide those conditions has not executed
the case. No current-main implementation result, official seed-runner result,
CI, publication, landing, or trust measurement is claimed.
