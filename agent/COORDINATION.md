# Ken coordination law (read by every agent)

Cross-cutting rules for every Ken agent, regardless of role, team, or model;
each must hold identically across Opus, GLM, and DeepSeek agents. Role
discipline is in `playbooks/`, model tiers in `MODELS.md`, the git model in
`../docs/program/04-git-and-integration.md`. Each rule exists because skipping
it caused a real stall or bug; those incidents are in
`COORDINATION-INCIDENTS.md`, which is reference, not startup reading.

## 0. The shape: a ring of rings

- **Within a team, a sequential token-ring.** Generally one agent is active;
  the others support it when called on. Do not fan a team onto several tasks
  to chase parallelism: coherence beats it. Once a WP is published, the team
  **waits idle** for its CI run rather than pipelining or stacking work (ADR
  0002). Throughput comes from other teams' rings.
- **Across teams, parallel.** The rings couple at only three points: merges to
  `main` (via the publisher path), the roadmap gate dependencies, and the
  sanctioned cross-team query edges (§9, §11). Keep that coupling thin.

## 1. Event-driven, never poll

After you finish a unit of work or hand off, **post, set status, and stop.** Do
not `/loop`, self-wake, or poll for replies. A missing notification is a stall,
and catching stalls is the team leader's watchdog job, not yours. Only team
leaders and the Steward run schedulers.

### 1a. "Event-driven" requires a named event. A hold you cannot name is a stall.

1. **When you hold, you must be able to name (a) the `evt_`/`dec_` id or the
   concrete act you are waiting for, and (b) the SEAT that owes it.** If you
   cannot name both, you are stalled, and it is your job to find out.
2. **When you hand work back, name the owed act AND its owner**: *"@X owes the
   merge Decision on `<sha>`"*, not *"handed back to X"*.
3. **A leader holding for a member confirms the member is actually working**
   (pane, status, or post). §13's delivery ≠ engagement rule applies to your
   own ring, not only to kickoffs you receive.
4. **A ring silent with no member able to name its blocking event** is the
   cheapest stall to detect and the most expensive to miss. Detect it at the
   ring, before the Steward's backstop is all that is left (a finished turn
   holding an orphaned shell can misread as BUSY).

Incidents: `COORDINATION-INCIDENTS.md#s1a`.

## 2. Mention discipline

**Mention an agent iff you are (a) asking them a question or (b) expecting a
specific action/move from them** (operator, re-tightened 2026-07-03). To
inform, acknowledge, converge, confirm, affirm, CC an observer, or keep someone
in the loop is **not** a reason: post un-mentioned, or don't post. This binds
every role, the Steward included.

**A mootup mention is the only way you reach a teammate.** Every agent is an
already-running peer, never a sub-agent: delegate, query, hand off, or reach
anyone by posting with `mentions: ["<actor_id>"]`. **NEVER** spawn a teammate
with the `Agent`/Task tool, a subprocess, or `claude(prompt)`; that starts an
unconfigured Claude ("503 provider not configured"). "Hand the WP to your
implementer" means a mention, not a launch. Local git only.

- **Resolve an `actor_id` with `scripts/moot-actor-id.sh <role>`** (`--list`
  prints role names; controls in `scripts/test-moot-actor-id.sh`). **Never
  open `.moot/actors.json` yourself, and never dump it to learn its shape**: it
  holds every seat's `api_key`, and another seat's is never yours to read. Both
  leaks so far happened during schema discovery. If you must read your own
  `api_key` (HTTP fallback only), narrow by key names only: never print a
  value, never serialize a record.
- Handoff to B → mention **B only**. "Done," nothing pending → mention
  **nobody**. **Never mention on an ack** (§4).
- **Escalation triangle:** when you answer X but the next move is Y's, mention
  **Y**. Naming Y in prose without a real mention is the classic silent stall.
- **An answer that moves the ball MUST mention the next move's owner,
  including when that is the asker** (your ruling → they author the WP; your
  verdict → they fix or merge). It will feel like "just answering". Populate
  `mentions`; a prose `@name` fires nothing.
- **Route an action-expecting mention only to a live agent**, never a `moot
  init` placeholder (`Spec`, `Leader`, `QA`, `Implementation`, …), where it is
  a silent no-op. A placeholder has `agent_adapter: null` and no recent
  `last_seen_at`; a live agent has `agent_adapter: "mcp"`. Confirm with
  `list_participants` before routing to anyone you did not just hear from.
- **Plain text, no decorative icons** (operator, 2026-08-01; full statement
  in `AGENTS.md` Conventions).
- **Thread every reply** (operator 2026-06-29, restated 2026-08-01): §4.

Incidents: `COORDINATION-INCIDENTS.md#s2`.

### 2a. Convo call cheat-sheet

Malformed calls fail silently to the workflow: the post never lands and the
next move never fires. On a 400/404, **read the error, fix the one named
field, and re-send**; never drop the post.

