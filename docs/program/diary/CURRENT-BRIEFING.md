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

**`main` = `f7ed6dea`.** Landed today through **PR #2219**. Tree clean, nothing
unpublished, no publisher running.

**TWO RINGS WORKING. Kernel, Verify and Foundation are idle and NONE of them
is framing debt** — measured 2026-08-14, not assumed.

| ring | node | anchor | state |
|---|---|---|---|
| runtime | `RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` (XS) | `evt_28wapm370mj11` | exact `e84dc867` **with QA**, routed `evt_6b6wn8b8t33bd` |
| language | `LANG-MATCH-DIAGNOSTIC-PROSE` (S) | `evt_4zk1wykfjspp9` | implementer mid-turn at `c4ead9b3` |
| doc | none — stood down | — | `TEST-NATIVE-STACK-PROVISIONING-STANDARD` merged, retros in |

**Merged today:** #2214 (Steward), #2215 `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`
exact `626a9c8a`, #2216 `TEST-NATIVE-STACK-PROVISIONING-STANDARD` exact
`695eff8b`, #2217 (M7 bookkeeping), #2218 (step 10 + briefing), #2219 (the two
Language successors below).

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

### THE LANGUAGE ESCALATION IS WITHDRAWN — IT WAS FALSE

**This block previously said Language had no ungated work and that I owed the
operator an escalation. A spec sweep refuted it before I sent it.** Language now
has a framed, ungated successor and a second obligation filed behind it.

- **`LANG-WITNESS-ARITY-DERIVED`** — `ready`, `S`, ungated. **Language's next
  node.** Kick it once `LANG-MATCH-DIAGNOSTIC-PROSE` merges; flip that dep
  `merged` first or `--strict` warns.
- **`LANG-CONVOY-ENCLOSING-FIELD`** — `draft`, unsized. Spec `34 §3.2` names the
  two-vector `zip` recursive step a known gap **and a follow-on**, with zero
  tracker rows. Mechanism measured: `outer_scope_depth = cx.ctx.len() - n`
  (`elab.rs:2204`) defines "outer" as raw context depth below the branch's own
  fields, so an **enclosing match's bound fields are indistinguishable from
  genuine parameters**. Completeness, not soundness — the spec says the
  substitution is always kernel-proved. `draft` because the discriminator's
  shape is an Architect call, with the routing question in its flip condition.

**How both were found, and this is the transferable part: grep the spec chapter
for DEFERRAL PHRASING, not the tracker for gaps.** "tracked follow-on", "is a
follow-on", "deferred to a later", "not delivered here". **A tracker audit cannot
see an obligation that was never entered into it.** That sweep has now produced
three real nodes (spec 37's `filter`, its `DecEq Char` transport, and the convoy
gap). It also cleared one false positive, recorded so nobody re-investigates:
`33-declarations.md:751` defers the `export`/re-export build to "the named
Language follow-on", but **that build has substantially landed** — `modules.rs`
carries the export tables and abstract export, `error.rs:612` the re-export
collision error.

### STILL OWED BY THE OPERATOR — BOTH ALREADY RAISED, NEITHER BLOCKING

Do **not** re-raise these without new information; re-posting a standing
question is the servicing loop `§10⁻a` exists to stop.

1. **`LANG-FOREIGN-NAME-FORMAT-CHARS`** — *whose reading is the threat model?*
   If Ken source is read by agents consuming bytes, a bidi override deceives
   nobody; if by humans in a terminal or web view, it may. That answer decides
   whether a whole-source lexical policy has a victim at all, or the node closes
   with the reason recorded. The node is `gate: operator` and its body has said
   since 2026-08-13 that **neither disposition may be built** until it is
   answered.
2. **The decidable-equality TCB question** (`evt_30gckze0jryj4`) — is widening
   decidable equality worth two irreducible postulates per registrant? It gates
   `LANG-DECEQ-CHAR-LAWFUL-INSTANCES`, which cannot be scoped or sized until it
   is answered.

### ADVERSARY HUNT `evt_4zx9xp7qkf6rm` — TRIAGED, CLOSED AS A NODE, NOT REPLIED

**Its `AC-1` re-verification is accepted in full** and is the stronger half: it
reverted `Display` to a name-only render, got exactly the old
`missing constructor 'ConsVector'`, and observed that **only one test in the
crate reddened** — a universal probe proving one test sees the arity property.

**Its causal claim was narrowed by measurement at `96c95586`, and that changed
the deliverable.** All four `missing_pattern_witness` sites pass the
`.args.len()` of the **same constructor whose `id` they pass**, and `:2135` /
`:2314` share one `n` bound at `:2097`. ⇒ **There is no live wrong-arity
output.** What survives is the absent guard: the pairing holds by convention per
call site rather than by construction, and three sites have no witness-inspecting
test, so a later refactor breaks a user-visible diagnostic silently.

**The remedy is therefore a deletion, not the four fixtures the hunt ranked
first:** `ken-kernel/src/env.rs:495` `constructor(id) -> Option<(&InductiveDecl,
usize)>` makes the arity derivable from the id, so the parameter goes and the
class is unrepresentable. Fixtures are the fallback if that lookup is not total
at an emission site. **The narrowing is recorded in the node itself** so the
"three emitters emit wrong arities" framing is not re-surfaced from the hunt
text by a later reader.

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
