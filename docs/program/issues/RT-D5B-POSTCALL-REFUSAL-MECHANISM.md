---
id: RT-D5B-POSTCALL-REFUSAL-MECHANISM
title: "What is the mechanism of the CheckedIhDetachedCallerCut refusal raised WHERE A TWO-STEP SelfDefining POST-CALL CONSUMER RECEIPT MEETS ONE COMPUTATIONAL ELIMINATOR FRAME -- the only distinct Packaging reason in abi_s6_mapping_file_backed_native. THE SUBJECT IS THAT CONDITION, NOT A LINE NUMBER: this node was cut against an unlanded tree, and every lowering/core.rs:7720 in its body is a coordinate ON THAT TREE recording where a measurement was taken, never a pin to re-resolve against main. THREE mechanisms have been proposed and measured away in one session, all sharing the premise that the defect is reachable from the call site's inputs; the measurement says those inputs are internally consistent and the disagreement is upstream of them. The mechanism is UNKNOWN and this node exists to find it, not to carry a candidate. NOT WORKABLE ON MAIN TODAY: main can NAME this refusal and cannot EMIT it -- CheckedIhDetachedCallerCut appears in crates/ only in planning/static_transition/responses.rs, 9 occurrences, and the emitter bind_checked_ih_detached_caller_cut is absent. WHICH blocker unblocks it is OPEN and there are three live readings -- see the banner; do not assert one in this frontmatter"
status: draft
owner: runtime
size: M
gate: architect
depends_on: [ABI-S6-HS18-D5B-SUBSTRATE-PORT]
blocks: []
github: null
tier: T1
origin: "Cut by the Steward 2026-09-14, superseding two unlanded drafts withdrawn before commit (RT-POSTCALL-RECEIPT-ALIGNMENT-AUTHORITY, RT-SELFDEFINING-RECEIPT-FRAME-ARITY), each named for a mechanism refuted within minutes of being written. This node is deliberately named for the QUESTION. Chain: implementer evt_4y7sypns2j0v, Steward hold evt_5t0n56p3jvpck, implementer withdrawal evt_1281f1q5v7j9p, Architect evt_5x5cfwgp4bk24 + evt_41r81fdj0741f + evt_pzd11rqycqe, implementer measurements evt_49vn1p2c57e7b + evt_5ycnn29hy38qs + evt_3gyahpfpnja76. COORDINATION §1a fired by the Steward on the third refuted mechanism."
---

> # DEMOTED `ready` -> `draft` 2026-09-17. DO NOT PULL THIS NODE.
>
> **It is not workable on `main` and nothing it needs has landed.** It was
> `ready` on the premise that its subject was reproducible; the measurement below
> refutes that: at `origin/main` `b0eb29e71`, `CheckedIhDetachedCallerCut`
> appears in `crates/` **only** in `planning/static_transition/responses.rs`
> (9 occurrences), and the emitter `bind_checked_ih_detached_caller_cut` is
> **absent**. **`main` can NAME this refusal and cannot EMIT it** — measured by
> two independent routes (runtime-implementer `evt_4gscc5na6pywv`, Architect
> `evt_1grgpvv8cjq66`).
>
> **The three refuted mechanisms and every measurement below remain valid and
> are why this node still exists** — the demotion is about reachability, not
> about the question going away.
>
> **WHICH blocker unblocks it is OPEN. See "Three readings" below. Do not pull
> this node, and do not assert a blocker that has not been measured.**

