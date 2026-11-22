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

The root-scope rows follow the ruled RT-NET-UNIX-PATH-FS-ROOT-SCOPE AC-0.
For existing valid Unix addresses, the harness uses a fresh short canonical
scratch CWD `D` and its spelling `R` as the default `FS AFull` root, keeping
valid path spellings within 2–107 bytes. For the confinement fixture, use
canonical CWD `D`, `FS AFull "./jail"`, and
`R = canonicalize(D/jail)`, with a pre-created sibling `D/outside`. The
abstract namespace remains unchanged and is not used in these rows.

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
  case. Give each request the same `FS AFull` root `R` and `Net Unix` authority.
  Pair each with the same operation using the valid absolute address `R/x`.
  Record socket-backend calls.
- expect: each invalid decoded address returns exactly
  `ResourceHostIO IOError.InvalidInput` before any socket syscall; each valid
  control reaches its backend exactly once.
- why: builder rejection happens before the host boundary and cannot establish
  that the host revalidates a decoded address.

### surface/ffi-io/socket-class-grants-are-address-scoped

- promise class: durable invariant.
- spec: §1.13 and AUTH-NET-SURFACE.
- given: under `Net Loopback`, try listen and connect on
  `127.0.0.1:4242`, `127.1.2.3:4243` and `[::1]:4242` as positive controls;
  repeat those endpoints under `Net Inet` to exercise its Loopback
  implication. For both operations, also try `128.0.0.0:4243`, just beyond
  the upper edge of `127.0.0.0/8`, under `Loopback` and `Inet`. Use the same
  non-loopback controls `192.0.2.1:4242` and IPv6 `2001:db8::1:4242` under both
  grants; also try the same absolute Unix path `R/class.sock` beneath the
  supplied `FS AFull` root under `Loopback` and `Unix`. Use a backend spy for
  the address-class boundary.
  Separately, test `Net Unix` admission at the same in-root path `R/class.sock`
  with `FS AFull`, `FS APartial` and `FS ANone`, keeping the path and Net class
  fixed. `Net AFull` is an invented class
  control.
- expect: for each in-range loopback address, including `127.1.2.3:4243`,
  both listen and connect reach the backend under `Loopback` and `Inet`, with
  exactly one backend call per request. `128.0.0.0:4243` is refused under
  `Loopback` as exactly `ResourceHostIO IOError.CapabilityDenied`, with zero
  backend calls, and reaches the backend under `Inet`. The other non-loopback
  IP addresses have the same refusal/reachability split. A Unix address
  returns exactly `ResourceHostIO IOError.CapabilityDenied` under `Loopback`,
  with zero backend calls, and reaches the backend under `Unix` with `FS
  AFull`. `FS APartial` and `FS ANone` with `Net Unix` are refused at
  admission; `Net AFull` is rejected as an invalid header. No diagnostic
  spelling is pinned where §1.13 does not specify one.
- why: `127.1.2.3` distinguishes the specified `/8` membership from an
  implementation special-cased to `127.0.0.1`. The boundary address is
  out-of-range under Loopback but has an Inet positive control; both operations
  use a spy, so the observations do not depend on OS reachability. Each
  address-class pair holds the address and operation fixed while the grant
  changes. The Unix authority pair holds `Net Unix` fixed while varying only
  the FS authority.

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
  the same live Net class and, for Unix, `FS AFull` with its socket path beneath
  root `R`. Keep buffer bounds, rights and resource kind valid. Record guarded
  host-leaf calls and bracket `ResourceRelease` calls.
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
  the same live Net class and, for Unix, `FS AFull` with its socket path beneath
  root `R`. Keep buffer bounds, rights and resource kind valid. Record guarded
  host-leaf calls and bracket `ResourceRelease` calls.
- expect: after unification, each revoked held-resource operation returns
  exactly nullary `ResourceError.Revoked` and makes no guarded host-leaf call;
  each live control reaches its leaf. The bracket still settles an already
  acquired resource via `ResourceRelease`.
