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

> **Re-arm TWO things on every resume**, both process-local and both silently
> dead after an MCP restart or compaction:
> 1. **the watchdog tick** (`set_interval`);
> 2. **the daily briefing flush** (`CronCreate`, off-`:00` minute) — it is
>    session-only and auto-expires after 7 days. See
>    `agent/playbooks/federation/steward/briefing-flush.md`.
>
> Builds allowed, targeted only, never `--workspace`.

> ### RESUME HERE — state at 2026-08-13 ~18:3xZ. **`main` = after PR #2114.**
> **BOTH RINGS WORKING — do not kick either. Publish queue is EMPTY.**
>
> **Runtime — `RT-4B-WALKED-CONSTANCY`**, anchor `evt_7s6enk2syyct1`
> (re-kick; the first kick `evt_3haj2jas0n876` fired before the frame was on
> `main` and Runtime correctly refused it). Comment-only, XS, **four sites**.
> **Amended twice — D3/AC-6 and D4/AC-7 — and the frame now says no further
> amendment is coming. Hold to that.**
> **Language — `LANG-SURFACE-LITERAL-ESCAPES`**, anchor `evt_32763was18rw6`.
> The leader had been stalled awaiting clarification on the duplicate
> `LANG-RECORD-STACK-OVERFLOW` kickoff; cleared. **Next ready is
> `LANG-SURFACE-BLOCK-COMMENTS` and the leader takes it without returning to
> the Steward.**
>
> ### 4b IS UNBLOCKED AND ITS NEXT INCREMENT IS FRAMED
>
> **`RT-4B-C2-REACHABILITY` is `closed`, answered with the second licensing
> row: reachable, BUT THE REACHING BUILD DIFFERS FROM PRODUCTION.** The D2f
> record, storage, note/take and sole write are all `#[cfg(test)]`, and C2 is a
> `ken-elaborator` integration test whose Runtime dependency carries default
> features only. The answer is recorded in the node, not only in the thread.
>
> **The Architect ruled (`evt_4a1pf1jfmdemd`): a default-off feature is INSIDE
> the 4b envelope** — one of the two mechanisms his original 4b ruling named.
> **4b is not blocked on the envelope, not awaiting a count, and not blocked on
> cross-crate gate expressibility.**
>
> **`RT-4B-OBSERVATION-FEATURE-GATE` (`ready`, M) is Runtime's next node.** Its
> five review conditions are ACs. **The one that bites: identity must be proven
> across TWO COMPILATIONS, not a runtime toggle inside one.** A feature is a
> compile-time property; the landed switch proves the *recording* inert, not the
> *feature*. Cargo unifies features across a build graph, so the two cannot
> coexist in one compilation — and the two builds must not share a
> `CARGO_TARGET_DIR` or the comparison reads one artifact against itself.
>
> `RT-4B-UNIQUENESS-GATE-REACH` stays `draft` until that identity proof lands,
> then re-points at C2. `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` stays `draft`
> behind it. **Do not cut either.**
>
> ### OPEN TO THE OPERATOR — VERIFY IS IDLE AND FRAMING CANNOT FIX IT
>
> **`SEC1-IFC-R3` is Verify's only open node**, and it needs an SMT/Z3 backend
> that **is not a dependency of this workspace at all**. `AC-R3b`/`AC-R3c`
> cannot be discharged without building the prover backend, which the node's own
> scope bans — **the frame is unsatisfiable as written.** The Z3-free widening
> is recorded vacuous: `declare_deceq_certificate` has exactly one caller,
> registering `Int`, so there is no second type to generalize to.
>
> **Escalated 2026-07-27, unanswered as of 2026-08-13.** Re-raise it; do not
> re-derive it as framing debt.