> # RE-ANCHORED 2026-09-17: THE SUBJECT IS A PREDICATE.
> # `:7720` IS A RECORD, NOT A PIN.
>
> **This node's subject is the CONDITION named in the title** — a two-step
> `SelfDefining` post-call consumer receipt meeting one computational eliminator
> frame. It was previously named for `lowering/core.rs:7720`, **a coordinate on
> a tree that never landed.** Measured at `origin/main` `b0eb29e71`:
>
>     crates/ken-runtime/src/cranelift_backend/lowering/core.rs
>       origin/main   16487 lines      :7720 is a comment about recursive positions
>       30d35f625     17388 lines      the tree this node was cut against
>
>     CheckedIhDetachedCallerCut, tree-wide in crates/
>       origin/main    9 occurrences, 1 file  (planning/static_transition/responses.rs)
>       30d35f625     29 occurrences, 4 files
>
> ⇒ **Whatever sits at `:7720` on `main` is a different site.** The 32
> occurrences of `7720` in the body below are left in place deliberately: each
> **records where a measurement was taken on the checkpoint tree**, and rewriting
> them would destroy the argument the three refutations rest on. **A record must
> not change; a pin must. Do not resolve any of them against `main`.**
>
> ## NOT WORKABLE ON `main` TODAY, AND THE SUBSTRATE PORT DOES NOT CHANGE THAT
>
> The Steward's 2026-09-17 sequencing ruling (`evt_14chw4xj920d4`) said the
> `ABI-S6-HS18-D5B-SUBSTRATE-PORT` would give this node *"a reproduction on main
> for the first time."* **That premise was refuted by reading**
> (`evt_30p9m0j8bj1f2`), and the claim is withdrawn:
>
>     lowering/effects.rs :2812   if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1.contains(&operation)
>                                    { return Err(unsupported(...)) }     at origin/main
>
>     abi_s6_mapping_file_backed_native.rs at 30d35f625  (ken-cli/tests/, not ken-runtime/)
>       raw strings in the file                 exactly ONE   const SOURCE
>       build/run call sites                             15
>         passing SOURCE                                 15
>         passing anything else                           0
>       SOURCE :67   (withMapping ... (FileBacked file (8 : Int)) ReadWrite ...)
>
> **Thirteen tests, one program, and that program acquires a file-backed
> mapping.** With the `MappingAcquireFile` grant excluded the op is absent from
> the roster, so every test is refused at `:2812` **before lowering** — **no test
> in the acceptance file can reach the refusal.**
>
> ## THAT MEASUREMENT IS ABOUT ONE POPULATION. IT WAS OVER-READ ONCE ALREADY.
>
> **The Steward first wrote `depends_on: [RT-D5B-MAPPING-AVAILABILITY-FLIP]` off
> the paragraph above, and that edge is WITHDRAWN.** The acceptance test's census
> is complete and correct, and it answers *"can the ACCEPTANCE TEST reach the
> refusal"* — **not** *"can anything on `main` reach the refusal."* The Architect
> measured under it rather than inheriting it (`evt_1grgpvv8cjq66`), and the
> generalisation does not hold:
>
>     lowering/source.rs :4412  fn bind_checked_ih_detached_caller_cut
>                               11 x Err(unsupported("CheckedIhDetachedCallerCut", ...))
>                               in that one body, :4425 .. :4544
>     lowering/source.rs :4558  fn bind_checked_ih_detached_caller_cut_from_call  (+1 at :4573)
>
>     scope = source.rs ONLY      bind_ / CheckedIhDetachedCallerCut
>       prefix base 2a74775ae         0 / 0
>       prefix tip  30d35f625         7 / 12
>       origin/main b0eb29e71         0 / 0
>
> **`source.rs` is a REGION 2 file (`+669/-266`, on the 13/13 clean-replay list).
> The port LANDS the emitter.** And its four live call sites sit in
> `source_call_state` — general call lowering — gated on **tail shape**
> (`tail_worker_body_is_ret_kmatch`, `tail_route_is_forward_edge_collapsible`,
> `ComposedReturnForwardRetAuthorityOutcome::{NonApplicable, SuppressedForInertness}`),
> with the comment above `:4985` routing the **effect** case the other way.
> **The detached-caller-cut path is the NON-effect branch**, so an op-invoking
> program is the case routed AWAY from this emitter, not the paradigm case for
> reaching it.
>
> ## THREE LIVE READINGS OF THE BLOCKER. None is asserted; the port discriminates.
>
>     (A) subject is the source.rs EMITTER      Region 2 -- LANDS with the port.
>                                               Reachable by TAIL SHAPE; the grant
>                                               is not what gates it.
>     (B) subject is the core.rs VALIDATE path  validate_checked_ih_detached_result_shape
>                                               at core.rs:9181, called :6736/:6836 --
>                                               all REGION 3, which the port does NOT
>                                               carry. Blocker is Region 3, and the
>                                               flip node would never discharge it.
>     (C) the grant gates reachability          the Steward's original edge. TRUE for
>                                               the acceptance test, UNPROVEN in general.
>
> The retired coordinate `core.rs:7720` sits between (B)'s two call sites and what
> is there is the
> `EliminatorFrame::{Computational,Ordinary,PendingLet,InvocationReturn,Active}`
> match — which matches this node's re-anchored subject wording closely, so **(B)
> is live and not a formality.**
>
> **What nobody has measured: whether any program in the ported corpus actually
> HAS the tail shape.** A general guard is not a reachable path. One of the four
> sites (`:5093`) additionally needs `#[cfg(feature = "px8-ds-test-support")]`.
>
> Two facts that constrain it cheaply: the two symbol sets are **independently
> closed** — `source.rs` never names `validate_`, `core.rs` never names `bind_` —
> so the port lands no dangling reference under either reading.
>
> ⇒ **`depends_on: [ABI-S6-HS18-D5B-SUBSTRATE-PORT]`**, which is what every
> reading agrees moves the tree next. **Do not add a blocker edge that has not
> been measured.**
>
> ## THE REFUTATION TRIGGER — this node's re-examination must not depend on memory
>
> A `draft` node is pulled by nobody, so **if the demotion is too strong nothing
> re-examines it.** Written so the re-examination has a cause:
>
> > **If the port's AC-1/AC-4 build produces any red carrying the
> > `CheckedIhDetachedCallerCut` Packaging reason WITH THE GRANT FULLY EXCLUDED,
> > then reading (C) is refuted, reading (A) is confirmed, and this node returns
> > to `ready` with no dependency on `RT-D5B-MAPPING-AVAILABILITY-FLIP`.**
>
> **That is an unblock path that does NOT run through the Architect's refusal**,
> and the Architect asked for it in writing knowing that (`evt_1grgpvv8cjq66`) —
> having just disclosed that they gate this node and authored that refusal. A
> node whose only route to `ready` runs through one seat's prior decision is worse
> than one with two routes, whoever holds the decision.
>
> **Quote a `CheckedIhDetachedCallerCut` red's reason string exactly rather than
> classifying it** — it is now evidence for two nodes, not one.
>
> **The 11-base-red population is a CHECKPOINT-TREE measurement.** Those 11 reds,
> 8 of them this refusal, were taken on a tree **with** the grant. The node's own
> text already concedes they *"were never a stable population"*; the port will
> produce a different population by construction, and that is not a discrepancy
> to reconcile.
>
> ## ARCHITECT DISCLOSURE — live under (C), and why (C) gets no benefit of the doubt
>
> The Architect is `gate: architect` on this node **and** is the seat that
> refused the `MappingAcquireFile` grant twice and defined its scope both times
> (`evt_prasphavwv64`, unprompted). **Under reading (C) their own refusal is this
> node's only blocker** — which is precisely the configuration they then went and
> measured against, producing (A).
>
> **Price `RT-D5B-MAPPING-AVAILABILITY-FLIP` without deference to that refusal,
> and do not let it through on the argument that a blocked node needs it** —
> that is precisely the argument the original refusal rejected (*"membership is
> a plan, not evidence"*). The correct input is the refusal's **ground**, not its
> authority: if the flip node supplies evidence of native availability, the
> refusal is discharged on its own terms.

> # READ FIRST: THREE REFUTED MECHANISMS. THIS NODE CARRIES NO CANDIDATE.
>
> Each was proposed by a competent seat, was locally reasonable, and was killed
> by measurement — two of them by the proposer's own stated criterion. **Naming
> a fourth before reading this list is the failure mode this node was recut
> twice to avoid.**
>
> **1. TRIM PLACEMENT — VOID.** *"Where does trim authority live for `:7720`"*
> has no content: `:7720` is reached only under `SelfDefining`, where
> `executable_exits()` and `selected_case_exits()` return the **same value**.
> There is nothing to trim. (Architect fork `evt_5x5cfwgp4bk24`, voided by
> measurement `evt_49vn1p2c57e7b`.)
>
> **2. ACCESSOR CHOICE — DEAD EVERYWHERE, NOT JUST HERE.** Swapping the accessor
> at `:7720` changes nothing. Grounded by a **closed producer enumeration**, not
> by sampling: a `CheckedIhPostCallConsumer`'s selection is `None` or
> `SelfDefining` and `CallerCompleted` is **unconstructible** on it
> (`evt_5ycnn29hy38qs`).
>
> **3. MISSING ROUTE — REFUTED NUMERICALLY.** `:7720` not routing a
> required-consumer incoming edge is real (see the surviving census below) but
> **does not explain the mismatch**: `incoming_consumer_edge_index` is **0** at
> all 11 reaches, so `executable_exits()[0..]` is the whole 2-chain and
> `&eliminators[0..]` is the whole single frame. **Both halves of the
> correspondence are no-ops at index 0.** Routing yields the identical 2-against-1.
> (Architect account `evt_pzd11rqycqe` §5, refuted on its own stated criterion by
> `evt_3gyahpfpnja76`.)
>
> **The shared premise of all three: that the defect is reachable from this call
> site's inputs.** The measurement says those inputs are internally consistent at
> index 0, and the disagreement is **upstream of all of them**.

## The observation

`lowering/core.rs:7719-7720` at `0d94d58b6` — re-derive at your base (`AC-1`):

    let residual = self
        .checked_ih_post_call_residual(consumer.selected_case_exits(), eliminators)?;

Measured across the whole suite, no variation:

    11 x   site=7720  index=0  executable_exits=2  selected_case_exits=2
                              suffix=2            eliminators=1

The receipt describes a two-step exit chain; the site holds one computational
eliminator frame; the bounds guard at `core.rs:9256` refuses. **This is the only
distinct `Packaging` reason in the run and the cause of 8 of 11 base reds.**

## What is NOT in doubt

- **Completeness, not soundness.** The compiler **rejects a valid program**; it
  does not admit an invalid one. Rests on the `:9256` bounds guard and the
  `#[track_caller]` counts, which survived every correction.
- **The guard works and must not be weakened.** When a probe equalized the
  counts, the refusal **moved** to the per-frame content arm at `:9269` and the
  wrong defining call still failed to compile.
- **No length-derived trim, at any of the five call sites, ever.** Architect,
  `evt_41r81fdj0741f`: `caller_exit_index` is an alignment witness; a difference
  of two lengths is not. Structural and variant-independent — **and now vacuous
  here**, since there is nothing to trim.
- **`:7720` is the only site in the crate that stops after the first half of the
  two-step protocol.** Census at `evt_pzd11rqycqe` §4; the protocol is written
  out at `planning/static_transition/aggregates.rs:7826-7829` and
  `responses.rs:598-606`. **This survives the refutation of mechanism 3** — it is
  a real asymmetry, it is simply not the cause.

## Deliverables

> # EVERY DELIVERABLE IS ANSWERED. THE NODE IS NOT CLOSEABLE, AND THE REASON IS
> # ITS BASE, NOT ITS CONTENT.
>
> **Disposition recorded by the Steward 2026-09-14 on the Architect's
> `evt_w1vnqexkhmxx`** (*"`MECH` answered, `CTRL` complete, `R3` landed, `ROUTE`
> declined with reason ⇒ the node's deliverables are closed"*):
>
>     MECH    ANSWERED   ruling below, evt_5k7jd5agh133j + evt_11zby14s19hkh
>     CTRL    COMPLETE   88893210a (the class assertion, ruled as D3)
>     R3      BUILT      43e4451e3 + ff7638ff6, QA APPROVED
>     ROUTE   DECLINED   evt_w1vnqexkhmxx -- a theorem, not a deferral
>     MECH-2  ANSWERED   2026-09-16, answer section below -- a SCOPE
>                          difference, not a phase artifact; corroborates `R3`
>
> **`MECH-2` postdates the banner and the heading above is therefore no longer
> literally true.** It is a read-only divergence measurement, framed at
> runtime-leader's request on the Architect's `N` read; see its own section
> below. **It does not change this node's disposition** — the node is held on
> its base either way, and `MECH-2` produces a recorded answer rather than a
> commit, so it cannot clear that hold.
>
> **"`R3` landed" means landed ON THE BRANCH, not on `origin/main`.** `43e4451e3`
> and `ff7638ff6` sit on `wp/ABI-S6-d5b-file-backed`, 24 commits deep, on top of
> four `ABI-S6 D5b WIP` commits carrying 12374 unlanded insertions.
> `git merge-tree --write-tree origin/main ff7638ff6` differs from
> `origin/main^{tree}`, so none of it is on `main`.
>
> ⇒ **DO NOT FLIP THIS NODE `merged`.** Deliverables being answered is not the
> node being closed; **closure follows the merge.** This node joins `D2`, `R1`,
> `R2` and entry-18 in the held-pending-D5b set, and it is held by the same
> thing they are: there is no clean base below the work. See
> [[RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]]'s held block.
>
> # AND DO NOT DISPATCH `CTRL-a`. IT IS BUILT. THE REMAINING OBSTACLE IS
> # LANDING, NOT A DELIVERABLE.
>
> **The Steward told the ring twice that `CTRL-a` was "startable now". That was
> wrong when it was said, not merely stale** — `88893210a` was already in the
> branch at the time (Architect, `evt_6zm1feakd1k6p`; verified independently):
>
>     git merge-base --is-ancestor 88893210a ff7638ff6   -> YES
>     git merge-tree --write-tree origin/main 88893210a  -> 4a52074f91586eb514e8b3116e5b1b1ef889f58d
>     git rev-parse origin/main^{tree}                   -> 07936923a39de217dfccf9c97bc30033d4eb8be0
>
> Ancestor of the branch tip; its merge against main differs from main's tree.
> **Built and unlanded, exactly like `R3`.** Dispatching it sends an implementer
> to rebuild what the branch already holds, and they would correctly hard-stop.
>
> **Read this deliverable list as a RECORD, never as a queue.** Every entry is
> answered or built; none is pullable. The one thing that would reduce it is a
> clean base under 24 commits, which is not work a build seat can pull.
>
> **State the scope beside every branch number, because three true numbers
> disagree:**
>
>     git diff --stat origin/main...ff7638ff6   35 files 18592+ 6144-   vs MERGE-BASE: what the branch ADDS
>     git diff --stat origin/main   ff7638ff6   47 files 18612+ 9062-   vs MAIN TIP: also reverses main's 21 commits
>     git diff --stat 4bf1ad362     5a9a840ba   34 files 12374+ 5644-   the four bottom WIP commits ALONE
>
> The Steward and the Architect each reported one of the first two without
> naming which question it answered, within an hour of reconciling the same
> defect shape one artifact down.
>
> **What is NOT a blocker, measured so nobody re-derives it:** `0bbe4a175`
> ("REFUTED ... DO NOT BUILD ON") leaves ZERO residue —
> `git rev-parse 936294421^{tree} 686ffa8ac^{tree}` returns
> `b41367855cc5f7b2a0ea098ed94a83b27c15cbc8` for **both**, and the diff between
> them is empty. Under squash-merge no intermediate commit lands at all. The
> chain's shape is not what holds this work.

> **DELIVERABLE LABELS ARE `MECH` / `CTRL` / `ROUTE`, DELIBERATELY NOT `D1`/`D2`/`D3`.**
> An earlier revision used `D2` for the control repair while
> `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`'s **held** `D2` was live in the same
> thread — **one term naming two artifacts**, which is the exact defect class
> this node was cut to chase, appearing in its own labels. The control-repair
> commit `88893210a` carries the old label. **Renamed rather than annotated:**
> annotation leaves a satisfied criterion and an unsatisfied one looking alike.

**`MECH` — FIND THE MECHANISM. Upstream of the call site, not at it.** Which
producer emits a two-step exit chain for a body the lowering site reaches with
one computational frame, and **where does the disagreement actually live?**
**Both operands are internally consistent at `:7720`** — start where they are
produced, not where they meet.

> **`MECH` ADMITS THREE OUTCOMES AND MUST NOT BE READ AS A TWO-WAY CHOICE**
> (Architect, `evt_53mb86qj64fj1`):
>
>   1. the receipt is wrong, **or**
>   2. the eliminator frame set is wrong, **or**
>   3. **BOTH ARE CORRECT AND THE CUT BETWEEN THEM IS MISPLACED.**
>
> **An earlier wording of this deliverable asked "which of the two is
> inconsistent with the program?", which presupposes outcome 3 away.** That is
> the shared premise of the three refuted mechanisms, reappearing in the sentence
> meant to escape it — and the evidence points at 3: the frames equal
> `receipt[1..]` elementwise, and the index producer names the consumer's **own**
> occurrence, so both operands may be correct under one convention applied at the
> wrong place. **Do not re-narrow it.**

**State the DIRECTION and report counts with the population traced** (`AC-2`).
**"I could not determine it" is an acceptable answer** and must be reported as a
finding with its argument, not as an absence.

**`CTRL` — repair the co-class control. Independent of `MECH`; does not wait on it.**
`wrong_defining_call_breaks_the_required_consumer_edge`
(`abi_s6_mapping_file_backed_native.rs:463`) asserts one of **three co-class
reason strings**, all emitted as `CheckedIhDetachedCallerCut`:

    :9256   "is longer than the exact local eliminator prefix"
    :9263   "does not match a computational local prefix"
    :9269   "does not match the exact local eliminator prefix"

All three mean *the edge broke*, so the control **cannot distinguish "the guard
fell" from "the guard moved"** — and on 2026-09-14 it reported the second as the
first, producing a false refutation that reached an Architect ruling before
anyone opened the panic line. Assert the refusal **class**; if an exact arm is
load-bearing, assert it separately and say why.

**This is the durable product of the session and no correction touched it.**

### `CTRL` HAS THREE PARTS AND THEY DO NOT SHARE A SEQUENCE

Folded here rather than cut as a sibling: same control, same file, same guard, so
a sibling node would only lengthen the path (`§4e` — a duplicate is a fold, not a
frame). **But the parts have different dependencies, and leaving that implicit is
what the Architect warned this must not become — an unwritten expectation.**

**`CTRL-a` — assert the refusal CLASS. Independent. Startable now.** The repair
described above. Depends on nothing in this node.

**`CTRL-b` — RETIRE `D5bHs17PostCallConsumerMutation::ReplayCompletedSelectedExit`.
Depends on `R3`.** Under the anchor join, accessor choice becomes
**unobservable**: the extra leading step in the untrimmed chain is exactly what
the anchor skips, so both paths land on the same window against the same frames.
**The hazard did not become undetectable — it ceased to be a hazard**, and those
two look identical on a red dashboard, which is why the distinction is written
here.

**Put the reason in the enum's DOC COMMENT, not only in the commit message**
(Architect, `evt_5b93c7nrh14r0`). An inert mutation left in place invites a
future reader to "restore" the control by weakening the join — the worst
available outcome. The next reader asking *"why is accessor choice not tested
here?"* must meet the answer at the code.

**`CTRL-c` — COVER THE JOIN'S TWO REFUSALS. Depends on `R3`. BOTH, not one.**

    no-match         perturb a receipt step's (eliminator_origin,
                     checked_frame_id) so frame 0 matches nothing -> must refuse
    multiple-match   duplicate the frame-0-matching step -> must refuse on
                     non-uniqueness

**Why this is required rather than thorough: `R3`'s entire soundness argument
rests on these refusals, and nothing currently exercises either.** *Manufacture
cannot refuse, a witness can* — the join is admissible **precisely and only**
because it fails closed on no-match and on multiple-match. **A soundness argument
resting on a refusal no test reaches is the defect class this whole node exists
to chase.** If those paths are unreachable, or are later "simplified" to a
sensible default, nothing catches it and the type goes on proving something it no
longer proves.

**One mutation is not enough.** They are different refusals on different
conditions; a control exercising only the first says nothing about whether the
second arm is live — the same reason `AC-2` demands a population rather than a
verdict.

**The direction argument, since re-pointing a sentinel requires one:** pre-`R3`
the movable operand at these sites was **which accessor** was passed; post-`R3`
that is fixed by construction and what remains movable is **the correspondence
between receipt steps and frames**. **A mutation control must move the operand
the contract actually depends on**, and `R3` changed which operand that is. This
is a re-point *along the contract*, not to whatever is convenient.

**`AC-6` is not in tension.** It forbids synthesizing an operand in the
**production** path to make a check pass. A **test-only** mutation that corrupts
an operand to verify a **refusal** is the opposite, and is what the mutation enum
exists for.

### STANDING RISK [DISCHARGED 2026-09-14, see below]: `by_identity_join` WAS TAKEN ON A PRODUCTION PATH THAT NO FIXTURE MEASURED

**The record below is kept in its original tense deliberately.** It states the
risk as it stood, then discharges it. A heading asserting a condition that no
longer holds is how a node mis-frames a reader who never reaches the body — the
defect this node was recut twice to avoid — so the discharge is in the heading
and the reasoning is preserved intact underneath.

**`CTRL-c` does NOT cover this**, and that is why it is written here rather than
left as a line in the thread. The two join refusals are exercised at the sites
the fixtures reach; **this is not one of them.**

Verified at `0d94d58b6` — the ambient caller of
`apply_required_consumer_incoming_edge` exists in **both** feature profiles:

    :4339   #[cfg(feature = "px8-ds-test-support")]
    :4349       self.apply_required_consumer_incoming_edge(edge, eliminators)?
    :4356   #[cfg(not(feature = "px8-ds-test-support"))]
    :4358       self.apply_required_consumer_incoming_edge(edge, eliminators)?

**The same logical call under the two features.** ⇒ with the test-support feature
off — the shipping configuration — this path still runs and still takes
`by_identity_join`, **with no measurement behind it in any current fixture.**

**Accepted by the Architect at `evt_5951xezxy3c00`, for a stated reason that is
the point of recording it:** the join **fails closed**, so an unexercised path
taking it **cannot mis-compare silently** — it either produces a witnessed anchor
or refuses. **Defaulting that path to anchor 0 would have been the dangerous
choice**, because a default cannot refuse. This is the same discriminator as `R3`
itself, applied to the question of what to do about an unmeasured path:
**manufacture cannot refuse, a witness can.**

**Do not close this node by claiming the join is exercised everywhere it runs.**
It is not, and the argument for accepting that is the fail-closed property, not
coverage.

### DISCHARGED 2026-09-14 — and the transfer to the shipping profile is a CONSTRUCTION, not a sample

**The risk above was written about the VALUE** — *"the anchor is 0 where the
EDGE arm runs is NOT proved"* — and that is what is now discharged. One census
answered it and `ROUTE`'s refuting criterion together (`evt_2e6r1rza68k5w`): the
ambient edge-arm at `:4366` / `:4383` had no fixture behind it and now has **15
measured reaches across two venues, all `index = 0, anchor = 0`, all accepted**.

**The cfg objection, raised and then closed rather than waved through.**
`ken-cli` enables `px8-ds-test-support` unconditionally (`Cargo.toml:28`), so all
15 reaches are the `#[cfg(feature = ...)]` copy and the shipping
`#[cfg(not(...))]` copy is executed by no fixture. **The two copies are NOT
identical** — the Architect opened both at `ff7638ff6` rather than reasoning from
"it is the same logical call", which was a paraphrase that had been carrying the
discharge:

    test copy       an extra `record_required_consumer_call_selection(edge)?` before the
                    apply, and a PRECEDING arm gated on `d5b_hs17_post_call_consumer_mutation()`
    shipping copy   neither

**Neither difference can reach this question, and the reason is a construction:**

- The anchor comes from
  `AnchoredEliminatorWindow::by_identity_join(edge.executable_exits(), eliminators)`
  and the index from `edge.incoming_consumer_edge_index()`. **Both expressions
  are textually identical in the two copies**, over `edge` and `eliminators`,
  which are bound BEFORE the `cfg` split and are not derived from
  `static_transition_plan`. `edge` is live across that `&mut` call and the crate
  compiles, so the recording provably cannot mutate or invalidate either input.
- The preceding arm is gated on an INJECTED MUTATION SELECTOR, not on a property
  of the program being compiled, and when it fires it diverts to the residual
  path and never reaches the edge arm. It can neither divert an unmutated program
  away from the edge arm nor contaminate an edge-arm observation.

⇒ **The anchor and index at the edge arm are profile-independent by
construction, so the 15 measurements transfer.** What remains unmeasured in the
shipping profile is EXECUTION, not the VALUE. **Do not re-state this as "logic
measured, shipping profile not"** — that hedge sounds narrower and is less
accurate, and it invites the census to be re-run for nothing.

**Verified independently by the Steward at `ff7638ff6`, `core.rs:4337-4390`,
before recording the discharge:** `required_consumer_edge` is bound at `:4337`,
ahead of the `cfg` split; both edge arms call
`by_identity_join(edge.executable_exits(), eliminators)` verbatim; the HS17 arm
routes to `checked_ih_post_call_residual`, not to the apply.

**What this does NOT license.** The join is still fail-closed rather than
coverage-complete, and that is what made the pre-discharge posture defensible —
see `ROUTE`'s converse above for the one configuration (`anchor != 0` with
`index > anchor`) that no constructor has been shown to exclude.

