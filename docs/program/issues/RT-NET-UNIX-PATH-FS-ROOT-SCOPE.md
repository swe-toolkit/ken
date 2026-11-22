---
id: RT-NET-UNIX-PATH-FS-ROOT-SCOPE
title: "A program declaring FS AFull on a root and Net Unix can listen on or connect to a Unix socket path anywhere on the host, because NetScopeV1::admits checks only the Unix class bit and never the Cap's FS root. The same program's file acts are refused outside that root. Rule from a D0 whether a Unix path is confined beneath the declared root or stated as unbounded, as Process is, and make the host, spec 38 and a pin agree"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [PX11-STREAM-SOCKET-CONNECT-ACCEPT]
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Adversary evt_7hffqy9m7v6yb on 99d594ff7 (PX11-STREAM-SOCKET-CONNECT-ACCEPT), Steward-verified. L1 Linux ABI program (operator 2026-10-30: \"Order the linux ABI campaign as you see fit\"). Native parity must reproduce whichever boundary stands, so this precedes it. Steward-filed per COORDINATION section 2."
---

# A Unix socket path and the declared FS root

## Objective

For a program declaring `FS AFull "<root>"` and `Net Unix`, the reach of
`NetListen` and `NetConnect` on a Unix path agrees with spec 38 and with
a pin that varies the path against the root. Either (a) a path outside
the root is refused at acquisition and creates no inode, or (b) spec 38
states, as it does for Process, that a `Unix` grant is not bounded by the
FS root, and a pin holds that reach. AC-0 rules which.

## Settled inputs (main `99d594ff7`; anchors re-checked at release)

- **The guard.** `NetScopeV1::admits` (`crates/ken-host/src/capability.rs`
  ~`:248`) admits every `SocketAddressV1::Unix` on the class bit alone. The
  `Cap` carrying it also holds `scope: FsScope`, which the check never
  reads. `effect_v1.rs` ~`:3126` is the only Net check before acquisition,
  and it serves both listen and connect.
- **The syscalls.** `net_v1::listen` (`:149`) and `connect` (`:183`)
  pass the absolute path to `bind` and `connect` unchanged. They do not
  resolve it through the root handle.
- **The admission gate.** `program_admission.rs` ~`:165` requires `FS
  AFull` for `Unix`. That checks the rights level, not the location.
- **The spec.** Spec 38 ~`:1546` says "`Unix` covers absolute Unix paths"
  and requires `FS AFull`. Spec 38 ~`:1485` excludes abstract names
  because they "lack this slice's filesystem-scoped authority model".
  Spec 38 ~`:1126` states that the FS root is not a child sandbox for
  Process. Net has no such statement.
- **The measurement.** The Adversary probe exits 84 for `append_file`
  outside `./jail` and 0 inside it. Under the same root, it exits 0 for
  `with_socket_connection` to `outside/svc.sock`, with the peer
  accepting, and for `with_socket_listener` on `outside/made.sock`, which
  creates the file. The existing PX11 Unix rows use a path inside the
  default root, so they cannot see this axis.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The Net acquisition check and the Unix `bind` and `connect` path in
`ken-host`, spec 38's `Net` class text, the PX11 Unix rows and seeds that
the ruling moves, and one new pin. No change to the Loopback or Inet
classes, the abstract namespace, or the FS act semantics.

## Acceptance

- **AC-0 (Architect, before code).** Rule (a) or (b), against
  PRINCIPLES' no-amplification and honesty-about-the-boundary items and
  spec 38 ~`:1485`. For (a), also rule:
  - the confinement mechanism, either resolution beneath the root handle
    (symlink-safe, within the 108-byte `sun_path` limit) or a lexical
    prefix test, with the symlink and `..` cases each one admits;
  - the refusal's `IOError` identity;
  - the default-root behaviour when no root is declared;
  - every consumer of a Unix row across `crates/`, `conformance/` and
    `spec/`.
- **AC-1.** One test varies only the path against a declared root,
  across four rows: listen and connect, each inside and outside the root.
  Under (a), the two outside rows refuse with the ruled identity, and no
  socket file exists outside the root afterwards. Under (b), all four
  succeed, and spec 38 states the unbounded reach.
- **AC-2 (QA mutation).** Under (a), make `admits` ignore the root again.
  Both outside rows redden, and the inside rows stay green. Restore it
  byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The ruling needs a new authority class, a TCB change, or a change to
  the abstract-namespace exclusion.
- Confinement cannot be symlink-safe within `sun_path`'s limit. Report
  that to the Architect; do not fall back to a lexical test unruled.

Checks: 2 fired. I read `admits` on `99d594ff7`: the `Unix` arm returns
on the class bit with no other input. The fan-in to out-of-root reach is
the FS acts (confined), Process (stated as an escape), and listen and
connect (both through the one check at `effect_v1.rs` ~`:3126`). 8 fired.
AC-1's outside rows are the observation that separates (a) from the
current host: today they succeed, and under (a) they refuse. The inside
rows guard against an over-broad fix that refuses every Unix path.
