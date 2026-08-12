---
name: a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about
description: >-
  A ruling that fixes a rejection by widening a shared field silently widens
  every other consumer of that field, and the ruling names only the consumer it
  was about — so nobody re-derives what the others now accept.
metadata:
  type: feedback
scope: roles/adversary
---

# A ruling that widens a shared map names only the consumer it was about

An implementer hit a spurious rejection: `construct_static_worker_binding`
validated through `function_local.unit_calls`, which on the ordinary path holds
only **this caller's** static-body call edges, while the value it needed was in
`worker_calls`, the **whole emittable-unit** projection. The Architect ruled the
correct local fix — seat the same declared objects in both maps. The implementer
applied it exactly.

**The ruling was right and the application was faithful. The widening still
reached a consumer neither of them was looking at.** `unit_calls` has a second
reader, `call_declared_unit`, whose rejection text claims a *graph-derived*
target and which is reachable from the body the same function then lowers. In
that function the field no longer means "call edges into this caller", so that
check silently relaxed from "the call graph sanctions this callee" to "this
callee exists at all".

**Why nobody catches it in the normal course:** a ruling is scoped to the
refusal that provoked it, and passing the ruling *is* the definition of done.
The implementer re-derives the mechanism the ruling names, never the mechanism
it did not.

**How to hunt it.** When a ruling resolves a rejection by **widening a
population rather than narrowing a check**, do not read the fixed consumer —
`grep` every other reader of the widened field and ask what each one's
rejection *claims*. The tell is a field with **two readers and no doc comment**:
here the contract both depended on was stated only inside one of their error
strings, and the field sat undocumented between two documented neighbours.
An error string is not a contract, and it binds nobody.

**Report the axis, not an alarm.** A widened check has no repro until some
program reaches it; say the claim is about the check's *strength*, not about an
observed wrong output, or the finding overstates and gets discounted whole.
Related: [[pre-emptive-yield-is-at-the-gate-layer-not-the-code-layer]],
[[a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution]],
[[preventive-findings-are-unfalsifiable-so-keep-them-cheap]].