**SEPARATE, NOT A DEFECT CLAIM, AND NOT THIS NODE'S** (Architect): the shipping
build does not call `record_required_consumer_call_selection` at all. Whether
anything downstream needs that record is unestablished. A recording that exists
only under a test feature is worth someone's eye on its own terms.

### `ROUTE` IS DECLINED. NOTHING TO BUILD. THIS IS A THEOREM, NOT A DEFERRAL.

**Architect ruling `evt_w1vnqexkhmxx`, recorded here rather than left in thread.**
`ROUTE` was framed as a CORRECTNESS deliverable — route the required-consumer
incoming edge at `:7720`, following `aggregates.rs:7826-7829`.
**It is declined, and the reason is not "it measured zero."**

**Routing is the identity wherever the anchor is non-zero, by construction, and
the anchor is non-zero exactly when the ambient stack begins after the defining
occurrence — which is the fact `R3` encodes.** `ROUTE`'s subject and `R3`'s
premise are mutually exclusive and cannot both hold.

The two numbers are independent joins, which is what makes the relation real
rather than a tautology — different needles into the same receipt:

    anchor (by_identity_join)     needle = eliminators[0]'s (static_origin, checked_frame_id)
    index  (SelfDefining producer needle = required_call.destination().consumer_occurrence()
            responses.rs:2400-2412)

