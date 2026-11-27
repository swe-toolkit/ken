# Typed datagram socket conformance — PX11 seed

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.13.1, with the shared address, class
and root rules of §1.13, and `spec/60-security/62-authority.md`
AUTH-NET-SURFACE.

These acceptance cases are not seed execution or official conformance
execution. Targeted interpreter and host tests are implementation evidence
only; they do not execute this seed. Full-workspace CI, publication, landing,
native parity and trusted-base measurement are not claimed by this seed.

For valid Unix addresses the harness uses a fresh short canonical scratch CWD
`D` and its spelling `R` as the default `FS AFull` root, keeping valid path
spellings within 2–107 bytes. For the confinement fixture, use canonical CWD
`D`, `FS AFull "./jail"`, and `R = canonicalize(D/jail)`, with a pre-created
sibling `D/outside`. A fixed seven-byte payload `P` is read into a checked
buffer to obtain a `BufferSpan`. Fixtures read their arguments from the end
and match carried error identities by pattern. The abstract namespace is not
used in these rows.

## Surface and authority

### surface/ffi-io/datagram-operations-require-net-row

- promise class: durable invariant.
- spec: §1.13.1 operation signatures; `36 §1.4` escape check.
- given: for each of `with_datagram_socket`, `send_datagram`,
  `receive_datagram` and `datagram_local_address`, typecheck a well-formed
  client with a `Net` row and the otherwise identical client with only `Net`
  removed. Take resource, buffer and address inputs as parameters.
- expect: each of the four clients with `visits [Net]` is accepted; each
  client omitting only `Net` is rejected by the effect-escape check before
  execution.
- why: the `Net` row on one datagram operation cannot stand in for another.

### surface/ffi-io/datagram-program-without-net-is-refused-before-main

- promise class: durable invariant.
- spec: §1.13 `program capabilities Net <class>+`; §1.13.1 bind.
- given: a program declaring `FS AFull` and no `Net` class whose `main`
  visits `[Net]` and binds a Unix datagram socket at `R/none.sock`.
- expect: the program is refused at program admission before `main` runs;
  no inode exists at `R/none.sock`. At the host boundary, a bind presented
  with a grant carrying no Net class returns
  `ResourceHostIO IOError.CapabilityDenied` with zero backend bind calls.
- why: without a declared class there is no Net authority to bind with; the
  refusal precedes any socket.

### surface/ffi-io/datagram-bind-is-class-scoped

- promise class: durable invariant.
- spec: §1.13.1 bind; §1.13 class membership.
- given: under `Net Loopback`, bind `127.0.0.1` port 0 and, separately,
  `192.0.2.1` port 0. Under the same grant revoked out of band by the
  harness, bind `127.0.0.1` port 0. Record backend bind calls.
- expect: the loopback bind acquires `ResourceKind.SocketDatagram`; the
  `192.0.2.1` bind returns `ResourceHostIO IOError.CapabilityDenied` and the
  revoked bind returns `ResourceHostIO IOError.Revoked`, each with zero
  backend bind calls.
- why: bind uses the stream listener's admission, before any socket syscall.

### surface/ffi-io/datagram-send-peer-is-class-scoped

- promise class: durable invariant.
- spec: §1.13.1 send admission.
- given: under `Net Loopback`, bind `127.0.0.1` port 0 and send the span of
  `P` to the Inet peer `192.0.2.1` port 9. As a control, send the same span
  to the socket's own local address.
- expect: the Inet send returns `ResourceHostIO IOError.CapabilityDenied`
  before any syscall; the control returns `Ok Unit`.
- why: `send_datagram` takes no `Cap`; the peer is admitted against the grant
  the socket retains.

### surface/ffi-io/datagram-send-family-mismatch-is-invalid-input

- promise class: durable invariant.
- spec: §1.13.1 send.
- given: bind an IPv4 loopback socket and send a one-byte span to the IPv6
  peer `::1` port 9 under `Net Loopback`.
- expect: `ResourceHostIO IOError.InvalidInput`, with no send syscall.
- why: a peer must be of the socket's bound family.

### surface/ffi-io/datagram-send-payload-bound

- promise class: normative compatibility vector — the 65507-byte payload
  maximum, for each family separately.
