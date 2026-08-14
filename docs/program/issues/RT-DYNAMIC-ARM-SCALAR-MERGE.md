---
id: RT-DYNAMIC-ARM-SCALAR-MERGE
title: "A carried Match arm carrying a nested-IH result cannot satisfy merge_scalar_operand -- measure what the arm actually produces before bounding the repair"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [KERNEL-NESTED-IND, RT-NESTED-IH-NATIVE-REALIZATION]
github: null
origin: Measured by KERNEL-NESTED-IND D5 at WIP 51c482a5 (evt_3evnpax25tckf, 2026-08-09). Kernel reached the native boundary after interpreter Nat-3 and provenance-gated erasure both passed, and stopped without Runtime edits exactly as the durable D5 ruling at main 46c12adb requires. Steward-filed (agents cannot create tracked work per COORDINATION §2). Steward owns the frame and AC/control placement.
---

> # `c2` IS AUTHORIZED — Steward, 2026-08-12. THE GATE WAS LEFT SHUT AFTER ITS
> # CONDITION WAS MET.
>
> **The whole node is now shovel-ready: `c2-pre`, then `c2`.** No further
> Steward authorization is owed on this node, and nothing here re-ranks it.
>
> **The gate said `c2` "returns to the Steward before assignment with its
> `AC-K12` relationship stated" (`evt_6z7wf6dw94cym`). That statement arrived at
> `evt_77pege8j5cv14`** — runtime-leader's answer that `c2` *partially advances
> `AC-K12` and does not discharge it*, because it clears a refusal where
> `AC-K12` needs an execution. **I acted on it the same day** by filing
> [[RT-NESTED-IH-NATIVE-REALIZATION]] to own the missing capability, and then
> did not come back and open this gate. The condition was discharged and the
> durable record still read `unauthorized`.
>
> **This is the failure direction that hides an idle ring, for the third time on
> this one chain**, and each time by a different mechanism: a missing
> `depends_on` edge (recorded on `RT-NESTED-IH-NATIVE-REALIZATION`), a stale
> `active` status (the block below), and now a **satisfied precondition whose
> gate nobody reopened**. All three read to a tracker reader as *work in
> progress*. **A precondition is discharged by the artifact that records it,
> never by the act that satisfied it** — the seat that supplies the answer has
> no way to open a gate it does not own, so the owner has to close the loop
> explicitly or it stays shut silently.
>
> ## What `c2` may and may not claim
>
> - **May:** the semantic admission itself — proving the arriving operand is
>   `StructuralNat` and clearing the real `D5` scalar-merge refusal, while
>   retaining the unrelated-`Data`, merge-shape, and catch-all boundaries.
> - **May NOT:** claim, advance, or partially credit **`AC-K12`**. `c2` clears a
>   refusal; `AC-K12` requires an execution that forms a valid native artifact,
>   passes Cranelift verification, runs, and agrees with the interpreter at Nat
>   3. Those are different capabilities and
>   [[RT-NESTED-IH-NATIVE-REALIZATION]] owns the second.
> - **Sequencing is unchanged:** `c2-pre` first — it corrects a comment `c2`'s
>   own author would otherwise read as licence — then `c2`.
> - **Standing hard-stop instruction stands:** if the admission validator alone
>   consumes a turn, stop with a measured population rather than pushing into
>   the consumer migrations.
>
> **This is not a lane request, a re-ranking, or a pre-emption.** Runtime is on
> `RT-LEXICAL-R3-FUSION-EMITTER` (`DP-1`, then `DP-2`), Runtime runs one node at
> a time, and Runtime's ordering behind the `RecursiveDescent` retirement chain
> is the operator's standing priority. Authorizing `c2` only means that when
> Runtime next frees, nothing on this node waits on me.

> # STATUS `active` -> `ready`, 2026-08-12. NOTHING WAS UN-LANDED.
>
> **`c1` merged at `7bfc8ae5` and stays merged.** This is a tracker correction,
> not a rollback. The node carried `active` while **no seat held it** — the
> Runtime ring moved to `RT-LEXICAL-RECURSOR-CONSUMERS` (`D2k`) at
> `evt_9tx4kt0k8epm` and has been there since. `depends_on` is empty, the frame
> below is shovel-ready, and the remaining slices (`c2-pre`, `c2`) are
> unassigned. Per the tracker legend that is **`ready`**: deps met, unassigned.
>
> **The lie was in the direction that hides an idle ring.** `KERNEL-NESTED-IND`
> is blocked on this node and reads *"blocked by `RT-DYNAMIC-ARM-SCALAR-MERGE`
> (status: active)"*, which tells a tracker reader — including the operator —
> that Kernel's unblock is in progress. It is not; it is on the shelf. Kernel's
> three seats and, behind them, `DS-9` are waiting on a node nobody is building.
> This is the same failure direction [[RT-NESTED-IH-NATIVE-REALIZATION]] records
> for the undeclared edge, one node over and reached by a different route: there
> the edge was missing, here the edge is declared and its endpoint's status is
> wrong.
>
> **Not a lane request and not a re-ranking.** Runtime's ordering behind the
> `RecursiveDescent` retirement chain is the operator's standing priority and is
> unchanged. Making the shelf visible is what lets that priority be re-examined
> on real state; it does not pre-empt it.

> # `c1` SCOPE AMENDED IN PLACE 2026-08-10 ~08:3xZ — Steward `evt_28pvpc4rpyvyx`
>
> **Candidate `dfea0f38` was REJECTED by the Architect (REQUEST CHANGES in
> thread `thr_1wn4ydb4kjqxt`, 2026-08-10 08:13Z, on Decision
> `dec_6qgvd1v626s62`) and every verdict bound to that SHA is spent.** The
> production boundary was right; the *consumer* edge was not closed.
>
> **The rejection's finding was incomplete, and the implementer's sweep is the
> durable part.** The Architect named 2 cases; the real population is **5 cases
> in 3 files** — `ken-interp/tests/nc7_differential_trust_report.rs` (2),
> `ken-interp/tests/nc9_proof_erasure_boundary_checker.rs` (1), and
> `ken-elaborator/src/erasure.rs` px7l (2). `ken-cli/tests/px4b_native_production.rs`
> is green; its uses are compile-probe strings, not calls.
>
> **Both offered repairs were blocked by causes that predate `c1`:**
>
> - *Rebuild from a real erased checked package* clears the authority gate and
>   then dies on `reject_program_blockers`, which refuses any non-empty
>   `assumptions` map — and **every** driver-compiled package carries prelude
>   trusted-base assumptions.
> - *Relocate into `ken-runtime` unit tests* cannot preserve NC7's claim:
>   `ken-runtime` cannot reach `ken-interp` without a dependency cycle, so it
>   would **delete** the oracle-provenance claim rather than move it.
>
> ⇒ **Nothing had ever run a real package through the native lane.** It had only
> ever seen synthetic programs, which is precisely why this stayed invisible
> until `c1` made absence fatal.
>
> ## What is now authorized, and why it is a fold rather than a new node
>
> Architect ruling `evt_7gwz3dnthfxyh` supplies a **bounded native-subset
> admission**: one private `NativeProgramAdmission` validating only closed,
> hash-covered, compiler-origin, supported-primitive trust tuples, run after the
> `c1` role authority and before the existing blockers, with admitted trust
> propagated honestly into `CraneliftRunReport.trust.assumptions` and
> `CraneliftObjectArtifact.assumptions`.
>
> **This does NOT grow the TCB, which is the axis that would have made it the
> operator's call.** `spec/60-security/64-trust-model.md §4.3` already holds the
> native runtime and `foreign` postulates as trust assumptions, "minimised and
> listed, but not proven". The ruled tuple requires the target to already sit in
> `metadata.trusted_base_delta`, so an admitted assumption is one **already
> declared** in `trusted_base()`; the propagation requirement is §4.3's "listed"
> discipline made mechanical. The trusted base is unchanged — the native lane
> merely stops refusing to *execute* programs whose trust was already listed.
>
> **Folded, not cut as a new node**, because the admission is the only remaining
> discharge path for `c1`'s own consumer edge, not scope pursued on its merits.
> The set of WPs is unchanged, so `thr_1wn4ydb4kjqxt` remains the anchor.
>
> **Lands as one unit.** Slicing the mechanism out first would put five red
> cross-crate tests on `main` — a mechanical publisher blocker for the whole
> fleet, not the "a working path would go red" intuition the no-users ruling
> retired. Semantic atomicity is met.
>
> **Size revised TBD → `M`**; expected to exceed one turn. Standing instruction
> to the ring: if the admission validator alone consumes a turn, hard-stop with a
> measured population rather than pushing into the consumer migrations.
>
> ~~**`c2` remains unauthorized** and returns to the Steward before assignment
> with its `AC-K12` relationship stated.~~ **`c2` IS AUTHORIZED — Steward,
> 2026-08-12. This condition is DISCHARGED**; see the authorization block at the
> head of this node for what `c2` may and may not claim. `AC-K12` is still not
> claimed or advanced here.

