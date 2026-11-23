---
id: RT-NET-UNIX-STREAM-LOCAL-ADDRESS
title: "Since RT-NET-UNIX-PATH-FS-ROOT-SCOPE binds Unix listeners through /proc/self/fd/<parent>/<leaf>, socket_local_address on a stream accepted from a Unix listener returns that host-internal spelling, which carries a descriptor number and refuses when fed back to connect. Spec 38 §1.13 forbids both. Report the listener's validated address on its accepted streams, and state the stream readback in spec 38"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-NET-UNIX-PATH-FS-ROOT-SCOPE]
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Adversary evt_q3fnv2qge1xg on 1a98f6d8a, Steward-verified. A regression introduced by RT-NET-UNIX-PATH-FS-ROOT-SCOPE. It precedes PX11-NATIVE Inc1 because native parity reproduces the interpreter's readback, and both touch the same SocketAddress reply arm. L1 Linux ABI program, Steward order."
---

# An accepted Unix stream's local address

## Objective

On a stream accepted from a Unix listener acquired at `R/svc.sock`,
`socket_local_address` returns `R/svc.sock`, the listener's validated
address. Connecting to that address succeeds. No readback of any socket
resource contains a `/proc/self/fd/` spelling.

## Settled inputs (main `1a98f6d8a`; anchors re-checked at release)

- **The bind.** `listen_unix` binds to `/proc/self/fd/{parent}/{leaf}`
  (`ken-host/src/net_v1.rs` ~`:256`, ~`:327`).
- **The stored address.** Acquisition stores the validated address only
  for a `SocketListener` (`effect_v1.rs` ~`:3943`). `NetAccept` creates
  a `SocketStream` with no stored address.
- **The readback.** The `NetLocalAddress` arm (`effect_v1.rs` ~`:3999`)
  returns a listener's stored address. For `SocketStream`, and for a
  listener with no stored address, it calls `net_local_address(fd)`,
  which is getsockname. For an accepted Unix stream, getsockname returns
  the listener's bound `sun_path`.
- **The spec.** Spec 38 §1.13 (~`:1585`): "a Unix listener reports the
  validated address it was acquired with, not the host-internal pinned
  descriptor spelling", and listener and stream resources "expose no raw
  descriptor".
- **The measurement.** The Adversary's probe calls
  `with_socket_connection` with the accepted stream's local address, and
  it exits 84 with `CapabilityDenied`. The same program reconnecting with
  the original address exits 0. strace shows `getsockname` returning
  `/proc/self/fd/3/svc.sock`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The accepted stream's stored address in `ken-host`, the readback arm,
one sentence in spec 38 §1.13, and the pins below. No change to the bind
or connect confinement, to IP readback, or to peer-address readback,
which stays out of this slice.

## Acceptance

- **AC-1.** A row listens on `R/jail/svc.sock`, connects, accepts and
  reads the accepted stream's local address. It asserts that the address
  equals the listener's address exactly, and that connecting to it
  succeeds. A second listener at a different leaf shows the address is
  per listener, not a constant.
- **AC-2.** A row reads the local address of a connected Unix stream,
  the client side. Its result contains no `/proc/self/fd/` spelling. The
  implementer measures it first; the Architect's review rules it if it
  is not already an unnamed address.
- **AC-3 (mutation).** Restore the getsockname path for accepted
  streams. AC-1 reddens and AC-2 stays green. Restore byte-identically.
- **AC-4.** Spec 38 §1.13 states the accepted-stream readback as a Spec
  co-turn, and full-workspace CI is green on the exact candidate.

## Stop conditions

- The validated address cannot reach the accepted stream without
  changing the resource-table ABI that PX11-NATIVE Inc1 owns. Report that
  to the Architect so the repair folds into Inc1.

Checks: 2 fired. The property is that no readback leaks the pinned
spelling. Its fan-in is the listener arm (fixed by RT-NET), the
accepted-stream arm (leaks) and the connected-stream arm (AC-2).
`NetAccept` returns no address of its own. 8 fired. AC-1's reconnect
fails on the leaked spelling, and the second listener separates a stored
address from a constant.
