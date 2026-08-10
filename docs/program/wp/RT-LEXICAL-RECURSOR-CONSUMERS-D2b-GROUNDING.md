# RT-LEXICAL-RECURSOR-CONSUMERS — `D2b` grounding

> # ⛔ HISTORICAL. FROZEN AT THE `D2b` GROUNDING COMMIT.
>
> Named by **subject**, not SHA — the commit whose subject is *"`D2b`
> grounding: the closed projection IS available, measured"*. Re-anchoring
> rewrites SHAs; it does not rewrite subjects.
>
> **The claim below that is NO LONGER TRUE:** *"Evidence only: no repair, no
> control, `crates/` byte-identical"*. That was true of the grounding commit.
> The **planner plane of `D2b` has since landed on this branch**, so `crates/`
> has changed and the branch now carries a repair and a control. See
> §6 and the accepted-partial section at the end.
>
> ⛔ **The MEASUREMENTS in §1-§2 are not withdrawn.** They were taken on the
> grounding tree and stand; what went stale is this file's description of the
> *candidate*.

**As of the grounding commit: evidence only, no repair, no control, `crates/`
byte-identical.**

## 0. Provenance — two coordinates

| coordinate | exact | moves? |
|---|---|---|
| **measurement base** — the tree every figure was taken on | `a6186741` | **no.** A commit on `main` |
| **candidate merge-base** — where this candidate sits | `c3162c99` | **yes.** Every re-anchor moves it; it was `a6186741` before this one |

⛔ **They no longer coincide, and every figure below is a statement about
`a6186741`.** They did coincide when this record was written; the re-anchor onto
`c3162c99` separated them, and the sentence that said *"they coincide today"*
survived the table row it depended on — which is the same defect, one field
lower, that the two rows exist to prevent.

⛔ **Path-disjointness is NOT promoted into provenance.** The re-anchor touched
no path these measurements concern, which is why the evidence remains
*applicable* at the candidate base. It is **not** a reason to say the figures
were taken there. "Still applicable" and "taken here" are different claims, and
only re-measuring buys the second.

> # THE GATING CONDITION PASSES. `D2b` IS NOT BLOCKED.
>
> The ruling's stop clause — *"if closed projection is unavailable, stop rather
> than widening guard/ABI"* — **does not fire.** The closed projection is
> available from existing interned planner facts, measured below.
>
> This checkpoint stops for **budget, not for a blocker.** §4 states exactly
> what remains.

## 1. The defect, confirmed and sharper than stated

`ContinuationUnitView::ordinary_envelope`
(`planning/static_transition.rs:1726`) builds its nonrecursive population as:

```rust
let mut nonrecursive = (0..field_count)
    .filter(|position| *position != selected)   // selected = key.recursive_position
```

⛔ **It omits exactly ONE position**, `self.key.recursive_position` — because
the key carries a **singular** `recursive_position`, not a set. Every other
constructor field is called nonrecursive by construction.

Row 3's producer has **two** recursive positions, so the second one survives
into the ordinary ABI run and a `Specialized(Closure)` is cloned into it. That
is the pre-loop misclassification the ruling names, and the method's own
`D8l2` comment block is about a *different* defect in the same loop (envelope
index vs source position) that was already repaired.

## 2. The projection IS closable — measured, not argued

The ruling asks for a projection keyed by *(emission owner, producer
result/construct, alternative, consumer/frame, recursive position)*, **set-equal
to checked `recursive_positions`**.

**Every one of those coordinate fields already exists on the interned
continuation key** (`emission_owner`, `producer_result_origin`,
`producer_construct_origin`, `producer_alternative`, `consumer_owner`,
`continuation_origin`, `recursive_position`). What does not exist is a *set* —
the planner interns **one unit per recursive position**.

⇒ So the projection is closable **iff** grouping interned units by the
coordinate-minus-position recovers the whole checked set. Measured on row 3:

**Checked:** `case[0] ctor:fixture::PX8JSiblingTree::Node → recursive_positions
[0, 1]`.

**Interned units, grouped:**

| emission owner | result / construct | alt | consumer | continuation | recursive positions |
|---|---|---|---|---|---|
| `Predeclared(0)` | `36` / `36` | 0 | `0` | `5` | **{0, 1}** |
| `Specialization(1)` | `25` / `25` | 0 | `0` | `5` | **{0, 1}** |
| `Specialization(0)` | `34` / `34` | 0 | `0` | `5` | **{0, 1}** |

**Three groups, each set-equal to the checked `[0, 1]`, each unique by source
position.** No group is short, and no position appears twice within a group.

⇒ **No new planner population is needed.** The projection is a *grouping* of
facts the planner already interns, which is what makes it readable-only by
lowering without reconstructing membership from lowered shape.

