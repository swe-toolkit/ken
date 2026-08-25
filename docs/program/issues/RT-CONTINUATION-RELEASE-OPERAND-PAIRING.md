---
id: RT-CONTINUATION-RELEASE-OPERAND-PAIRING
title: "Lane-1 successor after the merged carried-Bool eliminator — bind px8ta's next causal object: the generated-continuation-to-specialization ResourceRelease operand pairing. A second logical release attempt in funcid50 selects an operand that is not the origin-171 resource token, failing the exact BoundaryTag::InvocationBorrowed guard in lower_resource_token_seat before host dispatch. D0 produces the exact source-to-slot lineage for funcid50's origin-171 release operand; repair at assemble_continuation_call_operands so the second release attempt disappears"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-BOOL-ELIMINATOR-DISPATCH]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect object read evt_5rec979da0gh6 (thr_5e7s6tnxzgeq0) on px8ta after the carried-Bool eliminator (RT-CARRIED-BOOL-ELIMINATOR-DISPATCH) merged at d82ea01e7. The Architect proved the Bool eliminator is CLEAN (sentinel-replacement probe: host ConsoleIsTerminal(false) reaches canonical carried Bool payload 0 and selects False — false-arm 40, true-arm 41) and DO NOT frame another Bool or case-mapping repair. px8ta advances to a new sentinel but remains in the broader operand-provenance class: a distinct ResourceRelease operand-pairing object. Steward owns the final ID/frame; no Decision fork (the object read fixes the layer + first authority, D0 fixes the exact seam). COORDINATION section 2 framing call."
---

> # CLOSED / FALSIFIED 2026-08-25 — Architect ruling evt_7rhvct552ts2w (hard stop 1)
>
> The Architect UPHELD the implementer's D0 hard stop. The lineage table is
> correct: `Parameter[0][0][1]` is the exact raw capture `v11` (runtime word
> `0x0507`), and `assemble_continuation_call_operands`, the target-1 frame, and the
> origin-171 `Var(1)` binding all preserve it. There is NO wrong operand and NO
> second logical release attempt. This node's TITLE, OBJECTIVE, D0/D1, AC-2, and
> AC-3's "`lower_resource_token_seat` unchanged" clause are FALSIFIED. `D1 as
> framed is unauthorized` (no operand-pairing repair; `core.rs`/`units.rs` stay
> byte-untouched).
>
> The truthful object is a resource-carrier TAG-DOMAIN correction at the consumer:
> `lower_resource_token_seat`'s carried arm compares `emit_carrier_tag` (semantic
> `NODE_TAG_ID` = `0` for a borrowed opaque node) against
> `BoundaryTag::InvocationBorrowed == 7` (a transport tag byte) — two vocabularies
> sharing an integer type. Replaced by the READY node
> [[RT-RESOURCE-SEAT-TAG-DOMAIN]] (Steward, 2026-08-25). Nothing merged from this
> node; closed unbuilt. Symptom inventory durable at
> `architect/rt-release-pairing-inventory @ 1e16fd99`.

> # READY 2026-08-25 — carried-Bool eliminator merged; this is the next lane-1 object
>
> [[RT-CARRIED-BOOL-ELIMINATOR-DISPATCH]] merged at squash `d82ea01e7`
> (blob-audit clean, 12/12), and the Architect's object read (evt_5rec979da0gh6)
> proved that fix CLEAN for px8ta's Bool eliminator. px8ta does NOT close: the next
> failure is later and independently located — a ResourceRelease operand-provenance
> failure. This is the next lane-1 object. Base the successor branch on the `main`
> that carries the carried-Bool merge (`d82ea01e7` or later).

# Fixed inputs (the Architect object read; grounded at landed squash d82ea01e7)

The unchanged ignored row still fails:

```text
scripts/ken-cargo test -p ken-cli --test px8ta_oriented_subcontinuation \
  px8ds_real_same_depth_path_runs_exact_edges -- --ignored --exact --nocapture