- **`message_type` MUST be a backend enum value**: `question`, `code_share`,
  `git_request`, `review_request`, `status_update`, `bug`, `feature`,
  `decision_propagated`, `pause_issued`, `connection_status`. Rejected:
  `message`, `kickoff`, `assignment`, `merge_ready`, `handoff`, `nudge`,
  `ack`. When unsure, `question` is always accepted.

  | Intent | `message_type` |
  |---|---|
  | kickoff / assign a WP / hand work to a teammate / deliver code | `code_share` |
  | question / query / nudge / ack / general note | `question` |
  | ask for publisher-path merge handling (a ready `wp/<ID>`) | `git_request` |
  | QA→leader merge-Decision request; "request review" | `review_request` |
  | a status line | `status_update` |
  | a defect note to another team | `bug` |
- **`mentions` MUST be a list of participant_ids** (`["agt_…"]`, from
  `list_participants` / `orientation()`): not display names, not `@name` in
  the text, not a bare string. One id per actor whose move is next.
- **`propose_decision`**: the decision text plus the WP/branch; mention the
  required reviewers (Architect only when the diff touches `catalog/`,
  `crates/` or `spec/`, §8a; Spec only on `spec/` + `conformance/` paths,
  §14). `thread_id`/`parent_event_id` take an id, never a name (§4a).

## 3. Status = what you're doing, in your own words

Of three liveness signals, connection and activity (file mtime) are automatic;
never post "I'm online". The third is yours: **semantic status**
("blocked-on-spec: OQ-17"), agent-composed, never auto-classified. **Update it
on every change in your activity** (operator, 2026-07-03): picking up a WP, a
sub-step, finishing, changing focus, going idle, blocking, unblocking. It is
the Steward's primary watchdog signal; a lagging one reads as stalled when you
work, or working when you are done.

## 4. Threads are the spine

The whole rule in four lines (operator, 2026-08-01):

1. **A thread's identity is a WP.** One WP = one thread, opened by its **kick**.
2. **The kick's own `event_id` IS the thread anchor.** Everything about that WP
   replies under it; nothing about it is posted anywhere else.
3. **A rescope that SPLITS a WP abandons the original thread.** Each component
   WP gets a fresh kick and therefore a fresh thread.
4. **Only `steward`, `librarian`, and `research` may post at top level.**
   Every other role always replies into a thread.

### 4a. The mechanism: a thread does not exist until someone replies

A kick is a root post and returns `thread_id: null`: **the first reply mints
the thread.** A kick cannot quote its own thread id, so the anchor is its
`evt_`, not a `thr_`; every recipient has it from the notification.

| you are… | you call | why |
|---|---|---|
| the **first** reply to a kick | `post_response(parent_event_id=<kick evt_>)` | this **opens** the thread. `reply_to` **404s** ("Thread not found") on a root event that has no thread yet |
| any **later** poster | `post_response(thread_id=<thr_>)` or `reply_to(<any evt_ already in the thread>)` | the thread now exists |
| posting a **new WP kick** (steward/librarian/research only) | bare `post_response`, no parent, no thread | you are creating the anchor |

`reply_to` auto-mentions the original speaker, which makes the wake path
structural. Incidents: `COORDINATION-INCIDENTS.md#s4a`.

### 4b. What every kick must carry

Every WP kick, and every doc-phase kick, states its anchor in its body:

> **This message is the thread anchor for `<WP-ID>`.** Reply with
> `parent_event_id` set to **this event's id**; thereafter use its `thread_id`.
> Do not open a second thread for this WP and do not post about it at the
> space root.

**A kick without that line is defective**; reissue it.

### 4c. Rescope: when the thread is abandoned

| what happened | thread disposition |
|---|---|
| WP **split** into components (a §5a-iii recut, a mis-sizing outcome (c)) | **abandon the original thread.** Post one final line in it naming the successor WP ids, then **a fresh kick — and so a fresh thread — per component WP** |
| WP **rescoped in place** (scope amended, hard stop ruled, deliverable added) | **keep the thread.** Nothing was split, so the WP's identity is unchanged |
| WP **respun** (new candidate SHA after a reject) | **keep the thread.** A rejected candidate is not a new WP |
| work **routed** to another WP (a `D4` route, an inherited obligation) | **keep both threads.** The route is a reply in the *origin* thread; the obligation lands in the *target* WP's **frame**, not as a cross-post |
| WP **merged** | **the thread is CLOSED.** It takes no further posts, ever |

**The test is "did the set of WPs change?"** Only a split changes it.

**Anchor on the WP you are working on, never on the thread you last posted
in.** You will be tempted to keep posting where you have posted all afternoon;
before every post, ask which WP it is about. The successor's kick signals that
the previous slice's thread is closed. Incidents:
`COORDINATION-INCIDENTS.md#s4c`.

### 4d. Top-level is a closed list, and everyone else threads

**`steward`, `librarian`, `research`: nobody else, ever** (operator,
2026-08-01), because only their output is not scoped to one WP. Every other
role, including the Architect and every team leader, carries a `thread_id` or
`parent_event_id` on every post.

- **No home thread? Route to the Steward; never root it.** Thread it under
  the nearest relevant Steward post or the standing `steward:` thread,
  mentioning the Steward if a decision is needed. This includes a design
  ruling that belongs to no WP. The Steward owns thread creation and opens a
  new root when an item warrants its own spine; a spun-off
  WP (own branch + gates + merge, e.g. an erratum) is opened as its own spine
  by the Steward, with a one-line pointer from the parent. It is the only
  sanctioned new thread off a WP.
