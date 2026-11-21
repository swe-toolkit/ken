# Typed stream socket conformance — PX11 seed

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.13 and
`spec/60-security/62-authority.md` AUTH-NET-SURFACE.

These acceptance cases are not seed execution or official conformance
execution. Targeted interpreter and host tests are implementation evidence
only; they do not execute this seed. Full-workspace CI, publication, landing,
native parity and trusted-base measurement are not claimed by this seed. The
first slice deliberately refuses native execution for a Net-declared program
before object emission.

## Surface and authority

### surface/ffi-io/socket-address-builders-enforce-indexed-lengths

- promise class: durable invariant.
- spec: §1.13 address type and public constructors.
- given: call `socket_ipv4` with both a `Vec UInt8 4` and a `Vec UInt8 3`;
  call `socket_ipv6` with both a `Vec UInt8 16` and a `Vec UInt8 15`. Keep the
  port and all supplied octets otherwise fixed.
- expect: the exact-length calls are accepted and produce `SocketAddress`;
  the shorter vectors are rejected during checking as wrong constructor
  arguments. No diagnostic code is pinned where §1.13 does not specify one.
- why: each positive control holds the constructor and values fixed while the
  vector index changes by one.

### surface/ffi-io/socket-address-operations-require-typed-inputs

- promise class: durable invariant.
- spec: §1.13 address type and public constructors.
- given: build IPv4, IPv6 and Unix addresses through their public smart
  constructors, then pass each resulting `SocketAddress` to
  `with_socket_listener` or `with_socket_connection`. In otherwise identical
  calls, pass a raw `Vec UInt8 4`, `Vec UInt8 16` or `Bytes` in the address
  argument position instead.
- expect: builder-produced addresses are accepted by the typed operations;
  raw vectors and bytes are rejected during checking as not being
  `SocketAddress` values.
- why: this pins the specified client-facing type boundary without guessing
  private representation-constructor names. Constructor visibility is covered
  by `public-type-does-not-publish-constructors` in
  `conformance/surface/modules/seed-modules.md`.

### surface/ffi-io/socket-unix-invalid-paths-rejected-at-builder

- promise class: durable invariant.
- spec: §1.13 Unix path validation.
- given: vary exactly one property of an otherwise valid absolute path:
  a relative two-byte spelling, a zero byte at the same total length, one byte
  total, or 108 bytes. Pair with
  accepted paths of two and 107 bytes. Submit these values through
  `socket_unix_path`.
- expect: malformed paths return `IOError.InvalidInput` without producing a
  `SocketAddress`; the accepted neighbors produce addresses. This case does
  not claim to exercise the separate host-boundary revalidation rule.
- why: the public builder accepts only this slice's absolute pathname form;
  host-boundary revalidation is exercised separately below.

### surface/ffi-io/socket-host-revalidates-decoded-address

- promise class: durable invariant.
- spec: §1.13 host validation of decoded addresses.
- given: at the host-operation boundary, submit otherwise well-formed
  `NetListen` and `NetConnect` requests whose decoded Unix address bypasses
  `socket_unix_path`, exercising each invalid path property from the preceding
  case. Pair each with the same operation using a valid `/x` address, `Net Unix`
  authority and `FS AFull`. Record socket-backend calls.
- expect: each invalid decoded address is refused before any socket syscall;
  each valid control reaches its backend exactly once. No particular
  diagnostic code is asserted because §1.13 does not specify one for this
  boundary input.
- why: builder rejection happens before the host boundary and cannot establish
  that the host revalidates a decoded address.

### surface/ffi-io/socket-class-grants-are-address-scoped

