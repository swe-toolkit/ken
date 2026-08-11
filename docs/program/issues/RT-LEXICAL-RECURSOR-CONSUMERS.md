---
id: RT-LEXICAL-RECURSOR-CONSUMERS
title: "Repair the LexicalCallArgumentRecursor consumer population on the functionized lane, activated by B-only exclusion before the retirement removes the seam"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-MATCH-RECURSOR-CONSUMERS]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Architect ruling evt_5w09dcwbf7k70 (2026-08-08) on RT-RECURSOR-TRANSPORT hard stop 4, narrowed to rows 1-5 by the re-rule evt_3r4j14fv1jtj2 on the nine-expression census evt_16cmej481q7ns. Campaign docs/program/16-recursive-descent-retirement.md node #6d. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THIS NODE IS ROWS 1-5 ONLY. ROW 6 IS NOT ITS WORK.
>
> **Narrowed 2026-08-08 by Architect re-rule `evt_3r4j14fv1jtj2`**, on the
> measured census. An earlier revision of this file claimed all six red rows and
> asserted they shared `host_result_closure_match`. **Both claims are false and
> are withdrawn.**
>
> Row 6 (`d8d`) enumerates exactly `{MatchScrutineeRecursor}` — **it was never in
> this node's population.** It belongs to [[RT-MATCH-RECURSOR-CONSUMERS]].
>
> **Do not fold the two nodes back together.** The exact residual producer, the
> activation hook, the observed boundary and the completion owner all differ. If
> the two `D1` causal partitions later prove one exact shared production root,
> **route a subsumption proposal before coding** — it may not be inferred from
> shared retirement timing or shared syntax.

> # FILED `draft` ON PURPOSE — RELEASED WHEN THE A NODE MERGES
>
> The frame is written and shovel-ready; the node is held at `draft` only to
> enforce the ruled release order. **The A node goes first** because it closes
> the Position-A claim that the `D2` record correction is narrowing, and the
> fleet runs one Runtime ring at a time.
>
> ⇒ **Nothing here is owed further analysis.** The moment
> [[RT-MATCH-RECURSOR-CONSUMERS]] merges, this flips `ready` and releases.

## What it is

**Eight expressions across five test families**, previously green, that fail
closed on the functionized lane once [[RT-RECURSOR-TRANSPORT]]'s `D3` retires
the `LexicalCallArgumentRecursor` residual class:

| row | fixture family | expressions |
|---|---|---|
| 1 | `owned_scope_deletion` | 1 |
| 2 | `all_three_producer_paths` | 1 |
| 3 | `siblings_share_an_origin` | 1 |
| 4 | `scope_segments` depth 1, 2, 3 | 3 |
| 5 | `selected_scope` before / after hole | 2 |

Every one enumerates **exactly `{LexicalCallArgumentRecursor}`**, and every
unexcluded compile returns `Ok` — so each row's red is produced by the lane
change and not by a fixture that was already broken.

This node repairs that consumer population **on the pre-retirement tree**, so
`D3` can then retire the class and prove these rows green with no exclusion hook
and no `#[ignore]`.

## The activation seam, measured rather than assumed

**B-only exclusion is this node's seam and it is proven.** At exact `D2`
`8efdfdb3`, excluding **only** `LexicalCallArgumentRecursor` leaves the residual
set empty for each of the eight expressions, so the selector reaches
`FunctionizedUnits` while production continues selecting `RecursiveDescent`.
Every row carries a real activation denominator — a compile through
`px8j_capture_source_trace` — so a refusal cannot be credited where the harness
never reached the path.

⇒ **The repair can be built and proven before the retirement**, which is what
makes this an independently mergeable node rather than a quarantine.

> **The seam was asserted before it was measured, and it nearly set the
> sequencing.** Both the Architect and the Steward stated that the existing hook
> activates all six fixtures; that was a candidate promoted to a fact. The hook
> removes B from the **complete** residual set and reaches `FunctionizedUnits`
> **only when the remainder is empty** — true for these eight, and inapplicable
> to row 6, whose set never contained B at all. The census
> (`evt_16cmej481q7ns`) is the object of record.

## The population is the production predicate, not this list

