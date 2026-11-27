---
id: PX11-NATIVE-STREAM-SOCKET-PARITY
title: "PX11's first slice runs stream sockets only on the interpreter: the seven Net operations are RepresentedUnavailable, executor_admits carries a named exception for exactly them, and a native build of any Net-declared program refuses with host_effect_lowering_unavailable, so under the operator's bar the slice delivers no PX11 value yet. Lower the seven operations natively with interpreter parity, promote them to NativeTested and close the interpreter-admission set again"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX11-STREAM-SOCKET-CONNECT-ACCEPT, RT-NET-UNIX-PATH-FS-ROOT-SCOPE, RT-NET-UNIX-STREAM-LOCAL-ADDRESS, RT-NATIVE-BYTES-TO-LIST, RT-NATIVE-CLOSURE-INDEX-COST, RT-NATIVE-RESULT-JOIN-PHASE, RT-NATIVE-INDEXED-FAMILY-LOWERING, ELAB-OMEGA-PLAN-ELIM-RULE-TYPES, RT-ORIENTED-PARENT-EDGE-COMPOSITION, RT-STATIC-POSITIONAL-CAPTURE-RUN, RT-RETURNED-VIS-IH-ENDPOINT, RT-CHECKED-IH-FRESH-RESULT-CAPTURE-POPULATION, RT-NATIVE-LOWERING-REPRESENTATION-AUTHORITY, RT-NATIVE-AGGREGATE-DATA-NOT-CODE]
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

## Known native-planner limits, out of scope

Recorded so a later WP does not rediscover them as new walls (Architect
`evt_15q6qwa3yyh67`, found while withdrawing AC-16-3):

- a returned-Vis "unknown endpoint" under curried unfolding;
- "no derivable successor response row".

No small reaching shape below these limits produced the AC-16-3 row, so
AC-16-2(a) and (b), the exchange mutation witness, replace it.