- promise class: durable invariant.
- spec: §1.13 and AUTH-NET-SURFACE.
- given: under `Net Loopback`, try listen and connect on `127.0.0.1` and `::1`
  as positive controls; repeat those endpoints under `Net Inet` to exercise
  its Loopback implication. For both operations, use the same non-loopback
  addresses `192.0.2.1:4242` and IPv6 `2001:db8::1` at port `4242` under
  `Loopback` and `Inet`; also try an absolute Unix path under `Loopback` and
  `Unix`. Use a backend spy for the address-class boundary. Separately, test
  `Net Unix` admission with `FS AFull`, `FS APartial` and `FS ANone`, keeping
  that class fixed. `Net AFull` is an invented class control.
- expect: for each loopback address, both listen and connect reach the
  backend under `Loopback` and `Inet`, with exactly one backend call per
  request. Each non-loopback IP address returns exactly
  `ResourceHostIO IOError.CapabilityDenied` under `Loopback`, with zero backend
  calls, and reaches the backend under `Inet`. A Unix address returns exactly
  `ResourceHostIO IOError.CapabilityDenied` under `Loopback`, with zero backend
  calls, and reaches the backend under `Unix` with `FS AFull`. `FS APartial`
  and `FS ANone` with `Net Unix` are refused at admission; `Net AFull` is
  rejected as an invalid header. No diagnostic spelling is pinned where
  §1.13 does not specify one.
- why: each address-class pair holds the address and operation fixed while the
  grant changes. The Unix authority pair holds `Net Unix` fixed while varying
  only the FS authority.

### surface/ffi-io/socket-wildcard-addresses-require-inet

- promise class: durable invariant.
- spec: §1.13 address-class grants; AUTH-NET-SURFACE.
- given: for each of `NetListen` and `NetConnect`, submit the exact IPv4
  wildcard `0.0.0.0:4242` and IPv6 wildcard `[::]:4242` (the sixteen address
  octets are zero; the port is 4242). For each fixed operation and address,
  compare `Net Loopback` with `Net Inet` under the same FS authority and live
  grant. Observe the address-class admission boundary with a backend spy;
  backend reachability, not a successful OS connect to a wildcard, is the
  positive observation. Keep `127.0.0.1:4242` and `[::1]:4242` under
  `Net Loopback` as same-family positive controls.
- expect: both wildcards return exactly
  `ResourceHostIO IOError.CapabilityDenied` under `Loopback` with zero backend
  calls and reach the backend under `Inet`; each loopback control reaches the
  backend under `Loopback`. A generic host I/O error after backend admission
  does not count as the denied-class result.
- why: an implementation that treats either wildcard as a Loopback endpoint
  passes the non-loopback-address case above but fails this fixed-address,
  grant-only pair for that family.

### surface/ffi-io/socket-net-effect-row-is-required

- promise class: durable invariant.
- spec: §1.13 operation signatures; `36 §1.4`.
- given: under the same valid `Net Loopback` program capability header, for
  each of `with_socket_listener`, `with_socket_accept`,
  `with_socket_connection`, `receive_socket`, `send_socket`, `shutdown_socket`
  and `socket_local_address`, typecheck a well-formed client with a `Net` row
  and the otherwise identical client with only `Net` removed. Take resource,
  buffer and address inputs as parameters so no other Net operation is needed
  to reach the operation under test.
- expect: each of the seven clients with `visits [Net]` is accepted; each
  client omitting only `Net` is rejected by the effect-escape check before
  execution.
- why: the `Net` row on one operation cannot stand in for the other six;
  capability admission and effect-row permission are independent gates.

### surface/ffi-io/socket-acquisition-revocation-uses-host-error

- promise class: durable invariant.
- spec: §1.13 acquisition revocation; `62 §4.1`.
- given: for `NetListen` and `NetConnect`, use a valid loopback address and
  `Net Loopback` authority. A test harness revokes the grant out of band before
  admission; compare with the same grant left live. Record guarded
  socket-backend calls. No Ken-callable revoke term is used.
- expect: revoked acquisition returns
  `ResourceHostIO IOError.Revoked` before a socket syscall; the live control
  reaches the backend. A wrong-class address remains the separate
  `ResourceHostIO IOError.CapabilityDenied` case above.