> # `c2-pre` — ADDED 2026-08-10. THE FAIL-CLOSED ARGUMENT IS WRONG IN BOTH
> # DIRECTIONS. THE PROPERTY ITSELF HOLDS.
>
> **Confirmed Adversary finding `evt_2xryrnxz7g0mb` on merged `c1`
> (`7bfc8ae5`), measured at `90ddcf1c`, triaged by the Steward and folded here
> rather than filed as a node** (`steward.md §4c` — same file, same premise, and
> the harm lands on this node's own next author).
>
> **Severity, in the honest direction: a documentation defect on a load-bearing
> safety argument. Over-claim. The mechanism is correct; the reason given for
> trusting it is not.** The Adversary attacked the impossibility claim on four
> axes — parameter, wider-typed sibling, module scope, call graph — and **all
> four hold**. Do not repair the mechanism.
>
> `crates/ken-runtime/src/cranelift_backend/artifact/mod.rs:71-76` discharges
> *"implicit legacy fallback is structurally unreachable"* with a producer
> enumeration. **The type system supplies only the first clause** — the
> authority is a required parameter, not an `Option`. Everything after the
> semicolon is prose, and both halves of it are wrong:
>
> 1. **`program_authority` exists in no tree.** One hit at `7bfc8ae5` and it is
>    this comment; zero at the declared base `b654d33a`, so it is not a stale
>    operand. `git log -S` places it: introduced as `fn program_authority` in
>    `b24a537e`, renamed to `program_admission` in `39bc86f7`, **both inside this
>    branch**. The rename moved the function eleven lines above (`:63`) and left
>    the sibling citation behind.
> 2. **There is a third production producer and it is not `#[cfg(test)]`.**
>    `lowering/core.rs:1771` `seed_only_legacy_authority()` is ungated
>    production, reaches lowering via `unwrap_or_else` at `:1744`, and
>    `NativeProcessSymbols::legacy_prelude()` (`native_process_entrypoint.rs:67`)
>    is `pub(crate)` with no `cfg`.
>
> ⇒ *"the only other producer is the `#[cfg(test)]` synthetic entrypoint"* and
> *"no third way to reach lowering"* are both false **as written**. The true
> statement is narrower and the seed-lane comment at `core.rs:1738-1742` already
> makes it correctly: no third way for a **package-backed** program.
>
> ### WHY THIS IS WORSE THAN A TYPO, AND WHY IT IS `c2`'s
>
> **The two defects hide each other.** A reader auditing *"is there really no
> third way?"* greps `program_authority`, gets zero, and silently repairs it to
> `program_admission` — the obvious and correct fix. **The citation defect then
> vanishes and the enumeration is never re-audited**, which is the half that is
> actually wrong.
>
> **The unqualified sentence is what a `c2` author adding a lowering entry
> reads**, which is why this is folded here and not deferred: `c2` is the
> deliverable that adds lowering outcomes against this exact boundary.
>
> **The shape is the durable lesson.** The safety property is carried by a
> **call-graph fact**, not by the type. A non-`Option` parameter proves that
> whoever calls that function supplies authority; it proves **nothing** about
> which function a package-backed compile calls. That gap is precisely the
> obligation the comment set out to discharge.
>
> | AC | criterion |
> |---|---|
> | `AC-c2p-1` | the comment names `program_admission`; `git grep program_authority -- crates/` returns zero |
> | `AC-c2p-2` | the impossibility claim is **qualified to package-backed**, matching `core.rs:1738-1742` |
> | `AC-c2p-3` | `seed_only_legacy_authority` is named as a real production producer, not omitted and not described as test-gated |
> | `AC-c2p-4` | the argument states it rests on a **call-graph fact**, not on the parameter's type |
>
> **Scope: comment prose only.** No signature change, no `cfg` change, no
> mechanism change. All four attacked axes hold and must stay as they are.
>
> ⛔ **This does NOT go in front of the RecursiveDescent campaign.** It rides
> with `c2` whenever `c2` is authorized. Filing an Adversary finding into
> Runtime's lane ahead of the campaign is exactly the shadow-gate error that
> cost this program ~13.5 hours on 08-09/08-10.

> # `D1b-role-b` MERGED 2026-08-10. SLICE `c` IS CUT IN TWO. START AT `c1`.
>
> Exact `7e918bdf`, PR #1771, CI green, `main` `8e2883b0`. All four declared
> paths blob-verified from the declared merge-base `faabc2ed`, count checked
> against the ring's declared scope. Decision `dec_1amndpa3aaay5`.
>
> **Landed:** `RuntimeCheckedRoleSymbolsV1` decoded from the `semantic.metadata`
> lane at erasure into typed Runtime metadata and validated -- every role in
> `semantic.symbols`, constructor roles resolving through `data_metadata` to
> exactly one family. Absence stays `None`; corruption is
> `ErasureError::InvalidRuntimeRoleAuthority`. Partial-b only: **no `AC-K12`
> claim**.
>
> ## Slice `c` cut, confirmed by the Steward 2026-08-10 (`evt_6z7wf6dw94cym`)
>
> Proposed by runtime-leader (`evt_5bhxf5cj2xh6b`) and confirmed as cut, not
> subdivision -- `c1` is a **fail-closed contract** that merges independently,
> `c2` is **semantic admission**. Combining them would put an unbounded
> question behind a bounded one, which is what made slice `a` too wide.
>
> - **`c1` -- the consumption boundary.** Require the typed record for
>   package-backed compilation; reject missing/malformed/duplicate/inconsistent
>   authority before `plan_static_transition_graph_with_symbols`; pass the exact
>   record into lowering; make package lowering take non-`Option` authority so
>   implicit legacy fallback is structurally unreachable. Controls: the three
>   named pre-lowering rejection cases, explicit-legacy lowerer causality, and
>   the seed-only-explicit-legacy versus no-package-implicit-fallback boundary.
> - **`c2` -- the lowering outcomes that rely on that boundary.** Real-`D5`
>   refusal `1→0`/`StructuralNat`; unrelated Peano-shaped user `Data` remaining
>   `Constructor`; all merge shapes and the catch-all retained.
>   **Comes back to the Steward before assignment.**
>
> **Two Steward conditions on the cut:**
>
> 1. **`c1`'s three rejection controls sit BEHIND the validation `D1b-role-b`
>    just landed.** Each may red at erasure validation before the new
>    pre-lowering check runs -- the same false discharge this ring documented
>    an hour earlier. Establish which layer produced each red; where the outer
>    layer subsumes the inner, use the two-factor construction and say so
>    rather than reporting three independent controls. An unreachable case is a
>    finding.
> 2. **`c2` must state its relationship to `AC-K12` before assignment.**
>    `KERNEL-NESTED-IND` is `active` on that one criterion, it is
>    Runtime-owned, and two Kernel seats are idle behind it. Discharges,
>    partially advances, or orthogonal -- and if not discharged, what still
>    stands between them.
>
>    **DISCHARGED 2026-08-12** — runtime-leader `evt_77pege8j5cv14`, on Steward
>    request `evt_6pmftb5fpxrkm`. **`c2` PARTIALLY ADVANCES `AC-K12`; it does
>    NOT discharge it.** It clears the real `D5` scalar-merge refusal by proving
>    the arriving operand is `StructuralNat`, retaining the unrelated-`Data`,
>    merge-shape and catch-all boundaries. `AC-K12` still needs the nested-IH
>    computation to form a valid native artifact, pass Cranelift verification,
>    execute natively, and agree with the interpreter at Nat 3, with its carried
>    control no longer ignored.
>
>    **The remainder is cut as [[RT-NESTED-IH-NATIVE-REALIZATION]]** — native
>    realization of the full nested-IH continuation beyond scalar admission
>    (emitted definition, ABI/owner wiring, execution surviving the verifier and
>    agreeing with the interpreter). **Not a third slice here:** it is the
>    unbounded question, and folding it behind `c2`'s bounded one is precisely
>    what made slice `a` too wide.
>
>    ⇒ **`c2`'s pre-assignment conditions are now both met. Nothing on the
>    Steward stands between `c2` and assignment when the lane reaches it.**
>
> # `D0` AND `D1a` ARE DONE.
>
> ⚠ **`D1a` closed 2026-08-09 (`evt_3g4n00s7ftd9q`) with a verdict my own
> taxonomy could not express**, and Architect ruling `evt_2wm35zk98p9nr` recut
> the repair. **`D1b-cov` and `D1b-rep` are both WITHDRAWN; the deliverable is
> `D1b-role`.** Read that section, not the fold-coverage framing that preceded
> it — and note that `D0`'s inductive-cascade mechanism story is retracted.
>
> ⚠ **`D1b-id` is ALSO superseded, recut 2026-08-09 as `D1b-role` on Architect
> ruling `evt_23eb7gp8sz4an`.** It is not a transport: the authority does not
> exist on the value path and must be produced. Two conclusions from the
> falsifying measurement are themselves false — see the superseded block.
>
> `D0` closed 2026-08-09 (`evt_1ct16entsqn94`) and answered all four questions
> with `file:line` evidence. **It also measured two of this frame's own fixed
> inputs FALSE and reported them instead of building around them**, which is
> what the perishable-anchor instruction below asks for. Both are corrected in
> place.
>
> **The repair is bounded now, and by a different question than this frame
> originally asked.** `AC-2` posed *scalar-representable vs structurally wider*;
> the measured answer is that it **is** representable, and the thing that
> actually bounds the fix is **where the Peano fold's induction broke**. `D1` is
> cut against that, with both outcomes pre-ruled.
>
> `size:` stays `TBD` until `D1a` reports, and that is still honest rather than
> lazy — one of `D1`'s two branches is a repair and the other is an Architect
> escalation, and they are not the same size.

Treat every anchor below as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around it.

## What it is

`KERNEL-NESTED-IND` `D5` made nested-inductive elimination work through the
elaborator, the interpreter, and checked-artifact erasure. It then reached
native lowering and refused:

```text
NativeLoweringOrExecution: a carried Match arm
  -- dynamic arms must produce scalar Int or Bool values
```

**This is a Runtime-owned capability gap, not a Kernel defect.** Kernel may not
edit `crates/ken-runtime`; the planner/lowering invariant is Runtime's, and a
Steward authorization to the contrary was overruled once already.

## Fixed inputs, measured at `main` `46c12adb`

| fact | value |
|---|---|
| refusal site | `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:15898-15900`, the `_ =>` arm |
| enclosing function | `merge_scalar_operand`, `:15749` |
| `ScalarMergeKind` | `:14138` — `Int`, `Bool`, `StructuralNat`, `ExitCode`, `RecursiveBackedge` |
| producing WIP | `51c482a5` on `wp/KERNEL-NESTED-IND-D5`, six paths, `crates/ken-runtime` diff **empty** |

⛔ **TWO FIXED INPUTS ABOVE AND BELOW WERE WRONG. `D0` measured them false and
they are corrected here.** The originals are struck through rather than deleted,
because a reader who saw the earlier version needs to recognise what changed.

**Correction 1 — the admitted set omitted three live arms.** It listed only
`StructuralNat`, nullary bool, `ProcessExitStatus`, and the checked-root-exit
path. The general match **also** admits:

| arm | site |
|---|---|
| `Lowered::Int` | `:15846` |
| `Lowered::Bool` | `:15853` |
| `Lowered::RecursiveBackedge` | `:15839` |

**Correction 2 — `D0` question 3's premise was false.** It asserted
`RecursiveBackedge` is *"a declared `ScalarMergeKind` variant that no arm here
produces."* **Two arms produce it** — `:15804` in the `required_kind ==
ExitCode` branch and `:15839` in the general match — and there is a third,
explicit `RecursiveBackedge` **refusal** on the carried path at `:15785`. ⚠ The
reachability *question* was still real and `D0` answered it; only its premise
was wrong.

So the corrected admitted set is: `StructuralNat`; nullary `bool_true`/
`bool_false` constructors; `ProcessExitStatus`; `Int`; `Bool`;
`RecursiveBackedge`; and any `lowered` under `checked_root_exit_representation`.

## `D0` RESULT — measured, all four questions answered

**Anchor:** `lowering/mod.rs` is the same blob `f9601b12` at `46c12adb`,
`c34317f3`, `51c482a5`, and the measuring tree, so every line number above is
interchangeable across all four.

**Q1 — the variant at the seat.** `Lowered::Constructor`, `Nat::Suc`, arity 1,
whose single argument is itself a `Constructor` — **an unfolded Peano chain**,
not a `StructuralNat`. Read from an instrument inside the `_ =>` arm, which is
what `AC-1` required.

**Q2 — scalar-representable, direction stated.** ⭐ **It IS representable and is
NOT structurally wider than the pair.** `StructuralNatV1` is a single `i64`
(`mod.rs:10143`), and the backend already folds Peano chains into it at
`mod.rs:17257-17267` and `core.rs:13940-13951`.

> ⭐⭐ **THE DISCRIMINANT IS NOT "SCALAR VS WIDER". `AC-2` ASKED THE WRONG
> QUESTION AND `D0` ANSWERED THE RIGHT ONE.**
>
> **Both folds are inductive on their own output.** `Suc` folds only if its
> predecessor **already** folded:
>
> ```rust
> if constructor == self.process_symbols.nat_suc {
>     if let [Lowered::StructuralNat(predecessor)] = lowered_args.as_slice() { ... }
> }
> // otherwise falls through to Lowered::Constructor
> ```
>
> ⇒ **A single unfolded link makes every enclosing `Suc` fall through too** —
> exactly the measured shape (`Suc` with `arg_kinds=[Constructor]`). The
> question that bounds the repair is therefore **where the induction broke**,
> not how wide the value is.

**Q3 — reachability, with a positive control that fired.** Over `-p ken-runtime
--lib`, 843 tests: 775 seat entries, 776 general-match arrivals, **0 refusals**;
carried path 0, `ExitCode` branch 0. Arrivals were `Constructor` 548, `Int` 110,
`ProcessExitStatus` 80, `Bool` 38. **`RecursiveBackedge` is not reached at this
seat**, zero at both producing arms.

⭐ **The positive control, and the failed first attempt that makes it
trustworthy.** Widening `:15839` to also admit `StructuralNat` printed **zero**
— because `StructuralNat` never reaches that match at all, so a zero would have
looked exactly like the wanted answer. That failure forced the branch
partition, which supplied a witness **chosen from the measured arrivals rather
than guessed**: widening with `ProcessExitStatus` fired **50 times**.

⚠ **Honest split, carried forward:** the carried path (`:15785`) and the
`ExitCode` branch (`:15804`) each take **0 arrivals**, so for those two the
claim is *"the branch is unreached"*, ⛔ **not** *"the arm is unreached"* — no
positive control is available for them from this population.

**Q4 — `D5` is the first arrival of this shape.** Zero refusals across 775
entries. Sharply: `Lowered::Constructor` reaches this seat **548 times and never
refuses**, because those are nullary `bool_true`/`bool_false` caught at
`:15866`. The `D5` value is a `Constructor` that is **neither nullary nor
bool**. ⇒ The variant is common here; **the shape is new.**

⚠ **Domain bound, stated by the measurement and not to be dropped:** this is the
`ken-runtime --lib` population plus the one `D5` cross-crate case. `ken-cli`,
`ken-verify`, and elaborator entries to this seat were **not** censused, so
*"no other caller is already refusing"* holds over the in-crate population, not
over every compilation entry.

## `D0` — measure the produced value. This is the whole first slice.

**Do not repair anything in `D0`.** Report:

1. The exact `Lowered` variant the refused arm carries, at the refusal, for the
   `LiftRose`/`Bag` Nat-3 case that `KERNEL-NESTED-IND` `D5` drives.
2. Whether that variant is **scalar-representable at all** in
   `NativeScalarPairV1`, or whether it is structurally wider than the pair.
   **These are materially different futures and they bound different repairs**
   — one is a missing arm, the other is a representation question.
3. Whether `RecursiveBackedge` is reachable at this seat. It is a declared
   `ScalarMergeKind` variant that **no arm here produces**, and
   `RT-MATCH-RECURSOR-CONSUMERS` records it as protocol-only and untouched.
   Say which it is; do not assume from the enum.
4. Whether any **other** caller of `merge_scalar_operand` already passes a
   value of the same shape and is therefore already refusing in production, or
   whether `D5` is the first arrival.

**`D0` closes when those four are answered with `file:line` evidence.** It does
not authorize a repair and it does not size one.

## `D1` — FRAMED 2026-08-09 against `D0`'s fold-induction discriminant

> **Both outcomes are ruled below, so `D1a` does not come back to me.** One
> branch proceeds straight to the repair; the other is a genuine stop. ⛔ Do not
> treat the stop as a failure — it is a different and larger design question,
> and reaching it is a real result.

### `D1a` — measure WHERE the fold's induction broke. One instrument.

**This is the measurement `D0` named as one instrument away and deliberately did
not take.** Walk the `Nat::Suc` chain the `D5` case presents and find the
**innermost link that failed to fold**, then report which of exactly two things
it is:

1. **Coverable** — the base `Zero`, or an intermediate link, failed to fold for
   a reason the existing fold could handle: an unmatched constructor spelling, a
   missing `Zero` base case, an argument arriving in a shape the `if let`
   pattern does not destructure.
2. **Genuinely dynamic** — the link's predecessor is not a compile-time-known
   constructor at all, so no inductive fold can reach it in principle.

⛔ **Report the innermost failing link, not the outermost.** Every enclosing
`Suc` falls through once one link does, so the outermost failure is guaranteed
and says nothing. ⚠ This is the same trap as a short-circuiting probe measuring
the first cause rather than the set.

> ## ⛔⛔ `D1b-cov` AND `D1b-rep` ARE BOTH WITHDRAWN. THE REPAIR IS `D1b-id`.
>
> **`D1a` measured a THIRD case my taxonomy did not contain, and Architect
> ruling `evt_2wm35zk98p9nr` named it: an identity-authority TRANSPORT defect.**
> Not missing fold coverage, and not a representation problem.
>
> ⛔ **My `D1b-cov` prohibition forbade the repair.** It banned Elaborator and
> compiler-driver edits — and the compiler driver is **half the fix**, because
> it is the producer of the checked constructor identities. A frame whose banned
> scope excludes the only route to its own AC is a defect in the frame, and this
> one was mine.
>
> ⚠ **`D0`'s inductive-propagation story is RETRACTED, by its own author.**
> `D0` reported that `Suc` folds only if its predecessor folded, so one broken
> link cascades. `D1a` measured that **the fold never engages on ANY link** —
> so cascade was never what was happening. Anything upstream of this node that
> repeats the cascade story, including this frame's earlier text and my
> `b92b3f3f` briefing, is wrong on the mechanism.

### What `D1a` measured

Innermost failing link is **depth 1, the base
`ctor:nested_inductive_pkg::Nat::Zero`**; the chain is `Suc(Zero)` and every
link is compile-time known. The fold compares against
`ctor:prelude::Nat::{Zero,Suc}`.

⭐ **A walker trap the implementer hit and corrected**, worth keeping: the first
walker descended on `*constructor == nat_suc` — **the very predicate under
test** — so it stopped at depth 0 and would have reported the *outermost* link
as innermost, which `AC-6` explicitly rejects. Descending on structure reached
the real base.

### Why this is NOT a user type, and why structural recognition is unlawful

**The measured `nested_inductive_pkg::Nat` is the PRELUDE `Nat`.** The `D5`
source declares `Bag` and `LiftRose` and refers to the prelude `Nat` already in
the live elaboration environment; `stable_symbols_for_env` renders every live
`GlobalId` through the package's stable-symbol table, so prelude `Nat` renders
package-qualified. No user type is being folded and no constructor identity is
being discarded.

⛔ **The structural Peano criterion is NOT lawful at this seat** (Architect).
Constructor *shape* is not checked constructor *identity*: an unrelated user
`Data` can be nullary-plus-unary-recursive and therefore Peano-isomorphic
without being Ken `Nat`. Folding on shape would erase an observable identity —
the blanket widening this node forbids.

### The actual defect

The `D5` differential helper erases a generic `CompilerDriverOutput` to
`RuntimeProgram` and calls the value runner, which reaches
`compile_expr_into_module` with **`process_symbols=None`**. Runtime substitutes
`NativeProcessSymbols::legacy_prelude()`, so the exact package constructors are
compared against `ctor:prelude::Nat::{Zero,Suc}` and miss. **The producer had
the right identities; the consumer never received them.**

> ### `D1b-id` IS SUPERSEDED BY `D1b-role`
>
> **Recut 2026-08-09 on Architect ruling `evt_23eb7gp8sz4an`.**
>
> **The `D1b-id` frame below was defective and the defect was mine:** its
> producer, `compiler_driver.rs:3336-3337`, runs only in the process-starter
> transaction, so the generic value package never materializes a role table at
> all. Runtime measured that (0 instrumented producer lines with the instrument
> confirmed present and the `D5` refusal confirmed firing) and stopped instead of
> building an inert transport. That was the right call.
>
> **But two conclusions drawn from those measurements are FALSE, including one I
> propagated when I withdrew controls #1 and #3 as unsatisfiable. They are not
> unsatisfiable.** Correcting both, because a recut built on either is wrong:
>
> | conclusion | status | what is actually true |
> |---|---|---|
> | *"zero `Data` rows ⇒ the package `Nat` identity is absent from `RuntimeProgram`"* | **FALSE** | erasure keeps `declarations` minimal but independently copies every `semantic.symbol` into `erased_core.symbols` (`erasure.rs:195-205`) and every `semantic.data_metadata` entry into `erased_core.metadata.checked_core.data_metadata` (`:5918-5952`), which `ir.rs:43-50` calls authoritative after proof erasure. The probe counted **executable declarations only** and inspected neither authoritative metadata lane. The correct statement is *"`Nat` is not an executable target declaration"* |
> | *"resolving the prelude `GlobalId` yields a legacy-prelude symbol, byte-equal to `legacy_prelude()`"* | **FALSE** | `emit_package_from_env` calls `stable_symbols_for_env(&manifest.package_name, ...)` at `compiler_driver.rs:2960`. That table maps every non-primitive declaration id through `declaration_symbol(package_name, name)` (`:3492-3496`) and constructors under the package-qualified parent (`:3499-3509`). Applied to **that** table, `resolve_id(prelude_env.zero_id/suc_id)` yields `ctor:nested_inductive_pkg::Nat::{Zero,Suc}` |
>
> ⇒ **The ids' prelude origin and their current artifact stable spelling are
> different axes.** Controls #1 and #3 become satisfiable the moment the producer
> is placed on the real generic package-emission path — which is the recut.
>
> **A conditional probe is what made both look settled.** The first `Data`-only
> probe could not separate *"no `Data` declarations"* from *"not on this path"*;
> the same conditional-probe mistake, in another guise, made the first `D1a`
> walker report the outermost link. Make probes unconditional.

### `D1b-role` — materialize a COMPLETE checked-runtime role record, carry it
### through erasure, and REQUIRE it at package-backed native compilation

**Not a transport of an existing authority — the authority does not exist on this
path and must be produced.** ⛔ **The scalar merge is still NOT the repair
site**: no new `merge_scalar_operand` admission, no `ScalarMergeKind` change.

**The executable-erasure boundary is preserved.** ⛔ Do **not** add `Nat`,
`Bool`, or any prelude type as `RuntimeDeclarationKind::Data` entries to teach
native lowering their roles. That widens the executable closure to solve a
**metadata** problem. This deliverable is a checked-artifact metadata extension
plus Runtime consumption — not a change to proof erasure, and not a change to
runtime value representation.

1. **Produce.** One versioned, hash-covered checked semantic record,
   conceptually `CheckedRuntimeSymbolsV1`, built **inside `emit_package_from_env`
   after the exact `stable_symbols_for_env` call** (`compiler_driver.rs:2960`)
   and **before the live `ElabEnv` is lost**. Derived from prelude `GlobalId`s
   through that exact table. ⛔ No source-name, suffix, package-name
   reconstruction, or structural Peano inference.
2. **Complete, not Nat-only.** The record must cover **every constructor role to
   which Runtime assigns special meaning**. The current complete population is
   the existing `NativeProcessSymbols` field set: Bool, Nat, Unit, Result/Option,
   process/list/product/exit, file/resource/progress and related constructors.
   ⛔ **A Nat-only sidecar reproduces this exact defect at the next special
   constructor** and does not discharge this deliverable.
3. **Store canonically.** In the checked package's semantic lane — the existing
   versioned `semantic.metadata` lane is lawful — so it participates in
   `core_semantic_hash` and survives serialized-package consumption. ⛔ A live
   `CompilerDriverOutput` sidecar is **not** lawful: the compiler's semantic
   input is `CheckedCorePackage`, not a retained `ElabEnv`.
4. **Decode and validate at erasure.** Into a typed Runtime field, preferably
   `RuntimeCheckedCoreMetadata.runtime_symbols: CheckedRuntimeSymbolsV1`.
   Validate every role symbol against `semantic.symbols`; constructor roles must
   also resolve **uniquely** through the existing `data_metadata`
   family/constructor entries with their recorded arity and recursive positions
   — for `Nat`, `Zero` nullary and `Suc` unary with the recorded recursive
   position. ⚠ These checks detect **stale or mismatched metadata**; they do not
   infer the `Nat` role from shape, and must not be written as if they could.
5. **Require at consumption.** Package-backed compilation requires the typed
   field. `compile_program_expr` passes its exact table to the lowerer. Missing,
   malformed, duplicate, or metadata-inconsistent authority **rejects before**
   `plan_static_transition_graph_with_symbols`.
6. **Remove the ambiguity structurally, not by discipline.** The inner package
   lowerer takes `&CheckedRuntimeSymbolsV1` (or `&NativeProcessSymbols`), **not**
   `Option<&...>`. `core.rs:1781-1783`'s `unwrap_or_else(legacy_prelude)` must
   not remain reachable from a `Some(program)` compile. Seed-only `compile_expr`
   **may** construct and pass `legacy_prelude()` **explicitly**, because its IR is
   deliberately minted in that namespace.
7. **Folds unchanged.** Both Nat folds stay **exact-identity and inductive**. ⛔
   No fold-code change, structural criterion, eager chain walk, whole-chain
   walker, declaration-shape recognizer, or name/suffix matching.
   `StructuralNat` remains the native representation of the checked `Nat`
   identified by that authority.

> ### SLICE `a` WIDENED IN PLACE 2026-08-09 — the immutable canonical role roster
>
> **Architect ruling `evt_6q4tvtenb1wps`, recorded here by the Steward so the
> enlarged diff is self-authorizing.** Decision `dec_7v589ezdeq321` rejected
> `aade3c2f` on one authority defect — **not** on the hash control, which stands.
>
> **The defect:** both producers select authority by **mutable source spelling**
> (`env.globals.get(name)` after package source elaboration). Mapping a
> name-selected id through `stable_symbols_for_env` does not cure that — the id
> is already the wrong one. Every Runtime constructor role must originate from an
> **immutable canonical prelude `GlobalId`** captured at prelude registration,
> before package source elaboration, and every stored symbol must be
> `exact_stable_table[canonical_role_global_id]`.
>
> ⛔ **Do NOT split this off as a preparatory `a0` WP.** The Architect ruled
> against it directly: the roster is the **only lawful implementation of item 1**,
> and `CheckedRuntimeSymbolsV1` embeds `CheckedHostSpineV1`, so produce/store/
> hash-cover cannot be an independently correct accepted partial while either
> producer still resolves by name. A split would draw an artificial boundary
> inside one authority producer and yield no separately usable contract. **One
> fresh slice-`a` candidate, one fresh QA/Decision lineage.**
>
> **Authorized scope widening** (this is what the carve-out above now grants):
> `prelude.rs`/`PreludeEnv` registration, **both** producer resolvers in
> `compiler_driver.rs`, the existing slice-`a` record/storage/hash paths, and
> focused tests. Prefer a **nested immutable roster type** inside `PreludeEnv`
> over expanding unrelated top-level fields. Use semantic fields or a **closed
> internal role enum** for collections — ⛔ do not replace the string lookup with
> a later string-keyed authority map, which reproduces the defect one layer up.
>
> **Completeness is every currently name-resolved entry of BOTH producers**, not
> the six former plan roles: host families, constructors, error/resource/progress
> roles, Bool/Unit, and the public operation identities. Existing private-operation
> ids remain valid roster members. Measured starting point, from the implementer's
> roster survey — this is the part that needed measuring and it is done:
>
> | roster state | roles |
> |---|---|
> | canonical id **exists** in `PreludeEnv` | `Nil` (`prelude.rs:130`), `Cons` (`:131`), `MkProd` (`:141`); and on the spine side `Some`, `Err`, `Ok`, `MkUnit` |
> | canonical id **MISSING**, must be captured at registration | `MkProcessInput`, `Success`, `Failure`; and on the spine side `True`/`False` and most resource/progress roles |
>
> **The committed discriminator must cover two properties SEPARATELY.**
> (1) **inventory completeness** — no name-resolved role remains in either
> producer; (2) **substitution resistance** — package declarations shadow
> representative constructor, family, and operation spellings while the emitted
> record still equals the exact canonical-id-to-stable-symbol projection,
> **including parent identities**. ⛔ **Bare-name containment is not evidence** —
> that is precisely the blindness that let the rejected candidate pass. If one
> fixture can lawfully shadow the full public roster, assert all entries;
> otherwise use a table-driven exact projection plus representative collisions
> per namespace/path class.
>
> **Retained unchanged:** the record-presence/version control and the
> semantic-hash mutation/removal pair, both already valid. **All partial-`a`
> negative boundaries still bind** — no decode/consumption, no executable `Data`,
> no fold/scalar work, no native admission, no `AC-K12` claim.
>
> ⚠ **`9d3273a8` is the blocked baseline only** (`aade3c2f` replayed onto
> `4a903d46`, content identical, object different). **No prior SHA-bound verdict
> transfers** — not QA's approval, not the Architect's earlier vote.

**Sizing: land this as up to three accepted partials, in this order.** Each is
independently reviewable and mergeable per the accepted-partial policy, and each
is roughly a one-hour turn; do not hold the whole chain for one PR.

| slice | scope | closes |
|---|---|---|
| `a` | produce + store + hash-cover the record (items 1-3) | control 1 |
| `b` | erasure decode + validation (item 4) | control 2 |
| `c` | require + consume + structural de-`Option` (items 5-7) | controls 3-6 |

### Required committed discrimination — all six

| # | control |
|---|---|
| 1 | on the **real generic `D5` package-emission path**, the produced Nat roles are exactly `nested_inductive_pkg::Nat::{Zero,Suc}` and **differ** from the explicit legacy pair. ⛔ The probe must fire **on that path** — an unconditional probe, for the reason recorded above |
| 2 | after erasure, the executable declaration set is **still exactly** `liftAdd` and `liftSize`, **while** `erased_core.checked_core.data_metadata` and the typed runtime-symbol record both carry the exact Nat family and pair. This pins metadata preservation **without** closure widening — both halves are required |
| 3 | **three separate** rejection controls: delete the semantic role record; corrupt its header; mutate **only** Nat `Zero` to the legacy symbol. Each must reject **before native semantic lowering**, through a **named** authority-validation lane. ⚠ Separately, a focused lowerer control with an **explicitly supplied** legacy table must leave the package-qualified chain as `Lowered::Constructor` — that proves the identity operand is **causal**, rather than resting only on preflight |
| 4 | `AC-10` re-run at the real `D5` seat: refusal **1 → 0** and the operand arrives as `StructuralNat`. ⛔ A green end-to-end result alone is insufficient |
| 5 | an unrelated **nullary-plus-unary-recursive** `Data` remains `Lowered::Constructor` and retains its constructor identity — the counterexample excluding structural widening |
| 6 | existing **explicit** legacy seed controls stay green, **and** a structural control proves no package-backed compile can reach an **implicit** legacy fallback. All **six** admitted merge shapes and the fail-closed catch-all preserved |

⛔ **No Kernel, interpreter, match-semantics, `ScalarMergeKind`, or `AC-K12`
claim follows from this ruling.** `AC-K12` **is** reachable on the current
architecture — the Architect states this — but this deliverable discharges only
the **first** native-lowering refusal. Verifier passage and interpreter/native
agreement remain separate gates.

### Acceptance for `D1`

| AC | criterion | control |
|---|---|---|
| `AC-6` | `D1a` names the **innermost** failing link with `file:line` and the constructor at that link | a report naming only the outermost `Suc` does not discharge it — that failure is entailed by any inner one |
| `AC-7` | `D1a`'s verdict is **coverable** or **genuinely dynamic**, stated in a direction | if it genuinely depends, name the discriminant, as `AC-2` required and `D0` did |
| `AC-8` | Any `D1b-cov` repair leaves the **six** currently-admitted shapes byte-for-behaviour unchanged | the corrected admitted set above, **not** the four the original frame listed |
| `AC-9` | `D1b-cov` keeps the fold **inductive**, not eager | the existing `if let [Lowered::StructuralNat(pred)]` shape survives; a whole-chain walk fails this row |
| `AC-10` | A positive control proves the repaired fold **actually folds the `D5` chain** at the seat | ⚠ re-run `D0`'s seat instrument: the refusal count at `_ =>` for the `D5` case must go 1 → 0, **and** the arrival must be `StructuralNat`. A green `D5` test alone does not discharge this — it could pass by a different arm admitting the `Constructor` |

⛔ **`AC-5`'s fail-closed requirement is unchanged and now matters more.**
Widening the fold must not widen the `_ =>`. A value outside the admitted set
still refuses with a diagnostic naming it.

### What `D1` still does NOT discharge

`KERNEL-NESTED-IND` `AC-K12` needs native lowering **and** the Cranelift
verifier **and** interpreter/native agreement. `D1` addresses the first refusal
only. ⛔ Do not report `AC-K12` as discharged because the `D5` case stops
refusing here; the verifier and the differential are separate stages and may
surface their own gaps.

## Acceptance

| AC | criterion | control |
|---|---|---|
| `AC-1` | `D0` names the exact `Lowered` variant at the refusal | the variant is read **at the seat**, not inferred from the arm's source expression. A characterization taken upstream of `merge_scalar_operand` does not discharge this |
| `AC-2` | The scalar-representability question is answered **in a direction**, not hedged | state whether it fits `NativeScalarPairV1` or exceeds it, and why. "It depends" is not an answer; if it genuinely depends, name the discriminant |
| `AC-3` | The `RecursiveBackedge` reachability claim carries a witness **or** an explicit "not reachable here, and this is how I established that" | a negative check passes for any reason, so an unreached-variant claim needs a positive control showing the instrument would see it if it fired |
| `AC-4` | Any repair (`D1`, when framed) leaves the four currently-admitted shapes **byte-for-behaviour unchanged** | `StructuralNat`, nullary bool, `ProcessExitStatus`, and the checked-root-exit path each keep their existing arm and result |
| `AC-5` | Any repair keeps the `_ =>` **fail-closed** | widening the admitted set must not convert the catch-all into an accept. A value outside the new admitted set still refuses with a diagnostic that names it |

## Forbidden

- **Blanket relaxation of the scalar contract.** Widening `merge_scalar_operand`
  to accept arbitrary `Lowered` values is not the repair, whatever `D0` finds.
  Same reasoning as [[RT-CARRIER-BYTESPAN-OBSERVE]]: availability is per seat,
  never a blanket phase relaxation.
- **Folding this into [[RT-CARRIED-RESOURCE-SCALAR]].** That node's refusal is
  an effect-seat `ResourceScalar`-in-`CarriedWord` shape — a different need on
  different seats. Its own frame warns against exactly this
  same-shape-different-population fold, and it is `draft` with no frame.
- **Folding this into [[RT-TERMINAL-ALL-ELIM-AUTHORITY]].** Different seat:
  that node owns `lowering/core.rs:6178-6183`, the `ComputationalRecursorClosure`
  remainder arm. This is `lowering/mod.rs:15898`. Checked, not assumed.
- Editing `crates/ken-elaborator`, `crates/ken-kernel`, or `crates/ken-interp`
  to make the arm produce something the existing seat already accepts. That
  moves a Runtime gap into Kernel's landed work.
  > ⚠ **NARROWED 2026-08-09 for `D1b-role` only, and WIDENED the same day by
  > ruling `evt_23eb7gp8sz4an`.** Read the whole carve-out; the first version of
  > it was too narrow to reach the repair.
  >
  > ⇒ **In scope for `D1b-role`, inside `crates/ken-elaborator`:**
  >
  > - `compiler_driver.rs` — `emit_package_from_env`, to build the
  >   `CheckedRuntimeSymbolsV1` record from the exact `stable_symbols_for_env`
  >   table while the live `ElabEnv` is still in hand, **and both producer
  >   resolvers** (`checked_runtime_symbols_v1` and `checked_host_spine_v1`);
  > - **`prelude.rs` / `PreludeEnv` registration**, to capture the immutable
  >   canonical role-`GlobalId` roster — added 2026-08-09 by ruling
  >   `evt_6q4tvtenb1wps`, see the block below;
  > - the checked package's versioned `semantic.metadata` lane, to carry that
  >   record under `core_semantic_hash`;
  > - `erasure.rs`, to decode and validate it into the typed Runtime field.
  >
  > ⛔ **Everything else in this bullet still binds**, and the original intent is
  > untouched: **you may not reshape what the arm *produces* to dodge the Runtime
  > gap.** The carve-out is for *carrying identity that already exists*, never for
  > changing the value. `ken-kernel` and `ken-interp` remain fully out of scope,
  > and so does the rest of `ken-elaborator`. Adding `Data` declarations to the
  > executable closure is forbidden by `D1b-role` item 2 regardless of crate.
  >
  > **This bullet forbade the only route to the repair for about forty
  > minutes, and its first repair was still too narrow to reach the second.**
  > Recorded because a Forbidden list is read on its own, far from the
  > deliverable that carves it out.

## `c2` MERGED 2026-08-14 — and the "`c2` proper is owed" reading below is WRONG

> ### CORRECTED SAME DAY, BY THE STEWARD, AGAINST THE OBJECT.
>
> **`57bf1721` is `c2`, not a `c2-pre` prose slice with `c2` still owed.** The
> record below called it *"the prose-and-observation slice"* on one inference —
> *production is untouched, therefore the semantic admission has not landed.*
> **That inference reads a code-change requirement into an AC that never had
> one.**
>
> `c2`'s acceptance is rows **4** and **5** of `Required committed
> discrimination — all six`, and `AC-10` is explicit about its own shape:
> *"re-run `D0`'s seat instrument: the refusal count at `_ =>` for the `D5` case
> must go 1 → 0, **and** the arrival must be `StructuralNat`. A green `D5` test
> alone does not discharge this."* **That is a measurement at the seat, not a
> new admission arm.** The fold that produces `StructuralNat` landed earlier in
> the arc; what `c2` owed was proof at the seat, discriminated from a different
> arm admitting the `Constructor`.
>
> **Measured on the landed diff.** In `lowering/mod.rs` the `match lowered`
> arms are **byte-unchanged** — the hunk renames the match's value to `result`
> and records `admitted: result.is_ok()`. So:
>
> - **Row 4** — `d5_native_scalar_merge_admits_checked_structural_nat` reads
>   `operand_kind == "StructuralNat"` and `admitted` at the seat on the real
>   package path, and QA's mutation reddens it at `admitted: false` with
>   `StructuralNat` count `0 != 1`. The mutation pins the **admission path**,
>   which is exactly the failure `AC-10`'s warning names.
> - **Row 5** — `peano_shaped_user_data_remains_an_exact_constructor` keeps the
>   independently-named Peano shape at exact `Constructor PSuc`, refused.
> - **Row 6's preservation half** is discharged *a fortiori*: an unchanged arm
>   set cannot have widened the `_ =>`, which is what `AC-5` guards.
>
> **The corroboration was on the record before I wrote the wrong version.** The
> Architect's approval commit is titled `architect: approve
> RT-DYNAMIC-ARM-SCALAR-MERGE c2 3e8de4b8`, and the candidate's own commit body
> says *"`c2-pre` **plus the `c2` admission observation**."* Three parties named
> it `c2`; only this record said otherwise.
>
> ⇒ **[[RT-NESTED-IH-NATIVE-REALIZATION]] FLIPS ON THIS MERGE.** The condition
> was `c2` merging and `c2` has merged. The node stays `active` only for the
> `c3` closure slice authorized below, and **`c3` does not gate the successor.**

## `c2` merge record (the facts below stand; only the scope claim was wrong)

**Candidate `3e8de4b87962442b77283dcacfce2c81d6d98cfa`, landed as squash
`57bf1721`** (PR #2163, CI green; Decision `dec_nram6n8jkpgn`, Architect, read
`resolved` from the object). Merge-base `be8535b9`, derived independently and
matching the declared value; one commit, six paths, `+341/-6`; **6/6 blobs
verified identical after landing.** Both SHAs recorded — a squash rewrites the
candidate, so it is never an ancestor of `main`.

~~**Status deliberately unchanged.** `c2-pre` is the prose-and-observation
slice. `c2` — the semantic admission that clears the real `D5` scalar-merge
refusal — has not landed, so nothing downstream is unblocked yet. In particular
**[[RT-NESTED-IH-NATIVE-REALIZATION]] does NOT flip on this merge**; its
condition is `c2` merging, and reading `c2-pre` as satisfying it would arm a
frame against an admission surface that does not exist.~~

**Struck — see the correction banner above.** The admission surface does exist:
it arrives as `StructuralNat` on the real `D5` path and is now pinned at the
seat by a mutation-discriminated control. The node stays `active` for `c3`
only.

**Stale-base check, recorded because the answer was "do nothing":** `main` moved
five doc-only commits under this candidate. The intersection of the candidate's
changed paths with `main`'s was **empty**, so the staleness was immaterial and no
rebase was taken — the branch was frozen under an open PR and a rebase would have
re-pointed it at a SHA no reviewer approved.

## Residuals from `c2-pre`'s approval — two, both non-blocking

Architect findings at `evt_3yk7f4p6k7y64`, on candidate `3e8de4b8`. **Neither is
a defect in what landed and neither holds `c2`.** Recorded here so they are not
re-derived, and so the second is not lost — it is a concrete small fix.

**1. The gating is asymmetric with its sibling, in the direction that makes an
identity control MORE necessary, and it is the one without one.**

```toml
r3-4b-observation = ["ken-runtime/r3-4b-observation"]   # ken-elaborator [features], opt-in
ken-runtime = { path = "../ken-runtime", features = ["dasm-c2-observation"] }  # dev-deps, always on
```

`dasm-c2-observation` is on for **every** `ken-elaborator` test build, and by
Cargo feature unification a `--workspace` test run — what CI does — compiles
`ken-runtime` once with the observation in and links **every other crate's
tests** against that copy.

**The trade may be the better half and the frame does not prejudge it.** An
opt-in control does not run in a default CI invocation at all, so
`r3_4b_observation_feature_is_native_artifact_identical` never executes there;
always-on means this control actually runs. **What is missing is that the trade
is unrecorded.** The opt-in sibling carries **two** identity controls
(`r3_c2_source_mixed_branch.rs:621`, `control.rs:5001`); the always-on one
carries **none**, and "both configurations compile" is a different property from
artifact identity.

**2. The disabled instrumented path still does work.**

```rust
#[cfg(any(test, feature = "dasm-c2-observation"))]
let observed_operand_kind = lowered_value_kind(&lowered);
#[cfg(any(test, feature = "dasm-c2-observation"))]
let observed_constructor = match &lowered {
    Lowered::Constructor { constructor, .. } => Some(constructor.clone()),
    _ => None,
};
```

Both sit **outside** the `DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED.get()` guard,
which lives inside `dasm_c2_record_scalar_merge`. With the feature on and no
scope active — the state of every other crate's tests in a workspace run — each
scalar merge classifies the operand and heap-allocates a constructor name that is
discarded. **Not a correctness finding.** It is the difference between *"inert
when disabled"* meaning **no observable effect** and meaning **no work**, and
only the second is what you want under an always-on feature. Moving both inside
the enabled check makes the disabled path free and the identity claim easier to
state.

~~**Disposition: take them with `c2` proper if they fit, or say so and the
Steward cuts a slice.**~~ Do not treat either as an acceptance criterion on
`c2`.

## NODE COMPLETE — `c3` MERGED 2026-08-14. `status: merged`.

**Every slice has landed: `c1` `7bfc8ae5`, `c2` `57bf1721`, `c3` `6b3b5b40`.**

`c3` candidate `e4e308d19db11403a8d5368d34424eae7db8caee`, **landed as squash
`6b3b5b40`** (PR #2171, CI green; Decision `dec_77sd73v2kqewh`, Architect, read
`resolved` from the object). Merge-base `246019b9` derived independently and
matching the declared value; one commit, two paths, `+26/-11`; **2/2 blobs
verified identical after landing.** Both SHAs recorded — a squash rewrites the
candidate, so it is never an ancestor of `main`; ask content, not ancestry.

**Runtime did not take the trap below.** They kept the always-on
`ken-elaborator` dev-dependency and recorded the rationale, rather than copying
the sibling's opt-in shape — which would have made the two `D5` seat controls
silently absent from a default test run. The Architect confirmed the hoist is
behaviour-preserving on evidence rather than assertion: `lowered_value_kind` is
`&Lowered → &'static str`, one arm per variant with **no `_ =>`**, an ordinary
production function with ~40 call sites, so it is pure and exposes no dead code
in any `cfg` configuration. After filtering `cfg`-gated lines the entire
non-`cfg` delta is a re-indentation plus `admitted: result.is_ok()`.
**Production is untouched.**

### The node closed on a two-clause finding having discharged ONE clause. The other is carried to [[RT-C2-OBSERVATION-ARTIFACT-IDENTITY]].

**Adversary `evt_7cyndqwye5sfr`, confirmed by the Steward against `6b3b5b40`.
The defect is in the `c3` frame, which is the Steward's.**

The Architect's `c2-pre` finding had a heading and a remedy sentence:

> **heading:** *"The gating is asymmetric with its sibling, in the direction
> that makes an identity control MORE necessary, and it is the one without
> one."* The opt-in sibling carries two identity controls; the always-on one
> carries none, and *"both configurations compile"* is a different property
> from artifact identity.
>
> **remedy sentence:** *"what is missing is that the trade is unrecorded."*

**`D-c3-2` offered two options and neither required an identity control, and
`AC-c3-4` checked only that the direction was written down.** The ring chose
always-on — the branch the heading says makes the control *more* necessary —
and the node closed.

**Measured, not inferred.** `dasm_c2` appears in five files under `crates/`;
the only test-side one **uses** the observation. Nothing compares a feature-off
artifact to a feature-on one. The sibling's control is
`r3_c2_source_mixed_branch.rs:621`,
`r3_4b_observation_feature_is_native_artifact_identical`, which compiles one Ken
source twice into separate target directories and asserts the emitted native
objects are byte-identical.

**The mechanism, which is the part worth carrying past this node.** The
deliverable was written from the finding's **remedy sentence** rather than from
its **heading**. That sentence characterizes the *decision* half of a two-part
finding, and it is quotable, so it became the deliverable. The *evidence* half
had no remedy sentence of its own, so nothing derived a deliverable from it.
⇒ **A finding whose two clauses are joined by "and" gets discharged on
whichever clause its author phrased as an ask.**

**Why this does not reopen the node.** The code is landed and correct; the
Architect grounded the disabled path's freeness by reading (`lowered_value_kind`
is pure, one arm per variant, no `_ =>`). What is absent is the **probe** that
would measure it. That is a successor's work, not a retraction of this one, and
the successor is filed rather than left as *"rides the next candidate"* — that
premise expires, and it expired on a Language node this same day.

