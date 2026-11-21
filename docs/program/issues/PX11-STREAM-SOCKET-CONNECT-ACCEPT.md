---
id: PX11-STREAM-SOCKET-CONNECT-ACCEPT
title: "A Ken program cannot open a network connection: HostOpV1 has no Net operation, spec 38 records that no Net producer has landed, and System.Error has no socket context. Give PX11 its first slice: typed IPv4, IPv6 and Unix-domain addresses, stream sockets with listen, accept, connect, bounded send and receive, and shutdown, each failure a System.Error with a socket context, on the interpreter"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX9, ABI-M1]
blocks: [PX11]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track T: PX11 is operator-COMMITTED scope (sockets and an injected resolver). L1 objective: the Linux ABI program, in Steward order (operator 2026-10-30), PX10's slices then PX11. This is PX11's first slice; datagrams, option families, the resolver capability and native parity are its successors. Steward-filed per COORDINATION section 2."
---

# A Ken program connects, accepts and exchanges bytes on a stream socket

## Objective

A Ken program listens on a typed address, accepts a connection, connects
to a typed address, sends and receives bounded byte chunks, and shuts a
direction down. An address is IPv4, IPv6 or Unix-domain, never a string
the runtime parses. Each failure is a `System.Error` whose operation and
resource name the socket act and its address.

## Settled inputs (main `4e6c4e62b`; re-measure on the cut base)

- **The contract source.** The PX11 row of `10-linux-abi-completion.md`
  §4 is operator-committed: typed IPv4/IPv6/Unix addresses, stream and
  datagram kinds, bounded send/receive, accept/connect/listen/shutdown,
  socket-error context, option families rather than integer pairs, and an
  injected resolver. DNS is a service boundary, not a syscall.
- **No socket exists today.** `HostOpV1` (`crates/ken-host/src/effect_v1.rs`)
  runs from Console `0x0101` to Process `0x0606` and has no `Net` family.
  Spec 38 says "Ken has no landed `Net` operation in this round", and
  defers a `Net` operation-binding witness "until a real `Net` producer
  lands" (`spec/30-surface/38-ffi-io.md`, near `:159` and `:407`).
- **The authority model.** Spec 62 already states `send` on a socket as
  admitted only with a capability for `Net` authority and `ℓ ⊔ pc ⊑ κ`
  (`spec/60-security/62-authority.md`, near `:400`). This slice adds a
  producer for that rule; it does not change the rule.
- **The error identity.** `Capability/System/Error.ken.md` has
  `Operation = FilesystemOp | ProcessOp` and
  `ResourceRef = FilesystemResource | ProcessResource`, keeping the
  canonical `IOError` identities. A socket failure joins as a third arm of
  each, not a new error family (PX9 row, "process, socket, and later
  completion contexts").
- **The resource pattern.** PX10's pipe ends are Ken resources read and
  written through `ResourceStreamRead` and `ResourceStreamWrite`
  (`0x0408`, `0x0409`) and released by `ResourceRelease`. A connected
  stream socket is the same kind of bidirectional byte resource.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-19)

Architect D0 `evt_23ptn1v4a1jkm` found one settled input false. On
`4e6c4e62b`, `executor_admits` admits only `NativeTested` operations
(`effect_v1.rs:235`-`:240`), so a `RepresentedUnavailable` operation cannot
run on the interpreter. Steward ruling: arm I. A named interpreter-admission
set for exactly the seven `Net` operations is pinned by
`executor_admits_exactly_native_tested_plus_px11_interpreter_set`. The
pre-emission native gate is restored, keyed on `plan.net_declared`. Native
parity is this slice's immediate successor and closes the set again. Until
it lands, this slice delivers no PX11 value under the operator's bar
(`10-linux-abi-completion.md:84`-`:87`).

The D0 governs the host family (`0x07xx`), the address wire form, the
`Net` grant classes, the error arms and the catalog homes. The spec child
also owns the currency repair of `spec/60-security/62-authority.md`'s
`Net` statements. AC-3 and AC-4 below are replaced:

- **AC-3.** The receive-bound row; the `/proc/<child>/fd` row; and an
  `FD_CLOEXEC` row read by `fcntl(F_GETFD)` on a listener, an accepted
  stream and a connected stream. The bound mutant reddens the bound row,
  and dropping `SOCK_CLOEXEC` from `accept4` reddens the `FD_CLOEXEC` row.
  Spawn's `closefrom(3)` masks that mutant in the `/proc` row.
