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


> ### RESUME HERE — state at 2026-08-14 ~11:2xZ. **`main` = `6c574cdd`.**
> **A PUBLISHER IS IN FLIGHT: PR #2196**, `RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH`
> exact `ce2a7d37`, 822s pre-poll, log `publish14.log`. **`pgrep -af '^bash
> scripts/scripted-pr-automerge.sh'` before relaunching anything** — it survives
> compaction. **Do not `git fetch` while it holds the window.**
>
> **Eleven PRs landed today, all M6 blob-verified.** Most recent: `#2189`
> `294fceac` `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA` · `#2190` `d8de7023` tracker
> sync · `#2191` `7543ddcc` ABI-order fold · `#2192` `3f04953b` coldness fold ·
> `#2193` `fea9cd96` releases · `#2194` `5edd3de3` successor frame · `#2195`
> `6c574cdd` `LANG-REFINED-FALLBACK-COLDNESS-CLAIM`. M8/M9 sent for every code
> merge.
>
> **`#2189` cost one publisher abort on a FALSE RED, and the lesson is
> positional.** The SHA was unchanged by design, so GitHub still carried the
> **pre-repair** check-runs from `06:57`/`07:08`. **A failing check on an
> unchanged SHA may be history — discriminate by `started_at`.** The publisher
> does not. A direct `gh pr merge` was correctly refused by branch protection;
> `--admin` is not the Steward's to use.
>
> ### UNPUBLISHED LOCAL WORK — publish when #2196 clears
>
> **This commit** (node flip to `merged` + tracker) and
> **`LANG-POW10-CASCADE-LITERAL-CLAUSE`** (`ready`, `XS`, untracked until it
> lands). The latter is an Architect scope hand-off from `evt_5jmye3pdj3ra7`.
> **Read its "why this is not a second COLDNESS-CLAIM" section before sizing
> it** — `decimal_char.rs:60-62` is over-broad in the same way, but its
> conclusion rests on a **different and TRUE** property (no
> `saturating_*`/`.min(_)`/`clamp` in the generated cascade). **A wording repair
> on a sound argument, not a second false justification.**
>
> ### OWED AFTER #2196 LANDS, IN ORDER
>
> 1. **M6** blob 2/2 from **declared** base `5edd3de3` —
>    `crates/ken-runtime/src/cranelift_backend.rs`,
>    `crates/ken-cli/tests/dasm_c2_observation_artifact_identity.rs`. **The
>    declared base is NOT `main` and that is by design** (`6c574cdd` landed
>    while it queued); `merge-tree` against live `main` already gave exactly
>    those two paths, both `M` — no false delete.
> 2. **M7** flip `RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH` to `merged`,
>    `gen-progress.sh`. **M8** Adversary `agt_37vnwmcdxhw00` naming the landed
>    squash. **M9** runtime-leader `agt_37reqrd72cg00`.
> 3. **Gate + compact the Runtime ring, THEN release
>    `RT-CHECKED-IH-REALIZATION-AUTHORITY`** (`ready`, `M`, no deps). **Never
>    publish inside a gate window** — the reset is a snapshot and pins seats to
>    a stale `main`.
> 4. **Language next: `LANG-GADT-SEQUENCE-TRACKER-GAP`** (`ready`, `S`). The
>    leader has the order; gate before kicking.
>
> ### M3 CITED-SOURCE HIT — routed, and the direction is the point
>
> `crates/ken-runtime/src/cranelift_backend.rs` is attested at
> `SOURCE-ATTESTATIONS:60`. **The attestation was ALREADY stale on `main`
> before this candidate** — attested `d317ad9c`, `main` was `419df20b`. The
> candidate moves it a second time; it did not originate the drift. Routed to
> the Librarian at `evt_6yhr7qe9tee82` with that distinction, because
> re-attesting against the post-merge blob would **silently absorb whatever
> earlier change went unrecorded.** Currency is generated at release points,
> not enforced per merge — nothing is gated on it.
>
> ### BOTH ADVERSARY HUNTS ARE TRIAGED AND FOLDED — do not re-read them
>
> **`evt_54sb0z31q5qhn` on `f7ec9f59` → `RT-NESTED-IH-NATIVE-REALIZATION`.**
> Accepted. Transposing the ABI order left all three suites green while the arm
> was reached twice. **Structural, not a test gap:** `D2` is terminal, so the
> population that exercises the ABI is exactly the population that never emits.
> The obligation lands on the successor, which is where the stop lifts and the
> order becomes load-bearing with nothing having ever tested it.
>
> **`evt_6r8qxhyn3hcb6` on `294fceac` → `LANG-REFINED-FALLBACK-COLDNESS-CLAIM`
> as `D5`/`D6`/`AC-6`/`AC-7`.** It **cleared** the candidate (zero frame bytes,
> measured both ways) and surfaced a term the stack arc has never stated:
> **`register_prelude`'s own frame is ~196 KiB against the ~96 KiB total
> margin.** Recorded, explicitly not acted on — the trend read was not run.
>
> ### OWED BY ME
>
> 1. **Runtime's next release** when the selfcheck candidate reaches review.
>    `RT-CALL-EDGE-EXECUTABILITY-AXIS` (`ready`, `S`) is shovel-ready; three
>    more `S` nodes behind it. **I told the leader there was no framed
>    successor — that was wrong and is corrected at `evt_3wh8wg6gpyzc2`.**
> 2. **Architect answer pending at `evt_1469rndt5745r`:** is checked-IH
>    *realization* authority the same mechanism as
>    `RT-TERMINAL-ALL-ELIM-AUTHORITY` (terminal-`All` *elimination*)? **It
>    decides whether the nested-IH arc is parked behind `KERNEL-NESTED-IND` or
>    framable now.** Nothing is blocked on it today.
>
> **Owed by others:** operator decidable-equality TCB (`evt_30gckze0jryj4`);
> Architect on `TEST-NATIVE-STACK-PROVISIONING-STANDARD`.
>
> ### NO FRAMING DEBT ANYWHERE — swept 2026-08-14, do not re-derive it
>
> Every idle team is grounded-blocked, and the blocks chain into the work in
> flight. Re-deriving this costs an hour and returns the same answer:
>
> | team | why idle | grounded in |
> |---|---|---|
> | Kernel | `KERNEL-NESTED-IND` blocked at `AC-K12` | Runtime `D2`, in flight |
> | Foundation | `DS-9` `depends_on: [KERNEL-NESTED-IND]` | same chain, transitively |
> | Verify | `SEC1-IFC-R3` needs an SMT/Z3 backend absent from the workspace | operator V3 fork |
> | Doc | program complete — 29 merged, 4 closed, zero open | nothing to frame |
> | Ergo | all three nodes merged or closed | nothing to frame |
>
> **`SEC1-IFC-R3` says in terms: do not frame a Verify slice against it.** Its
> `AC-R3c` is blocked on Spec closure plus Architect-owned kernel-facing work,
> and the smallest releasable property is not Verify-shaped.
>
> ### OWED BY OTHERS — waiting, acting on neither
>
> **Operator: the decidable-equality TCB call**, re-asked with corrected scope
> at `evt_30gckze0jryj4`. Each new primitive registrant admits **two** trusted-
> base entries (`declare_deceq_certificate` calls `declare_postulate` twice,
> `check.rs:1302`/`:1308`). It gates Verify's whole backlog and Language's only
> two drafts (`LANG-DECEQ-CHAR-LAWFUL-INSTANCES`,
> `LANG-FOREIGN-NAME-FORMAT-CHARS`, both `gate: operator`). Nothing is stalled
> on it today.
>
> **Architect: `TEST-NATIVE-STACK-PROVISIONING-STANDARD`** (`gate: architect`,
> now on `main`). Disposition 3 recommended, carrying the Architect's own two
> amendments: gate provisioning on **evidence not motive** (no open measured
> regression on that test), and **define "derived"** as measured peak times a
> stated headroom factor, both numbers written down.
>
> ### PUBLISH DISCIPLINE — it cost two failed gates on 2026-08-14
>
> **After EVERY publish lands: `git reset --hard origin/main`.** Commits are
> **squash**-merged, so a branch carrying an already-landed commit does not
> merge cleanly, and a two-way diff against `main` shows files landed since the
> base as **DELETIONS** — this nearly reverted PR #2179. Rebuild by:
> `reset --hard origin/main`, then `git checkout <old-tip> -- <explicit doc
> paths>`, then **grep the status for `crates/` before committing.** The
> publisher takes `--target <SHA>`, never `HEAD`.
>
> ### LANDED 2026-08-14, EARLIER — the later six are in the live block above
>
> **#2179** `RT-C2-OBSERVATION-ARTIFACT-IDENTITY` (squash `79fddb0d`, 2/2
> blob-verified, `crates/`, Adversary notified) · **#2181** doc batch (squash
> `aa7fa99a`, 9/9) · **#2183** Adversary triage fold (squash `f34ab271`, 1/1).
> **#2178 closed** (withdrawn approval) · **#2180, #2182 closed** (superseded
> by base drift).