**`D0` closes the population from the production `LexicalCallArgumentRecursor`
predicate.** The eight measured expressions are a **floor**, not a perimeter.

Sweep every compilation entry that can supply the predicate. Helper spelling,
snake_case fixture spellings and `BodyEmissionAuthority::RecursiveDescent`
assertions are **candidate selectors, not closure** — a grep tells you which
fixtures might be in the family, never what any one of them enumerates.

## Size

**`M`, provisional.** It is a scoping figure taken from a symptom count, and
five rendered refusal strings are **not** five proven causes.

**`D0`/`D1` are authorized to return a partition instead of a repair.** If the
causal partition finds materially distinct authorities rather than downstream
symptoms of one root, or any repair needs a new planner/ABI population, **return
the partition before coding.** Do not silently turn one symptom-named node into
five repairs.

## The edge that is not in the frontmatter

This node's base is **post-`D2`-correction `main`**, and that correction is a
*partial* merge of [[RT-RECURSOR-TRANSPORT]], not its completion. A `depends_on`
naming that node would be a **cycle** — its `D3` is blocked on this one.
`depends_on` is empty and the base is stated in prose and in the frame.

⇒ **Read the base from the frame, not from the edge.** The machine-checked edge
that matters is `blocks: [RT-RECURSOR-TRANSPORT]`.

## What is ruled and not reopened

- `10369776252861e8b15e613576256a3682c70066` is **held evidence only** — not a
  candidate, not a repair base, not to be continued.
- **Zero new `#[ignore]`.** The Steward ruled these quarantinable at
  `evt_7vhjcstd37a50`; that ruling is **withdrawn** and was not revived by any
  later correction.
- The old-green semantic controls are **not disposable**. Surface-Ken
  reachability is unproved; old-green runtime capability is **proved**, and
  these rows are the only probes for the guards they exercise.

## `D2f` ABI-class accepted partial — MERGED 2026-08-11, PR #1897

Exact `006730d4085a04e95dc6b2ca7bebe19d1fbcb6d4` from declared base `84a8f66d`;
one commit, six paths, `+285/-35`, no added ignores. M6 blob identity 6/6 MATCH.
**This node stays `active`.**

> ### IT CLOSES ZERO OF THE EIGHT EXPRESSIONS, BY DESIGN
>
> **A landed partial on a node with 27 merges reads as progress against the row
> count unless it says otherwise.** This one is a **structural prerequisite**:
> the fail-closed `StaticContinuationFusion` ABI class and
> `ContinuationEmissionOwner::Fusion`, with every consumer disposition refusing.
>
> **No constructor, no emitter, no source-body emission authority, no redirected
> producer edge, and no fusion runtime behaviour.** Verified on the object:
> seven `Err(` sites added and **zero** panic-style macros, so the class refuses
> rather than traps; and the only two matches for redirected-edge or emission
> vocabulary are **doc comments stating their own absence.**

## Second `D2f` partial — identity-plane wiring, MERGED 2026-08-11, PR #1899

Exact `1b362f5ea3201ba4dc54d74f0dc88462e3fa4f19` from declared base `e0e4aeb3`;
one commit, five `ken-runtime` paths, `+123/-4`. M6 blob identity 5/5 MATCH.

**The landed fusion identity plan now reaches the sole production compile
path**, with a causal arrival control. **Empty resolution remains legal.**
Excludes definitions, descriptors, authorities, edge redirection, emitter
behaviour, planner signature widening, and every emitter AC claim.

**This one also closes zero of the eight expressions.** Two partials, two
structural prerequisites, zero rows.

