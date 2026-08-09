---
id: RT-TERMINAL-ALL-ELIM-AUTHORITY
title: "Issue the typed terminal-All structured-IH elimination authority upstream in checked erasure/planning, and let only that issued relation license the source-machine Match seat to consume a ComputationalRecursorClosure"
status: ready
owner: runtime
size: M
gate: none
depends_on: [KERNEL-NESTED-IND]
blocks: []
github: null
origin: Architect mechanism ruling evt_33v0hx3k3ygjm (2026-08-09), issued on the discharged D0 record RT-SPECIALIZED-MATCH-ATTRIBUTION exact f8250c5a, merged at f9146b91 (PR #1702, Decision dec_3b9y90ag7m91). The ruling names this successor and requires Steward framing and explicit release before Runtime edits production. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THE BASE THIS NODE NEEDS IS NOT ON `main`. READ THE SEQUENCING SECTION FIRST.
>
> The kernel-issued relation this authority must be derived from does not exist
> in `crates/` on `main`. It exists only on Kernel's held branch. This node is
> framed and **not released**; releasing it against `main` today would ask
> Runtime to build on a relation it cannot read.

Treat every anchor below as perishable. If a fixed input turns out false against
the landed code, say so and escalate — do not quietly build around it.

## What it is

`D0` of [[RT-SPECIALIZED-MATCH-ATTRIBUTION]] measured a refusal and stopped
there. The Architect then ruled the mechanism, and it is narrower than any of
the three candidates the record left open.

**This is a typed terminal-`All` structured-IH elimination authority.** It is
**not** generic ordinary-`Match` widening, **not** terminal propagation, and
**not** an untyped "eliminate before the scrutinee" repair. Those three
readings are excluded by the ruling, not merely disfavoured.

The authority must be **issued upstream**, where checked erasure and planning
still know all five of the following as **one relation**. Downstream, they have
already been separated, which is exactly why the seat cannot decide for itself:

1. the terminal-support family identity;
2. its kernel-owned `(host, parameter, sort)` origin;
3. the host-constructor to support-constructor alignment;
4. the evidence-field topology for each aligned constructor;
5. the exact Runtime `Match` occurrence licensed to consume it.

Only that issued relation may let the source-machine ordinary-`Match` seat
consume a `ComputationalRecursorClosure` through the **existing** checked
invocation/decomposition machinery. Nothing here authorizes new machinery.

## The semantic obligation

Nested iota, `spec/10-kernel/14-inductive.md` §3.2 (nested arguments and lifted
induction hypotheses) and §7.8 (nested ι):

> **Preserve the enclosing host topology, and supply exactly one recursive
> result per contained guest occurrence.**

**A materialized support value and a fused support elimination are both
acceptable representations** if they satisfy that same law. The representation
is the owner's call; the law is not.

## Fixed inputs, measured at `main` `f9146b91`

The `D0` facts, from the merged record
`docs/program/wp/RT-SPECIALIZED-MATCH-ATTRIBUTION.md`:

| fact | value |
|---|---|
| firing remainder operand | `ComputationalRecursorClosure` |
| the ordinary `Match` | `StaticOriginId(53)`, three case constructors |
| eliminated family | `global_574` — kernel-generated terminal `All` support for host `g570` = `Bag`, parameter 0, sort `Type` |
| `Bag` itself | `g570`, and it is **not** the eliminated family |
| preceding scrutinee occurrence | `SOI(52)`, `RuntimeExpr::Var(0)`, binder 0, environment length 3 |
| that binder's role | `Value(Specialized(ComputationalRecursorClosure))` |
| continuation stack at arrival | complete `Terminal` |
| the seat | `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:6178-6183`, the remainder arm after six explicit acceptances |
| remainder width | fifteen of `LoweredVariant`'s twenty-one members; six accepted at this seat |

Landed structure the authority has to travel through, measured on `f9146b91`:

- `DataMetadata` — `crates/ken-elaborator/src/checked_core.rs:1133`.
- Checked erasure reads `semantic.data_metadata` at `erasure.rs:3525`, `:3858`,
  `:3971`, and projects it to the runtime audit record at `:5864-5867` via
  `runtime_data_metadata` (`:5982`).

**The relation itself is absent from `main`.** `git grep` for
`terminal_support`, `all_support_origin`, `support_family`, and `support_origin`
across `crates/` at `f9146b91` returns **nothing**.

Where the pieces actually are, measured per branch — **they are not both on
both**, and the difference decides what the Kernel cut has to contain:

| identifier | `dd3cd050` | `a577f136` (child) |
|---|---|---|
| support registry / generation | present (`ken-kernel/src/{inductive.rs,env.rs,check.rs}`) | inherited |
| `all_support_origin` | **absent** | present — `ken-kernel/src/env.rs:400`, `inductive.rs:1101` |
| `DataMetadata.terminal_support` | absent | present, but only as a **`bool`** (`checked_core.rs:1136`) |

**And the runtime projection drops it.** `runtime_data_metadata`
(`a577f136:erasure.rs:6026`) carries `parameter_count`, `index_count`, and
`constructors` — `terminal_support` is not among them. So even on the held
snapshot, nothing of this relation reaches the Runtime side today.

⇒ **A Boolean and an inverse accessor are not the authority.** `D1` still owns
building the five-part relation; the Kernel cut owns making its answer
authoritative. That is the whole of the sequencing problem below, and it is a
measurement, not an inference.

## Deliverables

- **`D1` — issue the relation.** In checked erasure/planning, emit the
  five-part authority above as one record, keyed so that consumption can name
  it. Derive facts 1 and 2 from the **kernel-issued** relation, never
  reconstructed downstream.
- **`D2` — consume it at the seat.** Let `core.rs:6178-6183` accept a
  `ComputationalRecursorClosure` **only** when the issued authority names that
  exact `Match` occurrence, routing through the existing checked
  invocation/decomposition machinery.
- **`D3` — the nested-iota law.** Preserve the enclosing host topology and
  supply exactly one recursive result per contained guest occurrence.
- **`D4` — the mutation proof.** Six independent reds, per `AC-5`. Follow
  `agent/playbooks/tools/mutation-prove-a-pin.md`.

## Acceptance

| AC | criterion | control |
|---|---|---|
| `AC-1` | The authority carries all five facts **as one issued relation**, derived from the kernel-issued origin | perturb the recorded `(host, parameter, sort)` for the family → consumption must refuse. If it still succeeds, the record is inert and the site recomputed |
| `AC-2` | A nested recursive computation over the `Bag` host **distinguishes `Empty`, `One`, and `Join`** | three-way: each constructor takes its own arm and produces its own result |
| `AC-3` | **Both `Join` leaves contribute** to the expected `Nat` 3 | a fold that drops either leaf yields 2, not 3. A control that only checks "it evaluates" does not discharge this row |
| `AC-4` | **Interpreter and native agree** on that computation | differential; not "native is green" |
| `AC-5` | Six **independent** reds: missing, duplicated, swapped, wrong-host, wrong-parameter/sort, and **same-cardinality foreign-support** mappings | each mutation applied alone must red. The last is the load-bearing one — the generated `All` support for `Bag` has three constructors mirroring its host's, so **a same-cardinality foreign family is exactly what a count cannot discriminate**, and that is the error `a030d2a1` actually made |
| `AC-6` | An **unmarked** ordinary `Match`/capsule pair keeps the present refusal, byte-for-byte | the pre-existing fail-closed path, unchanged |
| `AC-7` | No control keys on `("Match", "scrutinee is not a constructor value")` | the `6164` test hook produces that pair by construction, so a control keyed on it passes for a reason unrelated to the property |

## Forbidden

**Authority may not be inferred from any of these, alone or in combination:**
constructor count, constructor names, graph shape, `terminal_support` alone,
`DirectScrutinee`, or `Terminal`. Each of them is true of the measured
occurrence and none of them licenses it.

Also unauthorized:

- Generic ordinary-`Match` widening; terminal propagation; an untyped
  "eliminate before the scrutinee" repair. All three are excluded by the ruling.
- Letting the capsule become the terminal result. It stays
  **specialization-only and non-transferable.**
- Moving ordinary matching of the source `Bag`, or generic `Match` acceptance.
  Neither changes.
- Re-reviewing or touching `876450ab`.

## Sequencing

**This node is framed and NOT released.** The two conditions are separate and
both are open:

1. **The kernel-issued relation must be on `main`.** Measured above: it is not.
   `D1` cannot derive facts 1 and 2 from a relation that `crates/` does not
   contain, and the `D0` measurement itself only reached those facts on a
   disposable venue composing `main` with `a577f136`.
2. **Runtime is single-threaded.** Re-derive contention against the
   then-current tree rather than trusting this sentence — a disjointness claim
   of the Steward's has died twice on this chain.

### The order is RULED, not proposed

Architect, `evt_26dtkkpc3gngw` (2026-08-09). Kernel-partial-first satisfies the
constraint; it is the only sound way to make the ruled source authority
available without asking Runtime to reconstruct it.

1. Land a coherent **relation-bearing** [[KERNEL-NESTED-IND]] accepted partial
   on `main`.
2. Re-ground Runtime contention, then **explicitly release** this node.
3. This node closes.
4. That result discharges KERNEL-NESTED-IND `D5`, and the Kernel node closes.

**Not circular, and no reverse edge may be added.** This node has a real *input*
dependency on Kernel's issued representation. `D5`'s Runtime-consumability
requirement is a later **acceptance condition** of the Kernel node, not a
reverse implementation dependency. The recorded tracker direction is correct.

**The Kernel partial may land while native execution still refuses.** That
refusal is fail-closed and already measured. The partial must be labelled
exactly that: it does not claim native consumability, does not discharge `D5`,
does not close KERNEL-NESTED-IND, and does not by itself release this node.

### What "relation-bearing" means, and what it does not

**Load-bearing sharpening from the same ruling.** It does **not** mean a naked
accessor or a `terminal_support: bool`. The accepted Kernel cut must coherently
retain, together:

- the generated terminal-`All` declarations;
- their **atomic admission/provenance**;
- the family-to-`(host, parameter, sort)` relation;
- the checked constructor/evidence alignment that relation certifies.

**Do not cherry-pick an inverse accessor detached from the declarations and
transaction that make its answer authoritative.** Neither raw `dd3cd050` nor an
env-only child qualifies — see the per-branch table above for why.

**Conversely, the Kernel partial need not pre-build `D1`'s finished Runtime
projection.** `D1` still owns carrying the family identity, origin, constructor
alignment, evidence topology, and the exact `Match` occurrence as one
checked-erasure/planning relation. That is precisely why this node stays held
until the coherent source cut is actually on `main` — and not one moment longer.