**Corroborating the defect from the same rows:** every unit reports
`ordinary_parameters = 1` with `captures = 0`, so `field_count = 2` and the
envelope emits **one** ordinary parameter while **two** positions are recursive.
Exactly one recursive field survives — the closure.

## 3. Bounds on this grounding

- **One row.** Row 3 is `D2b`'s entire population, so this is closure for the
  population — but it is not a claim about producers with three or more
  recursive positions, or with captures, neither of which occurs here.
- **Grouping is not yet validated in code.** That the grouping *recovers* the
  set is measured; the ruling additionally requires it be *enforced* set-equal,
  unique, and resolved to interned worker facts. That enforcement is `D2b`
  implementation, not this checkpoint.
- **Nothing about the four fail-closed caller conditions is measured here** —
  captured sibling, generated-context suffix, missing exact sibling unit/callee,
  recursive-set disagreement. Row 3 is stated by the ruling to be the
  zero-capture raw-route case; the measurement above shows `captures = 0`,
  consistent with that, and does not establish the other three are absent.

## 4. What remains, in the ruling's own order

1. planner-owned closed projection, with set-equality / uniqueness / worker-fact
   resolution **enforced**;
2. `ordinary_envelope` omitting **every** recursive position;
3. reconciliation of descriptor `ordinary_parameters`, slots, offsets, caller
   inputs and callee loads to the runtime-only envelope;
4. compiler-only binders — sealed case-binder plan, nonselected built at callee
   as `StaticWorkerBinding`, zero ABI slots, never a `LoweringOperand`;
5. caller-side reconciliation before `call_declared_unit_target`, with the four
   fail-closed conditions;
6. the control matrix;
7. the carried `control.rs` rider — make `D2a`'s `arrivals > 0` visibly prior to
   or inseparable from the prospective `forwards == arrivals`.

**Item 7 is due with the first candidate that touches `control.rs`**, which is
whatever lands item 6. This checkpoint touches no `crates/` path, so the rider
correctly does not apply to it.

## 5. Scope

No repair, no control, no `D2c`/`R3` traversal, no closure-transferability
change, no new `Lowered`/ABI/capture lane, no `StaticWorkerBinding`
successor-wall work, no `D3`, no `R4`, no tracker `status:` change, no
retirement. The probes were temporary and are removed —
`planning/static_transition.rs` sha256 `609e5bec…869f` matched the MEASUREMENT
base `a6186741` when the probes were removed. ⛔ It does not describe this
candidate: the planner partial has since modified that file.

## 6. ACCEPTED PARTIAL — the planner plane landed after this checkpoint

**Landed:** the closed projection is stored on the continuation key and
**enforced** — unique by source position (a set, and a duplicate in the checked
case refuses), self-membership required, and copied from the checked
`recursive_positions` rather than derived from any body, shape, arity or
constructor symbol. Its two consumers are corrected:

| site | was | now |
|---|---|---|
| `exact_continuation_ordinary_parameters` | counted every field except **the** recursive position | counts only fields in **no** recursive position |
| `ordinary_envelope` | omitted `selected` only | omits **every** projected position |
| `ordinary_envelope` field count | `nonrecursive + 1` | `nonrecursive + |projection|` |

⛔ **The `+ 1` was a second expression of the same singular assumption**, and
fixing only the filter left the envelope failing to cover its own `Parameter`
slot run — four suites caught it immediately, which is what identified it.

**Measured effect on row 3:** the `Closure` refusal is **gone**. The value no
longer reaches `boundary_transfer_admissibility` at all, because no recursive
field is an ordinary ABI parameter any more.

**Row 3 still does not compile.** It now stops at:

```text
Backend(Module("function left planned source join StaticOriginId(12) neither
emitted nor statically unselected"))
```

That is **`AC-3` guard 4, intact and firing** — the sibling's source join is
neither emitted nor unselected, because the compiler-only binder that would
account for it is **not in this candidate**.

## 7. What remains of `D2b`

1. compiler-only binders — sealed case-binder plan; nonselected built at callee
   as `StaticWorkerBinding` via `construct_static_worker_binding`, entering
   `LoweringEnvironmentBinding::StaticWorker`, zero ABI slots, never a
   `LoweringOperand`;
2. caller-side reconciliation before `call_declared_unit_target`, with the four
   fail-closed conditions (captured sibling, generated-context suffix, missing
   exact sibling unit/callee, recursive-set disagreement);
3. the remainder of the control matrix — live row 3 with a nonzero denominator
   and its exact successor refusal, the suppression leg restoring the sibling to
   the ordinary envelope, the direct raw-`Closure` boundary row, descriptor and
   assembly reconciliation rows, and the callee body/arity/capture rows.

**No wider projection, ABI, capture lane or guard change was needed for the
planner plane** — the stop clause did not fire.

## 8. Lowering-plane grounding — the three seams, located

Taken after the planner partial was approved. **Grounding only; no lowering
change is in this candidate.**

