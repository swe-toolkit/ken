# RT-COMPMATCH-TREE-SCRUTINEE — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`831e521e5187e28cdd7c24668bbafbaa0eec3e8d`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."* Ledger row 14, and the last row in the
`RT-IGNORED-FAILING-ROWS-INVENTORY` debt to be framed.

> **Every coordinate, count, line number and status in this frame is PERISHABLE
> and was measured at the base SHA above.** Re-ground each one against
> `origin/main` before you act on it — including the prose. A frame is not a
> status source and a line number is not a binding. If a re-measurement
> disagrees with this document, the tree wins and the disagreement is a finding
> worth reporting, not a discrepancy to reconcile silently.

## 0. THIS FRAME DOES NOT RESTATE THE NODE. IT AMENDS IT IN TWO PLACES.

`docs/program/issues/RT-COMPMATCH-TREE-SCRUTINEE.md` is unusually complete: it
carries `D0`-`D4`, `AC-0`-`AC-8`, a contention section and a what-must-not-happen
list. **All of it is binding as written except the two amendments below**, which
are stated here because they are measurements the node was filed without.

    AMENDED   the node's §`D1` "THE BEST GUESS" paragraph
              -> it names two roles that DO NOT EXIST. See §3.
    AMENDED   the node's `AC-4`
              -> "which roles are now accepted" is the wrong question for the
                 repair this frame's measurement points at. See §5.

Everything else in the node stands. Read it first; this frame is the fixed
inputs and the design judgment, not a replacement.

## 1. Objective

Unchanged from the node. Force past the first stop **once**, record whether the
`ComputationalMatch` scrutinee refusal is still behind it, and repair the first
stop if the repair is a lookup rather than a relaxation.

## 2. Fixed inputs, measured at `831e521e5`

### The row

    crates/ken-cli/tests/rt_span_prov_native.rs
      :309-328   the stale comment block  ("Observed signature, exactly")
      :330       the #[ignore] attribute  <- the row coordinate
      :331       fn sp_a_foreign_span_freeze_rejects_own_span_succeeds_
                    on_both_engines

**The file carries exactly ONE `#[ignore]`.** That makes `AC-5`'s grep a clean
instrument here: any live hit for `tree-producing match scrutinee` in this file
outside a block labelled superseded is this row's, with no second row to
confuse it.

> ### THE TOP-LEVEL FAILURE OUTPUT CARRIES NO SIGNATURE. THE ANNOTATION SAYS SO.
>
> Fixed input, from the row's own comment block at `:323-327`:
>
> > *"The refusal surfaces on the helper thread `sp-a-freeze`; this test thread
> > then fails only with the wrapper `called Result::unwrap() on an Err value:
> > Any { .. }` which carries no signature of its own."*
>
> ⇒ **Un-ignoring the row and reading what the test harness prints gives you
> `Any { .. }`, which is compatible with every refusal in the planner.** `AC-0`
> requires the verbatim first refusal, and the obvious way to obtain it does
> not produce it. **Capture the helper thread's output** — that is where the
> signature is. A run that reports only the wrapper has not discharged `AC-0`,
> and reporting the wrapper as the signature is the failure this row's
> annotation was written to prevent.

### The refusal site

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      aggregates.rs                                       13007 lines
      :226    mod required_consumer_destination
      :243    fn pair_detached_required_consumer
      :253-268  the reverse find_map and its ok_or_else
      :266    "an exact detached required consumer has no computational
               occurrence"

The body, as it stands:

```rust
let consumer_origin = projection
    .steps()
    .iter()
    .rev()
    .find_map(|step| {
        matches!(step.role(), SourceReturnContextRole::ComputationalMatchCase(_))
            .then_some(step.parent_origin())
    })
    .ok_or_else(|| planner_error(
        "an exact detached required consumer has no computational occurrence",
    ))?;
```

### The role enum is CLOSED, and closed ON PURPOSE

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      continuations.rs:1203-1210

    enum SourceReturnContextRole {
        CheckedBody,
        LetBody,
        IfThen,
        IfElse,
        MatchCase(u32),
        ComputationalMatchCase(u32),
    }

Its doc comment at `:1200-1201` states the reason, and it is a reason against
widening:

> *"The role is closed because lowering must be able to reject a template whose
> recorded source child no longer has the semantics under which it was issued."*

**Six variants, and `MatchCase` versus `ComputationalMatchCase` is the enum's
deliberate distinction** — the one the search is keyed on.

### The projection carries THREE places a consumer can live; the search reads ONE

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      continuations.rs:1263-1270

    struct SourceReturnContextTemplate {
        root_origin:    StaticOriginId,
        result_origin:  StaticOriginId,
        steps:          Vec<SourceReturnContextStep>,      <- searched
        caller_suffix:  Vec<StaticOriginId>,               <- NOT searched
        worker_return:  Option<Box<SourceWorkerReturnBoundary>>,  <- NOT
                                                                     searched
    }

`caller_suffix`'s own doc comment, `:1267-1268`:

> *"Computational consumers already pending outside this result root, in exact
> inner-to-outer return order."*

