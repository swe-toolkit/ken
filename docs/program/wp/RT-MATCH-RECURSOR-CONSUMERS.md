# RT-MATCH-RECURSOR-CONSUMERS — complete the Position A consumer repair

Owner: Runtime. Size: **M, provisional** — see Sizing.
Authority: Architect partition `evt_3r4j14fv1jtj2` (2026-08-08) on the
nine-expression census `evt_16cmej481q7ns`.

**Read `docs/program/16-recursive-descent-retirement.md` first** — the campaign
context and the five traps that bind every node in this arc. Trap 1 and Trap 2
are both live here.

> # WHAT THIS NODE IS FOR, IN ONE SENTENCE
>
> [[RT-RECURSOR-TRANSPORT]]'s `D2` closed **one** Position-A witness and its
> record said it had closed **the position**; `d8d` is a counterexample
> reachable at that same object, and this node closes the population for real.
>
> **The `D2` production mechanism is sound and is not being reverted.** It
> correctly closes the exact `D1` A witness at the exact
> `resume_active_continuation` seat. What was wrong was the *scope claimed for
> it*.

## 1. Fixed inputs

**Measure every one of these yourself at your pinned base.** The values below
were measured by the implementer at exact `D2`
`8efdfdb3fb39fc6e66708635cdf11269758d77ed` and are **anchors to re-find, never
values to trust** — your base is later than that by construction, because the
`D2` record correction lands first.

| input | as measured at `8efdfdb3` |
|---|---|
| the witness | `d8d` composed binding site |
| its complete residual set | **exactly `{MatchScrutineeRecursor}`** — never contained `LexicalCallArgumentRecursor` |
| lane, unexcluded | `RecursiveDescent` |
| lane, **A**-only exclusion | **`FunctionizedUnits`** |
| activation denominator | `Some({MatchScrutineeRecursor})`, a real compile through `px8j_capture_source_trace` |
| unexcluded compile outcome | `Ok` — the fixture was not already broken |
| refusal under A-only exclusion | `Unsupported(UnsupportedLowering { construct: "RecursiveBackedge", reason: "protocol machinery is never a source value at a boundary" })` |
| B-only exclusion on this row | **inapplicable** — the hook's own `debug_assert` refuses removing a variant that is not in the set |

## 2. What is owed

**A repair of the `MatchScrutineeRecursor` consumer population on the
functionized lane, proven on the pre-retirement tree.**

Not owed, and banned: any deletion of the `RecursiveDescent` lane, selector,
enum or authority (that is [[RT-DESCENT-RETIRE]]); any retirement of a residual
variant (that is [[RT-RECURSOR-TRANSPORT]] `D3`); any work on rows 1-5 (that is
[[RT-LEXICAL-RECURSOR-CONSUMERS]]).

## 3. Deliverables

### `D0` — close the population from the production predicate

**The population is defined by the production `MatchScrutineeRecursor`
predicate. `d8d` is a floor, not the perimeter.**

Sweep every compilation entry that can supply that predicate. Record every
firing fixture and every same-family green control.

**Helper spelling, snake_case fixture spellings and
`BodyEmissionAuthority::RecursiveDescent` assertions are candidate selectors,
not closure.** A grep tells you which fixtures might be in the family; it never
tells you what any one of them enumerates. **This node exists because exactly
that inference was made twice** — a class-wide claim from one green witness,
then a shared-helper claim over a fixture nobody opened. Do not make it a third
time: **enumerate, per fixture, by measurement.**

### `D1` — activate and attribute, before any repair

Under **A-only exclusion** — the existing one-variant hook, used as designed:

- each firing row reproduces its **exact first refusal**;
- the ordinary retained run stays **green**;
- at least one same-family control that already works on `FunctionizedUnits`
  stays green as a **positive control**;
- **exact activation denominators are recorded**, so a refusal cannot be
  credited when the selector or harness never reached the path.

**Then trace each red to the first missing or mis-consumed static fact, and name
its owner.** Partition by correlated continuation owner/origin, pending suffix,
operand phase/kind, source-machine/composed consumer seat, and boundary reached.

