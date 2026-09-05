---
id: ABI-REVOKE-D0
title: "RevocationDomain substrate + lineage tree — the host-internal authority-lineage foundation for the revocation membrane; no dispatch wiring, lands green with zero observable Ken change"
status: active
owner: runtime
size: S
gate: none
depends_on: [ABI-R3]
blocks: [ABI-REVOKE-D1]
github: null
origin: "Architect decomposition of ABI-REVOKE, evt_30z9y81yxvdyv (2026-09-05), grounded from origin/main against ADR-0022 plus the merged behavioral contract (spec @9ebebb8e, PR #865). ABI-REVOKE is now sizable — both of dec_p1dv4gw6bsc2's before-sizing prerequisites are discharged (behavioral contract merged; Runtime ADR-0022 landed 2026-09-05) and ABI-R3 is merged. It decomposes into three increments (D0 S + D1 M + D2 M, ~M-L total); D0 is the independently-testable, risk-free foundation the Architect directed be kept as its own increment. Cut and released by the Steward per COORDINATION §2 and the operator's 2026-09-05 runtime-lane ruling (runtime lane-1 objective = ABI-REVOKE)."
---

> # THREAD ANCHOR
>
> This node's kick is the thread anchor for `ABI-REVOKE-D0`. Reply with
> `parent_event_id` set to the kick's event id; thereafter use its `thread_id`.
> Do not open a second thread for this WP.

## Objective

Build the `RevocationDomain` substrate and its lineage-tree semantics as a
**host-internal** structure owned by `ProcessContext` — the authority projection
of Ken's current implicit root execution space (Architect ruling
`dec_p1dv4gw6bsc2` §4). This increment adds the domain and its API and unit-tests
its tree semantics in isolation. **It does NOT wire the domain into
`dispatch_host_op_v1`** — no admission check, no capability/resource lineage, no
error identity. Those are D1 and D2. D0 lands green with **zero observable Ken
behavior change**, which is exactly why it is the safe foundation the rest stands
on.

## Why this is sizable now (grounding, re-measure before building)

The Architect grounded this from `origin/main` (evt_30z9y81yxvdyv). The two
before-sizing prerequisites `dec_p1dv4gw6bsc2` required both exist:

- **Behavioral contract MERGED** — spec `@9ebebb8e` (PR #865): the two public
  projections and admission/settlement are pinned; `attenuate`/`revoke` stay
  non-Ken-visible (I-4).
- **Runtime ADR-0022 LANDED** (2026-09-05): the full mechanism decision plus the
  five-clause structural closed-world isolation argument the ruling required.
- **ABI-R3 MERGED** — the implementation gate; its generated inventory is the
  closed admission boundary (ADR-0022 trust-clause 4). So `PX8 → ABI-R3 →
  ABI-REVOKE` is satisfied.

Every anchor below is perishable — re-measure each cited line against the landed
tree and escalate if a fixed input is false; do not build around it.

## The load-bearing architecture fact (why there is no TCB/option-(b) fork)

Every guarded host op on **both** engines routes through the single shared
`dispatch_host_op_v1` (`ken-host/src/effect_v1.rs:1536`) — called by the
interpreter (`eval.rs` ×4), the native ABI path (`abi_v1.rs:1424`), and native
runtime (`object_linker_packaging.rs`). D1's admission hook lands at that one
choke point and both engines inherit it. The whole membrane is host-runtime in
`ken-host`: **no native-backend work, no new `ir::RuntimeExpr` variant, no value
crossing a cranelift fn-return, no kernel `Term`.** GATE-0 resolves to
**option-(a) by construction** — zero kernel/TCB delta. D0 touches none of the
dispatch path at all, so it is trivially option-(a).

## Deliverables (Architect D0, verbatim in substance)

**`D0.1` — `RevocationDomain` as a third field on `ProcessContext`**
(`abi_v1.rs:314`, beside `capabilities`/`resources`), minted in
`admit_root_execution` (the ADR-0019 single admission point, before any
capability insert) and torn down with `ProcessContext`. **Thread the field
through all three `ProcessContext` ctor sites** (`756`/`1523`/`1605`).

**`D0.2` — `RevocationNodeId`**: opaque, monotonic counter, never reused,
non-Ken-visible, a **distinct id space** from `CapabilityTokenV1` and
`ResourceTokenV1`. No raw pointer or reference to a validity cell crosses the
host boundary.

**`D0.3` — the host-internal domain API:**

- `mint_root`;
- `attenuate(parent) → child-with-parent-link`;
- `copy` preserves id (**no new node**);
- `revoke(node) → close the node and all descendants` (never parent/siblings);
- `is_admissible(node) → node AND every ancestor live` (walk the ancestry; a
  cached leaf-live bit is insufficient).

## Acceptance criteria

**`AC-TREE` — unit tests on tree semantics ALONE**, one per property, each named:

- `copy` preserves node id (no new node minted);
- `attenuate` creates a child with a parent link;
- `revoke` closes the addressed node's whole subtree and leaves parent and
  siblings live;
- `mint_root` / id-space: ids are monotonic, never reused, and disjoint from the
  capability/resource token id spaces.

**`AC-ANCESTRY-CONTROL` — the ADR's rejected alternative as a RED control.** An
ancestor `revoke` performed **after a leaf was already observed live** must make
that leaf `is_admissible → false`. **Control:** neuter the ancestry walk to a
cached leaf-live bit and this test must go RED. This proves admission walks the
ancestry rather than reading a stale leaf cache — the exact soundness the whole
membrane rests on.

**`AC-INERT` — zero observable Ken change.** The three `seed-capabilities.md`
oracle cases (`:96` `revoked-path-operation-is-distinct-fileerror`, and the two
resource cases) **stay RED / deferred** — D0 turns none of them green. No Ken
program observes any new behavior. **Control:** the runtime lib suite and
`px8f_buffer_native` are green with no diff to any oracle expectation.

**`AC-NO-REGRESSION`** — green in CI (`COORDINATION §12`). Targeted locally:
`-p ken-host` as touched. Never `--workspace`.

## Banned scope

- **NO dispatch wiring.** `dispatch_host_op_v1` is untouched in D0 — no admission
  lease, no `is_admissible` call on the guarded path. That is D1.
- **NO capability/resource lineage on the slots, and NO error identity.**
  `CapabilityGrantV1`/`ResourceTableV1` slots gain no `RevocationNodeId` in D0;
  `FileErrorCauseV1::Revoked` / `ResourceErrorV1::Revoked` are D1/D2.
- **DO NOT build concurrency machinery.** Today's unique synchronous
  `ProcessContext` drains immediately, so lease/atomicity/drain are
  single-threaded invariants; ADR-0022 reserves the proved atomic/locked
  linearization for PX12 and forbids inheriting today's proof by assertion.
  Building it now is inventing scope.
- **DO NOT reopen the elaborator static contract** (`RevocationHandle`,
  `check_revocation_transitive`, `discharge_attenuation`) — I-4; this is a
  separate host lineage, non-Ken-visible.

## Capability tier

**T2.** A host-internal data structure with a fully-specified API and
tree-semantics unit tests — the Architect called it "the risk-free foundation,"
trivially option-(a). The Architect is the required reviewer of record on the
candidate (against the ADR-0022 clauses); that review is the correctness gate,
not the implementer's tier. Confirm the runtime seat's live model at kick.

## Sequencing

D0 → D1 (`ABI-REVOKE-D1`, authority-side lineage + admission + the path-side
error identity) → D2 (`ABI-REVOKE-D2`, resource provenance + close-after-drain
settlement). D1+D2 may merge into one M increment at the ring's discretion (they
share the dispatch edit); **D0 stays its own S increment.** Each increment lands
green. The Architect reviews each candidate against its ADR-0022 clauses and the
mapped oracle case.
