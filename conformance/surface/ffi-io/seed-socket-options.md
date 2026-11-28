# Typed socket option family conformance — PX11 seed

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.13.2, with the shared address, class
and root rules of §1.13, and `spec/60-security/62-authority.md`
AUTH-NET-SURFACE.

These acceptance cases are not seed execution or official conformance
execution. Targeted interpreter and host tests are implementation evidence
only; they do not execute this seed. Full-workspace CI, publication, landing,
native parity and trusted-base measurement are not claimed by this seed.

For Unix addresses the harness uses a fresh short canonical scratch CWD `D`
and its spelling `R` as the default `FS AFull` root, keeping valid path
spellings within 2–107 bytes. Fixtures read their arguments from the end and
match carried error identities by pattern. "A connected pair" on an address
`A` means: acquire a listener on `A` (port 0 for IP), learn its address with
`socket_local_address`, open `with_socket_connection` to that address, and
inside it accept one stream with `with_socket_accept`; the pair is the
outbound stream and the accepted stream.

## Surface and authority

### surface/ffi-io/option-operations-require-net-row

- promise class: durable invariant.
- spec: §1.13.2 operation signatures; `36 §1.4` escape check.
- given: for each of `with_reusable_socket_listener`, `set_stream_option`
  and `stream_option`, typecheck a well-formed client with a `Net` row and
  the otherwise identical client with only `Net` removed. Take resource and
  address inputs as parameters.
- expect: each of the three clients with `visits [Net]` is accepted; each
  client omitting only `Net` is rejected by the effect-escape check before
  execution.
- why: the `Net` row on one option operation cannot stand in for another.

### surface/ffi-io/option-program-without-net-is-refused-before-main

- promise class: durable invariant.
- spec: §1.13 `program capabilities Net <class>+`; §1.13.2 reusable
  listener.
- given: a program declaring `FS AFull` and no `Net` class whose `main`
  visits `[Net]` and acquires a reusable listener at `R/never.sock`.
- expect: the program is refused at program admission before `main` runs;
  no inode exists at `R/never.sock`. At the host boundary, a reusable listen
  presented with no grant returns `ResourceHostIO IOError.CapabilityDenied`
  with zero backend listen calls.
- why: without a declared class there is no Net authority to listen with.

### surface/ffi-io/option-call-on-listener-is-ill-typed

- promise class: durable invariant.
- spec: §1.13.2 operation shape (the target is the stream resource).
- given: a program applying `set_stream_option` to a
  `Resource ResourceKind.SocketListener`.
- expect: elaboration fails with a kernel type mismatch; the program never
  runs, and a native build produces no artifact.
- why: an option call on a listener is excluded by type, not refused at run
  time.

### surface/ffi-io/option-call-on-datagram-is-ill-typed

- promise class: durable invariant.
- spec: §1.13.2 operation shape (the target is the stream resource).
- given: under `Net Loopback`, two programs that differ only in the resource
  passed to `stream_option AFull r NoDelay`. In the first, `r` is the
  `Resource ResourceKind.SocketDatagram` bound by `with_datagram_socket` on
  `127.0.0.1` port 0; in the second, its twin, `r` is the outbound
  `Resource ResourceKind.SocketStream` of a connected pair on `127.0.0.1`.
  Each program exits 0 when the call returns `Ok False`.
- expect: the datagram program fails elaboration with a kernel type
  mismatch, never runs, and a native build of it produces no artifact. The
  stream twin elaborates and runs with exit 0.
- why: an option call on a datagram socket is excluded by type. The twin
  differs only in the resource kind, so the refusal is the kind index and
  not a defect of the program around the call.

## Readback

### surface/ffi-io/stream-options-read-back-on-ip-streams

- promise class: durable invariant.
- spec: §1.13.2 set and get.
- given: under `Net Loopback`, a connected pair on `127.0.0.1` and,
  separately, on `::1`. On each of the four streams, for each of `NoDelay`
  and `KeepAlive`: read it, set it `True`, read it, set it `False`, read it.
