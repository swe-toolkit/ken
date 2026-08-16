---
id: RT-NESTED-IH-NATIVE-REALIZATION
title: "Native realization of the nested-IH recursive computation beyond scalar admission -- emitted definition, ABI/owner wiring, and execution that survives the Cranelift verifier and agrees with the interpreter at Nat 3"
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-DYNAMIC-ARM-SCALAR-MERGE, RT-CHECKED-IH-REALIZATION-AUTHORITY]
blocks: [KERNEL-NESTED-IND]
github: null
origin: Steward-filed 2026-08-12 (COORDINATION §2) on runtime-leader's statement of c2's AC-K12 relationship, evt_77pege8j5cv14, requested at evt_6pmftb5fpxrkm. Discharges the second Steward condition on the c1/c2 cut (evt_6z7wf6dw94cym), which required c2 to state that relationship before assignment.
---

> ## D2 MERGED 2026-08-14 as squash `f7ec9f59`. THE NODE STAYS `active`.
>
> **Candidate `20c12c64`** (PR #2187, CI green; Decision `dec_3e9rjnmt9e9z6`
> resolved APPROVED by the Architect, read from the object; QA
> `evt_4bfp301wwz00m`). Declared base `f34ab271`; one commit, two crate paths,
> `+148/-29`; **2/2 blob verified identical after landing.**
>
> **What landed:** the static-constructor `Match` repair — the selected
> one-parameter `LexicalClosure` is **applied** through its retained
> `StaticBody` rather than lowered as a value. `ground_value`, scalar-merge
> admission, and the fail-closed catch-all are unchanged.
>
> **`D2` is TERMINAL at a measured stop** (Architect `evt_3my27jtzxmhh3`). The
> checked-IH realization route at `core.rs:10521` was **measured unreachable**:
> `mint_checked_computational_ih_instance` (`mod.rs:16892-16894`) `.take()`s
> `pending_computational_ih_call` **first** and returns `Ok(None)`. The
> activation is live (`ContinuationActivationId(0)`, live at its resume cursor);
> **the authority was never minted** — plan, call template, slot, and parent are
> all absent. **That is a planner capability, not a missing line.**
>
> **The `D1` sentinel changed CLASS, it was not restated.** `D1`'s assertion —
> *"native lowering declines a real package-backed `Closure` predecessor before
> scalar merge"* — was **falsified by construction** and is **retired
> explicitly**; a **separately named** control now asserts the new stop. Do not
> read the retirement as a regression: the refusal **advanced**, which is the
> evidence the increment worked.
>
> **Successor scope is minting checked realization authority for this seam, and
> the successor now EXISTS: [[RT-CHECKED-IH-REALIZATION-AUTHORITY]]** (`ready`,
> `M`, `depends_on: []`).
>
> **THE SUCCESSOR QUESTION IS ANSWERED — do not re-litigate it.** Architect
> ruling `evt_7mgzv23cnjm0a` (2026-08-14): checked-IH **realization** authority
> and [[RT-TERMINAL-ALL-ELIM-AUTHORITY]]'s **terminal-`All` elimination**
> relation are **TWO mechanisms**. Refuted on three independent grounds — the
> producers never read each other's relation; the other node's own text forbids
> it to build the new machinery this seam needs; and the fold would turn a
> well-founded dependency into a **cycle**, since realization would then sit
> behind the `KERNEL-NESTED-IND` block that this very arc is what unblocks.
>
> ⇒ **The realization successor carries NO `KERNEL-NESTED-IND` dependency and
> was framable immediately. Runtime's nested-IH arc is a live critical path, not
> a parked one.** `RT-TERMINAL-ALL-ELIM-AUTHORITY` stays `draft`, held on
> Kernel, untouched.
>
> **The gating edge is now IN `depends_on`, 2026-08-16 (Steward).** It was
> stated only in this prose, and `scripts/gen-progress.sh` reads `depends_on`
> and nothing else — so every generated view showed this node `active` with its
> single dependency **merged**, i.e. fully unblocked and in progress, while
> [[RT-CHECKED-IH-REALIZATION-AUTHORITY]] carried `blocks: []` and read as a
> startable node with no consumer. **Both halves of one edge were invisible, and
> the direction of the error was the dangerous one: it advertised gated
> deliverables as dispatchable.** Nothing about the scope or the ruling below
> changed; only the graph now says what this paragraph already said.
>
> **`D3`-`D5` of THIS node are gated on that successor landing.** They are
> outcomes — native execution, interpreter agreement at `Nat 3`, the carried
> control no longer ignored — and each requires the authority the successor
> mints. **The cut is capability there, outcomes here.** Do not restate the
> capability as a sixth deliverable on this node.
>
> **The selector wall is untouched by both, and this is the line most likely to
> be crossed by a change that looks like progress:** a
> `ComputationalRecursorClosure` is a suspended activation, not a constructor
> value. **Realize before selection; never accept at selection.**
>
> **Still bound, and the first is the one closest to hand:** `.residual` is not
> inspected or unwrapped (`mod.rs:3327-3331`); no ordinary-`Match` selector arm
> for the capsule and no catch-all change; no unchecked plan minted to get past
> the chain.