- spec: §1.13.1 send payload bound.
- given: under `Net Loopback`, run the case once for each family: IPv4
  receiver and sender both bound at `127.0.0.1` port 0; IPv6 receiver and
  sender both bound at `::1` port 0; Unix receiver `R/max-receiver.sock` and
  sender `R/max-sender.sock`. Read the receiver's address with
  `datagram_local_address`. Use a buffer of capacity 65508 whose every byte is
  initialized, byte `i` being `i mod 251`. From the sender, first request a
  send of the effective span of all 65508 bytes to the receiver, then a send
  of the effective span of its first 65507 bytes. Then call
  `receive_datagram` on the receiver with a window of 65508. Record backend
  send calls. The harness bounds the receive with a timeout.
- expect, per family: the 65508-byte request returns
  `ResourceHostIO IOError.InvalidInput` with zero backend send calls. The
  65507-byte request returns `Ok Unit` with exactly one backend send call.
  The receive completes, and its first receipt is `ReceivedDatagram` with a
  `TransferCount` of 65507 of an effective 65508, `WholeDatagram`, exactly
  the first 65507 bytes of the buffer, and `Some` sender equal to the
  sender's own local address.
- control: a family whose maximum is too high delivers the 65508-byte
  datagram first, so the first receipt has a count of 65508 and fails this
  case. A family whose maximum is too low refuses the 65507-byte send, and the
  receive does not complete within the timeout, which also fails this case.
- why: the bound is exact, not off by one, applies to every family, and is
  enforced before any syscall. The accepted maximum is observed as a
  delivered datagram, not only as a request that passes the bound check.

### surface/ffi-io/datagram-empty-span-and-zero-window-are-invalid-bounds

- promise class: durable invariant.
- spec: §1.13.1 send and receive bounds.
- given: on a bound socket, send an empty effective span; separately, call
  `receive_datagram` with a zero-length `BufferWindow`.
- expect: both return `ResourceError.InvalidBounds`; the receive makes no
  host call, and neither reaches a socket syscall. Neither is `ReadEof`.
- why: an empty datagram send is outside this slice, and a datagram socket
  has no end of stream.

### surface/ffi-io/datagram-stream-resource-is-a-kind-mismatch

- promise class: durable invariant.
- spec: §1.13.1 kinds.
- given: pass a `Resource ResourceKind.SocketStream` where
  `send_datagram` expects a `Resource ResourceKind.SocketDatagram`.
- expect: rejected during checking by the kind index. At the host, a stream
  or buffer token presented to a datagram operation returns
  `ResourceKindMismatch`, and a released datagram socket returns `Closed`.
- why: the kind index excludes cross-kind use statically; the host repeats it.

## Interpreter behavior

### surface/ffi-io/datagram-loopback-exchange-reports-exact-sender

- promise class: durable invariant.
- spec: §1.13.1 bind, send, receive, local address and sender identity.
- given: for each of IPv4 `127.0.0.1` port 0, IPv6 `::1` port 0, and the Unix
  paths `R/receiver.sock` and `R/sender.sock`, bind a receiver and a sender.
  Read the receiver's address with `datagram_local_address`, send the span of
  `P` from the sender, and receive into a window of 16.
- expect: the receipt is `ReceivedDatagram` with exactly the bytes of `P`,
  `WholeDatagram` and `Some` sender equal to the sender's own local address:
  for IP its OS-assigned port, for Unix its validated bind address, not a
  `/proc` spelling. A reply sent to the reported sender reaches the sending
  socket.
- why: one live program per family reaches all four datagram operations and
  the typed sender result.

### surface/ffi-io/datagram-truncation-is-a-typed-fit

- promise class: durable invariant.
- spec: §1.13.1 receive with `MSG_TRUNC`.
- given: send the seven-byte `P` and receive it into a window of six. Then
  send a two-byte datagram and receive it into a window of six.
- expect: the first receipt is `ReceivedDatagram` with the six-byte prefix,
  a `TransferCount` of six of an effective six, and `TruncatedDatagram`. The
  second is `WholeDatagram` with exactly its two bytes; no excess of the first
  datagram is delivered.
- control: a receive that loses the datagram's true length reports the
  first receipt as `WholeDatagram` and fails this case.