- expect: every set returns `Ok Unit`; the three reads on every stream and
  option are `Ok False`, `Ok True`, `Ok False` in that order.
- why: both options apply to an IPv4 or IPv6 stream of either origin, and a
  fresh stream reads `False`. This row alone cannot tell the two options
  apart: a host that swapped them consistently would pass it. The mapping
  row below separates them.

### surface/ffi-io/stream-option-codes-name-their-kernel-options

- promise class: normative compatibility vector — the option code and its
  kernel option are the contract.
- spec: §1.13.2 the families and their wire codes.
- given: at the host boundary under `Net Loopback`, two fresh IPv4 outbound
  streams. On the first, set option code 0 to `True`; on the second, set
  option code 1 to `True`. After each set, read both `TCP_NODELAY` (level
  `IPPROTO_TCP`) and `SO_KEEPALIVE` (level `SOL_SOCKET`) on that stream's
  descriptor by `getsockopt`, independently of the operations under test.
  Then, through checked Ken on a third fresh stream, set `NoDelay` to `True`
  and read `KeepAlive`, and on a fourth set `KeepAlive` to `True` and read
  `NoDelay`.
- expect: after code 0, `TCP_NODELAY` reads nonzero and `SO_KEEPALIVE`
  reads 0; after code 1, `SO_KEEPALIVE` reads nonzero and `TCP_NODELAY`
  reads 0. Through checked Ken, each cross read is `Ok False`.
- why: each code sets exactly its own kernel option and leaves the other
  unset, and each `StreamOption` constructor reaches only its own code. A
  swapped or aliased mapping fails one of the four independent reads or a
  cross read.

### surface/ffi-io/stream-option-readback-is-the-kernel-value

- promise class: durable invariant.
- spec: §1.13.2 set and get (`getsockopt`, never a cached copy).
- given: at the host boundary, an IPv4 outbound stream whose backend answers
  the option read with the opposite of the value just set. For each option,
  set `True` and read, then set `False` and read.
- expect: the reads are `Ok False` after the `True` set and `Ok True` after
  the `False` set, the backend's answers. The same stream with the real
  kernel answer reads `Ok True` and `Ok False`.
- why: a readback that only echoes the last set would pass the readback row;
  this row is the one an echo fails.

## Fit and codes

### surface/ffi-io/stream-options-refuse-on-unix-streams

- promise class: durable invariant.
- spec: §1.13.2 set and get (Unix stream).
- given: under `Net Unix` and the default `FS AFull` root `R`, a connected
  pair on `R/listener.sock`. On both streams, for each option, set it `True`
  and read it. Record backend option calls.
- expect: the listener acquires `ResourceKind.SocketListener` and both
  streams are acquired, so `R/listener.sock` exists; every set and read
  returns `ResourceHostIO IOError.InvalidInput`, with zero backend option
  calls.
- why: no family fits a Unix stream. The control is the failure it guards
  against: with the fit check removed, the `NoDelay` set returns
  `ResourceHostIO IOError.Unsupported`, the host's canonical identity for
  `ENOTSUP`.

### surface/ffi-io/stream-option-code-above-one-is-invalid-input

- promise class: durable invariant.
- spec: §1.13.2 wire code.
- given: at the host boundary, an IPv4 stream, and a set and a read whose
  option code is 2, and again with the largest 64-bit code.
- expect: each returns `ResourceHostIO IOError.InvalidInput` with zero
  backend option calls.
- why: the option code is a closed family on the wire as well.

### surface/ffi-io/stream-option-revoked-and-closed

- promise class: durable invariant.
- spec: §1.13.2 set and get (revocation, release).
- given: an IPv4 stream whose originating grant the harness revokes, then
  a set and a read; and a released stream, then a read.
- expect: on the revoked stream, set and read return
  `ResourceHostIO IOError.Revoked`; on the released stream the read returns
  `ResourceError.Closed`. Zero backend option calls in both.