`index=0, anchor=1` therefore says: the destination's consumer occurrence sits at
`receipt[0]`, the ambient window begins at `receipt[1]`, **and the route's cut
point lies strictly BEFORE the window.** `index` names the consumer's own
defining occurrence; the ambient stack begins after it; cutting a list at a point
preceding its element 0 yields the whole list. ⇒ **At any ambient site with a
non-zero anchor, routing is the identity structurally — not because this program
happened to measure 0.**

The converse closes it, and the strictness matters: for `ROUTE` to have content
at an ambient site you need **`index > anchor`** — routing removes an element
only when `index - anchor > 0`. The ambient stack merely CONTAINING the
consumer's own defining occurrence is the weaker predicate `index >= anchor`,
which is **necessary and not sufficient**: at `index == anchor` the definition
is the window's FIRST element, so containment holds and cutting there removes
nothing. That boundary case is not hypothetical — it is exactly what
ambient-apply measured 15 times on 2026-09-14 (`index = 0, anchor = 0`).

**Two disjoint ways routing fails to have content, and this node exhibits both:**

    :7720           index <  anchor   0 - 1   cut precedes the window; UNREPRESENTABLE
    ambient-apply   index == anchor   0 - 0   cut is the window's head; the IDENTITY

**What is NOT established, stated plainly because an earlier version of this
paragraph asserted it.** That earlier text said if routing ever had content
"the anchor would be 0 and `apply_required_consumer_incoming_edge` would
ACCEPT", and concluded the refusal and `ROUTE`'s subject are *exactly
complementary* so declining costs no capability. **`index > anchor` does not
force `anchor == 0`.** The anchor is obtained by identity join and `:7746` says
it is "never computed as `index + 1`", so "the stack begins after the defining
occurrence" describes what was measured, not what a constructor enforces. A
site with `anchor != 0` AND `index > anchor` would have routable content and
would be refused by `R3`'s guard — the one combination in which declining costs
capability. **No such site is known or measured, and no constructor excluding
it has been found.** That case is what the refuting criterion below exists to
find; it is not excluded by argument.

⇒ The decline stands on what is measured and derived: at every observed ambient
site routing is the identity, by one of the two routes above. It does not stand
on a proof that no other configuration exists.

**The `index - anchor` repair is not merely unmeasured, it is UNREPRESENTABLE.**
At `:7720` it is `0 - 1`, and no `usize` names a position before the window
starts. The semantically correct answer in that regime is "the whole window" —
which is exactly what not routing already does.

**THE CENSUS THAT WARRANTED `ROUTE` IS REAL AND WAS MIS-READ. RECORD IT AS A TYPE
DIFFERENCE, NEVER AS A MISSING ROUTE.** The two halves of the protocol operate on
**different lists**:

    aggregates.rs:7826   planner    applies the index to STEPS  (required_consumer_executable_suffix)
    core.rs:7757         lowering   would apply it to FRAMES    (the ambient eliminator stack)

The planner's index is coherent because the list it indexes IS the receipt.
Lowering's ambient list is not the receipt and does not share its membership
convention. **The asymmetry is not lowering forgetting to route; it is a type
difference between the two halves of the protocol.**

> #### WHAT WOULD REFUTE THIS RULING — stated in advance, per `AC-7`
>
> **If any ambient post-call consumer site has `index > anchor` — equivalently,
> `window_anchor == 0` with `incoming_consumer_edge_index > 0` in the anchor-zero
> case — then routing has real content at an ambient site and this ruling is
> wrong.**
>
> **The strictness is load-bearing and was wrong when first written.** The
> original general form read `index >= anchor`, which the 2026-09-14 census
> SATISFIES at ambient-apply (`index = 0, anchor = 0`, 15 reaches) while routing
> there is `[0..]`, the identity. A refuting criterion that fires on data all
> parties agree is inert is worse than no criterion, because the hit looks like
> evidence. Caught by `runtime-implementer` against the census run to check it.
>
> **THE CENSUS HAS BEEN RUN — 2026-09-14, `evt_2e6r1rza68k5w`, `ROUTEPROBE` over
> every edge-bearing ambient site in both venues.** No hit; no variation:
>
>     site                          observations          index   anchor
>     ambient-apply  (:4366/:4383)  11 mapping + 4 px8f     0       0
>     :7720                         11 mapping              0       1
>
> **The criterion is retained as a STANDING one, not as outstanding work.** It is
> not blocking, and it does not license the ruling — declining is
> behaviour-neutral. It is what would REOPEN the question at any site that
> appears later.

**`ROUTE` is the FOURTH proposed view of `:7720`, and like the first three it is
the identity at the site.** What is new is that the reason is now derived rather
than measured. This is the same predicate `§1b` already closed — *a receipt index
meeting a list with a different membership convention* — and declining `ROUTE` is
that closure holding, not a fourth refutation of it.