### Residual carried out of the node, non-blocking: the enable flag is read twice

Architect observation on `dec_77sd73v2kqewh`, explicitly **not** a merge
condition. `ENABLED` was previously read once, inside
`dasm_c2_record_scalar_merge` (`mod.rs:15992`); it is now also read at `:17769`
before the match, to decide whether to compute. Both must be true to record.

⇒ **That introduces an invariant the previous shape did not need: the flag must
not go `true` → `false` between the two reads.** It would then have computed and
declined to record, dropping an observation the pre-`c3` code would have kept.

**State the condition in one direction, not as "must not flip."** Adversary
refinement at `evt_7cyndqwye5sfr`. `false` → `true` records nothing, which is
exactly the pre-`c3` behaviour, so it is harmless. **A residual that overstates
its own condition is harder to discharge than one that states it exactly** — the
one-directional form halves what a future reader has to rule out.

**Why it is a residual and not a defect.** Reaching the harmful direction
requires a `DasmC2ScalarMergeObservationScope` to **drop during** the
`match lowered`. Scopes are RAII around whole compile calls, created by test
harnesses; nothing inside lowering constructs or drops one. **"Not believed
reachable" is a claim about scope lifetime, not about threading.** And the
direction is **fail-closed**, verified at the controls rather than argued:
`nc14_data_match_lowering.rs:467` indexes `psuc_arrivals[0]`, which panics on an
empty vector, and `:462`/`:404` are `assert_eq!` on counts. A dropped record is
loud.

