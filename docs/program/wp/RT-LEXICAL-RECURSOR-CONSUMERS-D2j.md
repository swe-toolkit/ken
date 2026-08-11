# RT-LEXICAL-RECURSOR-CONSUMERS D2j — per-member derivation provenance

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Successor of [[RT-LEXICAL-RECURSOR-CONSUMERS-D2h]], created by the Steward
scope ruling `evt_2vfgg71s847ns` as corrected by Architect ruling
`evt_4psbpktt6tv75`.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

> ## TWO ACCEPTED PARTIALS LANDED. THIS FRAME IS NOT DISCHARGED AND `#6d` IS
> ## NOT CLOSED.
>
> **Partial 2 — `f3f2c1a0`, PR #1878, Decision `dec_35g9zxj3a8c0b`**, merged
> 2026-08-11 from base `5f4b514b`: one planning path, `+294/-0`, test-side
> only. **This discharges Deliverable 2.**
>
> **Partial 1 — `0e5aba4e`, PR #1874, Decision `dec_4gds5week8a6w`**, merged
> from base `301e1099`: two paths, `+142/-4`, crate change **comments-only**
> (zero non-comment changed lines).
>
> **STILL OUTSTANDING, and this list is the node's remaining work — not a
> record of what was once owed:**
>
> - the **source-side provenance mutations** (Deliverable 1's third column),
> - the **six planner-valid refusals** (Deliverable 3).
>
> ### What Deliverable 2 cost, and the lesson belongs in Deliverable 1
>
> The witness is a paired control — one bare candidate projecting **zero**
> ordered inputs, one wrapped candidate projecting **two**, with a
> hand-derived shifted marker plan reaching exact transport validation.
>
> **Two successive revisions carried an owner claim with no assertion behind
> it.** First that the ordered inputs were the *producer's* parameters; then,
> after that was caught, that they were *neither* fusion side. Both were
> wrong. The test had been reading `first_owner` **out of the run it was
> checking**, so "every input shares it" was self-consistency and could not
> constrain which unit it is — **the claim beside it could not fail.**
>
> Measured: they are entry-ABI parameters of the **consumer's own** unit.
> Three assertions now stand where the prose was — the owner equals the
> consumer's unit, differs from the producer's, and is independently pinned to
> `PredeclaredFunctionId(3)`. QA confirmed it causally by mutating the sole
> production call site of `continuation_owner_entry_sources` to
> `producer_owner`, reddening the unchanged control, then restoring
> byte-identically.
>
> The implementer's own statement of it, which is the standard for every
> remaining row: **"A relation I haven't asserted isn't a finding, however
> carefully I've written it down."**
>
> ⇒ **`AC-1`'s "gap that reads as coverage" is exactly this.** A per-member
> row resting on prose is worse than a missing row, because nothing prompts a
> reader to check it. **Every row needs its assertion.**
>
> **What landed is a measurement and a boundary, not evidence.** Specifically:
> the completed-range record, and the census result that **a generic non-empty
> projection census does NOT promote an unmeasured fusion candidate** — that is,
> the census is not a substitute for the witness Deliverable 2 still owes, and
> it was measured in order to establish that it is not.
>
> **Plus the provenance-boundary correction**, which is the Adversary finding on
> merged `D2h` (`evt_5qc5nz5k3x5c`) triaged here rather than reopening `D2h`
> (`evt_7da668d2tw9pk`). `rederive_fusion_key` derives `recursive_position` from
> `key.consumer_binding.recursive_position` — a field of the key — while its
> comment claimed independence from the key. The comment now states the narrow
> truth: the selector establishes only that the position named by
> `key.consumer_binding` is **declared on the case**, with independence
> **conditional** on the later `ih_bindings` re-establishment plus the caller's
> whole-key equality. The enclosing comment names precisely the three
> **unconditional** locators and routes the position to that qualification.
>
> **When you write the `recursive_position` row of Deliverable 1's matrix, it
> inherits that conditionality.** A row claiming unconditional independence
> there would be false, and `AC-1` calls that the gap that reads as coverage.

> ## GATE — SATISFIED. `D2h` landed at `30efb016`; this frame is startable.
>
> **Verified by the Steward on the landed tree, 2026-08-11**, so you do not
> repeat it: `StaticContinuationFusionId` (`static_transition.rs:8509`),
> `StaticContinuationFusionPlan` (`:8746`) and
> `build_static_continuation_fusion_plan` (`:8823`) each carry
> `#[cfg_attr(not(test), allow(dead_code))]` — **not `#[cfg(test)]`.** They are
> compiled into a non-test build. The `allow` suppresses the dead-code lint
> only because their consumer is `D2f`, which does not exist yet; that is the
> designed state, not a shortfall.
>
> **Do not treat `allow(dead_code)` as test-only gating** and do not
> "re-productionize" anything — the plane is already production planner state
> and is `D2f`'s fixed input. Re-derive your merge-base from `origin/main`
> (currently `30efb016` or later); **do not start against a `D2h` branch.**
>
> The original stop still stands in one form: if you find the plane behind
> `#[cfg(test)]` on the tree you actually build, **stop and tell me** rather
> than working around it.

## Why this node exists

`D2h`'s original `AC-1` demanded *"two complete planner-valid keys per
distinguishable identity class."* The Architect's correction is that this
sentence carried **two different obligations**, and conflating them is what
produced both an over-sized estimate and an under-powered candidate:

| obligation | property | where it lives |
|---|---|---|
| **Collision** | the interner is a function of the whole structural key | `D2h`, as an interner-unit matrix |
| **Derivation** | each key member equals the planner fact it claims to record | **here** |

**`a77ba94a` satisfied neither**, and it is worth being exact about why, because
the shape recurs. It mutated **clones** of the interned key and asked `id_for`
for a lookup. A `None` proves the map is keyed on that field — it is not an
interning test, because nothing was ever interned; and it says nothing at all
about derivation, because a clone is not something the planner produced.

**This node owns the half that no synthetic mutation can reach.** A key member
can be structurally distinct in the map and still be derived from the wrong
planner fact, and the interner would never notice.

## The measured fact that made this unavoidable — NOW DISCHARGED by `f3f2c1a0`

> **Read this section as history, not as current state.** It is why
> Deliverable 2 existed, and Deliverable 2 has landed: there is now a paired
> witness projecting **two** ordered inputs against a bare candidate
> projecting zero. **The sentence below was true until `f3f2c1a0` and is
> false now**, and it is kept because the reasoning still governs any *new*
> member you cannot reach.

**The ordered ABI input projection had never run non-trivially on any witness.**
`intrinsic_environment_floor` is `entry_sources.len()`
(`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:6947`),
and `required_input_count` rises above it only when a case body needs a longer
surrounding prefix (`:6948` onward). The landed `D2g` twin's consumer has
neither, so its projection is **empty**.

⇒ Nothing has checked that this member is derived correctly, independent of
whether two keys can differ in it. Runtime's `continuation_inputs.clear()`
mutation on `a77ba94a` was a no-op **that would have read as coverage** had the
differ-from-base guard not caught it.

**No test-side work produces a non-empty projection.** It requires a
structurally different consumer — one that takes entry values, or whose case
body needs a longer required environment. That witness is Deliverable 2 and it
is not optional scope.

## Deliverables

### 1. The per-member provenance matrix

**One row per key member.** The members are `D2h`'s, unchanged — the
domain-tagged original producer-invocation emission owner and exact edge;
producer owner, result root, construct origin, selected alternative, recursive
position; consumer owner, continuation frame, selected body, and the exact
IH-consuming `Call`; the checked transport coordinate **counted as its three
resolved authorities** (frame, slot template with its occurrence path,
invocation template with its occurrence path); and the complete ordered ABI
input projection.

Each row states four things:

1. **The exact authoritative planner fact** the member comes from — a
   `file:line` and the function that owns it, not a description.
2. **A reaching positive witness on which that fact is non-degenerate.** An
   empty vector, a single-element set, a `None`, or a value that coincides with
   its neighbour is **degenerate**, and a row resting on one is not discharged.
   This is the criterion the ordered-input member failed.
3. **An independent source-side mutation or transplant** that either changes
   the re-derived member or refuses before interning. Source-side means the
   declaration or plan input — **not** a mutated key struct, which is `D2h`'s
   instrument and answers a different question.
4. **Agreement** between the primary derivation and the independently authored
   re-derivation `D2h` landed.

### 2. The non-empty ordered-input witness — LANDED at `f3f2c1a0`

A consumer with entry values, or a case body requiring a longer surrounding
prefix, such that `required_input_count` exceeds zero and the projection is
genuinely populated. **State the count.**

**Delivered: a paired control, zero inputs on the bare candidate and two on
the wrapped one**, with a hand-derived shifted marker plan reaching exact
transport validation, pinning `EntryAbi`, ordinal order, and `Parameter`
source. **This discharges the ordered-input row of Deliverable 1**, and any
other row it can reach — but it discharges those rows **only where you write
the assertion**, per the owner-claim lesson in the header block.

### 3. The six relocated pre-interning refusals

Frame, selected slot, invocation, exact suffix, call identity, and segment
owner — each **independently** reaching no id and no descriptor.

These relocated on measurement, not estimate: Runtime enumerated
`ContinuationProductionMutation` on exact `1139e0be` and its complete variant
set is `Exact`, `ResultLifetimeProxy`, `ConstructorFieldCountPrefix`,
`DescriptorOrdinalSources`, and `DescriptorInputCountTruncation` — **none of the
six.** So each needs a planner-valid transplant, which is why they are here and
not in `D2h`.

**These are the `D2b`/`D2d` inheritance.** They are what separates an identity
from a value that happens to be unique in the measured population, and that is
the whole reason `#6d` stays open until this node lands.

## Sizing — read this before you estimate

**One real witness may discharge many rows.** The matrix is per-member in its
*claims*, not necessarily in its *fixtures*.

- Additional `d2g_declaration` knob variants are owed **only** where the
  existing witness plus the production-mutation harness cannot make a member's
  source causal. The builder is already parameterized (`:14874`).
- A **pair** of planner-valid programs is owed **only** for a member whose
  derivation could otherwise alias or normalize two genuinely distinct planner
  facts. **Never as a blanket condition on every field** — that reading is the
  one the Architect explicitly corrected, and it is what produced the "roughly
  twenty fixtures" estimate that stopped `D2h`.

If your fixture count is approaching one-per-member, that is the signal you
have inherited the retired reading. Come back to me before building it.

## Acceptance criteria

**AC-1 — every row is discharged or explicitly owed.** A member with no row is
a gap; a member whose row rests on a degenerate witness is a gap **that reads
as coverage**, which is worse. Name any row you cannot discharge and why,
rather than omitting it.

**AC-2 — the ordered-input row is discharged on a non-empty projection**, with
the count stated in the claim. This is the row the node exists for.

> **STILL OPEN, and do not read `f3f2c1a0` as having closed it.** That merge
> landed the **witness**; `AC-2` is about the **row**, which lives in
> Deliverable 1's matrix and is not written yet. What changed is that the
> obstacle is gone — a non-empty projection exists now, count **two**, so the
> row can be discharged rather than merely owed. **The witness is the
> precondition, not the discharge.**

**AC-3 — the six refusals each reach no id and no descriptor, independently.**
Baseline mints exactly one identity first, so each zero is a change rather than
the fixture's resting state. Earlier `D2g`/`D2i` controls do not substitute.

**AC-4 — source-side, not key-side.** No row is discharged by mutating a key
struct. If a row can only be reached that way, it belongs to `D2h`'s
interner-unit matrix and you say so rather than counting it here.

**AC-5 — `D2h`'s interner-unit matrix is not re-litigated.** It is a landed,
labelled instrument for a different property. Do not extend it, do not restate
its results as derivation evidence, and do not treat its passing as covering a
row here.

**AC-6 — measurements carry their population in the claim.**

## Excluded scope

- **No `D2f` work.** No emission, no `AbiUnitDefinition` arm, no
  `ContinuationEmissionOwner::Fusion`, no producer-invocation edge redirection.
- **No `R3` green claim.**
- **`continuation_result_origins` must not be widened** (Architect
  `evt_1dgwdvxhnabg4`). If a witness appears to require it, that is a stop.
- **No eighth fact.** The key is closed at the Architect's seven. If a row
  cannot be discharged without one, **stop and report** rather than extending —
  that is the closed-contract failure and it is mine, not a slice.
- No traversal widening, worker scan, parallel fixed point, optional
  transport/worker, or continuation-specialization change.

## Stop conditions — return to me, do not decide

- **`D2h`'s plane is not reachable from a non-test build** (the gate above).
- **A non-empty ordered-input projection turns out to be unreachable** by any
  consumer shape. That would mean the member is not derivable rather than
  merely unexercised, which is a mechanism finding and changes the key.
- **A row needs an eighth fact.**
- **Your fixture count approaches one per member** — the sizing note above.

## Contention

Runtime's own lane, `crates/ken-runtime/src/cranelift_backend/planning/`. The
concurrent Language node touches `crates/ken-elaborator/`; the intersection is
empty. **Re-derive it at candidate time** — a merge-base goes stale without your
branch moving.

## Validation

`scripts/ken-cargo test -p ken-runtime`, and the focused suite for the new
controls. **Never `--workspace`** — that is CI's gate, not the laptop's.