- why: acquisition consumes a host capability, not a previously acquired
  SocketListener or SocketStream token.

### surface/ffi-io/socket-resource-revocation-uses-current-hostio-carrier

- promise class: durable invariant.
- spec: §1.13 delivered FS/Net resource-token carrier; `62 §4.1`.
- given: create independent, otherwise valid fixtures for each held-resource
  operation: `NetAccept` on a listener, and `NetReceive`, `NetSend`,
  `NetShutdown` and `NetLocalAddress` on a stream. A test harness revokes the
  originating grant out of band before admission; no Ken-callable revoke term
  is used. Compare with a separately acquired resource of the same kind under
  the same live Net class and, for Unix, `FS AFull`. Keep buffer bounds,
  rights and resource kind valid. Record guarded host-leaf calls and bracket
  `ResourceRelease` calls.
- expect: each revoked held-resource operation returns exactly
  `ResourceError.ResourceHostIO IOError.Revoked` and makes no guarded host-leaf
  call; each live control reaches its leaf. When a body operation returns the
  revocation error, its surrounding bracket still settles the acquired resource
  via `ResourceRelease`.
- why: this pins current delivery for the complete five-operation held-token
  set. These are ordinary FS/Net resource-token operations, not Process host
  operations; the carrier retains the same `IOError.Revoked` cause.

### surface/ffi-io/socket-resource-revocation-uses-successor-projection

- promise class: normative property — post-unification resource-token
  projection.
- spec: §1.13 held-resource revocation; `62 §4.1`.
- fixture: GATED on ABI-REVOKE resource-token projection unification. The live
  current carrier for all five operations is pinned by the case above.
- given: create independent, otherwise valid fixtures for each held-resource
  operation: `NetAccept` on a listener, and `NetReceive`, `NetSend`,
  `NetShutdown` and `NetLocalAddress` on a stream. A test harness revokes the
  originating grant out of band before admission; no Ken-callable revoke term
  is used. Compare with a separately acquired resource of the same kind under
  the same live Net class and, for Unix, `FS AFull`. Keep buffer bounds,
  rights and resource kind valid. Record guarded host-leaf calls and bracket
  `ResourceRelease` calls.
- expect: after unification, each revoked held-resource operation returns
  exactly nullary `ResourceError.Revoked` and makes no guarded host-leaf call;
  each live control reaches its leaf. The bracket still settles an already
  acquired resource via `ResourceRelease`.
- why: this stages the successor projection without treating it as a current
  constructor or leaving current FS/Net resource-token revocation untested.

## Interpreter behavior

### surface/ffi-io/socket-loopback-exchange-and-eof

- promise class: durable invariant.
- spec: §1.13.
- given: for each of IPv4 `127.0.0.1`, IPv6 `::1` and a Unix path in a
  scratch directory, listen and connect to the resulting listener; read the
  OS-assigned IP port with `socket_local_address`. Accept through a bracket.
  Send a known seven-byte chunk in each direction. Receive into a checked
  buffer and compare every octet to the known chunk; after write shutdown,
  read again. Include a first receive bound of four and drain the remainder.
- expect: both received streams contain exactly the sent bytes, the first
  bounded read exposes at most four bytes, and the peer observes `ReadEof`.
  Releasing a Unix listener closes the descriptor but leaves its pathname.
- why: one live Ken program for each family reaches the interpreter's seven
  socket operations, typed resource brackets and bounded stream-transfer path.

### surface/ffi-io/socket-error-preserves-address-and-canonical-cause

- promise class: durable invariant.
- spec: §1.13 System.Error and absent-path behavior.
- given: connect to an absolute Unix pathname not created in a fresh scratch
  directory. Classify the returned `ResourceHostIO` cause with
  `socket_error_to_system ConnectSocket (Some SocketAddress address)`. Pair
  with a wrong-class request whose backend spy would report an attempted
  syscall.