> **What its control proves, CORRECTED 2026-08-11.** *"Empty resolution is
> legal"* is exactly the condition that lets an arrival control pass for free,
> so what the control does and does not establish is worth stating exactly.
>
> **It proves arrival, and that half is real.**
> `NonZeroUsize::new(planes.len()).expect(...)` panics on an empty drain, so
> **a compile that never reached the production builder fails the control
> rather than passing it.** Resolved sizes are **recorded, not pinned** — this
> witness plans no admitted fusion, so a control asserting resolution *success*
> would have been either vacuous or wrong.
>
> **STRUCK — *"the assertion equates the recorded planes with the established
> arrival count as one population."*** **That is false, and it was published
> here and in PR #1899's body.** `planes` is read **once**; `reached` is
> `NonZeroUsize::new(planes.len())`; the assertion then compares `planes.len()`
> to `reached.get()`. Both sides come from that one read, so it is
> `planes.len() == planes.len()` — **a tautology, not a second measurement.**
> Adversary-measured; Steward disposition `evt_7ewdkteptjr8t`.
>
> **An equality is a measurement only if its two sides come from different
> reads.** No count of intervening named bindings changes that, and naming the
> doubt in the merge notification after the fact was not the same as checking
> it.
>
> **The repair is folded into the `D2f` emitter increment, not re-reviewed
> here** — the merged code is not wrong, only weaker than the sentence above
> claimed. The honest form is to drop the `assert_eq!` and keep the `expect`;
> **a second counter manufactured to make the equality look measured is the
> cosmetic repair and is forbidden.** A real equality becomes available only
> once a fusion resolves, at which point builder arrival and resolved-plane
> population are genuinely independent quantities.

**The `D2f` emitter is the next increment**, scoped to the one `R3` before-hole
witness — **not** an eight-row repair. The `R3` after-hole / missing-`Mint` cell
is excluded from `D2f` and owned by [[RT-LEXICAL-ROW2-MISSING-MINT]].

**Measured remainder** (runtime-leader, `evt_645tm43wf1cne`): the `D2f` emitter
plus its review cycle is **closer to one working day**; **`#6d` closure is
closer to a week.** ⇒ **`D2f` completion and `#6d` closure are separate planning
milestones**, and the former does not discharge the latter. The Steward examined
a re-cut on that estimate and **declined** — the remainder is bounded and named,
so cutting further would manufacture nodes rather than reveal them.

**The `D2a` rider below is discharged by this partial** — it is one of the six
paths.

## The `D2f` gating measurement came back EMPTY. 2026-08-11.

**The emitter increment is stopped, and it stopped for the right reason.**
Runtime measured the gate before touching the emitter, which is exactly what
the gate was placed there to do.

Measured on the exact `R3` before-hole `B`-only compile
(`px8j_equal_payload_hole_placement(BeforeReturnHole)` through
`px8j_capture_source_trace`), with temporary instrumentation at the production
call site, since reverted:

```
planes=[0]   oriented_present=[false]
```

One production compile reached the builder. It resolved **zero**. The first
reason is the `oriented` gate at `planning/static_transition.rs:8901` and
`:9058`, which returns an empty plan **before any candidate enumeration runs** —
before checked transport, IH bindings, or the root walk. The `None` originates
at `cranelift_backend/test_objects.rs:70`, where the harness passes a **literal**
`None` for `oriented_subcontinuation_plan`. Production oriented plans are
decoded from a checked package's metadata (`planning.rs:144`); the `px8j`
witness is a **seed-lane** compile, so there is no metadata to decode and
nothing that could supply one.

⇒ **This is structural, not a defect in the enumerator.**

**Every gate below the `oriented` check is UNMEASURED on this witness.** The
probe short-circuits at the first cause, so it measured that cause and not the
set. Nothing here says the enumerator would or would not find a candidate if a
plan were supplied.

**And no control has ever exercised this mechanism on the witness the ACs
name.** Every `D2h`/`D2j` control that reaches a fusion candidate uses its own
synthetic fixture (`d2j_entry()` / `D2J_DECLARATION` with a hand-authored
`d2j_oriented_plan_under(cause)`) and calls the builder directly. **None of them
compiles `px8j`** — so the mechanism is untested against the acceptance fixture
on *both* sides of the gate.

### The frame defect is the Steward's, and it is named here so it is not inherited

**`AC-1` requires the fusion to occur on the exact `px8j` `R3` before-hole
compile. That compile structurally cannot carry an oriented plan today.** The
frame therefore pins acceptance to a witness that cannot carry the mechanism's
required input — **a defect in the frame I wrote, not in the work.** It is
recorded now rather than after the ruling, because the next slice frame would
otherwise inherit the same witness by citation.

**`AC-1` is NOT amended yet, deliberately.** Which amendment is correct depends
on the mechanism ruling below, and amending first would presume its answer.

### The fork is RULED. Architect `evt_6vf66hmwv52y6`, 2026-08-11.