**Not attributable to `c3`:** both reads are of the same thread-local, so
lowering on a thread other than the scope's creator would record nothing — but
the recorder's guard was already that same thread-local before `c3`. The
property is unchanged by this slice.

**What is owed, if anything, is one clause at the new read** stating that the
flag is assumed stable across the merge, so the next person adding a scope
inside a lowering path meets the assumption instead of discovering it. **It
rides the next Runtime candidate that enters `lowering/mod.rs`; do not recut
for it.**

## `c3` — AUTHORIZED 2026-08-14, the closure slice. Both residuals, ~10 lines.

**Steward's call on runtime-leader's question `evt_54vygsecrhvr`** (fold into a
`c2` closure slice, or cut a node). **Neither: it is a slice on this node, and
it is authorized now — no separate node.** `c2` is done, so there is nothing
left to fold into; and a new node for ten lines lengthens the critical path for
a change that touches one file, one feature and one ring. Preferring the fold
over a new node is `§4c`.

**`c3` does not gate [[RT-NESTED-IH-NATIVE-REALIZATION]]**, which flips on `c2`
and is where the critical path actually runs. If Runtime's next turn has to
choose, the successor outranks `c3`.

### The trap, and it is the whole reason this is framed rather than just assigned

**The naive fix deletes rows 4 and 5 — the controls `c2` just landed.**