> ### THE ABI ORDER IS UNPINNED, AND IT BECOMES LOAD-BEARING AT THE MOMENT THE SUCCESSOR LIFTS THE STOP
>
> **Adversary hunt `evt_54sb0z31q5qhn` on `f7ec9f59`, triaged by the Steward and
> ACCEPTED. Re-verified against `main` `d8de7023` before filing.** This is a
> confirmed defect whose severity is **deferred, not absent**, and it is
> recorded here rather than as a new node because it is an obligation on this
> node's own successor.
>
> **The measurement.** The Adversary transposed the ABI order in the new arm —
> one `inputs.rotate_left(args.len())` before the call, turning `[f0, c0, c1]`
> into `[c0, c1, f0]` on a **non-empty** capture suffix, which is the
> configuration the hazard needs. The arm was **reached twice**
> (`fields=1 captures=2`). Result:
>
> | suite | under the transposition | reached the arm |
> |---|---|---|
> | `ken-elaborator --test nc14_data_match_lowering` | 18 passed, 0 failed | **yes, twice** |
> | `ken-runtime --lib` | 926 passed, 0 failed | no |
> | `ken-cli --test px4b_native_production` | 16 passed, 0 failed | no |
>
> ⇒ **A swapped ABI order compiles a wrong program and nothing in the tree
> notices.** Mutation restored and re-run green.
>
> **Why no execution control can catch it today — and this is the part that
> makes it structural rather than a test gap.** `D2` is terminal at a measured
> stop, so every program that reaches this arm builds `inputs`, **commits the
> order, and then refuses** for absent checked-IH plan authority. **The
> population that exercises the ABI is exactly the population that never
> emits.** There is no executed artifact to be wrong. The only things holding
> the order are the call-position precedent at `core.rs:18186` and the
> `Parameter ++ Capture` statement at `core.rs:18143` — **and the latter says in
> terms that the two runs "are not interchangeable even when their arities
> coincide," which is the hazard stated and not enforced.**
>
> **THE FORWARD CONSEQUENCE, which is the obligation.** When the successor mints
> checked-IH realization authority and the stop lifts, this arm's output becomes
> an **executed program for the first time**, and the ABI order becomes
> load-bearing **at exactly the moment nothing has ever tested it.** Until then
> the failure mode is not "a refusal changes" — it is **"a wrong answer appears
> in the first release that removes the stop."** The two events arrive together
> unless something is written first.
>
> **A structural pin is writable NOW and does not need the authority.** The two
> halves have *different* lengths here (1 and 2), so a **positional** assertion
> discriminates where a length check would not: an in-crate `#[cfg(test)]`
> observation that the first `args.len()` entries of `inputs` are the matched
> fields, in source order. Same shape as the `d2f` / `dasm_c2` observers already
> in this file; **adds no admitted merge shape and touches no boundary.** The
> Adversary did not write it.
>
> **The transferable point, which is why this is filed and not carried:** the
> blocked-by-the-stop argument is correct for an **execution** control and does
> **not** reach a **structural** one. "Nothing executes it yet" is a reason a
> runtime test cannot exist; it is not a reason an assertion about the
> constructed vector cannot exist.
>
> **Also closed by `D2`, recorded so the cheaper repair is the one reused:** the
> `D1` hunt reported a shared helper carrying an `AC-6` sentinel documented at
> only one of its three call sites. **Renaming the helper to
> `assert_nested_checked_pipeline_nat` put the new promise class at all three
> call sites** — the disclosure travels with the name rather than with a comment
> on one caller. That is cheaper than the three comments originally suggested,
> and it is the form to prefer next time.

