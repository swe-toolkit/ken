# Typed stream socket conformance — PX11 seed

Format: `../../README.md`.
Spec: `spec/30-surface/38-ffi-io.md` §1.13 and
`spec/60-security/62-authority.md` AUTH-NET-SURFACE.
Frame: `docs/program/issues/PX11-STREAM-SOCKET-CONNECT-ACCEPT.md`.

These are acceptance cases, not official conformance execution. The checked
interpreter and host tests are implementation evidence; full-workspace CI,
publication, landing, native parity and trusted-base measurement are not claimed
by this seed. The first slice deliberately refuses native execution for a
Net-declared program before object emission.

## Surface and authority

### surface/ffi-io/socket-address-builders-use-closed-typed-inputs

- promise class: durable invariant.
- spec: §1.13 address type and constructors.
- given: construct IPv4 from `Vec UInt8 4`, IPv6 from `Vec UInt8 16`, and Unix
  from absolute pathname bytes. Try the public qualified constructor names
  for the three private address representation arms. Pair each refusal with
  its public smart-builder success.
- expect: a checked source client obtains the three opaque `SocketAddress`
  values through the builders, and cannot construct raw representation arms.
  No duplicate `Vec` family or raw `sockaddr` shape is public.
- why: package authority is the typed address value, not a forged wire tag.

### surface/ffi-io/socket-unix-invalid-paths-refuse-before-syscall

- promise class: durable invariant.
- spec: §1.13 Unix path validation.
- given: vary exactly one property of an otherwise valid absolute path:
  relative spelling, a zero byte, one byte total, or 108 bytes. Pair with
  accepted paths of two and 107 bytes.
- expect: `socket_unix_path` returns `IOError.InvalidInput` for malformed
  inputs, and the host revalidation prevents every invalid decoded address
  from reaching a socket syscall. The accepted neighbors produce addresses.
- why: the filesystem authority model has no class for Linux abstract names,
  and the wire cannot truncate the path or admit the terminating zero.

### surface/ffi-io/socket-class-grants-are-address-scoped

- promise class: durable invariant.
- spec: §1.13 and AUTH-NET-SURFACE.
- given: grant `Loopback`, try listen and connect on both `127.0.0.1` and
  `::1`, then try a non-loopback IPv4 address and an absolute Unix path.
  Grant `Inet` and `Unix` separately as positive controls. A Unix declaration
  without `FS AFull`, and `Net AFull` as an invented class, are invalid headers.
- expect: each denied acquisition returns `ResourceHostIO
  IOError.CapabilityDenied` before a socket backend leaf; a supported grant
  reaches the backend. The header rejects the invalid forms before execution.
- why: a `Net` row describes effects, not ambient authority.

## Interpreter behavior

### surface/ffi-io/socket-loopback-exchange-and-eof

- promise class: durable invariant.
- spec: §1.13; frame AC-1.
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
- spec: §1.13 System.Error and absent-path behavior; frame AC-2.
- given: connect to an absolute Unix pathname not created in a fresh scratch
  directory. Classify the returned `ResourceHostIO` cause with
  `socket_error_to_system ConnectSocket (Some SocketAddress address)`. Pair with
  a wrong-class request whose backend spy would report an attempted syscall.
- expect: the absent path yields `SocketOp ConnectSocket`, `SocketResource`
  naming that same address, canonical `IOError.NotFound`, and no safe context.
  The wrong-class request yields `IOError.CapabilityDenied` with zero backend
  calls. A checked equality establishes that the lift preserves its address.
- why: an error in a different operation or a second identity family does not
  meet the typed failure contract.

### surface/ffi-io/socket-fds-are-noninherited-and-close-on-exec

- promise class: durable invariant.
- spec: §1.13 descriptor acquisition; frame amended AC-3.
- given: hold a listener, accepted stream and connected stream while PX10
  spawns a child blocked on stdin. Read child `/proc/<pid>/fd` links and the
  parent's three `fcntl(F_GETFD)` flags. Drop `SOCK_CLOEXEC` from `accept4`
  in a separate host mutation.
- expect: no parent socket inode appears in the child, and every socket has
  `FD_CLOEXEC`. The mutation reddens the accepted-stream flag row, while PX10
  closefrom may leave the child row green.
- why: the child check alone cannot pin atomic close-on-exec acquisition.

### surface/ffi-io/socket-declared-native-refusal

- promise class: transition sentinel, retired by the native-parity successor.
- spec: §1.13 native boundary; frame AC-4.
- given: native-build the accepted exchange program with a resource profile
  and an empty output directory.
- expect: `host_effect_lowering_unavailable` names `NetListen` and
  `RepresentedUnavailable` before object emission. The output remains empty.
- why: interpreter admission is deliberately not native implementation.