- why: truncation is a field of a successful receipt and the excess is
  discarded.

### surface/ffi-io/datagram-zero-length-is-empty

- promise class: durable invariant.
- spec: §1.13.1 receive.
- given: from another loopback sender, send a zero-length datagram to a bound
  socket and receive it.
- expect: `EmptyDatagram` with `Some` sender equal to that sender's address.
- why: a zero-length datagram is a datagram, not an end of stream.

### surface/ffi-io/datagram-sender-identity-is-representable-only

- promise class: durable invariant.
- spec: §1.13.1 sender identity.
- given: receive datagrams whose senders are an IPv6 address with nonzero
  flowinfo and zero scope ID, an IPv6 address with a nonzero scope ID, an
  unnamed Unix socket, an abstract Unix name, and a named Unix socket that is
  not a live datagram socket of this process.
- expect: the first is `Some` of the address and port with flowinfo dropped;
  the scoped, unnamed and abstract senders are `None`; the named sender is
  `Some` of its absolute path when that path validates as a `SocketAddress`.
- why: only identities `SocketAddress` can carry are reported.

### surface/ffi-io/datagram-unix-bind-outside-root

- promise class: durable invariant.
- spec: §1.13.1 bind; §1.13 root rule.
- given: in the confinement fixture, bind `D/outside/made.sock`, then
  separately `R/../outside/dot.sock` and a leaf symlink `R/alias.sock` to
  `D/outside/linked.sock`.
- expect: each returns `ResourceHostIO IOError.CapabilityDenied`; no inode
  exists at `D/outside/made.sock`, `D/outside/dot.sock` or
  `D/outside/linked.sock`. Binding `R/made.sock` creates the socket.
- why: a datagram bind obeys the stream FS-root rule and leaves no inode on
  refusal.

### surface/ffi-io/datagram-unix-leaf-bound

- promise class: normative compatibility vector — the 79-byte leaf cap.
- spec: §1.13.1 Unix bind spelling.
- given: under a short root, bind Unix paths whose final component is 79 and
  80 bytes, both within the 107-byte path limit.
- expect: the 79-byte leaf binds; the 80-byte leaf returns
  `ResourceHostIO IOError.InvalidInput` and creates no inode.
- why: the `/proc/<pid>/fd/<parent fd>/` prefix takes at most 28 bytes.

### surface/ffi-io/datagram-unix-errors-preserve-identity

- promise class: durable invariant.
- spec: §1.13.1 bind; §1.13 Unix errors.
- given: bind `R/missing/s.sock`, a Unix path under a regular file `R/file`,
  and the empty or final-dot names `R` and `R/.`.
- expect: `IOError.NotFound`, `IOError.NotDirectory` and
  `IOError.InvalidInput` respectively, each without an inode.
- why: confinement does not conflate path and input errors with denial.

### surface/ffi-io/datagram-release-does-not-unlink

- promise class: durable invariant.
- spec: §1.13.1 bind and release.
- given: bind `R/kept.sock` in a bracket whose body returns at once.
- expect: after the bracket settles, the socket is released and the pathname
  `R/kept.sock` still exists.
- why: deleting the pathname is a separate FS act.

### surface/ffi-io/datagram-fds-are-close-on-exec

- promise class: durable invariant.
- spec: §1.13.1 bind.
- given: bind one socket of each family and inspect its descriptor flags and
  socket type at the host.
- expect: each descriptor is close-on-exec and of type `SOCK_DGRAM`.
- why: datagram sockets keep §1.13's no-inheritance property.

## Native refusal

### surface/ffi-io/datagram-native-build-refuses-by-typed-cause

- promise class: durable invariant.
- spec: §1.13.1 native refusal.
- given: native-build each program of the exchange, truncation,
  outside-root and wrong-grant cases above.
- expect: each build exits nonzero with `N0001` and a reason containing
  `is a represented unavailable lane` naming an `FSOp` code in 1800–1803.
  Which of the four codes appears first is not pinned.
- control: the stream-only program of
  `surface/ffi-io/socket-loopback-exchange-and-eof` builds and runs natively
  in the same suite.
- why: the control separates a typed datagram refusal from a regression that
  refuses every `Net` program natively.
