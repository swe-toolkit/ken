# `RT-PLANNER-KRET-GRAFTED-SPINE` — frame

**Owner:** runtime. **Size:** M. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `ddbc803228dbb56f73df31a906ec1fe9a413910e`.

> ## THE DELIVERABLE IS THE REPAIR AND THE TWO ROWS IT CLEARS.
>
> The diagnosis is **finished** and is not this node's work. `D0` is a repair,
> not a measurement. A candidate whose roster contains no change under
> `crates/` has not advanced this node.

## 1. Fixed inputs

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs
      exact_response_ret_identity          the defect site

    crates/ken-runtime/.../units.rs        LOWERING, added by amendment below
      define_static_response_owner_bodies  the SECOND statement of the same
                                           expectation (~:3430-3432 at
                                           1be846b2d)

Re-derive by **symbol**, not by line — the `:1379` coordinate in the ruling was
measured on another tree and coordinates are perishable.

**THE EXPECTATION IS STATED TWICE, AND ONLY ONE STATEMENT IS DERIVED**
(Architect `evt_1cwnksydmx1rz`). In `define_static_response_owner_bodies`,
two lines apart:

    check 1  the carrier's TAG          parameterised by k_ret_identity
    check 2  the carrier's FIELD COUNT  a hardcoded literal 1, which is
                                        ITree::Ret's arity and nothing else

**On these two rows**, whose composed K result is predicted to be `Vis`, a
correct derivation makes check 1 pass and check 2 fail. **Predicted, not
measured** — nobody has read what `emit_carrier_field_count` returns for a
`Vis` carrier. It is **not** a general property of the repair: wherever the
composed K's result is itself a `Ret`, the literal `1` is accidentally correct
and both checks pass. Both live in the same `require_i64` family, produce the
same trap and the same `UnclassifiedRuntimeTrap { terminal_value: -1 }`, and
therefore have an **identical external signature** — which is why a repair that
works **can** look exactly like a repair that did nothing. Do not attribute a
`-1` to a particular check without a probe.

**The defect, settled and not to be re-litigated:**

    exact_response_ret_identity reads ONE syntactic occurrence and composes
    nothing.
    42 §6.4 requires the grafted spine.

**The witnesses**, in `crates/ken-cli/tests/px7f_resource_native.rs`, owned by
`RT-PX7F-LINKED-PUBLIC-ROWS` until this node's repair lands:

    :314  linked_public_right_denial_preserves_exact_masks   DISCRIMINATING
    :348  linked_public_second_release_is_closed_and_the_handle_closes_once

`:348` **discriminates nothing** — its `K` operation equals its own origin's
operation by coincidence of constructor, which is value equality and not
identity. Do not read it as a second confirmation.

## 2. Deliverables

- **`D0`** — the repair: derive `k_ret_identity` following `bind`'s grafting
  rather than the immediate occurrence, **and derive BOTH statements of the
  K-result expectation**. The planning-side tag derivation and the lowering-side
  field count are one deliverable: a literal `1` two lines below the check you
  parameterised **is** that derivation with the derivation missing. Finishing
  one and not the other leaves the row red with no correction upstream able to
  move it. **This is `D0` finished, not `D0` widened** — the expectation was
  written in two places by an author who had one answer for both.
- **`D1`** — the two rows above un-ignored and green, or a grounded statement of
  what still stops each one.

## 3. Acceptance criteria

- **`AC-1` — `right-denial` GREEN IN CI.** The discriminating witness passes.
  *(Control: the row FAILS at this node's base — run it before the repair and
  record the refusal. An AC met at the base measures nothing.)*
- **`AC-2` — THE REPAIR IS DERIVATIONAL, NOT A SPECIAL CASE.** No branch keyed
  on an operation identity, an arena ordinal, a fixture name, or a bracket
  shape.
  *(Control: state which construct in the repair supplies the composed
  continuation, and name one program NOT in the px7f family whose planning path
  goes through the changed code. If no such program exists, say so — that is a
  finding about reach, and it is a result.)*
- **`AC-3` — NO REGRESSION.** Green in CI, never a local `--workspace` run
  (`COORDINATION §12`). Local work is `scripts/ken-cargo -p ken-runtime` and
  `-p ken-cli --test px7f_resource_native`, nothing wider.

## 4. Hard stops — report, do not work around