## `MECH` IS ANSWERED, AND THE RULING IS RECORDED HERE RATHER THAN IN THREAD

**Outcome 3: both operands are correct and the cut between them is misplaced.**
The two site families build their eliminator lists under **different membership
conventions** while sharing **one index convention**.
`checked_ih_post_call_eliminators` emits exactly one frame per step or fails —
no skip, no filter, both in-body branches are error returns — so at the
constructed sites correspondence holds by construction. At `:7720` the list is
the **ambient** local frame list, where it is an assumption.

`incoming_consumer_edge_index` names the position of the consumer's **own**
defining occurrence, uniqueness enforced (`responses.rs:2366-2396`). At
`index=0`, `[index..]` is a no-op and `[index+1..]` matches — which is why
mechanism 3 measured as vacuous.

### THE RULING (Architect, `evt_5k7jd5agh133j` + `evt_11zby14s19hkh`)

**`R3` — THE FRAME LIST MUST CARRY ITS ANCHOR.** The eliminator slice must carry
**which receipt element its element 0 answers to**, as a type;
`checked_ih_post_call_residual` takes that type and never a bare slice.

**Two rejected repairs, and why — both remain rejected:**

- **`R1`, give `:7720` a constructed list** — rejected because it makes the
  guard's only discriminating check a tautology. **`:7720` is the one site where
  this guard has teeth**, and weakening it by other means is still weakening it,
  even though no arm's condition changes.
- **`R2`, a bare `[index + 1..]`** — rejected as re-encoding the same unstated
  convention one site over.

**THE DISCRIMINATOR, which generalizes beyond this node: MANUFACTURE CANNOT
REFUSE, A WITNESS CAN.** `index + 1` is a function of the index alone — it
**cannot fail**, and yields an anchor that makes the counts agree whether or not
the frames relate to the receipt at all. The identity join **fails closed**: no
match or multiple matches, and it refuses. That is the whole test, and it is why
the join is a witness where arithmetic is a synthesized one.

**The anchor must derive from identity ORIGINATING AT PRODUCTION, never from
arithmetic over the quantities being reconciled.** The Architect's earlier
phrasing — *"obtained from where the ambient frames are produced"* — was
corrected as unbuildable: no receipt is in scope on the
`:3320 → :3364 → :3411 → :3798 → :7720` descent, and the anchor is a property of
the **(frames, receipt) pair**, which first exists at `:7720`. **The line is the
source of authority, not the location.**

`core.rs:3330-3332` already declares this tuple the shared derivation —
*"the checked bridge must carry this exact tuple, and two spellings is how they
part"* — so the join **consumes** the mechanism the code provides rather than
re-spelling it.

**REQUIRED CONDITION — the join fixes WHERE the window starts, never WHAT is in
it.** `checked_ih_post_call_residual` must still compare the full anchored
window `receipt[anchor..]` against the frames **including element 0**, and still
enforce the length relation. **Do not short-circuit element 0 as redundant
post-join** — that turns `:7720` into exactly the tautology `R1` was rejected for
creating. Preserved and non-definitional: the join's existence and uniqueness,
the length relation from anchor to end, and the tail compared pairwise.

**THE RESIDUAL, RECORDED AS A TRADE AND NOT AS A FREE REPAIR:** at a length-1
frame list the pairwise content check is entirely definitional after the join, so
the guard there reduces to existence, uniqueness and length. **That is weaker
than today's positional check — in a case where today's positional check is
wrong.**

**The type takes exactly two constructors and no general one from a bare
`usize`:** anchor 0 from a constructed list (justified by
`checked_ih_post_call_eliminators` emitting one frame per step or failing), and
the identity join (the unique receipt position matching `frame[0]`'s
`(static_origin, checked_frame_id)`, or refuse). **A `usize` constructor would
let `index + 1` back in through the front door and the type would prove
nothing.**

**Sequencing, approved:** land the anchor type with the constructed sites at
anchor 0 — behaviour-identical and independently verifiable — with `:7720`
untouched until the ambient half follows.

**`required_consumer_executable_suffix` and `:7720` must not be decided
separately.** They read the same cut, and `aggregates.rs:7826` consumes it too.

## Acceptance criteria

**`AC-1` — re-derive every line number and count at your own base.**

**`AC-2` — every instrument reports counts and the population it searched, never
a bare verdict.** Non-negotiable on this arc: a bare red produced a false
refutation today, and a zero-hit grep on a wrong path produced a false absence.

**`AC-3` — an enumeration must be closed at its PRODUCERS, not sampled.** The
`SelfDefining` fact generalized only when every construction site was enumerated
and found closed at two. **A larger sample of instances would never have licensed
it** (`evt_5ycnn29hy38qs`). Any "by construction" claim in this node's closure
must name its producer set.

**`AC-4` — `CTRL`'s repaired control must be SHOWN to discriminate** *guard fell*
from *guard moved*. Passing is not sufficient. **Point it at the ruling
(Architect, `evt_11zby14s19hkh`): it must show a wrong defining call STILL
REFUSES under the anchored path, and the join's new refusals — no match, multiple
matches — belong in the asserted refusal class.** The residual above makes this
the check that carries the guard's remaining teeth at a length-1 frame list.

**`AC-5` — the guard is untouched.** No change to the conditions at `:9256`,
`:9263`, `:9269`.

**`AC-6` — nothing is synthesized to make two counts agree.** No fabricated
receipt, step, eliminator frame, or index. Manufacturing either operand is
indistinguishable from fixing the defect and tests a shape no program presents.

**`AC-8` — A MUTATION'S EVIDENCE IS A PAIR. THE MUTATED RUN ALONE IS
UNINTERPRETABLE.** Every mutation result must be reported with its **unmutated
counterpart**. `anchor=1, window=1, eliminators=2` reads equally as *"the join
skipped an element to hide a mismatch"* and as *"the join correctly skipped the
caller-completed prefix"* — **only the unmutated run separates them.** A control
that reports one run is reporting a verdict, not a measurement. This is `AC-2`
specialized to mutations, and it caught a nearly-reported unsound `R3` on
2026-09-14 by firing against its own author's build.

**`AC-7` — a proposed mechanism must be stated with its falsification criterion
before it is measured.** Both refutations that landed today worked because the
proposer named in advance what would refute them. Adopt it as the standard here.

## COORDINATION §1a FIRED ON THIS CHAIN AND IS NOW DISCHARGED

**Chain: *"what is the mechanism of the `:7720` refusal."* Three stops, all
refuted by measurement, 2026-09-14.** The Steward's tracker is the count of
record and this is stop three. ⇒ **the Architect HELD on proposing a fourth
mechanism, and research was called for a prior-art advisory.** Both are
discharged; see immediately below.

**DISCHARGED 2026-09-15 — the Architect is NOT held.** Research returned about
four days early, delivered all three advisories, and the Architect dispositioned
them in one pass. **Advisory 2 is this chain** — the D5b *"receipt `N` vs
`M<N`"* question, which is this node's two-element receipt against one
eliminator frame (`evt_1cp7w7w3qqpa`; advisories 3 and 1 at
`evt_1x623f2dwats0` and `evt_96xz3xa5gvex`). Architect discharge:
`evt_40ebfajeaf9j8`. Nothing is queued.

This replaces a paragraph asserting the advisory was QUEUED UNCONSUMED against a
~2026-09-19 research recovery. That was true when written on 2026-09-14 and
stopped being true the next day; it is recorded here rather than deleted because
an implementer who read the node in between concluded the Architect was held
until the 19th.

**What the advisory left was not a fourth mechanism but one decidable read —
*what does `N` count* — and THAT READ IS NOW DONE** (Architect,
`evt_245m3hv4r8h88`, measured on
`origin/backup/ABI-S6-d5b-file-backed-0d94d58b6`; runtime-leader assigned it to
the Architect at `evt_5105pq70kbwym`). It was `MECH`-shaped — a measurement over
producers, not a ruling — so `§1a` would not have blocked it even while the hold
stood.

**THE DISSOLUTION HORN IS DEAD.** The fork's criterion was fixed before the
measurement, per `AC-7`: the arm dissolves only if a receipt element is
constructed one-per-resumption-step, and not if an element is built from a
binder, an occurrence, or a frame.

    N = expected.len()      CheckedIhPostCallConsumerStep, supplied as
                            edge.executable_exits()      core.rs:9256
    M = eliminators.len()   the AMBIENT local frame list at lowering

Producer set closed at one site (`AC-3`): `continuations.rs:6217` is the only
construction in `crates/`, and it pushes **one Step per `frame_origin`, with no
skip and no filter** — an absent consumer aborts the whole chain with `Ok(None)`
rather than omitting an element. So `N == frame_origins.len()` by construction.
⇒ **`N` counts static source consumer occurrences, one per caller-supplied frame
origin.** `spec/40-runtime/42-evaluation.md §6.2` — *"`k` is applied once, in
tail position, so no continuation is reified and no stack of suspended
resumptions is needed"* — is about **runtime resumption stacks**, which `N` was
never counting. It does not bear on this comparison. Refuted on the criterion
stated in advance.

