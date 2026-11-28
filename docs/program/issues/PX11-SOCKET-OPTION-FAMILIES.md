---
id: PX11-SOCKET-OPTION-FAMILIES
title: "PX11's committed scope requires explicit socket option families rather than integer (level, optname) pairs, but spec 38 §1.13 sets no option at all (its sockets carry no implicit SO_REUSEADDR or other option) and lists typed option families as subsequent work, so a Ken server cannot rebind a port in TIME_WAIT or disable Nagle on a stream. Admit a closed set of typed option families on the host and interpreter lanes: set and read back each option on the resource kinds it applies to, under the Net capability, with the native lane refusing the new operations by a typed build-time cause until a parity node lands"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: [PX11-DATAGRAM-SOCKETS]
blocks: []
github: null
origin: "Operator 2026-11-27: \"Use L1b to handle any and all nodes in the linux ABI campaign nodes that can run alongside L1.\" PX11's committed objective (docs/program/issues/PX11.md:26, :33) requires option families, not integer pairs; spec 38 §1.13 lists them as subsequent work. The interpreter and host halves do not touch native lowering, so they run on L1b. Follows PX11-DATAGRAM-SOCKETS in the L1b order so the families cover every socket resource kind. Steward-filed per COORDINATION section 2."
---

# Typed socket option families

## Objective

A Ken program holding a Net grant sets a typed socket option on a socket
resource and reads its current value back. Each option belongs to a named
family that states the resource kinds it applies to and its value type.
There is no integer `(level, optname)` form anywhere in the surface. An
option applied to a kind it does not fit refuses with a typed error and
makes no syscall. A native build of such a program refuses at build time
with a typed cause that names the operation.

## Settled inputs (main `eb372244a`; anchors re-checked at release)

- **The committed rule.** `docs/program/issues/PX11.md:33`: "`(level,
  optname)` integer pairs are an untyped hole through the capability
  surface."
- **The current surface sets nothing.** Spec 38 §1.13
  (`38-ffi-io.md:1701`) says the backend uses blocking stream sockets
  "with no implicit `SO_REUSEADDR` or other option", and `:1770` lists
  typed option families as subsequent work. `crates/ken-host/src/net_v1.rs`
  has no `setsockopt` call.
- **The template.** The datagram node (`PX11-DATAGRAM-SOCKETS`) adds the
  datagram resource kind and the next host-family codes after `0x0707`.
  This node takes the codes after the datagram node's, and it covers the
  listener, stream and datagram kinds.
- **The native lane is L1's.** Native lowering is not in scope beyond the
  typed refusal, as for datagrams.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

- Spec 38: a new §1.x stating the families, their applicable kinds, value
  types, the set and get operations, their host-family codes and error
  context.
- The host implementation and the interpreter execution.
- The typed native refusal.
- Conformance seeds and paired tests.

Out of scope: nonblocking mode and readiness (PX12), multicast, broadcast,
raw sockets, the resolver and DNS boundary, and native parity.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - the closed family set. The starting proposal is address reuse on a
    listener, no-delay on an IPv4 or IPv6 stream, and keepalive on a
    stream. Add a family only for a named client need, and remove one that
    has none;
  - whether set and get are one operation pair over a family sum or one
    pair per family, and the host-family codes;
  - the refusal for an option on a kind it does not fit, and for no-delay
    on a Unix stream;
  - the native refusal seat and cause;
  - the census of consumers of the Net operation tables (codec, trace wire,
    availability rosters, spec rosters).
- **AC-1, behavior.** Interpreter rows over loopback:
  - for each family on each kind it fits, set a value and read the same
    value back;
  - a listener with address reuse rebinds a port that a closed connection
    left in TIME_WAIT, and its twin without the option fails with the
    typed address-in-use error;
  - an option on a kind it does not fit refuses with the typed error and
    no syscall;
  - an option call without the Net grant refuses before any syscall.
- **AC-2, native refusal.** Building each AC-1 program natively fails with
  the typed cause. A stream-only program with no option call still builds
  and runs natively.
- **AC-3, mutation.** Make the family-to-kind check accept every kind. The
  ill-fitting row reddens. Restore byte-identically.
- **AC-4.** Every touched test target passes on the exact candidate with no
  name filter, and QA re-runs it there. Full-workspace CI is green on the
  exact candidate.

## Stop conditions

- The design needs an integer level or option-name escape hatch.
- The design needs a new kernel rule or trusted postulate. That is an
  operator stop.
- Admission needs a change to native lowering beyond the typed refusal.
  That belongs to L1.
- Changing an existing stream or datagram operation, code or resource kind.

Checks: 4 fired. I read PX11's committed objective and spec 38 §1.13 on
`eb372244a`; option families are committed scope, and no option is set
today. 3 fired. The listener, stream and datagram kinds are the population
each family is checked against. 8 fired. The TIME_WAIT pair separates a
real option effect from a read-back that only echoes stored state, and the
ill-fitting row separates a typed family from an unchecked pair.
