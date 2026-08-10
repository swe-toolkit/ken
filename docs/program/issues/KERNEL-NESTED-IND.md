---
id: KERNEL-NESTED-IND
title: "admit nested strictly-positive inductives in the kernel — structural positivity through declared parameter positions, generated and checked dependent eliminators with one lifted IH per contained recursive occurrence, iota, and surface consumability"
status: active
owner: kernel
size: L
gate: none
depends_on: [SPEC-NESTED-IND, RT-JOIN-ORIGIN-ATTRIBUTION, RT-BODY-OCCURRENCE-PROVENANCE, RT-SPECIALIZED-MATCH-ATTRIBUTION, LANG-NESTED-MATCH-LIFT-ALIGNMENT]
blocks: [DS-9]
github: null
origin: Architect ruling evt_55k9f9efvd8jk, Decision dec_13af1mercv2m0 resolved. Demand-pulled by DS-9, which blocked at its first deliverable on `JsonArray (List Json)`; fork raised by the Steward as evt_1ykvpj7yvtg18. The five-point prerequisite contract below is the Architect's, transcribed verbatim in substance. Steward-filed; Steward owns the frame and AC/control placement.
---

> ## UNHELD 2026-08-09 — THE RUNTIME WALL IS DOWN. RESUME FROM `da917653`.
>
> **The hold of Architect ruling `evt_j8t0ktxbmck` is discharged: all four
> `depends_on` are merged or closed and this node has zero unmerged
> dependencies.** [[RT-JOIN-ORIGIN-ATTRIBUTION]] merged at `72e0d7c` (PR
> #1686); [[RT-SPECIALIZED-MATCH-ATTRIBUTION]] merged;
> [[RT-BODY-OCCURRENCE-PROVENANCE]] closed with every AC discharged —
> production at `1f706520`, `D7` at `1105671a` (PR #1727, CI green).
>
> ### `D5`'s LANE SURFACE IS DEFINED BY `AC-K12`'s STAGES, NOT BY A CRATE LIST
>
> **Steward ruling 2026-08-09, on `kernel-leader`'s spillover question
> `evt_5881mybzdzs0m`.** Fresh `T1` grounding repaired two elaborator defects
> and reached exact dependent-`Omega` acceptance; the valid Nat-3 control then
> elaborates, passes both kernel re-checks, and fails independently at two
> consumers outside `elab.rs`. **Both are authorized spillover in this `D5`
> landing. The acceptance boundary is NOT amended, because `AC-K12` already
> spans them.**
>
> | consumer | disposition | why |
> |---|---|---|
> | `ken-interp::eval::elim_reduce` — legacy direct/Pi-only `recursive_arg_arity` supplies no structured `All` IH for `LiftNode : Bag LiftRose -> LiftRose` | **IN** | `AC-K12`'s own row says *"the evaluator and native-lowering paths **re-derive** recursive positions"* and requires a recursive computation to **evaluate**. Interpreter Nat-3 is a named `AC-K12` stage |
> | `erase_checked_core_package_for_target` — rejects the generated support-family dependent motive as `unsupported_dependent_motive` | **IN** | it is in `crates/ken-elaborator/src`, which this block already rules is *"the work, not an out-of-lane escalation"*. `AC-K12` requires a built artifact, which cannot exist while erasure refuses |
> | `crates/ken-runtime` planner/lowering | **OUT, unchanged** | off-limits to Kernel. The Steward authorized an edit there once and was overruled. If the failure moves there after these two, it is a Runtime attribution and Kernel **stops** |
>
> **This is the same defect class the Architect already ruled, one crate over:
> a legacy direct/Pi-only producer against the kernel's `recursive_shapes`.**
> `recursive_arg_arity` is to `ken-interp` what `recursive_args` was to
> `elab.rs`, so mechanism point 1 — *take the telescope from kernel
> `method_type`/`recursive_shapes`; never reconstruct a nested lift or add a
> second topology rule* — **binds verbatim at both new sites.**
>
> **The one part that is soundness-adjacent, and it is not a scope question.**
> Relaxing `unsupported_dependent_motive` must key on **kernel provenance**
> (`all_support_origin`), never on the motive's shape. A shape-keyed relaxation
> widens erasure for arbitrary dependent motives, which is a TCB-adjacent
> widening this node does not authorize. Ground the discriminator in `spec/`
> first; if `spec/` does not settle it, that single question routes to the
> Architect — not the scope call, which is ruled here.
>
> **Validation:** a change to `ken-interp` evaluation requires the **full
> `-p ken-interp` suite**, not the one focused test. Reifier and `elim_reduce`
> changes have reached distant cases in that crate before.
>
> **One branch, one candidate.** Elaborator, interpreter, and erasure ride the
> same `wp/KERNEL-NESTED-IND-D5` and one Decision. Do not publish the
> elaborator piece alone while the consumers wait — that is the exact assembly
> failure `COORDINATION §14(4)` exists to prevent.
>
> **Contention, re-measured rather than asserted.** No seat holds uncommitted
> edits in `ken-interp/src` or `ken-elaborator/src/erasure.rs`.
> `verify-implementer` holds `crates/ken-interp/tests/l1_acceptance.rs`, but
> Verify is held on the lane cap and that is a **test** file against Kernel's
> `src` — the file intersection is empty. Flagged forward: Verify re-bases onto
> a moved `ken-interp` when `CI-ASSERTIONLESS-L1` resumes.
>
> ### THE LANE SURFACE IS NOT A CRATE LIST. I HAVE MISREAD IT THREE TIMES.
>
> **Three times on this node I have under-described `D5`'s surface** — first
> excluding `crates/ken-elaborator/src`, then attributing the refusal to
> Runtime, now excluding the evaluator. Each time the omitted path was
> **required by `AC-K12`'s own stages**, and each time a ring stopped to ask.
>
> ⇒ **`D5`'s surface is not a crate list. It is: every path an `AC-K12` stage
> traverses, minus `crates/ken-runtime`.** `AC-K12` names lowering, native
> execution, the Cranelift verifier, and interpreter agreement at Nat 3. **Any
> path on the way to those stages is in scope by construction**, and a crate
> list written from the current failure site will always be one consumer short
> of the next one. Do not ask whether a consumer is "in the lane" — ask which
> `AC-K12` stage it blocks.
>
> ### KERNEL'S LANE DOES NOT FREE AFTER `D5`. `D6` IS NEXT.
>
> **Steward ruling, 2026-08-09.** `AC-K12` is blocked at
> [[RT-DYNAMIC-ARM-SCALAR-MERGE]], and the tempting inference —
> *node blocked on Runtime, therefore Kernel is idle, therefore a lane frees* —
> is **false**. Two deliverables remain that native lowering does not gate:
>
> | deliverable | gated on native? | why |
> |---|---|---|
> | `D6` — the four conformance rows of contract point 4 | **no** | every row is admission, rejection, or reduction; the one computation row says *reduces-to `3`*, which is the evaluator, not codegen |
> | `D7` — the `trusted_base()` delta report (`AC-K10`) | **no** | a set-valued accounting over the declaration ledger — see the `AC-K10` metric ruling |
>
> ⇒ **Do not release a lane on the strength of `AC-K12` being blocked.** Verify
> stays held on the two-lane cap; Kernel goes `D5`-partial → `D6` → `D7`.
>
> ## `D6` IS RECUT TO SEVEN CASES 2026-08-10. THE HEADLINE ROW STAYS GATED.
>
> **Architect ruling `evt_2s6gmzqvaj5mr`, executed by the Steward, who owns this
> recut and the separate-node routing.** ⛔ **Kernel must not attempt a fourth
> `D6` candidate until this recut has landed.** Everything below this block
> describing `D6` as eight cases is superseded on that one point; the rest of it
> stands.
>
> ### What three rejections established
>
> `nested-size-uses-lift` requires a fold over **all** `Bag`-indexed Nat leaves.
> **The current surface cannot express it.** In an `All_Bag.Join` method,
> `method_type` already supplies exactly what the fold needs — one recursive
> method result per recursive support field, after the support constructor's
> fields and evidence — but `check_match_with_lift` hides those binders and
> **no term in today's source vocabulary denotes one**. A residual `match` is a
> fresh finite elimination; a source variable denotes the `Bag` value;
> `liftSize xs` is ill-typed at `Bag LiftRose` versus `LiftRose`; helper or
> general self-recursion reconstructs calls instead of consuming the
> kernel-supplied result. No elaborator-only reinterpretation of the existing
> forms is faithful.
>
> Three candidates were built cleanly and rejected cleanly, each moving the
> counterexample exactly one level deeper — `d6a72371` (header only),
> `916af824` (depth two), `d7681153` (depth three,
> `LiftNode (Join (Join (Join (One LiftLeaf) (One LiftLeaf)) Empty) Empty)`
> returns `1` where the fold requires `3`). ⇒ **Successive repairs defeated the
> same way by one checker mean the default branch is wrong.** The wrong default
> was the Steward's *"`D6` is a binding task"* framing directly below: it was
> written from a measurement that the behaviour was covered and only provenance
> was missing. **The behaviour is not covered.**
>
> ### The recut — this is `D6` now
>
> | change | detail |
> |---|---|
> | bind **seven** cases | the contract-point-4 subset minus `nested-size-uses-lift` |
> | **restore** `[KERNEL-NESTED-IND]` on `nested-size-uses-lift` | and **remove its claimed exact executing binding**. It is blocked on the recursive-result surface node, and the seed row must say so |
> | seed marker census **19 → 14** | ⚠ **corrected 2026-08-10, see the block below — an earlier version of this row said `14 → 15` and was wrong in both direction and target.** `nested-size-uses-lift` keeps the marker it always had; nothing is restored |
> | the shallow and finite-depth controls | **may remain, only if labelled partial topology/association regressions.** ⛔ They may not be cited as the unbounded fold, nor as discharge of `nested-size-uses-lift` |
> | the other seven bindings and their evidence | preserved, but **only after fresh scope/QA/review** — no prior verdict transfers |
>
> ⛔ **`dec_8pyjkfs3qv7m` and its whole lineage are spent.** No QA, Architect,
> or conformance-validator vote from any earlier `D6` SHA transfers.
>
> ### THE CENSUS NUMBER WAS WRONG. `14` IS CORRECT. Measured 2026-08-10.
>
> **This row blocked a correct candidate**, so read it before using any count
> from this node as a check.
>
> | object | `^### .*[KERNEL-NESTED-IND]` in `seed-nested.md` |
> |---|---|
> | `main` `d2da54f8`, the true baseline | **19** |
> | correct post-recut census | **14** |
>
> Of the eight contract-point-4 cases, only **six** ever carried a heading
> marker — `nested-negative-existing-pair-control` and
> `nested-direct-and-wstyle-controls-unchanged` were unmarked before any `D6`
> work. Un-gating seven therefore removes **five** markers, and
> `nested-size-uses-lift` keeps the sixth. 19 − 5 = **14**.
>
> **Where `14 → 15` came from.** The ruling's arithmetic assumed the earlier
> candidates had *removed* size's heading marker, so restoring it would add one
> back. **They never removed it** — they bound the row with an executing test
> while leaving the marker in place, which is why they reported 14 from a
> baseline of **19**, not from a baseline of 14. The baseline was misidentified,
> so both the direction and the target were wrong. Nothing is restored; size
> stays gated because it never stopped being gated.
>
> ⇒ **A candidate reporting `14` HAS done the recut.** The earlier text here said
> the opposite and was used as a closure check, which is how it blocked
> `d9b1d5b1` on its one correct property.
>
> **The lesson, stated where it will be read rather than in a retro:** the
> Steward asserted this number without measuring it and then wrote it in as *"the
> cheapest single check for whether this landed correctly."* **Promoting an
> unmeasured number to a gate makes it authoritative against the artifact.**
> Verify a count against the object before any node states it as a criterion —
> the check is worth exactly as much as the measurement behind it.
>
> ⛔ **Do NOT widen `D6` to invent the missing surface.** Two repairs are
> specifically prohibited: making hidden binders visible through `surface_var` or
> as ordinary constructor-pattern fields, which changes source constructor arity
> and exposes kernel-internal support topology; and coercing an ordinary field
> reference or an owner self-call into the IH, which either changes a field's
> source type contextually or admits a call that is neither a direct
> guest-motive instance nor SCT-justified.
>
> ### The capability is a separate node
>
> [[KERNEL-RECURSIVE-RESULT-SURFACE]]. A lawful mechanism exists and the
> Architect has approved its shape, but it is **a new explicit surface
> capability**, not a bounded `D6` repair — the spelling, scoping, diagnostics,
> and interaction with ordinary direct/W-style matches need a Spec-owned
> contract before any implementation frame. The kernel and the generated `All`
> representation need no change.
>
> **Gating a conformance row for a capability that does not exist is the correct
> outcome, not a failure.** What was wrong was claiming an executing binding for
> it.
>
> **`D6` IS A BINDING TASK, NOT A TEST-AUTHORING TASK. Size it accordingly.**
> ⚠ **True for the seven remaining rows; it is exactly what was FALSE for
> `nested-size-uses-lift`.** Read the recut block above before this table.
> Most of the contract-point-4 *behaviour* is already covered by landed kernel
> tests — what is missing is the **binding to the conformance corpus**:
>
> | contract point 4 clause | already covered on `main` by |
> |---|---|
> | positive nested admission | `nested_inductives_remaining.rs::declared_positive_paths_admit_list_pair_and_fresh_container_nesting` |
> | the three rejections, **separately** | `nested_inductives_remaining.rs::nested_negative_unknown_and_non_positive_paths_reject_separately` |
> | a real recursive computation | `production_nested_lift_is_consumed_and_iota_computes`, plus `D5`'s interpreter Nat-3 |
> | direct and W-style unchanged | `k1p5_wstyle.rs`, green and untouched (`AC-K8`) |
>
> ⇒ **The gap is provenance, not coverage.** `k1p5_wstyle.rs` opens with a `//!`
> line binding it to `conformance/kernel/inductive/seed-wstyle.md` AC1–AC5.
> **No file in `crates/` references `seed-nested.md` at all** — verified by grep
> over `crates/` and `scripts/`; `k1p5_wstyle.rs` is the only conformance
> binding in the tree. So the corpus's cases stay `[KERNEL-NESTED-IND]`-gated
> while the behaviour they describe is landed and green, and **nothing detects
> the divergence** — the gate marker and a genuinely unimplemented case read
> identically.
>
> `D6` therefore is: bind the executing tests to the named seed cases, fill the
> rows that have no executing test, and drop the `[KERNEL-NESTED-IND]` gate
> marker on exactly the contract-point-4 subset. ⚠ **Do not assume the four
> clauses are fully covered because the table above is dense** — the table is
> keyed on the clause, and a seed case names specifics (`Bag`/`Rose` carriers,
> the `Deep` composition chain) that a landed test keyed on `List`/`Pair` may
> not exercise. Check case by case.
>
> The four clauses map to these seed cases:
>
> | contract point 4 clause | seed case(s) | AC |
> |---|---|---|
> | positive nested Rose-style with a real recursive computation | `nested-size-uses-lift`, `nested-ds9-shapes-admitted`, `nested-fresh-carrier-admitted` | `AC-K1`–`AC-K4` |
> | retained nested-negative rejection | `nested-negative-under-positive`, `nested-negative-existing-pair-control` | `AC-K5` |
> | retained rejection through unknown or non-positive | `nested-unknown-head-rejected`, `nested-nonpositive-rejected` | `AC-K6`, `AC-K7` |
> | direct and W-style unchanged | `nested-direct-and-wstyle-controls-unchanged` | `AC-K8` |
>
> **The implementation pattern already exists as a sibling.**
> `crates/ken-kernel/tests/k1p5_wstyle.rs` implements
> `conformance/kernel/inductive/seed-wstyle.md`'s AC1–AC5. There is no automatic
> harness that reads these markdown seeds — `conformance/README.md` records the
> harness as an open question — so `D6` is a Rust test file mirroring that
> sibling, ⛔ **not** a new harness. Building one is out of scope and would be
> the mechanism expansion `§4c` exists to stop.
>
> ⚠ **`D6` is the contract-point-4 subset, NOT all of `seed-nested.md`.** The
> seed carries roughly eighteen cases across `AC1`–`AC5`, including the
> sort/level and transactional-rollback soundness rows. Those belong to
> `SPEC-NESTED-IND`, which the seed itself names as its frame. ⛔ Do not absorb
> them into `D6` because they sit in the same file.
>
> **`D5`'s delivered capability feeds `D6`'s headline row directly.**
> `nested-size-uses-lift` expects
> `size (node (join (one leaf) (one (node empty))))` to reduce to `3` — the same
> `LiftRose`/`Bag` Nat-3 result `D5`'s interpreter path now computes.
>
> ### THE NEXT SLICE IS `D5`. THE POSITION IS FOUR DELIVERABLES FURTHER ON.
>
> **Architect ruling `evt_3cnnt1megm88h`, 2026-08-09, and it is the authority
> here.** Two earlier resume instructions of mine in this block were wrong and
> are **withdrawn**: *re-run the `AC-K12` differential*, and its replacement
> *resume at `D3a`*. Both rested on the premise that `D3a` and `D1b` were
> unbuilt. **That premise is false on this node's own parent commit.**
>
> **What is actually landed**, each verified an ancestor of `origin/main`:
>
> | commit | deliverable |
> |---|---|
> | `88196527` | `D1a` — per-parameter polarity derived at admission |
> | `ac86b2d7` | `D3a` — exhaustive recursive-shape descriptor, inert, with `AC-K15` invariance |
> | `433dd12b` | `D3b`+`D4` atomic — structured descriptor with universe-level transport |
> | `afb38934` | **`D1b` — production nested admission is OPEN** |
>
> `afb38934` rewrote `check_pos_arg` from the blanket non-`D`-head guard to
> traversal through a former's recorded `StrictlyPositive` positions, and added
> `declared_positive_paths_admit_list_pair_and_fresh_container_nesting` and
> `production_nested_lift_is_consumed_and_iota_computes`. Its commit subject
> names only the terminal-All source relation, which is what made it easy to
> read as a `D1a` partial. **Read the node's landed record, not a commit
> subject, and not the frame's plan.**
>
> ### The refusal is the remaining `D5` elaborator boundary
>
> ⚠ **SUPERSEDED 2026-08-09 — the producer split described below is FIXED, and
> the boundary moved.** `D5`'s WIP `51c482a5` closed it: the elaborator now takes
> the method telescope from kernel `method_type`/`recursive_shapes`, the
> interpreter evaluates the `LiftRose`/`Bag` Nat-3 case to `3`, and
> checked-artifact erasure admits the generated support `Elim` gated on
> `all_support_origins`. **The `TypeMismatch` at
> `nc14_data_match_lowering.rs:136` no longer reproduces, and the test that
> reported it has been renamed.** The block is retained because the mechanism
> section under it is still the ruled design — read it as *how `D5` was built*,
> ⛔ not as a live failure to reproduce.
>
> **The live boundary is now Runtime's**, one stage further on: a carried `Match`
> arm refuses at `merge_scalar_operand`
> (`ken-runtime/src/cranelift_backend/lowering/mod.rs:15898`) with *dynamic arms
> must produce scalar Int or Bool values*. That is
> [[RT-DYNAMIC-ARM-SCALAR-MERGE]], not this node, and Kernel may not edit it.
>
> The original, now-historical statement of the split follows.
>
> `nested_recursive_field_lowers_and_native_agrees_at_nat_three` failed at
> `ken-elaborator/tests/nc14_data_match_lowering.rs:136` with `KernelRejected
> TypeMismatch`. **The source got past declaration admission; its first
> failure was method checking**, exactly where the error location said.
>
> It is a **producer split**, verified at `origin/main`:
>
> - `ken-elaborator/src/elab.rs:1252` derives hidden method binders with legacy
>   `recursive_args(ctor, d_id, m)`, which returns **no entry** for
>   `LiftNode : Bag LiftRose -> LiftRose`.
> - `ken-kernel/src/inductive.rs:2101` builds the checked method type from
>   `recursive_shapes`, so it requires
>   `Π (b : Bag LiftRose). All^Type_{Bag,0} (λr. M r) b -> ...`.
>
> The elaborator emits the `b` method without the lifted binder. The reported
> expected-`Π All` / found-body mismatch **is** that split made visible.
>
> ### The `D5` mechanism, as ruled — `crates/ken-elaborator/src/elab.rs`, kernel unchanged
>
> 1. Take the complete constructor-method telescope from kernel
>    `method_type`/`recursive_shapes`. **Do not** reconstruct a nested lift with
>    `recursive_args`, and do not add a second elaborator topology rule.
> 2. For each nested source field, carry the corresponding hidden,
>    literally source-indexed `All` binder as an elaboration-context pair
>    `(source value, lift evidence)`.
> 3. When source code matches that field, lower the source match and its
>    kernel-generated `All` inhabitant **in lockstep**, validating support
>    through kernel provenance (`all_support_origin`) and aligned evidence
>    positions (`all_support_evidence_positions`). An exposed recursive child
>    receives its exact motive instance; an exposed enclosing child retains its
>    residual `All` inhabitant.
> 4. A structural self-call on an exposed child consumes that motive instance.
>    Do not emit an unrestricted recursive call, expose generated support names,
>    add equality/transport, or loosen `ken-kernel`.
> 5. Kernel-check the completed `Elim` unconditionally. Valid Nat-3 and
>    dependent-`Omega` lockstep cases must accept; a missing, swapped, or
>    foreign evidence association must still reject; direct and W-style methods
>    remain unchanged.
>
> Normative source: `spec/30-surface/39-elaboration.md §2.2 item 5` and
> `spec/30-surface/34-data-match.md §3.1`. **Implement the contract afresh** —
> do not cherry-pick or revive rejected `e8cdc8b9`; retain its durable RED as
> the adversarial input.
>
> ### Scope: `crates/ken-elaborator/src` is in scope and is where `D5` lands
>
> An earlier line in this block described the lane surface as `ken-kernel` plus
> `ken-elaborator/tests`. **Withdrawn** — it was measured off the retained
> snapshot branch rather than the node. `D5` is *surface consumability:
> matching, elaboration, structural-recursion/termination*, and the ruled
> mechanism is in `crates/ken-elaborator/src/elab.rs`. Still disjoint from
> Runtime's `ken-runtime`/`ken-cli`, so the concurrent lane is unaffected.
>
> **On the retained branches:** `wp/KERNEL-NESTED-IND-relation-partial` rebases
> to an empty tree — both its commits are already upstream in `afb38934`.
> `wp/KERNEL-NESTED-IND` still holds the full snapshot and the durable RED.
> Nothing there is owed forward.
>
> ### One thing the unblock does NOT change
>
> **`AC-K12` still requires lowering AND evaluation.** The merged contract
> requires the nested-IH constructor to reach native execution, pass the
> Cranelift verifier, and agree with the interpreter at Nat 3. **A production
> native refusal cannot be the terminal positive control** — that was true
> while held and it is true now.
>
> **`crates/ken-runtime` stays off-limits to Kernel** — the Steward authorized
> an edit there once and was overruled; Runtime owns the planner/lowering
> invariant.
>
> **But this refusal is NOT Runtime's**, and the earlier wording here — *"a
> surviving refusal is not yours to fix, report it as a fresh Runtime
> attribution finding"* — is **withdrawn** by `evt_3cnnt1megm88h`. The
> `AC-K12` differential's first failure is method checking on the elaborator
> side of the producer split, inside this node's own `D5`. Repairing it in
> `crates/ken-elaborator/src` is the work, not an out-of-lane escalation.
>
> ## RE-RELEASED 2026-08-09 — AND THE RELATIONAL ZIPPER IS NOT THE ROUTE
>
> **The Spec representation contract merged at exact `c7f8913c` (PR #1678, CI
> green including conformance), and Kernel is re-released on that basis.**
>
> ### READ THIS BEFORE RESUMING: a superseded instruction is still in the thread
>
> **Architect ruling `evt_1d8dczzb9ts7h` told the ring to build a paired-decoder
> RELATIONAL ZIPPER (`decode_ty_F` / `decode_tm_F` as separate eliminators).
> THE MERGED CONTRACT SUPERSEDES THAT MECHANISM. Do not build it.** The ring
> stopped before building it, which is why nothing is wasted — but **a seat
> resuming from the last instruction it received would build the wrong thing**,
> and that instruction sits *above* the superseding one in the same thread.
>
> ### What the contract settled
>
> Before guest methods exist, `method_type` names the **intrinsic,
> source-indexed `All^Type` / `All^Omega` application** derived from the motive
> and the original host source. After the complete guest method vector exists,
> `lift-elim_D` constructs an inhabitant of **that literal same `All P v` type**
> — guest elimination at leaves, host elimination / IHs at host children.
>
> ⇒ **The neutral public obligation no longer requires converting between two
> host eliminators.** That conversion was the entire reason a zipper was
> proposed, so the ordering answer removes the mechanism rather than
> implementing it. **This resolves the ring's stop** — the joint decoder could
> not inhabit the public method-independent lift *before methods exist*, and the
> contract makes that ordering coherent instead of working around it.
>
> ### Still binding, none of it discharged by the merge
>
> Generated **closed telescopes** and exact `Type`/`Omega` behaviour; terminal
> **first-order support only (2p)**; private composed-support use; **atomic
> host-plus-support admission/rollback**; host-rank termination; **frozen
> 3-Decl / 6-ID / 2-edge carrier**; **zero `trusted_base()` delta** with audited
> generator/transaction/iota TCB; surface lockstep.
>
> ### ONE CARRY FROM THE SPEC RETROS — it bears on the frozen-carrier constraint
>
> **Generated kernel support needs TWO separately grounded closures before an
> exact oracle is frozen**, and conflating them is how a carrier oracle gets
> frozen against the wrong set:
>
> 1. **finite provenance / generation** — host versus terminal support, and
>    private edges;
> 2. **the actual published carrier** — declarations, embedded constructor
>    records, IDs, graph edges, derived term forms.
>
> ⇒ This is the practical shape of the **frozen 3-Decl / 6-ID / 2-edge carrier**
> constraint below. **Ground both closures separately**; a single closure that
> looks like it covers the carrier is the failure mode.
>
> ### Forbidden, all six
>
> Restoring method-dependent binders; adding equality/transport or a conversion
> axiom; decorated containers; relaxing checking; narrowing admission; and
> treating the merged spec as implementation approval.
>
> **`e8cdc8b9` REMAINS REJECTED** and the durable RED stays preserved until a
> fresh candidate replaces it. No admission-only or test-only partial.
>
> **The contract merge discharges the REPRESENTATION STOP ONLY, not the
> implementation gates** — fresh exact SHA, fresh QA, fresh Architect review,
> new Decision, then a publish request.

> ## ⛔⛔ `D1b`/`D2` GATE (2026-07-28) — THE POLARITY PRODUCER IS FAIL-OPEN
>
> ⛔ **Fail-open on three of the four positions the record claims to cover.**
>
> **Authority:** Architect ruling `evt_3edf99cq5mrka` and merge Decision
> `dec_b1hj6th3363a` (resolved APPROVE), on the adversary finding
> `evt_79m7a5y9d1b4g`. ⛔ **This is a gate on `D1b`, not a reminder.**
>
> > ⛔ **`D1b` MUST NOT open production nested admission until polarity
> > derivation is FAIL-CLOSED over all four positions:** constructor
> > **arguments**, constructor **target indices**, **inductive indices**, and
> > **dependent parameter types.**
>
> **What is actually wrong on `main`.** `derive_parameter_polarities` scans only
> `constructor.args`, while `derive_recursive_shape` admits a nested recursive
> `Former` **only** on a recorded `StrictlyPositive`. ⇒ A negative occurrence in
> any of the other three positions is **recorded positive**, and the adversary
> demonstrated that target-index placement flips the permissive gate from
> **reject** to **accept**. ⭐ `D1a` shipped the record; it did not ship coverage
> of the positions the record claims to summarise.
>
> ### ⭐ WHY THIS IS NOT ALREADY A DEFECT — read this before re-deriving it
>
> ⚠ **The staged `D3b`+`D4` slice was audited against this finding and cleared**,
> so a reader who sees only the gate above must not conclude the slice was never
> examined. The clearance is narrow and rests on **two** facts, both of which
> `D1b` destroys:
>
> 1. `D3b`+`D4` reaches the nested-`Former` controls **only** through the
>    explicitly test-only `env.add_decl` fixture
>    (`install_test_only_nested_family` and its polymorphic sibling), which is
>    ⛔ **not a production admission route**; and
> 2. the production-live primitive-`Sigma` path does **not** consult another
>    former's parameter-polarity record.
>
> ⇒ **No declaration capable of connecting the malformed record to the new
> semantic method/iota consumers is admissible.** ⭐ **`D1b` is precisely the
> change that makes such a declaration admissible** — at that moment the same
> false record becomes an **executable soundness boundary**, not a latent one.
>
> ⛔ **Do not read "`D3b`+`D4` is staging-safe" as "the polarity record is
> sound."** The record is unsound now; it is merely unreachable. The semantic
> consumers already exist — only the admission route is missing.
>
> ⚠ **A `D1b` implementer's first act is to establish coverage over all four
> positions and a control that discriminates each**, before any admission
> widening. A control that only exercises constructor arguments passes on the
> landed producer and says nothing about the other three.

> ## ⭐ `D1a` + `D3a` + the ATOMIC `D3b`+`D4` HAVE ALL LANDED.
>
> | slice | PR | landed | evidence |
> |---|---|---|---|
> | `D1a` — per-parameter polarity | #1077 | `88196527` | `inductive.rs` `e37e906f`, `nested_inductives_d1a.rs` `280025f1` |
> | `D3a` — recursive-shape descriptor, inert | #1089 | `ac86b2d7` | `inductive.rs` `d6ab179c`, `nested_inductives_d3_shapes.rs` `33a3efbf`; +858/−1 |
> | `D3b`+`D4` — atomic, descriptor consumed | #1162 | `433dd12b` | `check.rs` `a133f025`, `conv.rs` `d338988f`, `inductive.rs` `79a85c6e`, `k1p5_wstyle.rs` `4b9784a6`, `nested_inductives_d3b_d4.rs` `2cff84fc`, `compiler_driver.rs` `85d45d92`, `b2_acceptance.rs` `14c6e5c7` |
>
> All CI-green; all verified by **blob identity** with discriminating pre-merge
> controls. `D3a` took **six** candidates — five rejected objects preserved as
> ancestors on `wp/KERNEL-NESTED-IND-D3`, none rewritten.
>
> ⛔ **The node is NOT complete** — but the sentence that used to stand here was
> **STALE IN THE DIRECTION THAT CHANGES WHAT YOU DO**, so read the correction
> before the table. It said *"`D1b`, `D2`, `D5`, `D6`, `D7` remain"* and *"a
> nested inductive is still rejected on `origin/main`"*. ⛔ **Both are false as
> of 2026-08-09**, and the second would send a reader to re-open admittance that
> is already open.
>
> **Corrected, verified against the code on `origin/main` rather than against
> either prose block in this file:**
>
> | deliverable | state | verified by |
> |---|---|---|
> | `D1a`, `D3a`, `D3b`, `D4` | in | the table above |
> | `D1b` — nested admittance OPEN | **in**, `afb38934` | `inductive.rs` `check_pos_arg` traverses recorded `ParameterPolarity::StrictlyPositive`; `declared_positive_paths_admit_list_pair_and_fresh_container_nesting` |
> | `D2` — fail-closed unknown / non-positive | **in** | `nested_inductives_remaining.rs::nested_negative_unknown_and_non_positive_paths_reject_separately`, which reds the three reasons **separately** as `AC-K5`–`AC-K7` require |
> | `D5` — surface consumability | accepted partial in flight; native stage blocked at [[RT-DYNAMIC-ARM-SCALAR-MERGE]] | |
> | `D6`, `D7` | remain | see the successor ruling at the top of this file |
>
> ⇒ **Six of eight are in and a nested inductive is ADMITTED on `origin/main`.**
>
> ⚠ **Why this went stale invisibly, and the lesson for the next editor.**
> `afb38934`'s commit subject is *"issue the terminal-All source relation
> (accepted partial)"* — it names one part of what it did and never says
> `D1b`. The node's status stayed `active` correctly throughout, so no tracker
> check could see the drift, and **this file simultaneously contained the
> correct claim** (the landed-record table near the top, which does attribute
> `D1b` to `afb38934`) **and the false one, roughly three hundred lines apart.**
> A grep for either statement finds a true sentence. ⛔ **Verify a
> remaining-work claim against the code, never against a sibling paragraph.**
>
> ⭐ **What `D3b`+`D4` actually bought** (Decision `dec_b1hj6th3363a`, resolved
> APPROVE): the structured recursive-shape descriptor is consumed **atomically**
> by `method_type` **and** `iota_reduct`, primitive dependent-`Sigma` topology is
> preserved, `Former` evidence is built by the admitted host eliminator, and level
> arguments are transported from the **normalized actual host head** plus the guest
> instantiation. ⇒ `AC-K14` is satisfied in the strong form it demanded: **no
> commit exists in which a generated method binder carries a lift that ι does not
> construct.** ⛔ Admission is **not** widened — the nested-`Former` fixtures are
> explicitly test-only and production declaration still fails closed.
>
> ⇒ **Next is `D1b`, and it is GATED** — see the polarity block at the top of this
> file; that gate is a hard prerequisite, not a reminder. ⚠ `D1b`'s *external*
> gate lifted when `SPEC-NESTED-IND` merged; ⛔ that did **not** reorder the work
> and it does **not** discharge the polarity gate, which is newer.
>
> **`D1a` = per-parameter polarity, derived at admission and consumed by the
> positivity gate.** Candidate `e685570c1b8403c38af7ed0f45c205a6bc2eeb90`, **CI
> checks passed**, five `ken-kernel` paths, +463/−2. Verified by blob identity:
> `src/inductive.rs` `e37e906f`, `tests/nested_inductives_d1a.rs` `280025f1`, with
> discriminating controls at `b5c448d1`. Decision `dec_3k5rnnx0e04nz` read
> `resolved` from the object (07:39:48Z).
>
> ⭐ **`SPEC-NESTED-IND` merged (PR #1076), so the `D1b` gate is lifted.** The
> governing chapter is `spec/10-kernel/14-inductive.md` blob **`4dab9d0e`** on
> `origin/main` — ⛔ re-bind from the object, never from a worktree copy.
> ⚠ **Un-gating `D1b` does not reorder the work:** frame §4 still puts `D3` first,
> because `D1b` opens admittance and is the change that yields the inert outcome
> with nothing red to say so.
>
> ### ⚠ Three candidates, two rejections, and the fault was in this frame
>
> | candidate | Decision | outcome |
> |---|---|---|
> | `83d6a7c3` | `dec_3g5qg6f9hzge5` | **rejected** — `Pol::Minus` used for "unknown" is not absorbing (`Minus.flip() == Plus`), so a nested `Pi` laundered it positive |
> | `6103d321` | `dec_2r7xykp0aswe5` | **rejected** — the producer was not total: `declare_inductive` **panicked** on an accepted field type, violating `18 §4` |
> | `e685570c` | `dec_3k5rnnx0e04nz` | **resolved** — landed |
>
> ⛔ **Both rejections landed on an axis this frame never named.** `D1a` as
> originally written specified a polarity notion *derived, recorded, readable* —
> all three about the **record** — and `AC-K11` guarded **consumption**. Nothing
> specified the **producer**, so no control existed that could have failed.
> `AC-K13` is that specification, added after the fact.
>
> ⭐ **`AC-K13` was then discharged by closing the class, not the instance:** Pi,
> Sigma, Lam and Let are `Term`'s complete syntactic binder set, each given a
> depth-aware arm, and every fallback-traversed `children()` form adds no binder —
> *"thus no differing-depth fallback edge remains."* That is the standard for
> `D3`–`D5` as well.

> ## ▶ THE KERNEL HALF OF A TWO-STAGE PREREQUISITE
>
> **Frame:** [`kernel-nested-inductives.md`][f], under `docs/program/wp/`. The
> frame is the executable artifact — measured substrate, slicing order, control
> recipes, validation set, contention. This node carries the contract and the ACs.
>
> **Sequence:** `SPEC-NESTED-IND` → **`KERNEL-NESTED-IND`** → `DS-9`.
>
> ✅ **`D1a` is landed and `SPEC-NESTED-IND` has merged, so the `D1b` gate is
> lifted** (see the banner above for the evidence). `D1a` was released alone
> because it **admits nothing new** — the nested declaration stayed rejected
> throughout, making the inert outcome unreachable while the rule was still being
> written. ⚠ That gate is now discharged; ⛔ the **slicing order is not**. Frame §4
> still puts `D3` before `D1b`.
>
> ⚠ **This node changes the TCB.** Read `docs/PRINCIPLES.md` on the small
> auditable trusted base before slicing it.

## Why this exists

`DS-9` blocked at `D1` on `JsonArray (List Json)` — the `List (Rose A)` class that
`spec/10-kernel/14-inductive.md` §8.5 **deferred at the time** (it now states the
nested rule; `SPEC-NESTED-IND` merged 2026-07-27). The Architect ruled **B,
nested-only**: preserve DS-9's ordinary six-constructor `Json` and lift the
kernel restriction, rather than re-encode the value model.

⭐ **The rejection being lifted is sound, not broken.** Architect, verbatim: *"The
present rejection is a safe, deliberate completeness/staging boundary, **not an
unsound kernel result**."* This node adds capability; it does not fix a bug.

## ⛔ SCOPE — NESTED ONLY, and the exclusion is load-bearing

**Architect, verbatim:** *"Do **not** bundle mutual inductives. Mutual families are
a distinct extension, are not required by DS-9, and would enlarge the trusted
change without present demand."*

⛔ Mutual is **out**, and the landed spec now says so in its own place: `14 §8.5`
is *"Nested inductives — structural parameter polarity"* and **`14 §8.6` is
*"Mutually-defined inductives — still deferred"***, with its own reason
(simultaneous-block positivity, jointly generated eliminators, joint termination,
no present consumer). ⚠ Before `SPEC-NESTED-IND` merged, one §8.5 clause deferred
both, so "un-defer §8.5" read as both; that ambiguity is now removed in the text.
⚠ If a slice finds mutual machinery falling out for free, that is **not**
authorization to land it; bring it back to the Steward as a separate node.

## ⛔ THE FIVE-POINT CONTRACT — complete only when ALL FIVE hold

Transcribed from `evt_55k9f9efvd8jk`. ⛔ Not a summary — these are the completion
conditions.

1. **Positivity is structural through declared strictly-positive type-parameter
   positions**, sufficient for **both** `List Json` **and**
   `List (Pair String Json)`. Unknown and negative positions **fail closed**.
   ⛔⛔ **There is NO `List` name allow-list.**
2. **The kernel generates AND checks the dependent eliminator**, with **one lifted
   induction hypothesis for every contained recursive `Json`**, and the
   corresponding **iota reductions**. ⛔⛔ *"Merely deleting or relaxing the current
   `occurs` guard is **not delivery**: that would admit the declaration without
   supplying sound recursion/proof machinery."*
3. **Surface matching, elaboration, and structural-recursion/termination checking
   can consume those lifted hypotheses**, so that a theorem over the array and
   object branches is **actually writable**.
4. **Conformance** includes: a **positive** nested `List`/Rose-style declaration
   **with a real recursive computation or proof**; a retained **nested-negative
   rejection**; a retained **rejection through an unknown or non-positive
   parameter**; and evidence that **direct and existing W-style inductives are
   unchanged**.
5. ⛔ **No new axiom, postulate, trusted escape, or library-side representation
   workaround** enters the solution.

## ⭐⭐ The anti-pattern point 2 exists to forbid — read this before slicing

The cheap version of this node is: find the `occurs`-guard (§8.2, cited at
`14-inductive.md:569-570`), relax it so the declaration is admitted, watch
`data Json = ... | JsonArray (List Json)` type-check, and report success.

⛔ **That is explicitly not delivery**, and it is worse than nothing: the
declaration would be admitted with **no sound way to induct over it**, so the
first person to try proving anything about the array branch discovers the gap —
after the TCB already grew.

⭐ **This is structurally the same rule as hard-stop `#11`'s inertness clause**
(`RT-FNSPLIT-C1`): *a prerequisite may be inert only in the sense that production
routing has not switched to it yet; its producer → validator → eliminator edge
must nevertheless be real and executable.* Here the edge is **declaration →
eliminator + IH + iota → a writable theorem**. Point 3 is what makes the far end
of that edge observable.

⇒ **The AC that discharges this node is `AC-K3`**, not `AC-K1`.

## ⭐⭐ MEASURED SUBSTRATE — and it makes contract point 1 bigger than it looks

Measured at `origin/main = 10b2f56a`, every citation re-verified to resolve.
⚠ Re-derive before starting; these line numbers move.

**The single line that rejects nesting** — `crates/ken-kernel/src/inductive.rs`,
inside `check_pos_arg` (`:86`, the `14 §8.2` judgment):

```rust
Term::IndFormer { .. } | Term::Const { .. } | Term::Constructor { .. } | Term::Var(_) => {
    // `C u` with a non-`D` head: recurse into the (atomic) head
    // and `occurs`-guard every argument.
    check_pos_arg(d, pol, &head) && args.iter().all(|x| !occurs(d, x))
}
```

For `List Json` the head is `List` and the args are `[Json]`, so
`!occurs(Json, Json)` is `false` and the declaration is rejected. **That
`args.iter().all(|x| !occurs(d, x))` is the whole mechanism.**

⛔⛔ **And this is precisely why "relax that line" is not delivery.** Replacing it
with `args.iter().all(|x| check_pos_arg(d, pol, x))` would admit `List Json`
**today**, in one line, with no eliminator, no lifted IH, and no iota — the exact
inert outcome contract point 2 forbids. ⚠ **Expect this to be tempting: it is a
one-line diff that makes the blocked declaration type-check.**

### ⭐⭐ `recursive_args` returns `[]` for a nested arg — SILENTLY

**This is what makes contract point 2 mechanically precise.** `recursive_args`
(`inductive.rs:183`) is the single producer of *"which arguments are recursive
and what IH does each need."* It peels Π binders, peels the application spine,
and fires only when the head **is** the family. For `JsonArray (List Json)` the
head is `List`, so the arm never fires and it returns `[]`.

⛔⛔ **`[]` is not an error — it is the correct answer for `JsonNull`.** So with
`check_pos_arg` relaxed and nothing else: the declaration is **admitted**;
`method_type` (`:211`) generates the `JsonArray` method with **zero IHs**;
`check.rs:555` **accepts** that method type; `iota_reduct` (`:339`) **fires**;
and **every existing test stays green**. ⇒ A `Json` that can be declared,
constructed and matched but **not inducted over**, with the TCB already grown and
no red test anywhere.

⚠ **The return type cannot express a nested occurrence.** Its triple says *"arg
`pos` has type `Π tel. D params idxs`"*; a nested occurrence puts the recursive
occurrences **inside a container**, so the IH must be **lifted through** it. ⇒
`D3` widens a public API with consumers in three crates — and per the frame's
census, `sct.rs:241` and `ken-interp` `eval.rs:557` **re-derive** this test
rather than calling it, so they will not follow. Frame §2c–§2d.

### ⭐ The machinery contract point 1 requires DOES NOT EXIST YET

The ruling requires positivity *"structural through **declared** strictly-positive
type-parameter positions"*, with unknown and non-positive positions failing
closed. To honour that, the kernel must be able to ask *"is `List`'s first
parameter declared strictly positive?"* — **it cannot.**

| measured | consequence |
|---|---|
| `InductiveDecl` (`crates/ken-kernel/src/env.rs:144-159`) carries `params: Vec<Term>` — parameter **types** only, **no polarity** | there is nowhere to read a declared parameter polarity from |
| `Pol` (`inductive.rs:43-46`) is a **private**, two-valued enum used only *within* one `check_pos_arg` traversal | polarity is a transient of the check, not a recorded property of a declaration |

⇒ **A per-parameter polarity notion — computed at admission, recorded on the
declaration, and consulted when checking a nested occurrence — is a deliverable
of this node, not a given.** `D1` is written accordingly.

⚠ **This is also what makes `AC-K2`'s control meaningful.** Declaring a *new*
container and nesting `Json` in it must work with **no kernel change** — which is
only possible if polarity is derived from the container's own declaration. If
`AC-K2` requires a code change, the implementation has hardcoded a set of known
containers, which is the allow-list the ruling forbids.

## Deliverables

- **`D1a`** — ⭐ **the missing machinery**: a per-parameter polarity notion for an
  inductive family — derived at admission, recorded on the declaration, and
  readable when checking a nested occurrence. ⚠ Sizing input: this does not exist
  today (see the substrate section), so `D1` cannot be a local edit to
  `check_pos_arg`.
  ⛔ **AND the producer must be TOTAL over every accepted constructor field
  type** — a polarity record or a *rejection* for each, and **never a panic**
  (`18 §4`: the kernel contract is yes/no, never a crash). ⚠ This clause was
  **added 2026-07-27 after two consecutive Architect rejections landed on it**;
  the original three properties (derived / recorded / readable) are all about the
  *record* and say nothing about the *producer*. See `AC-K13`.
- **`D1b`** — structural positivity through those declared strictly-positive
  parameter positions, replacing the blanket nested rejection at
  `inductive.rs` `check_pos_arg`'s non-`D`-head arm. ⛔ Keyed on **declared
  parameter polarity**, never on a type-constructor name.
- **`D2`** — fail-closed handling for unknown and non-positive parameter
  positions.
- **`D3`** — eliminator generation extended: one lifted IH per contained recursive
  occurrence, extending §3.1's Π-abstracted-IH machinery.
- **`D4`** — the matching iota reductions, and the kernel **checks** the generated
  eliminator rather than trusting it.
- **`D5`** — surface consumability: matching, elaboration, and
  structural-recursion/termination checking accept the lifted hypotheses.
- **`D6`** — the four conformance rows of contract point 4. **MERGED 2026-08-10**
  as `d9b1d5b1` (PR #1753), `main` `276d5ae4`, both blobs verified. **Seven cases
  bound; `nested-size-uses-lift` gated** on [[KERNEL-RECURSIVE-RESULT-SURFACE]].
  Heading census `19 → 14`, re-measured on `main` after the merge with the size
  row's marker still present. ⚠ The node stays `active`: `D7` and `AC-K12` are
  open, and `D6` closing is **not** `AC-K12` progress.

> ### COUNTING THE GATE MARKERS IN `seed-nested.md`: use the FORM, never the word
>
> **Recorded 2026-08-10 from Adversary `evt_1dj98k67j19mt`, measured on `main`
> `aab04044`. The next person to census this file is the audience.**
>
> `D9` un-gated `nested-size-uses-lift` across **eight sites**, and its sibling
> `nested-dependent-motive-uses-lift` — which stays gated — carries a
> near-identical `Status:` paragraph and its own four identical qualifiers. The
> control on that edit is a **count**, so the count has to be reproducible.
>
> **The status marker is the parenthesised form `(future binding, gated)`, not
> the word `gated`.** Measured on `aab04044`:
>
> | keyed on | count |
> |---|---|
> | `(future binding, gated)` | **4** — all in the dependent-motive row |
> | bare `gated` | **15** |
>
> The gap is not noise. The file uses `gated` in a **mechanism** sense too —
> `gated erasure admitting the generated support Elim …` sits eleven lines from
> the qualifiers, **inside the same row's region** — alongside `executes
> un-gated`, `implementation-gated` and `Independently gated`.
>
> ⇒ **`D9`'s census was correct because the qualifier happens to have a
> distinctive parenthesised form, not because `gated` is unambiguous here.**
> Re-deriving it from the bare word over-counts by nearly four times, and a
> qualifier later spelled without the parenthetical merges the two senses.
> **The failure direction is the bad one: a row that silently loses its gate
> reds nothing in CI.**
>
> **When the dependent-motive row is eventually un-gated, state the marker form
> in `seed-nested.md` itself** — that file carries the counting claim and this
> node does not. I am recording it here rather than editing `conformance/`,
> which is not mine to touch unilaterally.
>
> **Why this is worth a paragraph at all:** the Adversary reproduced every
> figure by deriving the file's gating vocabulary independently rather than
> searching the three tokens I handed it — a token-keyed sweep verified by a
> token-keyed check agree for free. That is the method the next census needs,
> and it is what surfaced the ambiguity the count had been surviving by luck.
- **`D7`** — the `trusted_base()` delta, **in two parts with two different
  kinds of answer.** Metric ruled by the Steward 2026-08-10; see the ruling
  block below the AC table, which supersedes the withdrawn "not a zero" wording
  this line used to carry.
  1. **The measurement, and it is set-valued.** `GlobalEnv::trusted_base()`
     (`crates/ken-kernel/src/env.rs:568`) is the unchecked-assumption ledger of
     spec `18 §5` — non-prelude `Opaque` plus non-literal `Primitive`
     `GlobalId`s. The expected and correct answer is the **empty delta with set
     identity**. Baseline is **within one run**: `before`/`after` around the
     nested elaboration in a single `ElabEnv`, so no historical SHA is a
     baseline and none has to be chosen.
  2. **The audited-code half, named and not numbered.** Enumerate by
     `file:line` the kernel paths that became load-bearing for nested-inductive
     admission — the support generator, the atomic host-plus-support
     admission/rollback transaction, and iota. Citations and prose. **Do not
     invent an LOC, function, or file count**; the repository's kernel audit
     lists trusted-vs-test LOC accounting as an unwired follow-on, so a number
     here would have no instrument and no baseline behind it.

## Acceptance criteria

Each names its positive control.

| AC | claim | positive control |
|---|---|---|
| `AC-K1` | `data Json = ... \| JsonArray (List Json) \| JsonObject (List (Pair String Json)) \| ...` is **admitted**. | ⚠ **necessary, not sufficient** — a guard-deletion passes this row. It is listed to be discharged, not to be relied on |
| `AC-K2` | Admission is keyed on **declared parameter polarity**, not on a name. | declare a **new** strictly-positive container of your own and nest `Json` in it → must be admitted **with no kernel change**. ⛔ If it needs one, an allow-list is hiding somewhere |
| `AC-K3` | ⭐ **A real theorem over the array branch is written and kernel-checked**, consuming a lifted IH. | delete the lifted IH from the generated eliminator → the theorem must **fail to check**. ⛔ If it still checks, `AC-K3` was never testing the IH |
| `AC-K4` | Iota reduces for nested occurrences; a **recursive computation** over `JsonArray` evaluates. | perturb one iota rule → the computation's result changes or it fails to reduce |
| `AC-K5` | Nested-**negative** rejection retained. | the known-bad `(D → Bool) → D` under a container must still be **rejected**, asserted as the specific rejection |
| `AC-K6` | Rejection through an **unknown** parameter retained. | nest `Json` under a parameter whose polarity is undeclared/unknown → **rejected**, not admitted-by-default |
| `AC-K7` | Rejection through a **non-positive** parameter retained. | as `AC-K6` with a declared-negative position |
| `AC-K8` | Direct and existing **W-style** inductives unchanged. | the K1.5 Π-bound suite (`(Nat → D) → D`, §2.1) runs green **untouched**; ⛔ a diff to those tests is itself a finding |
| `AC-K9` | ⛔ **Zero** new axiom, postulate, trusted escape, or library-side representation workaround. | grep the diff for `Axiom`/`postulate`/`sorry`/`unsafe` additions; a hit fails the row |
| `AC-K10` | The `trusted_base()` delta is **empty with set identity** — asserted as the before/after `GlobalId` sets, never as a cardinality. Plus a named, unnumbered enumeration of the kernel code that became load-bearing (`D7` part 2). | **Mechanical, and the idiom already exists in-tree.** `BTreeSet` before/after around the nested elaboration in one `ElabEnv`, as in `ds6c_intlit_elaborator_emission.rs:184` (`trusted_base_delta_is_exactly_empty_not_a_shrink`) and `either_catalog_package_acceptance.rs:69`. Set identity, not `len()`, is the point: a swap reads as zero under a count. An **executed** identity assertion is what makes "measured, empty" and "never measured" different objects — prose cannot do that |
| `AC-K11` | ⭐ `D1a`'s recorded polarity is **populated at admission and read by the positivity check** — not recorded-then-ignored. | perturb the **recorded** value for one parameter → admittance must change. ⛔ If it does not, the check recomputes and the record is inert — the `ConstructorDecl.recursive_positions` failure repeated (frame §2e) |
| `AC-K14` | ⛔⛔ **`D3b` and `D4` land in ONE commit, and the pair is kernel-checked.** No commit exists in which a generated method binder carries a structured lift that `iota_reduct` does not construct. | ⚠ **`Σ (_ : D). D` is the control, because it is admitted on `main` TODAY with zero IHs** (`inductive.rs:91` checks both Sigma components at the same polarity; `:90` flips only `Pi`'s domain). Exercise the eliminator on it: the method binder's lift and ι's constructed term must agree, and the kernel must check the pair. ⛔ A `method_type` change without the matching ι is a **subject-reduction defect**, not an incomplete step. Architect `dec_351mz4r239398` |
| `AC-K13` | ⭐ **The polarity producer is TOTAL over every accepted constructor field type** — every such field yields a polarity record or a rejection, ⛔ **never a panic** (`18 §4`). | ⚠ **Enumerate by `Term` form, not by example.** For each form the fallback traverses, exercise a field of that form that mentions the parameter. Two named controls, both from Architect rejections: (a) `Term::Let { ty: Bool, val: false, body: pi(var(1), Bool) }` — an accepted field reducing to `A -> Bool`, which must record `NonPositive`; its `body` binds index 0, so a fallback that traverses children at one depth reads `A` at the wrong index. (b) index selection must be non-panicking for an out-of-range relative index — ⛔ `bool::then_some` evaluates its argument **eagerly**, so `(r < n).then_some(n - 1 - r)` underflows *before* the condition can yield `None`; `then(\|\| …)` is the lazy form |
| `AC-K12` | A nested-IH constructor **lowers and evaluates**, not just type-checks. | the evaluator and native-lowering paths **re-derive** recursive positions (frame §2d, §2f) and one lowering site computes binder arity as `argument_binders + recursive_positions.len()`. Control: a recursive computation over `JsonArray` evaluates, and the built-artifact suite is green. ⛔ **AND the carried control `liftrose_synthetic_witness_closes_owner_two_required_joins` RUNS** — it is `#[ignore]`d in `crates/ken-runtime/.../planning/static_transition.rs`, and this criterion is the tracked owner of its release condition. **Do not report `AC-K12` green while it is still `#[ignore]`d.** Full rider below in this node; it is stated here because a discharge check reads this row and not the prose |

### `AC-K10` METRIC RULING — the "not a zero" clause is withdrawn

**Steward, 2026-08-10.** `D7` stopped here rather than authoring, which was the
right call. `kernel-implementer`'s `D7` hard-stop post in `thr_14s3` grounded
that `GlobalEnv::trusted_base()` counts only non-prelude `Opaque` and
non-literal `Primitive` `GlobalId`s, while the nested support families are
checked `Inductive`s, so its real delta is `+0` with set identity — and `D7`
simultaneously ruled that the outcome is not zero. Both cannot hold.

**The ruling: `+0` with set identity is the correct answer, and the withdrawn
clause was the defect.** Three independent groundings, none of them new
judgment:

1. **`AC-K9`, two rows up, forbids exactly what a nonzero delta would
   report.** It requires zero new axiom, postulate, or trusted escape, with a
   grep control. `trusted_base()` counts postulates (`Opaque`) and real
   primitives. A nonzero `trusted_base()` delta therefore **fails `AC-K9`**.
   The AC table already answered this question in the opposite direction.
2. **This node's own still-binding constraint list says so.** The "Still
   binding, none of it discharged by the merge" list under the RE-RELEASED
   2026-08-09 block (Spec representation contract, exact `c7f8913c`, PR #1678)
   requires *"zero `trusted_base()` delta with audited
   generator/transaction/iota TCB"* — zero on the ledger, audited on the code,
   as two separate obligations. `D7` part 2 above is that second obligation
   restated where the deliverable can see it.
3. **The in-tree idiom expects empty and says why.** `ds6c`'s test carries
   Architect-corrected wording — *"assert the honest claim, not an over-claimed
   shrink."* The same discipline in the other direction is what `AC-K10`
   needed.

**What is genuinely true is that the audited kernel code grew, and that is not
the same object as the ledger.** `trusted_base()` measures declarations trusted
*without* kernel proof; the nested mechanism adds none, because the support
families are checked. The checker that does the checking did grow. Reporting
`+0` on the first and naming the second is the honest accounting the node asked
for — not a weaker version of it.

**Refused, and the implementer was right to refuse it first:** substituting net
LOC, changed functions, files, or "semantic mechanisms" for the ledger unit to
make some number nonzero. That is fitting the measurement to a frame assertion.

**The baseline question dissolves.** It was only hard because a historical unit
was assumed. The in-tree idiom takes `before`/`after` around the elaboration in
one `ElabEnv`, so none of the three candidate SHAs (`10b2f56a`, `b5c448d1`, or
a sum over the landed production objects) is needed or wanted.

**Report location:** the executed assertion is the authoritative artifact; the
prose half lands as a `D7` block in this node. No separate report file.

**Origin of the defect, recorded because it has already been copied once:** the
"not a zero" clause was a Steward assertion made without checking that
`trusted_base()` was the right instrument, then repeated in the `D7` kick, at
which point it bound the ring. Anyone reading a downstream artifact that still
says the delta is nonzero is reading the withdrawn clause.

### `D7` report — declaration ledger unchanged; checker growth enumerated

**Authoritative mechanical control.**
`crates/ken-elaborator/tests/nc14_data_match_lowering.rs:242` captures
`GlobalEnv::trusted_base()` as a `BTreeSet<GlobalId>` before and after nested
elaboration in one `ElabEnv`, then asserts set identity. The declaration-ledger
delta is **`+0 GlobalId`**: generated support families are checked `Inductive`
declarations, not non-prelude `Opaque` or non-literal `Primitive` assumptions.
Set identity is the result; equal cardinality would miss an assumption swap.

**Audited checker growth, named rather than assigned a fabricated count.**

- `crates/ken-kernel/src/inductive.rs:954` — `build_all_support_decl` constructs
  each terminal, source-indexed support family and its topology-aligned
  evidence constructors.
- `crates/ken-kernel/src/check.rs:921` — `declare_inductive` owns the atomic
  host-plus-support admission transaction; its publication and rollback paths
  are at lines 953–1014, so a failed host or support check cannot expose a
  partial declaration set.
- `crates/ken-kernel/src/inductive.rs:2185` — `iota_reduct` derives recursive
  shapes and constructs the matching lifted terms at lines 2233–2259 before
  applying the selected method.

- **`crates/ken-kernel/src/env.rs` — the support-authority registry.** ⚠ **Added
  2026-08-10 from Adversary finding `evt_3ycbbm7yydhva`; the enumeration as
  merged omitted this file entirely.** It holds the state and the relations that
  decide support identity and terminality:

  | line | symbol |
  |---|---|
  | `:254` | `all_supports: HashMap<(GlobalId, usize, AllSupportSort), GlobalId>` |
  | `:258` | `terminal_supports: HashSet<GlobalId>` |
  | `:400` | `all_support_origin(...)` |
  | `:412` | `is_terminal_support(family)` |
  | `:416` | `register_all_supports(...)` |

  This is **authority, not construction**, which is why it belongs in an audit
  enumeration at least as much as the constructors do. `all_support_origin` is
  the provenance gate on which checked-artifact erasure admits a generated
  support `Elim`; `is_terminal_support` is the predicate behind the seed's
  `nested-generated-all-support-is-terminal (soundness)` row. ⚠ **A wrong answer
  from either is silent, where a wrong constructor is loud.** There is also an
  open, unclosed soundness concern against this exact registry —
  [[RT-TERMINAL-ALL-ELIM-AUTHORITY]] `AC-8`, that a reverse scan over a
  randomized `HashMap` fails as a nondeterministic authority answer rather than
  a refusal.

**Scope of this enumeration, stated because the omission proved it was not
obvious:** it covers **every kernel surface a reader must newly trust** for
nested-inductive admission — the constructors, the admission transaction, iota,
and the support-authority registry. ⛔ It is **not** narrowed to "the checker"
with registries and accessors excluded.

Thus the kernel checker grew by these four load-bearing mechanisms while the
unchecked-assumption ledger grew by **`+0 GlobalId`**. No LOC, function, or file
count is claimed because the repository has no instrumented baseline for one.
This report does not advance `AC-K12`, which remains Runtime-blocked.

> ### Why neither half of `D7` could catch the other's gap
>
> `all_supports` and `terminal_supports` are **`GlobalEnv` struct fields, not
> `Decl`s**, and `trusted_base()` iterates `self.decls` filtering `Decl::Opaque`
> and non-literal `Decl::Primitive`. ⇒ **The `AC-K10` control structurally
> cannot observe the support-authority state.** No outcome of that test says
> anything about it.
>
> So `D7` shipped as a mechanical control that provably cannot see this surface,
> plus a prose enumeration that omitted it. **The defect was in the conjunction,
> and reviewing either half alone reads complete.** ⇒ Reusable: when a
> deliverable is *"a control plus a prose claim"*, **the control's blind spot is
> exactly where the prose needs auditing hardest** — and that is the one place
> neither reviewer is looking, because each is checking the half that is fine.
>
> **What proved this was an omission and not a scoping choice:** `git log -S`
> over `crates/ken-kernel/src/` shows `build_all_support_decl` — the anchor that
> *was* named — together with `all_support_origin`, `is_terminal_support`, and
> `register_all_supports` **all introduced by the same commit, `afb38934`.** The
> enumeration took one symbol from that change and left three, in a file it
> never mentioned. Re-verified by the Steward at `d1c91369`; all five line
> citations above resolve there.

**Validation.** After the operator all-clear, the exact set-identity control
passed 1/1 and the complete affected `nc14_data_match_lowering` suite passed
12/12. `D7`/`AC-K10` is ready for fresh QA; this does not advance `AC-K12`.

### `AC-K12` ACQUIRED A RIDER — it now owns a carried Runtime control

**Added 2026-08-09 from Adversary triage of finding `evt_37y39vcj7y695` on
`82918b6a`.** Not a new acceptance criterion and not `D6`/`D7` work.

`ken-runtime/src/cranelift_backend/planning/static_transition.rs` carries an
`#[ignore]`d control, `liftrose_synthetic_witness_closes_owner_two_required_joins`,
whose release condition is stated in its own doc comment as *"nested-inductive
admission is on `main`"*. That condition is exactly what `AC-K12` requires, and
`RT-BODY-OCCURRENCE-PROVENANCE` — the node that authored it — is **closed**, so
the condition had no tracked owner. It has one now: **`AC-K12`**.

Two consequences, and only the second is anyone's task today:

1. **Discharging `AC-K12` also obliges running that carried control.** Do not
   report `AC-K12` green while it is still `#[ignore]`d. The obligation rides on
   the capability, not on this node's closure — if `AC-K12` is later recut, the
   rider follows whatever criterion inherits the capability.
2. **The code edit is Runtime's, not Kernel's** — it lives under
   `crates/ken-runtime`, which `D5` correctly refused to enter. It is folded into
   [[RT-MATCH-RECURSOR-CONSUMERS]] as `D10`.

**Why this is recorded rather than re-worded in place.** The comment's release
condition has now been corrected three times, and each correction restated it
somewhere with no owner to re-read it. A fourth wording would repeat that. The
condition belongs in a tracked node that a status check can see, which is what
this block makes it.

> **This block was not enough on its own, and the gap is worth keeping.**
> Adversary `evt_51haf6m1p83b0` (2026-08-10, confirmed and repaired the same
> hour): a status check reads the **`AC-K12` row**, which is the criteria
> table's **last** row — and that row said nothing about this obligation, with
> no forward pointer across the ~150 lines between them. So the chain ran
> *code comment → "tracked at `AC-K12`" → the row → silence*, and the
> obligation sat at neither end of the last hop. The rider is now **in the
> row's control column**; this block remains the full statement.
>
> **The direction is what makes it worse than the usual version.** Elsewhere
> the loose summary sits in the operative text and the precise sentence in the
> prose, which fails toward a confusing red. Here the precise obligation was
> in the prose and the **operative row was silent**, which fails toward
> **green** — a discharge check would have passed `AC-K12` with the control
> still `#[ignore]`d. `RT-DYNAMIC-ARM-SCALAR-MERGE` `c1` merged at `7bfc8ae5`
> the same day, retiring this criterion's stated blocker, so that check was
> imminent rather than hypothetical.
>
> ⇒ **The rule that covers both directions: the operative artifact — the row,
> the assertion message, the AC — carries the claim, whichever pass wrote it.**
> Prose beside it is elaboration, never the only statement.

⛔ **`AC-K3` and `AC-K8` are the pair that matters.** `AC-K3` proves the new
capability is *usable*; `AC-K8` proves the old capability is *undamaged*. A node
that greens one and quietly weakens the other has widened the TCB for nothing.

⚠ **Report `AC-K5`–`AC-K7` as three separate rows.** They are three different
rejection reasons and an aggregate "negatives still rejected" pass would hide one
of them defecting.

## Validation — targeted only

⛔ **NEVER `--workspace`** (operator hard rule, `agent/COORDINATION.md §12`). Scope
to the crates you touch (`-p ken-kernel` and, for `D5`, `-p ken-elaborator`), plus
the kernel conformance suite. **The full build, `--locked`, and conformance run in
CI on GitHub.** "No regression" means **green in CI**.

⚠ **A kernel change is the case where the local/CI split bites hardest** — the
blast radius is every crate. ⛔ Do not conclude "no regression" from a green
targeted run; say what you ran and let CI answer the rest.

## Contention

⚠ **This is the one node in flight with a wide blast radius.** It changes the
kernel's admittance surface, so it is **not** contention-free the way DS-9 and
`ABI-S3` are with each other. ⛔ Re-derive contention at kickoff against whatever
is then active in `crates/` — this section will be stale.

## What this unblocks

`DS-9`, and with it Phase 3 of the catalog data-structures campaign. ⭐ More
broadly: nesting a `List` inside a recursive type is the shape of **every tree
with a list of children** — JSON, XML, S-expressions, ASTs, rose trees. ⚠ That
breadth is the argument for doing it properly, ⛔ not for widening scope past
nested-only.

[f]: ../wp/kernel-nested-inductives.md