### 8.1 The guard row 3 now stops at

`Lowering::finalize_join_disposition` (`lowering/mod.rs`) requires, for the
function's `required` join set:

```text
required  ==  consumed_join_origins  ∪  dispositioned_join_origins
```

and refuses a member of neither with *"function left planned source join
`<origin>` neither emitted nor statically unselected"*. Row 3's sibling join is
in neither set: the planner no longer emits it as an ordinary parameter, so it
is not **consumed**, and nothing yet declares it **dispositioned**.

⛔ **The guard is correct and is not the thing to change.** It is `AC-3` guard 4.

### 8.2 The disposition mechanism already exists

`Lowering::disposition_statically_unselected_source_subtree(root)` marks every
planned join in a statically unselected source branch, deriving the subtree from
the planner's validated positional-child inventory — *"lowering maintains no
second source spelling inventory"*. **Three production call sites already use
it**, so accounting a nonselected sibling's joins is an established pattern
rather than a new one.

### 8.3 The binder constructor and its required decisions

`construct_static_worker_binding(closure_origin, body_origin, declared_arity,
source_capture_count, captures, route, discharge)`.

Its `discharge` argument is deliberately **required and not inferred from
`route`** — the two facets are independent, and its own comment says a caller
that has not decided which causal obligation the binding may answer for *"has
not finished building it"*. So the nonselected sibling's `RawWorker` /
`DirectSpecializationCall` choice is an explicit decision at the call site, not
a default.

### 8.4 What this does and does not settle

**Settles:** every seam the ruled mechanism names exists, is production-reachable,
and has a precedent — no new lowering population, ABI lane or guard is implied.

**Does not settle:** whether dispositioning the sibling subtree is *sufficient*
for row 3, or whether the caller-side omission must also account for something
the subtree walk does not reach. That is measurable only by building it, and it
is the first thing the next turn should measure rather than assume.

## 9. The ruled lowering plane, corrected — for whichever turn implements it

⛔ **§8 located the seams; this section records the RULED SHAPE, including one
correction to what I proposed.** No implementation is in this candidate.

### 9.1 The telescope — the sibling gets an IH, it is NOT skipped

I proposed skipping the nonselected recursive position in segment 1. **That is
wrong and is superseded.** The ruled telescope is:

```text
[ IH for EVERY recursive position, in reverse position order ]
    ++ [ argument for EVERY constructor field, in source order ]
    ++ [ continuation inputs ]
```

with the **sealed cardinality unchanged**. So segment 1 keeps one
`InductionHypothesis` per recursive position — the sibling included — and
segment 2 has an argument member for **every** recursive source field, not only
the selected one.

⛔ **The callee resolves both roles only through the planner's closed-member
projection — never by cloning the selected worker.** Cloning would make the
sibling's binding a copy of a different unit's facts, which is exactly the
substitution the projection exists to prevent.

⛔ The blocker `segment 1` currently presents — *"a recursive position that the
continuation specialization projects no worker for"* — is therefore **not**
removed by skipping. It is removed because the projection can now resolve that
position to its own interned worker.

### 9.2 The lookup I owe, and its validation contract

Keyed by `(emission_owner, producer_result_origin, producer_construct_origin,
producer_alternative, consumer_owner, continuation_origin)` **plus position**.
It accepts the selected exact view and **validates the whole group before
returning**:

- group keys **set-equal** to the checked positions;
- **exactly one** unit per position;
- every member agrees on the group coordinate **and** the checked set;
- exposes specialization, closure/body occurrences, declared arity, capture
  count and provenance, IH route/context.

⛔ **Zero, duplicate, conflict, short or extra is a planner invariant failure** —
never `None`, never first-match, never a fallback, never a lowering-side filter.

**This is the clause I under-delivered.** The landed projection is
`recursive_positions: BTreeSet<u32>` — set-equality and uniqueness only. It
closed the envelope question completely, which is why the planner plane passed;
it carries no worker facts, and the lowering plane is the only consumer that
needs them.

### 9.3 The bounded member for row 3

Exact worker facts, **empty captures**, `RawWorker`, `DirectSpecializationCall`;
IH only on the exact planner-issued raw/capture-free route. Installed as a
compiler-only `StaticWorker` with **zero ABI slots** and never a
`LoweringOperand`.

**Stop conditions:** the group is not uniquely complete, or the sibling is
captured or carries a generated context.

### 9.4 Unchanged from the earlier rulings

The caller-side reconciliation and the live-`Call` consumption of origin 12 are
as previously ruled — the caller omits the runtime ABI field only and consumes
no join; origin 12 is consumed by ordinary `lower_expr(Call)` →
`enter_source_occurrence_plan` → `call_static_worker` once the binder exists.
`finalize_join_disposition` stays byte-for-behaviour unchanged.