> ### R3 GATE 4b — WHY C2 HAS NEVER BEEN MEASURED. READ THIS BEFORE ANY 4b WORK.
>
> **The reachability half of this block is now ANSWERED — see the block above.
> What follows is why the measurement was never taken, and it is still live.**
>
> **A previous version of this block said the observation route was exhausted
> and that "there was nothing to fuse" was dead. BOTH WERE WRONG and are
> withdrawn** (Architect `evt_6hfw027f43cgg`, verified by the Steward against
> the diff).
>
> `(4, 2, 0, 2, 1)` is asserted on `arrived_empty` in
> `d2f_0_the_applied_root_production_path_gate`, which iterates exactly
> `[D2jCause::ExactSuffix, D2jCause::CallIdentity]` — **deliberate perturbations
> authored so fusion does NOT form**, under the test's own comment
> `// AC-6a phase B: arrived once, resolved nothing`.
>
> ⇒ **Four candidates entered and nothing survived BECAUSE THE FIXTURE WAS
> PERTURBED TO MAKE NOTHING SURVIVE.** It is a negative control reporting its
> designed outcome.
>
> **The same assertion shows the three unperturbed rows each resolve exactly one
> key and one descriptor. THIS PLANNER FUSES.** The artifact-identity control
> drives `d2j_checked_fixture_under(D2jCause::Exact)` — **not `C2_MIXED_SOURCE`.**
>
> **`C2`'s walked count is UNMEASURED.** `RT-4B-UNIQUENESS-GATE-REACH` and
> `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` are **both `draft`; do not cut either.**
> Pointed as written, reach would count the same comparators.
>
> **And a `walked` count would not have settled it anyway.** Measured on the
> two landed assertions: `(walked, frames, recursive, slots, calls)` is
> `(4, 2, 0, 2, 1)` for `ExactSuffix` and `CallIdentity` (0 keys) **and for
> `Exact` (1 key)** — the perturbation moves the outcome and leaves all five
> numbers unchanged. `walked` is the enumerator's INPUT size, so **no claim
> about candidate formation can be supported by it, on any cause.**
> `RT-4B-WALKED-CONSTANCY` is landing that sentence into the observation's own
> doc.
>
> **THE ACTUAL BLOCKER, unchanged since 4b opened: cross-crate gate
> expressibility.** The observation is `#[cfg(test)]` inside `ken-runtime`; C2
> drives through `ken-elaborator`, which links a build where those calls do not
> exist. **The reach frame asked for a C2 run its own fixed inputs made
> impossible.** Re-pointing at C2 is a scope question, not an edit.
>
> **THE FRAME DEFECT WORTH CARRYING:** the C2 requirement lived in D2 prose and
> **was never carried into an AC**. Five ACs constrained everything except
> *which witness*, so an in-crate implementation satisfied all of them while
> answering a different question. **A deliverable stated in prose and not in an
> AC is one the frame cannot check.** QA hedged correctly — *"on these rows"* —
> and the Steward dropped the qualifier when restating it. **The hedge was the
> finding.**
>
> ### TWO ERRORS TODAY, BOTH THE SAME SHAPE: A CLAIM WIDER THAN ITS INSTRUMENT
>
> **1. I kicked Language onto already-merged work.** `LANG-RECORD-STACK-OVERFLOW`
> merged as #2098 at `b4d38b8a` and still read `status: ready`. ⇒ **A node's
> `status:` is a claim ABOUT a node; only the tree is evidence about the code.**
> Every claim the kick derived from that frame went stale with it. **Corpus-wide
> sweep done and clean — do not redo it.**
>
> **2. `RT-CONTSRC-FRAME-FINALIZE` is CLOSED, premise refuted by one probe.**
> Stage 2 already runs and is correctly sequenced
> (`finalize_continuation_availability_plan:1292`, called `:12174`); the five
> governed rows carry **zero** projections and **zero** requirements. The
> Architect withdrew his successor on the same measurement. **`D2k-1b` is
> UNPARKED and was never blocked on it** — its real walls are `NativeJoinPlanV1`
> / `StaticWorkerBinding`. The picture had been built from three code sites and
> their doc comments, one of which is TRUE and simply never applied to those
> rows.
>
> ### PUBLISHER RULES
>
> `gh pr view N --json state,mergeCommit,statusCheckRollup`. **MERGED ⇒ never
> restart.** `pgrep` errs BOTH ways — print the lines, never a boolean.
> Pre-publish CI history: `gh api .../commits/<sha>/check-runs`, **422 = never
> pushed = clean.** `gh` is not authed in a plain shell — mint via
> `.devcontainer/mint-gh-token.sh` with `dangerouslyDisableSandbox`.
>
> **MERGE NOTIFICATIONS name the LANDED SQUASH or `merge-base...tip`, never a
> bare tip SHA** (Adversary; now playbook M8). `git show <tip>` showed 2 files
> when the change was 10, and git returns no error.
>
> ### LANDED 2026-08-13
>
> #2091 gate 4a · #2093 two language frames · #2094 the Sonnet 5 reseat · #2095
> checkpoint · #2096 the C2 slot-consumption repair · #2097 the briefing flush ·
> #2098 record literals + dispatch-frame repair · #2099 the gate-4b ruling fold ·
> **#2103 `LANG-VIEW-RETIRE`** · **#2104 the KwView spec erratum** · **#2105 the
> RT-CONTSRC closure + 4b size frame**. **#2088 and #2092 closed.**