**A rendered refusal string is a symptom, not a cause**, and a shared refusal
string is not proof of a shared mechanism.

### `D2` — repair only the proven root boundary or boundaries

Reuse planner-owned continuation specialization / call identity and ordinary
typed-value transport **where they already name the edge**. Do not infer one
mechanism from shared syntax.

**The lawful shape of the fix:** make the protocol or fact get **consumed or
represented at its owner, before the guard**. Never teach a downstream guard to
accept a forbidden state.

### `D3` — discriminating controls

Every row `D0` found stays **enabled and unchanged in meaning**, and runs green
under A-only exclusion at the pre-retirement base. **A mutation at each repaired
root recreates the attributed refusal while proving the detector was reached.**
Unaffected same-family controls stay green.

> ### `D3` OWNS GIVING THE `D2` COUNTERS A CONSUMER — THEY CURRENTLY HAVE NONE
>
> **Added 2026-08-08 from an Adversary finding on merged `3061a645`; no
> correctness defect, and the sequencing is not being reversed.**
>
> The accepted partial landed four `#[cfg(test)]` accessors and a mutation knob
> — `mrc_d2_backedge_arms_seen`, `mrc_d2_inert_words`, `reset_mrc_d2_counts`,
> `set_mrc_d2_suppress_inert_word` — with **zero callers anywhere in the repo**.
> No test reads either counter, nothing ever sets the suppression, so the
> `if !suppress_inert_word` guard has one reachable side. **No lint catches
> this**: `dead_code` does not fire on `#[cfg(test)]` items at that visibility,
> and CI is green.
>
> **The risk is citation, not correctness.** The denominator counter carries a
> careful anti-vacuity rationale — counted *before* the representation arm, so
> suppression cannot drive it to zero and "no inert word" reads as "the arm
> declined" rather than "the seat was never reached." That reasoning is right
> and currently **inert**. A future reader greps the counter, finds a documented
> denominator with an explicit anti-vacuity argument beside it, and reasonably
> concludes the property is **measured**. It is only **measurable**, and only
> the first is what a reader takes away.
>
> ⇒ **`D3` either gives the accessors their consumer — which it needs anyway —
> or states at the declaration site that they are unread until it does.** Either
> discharges this; leaving them silent does not.

## 4. Acceptance criteria

- **AC-1 — the population is closed by measurement, not by grep.**
  *Control:* the handback enumerates each fixture in the production-predicate
  population with its complete residual set, and states which selectors were
  used as candidates. A grep list alone does not discharge this.
  **`AC-1` is unqualified and stays unqualified** — it ranges over *every*
  compilation entry that can supply the predicate, not one crate's unit tests.
  A closed subset does not discharge it, and a domain parenthetical on a
  discharge claim is an amendment to this AC rather than a hedge. The
  `ken-runtime --lib` census merged at `28edeb00` is a **partial** and claims no
  `AC-1`. The remaining domain is the cross-crate `px8-ds-test-support` census
  authorized in section 4a — **in-process entries under 4a items 1-5, and
  child-process native-compile entries under 4a.1.** Both are inside this AC;
  neither closes it alone.
- **AC-2 — every repaired root has a committed discriminating control.**
  *Control:* the control reds under a mutation at that root and greens without
  it, **from the committed tree**, with evidence the detector was reached. A
  hand-run mutation does not discharge this.
- **AC-3 — the `RecursiveBackedge` guard is intact.**
  *Control:* `RecursiveBackedge` remains protocol-only; a committed negative
  witness shows it is still refused as a source boundary value, with a positive
  control proving that path is reached.
- **AC-4 — no banned mechanism.** No fallback to `RecursiveDescent`, no
  `BoundaryUse`, no `PlannedEffectSeat` widening, no lowering-minted token, and
  no invocation-local activation/resume/return-hole state in ABI data.
  *Control:* name the ABI payload at each new crossing and show its fields are
  ordinary typed values; `BoundaryUse` stays at zero production hits.
- **AC-5 — zero new `#[ignore]`, anywhere in this lineage.**
  *Control:* `git diff` on the candidate contains no added `#[ignore]`.
