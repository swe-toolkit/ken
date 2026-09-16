---
id: SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION
title: "Re-gate the application-atom contraction rows in seed-reserved-infix-names.md OFF LANG-RESERVED-INFIX-NAMES (A0) and ONTO LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE. The rows are CORRECT as written -- they transcribe the contract pin already on main at spec/30-surface/32-grammar.md §3 -- but they exceed A0's unchanged-syntax scope, and the catalog control they break is itself written against a non-conforming parser. D0 RULED RELOCATE (evt_5sf71fnjxpzmb), reversing the earlier WITHDRAW. Spec-author authors, CV validates, spec-leader Decision, Steward M1-M4."
status: merged
owner: spec
size: S
gate: none
depends_on: []
blocks: [LANG-RESERVED-INFIX-NAMES, LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]
github: null
tier: T1
origin: "Steward cut 2026-09-14 on the spec-leader's request (evt_1atgv5dm04ew4) and behavioral ruling (evt_234t0amx3ht7x). Language grounded (language-implementer evt_7x7th49xa2xv3) that the seed's application-atom contraction regresses the priority-queue catalog control priority_queue_actual_export_table_is_exactly_the_six_name_api; the clean bounded A0 checkpoint 2c63d409 omits the contraction and passes 47 focused tests + 10 mutation seams (HISTORICAL GROUNDING ONLY -- 2c63d409 is a real object but a DIFFERENT artifact from the A0 that landed: LANG-RESERVED-INFIX-NAMES shipped at 3ba6e8b04 measuring 11 focused tests and a 9-mutation AC-8 campaign. The 47/10 was the pre-decision probe that justified this forward correction; do not try to verify it against the shipped code). The contraction also contradicts the Architect's A0 decomposition (evt_784ge2nq65dfy, 'unchanged syntax'). SPEC-RESERVED-INFIX-NAMES already merged (7dea59366), so this is a forward spec correction, not a withheld merge. Re-measure the exact seed rows at the cut."
---

> ## MERGED 2026-09-16 at `98d98440da2215f33b654fd6b2ac9709c3d5ab2e`
>
> **Verified by blob, not by ancestry** — the publisher squashes, so a routed
> commit is an ancestor of nothing. The single touched path
> (`conformance/surface/operators/seed-reserved-infix-names.md`) is
> byte-identical between the approved candidate
> `12f3b4672eb35ba90ab0bd9795b33d30545910a6` and `main`, landed via PR #3768.
> Six seats were parked behind this correction; both the language and spec
> rings are kicked from this landed tree.

> # CORRECTED 2026-09-16 -- the D0 was RULED, and it REVERSED. Read this first.
>
> **The ruling is RELOCATE** (Architect, `evt_5sf71fnjxpzmb`). The earlier
> WITHDRAW ruling at `evt_64avxs9ashqqk` is **void**; nothing may be authored
> against it.
>
> **The premise this node was cut on is refuted.** It said the seed *over-reached
> into general application/projection grammar*. It did not: it transcribes a
> contract pin that is already on `main` at `spec/30-surface/32-grammar.md §3`
> (`:357-363` and `:368-378`, verified verbatim by the Steward at `origin/main`
> `e11341c7b9d11cd74879d27d555d2a5729837847`). The rows are **correct as
> written**, and so are both section titles.
>
> ⇒ **What the rows record is a live parser divergence from the spec**, and
> WITHDRAW would have deleted the only conformance witness of it. They are
> re-gated onto [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]], not corrected to
> measured behaviour and not deleted. The prose below is retained where it is
> still true and marked where it is not.

# Objective

Re-gate the application-atom boundary rows of
`conformance/surface/operators/seed-reserved-infix-names.md` off
[[LANG-RESERVED-INFIX-NAMES]] (A0) and onto
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]].

A0's released scope is bounded reserved-name admission with **unchanged syntax**
(Architect A0 decomposition `evt_784ge2nq65dfy`). The rows exceed that scope —
that part of the original reading stands, and it is why the re-gate is right.
What does not stand is the inference from it: exceeding A0's scope means the
rows belong to a different node, not that they are wrong.

# The problem (grounded; the framing is superseded — see the CORRECTED block)

The merged seed marks two row groups **RED-UNTIL-LANG-RESERVED-INFIX-NAMES**,
i.e. it demands A0 make them go green:

- `surface/operators/bare-operator-value-requires-grouping` — a bare operator
  value must newly require grouping `(OP)`; an ungrouped zero-application value
  must reject.
- the **non-temporal** rows of
  `surface/operators/identifier-headed-non-atom-arguments-require-grouping` —
  ungrouped non-atom arguments must reject, and the projection row contracts
  `keep box.value` to `Proj(A(keep, box), value)` (projection **outside** the
  application) rather than the grouped `A(keep, Proj(box, value))`.

Both state `32 §3`'s existing contract pin for general application/projection
grammar — they do not introduce it. The language ring implemented
the seed interpretation, which is the spec, and the focused catalog control
`modules::namespace_effect_tests::priority_queue_actual_export_table_is_exactly_the_six_name_api`
passed 1/1 on the base parser and **failed candidate-only** with
`TypeMismatch ... projection base's type is not a named-field owner`
(span `27523..27542`); restoring legacy application/projection parsing returned
it to 1/1. Existing catalog source relies on ungrouped projected arguments, so
satisfying the seed row requires catalog source migration — out of scope for
A0, and **in** scope for the successor by construction.

