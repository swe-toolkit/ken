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

> ### RESUME HERE — state at 2026-08-13 ~16:1xZ. **`main` = `d1fb2763`.**
> **I hold nothing. No publisher. BOTH RINGS ARE WORKING — do not kick either.**
>
> **Runtime — `D2k-1b`**, the five-expression `StaticWorkerBinding` wall
> (`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md`, framed cold).
> **Gate 4b is authorized and QUEUED BEHIND IT.**
>
> **Language — `LANG-VIEW-RETIRE`.** D1 census and D2 purity-failure measurement
> done; D3 (remove `KwView`, decide the post-removal spelling) in progress.
> **If it asks what `view` becomes after removal and the spec does not settle
> it, that is the ENCLAVE's call** — not mine, not theirs.
>
> ### GATE 4b — THE OBSERVER ALREADY EXISTS. DO NOT BUILD ONE.
>
> `d2f_gate_note_arrival` at
> `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:2182-2194` already
> records the 4b population — fusion-plan `keys` and `descriptors` plus the
> transition plan's fusion-definition count — at the exact point production
> computes them. It landed under `D2f`. **4b does not need a read built; it
> needs the existing one REACHABLE.**
>
> **My premise that this was a new seam was WRONG**, and the Architect measured
> it rather than accepting it (`evt_4hpn5331ye325`, folded into the frame's
> newest box at #2099 — **read the frame, not this block**). The obstacle is
> **cross-crate gate expressibility**: `#[cfg(test)]` fires only for that
> crate's own tests, and `ken-elaborator` depends on `ken-runtime`, not the
> reverse. The real-source witness is above the boundary and the population
> below it.
>
> **The error shape, because it recurs:** I characterized a mechanism from a
> handback describing *where a seam would go*, and never checked whether one was
> already there. **A description of where to add something is not evidence that
> nothing exists.**
>
> **R3 DOES NOT CLOSE WITHOUT 4b.** State 4a at exactly its own strength — the
> compiler produces the checked IH slot/call population and the validated
> oriented plan, and they **arrive at the preparation boundary**. That is *not*
> the claim that the Runtime fusion planner forms the expected population from
> them. This arc has paid three times for a claim written wider than its
> measurement.
>
> ### THE PUBLISHER RULE I GOT WRONG TODAY — ASK THE PR, NOT THE PROCESS
>
> ```
> gh pr view N --json state,mergeCommit,statusCheckRollup
> ```
>
> **MERGED ⇒ never restart**, whatever `pgrep` says; kill any live publisher.
> **OPEN + checks running ⇒ wait.** **OPEN + checks done + `pgrep` empty ⇒
> restart.**
>
> **`pgrep` is necessary and NOT sufficient, and it errs in BOTH directions.**
> On #2098 it reported "not running" while the publisher sat in its opening
> wait, and I started a second one against one PR. It also matches **my own
> shell wrapper**, so the naive form reports "running" when nothing is.
> ⇒ **Print the matched lines, discount your own wrapper, and never collapse it
> to a boolean you cannot audit** — when the instrument is wrong, the boolean
> has destroyed the evidence that would have shown you.
>
> Also before any publish: `gh api repos/swe-toolkit/ken/commits/<sha>/check-runs`.
> **422 = never pushed = clean.** A candidate that was already red does not
> become green by being reviewed.
>
> ### LANDED 2026-08-13
>
> #2091 gate 4a + the one-cut sweep repair · #2093 two language frames · #2094
> the `language-implementer` reseat to Sonnet 5 · #2095 checkpoint · #2096 the
> C2 slot-consumption repair · #2097 the briefing flush (4648 → 356) · #2098
> record literals + the dispatch-frame repair · #2099 the gate-4b ruling fold.
> **#2088 and #2092 closed**, each with its reason in the PR.
