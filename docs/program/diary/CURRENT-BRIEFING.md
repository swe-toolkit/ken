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

> ### RESUME HERE — state at 2026-08-13 ~17:0xZ. **`main` = `89050686`.**
> **I hold nothing except a publisher on the spec erratum. BOTH RINGS ARE WORKING — do not kick either.**
>
> **Runtime — `RT-CONTSRC-FRAME-FINALIZE`**, kicked at anchor `evt_5mazfrtngffvg`.
> Finish the two-stage continuation availability lifecycle so Stage 2 runs for
> the five governed rows. **`D2k-1b` stays parked until it lands.**
>
> **Language — `LANG-SURFACE-LITERAL-ESCAPES`**, kicked at anchor
> `evt_1geg939by8h4r`. The lexer does no escape processing at all; I verified
> that against `origin/main` before sending it.
>
> ### THE ERROR I MADE TODAY, AND IT IS THE ONE THIS FILE EXISTS TO PREVENT
>
> **I kicked Language onto `LANG-RECORD-STACK-OVERFLOW`, which had already
> merged as PR #2098 at `b4d38b8a`.** The node still said `status: ready`
> because nobody flipped it when the PR landed. I read the status as the state
> of the tree.
>
> ⇒ **A node's `status:` is a claim ABOUT a node. Only the tree is evidence
> about the code.** Before any kick: check that the work has not landed —
> `gh pr list --search "<NODE-ID>" --state merged`, or grep the mechanism on
> `origin/main`. The language leader caught this and blocked on a node-identity
> question rather than assigning it, which cost one round trip instead of an
> implementer turn.
>
> **It compounded:** my kick also warned about a `=>`-vs-`|->` fixture trap that
> the same PR had already repaired. **A stale node status makes every derived
> claim in the kick stale too.**
>
> I flipped `LANG-RECORD-STACK-OVERFLOW`, `LANG-SURFACE-RECORD-LITERAL` and
> `LANG-VIEW-RETIRE` to `merged`, and reverted a `depends_on` edge I had added
> from the same wrong premise.
>
> ### GATE 4b IS ANSWERED AS FAR AS OBSERVATION CAN ANSWER IT
>
> Runtime handed 4b back with a named stop, no candidate. **Three of its four
> measured values did not mean what the handback read them as:**
> `fusion_definitions = 0` is the **pinned expected value** ("zero until the
> emitter exists"); `oriented_present` is a **boolean over an `Option`**, so it
> says a plan arrived, not that a population did; and `keys = []` is exactly
> `candidates.len() == 0` because the interning loop **has no decline path**
> (Architect, `static_transition.rs:10030-10053`).
>
> **I measured all thirteen elimination exits in
> `enumerate_live_fusion_candidates` (`:10242-10365`): NONE is distinguished** —
> every one is a bare `continue` or bare early return. By the Architect's own
> criterion that puts the per-gate census **out** of the observation gate.
>
> **`RT-4B-ENUMERATION-INPUT-SIZE` is framed and `ready`** behind
> `RT-CONTSRC-FRAME-FINALIZE`. It records the **admitted-discovery ledger's**
> length — not the oriented plan's, because the ledger is what enumeration
> actually iterates — and its frame **pre-states what each outcome licenses**,
> so no one needs a round trip to find out. Non-empty licenses **nothing about
> the planner**.
>
> ### THE PUBLISHER RULE — ASK THE PR, NOT THE PROCESS
>
> ```
> gh pr view N --json state,mergeCommit,statusCheckRollup
> ```
>
> **MERGED ⇒ never restart**, whatever `pgrep` says. **OPEN + checks running ⇒
> wait.** **OPEN + checks done + `pgrep` empty ⇒ restart.**
>
> **`pgrep` errs in BOTH directions** — it reported "not running" while a
> publisher sat in its opening wait, and it also matches my own shell wrapper.
> Print the matched lines; never collapse it to a boolean.
>
> Before any publish: `gh api repos/swe-toolkit/ken/commits/<sha>/check-runs`.
> **422 = never pushed = clean.** `gh` is not authenticated in a plain shell —
> mint a token with `.devcontainer/mint-gh-token.sh` (needs
> `dangerouslyDisableSandbox`).
>
> ### LANDED 2026-08-13
>
> #2091 gate 4a + the one-cut sweep repair · #2093 two language frames · #2094
> the `language-implementer` reseat to Sonnet 5 · #2095 checkpoint · #2096 the
> C2 slot-consumption repair · #2097 the briefing flush (4648 to 356) · #2098
> record literals + the dispatch-frame repair · #2099 the gate-4b ruling fold ·
> **#2103 `LANG-VIEW-RETIRE`** (landed tree `5f5469c8`). Spec erratum
> `93e92812` publishing. **#2088 and #2092 closed**, each with its reason in
> the PR.