## D1 MERGED 2026-08-14 as an ACCEPTED PARTIAL. The node stays `active`, and what remains is GATED, not merely unstarted.

**Candidate `ed854c859b0daa58a0664e38ad1526659c8ce0a9`, landed as squash
`2e1f0340`** (PR #2176, CI green; Decision `dec_y75rp1fhq0y8`, Architect, read
`resolved` from the object; QA `evt_27yb8tcs78tx8`). Merge-base `6b3b5b40`
derived independently and matching the declared value; one non-merge commit,
one path, `+105/-23`; **1/1 blob verified identical after landing.**

**`D1` delivered an `AC-6` hard stop, and that is the outcome the frame asked
for.** The real package-backed in-edge `emit_runtime_ir_object_with_cranelift`
**reaches** native lowering and then refuses before `merge_scalar_operand`:

```
Unsupported { construct: "Closure",
              reason: "closures are callable but not observable ground values
                       in native lowering" }
```

The feature-gated scalar-merge observer records **zero arrivals for that exact
attempt**. ⇒ **This is a predecessor decline, not the absence of a native
path** — which is the distinction `AC-6` exists to force, and the two have
different repairs. The committed sentinel reports the declaration set
`{liftAdd, liftSize, liftSizeResult}`, target presence, the refusal, and the
zero count; its refusal-text mutation reddened it and then restored green, so it
is a control rather than a claim. **`AC-5` holds** — both `c2` seat controls
pass and the `c2` result-match region is byte-identical.

**No `AC-K12` claim is made and none is owed here** (`AC-7`).

**Claim scope, and it bounds what `D1` bought.** *"Declined by a predecessor"*
is measured. *"A native path exists"* is **not measured past the `Closure`
arm** — the instrument sees only the **first** refusal, so a second, later
decline is invisible until the first is repaired, and `D2`/`D3` would discover
any further ones **one at a time**. Adversary `evt_2j0h3vgn6gtqv`. Stated
because `AC-6`'s own distinction is that *"never reached"* is two facts, and the
residual half should be written rather than left to the reading.

**Residual, non-blocking, rides the next candidate entering this file.**
`assert_nested_full_pipeline_nat` is called at `nc14_data_match_lowering.rs`
`:316`, `:559` and `:581`, and **only `:316` records that the helper carries an
`AC-6` transition sentinel.** The other two declare a durable-invariant promise
about the interpreter pipeline. **When the boundary moves, all three red, and
two will read as a regression in a fold discriminator.** The helper's
`Ok(artifact)` arm carries the right instruction and will be found; the call
sites' own prose will still say the test is about something else. One sentence
at each of `:559` and `:581`.

### What remains, stated as of 2026-08-14 — READ THE GATE BEFORE PLANNING D2

**`D2`-`D5` are owed. `D2` was gated and is now UNGATED — see the
ruling below**, and prose that reads otherwise would send the next turn
straight into the boundary question this node was cut to keep out.

The implementer identified a repair direction: the static-constructor `Match`
path binds fields into `case_env`, then lowers `LiftNode`'s one-parameter
`LexicalClosure` **as a value**, reaching `ground_value` instead of applying it.

**The Steward did NOT authorize that direction**, because the description is
equally consistent with two situations that have different owners:

| reading | disposition |
|---|---|
| a **defect in the `Match` lowering path** — it should apply the closure and instead grounds it | in scope, no design call, `D2` proceeds |
| a **widening of what native lowering accepts** — the refusal is correct and something upstream should not demand an observable ground value there | moves the fail-closed boundary `c1`/`c2` were cut to establish; **not Runtime's and not the Steward's** |

**These are not distinguishable from the sentence.** Routed to the Architect as
*"which of these is it?"* — deliberately **not** as *"may I apply the
closure?"*, which presumes the first answer.

#### RULED 2026-08-14: reading 1. `D2` IS UNGATED. Recorded here because an in-thread ruling is not a durable artifact.

**Architect, thread `thr_1kvgpwt7b057w`, bounds restated at
`evt_2mvbnxfejmn6p`.** The repair is a **defect in the `Match` lowering path**
and is in scope; it does **not** widen the `c1`/`c2` boundary.

