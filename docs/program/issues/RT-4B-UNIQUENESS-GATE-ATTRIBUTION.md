---
id: RT-4B-UNIQUENESS-GATE-ATTRIBUTION
title: "Gate 4b's observation route is exhausted -- four admitted discoveries enter enumeration and zero candidates leave, and none of the fourteen elimination routes is attributed; the Architect named `fusion_unique_static_body_triple` as the cheapest and most informative place to recover attribution, but instrumenting one gate is only decisive if the eliminations actually happen there, and nothing measured says they do"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-4B-UNIQUENESS-GATE-REACH]
blocks: []
github: null
origin: Architect evt_7011z8x4x2j3d naming the one-function attribution start and explicitly NOT authorizing it, on the stated condition that the size increment return a non-empty population. It did -- `(4, 2, 0, 2, 1)` at RT-4B-ENUMERATION-INPUT-SIZE. Steward taking the scope question to the Architect 2026-08-13. DRAFT until that scope call lands.
---

> **TRIGGER NOT MET — `reach > 0` was never measured on the population in
> question (Architect `evt_6hfw027f43cgg`). The `(4, 2, 0, 2, 1)` reading came
> from D2j comparators PERTURBED so fusion cannot form, not from `C2`. See the
> banner on [[RT-4B-UNIQUENESS-GATE-REACH]], which is also pulled back to
> `draft`. This node stays `draft` and is further from lawful than before.**
>
> **SPLIT 2026-08-13 (Architect `evt_5gck3qg72xe37`). THIS NODE IS NOW THE
> CONDITIONAL HALF ONLY, AND IT IS NOT LAWFUL YET.**
>
> The reach half — counting whether exit 12 is reached at all — moved to
> **`RT-4B-UNIQUENESS-GATE-REACH`**, which is `ready` now. It is a call-site
> counter, changes no signature and no control flow, and is therefore **pure
> recording inside the 4b observation gate already authorized**. It needed no
> exception.
>
> **What remains here is attribution: which arm of
> `fusion_unique_static_body_triple` fired.** That requires widening the
> function's return type, which is a builder change outside the observation
> gate, and it is the one-function exception the Architect named.
>
> ⇒ **This node becomes lawful the moment reach is measured POSITIVE, and not
> otherwise. If reach is zero, CLOSE this node rather than kicking it** — a
> widened return type observing a branch no candidate enters is the "working
> instrument nothing reaches" artifact, built deliberately.
>
> **The reasoning is the Architect's own, applied to himself:** *"I placed five
> rows inside a lifecycle without counting whether they produced anything for
> it... Count the population before you build the thing that classifies it."*
> That is the `D2k-1b` failure from earlier the same day, and this split is what
> keeps it from repeating one level up.

## Where the arc actually stands

`RT-4B-ENUMERATION-INPUT-SIZE` measured `(4, 2, 0, 2, 1)`: **four admitted
discoveries enter `enumerate_live_fusion_candidates`, and `keys = []` comes
out.** The Architect established that the interning loop has no decline path,
so `keys.len() == candidates.len()` identically — every one of the four was
eliminated inside enumeration.

**That result rules out exactly one explanation — "there was nothing to fuse" —
and licenses nothing about the planner.** All fourteen elimination routes
remain equally consistent with it, and at least one is documented as lawful.

⇒ **The observation route is exhausted.** Further progress on 4b needs either
attribution or the emitter, and the emitter sits behind two held gates.

## The named start, and why it is the right one

Architect, `evt_7011z8x4x2j3d`: `fusion_unique_static_body_triple`
(`planning/static_transition.rs:10099`) is both the cheapest and the most
informative of the fourteen.

- **Cheapest.** The refusal is one line — `if matching.len() != 1 { return
  Ok(None) }`. Distinguishing absence from multiplicity is `len() == 0` versus
  `len() > 1`, already computed.
- **Most informative.** It is the only elimination with **documented
  lawful-refusal semantics** — *"Absence and multiplicity are both refusals:
  'the only edge' would be an existential and choosing among several would be a
  guess."* Every other exit is a structural mismatch saying the shape was
  wrong. **This one says the shape was right and the edge population was not.**
- **Already isolable, and this is a fixed input worth the frame's attention.**
  A `#[cfg(test)] DUPLICATE_STATIC_BODY_TRIPLE` control at `:10117-10123`
  already arms the multiplicity arm, and its own comment records why the
  isolation holds: *"the transport gate, the bindings, the exact consuming
  suffix and the input projection have all already been satisfied by the time
  this runs, so a candidate that disappears here disappeared at the uniqueness
  gate specifically."*

## THE SIZING RISK — NOW DISCHARGED BY THE SPLIT, kept for the reasoning

**Instrumenting one gate is only decisive if the four candidates die at that
gate. Nothing measured says they do.**

`fusion_unique_static_body_triple` is the **twelfth** of thirteen exits. A
candidate reaches it only after surviving eleven earlier ones — the match-shape
check, the constructor-identity comparison, the argument and binding lookups,
the consuming-call shape, the binding equality, and the transport lookup.

⇒ **If the four die earlier, this increment returns "none reached me" and buys
one bit: the eliminations are upstream of the twelfth exit.** That is real
information and it is much less than the node's framing implies.

**This is the same defect the arc has now paid for five times — a claim written
wider than its instrument — and it applies to this proposal, not just to other
people's reports.** The Architect's "highest-information gate" reasoning is
about *what the gate means when it fires*, which is correct, and it is **not**
evidence that this gate is where the population dies.

**The honest cheap alternative to weigh against it:** a single reach-count at
the uniqueness gate answers "do candidates get this far at all?" for strictly
less than full absence/multiplicity attribution — and if the answer is zero, the
richer return type would have told us nothing. **Whether that ordering is worth
one extra increment or is over-caution is exactly the scope call.**

## Not this node

- **Attributing the other twelve exits.** Measured and ruled out at
  `evt_ky5f547e6hjz`: none is distinguished, so it is an all-thirteen builder
  change.
- **Any enumeration, classifier, checker, marker, fusion-candidate,
  representation, ledger or closure-boundary repair.** Gates 5 and 6 stay held;
  production stays unarmed.
- **Changing which candidates survive.** This is attribution, not admission. If
  the eliminated population changes, that is a hard stop.
