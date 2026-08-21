---
id: RT-CAPTURE-SUPPLY-DECLARED-INPUTS
title: "the branch-local callable-authority cut is necessary but not sufficient -- every witness's residual blocker is capture supply; a capture-bearing LexicalClosure at a recursive position can present its captures as planner-owned declared inputs ONLY if every capture's value is recoverable as a planner-assigned ABI operand with zero read of the carried word -- D0 measures that per witness before any implementation"
status: merged
owner: runtime
size: L
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect D0 approval of RT-BRANCH-LOCAL-DECLARED-CALLABLE at f6b7f0473 (evt_6n9byepz6wzgt, point 3) measured the branch-local partition necessary but NOT sufficient: all 16 witnesses' residual blocker is capture supply, outside RT-BRANCH's scope. The Architect then delivered the fork ruling evt_sjjmcap9293y (grounded on origin/main f5c006ab; the seam files are unchanged since D0 base 6f86c9449), settling all three sub-questions and fixing the node's shape as D0-first. Steward-filed per COORDINATION section 2 and framed to that ruling."
---

# WHY THIS NODE EXISTS

[[RT-BRANCH-LOCAL-DECLARED-CALLABLE]]'s D0 (f6b7f0473, Architect-approved as an
exemplary D0) measured the branch-local `(selected-constructor,
recursive-position)` partition to be **necessary but not sufficient**. All 16
witnesses satisfy Set 1's predicate, but every row's **residual blocker is
capture supply** -- a `LexicalClosure` carrying 3-5 captures at a recursive
position. The admission gate in `resolve_recursive_unit_body` (core.rs) admits a
recursive-position `Closure` unconditionally but a `LexicalClosure` **only under
`if captures.is_empty()`**; a capture-bearing `LexicalClosure` falls through to
`Ok(None)` regardless of the partition.