**The ruling rests on the declared type, not on what the boundary admits.**
`liftSize : LiftRose -> Nat` is kernel-checked, **which makes the `Match` path's
closure-valued result wrong independently of what any lowering boundary would
accept.** That entailment is the Architect's and it does not depend on the
in-tree evidence below.

**Supporting evidence, located by the Adversary (`evt_2j0h3vgn6gtqv`) and
verified by the Steward.** `control.rs:11541-11545` records that a closure at
the root is refused while the **called** form
`Call { callee: LexicalClosure }` reaches emission and mints a `StaticBody`
edge — a passing control predating this node. ⇒ *"Any repair would widen the
boundary"* is refuted **positively**, not merely structurally.

**THE BOUND THAT TRAVELS WITH IT — carry this sentence into the repair:**

> **The applied form is admitted; the `D1` applied form is unmeasured.**

The supporting fixture is **degenerate on every axis the `D1` case is
non-degenerate on**: zero `params` against `D1`'s one, zero `args` against
`D1`'s matched field, and a constant `Bool` body against a body that must
produce a `Nat`. **Captures are a third axis and nobody has measured it** — the
`case_env` binding is the mechanism in question, and *"binds fields into
`case_env`"* does not say whether those become closure captures. So the control
establishes that `Call{callee: LexicalClosure}` **is not refused as such**, and
nothing more. It is worth exactly the objection it kills.

**Two places the repair can stop short of green, both of which read as
progress rather than failure:**

1. **`StaticBody` inside a recursive function.** *"Mints a `StaticBody` edge and
   therefore a second function unit"* is not free — every lifted nested match
   becomes its own unit, and in `liftSize` that unit sits **inside a recursive
   function**, where `Lowered::RecursiveBackedge` has its own escape rule at
   `mod.rs:21278-21281`. Whether a `StaticBody` edge composes with the backedge
   machinery is **untested** by a fixture whose body is a constant.
2. **The `StructuralNat` relocation**, flagged separately by the Architect.

### The wall D2 actually reached was a THIRD boundary, and it is CORRECT

**Measured 2026-08-14 by runtime-implementer (`evt_6cd51j51pqvvk`); ruled by the
Architect at `evt_46hx0g4gyj0r`. Folded here by the Steward because it was a
thread-only ruling, and the prohibition below is the part that gets lost.**

The two traps above are **predictions and neither fired.** The bounded `Match`
repair applies the selected one-parameter `LexicalClosure` through its retained
`StaticBody`, and the emitted body then reaches the source-machine ordinary
`Match` selector carrying
`LoweringOperand::Specialized(Lowered::ComputationalRecursorClosure { .. })`,
where the existing catch-all reports `Match: scrutinee is not a constructor
value`. It is neither `RecursiveBackedge` nor `StructuralNat`.

**The ruling: that wall is correct and it is NOT `D2`'s stop.** Both hold at
once. `core.rs:17716-17719` requires `Specialized(Lowered::Constructor { .. })`;
a `ComputationalRecursorClosure` is a **suspended recursor activation**, not a
constructor value, and no widening could lawfully make it one — accepting it
would mean matching on an undriven computation. **Do not add a selector arm for
this variant and do not touch the fail-closed catch-all.**

⇒ **What is wrong is what ARRIVED at the wall**: an unrealized capsule handed to
a selector that requires a realized value. **Realizing it is what this node is
named for** — a wall saying *"this IH was never realized"* is the node's
subject, not its boundary.

**Two landed claims that would have made the wall lawful were checked, and
neither reaches it.** Recorded so they are not re-cited as a boundary:

| claim | its actual scope | why it does not reach |
|---|---|---|
| `mod.rs:11784` — `ComputationalRecursorClosure => FailClosedForbidden`, *"not a transferable value"* | **transfer across a unit boundary** | says nothing about consuming the capsule **in place**, which the tree already does lawfully; read at its widest it would forbid `core.rs:10521`, which is landed and green |
| `control.rs:8544` — AC-6, *"out of the covered population ... declares no body carrier"* | **re-lowering from a source body** | driving a capsule to a value does not re-lower it from a source term; the pin fires only if the variant acquires a body-carrying field |