- **AC-6 — no retirement and no lane deletion in this candidate.**
  *Control:* both residual variants, the classifier insertions, the collector
  insertions and the per-variant exclusion hook are **present and unchanged** at
  the final SHA.
- **AC-7 — the candidate contains NO tracker `status:` change.** The flip is the
  Steward's, post-merge.
  *Control:* `git diff` over `docs/program/issues/` is empty of `status:` lines.
- **AC-8 — CI green** on the merge. Not a local `--workspace` run, which is
  banned (`COORDINATION §12`).

## 4a. The cross-crate census: observation extends, activation does NOT

**Architect ruling `evt_2gp8nk2sn7xn2`, 2026-08-09.** This section exists
because the frame previously required an unqualified `AC-1` while banning the
only instrument that could reach the residual domain. That was a defect in this
frame, not in any handback.

**The decisive split is a real property of the code, not a convention.**
`enumerate_recursive_descent_residuals` is **ordinary production code** and
already walks the exact `RuntimeExpr` and declarations exhaustively; only
today's *recorder* is `#[cfg(test)]`. By contrast
`set_selector_variant_exclusion` **and the selector branch it controls** are
both `#[cfg(test)]`, so making either reachable cross-crate would build the
behavior-changing generalized activation seam section 5 bans.

⇒ **The observation extends. The activation does not.** An earlier phrase of the
Architect's, "extend the same instrumentation through the feature", is narrowed
by that ruling to exactly this.

**Authorized instrument:**

1. Under `#[cfg(any(test, feature = "px8-ds-test-support"))]`, a hidden scoped
   census recorder around the existing common compilation entry: inactive by
   default, restores prior state on unwind, exposes only recorded evidence to
   the calling harness.
2. At `compile_expr_into_module_with_root_projection` entry, **before**
   `validate_oriented_subcontinuation_transport`, record the complete residual
   set from the exact `expr` and declarations being compiled. Correlate that
   same invocation with validator outcome, selector arrival, and the
   **unmodified production** authority selected if it reaches the selector.
3. **Key rows by test-binary/run identity, plus test/thread identity, plus a
   per-run compilation ordinal. A test name alone is not an identity.** Preserve
   the exact equation `entry = selector-arrival ⊎ pre-selector-return`, and the
   full residual set for **every** entry, not only firing rows.
4. The recorder **may not** remove a residual, set an exclusion, choose an
   authority, alter a planner/ABI value, or affect any result. Feature-off is
   byte- and behavior-equivalent; feature-on with no recorder installed is
   inert. Required controls: a known cross-crate compile captured **exactly
   once**, a known non-member still a non-member, and feature-on/off result
   parity.
5. Run the targeted `ken-cli`, `ken-verify`, and elaborator-driven compilation
   suites with the dependency feature enabled. Forwarding the default-off
   test-support feature through test/dev surfaces is permitted; **a
   production-default or user-facing activation control is not.**

**Outcome routing.** No additional `MatchScrutineeRecursor` row closes `AC-1`'s
population coverage. A row found preserves the exact compiled input and returns
through this node's existing `D1`/hard-stop path. **Activation stays crate-local
through the existing `#[cfg(test)]` one-variant hook** — if the exact
cross-crate input cannot be reproduced in that authorized harness without
semantic reshaping, **stop and return rather than widening the hook.**

**[[RT-RECURSOR-TRANSPORT]] `D3` does not subsume this**, ruled explicitly: its
evidence does not range over these compilation entries. **No successor node and
no residual transfer** — this remains `AC-1` of this node.

### 4a.1 Child-process native-compile entries: a bounded transport extension

**Architect ruling `evt_7shkn2kebhswm`, 2026-08-09.** Items 1-5 above authorize
an *in-process* scoped recorder and do not name an environment-carried artifact
transport. **The implementer was right to stop.** This subsection is the
operative text that unblocks it. It extends the transport of the same recorder
and **authorizes no second census mechanism.**

**The population boundary is the real compilation gate** — not `Command::new`,
not "runs in a child":