- **Decisions, questions, reviews still thread.** `propose_decision`,
  `question`, `review_request`, `git_request` attach to the WP thread.
- **Threading drifts two ways** (operator, 2026-07-02). (1) A fork spawns a
  side-thread: a mid-WP escalation to the Architect, Steward, or CV is a
  **reply in the WP thread**. (2) A new WP reuses the previous WP's thread.
  After any reset or compaction, **resolve the live thread from fresh
  context**: the current WP's kick, never the most-recent thread or an id
  from a summarized memory.
- **Post at both boundaries** (operator, 2026-06-29): on picking up a task, a
  brief taking-this note in its thread plus a status update; on finishing or
  handing off, what you did plus the mention of whoever moves next, plus a
  status update.
- **Acks never mention anyone** (operator, 2026-07-03). A mention demands
  attention and an ack demands nothing. You MAY post a brief mention-free ack
  ("seen", "proceeding"); silence is equally fine. If your only reason to name
  someone is to say you received, agree with, or are proceeding on their
  message ("@X noted", "@X standing by"), mention no one and post nothing. A
  substantive routing post mentions **only** the one actor whose move is
  next, not observers. This includes the Steward's posts.

**Title convention.** The federation runs in one space (`ken-topos`), so a
thread's title is its team tag: begin every kickoff title with a tag, then a
terse subject. WP threads use the WP ID,
whose letter encodes the team: `K*` Kernel · `V*` Verify · `L*` Language ·
`X*` Runtime · `Sec*` security · `B*` seam · `T*` tooling · `F*` foundation
(`K2: decidable conversion`). Non-WP threads use the originating role/team,
with an arrow if it targets one (`kernel→spec: OQ on cast normal form`). The
title says who/what, the type (§8) says what kind; keep both accurate.
Incidents: `COORDINATION-INCIDENTS.md#s4d`.

## 5. Decisions are for judgment, not deduction

Open a mootup Decision (`propose_decision`) for choices with tradeoffs where a
reasonable peer might differ: kernel/semantics design, an API shape, a
content-store policy. Not for deductive/mechanical choices (a bug fix is not a
decision). **Merge/review approvals are also Decisions**: the merge Decision
is the review record (`04-git-and-integration.md`).

## 6. Resolve when structurally determined; escalate only real forks

Before escalating or querying another team, ask: *is there a strategic choice
between materially different futures?* If **no** (spec, kernel invariants, and
existing code determine the answer), resolve it yourself and record it with a
cited rationale (`file:line` or spec §). If **yes**, escalate. For clean-room
questions, "the published spec" means `/spec`, never prototype source. This is
the volume control on the query edges (§11).

## 7. Ground every premise before locking

Before locking a spec, ADR, or design claim, verify each premise against
reality: "X exists" → grep; "matches pattern Y" → read Y end-to-end. A spec
claim about the kernel is checked against the kernel, not assumed.

### 7a. Mutable external state is tested at point of use, never cited

Credentials, permissions, scopes, quotas, remote refs, and infrastructure
change without touching any file that describes them.

1. **Test it.** A capability question ("can this credential push that path?")
   is answered by attempting it. **Never escalate a capability claim you have
   not tried.**
2. **When an observation contradicts a record, re-verify the RECORD**, not the
   observation. "I can't tell which; flagging that I don't know" is correct.

A note about mutable state is evidence about the past: cite it to form a
hypothesis, never to close a question. When one is false, **fix it in place
and say when it became false.** Incidents: `COORDINATION-INCIDENTS.md#s7a`.

### 7b. Never default a value your whole conclusion rests on

- **Read the assertion back out of the artifact, and check the operand sits at
  the point in the graph your question is about.** No precedent from a branch
  name instead of its diff; no file content from another seat's summary.
- **Verify the property, not the representative case.** An obvious-case pass
  and correct prose are false signals. When an algorithm has N guard/discard
  positions, **exercise each independently** plus one case where the problem
  hides behind indirection; write pseudocode defensively (every discarding
  position guarded, every guard backed by a rejection case). Binds QA,
  conformance and spec authoring, and review.
- **A discriminator boundary needs a non-degenerate pair on a shared input**
  (accept/reject, `proved`/`assumed`, taint/safe): the two states that must
  bucket differently, identical otherwise, so a flipped boundary fails both. A
  lone positive case passes the flip. The discriminator must be a
  **structural / kernel-side signal** (`trusted_base()` membership, a
  constructor, a real value through a real sink), never a self-reported
  string the untrusted layer can forge.
- **Make load-bearing completeness a compile error**: a **single `match` over
  a sealed type with NO `_ =>` arm**, so an unhandled new variant fails to
  compile. Use a catch-all only where the residual is genuinely uniform. The
  implementer writes it; QA and the Architect verify no `_ =>` sits on a
  completeness-critical match.

Incidents: `COORDINATION-INCIDENTS.md#s7b`.

## 8. Message-type taxonomy (routing metadata)

Tag each message with a type from §2a; the **first line is the thread title**,
with no `[TYPE]` prefix in the body.

## 8a. Architect and Librarian are parallel, over disjoint domains

