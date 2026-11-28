---
id: PX11-DATAGRAM-UNIX-SENDER-STALE-NAME
title: "A Unix datagram queued by a socket that has since been released is reported as coming from a later socket that reused the same kernel-recorded name, and a reply to that reported sender reaches the later socket: the per-process sender table matches the name of a released bind's /proc/<pid>/fd/<P>/<leaf> spelling once a new bind reuses descriptor P with the same leaf. Report a Unix sender only as an address that bound the name and was live when the datagram was sent, or report it unattributed, and correct spec 38 §1.13.1's sender-identity clause and residual to the rule"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_3aw3ffpwd5cnx on PX11-DATAGRAM-SOCKETS (83bfb18a4), measured: the probe exits 99 (the reply lands on the later socket) and its control exits 53. Correctness, not soundness: a reply still passes the replying socket's own grant admission. Operator 2026-11-27: L1b handles the Linux ABI campaign nodes that can run alongside L1. Steward-filed per COORDINATION section 2."
---

# A released Unix datagram sender is not reported as a later socket

## Objective

A Unix datagram's reported sender is never the validated address of a
socket that did not send it. If the sending socket was released before the
datagram is received, and a later bind registered the same kernel-recorded
name, the receive reports either the original sender's address or no
validated address. It never reports the later socket. A reply to the
reported sender therefore never reaches a socket that did not send the
datagram.

## Settled inputs (main `83bfb18a4`; anchors re-checked at release)

- **The mechanism (Adversary `evt_3aw3ffpwd5cnx`).**
  - A Unix bind's kernel-recorded name is `/proc/<pid>/fd/<P>/<leaf>`, and
    P is the bind's pinned parent descriptor.
  - Release drops the table entry and closes P (`Drop for
    DatagramSocketV1`, `crates/ken-host/src/net_v1.rs:485`).
  - A later bind in another directory with the same leaf can reuse
    descriptor number P, so it registers the identical name.
  - A datagram still queued from the released socket carries the old name.
    The sender lookup (`net_v1.rs:725`) matches it to the later socket's
    live entry and reports that socket's validated address.
- **The reproducer.** One interpreter process, root R, `FS AFull` and
  `Net Unix`:
  1. Bind B at `R/b.sock`.
  2. Inside B's bracket, bind A at `R/a/s.sock`, send 7 bytes to B, then
     release A.
  3. Bind C at `R/c/s.sock`.
  4. Inside C's bracket, B receives and replies to the reported sender.

  C receives the reply, and the probe exits 99. The control places C at
  `R/c/t.sock`: the names differ, the raw `/proc` spelling is reported,
  and the reply is refused with `CapabilityDenied` (exit 53).
- **The spec claim is too strong.** Spec 38 §1.13.1, "Sender identity"
  (`38-ffi-io.md:1894`), reports a name equal to a live entry as that
  entry's address. Its residual (`:1906`) covers only an unlink by a
  separate FS act and a bind by another process. This case needs neither.
- **The existing pin misses it.** `unix_datagram_name_registry_lives_from_
  bind_to_release` (`net_v1.rs:1166`) checks one socket's entry before and
  after release. It never rebinds the name while an old datagram is
  queued.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The host's Unix sender attribution, spec 38 §1.13.1's sender-identity
clause and residual, and the paired test rows. Out of scope: the IPv4 and
IPv6 sender rules, the bind and send authority checks, any change to an
operation code or resource kind, and native lowering.

## Acceptance

- **AC-0 (Architect, before code).** Rule the attribution rule and its
  mechanism, for example:
  - a bind name that no released bind in this process can share, such as a
    per-bind directory spelling or a descriptor held for the process
    lifetime;
  - or reporting no validated address for any name that a released bind
    has used.

  State the rule's cost to the existing live-sender case, rule whether
  §1.13.1's residual still holds, and give the exact replacement text.
- **AC-1, behavior.** The reproducer, as an interpreter row, does not
  deliver the reply to C. The control row still reports the raw path and
  refuses with `CapabilityDenied`. A live sender's datagram is still
  reported as its validated address.
- **AC-2, mutation.** Revert the attribution change. The AC-1 reproducer
  row reddens and the live-sender row does not. Restore byte-identically.
- **AC-3.** Every touched test target passes on the exact candidate with
  no name filter, and QA re-runs it there. Full-workspace CI is green on
  the exact candidate.

## Stop conditions

- The fix needs an operation, code, resource kind or authority check
  changed.
- The fix needs a kernel rule or trusted postulate. That is an operator
  stop.
- AC-0 finds a further misattribution path the rule does not close.
  Report it rather than widening this node.

Checks: 4 fired. I read the Drop impl, the sender lookup and the
§1.13.1 sender clause and residual on `83bfb18a4`; they match the
Adversary's mechanism. 8 fired. The reproducer and its different-leaf
control separate a repaired attribution from one that only stops
matching. The live-sender row guards the case the table exists for.