- A child `ken native-build` invocation reaches `ken_cli::build_native_program`
  (`crates/ken-cli/src/lib.rs:21`) and the common Runtime compilation entry, so
  it **is in `AC-1`**.
- Child `ken run`, `ken check`, and `ken fmt` do not reach that native
  compilation entry, and are outside this census **for that reason**.
  **Launching the same binary is not membership.**
- Any additional child command is classified by the same gate. **Do not
  classify by process shape or command-name resemblance.**

**Authorized transport**, under the existing default-off `px8-ds-test-support`
feature:

1. The parent test creates a **unique per-child census session** and a **unique
   private output artifact** in its own test temp directory, and passes the
   child a hidden session identity, the parent test/thread identity, and the
   artifact location. **These values may select only the observation session and
   sink.** They may not choose residuals, exclusion, authority, lane, source,
   planner/ABI state, or any compilation behavior.
2. The feature-enabled child installs the **existing**
   `with_match_recursor_census` scope around the native-build compilation
   attempt, using the same pre-validator rows and the same later
   validator/selector/authority correlation as the in-process path. **No second
   enumerator, row schema, or sampling rule is permitted.**
3. After the attempt, **before the CLI converts its result to an exit**, the
   child writes **one versioned envelope** — session, parent test/thread
   identity, and the complete ordered row vector — to that unique artifact.
   Create it with **no-overwrite semantics**; never append to, or share, one
   sink across children. The parent reads it **only after the child exits**.
4. The merged identity is exactly **`(session/run, parent test/thread,
   child-local compilation ordinal)`**. A PID or the child thread name is
   **supplementary evidence, not an identity axis** — PIDs are reused, and the
   child thread is commonly just `main`. Ordinals stay **dense from zero within
   each child session**, and rows stay a **vector, never a deduplicating set or
   map**.
5. **Observation failure is a broken instrument in the parent test.** Missing,
   duplicate, malformed, wrong-session, or incomplete output **must fail the
   census control**. It must not change the child's compiler exit status,
   stdout/stderr, artifact result, or diagnostic. Environment absent means no
   scope and no artifact; feature-off stays byte- and behavior-equivalent;
   feature-on with no session stays inert.

**Committed transport controls must show:**

- the same child native-build command with observation **absent and present**
  has the same compilation result and output, after normalizing **only** the
  test-chosen artifact path;
- a known child native compile yields **exactly one** matching envelope, **at
  least one** entry row, dense ordinals, and the exact
  `entry = selector-arrival ⊎ pre-selector-return` partition;
- **two concurrent child launches** use distinct sessions and artifacts, and
  union without collision or loss;
- feature-on with no session is **inert**;
- a command that never reaches the common compilation entry yields **zero rows
  and is classified as a non-entry**, not silently omitted.

**Why the "at least one entry row" control is load-bearing, and may not be
weakened.** Measured on `main` at `8729e493`: `ken-cli` receives
`px8-ds-test-support` only through **`[dev-dependencies]`**
(`crates/ken-cli/Cargo.toml:25`), under workspace `resolver = "2"`. Whether the
`ken` **binary** the parent launches is itself built with that feature is a
property of Cargo's unit graph, **not something this frame asserts**. If it is
not, the child installs no scope and the census reports **zero rows** — which is
also the correct answer for a non-member command. **A zero-row result is
therefore ambiguous between "correctly a non-entry" and "the instrument never
existed in the child", and the positive control is the only thing that separates
them.** Establish it by measurement before reading any zero as clean.

**Fixed input, measured, and it is not on `main`.**
`with_match_recursor_census` exists **only on Runtime's unmerged census
candidates** — `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:472`,
re-exported at `cranelift_backend.rs:85`, on `de1434cd` and `bb9fad0a`. `git
grep` across `crates/` at `main` `8729e493` returns nothing. The word
"existing" in item 2 means **existing on that candidate lineage**. If that
lineage is re-cut, re-verify the symbol before building on it.

