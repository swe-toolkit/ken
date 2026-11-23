---
id: PX11-NATIVE-STREAM-SOCKET-PARITY
title: "PX11's first slice runs stream sockets only on the interpreter: the seven Net operations are RepresentedUnavailable, executor_admits carries a named exception for exactly them, and a native build of any Net-declared program refuses with host_effect_lowering_unavailable, so under the operator's bar the slice delivers no PX11 value yet. Lower the seven operations natively with interpreter parity, promote them to NativeTested and close the interpreter-admission set again"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX11-STREAM-SOCKET-CONNECT-ACCEPT, RT-NET-UNIX-PATH-FS-ROOT-SCOPE, RT-NET-UNIX-STREAM-LOCAL-ADDRESS, RT-NATIVE-BYTES-TO-LIST, RT-NATIVE-CLOSURE-INDEX-COST, RT-NATIVE-RESULT-JOIN-PHASE]
blocks: [PX11]
github: null
origin: "Steward ruling arm I (evt_y7p1h3h3ybys) on PX11-STREAM-SOCKET-CONNECT-ACCEPT's AC-0 (Architect D0 evt_23ptn1v4a1jkm): native parity is that slice's immediate successor and closes the interpreter-admission set. docs/program/10-linux-abi-completion.md:84-87 (an operation counts only with native parity). Steward-filed per COORDINATION section 2."
---

# Stream sockets run natively with interpreter parity

## Objective

A Ken program that listens, accepts, connects, sends, receives and shuts
down a stream socket builds natively and behaves exactly as it does on
the interpreter. The seven `Net` operations are `NativeTested`, and
`executor_admits` again admits only `NativeTested` operations.

## Settled inputs (PX11 candidate `6bc0cc26e`; re-measure on its landed squash)

- **The interpreter-admission set.** `HostOpV1::availability` marks
  `NetListen` to `NetLocalAddress` (`0x0701`-`0x0707`)
  `RepresentedUnavailable`. `executor_admits` admits exactly those seven
  besides `NativeTested` (`crates/ken-host/src/effect_v1.rs:250`-`:270`),
  pinned by `executor_admits_exactly_native_tested_plus_px11_interpreter_set`
  (`:6169`).
- **The native gate.** `prepare_native_program_from_elaborated` refuses
  on `plan.net_declared` with `host_effect_lowering_unavailable`, before
  plan bytes and objects (`crates/ken-elaborator/src/compiler_driver.rs:2647`).
  `net_declared` is set from `checked.net_classes` (`:4235`) and is part of
  the canonical plan bytes (`:4745`). The Architect measured a second
  fail-closed plane: `host_effect_wire_layout_v1` (`effect_v1.rs:492`)
  returns `OperationUnavailable` for all seven (`evt_5mj97002r0088`).
- **The interpreter contract.** PX11's AC-1 to AC-3 rows and fixtures
  (`crates/ken-cli/tests/px11_stream_socket.rs`,
  `crates/ken-cli/tests/fixtures/px11_*.ken`), the host backend
  (`crates/ken-host/src/net_v1.rs`: `SOCK_CLOEXEC`, `accept4`,
  `MSG_NOSIGNAL`), the `Net` grant classes and their pre-syscall refusals,
  and spec 38 §1.13 as landed. The native build must match these; it does
  not redefine them.
- **The promotion precedent.** `RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION`
  and `PX10-NATIVE-SPAWN-PARITY` moved operations to `NativeTested` with
  parity rows against a fresh interpreter run. Receive and send share the
  resource buffer path (`with_pipe_and_buffer_mut`) under their own
  codes.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-20)

Architect D0 `evt_4k6401k1tgy9k`, anchored on the routed PX11 SHA
`322528a98`. Re-measure its movable-anchor list on the landed squash at
release; a mismatch is a stop to the Architect. Steward rulings:

- **Two increments, no partial promotion.** Inc1 is the host ABI plane:
  the C request records, the factored native decode, the `SocketAddress`
  reply arm that replaces `set_reply`'s `Success(_)` wildcard, and Net
  grant transport as one `NetScopeV1` bit set through the plan byte and
  native init. Inc1 changes no availability label and neither lane's
  behaviour. Inc2 is the lowering plane, the promotion of all seven, the
  deletion of the gate and the exception, and the parity rows.
- **The refusal fan-in is five planes** (P1 to P5 in the D0), not the
  two named in Settled inputs. The D0's rows I1-a to I1-e govern Inc1.
  Its refined AC-1 governs Inc2: known answer as well as equality, an
  Inet-bit row, an fdinfo `O_CLOEXEC` row and a stated non-binding
  profile. So does its AC-2: mutations (a) to (d).
