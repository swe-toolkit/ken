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

**`main` = `15c21269`.** Landed through **PR #2224**. Tree clean, nothing
unpublished, no publisher running. **Eleven PRs merged 2026-08-14** (#2214-#2224);
three of them code: `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`,
`RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED`, `LANG-MATCH-DIAGNOSTIC-PROSE`.

**NOTHING IS UNFRAMED.** The D2k successor is framed and released; both build
rings have a node. Kernel, Verify and Foundation are idle and none of them is
framing debt -- measured, not assumed.

| ring | node | state |
|---|---|---|
| runtime | `RT-CONTKEY-CONSUMER-DESCENT-CARRY` (S) | `ready`, released 2026-08-14 |
| language | `LANG-WITNESS-ARITY-DERIVED` (S) | assigned on `eb3806de`, implementer mid-turn |
| doc | none -- stood down | `TEST-NATIVE-STACK-PROVISIONING-STANDARD` merged, retros in |

### THE D2k ROUTE WAS RECOVERED BY ASKING, NOT BY FRAMING

**Runtime's bounded probe returned "No", which by the Architect's own fork
selected route (b). The No was his own off-by-one.** The raw pairs showed
depth 3's requirement `(26,21)` **is** depth 2's derived pair -- so "not equal"
held, but "the consumer is outside both carriers", the stated warrant for (b),
did not. Ruled at `evt_56dvtaft7ep38`: **`required(N)` = the consumer
established at level `N-1`; route (c) survives.**

**Two things made it recoverable, and both are worth keeping.** Runtime reported
**raw values** rather than a bare verdict. And the fork's **No branch carried its
warrant in writing**, so the data could refute the branch rather than only
select it.

**Both rulings are transcribed in
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md`, with a hard-stop header
above the superseded probe spec. Read them there.**

**The one constraint that must not slip, ruled twice:** that supplying the
relation **closes** the route is **not established** -- the original stop named a
further `Closure`/static-worker refusal and a retained standalone recognition.
**An AC assuming closure assumes what nobody measured.** The frame has no such
AC, deliberately, and says so.

### RUNTIME SEQUENCING -- THE ORDER IS NOT ARBITRARY

`RT-CONTKEY-CONSUMER-DESCENT-CARRY` runs **before**
`RT-CONTKEY-REFUSAL-PROFILE-SPLIT`. They share `static_transition.rs` and must
not run concurrently. The Architect ruled the split's new refusal variant *"must
be named for what it OBSERVES, since if the successor lands the absence stops
being structural"* -- **this is that successor**, so landing it first lets the
split name reality rather than a prediction.

`TEST-STATED-STACK-SITE-RECONCILE` (`ready`) and `RT-C2-DRIVER-STAGE-ATTRIBUTION`
sequence **after** the `RecursiveDescent` chain, the operator's standing
priority. Neither is Runtime's next node.

### FOUND BY GREPPING THE SPEC FOR DEFERRAL PHRASING -- KEEP DOING THIS

**Grep the spec chapter for "tracked follow-on", "is a follow-on", "deferred to
a later", "not delivered here". A tracker audit cannot see an obligation that
was never entered into it.** Three real nodes so far (spec 37's `filter`, its
`DecEq Char` transport, the convoy gap) and one false positive cleared.

**It also refuted an escalation I was about to send** -- "Language has no ungated
work" -- before it went out. Language now has `LANG-WITNESS-ARITY-DERIVED`
(`ready`, kicked) and `LANG-CONVOY-ENCLOSING-FIELD` (`draft`, unsized, waiting on
an Architect call about the discriminator's shape).

Cleared false positive, recorded so nobody re-investigates:
`33-declarations.md:751` defers the `export`/re-export build to "the named
Language follow-on", but **that build has substantially landed** -- `modules.rs`
carries the export tables and abstract export, `error.rs:612` the re-export
collision error.

### STILL OWED BY THE OPERATOR -- BOTH ALREADY RAISED, NEITHER BLOCKING

Do **not** re-raise these without new information; re-posting a standing
question is the servicing loop `§10-a` exists to stop.

1. **`LANG-FOREIGN-NAME-FORMAT-CHARS`** -- *whose reading is the threat model?*
   If Ken source is read by agents consuming bytes, a bidi override deceives
   nobody; if by humans in a terminal, it may. `gate: operator`, and its body has
   said since 2026-08-13 that **neither disposition may be built** until it is
   answered.
2. **The decidable-equality TCB question** (`evt_30gckze0jryj4`) -- is widening
   decidable equality worth two irreducible postulates per registrant? It gates
   `LANG-DECEQ-CHAR-LAWFUL-INSTANCES`, which cannot be scoped or sized until it
   is answered.

### ADVERSARY -- THREE HUNTS TRIAGED, NONE REPLIED TO

`COORDINATION §10-a`: the edge is report-only. **Dispositions are recorded in the
node, never sent back down the edge** -- a sentence in the artifact stops a shape
being re-surfaced permanently; a message stops it once.

- `evt_4zx9xp7qkf6rm` -- narrowed by measurement before filing. There is **no
  live wrong-arity output**; all four sites pass the `.args.len()` of the same
  constructor whose `id` they pass. The remedy is a **deletion** (`env.rs:495`
  makes arity derivable from the id), not the four fixtures the hunt ranked
  first. Filed as `LANG-WITNESS-ARITY-DERIVED`; the narrowing is recorded **in
  the node** so the "three emitters emit wrong arities" framing is not
  re-surfaced.
- `evt_2e245r28s3m6n` -- folded into `RT-CONTKEY-REFUSAL-PROFILE-SPLIT` rather
  than queued behind it. **The amendment exposed an `AC-3` that banned the only
  clean discharge of its own new `D3`**; rewritten, with a new `AC-6` pinning
  accept/reject incidence so the loosening is safe.
- `evt_4d10j8tmjsbhj` (on `c2f285ee`) -- everything verified, nothing to fix. Its
  own finding is the useful part: the node was an **as-built correction**, not a
  citation repoint.

### MY WP CUT WAS ACCEPTED OVER THE ARCHITECT'S

Row 1's `None` split moved out of the D2k successor into
`RT-CONTKEY-REFUSAL-PROFILE-SPLIT`'s `H4`. His words: *"your cut is better than
mine -- I grouped it by which node noticed it; you grouped it by defect class."*
**Grouping by defect class is the rule to reuse.**

### READY AND DELIBERATELY UNRELEASED

- `PROG-TRACKER-MERGE-DRIVER` -- `owner: steward`, mine when no ring needs
  framing. **Its `D0` exists because the recorded reason for rejecting
  `merge=union` is FALSE**: `gen-progress.sh --check` greps `TS_PATTERN` out of
  both sides, and that pattern covers the issue **count** as well as the
  timestamp, so a doubled header is invisible to the check.

### RULES EARNED 2026-08-14

1. **Read the Decision object at publish time**, never from an earlier
   `list_decisions` dump.
2. **Do release step 10 in the same turn as any kickoff.**
3. **A publisher that looks stalled may be in its initial `sleep`.** Check
   `pgrep -aP <pid>` for a `sleep N` child **before** diagnosing CI. Measured at
   865s and 1004s.
4. **Merge order: docs candidates collide only on the generated tracker, and my
   own M7 is the collider -- not a code candidate.** Order the merges; do not
   hold them.
5. **A fork's No branch must carry its warrant in writing.** That is what let the
   D2k probe's data refute the branch instead of merely selecting it.