> **There is no lawful plan-supply route for the exact unmarked `px8j` seed
> witness, so `D2f`'s `AC-1`/`AC-2` fixture binding was UNSATISFIABLE.**
>
> This follows from the **landed required-member ruling**, not from the empty
> measurement — the measurement revealed the defect rather than causing it. The
> `px8j` witness was **deliberately preserved as the unmarked negative**: no
> checked frame, no selected-IH-slot, no checked-IH-invocation marker, so it is
> not a fusion candidate. `validate_oriented_subcontinuation_transport` makes
> the boundary structural — unmarked IR with `None` is lawful seed IR producing
> no fusion; unmarked IR with a non-empty plan is a marker/plan mismatch that
> must **reject**; an empty supplied plan carries no checked transport
> coordinate; wrapping changes the occurrence tree.
>
> **Route 2 — making fusion independent of `oriented` — is REJECTED outright.**
> It would reopen `D2h`'s soundness-bearing identity and contradict the
> required-member ruling.
>
> **The lawful positive already exists**: the landed `D2g`/`D2j` checked
> `R3`-shaped fixture and its complete, independently authored
> `OrientedSubcontinuationPlanV1`, consumed through **one hoisted `#[cfg(test)]`
> constructor** and entered through `compile_expr_into_object_module` with
> `Some(oriented)` — never by calling the builder or emitter directly.

**Frame correction landed at `main` `17f68eb1`** (PR from the Steward; Runtime
notified `evt_r775vtj0pqye`). `AC-1` now names the checked `D2j` witness as the
positive full-pipeline baseline; `AC-2` binds to that twin's **own freshly
derived coordinates**, with the origin-23 reference struck as an old `px8j`
coordinate. **`px8j` is retained as the absence / ordinary-refusal comparator**
and must never again be described as the fusion-positive.

**A new `Deliverable 0` gates the emitter**: the old negative at resolved plane
`0`, the checked positive at resolved plane `1` with exactly one
key/ID/descriptor, and a one-marker-stripped exact validator refusal — committed
**before any emitter definition**. **No emitter AC may be credited until the
positive row is non-zero.**

### The two routes as they stood before the ruling, retained

Both crossed lines a leader or the Steward could not cross alone, which is why
this went to the Architect rather than being decided in the ring:

1. **Give the `px8j` witness a lawful oriented plan.** Keeps `AC-1` as written.
   But authoring the plan is authoring the input the key re-derives against, and
   the line between *supplying the witness's real oriented facts* and
   *fabricating a candidate so the emitter has something to emit* is exactly the
   line the ring was told not to cross.
2. **Make fusion not require an oriented plan.** This reopens `D2h`'s key
   re-derivation, which is the soundness-bearing half and is **excluded scope**
   under this frame.

**Neither was started.** The standing risk handed back one turn earlier is now
**confirmed rather than suspected**: an emitter built against this witness would
discharge `AC-4` **vacuously** — a no-activation proof over nothing emitted
passes for free — and `AC-6a`'s refusal controls would assert against a resting
zero, which the frame already warns proves nothing.

⇒ **Do not authorize a synthetic candidate or a zero-definition emitter to
unblock the increment.** That trades a stop for a control that cannot fail,
which is the failure this node has now filed against itself three times.

**The ruling closed both routes and supplied a third the frame had not
considered** — reuse of the already-landed checked fixture. **The forbidden
routes are now named in the frame** so they are not re-derived as options: no
`Some(plan)` to `px8j_capture_source_trace`, no synthesized default plan, no
marker inference from the Runtime shape, and no weakening of the required
checked-transport key member.

## Third `D2f` partial — the arrival-control repair, MERGED 2026-08-11

Exact `aa3b78f8680c9637b754d524012b0d7d48c38834` from declared base `87f6983f`;
one commit, one path (`lowering/core/tests/control.rs`), `+18/-8`. Decision
`dec_17c3zfw5zxwk8` resolved APPROVE — Architect `evt_1y9jzz923ymdd`, QA
`evt_1ry88yshf6629`.

**Deletes the tautological equality corrected above.**
`NonZeroUsize::new(planes.len()).expect(...)` now stands as the whole control.
**No second counter was manufactured.** Causal A/B: replacing the production
observation with a discarded length reds the control at its own arrival message;
restored byte-identically.