**The next bounded step, and it is a measurement rather than an
authorization.** Determine whether the matched-field application can route its
result through the **existing** in-place checked-IH realization at
`core.rs:10521` (`mint_checked_computational_ih_instance` →
`OrientedSubcontinuationPlanV1` / `computational_ih_call`) **before** ordinary-
`Match` selection. Three further in-place sites treat the variant as a callee
(`core.rs:4031`, `:4271`, `:18294`). **The Architect read this route, did not
run it, and explicitly declined to assert it is reachable from the seam.** If
the seam has no activation, no oriented plan, or no call template, **that
absence is the real `D2` stop and is the finding worth reporting.**

**FORBIDDEN, and this one is a trap one step away from the obvious fix: do not
reach into `.residual`.** `mod.rs:3327-3331` is explicit that the admission walk
rejects the capsule *before* inspecting or emitting its residual, and that *"a
carried residual must not become a way to reach the carrier through a capsule
that is otherwise refused."* Unwrapping `residual` to obtain something matchable
is **the exact evasion that ordering ruling exists to prevent**, and it is the
most available-looking repair at this wall.

**The bounded `Match` repair stands and must not be reverted.** The original
`Closure` refusal disappearing — with `ground_value`, scalar-merge admission,
the `D1` declaration sentinel, and the interpreter `Nat` 3 result all unchanged,
and scalar-merge arrivals still zero — is the `D1` ruling confirmed by
construction. **The refusal ADVANCED rather than recurring, which is the
evidence the increment worked.**

**`Forbidden` is unchanged and still binds** — the ruling authorizes repairing
the `Match` path, **not** a seventh admitted merge shape.

**Runtime's own retro reached the same first proof independently:** the first
new proof must show the case closure **consumes the matched field** and that
`ground_value` **remains unchanged**; only then may native definition,
call-site replacement, and `D2`/`D3` evidence proceed.


## Why this node exists

`KERNEL-NESTED-IND` is `active` on **one** open criterion, `AC-K12`. Its four
other late deliverables are merged. `AC-K12` requires a nested-inductive
recursive computation to **form a valid native artifact, pass Cranelift
verification, execute natively, and agree with the interpreter at Nat 3**, with
its carried control no longer ignored.

`RT-DYNAMIC-ARM-SCALAR-MERGE` slice `c2` was the candidate for that. It is not
sufficient, and runtime-leader stated the gap when asked (`evt_77pege8j5cv14`):

> **`c2` partially advances `AC-K12`; it does not discharge it.** It clears the
> real `D5` scalar-merge refusal by proving the arriving operand is
> `StructuralNat` while retaining the unrelated-`Data`, merge-shape, and
> catch-all boundaries. **The missing capability is native realization of the
> full nested-IH continuation/recursive computation beyond scalar admission:**
> its emitted definition, ABI/owner wiring and runtime execution must survive
> the verifier and reach the same result as the interpreter.

⇒ **`c2` clears a refusal; this node supplies an execution.** Those are
different capabilities and only the second satisfies `AC-K12`.

## Why it is a node rather than a third slice of `RT-DYNAMIC-ARM-SCALAR-MERGE`

The same reason `c1` and `c2` were cut apart in the first place: `c1` is a
fail-closed contract, `c2` is semantic admission, and **combining an unbounded
question with a bounded one is what made slice `a` too wide.** Native
realization is the unbounded one. Folding it into `c2` would reproduce exactly
the defect the `c1`/`c2` cut repaired.

The constraint is grounded, per `steward.md §4c`: `AC-K12` is a written
criterion on a live node, the gap is a **measured capability gap** stated by the
owning team, and the node is on the graph because three Kernel seats and, behind
them, `DS-9` are waiting on it. It is not a tidier-graph preference.

## The edge this node also repairs

**`KERNEL-NESTED-IND` declared no dependency on Runtime at all.** All five of
its `depends_on` entries are `merged` or `closed`, so `gen-progress.sh` showed it
**active with no blockers** while it was in fact blocked on
`RT-DYNAMIC-ARM-SCALAR-MERGE` — a relationship that existed only in prose inside
the node body.

**It failed in the direction that hides an idle team.** A reader of the tracker,
including the operator, saw a Kernel node active and unblocked and would conclude
the ring was progressing. Both edges are now declared, in `depends_on`, which is
the side `gen-progress.sh` reads.

This is a **different defect class** from the one-way `blocks:` edge fixed in PR
#1951 and is invisible to the sweep that found it: there the edge existed on one
side, here on neither. A prose-mention sweep across every live node found no
other instance — `DS-9`/`RT-SCALE-B`-style mentions are documented deliberate
non-edges, not omissions.