`dasm_c2_scalar_merge_observation_scope` is gated
`#[cfg(feature = "dasm-c2-observation")]` **alone** (`lowering/mod.rs:16040`),
not `cfg(any(test, …))` like the recorder. An external crate can therefore only
call it when the feature is on. `nc14_data_match_lowering.rs` is in
**`ken-elaborator`** and drives both `D5` controls through that scope — so the
always-on dev-dependency is **what makes those controls compile at all.**

⇒ **Copying the sibling's opt-in shape and stopping there does not "make it
symmetric"; it makes the `D5` seat controls silently absent from a default test
run.** The Adversary's read — *"the one read that decides it is whether any
`ken-elaborator` test needs the observation"* — has an answer, and it is **yes**.

### Deliverables

**`D-c3-1` — make the disabled path free.** Move `observed_operand_kind` and
`observed_constructor` inside the `DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED`
check so a disabled build classifies nothing and clones no constructor name.
One `if` at the call site is sufficient: `LocalKey<Cell<bool>>::get()` is
already used bare at `:15992`, `:16017` and `:16044`.

**`D-c3-2` — resolve the asymmetry in a stated direction.** Either keep
always-on and record why, or move to the opt-in shape **and** carry the
`ken-elaborator` `D5` controls with it so they still run by default. **State
which and why in one paragraph on this node.** Both are acceptable; an
unrecorded choice is not, which was the Architect's actual finding — *"the
trade may be the better half and what is missing is that the trade is
unrecorded."*