**This one also closes zero of the eight expressions.** Three partials, three
structural prerequisites, zero rows — stated here because the merge count is
what a later reader will otherwise use to size this node.

### The arrival proof rests on the CALL SITE, and nothing says so. Fold into `D0`.

Adversary `evt_23zyn4pywy6yg`, measured on `446c3e79`. **The vector-shape
argument holds** — `d2f_note_production_fusion_plane` has one writer
(`core.rs:444`) and one call site (`core.rs:2057`), and **the call is
unconditional**. So a reached compile pushes exactly one element (possibly `0`)
and an unreached compile pushes nothing, which is what makes `planes.len()`
discriminate arrival from non-arrival.

**The durability defect: that reason is not written beside the thing it
protects.** The comment at `:2053-2055` explains why the observation is the
consumer; **it does not say that the unconditional push is what makes `len()`
discriminate.**

⇒ **Pushing a `0` looks pointless in isolation**, so `if !static_continuation_`
`fusion_plan.is_empty() { … }` is a plausible tidy. It would silently convert
the control from *"the path was reached"* into *"the path resolved something"*
and make the `expect` **panic on a legal empty resolution** — which is exactly
the state this witness is in.

**Disposition: one sentence at `:2057`, folded into `D2f` Deliverable 0.** No new
node, no re-review of `aa3b78f8`. Same file family, active ring, and it is
cheaper than the grep that found it.

## `e4531318` — APPROVED, THEN WITHDRAWN UNPUBLISHED. Not a dropped merge.

Deliverable 0's first candidate `e45313180eb6404a309df0d0234a686c2d239405`
(one commit, five `ken-runtime` paths, `+323/-5`) reached a **resolved APPROVE**
— `dec_5x9mfj08wfftt`, Architect `evt_79wp5r4wj64jz`, QA `evt_3gpsy11vbr0pn` —
and was routed to the Steward for publication. **It was withdrawn by
runtime-leader before the publisher ran** (`evt_2yr8wqjknbmvs`), on Architect
ruling `evt_6907h4rv5kq1a`. **It never became a PR and nothing was reverted.**

**Why, and it is not a defect in the candidate.** The Architect's own words: it
**remains a sound identity-plane partial**, but its **bare-root observation
cannot carry into emitter acceptance.** The gate observed the bare
`DeclarationRef`, and root projection stops there at `Unsupported(Closure)` — so
that shape **cannot reach the definition movement Deliverable 0 exists to
prove.** A positive that cannot reach the claimed movement is not a positive.

**`dec_5x9mfj08wfftt` is spent on `e4531318` alone.** The applied-root recut
needs **fresh QA and Architect review**; no coordinate and no vote carries.

⇒ **This is the accepted-partial policy working, not failing.** The candidate was
correct for the claim it made and merging it would have put a bare-root baseline
on `main` under a deliverable whose successor must not use one.

**Also fold: record where the "sole production compile path" claim is
established.** That question has been open across all three `D2f` partials. The
enumeration was performed once by the implementer during grounding and lives
only in a retro, so it currently reads as asserted. **Deliverable 0 measures
arrival counts, which is not the same claim** — arrival-is-one does not establish
that only one production call site exists. Record the caller enumeration beside
the control.

## `D2f` Deliverable 0 — the per-cause applied-root gate, MERGED 2026-08-11

Exact `068bd6bcd7a74fe970460f6dc54c842d7dc9edf0` from declared merge-base
`1585a2e6`; one commit, five `ken-runtime` paths, `+489/-21`. Decision
`dec_1n9rxnp3tbfjc` resolved APPROVE — Architect `evt_3cgg4nab999t6`, QA
`evt_2b6g8xk7mtza3`. PR #1910; `origin/main` is `f81e36f6`. M6 blob identity
5/5 MATCH. The staleness intersection against `main`'s changes since the
merge-base was empty, so no rebase was owed despite the base sitting two merges
back.

