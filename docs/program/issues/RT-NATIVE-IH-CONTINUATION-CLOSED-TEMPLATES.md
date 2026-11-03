---
id: RT-NATIVE-IH-CONTINUATION-CLOSED-TEMPLATES
title: "A relay that forwards an operation out of a recursive ITree fold re-yields the fold's induction hypothesis as its continuation, and in the native backend that K is a carried runtime word with no planner-typed identity: core.rs refuses a call on a Carried callee, and five D0s found no planned route from a foreign relay back to its row's continuation inputs. Ken's backend is first-order, so a non-code-known K is resumable only from a closed template set. Give each IH-bearing method of a data-motive ITree fold one K identity, enumerated at the backend's K-minting site, resumed by applying that eliminator's own method; an eliminator outside the set refuses closed"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Steward resequence evt_6ppgz0hsqxmm on Research evt_1ebcpy76v1e9a, after PX10 B-closure D0-17 to D0-22; routing D0-23 ruled YES on argv, null and live by the Architect (evt_jc75x84wmcw7). L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A fold's continuation is one of its own methods

## Objective

The native backend resumes a relay's continuation when that continuation
is the induction hypothesis of a data-motive ITree fold. Each IH-bearing
method of each such eliminator has one K identity. Resume applies that
eliminator's method to the forwarded response, selected by the carried
operation word. Any K outside the enumerated set is refused at build
time, and never falls back to a call on a carried word.

## Settled inputs (main `5f070eb32`; measurements at `928313aa6`)

- **The wall.** A relay's K is `CheckedComputationalIHInvocation` with
  body `Call { callee: Var(0), args: [] }` (argv 1245, live 686), and at
  emission it lowers to `LoweringOperand::Carried`. The Carried and
  Specialized callee arms at `lowering/core.rs:18337` refuse; only the
  Closure arm calls (D0-16, evt_ht75yt85jteg).
- **The population (D0-23, evt_jc75x84wmcw7).** The decoded checked-core
  matches number 192, 183 and 192 (argv, live, null). Nine, eight and nine
  carry an IH, and each of those is a relay fold with family and head
  `ITree`: `bind`, `inject_l`, `inject_r`, `drain_pipe` (argv and null)
  and five matches in `main`. No IH match has a `pi` head, and no
  const-headed match contains an IH. Each IH at the function-typed field
  `cont` is `\r. elim (cont r)`, which the eliminator closes itself.
- **Not established.** That population is what
  `decode_supported_match_view` (`checked_core.rs:4461`) decodes. Erasure
  builds `ComputationalMatch` at other sites, including the HostIO
  deforestation route (`erasure.rs:1899`), and no D0 linked a checked-core
  line to the native functions holding the foreign relays (funcid79,
  funcid77).
- **Existing machinery.** An escaping functional IH is materialized as
  its captured environment (`emit_checked_ih_captured_environment`,
  `lowering/aggregates.rs:4065`). Its code identity is the planned static
  worker fixed by the call template. A crossing goes through
  `call_checked_ih_environment_transport` (`core.rs:9313`).
- **Held.** PX10 B-closure's checkpoint `2ad1b31d0` is held on this
  WP. It keeps operation-keyed numbering, the relay as a numbered member,
  and the arm chosen from the carried operation word.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The closed K-identity authority, its census and its refusal arm, in one
runtime candidate. No relay consumer is switched; PX10 B-closure's resume
reads it next.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded timeout.
  The runtime implementer runs it on scratch and reports. The Architect
  rules from that report, and does not build or test. It rules:
  - the K-identity key, (eliminator match, IH-bearing method), and the
    backend site where it is minted and enumerated;
  - the arm table that resume reads, and how the carried operation word
    selects it;
  - the refusal arm and its named reason;
  - whether every erasure route that builds a `ComputationalMatch`
    reaches the minting site, enumerated from the erasure sites, not from
    `decode_supported_match_view` alone.
- **AC-1, census pin.** A named test pins the enumerated set, keyed at
  the minting site, on argv, null and live: the folds behind relay
  K 1241 and 682 resolve to identities, and their count matches D0-23's
  IH-bearing population for those owners.
- **AC-2, refusal pin.** A fold whose IH-bearing method is not in the
  set refuses at build time with the named reason. A mutant that
  falls back to a Carried-callee call reddens a named row.
- **AC-3.** Every native plan output is unchanged, because no consumer
  reads the authority. The 17 D0-14 binaries run base against candidate
  with identical results.

## Stop conditions

- A kernel or trust change.
- An IH-bearing fold whose method result is function-typed but not a
  lexical lambda or an IH composition: that is the full
  `RT-NATIVE-VIS-MAY-YIELD-SUMMARY`'s case, not this WP's.
- A pin that can be met only by switching a consumer.

Checks: 4 fired: the resume obligation is written in the code's
vocabulary (the IH invocation, the Carried refusal at `core.rs:18337`, the
captured-environment emitter), and the one unmeasured link, every erasure
route reaching the minting site, is AC-0's to enumerate. 6 fired: the
population is D0-23's IH-bearing matches counted per fixture (9, 8, 9),
and the census is keyed at the backend, not inherited from the decoder. 8
fired: AC-2's fallback mutant produces the Carried call that the
refusal arm forbids.