**THE READ CORROBORATES `R3` RATHER THAN DISPLACING IT, and the asymmetry is
PHASE.** Both operands are now known internally exact — neither list skips — so
the defect is not in either list's internal discipline:

    N   planner-derived, FROZEN onto the edge at PLANNING time (executable_exits)
    M   the LOWERING-time ambient local frame list

Two enumerations of different populations, built in different phases, compared
through a shared index convention. That is what `R3` — the frame list must carry
its anchor — already addresses.

**THE NAMING TRAP, which is a plausible common cause of all three refuted
mechanisms.** The refusal text says *"a post-call consumer **receipt** is longer
than…"* and a type named `CheckedIhStaticResponseReturnReceipt` exists. **It is
not either operand.** It is five scalars — `boundary`, `selected_caller`,
`call`, `emission_transport`, `returned_word` — and has no length at all. The
word "receipt" in the message is loose prose for the *step list*. Anyone who
reads the message and greps `receipt` lands on a type that cannot be either
operand. **The message text is worth fixing**, and that repair belongs to
`CTRL`.

**WHAT IS STILL OPEN, and it is the node's next measurement:** why the two
populations diverge at the failing site. Sharply: *what is in `frame_origins` at
the failing call, and how does it relate to the ambient `eliminators` list?*
Both are reachable without a hypothesis — which is what makes this the first
step in the chain that is not a candidate mechanism.

**STALE COORDINATE IN THIS NODE'S OWN `title`, not yet corrected here.** The
title cites `lowering/core.rs:7720`; the guard is at `:9256`/`:9263`/`:9269` on
the branch, which is what `AC-5` below already names correctly. The fix is
deferred on purpose: `title` is frontmatter, so editing it regenerates
`IMPLEMENTATION-PROGRESS.md`, and a header candidate touching that file is in
flight. It lands as a separate edit once that candidate is on `main`.

**This costs the lane nothing, and the reason is a distinction worth stating
plainly: `§1a`'s hold binds the Architect's RULING, not the ring's
MEASUREMENT.** `MECH` is a measurement over producers and `CTRL` is a control repair;
**neither needs a ruling to proceed, and both are the whole of the ring's next
work.** So there is no five-day block to trade against, and the
rule-unaided-provisionally deviation taken on the HS24/entry-18 arc **does not
apply here** — its premise was a lane blocked on a ruling, and this lane is not.

**What the hold actually prevents** is a fourth mechanism entering the tree as
design authority on the same unexamined premise as the first three. **That is
worth holding for, because it is exactly what the last three hours produced.**

**This is not a judgment on any seat.** Two of the three were refuted on their
proposer's own stated criterion, which is the system working. `§1a` exists for
exactly this shape: a design chain that has absorbed three competent attempts
sharing an unexamined premise. The implementer declined to propose a fourth and
named the pattern in their own record; the Architect named the shared premise in
theirs.

**Distinct from the `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `D2` chain** (*"how does
`D2` obtain discriminating evidence"*), which stands separately at **two**.

> ### THREE DISTINCT CHAINS RUN THROUGH THIS LANE. DO NOT MOVE A FACT BETWEEN
> ### THEM — THE STEWARD DID, AND IT NEARLY REACHED THE OPERATOR.
>
> | chain | count | advisory | status |
> |---|---|---|---|
> | *"what is the mechanism of the `:7720` refusal"* — THIS node | stop **3** | **DELIVERED 2026-09-15 AND DISPOSITIONED** — advisory 2, `evt_1cp7w7w3qqpa`. The cell formerly read *"QUEUED UNCONSUMED at the quota-dead research seat"*; research returned about four days early. | it did NOT contradict the measurement. It left one decidable read — *what does `N` count* — now taken (`evt_245m3hv4r8h88`): the dissolution horn is dead and the result corroborates `R3`. |
> | *"how does `D2` obtain discriminating evidence"* — [[RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]] | stop **2** | none called | separate |
> | ABI-S6's HS chain, of which **HS18** is a member | HS18 is stop **18**; its `§1a` was the **6th** trigger | **DELIVERED in three parts and CONSUMED** (`evt_2qdk2a3qwa4hs`, `evt_71qmyd36rwygd`, `evt_72yy838768mgk`) | the Architect states it is RULED TWICE — `evt_4ms8rwyhnvgs9`, `evt_1hnd8tta02f59`, plus a corrected Deliverable 2 at `evt_7eqc0hmbanyzm` |
>
> **The dead-seat advisory belongs to the FIRST row only.** The Steward wrote
> that HS18 was *"held at stop 3 behind an advisory queued against a seat that is
> quota-dead"* — **one true fact about one chain, applied to another.** Caught by
> the Architect at `evt_4twx5ypjqe0eh` before it reached Pat, where it would have
> put the block on a seat that had answered nine hours earlier and pointed the
> remedy at a 2026-09-19 reseat that changes nothing.
>
> **This node's own `§1a` section above is where that sentence was refuted** — it
> already said the queued advisory is the MECHANISM chain's, and that `§1a`'s
> hold binds a RULING and not the ring's MEASUREMENT. The Steward authored that
> and then contradicted it in a post the same hour.
>
> **The HS18 row is recorded as the Architect's statement, not as the Steward's
> finding, and the Architect has since REPLACED the caveat that sat here.** The
> earlier version said the three rulings "predate `R3` landing, so whether the
> projection deliverable survives the current tip is unmeasured. Check the three
> events against the tip." **That instruction was wrong, and so is the
> supersession reading offered in its place.** The three events have three
> different dispositions and must not be carried as a set:
>
> - **`evt_1hnd8tta02f59` is VOID, and not by supersession.** Its ruling ("ADD
>   the projection on the deficient route") was issued with three pre-committed
>   outcomes on a precondition read: **(a)** `ITree::Ret` carrier ⇒ the ruling
>   stands; **(b)** already a `Result` carrier ⇒ STOP, the Architect re-rules;
>   **(c)** already-projected contents ⇒ STOP, the Architect re-rules. The read
>   came back **(b)** — `686ffa8ac`'s own subject is *"restore landed D6a route
>   -- precondition read is (b)"*. The ruling self-voided on a branch written
>   before the evidence arrived. **It was then established that (a) was
>   STRUCTURALLY UNREACHABLE** — D6a's guard is
>   `if let Some((_return_index, _return_case)) = return_case`, so the one-binder
>   `::ITree::Ret` case was necessarily compared and missed, and **no value
>   reaching the D6a jump can be a Ret carrier on any route, ever.** The repair
>   would have projected field 0 of a non-Ret carrier for every program. There is
>   nothing to check against the tip, and that is the correct reason.
> - **`evt_4ms8rwyhnvgs9` is DISCHARGED, not withdrawn.** What was withdrawn
>   inside it was the static `Ok` selection, with Condition C revoked; its
>   operative act was to redirect the arc UPSTREAM, which is what produced
>   `evt_1hnd8tta02f59`. A withdrawal *in* a ruling is not a withdrawal *of* it.
> - **`evt_7eqc0hmbanyzm` IS LIVE and nothing here touches it.** Its corrected
>   Deliverable 2 — trap identity must carry its EMISSION COORDINATE, under a
>   DISCRIMINATION acceptance criterion that the originally proposed fix FAILS —
>   is the Steward's own stated input to `RT-TRAP-IDENTITY-EMISSION-COORDINATE`
>   `D1`, the one item in lane 1 with a clean base. **Recording all three as
>   superseded would kill the basis of the lane's only pullable work.**
>
> **The shape, for the fourth time in this lane today: one true fact about one
> member of a set, applied to the set.**

**THE `ROUTE` DECLINE DOES NOT ADVANCE THIS COUNT, AND THE STEWARD RECORDS IT AS
SUCH.** `evt_w1vnqexkhmxx` is hard-stop **one** on *"should `:7720` route"* — a
different design question from the mechanism chain, which closed at three. **No
research trigger fires.** Recorded here because the Steward's tracker is the
count of record and a decline that looks like a fourth refutation would otherwise
be read as one: the mechanism chain's stops were accounts that measurement
killed, whereas `ROUTE` was declined by DERIVATION from a fact `R3` had already
established. **A closure holding is not a stop.**

## `MECH-2` — THE DIVERGENCE MEASUREMENT. Released to Team Runtime 2026-09-16.

Framed at runtime-leader's request (`evt_7hbthwggypwqr`) on the Architect's `N`
read (`evt_245m3hv4r8h88`). **Size S. Tier T2 — this is a read, not a design
call; the design question it feeds is `R3`, which is already ruled.**

**No urgency on dispatch.** The runtime implementer ran the full `#3676` arc and
the header fix in one night; runtime-leader flagged that explicitly. This is the
lane's next item, not tonight's.