- why: option calls are ordinary resource-borrowing stream operations.

## Reusable listener

### surface/ffi-io/reusable-listener-rebinds-a-time-wait-port

- promise class: durable invariant.
- spec: §1.13.2 reusable listener.
- given: under `Net Loopback`, in one process, a first listener on
  `127.0.0.1` port 0 accepts one connection from a client in the same
  program. The accepted stream is released inside the client's bracket, so
  the server side closes first; the client is released next and the first
  listener last. A second listener is then acquired on the first listener's
  port. Run four arms: both listeners reusable; both plain
  (`with_socket_listener`); the first plain and the second reusable; and the
  first reusable and the second plain.
- expect: with both reusable, the second acquisition succeeds; in each of
  the other three arms it returns `ResourceHostIO (IOError.Other 98)`.
- why: Linux rebinds a port in `TIME_WAIT` only when both the closed
  connection's socket and the rebinding socket carry the option. The plain
  twin and the two mixed arms are the failing configurations the reusable
  arm must differ from. The reusable-first, plain-second arm fails an
  implementation that sets reuse on every listener; the plain-first,
  reusable-second arm fails one that sets it only on the socket leaving
  `TIME_WAIT`.

### surface/ffi-io/reusable-listener-never-takes-a-live-port

- promise class: durable invariant.
- spec: §1.13.2 reusable listener.
- given: under `Net Loopback`, a reusable listener on `127.0.0.1` port 0 and,
  inside its bracket, a second reusable listener on the same port.
- expect: the second acquisition returns `ResourceHostIO (IOError.Other 98)`.
- why: address reuse does not admit a second listener on a live port.

### surface/ffi-io/reusable-listener-refuses-unix-before-any-syscall

- promise class: durable invariant.
- spec: §1.13.2 reusable listener (Unix address).
- given: under `Net Unix` and the default `FS AFull` root `R`, a reusable
  listener on `R/reusable.sock`; and, under `Net Loopback` only, the same
  Unix address. Record backend listen calls.
- expect: under `Net Unix` the acquisition returns
  `ResourceHostIO IOError.InvalidInput`, and under `Net Loopback` it returns
  `ResourceHostIO IOError.CapabilityDenied`, the grant admission coming
  first. Both make zero backend listen calls, and no inode exists at
  `R/reusable.sock`.
- why: the reusable listener is IP only; its Unix refusal follows grant
  admission and precedes root resolution and every syscall.

### surface/ffi-io/reusable-listener-sets-reuse-only-on-itself

- promise class: durable invariant.
- spec: §1.13.2 reusable listener; §1.13 (no implicit option).
- given: at the host boundary under `Net Loopback`, a reusable listener and
  a plain listener on `127.0.0.1` port 0, and the same on `::1`. Read
  `SO_REUSEADDR` on each listener's descriptor independently of the
  operations under test.
- expect: each reusable listener reads 1 and each plain listener reads 0.
- why: `with_socket_listener` still sets no option; only the reusable
  acquisition sets one.

## Native refusal

### surface/ffi-io/option-programs-refuse-native-build

- promise class: durable invariant.
- spec: §1.13.2 native refusal.
- given: three programs, each using exactly one option-family operation
  and otherwise only stream operations: one acquires a reusable listener on
  `127.0.0.1` port 0; one sets `NoDelay` on the outbound stream of a
  connected pair on `127.0.0.1`; one reads `NoDelay` on such a stream.
  Native-build each. As a control, native-build the stream-only exchange
  program of the stream seed.
- expect: each program fails at build time with `N0001`, reason
  `effect <owner>::FSOp.<code> is a represented unavailable lane`, where
  `<code>` is 1804 for the reusable listener, 1805 for the set and 1806 for
  the read, and emits no artifact. The stream-only control builds and its
  artifact runs with exit 0.
- why: the native lane refuses option-family operations by a typed cause
  and never miscompiles, while stream parity is unchanged.
