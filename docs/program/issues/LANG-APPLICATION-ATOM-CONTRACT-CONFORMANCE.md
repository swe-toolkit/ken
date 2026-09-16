---
id: LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE
title: "Bring the parser into conformance with spec/30-surface/32-grammar.md §3's application-atom contract pin -- the five ungrouped leading forms reject at their leading token, and arrow/projection nest the application in their left expr ((keep Nat) -> Nat, (keep box).value) -- and migrate the catalog source written against the current non-conforming parse. Two measured divergences, in opposite directions: `if` is ACCEPTED as a bare application argument where the pin requires rejection, and `keep box.value` parses A(keep, Proj(box, value)) where the pin requires Proj(A(keep, box), value)."
status: draft
owner: language
size: L
gate: none
depends_on: [SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 at the Architect's naming, on their D0 REVERSAL (evt_5sf71fnjxpzmb), which vacates the earlier WITHDRAW ruling (evt_64avxs9ashqqk). The divergences were measured by spec-author under the SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION hard stop (evt_7p78v6qb429rz) and read against the spec by the Architect. Steward verified the two quoted spec passages verbatim at origin/main e11341c7b9d11cd74879d27d555d2a5729837847: 32-grammar.md :357-363 (ungrouped operator_name is not a general application_atom, 'applies equally to generic and reserved operator names') and :368-378 (the five leading forms reject; arrow and projection nest, as `(keep Nat) -> Nat` and `(keep box).value`; 'part of §3's contract pin; an implementation must not restore a second unrestricted application production'). The catalog-source extent is UNMEASURED -- see Sizing. Not yet framed; the Steward frames and releases it when the language lane reaches it."
---

> # DRAFT -- cut so the seed rows have a real node to point at.
>
> This node exists because [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]]
> re-gates two row groups of `seed-reserved-infix-names.md` onto a named
> successor, and a `RED-UNTIL-<node>` tag naming a node that does not exist is
> not a gate. It is **not released**: it has no frame, and the catalog
> migration is unmeasured. Do not start it.

# Objective

Make the parser conform to the application-atom contract pin in
`spec/30-surface/32-grammar.md §3`, and migrate the catalog source that was
written against the current non-conforming parse.

The pin is already on `main` and is not in question. This node does not decide
whether the contraction is wanted -- the spec decided that. It closes the gap
between the spec and the implementation.

# The divergence (measured)

Measured by spec-author against the base parser; read against the spec by the
Architect. Two divergences, in opposite directions, out of four rows:

| row | `32 §3` requires | parser today | verdict |
|---|---|---|---|
| lambda / `let` / `match` | reject ungrouped | rejects | CONFORMS |
| `if` | reject ungrouped | ACCEPTS `A(keep, If(...))` | **DIVERGES** |
| arrow | `(keep Nat) -> Nat` | same | CONFORMS |
| projection | `(keep box).value` | `A(keep, Proj(box, value))` | **DIVERGES** |

The bare-operator-value group is pinned by the same section at `:357-363`, for
generic and reserved names alike -- an ungrouped `operator_name` is admitted
only as the head of `operator_prefix`, so `(OP)` grouping is required to use it
as an ordinary atom.

# Why the catalog migration is in scope by construction

The language ring's A0 checkpoint `2c63d409` implemented the seed -- which is
the spec -- and the focused catalog control
`modules::namespace_effect_tests::priority_queue_actual_export_table_is_exactly_the_six_name_api`
failed candidate-only with `TypeMismatch ... projection base's type is not a
named-field owner`. Restoring the legacy parse returned it to 1/1.

That failure is not evidence against the contraction. **Existing catalog source
relies on ungrouped projected arguments, i.e. it is written against the
non-conforming parse.** Conforming the parser therefore necessarily moves the
catalog source with it, and the two cannot be separated -- which is precisely
what made the contraction out of scope for
[[LANG-RESERVED-INFIX-NAMES]] (A0), whose decomposition is bounded to unchanged
syntax.

# Deliverables (indicative -- this node is not framed)

- Parser conformance for the `if` row: an ungrouped `if` after an application
  head rejects at its leading token.
- Parser conformance for the projection row: `keep box.value` parses as
  `Proj(A(keep, box), value)`.
- Bare-operator-value grouping per `:357-363`, generic and reserved alike.
- Catalog source migration for every site that relies on the non-conforming
  parse, with a census establishing the set.
- The re-gated `seed-reserved-infix-names.md` rows go green.

# Not this node

- Any change to `32 §3` itself. The pin is the authority this node answers to;
  narrowing it would remove the grounding rather than satisfy it.
- The six-name admission of [[LANG-RESERVED-INFIX-NAMES]] (A0), which is
  released and bounded to unchanged syntax.
- The temporal rows (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`), a separate gate.
- The lambda/`let`/`match` and arrow rows, which conform today.

# Sizing / tier

**Size L, tier T1 -- and the L is a placeholder over an unmeasured quantity.**
The two parser divergences are bounded and well-specified. The catalog
migration is not: its extent is the number of catalog-source sites relying on
ungrouped projected arguments, and **nobody has counted them.** Exactly one is
known, from the control that failed.

⇒ **The first framing act is that census**, not a frame. If it returns a large
or heterogeneous set, this node decomposes -- parser conformance and catalog
migration may not be one deliverable, and "must land as one commit" is not a
constraint anyone has established here.

# Contention

`crates/ken-elaborator/src/parser.rs` and catalog source. Overlaps A0's file
set, so it must not run concurrently with a live A0 candidate -- check
[[LANG-RESERVED-INFIX-NAMES]]'s node status at `origin/main`, not for a branch
ref, before releasing this.
