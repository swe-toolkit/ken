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

> ### PRE-2026-07-26 CONTENT IS AT BLOB `c26ee67f`
>
> ~2700 lines of windows back to 2026-07-21, archived here on 2026-07-26 --
> `git show c26ee67f`. **The rewrite audit was a SCAN, not exhaustive**: headings,
> authoritative-looking blocks, then a sweep for sole-source markers, decision
> ids, held items and preserved refs. A reader needing something from before that
> date should assume it is in the blob, not that it was considered. (The scan is
> what found two self-declared-authoritative blocks that were wrong, and a
> hand-maintained list of 6 preserved refs when origin held 26.)

## LIVE — 2026-08-15 08:20Z

**`main` = `a737d8c9b`.** One publisher running (PR #2299, language `D1`); node
edits are held uncommitted until it clears, because a background publisher
races the next commit in this worktree.

**TWO LANES AND NOTHING ELSE GETS A RING** (operator, 2026-08-15; `steward.md`
§0). Runtime retires `RecursiveDescent`; language + verify do the z3 round-trip.
Finished work still merges, filings queue, and framing for these two lanes is
lane work.

### LANE 1 — runtime is working, and the chain moved twice this morning

**`RT-REQUIRED-OCCURRENCE-PROJECTION` merged at `66715f9fb`** (PR #2293), `D1`
through `D4` delivered. Its `D4` advanced **row 4 depths 2 and 3** to a `Closure`
refusal. The full record, including the cross-node population collision that
reddened it once, is in the node.

**The successor is [[RT-REQUIRED-CONSUMER-REACH-CENSUS]]** — filed, kicked
(anchor `evt_37bptmz5tgse5`), and **active**. It exists because settling the
question in the tree turned up something no artifact recorded:

> **The projection is minted only under `if required != source`**
> (`static_transition.rs:11683`). Where the two coincide there is **no
> projection to read** — and they coincide at **row 4 depth 1**. ⇒ **Depth 1 is
> outside the new surface by construction, not behind its boundary.** A repair
> cut on "the projection now serves rows 4 and 5" would be cut wrong in exactly
> the way `D2k-1c` was.

**`D1` is discharged** (`evt_6qc0vkzj43c0e`): rows 4 depths 2 and 3 both refuse
at `lowering/mod.rs:11550-11552` — *"a closure cannot cross the boundary: it is
runtime-local and live-domain only, and it has no durable lane"*. **That is
[[RT-CLOSURE-BOUNDARY-LANE]]'s exact signature, for a different population.**

**Do not fold the two nodes.** The site is the closure arm of
`boundary_transfer_admissibility`, a **total** graph walk — every
closure-carrying graph refuses there, so a shared sentence is evidence the gate
is total, not of a shared root. The Architect sharpened it: the function carries
a **second** closure arm for `ComputationalRecursorClosure`, and these rows are
**not** hitting it, so the offending child is a general closure value the
function itself distinguishes one arm away.

**`D5` is released and carries three pre-committed branches**
(`evt_3q0742egf06dg`, released `evt_686me47k2edj2`). The third — closure present
under suppression but the **crossing not reached** — is the one my own two-way
fork was missing, and a two-way measurement would misattribute it. `AC-8` exists
to force the separation.

**The one live stop:** if the row reaches no transfer under either setting, the
attribution is misaddressed — say so rather than forcing a branch.

**Chain:** `PROJECTION` (merged) → `CENSUS` (active) → ? → `TRANSPORT` →
`DESCENT-RETIRE`. `RT-RECURSOR-TRANSPORT`'s `D3` gate is keyed on the **tree**
(`enum RecursiveDescentResidual`, `core.rs:1979`, two live variants), never on
node status. **Do not "fix" that wording.**

### LANE 2 — language's `D1` was already landed, verify between work

**`LANG-INTERVENING-LET-FRAME-WEAKENING` `D1` LANDED AT `beb31566b`, PR #2282,
2026-08-15T03:49:57Z.** The test blob
`7c9d1eb06e7ad3f2e6a6d2be4579f1d28359caf9` is byte-identical on `origin/main`
and on the candidate `fe7be838`, and a squash merge of that candidate onto
`main` stages **nothing**. **`fe7be838` is not an ancestor of `main` and never
will be** — a squash rewrites the commit, so a landed branch head reads as owed
forever. Both `language-qa`'s and the leader's statuses were stale reads of
merged work. PR #2299 closed, branch deleted, evidence in the close comment.

**Two corrections, both mine.** I did mis-resolve a participant id at 08:04 and
that post woke nobody — real, and the reason to use
`scripts/moot-actor-id.sh <role>` every time. **But I then attributed a
four-and-a-half-hour delay to it, and that delay did not exist**: `D1` had
merged four hours earlier. **A stale status corroborated by a second stale
status is not corroboration** — both seats were reading the same unlanded-looking
branch head. The blob test settles it in one command and should lead, not
follow, the next such exchange.

`D4` is on its own child branch, unblocked, and the rebase deferred *"until D1
lands"* can proceed.

**`V3-Z3-PROCESS-ADAPTER` merged.** `blocks: []`, and the throughput successor
needs a catalog-scale corpus that does not exist. **Do NOT invent a z3
successor.**

**LIVE HAZARD:** the `z3-process-adapter` CI job is in the **required**
aggregate, so an apt failure reds every PR fleet-wide. **Do not just drop it** —
only that job witnesses SMT-LIB emission, because `stub(..)` discards stdin.

### The operator brief for 11:30 — three items, and that is the whole list

1. **Verify has no directed successor.** Between work, not blocked.
2. **`V3-KRIPKE-THEORY-CLOSURE` is `ready`, framed, enclave-owned**, and closes
   lane 2's FO Kripke half; `spec-leader` is idle awaiting a released node. **I
   did not kick it.** §0 rule 1 bans starting a third ring *"however well-framed
   and however idle the team"*, and *"this idle team unblocks a priority lane"*
   is recognizably one of the three arguments that already defeated the priority
   once today. **Operator's call, and the work is not wasted whenever it runs.**
3. **Language has no lane-2 successor.** Its current node is elaborator work
   discharging an Architect follow-up, not the z3 round-trip.

**Still owed by the operator — three, do NOT re-raise:** `evt_h6pbx30amprj`;
the `LANG-FOREIGN-NAME-FORMAT-CHARS` threat model; `evt_30gckze0jryj4`.

### Queued, and none of it may jump the lanes

- **The publisher gate misdiagnoses an already-landed candidate as an
  environment fault.** `scripts/scripted-pr-automerge.sh:519` does `git merge
  --squash` then `git commit`; when the candidate is **already contained in
  `main`** the merge stages nothing and the commit exits non-zero with *"nothing
  to commit"*. The gate carefully separates merge-failure from commit-failure
  and then reports *"an environment fault in the publisher... check that the
  scratch worktree is writable"* — **which sends the reader to look at disk and
  permissions for a condition that is neither.** Measured 2026-08-15 on PR #2299.
  **The fix is one probe:** if `git diff --quiet origin/main <head>` (or the
  squash stages nothing), say *the candidate is already contained in `main`* and
  name the landing commit. **Mine, and it queues** — it is `scripts/`, so it is
  outside the Adversary's scope by `COORDINATION §10⁻a` and is found only by
  whoever trips over it.
- The **tri-state convention** the Adversary raised (`evt_62attjpj3esa`): the
  empty-scan fallback and `validator_admitted` on `D2k-1e` are the same shape —
  a two-state observation standing in for a three-state question whose missing
  state is *the instrument did not observe*. **Two nodes owe one conversion**,
  so it wants one convention, not two half-arguments. Explicitly excluded from
  the census node's scope.
- `CONF-BLOCKER-OWNER-RESOLVABILITY` — `ready`, NOT kicked; enclave.
- ken-interp `Term::J`/`cast_reduce` — a **declared** G1 oracle boundary at
  `16 §9.1`, **not a defect**.

### Rules earned 2026-08-15

1. **"Unblocked" is not "reachable".** A surface can land, clear every tracker
   edge, and still not reach part of its population — check the guard that
   mints it, not the node's status.
2. **Scope a stop condition to the deliverable that depends on it**, never to
   the node. Mine stopped `D2` and `D4`, which never depended on the fork, and
   the ring's wide reading was the correct reading of what I wrote.
3. **A shared refusal sentence is shared syntax.** When the gate is total, every
   population reaches it; the attribution is upstream.
4. **Resolve a participant id at post time, from the script.** A guessed id
   posts successfully and notifies nobody — it looks like a delivered message
   and is a silent stall.
5. **A control's population is a claim a sibling node can falsify** without
   touching the control.
6. **Check the PASSING half before disposing of a red** — an absence-assertion
   passes for free once the row dies earlier.
7. **A pooled denominator structurally cannot report per-row facts.**
8. **Attribute before re-triggering:** is `main`'s `crates/` tree identical to
   the last **green** code merge?