**What landed.** One exported `d2j_checked_fixture_under(cause)` with a
**per-cause** root family, every arm spelled out so a new cause is a compile
error rather than an inherited default: the `Exact` family takes an applied
`Call(DeclarationRef(D2J), [Unit, Unit])`; `ReHomed` takes a bare
`DeclarationRef` in its own explicit branch. The planner-only bare-entry helper
is retired with zero callers. All four `D2j` planner controls were rebaselined
on their own cause-selected roots — one had been sharing a single entry across
both causes, so one of its two sides was being measured against a program it
does not describe.

**The node stays `active`.** No emitter definition, authority, edge redirection,
or emitter AC is credited, and none of the eight target expressions closes.
This is the fifth merge on `#6d` and the count still overstates progress
against the node's actual surface.

### Both carried riders above are discharged in this range

The `core.rs` doc now names the **unconditional push** as why arrival *length*
discriminates never-reached from reached-and-empty — a conditional push would
collapse the three-versus-two phase split into an unreadable zero. And the
"sole production compile path" claim is now recorded as what it is: a
**structural** claim about four delegating entries, not a measurement over
program shapes. A reader can now tell which kind of claim it is, which is the
thing the rider asked for.

### Four candidates for one deliverable, and what that bought

`e4531318` approved-then-withdrawn (above); `9d942c4b` and `ce5323ca` QA-blocked.
**Neither block was about code.** The first: replacing a test body from `#[test]`
downward left the previous doc block in place, so the item carried two doc
comments, Rust concatenated them, and the **withdrawn** bare-root contract was
the first durable reading — on a commit whose central claim is that the
withdrawn revision does not extend. The second: a decorative glyph. QA named one
occurrence; the implementer swept the **class** and found five across two files,
leaving pre-existing ones alone on the grounds that copying nearby style is how
it introduced them.

**The mutation-evidence carry is MEASURED, not argued.** The three A/B mutations
were taken on `9d942c4b`'s tree. Each recut step was checked comment-only by its
author, but **nobody had checked the composition end-to-end** — three
individually-comment-only steps is not the same claim as the whole chain being
comment-only, and the raw stat across it is 3 files and 100 changed lines, net
negative, which `--stat` cannot separate into comments and code.

Measured at `f81e36f6`, on both trees present locally:

```sh
git diff -w 9d942c4b 068bd6bc -- crates/ | grep -E '^[+-]' \
  | grep -vE '^(\+\+\+|---)' | grep -vE '^[+-][[:space:]]*(//|/\*|\*)'
```

**Zero non-comment changed lines.** ⇒ The executable tree the mutations were
taken on is byte-identical to the merged one, and the evidence transfers. This
was first written here as a caveat *"it is an argument, not a re-measurement"*;
the adversary's point (`evt_2kn8jtgn64d9s`) was that a one-command conversion
from argument to measurement should never be left as a caveat, and that a decent
prior on the answer is a reason to expect the empty result, not to skip it.

## `D2f` Deliverable 5 — the complete-key redirect selector, MERGED 2026-08-11

**Exact `e89de6674f283b80184acd4228ca8a6ae506f6fb`, PR #1915.** Decision
`dec_svd7p853crep` resolved: Architect `evt_7ey1xa79ef22t`, QA
`evt_1nt2vy2rdh14h`. One non-merge commit from declared merge-base `16d7e467`,
one path — `cranelift_backend/planning/static_transition.rs` — `+198/-0`, clean
`diff --check`. M6 blob identity **1/1 MATCH**, path count equal to declared
scope. Current-main path intersection empty, so no rebase was owed.

**The node stays `active`, and this is the sixth merge on `#6d`.** It creates no
ABI arena, definition, emitter, or redirection, and credits **no emitter AC**.

**This is the emitter turn's first partial, and the split was the ring's own
call.** The turn was released whole; the implementer declined to start it on
capacity, and after a fresh turn the leader cut the selector out and shipped it
rather than stretch the turn or leave a half-built ABI class. **The ABI/emitter
class remains unstarted by construction, so more partials are expected before
`D2f` completes.**

### What it establishes

`fusion_redirect_target` selects the redirect edge **once, from the complete
key** — `invocation_caller`, `invocation_callee`, `invocation_callee_entry` —
and from nothing else. The `StaticBody` edge kind is validated **on the
survivor**, not used to pre-filter. Zero matches and more-than-one are
separately named errors.

