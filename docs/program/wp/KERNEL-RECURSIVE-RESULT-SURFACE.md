# KERNEL-RECURSIVE-RESULT-SURFACE — the Spec contract for naming a
# kernel-supplied recursive method result

**Owner:** `spec-enclave`. **Size:** M. **Deliverables in this frame:** `D0` and
`D1` only. **Implementation is an uncreated successor** owned by
Language/elaborator; the Steward creates it when `D0` lands.

Tracker node: `docs/program/issues/KERNEL-RECURSIVE-RESULT-SURFACE.md`. **Read
it first** — it carries the measured obstruction, the Architect's approved
semantic shape, and the two prohibitions. This frame does not restate them; it
fixes inputs, scopes `D0`/`D1`, and states acceptance.

## Fixed inputs, measured at `origin/main` `90ddcf1c` (2026-08-10)

Perishable-frame discipline: every figure below is measured at that SHA. **If
`main` has moved when you pick this up, re-measure before relying on any of
them, and say so in your first checkpoint.** A number in this frame is a fixed
input, never an acceptance criterion.

| input | value at `90ddcf1c` |
|---|---|
| governing surface spec for `match` → `elim_D` | `spec/30-surface/34-data-match.md`, 834 lines; §3 is `Pattern matching → elim_D`, §3.1 compilation, §3.2 dependent-motive recovery |
| governing elaboration spec | `spec/30-surface/39-elaboration.md`, 1063 lines; §2 what elaboration does, §3 what it must guarantee, §4 errors and diagnostics |
| the two gated conformance rows | `conformance/kernel/inductive/seed-nested.md` — `nested-size-uses-lift`, `nested-dependent-motive-uses-lift` |
| marker census, `^### ` headings in `seed-nested.md` | **14** |
| corpus-wide equivalent | **15** (the extra marker is in `conformance/kernel/judgments/seed-judgments.md`) |

⚠ **Any criterion citing a census count must name its population.** The two
counts above are both correct for different populations, and conflating them is
how the node's own prediction was wrong by one row before the Architect ruling.

## `D0` — the Spec contract

**This deliverable lands before any implementation frame exists.** Do not skip
it on the grounds that the semantic shape is already ruled: the Architect ruled
the *shape* and explicitly refused to fix the *spelling*.

`recursive-result` as it appears in the node is **metanotation**. Choosing the
real surface spelling is `D0`'s first task, not an inherited decision.

`D0` must settle four things, in `spec/30-surface/`:

1. **The surface spelling**, with its grammar production. It is not a function,
   not a generated identifier, and not general recursion — say what it *is* in
   the grammar's own terms.
2. **Scoping rules.** It is valid only for a surface variable carrying the exact
   recursive-result association, and must reject everywhere else. State the
   scope in which that association exists.
3. **Diagnostics.** What the elaborator says when it is used outside a lifted
   recursive field, and when the association is missing, duplicated, swapped, or
   foreign. `39-elaboration.md §4` is the diagnostics home.
4. **Interaction with ordinary direct and W-style matches.** Both must be
   unaffected; the contract must say so normatively rather than by omission.

**Scope: `spec/` only.** No `crates/` change, no conformance edit (that is
`D1`), no elaborator work.

## `D1` — the conformance contract

What `seed-nested.md` must say once the capability exists, plus the restoration
of `nested-size-uses-lift`'s executing binding.

**Route with the conformance-validator.** It rejected three `D6` candidates on
exactly this row and holds the fidelity standard the eventual binding must meet.

⛔ **`D1` states the contract; it does not bind the rows.** Binding is a `D6`
successor with fresh QA, Architect, and frontier-class CV review. **No verdict
from the four spent `D6` candidates transfers to anything.**

## Acceptance

The node's `AC-1` … `AC-5` are the *capability's* acceptance and are inherited
by the implementation successor, not discharged here. This frame's ACs are the
contract's:

- **`AC-D0a` — the spelling is fixed and grammatical.** A production exists in
  `32-grammar.md` or `34-data-match.md`, and no text in `spec/` still uses
  `recursive-result` as metanotation for the chosen form.
- **`AC-D0b` — the rejection surface is specified positively.** The contract
  states where the form is *valid*, and rejection everywhere else follows from
  that statement rather than from an enumeration of bad cases. **An enumeration
  fails this row** — it cannot cover associations nobody listed.
- **`AC-D0c` — each of the four failure modes (missing, duplicate, swapped,
  foreign) has a named diagnostic**, and the contract says the elaborator fails
  closed on each.
- **`AC-D0d` — direct and W-style matching are normatively unchanged.** The
  contract asserts it; a reader must not have to infer it from silence.
- **`AC-D1a` — the conformance contract names both gated rows** and states what
  each must assert once the capability lands, without binding either.
- **`AC-D1b` — the census effect is stated with its population named.** Binding
  both rows later reads `14 → 12` in `seed-nested.md` and `15 → 13`
  corpus-wide.

**Control for `AC-D0b`:** construct one association the contract does not
mention by name and check the stated rule decides it. If you must extend the
text to answer, the rule was an enumeration.

## Contention check

**No file contention with any in-flight lane.** `D0` touches `spec/30-surface/`;
`D1` touches `conformance/kernel/inductive/`. Runtime's active campaign is in
`crates/ken-runtime/src/cranelift_backend/`, and Kernel is `active` only on
`AC-K12`, which is Runtime-blocked.

**This node consumes a spec-enclave seat, not a build lane.** That is why it can
run concurrently with the RecursiveDescent campaign rather than behind it.

## The dependency in the frontmatter is whole-node and does NOT gate `D0`

`depends_on: [KERNEL-NESTED-IND]`, and that node is `active` **solely on
`AC-K12`**. `D0` is a Spec contract about spelling, scoping, diagnostics, and
match interaction. **None of that is a function of `AC-K12`**, and `D0` may
start immediately.

Recorded so the sequencing is a decision rather than an inherited assumption:
a constraint can be practically binding for a reason that is false.

## What this unblocks

Two `seed-nested.md` rows, and — via Architect `evt_6ysrp62e4zayg`, which
extended the obstruction to `List`-carried recursion — **DS-9 `D3`+**, the
unbounded Json fold over `JsonArray : List Json` and
`JsonObject : List (Pair String Json)`. Foundation is idle directly behind that;
DS-9 `D1` and `D2` have both merged.

`D2`'s standalone `List Char` recursion does **not** share the blocker and is
not reopened.