- why: this stages the successor projection without treating it as a current
  constructor or leaving current FS/Net resource-token revocation untested.

## Interpreter behavior

The eight AC-1 confinement rows below use fresh runs of the ruled fixture.
Let canonical CWD be `D`; create real directories `D/jail` and writable
`D/outside`. Choose `D` so the valid and adversarial pathname spellings are
2–107 bytes. The host harness supplies `FS AFull` rooted at `./jail`,
so `R=canonicalize(D/jail)`, and the program declares `Net Unix`. Keep listeners
at `R/svc.sock` and
`D/outside/svc.sock` live for the connect and guard rows. Keep
`R/made.sock` and `D/outside/made.sock` absent before the listen rows.
Record socket-backend calls. These pathname cases do not add or change an
abstract Unix address.

The additional pathname-contract rows below use separate stated fixtures.
They do not change AC-2's two outside rows or four guard rows.

### surface/ffi-io/socket-unix-root-listen-inside

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, listen at the absent in-root path
  `R/made.sock`.
- expect: listen succeeds, reaches its socket backend, and creates
  `R/made.sock`; bracket release closes the descriptor but leaves the path.
- controls: the outside-listen row keeps the operation, grant and fixture
  fixed while moving only the path outside `R`.
- why: the in-root row guards against a fix that refuses every Unix path.

### surface/ffi-io/socket-unix-root-listen-outside

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the same AC-1 fixture, `D/outside` exists and is writable;
  listen at the absent sibling path `D/outside/made.sock`.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; `D/outside/made.sock` remains absent.
- controls: the in-root listen succeeds with the same operation, grant and
  fixture. The existing writable parent rules out a missing-directory error.
- why: this pins outside bind refusal and the absence of an outside inode.

### surface/ffi-io/socket-unix-root-listen-leaf-symlink

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, make `D/jail/alias.sock` a symlink to
  `../outside/svc.sock`, keep the outside listener live, and listen at
  `D/jail/alias.sock`.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the alias keeps its symlink target and inode, and the outside
  listener keeps its inode and remains live.
- controls: in a paired fresh run, keep the same class, grant and lexical
  path but leave `D/jail/alias.sock` absent instead of symlinked; listen
  succeeds.
- why: this applies the final-component symlink refusal to `NetListen`;
  the connect leaf-symlink row tests only `NetConnect`.

### surface/ffi-io/socket-unix-root-connect-inside

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, keep a listener live at `R/svc.sock` and
  connect to that in-root address.
- expect: connect succeeds and reaches its socket backend exactly once.
- controls: the outside-connect row uses the same class, grant and fixture
  with a live outside peer; only the target path crosses `R`.
- why: this positive row guards against an over-broad root refusal.

### surface/ffi-io/socket-unix-root-connect-outside

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, keep a host-harness listener live at
  `D/outside/svc.sock`; connect to that outside address.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the live outside listener receives no connection.
- controls: the in-root connect succeeds. The live peer distinguishes root
  denial from `NotFound` or connection refusal.
- why: the outside path is otherwise valid and its peer is reachable.

### surface/ffi-io/socket-unix-root-connect-dotdot

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, keep `D/outside/svc.sock` live and connect to
  `D/jail/../outside/svc.sock`.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the outside listener receives no connection.
- controls: the in-root connect succeeds; the AC-2 raw-path bypass reaches
  this live outside listener.
- why: the address contains a `..` component even though it resolves to the
  outside socket.

### surface/ffi-io/socket-unix-root-connect-parent-symlink

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, make `D/jail/link` a symlink to `../outside`,
  keep `D/outside/svc.sock` live, and connect to `D/jail/link/svc.sock`.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the outside listener receives no connection.
- controls: the in-root connect succeeds; the AC-2 raw-path bypass follows
  this parent symlink to the outside listener.
- why: the requested address contains no `..`; refusal pins parent-symlink
  handling.

