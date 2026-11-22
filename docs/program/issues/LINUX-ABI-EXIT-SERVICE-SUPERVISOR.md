---
id: LINUX-ABI-EXIT-SERVICE-SUPERVISOR
title: "The Linux ABI program exits when a Ken network service and a Ken process supervisor run as native, single-threaded, event-driven binaries with no unsafe pointers in application Ken, and their protocol state machines and path policy are proved (program §6). No node delivers those two programs. Build and prove them on the completed floor"
status: draft
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10, PX11, PX12, RT-CLOCK-OP-NATIVE-PROMOTION, RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC, RT-FS-DESCRIPTOR-INHERITANCE-DUPLICATE, ABI-M2, ABI-S4, ABI-S2, ABI-FSCREATE-RECURSIVE-CONTRACT]
blocks: []
github: null
origin: "Steward 2026-11-22 on operator direction (\"Create WPs and update the campaign docs as necessary to capture the gaps you have identified\"). The exit in docs/program/10-linux-abi-completion.md §6 names two programs; PX12 describes itself as the last node before that exit, and no node delivered it. L1 Linux ABI program, Steward order."
---

# The program exit: a service and a supervisor

Draft. The frame is written when PX12 nears landing, because PX12's
readiness surface decides both programs' event loops.

## Objective

Two Ken programs, built natively and run in CI:

- **A network service.** It accepts stream connections, serves a small
  request and response protocol, and shuts down on a signal.
- **A process supervisor.** It spawns children from declarative plans,
  observes their exits and signals, and restarts them by policy.

Each runs single-threaded on PX12's readiness loop. Application Ken in
each holds no unsafe pointer. Each has a checked proof of its protocol
state machine and of its path policy. The lifetime claims say
`runtime-enforced` and `tested`, never `proved` (charter PX7).

## What the frame owes

- The protocol for each program, and the property each proof states.
- The proof's home: a catalog package, or the program's own module.
- The native differential rows that observe each program end to end.
- A stop condition for a proof that needs a kernel or trust change.
