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

## LIVE

**`main` = `c932e7b4`.** Landed through **PR #2226**. Tree clean, nothing
unpublished, no publisher running. **Thirteen PRs merged 2026-08-14**
(#2214-#2226); three of them code: `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`,
`RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED`, `LANG-MATCH-DIAGNOSTIC-PROSE`.

**NOTHING IS OWED AND NOTHING IS UNFRAMED. Both rings are working and each has
a `ready` successor behind it.** Kernel, Verify and Foundation are idle and none
of them is framing debt -- measured, not assumed.

| ring | in flight | successor, `ready` and framed |
|---|---|---|
| runtime | `RT-CONTKEY-CONSUMER-DESCENT-CARRY` (S), implementer engaged from `6da108b6` | `RT-CONTKEY-REFUSAL-PROFILE-SPLIT` (S) |
| language | `LANG-WITNESS-ARITY-DERIVED` (S), implementer mid-turn | `LANG-REACHABILITY-SUBSUMING-ARMS` (M) |
| doc | none -- stood down | `TEST-NATIVE-STACK-PROVISIONING-STANDARD` merged, retros in |

**Verify a seat by its PANE, not its convo status.** `language-implementer` read
"idle, awaiting next WP" at 17:16 while 22 minutes into a targeted
`ken-cargo test -p ken-elaborator`. **A status is only as fresh as the seat's
last post**, and reading one as state is how an idle-looking seat gets re-kicked
mid-turn.

### THE D2k ROUTE WAS RECOVERED BY ASKING, NOT BY FRAMING

**Runtime's probe returned "No", which by the Architect's own fork selected
route (b). The No was his off-by-one.** The raw pairs showed depth 3's
requirement `(26,21)` **is** depth 2's derived pair -- "not equal" held, but
"the consumer is outside both carriers", the stated warrant for (b), did not.
Ruled `evt_56dvtaft7ep38`: **`required(N)` = the consumer established at level
`N-1`; route (c) survives.** Two things made it recoverable: Runtime reported
**raw values**, and the fork's No branch **carried its warrant in writing**.

**Both rulings are transcribed in
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md`, above a hard-stop header
marking the superseded probe spec. Read them there, not here.**

**The one constraint that must not slip, ruled twice:** that supplying the
relation **closes** the route is **not established** -- the original stop named a
further `Closure`/static-worker refusal and a retained standalone recognition.
**An AC assuming closure assumes what nobody measured.** The frame has none,
deliberately. If Runtime hands back "the refusal is gone", that is a welcome
observation and **not** an acceptance criterion.

**Sequencing:** `RT-CONTKEY-CONSUMER-DESCENT-CARRY` runs **before**
`RT-CONTKEY-REFUSAL-PROFILE-SPLIT` -- same file, and the split's new refusal
variant *"must be named for what it OBSERVES, since if the successor lands the
absence stops being structural."* `TEST-STATED-STACK-SITE-RECONCILE` and
`RT-C2-DRIVER-STAGE-ATTRIBUTION` sequence **after** the `RecursiveDescent`
chain, the operator's standing priority.

### SWEEP THE SPEC CHAPTER, NOT THE TRACKER -- FOUR NODES SO FAR

**Grep the spec chapter for "tracked follow-on", "is a follow-on", "deferred to
a later", "not delivered here". A tracker audit cannot see an obligation that
was never entered into it.** Four real nodes now, plus one false positive
cleared -- and it refuted an escalation I was about to send ("Language has no
ungated work") before it went out.

**`LANG-MATCH-PATTERN-FORMS-ABSENT` is the fourth and the largest.** `34 §3`
lists nine pattern forms normatively; `PatKind` is exactly `Wild | Var | Ctor`
(`ast.rs:167`) and `MatchArm` has no guard field (`ast.rs:86`). **Literals,
tuple/pair patterns, record patterns, as-patterns, or-patterns and guards are
all absent**, in a chapter marked *impl-ready (L2). Normative and
high-priority*, with **no deferral statement for any of them** and no tracker
row. `draft` and unsized because **the deliverable is the CUT**, which is not
made: literals are gated on the open DecEq TCB question, guards change the
exhaustiveness contract rather than only the grammar, and or-patterns add a
binder well-formedness rule. **It was reached from the Adversary's hunt, which
measured the symptom while reviewing something else.**

### "BACKLOG CLEAR" AND "BACKLOG GATED" ARE DIFFERENT FACTS

`LANG-REACHABILITY-SUBSUMING-ARMS` was `draft` behind a flip condition reading
*"when Language's conformance-grounded backlog is clear."* **It is not clear --
it is GATED**, and every conformance-grounded alternative waits on someone else:
the convoy discriminator on an Architect ruling, the pattern-forms census on a
cut, and two nodes on operator answers.

⇒ **Releasing it is SEQUENCING, which is mine (`ken-steward §3`), not a priority
call between `ready` WPs, which is the operator's.** Nothing conformance-grounded
is displaced because nothing conformance-grounded can be started. **The node is
not reclassified** -- still ergonomics, and `§4.2` still mandates only detection.

### TWO QUESTIONS ROUTED, NEITHER BLOCKING -- DO NOT RE-ASK

Both gate the node **after** next, not the one in flight.

**spec-leader, `evt_26sk9m51rd5nk`** -- what does `34 §3` oblige of the six
absent pattern forms, and what of it is genuinely stageable? Gates
`LANG-MATCH-PATTERN-FORMS-ABSENT`, which is the node after next. **Raised as
"what does the chapter oblige", never as "which node should this be"**, which
presumes the answer.

### THE CONVOY QUESTION IS ANSWERED, AND THE ANSWER WAS "NEITHER"

**Architect `evt_1rk8wyak0z7sr` refused both options I offered** and named a
third whose carrier already exists. `LANG-CONVOY-ENCLOSING-FIELD` is now
`ready`, `S`, **narrowed to the discriminating fixture alone.**

- **Entry-depth is insufficient:** when the inner match is entered, the
  enclosing match's fields are **already in `cx.ctx`**, so its entry depth
  includes them. The needed quantity is the depth at the **enclosing** match's
  entry, threaded down -- which nothing carries.
- **A threaded floor would work but is coarse** -- it excludes `let`s and every
  other binder between the two matches, **trading a wrong index for a possible
  new incompleteness**, the same failure class.
- **The third candidate:** `cx.var_refinements` is keyed by `bottom_pos`, which
  equals `abs_pos` -- **absolute and stable across nesting depth** -- so an
  enclosing and an inner refinement for one binder **land on the same key**, and
  capability 2 (`:3000`-`:3027`) **inserts without consulting it**. The
  per-entry discriminator already exists.

**He explicitly bounded his own reading: he has NOT established that the
overwrite produces the `zip` failure**, and declined to price a remedy from a
mechanism found by reading -- *"I did that in this arc already and it cost a
turn."* So the node measures three things and **implements no remedy**, with the
third outcome being *both hypotheses are wrong*, which he named himself and
called legitimate.

> **THE TRANSFERABLE MOVE: when routing a design question, offer the cheap
> increment that needs no ruling alongside it.** I added *"a failing two-vector
> `zip` fixture is the cheapest thing that makes this concrete and does not
> require the ruling"* as a fallback; he adopted it in terms. **A blocked node
> became a released one in one exchange.**

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
- `evt_4d10j8tmjsbhj` (on `c2f285ee`) -- everything verified, nothing to fix, and
  **its side observation was worth more than a defect would have been.** It
  measured, by enumerating AST variants rather than by a grep that found nothing,
  that `34 §4.2`'s two reachability caveats are **both** vacuous. That is the
  symptom whose cause is `LANG-MATCH-PATTERN-FORMS-ABSENT`. The contingency --
  **adding guards or literal patterns makes both caveats live at once against an
  `arm_used` with no `§3.3` exception** -- is recorded in **both** affected
  nodes, where the person who adds those features will actually be working.

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

Each is stated in full in its own section above; this is the index.

1. **Read the Decision object at publish time**, never from an earlier
   `list_decisions` dump.
2. **Do release step 10 in the same turn as any kickoff.**
3. **A publisher that looks stalled may be in its initial `sleep`** -- `pgrep -aP
   <pid>` for a `sleep N` child before diagnosing CI. Measured at 865s and 1004s.
4. **Merge order: my own M7 is the tracker collider, not a code candidate.**
   Order the merges; do not hold them.
5. **A fork's No branch must carry its warrant in writing**, or data can only
   select the branch, never refute it.
6. **Read a seat's PANE before ruling on its state.** A convo status is only as
   fresh as the seat's last post.
7. **Offer the cheap no-ruling-needed increment alongside any design question
   you route.**
8. **"Backlog clear" and "backlog gated" are different facts**, and only the
   first makes a release a priority call rather than a sequencing one.