```

Trace `BufferAllocate -> ConsoleIsTerminal(false) -> ResourceRelease`, then
`ControlledTrap RuntimeTrap(1)`, exit 1 (baseline log SHA-256
`10a27336a6385e26e84dd79279980dd4f9bdf77b7a264e648d85418e5e1a291c`).

- The Bool eliminator is CLEAN (do NOT re-frame it). The Architect replaced only
  funcid51's `StaticOriginId(615)` Bool-scalar-dispatch false/true arm entries with
  distinct sentinels: the natural run returned the false-arm sentinel (40), so host
  `ConsoleIsTerminal(false)` correctly reaches canonical carried Bool payload 0 and
  selects False. Probe log SHA-256
  `d71c3ed20e898a89be5bd45059aff3f69751e56b44eed265e2179445ef1d5d6f`.
- The next failure, located by numbering emitted `-1` failure constants:
  `RuntimeTrap(20416)` = specialization `funcid50`, zero-based failure site 416 =
  `block1108` in the unmodified CLIF:

  ```text
  v2250 = stack_load.i64 ss164
  v2251 = icmp_imm eq v2250, 7          ; InvocationBorrowed
  brif v2251, block1107, block1108
  block1108:
      v2252 = iconst.i64 -1
      return v2252
  ```

  Numbered-site log SHA-256
  `ab466f4a810eef2eb84e5fa70317bb178163dd430eeb552169d62ca1c17b0be5`; unmodified
  `funcid50` CLIF SHA-256
  `c96665fe36edcc722e566806127d462c2359f52198ec28a149255e2e3ba3848a`.
- Compile-side binding: the site is `effects.rs::lower_resource_token_seat`,
  reached for `ResourceRelease` at `StaticOriginId(171)` in `funcid50`. It is
  CARRIED and fails the exact `BoundaryTag::InvocationBorrowed` guard BEFORE host
  dispatch. The already-recorded successful `ResourceRelease` precedes a SECOND
  logical release attempt whose selected operand is NOT the resource token the seat
  requires. The terminal prose `malformed borrowed process input` is only the
  generic object-linker spelling for final value `-1`; it did NOT classify the
  object.

# Layer and first authority (ruled by the object read)

NOT `joins.rs`, NOT the carried-Bool representation. The failure is the
continuation-call operand assembly / call-frame seam:

- `core.rs::assemble_continuation_call_operands` is the FIRST AUTHORITY — it
  selects the target specialization's ordinary envelope plus the ordered
  continuation inputs.
- `claim_and_call_resolved_continuation_inner` emits that run into
  `ContinuationSpecializationId(1)` / `funcid50`.
- The callee interprets its first parameter's nested `[0][1]` member as the
  origin-171 release resource, but the emitted runtime value is not
  `InvocationBorrowed`.

# Deliverables

- D0 — the exact source-to-slot lineage table for `funcid50`'s origin-171 release
  operand: which operand `assemble_continuation_call_operands` pairs into the first
  parameter's nested `[0][1]` member for the SECOND release attempt, tracked from
  its source through the exact `funcid50` Parameter/Capture frame to origin 171,
  and why it is not the `InvocationBorrowed` resource token.
- D1 — repair at the first authority (`assemble_continuation_call_operands` through
  the `funcid50` Parameter/Capture frame) so the correct resource token reaches the
  seat and the SECOND release attempt DISAPPEARS — not merely accepting a
  non-resource tag. Preserve the existing tag/class guards; do NOT weaken
  `lower_resource_token_seat`.

# Acceptance criteria

- AC-1 — the ignored row `px8ds_real_same_depth_path_runs_exact_edges` advances
  past the ResourceRelease trap: `ControlledTrap RuntimeTrap(1)` at the second
  release is gone (px8ta advances to a new sentinel or closes).
- AC-2 — the fix is by OPERAND PROVENANCE: the second release attempt is eliminated
  because the correct origin-171 resource token is paired, established by
  identity/lineage (the D0 table), NOT by relaxing the `InvocationBorrowed` guard or
  accepting a non-resource tag.
- AC-3 — `lower_resource_token_seat` is UNCHANGED / not weakened; the exact
  `BoundaryTag::InvocationBorrowed` guard and the tag/class guards stay intact (a
  mutation that weakens the guard to accept the wrong operand must RED this AC).
- AC-4 — the already-recorded first (successful) `ResourceRelease` is preserved; no
  regression to the `BufferAllocate -> ConsoleIsTerminal -> ResourceRelease` prefix.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli --test px8ta_oriented_subcontinuation` only, never `--workspace`.

# Reviewers

Architect (the repair sits at `assemble_continuation_call_operands` and eliminates
the second release by provenance, not by weakening the seat guard) + runtime-qa
(the guard-weakening mutation reds AC-3; the fix is measured by operand identity,
not tag acceptance). No Decision fork.

# Capability tier

T1 — an operand-provenance / call-frame repair at the continuation-call seam,
reviewed on the lineage argument (which operand is paired and why), not a
differential diff. Size M.

# Sequencing

Lane-1 (runtime, priority) — the next px8ta object after the carried-Bool
eliminator. px8ta stays OPEN until the native carried-value program lands; this is
the next object in that program, in the broader operand-provenance class. Frame the
seam from `assemble_continuation_call_operands` through the exact `funcid50`
Parameter/Capture frame to origin 171.