**That ordering is the substantive design call.** Pre-filtering by edge kind
applies a criterion the key does not contain: redundant if the three members
already determine one edge, and silently resolving an ambiguity a redirection
may not have if they do not.

### The coordinate this deliverable used to require

The frame previously required redirecting a literal `StaticBody` edge
`0 -> 2`. **No edge of that shape exists on the checked twin** — its invocation
is caller 3, callee 2, and unit 0 is a `SchedulingEntry` that invokes nothing.
`0 -> 2` was measured on the retired `px8j` witness. The frame was amended to
state the derivation (PR #1913), and the candidate writes **no coordinate into
the derivation**: `3 -> 2` lives only in the control, where it is a measurement.
The Architect's resolution confirms it is kept control-only.

⇒ **The general form, since this node will cut more slices:** pin a frame
against the derivation, never against the number the derivation produced on
whichever witness was current. A number outlives its witness and stays
syntactically valid after it stops being true — nothing goes red.

**The scope loss was local to `D2f`.** `D2d-GROUNDING` records the coordinate
under "measured coordinate on this witness" and `D2e` says "do not re-derive
those coordinates; derive the mechanism that produces them". Both upstream
sources were correct; one restatement dropped the qualifier. No sibling frame
edit is owed — checked, not assumed.

### Non-vacuity, and the bound

The control's discriminator is written **before** its positive and is **per
member**: each invocation member is independently repointed at another identity
the same plan really contains — unit 2 a real caller, unit 1 a real callee,
origin 34 a real callee entry — and each repointing must refuse. That is the
right shape for a selector, because one matching on a **subset** of the key
would still pass a whole-key positive.

### The bound, split into its two halves — they cost very differently

**This was first written here as one item, "whether the three members are
jointly sufficient on any witness other than this one." That conflates two
questions** (adversary, `evt_2rzveprrs80p0`), and only one of them is open:

| half | state |
|---|---|
| a **subset** match would still pass a whole-key positive | **closed by construction** at this SHA — all three conjuncts are visibly present in the predicate |
| a proper subset would be **insufficient** to name one edge on some other witness | **open, witness-dependent, not answerable by reading** |

The predicate is the whole of it:

```rust
edge.caller()        == key.invocation_caller
    && edge.callee()        == key.invocation_callee
    && edge.callee_origin() == key.invocation_callee_entry
```

⇒ **The per-member controls are not what excludes a subset match today — the
code is. What they protect against is REGRESSION to a subset**, which is a
different and still-worth-having job. Saying "the controls guard joint
sufficiency" credits them with the wrong half.

**No coordinate leaked into the derivation.** The function body contains no
literal identity — no `PredeclaredFunctionId(N)`, no `StaticOriginId(N)`, no
bare `== N`; the only comparisons in it are those three. That was the specific
thing the frame amendment existed to prevent, and it is measured rather than
assumed.

### The open half and the ordering question need the SAME missing witness

The validate-on-survivor ordering differs from a `StaticBody` pre-filter **only
when the three members fail to determine one edge.** That is the same ambiguous
key the necessity half needs.

⇒ **One fixture with an ambiguous key would settle both**, and if the key is
provably unambiguous, **both dissolve together** rather than needing separate
answers. Do not scope two controls here. This is the adversary's observation and
it is the most useful thing on this record for whoever cuts the next slice.

### A claim chain that looked corroborated and was inherited

The per-member repointing description travelled: implementer's commit message →
my PR body and this node → the adversary, which **explicitly took my description
rather than opening the controls.** Three artifacts agreeing, one source.

**Now read directly at the merged tree:** the controls are as described — a
per-member table repointing `invocation_caller` at `PredeclaredFunctionId(2)`,
`invocation_callee` at `PredeclaredFunctionId(1)`, and the callee entry at
origin 34, each required to refuse. **Confirmed, and it was worth confirming**;
agreement among readers who share a premise is not corroboration of it.

## Carried rider — the `D2a` control's durability. Owed, not optional.

**DISCHARGED 2026-08-11** by the `D2f` ABI-class partial above, which carries
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2a.md` among its six paths.
Retained below as the statement of what was owed and why.

**Land this with the next candidate that touches `control.rs`.** It is a rider,
not a deliverable, and it does not earn its own node. It is recorded here
because a rider stated only in a thread strands.

Adversary `evt_xyj8813ymrad`, Steward disposition `evt_3k5eg1trmw2q9`,
independently re-derived at `origin/main` before it was routed.

In the `D2a` arm, `arrivals` is incremented at `core.rs:6686` and `forwards` at
`:6696`, and **between them there is no branch, no fallible step, and no early
return.** In the non-suppressed leg every arrival forwards by construction, so
`assert_eq!(forwards, arrivals)` is decided by the test's own suppression flag
and by nothing about the mechanism.

⛔ **The hazard is not the tautology, it is that the tautology passes at
`0 == 0`** while looking like the stronger of the pair. A later trim that keeps
the equality and drops `arrivals > 0` leaves the whole control vacuous and
green, with every `!contains(R1)` holding because the marker never arrived —
the exact defect class the control was written to avoid. Both counters are
`#[cfg(test)]`, so a pass removing test-only machinery from production `core.rs`
has a motive to touch this precise block.

> **The property, stated so the route stays with the ring:** a later trim must
> not be able to retain the half that passes at zero. Label it, or make the two
> inseparable so there is no half to keep. Durability by instruction or by
> construction — the Steward leans to construction where it is clean, and that
> is a preference, not a ruling.

**Do not change the predicate to make the equality informative** — no failure
mode is available between those two increments, so it cannot be made
informative, only labelled or fused. **Do not pin a fixed count.**

**The same candidate corrects the `D2a` record's own sentence**, *"asserted as a
relation (`forwards == arrivals`, `arrivals > 0`), never as a fixed count"*,
which lists the two as equals and is where the mis-weighting was taught. Fixing
the code and leaving the record reproduces the finding one layer up, which is
the failure this lane has already had twice.

Severity is **control durability, not correctness.** `D2a` merged correctly at
`41b75c7c`; this reopens nothing.

## Carried rider — `D2j`'s non-degeneracy: two groups assert it, five claim it

**Adversary finding `evt_99agje0m3rx1`, measured on `22fb3a61` after `D2j`
merged. Confirmed. It does not reopen `e2907c5e`.**

`D2j`'s matrix closed seven member groups, and `D2j`'s own `AC-1` requires each
row to rest on a **reaching non-degenerate witness** — "an empty vector, a
single-element set, a `None`, or a value that coincides with its neighbour is
degenerate, and a row resting on one is not discharged."

**Measured across the 72 added assertions: two groups assert a non-degeneracy;
five state one in prose.** Of six cardinality pins, four are `len() == 1`
uniqueness pins, which are a different property. The two real guards are
`:438` (`ih.len() == 2`, "so neither lookup is forced") and `:639`
(`widened_args.len() == 2`), and **those are precisely the two rows where a
degeneracy was already caught** — the IH lookup, and the one-child producer
construct the Architect found.

⇒ **The population was the class and the repair took the instances.** All seven
groups share one witness, so a cardinality that collapses a distinction for one
row can collapse it for another. Nothing about either fix generalises to the
five groups nobody looked at. The prose claim for those five is the exact state
the producer-construct row was in until the Architect caught it.

### This is one read per group, not five assertions

**The Adversary's bound is carried and is load-bearing:** it searched for
cardinality pins only. A group could establish non-degeneracy by an
`assert_ne!` between two candidate positions or by a distinctness check, and
that search would not see it; and a member whose authoritative fact does not
depend on cardinality needs no such guard at all. So the honest claim is *five
groups carry no cardinality assertion*, **not** *five groups are unguarded*.

| finding for a group | action |
|---|---|
| no non-degeneracy established, and the fact is cardinality-sensitive | add the guard in `:438`'s form — the count **plus** what it buys |
| established by a different instrument | record where, and the group is done |
| the fact does not depend on cardinality | say why, and the group is done |

**Adding an assertion to a group in the third case is worse than the gap** — it
is a control that cannot fail, which is the failure this lane has now filed
against itself twice.

**`:438` is the model to copy:** assert the count *and* state the reason in the
message. A bare `len() == 2` with no reason is the next thing to rot.

Severity is **evidence completeness, not correctness.** `D2j` merged correctly
and its three deliverables are discharged; this is inherited by the next `#6d`
slice frame.
