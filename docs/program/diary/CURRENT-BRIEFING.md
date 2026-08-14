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


> ### RESUME HERE — state at 2026-08-14 ~15:0xZ. **`main` = `6c2a7a7b`.**
> **BOTH BUILD TEAMS ARE WORKING. Nothing held, tree clean, no publisher.**
> Landed since the last block: `#2209` `a998d3f6` Runtime `RT-CONTKEY` (M6 tree
> `1ac31455`) · `#2210` Language GADT audit (tree `8ed1d238`) · `#2211` Steward
> M7 + rulings · `#2212` Language's next node.
>
> ### RUNTIME: `D2k-1c` — anchor `evt_296nwf42qd2ft`, thread `thr_5ngmcb9tnhym`
>
> Gated + compact-verified at `a998d3f6`, leader confirmed `Working`. The
> relation it waited for exists: `ContinuationSpecializationKey` carries
> `consuming_occurrence` beside an unchanged `consumer_owner`.
>
> **THE GRADING INVERTED.** `RT-CONTKEY`'s `AC-6` required rows 4/5 to **still
> refuse**; `D2k-1c` requires them to **consume**. The kick says so explicitly.
>
> **Row 1: measurement requested, not a fix.** It was blocked by this relation
> *and* by an earlier `NativeJoinPlanV1` refusal. The relation now exists ⇒ if
> row 1 still blocks, `NativeJoinPlanV1` is the **sole** blocker — a different
> shape from what was reported. Report only.
>
> ### THE ADVERSARY FOUND A REAL HOLE IN THE MERGE I JUST PUBLISHED
>
> `evt_7b75nbgqbw04z`, triaged CONFIRMED → **[[RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED]]**
> (`ready`, `XS`, runtime). `consuming_occurrence` has two fields;
> **`eliminator_origin` is copied from the input into the re-derivation before
> the comparison, so `AC-1` asserts `x == x` on it**, and `AC-2`'s mutation
> perturbs `body_origin` only — **step 1 has never been fired.** Nothing is known
> wrong; there is no evidence that half is right. `AC-3`'s "population of two" is
> **one** on the copied axis: one eliminator, two bodies.
>
> ⇒ **`eliminator_origin` is exactly the field with no red between that merge and
> `D2k-1c`, which is consuming it now.** Sequenced after `D2k-1c`, or as fill-in.
> Recorded in the merged node; **not replied to the Adversary (`§10⁻a`).**
>
> ### LANGUAGE: `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD` — anchor `evt_4mwvc7gg2bzrt`
>
> Gated at `6c2a7a7b`. **`language-leader` COMPACTED this time** — the twice-refused
> `/compact` cleared once it left the WP boundary. Implementer was mid-compaction
> at 75% when I posted (gate allows "compacting"); **verify it finished.**
>
> **All four `SURF-gadt-*` nodes came back `merged`**, so the audit supplied no
> next work — its `D3` residual is the node instead. `34 §4.1` needs the applied
> **pattern witness**; `ExhaustivenessError { missing: String }` is documented as
> a constructor **name**, so no emission-site change reaches it. **It reads as
> satisfied because every omission test uses a zero-arity constructor, where name
> and most-general pattern coincide.** `AC-1` is the criterion: a test that FAILS
> against the old payload.
>
> ### MY TWO ERRORS TODAY, both stated publicly, both with a durable rule
>
> 1. **I asserted a Decision's current state from a cached read.** Nudged the
>    Architect that `dec_28s60t6n2w5y5` still named `07da235f`; he had re-resolved
>    on `4b8f6777` eighteen seconds earlier. **I re-measured the git side against
>    the new `main` and inherited the Decision side from a dump.** ⇒ **Read the
>    Decision at publish time, never from a dump taken earlier in the turn.**
> 2. **Release step 10 missed** on the GADT kickoff — node sat `ready` 45 min.
>
> ### THE COLLISION CLASS — resolved procedurally, fix not yet filed
>
> `IMPLEMENTATION-PROGRESS.md` is generated from `docs/program/issues/`; **every**
> node add/flip rewrites it and CI runs `gen-progress.sh --check`, so a candidate
> **cannot drop it**. Two docs candidates in flight always conflict there and
> nowhere else. **A code candidate is NOT the collider — my `M7` is.** The working
> procedure: **order the merges, do not hold them**, and resolve any rebase
> conflict by re-running the generator, never by hand.
>
> **Registered fix (Architect `evt_2kf7xke2q2nvc`), my lane:** a custom merge
> driver on that path re-running `gen-progress.sh`. **`merge=union` DOES NOT
> WORK** — doubles the timestamp/count and reds `--check`. **Still unfiled. This
> is the one piece of framing debt I know about.**
>
> ### KERNEL AND VERIFY ARE IDLE, NEITHER IS FRAMING DEBT
>
> **Kernel** — `KERNEL-NESTED-IND` blocked at `AC-K12` on
> `RT-NESTED-IH-NATIVE-REALIZATION`, parked behind the operator's RecursiveDescent
> ruling; Kernel has no other node, so **the ruling idles that ring.**
> **Verify** — `SEC1-IFC-R3` forbids framing a slice against it; blocked on Spec
> closure, Architect theorems, and the operator's V3 fork. Both routed.
>
> ### THE OPERATOR PRIORITY RULING STILL GOVERNS RUNTIME
>
> *"that is the priority for the runtime team. prioritize that work over other
> runtime work."* Chain: `D2k-1c` → rest of `RT-LEXICAL-RECURSOR-CONSUMERS` →
> `RT-RECURSOR-TRANSPORT` (`draft`, every other dep merged) → `RT-DESCENT-RETIRE`.
> `RT-CHECKED-IH-REALIZATION-AUTHORITY` stays `ready` and held. **My contrary
> sequencing call was overruled — do not re-derive it.**
>
> **Runtime fill-in order if `D2k-1c` stops early:**
> `RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` first, then
> `RT-C2-DRIVER-STAGE-ATTRIBUTION`. **Fill-in, never passengers on a candidate.**
>
> ### TEST-NATIVE-STACK-PROVISIONING-STANDARD — RULED AND LANDED in `#2211`
>
> `evt_4rz7hp11f33wj`. Property is **STATEDNESS**; three acts, only "masking a
> regression" forbidden, its test objective, so the standard **cannot unblock a
> candidate**. Census refuted the node's own evidence (15 sites/14 files/4
> crates/3 constants vs an asserted 6/5/1). **The Architect withdrew his own
> Amendment 2** — its arithmetic had **one sign**. Venue (§3):
> `agent/playbooks/tools/stated-stacks.md`, `pin-a-property` shape. Owner `doc`.
> Site pass = `TEST-STATED-STACK-SITE-RECONCILE`, **`draft` deliberately.**
>
> **Nothing is released to the doc ring yet — that node is `ready` and unkicked.**
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