## Inc2 symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. A constructor field's byte span, Int and resource scalar had no planner-issued authority: the seat record is keyed on (occurrence, argument slot) with one need, and a field is not a seat — keyed on field position under a matched constructor root. (evt_2x1vqnv352kxa)
2. The native bracket collector's acquisition inventory and the native synthesized ResourceKind set enumerated only pre-PX11 members — keyed on checked acquisition operation identity and ResourceKind wire tag. (evt_6xzjree7s64wy)
3. The native resource-error reply validator admitted only the FsHandle, Buffer and Mapping kind tags in ReleaseFailed.kind and in both ResourceKindMismatch kinds — keyed on ResourceKind wire tag. (evt_42ncdsdxj34kk)
4. The two erasure capability-operand classifiers treated every non-ambient operation as carrying an explicit Capability operand, except a resource-only list frozen at PX10 — keyed on HostOpV1 membership. (evt_5kc14zehcdrkj)
5. The prelude's socket_unix_path could not lower natively: bytes_to_list is outside the native primitive set, and uint8_to_int refuses a List UInt8 element — keyed on the primitive symbol and the operand's lowered representation. (evt_773gmfx4vmmab)
6. The exchange's native preparation ran past 450 s after the closure-index repair, in the normalized-declaration loop (kernel subst::shift, recursive Term drop, infer_elim/check under check_app_spine) — keyed on the size of each executable declaration's normalized body. (evt_7s7gc8j8vnamx)
7. Oriented frame composition refused exchange main once indexed families reached lowering, because post-order adjacency within a segment stood in for the parent edge — keyed on post-order adjacency of frames within a segment. (asked evt_5art9xrpx28s9; confirmed evt_5z0179ybnz8bd)
8. The static positional capture model refused a 79-capture environment in exchange main, because a fixed-length &'static positional run bounded capture arity at 64 — keyed on captured-environment arity. (asked evt_3ja9x7a1mxxyk; confirmed evt_1552vcx50h6qt)
9. The returned-Vis settlement could not derive response owner 9's returned set: the Call at StaticOriginId(33) has no emitted call edge — keyed on the callee having a static call edge (provisional, pending P-RV). (evt_12a6ccjxmfnjw)
9. The returned-Vis walker resolved callees only through emitted call edges, and a computational IH callee, which lowers in its owning unit, has none — keyed on callee class (an IH binder resolved by checked binder provenance). (asked evt_12a6ccjxmfnjw; confirmed evt_427vemb7ks8fd)
10. checked-IH continuation inheritance: the fresh-result destination record held exactly one capturing closure, but the active frame's Ret constructor child is captured and read by two sibling closures in main — keyed on the cardinality of the record, over a checked relation that is one binder to many captures. (evt_66ga05tmvxvdw)
11. native lowering: a primitive with a runtime bytes operand had no emitted route. bytes_concat folded only two compile-time literals, so a carried accumulator reached the specialized-only else. Keyed on primitive symbol. (evt_7ahjpbxbgvw09)
12. native lowering: a composed computational match received a BorrowedOption scrutinee that its consumer does not enumerate (bytes_at on a carried Bytes, origin 3918, under an Ordinary head 3919 in decl main, one eliminator remaining). The ordinary and producer match consumers both accept BorrowedOption. The composed consumer's let-else at core.rs:5205 refuses it with N0001. Exposed by the entry-11 fix, which made the Bytes operand carried. Keyed on the scrutinee's Lowered variant. (evt_5116n6wy5wt40)
13. native lowering: list_to_bytes has no NativePrimitive row, so the exchange's socket-address builders (prelude net.rs private_socket_ipv4_octets / private_socket_ipv6_octets) refuse at the table's unknown-symbol arm (primitive.rs:422) — keyed on primitive-symbol membership in the native set. Not R: the operand passed specialized_operands_at; R's closure held. Shares 'checked primitive with no native lowering' with entry 5's bytes_to_list half; two members, no closure. (evt_3jygqenew6zyx)
14. native lowering: list_to_bytes's emitted route enumerated only a statically shaped Specialized Cons spine; the exchange's second call (origin 3775, owner PredeclaredFunctionId(33)) passes a carried List UInt8 word and refused at the root arm (primitive.rs:1260) — keyed on the operand being Carried. (evt_43dft16vbc30q)
15. native lowering: bytes_to_list's synthesized Nil/Cons records at seat 2743 are requested under owner Specialization(ContinuationSpecializationId(3)) from generated context u3:93, and the planner's inline synthesized-seat owner census issued none for that owner -- keyed on the census's own statement of which emission owners contain a seat (for a specialization, the syntactic subtree of its selected case body). Reproduced on Inc2's own exchange after AGG exposed it at seat 2709. (this ruling)
16. The finished generated-Result realization relation admitted only staged continuations, contexts and responses as participants. A generated context forwarding a predeclared unit's protocol result (Context(11) -> PredeclaredFunctionId(3)) could never be realized, and the 15-function demand chain behind it starved (N0004). Keyed on the callee's emission class (predeclared unit vs staged generated body), not on whether it publishes through the unit Result frame protocol. Closed by admitting predeclared units as proof-only participants, with realization taken as a greatest fixed point because predeclared units are the recursive population.
17. K context tail-returning a predeclared unit's call publishes the unit's direct-style answer (Result::Ok, InvocationAggregate) where the Ret-only response owner requires ITree::Ret — keyed on which result protocol (unit answer vs K ITree carrier) produced the word.
18. A carried Bytes word at a host byte-span seat was refused before dispatch: observe_carried_bytes_span admitted only the persistent byte-span classes (helper status BOUNDARY_ERR_CLASS on tag 7 InvocationBorrowed), while the primitive carried-Bytes views also admit BorrowedOpaque. The exchange's Unix SocketAddress, built from argv by socket_unix_path and carried through closure captures, reached NetListen as a borrowed word, got MalformedResource with no host call, and main exited 71 — keyed on the carried word's boundary class. (this ruling)
19. CI-red after approval: 12 of 13 px11_stream_socket rows had never run natively. (a) Main's root-scope fixture read argv from the front, but native ProcessInput.arguments includes argv[0], keyed on the lane's argv shape. (b) The typed-error rows converted a carried response identity inside a bracket continuation, which native lowering refuses on main too, keyed on a carried response field flowing into a matched constructor. Fixture-repaired. Both facts were carried out of scope.

