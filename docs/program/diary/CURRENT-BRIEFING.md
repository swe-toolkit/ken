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


> ### RESUME HERE — state at 2026-08-14 ~14:3xZ. **`main` = `a998d3f6`.**
> **BOTH BUILD TEAMS ARE WORKING.** Recent: `#2206` `3ea9bef4` · `#2207`
> `018abf96` · `#2208` `10101777` · **`#2209` `a998d3f6` Runtime
> `RT-CONTKEY-CONSUMING-OCCURRENCE`, M6 tree `1ac31455` verified.**
>
> ### I AM HOLDING THREE UNPUBLISHED COMMITS ON PURPOSE. PUBLISH ORDER MATTERS.
>
> **`b14b2c1e` + `5a077459` (+ this briefing) are held until Language's
> `4b8f6777` lands.** Both regenerate `docs/program/IMPLEMENTATION-PROGRESS.md`,
> which is the one path Language's candidate also writes. **Publishing first
> forces a third rebase on them.** The moment `4b8f6777` merges: rebase onto the
> new `main`, re-run `gen-progress.sh`, publish. **Do not publish them before.**
>
> ### THE COLLISION CLASS, because it will recur and "rebase faster" cannot win
>
> `IMPLEMENTATION-PROGRESS.md` is generated from `docs/program/issues/`, and
> **every** commit that adds or flips a node rewrites it — the `Last generated`
> line and file count change on every regeneration. CI runs
> `gen-progress.sh --check`, so a candidate **cannot drop it**. ⇒ Two docs
> candidates in flight **always** conflict on exactly that path and nothing else.
> It fired twice in forty minutes.
>
> **A code candidate is NOT the collider — my `M7` is.** Runtime's `16eb2618`
> was `crates/`-only and merged with no effect on Language. Order the merges, do
> not hold them.
>
> **Registered fix (Architect `evt_2kf7xke2q2nvc`), my lane to choose:** a custom
> merge driver on that path re-running `gen-progress.sh` — correct by
> construction. **`merge=union` DOES NOT WORK** and is the obvious wrong reach:
> it doubles the timestamp/count and reds `--check`. Third option: stop
> committing the file, generate in CI — loses in-tree readability. **Not filed
> as a node yet. That is the one piece of framing debt I know about.**
>
> ### RUNTIME: `D2k-1c` RELEASED — anchor `evt_296nwf42qd2ft`, thread `thr_5ngmcb9tnhym`
>
> Ring gated and compact-verified at `a998d3f6`; leader confirmed `Working`.
> **`RT-CONTKEY-CONSUMING-OCCURRENCE` merged**, so the absent relation that
> stopped all five expressions now exists: `ContinuationSpecializationKey`
> carries `consuming_occurrence` beside an unchanged `consumer_owner`, naming the
> exact outer selected case body **and** its eliminator. Landed control: row 4
> body `16`, row 5 body `12`, both eliminator `5`.
>
> **THE GRADING INVERTED AND THE KICK SAYS SO.** `RT-CONTKEY`'s `AC-6` required
> rows 4/5 to **still refuse**. `D2k-1c` requires them to **consume**. A carried
> habit from the last turn fails this one.
>
> **Row 1's status genuinely changed and I asked for a measurement, not a fix.**
> It was blocked by this relation *and* by an earlier `NativeJoinPlanV1` refusal.
> The relation now exists ⇒ if row 1 still blocks, `NativeJoinPlanV1` is the
> **sole** remaining blocker, which is a different shape from what was reported.
> Report only; not a deliverable.
>
> ### LANGUAGE: IDLE, BLOCKED ON ONE `resolve_decision`
>
> `LANG-GADT-SEQUENCE-TRACKER-GAP` tip **`4b8f6777`**, twice-rebased, content
> approved. `dec_28s60t6n2w5y5` still reads `resolved_at 13:15:24` cast on
> `07da235f` — **it names a SHA that will not land.** The Architect refused to let
> me publish a tip no resolution names (`evt_2kf7xke2q2nvc`) and gave four
> standing carry conditions instead. **I verified 1, 2 and 4 myself against
> `a998d3f6`; condition 3 (`gen-progress --check`) I did NOT reproduce and said
> so — CI is the gate for it.** Nudged at `evt_gshxxc9wkyrh`.
>
> **The leader claimed the old resolution carries to byte-identical content. It
> does not, and I read the object rather than the claim.**
>
> `language-leader`'s seat is **still not compacted** — `/compact` refused twice.
> Retry at the next WP boundary.
>
> ### TEST-NATIVE-STACK-PROVISIONING-STANDARD — RULED, RECUT, HELD IN `b14b2c1e`
>
> Architect `evt_4rz7hp11f33wj`. Disposition 3, **re-grounded on STATEDNESS** —
> the property is that a stack is *stated*, not that it is large. Three acts,
> only "masking a regression" forbidden, and its test is objective (no open
> measured regression), so the standard **cannot unblock a candidate**.
>
> **The census refuted the node's own decisive evidence:** 15 sites / 14 files /
> 4 crates / 3 constants, against an asserted 6 / 5 / 1. **The Architect withdrew
> his own Amendment 2** — its arithmetic had **one sign**, so it would have marked
> the repo's best stack site non-compliant. Enumerate the acts, not the sites.
>
> **Venue decided (§3): `agent/playbooks/tools/stated-stacks.md`**, skill-linked,
> one line each from `build/implementer.md` and `build/qa.md`. `pin-a-property`
> is the precedent. `COORDINATION §12` rejected — it owns laptop resource
> discipline, so filing it there is a category error. Owner moved to **`doc`**.
> Site pass split to `TEST-STATED-STACK-SITE-RECONCILE`, **`draft` deliberately**
> (its dep has not landed; `--strict` is right to fail that).
>
> ### KERNEL AND VERIFY ARE IDLE, NEITHER IS FRAMING DEBT
>
> **Kernel** — `KERNEL-NESTED-IND` blocked at `AC-K12` on
> `RT-NESTED-IH-NATIVE-REALIZATION`, parked behind the operator's RecursiveDescent
> ruling. Kernel has no other node; **the ruling idles that ring** — a consequence
> for the operator, not to re-derive. **Verify** — `SEC1-IFC-R3` forbids framing a
> Verify slice against it; blocked on Spec closure, Architect kernel-facing
> theorems, and the operator's V3 fork. Both hard stops already routed.
>
> ### THE OPERATOR PRIORITY RULING STILL GOVERNS RUNTIME
>
> *"that is the priority for the runtime team. prioritize that work over other
> runtime work."* Chain: `D2k-1c` → rest of `RT-LEXICAL-RECURSOR-CONSUMERS` →
> `RT-RECURSOR-TRANSPORT` (`draft`, every other dep merged) → `RT-DESCENT-RETIRE`.
> `RT-CHECKED-IH-REALIZATION-AUTHORITY` stays `ready` and held.
> **My contrary sequencing call was surfaced and overruled — do not re-derive it.**
>
> **Runtime fill-in if `D2k-1c` stops early:** `RT-C2-DRIVER-STAGE-ATTRIBUTION`
> (`ready`, `XS`) — as fill-in, **not** a passenger on the candidate.
>
> **Still ruled:** `10369776` is held evidence only. **Zero new `#[ignore]`.**
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
> | Verify | `SEC1-IFC-R3` — read the node, not this cell; the reason was re-derived 2026-08-14 | operator V3 fork |
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