- expect: the absent path yields `SocketOp ConnectSocket`,
  `SocketResource` naming that same address, canonical `IOError.NotFound`,
  and no safe context. The wrong-class request yields
  `IOError.CapabilityDenied` with zero backend calls. A checked equality
  establishes that the lift preserves its address.
- why: an error in a different operation or a second identity family does not
  meet the typed failure contract.

### surface/ffi-io/socket-closed-peer-send-preserves-broken-pipe

- promise class: durable invariant.
- spec: §1.13 `NetSend` closed-peer result and `MSG_NOSIGNAL`.
- given: hold a live, writable SocketStream and a valid positive `BufferSpan`.
  A separate peer closes the other stream; synchronize that close and make
  bounded positive sends until the first closed-peer send is attempted, since
  a send racing a queued close may succeed. Run the sending boundary in a
  subprocess with SIGPIPE at its default disposition immediately before the
  tested send (not merely at launch, where Ken may change it). Keep the grant,
  stream token, buffer and span valid, and observe the subsequent result and
  a post-send process marker. Pair with the same send while the peer remains
  open, and require the peer to receive the known bytes.
- expect: the closed-peer attempt returns
  `ResourceHostIO IOError.BrokenPipe`; the sender reaches the marker and
  exits normally, not from SIGPIPE. The live-peer control transfers the bytes.
  A successful send that raced the close is not the closed-peer oracle; an
  unbounded retry or an unrelated resource refusal does not satisfy it.
- why: an ordinary Ken process that ignores SIGPIPE could mask a dropped
  `MSG_NOSIGNAL`. Restoring the default disposition at the send boundary
  distinguishes a per-send suppression from process-wide signal policy.

### surface/ffi-io/socket-send-zero-and-short-progress

- promise class: durable invariant.
- spec: §1.13 `NetSend` progress; §1.7.2 write partition.
- given: at the host transfer boundary, use a live SocketStream and a
  seven-byte checked `BufferSpan` with a positive effective request. In one
  controlled send, have the backend return zero; in a separate otherwise
  identical send, return three, then send the remaining four from the
  advanced span. Observe both `WriteProgress` results, the exact requested
  ranges and the bytes the peer receives. Include a full-seven positive
  control with the same buffer and grant.
- expect: a zero host result returns the error `ResourceError.NoProgress`,
  never `WriteProgress.Wrote 0`. The short send returns `Wrote 3`, advances
  only the first three bytes, and a subsequent `Wrote 4` completes the
  remaining four; the peer receives the seven known bytes exactly once. The
  full control returns `Wrote 7`. No result exceeds its admitted span.
- why: a successful short send is not a complete send, and a zero result
  cannot manufacture positive progress. Controlled backend counts make both
  branches reachable without relying on TCP packetization.

### surface/ffi-io/socket-shutdown-read-and-both-directions

- promise class: durable invariant.
- spec: §1.13 shutdown-direction family.
- given: on separate live connected streams, call `NetShutdown` with checked
  `ShutdownRead`, `ShutdownBoth` and the existing `ShutdownWrite` control.
  Observe the host shutdown direction selected for each; keep the stream
  kind, grant and prior byte exchange identical. For `ShutdownRead`, send a
  known outbound chunk afterward and observe it at the peer. For
  `ShutdownBoth`, try the same outbound send and a receive after the
  shutdown, with the peer still open and ready.
- expect: the three constructors select `SHUT_RD`, `SHUT_RDWR` and
  `SHUT_WR`, respectively. Read-only shutdown leaves outbound transfer
  possible; both-direction shutdown admits no further outbound bytes and
  produces `ReadEof` on subsequent receive. The pre-shutdown control
  transfers in both directions. An error from a closed resource token before
  the host call does not count as choosing a direction.
- why: exercising only `ShutdownWrite` leaves a swapped or collapsed read
  and both code undetected; independent streams distinguish all three tags.