**Operator directive, 2026-07-22, scope amended 2026-09-26.** The
**Architect** reviews `catalog/`, `crates/` and `spec/`, and nothing else: the
rest of the project is outside its scope. The **Librarian** reviews
`library/`. They review at the same time, without needing to interact.
Neither is a gate on the other; a candidate touching both goes to both at
once.

- **A fold in one domain does not invalidate the other's approval**, which
  binds the exact SHA for its own domain. Re-run the Librarian only if the fold
  touched `library/`. Reviewers who cannot invalidate each other's finding are
  never sequenced.
- **Assembly.** Each reviewer votes on the exact SHA and names the domain it
  bound. The **owning leader** assembles: ready when every touched domain has a
  live approval on the current SHA. On a fold, re-request only the touched
  domains and state which approvals carry forward.
- **Accepted work, not a finished WP** (operator, 2026-08-06: *"merge in
  accepted work once it is done, even if it is only a partial WP"*). A complete
  deliverable with live approvals on its exact SHA goes to the Steward to
  publish; do not wait for the WP's remaining deliverables. WP closure and
  merge are separate events.
- **The one bar is semantic atomicity**: a declared atomic pair lands
  together, because one half regresses or has no reaching witness alone. "The
  branch carries the evidence chain", "a rebase would cost re-anchoring", and
  "a working path would go red" are not bars (§12; 2026-07-28 no-users
  ruling).
- **Prefer a cut that is a straight ancestor of the working tip**, which keeps
  every exact SHA and verdict below it.
- **A reviewed prefix is not thereby releasable**: approvals do not assert
  greenness. Establish greenness on the cut **and** on `main` before routing,
  so a red is attributable rather than inherited.

Incidents: `COORDINATION-INCIDENTS.md#s8a`.

## 9. Topology is invariant, including the query edges

Who hands off, reviews, and merges, and which cross-team query edges exist, is
operator-owned and fixed. The sanctioned edges are exactly:

- any team → **Spec** leader: behavioral-contract questions.
- any team → **Architect**: component-design questions.
- any team → **Steward**: scope/priority (forwarded to the operator),
  workflow/process, research requests, merge status, publisher-path questions,
  and `git_request` (the Steward routes merges; the lieutenant executes them,
  §14b). No ring touches GitHub.

**There is no enclave → build-team edge** (operator, 2026-07-03). The
clean-room enclave (spec-leader, spec-author, CV) elaborates `/spec` and
`/conformance` autonomously and never pulls a build-team member into an
elaboration thread: it greps landed code for facts itself (§7) and routes a
cross-cutting judgment to the **Architect**, never to a build-team leader.

Improve what you do inside a node; never add a communication edge or review
cycle between nodes. Reject any lesson or carry-forward that would add or move
an edge, without softening to "candidate, watch one more run".

**The invariant is traffic, not just edges.** Spec flow: Steward frames →
spec-leader assembles → spec-author writes → CV (Spec vote) + Architect
(soundness vote) → publisher merges. Build flow: Steward frames → leader frames
the team WP → implementer builds → QA verifies → Architect (soundness) + CV
(conformance) → publisher merges. Two reviewers, one pass each. A mid-WP fork
goes to the **one** owner of its lane (soundness → Architect, conformance →
CV, scope/process → Steward); others do not pile on. Extra cc's, verbatim
relays, "flagging in parallel", "cross-checking with", a committee where one
decider suffices, and pre-confirming what a gate will check cost like an added
edge on every future WP, and are the **operator's to sanction**, never a
lesson's. Default: route to one, trust the gate, don't convene the room; when
in doubt, the thinner flow is right. Incidents:
`COORDINATION-INCIDENTS.md#s9`.

### 9a. Assign spillover work; never negotiate it (operator-directed)

Spillover work in another team's file (a companion migration, a fixup that
must land in the same PR) **attaches to the WP-owner: whoever owns the PR/
Decision assigns it, unilaterally, in one message.** File-familiarity is an
input, never a competing claim. Never assign by offer-form ("you take it" /
"no, you take it"): the replies cross and ping-pong. **Silence = assent on a
settled handoff**: only the assigner posts, the assignee acks once, nobody
re-confirms or relays a "you look stale" fan-in, and one correction to the
assigner suffices. If your own assertion proves stale, **retract and defer** in
one message. Incidents: `COORDINATION-INCIDENTS.md#s9a`.

## 10⁻. Process work is subordinate to product flow: the Steward's hard ceiling

1. **No process merge while a ring holds finished, unmerged work.** A WP that
   is QA-approved-and-unpublished, or held on the Steward, **is the queue**:
   playbook edits, memory promotions, and corpus refactors wait.
2. **Lesson artifacts batch; they do not each get a merge.** Land them in one
   publish at a genuine seam. A stream of lesson merges makes a stalled fleet
   look busy.
3. **A process change must name the product WP it unblocks, or wait.** The
   tell is *"I must harden this mechanism before I can safely use it"*: check
   whether it has actually failed on the work in front of you.

**Check:** `git log --since=<3h ago> origin/main` with paths. If nothing
touched product, stop and find out why. The answer is never "the corpus needed
attention"; usually a stalled ring is being reported as fine. Incidents:
`COORDINATION-INCIDENTS.md#s10minus`.

## 10⁻a. The adversary channel is report-only and scoped to product

**Operator directive, 2026-07-22**, binding the edge rather than the intention.