**Chosen direction: retain the always-on `ken-elaborator` dev-dependency.** The
external observation scope is feature-gated, and the two D5 seat controls in
`nc14_data_match_lowering.rs` must compile and execute in the default targeted
test invocation. Making the dependency opt-in would require a second default-on
feature carrier merely to preserve that coverage, adding indirection without
reducing the feature set compiled by default tests. The observation therefore
stays always-on for `ken-elaborator` tests, while `D-c3-1` makes its disabled
runtime path free.

### Acceptance

| AC | criterion | control |
|---|---|---|
| `AC-c3-1` | The `D5` seat controls still run in a **default** `-p ken-elaborator` invocation | name the two tests and report them passing with no feature flag on the command line. This is the row that catches the trap above |
| `AC-c3-2` | The disabled path does no work | show both bindings inside the enabled check; a build with the feature on and no scope active classifies no operand |
| `AC-c3-3` | The observation still discriminates | re-run QA's mutation and report it reddening at `admitted: false`, `StructuralNat` count `0 != 1` |
| `AC-c3-4` | The direction chosen in `D-c3-2` is written down | one paragraph on this node giving the reason, not the change |
| `AC-c3-5` | No-regression, in CI | `COORDINATION §12` — the venue is CI, not a local `--workspace` run |

**Forbidden in `c3`:** any change to the `match lowered` arms, to the admitted
merge-shape set, or to the `_ =>` catch-all. `c3` is gating and placement only.
Touching an arm turns a ten-line cleanup into a re-review of `c2`.