### surface/ffi-io/socket-unix-root-connect-leaf-symlink

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: in the AC-1 fixture, make `D/jail/alias.sock` a symlink to
  `../outside/svc.sock`, keep the outside listener live, and connect to
  `D/jail/alias.sock`.
- expect: return `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the outside listener receives no connection.
- controls: the in-root connect succeeds; the AC-2 raw-path bypass follows
  this leaf symlink to the outside listener.
- why: this isolates the final-component symlink guard from parent walking.

The effective AC-2 mutation routes Unix listen and connect through raw
absolute-path `bind` and `connect`, bypassing root resolution. It must make
exactly the two outside rows and all four guard rows—`..`, parent-symlink
connect, leaf-symlink connect and leaf-symlink listen—fail their refusal
expectations, while both in-root rows stay green. Under the bypass, outside
listen creates the inode and all outside connects reach the live listener. The
listener-leaf row reaches `bind` at the existing outside socket name, so it
cannot return the required pre-syscall `CapabilityDenied`. This is a mutation
expectation, not an executed test or mutation.

These additional rows are not part of the eight-row AC-1 matrix; each uses
its stated fresh fixture. For each root `R`, choose a canonical spelling so
all addresses stay within 2–107 bytes; the 82-byte case states its tighter
bound.

### surface/ffi-io/socket-unix-root-alias-through-symlinked-ancestor

- promise class: durable invariant.
- spec: §1.13 Unix acquisition and FS authority.
- given: create canonical CWD `D` with `jail/`, set the host root to
  `FS AFull "./jail"`, and define `R=canonicalize(D/jail)`. Create
  `D/alias -> D`; construct `R/svc.sock`, `D/alias/jail/svc.sock`,
  `R/made.sock` and `D/alias/jail/made.sock` with
  `socket_unix_path`; all four constructors succeed.
  Choose `D` so all addresses meet the 2–107-byte bounds. Keep a host
  listener live at `R/svc.sock` and record socket-syscall attempts. With
  the same root and `Net Unix` grant, request `NetListen` at
  `D/alias/jail/made.sock`, then at `R/made.sock`; request `NetConnect`
  at `R/svc.sock`, then at `D/alias/jail/svc.sock`.
- expect: alias listen returns exactly
  `ResourceHostIO IOError.CapabilityDenied` before any socket syscall and
  leaves `R/made.sock` absent; direct listen succeeds and creates it.
  Direct connect succeeds. Alias connect returns exactly
  `ResourceHostIO IOError.CapabilityDenied` before any socket syscall; the
  live listener receives only the direct connection.
- controls: each direct/alias pair uses the same operation, root, grant and
  target; only the spelling changes. The direct listen and connect
  requests succeed; the live listener distinguishes alias refusal from
  `NotFound` or connection refusal.
- why: the alias spelling is outside `R` but reaches the same in-root
  paths through a symlinked ancestor. This tests the §1.13 alias refusal,
  distinct from links below `R` that point outside.

### surface/ffi-io/socket-unix-empty-or-final-dot-name

- promise class: durable invariant.
- spec: §1.13 Unix acquisition path names.
- given: in a fresh AC-1 fixture, submit decoded Unix addresses `R/` and
  `R/.` at the host boundary, bypassing `socket_unix_path`. Choose `D` so
  both absolute spellings contain no zero byte and lie within the constructor's
  2–107-byte total path bounds. Try both `NetListen` and `NetConnect`; pair each
  operation with its valid in-root control (`R/made.sock` for listen and the
  live listener at `R/svc.sock` for connect).
- expect: each empty or final-dot name returns exactly
  `ResourceHostIO IOError.InvalidInput`. Both valid controls succeed.
- controls: each control holds the operation, root and Net class fixed while
  changing only the final component to a valid socket name.
- why: §1.13 rejects an empty or final-dot socket name, not the root or the
  Unix class; the valid controls distinguish that boundary.

### surface/ffi-io/socket-unix-listener-final-component-bound

- promise class: durable invariant.
- spec: §1.13 Unix listener final-component bound.
- given: in a fresh root fixture, use an existing, non-symlink parent `R`
  and an ASCII `x` leaf of 82 bytes, then one of 83 bytes. Choose `R` to be
  at most 23 bytes, so the 83-byte leaf keeps the absolute path within 107.
  Construct both addresses with
  `socket_unix_path`, keep both leaves absent, then listen under the same
  `FS AFull` root and `Net Unix` grant.
- expect: the 82-byte leaf is accepted and creates a listener. The 83-byte
  leaf returns exactly `ResourceHostIO IOError.InvalidInput`.
- controls: the two requests use the same operation, root, parent and path
  form; only the final-component length changes by one byte.
- why: this isolates the listener-specific 82-byte bound from the general
  107-byte `sun_path` limit.

### surface/ffi-io/socket-unix-missing-parent-is-not-found

- promise class: durable invariant.
- spec: §1.13 Unix parent errors.
- given: in a fresh AC-1 fixture, leave `R/parent` absent and request
  `R/parent/made.sock` with `NetListen` and `R/parent/svc.sock` with
  `NetConnect`, under the same `FS AFull` root and `Net Unix` grant. Construct
  both addresses with `socket_unix_path`. In paired fresh runs, make the same
  `R/parent` an existing directory; keep `made.sock` absent and the `svc.sock`
  listener live.
- expect: each absent-parent request returns exactly
  `ResourceHostIO IOError.NotFound`; the same-spelling existing-parent
  controls succeed.
- controls: path, operation, grant and root stay fixed; only the parent
  directory's existence changes.
- why: a missing parent has the specified `NotFound` identity, distinct from
  the outside-root and symlink refusals.

### surface/ffi-io/socket-unix-nondirectory-parent-is-notdirectory

- promise class: durable invariant.
- spec: §1.13 Unix parent errors.
- given: in a fresh AC-1 fixture, make `R/parent` a regular file and request
  `R/parent/made.sock` with `NetListen` and `R/parent/svc.sock` with
  `NetConnect`, under the same `FS AFull` root and `Net Unix` grant. Construct
  both addresses with `socket_unix_path`. In paired fresh runs, make the same
  `R/parent` a directory; keep `made.sock` absent and the `svc.sock` listener
  live.
- expect: each file-parent request returns exactly
  `ResourceHostIO IOError.NotDirectory`; the same-spelling directory-parent
  controls succeed.
- controls: path, operation, grant and root stay fixed; only the parent
  directory's type changes.
- why: a file used as a parent has the specified `NotDirectory` identity,
  not the missing-parent `NotFound` identity.

### surface/ffi-io/socket-unix-unreadable-or-deleted-root

- promise class: durable invariant.
- spec: §1.13 Unix root errors.
- given: construct `SocketAddress` values for `R/made.sock` and
  `R/svc.sock` with `socket_unix_path`; mint `FS AFull` for readable root
  `R` with `Net Unix`. Bind `R/svc.sock` while readable and keep its listener
  descriptor open. In paired fresh runs, after minting the capability, either
  remove `R`'s read/search access for the host process or unlink its socket
  path and remove `R` while retaining the listener descriptor. Attempt
  `NetListen` at `R/made.sock` and `NetConnect` to `R/svc.sock`. In paired
  controls with a readable root, listen at `R/made.sock` and connect to
  `R/svc.sock`.
- expect: both root-failure states return exactly
  `ResourceHostIO IOError.CapabilityDenied` for listen and connect. The
  readable-root controls succeed.
- controls: the address, operations and Net class are fixed; only root
  readability or existence changes.
- why: an unreadable or deleted root has the specified `CapabilityDenied`
  identity, distinct from ordinary missing-parent errors.

### surface/ffi-io/socket-unix-default-root-is-execution-start-cwd

- promise class: durable invariant.
- spec: §1.13 default FS root.
- given: create a canonical scratch directory `D` beneath `/tmp`, a `D/sub`
  directory, and a live listener at `D/svc.sock`. Choose `D` so the absolute
  address is 2–107 bytes. Construct it with `socket_unix_path`. Run with no
  explicit FS root and execution-start CWD
  `/`, then connect to `D/svc.sock`. Pair with
  a fresh run using the same address and listener but execution-start CWD
  `D/sub`; both programs declare `FS AFull` and `Net Unix`.
- expect: the CWD `/` run connects successfully. The `D/sub` run returns
  exactly `ResourceHostIO IOError.CapabilityDenied` before any socket
  syscall; the live outside listener receives no connection.
- controls: the endpoint, peer, FS declaration and Net class stay fixed;
  only execution-start CWD, hence the default root, changes.
- why: without an explicit root, the execution-start CWD is the root; `/`
  admits this non-symlink path, while `D/sub` does not.

### surface/ffi-io/socket-loopback-exchange-and-eof

- promise class: durable invariant.
- spec: §1.13.
- given: for each of IPv4 `127.0.0.1`, IPv6 `::1` and the Unix path
  `R/exchange.sock` beneath its default `FS AFull` root, listen and connect to
  the resulting listener; read the OS-assigned IP port with
  `socket_local_address`. For Unix, compare the returned `SocketAddress`
  with the requested address and use it to connect. Accept through a bracket.
  Send a known seven-byte chunk in each direction. Receive into a checked
  buffer and compare every octet to the known chunk; after write shutdown,
  read again. Include a first receive bound of four and drain the remainder.
- expect: both received streams contain exactly the sent bytes, the first
  bounded read exposes at most four bytes, and the peer observes `ReadEof`.
  Unix `socket_local_address` equals the validated requested address.
  Releasing a Unix listener closes the descriptor but leaves its pathname.
- why: one live Ken program for each family reaches the interpreter's seven
  socket operations, typed resource brackets and bounded stream-transfer path.
  The Unix readback must be usable as the address clients observe.

### surface/ffi-io/socket-error-preserves-address-and-canonical-cause

- promise class: durable invariant.
- spec: §1.13 System.Error and absent-path behavior.
- given: connect to the absent absolute Unix pathname `R/absent.sock` inside
  the fresh default-root fixture. Classify the returned `ResourceHostIO` cause
  with `socket_error_to_system ConnectSocket (Some SocketAddress address)`.
  Pair with a wrong-class request to the in-root path `R/wrong-class.sock`,
  whose backend spy would report an attempted syscall.
- expect: the absent path yields `SocketOp ConnectSocket`,
  `SocketResource` naming that same address, canonical `IOError.NotFound`,
  and no safe context. The wrong-class request yields
  `IOError.CapabilityDenied` with zero backend calls. A checked equality
  establishes that the lift preserves its address.
- why: an error in a different operation or a second identity family does not
  meet the typed failure contract.

### surface/ffi-io/socket-operation-idempotence-is-exact

- promise class: normative compatibility vector — exact mapping of the closed
  `SocketOperation` family.
- spec: §1.13 `SocketOperation` and operation idempotence; `38 §1.8` retry
  guidance.
- given: import the public `SocketOperation` constructors, `Operation`,
  `SocketOp`, `Idempotent`, `NonIdempotent`, `RetryAdvised`,
  `RetryUnsafeNonIdempotent`, `operation_idempotence`, `retry_guidance` and
  `Transient` from `Capability.System.Error`; construct one value for each of
  `ListenSocket`, `AcceptSocket`, `ConnectSocket`, `ReceiveSocket`,
  `SendSocket`,
  `ShutdownSocket`, `LocalSocketAddress` and `CloseSocket`. Use an exhaustive
  match with one arm per constructor and no catch-all, and evaluate
  `operation_idempotence (SocketOp operation)` in each arm. Also pass the
  `LocalSocketAddress` and `ListenSocket` results to `retry_guidance` with the
  same `Transient` input.
- expect: the exact mapping is `ListenSocket`, `AcceptSocket`,
  `ConnectSocket`, `ReceiveSocket`, `SendSocket`, `ShutdownSocket` and
  `CloseSocket` to `NonIdempotent`; `LocalSocketAddress` maps to
  `Idempotent`. For the shared `Transient` input, the LocalSocketAddress and
  ListenSocket retry results are respectively `RetryAdvised` and
  `RetryUnsafeNonIdempotent`.
- control: `LocalSocketAddress` is the positive idempotence arm;
  `ListenSocket` is the negative arm under the identical `Transient` input. A
  mapping that makes all socket operations idempotent or all non-idempotent
  cannot produce both expected retry verdicts.
- why: the exact inventory asserts the classification of every current socket
  operation, including `CloseSocket`. The exhaustive match makes the case
  require an explicit arm if the closed family changes.

### surface/ffi-io/socket-errors-map-to-exact-system-error-context

- promise class: normative compatibility vector — specified Linux errno
  mappings and System.Error fields.
- spec: §1.13 socket error mapping and `socket_error_to_system`.
- given: import `socket_error_to_system`, `SystemError`, `MkSystemError`,
  `SocketOp`, `SocketResource`, `IOError.Other` and `NoSafeContext` from
  `Capability.System.Error`. Use four independent, otherwise-valid cases
  with `Net Loopback` and the same backend spy. Build addresses through
  `socket_ipv4` from `Capability.Net.Stream` and bind the returned values as
  `connectAddress` = `127.0.0.1:4243`, `listenAddress` =
  `127.0.0.1:4242`, `receiveAddress` = `127.0.0.1:4244`, and
  `shutdownAddress` = `127.0.0.1:4245`. At the final host syscall boundary,
  inject the raw error without pre-converting it to `IOError`: `ECONNREFUSED`
  from `NetConnect`; `EADDRINUSE` from `NetListen`; `ECONNRESET` from
  `NetReceive` on an outbound stream connected to `receiveAddress`; and
  `ENOTCONN` from `NetShutdown` on an outbound stream connected to
  `shutdownAddress`. Pair each injected error with the same operation and
  address under a backend-success response. For each error, take the resulting
  `ResourceError.ResourceHostIO` cause and apply `socket_error_to_system`
  with its exact operation and known address.
- expect: each control reaches its host leaf exactly once and succeeds. The
  four error paths produce these exact nested causes and System.Error fields:
  - `ECONNREFUSED` gives
    `ResourceError.ResourceHostIO (IOError.Other 111)` and `MkSystemError`
    fields `SocketOp ConnectSocket`,
    `SocketResource (Some connectAddress)`, `IOError.Other 111`, and
    `NoSafeContext`.
  - `EADDRINUSE` gives `ResourceError.ResourceHostIO (IOError.Other 98)` and
    `MkSystemError` fields `SocketOp ListenSocket`,
    `SocketResource (Some listenAddress)`, `IOError.Other 98`, and
    `NoSafeContext`.
  - `ECONNRESET` gives
    `ResourceError.ResourceHostIO (IOError.Other 104)` and `MkSystemError`
    fields `SocketOp ReceiveSocket`,
    `SocketResource (Some receiveAddress)`, `IOError.Other 104`, and
    `NoSafeContext`.
  - `ENOTCONN` gives
    `ResourceError.ResourceHostIO (IOError.Other 107)` and `MkSystemError`
    fields `SocketOp ShutdownSocket`,
    `SocketResource (Some shutdownAddress)`, `IOError.Other 107`, and
    `NoSafeContext`.
- control: the backend-success partner for each row keeps the operation,
  address, grant and resource valid while changing only the host result. A
  wrong errno conversion, operation, address, or cause fails its exact typed
  observation; a missing host-leaf call cannot satisfy the success partner.
- why: `SystemError` must retain the operation and known socket address as
  well as the canonical cause and `NoSafeContext`. The four distinct
  addresses make address or operation swaps observable. These are controlled
  host-boundary error cases, not claims that a particular live OS setup
  naturally produces each errno.

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

Clean-room provenance: no external references were consulted. This amendment
is grounded in §1.13, the settled AC-0 ruling and first-principles behavior.