§1b classifications:
- At entry 3 (evt_42ncdsdxj34kk): entries 2, in its ResourceKind half, and 3 share a predicate. The native lane holds hand-enumerated copies of a closed population the host owns, each frozen at the wave that wrote it. The closure was ruled. Entry 1 is independent: it was a missing authority record, not a stale enumeration.
- Entry 4 (evt_5kc14zehcdrkj) falls under the entry-3 predicate, widened to host-owned closed classifications.
- Entry 5 (evt_773gmfx4vmmab) is independent of it: a lowering that was never written, not a copy.
- At entry 6 (evt_7s7gc8j8vnamx): No. Entry 6 copies nothing, and entries 1 and 5 are not keyed on term size. A narrower shared property was left unnamed pending measurement.
- At entry 9 (evt_12a6ccjxmfnjw): pending P-RV. The candidate predicate X, expansion, covers entry 6 and possibly 8 and 9. Its closure is the M-a deforestation carry (evt_3fx24w31wxxps). P-RV's V-inline arm confirms X, and its other arms answer No for entry 9.
- The next §1b check is at the 12th entry.
- At entry 9: No for X. The refusal recurs in bind's own body without inlining. Entries 7 and 9 share a narrower property, a proxy relation used where a checked witness existed; it is recorded, with no class closure at two members. (evt_427vemb7ks8fd)
- At entry 10 (evt_66ga05tmvxvdw): No for the 7/9 property; entry 10 uses the checked relation and holds it in a record of the wrong cardinality. Entries 8 and 10 share "a population carried in a fixed-cardinality representation"; recorded with two members and no closure. The next §1b check is at the 12th entry.
- At entry 12 (evt_5116n6wy5wt40): Yes. Entries 5, 11 and 12 share predicate R, a native-lowering consumer that enumerates per site which Lowered representations it accepts. Closure: RT-NATIVE-LOWERING-REPRESENTATION-AUTHORITY. R is independent of 2-4, 7/9 and 8/10.
- At entry 13 (evt_4fhp3j0qtgaz1): Not R. RT-NATIVE-LOWERING-REPRESENTATION-AUTHORITY merged (`fc5953284`), and its closure held. Entry 13 finds no row in the operand table. The fix is a ListToBytes row with its lowering, as Inc2's next commit (Steward `evt_1nqfeawrftn6j`). The next §1b check is at the 15th entry.
- At entry 14 (evt_6a98as689pycd): shares a narrow predicate with entry 13, the entry-13 route scoped by the one operand measured (Check 6 applied too late). Not R: no authority for reading a carried List UInt8 exists to share. The carried arm is the missing lowering, as Inc2's second commit on `8275787cc`. The next §1b check is at the 15th entry.
- At entry 15 (this ruling): Yes, with the entry-3 family: a planner census held its own enumeration of an emission population that lowering owns. Closed by source (g), keyed on the planned context population lowering stages, not a hand-listed seventh case. Relays are deliberately excluded and their population is measured. The next §1b check is at the 18th entry.
- At entry 16 (evt_16agctxpgjnan): recorded for the 18th-entry check: entry 16 has the same shape as entries 3 and 15, a closed enumeration of participants that is narrower than the population lowering produces. The next §1b check is at the 18th entry.
- At entry 18 (this ruling): Yes, two predicates. Entries 16 and 17 share 'the unit-answer versus ITree-carrier protocol at a K context's call into a predeclared unit is not a plan fact'; closed by entry 17's planner fact and per-occurrence wrap, with entry 16's proof-only participants retired. Entry 18 is predicate R (entries 5, 11, 12) at the carried-class level; R's closure governs Lowered variants and held; closed by one carried-Bytes class dispatch, view_carried_bytes_word, shared by all three consumers. The next §1b check is at the 21st entry.
```
