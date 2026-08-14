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


> ### RESUME HERE — state at 2026-08-14 ~10:0xZ. **`main` = `294fceac`.**
> **BOTH BUILD TEAMS ARE IDLE and both have a `ready` successor. Publish queue
> empty, no publisher running. §1 says the only work is release the next WP.**
>
> **Six PRs landed today, all M6 blob-verified:** `#2184` `cae1c36f` briefing
> flush · `#2185` `842f2d5f` ruling fold · `#2186` `51b3a75c`
> `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` · `#2187` `f7ec9f59`
> `RT-NESTED-IH-NATIVE-REALIZATION` `D2` (node **stays `active`**, `D3`-`D5`
> remain) · `#2188` `c035c8a4` doc bundle · `#2189` `294fceac`
> `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA`. M8/M9 sent for every code merge.
>
> **THE FROZEN CANDIDATE IS LANDED — that whole block is retired.** `98e6ac51`
> re-voted unchanged and merged as `294fceac`; `wp/…-BLOCK-DELTA` is deleted at
> origin. Do not go looking for it.
>
> **`#2189` cost one publisher abort on a FALSE RED, and the lesson is
> positional.** The SHA was unchanged by design, so GitHub still carried the
> **pre-repair** check-runs from `06:57`/`07:08`. **A failing check on an
> unchanged SHA may be history — discriminate by `started_at`.** The publisher
> does not. A direct `gh pr merge` was correctly refused by branch protection;
> `--admin` is not the Steward's to use. Relaunching the sanctioned publisher
> after the condition changed was what worked.
>
> ### OWED BY ME, IN ORDER — both are §1 releases
>
> 1. **Release `RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH`** (`ready`, `S`) to
>    Runtime. The leader asked at `evt_250nev2zanqef` and reported the ring
>    idle and wanting compaction. `D3` (make the **disabled** arm probe) may
>    close the exposure more directly than `D1`'s constant — the node says to
>    price them separately rather than landing both.
> 2. **Triage Adversary hunt `evt_54sb0z31q5qhn`** on `f7ec9f59`, still unread
>    in full. Headline: it **transposed the ABI order** — the exact silent
>    failure mode I asked it to hunt — and **nothing in the tree reds,
>    structurally**. Record the disposition **in the node**, never as a reply
>    (`COORDINATION §10⁻a` is report-only).
> 3. **Language's next**, leader's call on order: `LANG-GADT-SEQUENCE-TRACKER-GAP`
>    (`ready`, `S`) or `LANG-REFINED-FALLBACK-COLDNESS-CLAIM` (`ready`, `S`).
> 4. **Ask the Architect** whether the `D2` successor (checked-IH
>    *realization* authority) is the same mechanism as
>    `RT-TERMINAL-ALL-ELIM-AUTHORITY` (terminal-`All` five-fact *elimination*
>    relation, blocked on `KERNEL-NESTED-IND`). **A mechanism question, not a
>    cut** — do not presume the fold.
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
