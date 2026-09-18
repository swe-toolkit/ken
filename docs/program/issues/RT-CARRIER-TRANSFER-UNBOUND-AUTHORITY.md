---
id: RT-CARRIER-TRANSFER-UNBOUND-AUTHORITY
title: "Can a PRODUCTION path reach a carrier transfer while `defining_emission_owner` is None -- that is, with no AmbientBodyAuthority bound? RT-CARRIER-PRODUCER-OCCURRENCE's D2 ruled the failing row's state RIG-MADE (a top-level authority state driving an in-body operation, a pairing no production path was shown to produce), and that ruling deliberately does NOT establish the production negative. The field being None is DESIGNED and happens on every compile at top level, so the question is the PAIRING, never the value. Filed BEFORE the D3 repair lands, because D3 turns the row green and a green row is a suppression instruction with no author."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, on the Architect's D2 ruling for RT-CARRIER-PRODUCER-OCCURRENCE (evt_6yg5ara1b9wwb), which splits this half off explicitly and requires it tracked before D3 lands: 'D2 ruling rig does not establish that no production path reaches a carrier transfer unbound. It establishes that this row's state is rig-made and that D3 proceeds. The other half is a separate question... It must be a tracked node before D3 lands, not after.' runtime-implementer is holding D3 at merge_ready on the existence of this node (evt_7w85rfbzr6eew). Steward-filed per COORDINATION section 2."
---

> # WHY THIS IS FILED BEFORE THE REPAIR, AND NOT AFTER
>
> **`D3` turns the failing row green. A green row is a suppression instruction
> with no author — nobody re-runs a passing test.** The row is currently the
> only live signal anyone is looking at for this question, and landing the
> repair removes it. The Architect's words (`evt_6yg5ara1b9wwb`): *"Filing it
> after is filing it never."*
>
> **This node existing is the gate. It does not need to be released to clear
> it.** `runtime-implementer` is building `D3`/`D4`/`D5` and holding at
> `merge_ready` until this file exists; that hold is correct and it is
> discharged by this file, not by this node being worked.

# What `D2` established, and what it deliberately did not

**Established: the failing row's state is rig-made.** `bare_carrier_test_lowering`
(`constructors.rs:1926`, `#[cfg(test)]`) builds a `Lowering` with
`defining_emission_owner: None` and drives a carrier transfer **without ever
binding**.

> **The rig does not manufacture an impossible VALUE. It manufactures an
> impossible PAIRING: top-level authority state with an in-body operation.**

**NOT established: that no production path does the same.** `D2` is a ruling
about one row's provenance, not a whole-program negative, and the Architect
named the gap themselves rather than letting it ride.

# THE QUESTION IS A PAIRING, NOT A VALUE — this is the whole node

    NOT  "can `defining_emission_owner` be None?"
         Yes. ALWAYS. By design, on EVERY compile, at top level.
    BUT  "can a CARRIER TRANSFER run while it is None?"

**These are different questions and the first one's answer is a design
statement, not evidence.** `core.rs:1758-1760`, verbatim:

> *"On exit both fields are RESTORED to what the enclosing scope held, **which
> is `None` at the top level.** Clearing unconditionally would be wrong for a
> nested pass; restoring is correct for both and does not have to know which it
> is in."*

⇒ **Any instrument whose counterexample condition is "a path reaches here with
no bind above it" is satisfied by correct code on every compile.** A previous
instrument on this question was specified that way; it produced 13 apparent
counterexamples, none of which were counterexamples. **Do not re-derive that
instrument.**

# Fixed inputs, verified by the Steward at `e75f1fe27`

**Every production write to `defining_emission_owner` in the crate — three,
plus the declaration:**

    mod.rs   3146   the field DECLARATION
    core.rs  2671   `defining_emission_owner: None,`  the ONE production
                    `Lowering` construction
    core.rs  1778   `compiler.defining_emission_owner = Some(owner);`   bind
    core.rs  1814   `compiler.defining_emission_owner = self.enclosing_owner;`
                    release