- A repair that reaches outside `ken-runtime` planning, **except for the one
  emission call named in `§1`**: in `define_static_response_owner_bodies`, the
  literal `1` in `require_i64(ret_fields, 1)` becomes the arity of the
  constructor `k_ret_identity` names. Nothing else in emission is in scope, and
  needing a second site is a fresh hard stop.
  *(Steward scope ruling `evt_88t2h5g77dkg`, 2026-09-19, on the Architect's
  recommendation. The original bullet was this frame's own prose, not a spec
  rule — and it made `D0` unachievable, because the expectation is stated in two
  files and only one is reachable from planning. A frame whose deliverable
  cannot be reached inside its own scope is a defective frame. The implementer
  was correct to stop rather than reach: the bullet fired exactly as written.)*
- A row that needs a **new** `#[ignore]` anywhere to make progress.
- The repair lands and `right-denial` still fails — that is a finding that the
  mechanism statement is incomplete, and it **outranks** finishing the node.

## 4a. The mapping check — RUN AND PASSED before this node started

**Nothing is owed here. This section records a discharged check rather than
asking for one, and it is kept precisely because it passed.**

The Architect's operation-to-ordinal mapping rested on one link **they named
rather than glossed**: that `declare_inductive` allocates constructor ids
sequentially in declaration order. They did not read the allocator, and said so.

**The check, run by runtime-implementer at
`29e6b21e1f8c5b80557428693a66eb61c19ea4c4`** — count the `FSOp`-parented entries
in the `names` buffer already dumped. Predicted 21: ten carrying real spellings
and eleven carrying `ctor_NNN`, the eleven strictly above every named one.
Measured:

    FSOp total = 21    named = 10    unnamed = 11

    named     AppendFile ChangeMode CreateDirectory Metadata ReadDirectory
              ReadFile RemoveDirectory RemoveFile Rename WriteFile
    unnamed   ctor_541 .. ctor_551, contiguous

**Exactly the predicted partition**, and the named ten are exactly the public FS
operations. ⇒ The window's position is fixed **by the artifact**, not by a read
of the prelude, and `ctor_543 = PrivateResourceRelease` stands on a measurement.

**Why this section survives having passed.** A discharged check deleted from a
frame leaves the mapping looking *assumed* to every later reader, and the next
seat cannot tell the difference between a thing that was verified and a thing
nobody thought to question. **Keep passed checks; mark them passed.** The
prediction is recorded alongside the measurement so the agreement is legible as
an agreement rather than as a lone number.

**The ruling never depended on this.** `§1`'s two sentences are normative and
would stand whatever the buffer said. What the check protected is the
*row-level* reading in `§1` of the node file — **which row departs** — and that
reading is now measured: `right-denial`.

## 5. Symptom inventory — ARMED AT FILING

**Seeded at framing this time.** The parent node reached two hard stops before
anyone seeded one, and reconstructing it from the thread was recovery work
`§1b` exists to make unnecessary: the count is re-derivable from a thread and a
pattern across stops is not, and the thread is the first thing a compaction
discards.

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...

1. the K-result expectation is stated TWICE in the emission and only one
   statement is derived (tag parameterised by k_ret_identity, field count
   a literal) -- keyed on a constructor's ARITY hardcoded as the Ret
   assumption
```

**Hard-stop count on this WP: 1.** The `§1a` research pull fires at 3. **The
parent node's count of 2 does NOT carry** — different WP, different question.

## 5a. A known boundary — RECORD IT, do not work on it

`k_ret_identity` is a **single** `ConstructorIdentity`, so the plan assumes the
composed K's result constructor is statically unique. **Under grafting that
assumption can fail:** `bind (f r) k` may reduce to `Ret` on one response arm
and `Vis` on another. The checkpoint code already fails closed on it ("more
than one constructor result"), which is the right behaviour.

This is here so the next reader meets it as a known boundary rather than as a
surprise. **Not this node's work unless a row reaches it** (Architect,
`evt_1cwnksydmx1rz`).

## 6. Contention

The repair is in `ken-runtime` planning; the rows are in `ken-cli` tests and are
**owned by `RT-PX7F-LINKED-PUBLIC-ROWS` until this node clears them**. That node
may relabel those rows to point here before this node starts. Coordinate through
the runtime-leader rather than both editing
`crates/ken-cli/tests/px7f_resource_native.rs`.

`RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION` is **not** this subject — the
Architect read its row and it is producer/sink placement at the true
`StaticWorker` producer. Same family, different question.
