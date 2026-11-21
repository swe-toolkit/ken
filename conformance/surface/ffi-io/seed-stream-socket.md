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
- expect: loopback addresses reach the backend under `Loopback`; both
  non-loopback IP addresses are denied under `Loopback` before a backend call
  and reach it under `Inet`. A Unix address is denied under `Loopback` and
  reaches the backend under `Unix` with `FS AFull`. `FS APartial` and
  `FS ANone` with `Net Unix` are refused at admission; `Net AFull` is rejected
  as an invalid header. No diagnostic spelling is pinned where §1.13 does not
  specify one.
- why: each address-class pair holds the address and operation fixed while the
  grant changes. The Unix authority pair holds `Net Unix` fixed while varying
  only the FS authority.

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

### surface/ffi-io/socket-resource-revocation-is-resource-error

- promise class: durable invariant.
- spec: §1.13 held-resource revocation; `62 §4.1`.
- fixture: GATED on source/spec shape mismatch. The delivered
  `ResourceError` definition in `crates/ken-elaborator/src/prelude.rs` has no
  `Revoked` constructor; `surface/ffi-io/revoked-single-identity` in
  `conformance/surface/ffi-io/seed-error-classification.md` is gated on the
  same mismatch. Keep this case gated until the resource revocation carrier is
  unified with the normative `ResourceError.Revoked` projection.
- given: create independent, otherwise valid fixtures for each held-resource
  operation: `NetAccept` on a listener, and `NetReceive`, `NetSend`,
  `NetShutdown` and `NetLocalAddress` on a stream. A test harness revokes the
  originating grant out of band before admission; no Ken-callable revoke term
  is used. Compare with a separately acquired resource of the same kind under
  the same live Net class and, for Unix, `FS AFull`. Keep buffer bounds,
  rights and resource kind valid.
  Record guarded host-leaf calls and bracket `ResourceRelease` calls.
- expect: each revoked held-resource operation returns exactly
  `ResourceError.Revoked` and makes no guarded host-leaf call; each live
  control reaches its leaf. When a body operation returns the revocation
  error, its surrounding bracket still settles the already acquired resource
  via `ResourceRelease`.
- why: this covers the complete five-operation held-token set. These are
  ordinary resource-token operations, not Process host operations; the
  acquisition projection above is intentionally different.

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
  and an empty output directory.
- expect: `host_effect_lowering_unavailable` names `NetListen` and
  `RepresentedUnavailable` before object emission. The output remains empty.
- why: interpreter admission is deliberately not native implementation.