**Scope, stated as a boundary.** This is still `RT-MATCH-RECURSOR-CONSUMERS`
`AC-1` **observation** work. It authorizes **no activation, no production
repair, no `D2`/`D3` work, no successor, no tracker change, and no `AC`
discharge.** Any newly observed residual row **preserves the exact input** and
returns through this node's existing `D1`/hard-stop route, exactly as the
in-process path does.

**`bb9fad0a` and `de1434cd` receive no Architect approval from this ruling.**
Runtime may continue the mechanical in-process wiring and the comment-only QA
correction on that lineage; review and merge remain separate events.

### 4a.2 `D8` — pin the premise that keeps the transport out of production

**Added 2026-08-09 after `0d3f413c` merged, from an Adversary sweep of
`e8feb236` that found no defect. Folded into this node rather than filed as a
new one: the premise is this section's own, not a neighbouring concern.**

The entire safety argument for `with_child_match_recursor_census` being an
unconditional `pub` item called from production `ken-cli/src/main.rs` is that
**the feature never reaches the shipped binary**. Verified by the Steward at
`e8feb236`, that argument rests on exactly three facts:

1. the workspace root `Cargo.toml` declares `resolver = "2"`;
2. `px8-ds-test-support` is not a default feature of `ken-runtime`;
3. `ken-cli` depends on `ken-runtime` **without features** in
   `[dependencies]`, taking the feature only on its `[dev-dependencies]` edge.

**Fact 1 is one line, it is load-bearing, and nothing in the repository checks
it.** A `grep` for `resolver` across `crates/` and `scripts/` at `e8feb236`
returns only unrelated semantic uses of the word.

**Why it matters more than a normal missing pin.** A virtual workspace
manifest with no `resolver` key **defaults to resolver 1 even when every
member is edition 2021** — the workspace does not inherit it from members.
Under resolver 1, dev-dependency features unify into normal builds, so
`cargo build --release --bin ken` would compile the transport into the
**shipped** binary. There, three environment variables any caller can set
(`KEN_MRC_CENSUS_{SESSION,PARENT,SINK}`) turn a production `ken` into a file
writer against a caller-chosen sink. **The gap between "safe" and "a shipped
binary writes attacker-named files" is one absent line in a manifest, and it
is invisible at the call site.**

**`D8`.** A committed pin that fails if the premise stops holding. Assert all
three facts, not only the first — fact 1 alone is the subtlest, but facts 2
and 3 are equally load-bearing and equally editable. The natural home is a
test in `ken-cli` that reads the manifests, since that is the crate whose
production binary is at risk.

- **Control:** removing `resolver = "2"` from the workspace manifest must
  redden the pin; so must adding `px8-ds-test-support` to a `default` list,
  and so must moving the featured `ken-runtime` edge from
  `[dev-dependencies]` to `[dependencies]`. Three mutations, three reds,
  proved separately — one pin that only catches the first is a third of a pin.
- **Not** a rule about how to spell the manifest. If a future change makes the
  transport unreachable from production by construction, `D8` is discharged by
  deleting it and saying so.

**Recorded as a known residual and deliberately NOT a deliverable.** The new
`pub use` sits outside the reach of the `surface.rs` set-equality oracle,
legitimately — the item comes from `lowering::core`, so it is not a missed
oracle update. But the backend's public surface now carries an unconditional
item that **no set-equality oracle governs**, and that oracle's own comment
concedes the compiler is not a net here because most of those names have no
in-repo consumer. A second ungated item would arrive the same way and be
caught by nobody. Extending the derived enumeration to non-`surface.rs`
bare-`pub` items in this module is the cheap fix **if anyone wants it**; the
Architect's ruling that the ungated shape is acceptable and forced is about
**this** item and is not reopened here.

**Reading note for whoever next greps this diff.** `git diff | grep '^-'` on
`lowering/core.rs` shows `-    validate_oriented_subcontinuation_transport(`.
That is a **re-wrap, not a removal**: the `?` was deferred three lines so the
census can observe the outcome. The validator call is unconditional, only the
two census calls are `cfg`-gated, and the error still propagates before
anything downstream runs. Behaviourally identical on both paths. The `-2` in
the numstat actively invites the false alarm.

## 5. Banned scope

