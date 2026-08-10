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

## 9. WITHDRAWN — the binder-telescope plan, and what replaced it

> # ⛔ THIS SECTION'S PLAN IS WITHDRAWN IN FULL.
>
> An earlier revision of §9 specified a fact-resolving member lookup, a plural
> IH telescope, a nonselected binder and caller reconciliation, and the
> consumption of origin 12 by a live `Call`. **All of it is withdrawn** by the
> Architect re-cut, and none of it is in this candidate.
>
> It is recorded as withdrawn rather than deleted, because the reason it failed
> is the useful part.

### 9.1 Why it was wrong, measured rather than reviewed

I built the whole mechanism. `nonselected_member_binding` was **never invoked**
for row 3 — zero calls — and origin 12 was **never entered**: the traversal goes
`8 → 13`.

⇒ **Row 3's failing compile never reaches a continuation-specialization
definition at all.** The binder telescope is a *specialization's* case
environment; the compile that fails is the **root machine's**. No amount of work
in that environment could consume a join the root machine never enters. That is
consistent with the earlier measurement that origin 12 lives only in
`PredeclaredFunctionId(0)`'s subtree and in no worker subtree.

It also reddened **four committed controls** whose subject is the
singular-specialization model — one of them exists solely to assert the hard stop
the plural telescope removes. Those controls are **retained unchanged**.

### 9.2 The actual route, and the landed repair

The root machine's `Let` value returns `Specialized(RecursiveBackedge)`;
`SourceContinuation::LetBody { body, env, next }` forwards it **without
scheduling the body**. Origin 12 is a **non-live body `Call`** — it never
executes, so there is nothing to consume it.

⇒ It is **dispositioned, not consumed**. At that existing arm only, before
forwarding the backedge, the landed repair calls
`disposition_statically_unselected_source_subtree(body.static_origin)`.

**The selector is the planner's own retained body root** — never a numeric
origin, never a worker or closure root, and never the whole root function, which
would swallow joins that legitimately executed. The body's source occurrence is
deliberately **not** entered: it did not execute.

### 9.3 What this candidate does and does not achieve

**Achieves:** the missing-join refusal is gone, proved by an A/B whose suppressed
leg reproduces the exact refusal from the committed tree.

**Does not achieve:** row 3 compiling. It advances to the
singular-specialization hard stop, which this deliverable **keeps** — and the
control asserts that advance positively, so a row failing *earlier* cannot pass
as a fix.

### 9.4 Wrong-root substitution is inapplicable at this typed arm

The re-cut asked additionally for a whole-root-function substitution and a
no-join sibling root alongside the suppression mutation. **Those mutations are
not missing coverage — they are inapplicable here, and structurally
discharged.**

**The boundary is the arm's own type structure.** `SourceContinuation::LetBody`
exposes exactly one root: `body: OwnedSourceOccurrence`, carrying a
`StaticOriginId`. The two candidates a wrong-root mutation would substitute are
**not of that type and not substitutable**:

| candidate | its type | substitutable? |
|---|---|---|
| the abandoned body — the production selector | `StaticOriginId` | it **is** the sole root |
| the owning function | `PredeclaredFunctionId` | **no** — not an origin at all |
| the enclosing selected scope | `RecursorProducerOriginId` | **no** — a different origin type |

⇒ **`body.static_origin` is the sole root reachable at this arm by dataflow**,
so there is no wrong root to pass. A mutation would have to *manufacture* one,
and manufacturing a root is not a perturbation of this seam — it is a different
seam wearing this one's name, and it would red for a reason unrelated to the
property.

**What discharges the concern instead** is the pairing of that sole-root
dataflow with the behavioural package already committed: the `Suppress`
mutation, the exact missing-join A/B, the positive singular-stop advance
assertion, the non-backedge live-body row, and the accounting / no-entry /
no-static-worker-call assertions. Between them, an arm that dispositioned the
wrong subtree could not stay green — the accounting would not close, or the
advance assertion would fail.

⛔ **This discharge is conditional on the interface, and it reopens.** If a
future change gives this arm a second `StaticOriginId`, or any means of
acquiring another root, the wrong-root mutations become constructible and are
**owed again**. They are discharged by the shape of the seam, not by a judgement
that they do not matter.

⛔ **Do not discharge it by other means.** No lookup, query, cast, synthetic
root, walk, or test-only alternate root — each of those *creates* the second
root whose absence is the discharge, which is the one way to make this section
false while appearing to strengthen it.
