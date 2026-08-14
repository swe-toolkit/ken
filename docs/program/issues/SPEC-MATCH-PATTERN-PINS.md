---
id: SPEC-MATCH-PATTERN-PINS
title: "the five spelling pins 34 §3's absent pattern forms need before any of them can be cut as a slice -- as-association/precedence, tuple comma-versus-grouping, record field_pat form, the or-pattern binder join, and the literal-kind-to-value-comparator table -- none of which exists today, so a Language slice built now is a slice that gets rebuilt"
status: merged
owner: spec-enclave
size: M
gate: none
depends_on: []
blocks: [LANG-MATCH-PATTERN-FORMS-ABSENT]
github: null
origin: "Spec-enclave disposition evt_12qrtnp7237dn (2026-08-14), answering the Steward's routed question on LANG-MATCH-PATTERN-FORMS-ABSENT. The enclave ruled 34 §3's six absent forms implementation debt rather than an aspirational menu, gave a prerequisite-ordered cut, and named its own next material in terms: 'the next material needed is small spec pins (as association; tuple comma/grouping; record fields; or join; literal comparator table), then release a first contained as-pattern slice rather than a six-form frame.' Steward-filed per COORDINATION §2 because a disposition posted in a thread has no node and evaporates -- the same reason RT-CONTKEY-REFUSAL-PROFILE-SPLIT and LANG-REACHABILITY-SUBSUMING-ARMS were filed the same day."
---

## What this is

**The enclave named its own next material and this is that material, filed so it
exists.** `34 §3` lists nine pattern forms; the AST has three
(`ken-elaborator/src/ast.rs:167`, and `MatchArm` at `:86` has no guard field).
The absent six are implementation debt, **stageable only as explicit tracked
slices with every remainder fail-closed until its slice lands.**

**But no slice can be cut yet, and the reason is spelling rather than
semantics.** Each form needs a pin that does not exist, and **a Language slice
built against an unpinned spelling is a slice that gets rebuilt.**

## The five pins

Ordered by the enclave's own cut order, so pin 1 unblocks the first slice.

**`P1` -- `p as x` association and precedence.** The first slice
(as-patterns) is the smallest delivery in the chapter and cannot start without
it. The pin must settle how `as` associates against constructor application and
against `|`, since `P4` depends on the interaction.

**`P2` -- tuple: `(p)` is grouping, and a tuple requires a comma.** Without
this, `(p)` is ambiguous between a one-tuple and a parenthesized pattern, and
the arity->right-nesting rule has nothing to attach to.

**`P3` -- record `field_pat` spelling and selection.** Five sub-questions the
enclave enumerated: label/value form; punning; omission, and whether a record
pattern is open or closed; duplicate and unknown fields; and whether source
order is significant. **Do not answer these by analogy to tuples** -- the
enclave was explicit that record patterns are not to be bundled with tuple
patterns merely because both project.

**`P4` -- the or-pattern binder join, and its association with `as`.** Four
parts: identical name sets across alternatives; **exactly one binding per
alternative**; corresponding types **definitionally equal in the common
pre-branch context**; and a canonical branch environment.

**`P5` -- a literal-kind-to-value-comparator table, plus the corrected
citation.** The enclave's finding here is stronger than the tracker previously
recorded: **`DecEq Char` alone is insufficient.** `Float`/`Float32` and
`Decimal` distinguish **runtime value equality** from **lawful proof `DecEq`**,
and numeric literals additionally require expected-type checking. The table must
say, per literal kind, which comparator applies and which equality authority
licenses it.

## Deliverables

**`D1` -- each pin written into the normative spec text**, in the chapter that
owns the spelling (`32-grammar` for surface form, `34` for match-specific
behaviour). **A pin recorded only in this node is not a pin.**

**`D2` -- for `P5`, the comparator table as a table**, per literal kind, naming
the comparator and the equality authority. Prose describing it is not the
deliverable; the next reader is building against the rows.

**`D3` -- the corrected citation `P5` names.** Identify what the current
citation says and what it should say. If the citation is already correct at this
base, **say so and move on** -- the enclave named it from its own reading and a
re-check that clears it is a legitimate outcome.

## Acceptance criteria

**`AC-1` -- all five pins are answered, or the ones that are not are named with
the reason.** A partial delivery is fine and expected; **a partial delivery that
does not say which pins are still open is not.**

**`AC-2` -- `P1` is answered.** It is the one that unblocks the first slice, and
a delivery without it leaves the chain exactly where it was.

**`AC-3` -- the pins are consistent with each other**, specifically `P1` against
`P4`: `as` and `|` association must compose, and answering them independently is
how they end up contradicting.

**`AC-4` -- no elaborator change.** This node is spec text. **If a pin turns out
to be unanswerable without an implementation decision, stop and report which pin
and why** -- that is a routing question, not something to settle by writing the
easier answer.

**`AC-5` -- conformance rows are not claimed for unimplemented forms.** Pinning
a spelling does not make the form exist; the fail-closed remainder rule stands
until each slice lands.

## Sizing

**`M`.** Five pins, four of them small spelling questions and one (`P5`) a table
that has to be right. **`P3` and `P4` are the two that can grow** -- record
open-versus-closed and the or-pattern binder join are semantic decisions wearing
spelling clothes. **If either does, deliver the other four and report it**;
`AC-1` is written to make that a clean outcome rather than a shortfall.

## Not this node

- **Not any pattern-form implementation.** Every slice is Language's and none is
  released. This node makes them cuttable, nothing more.
- **Not an amendment to `34 §3`'s obligations.** The enclave ruled them real and
  present-tense; the "drop a form from the surface" branch is closed.
- **Not the umbrella census.** [[LANG-MATCH-PATTERN-FORMS-ABSENT]] stays `draft`
  and holds the cut order; it flips **per slice**, after that slice's pin lands.
