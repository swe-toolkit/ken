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


> ### RESUME HERE — state at 2026-08-14 ~13:4xZ. **`main` = `018abf96`.**
> **BOTH BUILD TEAMS ARE WORKING. Publish queue empty, no publisher, tree
> clean.** Recent: `#2205` `0644ab95` briefing · `#2206` `3ea9bef4` the new
> Runtime node · `#2207` `018abf96` its `active` flip.
>
> ### RUNTIME: `D2k-1c`'s SIZING PROBE ANSWERED, AND THE SUCCESSOR IS A NODE
>
> **The probe came back NOT IN HAND** (`evt_3tkyp322dh4c7`, measured at
> `0644ab95`, three planner invocations per row). The outer scan holds `m=5`
> whose `children[0]` equals the key's `continuation_origin`; at interning what
> is in scope is the current **producer** occurrence with a different
> `children[0]`. Row 4 `21`/`31`, row 5 `17`/`27`. ⇒ The fact must be seeded at
> the outer-match walk and threaded to interning — **a plan-construction
> change**, which is the branch the Architect said deserves its own node.
>
> **[[RT-CONTKEY-CONSUMING-OCCURRENCE]] filed, released, `active`.** Anchor
> **`evt_39a7p1yhtb4je`**. Ring gated and compact-verified at `3ea9bef4`,
> `runtime-leader` confirmed `Working`. **The node is its own frame** — no
> `docs/program/wp/` file and none owed.
>
> **It supplies a RELATION and does NOT repair a route.** `AC-6` is the line
> that keeps that honest: rows 4 and 5 **must still refuse** at the end of it.
> `D2k-1c` keeps the route repair and stays framed in section 5 of the `D2k`
> frame; `RT-LEXICAL-RECURSOR-CONSUMERS` now carries the new node in its
> `depends_on` and shows as blocked in the tracker.
>
> **Four fixed inputs, from `evt_3zjhbbr7k3ky6`, written into the node as
> `F1`-`F4` — do not re-derive:** widening `consumer_owner` is closed
> structurally (`exact_continuation_source_environment` fails closed on the
> equality, so the all-`Fn(0)` match is an **enforced derivation**);
> `ContinuationInputProjection` cannot carry a per-edge fact for a **zero-input**
> edge; the carrier is the key itself **beside** `consumer_owner`, valued as an
> **occurrence coordinate not an owner** (precedent
> `producer_owner`/`emission_owner`); mint **forward** at the eliminator, never
> reconstruct from the continuation. **No Kernel block is inherited.**
>
> **Row 1 is out of scope** — blocked by the same absent relation
> (`evt_1f4yp49cx23m4`) **and** by a separate earlier `NativeJoinPlanV1` refusal
> this node does not supply.
>
> **If the node stops early, `RT-C2-DRIVER-STAGE-ATTRIBUTION` is the ring's
> fill-in** (`ready`, `XS`, Adversary-sourced, two comment clauses). Told the
> leader to take it as fill-in, **not as a passenger on this candidate** — this
> arc has fired four stops and every one was correct.
>
> ### LANGUAGE: working `LANG-GADT-SEQUENCE-TRACKER-GAP`, anchor `evt_6jb0p5w0zx69p`
>
> Ring reset to `f78b486d`. **`language-leader` is NOT compacted** — the gate's
> `/compact` was refused (*"disabled while a task is in progress"*) and stayed
> refused on retry; implementer and QA are compacted and verified. **Retry its
> compaction at the next WP boundary.**
>
> **Its status sat at `ready` for 45 minutes after the kick** — release step 10
> missed on that kickoff; flipped to `active` and published in `018abf96`.
>
> **Language's pipeline is filled BY THE NODE IT IS RUNNING.** That node's `D1`
> creates four tracker nodes for the four existing `SURF-gadt-*` frames, which
> become the next frontier. The only other open Language nodes are
> `LANG-FOREIGN-NAME-FORMAT-CHARS` (**`gate: operator`**, genuinely blocked) and
> `LANG-SORT-META-CAPABILITY` (spec-enclave, a ruling request). ⇒ **If the audit
> lands without producing those four nodes, Language has no ungated next node**
> and the format-chars gate becomes an operator ask.
>
> ### KERNEL AND VERIFY ARE IDLE, NEITHER IS FRAMING DEBT
>
> **Kernel** — `KERNEL-NESTED-IND` blocked at `AC-K12` on
> `RT-NESTED-IH-NATIVE-REALIZATION`, Runtime-owned and parked behind the
> operator's RecursiveDescent ruling. **Kernel has no other node.** The ruling
> idles that ring; a consequence for the operator, not to re-derive.
> **Verify** — `SEC1-IFC-R3`'s own text forbids framing a Verify slice against
> it; blocked on Spec closure, Architect-owned kernel-facing theorems, and the
> operator's V3 fork. Hard stop already routed. `verify-implementer`/`verify-qa`
> status lines still read "awaiting review" on `ce11f99e` — **stale**;
> `V3-KRIPKE-DECOMPOSITION` is `merged`.
>
> ### THE OPERATOR PRIORITY RULING STILL GOVERNS RUNTIME
>
> **Verbatim:** *"that is the priority for the runtime team. prioritize that
> work over other runtime work."* The retirement chain is
> `RT-CONTKEY-CONSUMING-OCCURRENCE` → `RT-LEXICAL-RECURSOR-CONSUMERS` →
> `RT-RECURSOR-TRANSPORT` (`draft`, every other dep merged) →
> `RT-DESCENT-RETIRE`. `RT-CHECKED-IH-REALIZATION-AUTHORITY` stays `ready` and
> held. **My contrary sequencing call was surfaced and overruled — do not
> re-derive it.**
>
> **Still ruled and relitigated often:** `10369776` is held evidence only, not a
> candidate or repair base. **Zero new `#[ignore]`** — the earlier quarantine
> ruling at `evt_7vhjcstd37a50` is withdrawn and was not revived.
>
> **`D2k-0` IS ANSWERED** — the control holds, 1/1 at `12cefd5b`, tested rather
> than inherited. `D2k-1a` is fully landed: zero `ken-runtime` delta between the
> QA-approved `afc97115` and `main`. **The `runtime-qa` pane still reads
> "awaiting the leader's merge Decision" — that is the squash-merge shape, not
> an obligation.**
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
