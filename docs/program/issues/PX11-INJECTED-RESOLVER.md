---
id: PX11-INJECTED-RESOLVER
title: "PX11 commits an injected resolver capability whose trust source and policy are visible, but no resolver exists: a Ken program can connect only to a literal IPv4, IPv6 or Unix address, and spec 38 §1.13 lists the resolver and DNS boundary as subsequent work. Give checked Ken a resolver that a program receives as a capability, names its trust source in the grant, and returns typed socket addresses, with a hermetic source first so the contract is testable without network access"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "docs/program/10-linux-abi-completion.md §4 PX11 (operator-COMMITTED): \"an injected resolver capability whose trust source and policy are visible. DNS is a service boundary, not a syscall.\" Operator 2026-11-27: L1b handles the Linux ABI campaign nodes that can run alongside L1. Spec 38 §1.13 (`38-ffi-io.md:1782`) names it subsequent work. Steward-filed per COORDINATION section 2."
---

# A program resolves names only through a resolver it was given

## Objective

A checked Ken program turns a host name into socket addresses only through a
resolver capability it was granted. The grant names the resolver's trust
source, so a reader of the program's manifest can see where answers come
from. Resolution returns typed `SocketAddress` values that the existing
stream and datagram operations accept unchanged. A program without the
grant is refused at admission.

## Settled inputs (main `f68605014`; anchors re-checked at release)

- **No resolver exists.** The Net catalog homes are `Capability/Net/Stream`,
  `Datagram` and `Option`. The host's Net module states that no resolver
  enters it (`crates/ken-host/src/net_v1.rs:3`). Nothing in `crates/`,
  `catalog/` or `spec/30-surface/` calls `getaddrinfo` or reads
  `/etc/resolv.conf`.
- **The spec defers it.** Spec 38 §1.13 says "An injected resolver and DNS
  boundary, nonblocking/readiness and abstract Unix names remain subsequent
  work" (`38-ffi-io.md:1782`).
- **The commitment.** `docs/program/issues/PX11.md` says the resolver is an
  injected capability rather than a function, because its trust source is a
  policy decision the program must be able to see and choose.
- **The addresses already exist.** Stream, datagram and option operations
  take the typed `SocketAddress` of §1.13; resolution only has to produce
  it.
- **CI has no reliable DNS.** A row that needs a live name server is not a
  gate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The resolver capability's surface and grant, its host operation and
interpreter lane, one hermetic trust source, spec 38 §1.13.3, a seed and the
paired rows. Native lowering is L1's and out of scope; a native build of a
resolver program refuses with a typed cause, as for option families. A
system or network trust source is in scope only if AC-0 shows it can be
pinned without live DNS; otherwise AC-0 names it for a successor frame.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - the capability's form (a resource acquired in a bracket, or a grant
    carried by the effect row) and how its trust source appears in the
    manifest;
  - the operation, its code and wire tag, and its result and error types;
  - the first trust source: a hermetic table fixed at grant time, and
    whether a system source joins it now or later;
  - the consumer census of the Net operation roster, the executor roster
    and every count-bearing sentence;
  - the §1.13.3 text.
- **AC-1, behavior.** Interpreter rows:
  - a table-sourced resolution returns exactly the table's addresses for
    that name, in table order, and connecting to one reaches a local
    listener;
  - a name absent from the table returns the ruled not-found error, and no
    socket is created;
  - a program without the resolver grant is refused at admission, beside an
    admitted twin that differs only in the grant and reaches `main`;
  - a native build of a resolver program refuses with its typed cause and
    leaves no artifact.
- **AC-2, mutation.** Make the resolver ignore its table and answer from
  another source, such as a fixed loopback address. The table row reddens
  and the absent-name row does not. Restore byte-identically.
- **AC-3.** Every touched test target passes on the exact candidate with
  no name filter, and QA re-runs it there. Full-workspace CI is green on
  the exact candidate.

## Stop conditions

- The resolver needs a kernel rule or trusted postulate. That is an
  operator stop.
- A row needs live network or DNS access to pass.
- The design needs a change to an existing Net operation, code or resource
  kind.

Checks: 4 fired. I read the Net catalog homes, the host module header and
§1.13's subsequent-work sentence on `f68605014`; there is no resolver to
extend. 8 fired. The table row and the absent-name row separate a resolver
that reads its source from one that answers constantly, and the admission
twin separates a refused program from one that runs and fails later, the
gap QA found in option families' R6.
