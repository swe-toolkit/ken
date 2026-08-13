# Current briefing (live — read this first on every Steward resume)

> ## HOW TO READ THIS FILE, AND WHEN TO DISTRUST IT
>
> **`origin/main` outranks this file, always.** If anything below tells you to
> do something `git fetch origin` shows as landed, **this file is stale and the
> repository is right.** Re-read fresh, in this order:
>
> 1. `git fetch origin && git rev-parse origin/main`
> 2. the LIVE block below — **only** the LIVE block
> 3. the open tasks (do not re-derive priority from memory)
> 4. for what is HELD, DEFERRED, or WHOSE it is: **the node**
>    (`docs/program/issues/*.md`), its operative block — never this file
>
> **This file is a resume POINTER, not an archive. Git is the archive.** When a
> window closes its block is **deleted**, not demoted to a "superseded" section —
> a superseded block left in the file gets read by someone, eventually.

> ### REWRITTEN 2026-07-26 ~19:4xZ — 2866 lines → this. Read the bound.
>
> The prior content (~2700 lines of windows stacked back to 2026-07-21) is at blob
> **`c26ee67f29d42690f461d43fe15e21c2202a31df`** — `git show c26ee67f`. Nothing was
> lost; it was archived to git with this pointer.
>
> **HONEST BOUND ON THE AUDIT: I did not read all 2866 lines.** I read every
> heading, the blocks claiming to be authoritative, and then **scanned** the
> remainder for sole-source markers, decision ids, held items, and preserved refs.
> ⇒ **That is a scan, not an exhaustive audit**, and its surface was my own idea of
> what "load-bearing" looks like. A reader who needs something from before
> 2026-07-26 should assume it is in `c26ee67f`, not that it was considered.
>
> **What the scan found is why the rewrite was worth doing: two blocks that
> advertised themselves as authoritative were WRONG** (see *Corrections*), and a
> hand-maintained list of 6 preserved refs when origin held **26**.

## LIVE

> **Re-arm the watchdog on every resume** — it is process-local and dies with
> every MCP restart. Builds allowed, targeted only, never `--workspace`.