**The control's failure is not evidence against the rows.** It measures that
catalog source was written against a non-conforming parser. Reading it the
other way — as the seed being wrong — is the inference the D0 reversal
corrects.

No live regression on `main`: these are RED-UNTIL rows (expected-red, gated on
the future language node), and the catalog control stays green because the clean
A0 checkpoint `2c63d409` omits the contraction.

The **temporal** rows of the same table carry
**RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** (`OQ-syntax`) — a separate gate,
unrelated to A0. Do not touch them.

# D0 — RULED: RELOCATE (Architect, evt_5sf71fnjxpzmb)

Both row groups keep their `RED-UNTIL` semantics and move off A0 onto
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]], which also owns the catalog
migration. **WITHDRAW is void** (`evt_64avxs9ashqqk`).

Per-row dispositions, on spec-author's measurements:

- **projection** — CORRECT as written; it states the spec. Keep the tree,
  re-gate. Do **not** correct it to base behaviour.
- **`if`** — CORRECT as written; the spec requires rejection and the parser
  accepts. Re-gate. Correcting it to measured behaviour would encode the
  parser's divergence into the conformance seed.
- **the `expect-nontemporal` bullet** — its "four ungrouped leading-form rows
  reject" is the only genuinely false sentence. Split it four ways:
  lambda/`let`/`match` conform today (no gate); `if` is spec-required to reject
  and does not (red against the successor); arrow conforms (no gate at all);
  projection is red against the successor.
- **arrow** — conforms today. No tree change, and it should not carry a RED
  gate.
- **bare-operator group** — spec-pinned at `:357-363` for generic and reserved
  alike. Re-gate, do not correct. Stating today's behaviour in the row is
  allowed; the row's target stays the spec's.
- **byte-unchanged set unchanged**, including `:181` and all six temporal rows.
- **Both section titles stay byte-unchanged** — they assert the retained claim,
  so they are correct as written.
- **`:15-16` of the header status block** re-points to the successor. It states
  the mis-attribution in so many words and is outside the `:107`/`:140`
  coordinates, so an edit that fixed both row groups and left it standing would
  leave the summary paragraph asserting the old gate.

Disposition note reads **relocated, with the successor named** — not withdrawn,
not deferred. The debt is real and it is the spec's; naming it is the point.

**Count reporting.** Not a subtraction. `:101` expect-positive stays on A0 and
is untouched; splitting `:140` four ways changes how many gated bullets exist,
so report resulting counts per tag name with the moved bullets named by line and
by case id.

# Deliverable

- `seed-reserved-infix-names.md` edited per the D0 ruling so the two row groups
  above no longer gate `LANG-RESERVED-INFIX-NAMES`, preserving every other row
  (the six-name admission rows, the notation-identity rows, and the temporal
  rows) exactly.
- A one-line note in the seed (or its companion prose) recording the disposition
  and its grounding, so the change is auditable.

# Acceptance criteria

- The A0-conformance meaning of the seed is exactly: bounded reserved-name
  admission with unchanged application/projection grammar. The clean A0
  checkpoint `2c63d409` conforms to the corrected seed with no application-grammar
  change and no catalog migration.
- The bare-operator-value grouping requirement and the non-temporal projection
  contraction no longer carry RED-UNTIL-LANG-RESERVED-INFIX-NAMES. They carry
  RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE instead, except the rows
  the ruling takes off any gate (lambda/`let`/`match`, arrow).
- `:15-16` of the header status block no longer names A0 as the aggregate's
  gate.
- The temporal rows (RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE) and every non-contraction
  row are byte-unchanged. So are both section titles and every case id.
- **`spec/30-surface/32-grammar.md` is BYTE-UNCHANGED by this candidate.**
  Control: `git diff <base> <candidate> -- spec/30-surface/32-grammar.md` is
  empty. Under RELOCATE that file is the authority the whole re-gate rests on;
  narrowing its contract pin to the retained leading-keyword forms would delete
  the grounding for the successor in the same change that creates it. (Narrowing
  it was a reasonable act under WITHDRAW, which is why this is stated as a
  control rather than left to reading.)
- The priority-queue catalog control stays green with the base
  application/projection parser (no catalog source migration is implied by A0).
- Disposition matches the Architect's D0 ruling `evt_5sf71fnjxpzmb`: relocated,
  naming [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]], which exists as a real
  node. Not "withdrawn" and not "deferred".

# Not this node

- Implementing A0 itself ([[LANG-RESERVED-INFIX-NAMES]], already released;
  checkpoint `2c63d409` held pending this correction).
- Any application/projection grammar change or catalog source migration — that
  is [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]]'s scope, not this one.
- Any edit to `spec/30-surface/32-grammar.md` or `31`. The pin is what the
  re-gate cites; it is not this candidate's to narrow or clarify. If it needs
  either, that is a separate node and the Architect's call.
- The temporal-expression surface (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`).

# Sizing / tier

**Size S, tier T1.** The seed edit is small, but the disposition turns on a
design fork (Architect) and the correction must preserve every other seed row and
not silently re-introduce a grammar change. Architect required for the D0;
CV validates the corrected seed.

# Contention

Spec enclave, `conformance/surface/operators/seed-reserved-infix-names.md` only.
No `crates/` change — docs/spec content. No cross-lane contention. This UNBLOCKS
the L2 language lane's held A0 candidate; it is the precursor to A0's release.