## Cited-source hit, and it is the Steward's, not the ring's

`crates/ken-runtime/src/cranelift_backend.rs` is cited in
`library/SOURCE-ATTESTATIONS`, found by the `M3` check on `3e8de4b8`. **It routes
to the Librarian after the merge.** Recorded here only so the next person running
`M3` on this lane is not surprised by it. **The ring does not touch `library/`,
and this is not an AC** (operator ruling, 2026-07-26: the Librarian's
responsibilities are downstream and unobserved for build teams).

## Sequencing

**Runtime's next slice after the current `RT-MATCH-RECURSOR-CONSUMERS` work.**
Do not interrupt a slice in flight for it. `D0` is measurement and does not
contend with `D8`'s pin.

> **That condition is DISCHARGED, 2026-08-14.** `RT-MATCH-RECURSOR-CONSUMERS`
> is `merged`, and so is `RT-LEXICAL-R3-FUSION-EMITTER` (squash `34769380`).
> No Runtime slice is in flight. **Nothing on this node waits on another node.**

### No reverse edge, and the direction is deliberate

`KERNEL-NESTED-IND` `AC-K12` requires native execution, the Cranelift verifier,
and interpreter/native agreement, so that node **cannot close** until this one
lands. **That is an acceptance condition of the Kernel node, not a reverse
implementation dependency**, and `blocks:` stays empty here — the same call
[[RT-TERMINAL-ALL-ELIM-AUTHORITY]] records for the identical shape. Kernel's
`D5` work lands as an accepted partial in the meantime; it does not wait on
this node and this node does not wait on it.

> #### SUPERSEDED IN ITS CONCLUSION, 2026-08-14. The distinction above is
> #### still right; `blocks:` is **not** empty and must not be re-emptied.
>
> **Frontmatter reads `blocks: [KERNEL-NESTED-IND,
> RT-NESTED-IH-NATIVE-REALIZATION]`, and that is the later deliberate call.**
> The paragraph above and the frontmatter have contradicted each other since
> the edge was declared, which is the one thing a reader cannot resolve from
> inside this file.
>
> **Why the edge won.** This node's own 2026-08-12 banner enumerates three
> mechanisms that hid an idle ring on this chain, and **the first is "a missing
> `depends_on` edge."** An edge that is semantically defensible to omit is
> still the thing a tracker reads to decide whether anyone is blocked.
> `blocks:` is the frontier instrument, not a claim about implementation order.
>
> **What survives unchanged:** `AC-K12` is an acceptance condition, Kernel's
> `D5` lands as an accepted partial without waiting, and this node does not
> wait on Kernel. The direction is still one-way. Only the *"stays empty"*
> clause is dead.
