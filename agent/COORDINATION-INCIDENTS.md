# Coordination incidents (reference for `COORDINATION.md`)

This file is reference, not startup reading: open a section only when you need
to judge whether a `COORDINATION.md` rule applies. It holds the dated
incidents, measurements, and worked examples behind the law, keyed by the law's
section id. The rule itself is always the one in `COORDINATION.md`; nothing
here is law.

<a id="s1a"></a>

## §1a. Named events: the three-way circular wait

**Measured, 2026-07-22: a three-way circular wait that no participant could
see, and that no watchdog would ever have broken.**

| seat | believed it was waiting for | actually |
|---|---|---|
| `verify-leader` | the implementer to pick up a new kickoff | idle |
| `verify-implementer` | *the leader's merge Decision* on work already finished | idle |
| `verify-qa` | nothing — verdict delivered, handed back | idle |

Every seat was correctly event-driven by the letter of §1. The events they
were each waiting for were each other. A QA-approved WP sat with no Decision
opened, and the ring would have held indefinitely: nothing was going to
arrive. *"Handed back to X"* is what created the deadlock: it says who
received the work, not what they owe.

**Why the backstop could not be the answer:** the Steward's liveness sweep read
all three seats as BUSY for hours. A completed turn holding an orphaned shell
reads identical to a running one unless the detector is built to tell them
apart. The ring knowing what it waits for is the primary mechanism; the
watchdog is insurance, and insurance failed.

<a id="s2"></a>

## §2. Mention discipline

- **Ack-with-mention crosstalk** (operator, re-tightened 2026-07-03, second
  pass): "@X acknowledged / noted / tracked for later / standing by" fires a
  notification for zero required action. Every mention costs the recipient
  tokens.
- **Launch instead of mention.** DeepSeek leaders mis-read "hand the WP to
  your implementer" as a launch instead of a mention, spawning an
  unconfigured Claude that failed "503 provider not configured".
- **`actors.json` leaks.** Both leaks happened during schema discovery, not
  during the lookup: once by `sed`/`grep` over the raw file, once when a
  field-projecting one-liner returned nothing (the author had guessed the key
  names) and the author dumped the file "just to see the structure", printing
  three records verbatim. A rule saying "project only the fields you need"
  does not bind at the moment that leaks, because you cannot write a
  projection until you know the field names. `scripts/moot-actor-id.sh`
  projects `actor_id` by name, enforces an output whitelist (nothing but
  `<role> agt_<id>` can leave it), and fails closed.
- **Answers with an empty `mentions` array.** Observed 2026-07-11: three
  Architect design rulings that answered the Steward's questions each posted
  with an empty `mentions` array and no in-text address. They were invisible
  to the asker's `get_mentions`, surfaced only by manual thread-polling, so
  each was a latent silent stall.