**The question, and it is answerable without a hypothesis:** at the failing call,
**what is in `frame_origins`, and how does it relate to the ambient
`eliminators` list?** `MECH` established that neither list skips internally, so
the divergence is between the populations, not inside either one.

    N   planner-derived, FROZEN at PLANNING time   edge.executable_exits()
    M   the LOWERING-time ambient local frame list

**Read-only, on `origin/backup/ABI-S6-d5b-file-backed-0d94d58b6`.** Reading an
unmerged ref is permitted; **building on one is not** — and this node's base is
held (see the Deliverables banner), so there is nowhere for a fix to land even
if the read suggested one. **The deliverable is a recorded answer in this node,
not a code change.** A candidate arriving with a repair is out of scope and will
be sent back.

**`AC-M2-1`. The two populations are enumerated at the failing site**, each
named with the phase that built it and the code that built it. Not a count —
the members.

**`AC-M2-2`. The relation between them is stated as one of: subset, disjoint,
overlapping, or same-set-different-order** — and the evidence is the member
lists from `AC-M2-1`, not a length comparison. `N` and `M` being unequal is the
symptom already known; it is not an answer.

> **DISCHARGED 2026-09-16 BY THE PRODUCERS, NOT BY MEMBER LISTS.** The evidence
> clause above names `AC-M2-1`, which was retired as subsumed. The relation was
> established from the two producers and their ordering rules instead — which is
> the stronger route, universal in provenance where member lists from one run
> would be one instance. Read the evidence clause as superseded, not unmet: the
> answer section below supplies what it was asking for.

**`AC-M2-3`. The criterion is fixed BEFORE the measurement** (`AC-7`, which this
chain has now honoured twice). State in advance what result would show the
divergence is *not* a phase artifact, so a green read cannot be retrofitted into
whichever mechanism is convenient.

**`AC-M2-4`. Coordinates are resolved to their enclosing function**, not cited
as bare `file:LINE`. Three mechanisms died in this chain on premises that a
line number made look checked. The guard is at `:9256`/`:9263`/`:9269`, the sole
producer at `continuations.rs:6217`.

**`AC-M2-5`. If the read corroborates `R3`, say so and stop.** Corroborating the
standing ruling is a complete and successful outcome. This node exists to find
the mechanism, not to produce a fourth candidate — `§1a` fired at three for that
reason, and the discharge above did not reset the count.

**`AC-M2-6`. Name the comparison projection BEFORE enumerating either
population, and it must be the pair the guard actually uses.** At
`core.rs:9269-9270`, inside `checked_ih_post_call_residual`:

    M side   (frame.static_origin,                    frame.checked_frame_id)
    N side   (step.occurrence().eliminator_origin(),  step.checked_frame_id())

Both `checked_frame_id` components are `Option<u64>`. **Enumerating under
`StaticOriginId` alone can report "same set, different order" while the
`checked_frame_id` half diverges** — including the `Some`/`None` distinction,
which carries meaning: a Step's marker is derived from the source wrapper whose
sole body is that occurrence, so `None` asserts there is no such wrapper. A
relation stated under a partial projection is the defect class this chain
already died of three times — a comparison that looks checked.

**`AC-M2-6` is load-bearing rather than defensive, and this node already holds
the evidence** — `:68-70` above: *"when a probe equalized the counts, the
refusal moved to the per-frame content arm and the wrong defining call still
failed to compile."* ⇒ **Equalizing `N` and `M` does not clear the refusal.**
The count divergence is a symptom, not the defect, and the pair arm is where the
defect goes once the count arm is satisfied. *"Same set, different order"* is
not a hypothetical wrong answer here; it is the answer this site has already
produced once under a probe, and `AC-M2-5` would not catch it because it
corroborates nothing.

**`AC-M2-4` extends from enclosing FUNCTION to enclosing CALLER**, because the
guard is a shared callee. `checked_ih_post_call_residual` has four distinct call
sites, one of them duplicated across a `cfg`:

    :4345   replayed.as_slice()              behind d5b_hs17 mutation support
    :4351   caller_completed_exits()         cfg(px8-ds-test-support)
    :4360   caller_completed_exits()         cfg(not(...))  -- same call, cfg twin
    :7720   selected_case_exits()            THE FAILING SITE
    :9239   executable_exits()

⇒ **Naming an arm is not naming a site.** `:9256`/`:9263`/`:9269` name behaviour
reached from five places with three different `expected` slices. And **`:7720`
is the CALL SITE, not a stale coordinate for the guard** — it is the line that
calls this function, the node uses it as the site throughout, and `:56` carries
the diagnostic's own emitted `site=7720` field.

**Population provenance, which closes a gap rather than opening one.** The `N`
read was taken at `:9239`, where the argument is `edge.executable_exits()`,
while the failing site passes `consumer.selected_case_exits()`. The conclusion
survives, and here is its carry argument:

- `executable_exits()` is a **suffix** of `selected_case_exits` only under
  `CallerCompleted`; under `SelfDefining` it returns the same slice. This node
  records at `:22-30` that `:7720` is reached **only** under `SelfDefining` and
  that `CallerCompleted` is unconstructible (`evt_5ycnn29hy38qs`).
- At `responses.rs:2189` the two fields are one vector routed by a switch:

      let (consumers, selected_case_exits) = if detached_return_context.is_some() {
          (Vec::new(), derived_steps)      // DETACHED  -> selected_case_exits
      } else {
          (derived_steps, Vec::new())      // ordinary  -> consumers
      };

  **Exactly one is ever non-empty**, and the failing site reads the detached
  branch — which is what the refusal class `CheckedIhDetachedCallerCut` is
  named for. So `N`'s population is the **detached** step list while `M` is the
  ambient lowering frame list, which is not partitioned that way at all.

⇒ **A closed producer set for the ELEMENT TYPE does not close the population of
a particular FIELD.** Two fields of one struct hold
`[CheckedIhPostCallConsumerStep]`; state which one the site reads.

### `MECH-2` ANSWERED 2026-09-16 — not a phase artifact, and it corroborates `R3`

Measured read-only on `origin/backup/ABI-S6-d5b-file-backed-0d94d58b6`. No code
change; `AC-M2-5` invoked, `§1a` stays at three.

**The projection and the criterion were fixed BEFORE either population was
enumerated** (`AC-M2-3`, `AC-M2-6`), posted at `evt_axawb6te9h2w`, with the
Architect's shape (d) added at `evt_9tsyt5hmzez5` before the measurement:

    M side   (frame.static_origin,                    frame.checked_frame_id)
    N side   (step.occurrence().eliminator_origin(),  step.checked_frame_id())

`Option<u64>` on both `checked_frame_id` halves, `None` a value and never a
wildcard. A phase artifact requires `N` to be a positionwise prefix of `M` under
the full pair, with the difference explained by WHEN a list was captured. The
four shapes that are not a phase artifact, named in advance: (a) disjoint,
(b) overlapping without containment, (c) agreeing on `StaticOriginId` and
differing on `checked_frame_id` including `Some`/`None`, (d) same members under
the full pair but not in positionwise prefix order.

#### The guard asks an ORDERED question, which is what (d) exists for

`checked_ih_post_call_residual`, `lowering/core.rs:9251-9279`:

```text
:9256   if eliminators.len() < expected.len()                     -> refuse
:9262   for (frame, step) in eliminators.iter().zip(expected)
:9269   frame.static_origin != step.occurrence().eliminator_origin()
        || frame.checked_frame_id != step.checked_frame_id()      -> refuse
:9277   Ok(&eliminators[expected.len()..])
```

Positionwise from index 0, residual taken after `expected.len()`. **Set
containment is necessary and not sufficient.**

#### The two populations, by producer and by ordering rule

`AC-M2-4`, enclosing function and caller: the site is the CALL at `core.rs:7720`
inside `lower_computational_producer_construct` (spans `6735-7766`), not the
guard's four other callers.

`AC-M2-6`, the field rather than the type: at `responses.rs:2189` one vector is
routed by a switch, and `selected_case_exits` is non-empty only when
`detached_return_context.is_some()`. The failing site reads that field, so
`frame_origins` came from `detached_post_call_consumer_frames` and **not** from
`checked_ih_post_call_consumer_frames`.

```text
M   eliminators
    a PARAMETER of lower_computational_producer_construct, used from
    eliminators[0], not assembled or reordered between :7663 and :7720.
    One function's local lowering-time eliminator stack.

N   consumer.selected_case_exits()
    derive_checked_ih_post_call_consumer_chain, continuations.rs:6188-6224.
    ONE step per entry of frame_origins, in frame_origins order:
      occurrence       = post_call_consumer_in_frame(plan, frame_origin, actual)
      checked_frame_id = checked_frame_for_consumer(plan, frame_origin)

    frame_origins built by detached_post_call_consumer_frames,
    responses.rs:2065-2111, pushing in this order:
      :2070/:2075  context.steps() REVERSED, the ComputationalMatchCase parents
      :2095        target.key.continuation_origin
      :2098        source.eliminator_origin()        conditional
      :2104        required.eliminator_origin()      conditional
      :2108        RECURSES into boundary.caller_context()
```

