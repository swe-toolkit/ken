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

**`main` = `0db668d2`.** Landed through **PR #2230**. Tree clean, nothing
unpublished, no publisher running. **Seventeen PRs merged 2026-08-14**
(#2214-#2230); four of them code: `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD`,
`RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED`, `LANG-MATCH-DIAGNOSTIC-PROSE`,
`LANG-WITNESS-ARITY-DERIVED`.

**ONE LIVE CARRY, filed and NOT discharged by its merge.** The
`LANG-WITNESS-ARITY-DERIVED` `expect` on the diagnostic path has an
**error-policy asymmetry with `ctor_name`**, and **no silent arity fallback is
authorized** (Architect, non-blocking). It is in the node. Whoever next touches
that path owns it.

**NOTHING IS OWED AND NOTHING IS UNFRAMED. Both rings are working and each has
a `ready` successor behind it.** Kernel, Verify and Foundation are idle and none
of them is framing debt -- measured, not assumed.

| ring | in flight | successor, `ready` and framed |
|---|---|---|
| runtime | `RT-CONTKEY-CONSUMER-DESCENT-CARRY` (S), engaged from `6da108b6` | `RT-CONTKEY-REFUSAL-PROFILE-SPLIT` (S) |
| language | `LANG-REACHABILITY-SUBSUMING-ARMS` (M), kicked `evt_1b5egz25x3xs6` | `LANG-CONVOY-ENCLOSING-FIELD` (S), ready, unrouted by design |
| spec-enclave | `SPEC-MATCH-PATTERN-PINS` (M), kicked `evt_5xx5y7frrs4d7` | per-slice, after each pin lands |
| doc | none -- stood down | `TEST-NATIVE-STACK-PROVISIONING-STANDARD` merged, retros in |

**Verify a seat by its PANE, not its status** -- `language-implementer` read
"idle" while 22 minutes into a targeted `-p ken-elaborator` run.

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

**Grep the chapter for "tracked follow-on", "is a follow-on", "deferred to a
later", "not delivered here". A tracker audit cannot see an obligation never
entered into it.** Four nodes, one false positive cleared, and it refuted an
escalation I was about to send ("Language has no ungated work").

**`LANG-MATCH-PATTERN-FORMS-ABSENT` is the largest, and its prerequisite is now
released as `SPEC-MATCH-PATTERN-PINS`** -- the five spelling pins the enclave
named itself. **The next node in that chain is enclave-owned, not Language's.** `34 §3` lists nine pattern
forms; `PatKind` is `Wild | Var | Ctor` (`ast.rs:167`) and `MatchArm` has no
guard field (`ast.rs:86`). **Literals, tuple/pair, record, as-, or-patterns and
guards are all absent** from a chapter marked *impl-ready (L2). Normative and
high-priority*, with **no deferral statement for any** and no tracker row.
Reached from the Adversary's hunt, which measured the symptom while reviewing
something else.

### "BACKLOG CLEAR" AND "BACKLOG GATED" ARE DIFFERENT FACTS

`LANG-REACHABILITY-SUBSUMING-ARMS` sat `draft` behind *"when Language's
conformance-grounded backlog is clear."* **It is not clear -- it is GATED**, and
every alternative waits on someone else. ⇒ **Releasing it is SEQUENCING (mine),
not a priority call between `ready` WPs (the operator's).** Nothing
conformance-grounded is displaced because none of it can be started. **The node
is not reclassified** -- still ergonomics; `§4.2` mandates only detection.

### TWO QUESTIONS ROUTED, BOTH ANSWERED WITHIN THE HOUR, BOTH REFUSED MY FRAMING

**Each was routed with the cheap no-ruling-needed increment offered alongside
it, and that is what converted each into a released node in one exchange.**

**Architect `evt_1rk8wyak0z7sr` -- neither option.** `LANG-CONVOY-ENCLOSING-FIELD`
is `ready`, `S`, **narrowed to the discriminating fixture; it implements no
remedy.** Entry-depth fails because the inner match's entry depth **already
includes** the enclosing match's fields; a threaded floor works but is coarse,
trading a wrong index for a possible new incompleteness. **The third candidate
needs no new provenance:** `cx.var_refinements` is keyed by `bottom_pos ==
abs_pos`, absolute and stable across nesting, so enclosing and inner refinements
**collide on one key** -- and capability 2 inserts without consulting it. **He
bounded his own reading: he has NOT established that the overwrite causes the
`zip` failure.** The node's third outcome is *both hypotheses are wrong*.

**spec-leader `evt_12qrtnp7237dn` -- `34 §3`'s six absent forms are
implementation debt**, stageable only as tracked slices with **every remainder
fail-closed until its slice lands**. Cut order and per-slice pins are in
`LANG-MATCH-PATTERN-FORMS-ABSENT`; the pins themselves are released as
`SPEC-MATCH-PATTERN-PINS`.

**It corrected me, and both nodes are fixed rather than annotated:** guards and
literals each activate their **own** `§4.2` caveat -- what holds of either slice
is that **both the coverage and reachability obligations become live within
it**. And **literals are blocked on more than the open TCB question**: `DecEq
Char` alone is insufficient, because `Float`/`Float32` and `Decimal` separate
runtime value equality from lawful proof `DecEq`, and numeric literals need
expected-type checking.

### STILL OWED BY THE OPERATOR -- THREE, AND ONE OF THEM BLOCKS AN IDLE RING

Do **not** re-raise these without new information; re-posting a standing
question is the servicing loop `§10-a` exists to stop.

1. **THE V3 FORK, `evt_h6pbx30amprj`, raised 2026-08-14** -- and it is the one
   that **parks Verify**. Both sides are now priced: the **D fragment** costs two
   irreducible trusted-base postulates per registrant (`check.rs:1253`, `:1302`,
   `:1308`) across twenty closed atoms and **can start today**; the **Kripke
   embedding** is one hole of twenty-two but a hard `AC-R3c` requirement, and the
   merged decomposition report (`docs/program/v3-kripke-decomposition.md`) found
   it **presently unsizeable** -- no honest prover-side first increment exists,
   and its two kernel-facing theorems cannot be assigned to Verify at all.
   ⇒ **The question is not "which is worth more" but "TCB cost, or a parked
   team".** Both are the operator's.
   **This item had fallen off this list** while the fork sat priced since
   2026-08-14 02:51 and Verify sat idle. That is the failure to not repeat.
2. **`LANG-FOREIGN-NAME-FORMAT-CHARS`** -- *whose reading is the threat model?*
   If Ken source is read by agents consuming bytes, a bidi override deceives
   nobody; if by humans in a terminal, it may. `gate: operator`, and its body has
   said since 2026-08-13 that **neither disposition may be built** until it is
   answered.
3. **The decidable-equality TCB question** (`evt_30gckze0jryj4`) -- **this is the
   D-fragment side of item 1**, asked separately and earlier. Answering item 1
   answers this. It gates `LANG-DECEQ-CHAR-LAWFUL-INSTANCES`.

**Idle rings, and why each is idle -- so nobody re-measures this.** Kernel:
`KERNEL-NESTED-IND` waits on `RT-NESTED-IH-NATIVE-REALIZATION`, whose successor
`RT-CHECKED-IH-REALIZATION-AUTHORITY` is `ready` but **queued behind the
RecursiveDescent chain in Runtime's single-implementer ring** -- a resource
constraint under a priority the operator already set, **not framing debt and not
a question to re-ask**. Verify: item 1. Foundation: `DS-9` is `draft` behind
`KERNEL-NESTED-IND`, and the ring is stood down by me.

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
