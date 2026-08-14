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


> ### RESUME HERE — state at 2026-08-14 ~12:3xZ. **`main` = `8f709da0`.**
> **BOTH BUILD TEAMS HAVE APPROVED CANDIDATES AND THE PUBLISH QUEUE HAS TWO IN
> IT. Neither team is owed anything; both are mine to land.**
>
> ### THE TWO QUEUED CANDIDATES — this is the live work
>
> **1. Runtime `#2202`, exact `afc971158082f4d262a1915a2380b872526c3c04`,
> `dec_7kjw87yqbam7a` resolved.** M1/M2/M3 all done: one commit, one path
> (`.../lowering/core/tests/control.rs`), `+92/-51`, nothing outside `tests/`,
> no cited-source hits, base-drift intersection empty.
>
> **It reddened on a FLAKE and is on a re-trigger, not a first run.**
> `library_documentation_gates.rs` synthetic-git fixture died at
> `git commit -m "filler 16"` → `bad tree object HEAD`; shards 1/2/4 green,
> `main` green. **Re-triggered on the SAME SHA via `gh pr close` + `gh pr
> reopen`** (run `31798706708`) — `gh run rerun --failed` is refused to the
> publisher identity. Procedure now in `merge-procedure.md` **M5a**.
>
> ⇒ **When that run is green, relaunch:**
> `scripts/scripted-pr-automerge.sh --target afc971158082f4d262a1915a2380b872526c3c04 ...`
> It finds `#2202` rather than opening a second PR. **Check `gh pr checks 2202`
> shows the FRESH result first** — the SHA carries the old reds too.
>
> **2. Language, exact `8b58010f040cfa6820339e16f9a2f243246aad53`,
> `dec_4j8c4pcq1xs64` resolved APPROVED.** Queued second. **M2 already done and
> matches on all four counts** against declared base `8f709da0`: one non-merge
> commit, three `crates/ken-elaborator` paths, `+44/-4`. **`AC-4` verified
> structurally** — non-comment added/removed lines in `elab.rs` filter to
> **empty**, so the landed `-3120` extraction is untouched rather than merely
> un-reverted.
>
> ### TWO STEWARD COMMITS ARE HELD, DELIBERATELY
>
> `b845d694` (idle-ring measurements) and `2e26569c` (`merge-procedure` M5a) sit
> unpublished on `steward/work`. **`COORDINATION §10⁻` rule 1: no process merge
> while a ring holds finished, unmerged work.** Land them after both candidates
> above. **Rebase, do not reset** — a `git reset --hard origin/main` destroys
> them.
>
> **Seventeen PRs landed today, all M6 blob-verified.** Most recent: `#2195`
> `6c574cdd` `LANG-REFINED-FALLBACK-COLDNESS-CLAIM` · `#2196` `1200edf0`
> `RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH` · `#2197` `998c3c83` closeouts +
> the operator ruling · `#2198` `54f7c84a` the Runtime release · `#2199`
> `99869bb7` the briefing rewrite. **M8/M9 sent for every code merge.**
>
> **`#2189` cost one publisher abort on a FALSE RED, and the lesson is
> positional.** The SHA was unchanged by design, so GitHub still carried the
> **pre-repair** check-runs. **A failing check on an unchanged SHA may be
> history — discriminate by `started_at`.** The publisher does not. A direct
> `gh pr merge` was correctly refused by branch protection; **`--admin` is not
> the Steward's to use.**
>
> ### LANGUAGE IS WORKING TWO NODES AS ONE CANDIDATE — do not re-kick
>
> `LANG-STACK-ARC-EVIDENCE-USABILITY` (`S`) **carrying**
> `LANG-POW10-CASCADE-LITERAL-CLAUSE` (`XS`). Both touch `crates/ken-elaborator`
> and land as **one** candidate; `POW10`'s own Sizing section says it should
> ride rather than spend a ring turn. **They are independent repairs sharing a
> build, not one deliverable** — if either stalls, land the other.
>
> **I overrode the leader's stated next, deliberately and on the record.** The
> leader named `LANG-GADT-SEQUENCE-TRACKER-GAP`; its `AC-4` puts `crates/` out
> of scope, so it produces **no product change**, and an idle ring gets the
> product node first. That node is now sequenced second **on sequencing, not
> merit** — its own text says so, and its stale "Language has no other `ready`
> node" premise is corrected in place.
>
> **Read `LANG-POW10`'s "why this is not a second COLDNESS-CLAIM" section
> before sizing it.** `decimal_char.rs:60-62` is over-broad in the same way, but
> its conclusion rests on a **different and TRUE** property (no
> `saturating_*`/`.min(_)`/`clamp` in the generated cascade). **A wording repair
> on a sound argument, not a second false justification.**
>
> ### OPERATOR PRIORITY RULING 2026-08-14 — RUNTIME'S LANE IS RE-RANKED
>
> **Verbatim:** *"that is the priority for the runtime team. prioritize that
> work over other runtime work."* Issued on my measured answer that **nothing in
> the preceding twelve hours advanced the `RecursiveDescent` retirement** — 49
> commits on `main`, none touching `RecursiveDescent` in `crates/`.
>
> ⇒ **Runtime's next release is `RT-LEXICAL-RECURSOR-CONSUMERS`** (`ready`,
> `M`), **not `RT-CHECKED-IH-REALIZATION-AUTHORITY`**, which stays `ready` and
> held. Both nodes carry the ruling in their own text.
>
> **Why that node:** it is the **single unblocked node** on the whole retirement
> chain. Its two deps are merged; it blocks `RT-RECURSOR-TRANSPORT` (`draft`,
> other four deps merged), which blocks `RT-DESCENT-RETIRE` (`draft`, other four
> deps merged). **Everything else on both paths is done.**
>
> **The remaining work is `D2k`** — five expressions at the `StaticWorkerBinding`
> wall plus row 3's singular-specialization wall. Architect ruled
> (`evt_5wvk3e8k1bjqn`) it is `#6d`'s next `D2` increment **inside this node, no
> new node.** Runtime's own estimate to closure was *"closer to a week"*.
>
> **My contrary sequencing call was surfaced and overruled** — I had ranked it
> second on "six seats idle beats one lane's depth." **A priority call between
> `ready` WPs is the operator's under §3.** Do not re-derive it.
>
> ### RUNTIME IS WORKING IT — anchor `evt_gzh1p738kfa1`
>
> Ring gated and compacted at `998c3c83`, all three seats verified
> individually; implementer confirmed pickup. **Do not re-kick.** First
> increment `D2k-1a` handed back and approved within the hour
> (`dec_7kjw87yqbam7a`, exact `afc97115`, one `control.rs` test path, `+92/-51`)
> — accepted partials are landing as designed.
>
> ### KERNEL AND VERIFY ARE BOTH IDLE, AND NEITHER IS FRAMING DEBT
>
> **Checked why rather than assuming a thin frame.** Both are blocked on inputs
> that are the operator's, and both are already routed there.
>
> - **Kernel** — `KERNEL-NESTED-IND` (`active`, `L`) is blocked at `AC-K12` on
>   `RT-NESTED-IH-NATIVE-REALIZATION` (`active`, `L`, **Runtime**-owned). Runtime
>   is not working it, because the operator's RecursiveDescent ruling put
>   `RT-LEXICAL-RECURSOR-CONSUMERS` ahead of it. **Kernel has no other node at
>   all.** ⇒ **The ruling idles the Kernel ring** — a consequence worth the
>   operator knowing, not a call to re-derive.
> - **Verify** — its only non-merged node is `SEC1-IFC-R3` (`draft`), and its own
>   text says **do not frame a Verify slice against it**. `AC-R3c` is blocked on
>   Spec closure plus Architect-owned kernel-facing theorems **and the operator's
>   V3 fork**. `V3-KRIPKE-DECOMPOSITION` priced the embedding and returned
>   *"presently unsizeable"* rather than a number. Hard stop already routed to
>   the operator by `verify-leader`.
>
> **`verify-implementer` and `verify-qa` status lines still read
> "awaiting review" on `ce11f99e`. That is STALE** — `V3-KRIPKE-DECOMPOSITION`
> is `merged`. A seat's status line is not evidence about the tree.
>
> **The work is `D2k`, INSIDE this node** — Architect `evt_5wvk3e8k1bjqn`,
> **no new node.** Six expressions at two walls: rows 1/4 plus row 5's
> after-hole at the `StaticWorkerBinding` wall, row 3 at the singular-
> specialization wall. Runtime's own estimate to `#6d` closure was *"closer to
> a week"* ⇒ **expect increments and accepted partials, not one candidate.**
>
> **I made `D2k-0`'s redness claim a thing to TEST, not a premise.** The node
> records it as unverified by anyone. If it does not red as stated, **that is
> a finding worth more than the increment.**
>
> **Still ruled and relitigated often:** `10369776` is held evidence only, not
> a candidate or repair base. **Zero new `#[ignore]`** — my earlier quarantine
> ruling at `evt_7vhjcstd37a50` is withdrawn and was not revived. The old-green
> semantic controls are not disposable.
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