⇒ **`AmbientBodyAuthority` is the SOLE production mutator of the field.**

**Specificity caveat, carried from the Architect and re-confirmed here:** an
assignment key of `\.defining_emission_owner *=` **also matches `== Some(owner)`**
at `core.rs:5915`, `:6466`, `:15059`. Those are comparisons, not writes. Any
re-derivation must exclude them.

**The `#[cfg(test)]` boundary in `aggregates.rs`:** attribute at `:4457`,
`pub(in crate::cranelift_backend::lowering) mod tests {` at `:4458`. Confirmed
by direct read. **Each file under `lowering/` has its own boundary — find it per
file, do not carry this one.**

# The measurement — ENUMERATE CALLEES, DO NOT CLIMB CALLERS

The Architect's key, and the direction reversal is the point:

    KEY: `transfer_into_carrier\s*\(` under `lowering/`, minus the definition;
         split production vs `#[cfg(test)]` by each file's own boundary;
         then per site ask ONLY: is this line inside one of the bind spans?

**The absorbing set is the `AmbientBodyAuthority` bind spans** — the Architect
measured **nine** and built that half themselves. **Re-derive the count; do not
inherit it.** It is cited here as their measurement, not as a fixed input.

> **WHY THE DIRECTION REVERSES, AND IT IS STRUCTURAL RATHER THAN BAD LUCK.** A
> call site names its callee **literally on its own line**. Finding *callers*
> requires matching a name that may be path-qualified, re-exported, or
> method-dispatched — the prior instrument's caller pattern missed
> `super::units::define_static_continuation_fusion_bodies(` because a pattern
> tight enough to exclude definitions excluded that spelling too.
> **Caller resolution is hard here; callee resolution is not.** Enumerating the
> transfer sites and applying a span test to each is a lexical question end to
> end.

**No count is published in this node.** The Architect ran the grep and
deliberately did not split it: *"a number I have not run to closure is the thing
I promised to stop handing over."* Same discipline applies to whoever runs it.

# Outcomes

    (i)  every production transfer site sits inside a bind span
         => the refusal is a correct fail-closed guard over a pairing
            production never produces. That is a REAL RESULT and closes this
            node green. Record it as a positive property, not as "nothing found".
    (ii) at least one production site is NOT inside a bind span
         => the refusal is protecting a real hole, the row was evidence rather
            than debt, and this becomes a soundness question. STOP and escalate;
            do not size a repair here.

**The RULING between them is the Architect's**, per the RT-CARRIER frame's own
text (*"it is a mechanism question and returns to the Architect"*). The
measurement is runtime's.

# What must not happen

- **Do not relax the carrier's refusal.** `D2` STRENGTHENS this ban rather than
  weakening it: the refusal is the **correct** behaviour for the top-level
  state, and production reaches that state on every compile. Relaxing it would
  grant unearned authority in exactly the case the design handles deliberately.
- **Do not treat outcome (i) as "nothing to report".** A verified absence over
  an enumerated population is the deliverable. An unverified one is silence.
- **Do not re-run the withdrawn caller-traversal instrument** or re-report its
  13 roots. It chased a negative that is false as stated, and it had a second
  independent fault in its caller pattern. **Neither fault was visible from its
  output** — a broken traversal chasing a false negative produces the same shape
  as a working one.

# Related

- [[RT-CARRIER-PRODUCER-OCCURRENCE]] — the parent. `D2` = RIG
  (`evt_6yg5ara1b9wwb`); `D3` proceeds with `ac_c7_lowered_ctor`'s shape. **This
  node is the half `D2` did not settle**, and its existence gates that node's
  handoff.
- [[RT-CONTEXT-CAPTURE-CLAIM-ABSENCE]] — same crate, unrelated mechanism (a
  missing availability claim, not a missing emission owner). **Do not fold.**