## What makes this `draft`, and exactly what flips it to `ready`

**Not framing debt, and not a lane request.** Runtime's lane is on the
`RecursiveDescent` retirement chain (`RT-LEXICAL-ROW2-MISSING-MINT` →
`RT-RECURSOR-TRANSPORT` → `RT-DESCENT-RETIRE`), then `c1`, then `c2`. That
ordering is the operator's standing priority and is not in question here.

**This node cannot be written shovel-ready before `c2` lands**, because `c2`
defines its input: which operand shapes arrive at native lowering already
admitted, and which refusals `c2` deliberately retained. Framing the ACs against
a guessed admission surface would produce controls that measure the wrong
boundary.

> **Flip condition, stated so `draft` is checkable rather than a holding
> pattern:** the Steward frames this node and flips it `ready` **when `c2`
> merges**, using `c2`'s landed admission surface as the frame's fixed input.
> There is no other gate on it — not the retirement chain, not a decision, not
> the operator.

> **`c2` IS IN FLIGHT AS OF 2026-08-14.** `RT-DYNAMIC-ARM-SCALAR-MERGE` was
> kicked to Runtime at `main` `3ff6cd6e` (anchor `evt_52b0n0y09p379`), scoped
> `c2-pre` then `c2`. `c1` merged long since at `7bfc8ae5`, and the retirement
> chain named above is behind it — `RT-MATCH-RECURSOR-CONSUMERS` and
> `RT-LEXICAL-R3-FUSION-EMITTER` are both `merged`. **The flip condition is now
> one merge away, and it is mine to execute the moment `c2` lands.**

## What the frame must carry when written

Recorded now while the reasoning is fresh, so the framing turn does not
re-derive it:

- **Four stages, and they are separate observations.** Emitted definition,
  verifier acceptance, native execution, interpreter agreement at Nat 3. A
  control that collapses any two of them cannot say which failed.
- **Interpreter agreement is a differential, not a self-check.** The oracle is
  `ken-interp`'s result for the same computation; "the native run produced 3" is
  not the criterion.
- **The carried control must no longer be ignored** — that is part of `AC-K12`'s
  own wording and is the thing most likely to be satisfied vacuously.
- **`AC-K12` is not claimed or advanced by `c1` or `c2`.** Whatever this node
  discharges, the criterion belongs to `KERNEL-NESTED-IND` and closing it is
  Kernel's, on Runtime's delivered capability.

# FRAMED AND FLIPPED `ready` — 2026-08-14, on `c2` merging as `57bf1721`

**The flip condition above is met and this is the framing turn it called for.**
`c1` merged at `7bfc8ae5`, `c2` at `57bf1721`. The `c3` closure slice on
`RT-DYNAMIC-ARM-SCALAR-MERGE` is **not** a gate on this node — it is gating and
placement on a diagnostic feature and touches no admission arm.

## Fixed inputs, measured at `main` `d578a894`

**The frame's whole point is that these are read off `c2`'s landed surface
rather than guessed, which is what `draft` was protecting against.**

**1. The existing harness ends exactly one step short of `AC-K12`.**
`crates/ken-elaborator/tests/nc14_data_match_lowering.rs:201-217`,
`assert_nested_full_pipeline_nat`, runs elaborate → kernel-check → erase →
**interpreter**, and stops:

```rust
let program = nested_checked_runtime_program_for_source(package_name, target_name, source);
// ... asserts the target declaration is present in the runtime program ...
assert_eq!(interpreter_nat_for_source(source, target_name), expected);
```

⇒ **`program` is the erased `RuntimeProgram` already in hand, and
`interpreter_nat_for_source` is already the differential oracle, in the same
function.** The four stages this node owes are the continuation of that
function, not a new pipeline. `nested_recursive_bag_rose_elaborates_checks_
erases_and_interprets_at_nat_three` (`:245`) is the Nat-3 caller.

**2. What `c2` admits, and what it deliberately still refuses.** At
`merge_scalar_operand` the `match lowered` arm set is **unchanged by `c2`** —
six admitted merge shapes plus the fail-closed `_ =>`. On the real `D5`
package path the operand arrives as `StructuralNat` with no surviving
constructor identity and is admitted; an independently-named Peano-shaped user
`Data` reaches the same seat, stays exact `Constructor`, and is refused
(`nc14_data_match_lowering.rs:312`, `:420`).