#### `AC-M2-2`: the relation, and why no re-timing reaches it

**`N`'s last action is to recurse across the worker-return boundary into the
CALLER's context and keep appending.** `N` is an ordered chain assembled by an
outward walk across contexts; `M` is one function's local slice in lowering
order.

**AND THE RECURSION IS ENTAILED, NOT GUARDED.** The `:2078` test
`if let Some(boundary) = context.worker_return()` reads as contingent and is
not, for the context the failing site supplies. At `continuations.rs:6626` the
registration is

```text
} else if result_position.return_context.worker_return.is_some() {
    let detached = result_position.return_context.clone();
    ... insert into pending_detached_return_contexts ...
```

so `worker_return.is_some()` is the **condition under which a detached return
context exists at all**; and `selected_case_exits` is non-empty only when
`detached_return_context.is_some()` (`responses.rs:2189`). ⇒ **`N` crosses the
boundary by definition of the field the failing site reads**, not as a path the
code usually takes. The recursion fires at least once on every population this
site can present. (The walk terminates where an inner `caller_context()` has no
`worker_return`; the entailment is about the registered top-level context.)

Nothing constructs `N` as a prefix of `M`. The guard asserts the relation by
zipping and refuses when it does not hold. **Two lists built by different
traversals over different structures, one of which leaves the function, have no
reason to agree positionwise from index 0.**

Against the pre-registered criterion this is **not a phase artifact**: the extra
`N` members are not frames lowering has yet to install, they belong to a
different context reached by recursion. That is a scope difference, not a timing
one. It also fails (d) independently — even with membership equalized, `N` is in
outward-walk order and `M` is in local lowering order.

**`:68-70` is the confirming experiment already in this node.** Equalizing the
counts moved the refusal to `:9269`, the positionwise pair arm. Count-equality
did not deliver prefix agreement, which is what (d) predicts.

#### `AC-M2-5`: this corroborates `R3`, and that is the outcome

`R3` requires the eliminator slice to carry which receipt element its element 0
answers to, as a type. **This read supplies the mechanism behind that
requirement:** element 0 cannot be assumed to correspond because the two lists
are produced by different walks over different structures, and one of them
crosses a function boundary. The anchor is needed because the positional
presupposition is false **by construction**, not by timing — which is also why
`R1`'s constructed list and `R2`'s `index + 1` cannot substitute for it.

#### `AC-M2-1` — RETIRED AS SUBSUMED. Do not re-dispatch a run for it.

Ruled independently and identically by the Steward, who authored it at
`57d049190`, and by the Architect, who holds this node's gate.

`AC-M2-1` asks for the MEMBERS. The concrete `(StaticOriginId, Option<u64>)`
pairs are plan values at the failing site, obtainable only from a run, and this
node forbids building on the backup ref; the recorded receipts carry counts and
no member list.

**It is subsumed rather than undischarged because the relation was obtained by
the stronger route.** The two producers and their ordering rules are universal
in provenance: they fix what `N` and `M` can contain and in what order, for every
population this site can present. **An enumeration from one run would corroborate
a single instance and could not generalize past what the by-construction answer
already covers.**

⇒ A later reader should not read the absent member list as a gap. There is no
run to dispatch, and `AC-M2-2`'s answer does not depend on the concrete values.


## Obligations this node creates elsewhere

**The 11 base reds were never a stable population.** Every measurement over
`abi_s6_mapping_file_backed_native` on 2026-09-14 inherited this refusal as a
mask. [[RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY]] records a venue
finding *"the mapping fixture cannot reach the code at all — zero population"* —
**a compile dying early here produces exactly that reading.** Re-take it after
this node lands, **before** that node's `D1` is answered from it.

### CLOSEOUT DELIVERABLE — DELETE ANOTHER NODE'S BANNER WHEN THIS ONE LANDS

**[[RT-DISCHARGE-ARM-SUBSTITUTES-PLAN-FOR-OBSERVATION]] opens with a banner
saying every coordinate in it is on an UNLANDED candidate and none of the code
is on `main`** — `independent_contract` 0 hits, `realized_call_words` 0 hits,
the arm-1 conditional absent. **That banner becomes FALSE the moment this node
lands**, and it is the first thing a reader of that node sees.

⇒ **Delete it after this node lands, BEFORE any team pulls that node.** Replace
it with the landed SHA, or strike it; either way it does not survive this merge.

**WHY IT LIVES HERE RATHER THAN THERE.** That banner is not machine-checkable
and nothing detects it going stale — the issue schema has only `depends_on` and
`blocks`, both ranging over issue ids, so *"written against an unlanded base"*
has nowhere to live in frontmatter, and a migration across 626 files for one
instance is not worth it.

**So the obligation is filed against the node that CAUSES the staleness rather
than the node that SUFFERS it.** Whoever lands D5b is reading this node at the
moment the banner goes false; a reader of the other node has no reason to be.

> **THE GENERAL FORM, AND ONE CLAUSE OF IT IS NOT TRUE.** When a fact in A goes
> stale because of an event in B, move the obligation into B's own closeout —
> **not because that removes the dependence on someone looking, but because it
> re-points it at the party who cannot avoid looking.**
>
> **It still relies on a reader's attention.** What changed is WHICH reader,
> and "no reliance on attention" or "guaranteed to be looking" would be
> **dressing a reduced reliance as an eliminated one** — the same overclaim as
> the fourth cell's *"the independent test that the grep missed no writer"* and
> as `parent == origin/main`. Three in one evening, all in the direction of
> describing a check as CLOSING what it only NARROWS.

Architect `evt_227hnhfawfeqs`, `evt_7v53ykjzp94gc`; Steward-filed. **Not filed
under `Deliverables`**, whose own header at `:122` reads *"a RECORD, never a
queue... none is pullable"* — an obligation filed in a list marked
non-actionable gets read, not done.

## Sizing

**`M`, T1.** `CTRL` is small and independent. `MECH` is a reasoning deliverable over
producers; if it becomes an unbounded search, stop and report — that is `MECH`'s
stated acceptable outcome, not a failure.

## Not this node

- **Not any of the three refuted mechanisms**, re-proposed under a new name.
- **Not a repair of any refusal arm.** All three work.
- **Not `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` or its discharge ledger.** The nine
  *"finished generated-Result proof graph is not closed"* refusals behind this
  one are that mechanism — adjacent, and not evidence about the collision shape.

## INHERITED OBLIGATION from `RT-D5B-LIVE-WIRING` (slice 3): `AC-2b`

**This is not a new requirement on the investigation. It is one acceptance
criterion that could not be discharged where it was written, parked at the node
whose work makes it dischargeable.**

Slice 3 wires `publish_immediate_bridge_realization_plan` into
`construction.rs` immediately before
`install_static_response_context_plan_phase_b()`. The reference's comment at
that site claims **an ordering with two bounds**:

    lower   after continuation identities, source occurrences and transports
            are final          -> FALSIFIABLE at slice 3, and tested there
                                  (AC-2a: red when moved above :1406)

    upper   before response phase B "can decide whether an owner exists"
            -> NOT falsifiable at slice 3

**Why the upper bound is inert today**, measured at `main`:

    derive_ reads   abi, continuation_contexts, continuation_specializations,
                    continuation_specialization_calls, child_static_origin,
                    planned_occurrence_expr
    phase B writes  static_response_plan_installed,
                    static_response_continuations, static_response_deferred
                                                       (responses.rs:2296-2330)
    => DISJOINT

    production references to immediate_bridge_realizations:
      static_transition.rs:590  declaration      construction.rs:296  init
      immediate_bridge.rs:679   inside validate_, DEAD
      immediate_bridge.rs:694, :701   accessors
    => NOTHING on the live path reads the field

⇒ Moving the call after phase B changes no input and no observable output.
**The consumer that would make the upper bound matter is
`CheckedIhPostCallConsumer` — this node's subject.**

### The enabling condition, stated so it is checkable

**`AC-2b` becomes due when a live consumer reads
`immediate_bridge_realizations`.** At that point a test that moves the wiring
to after phase B must go red. Until then it cannot, and a candidate that
appears to satisfy it is testing something else.

**The wrong repair, named in advance.** A test-only ordering probe — a hook
recording that the call happened before phase B — satisfies the words while
testing the probe, because there is no behaviour on the other end of it. **Ask
what observable differs, not whether the assertion passes.**

### One open measurement that could pull `AC-2b` forward

`with_d5b_hs10_bridge_plan_mutation` (`immediate_bridge.rs:559`) is a
feature-gated plan mutation hook. **If it can perturb the plan between the
wiring point and phase B, an ordering observation with a real behavioural end
may be constructible without this node's mechanism.** Nobody has measured what
it reaches; slice 3 is asked to report it. If slice 3 finds it does reach,
`AC-2b` is discharged there and this section is struck.