- **Dead placeholder routing** (promoted B1, spec-leader's catch). The Sec1ct
  §14 breach: "Spec review" was routed to the dead `Spec` template
  placeholder, so the gate never ran.
- **`reply_to` on a kickoff** (L5): an implementer lost a round-trip when
  `reply_to` on the kickoff 404'd.
- **Threading history.** Operator 2026-06-29; hardened 2026-07-03; restated
  2026-08-01 with the closed top-level list (§4).

<a id="s4a"></a>

## §4a. Threads: the anchor mechanism

**Measured 2026-08-01, and it defeats the obvious reading of the rule.**
`post_response` with no parent returns `thread_id: null`, because the thread is
minted by the first reply, not by the root; the root event is then pulled into
that thread retroactively. So the corollary "every kick carries a thread id"
cannot be implemented as stated.

**`reply_to` auto-mentions the original speaker.** On 2026-08-01 the Runtime
ring deadlocked because an Architect verdict mentioned only the implementer and
not the leader who opened the Decision. Replying into the thread makes the
wake path structural instead of a habit someone must remember.

<a id="s4c"></a>

## §4c. A merged WP's thread is closed

**Measured 2026-08-01, one hour after this section landed.** Runtime QA posted
**`QA APPROVED — Slice 2 exact ed527eb7…`** into **slice 1's** thread. Slice 1
was merged and closed; slice 2 had its own kick and its own thread, and the
leader and implementer were both already posting there correctly.

**The cost is not untidiness; it is a hole at the point of inspection.** Anyone
reconstructing slice 2 from its own thread saw handoff → review request →
*nothing* → Architect request. The approval was invisible exactly where a
reader checks whether one exists, while a closed WP's thread carried a live
verdict about a different candidate.

**What did not cause it: no compaction, no stale id, no rescope.** The seat had
been living in that thread all afternoon through four review rounds, and
posted where it had been posting.

<a id="s4d"></a>

## §4d. Why top-level is a closed list

The closed list of 2026-08-01 superseded the Steward-only rule of 2026-07-03.
The three roles are exactly those whose output is not scoped to one WP: the
Steward coordinates across all of them, the Librarian holds a standing
as-built mandate over the whole corpus, and research reports are corpus-wide.

The rationale is structural: the Steward coordinates all activity, so any work
another agent does traces back, by origin, to a message the Steward sent.
Enforcing it makes the space a clean tree rooted at coordination posts and ends
the fragmentation where parallel root posts scatter one WP's exchange.

Boundary posts (operator, 2026-06-29) keep the flow legible in the web view and
preserve the interaction history; without them there is a silent gap between
assignment and completion that no one can audit or replay. The ack rule
(operator, 2026-07-03) refined an earlier "no bare acks" rule: the problem was
never the acknowledgment, but that a mention fires a notification and obligates
the recipient to spend tokens deciding whether to respond. The single
commonest cause of a days-old thread being revived is a stale id carried
through a compaction.

<a id="s7a"></a>

## §7a. The permission that already existed

**Measured, 2026-07-22.** A finished, QA-approved WP was declared undeliverable
by **two seats independently**, one citing a fleet memory note, one citing
`docs/ops/runbook-gh-identities.md:109`, both saying the publisher App lacked
`Workflows` permission. **The operator had granted it the day before.** The
Steward escalated and asked for a permission that already existed.

The disconfirming evidence was in the repository the entire time: three
commits under `.github/workflows/` authored by that very App.

`verify-leader` found the three commits, held both facts side by side, and
wrote *"I can't tell which; flagging that I don't know rather than guessing."*
That is what got this corrected. The Steward saw the same evidence and
explained it away as "probably operator-pushed", reconciling the anomaly to
the record instead of testing the record against the anomaly. Nothing goes red
when such a note goes stale, and a silently corrected doc teaches the next
reader nothing.

<a id="s7b"></a>

## §7b. Defaulted operands, representative cases, degenerate pairs

- **Defaulted operands**, 2026-07-22, three instances: an operand chosen
  without checking which point in the graph it sat at (`dd715950^`, a point on
  `main`'s history, used to answer a question about a PR's base); a precedent
  asserted from a branch name rather than its diff; a claim about a file cited
  from another seat's characterization rather than by opening the line. Each
  returned a confident, wrong answer rather than an obviously broken one.
- **Representative case** (promoted from F4 + K1). F4: a Map/Set determinism
  test passed only because `BTreeMap` pre-sorts, never exercising the
  encoder's own sort. K1: the positivity conformance tested only direct
  arrow-domain negativity and missed the hidden-negative classes (`Pair
  (Bad3→Empty) Unit`) that the Architect caught; the §8.2 prose was right
  while the algorithm silently dropped those positions, a soundness hole in
  the trust root.
- **Non-degenerate pairs** (promoted Sec1/Sec1ct/V3/B1, at least four
  domains, build and spec enclave). Sec1ct: a `ct⊤` value rejects while a
  `Secret`-not-`@ct` value accepts on the same sink shape. B1: a `proved`
  claim maps to `Q` while a hole maps to `P` on the same postcondition.
- **Exhaustive by construction** (promoted across the V-spine + X1-effects;
  verify-team-requested). The constructive form of the two-soundnesses
  omission backstop: a silently skipped case supplies nothing for a downstream
  checker to catch. Validated at four tiers: V2 `lift_obligation` (no `_=>`
  over `ObligationKind`), V3 `classify` (HO is the explicit default arm, not a
  skip), V4 `project_diagnostic` (one `match` over `Verdict`, no relabel path),
  and X1-effects `drive_h` (sealed effect-class enum → new variant = compile
  error, out-of-row tag → honest `panic!`).

<a id="s8a"></a>

## §8a. Serial review over disjoint domains; an unreleasable prefix

**SRC-ATTEST, 2026-07-22.** The WP was gated *Librarian QA → Architect*, in
series. Three Architect rejections, each landing in `scripts/`, each sent the
candidate back through a full Librarian re-review of `library/` that no fold
had touched. Every review found something real; the serialization was not
catching bugs, it was re-asking a question already answered while the fleet's
top priority sat blocked. Roughly an hour, entirely in the handoff.

**Partial-WP merge** (operator, 2026-08-06): *"From now on, merge in accepted
work once it is done, even if it is only a partial WP."* A leader holding
finished work for its unfinished siblings is what this retired.

**Measured 2026-08-06:** a complete, fully reviewed 34-commit prefix of
`RT-DECL-CLOSURE-PORT` failed eight CI checks against a green `main`.
Checkpoint approvals bind an exact SHA for a deliverable's own claim; none
asserts the tree is green there, because nobody asked. The node-id boundary is
bookkeeping; greenness is semantic, and the two need not coincide.

<a id="s9"></a>

## §9. Enclave crossing into the build lane

**Map elaboration:** spec-author mentioned runtime-leader to confirm that a
heap-tag (`0x07`/`0x08`) retirement was inert. That was a grep-able fact
(`canonical.rs:23-24`, zero refs), and the coupled soundness call was the
Architect's. Self-ground or route to the Architect; don't cross into the build
lane.

Coordination entropy creeps in below the edge level: none of the traffic forms
adds an edge, so an edge filter misses them, but each multiplies tokens on
every future WP, and the enclave is serial, so the cost compounds.

<a id="s9a"></a>

## §9a. Offer-form ping-pong

A companion migration once flipped ownership four times in one minute under
offer-form assignment between two leaders, and the pattern has recurred. A
fixed assigner has no cross-wire: one message settles it. A clean withdrawal
ends a ping-pong; a second asserted correction extends it.

<a id="s10minus"></a>

## §10⁻. Two and a half hours of process merges

**Measured, 2026-07-22.** Between 17:10 and 19:45, every one of eleven merges
to `main` was process: five memory files, three publisher-script changes, one
playbook, one frame, one tracker sync, plus a `library/REVISION` bump that
existed only to repair `main` after one of those merges reddened it. **Zero
touched `crates/`, `spec/`, or `conformance/`.** Meanwhile a QA-approved verify
WP sat undelivered and a QA-clean runtime WP sat held. The operator noticed
from outside and asked what was going on.

**Nothing external was pulling that work; the chain was self-sustaining.** A
publisher bug produced a playbook lesson, which produced a memory promotion,
which produced an adversary finding, which produced a frame, which produced an
Architect escalation. Every link was individually correct and defensible,
which is why it ran unchecked: there was no step at which the right local
decision was to stop. The Steward was both producer and consumer, so the loop
had no external brake.

A finished WP that cannot land is the most expensive object in the federation:
it has consumed a full ring's build, review, and QA and returns nothing. From
the outside, an active merge stream is indistinguishable from progress.

<a id="s10minus-a"></a>

## §10⁻a. Why the adversary edge is structural

§10⁻ named the failure (a high-quality finding stream plus a Steward who
services it immediately crowds out product) but left the Steward to police
themselves, which is the one thing the failure mode guarantees will not work.
The operator's directive of 2026-07-22 binds the edge instead. The ack is
banned specifically because it costs a message, invites a reply, and converts
a report into a thread, and every step is individually courteous.

<a id="s12b"></a>

## §12b. The disk at 99%

On 2026-09-07 the devcontainer disk hit 99% (2.7G free). Per-worktree cargo
`target/` dirs run 17–31G for an active build seat, but the cause was not any
one target: it was scratch, review, and reconcile worktrees created for a task
and never removed, measured at **100 worktrees, about 60 of them abandoned**,
before a prune back to 40. A clean worktree's branch and commits live in the
shared `.git`, so it can be re-added on demand. A shared `CARGO_TARGET_DIR`
across many branches causes cross-branch cache thrash; per-worktree targets
isolate that on purpose. Worktrees look isolated and their disk footprint is
not: it is one volume.

<a id="s13"></a>

## §13. Watchdog mechanism history

`set_interval` superseded the earlier `CronCreate`-only guidance (operator,
2026-07-20). `local/steward-watchdog-wake.sh` was the terra-seat stopgap before
`set_interval` existed. The convo `schedule_call` `get_recent_context` variant
reads its own prior fires and recursively nests them, an exponential
self-feeding loop the Architect and runtime-leader caught. A leader that tried
to arm a member's watchdog would destroy its own and believe it had armed two.
The operator caught a scheduler that never armed its watchdog: a QA-approved
WP left unmerged because the leader was not watching.

<a id="s14"></a>

## §14. Landing integrity incidents

- **(1)–(3), promoted V1, near-miss.** The K2c-s2 seam-3 erratum shipped its
  kernel fix in `ecbb279` but dropped the spec (`da344a6`) and conformance
  (`f3ece75`) pieces, leaving `16 §5.1` normatively contradicting its own
  kernel and the corpus guarding nothing. It survived the ship, invisible in
  every "shipped" status and notification, and was caught only by grounding
  against the landed files.
- **(4), promoted Σ-sort, the second recurrence.** The drop recurred
  immediately on the Σ-sort 3-piece: `badc78d` shipped only the kernel
  `sort_sigma` split; spec `13` and the pi-sigma conformance were absent from
  `main`. Root cause: a crates-only kernel piece's diff-scope pulls Architect +
  CI and no Spec vote, so it merges alone; the pieces were never assembled
  onto the branch that merged. Validated on `s51-sigma-reland`: one branch,
  both `spec/` + `conformance/` pieces, the Decision correctly pulled a Spec
  vote.
- **(5), promoted X1-effects-elab; both enclave authors and the Architect
  converged on it.** Two faults nearly merged a wrong tree: (a) the assembler
  took the author's §6-body commit but not their branch tip, dropping a
  follow-up (`a3b887e`) that fixed an actively wrong stale pointer; (b) the
  re-fold sat on a stale base.
- **(5) correction, 2026-07-22, Steward.** A former parenthetical said "a stale
  base silently reverts unrelated landed siblings", and the base/scope check
  was `git diff --stat origin/main <assembled>`. That was false: this repo
  squash-merges, and a squash applies merge-base → branch, never main →
  branch. A candidate on a stale base lacking two chapters merged, and both
  chapters survived. `git diff origin/main <sha>` fires identically on safe
  and unsafe candidates, so it cannot discriminate. Overlapping hunks conflict
  loudly (a Steward tracker branch with a four-file intersection was refused,
  `Pull Request has merge conflicts`, and reconciled by taking the union), but
  disjoint edits to a shared file merge silently as a union (measured: branch
  edits line 10, `main` edits line 90, `git merge-tree` merges with no
  conflict). The residual risk is semantic, e.g. a status table reading
  `active` in one row and `merged` in another. That is why the rule is
  inspect, not rebase, and "the failure is loud" is not the reason. Run
  post-merge, the test self-fires on every candidate, because `main` then
  contains its squash. Cost of the false version: on 2026-07-22 a 7-commit
  candidate re-anchored twice in about 25 minutes against a contention-free
  doc track, each cycle a full re-verify plus a second QA pass, for an
  intersection that was empty every time.
- **Lane note.** The X1-effects-elab drops happened in the git; the
  conformance-validator is stronger verifying the assembler's tip than
  performing the rebase itself.
- **(6), promoted Sec1; a race, not an assembly fault.** The Architect
  approved `61` with an N1/N2 honesty fold flagged "fold before build, not a
  merge-blocker." The author committed the fold declaring it "supersedes
  `b3e7989`"; the coordinator had already rebased the pre-fold tip and posted
  merge_ready; the publisher path squashed the pre-fold spec to `main` 9
  seconds before the fold landed. The honesty chapter hit `main` carrying the
  exact §9 over-claim the fold fixed; it was caught only by verify-on-main and
  re-landed in minutes.
- **Resolved-Decision gate** (promoted Sec1ct breach; fixed and validated
  2/2). The Sec1ct merge read "`(Architect + Spec)`" in prose as approval while
  the Architect never voted.
- **Soundness vote routing** (promoted Sec1ct/B1 + operator). The spec-leader
  was a DeepSeek coordinator; soundness judgment needs the strongest model
  (MODELS.md tiering, made explicit for review routing).

<a id="s14a"></a>

## §14a. Doc-only WPs and the Architect

**The escalation predicate used to be a list.** The rule read *"…or reaches
outside `library/` into `crates/`, `spec/`, `conformance/`, or `catalog/`."*
That enumeration silently defaulted to no-Architect for any path absent from
it. It fired live on PR #959 (2026-07-25): a doc-ring candidate touched
`docs/program/07-catalog-style-guide.md`, outside `library/` and in none of the
four buckets. A taxonomy with no cell for the actual case reads as complete, so
the routing looked settled when it was undefined. The Steward ruled it
in-route on substance and recorded the gap; the fail-closed predicate is the
repair.

**The retired ledger rider (2026-07-25 to 2026-08-02).** It said some
`docs/program/` files are attested `library/` sources (`07-catalog-style-
guide.md`, `12-documentation-program.md`, and the issue files `CAT-CAPEX.md`,
`DOC-W1.md`, `DOC-W2.md`); that editing one moves its blob OID and reddens the
currency gate; so such a change owes the `library/SOURCE-ATTESTATIONS` fold in
the same candidate. There is no currency gate: `LIB-GATE-DECOUPLE`
(`f84e4804`, merged 2026-07-26) removed live documentation/content CI
coupling, and the policy accepts that source attestations drift between
release points (`docs/program/12-documentation-program.md:635-641`). Measured
2026-08-02 at `91e34ab0`: `library/SOURCE-ATTESTATIONS` pinned
`12-documentation-program.md` at blob `5aed2550` while its blob was
`c27131c0`, and nothing reported it; `library/REVISION` lags by construction.
The rider stood for a week claiming a gate that did not exist: an obligation
that outlived its mechanism, phrased in the present tense, in law every seat
reads. Bringing one row current makes the ledger internally inconsistent and
reads as though the gate were live again.

**Why doc-only merges skip the Architect.** The doc track runs concurrently
with the build track because it is path-disjoint (`library/` and `agent/`
versus `crates/`). Routing every doc WP's Decision through the Architect
re-couples the tracks at the review seat: the paths do not contend; the
reviewer does. DOC-W1-1 was QA-approved 34 minutes after kickoff and then sat
unpublished behind an Architect working through seven consecutive build-track
frame reviews. The operator noticed the absence of `library/` commits on
`main` before the Steward escalated: a queue is invisible from the outside,
and "the ring is running" and "work is landing" are different claims.

<a id="s14b"></a>

## §14b. The double-publish race

The split was added when the lieutenant seat was introduced. **Measured
2026-08-23:** the Steward launched a publisher for a kernel PR while the
lieutenant was already executing the same merge, a double-publish race caught
in the pre-lock wait.

<a id="s15"></a>

## §15. Compaction incidents

- **The Adversary** (operator, 2026-08-17): *"There is nothing that compacts
  adversary and the adversary does not self-compact."* §15 used to list it
  with the self-compacting singletons, and that produced no compaction at all:
  the seat had no work boundary anyone else could see, because an idle
  event-driven seat drops out of every active-agent enumeration.
- **Resume watcher.** A self-compact otherwise leaves the seat idle at `❯`
  with nothing to re-invoke it. Typing `resume` right after `/compact` and
  relying on host buffering can fire it before compaction, which is why
  `scripts/postcompact-resume.sh` exists.
- **Resume grounding** (F4 + K1, operator-observed). Promoted to §15 because
  the Steward compacts every team at every WP boundary, so post-compact resume
  is constant for every team member; a mention that landed during a fleet
  restart is recorded but may never have woken the seat.
- **Stale base.** Building on stale local `main` or a stale worktree is the
  §1 / `04-git` worktree-mismatch trap.