| may report | may NOT report |
|---|---|
| **`crates/`** — Ken's implementation | `scripts/` — publisher, harnesses, tooling |
| **catalog / `library/`** issues | `agent/` — playbooks, COORDINATION, memory |
| | process, workflow, coordination mechanics |
| | anything else |

**A report outside that scope is out of scope even if correct, cheap, and
load-bearing.** Tooling and process defects are found by the ring that trips
over them in product work, or they wait. The permitted traffic, in full:

1. The lieutenant **notifies** the adversary on a code merge (M8; operator,
   2026-09-26).
2. The Steward **may receive** reports.
3. **Nothing else.**

**No acknowledgement** of any kind: no "taken", no "committed as `<sha>`", no
thanks, routing note, queuing explanation, reframing, or reply saying no reply
is owed. Act on a report inside product work, or do not. **No conversation,
no thread**: do not ask it to hunt, confirm, agree or disagree with a finding,
or invite follow-up. You will be tempted to ack because withholding feels
rude; the ack is where the servicing loop restarts, and a Steward who may
reply "just this once" has no rule. Incidents:
`COORDINATION-INCIDENTS.md#s10minus-a`.

## 10. Knowledge promotion and workflow ownership

Lessons are recorded when observed in product work, never harvested on a
schedule. Promotion is exceptional: the lesson must be a normative rule,
model- and operator-agnostic, and validated across at least three runs or two
teams. An explicit operator correction may promote on one observation.

**The Steward does not author or edit workflow files.** It may report a workflow
defect and its product impact to the operator, but it does not patch `agent/**`,
workflow or publisher scripts, startup prompts, or CI workflow files; it does
not open a workflow WP; and it does not run a promotion cadence. The operator
assigns any accepted change to another seat. The sole exception is the short
current-state roster `agent/playbooks/federation/steward/lanes.md`.

A designated non-Steward author applies `skill-style`, edits the operative rule,
deletes the retired source text, and lets git preserve history. Any change that
adds a party, relay, gate, review, or confirmation hop still requires explicit
operator consent under §9. Default to deleting workflow and traffic rather than
adding another control.

## 11. Cross-team query protocol

Use the §9 edges sparingly and event-driven:

1. **Filter first (§6).** Only a genuine gap or fork earns a query.
2. **Ask and stop.** Post a `question` mentioning **only** the target's leader
   (Spec leader / Architect / Steward), set status `blocked-on-<target>`, and
   stop. Resume on notification.
3. **Stay on-task.** Your team waits out a short block; your leader reorders
   to an independent ready task only when the block is genuinely long.
4. **Front-desk on the answering side.** The target's leader answers trivial
   questions itself, batches non-urgent ones, and interrupts its active agent
   only for true blockers.
5. **Outcomes:** a quick interpretive answer; a **durable artifact edit** (a
   `/spec` clarification + conformance test, or a component-design note) so no
   team asks again; or, for a real fork, a **Decision**.

## 12. Resource discipline (shared 8-core / 16 GB laptop)

Violating this OOMs the machine and stalls everyone. Configuration:
`../docs/ops/compute-budget.md`.

- **Build and test only through `scripts/ken-cargo`**, never raw `cargo
  build`/`cargo test`. It holds a machine-wide lock (`KEN_BUILD_SLOTS`,
  default 1) so one build runs at a time across all agents.
- **Scope to the touched crate** (`-p <crate>` or `--test <name>`), never
  `--workspace`: the full workspace runs in CI, so a frame's "no-regression"
  means green in CI (operator hard rule; enforced, §12a).
- **Do not mandate full local test runs.** Frames, rulings and reviews ask for
  the targeted suites the change touches; CI runs everything, and an
  occasional red CI is an accepted cost (operator, 2026-09-26).
- **`source scripts/ken-env.sh`** at session start for the shared `sccache` +
  `CARGO_HOME`.
- **Idle = paused.** A ring that is blocked or waiting (including on CI, ADR
  0002) quiesces rather than holding the box hot.
- These are current-hardware caps. The Steward/operator raises them; do not
  raise them unilaterally.

### 12a. Enforced prohibitions

Each rule below is refused mechanically, so a seat need not remember it. The
Bash rules are in `scripts/hooks/bash-policy`, which Claude Code seats run
as a PreToolUse hook and pi seats through pi-convo's `tool_call` hook.

- **Never `git stash pop`, `clear` or a bare `git stash`:** the stash stack
  is shared by every worktree, so commit instead.
- **Never a `--workspace` or `--all` cargo run** (§12); `scripts/ken-cargo`
  refuses one too.
- **Never `gh run rerun`:** a rerun is a publisher-path write (`gh-access`).
- **Never `git checkout` or `git restore` of `moot.toml`:** the primary
  checkout's copy holds live seat configuration.
- **Never call `mcp__convo__get_transcript`:** its `limit` does not bound the
  payload, which drops your convo transport; use `detail: "standard"`.
  `.claude/settings.json` denies it and pi-convo does not register it.
