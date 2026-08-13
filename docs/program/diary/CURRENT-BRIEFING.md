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
>
> ### THE THREE FILES, so you do not go looking in the wrong one
>
> | you want | read |
> |---|---|
> | the current window — **only** the live block lives here | this file |
> | permanent, undated material: operator rulings, preserved refs, standing traps | [`STANDING.md`](STANDING.md) |
> | what happened on day X | `2026/Mon/DD.md`, indexed by [`INDEX.md`](INDEX.md) |
>
> **This file holds ONE block: the current one. Under 250 lines.** A superseded
> block moves to the dated diary **even if it is an hour old** — "recent" is not
> the test, "current" is. Flushed daily by a delegated subagent; procedure in
> `agent/playbooks/federation/steward/briefing-flush.md`.
>
> ⚠ **It reached 4648 lines / 273 KB across 19 unflushed days before anyone
> noticed** — having already been rewritten to be small once, in July. Nothing
> reds when it grows. If you are adding a block and the file is over budget,
> flush first.

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