- **AC-4.** One named test: a `Net`-declared program's native build
  refuses with `host_effect_lowering_unavailable`, naming `NetListen` and
  `RepresentedUnavailable`, and leaves the output directory empty.

## Deliverable

1. **AC-0 first.**
   - **The Architect's component design** covers:
     - the host operations and their wire, including whether accepted and
       connected sockets reuse the stream read/write operations or need
       socket-specific ones;
     - the address record's wire form for each family, including the
       Unix-domain path's byte length limit and the abstract namespace;
     - the capability that authorizes listen and connect, and how it is
       scoped, for example by address family or by bound address;
     - close-on-exec on every socket descriptor, so a PX10 child inherits
       none;
     - interpreter and native parity, with native left
       `RepresentedUnavailable`.
   - **The Spec Leader rules** a new spec 38 section: the address type,
     the stream-socket operations, their row (`visits [Net]`), and the
     `System.Error` socket arms. The §1 `Net` deferral sentences are
     replaced, not appended to.
   - **Trust and scope.** Any trust growth is an operator stop. Datagrams,
     option families, the resolver capability and native parity are
     named successors, not this branch.
2. **One spec+crates+catalog branch.** It holds the spec section, the host
   operations on the interpreter, the catalog surface (its home ruled at
   AC-0), the `System.Error` arms, a seed under
   `conformance/surface/ffi-io/`, and tests. The Spec co-turn owns
   `spec/`, and CV votes on `spec/` and `conformance/`.

## Acceptance

- **AC-1 (loopback, each family).** On the interpreter, one Ken program
  listens, connects to its own listener, accepts, and sends a chunk each
  way. It reads exactly the bytes sent, and after the peer's write
  shutdown, it reads end-of-stream. It does this once each over
  `127.0.0.1`, `::1` and a Unix-domain path in a scratch directory. An
  OS-assigned port is read back from the bound listener, not fixed.
- **AC-2 (typed failure).** Connecting to a Unix path with no listener
  returns a `System.Error` whose operation is the socket connect arm,
  whose resource names that address, and whose cause is the canonical
  connection-refused or not-found identity that AC-0 rules. A second
  connect, to an address whose authority the capability does not grant,
  refuses before any syscall.
- **AC-3 (bounded and not inherited, QA mutation).**
  - A receive with bound `n` returns at most `n` bytes when the peer has
    sent more.
  - A PX10 child spawned while the socket is open holds none of its
    descriptors (read from `/proc/<child>/fd`).
  - Each of two mutants reddens the matching row: one that drops
    close-on-exec on the accepted socket, and one that ignores the
    receive bound. Restore each byte-identically.
- **AC-4 (native boundary).** A native build of an AC-1 program refuses
  before object emission through the `RepresentedUnavailable` path,
  naming the socket operation, with no artifact. One named test pins it.
- **AC-5.** Full-workspace CI is green on the exact candidate.

## §1b symptom inventory

Chain: the CI withdrawal of `365483284` (Steward `evt_7aeyfmh1ry82y`;
§1a 1 at Architect `evt_6sgn2694hyr41`).

1. The Net members were added to closed inventories (ABI catalog rows,
   BUFFER_REQUIRE_SAME_AT, the ken-host admission inventories, the
   prelude), while those inventories' non-match consumers (probe facts, a
   self-sourced oracle, hand-listed pins, literal SSA census pins,
   catalog-walk depth) sat outside the targeted runs. Keyed on inventory
   membership.

## Stop conditions

- The design needs a raw `(level, optname)` option pair, or a resolver
  call, to pass AC-1.
- Any trust growth, or a kernel change. These are operator stops.

Checks: 4 fired. PX9 and ABI-M1 are `merged`, and I read `HostOpV1`, the
`System.Error` sums and spec 38's `Net` deferral on `4e6c4e62b`. 6 fired.
The "no socket today" claim comes from grepping `socket` over `catalog/`,
`spec/` and `crates/` at `origin/main`. The only hits are `ken-host`'s
file-kind classification and spec 61/62's illustrative `Socket` signatures.
8 fired. AC-3's close-on-exec mutant passes AC-1 and AC-2, and reddens only
the PX10 child's descriptor row, so that row is what pins non-inheritance.
