---
id: RT-BRANCH-LOCAL-DECLARED-CALLABLE
title: "recursive_position_unit_body returns one Option<StaticOriginId> for the whole source, so whole-source agreement is too coarse for a Match whose arms differ -- the cut is constructor-and-recursive-position-specific callable authority installed inside the already-selected constructor case, which eliminates the closure crossing rather than opening a durable closure lane"
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-RECURSIVE-POSITION-ARM-ARITY]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Architect ruling evt_7aeb7hqrykgpz, Decision dec_7aajmm0eac45c, resolved 2026-08-18. Cut by the Steward on that ruling's explicit instruction to frame the branch-local design capability separately from the rejected D1 AC-3 recut. Surfaced by RT-RECURSIVE-POSITION-ARM-ARITY D1, whose repair moved the governed rows onto the BoundaryCarrier refusal. Steward-filed per COORDINATION section 2."
---

> # THE BINARY I ROUTED WAS FALSE. Read this before reasoning from the guard.
>
> I asked the Architect whether a function-valued recursive field was out of
> scope by design, because `reject_carried_residual_arguments` fires on CAP-41
> and its doc says the durable closure lane is withheld. **The ruling is that
> both halves are true and they do not conflict:**
>
> **The durable closure lane REMAINS EXCLUDED. A function-valued recursive
> field is NOT out of scope.** There is already a separate lawful route, and
> the gap is elsewhere.

# THE BOUNDARIES THAT STAY. None of these is what this node changes.

- A raw `LoweringOperand::Carried` is a **transferred value, never callable
  authority**.
- **`reject_carried_residual_arguments` remains the fail-closed guard** for
  non-empty invocation through that raw-value arm, before control installation.
- **Not authorized, and none of it is a fallback if this cut gets hard:** no
  `PersistentClosure` lane, no new carrier tag or class admission, no
  `FrozenClosure`, no implicit `StaticCallableRef` conversion, and no metadata
  recovered from the carried word.

# THE LAWFUL ROUTE THAT ALREADY EXISTS

In `lower_recursor_residual_call`, the `recursive_unit_body` /
`FunctionizedUnits` arm **runs before `reject_carried_residual_arguments`**. It
lowers explicit source arguments and calls
`call_declared_recursive_position_unit`, and `call_declared_context` can append
planner-authorized capture operands.

⇒ **Static code identity and capture authority stay compiler-owned; the carried
word contributes only the eliminated value. No `Closure` value crosses.**

# THE ACTUAL GAP

`recursive_position_unit_body` returns **one `Option<StaticOriginId>` for the
whole source**. [[RT-RECURSIVE-POSITION-ARM-ARITY]] `D1` was right to refuse to
select a surviving unit when an arm lacks the recursive position — **but
`Ret`/`Vis` proves whole-source agreement is too coarse.** `Ret` has no
recursive position; `Vis.k` does.

**The cut is constructor-and-recursive-position-specific callable authority,
installed only inside the already-selected constructor case.** It must name the
declared body plus its checked explicit-input/capture plan, from retained
source and planner authority.

- **`Ret` installs none.**
- **`Vis` may install one only when the body and captures are lawfully
  expressible as declared call inputs.** If the captures cannot be supplied
  through planner-owned operands or an already-admitted structural-value route,
  **that case still refuses. The guard is not weakened.**

# `D0` — CLASSIFY THE QUARANTINED ROWS. Do NOT bulk-assign them.

**The Architect ruled this explicitly, and it corrects my own framing.** I
grouped 16 rows across 7 test files by their shared error text and called them
this mechanism's population. **A shared error text proves only a shared
terminal guard.** It does not establish that the rows share a cause, and
assigning them on that basis would import rows this cut cannot close.

Per row, report:

1. the **selected constructor** and its recursive position,
2. **declared-body availability**,
3. **capture/input representation** — can the captures be supplied through
   planner-owned operands or an already-admitted structural-value route,
4. the **actual boundary kind** it hits.

Only then assign each row to this mechanism **or** record it as an intended
refusal. Files carrying the text: `px7f_resource_native`,
`px7l_checked_host_recursive_bind`, `px7m_hostresult_computational_match`,
`px8ta_oriented_subcontinuation`, `px8x_single_schema_observation`,
`rt_parity_native`, `rt_escape_second_resource_native`.

**A row that lands in "intended refusal" is a result, not a failure of `D0`.**
The quarantine reasons credit [[RT-SITEOP-CARRIED-WITNESS]] `D2`, which is
**merged**, so nothing owns their disposition today — that is what `D0` fixes.

# ACCEPTANCE

- **`AC-1`** — every row carrying the shared refusal text is classified on all
  four axes, or explicitly excluded with the reason. **Control:** the row count
  reconciles against a fresh census of the text, so a row added since framing
  cannot be silently omitted.
- **`AC-2`** — each row is assigned to this mechanism **or** recorded as an
  intended refusal, with the axis value that decided it. A row assigned on
  shared error text alone fails this criterion.

# BANNED SCOPE

- **No implementation before `D0`.** The population decides the shape.
- **Nothing from the not-authorized list above**, and it is not a fallback.
- **No weakening of `reject_carried_residual_arguments`.** It stays the
  backstop wherever branch-local authority is absent.
- **No `RT-RECURSIVE-POSITION-ARM-ARITY` work.** That node's `D1` recut is in
  flight and owns its own `AC-3` control; this node does not touch it.

# CONTENTION

Same file as the in-flight `D1` recut —
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs`, the
[[RT-BACKEND-MODULE-SPLIT]] decomposition target. **`D0` is classification only
and touches no production line.** A later implementation deliverable **will**
contend; sequence it after the `D1` recut lands, and re-derive every symbol by
name rather than by offset.