**That field is, by its own documentation, a list of computational consumers.**
`worker_return`'s `caller_context()` is a further whole
`SourceReturnContextTemplate` (`:1244`, `:1252`).

⇒ **The refusal's message is a claim about the projection; the instrument is a
claim about `steps`.** *"Has no computational occurrence"* is asserted on the
strength of a search that never looks at the two other fields where one could
be. Whether that gap is reached on this row is `D1`'s to measure — but the gap
is in the code at this base, independent of the row.

### THE WORKED COMPARATOR ALREADY EXISTS IN THE TREE

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/
      responses.rs:4491   fn detached_post_call_consumer_frames
        :4496   for step in context.steps().iter().rev()
        :4497   if matches!(step.role(), ComputationalMatchCase(_))
        :4501       frames.push(step.parent_origin())
        :4504   if let Some(boundary) = context.worker_return() {
        :4521       frames.push(target.key.continuation_origin)
        :4522-4531  ... plus consuming_occurrence.eliminator_origin() and
                    RequiredConsumerProjection::DirectOuter's
                    required.eliminator_origin()

**Same type, same predicate, same direction, same "detached post-call consumer"
subject — and it does not stop at `steps`.** After exhausting the steps it
descends into the worker-return boundary and collects further consumer origins
from it.

`responses.rs:4236-4238` likewise consumes `caller_suffix()` for its own
detached-frame accounting.

⇒ **Two sites in the same module treat this template as having consumers beyond
`steps`. `pair_detached_required_consumer` treats it as not having them, and
refuses.** This is the single most load-bearing input in this frame.

### Build discipline (`COORDINATION §12`)

Targeted only, through `scripts/ken-cargo`, scoped `-p ken-runtime` or `--test
rt_span_prov_native`. **Never `--workspace`** — the full build, the `--locked`
gate and the conformance suite run in CI, not on this box. "No-regression"
means green in CI.

## 3. AMENDMENT ONE — THE NODE'S BEST GUESS NAMES TWO ROLES THAT DO NOT EXIST

The node's `D1` reads:

> *"if the SP-A program's nested `ResourceBracketResult` matches produce steps
> under a **bracket or constructor role** rather than a match-case role, the
> reverse-find is asking for the wrong role and the value is right there under
> a different name."*

**There is no bracket role and no constructor role.** The enum is closed at six
variants (§2) and neither is among them. **The guess is refuted by enumeration,
before anyone runs anything** — which is the cheapest possible refutation and
the reason it belongs in the frame rather than in a handback.

**Fork `(i)` survives; only its stated mechanism does not.** The live reading is
not *"a different role"* — it is **the same role, in a place the search does not
look:**

    (i-a)  the computational consumer is in `caller_suffix`, which is
           documented as holding exactly that, and which `steps` does not
           cover.
    (i-b)  it is behind `worker_return.caller_context()`, which is where
           `detached_post_call_consumer_frames` finds further consumers for
           the same subject.
    (ii)   the projection genuinely carries none in ANY of the three fields
           => the defect is upstream, in whatever routed this shape onto the
              detached-required-consumer path. The node's `(ii)` unchanged.

**`(i-a)` and `(i-b)` are not a drop-in widening and must not be written as
one.** `caller_suffix` is a `Vec<StaticOriginId>` — bare origins with no role
on them — so "is there a computational consumer in `caller_suffix`" is a
different lookup, not a wider `matches!`. Say which question you asked of it.

> **THE ROLE-SET WIDENING IS THE REPAIR TO BE SUSPICIOUS OF, NOT THE ONE TO
> REACH FOR.** The only unused role that could plausibly be added is
> `MatchCase`, and `MatchCase` is precisely the non-computational one. Adding
> it makes the refusal unreachable and satisfies a green run by removing the
> distinction the enum exists to carry. If the measurement genuinely points
> there, that is a `D4` hard stop and routes to the Architect — not a repair.

## 4. Deliverables

**`D0`, `D1`, `D2`, `D3` and `D4` are the node's, unchanged**, with these
additions:

**`D0` — add the thread-capture requirement.** See the fixed input above. The
verbatim first refusal comes off the `sp-a-freeze` helper thread; the wrapper
`Any { .. }` is not it.

**`D1` — enumerate all THREE fields, not just the steps' roles.** The node asks
for the roles the template carries. Extend it: report, for this row's
projection,

    steps          count, and the role of each
    caller_suffix  count, and what each origin denotes
    worker_return  present or absent; if present, recurse once into its
                   caller_context and report the same three

**This terminates** — it is one program's projection, the fields are finite, and
the recursion is bounded at one level by this deliverable.

**`D2` and `D3` unchanged.** `D2` corrects both stale annotation records in the
same change; `D3` returns the descent-campaign witness verdict in its own
words. Confirmed at this base: `docs/program/16-recursive-descent-retirement.md`
entry `#6g` still reads **WITHDRAWN 2026-09-18, not refuted**, and still carries
the Architect's scoping (`evt_1efjy1jmsg0ry`) that the census ban does not cover
re-reading this one row's current refusal.

## 5. Acceptance criteria

**`AC-0` through `AC-8` are the node's and stand**, with one replacement and
two additions.

**`AC-4` REPLACED.** The node's `AC-4` requires, if `(i)`: *"a statement of
which roles are now accepted and why each denotes a computational consumer."*
Under §3 that question presumes the wrong repair. It becomes:

    AC-4'  If (i): the handback names WHICH FIELD the consumer was found in,
           states the lookup used to find it there, and states why that
           lookup denotes a computational consumer for this projection.
           If the repair adds a ROLE to the matched set, AC-4' is NOT
           satisfied and D4 applies instead.

**`AC-9` — the comparator is addressed either way.**
`detached_post_call_consumer_frames` handles the same subject and does not stop
at `steps`. The handback states whether the two functions **should** agree —
and if the answer is no, why the same template means different things to them.
**A repair that leaves them silently divergent, or a `(ii)` that does not
explain why the comparator looks further, fails this AC.**

**`AC-10` — the verbatim refusal is sourced.** `AC-0`'s pasted signature is
accompanied by a statement of where it was captured from. *"The test output"*
does not satisfy it; the wrapper is the test output.

## 6. The hard stop

**The node's `D4`, unchanged and restated because §3 sharpens what trips it.**
`pair_detached_required_consumer` is a planner invariant whose refusal says
*"please report this compiler bug."* Under `(i)` you are teaching it to see a
consumer that is there. **Under a role-set widening, or under any change that
lets the planner proceed with no computational occurrence at all, you are
teaching it to run without one — stop and route to the Architect.**

## 7. Contention — measured, not inherited

**Clean at this base.**

    planning/static_transition/aggregates.rs   13007 lines
      :243-278    this node's function
      :3358-3618  [[RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT]]'s region
                  (status: ready, runtime) -- the host-result recipe

**Same file, disjoint by roughly three thousand lines and by function.** The
node asserted "different function"; this is that claim measured. Expect a
`merge-tree` union rather than a conflict, and do not sequence behind it.

**Scanned at this base: no `wp/` or `steward/` branch has a hunk in
`static_transition/aggregates.rs`.** `RT-CARRIER-PRODUCER-OCCURRENCE` does not
touch it. `[[RT-PROCESS-EXIT-STATUS]]`, the sibling row framed the same day,
is in `boundary.rs` and `surface.rs`.

`D2` writes `crates/ken-cli/tests/rt_span_prov_native.rs`, which nothing else
in flight touches.

## 8. What must not happen — additions to the node's list

The node's list stands. Three more, all from this frame's measurements:

- **Do not report the wrapper as the signature.** `Any { .. }` carries none.
- **Do not widen the matched role set to make the refusal go away.** §3, and
  `D4`.
- **Do not "fix" `pair_detached_required_consumer` by copying
  `detached_post_call_consumer_frames` wholesale.** It collects a frame *list*
  for a different purpose; this function needs one consumer origin and pairs it
  with `checked_frame_for_consumer`. It is a comparator for **where to look**,
  not a body to clone.

## WRITE-BACK — `D0`, `D1`, `D2` landed 2026-09-18 and were never recorded here

**Written back at `c67a33fda46f93e7bfb89e2fc3d9a3d127679dbd`.** This frame's
deliverables above were discharged in `crates/` by `03976d2ac` (verified an
ancestor of `origin/main`), which touched
`crates/ken-cli/tests/rt_span_prov_native.rs` and
`.../static_transition/aggregates.rs` and **did not touch this frame or the
node.** Read the node's write-back section for the full record; the operative
facts for anyone picking this frame up:

- **`D0` returned outcome (a).** The first refusal cleared and a **third
  layer, `StaticResponseDeferred`, was found behind it.**
- **`D1`/`D2` are discharged** by that same commit, which found the consumer
  behind `worker_return` and corrected the annotation.
- **Re-verified at current `main`:** row 14 is still `#[ignore]`d and its
  label has been retitled to name the third layer.
- **`D3`: witness RETIRED — RELAYED, NOT RE-DERIVED.** A claim from the
  campaign thread, recorded because `D3` asks for it, marked because I did
  not verify it against an artifact.

**THIS FRAME'S FORCING STEP HAS ALREADY FIRED.** Its `D0` is described as *"a
single forcing step with a named target and a binary outcome, NOT a peeling
ladder"* — that is a true description of work **already done**, not of work
available. Anyone selecting this node for that property is selecting a
property that was consumed on 2026-09-18.

**No row is cleared.** Row 14 remains ignored, now stopping at the third
layer. **The third layer is neither investigated nor sized here** — sizing it
against the superseded analysis would size the wrong thing.

### The predicate this node is a worked example of

    Before starting any `ready` node:
        git log --oneline origin/main --grep=<NODE-ID>
    Commits the node file does not record ⇒ the node is stale and the
    write-back precedes the work.

**`status: ready` is a claim about a node, not evidence about the tree.** Here
the tree had moved three deliverables ahead of the file, and every reader
since would have re-derived landed work.
