---
id: PX11-DATAGRAM-SOCKETS
title: "Spec 38 §1.13 admits stream sockets on both lanes and leaves datagrams as subsequent work, so a Ken program cannot send or receive a UDP or Unix datagram at all. Admit datagram sockets on the host and interpreter lanes: typed bind, bounded send-to and receive-from with the peer address, under the existing Net capability and FS-root rules, with the native lane refusing the new operations by a typed build-time cause until a parity node lands"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-11-27: \"Use L1b to handle any and all nodes in the linux ABI campaign nodes that can run alongside L1.\" PX11's committed scope (docs/program/issues/PX11.md) includes datagram kinds; spec 38 §1.13 lists them as subsequent work. The interpreter and host halves do not touch native lowering, so they run on L1b beside L1's native chain. Steward-filed per COORDINATION section 2."
---

# Datagram sockets on the host and interpreter lanes

## Objective

A Ken program holding a Net grant can bind a datagram socket to a typed
IPv4, IPv6 or Unix address, send a bounded byte payload to a typed peer,
and receive a bounded payload together with its sender's address. It does
this on the interpreter lane under the same capability, FS-root and
resource-release rules as stream sockets. A native build of such a program
refuses at build time with a typed cause that names the operation, and it
never miscompiles.

## Settled inputs (main `833ea912b`; anchors re-checked at release)

- **The stream set is the template.** Spec 38 §1.13
  (`38-ffi-io.md:1764-1773`) defines seven Net operations, `0x0701` through
  `0x0707`, and the opaque kinds `SocketListener` and `SocketStream`. It
  ends: "Datagrams, typed option families, an injected resolver and DNS
  boundary, nonblocking/readiness and abstract Unix names remain subsequent
  work."
- **The committed scope.** `docs/program/issues/PX11.md` commits "stream and
  datagram kinds; bounded send/receive". The merged interpreter admission
  for streams, `PX11-STREAM-SOCKET-CONNECT-ACCEPT` (`99d594ff7`), is the
  pattern to follow.
- **The address and root rules exist.**
  - `RT-NET-UNIX-PATH-FS-ROOT-SCOPE` (`1a98f6d8a`) scopes Unix paths to
    declared FS roots.
  - `RT-NET-UNIX-STREAM-LOCAL-ADDRESS` (`5495a144b`) covers local-address
    readback.
  - A datagram Unix bind obeys the same root rule.
- **The native lane is L1's.** No native-lowering change is in scope. Today
  a declared Net grant builds natively, because §1.13 removed that refusal.
  So a datagram operation must reach a typed build-time refusal, not a
  generic planner error.
- **The host file.** `crates/ken-host/src/net_v1.rs` holds the stream
  implementation.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

- Spec 38: a new §1.x stating the datagram operations, their host-family
  codes, a datagram resource kind, the bounds, the peer-address results and
  the error context.
- The host implementation and the interpreter execution.
- The typed native refusal.
- Conformance seeds.
- Paired tests that run both lanes.

Out of scope: socket option families, the resolver and DNS boundary,
nonblocking and readiness (PX12), abstract Unix names, multicast and
broadcast, and native parity.

## Acceptance

- **AC-0 (Architect, before code).** Rule the following:
  - the operation set and order, the host-family codes after `0x0707`, and
    the resource kind;
  - the payload bound and how truncation of an oversize datagram is
    reported;
  - whether a receive on an unbound or unconnected socket is a typed error;
  - the native refusal seat and its cause;
  - the census of consumers of the Net operation tables (codec, trace wire,
    availability rosters, spec rosters).
- **AC-1, behavior.** Interpreter rows over loopback for IPv4, IPv6 and
  Unix.
  - Each row binds two sockets, sends a fixed payload one way, and
    receives it with the exact sender address and bytes.
  - A Unix bind outside the declared FS root refuses and leaves no inode.
  - A send without the Net grant refuses before any syscall.
- **AC-2, native refusal.**
  - Building each AC-1 program natively fails with the typed cause.
  - A stream-only program in the same suite still builds and runs natively.
- **AC-3, mutation.** Drop the FS-root check on datagram bind, and the
  outside-root row reddens. Restore byte-identically.
- **AC-4.** Every touched test target passes on the exact candidate with no
  name filter. Full-workspace CI is green on the exact candidate.

## Stop conditions

- The design needs a new kernel rule or trusted postulate. That is an
  operator stop.
- Admission needs a change to native lowering beyond the typed refusal.
  That belongs to L1.
- Changing an existing stream operation, code or resource kind.

Checks: 4 fired. I read spec 38 §1.13 and PX11's committed objective on
`833ea912b`; datagrams are committed scope and named as remaining work.
3 fired. The stream nodes and the two Unix-path nodes are the existing
population of Net rules that a datagram bind must reuse. 8 fired. AC-2's
stream-only control separates a typed datagram refusal from a regression
that refuses all Net natively.