### surface/ffi-io/socket-fds-are-noninherited-and-close-on-exec

- promise class: durable invariant.
- spec: §1.13 descriptor acquisition.
- given: hold a listener, accepted stream and connected stream while PX10
  spawns a child blocked on stdin after those descriptors have been acquired.
  Read child `/proc/<pid>/fd` links and the parent's three
  `fcntl(F_GETFD)` flags. Drop `SOCK_CLOEXEC` from `accept4` in a separate
  host mutation.
- expect: no parent socket inode appears in the later child, and every socket
  has `FD_CLOEXEC`. The mutation reddens the accepted-stream flag row, while
  PX10 closefrom may leave the child row green. This observation is only about
  the post-acquisition state.
- why: a later child check may be masked by PX10 closefrom; the flag row checks
  each socket descriptor directly.

### surface/ffi-io/socket-accept-cloexec-has-no-inheritance-window

- promise class: durable invariant.
- spec: §1.13 atomic close-on-exec acquisition.
- given: use a host-boundary schedule with a minimal exec observer on a
  second thread whose spawn actions do not close inherited descriptors.
  Trigger the tested path's observer at the accepted descriptor's underlying
  `accept`/`accept4` return boundary, before any later userspace
  `fcntl(FD_CLOEXEC)` can run. Inspect the exec child's `/proc/<pid>/fd` links
  for the accepted socket inode. Pair this with a control
  that uses non-atomic `accept` and blocks at the same return boundary before
  `fcntl(FD_CLOEXEC)`, then execs the same observer.
- expect: the specified accept path exposes no accepted socket inode to the
  exec child. The non-atomic control exposes it, proving this schedule reaches
  the vulnerable interval.
- why: the observer runs at descriptor creation, before any post-creation
  flag-setting on the control path. Starting the child only after the Ken
  operation returns could miss the interval; an observer that closes
  inherited descriptors would also mask it.

### surface/ffi-io/socket-declared-native-refusal

- promise class: transition sentinel, retired by the native-parity successor.
- spec: §1.13 native boundary.
- given: native-build the accepted exchange program with a resource profile
  and an empty output directory. Separately build a checked program declaring
  `Net Loopback` whose socket operation sits only in a branch known not to
  execute: the interpreter takes the non-socket branch and returns the known
  result. Also build a Net-declared program with the same Net-free body as the
  positive control below, so declaration alone is the only changed input.
  Give each native attempt a fresh empty output directory.
- expect: each Net-declared native build returns the typed
  `host_effect_lowering_unavailable` reason before object emission. The
  reason identifies `RepresentedUnavailable` for **each** of `NetListen`,
  `NetAccept`, `NetConnect`, `NetReceive`, `NetSend`, `NetShutdown` and
  `NetLocalAddress`; the exact set is seven, with no other operation named.
  Every negative output directory remains empty. Interpreter acceptance of
  the dormant-branch program is checked independently; a failure before
  native admission does not count as this refusal.
- why: a live socket call alone cannot prove that the declaration gate
  refuses a program whose socket branch is dormant, or a program that
  declares Net but contains no socket call at all. The named unavailable
  set distinguishes this transition from an arbitrary build failure.

### surface/ffi-io/socket-net-free-native-build-accepts

- promise class: durable invariant.
- spec: §1.13 native boundary and `45 §4` checked host entrypoint.
- given: use the same checked Net-free body, FS authority and resource
  profile as the declaration-only negative above, in a fresh output
  directory. The only program difference is the absence of `Net Loopback`
  in its capability header. Its main returns
  a known ground exit result without a socket effect.
- expect: native build succeeds, emits a runnable artifact and its invocation
  returns the known exit result. It does not merely fail with a different
  diagnostic. Interpreter execution of that same source gives the same
  result.
- why: rejecting every native build satisfies a negative-only refusal case;
  this paired positive case would reject such an implementation.
