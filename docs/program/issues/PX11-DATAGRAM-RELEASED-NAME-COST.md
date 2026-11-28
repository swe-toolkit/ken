---
id: PX11-DATAGRAM-RELEASED-NAME-COST
title: "The Unix datagram released-name record that STALE-NAME added is a list scanned linearly under the global name mutex on every release and on every receive from an in-process sender, and it gains one entry per distinct released (name, address) for the process lifetime: a server binding per-request sockets at unique leaves slows send and receive from 10.6 to 98.4 us over 20000 releases, and its bind-release total grows quadratically. Make release and receive cost independent of how many names were released, keep attribution unchanged, and state the record's memory cost"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_4rq75pw40c23c on PX11-DATAGRAM-UNIX-SENDER-STALE-NAME (c4ddff1d2), measured in a debug build with the untracked probe crates/ken-host/tests/zz_adv_dgram_released_growth.rs. Operator 2026-11-27: L1b handles the Linux ABI campaign nodes that can run alongside L1; the campaign exit is a service supervisor (docs/program/10-linux-abi-completion.md §6). Steward-filed per COORDINATION section 2."
---

# Releasing many Unix datagram names does not slow later traffic

## Objective

Release and receive on Unix datagram sockets take time that does not grow
with the number of names the process has released. Sender attribution is
unchanged: a live name is attributed only when every released record of
that name carries the live entry's address. Spec 38 §1.13.1 states what the
record retains and its memory cost.

## Settled inputs (main `4cda4f30f`; anchors re-checked at release)

- **The record.** `BOUND_DATAGRAM_NAMES` (`net_v1.rs:582`) holds a serial,
  the live entries, and a `Vec<(Vec<u8>, SocketAddressV1)>` of released
  pairs.
  - `Drop` (`net_v1.rs:615`) scans the whole released list to deduplicate
    before pushing.
  - `datagram_sender` (`net_v1.rs:865`) scans the whole list for every
    receive whose sender matches a live entry.
  - Both scans run under the one global mutex.
- **The measurement (Adversary `evt_4rq75pw40c23c`).** Over 20000
  bind-release-unlink cycles at distinct leaves, send plus receive rose from
  10.6 us with none released to 98.4 us at 20000, and per-op bind plus
  release rose from 48.7 to 148.0 us.
- **Only one fact per name is read.** The attribution test asks, for one
  name, whether every released address equals the live entry's address.
  A map from name to "the one address, or conflicting" answers it without a
  scan.
- **Retention is stated but not costed.** §1.13.1 says released names last
  for the process lifetime (`38-ffi-io.md:1907`), but not that the record
  grows with them.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The released-name record's representation in `net_v1.rs`, the §1.13.1
retention and cost text, and host rows. Out of scope: any change to which
sender is reported, any Net operation, code or resource kind, and the
live-sender rebind residual (folded into `PX11-INJECTED-RESOLVER`'s AC-0).

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - the representation, and whether retention can be bounded without
    reporting a wrong sender; if it cannot, the record stays process-lifetime
    and the spec states its memory cost per distinct name;
  - the census of the record's readers and writers;
  - the §1.13.1 text.
- **AC-1, behavior.** Host rows:
  - the STALE-NAME reproducer and its live and control rows stay green
    unchanged;
  - after many distinct releases (AC-0 picks the count), a receive from a
    live in-process sender still attributes correctly, and a name released
    twice with different addresses still falls back to the raw spelling.
- **AC-2, mutation.** Make the lookup ignore a conflicting released address.
  The two-address row reddens and the live row does not. Restore
  byte-identically.
- **AC-3.** Every touched test target passes on the exact candidate with
  no name filter, and QA re-runs it there. Full-workspace CI is green on
  the exact candidate.

No wall-clock row is a gate; cost is shown by the representation and its
census.

## Stop conditions

- The design changes which sender any existing row reports.
- It needs a change to an existing Net operation, code or resource kind.

Checks: 4 fired. I read the record, its `Drop` writer and the
`datagram_sender` reader on `4cda4f30f`; both scan the released list under
the mutex. 2 fired. The record has one writer and one reader besides bind,
so a keyed form changes two sites. 8 fired. The two-address row separates a
lookup that still sees conflicts from one that only sees the last address,
and timing rows are excluded because CI cannot hold them.
