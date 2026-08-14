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

**`main` = `c4ead9b3`.** Landed today through **PR #2217**. Tree clean, nothing
unpublished, no publisher running.

**THREE RINGS WORKING. Kernel, Verify and Foundation are idle and NONE of them
is framing debt** — measured 2026-08-14, not assumed.

| ring | node | anchor | state |
|---|---|---|---|
| runtime | `RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` (XS) | `evt_28wapm370mj11` | gated at `dfd00ba8`, implementer working |
| language | `LANG-MATCH-DIAGNOSTIC-PROSE` (S) | `evt_4zk1wykfjspp9` | gated at `c4ead9b3`, just kicked |
| doc | none — stood down | — | `TEST-NATIVE-STACK-PROVISIONING-STANDARD` merged, retros in |

**Merged today:** #2214 (Steward), #2215 `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`
exact `626a9c8a`, #2216 `TEST-NATIVE-STACK-PROVISIONING-STANDARD` exact
`695eff8b`, #2217 (M7 bookkeeping).

### THE ONE THING NOT FRAMED, AND IT IS NOT MINE TO GUESS

**The `D2k-1c` successor.** Runtime hit its named hard stop at the **second**
boundary (`evt_774xvjz3n5axs`), retained no candidate. **My scope disposition is
made and is not reopened:** the slice is not widened, and a WP that must cross
its own banned scope to discharge its AC has been **cut wrong**, so the repair is
a new cut.

**Architect mechanism ruling `evt_6td3bs6j6g14m`, transcribed in full into
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md` — read it there, not
here.** Its substance: the landed relation is **right** and composes along the
**producer** axis; `consuming_occurrence` is **SOURCE-keyed** and cannot name the
depth-2/3 consumer **in principle**, because that identity is determined by which
specialization realized the body. Route (c) is a **specialization-keyed** relation
minted at the existing descent push, on the `D5a` precedent.

**He declined to size it and named a PROBE that selects the successor:** at the
descent push, for row 4 depths 2 and 3, does the required consumer identity equal
the one derivable from `enclosing_specialization`? **The probe may not author a
candidate.** Frame the successor from its answer, and fold in the **three-way
`None`** split — row 1's `None` means *ambiguity, declined*; depth 2/3's `None`
means *no relation exists* — opposite facts wearing the same value.

### OWED BY THE OPERATOR, AND ONE IS NOW BLOCKING A RING

**Language has NO ungated work after its current node.** Its only two other nodes
are `LANG-FOREIGN-NAME-FORMAT-CHARS` and `LANG-DECEQ-CHAR-LAWFUL-INSTANCES`, both
`gate: operator`. **That is an escalation I owe the operator, not framing debt I
can discharge.** Also outstanding: the decidable-equality TCB question
(`evt_30gckze0jryj4`).

I checked the wider surface before concluding this: `36-effects` is substantially
built, and `34 §4.2`'s reachability subtleties are not live work because **guards
are not implemented in the surface at all**.

### READY AND DELIBERATELY UNRELEASED

- `TEST-STATED-STACK-SITE-RECONCILE` — runtime-owned, `ready` now that the
  standard landed. **Sequences AFTER the `RecursiveDescent` chain** (operator's
  standing priority). Not Runtime's next node.
- `PROG-TRACKER-MERGE-DRIVER` — `owner: steward`, mine to execute when no ring
  needs framing. **Its `D0` exists because the recorded reason for rejecting
  `merge=union` is FALSE**: `gen-progress.sh --check` greps `TS_PATTERN` out of
  both sides, and that pattern covers the issue **count** as well as the
  timestamp, so a doubled header is invisible to the check.
- `RT-C2-DRIVER-STAGE-ATTRIBUTION` — Runtime's next fill-in.

### RULES EARNED TODAY

1. **Read the Decision object at publish time**, never from an earlier
   `list_decisions` dump. I told the Architect a Decision was unresolved 18
   seconds after he resolved it, because I refreshed the git evidence and
   inherited the Decision evidence.
2. **Do release step 10 in the same turn as any kickoff.**
3. **A publisher that looks stalled may be in its initial `sleep`.** Check
   `pgrep -aP <pid>` for a `sleep N` child **before** diagnosing CI. `gh pr
   checks` pending=0 with the PR still OPEN is that, not a stuck suite.
4. **Verify a splice landed.** A failed python assert left a commit whose message
   claimed work it had not done.
5. **Merge order: docs candidates collide only on the generated tracker, and my
   own M7 is the collider — not a code candidate.** Order the merges; do not hold
   them.
