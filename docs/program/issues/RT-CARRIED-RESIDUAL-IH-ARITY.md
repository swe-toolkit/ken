---
id: RT-CARRIED-RESIDUAL-IH-ARITY
title: "Close the BoundaryCarrier refusal that a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments -- the arity property `reject_carried_residual_arguments` decides against a carried residual before any invocation segment is installed. Four ignored rows carry this signature byte-identically and their labels already name it; the design question is whether the repair belongs at the shared refusal or at its five call sites."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-17, sizing the repair program from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger (488acf24e) on operator directive 2026-09-15 'The other tests should be fixed.' The ledger's S1 is the largest signature cluster whose label AGREES with the run -- four rows, one byte-identical message, one named mechanism. Steward-filed per COORDINATION section 2."
---

# The defect, and it is already diagnosed in the tree

Four ignored rows fail with one byte-identical signature, measured from runs
at `c041de7c3` by `RT-IGNORED-FAILING-ROWS-INVENTORY` (ledger `488acf24e`,
`S1`, 4 of 15 rows):

    unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
    hypothesis is an eliminated value, not a callable, so it takes no
    arguments, but the call provides 1

**The rows, at `c041de7c3`:**

    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:153
      delayed_capturing_generic_bind_agrees_across_real_executors
    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:220
      runtime_selected_non_unit_response_is_consumed_across_real_executors
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:153
      dynamic_ok_payload_selects_a_multistep_tree_across_real_executors
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:185
      dynamic_err_payload_selects_a_multistep_tree_across_real_executors

## These rows are NOT stale, and the Steward's first reading of them was wrong

**All four `#[ignore]` labels are byte-identical and they predict the signature
exactly** — including the arity clause:

    #[ignore = "RT-SITEOP-CARRIED-WITNESS D2: the carried SiteOperand port
    succeeds; this row next refuses because a carried recursive hypothesis is
    an eliminated value, not a callable, but the call provides 1"]

I ruled the sizing on the reading *"four rows are blocked on a deliverable
that landed a month ago and still fail — either the repair was incomplete or
the rows were never blocked on it."* **Neither is true.** `RT-SITEOP-CARRIED-
WITNESS` landed its deliverable (`merged` 2026-08-17, PR #2557), and these
labels are **chain labels**: they record that D2 succeeded and name the layer
behind it. The row was correctly sequenced by whoever wrote it.

⇒ **This is not an investigation into why a row still fails. The mechanism is
named, the refusal function is named, and its witness is named.** That is why
this node is `S` rather than the diagnosis node the ledger's other clusters
need.

**These four are two of only five rows in the ledger whose label agrees with
the run** (`S1` x4 plus `S4` x1). The other ten labels name a mechanism the
run does not exhibit — which is what makes this the cluster to cut first and
**not** evidence about the other eleven.

# The refusal, located

    crates/ken-runtime/src/cranelift_backend/lowering/core.rs:3052
      pub(super) fn reject_carried_residual_arguments(arguments: usize)
          -> Result<(), CraneliftBackendError>

`crates/ken-runtime/src/cranelift_backend/lowering/aggregates.rs:3340-3356`
names it, deliberately, as the layer behind the carried SITE-OPERAND
projection, and rules it **out of scope for that projector in the strong
sense**:

> *"it is an arity property of an IH invocation against a carried residual,
> decided before any invocation segment is installed, and it does not become
> reachable or unreachable by anything this projector does. Widening this
> dispatch cannot close it and must not try."*

**That comment is an asset, not background.** A previous seat measured the
boundary, recorded the witness so a later reader would not mistake a still-red
enumeration row for an unfinished projection, and said which repair would be
the wrong one. **Read it before touching anything.**

Its cold-lowering enumeration witness is `rt_allocate_stage`
(`crates/ken-cli/tests/rt_cold_lowering_path_enumeration.rs:156`, dispositioned
`Completes` at `:547`).

# The design question, front-loaded — THE FAN-IN IS THE NODE

`reject_carried_residual_arguments` is **called from five sites**, measured on
`origin/main` at `fe649049f`:

    crates/ken-runtime/src/cranelift_backend/lowering/core.rs:3101
    crates/ken-runtime/src/cranelift_backend/lowering/core.rs:6063
    crates/ken-runtime/src/cranelift_backend/lowering/core.rs:16133
    crates/ken-runtime/src/cranelift_backend/lowering/source.rs:5016
    (definition at core.rs:3052)

**A fix at the definition changes all five paths. A fix at one call site
changes one.** Which is correct depends on a question nobody has answered:
**is a carried recursive hypothesis non-callable at every one of those five
sites, or only at the site these four rows reach?**

> **This is why the node is `T1` despite being four rows.** The repair is
> small; **deciding which of the five paths it is a property of** is not, and
> getting it wrong at the choke point silently changes four paths nobody
> measured.

**Do not assume the shared function is the right unit because it is shared.**
A choke point is a claim about fan-in, and the claim has to be checked rather
than inherited from where the code happens to put the helper.

# The ledger is PROVENANCE, not a dependency — `depends_on` is empty

**The schema checker flagged an earlier cut of this node as `ready` against a
`depends_on` that had not landed. It was right, and the fix is that the
relation was miscategorised rather than that this node should wait.**

**A read is not a dependency.** `RT-IGNORED-FAILING-ROWS-INVENTORY` is where
the `S1` signature was measured, and it is cited at `488acf24e`. This node does
not need it on `main` to start: **every input in §2 of the frame was
independently re-derived from the tree** — the four rows and their line
numbers, the byte-identical labels, `reject_carried_residual_arguments` and its
five call sites, the `aggregates.rs` scope comment, and the
`rt_allocate_stage` witness. The one input taken from the ledger rather than
re-run is the **observed signature**, and that is a measurement at a fixed SHA
whose validity does not turn on a merge.

⇒ **If the ledger were never to land, this node still stands**, because the
frame carries its inputs inline rather than by pointer. That is the test for
provenance-versus-dependency and it is the reason the edge is not declared.

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that measured `S1`; its
  §3 holds the signature and its §4 the clustering argument. **Provenance, not
  a blocker.**
- [[RT-SITEOP-CARRIED-WITNESS]] — `merged`; landed the D2 port these labels
  record as succeeding. **Not a blocker on this node.**
- The ledger's `S2` cluster (4 rows, planner-invariant) is the other node this
  sizing cuts. **Separate mechanism, separate crate surface; do not fold.**