⇒ **This node inherits a seat that admits, and must not widen it.** If native
realization appears to need a seventh admitted shape, that is a finding and a
question for the Architect, not a local edit — see `Forbidden`.

**3. There is a reusable seat instrument, and it is feature-gated.**
`DasmC2ScalarMergeObservation { construct, operand_kind, constructor, admitted }`
via `dasm_c2_scalar_merge_observation_scope()`
(`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:16040`). **Note the
gate is `cfg(feature = "dasm-c2-observation")` alone, not `cfg(any(test, …))`**,
so reaching it from `ken-elaborator` depends on the dev-dependency feature that
`c3` may change. **Do not build a second instrument** — if `c3` has moved the
gate, use the shape `c3` left.

## Deliverables — four stages, and they stay four observations

The node body already fixes this: *"a control that collapses any two of them
cannot say which failed."* Each `D` below produces its own reported evidence.

**`D1` — emitted definition.** The nested-IH continuation reaches native
emission as a definition in the artifact. Report the emitted declaration set and
show the target present in it, the way `assert_nested_full_pipeline_nat` already
does for the runtime program.

**`D2` — verifier acceptance.** The emitted artifact passes Cranelift
verification. **Report the verifier's own verdict**, not the absence of a panic
downstream of it.

**`D3` — native execution.** The artifact executes and produces a value.

**`D4` — interpreter agreement at Nat 3.** The native result equals
`interpreter_nat_for_source(NESTED_LIFT_NAT_THREE_SOURCE, …)`, computed in the
same run.

**`D5` — the carried control is no longer ignored.** `AC-K12`'s own wording
carries this and the node body already names it as *"the thing most likely to be
satisfied vacuously."* **Read `KERNEL-NESTED-IND`'s `AC-K12` text for which
control it means before writing this** — do not infer it from the name.

## Acceptance criteria

| AC | criterion | control |
|---|---|---|
| `AC-1` | `D1`-`D4` are reported as **four separate** observations | a single green end-to-end assertion does not discharge any of them. Name which stage produced each piece of evidence |
| `AC-2` | `D4` is a **differential**, not a self-check | the oracle is `ken-interp`'s value for the same source in the same run. *"The native run produced 3"* fails this row — 3 is also what a stub returns |
| `AC-3` | A **negative** control shows the differential would see a disagreement | perturb the native side (or the expected value) and report the differential reddening. `AC-2` passes vacuously if the two sides are never actually compared |
| `AC-4` | `D5`'s carried control is exercised, with its failing text reported | an assertion that cannot fail is the shape this arc keeps finding; show it firing under a mutation |
| `AC-5` | The `c2` admission surface is **unchanged** | the six admitted merge shapes and the fail-closed `_ =>` are byte-for-behaviour as `c2` left them, and the two `c2` seat controls still pass. Report both test names |
| `AC-6` | If a stage cannot be reached, that is a **measured finding with a route**, not a silent omission | name which in-edge was instrumented and what it showed. *"Never reached"* is two different facts — declined by a predecessor, or no such path exists — and only instrumenting the route tells them apart |
| `AC-7` | No `AC-K12` discharge claim | this node delivers a capability; closing `AC-K12` is Kernel's, on `KERNEL-NESTED-IND` |
| `AC-8` | No-regression, in CI | `COORDINATION §12` — the venue is CI, never a local `--workspace` run |

## Sizing and the hard stop

**`L`, and it is the unbounded question the `c1`/`c2` cut deliberately kept
out.** Per `steward.md §4b` the target is a releasable increment **or a genuine
hard stop** within about an hour. **A hard stop here is a good outcome and is
what `AC-6` is for:** if `D2` refuses, report the verifier's verdict and stop —
do not attempt `D3` or widen the seat to get past it.

**The natural first increment is `D1` plus `AC-5`**, because it extends an
existing function and re-runs two existing controls. If `D1` alone consumes the
turn, release it.

## Forbidden

- **No new admitted merge shape and no change to the `_ =>` catch-all.** If
  native realization appears to require one, that is an Architect question. The
  fail-closed boundary is what `c1` and `c2` were cut to establish.
- **No second observation instrument** — see fixed input 3.
- **No `AC-K12` claim**, and no advancing-it claim.
- **No change to the interpreter side of the differential.** Moving the oracle
  to make the two sides agree is the failure `AC-2` exists to catch.