⇒ A D1 on RT-BRANCH's authorized scope greens **zero** witnesses. **This node is
the real gate** for [[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] for this
population -- which is why the `blocks` edge for those dependants moves here from
RT-BRANCH (its point-2 frame-accuracy correction).

# THE INVARIANT -- INVIOLABLE, BINDS EVERY DELIVERABLE

RT-BRANCH holds a hard boundary this node inherits verbatim (Architect
evt_sjjmcap9293y):

> **Captures may be sourced ONLY from planner-owned state -- the retained-source
> capture structure, the planner capture plan, and the ABI operand slots -- with
> ZERO read of the carried word. The carried word contributes only the
> eliminated value; static code identity and capture authority stay
> compiler-owned.**

Any deliverable that recovers a capture VALUE from the carried word is out of
scope and unsound, however convenient -- recovering it would be "a `Closure`
value crosses," which is barred. Reading the capture STRUCTURE
(`LexicalClosure { captures, .. }`) from the retained-source occurrence is
compiler-owned and fine; reading a capture's VALUE from the word is not.

# WHAT THE FORK RULING GROUNDED (Architect evt_sjjmcap9293y)

**The precedent that IS the template -- `call_declared_context` (calls.rs:800).**
It supplies a generated continuation-context's captures WITHOUT reading any
carried word: `view.captures()` from the static transition plan, each resolved
via `resolve_context_capture_claim(coordinate, availability, defining_owner)` to
an immediate slot indexing `function_local.defining_abi_operands`. Its own
comment is the invariant verbatim: captures are "appended in the context's
DECLARED ORDER, and taken only from the immediate slots the planner assigned --
nothing is reconstructed from the raw worker, chosen by shape, or routed through
a runtime transport." This is the exact shape a compiler-owned capture supply
must take.

**The two "precedent" seam sites (core.rs:12741, source.rs:3947) --
NECESSARY but NOT SUFFICIENT (Sub-Q3, settled).** Both operate on a
`Lowered::Closure { captures, params, body }` and do `call_inputs.extend(captures)`
-- but those `captures` are ALREADY-LOWERED operands the compiler holds directly,
because the closure is a compile-time `Lowered::Closure` in the current lowering
context, NOT a carried word (source.rs states the boundary: "the ARGUMENTS cross
here; the CAPTURES do not... A capture arrives inside an already-lowered
`Lowered::Closure`"). They prove the seam **accepts** a compiler-owned appended
capture suffix; they do **not** prove a carried `LexicalClosure`'s captures are
compiler-owned. ⇒ Do NOT frame this as "generalize the precedent." Frame it as
**"establish the carried captures' provenance, then reuse the append."**

# DELIVERABLES

**`D0` -- the deciding provenance measurement (Sub-Q1). A classification, no
implementation.** For each of the 16 RT-BRANCH predicate-matching witnesses,
measure whether **every** capture's VALUE is available at the consumer's frame as
a planner-assigned ABI operand -- a `resolve_context_capture_claim`-style claim
into `defining_abi_operands`, template `call_declared_context` -- WITHOUT reading
the carried word. Classify each witness:

- **planner-recoverable** (ALL captures resolve to a planner-owned claim) =>
  mechanism-ownable (a D1 target);
- **word-only** (ANY capture reachable only through the carried word) => intended
  refusal, fails closed, out of scope under the current capture plan.

This is a NEW measurement: RT-BRANCH's D0 measured capture COUNT (3-5) and that
`captures.is_empty()` fails; it did NOT measure planner-recoverability. The two
are non-redundant. D0 is measurement-only and does not close the node (same
posture as RT-BRANCH D0).

**`D1` -- the capture-supply arm (Sub-Q2), conditioned on D0's
planner-recoverable subset.** Add a DISTINCT arm in the admission gate that
admits a capture-bearing `LexicalClosure` at a recursive position **only when
every capture resolves to a planner-owned claim**, reusing the
`call_declared_context` planner-owned append to present them as declared inputs;
otherwise it falls through to the EXISTING `captures.is_empty()`-failing refusal.
The gate condition moves from "captures empty" to "captures all
planner-recoverable" -- a strictly-justified widening that preserves the
fail-closed posture, never a blanket open. The `captures.is_empty()` fast path
stays (the trivial all-recoverable case: zero captures, nothing to claim). D1
needs RT-BRANCH's branch-local callable-authority seam in place (see Sequencing).

# ACCEPTANCE CRITERIA

**`AC-0` (D0) -- the classification is complete and disposition-split.** Every
one of the 16 witnesses is classified planner-recoverable or word-only by the
per-capture ABI-operand-recoverability predicate, with the word-only rows named
as intended refusals. The measurement reads only planner-owned state; a fixture
or audit shows no capture value is sourced from the carried word.

**`AC-1` (D1) -- the seam-property fixture, planner-recoverable subset only.** An
AUTHORED fixture whose recursive position is a capture-bearing `LexicalClosure`
all of whose captures are planner-recoverable greens through the planner-owned
append (no such fixture need exist in the witness set; author it). This is NOT a
"these 16 rows pass" criterion -- that is unachievable and would be a
measurement-vs-landing gate mismatch (Architect's binding framing note); it
applies to the planner-recoverable subset ONLY.

**`AC-2` (D1) -- the discriminating fail-closed control.** A capture-bearing
`LexicalClosure` with a capture that is NOT planner-recoverable still refuses,
fail-closed, unchanged through the existing refusal. This is the control that
proves the new arm did not widen into the word-only case.

**`AC-3` -- conformance (§7)** for the accept case (a planner-recoverable
capture-bearing recursive position greens) and the invariance case (a word-only
one still refuses), if D0 finds a non-empty planner-recoverable subset.

**`AC-4` -- no-regression** in CI (`COORDINATION §12`). Targeted local validation
only.

# `D0` PROVENANCE CLASSIFICATION at base `6eb90cfa80dc792295bad0545939543fba8619d6`

**Result: zero of the sixteen witnesses is planner-recoverable. All sixteen are
word-only.** The bounded-non-closure outcome below is therefore the one this D0
returns — reported as the frame defines it, a first-class result, not a failure,
and not forced in either direction. **One door this measurement does NOT close
is named at the end; it is a design question for the Architect, not something
D0 may decide.**

## What a capture actually is — the structural finding the rest rests on

`RuntimeExpr::LexicalClosure` declares `captures: Vec<RuntimeExpr>`
(`ir.rs:618-622`). **A capture is a source EXPRESSION evaluated in the enclosing
lexical environment, not an already-lowered value.** Measured across the whole
population: **all 108 captures in all 25 recursive-position closures are
`Var(i)`, forming a contiguous `Var(0)..Var(n-1)` prefix** of the producer's
environment. None is a literal `Value`, which would have been trivially
compiler-owned and needed no frame at all.

This is precisely why the two "precedent" seam sites do not transfer, and it
confirms the fork ruling's Sub-Q3 from the other side: at `core.rs:12741` and
`source.rs:3947` the captures are `Lowered::*` operands the compiler already
holds, whereas here they are unevaluated `Var` references into an environment
the consumer does not have.

## The instrument, and the positive control that is internal to the population

The measurement reads the planner's own answer rather than inferring one. For
each recursive-position capture-bearing `LexicalClosure`, it resolves the
closure's body origin, finds every generated continuation context whose
`worker_body_origin()` is that body, and reads that context's `captures()` view
— each capture's `coordinate` and its `availability.context_capture`, which is
exactly the field `resolve_context_capture_claim` (`core.rs:7064`) consumes.

That function is the authority on recoverability, and it refuses on **both** of
its non-`EntryFrame` paths:

- `views.context_capture == None` — "nothing says where this frame holds
  {coordinate}";
- `ContinuationEnvironmentClaim::CurrentLexical` — "presented to the entry-frame
  capture consumer, which holds an ABI operand run and no semantic environment;
  a nearest-alias lexical index is not a frame slot".

Only `ContinuationEnvironmentClaim::EntryFrame { frame, declared_slot }`
resolves, and then only after `verify_entry_frame` confirms membership.

**The instrument is not uniformly blind, and the proof is inside the measured
population.** Across all 30 planner claims observed:

| claim coordinate | `context_capture` | count | resolves? |
| --- | --- | --- | --- |
| `EntryAbi { source: Parameter, .. }` | `Some(EntryFrame { .. })` | 5 | **yes** |
| `ProducerLocal { binding, locator }` | `None` | 25 | no |

Zero exceptions in either direction. So the measurement **does** return
"recoverable" for the recoverable kind — it simply never returns it for enough
captures. That correspondence is the real boundary this node was asked to find:
**a capture whose value originates as an entry-ABI position of the emitting
function is planner-recoverable; a capture whose value originates as a
producer-local binding is not**, and a `ProducerLocal` coordinate names a
`ProducerLocalLocator { environment_origin, environment_index }` — an index into
a producer-side lexical environment, which is the very thing the consumer's
frame does not hold.

## Per-witness classification

Every row fails the "ALL captures planner-recoverable" predicate on **two
independent grounds**, and the first does not depend on the claim-kind analysis
above at all:

1. **Cardinality.** The owning context's capture plan has **at most 2** entries
   against **3 to 5** source captures. Even if every planned capture carried an
   `EntryFrame` claim, the plan cannot cover the closure's capture set.
2. **Kind.** Where claims exist, 25 of 30 are `ProducerLocal` with no
   context-capture claim, which `resolve_context_capture_claim` refuses.

| Witness | Source captures | Closures at the position | Contexts owning the body | Planner claims | Claims resolving (`EntryFrame`) | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| `px7f_resource_native.rs :: linked_public_right_denial_preserves_exact_masks` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `px7f_resource_native.rs :: linked_public_second_release_is_closed_and_the_handle_closes_once` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `px7l_checked_host_recursive_bind.rs :: delayed_capturing_generic_bind_agrees_across_real_executors` | 3 (`Var(0..2)`) | 3 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `px7l_checked_host_recursive_bind.rs :: runtime_selected_non_unit_response_is_consumed_across_real_executors` | 3 (`Var(0..2)`) | 3 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `px7m_hostresult_computational_match.rs :: dynamic_ok_payload_selects_a_multistep_tree_across_real_executors` | 4 (`Var(0..3)`) | 4 | 1 | 2 | 1 | **word-only, intended refusal** — 1 of 4 captures resolvable |
| `px7m_hostresult_computational_match.rs :: dynamic_err_payload_selects_a_multistep_tree_across_real_executors` | 5 (`Var(0..4)`) | 3 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `px8ta_oriented_subcontinuation.rs :: public_one_level_bracket_finishes_and_releases` | 5 (`Var(0..4)`) | 1 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `px8ta_oriented_subcontinuation.rs :: px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges` | 4 (`Var(0..3)`) | 1 | 1 | 2 | 1 | **word-only, intended refusal** — 1 of 4 captures resolvable |
| `px8x_single_schema_observation.rs :: linked_route_exposes_real_ordered_bindings_and_filters_reserved_input` | 5 (`Var(0..4)`) | 1 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `rt_escape_second_resource_native.rs :: escape_resource_plus_plain_matches_interpreter` | 5 (`Var(0..4)`) | 1 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `rt_escape_second_resource_native.rs :: escaped_buffer_used_by_fanning_host_op_matches_interpreter` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `rt_parity_native.rs :: buffer_allocate_malformed_capacity_narrows_to_invalid_bounds` | 5 (`Var(0..4)`) | 1 | **0** | 0 | 0 | **word-only, intended refusal** — no context owns the body at all |
| `rt_parity_native.rs :: fs_read_at_malformed_offset_narrows_to_invalid_offset` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `rt_parity_native.rs :: fs_read_at_malformed_window_narrows_to_invalid_bounds` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `rt_parity_native.rs :: fs_read_at_malformed_offset_without_read_right_narrows_to_invalid_offset` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |
| `rt_parity_native.rs :: fs_write_at_malformed_offset_without_write_right_narrows_to_invalid_offset` | 5 (`Var(0..4)`) | 1 | 1 | 2 | 0 | **word-only, intended refusal** — no resolving claim |

**Totals: 25 recursive-position closures, 108 captures, 30 planner claims, 5
resolving claims, 0 planner-recoverable witnesses.**

Six witnesses have **no generated continuation context owning the closure body
at all**, so for them there is not even a capture plan to interrogate — the
strongest form of the same answer.

## `AC-0` — the invariant audit on the measurement itself

The measurement was taken with a temporary, env-gated (`KEN_D0_PROBE`),
**uncommitted** probe. Its reads were audited mechanically over the added lines,
and they are confined to compiler-owned and planner-owned state:

- retained-source structure — `argument.expr`, the `captures` list, and
  `case_body_occurrence` / `child_occurrence` origin threading. Reading the
  capture STRUCTURE from the retained-source occurrence is explicitly the
  compiler-owned side of the invariant;
- planner state — `static_transition_plan.continuation_contexts()`,
  `ctx.worker_body_origin()`, `ctx.captures()`, `cap.coordinate`,
  `cap.availability`.

**Zero reads of the carried word.** The audit greps the probe's added lines for
`Carried`, `carrier_field`, `emit_carrier`, and `word`, and matches nothing —
no capture VALUE is sourced from the carried word at any point, which is the
one inviolable line. The three touched files were then restored and verified
**byte-identical by blob hash** to their base objects (`core.rs a616117a`,
`source.rs 7cf81a35`, `calls.rs 5e1c460e`) with a clean `git status`. D0 remains
measurement-only.

Reproduction:

```sh
scripts/ken-cargo build -p ken-runtime --lib          # materialize libken_runtime.a first
KEN_D0_PROBE=1 scripts/ken-cargo test -p ken-cli --test <file> \
    -- --ignored --nocapture --test-threads=1 <test-name>
```

The probe dumps, per recursive-position capture-bearing `LexicalClosure`: the
capture expression kinds, the owning contexts, and each planner claim's
coordinate and `availability.context_capture`. One caveat worth carrying: for
the twelve witnesses whose whole-source resolution short-circuits at the
missing-position veto, **production never inspects the capture-bearing arm**, so
the measurement needs a read-only walk over every arm to reach it. A probe
placed only on the production path returns nothing and reads as "no captures
found" rather than "not reached".

## THE ONE DOOR THIS MEASUREMENT DOES NOT CLOSE

**Measured:** under the planner's *current* capture plan, no witness's captures
are recoverable as planner-assigned ABI operands, so supplying them at the
consumer's frame would require reading the carried word, which the invariant
bars. That is the D0 answer and it is why the bounded-non-closure outcome fires.

**NOT measured, and it is a genuine question rather than a hedge:** whether the
planner *could* widen a generated context's entry frame so that today's
`ProducerLocal` bindings are passed as additional entry-ABI positions. The
`EntryAbi`/`ProducerLocal` correspondence above shows the mechanism already does
exactly this for parameter-origin captures, and doing so would source the value
from the producer side **at context entry** rather than from the carried word —
so it is not obviously an invariant breach.

**This is not D0's to decide, and D0 does not propose it.** It is recorded
because the difference between "inexpressible under the invariant" and
"inexpressible under the current capture plan" changes what
[[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] should do next, and only the
Architect rules that. If the answer is that the plan may not widen, this node
closes as a bounded non-closure exactly as framed and those dependants need a
different route; if it may, the widening is a new node, not this one's D1.

# THE BOUNDED-NON-CLOSURE OUTCOME (a first-class result)

D0 found **zero** planner-recoverable witnesses (all 16 word-only). Per the
Architect's open-door ruling (evt_6f4708amnwr4p), capture supply for these
witnesses is **inexpressible under the CURRENT capture plan** -- NOT "under the
invariant." The closing deliverable (extending the entry-source enumeration so a
producer-local value becomes a real entry-ABI member) was never attempted, so
this is a **non-total** bounded non-closure, not an uncloseable one. The named
forward route is the successor [[RT-CONTSRC-ENTRY-FRAME-WIDEN]], which now carries
the `blocks` edge for [[NATIVE-HANDLE-CARRIER]] / [[PX8-F-CAP-41]]; those
dependants are gated on that widening's feasibility+soundness D0, not on a wholly
different route. This is a first-class result, and D0 is the measurement that
produced it.

# BANNED SCOPE

- **Reading any capture VALUE from the carried word.** The one inviolable line;
  it is unsound (a `Closure` value would cross).
- **Blanket-relaxing `if captures.is_empty()`** to "captures allowed." That
  admits word-only witnesses = an invariant breach. Only the strictly-justified
  "all captures planner-recoverable" widening is authorized (Sub-Q2).
- **No `PersistentClosure` / `FrozenClosure` / new carrier tag / implicit
  `StaticCallableRef` conversion / metadata recovered from the carried word** --
  all still barred, inherited from RT-BRANCH.

# SEQUENCING

**`depends_on: []` -- D0 is grounded on the current tree.** The Architect
grounded the fork on origin/main f5c006ab: the seam files are unchanged since
RT-BRANCH's D0 base, so the provenance measurement runs on today's tree with no
predecessor landing required. D0 is therefore startable now.

**D1 needs RT-BRANCH's seam.** The capture-supply arm installs captures inside
the already-selected constructor case, which is
[[RT-BRANCH-LOCAL-DECLARED-CALLABLE]]'s branch-local callable-authority cut. D1
must not land before that seam exists; the runtime ring sequences D1 after
RT-BRANCH's D1. D0, being a measurement, does not.

**Capability tier: T1.** The D0 is a soundness-adjacent classification --
mis-classifying a word-only capture as planner-recoverable would authorize an
unsound arm -- so it demands the same reasoning tier as RT-BRANCH's D0, not a
mechanical census. The runtime seat is already T1.

**Review: Architect** reviews the D0 and any D1 (evt_sjjmcap9293y). The author is
not the reviewer.