- **No `#[ignore]`.** Quarantine was ruled out for this population and the
  ruling was not reopened.
- **No reshaping of a fixture, and no absorption of a refusal**, to make a row
  pass.
- **No simultaneous exclusion of both variants, and no generalized hook** — that
  fabricates a lane no program has. **This ban is unchanged and is not relaxed
  by section 4a.** What 4a authorizes is an *observation-only* recorder that
  cannot change a result; any cross-crate **activation** seam, feature-gated or
  otherwise, is still banned. If a change makes the selector behave differently
  when the feature is on, it is activation and it is out of scope regardless of
  how it is spelled. **Section 4a.1 does not relax this either** — the values
  the parent hands a child may select only the observation session and sink, so
  a child input that reaches compilation behavior is activation carried by a
  new route, not transport.
- **No reinterpreting a retained `RecursiveDescent` run as activation.**
- **No touching rows 1-5** or the `LexicalCallArgumentRecursor` population.
- **No resume or cherry-pick of `10369776252861e8b15e613576256a3682c70066`** —
  held evidence only.

## 6. Hard stops

**Return the partition before coding** if either fires:

1. **`D1` finds materially distinct authorities** rather than downstream
   symptoms of one root.
2. **Any repair needs a new planner or ABI population.**

**And a third, specific to this node's relationship with its sibling:** if the
causal partition here and [[RT-LEXICAL-RECURSOR-CONSUMERS]]'s appear to share
one exact production root, **route a subsumption proposal — do not fold.** A
shared root may not be inferred from shared retirement timing or shared syntax.

> ### FOURTH HARD STOP, added by the Steward on the `D0`/`D1` checkpoint `bcf3218b`
>
> **If closing both rows requires BUILDING the reduced-predecessor merge shape,
> stop and return before building it.**
>
> `D1` attributed the owner to **`carried_join_arm`** in `core.rs` — the
> function name is the handle, and it is the only handle this frame states,
> because `D2` moves that file and any line number here rots on contact. (`D1`
> first reported `core.rs:10842`, read off an instrumented worktree that
> displaced the file by a uniform +78; corrected to **`10764` at `89aa1550`** in
> `cf70db86`, with the attribution unchanged.) `D1` flagged
> that its existing `Trap` arm documents that shape as **not built by this
> route**. `Trap` can decline it because `Trap` **refuses**; a `RecursiveBackedge`
> arm has to **succeed**, so it cannot borrow that escape.
>
> **This is a checkpoint, not a ban.** I am not forbidding the shape — I am
> refusing to let it be discovered mid-implementation and absorbed silently into
> an `M`. Building a new control-flow shape in the lowering route is a component-
> design call that belongs to the Architect and a sizing call that belongs to me,
> and neither is dischargeable from inside a coding turn.
>
> **If the repair closes both rows by re-routing or re-representing the operand
> at `carried_join_arm` — the lawful shape `D2` already names — no stop fires
> and no routing is owed.** That is the expected path. Post the exact SHA and
> keep going.

## 7. Base

**Post-`D2`-record-correction `main`.** Not `8efdfdb3` — that object's record
overclaims and it is not landing as-is. Not `10369776`, ever.

Cut `wp/RT-MATCH-RECURSOR-CONSUMERS` from `origin/main` after the correction
merges. Keep **both** residual variants and the per-variant exclusion hook for
the whole node.

## 8. Contention

`lowering/core.rs` and `core/tests/control.rs` — the same two files as
[[RT-LEXICAL-RECURSOR-CONSUMERS]] and [[RT-RECURSOR-TRANSPORT]] `D3`. **All
three contend, which is why they are serialized rather than parallel.** This
node is first of the three.

## 9. Sizing

**`M`, provisional, and the provisional part is real.** `d8d` is one measured
expression; `D0` may find the A population materially wider, and `M` is a
scoping figure taken from a symptom count.

Checkpoints, exact SHA posted at each: `D0` population closure · `D1` activation
and causal partition · `D2` repair · `D3` controls.

**Post the `D0`/`D1` outcome as its own checkpoint before starting `D2`** —
that is where the Steward re-sizes if needed.