> ### RESUME HERE — state at 2026-08-13 ~14:4xZ. **`main` = `310e91c7`.**
> **I hold nothing; no publisher is running. BOTH TEAMS ARE WORKING.**
>
> **Landed today, in order:** `#2091` gate 4a + the one-cut sweep repair (tree
> `af944865` verified), `#2093` two new language frames, `#2094` the
> `language-implementer` reseat. `#2088` and `#2092` are **closed**, each with
> its reason in the PR — an abandoned PR ages into a revert.
>
> **Runtime** is on the next R3 increment (`evt_rygsspfm35qw`), subject
> `checked_computational_ih_slot_unconsumed`. **Language** is on
> `LANG-RECORD-STACK-OVERFLOW` (`evt_10zxe4cyxc9d4`) with
> `LANG-VIEW-RETIRE` queued behind it on the same `parser.rs` lane — that one is
> a **standing operator instruction**, so do not let it drift.
>
> ### THE THING I GOT WRONG TODAY THAT COST THE MOST: I PUBLISHED A SHA WHOSE CI
> ### HISTORY I NEVER READ
>
> I published `50da348a` as `#2092` on an Architect Decision plus a QA approval,
> having verified the merge tree, the path intersection and the base drift.
> **It had two red CI runs sitting on it already.** It aborts a real compile:
>
> ```
> SIGABRT ken-cli::mrc_4a_cross_crate_census mrc_4a_cross_crate_census_and_its_controls
> fatal runtime error: stack overflow, aborting
> ```
>
> ⇒ **NEW STANDING CHECK, BEFORE ANY PUBLISH: read the candidate SHA's existing
> check runs.** `gh api repos/swe-toolkit/ken/commits/<sha>/check-runs`. A
> candidate that was already red does not become green by being reviewed, and
> **every path-level check I ran is structurally blind to a semantic
> regression** — I treated that set as complete.
>
> **Four facts on that defect, each from a run:** `50da348a` has never been
> CI-green (run `31550144839`, 2026-08-12, same abort); `main` is green, so it is
> not inherited; **`766c9f07`'s 143-line "bound record parser stack use" does NOT
> fix it** — `8e9baa18` contains that rework and still aborts; and `766c9f07` was
> never CI'd alone. The arc's own depth fixture never detected it because it
> built arms with `=>`, and Ken has no such token (`MapsTo` is `|->`/`↦`,
> `lexer.rs:107`), so it died in the lexer at every SHA. **Do not record "depth
> 31 never held"** — a lexer-error red is blind in both directions.
>
> **`191a4659` LIVES ONLY IN THE IMPLEMENTER WORKTREE** — one commit past
> `origin`'s `8e9baa18`, correcting that fixture to `|->`. Unpushed, and the
> reseated session did not author it. If it is still unpushed, that is the first
> thing to save.
>
> ### TWO PUBLISHER FAILURE MODES OBSERVED TODAY
>
> **A 502 on the post-merge verification reads as a failed publish.** `#2094`
> exited non-zero with `502 Bad Gateway` and **had already merged** (`310e91c7`).
> Check `gh pr view <n> --json state,mergeCommit` before retrying; a retry would
> have published a duplicate. It did **not** set the freeze marker — check
> `$(git rev-parse --git-common-dir)/ken-publisher-FROZEN` anyway.
>
> **The publisher is exclusive by design** — one fetch/check/merge/verify
> critical section under a `flock`, which is the only reason "the tree we
> checked is the tree that landed" holds. A queue behind it is correct
> behaviour, not a stall. Its fixed opening wait (~780s) runs before the first
> poll, so a green PR sitting unmerged for ten minutes is normal.
>
> **THE GATE LIST FOR #2091 WAS QA PLUS MY SCOPE RULING — NO SECOND ARCHITECT
> PASS.** Its new commit touches only `scripts/` and
> `.github/ignored-test-exemptions.toml` and implements `evt_19gqrcrrbjx7c`; a
> Steward instrumentation ruling is not a design call. The leader routed the
> Architect anyway and I published rather than let the hop delay a green
> candidate. **Twice today the ring added a hop after the gate list was already
> satisfied** — a publisher handoff after an Architect resolution, and this.
> When I state the gate list, that is the complete list.
>
> ### LANGUAGE — THE DEPTH CONTROL IS INERT, NOT RED. DO NOT RECORD IT AS
> ### "DEPTH 31 NEVER HELD."
>
> `50da348a` had been QA-approved since **before** the provider refusal and was
> never routed for its Decision. The leader's status called the ring "paused on
> the operator-owned provider blocker" — true of the implementer, and it hid an
> independent stall underneath. **A correct diagnosis of one stall is how a
> second stall beneath it reads as expected.**
>
> **The fixture never measured depth at any SHA.** It builds nesting with
> `format!("match 0 {{ _ => {body} }}")`, but **Ken's lexer has no `=>` token** —
> the match-arm separator is `MapsTo`, spelled `|->` or `↦` (`lexer.rs:107`). The
> source dies in the lexer on the **first arm**, so the parser never reaches
> depth **one**. It fails identically at `57688110`, `50da348a`, `766c9f07` and
> `8e9baa18`. Architect `evt_1f9z6akt6vrj5`.
>
> ⇒ **A red whose cause is a lexer error cannot distinguish "the capability was
> absent" from "the capability was present" — it is blind to both.** My
> discriminator (run the control at the base; pass ⇒ regression, fail ⇒ never
> held) was sound in form and QA ran it faithfully, and **its fail branch is
> still not evidence.** The merge disposition it produced is right; the reasoning
> under it is not.
>
> **`766c9f07` — 143 lines of `parser.rs` titled "bound record parser stack use"
> — was written to satisfy an instrument that measures nothing.** Its
> justification is **unmeasured, not wrong.** The follow-up node fixes the
> fixture syntax to `|->`, re-runs at both SHAs, and only then decides whether
> the rework is needed. **Do not tune `NESTED_MATCH_DEPTH` to make it pass** —
> the constant is not what is broken.
>
> **My own error there, since the shape recurs:** I ruled the control was
> unframed scope from the frame's *silence* on depth. The grep was true and
> insufficient — the control's own function name (`..._retains_...`) contradicted
> the ruling, and I had not read it.
>
> **The provider refusal is a separate, subject-triggered stall.**
> `language-implementer` (`gpt-5.6-sol`) gets OpenAI's policy layer refusing
> outright — *"extra caution with cybersecurity requests"* — on a stack-depth
> question. It **re-trips on the subject**, so retrying does not clear it, and
> **the depth node cannot be worked by an OpenAI-backed seat.** Nothing on the
> merge path needed the implementer, which is why the publish moved while the
> seat stayed stuck.
>
> **Architect finding 1, owed BEFORE any record-pattern node:**
> `brace_starts_match_arms` is correct today only because no pattern token in its
> terminator set can precede `MapsTo` — and record patterns in `match`, the
> frame's own excluded scope, break exactly that (a first arm `{ x, y } |-> …`
> classifies as a record literal). Pin the invariant with a comment first.
>
> ### WHY #2088 WENT RED — kept because the instrument shape recurs
>
> `scripts/ci-ignored-sweep.py` counted ignored rows by
> `git grep` over a **hardcoded** `POPULATION_PATHS` — `ken-cli`, `ken-verify`,
> `ken-runtime`, `ken-interp` — while its listing half runs `cargo nextest list
> **--workspace**`. `crates/ken-elaborator` is absent from that list. At the
> candidate: census **50**, ken-elaborator ignores **2**
> (`src/compiler_driver.rs:5120`, `tests/r3_c2_source_mixed_branch.rs:439`),
> listing **52**. At `main`, ken-elaborator ignores **0**. ⇒ **Gate 4a is the
> first candidate to put an `#[ignore]` outside those four crates, and both of
> its ignores are ones the Architect REQUIRED it to keep.** The two halves of
> one instrument count different populations; neither half is broken on its own.
>
> **MY RULING IS `evt_19gqrcrrbjx7c` — read it there, it is one cut.** Headlines:
> derive the census population from the workspace scope the listing already uses
> (**do not append one path** — a hardcoded list drifts at the next crate's
> first ignored row); do not weaken the sweep, silence the rows, or relocate the
> tests to a scanned crate; and add class **`blocked-upstream-relation`** —
> fully authored, would execute, but a **named** upstream relation is absent so
> a run asserts nothing. Its `readmission` must name that relation as a
> **symbol** and the row's `#[ignore = "..."]` reason must name the **same** one,
> so retirement is greppable.
>
> **The job name lies in a way that invites merging past a real gate.** It reads
> "ignored-row sweep (findings non-blocking)", but `build + test` fails with
> `ignored-row-sweep did not pass`. **Non-blocking describes its findings, not
> the job.**
>
> **Gate 4a's actual result: THE REFUSAL MOVED.** It now stops **upstream inside
> the R3 mechanism** at `checked_computational_ih_slot_unconsumed` — production's
> own total-consumption validator — instead of downstream on unrelated
> closure-boundary debt. Read as "still stopped" it looks like nothing; it
> converted an instrument gap into a named first missing relation, which is what
> 4a was ordered to find. The stack regression is fixed **without** raising any
> limit. Both sentinels stay `#[ignore]`d and framed as blocked transitions,
> **not** equality evidence. 4b unreached, gates 5-6 held, production unarmed.
>
> **Two findings are owed on the next R3 increment** (landed at `#2089`, newest
> frame box): a **tautological assert** — the **fourth** hit of this node's
> vacuous-guard lens — and a const claiming **landed provenance** in the diff
> that authors it.
>
> **A RESOLVED ARCHITECT DECISION IS ITSELF THE PUBLISH SIGNAL.** I published
> `#2088` on the resolution; the leader's publisher handoff arrived afterwards
> and crossed it. Do not wait for a ring relay on a crates-only candidate.
>
> ---
>
> **State at ~12:5xZ follows.** Its `main` SHA is superseded; everything below
> about the reseat, the base-moved detector and the held-tip hazards still
> stands.
>
> **TWO SEATS WERE RESEATED BY OPERATOR INSTRUCTION AND THE SWAP CHANGES YOUR
> OPERATIONAL HABITS, not just a config line.** `architect` is now
> **Opus 5 (1M), high**; `runtime-implementer` is now **`gpt-5.6-sol`, high**.
> A straight exchange, landed in `moot.toml` at PR #2086, both verified on their
> own pane footers — **never infer a running model from config; read the
> footer.**
>
> **THE STRANDING FAULT MOVED WITH THE HARNESS, and this is the part that costs
> time if missed.** Codex seats strand convo mentions in the composer. The
> Architect stranded on nearly every mention on 2026-08-13 — five stacked
> pastes, then six — and on Claude Code it should stop. **The implementer
> inherits it**: watch that pane now, not the Architect's. Repairs differ by
> state — composer text with **no** `Working` line is stranded (bare `Enter`);
> composer text **with** `Working` and a `tab to queue message` hint is not yet
> stranded (send `Tab`, which delivers automatically at end of turn). The
> capacity modal **defaults to a model downgrade**: `Down` then `Enter` picks
> "Keep waiting" and preserves T1.
>
> **Pane ids changed on reseat.** Re-resolve with `tmux list-panes -a -F
> "#{pane_id} #{pane_title}"`; do not trust a remembered id.
>
> **RUNTIME IS ON R3 GATE 4a**, branch `wp/RT-LEXICAL-R3-GATE4A`, committed
> `72093dbf`, base `5e5998f1`, worktree clean.
>
> **DO NOT PUBLISH `72093dbf` — it carries a measured regression**, flagged in
> its own commit message as well as at `evt_7fj5rt590nw6k`. `ken-cli`
> `px7p::selected_ok_field_reaches_both_real_executors` stack-overflows,
> confirmed as the change and not the base. **Two dead ends are already
> eliminated and must not be re-spent:** boxing the large members did **not**
> fix it (the boxing stays because it is right on its own terms), and
> `RUST_MIN_STACK=16777216` **does** fix it — which establishes pure stack
> pressure from the added frame, not infinite recursion. Next step is deleting
> the `let x = *x;` unboxing block and re-measuring. **Never resolve it with
> `RUST_MIN_STACK`** — that hides it from CI and from every other caller.
>
> **The 4a cut itself is DONE and both prohibitions held** —
> `prepare_native_program_sources` is the exact former prefix split immediately
> before `build_bound_process_starter_executable_artifact`, consumed by
> `compile_native_program_sources` so there is one producer;
> `NativeProgramPreparationV1` is immutable; the plan-bearing erasure was not
> exposed and no second collector exists. **Still owed:** the gate-4a equality
> control. 4b untouched, gates 5-6 held, production unarmed.
>
> **THE BASE-STALENESS CHECK FIRED TWICE IN ONE HOUR, once on the ring and once
> on me.** Run `git diff --stat origin/main <tip>` at **handback**, and read the
> **deletion count on every path** — a delete on a file nobody touched means the
> base moved mid-turn. Re-deriving the base at **cut** time does not discharge
> it; it is a measurement with an expiry. A scope-filtered diff reads clean for
> the same reason the damage is invisible.
>
> **`architect/work` is 1220 commits on an ancient base** (merge-base
> `721eedce`, 461,952 deletions against `main`). Checkpoint records only —
> **never publishable**, and nothing is at risk in it. Do not try to land it.
>
> **`/workspaces/ken` shows `moot.toml` modified and that is CORRECT.** That
> worktree sits behind `main`; its working copy is byte-identical to what
> landed. **Do not revert it** — reverting restores the old seat config for the
> next `moot up`. It resolves when that worktree advances.
>
> ---
>
> **The ring took the kickoff at `evt_4mgjxadg404mn`** and delivered C1/C2-source
> as PR #2084; the gate-4a ruling `evt_5r8ka125spqm9` is folded in at PR #2085.
>
> **The witness-source question is RULED and R3 has resumed** — Architect
> `evt_10ayk8fbjsz74`, folded into the frame at `35b6389a` (PR #2082). Two
> deliverables are with the ring:
>
> - **C1-source-arrival** — commit the saved public-API nested-result probe as a
>   durable control, asserting the *relations* (unchanged
>   `NESTED_LIFT_NAT_THREE_SOURCE`, `liftSize` retained, Join recpos `[2,3]`,
>   selector occurrences `[1,0]` in range, non-empty erased census under both
>   Executable and Library), **not** the census alone.
> - **C2-source** — one mixed nested / `Fork (Bool -> Bag a)` candidate under one
>   lifted-family match, then a six-gate boundary walk that **stops at the first
>   missing relation**. Five stop conditions bind; a named stop is a complete
>   result, not a failure.
>
> **THE PREMISE THAT MUST NOT COME BACK: "surface Ken emits no
> `ComputationalMatch`" is REFUTED.** The producer probe at `d30e20ee` measured a
> retained generated lifted-family `ComputationalMatch` in erased Runtime IR from
> real source. The only valid narrow statements are that the **eight sampled**
> programs were zero and that the then-current seat controls were synthetic.
> Corrected in three places — frame `#2080`, source rows `f5afd91f` (#2081), and
> the ruling fold. **If you find a fourth site still asserting it, that is a real
> find, not a stale read.**
>
> **C1 proves CHECKER-TO-RUNTIME-IR ARRIVAL ONLY.** `ih_slots = 0`,
> `ih_invocations = 0` — it says nothing about fusion-plan population or R3
> composed-seat arrival. **Production stays unarmed**; full-pipeline acceptance
> and arming are blocked on what the boundary walk finds.
>
> **Two transport failures cost the ring ~50 minutes this morning; check for them
> before diagnosing thought.** Five mentions stranded unsubmitted in the
> Architect's composer (bare `Enter` cleared it), and it then hit the capacity
> modal that **defaults to a model downgrade** — `Down` then `Enter` selects
> "Keep waiting" and preserves T1. Neither is visible from convo: the posts
> exist and were delivered.
>
> **A publisher SURVIVES your compaction.** `pub-withdraw.log` stopping mid-wait
> reads exactly like a killed process; it was not. `pgrep -af
> scripted-pr-automerge` before restarting, or you get two on one PR.
>
> **RETIREMENT CAMPAIGN POSITION.** Three of five residual classes retired
> (`TransparentDeclarationClosure`, `SeedClosureCall`, `ProducerMatchCall`), plus
> `RT-FNUNIT-RESULT-TOKEN`. **`RT-DESCENT-RETIRE` now has exactly ONE unmet
> dependency — `RT-RECURSOR-TRANSPORT`** — which is blocked on
> `RT-LEXICAL-RECURSOR-CONSUMERS` and this node. Both successor frames were
> refreshed 2026-08-12 and are shovel-ready, so §4e is satisfied.
>
> ---
>
> **BELOW THIS LINE IS THE ~04:3xZ STATE, kept for its measured detail.** Its
> `main` SHA and its "routed to the Architect" framing are both superseded by the
> block above; the takeover-seat question it routes was answered.
>
> **R3 PARTIAL `11177a3c` IS LANDED** — PR #2053, 25 commits, 11 `crates/`
> files, CI green, blob identity verified on all 11, Adversary notified at
> `evt_6pyebj76ebv4d`. **The corrected unit is released at `evt_745ke8cg0sxn7`
> and the implementer is working from `main`.** The WP branch is merged and
> deleted on origin — a resumed seat must start from `main`, not from a branch.
>
> ### TERNARY PARTIAL `be769a17` LANDED — PR #2057, blob-verified on all 8 files
>
> 3 commits from `f8f853b8`, 8 files all `crates/`, suite 915 (916 minus the one
> control retired atomically with its mechanism).
>
> **THE FOURTH ROUTE IS LOCATED AND IT IS A MISSING SEAT, NOT A PROJECTION** —
> ring tip `8b35361a`, comment-only on top of `be769a17`, 915 green.
> `take_fused_region_at` is offered **only** on the source-machine
> `ComputationalMatchScrutinee` continuation and is **entered zero times** on
> both roots, so the takeover is never offered — not declined. **The guard is
> load-bearing:** `redirect_fused_producer_invocations` did install the
> consumer's call, so without takeover the body would **both call the fused
> function and lower the same producer inline** — the forbidden second lowering.
> Relaxing it lets a duplicate through rather than advancing the compile.
>
> **Routed to the Architect at `evt_1ff5k2b4cwvxj`: where does the takeover seat
> belong.** Fourth time on this node an instrument at one consumer was blind to
> another route (same shape as the `D1`/`D2` `DirectCall` move to the shared
> funnel). Precedent says a seat every route passes through; **which** point has
> been refuted six times when chosen at depth, so the implementer deliberately
> did not pick. I pre-authorized the scope to move the seat and invited a probe.
>
> **Step 2's armed falsifier chain is UNRUN, NOT FAILING** — it cannot run until
> that seat exists. State it that way.
>
> **SQUASH-MERGE TRAP, seen again and worth the line:** `git log origin/main..8b35361a`
> shows **4** commits because `main` squash-merged three of them; the real diff
> vs `main` is **2 files**. Judge by content, never by ancestry.
>
> **What it delivered:** the ternary `P = O ⊎ I ⊎ R` with `R` =
> `FusionOwnedOuterRealization` selected by the checked consumer binding plus the
> call-target bijection (coordinate equalities are a **closure check after
> selection**, never a selector); the splice-capability census forked as the
> ruling allowed — `SpliceCompositionCapabilityId` and
> `SegmentComposition::Composed` **retired**, `composed_frame_templates`
> **preserved** because it is checked-source **wire-format** state, so retiring
> it would be a format break rather than removal of dead authority (now zero
> readers under `crates/`); and the Adversary's vacuity finding repaired **inside**
> the rewrite — `I ∩ R = ∅` is now genuinely **live** (under the binary form it
> was algebra), and `O` comes from the single derivation the accessor calls, so
> validated and consumed cannot drift **because there is one derivation**, not
> because a check compares two.
>
> **THE ARMED STOP IS A DELIBERATE GUARD, NOT AN INCOMPLETE TURN.** A fourth
> projection still routes `R` into call machinery; the shared funnel now refuses
> `R` outright, fails closed, and names the residual. **Next unit's first step is
> finding and narrowing that projection; the falsifier chain runs after it.**
> Still owed: both same-body controls (`I`-vs-`O`, `R`-vs-`O`), selector nets 2
> and 4, corrected-seam controls, inner-slot witness, self-edge discriminator,
> route exclusions.
>
> **The implementer handled the squash-merge trap correctly** — `11177a3c` is not
> an ancestor of `f8f853b8` and reads as never landed by SHA forever; it checked
> **content** (`git diff ... -- crates/` empty) rather than ancestry.
>
> **ADVERSARY FINDING `evt_5nx5mft2yzyfa` — TRIAGED: confirmed defect, FOLDED
> into the in-flight unit at `evt_6rk65rc5r4q7f`. DO NOT re-file it as a node.**
> The `D3` partition validation that landed in `11177a3c` is **vacuous** —
> `static_transition.rs:14284-14307`, both conjuncts provably false because
> `residual` is `planned.difference(fused)` and `fused ⊆ planned` is already
> refused at `:14258`. **Nothing reads `residual_identities`/`residual_targets`**
> (six lines in the whole tree, all inside the dead conditions), while consumers
> derive `O` independently — so nothing checks that the blessed population is the
> consumed one. **The two `is_subset` refusals at `:14258`/`:14267` are the real
> content and must survive.** Directed repair: derive `residual` **from**
> `ordinary_continuation_call_identities()`/`ordinary_continuation_targets()`,
> which makes the law live and installs the missing validated-vs-consumed check.
> Constraint: `fusion_composed_calls` is assigned at `:14311`, **after** the
> block. **I had recorded that partition as settled and validated; it was
> neither.**
>
> Adversary verified **clean** and not to be re-audited: the `cb60be03`
> extraction is verbatim mechanically (zero removed-only lines; all 39
> added-only lines plumbing), and the test arm is closed **by construction** —
> `set_d2f_emitter_test_arm` is private with two callers and the guard is the
> only exported surface. Low-severity open: `let _ = D2fEmitterTestArm::arm();`
> drops immediately, so a refusal test can pass **vacuously** unarmed; one doc
> clause is proportionate. **Coverage was three surfaces of eleven files — the
> unhunted list is NOT cleared.**
>
> **STILL OFF `main`:** `wp/LIB-LINEAR-CAUSAL-OBLIGATION-CALCULUS` and
> `wp/catalog-style-guide` (doc-only). **That is the whole list now.**
>
> **`wp/LANG-SURFACE-RECORD-LITERAL` `8e9baa18` is NOT a candidate — it is RED.**
> See the measured box below: eight SIGABRT stack overflows while `main` was
> green. Branch preserved, PR #2062 closed. It needs the overflow fixed, not a
> publisher.
>
> **`wp/CAT-CAPEX` `31d1efcb` IS ALREADY LANDED — do not publish it, and do not
> re-add it to this list.** Verified 2026-08-13 by blob identity: both of its
> deliverables are byte-identical on `main` —
> `catalog/packages/Capability/Filesystem/Authority.ken.md` at
> `3ada325b` and `crates/ken-elaborator/tests/cat_capex_authority.rs` at
> `23e8a161`. **`git log origin/main..31d1efcb` still reports 2 commits, and
> that reading is wrong** — the publisher squash-merges, so a landed SHA reads
> as never-landed forever. The branch's base is **902 commits** behind, so its
> two-way diff against `main` shows a huge false delete (a whole
> `Json.ken.md`, among others) that has nothing to do with its deliverables.
> **Ask blob identity on the deliverables, never ancestry and never the
> two-way stat.**
> **THE NEVER-MERGE LIST IS A PUBLISH-TARGET BAN, NOT A CONTENT BAN — rationale
> reconstructed and evidenced 2026-08-13.** It had carried none, so it was being
> read as "this code may never reach `main`", which would have held
> `8e9baa18` forever. **The decisive counter-example is `d5c7df82`: it is on the
> list, and its WP `LANG-SURFACE-RECORD-DECL` ALREADY MERGED** (see the
> merge-base commit `57688110`, "M7: LANG-SURFACE-RECORD-DECL merged"). **A list
> entry whose own branch landed cannot be a content prohibition.** Every entry
> is a superseded or mid-branch attempt: `e4531318`/`9d942c4b`/`6676251a`/
> `ce5323ca` are four passes at one `D2f` "Deliverable 0" gate (three share an
> identical message — they are retries), `ea95a223` is `D2f` `D5`, `bd5961f8` is
> the emitter chain superseded by the un-wired `877fd731`, and `50da348a` is
> `LANG-SURFACE-RECORD-LITERAL`'s FIRST commit — "add named record expressions"
> **without** the stack bound that `766c9f07` adds immediately after it.
>
> ⇒ **Publishing any of these SHAs AS A TARGET would land a known-defective
> intermediate. Publishing a later tip that CONTAINS one is a different act and
> is not prohibited.**
>
> > #### MEASURED THE SAME HOUR — `8e9baa18` IS NOT PROHIBITED, AND IT IS ALSO
> > #### NOT GREEN. I conflated the two.
> >
> > I concluded from the above that the tip was "clear to publish." **It is
> > not.** Published as PR #2062 and **CI came back red while `main` was green**
> > (run `31670194325`), so it is attributable to the cut, not inherited.
> > **Eight distinct tests abort with SIGABRT** across all four shards on
> > `fatal runtime error: stack overflow, aborting`:
> > `host_reply_selects_the_continuation_outcome`,
> > `linked_console_broken_pipe_reaches_ken_instead_of_signal_termination`,
> > `mrc_4a_cross_crate_census_and_its_controls`,
> > `nested_err_payload_reaches_both_real_executors`,
> > `public_source_observes_raw_argv_environment_cwd_bytes_in_field_order`,
> > `real_source_builds_one_identity_bound_linked_process_artifact`,
> > `selected_err_field_reaches_both_real_executors`,
> > `two_vis_nodes_resume_once_in_source_order`.
> >
> > **None is a record-literal test**, so the 232-line recursive-descent rework
> > in `parser.rs` overflows on unrelated programs. The branch's own depth
> > control bounds the **record** path only. **This is a real regression, so the
> > operator's "ignore tests if necessary" latitude does NOT apply** — that
> > covers an incidental red row, not eight SIGABRTs.
> >
> > **PR #2062 closed; the branch is preserved.** The work stays QA-approved on
> > its own terms and needs the overflow fixed before it can land. **The lesson
> > is the standing one: `not prohibited` and `releasable` are different
> > properties, and so are `QA-approved` and `green`.**
>
> > ## STANDING OPERATOR RULE, 2026-08-13 — MERGE TIMING IS MINE
> >
> > *"Architect's domain covers the quality of the work done, not the timing of
> > when it hits main. You own git, and my instructions to you are not to have
> > long-running branches... we should recut at a seam with green CI and merge
> > that to main, whether or not it comprises a complete work package... the
> > incompleteness of a work package is immaterial to whether the code is in
> > main."*
> >
> > **A ring's "non-candidate" and an Architect "atomic object" ruling are NOT
> > merge prohibitions.** I read them as one and held a branch to 25 commits over
> > six hours and three rebases, on an object that was **inert on `main`**.
> > Policy amended at both ends of
> > `agent/playbooks/federation/steward/merge-policy.md`. **Never ask the
> > Architect whether something may merge — ask whether the work is right.**
>
> **THE PARTITION IS NOW TERNARY: `P = O ⊎ I ⊎ R`. The binary `P = O ⊎ F` IS
> WITHDRAWN** (Architect `evt_6bm54j10w1n88`, folded into the frame's newest
> box). `Outer` leaves `dom(FusionComposedEdge)` — it **is** the fusion-owned
> realization, not a missed local call. The splice capability is **retired from
> this mechanism, not re-seated**; I ruled its retirement in scope because
> removing dead authority shrinks the TCB. **Read the frame's newest box, never
> this line.**
>
> **A bounded probe run BEFORE the ruling eliminated a branch** — third time
> measure-first has paid on this seam.
>
> **Landed:** `11177a3c` (25 commits) via PR #2053 as the committed partial
> basis. The correction follows on current `main`.
>
> **HISTORY BELOW THIS LINE — the binary partition it calls settled is the thing
> that was just withdrawn.** Prior state: gate run exit 0, unit released at
> `evt_10fensp5rpey7`, suite 915 → **916**.
>
> **Superseded, kept only to date the change:** exact `P = O ⊎ F` and
> `T = O_t ⊎ F_t` validated as disjointness plus coverage (never counts); the
> fusion-scoped join `edge.fusion == claim.fusion` checked from both sides; `O`
> threaded through resolution, declaration and both ledger opens by **narrowing
> the input, not weakening a law**; and the stop-comment correction with a
> `cfg(test)`-only **RAII** arm (a set/reset pair would leak an armed gate on a
> panicking assertion). **The sentinel reddening on the implementer's own next
> commit was the control working** — the armed stop moved backwards to a
> fail-closed refusal, legitimate only because the intermediate refuses rather
> than half-applying.
>
> **DISK: RECLAIMED 2026-08-13 ~04:3xZ — 96% / 11G free → 81% / 42G free.**
> The scratch reaper fired **three times in one hour**, recovering only ~1G
> each, which is the tell that scratch was not the consumer. Measured: the two
> **Language** worktree `target/` dirs held **33G** (`language-implementer` 19G,
> `language-qa` 14G) while Language is stood down with the operator
> (`evt_5vwmmrr2w7ces`). Deleted both.
>
> **What it cost and who pays:** Language pays one cold rebuild whenever the
> operator returns it to work. Nothing else — branches, candidates and sources
> are git objects and were untouched; `wp/LANG-SURFACE-RECORD-LITERAL` still
> holds `50da348a`. Verified zero build processes fleet-wide and both worktrees
> clean before deleting.
>
> **Why it was worth doing rather than waiting for the operator:** a full disk
> **presents as a test regression** and `df /` cannot see it, so the cost of
> inaction lands as a false red on the critical-path Runtime turn plus a turn
> spent diagnosing it. **`runtime-qa/target` (13G) is the next lever and I did
> NOT pull it** — QA rebuilds on the critical path at review time.
>
> **The prior fork, for history:** Architect
> `evt_6kn9ckdnbf0ph` (durable `39ff9d8f`) took **reading two**: the falsifier
> was a false alarm against the mechanism — it tested the pre-mechanism ABI
> input run — and is **withdrawn**, replaced by a post-implementation chain
> falsifier. **Call 17/37 stays OFF `FusionComposedEdge`**, on
> `FusionRegionClaim`. **The owed-and-unhomed stop-comment correction is HOMED**
> to this implementation turn and must carry a control that goes red. Released
> at `evt_40xt053gtt9md`; the implementer is rebasing `e25ba27d` → `6d5a2d84`.
> **Full amendment is the newest box at the top of the frame's stop-4 region —
> read that, not this line.** Everything below in this block about an open
> question or an unhomed item is HISTORY.
>
> **Next Steward action: none until Runtime hands back.** Six mechanisms were
> refuted on this seam before this one; the seventh is the first built against a
> falsifier that survives contact. If it fails, it fails as a **stop**, not as a
> silent green.
>
> **THE FRONTIER SURVEY WAS RUN AND THE ANSWER IS: THERE IS NO FRAMABLE WORK.
> Do not re-derive this.** Every idle ring is idle for a measured reason, not
> for want of a frame. Measured at `4ca4fb58`:
>
> | ring | why idle | is it my debt? |
> |---|---|---|
> | Runtime | R3 blocked on the Architect's in-flight ruling; one ring, one WP | no |
> | Kernel | `KERNEL-NESTED-IND` `AC-K12` is Runtime-blocked; `KERNEL-SUBST-OUTER-INDEX-SCOPE` waits on the operator TCB call `evt_561jx5e0ffy40` | no |
> | Foundation | `DS-9` `D3`+ blocked for unbounded Json folds by `KERNEL-RECURSIVE-RESULT-SURFACE` (Architect `evt_6ysrp62e4zayg`), i.e. chained behind Kernel | no |
> | Verify | **queue exhausted** — every Verify node is `merged` except `SEC1-IFC-R3`, and its Z3-free widening is recorded in-node as **vacuous, do not frame** | no |
> | Language | with the operator, `evt_5vwmmrr2w7ces` | no |
> | Ergo / doc | no released work under the two-lane cap | no |
>
> ⇒ **The free lane can only be filled by an operator answer**, and the three
> pending ones are already forwarded. **Do not re-raise them individually and
> do not manufacture a node to fill the seat.**
>
> **§4e is satisfied:** R3's successor `RT-RECURSOR-TRANSPORT` is `ready` with a
> full 34 KB frame (deliverables `D0`-`D3`, `AC-1`-`AC-8`, banned scope, hard
> stops, base, contention, sizing). `RT-DESCENT-RETIRE` behind it is likewise
> `ready`. Re-anchor the transport frame's base at release; R3 will move `main`.
>
> **THE RING IS BLOCKED ON ONE ARCHITECT RULING AND NOTHING ELSE. I hold
> nothing unpublished.** The full arc is in
> `docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md`, top of the stop-4 region,
> newest box first. **Do not re-derive any of it from this file.**
>
> **Held R3 `5c84df84`, 16 commits, base `e25ba27d` (stale — rebase onto
> `ef4920df`), unarmed, unrouted, not a candidate, branch freed. Suite 913.**
>
> **THE OPEN QUESTION** (`evt_7a2mymej6y3s6`): the Architect's own named
> falsifier fired before any mutation — no transport instance is ever minted
> (`rebind` never fires; nothing destructures construct 30) and all six unit
> calls are emitted before construct 30 is built. **Either the composed-edge
> population should include the fusion's own consuming call at 17/37 (currently
> an ordinary unit call, outside `dom(FusionComposedEdge)`), or construct 30 is
> meant to be lowered INSIDE the composed consumer's selected body — in which
> case the falsifier measured the pre-mechanism state and is a false alarm.**
> The two readings cannot be distinguished without building one.
>
> **WHAT IS SETTLED AND MUST NOT BE REOPENED:** construct 30 stays in the R3
> atomic object; construct 30 and calls 36/39 are **opposite ends of the
> transport** and requiring them to coincide would alias producer to consumer
> identity; `emit_result` → `ground_value` stays a closed conservation boundary
> refusing `StaticWorkerBinding`; and the disposition collision is solved
> **without touching `CandidateDisposition`** — `FusionComposedEdge` is the
> authority and the ordinary ledgers are narrowed once to `P = O ⊎ F`.
>
> **OWED AND UNHOMED — do not let this evaporate:** the stale stop-comment
> correction. Clause 6 placed it "in this implementation turn" and the falsifier
> fired before any implementation turn existed. `core.rs` still names a
> step-5/step-6 stop the armed compile no longer reaches.
>
> **PROCESS STATE.** The Architect rules measure-first unprompted and now writes
> its own falsifiers — **do not re-rule sequencing here.** Its pane strands on
> stacked pastes constantly (nine times); clear it whenever the sweep flags it
> **idle**. The Runtime gate was run at 02:0xZ, exit 0. `runtime-leader` missed
> seven handbacks earlier but routed the last two correctly.
>

> ### SUPERSEDED WINDOW — everything from here to "The detail behind the above"
>
> **Its held SHA `430660fd` (14 commits) is STALE. The live one is `5c84df84`
> (16 commits), stated in the block above.** Kept only because its fork prose
> records how route C was reached; take no SHA and no lane state from it.

> **THE RING IS BLOCKED ON ONE ARCHITECT FORK AND NOTHING ELSE. I hold nothing
> unpublished; the frame on `main` carries the whole arc.** Read
> `docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md`, top of the stop-4 region —
> newest box first. Do not re-derive any of it from this file.
>
> **Held R3 `430660fd`, 14 commits, base `5b912876` (now stale — rebase onto
> `6b90dcf8`), unarmed, unrouted, not a candidate.**
>
> **THE FORK:** either the ruled incoming-call **partition guard is
> structurally unreachable** defence-in-depth — recorded like
> `FusionClaimRefusal::SelfRedirection`, one doc sentence and **no** control —
> or **the partition must key on something other than incoming-call
> multiplicity.** Mechanism, the Architect's alone. **(a) is not the cheap
> answer:** it replaces two tests with a claim that then has to be true.
>
> **THE ROUTE-C MECHANISM IS RULED** (`evt_1t3f4e8100rb5`): planner-authored,
> **call-edge-local continuation composition**, authorized by **one opaque exact
> `ContinuationCallIdentity`** — never target/body/origin/owner coincidence. The
> derivation **closes with no new source fact**: the two composed edges are named
> by the fusion key's two `CheckedIhBinding`s, a relation already in the file
> from the opposite direction.
>
> **WHY THE FIXTURES-FIRST UNIT STOPPED, and it is a result rather than a
> failure.** `intern_specialization` keys on the **whole**
> `ContinuationSpecializationKey` ⇒ distinct call identities imply **distinct
> targets**, so the residual-direct-caller population exists in-tree **only**
> under `ContinuationInternMutation` — an injected planner defect. Ten
> configurations measured. **My sequencing ruling is not refuted by this:** a
> singleton domain still makes the guard vacuously green; what changed is that
> the fixture cannot supply the population either, and the bounded escape hatch
> surfaced that **before** a mechanism was built on the guard.
>
> **SIX PREMISES REFUTED ON THIS SEAM, classification untouched by all six.** The
> Architect now rules measure-first unprompted — **do not re-rule sequencing
> here**; raise it only if a mechanism is prescribed with an unmeasured premise.
>
> **`runtime-leader` has missed SEVEN handbacks.** Its tick samples the
> implementer mid-work and concludes "no action owed". **Compare its last-turn
> time to the latest handback, never its own conclusion.** Rouse under 1000
> chars or it pastes and needs two Enters.
>

> **THE ARCHITECT RULED AND WITHDREW ITS OWN PRIOR RULING**
> (`evt_27qhdnnmv4h4z`, durable `fd62eb5d`): `FusionForward` and
> continuation-specialization subsumption are **withdrawn** — the earlier
> forward rested on a false temporal premise. The exact specialization stays
> **executable** and its `DirectCall` result is the **data-flow predecessor**
> the later redirect consumes. Neither the forward nor the redirect moves.
> **Full frame amendment is in `docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md`
> at the top of the stop-4 region — read that, not this line.**
>
> **`a3c25dae` is now NEGATIVE EVIDENCE and that is its value.** The mechanism
> is withdrawn; **the measurements are not.** The recut removes the code, so the
> ordering trace and the operand finding live in the frame and the thread by
> design.
>
> **The `StaticWorkerBinding` refusal RETURNS and is not a regression** — with
> the live specialization restored it is again the next honest boundary inside
> route C. The ruling forbids grounding/transferring `StaticWorker`, adding an
> ABI member, or guessing a routing mechanism in this recut.
>
> **HOLDING one doc-only commit deliberately** (`46680495`, the frame
> amendment). The implementer is mid-recut and the ruling explicitly named
> `7877a28d` as its rebase base; publishing now would stale a rebase in
> progress. **Ship it at the next gap between turns.**
>
> **FIVE premises refuted on this seam, classification untouched by all five.**
> The standing discipline is written into the frame: no further mechanism is
> prescribed here without its premise measured first.
>

> **THE PROBE ANSWERED THE FORK IN ONE RUN, AND THE ANSWER IS THE ORDERING
> BRANCH** (`evt_7ydbavjbtxx97`). At the consumer's producer-call seat the
> redirected fusion invocation **has not emitted yet** — it emits immediately
> afterwards, in the same function, downstream of the very call the forward
> replaces. **The forward seat is upstream of the value it is meant to
> forward.**
>
> **What that rules out, as fact rather than recommendation:** the fused answer
> is not merely out of scope at the forward seat — **it has not been produced by
> any seat in that function yet.** So no operand selection at that seat could
> ever have worked: not that position, not another position, not another operand
> class, and **not a source relation**. That kills the other branch outright
> rather than merely disfavouring it.
>
> **The counterfactual is in the same run**, so "does the redirect emit at all"
> is not left to inference: with the disposition bypassed, the same consumer-side
> invocation appears on both witnesses and that run reaches the old
> `StaticWorkerBinding` refusal. The redirect is real and executes; the
> disposition-active run simply refuses before reaching it.
>
> **The method note is the part to keep.** Getting both seats into ONE run took
> three temporary bypasses. The implementer **had the ordering across two
> configurations first and did not post it**, on the grounds that a
> cross-configuration ordering is a weaker claim than it reads as. That judgement
> is correct and is why this measurement is load-bearing where the previous four
> premises were not.
>
> **THE SEQUENCING RULING PAID FOR ITSELF.** The Architect recorded the hold
> (`evt_1yaespw07q1y3`) and did not rule; the probe cost minutes and eliminated
> a full branch. Ruling blind would have been a coin flip.
>
> **NOW WITH THE ARCHITECT** (leader routed `evt_1cb67mps4crsy`, 00:01:23Z): does
> the forward move, does the redirect move, or does a lawful new join establish
> their order? **That is mechanism and it is the Architect's alone.** It is
> grounding against the actual diff at `a3c25dae`, which is the right shape.
> Held R3 unchanged at `a3c25dae`, 12 commits, base `07b20585`, unarmed,
> unrouted, not a candidate.
>

> **A FOURTH PREMISE WAS REFUTED, AND I RULED THE SEQUENCING AGAIN
> (`evt_6j04882rsx096`): the bounded probe runs BEFORE the Architect rules.**
> The Architect had already ruled the forward disposition
> (`evt_713gc922d1d7g` — the region-owned call is an exact `FusionForward`, not
> a surviving standalone call). The implementer **built it exactly as ruled**,
> rebased onto `07b20585`, and its own fail-closed guard then measured the
> ruled operand premise **false on both witnesses**: recursive position 0 holds
> `Lowered::Closure`, and there is **no `Carried` anywhere in that field run**.
> Removing the emitted specialization call removed the only value the forward
> was to carry.
>
> **NEW TIP `a3c25dae`, 12 commits, base `07b20585`**, with the full 11-commit
> rebase mapping published and a **zero-`crates/` delta** proof, so every
> region-scoped verdict re-attaches. Unarmed, unrouted, not a candidate.
>
> **THE GUARD REFUSING IS THE MECHANISM WORKING** — recorded so it is not
> re-litigated. A forward that had silently taken the closure would have been
> the fifth inert fix and far more expensive to find. Declining to hunt the
> fused answer elsewhere was also correct: choosing a different operand source
> is the reserved decision, not an implementation detail.
>
> **THE FORK IS A QUESTION OF FACT, WHICH IS WHY THE PROBE COMES FIRST.** Has
> the redirected fusion invocation emitted by the time the producer-call seat
> has lowered its fields? If yes, the forward needs a **source relation** rather
> than the ruled field position. If no, the forward seat is **upstream** of the
> value and the **ordering** is what must change. Two branches, two different
> repairs, one probe at the redirect seat. **Ruling blind is a one-in-two chance
> of a fifth refuted premise at the cost of a full Architect turn plus a
> dispatch plus a grounding turn plus a re-route.**
>
> **THE PATTERN, now four for four:** every mechanism prescribed for this seam
> has been refuted by measurement — terminalization, the generic funnel, receipt
> ordering, and now the forward's operand premise. **The classification has
> survived all four untouched.** What keeps failing is the layer beneath it, the
> same way each time: prescribed against the emitter's documented intent,
> refuted by the measured control flow. The one time we measured first, it
> retired a question three ruling cycles could not settle and surfaced route C.
>
> **Independent confirmation the sequencing is right:** the Architect, on its
> own post-compaction turn and before reading my ruling, said it would *"issue
> only the bounded causal-read disposition this evidence supports."*
>
> **I HOLD TWO DOC-ONLY COMMITS AND THE HOLD IS CORRECT RIGHT NOW** — freshly
> derived, not inherited. The implementer rebased onto `07b20585` at 23:42 and
> is mid-probe on that base; publishing would stale a rebase minutes old. **Ship
> them the moment the ring is between turns.**
>

> **THE RING HOLDS THE NEXT MOVE. There is nothing to frame and nothing to
> start.** The Architect owes a ruling on the flagged not-surviving judgement
> (routed by `runtime-leader` at 23:23Z; the Architect pane was stranded on two
> pastes — I cleared it and it went to work). Held R3 is unchanged at
> `9177c6ce`, unarmed, unrouted. **Do not manufacture work while this is out.**
>
> **PUBLISHED PR #2042 (doc-only, `07b20585`).** The four held commits are on
> `main`; blob identity verified on both files; `steward/work` reset. **The base
> the ring must name from now on is `07b20585`** — `689dabd7` is now stale in
> the same way `d5912acd` was, and both are named in artifacts below. Posted to
> the thread at `evt_59s1yqvrv3xkb`.
>
> **Why I published into an idle ring rather than holding again.** Zero
> `crates/` delta, so no held verdict is disturbed, and the Architect had
> already instructed a rebase at the next natural seam — which absorbs it for
> free. **The hold was re-derived, not inherited.** Holding this same class of
> commit once cost three hours of a `main` that described a stop already closed.
>
> **CLOSED THIS TICK — `CI-OLD-PRESTATE-ROW-CURRENCY`.** `verify-qa` was
> awaiting "renewed Spec and Architect votes" on work **already on `main`**:
> `2bc6cb80` touches one file, its blob is byte-identical to `origin/main`'s,
> and it landed as squash `3a36d13c`. Told both Verify seats to stand down
> (`evt_15877p8y3s9h4`). **Third occurrence of the squash-head-reads-unlanded
> shape** — ancestry is the wrong instrument, blob identity is the right one.
>
> **IDLENESS IS NOT BACKLOG RIGHT NOW, and I re-checked rather than assuming.**
> Verify's only open node is `SEC1-IFC-R3`, `draft`, with the operator. Kernel
> is Runtime-blocked at `AC-K12` plus an unanswered TCB call. Language is with
> the operator. **The doc track — the one sanctioned concurrent lane — has zero
> open nodes; do not manufacture doc work.** The `integrator` roster line is a
> tombstone and its "PR #365" is not an open loop (merged 2026-07-08 as
> `3859aaff`).
>
> ### The detail behind the above — still current except for `main`
>
> > **THE STOP THIS NODE HAS BEEN ON SINCE `evt_2m62086x60c94` IS CLOSED.**
> > `evt_11werhky391ds`: armed on both witnesses, **the `StaticWorkerBinding`
> > refusal is GONE** — route C is no longer reached and `ground_value` is never
> > handed the worker-bearing intermediate. Disarmed before commit.
> >
> > **NEW TIP `9177c6ce`, 11 commits, BASE `689dabd7`** — the implementer
> > rebased and published the full old→new SHA mapping. **Verified: the rebase
> > delta is FOUR FILES, all four mine from PR #2041, ZERO `crates/` delta** ⇒
> > every region-scoped verdict in the thread **re-attaches** rather than
> > needing to be re-earned. The path intersection was empty exactly as claimed.
> >
> > **THE FINDING, not a detail:** the disposition **must not key on the
> > producer construct.** The claim names the fusion's producer (30/26) while
> > the specialization it subsumes names the **generic continuation producer**
> > (39/35) — matching on construct resolves **nothing** and would install,
> > report success, and change nothing. The join is the **worker/body**
> > relation. **Mutation proof:** keying on producer construct reds the control
> > with *"a static continuation fusion subsumes no generated continuation
> > specialization"* — the inert-install failure caught by a test instead of a
> > fifth turn.
> >
> > **FOUR projections, not three** — `verify_emitted_continuation_calls` also
> > required a declared `Function` per planned specialization, **found because
> > the armed compile refused there, not by inspection.**
> >
> > **A JUDGEMENT THE IMPLEMENTER MADE AND FLAGGED — THIS NEEDS THE ARCHITECT
> > AND MUST NOT GET BURIED.** The ruling says a **surviving** caller must make
> > preflight refuse. The consumer's own unit still calls the subsumed
> > specialization (cut-2 row 11), and the `Carried` the takeover forwards **is
> > that call's result.** The implementer classified the region's **own** call
> > as **not surviving** — it is the call the fusion exists to subsume — and
> > excluded exactly it, so preflight installs and the refusal lands at
> > emission. **Had it classified that call as surviving, preflight would refuse
> > on both witnesses and the ruling's own discriminator 1 would be
> > unreachable.** Both readings are defensible from the text, **they differ in
> > observable behaviour**, and inverting is **three lines**.
> >
> > **NEW BOUNDARY, reported not repaired:** nothing yet replaces that call's
> > emission with the fused result; the takeover intercepts at origin 10 **after**
> > the call is emitted. Making the consumer not emit it is **emission routing**,
> > outside the authorized disposition — so it stopped.
> >
> > **CONTROL GAP, stated in the test rather than left to be inferred: this
> > witness CANNOT discriminate identity keying from body keying** — its two
> > specializations have different worker bodies (34, 37), so a body-keyed
> > filter stays green. The disposition is identity-keyed **because the ruling
> > requires it, not because anything here proves it.** The same-body row needs
> > a fixture **the D2j family does not contain.**
> >
> > **Validation:** `-p ken-runtime --lib` 910/0/4, re-run **after** the rebase.
> >
> > **STILL OWED:** the caller-side replacement; armed discriminators 1/3/5;
> > discriminator 4's same-body fixture; selector nets 2/4; controls 2/3;
> > inner-slot witness; self-edge closeout.
>
> > **PUBLISHED. `main` MOVED `d5912acd` → `689dabd7` (doc-only PR #2041) — the
> > first merge in three hours.** It carried 14 commits of rulings,
> > refutations and corrections that existed only on `steward/work` while the
> > frame on `main` fell **four rulings** behind. `steward/work` is **reset to
> > `origin/main`** after verifying all four files byte-identical — a
> > squash-merged branch reads 14-ahead forever otherwise.
> >
> > **I HAD THE HOLD WRONG, and the correction is the lesson.** Holding was
> > right while the ring was mid-build against a hand-named base; it was wrong
> > once they were tracing and the branch was freed. **The path intersection was
> > empty the whole time** — no `crates/` paths — so the rebase was always
> > clean, and the only thing the hold bought was a `main` whose frame described
> > none of what had been ruled. **Re-derive a hold at each tick; it does not
> > stay true because it was true.** Ring told at `evt_2v4sdvr74yme2`; they must
> > not name `d5912acd` again.
> >
> > **SETTLED — THE ATOMIC BOUNDARY DOES NOT SHRINK (`evt_6xb2jnracee7x`).
> > DO NOT RE-RAISE THIS. I said "still one object" would be a legitimate
> > answer I would record as settled and stop asking; it is, and I have.**
> >
> > **MY INFERENCE WAS WRONG, and the reason is the reusable part.** The
> > criterion: **a unit-tested component law is NOT the positive for the
> > behavior-changing production slice that depends on it.** Every control I
> > cited exercises an invariant **directly** and not the production route the
> > invariant governs — control 1 is a **negative baseline** that never
> > exercises the composed positive; `dp_composition_time_membership...`
> > validates a **manually populated** `composed_frame_templates` sequence, so
> > **`DP`'s production positive is still behind the arm**; net 3 proves the
> > ledger's laws while its own comment says the unconsumed **refusal** lives in
> > armed splice closeout; `AC-D3-SELF` installs a plane and **emits no fusion
> > definition**; the binder-to-body control likewise installs directly while
> > production runs `preflight` over an **empty** population with
> > `D2F_EMITTER_ARMED` false. ⇒ **"Four controls run unarmed" was true and did
> > not mean what I took it to mean.** Same shape as
> > *a fixture satisfying its own relation while invisible to the consumer.*
> >
> > **No partial cut from `766cbdf0` is authorized.** The first test-only
> > baseline commit could be copied out harmlessly but **shrinks no executable
> > dependency and does not warrant another candidate or review hop** — so do
> > not.
> >
> > **RING INSTRUCTION FROM THE SAME RULING:** continue the already-ruled
> > route-C implementation; **rebase the whole held range onto current `main` at
> > the next natural seam; ARM LAST.** Never name `d5912acd` again.
> >
> > **The research advisory was received as framing and changes neither the
> > route-C ruling nor this sizing.** That is the correct disposition and I am
> > not re-carrying it.
> >
> > ~~**RE-PRICING THE ATOMIC BOUNDARY IS WITH THE ARCHITECT
> > (`evt_2v4sdvr74yme2`), and it is a real question, not schedule
> > pressure.**~~ **ANSWERED, above.**
> > `evt_4m0q1m4zn4k79` forbade merging **unexercised** mechanism. Measured on
> > `766cbdf0`, **four controls now run UNARMED in the tree** — control 1,
> > `dp_composition_time_membership_is_validated_and_changes_the_binding_fingerprint`
> > (**`DP` is exercised**), `d3_the_splice_capability_is_spendable_exactly_once...`
> > (net 3), and `ac_d3_self_the_recursive_edges_call_site_is_separated_from_its_callee_body`.
> > ⇒ **The ruling's own precondition is no longer uniformly true**; `D1`/`D2`
> > appear to be the only unexercised parts. **I proposed NO cut** — amending an
> > atomicity ruling is soundness and is the Architect's. **"Still one object"
> > is a legitimate answer I will record as settled and stop asking.** It must
> > not delay the route-C ruling.
> >
> > **RESEARCH ENGAGED (`evt_5h8qz9efzz1ky`), advisory and non-blocking,
> > bounded to one turn.** Question: in fusion/deforestation, **what makes the
> > fused-away producer DEAD and WHERE is that established** — asserted by the
> > transform, discovered by a later DCE/reachability pass, or a property of
> > ownership so the occurrence is never lowered again — plus the standard
> > failure mode when a fusion leaves its producer reachable. **Approach and
> > behavior only; no `crates/` read; no repair proposed.** *"No transferable
> > prior art, here is why"* is an accepted answer. **Two Enters were needed
> > (1192-char paste).**
>
> ### RESUME HERE — earlier state at 2026-08-12 ~22:0xZ. `main` was `d5912acd`.
>
> > **~23:0xZ — THE TRACE LANDED AND FOUND A THIRD CONSTRUCTOR ROUTE NOBODY
> > WAS RULING ABOUT. The sequencing ruling paid for itself in one turn.**
> >
> > **ANSWERED AND RETIRED:** producer and suffix are **ONE machine, joined,
> > and CORRECT.** `next=["Terminal"]` at construct 30 **is** the `ResumeOuter`
> > the machine was started with — that is how the suffix is reached, so
> > `Terminal` is **the join's spelling**, not a missing continuation. ⇒ **There
> > is NO answer selection to repair at any fused seat**, which is precisely why
> > the funnel and the receipt each had **nowhere to attach.** Three mechanisms
> > failed for one shared reason.
> >
> > **THE ESCAPE IS ROUTE C**, a third production of the same source occurrence:
> > `lower_expr`'s `RuntimeExpr::Construct` arm, **the sole builder of the
> > `StaticWorker` arm**, emitting at `machine_depth=0`, `fused_auth=None`, in
> > **root-projection `block3`** under
> > `compile_expr_into_module_with_root_projection` — **not the fused definition
> > and not a unit body.** It fires **after** the fused body exits `Carried`,
> > **after** takeover, **with the affine claim already SPENT.** Route A and
> > route B are where every prior ruling aimed. **Route B's template is consumed
> > inside the machine and never reaches a sink at all.**
> >
> > **TRAP, flagged before anyone fell in:** at that seat `defining=2` is
> > **ambient carrier state, not the enclosing emission** — backtrace is
> > authority, and they **disagree.** **Do not key anything there on
> > `defining_unit`.**
> >
> > **STILL OPEN, and much narrower:** why does the root projection lower the
> > claimed producer **at all** after body ownership and fused consumption — and
> > is the defect that **route C runs**, that **its result is selected**, or that
> > **ownership left the occurrence reachable from the root**? Different
> > repairs; nobody has picked. Architect grounding at `evt_7z2jq0k1g42np`.
> >
> > **REUSE THE SEQUENCING MOVE.** The Architect accepted evidence-first without
> > argument, **tightened the trace bounds instead of ruling**, and one trace in
> > two cuts retired what three full ruling cycles could not — for **one
> > implementer turn.** When a mechanism is refuted twice, **buy the measurement
> > before buying another ruling.**
> >
> > **~22:5xZ — THIRD MECHANISM, THIRD REFUTATION. I RULED THE SEQUENCING:
> > the causal trace runs BEFORE the next mechanism (`evt_41remx6xn6pda`).**
> >
> > The receipt re-ruling (`evt_26ganh9p86xw8`) got the **relation separation
> > right — that part is measured correct and stands.** What fails is **step 2
> > against step 3: the receipt cannot exist at the seat ruled to consume it.**
> > One armed monotone counter: the claimed producer's source-machine completion
> > is `[0]` (`Exact`) and `[4]` (`ReHomed`), and **every IH call mint follows
> > it**. There is exactly one such completion per compile ⇒ the "no receipt"
> > fallback **always** runs, the template still reaches root, **and all six
> > controls pass vacuously** — the inert shape again, one layer in, and this
> > time **the controls could not catch it.**
> >
> > **Why:** at that completion `next=["Terminal"]` — the outer eliminator is
> > pending as *authority* but **not on that machine's stack**, so the template
> > **is** that machine's final answer and nothing downstream can consume it.
> > **The control that stops the over-claim:** the same construct 30 with **no
> > fusion** is *also* `Terminal` **and does not escape** ⇒ Terminal there is
> > **normal**, not the defect. The implementer produced that against its own
> > tidier story.
> >
> > **THE STANDING QUESTION:** are the producer machine and the suffix **one
> > machine, or two with a defined answer join?** Every repair so far —
> > funnel reuse, receipt selection — **presumes one machine with a selection
> > point, and measured they are not.** Nobody has picked; do not pick.
> >
> > **WHAT I AUTHORIZED, sequencing only:** a bounded causal trace of the fused
> > body's machine structure, both witnesses, carrying the consumed-construct
> > contrast **and** the no-fusion control, **measure-only, no proposed repair**;
> > the Architect rules **against** it rather than ahead of it, and may re-bound
> > it. **No scope shrunk, no party added, no review hop added.** Three
> > mechanisms have each cost a full Architect turn plus dispatch plus grounding,
> > and each was refuted by **one probe costing minutes.**
> >
> > **Architect strand count is now SIX; I unstranded it again at ~22:5x, and I
> > posted the authorization BEFORE waking it so it would not produce a fourth
> > mechanism first.** That ordering is deliberate — repeat it.
> >
> > **~22:3xZ — `AC-D3-ANSWER` IS MEASURED INERT. DO NOT BUILD IT.** The
> > implementer grounded it before writing code and the grounding refuted it
> > (`evt_1twk028k03mxe`); **no code written, `766cbdf0` unchanged.** Re-ruling
> > requested at `evt_7snn4fr8b4yea`; **I unstranded the Architect's pane for
> > the FIFTH time** (stacked pastes again) and it is working it.
> >
> > **Why it is inert:** `continuation_call_binding_for(30)` returns **`None`** —
> > the continuation-call projection holds exactly `(36, 25)` and `(39, 10)` and
> > **never names the claimed producer 30** — so the ruled funnel falls through
> > to *"retain byte-identical ordinary construction"* and **the direct template
> > still reaches root projection.** Inert on the witness **its own row 1
> > names**.
> >
> > **Rows 3 and 4 are inverted:** the **fusion claim** names construct **30**;
> > the **continuation-call relation** (what the funnel keys on) names **39**.
> > So row 4 asks to prove non-authoritative **the construct that funnel is
> > authoritative for**, and row 3's "no exact binding" case **is** the claimed
> > producer. **39 never reaches that seam on the armed path at all.**
> >
> > **The escaping template is built INSIDE `Fusion(0)`**, producer half,
> > `fused_authority = Some((StaticOriginId(10), PredeclaredFunctionId(3)))`,
> > `defining_unit = Some(2)`, **claim still outstanding.**
> >
> > **THE FORK, with the Architect, and nobody has picked one:** is the
> > source-machine route the **wrong route** for the fused producer half (it
> > should lower through the generic eliminator route that already carries the
> > funnel — making the defect **one level up**), or is it the **right seam
> > keyed by the wrong relation**?
> >
> > **THE PATTERN, and the sizing consequence is mine.** This is the **second
> > consecutive ruling refuted by grounding**, and **both times I transcribed
> > the mechanism into the frame as an AC**. The **classification survived both
> > refutations**; it is the mechanism layer beneath a correct classification
> > that keeps failing. ⇒ **An AC transcribed from a ruling inherits that
> > ruling's mechanism risk** — an unmeasured mechanism now gets a warning box
> > **above** it, never silent inclusion beside measured criteria. And the
> > ring's grounding-before-coding is **twice vindicated**, not over-caution.
> >
> > **STOP 4 IS RE-RULED at ~22:1xZ — `AC-D3-ANSWER`, Architect
> > `evt_5rze80e6w9qz8`, durable at `9fd0731e`, leader dispatched
> > `evt_5s5hkcjr0e2c`. The implementer is LIVE on it.** The fork is answered:
> > the second root occurrence is **WRONGLY SELECTED, not wrongly produced.**
> > The defect is **answer routing at owned source-machine constructor
> > completion** — generic `lower_expr` lets a successful exact claim/call own
> > the answer, the owned source machine **unconditionally** returns a direct
> > constructor answer with no equivalent choice, and under `FunctionizedUnits`
> > that direct template is what the root projection sees. Repair closes that
> > seam with the **same existing funnel**; byte-identical ordinary construction
> > when no exact binding exists. **Six required discriminators are in the
> > frame.** The Architect **withdrew its own causal sentence** on the
> > implementer's four measurements.
> >
> > **IT ALSO CORRECTED ME, on a precision it called load-bearing.** `29`/`25`
> > is **NOT** a second same-spelling source constructor — it is **the claimed
> > producer occurrence itself, re-entered by a distinct lowering traversal.**
> > *"Different occurrence"* holds **only at the dynamic-construction axis** and
> > is **false at the planner source-origin axis** (`Exact` claim = construct 30
> > / field 29, other = 39/38; `ReHomed` = 26/25, other = 35/34). **A reader
> > taking my earlier sentence literally would hunt a second source constructor
> > that does not exist.** Corrected in the frame.
> >
> > **SUPERSEDING UPDATE at ~22:0xZ — read this before the ~21:3xZ text below
> > it, which it corrects on two points.**
> >
> > **TIP IS NOW `766cbdf0`, 10 commits**, base `d5912acd`, still unarmed and
> > unrouted.
> >
> > **`AC-D3-SELF` IS CLOSED** (`evt_6bg3en6yy4dgz`).
> > `FusionClaimRefusal::BinderBodyResolution` refuses **before claim
> > issuance**. **The route I recorded for it last turn was wrong** — the
> > redirect edge is already unique by callee, so that comparison **cannot
> > fail**; the working resolution uses the binder's **own members**, so
> > `recursive_position` is **used** rather than compared to a copy of itself.
> > Its control's first shape died to its own mutation proof because **a
> > two-step rule shadows itself**: one proof per **step**, not per rule.
> >
> > **`AC-D3-ROUTE`'s MECHANISM IS MEASURED FALSE AND I STRUCK IT IN THE
> > FRAME.** The classification (R3 routing defect) and **the atomic boundary
> > are unaffected** — only the causal sentence and therefore the repair. Four
> > probes: the fused suffix lowering returns **`Carried`** (a runtime call
> > result, terminalizing nothing); the takeover forwards it, so **the redirect
> > RAN and the suffix WAS consumed**; the escape backtraces to `emit_result`
> > from the **root projection**, not inside the fused function; and
> > `require_complete_static_worker_disposition()` **passes** just before. ⇒
> > **the escaping worker-bearing constructor is a DIFFERENT OCCURRENCE arriving
> > as the root answer** (field origin 29 `Exact`, 25 `ReHomed`).
> >
> > **WHY THAT MATTERS MORE THAN THE CORRECTION ITSELF:** read literally, the
> > struck repair says the takeover forwards too early, so the obvious build is
> > **to stop forwarding** — and the takeover is **correct**, so that would
> > break working code and reintroduce the double-suffix defect at `:650` the
> > forward exists to prevent. **A repair that would have looked ruled and been
> > wrong.** The implementer measured instead of building. Do not undo the
> > strike.
> >
> > **OPEN, with the Architect at `evt_2fanpwder54a0`:** is the second root
> > occurrence **wrongly produced** or **wrongly selected as the answer**? Both
> > fit every measurement and **they have opposite repairs.** Nobody has picked
> > one and neither should you.
> >
> > **Held commits now:** `f3a8808b` `c0422594` `3eaa9f3f` `2a0e7d31`
> > `f6eeabe6` `194faa93` `3d2d0d92`. The frame-predates-the-rulings hazard
> > below **still applies and has grown.**
>
> > **BOTH ARCHITECT QUESTIONS ARE RULED. THE GATE RAN. THE BRANCH IS
> > RE-RELEASED (`evt_5qex3d5c36q0k`) AND BOTH RUNTIME SEATS WOKE AND ARE
> > WORKING.** Nothing is owed by me right now.
> >
> > **THE OBJECT.** `wp/RT-LEXICAL-R3-FUSION-EMITTER`, **freed**, tip
> > **`5d322edf`**, **9 commits**, base `d5912acd`, unarmed and unrouted.
> > `89ee005b` preserved WIP. Still ONE atomic candidate; flag
> > `REPRESENTATION_RULE_VERSION 4 → 5` at review routing.
> >
> > **RULING 1 — `evt_2m62086x60c94`, and it went AGAINST the external reading.**
> > The `StaticWorkerBinding` stop is an **R3 routing defect**. **The atomic
> > boundary does NOT move or shrink.** I routed it as a scope fork and supplied
> > the measurement that R3's range does not touch the refusal site; **the
> > ruling used that fact and rejected the inference from it** — *"the ownership
> > of the refusal site is not the ownership of the defect"*, and do not declare
> > it external merely because R3 did not edit the site. **Supplying the
> > measurement and declining to rule on it was the correct split; do not
> > re-derive it as a mistake.**
> >
> > `AC-D3-ROUTE`: the fused path carries the worker-bearing `D2gOut`
> > intermediate through the authorized outer elimination, kind-preserving
> > rebind, and exact worker call, **then** emits the ground value. **Its
> > control must FAIL when the intermediate terminalizes early while the
> > post-consumer result stays accepted — a green armed run does not discharge
> > it.**
> >
> > **RULING 2 — the planner-side guard.** `AC-D3-SELF`'s open half is an
> > **owed planner obligation**, and the frame's "not closed" reading was
> > confirmed in the tree: the implementer committed a correction for its own
> > sentence claiming the resolution *"is established at preflight"*, which was
> > **false** — `BinderAgreement` proves only the marginal facts. Resolution
> > shape is scoped in the frame and is the released unit.
> >
> > **THE HAZARD ON RESUME, and it is mine.** **The frame at `d5912acd`
> > PREDATES both rulings.** All amendments are held on `steward/work`:
> > `f3a8808b`, `c0422594`, `3eaa9f3f`, `2a0e7d31`, `f6eeabe6`. A seat reading
> > the frame in its worktree will not find `AC-D3-ROUTE` and will see a stale
> > `AC-D3-SELF`. **I said so explicitly in the release rather than publishing**,
> > because moving `main` stales the base the gate just reset all three seats
> > to. **Publish at the seam where the object lands — and re-check this
> > sentence first.**
> >
> > **OWED BY RUNTIME, unshrunk by ruling:** the planner guard + its
> > discriminating control (released unit); selector nets 2 and 4; controls 2
> > and 3; the inner-slot-widening witness; the self-edge closeout coverage; the
> > `AC-D3-ROUTE` routing repair + discriminator.
> >
> > **RESOLVED, so it is not re-opened:** verify-qa's pane claimed an approved
> > `2bc6cb80` "awaiting votes". It is the **pre-squash head of merged PR
> > #1854**; node status `merged`; **the blob is byte-identical on `main`**.
> > **A pane's last conclusion is a stale CLAIM, not just a stale liveness
> > signal** — the blob check settles it in one command.
> >
> > **FLEET SHAPE, measured:** the program is **Runtime-bound** — 13 `ready` +
> > 33 `draft` Runtime nodes against one ring. Kernel, Foundation, Verify, Ergo
> > and Doc are idle for legitimate reasons, **not framing debt**. Language
> > holds **3 `ready` nodes it cannot execute** because its implementer seat is
> > down on a provider content refusal; that hold is with the operator at
> > `evt_5vwmmrr2w7ces` — **do not re-raise.**
>
> ### RESUME HERE — earlier state at 2026-08-12 ~21:0xZ. `main` = `d5912acd`.
>
> > **RUNTIME IS SELF-DRIVING ON R3 AND THE RING'S HANDBACKS HAVE BEEN CORRECT
> > EVERY TIME.** Do not read the length of this node as a ring problem — read
> > it as a sizing problem, which is mine. Four serial stops on one node, each
> > found by arming and hitting it.
> >
> > **THE OBJECT.** Branch `wp/RT-LEXICAL-R3-FUSION-EMITTER`, **freed**, tip
> > **`fe5c311e`**, base `d5912acd`, **7 commits**, unarmed and unrouted.
> > `89ee005b` stays preserved unrouted WIP. `DP`+`D1`+`D2`+`D3` merge as ONE
> > candidate (Architect `evt_4m0q1m4zn4k79`) — no standalone `DP` or `D1`
> > merge, no QA route on `89ee005b`. Flag `REPRESENTATION_RULE_VERSION 4 → 5`
> > at review routing.
> >
> > **WHAT I RELEASED, and it does not wait on any ruling** (`evt_7r043m92mz7fb`):
> > selector net 3 (duplicate/replay/escaped/unconsumed) plus the `D3`
> > call-site-versus-body discriminator. Both unarmed, both already owed, both
> > in scope under **every** disposition of the blocker below. The implementer
> > woke on that release and is working.
> >
> > **THE DISCRIMINATOR'S MEASURED INPUTS, which are the whole reason it is an
> > AC:** `consuming_call=17` while seat, `producer_body` and `redirect_callee`
> > **all print `37`**. Three-way coincidence; folding call-site into body
> > **type-checks**; a control whose expected values are all `37` passes under
> > the fold and proves nothing. In the frame.
> >
> > **TWO QUESTIONS ARE WITH THE ARCHITECT. Neither is mine and I did not
> > pre-empt either.**
> >
> > 1. **Planner-side guard** (leader routed, `evt_6pp2we806xy0p`). The ruling
> >    requires the consuming-callee/binder relation to **resolve to**
> >    `producer_body`; that resolution is preflight's and needs `ih_bindings` +
> >    `SemanticIr::child_origin`, `pub(super)` to the planner. The implementer
> >    checks the redirect producer entry against the claim's producer body
> >    instead — a real two-route cross-check, **not the ruled relation** — and
> >    named the gap in a comment. **`AC-D3-SELF` is NOT closed by that.** A
> >    named limit is not a met criterion; do not read the comment as either the
> >    closure or a ruling that the cross-check suffices.
> > 2. **The `StaticWorkerBinding` blocker's classification** (mine to route,
> >    `evt_7r043m92mz7fb`). The armed compile now advances past both D3
> >    refusals and stops where a constructor field escapes to a ground value
> >    transporting a static worker binding, which has no value representation.
> >    **This blocks selector nets 2 and 4, controls 2 and 3, and the
> >    inner-slot-widening witness** — every armed-path obligation.
> >
> > **WHY I DID NOT RULE (2), and do not rule it on resume.** The scope answer
> > depends on a design fact: is the **arrival** at that site correct-but-
> > unsupported, or is R3 routing something there that should not go? If the
> > arrival is correct, the armed-path obligations are **unreachable inside
> > R3** and the atomic object cannot satisfy its own ACs — relieving that means
> > moving the atomicity boundary of `evt_4m0q1m4zn4k79`, **the Architect's
> > ruling to amend, not mine to reinterpret as sizing.**
> >
> > **WHAT I MEASURED so the ruling rests on a fact, not the implementer's
> > characterization** — which I checked rather than took: **R3's range does NOT
> > touch the refusal site.** `lowering/mod.rs:3038` sits between diff hunks
> > that stop near 2839 and resume at 13814. The construct carries its own prior
> > `D2`/`D3` deliverables (`core.rs:12347` sole construction route,
> > `core.rs:15631` sole consumer) from an **earlier node** — those D-labels are
> > not R3's. So the **code** is untouched and only what **flows into** it
> > changed. **That is consistent with both readings and settles neither.**
> >
> > **HANDOFF GATE: DEFERRED, NOT DROPPED** (`evt_5rqhfkvrxz6xx`). The leader
> > asked for one; I checked preconditions, then checked the seat, and the
> > implementer was **already working at ctx 40%** on the release. Gating a live
> > turn resets and compacts it. **Preconditions are measured and it is now a
> > one-step action:** all three Runtime worktrees clean, on their own
> > `<role>/work` branches, **0 ahead of `origin/main`**, **none checked out on
> > the WP branch** — so no `preserved/` ref would be minted and the object is
> > not in the reset's path. Run it the moment the implementer hands back.
> >
> > **STILL HELD, DELIBERATELY UNPUBLISHED:** `f3a8808b` and `c0422594`
> > (briefing + all frame amendments). Publishing moves `main` and **stales the
> > base the leader named to its implementer by hand**; the commits unblock no
> > product WP (`COORDINATION §10⁻` rule 3). Publish at the next substantive
> > seam — landing the object is one.
> >
> > **SEAT MECHANICS, live this hour.** The runtime-leader stalled again in the
> > documented shape: its own tick sampled the implementer mid-work, concluded
> > "No action needed", and it did not wake for the 20:49 handback. **Compare
> > last-turn time to handback time; the tick is what makes the ring look
> > healthy.** The architect stranded on a `[Pasted Content 2754 chars]` for the
> > third time — a paste needs a **second `Enter`**. Both recovered by pane
> > rouse, neither by a convo read.
>
> ### RESUME HERE — earlier state at 2026-08-12 ~18:5xZ. `main` = `d5912acd`.
>
> > **HANDOFF GATE RAN, THEN THE RE-RELEASE — in that order, and the order is
> > the point.** `scripts/handoff-gate-compact.sh runtime-leader
> > runtime-implementer runtime-qa` at ~18:5xZ, on runtime-leader's own request
> > (`evt_5fvg2z9spef4h`) after the planned control-1 stop.
> >
> > **Preconditions I checked before launching, because the reset is
> > destructive:** all three worktrees clean, each on its own `<role>/work`
> > branch, **none checked out on `wp/RT-LEXICAL-R3-FUSION-EMITTER`**, and none
> > ahead of `origin/main` — so no `preserved/` ref was minted and **the atomic
> > object at `8cde622c` was never in the reset's path.** Nothing owed in
> > flight by any of the three.
> >
> > **I published NOTHING during the gate window.** `main` was `d5912acd` when
> > the gate started and `d5912acd` when it returned, so the three seats are
> > reset to the *current* `main`, not a snapshot of an older one. Stated
> > explicitly because that skew is silent in both directions.
>
> > **LANGUAGE RING IS DOWN — `language-implementer` stopped on a provider
> > content refusal at ~19:0xZ, reported to `language-leader` at
> > `evt_73ymsgtq0rxb6`.** It had committed `8e9baa18` (parser depth control,
> > `+30`) and its test came back **`FAILED. 0 passed; 1 failed`** before the
> > refusal ended the turn with no post. Branch `wp/LANG-SURFACE-RECORD-LITERAL`
> > at `8e9baa18`, 3 over base `57688110`, tree clean — nothing lost, because the
> > seat had been ordered commit-first. **Do not re-prompt that seat to explain
> > the failure; the refusal fires on SUBJECT and will re-trip.** The red test is
> > Language's to diagnose, not mine.
> >
> > **The catch was late and the reason was my own instrument.**
> > `scripts/steward-pane-sweep.sh` scored LIVE on the regex `…|esc to
> > interrupt|▰`, and the bare ellipsis matched **`… +617 lines (ctrl + t to view
> > transcript)`** — a truncation marker in stale tool output, not a spinner. A
> > stopped seat read as busy. **Fixed by stripping the `… +N lines` form before
> > the test**, verified with both controls: the dead seat no longer reads LIVE
> > and `runtime-implementer`, genuinely working, still does.
> >
> > **THE HOLD IS LEGITIMATE AND ALREADY WITH THE OPERATOR — do not re-raise
> > it.** After the rouse, language-leader posted `evt_7nta6rbkmfbtp` into
> > `thr_1j4sgf8wve5hh`: ring paused, work preserved, *"awaiting the operator's
> > lane/provider decision from `evt_5vwmmrr2w7ces`; no further implementer
> > prompt or reset is authorized."* That is a `COORDINATION §1a` hold with a
> > real address — named event, named owner — so it is a legitimate wait, not a
> > stall, and the leader's read is correct.
> >
> > **What does NOT reconcile, and it is the operator's to look at rather than
> > mine to decide.** My own superseded block at line ~327 records *"LANGUAGE is
> > with the operator (`evt_5vwmmrr2w7ces`) and stood down"* — dated 2026-08-10.
> > **Yet the ring built three commits today**, the last minutes before the
> > refusal. So either the operator ruled and no artifact records it, or the ring
> > resumed under a hold that was never lifted on the record. **This is the
> > "gate nobody reopened" shape running the other way:** the failure this fleet
> > keeps hitting is a satisfied precondition left shut, and this is work
> > proceeding while the durable record still says stood-down. Both directions
> > are invisible to a tracker reader, and neither is fixed by re-asking.
> >
> > **I am NOT re-raising the decision** — it is already with the operator, and
> > re-asking a question they hold is the thing my own do-not-re-raise list
> > exists to prevent. Recorded here because their view is this file.
> >
> > ### THE WAKE PATH IS THE FLEET'S WEAKEST LINK TODAY — four catches, one shape
> >
> > **Every one of these was found by reading a pane, never by a convo read.**
> >
> > 1. `language-leader` — mention did not wake it; pane rouse did.
> > 2. `runtime-leader`, twice — **its own watchdog tick sampled the implementer
> >    mid-work, concluded "no gap detected", and then did not wake for the
> >    handback that arrived minutes later.** The tick actively makes this worse:
> >    the leader has just affirmatively concluded all is well. **Always compare
> >    a leader's last-turn time against the handback timestamp.**
> > 3. `architect` — sat with **two** stranded pastes
> >    (`[Pasted Content 1730 chars][Pasted Content 1098 chars]`), last completed
> >    turn ~18:30, while the Runtime ring's fork waited on it. One `Enter`
> >    submitted both and it woke and re-oriented.
> >
> > **THE UNCONDITIONAL RE-SWEEP PAID OFF ON ITS FIRST REAL USE, ~20:3xZ.** It
> > caught a second Architect strand — `[Pasted Content 2754 chars]…</channel>`,
> > a truncated notification sitting unsubmitted while its last turn had
> > completed. **The conditional version would have missed it**: the Architect
> > was not obviously mentioned in the events I had just read, so I would not
> > have triggered the condition. One `Enter` cleared it and the seat went
> > `LIVE`. Keep the step unconditional.
> >
> > **My own rule would have caught (3) and I skipped it.** The tick says
> > *re-sweep after any post mentioning the architect*; I found it only because I
> > chose to verify the routing had landed. **A conditional step is one I have
> > now demonstrated I skip, so the tick now ends with an UNCONDITIONAL
> > re-sweep** — the sweep is cheap and the condition is what fails.
> >
> > **Rouse mechanics, learned the hard way this hour:** keep a pane rouse
> > **under about 1000 characters**. Over that it arrives as
> > `[Pasted Content N chars]` and the first `Enter` accepts the paste rather
> > than submitting — **a second `Enter` is required.** Shorter rouses submitted
> > on one.
> >
> > **And read panes WIDE.** Three times this hour a `tail -5`/`tail -8` caught a
> > working seat between renders and read as idle or failed. `capture-pane -p -S
> > -40 | tail -20` is the floor. This is the same positional trap as the
> > `Compacting` bar: the window structurally cannot answer, and it returns a
> > confident wrong answer rather than "unknown".
> >
> > **THE CONVO MENTION DID NOT WAKE THE LEADER — a pane rouse did.** Fifteen
> > minutes after `evt_73ymsgtq0rxb6` both Language seats still sat at an empty
> > composer and the leader had posted nothing. Roused at ~19:2xZ with
> > `send-keys -l <text>`, `sleep 2`, **separate** `Enter`, pointing it at the
> > event id rather than restating the report; it went `Working` within seconds.
> > **Escalate a silent mention to a pane rouse, not to a second post** — a
> > re-post lands exactly the way the first one did. Graduated recovery per
> > `COORDINATION §13`, and the pane is the only instrument that showed the gap.
> >
> > **The fix removes a false positive and does NOT give Codex seats a liveness
> > signal — they print `(no-footer)` whether working or dead.** For those seats
> > the instruments are the pane BODY and the seat's convo activity. Recorded in
> > the script header so `(no-footer)` is read as "cannot see", never as "fine".
> > Language had also posted **nothing to the space since before 15:00Z**, which
> > is the other half of why nobody noticed.
>
> > **THIS BRIEFING COMMIT IS DELIBERATELY UNPUBLISHED — publish it at the next
> > substantive seam, not on its own.** `COORDINATION §10⁻` rule 3: a process
> > merge must name the product WP it unblocks or wait, and this one unblocks
> > nothing. It is also live: runtime-leader told the implementer to rebase the
> > atomic branch onto **`origin/main = d5912acd` by name**, so moving `main`
> > for a diary-only delta would make the ring's own instruction read stale for
> > no gain.
>
> **RUNTIME IS BUILDING THE ATOMIC OBJECT'S NEXT UNIT — the `DP`+`D1`/`D2`
> composition splice.** Do not nudge it; the frontier is moving. Three
> publishes this arc: `e48c2f90` (`DP-0`, PR #2033), `75e693f6` (the `DP`
> sizing ruling, PR #2034), `cf955abd` (the Adversary triage, PR #2035).
>
> **`DP` IS RULED, SIZED AND RELEASED — the Architect-blocked stop is CLEARED.**
> Ruled **(a)** at `evt_w4nvsmrs1qhk`: one checked computational-IH invocation
> occurrence exists (`D2G_CALL`), the producer's `D2G_INNER_SLOT` is a binder
> template and not an invocation source, so (b) would invent a call occurrence
> the source does not contain. My sizing and contention re-check are the block
> above section 6 of the R3 frame, and **that block is the release.**
>
> **`DP` IS NOT SEPARABLE. THE REVIEW OBJECT IS `DP` + `D1` + `D2` + `D3`, ONE
> ATOMIC CANDIDATE** (`D4` only under the necessary-green rule). Architect
> `evt_2f0nnwtzqy65m` ruled my dispositions **2 and 3 together** and **withdrew
> its own population sentence** from `evt_w4nvsmrs1qhk`.
>
> **The corrected membership law, and it is the whole ruling in one sentence:**
> *checked occurrence identity is planner-authored; membership in a concrete
> invocation segment is established at the checked event that actually adds that
> semantic layer.* Membership happens **at the composition splice**, not at
> population time. Runtime may **validate** that relation; it may not
> **discover** it from shape, `ParentFrame`, segment-site equality, or a fusion
> label. **(a)-vs-(b) is preserved** — one call marker, one instance, no second
> source. What changed is **when** the producer frame becomes a member.
>
> **The base plan stays EXACT:** `outer slot`/`call = [outer frame]`, `inner
> slot = [inner frame]`. `D2G_OUTER_SLOT.frame_templates` must **never** be
> populated by transitive `ParentFrame` closure. **The unarmed `ReHomed` refusal
> is CORRECT** and the instantiator must not be weakened, given a fusion
> exception, or made to accept a subset.
>
> **FORBIDDEN, all three previously in play:** no standalone `DP` population
> merge, no standalone held `D1`/`D2` merge, no QA route on `89ee005b` (it is
> preserved WIP negative evidence, earns no credit, is not a candidate).
>
> > ### RULED AND DISPATCHED — `D3` SELECTOR. Runtime is building it.
> >
> > **Architect `evt_4g2hmsr8tb3bm` (20:04Z), leader dispatched
> > `evt_255qnj0b773ja`, implementer building.** The ruling and its **five
> > selector nets are now written into the frame as `AC-D3-SEL`** — that is a
> > frame amendment and it was mine to author, not the ring's to fold in.
> >
> > **The ruling:** an affine capability issued by the checked
> > fusion-composition splice and bound to **that splice's specific pending
> > semantic edge**, consumed at most once by the segment that actually consumes
> > the edge. **Forbidden:** an ambient body flag, a counter, a global or
> > body-scoped "next call wins", a Runtime-shape search, and **retaining
> > `fused_composition_extent`**.
> >
> > **MY FLAGGED TENSION WAS ANSWERED, AND FLAGGING RATHER THAN RULING IS WHY
> > IT WAS USEFUL.** `evt_2f0nnwtzqy65m` does **not** forbid the corrected
> > shape — the capability contributes no frame ID and proves no membership.
> > **But it does forbid the raw formulation** the implementer offered: a permit
> > grounded only in "we are in a fused body" or "first compose observed" makes
> > execution order the authority. The answer was neither yes nor no, and a
> > pre-emptive reading of my own would have destroyed it.
> >
> > **Not a planner per-occurrence key now** — it would duplicate authority the
> > fusion claim and pending edge already carry. Escalating to one is a **new
> > hard stop**, never licence to use the coarse permit.
> >
> > **SELECTOR NET 1 IS BUILT AND MEASURED WORKING** (`evt_2ffgnktmm34ta`,
> > 20:17Z). In one fused body `inst=1 layers=[1,0]` selects `Composed`
> > `expected={0,1}` and `inst=2 layers=[0]` selects `Ordinary` `expected={0}` —
> > the required net exactly. **`fused_composition_extent` is GONE** as the
> > ruling demands. All four capability refusals are in: replay/double-consume,
> > unconsumed/escaped, failed descent (**withdrawn, not consumed** — withdrawal
> > mints no receipt), and two splices on one segment.
> >
> > **One rot risk the implementer flagged unprompted, worth keeping:**
> > `ComputationalEliminatorFrame` is `Copy`, so **the id alone is not affine**
> > — a copied edge carries a copied id. Membership in
> > `outstanding_splice_capabilities` is the single spendable fact, and the
> > issuer is monotone so a spent id can never be answered by a later issue.
> >
> > **HELD at `da0e2ba2`, base `d5912acd`, 6 commits, unarmed, unrouted**;
> > `ken-runtime` 906/0/4, `ken-elaborator` 122/0. **Still owed: selector nets
> > 2, 3, 4; controls 2 and 3; the inner-slot-widening witness** — none earned by
> > the selector work. **Net 3 is buildable WITHOUT arming**, straight against
> > the capability ledger, and is the named next unit.
> >
> > **STOP 3 IS RULED AND DISPATCHED — the ring is fully self-driving.**
> > Leader routed `evt_7dp6b7ja5fp58` (20:31), Architect ruled
> > `evt_4x3291v9dx0vb` (20:40), leader dispatched `evt_6fzg11hpvfp4w` (20:41),
> > implementer building. **No rouse was needed at any hop** — the wake path
> > worked unaided this round, so it is unreliable rather than broken.
> >
> > **The ruling: the moved suffix's claimed IH invocation is a RECURSIVE CALL
> > TO THE SAME `Fusion(id)`** — not a no-op/current-result substitution, not a
> > call to the standalone producer. No new planner population; the preflighted
> > `FusionRegionClaim` already carries every identity. **Written into the frame
> > as `AC-D3-SELF`**, with the rejected repairs named so none is re-proposed.
> >
> > **The trap in that AC, and it is the reason I wrote it rather than leaving
> > it to the ring:** this fixture prints `37` for more than one axis, so a
> > control that passes on that numeric coincidence proves nothing. `AC-D3-SELF`
> > requires a **discriminator separating call-site identity from body/callee
> > identity**, and forbids keying on the coincidence or on "missing target
> > while in a fused body".
> >
> > **NEW HARD STOP, and it is the THIRD instance of one shape** — a premise
> > about the fused function written before `D1`/`D2` moved the suffix into it.
> > `define_static_continuation_fusion_bodies` declares only the producer's
> > edges; the armed compile now hits `call_declared_unit(StaticOriginId(37))`
> > against `unit_calls = [34]`. **Recorded as a table in the frame** with the
> > two measured facts that close off the obvious repair (the consumer's static
> > body edge set is EMPTY, and reaching seat 37 inside the fused body is a
> > question about what the suffix lowers to, not about edge tables). **Each
> > instance was found by arming and hitting it, so the remaining count is
> > unknown** — I offered a census as an option and deliberately did NOT make it
> > an AC.
> >
> > **The fork:** one fused function builds **two** checked segments — composed
> > (`layers=[inner,outer]`, needs `{0,1}`) and ordinary (`layers=[outer]`, needs
> > `{0}`). `DP`'s body-extent selector marks **both** composed, so the ordinary
> > one refuses with `expected={0,1} instantiated={0}`. **The validator is
> > behaving correctly and the selector is too coarse.** Three discriminators
> > were measured and eliminated: Runtime segment shape (forbidden and circular
> > — it is subset coverage renamed), `D1`'s per-phase authority (**measured
> > identical at both composes**), and `RecursorProducerOriginId` (a
> > lowering-minted counter, not a plan key). Remaining: **(a)** an affine
> > one-shot permit armed by the splice, fail-closed but resting on an ordering
> > premise, or **(b)** a planner-authored per-occurrence key naming the composed
> > segment from the checked source.
> >
> > **The routing call was mine and I made it: soundness/design ⇒ Architect**,
> > not me and not the leader. **I flagged one tension WITHOUT ruling on it** and
> > the leader carried it verbatim: does `evt_2f0nnwtzqy65m`'s prohibition on
> > Runtime discovering membership from Runtime context already exclude (a) one
> > level down? **That reading is the Architect's, not mine — do not pre-empt
> > it.** The implementer declining to settle it alone was correct: it had just
> > been burned by a false ordering premise on the same turn (`D3` defect 1, a
> > fused region taking over its own suffix), and trading one ordering premise
> > for another is not an implementer's call.
> >
> > **THE ATOMIC OBJECT IS UNDER CONSTRUCTION ON A BRANCH, NOT ON `main` — a
> > cold resume will not see it in the log.** `wp/RT-LEXICAL-R3-FUSION-EMITTER`
> > tip **`8caaa5d6`** (was `69f671d2` before the `D3` defect-1 repair),
> > base `d5912acd`, **5 commits**; the `DP` measurement below was taken at
> > `69f671d2`, **4 commits, 10 files, `+835/-26`**,
> > handback `evt_5pfgetdgv3bkf` at 19:32Z. `DP`+`D1`+`D2` are built, measured,
> > **unarmed and unrouted**. Suite `906` passing against `905`; `ken-elaborator`
> > clean; production-profile build of both crates clean, so nothing is
> > `cfg(test)`-only. **`D3` arming is authorized next** (runtime-leader,
> > ~19:4xZ), with controls 2 and 3 following on the armed composed path.
> >
> > **Superseded tips, both of which a compaction will happily carry forward:**
> > `8cde622c` (base `bd170bef`) → `a23a0393` → now `69f671d2`. I verified the
> > first rebase myself with `git patch-id --stable` — identical
> > (`2b7bb356e0db117c…`) — and the implementer reported the same for `D1`
> > (`4dd1e63c`→`7f31b267`) and `D2` (`7166baaa`→`32a5dc3e`). **Cite
> > `69f671d2`.**
> >
> > **`REPRESENTATION_RULE_VERSION` went 4 → 5** (length-prefixed sequence in the
> > encoding). Flag it at review routing — a representation-rule bump is the kind
> > of change a conformance reader wants to know about, and nothing in the frame
> > anticipated it.
> >
> > **THE MEASUREMENT CORRECTED THE FRAME, and the frame correction is MINE and
> > is now written.** An armed probe run before any code showed the producer
> > layer **already carries its own checked frame id** — `(frame=Some(1),
> > invocation=None)` and `(frame=Some(0), invocation=Some(1))`. `DP` does not
> > mint an identity; what was absent is **invocation-source coverage**. The
> > frame's leading DP sentence said *"give the producer semantic occurrence its
> > own transported checked identity"*, which contradicts its own PRECISION
> > CORRECTION eleven lines below it. **Struck in place and replaced**, with the
> > probe box, in section 5 — the withdrawn-claim-in-the-leading-sentence shape,
> > invisible to a line-local read.
> >
> > **Second measurement, and it is the stronger one:** one call template is
> > entered **twice in a single compile** — composed via
> > `lower_fused_producer_through_suffix`, ordinary via `define_unit_bodies`.
> > Widening the shared base refuses the second with `expected={0, 1}
> > instantiated={0}` — **the same refusal `89ee005b` produced, reached by a
> > second independent route.** No template-level widening can satisfy both.
> >
> > **It is NOT a candidate and takes no QA route and no credit.** Control 1
> > only — the base singletons plus the three unarmed uncomposed roots, both
> > halves in one assertion because either alone passes for the wrong reason.
> > A/B run by applying the preserved `89ee005b` population and reverting it:
> > RED with, GREEN without, and `ReHomed`'s error is the exact
> > `expected={0,1} instantiated={0}`.
> >
> > **Carried gap the implementer named rather than glossed:** the inner slot
> > stays `[1]` on **both** sides of that A/B, so the inner-slot-widening item
> > of control 4 is **untouched by the mutation rather than met** — it needs its
> > own witness and does not come for free. Do not read control 1 as covering it.
> >
> > **Next unit is the mechanism, not another control.** Controls 2 and 3 are
> > stated over the armed composed path and have no subject until `D1`+`D2`
> > build the splice and `D3` arms it. Hard stop at that boundary was correct
> > and is the predicted shape.
>
> **How I got here:** I released `DP-1`/`DP-2` arguing the split was safe
> because `D2F_EMITTER_ARMED` is `false`. **The flag gates the fusion emitter,
> not every consumer of the transported sequence** — unarmed `ReHomed` refused
> `{0,1}` vs `{0}` (`89ee005b`). I read one named gate as bounding the whole
> population reaching a mechanism. **Expect hard stops inside one atomic object,
> not a sequence of merges** — the one-hour target cannot be met here and that
> is the correct shape, since the alternative is a partial that cannot carry its
> own positive control.
>
> **SECTION 9 UNDERSTATES `DP`'s SURFACE — I measured this, the Architect flagged
> only the elaborator.** 24 sites across six files carry a slot `frame_templates`
> or a `callee_frame_templates`, and **four of the six are outside section 9**:
> `ken-elaborator/src/erasure.rs`, `oriented_subcontinuation_plan.rs` (both
> binding fingerprints consume the sequence), `lowering/mod.rs`, and
> `planning/static_transition.rs` (the whole `D2G_*` fixture, 49 refs, one file).
> Section 9 is a **floor** for this node, not its surface.
>
> **Contention is empty and STRUCTURALLY excluded for `DP`'s duration, not just
> absent today.** No worktree holds an uncommitted edit to `erasure.rs` (two
> `.claude/worktrees/agent-*` scratch trees hold other elaborator files; none is
> that one). [[KERNEL-NESTED-IND]] `D5` **does** claim `erasure.rs` and defines
> its lane as every path an `AC-K12` stage traverses minus `crates/ken-runtime`
> — but Kernel is blocked behind `RT-DYNAMIC-ARM-SCALAR-MERGE` (`ready`) and
> `RT-NESTED-IH-NATIVE-REALIZATION` (`draft`), both **Runtime** nodes, and
> Runtime runs one node at a time.
>
> **Re-measured at the re-release: the one other ring in flight is Language, and
> the intersection is EMPTY.** `language-implementer` is live on
> `wp/LANG-SURFACE-RECORD-LITERAL` (3 commits, base `57688110`). It touches
> `crates/ken-elaborator/` — **the same crate as `DP`** — but the files are
> disjoint: `ast/elab/lossless/modules/parser/resolve.rs` plus tests, versus
> `DP`'s `erasure.rs`. Same crate is a CI compile coupling, not a merge
> contention; the silent-union hazard is per-file and there is no shared file.
>
> **My commit `80f22449` says "KERNEL-NESTED-IND is unblocked" and that subject
> is narrower than it reads.** One dependency edge closed; two remain open. The
> kernel-leader's *"still Runtime-blocked at `AC-K12`"* is **correct** — do not
> "fix" it. Kernel and Foundation are both idle behind that same chain, and
> `RT-NESTED-IH-NATIVE-REALIZATION` at `draft` is the framing debt in front of
> them. **That is the next backlog item after `DP`.**
>
> **WIP AUDIT CLOCK: RE-ARMED on Runtime from the post-gate re-release
> (~18:5xZ), superseding the `DP`-release arming at `evt_37166e7aq0xts`.** A
> routine progress post does not reset it. Resets on a hard stop, an Architect
> ruling, or a candidate handoff. **Expect it to fire without a merge** — this
> object lands whole or not at all, so a long WIP is the designed shape here,
> not by itself a stall. Diagnose on whether the ring can name its next
> construction, never on elapsed time alone.
>
> **`DP-1` also carries two comment repairs and `DP-2` one added AC**, from the
> Adversary pass `evt_2933sm5wnh2je`, both reproduced by me before folding. The
> census at `core.rs:2252` **spells the id it counts** — `1` at its pinned
> `49072fb8`, `2` at the shipping tree where the second hit *is that sentence*.
> Repair positionally, not by re-pinning: a census that stops counting itself
> needs no custodian. `DP-2` gains a control that reds if the producer identity
> is ever obtained by copy rather than transport, with a **hard stop** if it
> cannot be written without the inference the ruling forbids.
>
> **RUNTIME is on `RT-LEXICAL-R3-FUSION-EMITTER`.** Its thread anchor is the
> root post `evt_391zsaw72wmna`, thread `thr_7fz4j2x2pgzvb` — **not** the D2k
> thread, where my original `DP` kick wrongly went and was reissued from.
> `thr_49738q826cs1t` stays D2k's and is still open.
>
> > **A PUBLISHER `GraphQL: Base branch was modified` CAN BE A FALSE FAILURE.**
> > PR #2035 returned it and **the merge had landed** — `cf955abd`, blob
> > identical — while GitHub left the PR record `OPEN`. Decide with M6 blob
> > identity, never the error text. Then **close the orphan**: an abandoned PR
> > whose content is already on `main` ages into a revert.
>
> **RUNTIME — `RT-LEXICAL-RECURSOR-CONSUMERS-D2k`.** Anchor
> `thr_49738q826cs1t`. The frame is the durable record and carries the full
> text; read it, not this block. The Architect rebound `AC-1`
> (`evt_290zp8kxn9jbs`, checkpoint 16532068), **superseding his own earlier
> *"route repair, never a criterion correction"*** once measurement falsified its
> premise: the five named witnesses cannot carry the repair's required input.
> **Route work stops and the frame's section 8 fires.** The rebound criterion is
> `AC-1a` (a checked positive resolving a real fusion) and `AC-1b` (the five
> seeds preserved as absence comparators, zero credit). `D2k-1c-1a` is open and
> unmeasured, and still needs the live recognize/rebind/consume arm.
>
> **MERGED: the R3 comment-only accepted partial, exact `112c07f5`, as PR
> #2028.** Range `21307d7f...112c07f5`, one commit, one Runtime path
> (`lowering/core.rs`), `+43/-13`, **every changed line a `//` comment**. It
> superseded `754e8c4e`, which was never routed. QA `evt_angdenz40e57`,
> Architect `evt_1farww1aqrbzj`, Decision `dec_31mc34k79wt2n` resolved.
> Adversary notified at `evt_36jxatgr8kk1w`; ring closed at `evt_6p3ezb3zamwrr`.
>
> > **THE TWO DISCRIMINATORS DIFFER NOW, AND I GOT THIS WRONG BEFORE PUBLISH.**
> > I twice told the ring this was a CODE merge taking a full CI poll, reasoning
> > from *"something under `crates/` moved"*. The operator widened `--doc-only`
> > on 2026-08-12 to cover **comment-only changes inside `.rs` files** — the
> > discriminator there is the **content of the diff**, not the file extension,
> > and it must be established mechanically in **both** directions, plus a check
> > for an added `///` fence, since a doctest is a compiled test wearing a
> > comment's syntax. *"Anything under `crates/` moved"* decides **M8 only.**
> > This candidate was therefore `--doc-only` (about two minutes, no CI poll)
> > **and** still owed the Adversary its notification. Do not fuse them again.
>
> **`M3` IS ALREADY DONE AND CLEAN ON THIS CANDIDATE — do not redo it.** The
> comment cites Architect `evt_1q7v9fcw5hd87`. A `get_thread` on that id returns
> `Thread not found`, which is the **expected** result for any non-root event and
> is **not evidence either way** — do not read it as a missing citation. It is
> corroborated by four durable citations on `main`, and the two load-bearing ones
> carry the exact claims the comment attributes. Both are in the R3 frame
> `docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md`:
> **`:124`** (the cumulative stop fired at `D2`) and **`:370-380`** (a
> fusion-only admission, and copying or inferring the consumer identity onto the
> producer, both ruled **unlawful**). Posted `evt_3ad99t706j226`.
>
> **That merge takes NO `DP`, AC, node-closure or arming credit.** The node
> stays `active` with `DP` unstarted and `D2F_EMITTER_ARMED` false.
>
> **`D2k-1e` is framed and landed at `58c82ba8` and is QUEUED BEHIND `DP`** —
> do not start it and do not fold it in. It belongs to the D2k node and its
> thread. It closes the Adversary's `evt_4n9x6a46whzqp`: `D2k-1d` repaired the
> reason and left the claim it supported, so *"The phase is asserted per row"*
> at `control.rs:2717` was weakly true before it and is **false after it**.
>
> **Two obligations were split out to `RT-LEXICAL-R3-FUSION-EMITTER` as its own
> `AC-9`/`AC-10`** (no new node): the semantic-effect half needs an **installed**
> fusion, which that node owns; and neither hole route has a lawful checked
> positive, because `D2jCause` has no hole axis at all — that axis is `px8j`
> seed-only.
>
> **The node's FIVE forced zeros, all recorded in the frame.** `installs==0`
> (branch never entered), `FUSIONS installed=0` (gate off, `D2F_EMITTER_ARMED:
> false` — a labelled un-wired partial, **not** an absent chain), `keys=0`
> (enumeration short-circuited on `oriented: None` before reading one planner
> fact), `Some(empty)` **admitted and resolving nothing** — measured 2026-08-12
> (`evt_6xywtcrdyq08s`), the only one with a demonstrated non-zero positive
> control — and **three `D2jCause` variants refused at the validator that never
> reach the builder at all.** That fifth one was caught inside an acceptance
> criterion: a uniform "must fail to resolve" would have credited three never-ran
> zeros as evidence the mechanism discriminates. **All five read as "the
> population lacks the property"; all five mean "the instrument never looked."**
> Any zero on this node must state which it is and name the precondition that
> tells them apart — and a never-arrived row is asserted **as non-arrival**,
> never as a zero key.
>
> **The standing lesson this node produced, and it is three instances deep.**
> `D2f` `AC-1`/`AC-2`, `D2k-1c-1`, and `D2k-1c-2` `AC-1` each pinned acceptance
> to a witness that cannot carry the mechanism's required input — twice at the
> identical `oriented: None` gate pair, on sibling fixtures of one family.
> **Run the witness and read the mechanism's input before pinning an AC.**
> The consequence still live: if the mechanism does require an oriented plan,
> `AC-1`'s five rows discharge **vacuously in both directions** — green on a
> no-op — which is measured `D2f` behaviour, not a worry.
>
> **`D2f` is DONE as a turn and its emitter is BUILT AND UN-WIRED.** Do not read
> `D2F_EMITTER_ARMED: false` at `lowering/core.rs:2231-2235` as an absent chain:
> `preflight`, `install_fusion_owned_bodies` and
> `define_static_continuation_fusion_bodies` run on every production compile,
> inert by empty population. The block's own comment says so. **Never merge
> `e4531318`, `9d942c4b`, `6676251a`, `ce5323ca`, `ea95a223`, `bd5961f8`,
> `d5c7df82`, `1ce8b424`, `50da348a`.**
>
> **Only three nodes are `active`: `RT-LEXICAL-RECURSOR-CONSUMERS`,
> `KERNEL-NESTED-IND`, `DS-9`.** The latter two hold `active` **deliberately** —
> that is not framing debt and not a stall.
>
> **LANGUAGE is with the operator** (`evt_5vwmmrr2w7ces`) and stood down. Every
> other build ring is idle **by design** under the two-lane cap; the doc track
> is exempt but has zero nodes left, so do not manufacture doc work. **The
> `integrator` roster line is a tombstone** — offline, absent from
> `actors.json`, no pane. Its status is stale by construction, never a stall,
> and its "PR #365 awaiting Steward routing" is not an open loop — **#365 merged
> on 2026-07-08 as squash `3859aaff`.** Neither recorded head is an ancestor of
> `main`, because a squash-merged head never is; that ancestry answer looks
> identical for a merged PR and an abandoned one, which is how the claim survived
> for weeks. **Ask the PR object for `merged`; never infer it from ancestry.**
>
> **Retirement order, all `ready` and framed:** `D2k` (active) →
> `RT-LEXICAL-R3-FUSION-EMITTER` → `RT-RECURSOR-TRANSPORT` →
> `RT-DESCENT-RETIRE`. `D2l` is **framed and NOT released**. The ranking itself
> is with the operator.
>
> **Awaiting the operator, do not re-raise:** SMT/Z3 gating `SEC1-IFC-R3`; the
> `COORDINATION §4a` threading amendment; whether to purge the dead integrator
> record. `a334b9a0` is evidence only and must never be published.

## CORRECTIONS — two claims the old file made that were FALSE

Both were **time-varying state wearing a permanent-looking hat** — the exact
failure the heartbeat prompt bans. Recorded so the *shape* is recognisable, not
just the instances.

### 1. "ARMED COUNTERS — the SOLE count of record" was stale AND retired

It read `RT-NATIVE-FNSPLIT: hard-stop 10 · next research pull #11` and `Architect
production blocks: 6 · next check #9`. **Both numbers were behind**, and the chain
they counted **is retired** — the operator stopped the FNSPLIT effort on
2026-07-26 and `SPEC-STORE-SPLIT` replaces it.

**A counter calling itself "the SOLE count of record" is the worst thing to
leave stale**: it invites a reader to trust it *instead of* measuring. ⇒ **There
are no armed counters now.** When the re-cut program exists, its node owns its
counts.

### 2. "TRANSPORT — convo MCP mostly DEAD" is FALSE

The old block claimed only `set_interval`/`subscribe` survived and routed all
reads through scratchpad HTTP scripts. **Measured across this entire session:
`orientation`, `list_decisions`, `post_response`, `list_participants` all work
over MCP.** Tracked as task `#110` because **the heartbeat prompt still repeats
the claim.**

**What IS true — the part worth keeping:**

- **NEVER call `mcp__convo__get_transcript`.** Its `limit` does not bound the
  response and it takes the stdio connection down with it. Operator prohibition;
  fleet law in `AGENTS.md`.
- **Mentions arrive TRUNCATED** — a doorbell, not a message. Fetch full text via
  the HTTP read path, with **your own** credential.
- **`list_decisions` can exceed the result cap** and spill to a file — grep the
  file rather than retrying the call.
- `claude mcp list` reporting `convo: ✔ Connected` **is not evidence** — it
  health-checks a fresh process.

## Preserved refs — QUERY LOCALLY. `origin` carries `main` ONLY.

> ### THIS SECTION WAS FALSE AS WRITTEN. Both halves.
>
> It said *"Origin holds 26"* and gave `git ls-remote origin
> 'refs/heads/preserved/*'` as the query. **Operator ruling, 2026-07-26:** *"clean
> up all of the non-main branches at origin."* ⇒ **All 63 non-`main` origin
> branches are deleted.** That `ls-remote` now returns **nothing**, and a reader
> running it would conclude the work was lost.

**Measured 2026-07-27 — the query is local, and the population is larger, not
smaller:**

```sh
git for-each-ref 'refs/heads/preserved/*'    # 78 refs
git ls-remote --heads origin                 # refs/heads/main — and nothing else
```

**A branch on one local ref is the NORMAL state of preserved work, not an
exposure.** Do not raise an unpushed ref as a finding.

**AND THE "EXISTS NOWHERE ELSE" CLAIM WAS WRONG ON EVERY ITEM IT NAMED.** Each
was checked at `origin/main = a1e29284`:

| the old claim | measured |
|---|---|
| `preserved/b2e-rejected-source-oracle` = `159f4109` | **present locally at that exact SHA** |
| `wp/RT-FNSPLIT-B2E-boundary-value-elimination` = `e1b540e2` | **present locally at that exact SHA** — delete neither |
| `preserved/rt-fnsplit-b2f-hardstop-{9,10,11}-evidence` | **no local ref of that name exists** — and it does not need to. Hard-stops #9/#10/#11 are all on `main`, across **12** files (`RT-FNSPLIT-B2{E,F,O,R,V}.md`, `RT-NATIVE-FNSPLIT.md`, `RT-VALUE-TOTALITY.md`, the B2O report + predictions, two WP frames, `diary/2026/Jul/25.md`). `bce75fec` is literally *"make hard-stop #11's evidence durable"*. |
| `preserved/architect-state-*` | **wrong prefix** — the refs are `preserved/architect-work-*` (5 locally). A ref name you cannot resolve is not a backup. |

**The transferable part: a "this exists nowhere else" note is a claim about a
population you did not enumerate, and it decays in both directions at once** — the
copy you were protecting had already landed in the repo, while the ref name you
recorded it under never existed. ⇒ **Re-derive from `for-each-ref` and `git grep`
on `main`; never from a hand-kept list of what is precious.**

## Operator rulings — 2026-07-21 ~12:45Z. SETTLED, do not reopen.

Kept inline deliberately: this is law, and a settled ruling is a **fixed input,
never a question to re-ask.**

- **No "ratification."** The Linux ABI II charter is a **planning document, not a
  commitment.** Nothing outside the project depends on our timelines. Do not
  re-raise status-correction as a decision.
- **Where anticipated and done diverge, fill the gap first** — hence
  `docs/program/10-linux-abi-completion.md`.
- **L2-1: no cross-compilation. CROSS-PLATFORM IS INDEFINITELY DEFERRED**
  (restated 2026-07-21 after I re-raised it). Manifest v2 is family-scoped and
  generated, **not** cross-target.
  **This ruling ALREADY ANSWERS any non-linux finding** — do not route one back
  as a scoping question. Record such findings as *observations against a deferred
  lane* and stop.
- **L2-0: all desirable, nothing deferred.** All nine `RepresentedUnavailable`
  operations get promoted.
- **Timing, timelines, and budget are the OPERATOR'S domain.** Do not reason
  about schedule or cost.
- ★ **My lane is token efficiency in terms of delivered work.** That is the axis
  to optimize and the one to report on.

**Standing test policy (operator, 2026-07-26):** *"Test oracles that assert facts
about source code, catalog, or documentation lines are an invitation for failure
and delay. Tests should focus on behavior."* ⇒ Executable form: **"does an edit
that changes nothing about how any program behaves make this test fail?"**

**Standing gate policy (operator, 2026-07-26):** the library currency ledger is
generated **at version release points**, **not enforced per merge.**

**`origin` CARRIES `main` ONLY (operator, 2026-07-26; restated 2026-07-28).**
A branch living on one local ref is **normal** and is never a finding. No
durability sweeps, no pushes of WP or seat branches, no ring reporting an
unpushed ref. The publisher's own candidate-branch push stays — that is how it
opens a PR.

**THE `integrator` SEAT IS RETIRED (operator, 2026-07-26).** *"remove any
references to the integrator. that seat was retired weeks ago."* ⇒ Every operative
reference is gone as of PR #1052 (`a1e29284`, 50 files) — PR template, CODEOWNERS,
`ci.yml`, four devcontainer files (including a **functional** `ctx-nudge.sh` case
arm), `COORDINATION.md`, `04-git-and-integration.md`, 40 WP frames, the roster
(29→28), git refs, worktrees. **The chronicles keep the word deliberately** —
`docs/program/diary/`, `agent/memory/MIGRATION-LOG.md`,
`docs/program/ds-campaign-judgment-log.md` (17 files, 501 occurrences): there it is
a true account of what the process **was**. **Instructions get corrected; records
stay records.** One residual is not mine to clear — the convo **participant**
still exists; see the LIVE block's operator-owed list.

**Canonical width: 96 (operator, 2026-07-26).** *"re 88 v 96. 96 is what it should
be. It was an incomplete revision, apparently."* ⇒ `spec/30-surface/31-lexical.md`
and `CANONICAL_WIDTH` are correct; `conformance/` is the stale side.
`SPEC-31-WIDTH-ERRATUM` reconciles it. Do not re-argue the value.

## Where durable law lives — do not restate it here

**The old file's real defect was restating durable rules inside a diary.** A
rule copied into a briefing drifts from its source and then contradicts it. ⇒
**Point, never copy.**

| what | where |
|---|---|
| federation law, §2c handoff gate, §14 merge gate | `agent/COORDINATION.md` |
| my playbook, publish discipline | `agent/playbooks/federation/steward.md` |
| hard-won operational lessons | `agent/memory/` (`fleet` + `enclave` + `roles/steward/`) |
| model tiers | `agent/MODELS.md` |
| reasoning charter | `docs/PRINCIPLES.md` |
| no local `--workspace` builds — CI only | `agent/COORDINATION.md §12` |
| build status against the DAG | `docs/program/IMPLEMENTATION-PROGRESS.md` |
| spec status | `spec/SPEC-PROGRESS.md` |

## Standing traps — only the POSITIONAL ones

Each is here because it fires **at a specific command**. That is the whole test
for belonging in this file rather than in `agent/memory/`.

- **Verify landed content by BLOB IDENTITY, never ancestry.** The publisher
  squashes, so an approved SHA is correctly *never* an ancestor of `main`.
- **Verify every object you NAME exists at the base you NAME** —
  `git cat-file -e <base>:<path>`, and quote the blob (§2c step 5b).
- **`git diff --stat` always exits 0.** Use `--quiet` for an emptiness test.
- **The publisher's exit code is the LAUNCHER's** — confirm it exited *and* that
  `main` moved.
- **Never `git fetch` while the publisher is inside its merge→verify window** —
  `refs/remotes/origin/main` is shared across ~70 worktrees.
- **Never `pkill -f`** (matches your own shell) · **never `git stash`**
  (`refs/stash` is shared) · **never `git checkout <ref> -- .`** (reverts
  uncommitted edits worktree-wide).
- **A probe truncated before its filter is not a measurement.** Search the full
  stream; truncate the RESULT.
- **Never dump `.moot/actors.json`** to learn its shape — use
  `scripts/moot-actor-id.sh <role>`; the schema-discovery step is what leaks a
  key. Look up a participant id **at post time**, never from memory.
- **`steward/work` is stale immediately after every publish** — reset onto the
  squashed `main` before writing anything new.
- **A `--doc-only` merge can redden `main` and is structurally unable to notice.**
  After one, **enumerate consumers of the touched paths** — attestation ledger,
  measured-token censuses, source-text oracles. This is how `95bc855c` broke three
  things and reported none.
