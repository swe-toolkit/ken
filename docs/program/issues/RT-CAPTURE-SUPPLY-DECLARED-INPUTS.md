---
id: RT-CAPTURE-SUPPLY-DECLARED-INPUTS
title: "the branch-local callable-authority cut is necessary but not sufficient -- every witness's residual blocker is capture supply; a capture-bearing LexicalClosure at a recursive position can present its captures as planner-owned declared inputs ONLY if every capture's value is recoverable as a planner-assigned ABI operand with zero read of the carried word -- D0 measures that per witness before any implementation"
status: ready
owner: runtime
size: L
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
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
  refusal, fails closed, out of scope forever under the invariant.

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

# THE BOUNDED-NON-CLOSURE OUTCOME (a first-class result)

If D0 finds **zero** planner-recoverable witnesses, capture supply for the
carried word is **inexpressible under the invariant**: the node closes as a
bounded non-closure, and [[NATIVE-HANDLE-CARRIER]] / [[PX8-F-CAP-41]] need a
DIFFERENT route for this population. Per the Architect, this is a first-class
result, not a failure -- and D0 is exactly the measurement that decides it, which
is why it runs before any D1 is attempted.

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