- **Spec 38 §1.13's lane-status paragraph** is a status edit routed with
  Inc2 as a Spec co-turn. It does not fire the stop on changes to spec
  38, which concerns the contract.
- **Arena-cost rows for Net** are out of scope. They are carried, not
  framed, until both the arena-cost instrument and
  `LANG-NATIVE-CLOSURE-DEAD-BINDER-CAPTURE` have landed.
- Probe arm S-NO (B3) and the D0's other named stops return to the
  Architect.

## Amendment: Inc2 ResourceKind closure (2026-11-23)

Inc1 merged (`f09912a10`). The Inc2 re-measure held (Architect
`evt_47czmv4vck7av`). At Inc2's third symptom-inventory entry, the
Architect measured seven native copies of the host-owned ResourceKind
population, each frozen at a different wave (validator 3, planner and
emitter 7, host 9; `evt_42ncdsdxj34kk`). Steward ruling
`evt_6fpdh2m2ywcvm`: Inc2 carries the closure in place of A2.

- **C1.** ken-host owns one ResourceKind population, built on an
  exhaustive match over `ResourceKindV1`. It replaces the separate
  `resource_kind_*` wire fields, which are removed, not kept alongside.
- **C2.** Every native consumer reads C1: the validator's three kind
  guards, the planner's alternatives, the emitter's list, and one
  exhaustive kind-to-role-and-symbol map.
- **C3.** A native totality row through the validator's admitted set,
  and two mutations. Dropping a kind from the map fails to compile.
  Dropping it from the validator only turns the row red.
- **C4.** If an in-crate harness can inject a ReleaseFailed reply, a
  known answer for kinds 4 (ChildProcess) and 8 (SocketStream).
  Otherwise the GAP is stated.

This closes a defect on main: the native validator refuses ReleaseFailed
and ResourceKindMismatch replies for the four PX10 process kinds. The fix
is claimed only when C3's row covers them.

## Scope

Native lowering and the native host leaf for the seven `Net` operations,
the native transport of the `Net` grant classes, their availability
labels, and the removal of the `net_declared` gate and the
interpreter-admission exception. No change to the interpreter's
behaviour, the `0x07xx` request and reply wire, the spec 38 contract, the
`Net` grant classes or `System.Error`'s socket arms. Datagrams, option
families and the resolver stay successors.

## Deliverable

One runtime candidate, or the increments AC-0 rules.

## Acceptance

- **AC-0 (D0, Architect, before code).**
  - Rule the native lowering of the seven operations: reuse of the
    resource stream path for receive and send, and the native form of the
    address record.
  - Rule how the native runtime receives the `Net` grant classes, so an
    out-of-class or revoked grant refuses before any socket syscall, as
    on the interpreter.
  - Rule the split into increments, if any. An increment that promotes
    only some operations keeps the `net_declared` gate until the last one
    lands.
  - Rule whether the arena-cost row family covers a promoted `Net`
    operation here or as a successor.
- **AC-1 (parity).**
  - PX11's AC-1 loopback program, over `127.0.0.1`, `::1` and a Unix
    path, builds natively. Its output and exit status equal a fresh
    interpreter run.
  - PX11's AC-2 typed-failure program does the same: the missing
    listener and the out-of-class grant.
  - PX11's `FD_CLOEXEC` row holds natively on a listener, an accepted
    stream and a connected stream.
- **AC-2 (the set closes, QA mutation).**
  - `executor_admits` admits exactly the `NativeTested` set, with all
    seven `Net` operations in it. The PX11 pin is replaced by one that
    names that set.
  - PX11's AC-4 native-refusal test is replaced by AC-1's positive rows.
  - A mutant that returns one `Net` operation to
    `RepresentedUnavailable` reddens the parity row that uses it, and no
    other row passes in its place. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- Parity needs a change to the interpreter's behaviour, the `0x07xx`
  wire, spec 38 or the grant classes.
- Any trust growth or kernel change. These are operator stops.
- A native row differs from the interpreter on output, exit status or
  the `System.Error` value.

Checks: 9 fired. Every anchor is on the unlanded PX11 candidate
`6bc0cc26e`, so the frame says to re-measure on the landed squash. The
availability match, `executor_admits` and the `:2647` gate were read
there; the wire-layout plane is the Architect's measurement. 8 fired.
AC-2's mutant reddens a parity row, which a lookup that silently
falls back to the interpreter could not pass. 2 fired. The native
refusal has two planes, the plan gate and the wire layout, and the frame
removes the first and requires the second to lower.