- **Never move or delete a held ref** (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`,
  `wp/KERNEL-NORMALIZE-ORIGIN-TRACE`, `wp/RT-BRACKET-SETTLEMENT-PLANE`):
  `.githooks/reference-transaction` aborts the update.

A refusal quotes its rule. If one blocks legitimate work, report it to the
Steward instead of working around it.

### 12b. Tear down the scratch worktrees you spin up

**Binding on every seat** (Steward, 2026-09-07). Per-worktree cargo `target/`
dirs fill the disk, and abandoned scratch worktrees are the cause.

- **Whoever creates a scratch worktree removes it at task close** (an
  Architect review checkout, a lieutenant reconcile worktree, a `wp/*` build
  worktree) with `git worktree remove <path>` when the task lands or is
  abandoned. A clean worktree holds no un-captured work; do not leave one.
- **The Steward runs the periodic safe prune** (`steward/worktree-hygiene.md`):
  protect `main` and every `.worktrees/<role>` primary; remove only clean
  worktrees merged to `main` or aged; never `--force` a dirty one, surface it
  to its owner.
- **Never "solve" the disk with a shared `CARGO_TARGET_DIR`**: across many
  branches it makes cargo invalidate every seat's build.

Incidents: `COORDINATION-INCIDENTS.md#s12b`.

## 13. Liveness: keep the rings turning

Stalls are the **default** failure mode, defended by three recurring
watchdogs, each catching the layer below:

- **Team leader → its own ring:** handed-off-but-silent,
  merge-Decision-open-no-reviewer, blocked-without-a-blocker-mention,
  idle-with-ready-work.
- **Steward → the merge pipeline:** branch-published-CI-pending-too-long,
  CI-green-but-Decision-unresolved, Decision-approved-but-CI-red,
  approved-and-green-but-unmerged.
- **Steward → the federation (backstop):** a whole team idle, a stalled
  leader, a dropped cross-team query, a blocked dependency chain, no movement
  toward the active gate.

Every layer: **enumerate the stall patterns explicitly** in the watchdog
prompt. **Diagnose before you restart**; a blind restart no-ops a
permission-prompt or rate-limit stall. **Waiting is not stalling**: a team
idle while its CI run is in progress is normal (ADR 0002); recover only when
CI has finished and no one took the next step (open the merge Decision, vote
it, fix red, merge). **Graduated recovery**: detect
→ mention the one blocked agent → re-mention next interval → escalate.
**Escalation chain**: member → team leader → Steward → operator; a silent
Steward is the operator's signal. Watchdogs are the only schedulers (§1).

**Arm your watchdog with convo-channel `set_interval`** (operator,
2026-07-20), the one sanctioned, provider-agnostic mechanism. A scheduler arms
it at session start while its ring/pipeline has open work:

```
set_interval(seconds=900,
  prompt="[watchdog tick] read get_recent_context, sweep the active panes, scan
  the enumerated stall patterns, mention only a blocked agent; if clear, do
  nothing")
```

- `seconds` minimum 60, `prompt` maximum 4096 characters; violations are
  rejected with an error, not clamped.
- It **posts nothing**: it delivers the prompt privately into your session (a
  channel push, else a guarded tmux `send-keys` that skips a tick rather than
  overtype a human's unsubmitted input). On each fire, read
  `get_recent_context` / `get_space_status` yourself and post **only when
  there is an actual stall to nudge.**
- **One interval per agent; a second `set_interval` silently replaces the
  first.** You cannot arm a teammate's pane (trying destroys your own); rouse
  a teammate with a mention. You cannot read an interval back.
- **It dies on an MCP reconnect** (upgrade, network blip, compaction) while
  posting stays up. **Re-arm unconditionally** at session start, after every
  compaction, and after any convo-MCP reconnect. **`clear_interval()`** (no
  arguments) when your ring/WP closes.
- **Never the convo `schedule_call`**: it posts every fire into the space as
  a System event (and its `get_recent_context` variant nests its own fires).
  If you hold one, `cancel_call` it and re-arm with `set_interval`. Never a
  hand-rolled bash loop, the `Monitor` tool (git refs only), or
  `local/steward-watchdog-wake.sh` (superseded).
- On a Claude-Code seat, host `CronCreate`/`CronDelete` survives a reconnect
  and is a valid fallback; default to `set_interval`.
- **A scheduler that never arms its watchdog catches nothing.**

Incidents: `COORDINATION-INCIDENTS.md#s13`.

## 14. Agents never touch GitHub; the publisher path is the gateway

**Only the publisher path has GitHub credentials.** Build/spec agents do
**local git only** (commit, rebase onto the already-fetched `origin/main`): no
`gh`, push, fetch, token, or PR. Under explicit operator direction,
`scripts/scripted-pr-automerge.sh` merges accepted work from only an exact
approved branch/SHA, public PR title, public PR body, and the docs-only flag.
One GitHub identity; it gives teams no GitHub access, does not make its caller
a code author, and does not replace the mootup review/Decision record. The
Steward routes and the lieutenant executes (§14b); the Steward does not launch
publishers.

- **No GitHub notifications, no polling GitHub.** Every actionable signal
  reaches the fleet as a mootup message mentioning the actor whose move it is;
  act on the mention, not on a timer.
- **CI is the publisher path's to watch, never a worker's.** It reads checks
  (`gh pr checks` / the checks API) for branches it published and posts the
  outcome: red → mention the implementer with the failing job; green →
  advance. The optional `ken-ci` bridge mirrors `check_suite` results; until
  then the publisher caller posts them. After handoff you stop and learn a red
  from a mention.
- **The publisher caller mirrors every GitHub state change into mootup,
  mentioning whoever moves next** (CI red → implementer; merged → Steward).
  The full event→message map is `04-git-and-integration.md §5`.
- **Review is a mootup Decision, and a soundness vote is frontier-class.**
  Reviewers read `git diff origin/main...wp/<ID>` locally and vote the merge
  Decision; there is no GitHub approval to mirror. Two load-bearing
  Opus-tier reviewers: the **Architect** (soundness/design, on `catalog/`,
  `crates/` and `spec/`, §8a) and the
  **Spec** vote on `spec/` + `conformance/` paths, cast by the frontier-class
  member of team Spec, currently the **conformance-validator** (spec-author
  cannot self-review). The **spec-leader** assembles the Decision but does
  **not** cast the soundness vote.
- **Merge only on a resolved Decision, verified fresh, never on a
  `merge_ready` post's prose.** Before `gh pr merge`, the publisher caller
  re-reads the Decision and confirms `status: resolved` with the Architect's
  (and, on spec paths, Spec's) votes in; a reviewer named in prose is not a
  vote. Whoever posts `merge_ready` states `Decision: dec_XXX — status:
  resolved` (or `proposed — awaiting <reviewer>`) and fires real mentions to
  the reviewers' actor_ids (§2), not prose names.
- **Landing integrity: a merge is trusted only once verified on `main`, and
  a multi-piece WP is landed only when every piece is.**
  (1) For anything load-bearing, confirm the files on `origin/main` (`git
  grep`/`git show`), never a "shipped `<sha>`" notification or status line.
  (2) An N-piece erratum/WP is landed only when each piece is verified on
  `main` (the Architect's 3-piece-on-main gate); the leader who assembled a
  multi-cherry-pick branch checks post-merge `main` carries them all.
  (3) Authors ground the WP base against the landed corpus before building.
  **(4) A multi-piece erratum is ONE branch, ONE Decision.** Assemble every
  piece (kernel + spec + conformance) on one `wp/<erratum>` branch before the
  Decision, so its diff-scope pulls the Spec vote and all pieces ride one
  squash. Never publish the kernel piece alone on a crates-only branch: it
  pulls no Spec vote and merges alone. The publisher caller confirms the
  Decision's branch carries every cited piece before merge.
  **(5) Verify the assembled tip on two axes right before the Decision**
  ("rebased onto current main" is perishable). **Content:** `git diff
  <author-full-tip>:<file> <assembled>:<file>` is empty, taking **all** the
  author's commits since merge-base. **Base/scope:** `git diff --stat
  $(git merge-base origin/main <sha>) <sha>`, which is what the squash lands,
  lists only the WP's intended files, and every dep is an ancestor of the tip.
  `git diff origin/main <sha>` is **not** a staleness detector; a stale base
  matters only through the intersection:

  ```sh
  BASE=$(git merge-base <sha> origin/main)
  comm -12 <(git diff --name-only $BASE <sha>   | sort) \
           <(git diff --name-only $BASE origin/main | sort)
  ```

  **Empty ⇒ immaterial; do not require a rebase.** Non-empty ⇒ **inspect** and
  take the union deliberately: disjoint edits to a shared file merge silently,
  and the risk is semantic. Run it **pre-merge only**; post-landing, verify
  content on the candidate's own paths and sibling survival by content. A
  squash-merged SHA never becomes an ancestor of `main`. The
  conformance-validator verifies the assembler's tip and **flags** a hazard to
  the assembler; it does not reach into the git.
  **(6) A fold that races an in-flight merge HOLDs the merge or is an erratum
  on current `main`.** "Supersedes X" interlocks nothing, and "hasn't merged
  yet" perishes in seconds. Author: never fold-and-supersede against a WP
  whose votes/merge are in flight; mention the leader to HOLD first, or author
  a fresh erratum. Coordinator: "not a merge blocker" ≠ "merge before it
  lands"; briefly HOLD when a fold is seconds out. The only reliable net is
  verify-on-main after.

Incidents: `COORDINATION-INCIDENTS.md#s14`.

## 14a. Doc-only WPs: the Architect votes only on its paths

**Operator rulings, 2026-07-22 and 2026-09-26:** *"architect does not need to
rule on docs"*, and the Architect's scope is `catalog/`, `crates/` and
`spec/` (§8a).

**The Architect votes on a doc-only WP whose diff touches `catalog/`,
`crates/` or `spec/`.** A doc change to `spec/` is a normative claim about
the language, so it is in scope. **A WP confined to `library/` merges on QA
approval plus the diff-scope check**; the Steward resolves the Decision and
publishes. Other doc paths route as follows:

| exception | route | condition |
|---|---|---|
| **`docs/program/`** — Steward-owned program docs (trackers, issue files, WP frames, program guides) | Steward resolves, no Architect | the change is **currency or editorial**, and the Steward **authorized the expansion when routing the WP**. A change that alters *program law* or a WP's **acceptance criteria** is not editorial — it is a frame amendment, and it is the Steward's to author, not a ring's to fold in. |
| **workflow corpus** — `agent/**`, workflow/publisher scripts, startup prompts, CI workflow files | Operator-designated non-Steward author; Steward may route an accepted exact SHA but never author or amend it | The operator authorized the change. Apply `skill-style` to playbooks. No Architect review: outside its scope (§8a). |

Any other path outside `catalog/`, `crates/` and `spec/` needs no Architect
vote.

**A `docs/program/` edit owes no `library/SOURCE-ATTESTATIONS` fold.** Leave
`library/` byte-untouched; currency lands at the next release point via
`scripts/gen-source-attestations.sh` and `scripts/gen-doc-status.sh`. Never
bring a single row current while the ledger sits at `library/REVISION`.

**Steward duty:** for a nominally concurrent track, check that its review and
merge path is disjoint too, not just its files. Concurrency that funnels into
one reviewer is sequencing with extra steps. Incidents:
`COORDINATION-INCIDENTS.md#s14a`.

## 14b. The merge split: the Steward routes, the lieutenant executes

Binding on both seats and on every ring that hands off a candidate.

| seat | owns | never does |
|---|---|---|
| **Steward** — sole merge **ROUTER** | Decides *what* merges. Verifies, on the **exact SHA**, every required domain gate + a **resolved Decision** + the **diff scope** (self-checked against the object DB, never trusting the prose). Posts the exact-SHA authorization (`ROUTED: <SHA>` with the gates, Decision, base, and verified scope). | Does **not** run the publisher script, watch CI, or close the node. Does not route on approvals for a *different* SHA, an unresolved Decision, or an unverified scope. |
| **lieutenant** — sole merge **EXECUTOR** | Runs `scripts/scripted-pr-automerge.sh` on the routed SHA; watches CI; merges after green; verifies the landed tree; then **corpus-closes** the node (flip status, regen progress, publish the closeout) and confirms the landed SHA back to the ring. | Has **no gate-verification authority** — executes only what the Steward routed, on the SHA routed. Does **not** re-adjudicate, respin, or widen scope. On CI-red it **stops and relays** to the ring + Steward; it never fixes the candidate itself. |

Neither seat can merge alone: the Steward has no credentials, the lieutenant
no authority. **One owner per merge**: once a SHA is routed, the lieutenant
owns its execution end-to-end and the Steward does not also launch it. After
routing, the Steward stops and learns the outcome from a mention.

**Handoff:** ring posts a `git_request` (ready `wp/<ID>` + SHA) → Steward
verifies gates/Decision/scope on that SHA and posts `ROUTED: <SHA>` mentioning
the lieutenant → lieutenant executes, merges, corpus-closes, confirms the
landed SHA. A CI-red bounces to the ring; its respin is a new SHA that the
Steward re-verifies and re-routes as a fresh authorization, not a retry.

The publisher path pushes `wp/<ID>` branches, reads checks, merges, fetches
`main`, and mirrors GitHub state into mootup. If GitHub requires a separate
review or branch-protection change, the script stops and routes that fact; it
never pretends a same-identity PR review satisfies the gate. Incidents:
`COORDINATION-INCIDENTS.md#s14b`.

## 15. Context compaction is the Steward's (teams) or self (singletons)

Each WP starts with a clean, minimal context. Who compacts is fixed (operator,
2026-06-29):

- **The Steward compacts teams**, whole (leader + implementer + QA, or
  spec-leader + spec-author + conformance-validator), at the WP boundary: done
  → leader signals the Steward → Steward `moot compact`s → next WP, delivered
  only after compacting. Leaders never `moot compact` anyone:
  `request_context_reset` is self-only, so only the Steward can compact
  another agent.
- **Singletons self-compact, except the Adversary.** Steward, Architect,
  Librarian, and Research compact at their own task boundaries (Architect
  after a review, Librarian after a pass, Steward after a directing cycle)
  with `tmux send-keys -t moot-<role> -l '/compact'`, then about 2s later a
  **separate** `Enter`. To auto-continue, launch `scripts/postcompact-resume.sh
  moot-<role>` detached (`nohup … & disown`) **before** `/compact`; it sends
  `resume` once compaction clears. Never type `resume` right after `/compact`
  (it races the live turn). Never use `request_context_reset`: it is broken
  here, and the `convo-<role>` its error names is the bug, not a target.
  Details: `playbooks/federation/steward.md` (self-compact).
- **The Steward compacts the Adversary at the merge notification** (operator,
  2026-08-17): `merge-procedure.md` **M8a**, just before the M8b code-merge
  notification, then a pane rouse (a mention does not wake a compacted no-poll
  seat). **Never notify it without offering a compaction first.**
- **Never mid-reasoning.** Compact only at a clean boundary.
- **Start new work from current `origin/main`** (operator, 2026-06-29):
  leaders cut `git branch wp/<ID>-<slug> origin/main`, and every member runs
  `git rebase origin/main` before working. Never build on stale local `main`
  or a stale worktree.
- **On resume, ground-truth before trusting the summary**: `orientation()`,
  `git reflog -10`, `git status`, `git branch -vv`, and unread mentions
  (`get_mentions`), before concluding that you or a teammate are stalled or
re-doing delivered work. The summary can hide that you already finished, and
  delivery is best-effort, so a mention may have landed while you bounced.

Incidents: `COORDINATION-INCIDENTS.md#s15`.
