# Merging: the M1-M9 procedure and corpus git routing

Steward task procedure. Read at the point of use. Governing playbook:
`../steward.md`.

`COORDINATION §14` defines the gate; this is the mechanics. **Run every step.
None is conditional on how routine the merge feels.**

**M1-M3a run before you publish (M3a is the authorization and is where a
seated lieutenant takes over), M4-M5 publish, M6-M9 run after it lands.**

> ### THE SPLIT: you ROUTE (M1-M3a), the lieutenant EXECUTES (M4-M9)
>
> `COORDINATION §14b` is binding. **M1-M3a are your routing work** — verify the
> Decision (M1), the exact SHA and its shape (M2), the cited sources (M3), then
> **post the exact-SHA authorization at M3a** (`ROUTED: <SHA>` with the gates,
> Decision, base, and self-verified diff scope, mentioning the **lieutenant**).
> **M4-M9 are the lieutenant's**, starting at M4's token
> mint — it runs the publisher, verifies the
> landed tree, flips the node, compacts the Adversary, and closes the loop with
> the ring. **You do not launch the publisher when a lieutenant is seated.**
>
> #### THIS BOUNDARY IS A NAME OTHER DOCUMENTS HOLD. RENUMBERING IS NEVER LOCAL.
>
> **If you ever move this split, the edit is not to this file — it is to every
> file that states it.** Measured 2026-09-16, when `M3a` was inserted and the
> boundary went from `M1-M4 / M5-M9` to `M1-M3a / M4-M9`: **six instruction
> files outside this one carried the old spelling**, including
> `steward.md`'s pointer *added an hour earlier*, which then named the range
> excluding the very step it was added for. A pointer that misdirects is worse
> than no pointer, because the reader stops looking.
>
> **Sweep by predicate, not by list** — a list cannot report being incomplete:
> *any statement of WHICH STEPS BELONG TO WHICH SEAT, in a file a seat reads as
> instruction.* That reaches beyond markdown: `moot.toml`'s `startup_prompt`
> strings and `steward-watchdog-tick-prompt.txt` both state it, and both sit
> outside a corpus sweep.
>
> **Records are not readers and must not be swept.** `docs/program/issues/**`,
> `docs/program/wp/**`, `docs/program/diary/**` and `lanes.md`'s status rows
> carry roughly a hundred mentions describing routings that happened under the
> old spelling. Rewriting those falsifies the record. **Leave a records file
> uniformly in the old spelling** — a mixture is worse than either end, because
> then both spellings look current.
>
> ⇒ **The failure is structural, not careless: the file that owns a name cannot
> enumerate its holders, and the holders cannot learn the name moved.** The only
> defence is to treat a renumber as a cross-file act from the start.
>
> #### `ROUTED:` IS IRREVOCABLE. DO NOT POST IT WHILE A REVIEW IS EXPECTED.
>
> **Posting `ROUTED: <SHA>` is the authorization and there is no way to recall
> it.** The window between your post and the merge is exactly when review
> findings arrive, and the executor is told to *"await the Steward's ROUTED
> authorization"* (`moot.toml:562`) — **the unqualified concept.**
>
> ⇒ **If a reviewer is still expected on a candidate, do not post `ROUTED:` at
> all.** Post the SHA, the gates and the scope, say plainly that it is **not
> authorized**, and post a bare `ROUTED: <SHA>` once the review clears.
>
> **A qualified routing (`ROUTED, HOLD FOR <reviewer>`) is NOT the fix and must
> not be used as protection.** It relies on the executor weighing a qualifier,
> which is the same channel that already failed: on 2026-09-16 the Architect
> asked the lieutenant to hold two candidates, both findings were correct, and
> **both landed anyway and became follow-up corrections to `main`.** Withholding
> the authorization is a mechanism; adding words to a message that was not acted
> on is not. **An absent `ROUTED:` cannot be misread.**
>
> Measured the same day: **nothing matches the token programmatically** —
> `grep -rn 'ROUTED' scripts/` is 0 (case-insensitive too), with reach proven by
> a positive control. Every reader is an agent reading prose, so the safe state
> when a candidate is not ready is that **no authorization exists**.
>
> Full account, with both measured instances:
> `agent/memory/fleet/a-merge-is-two-seats-the-router-and-the-executor-and-only-one-owns-each-merge.md`
>
> **Why:** one owner per merge. Measured 2026-08-23 — a Steward-launched
> publisher raced the lieutenant on the same PR (caught in the pre-lock wait).
> Once you route a SHA, the lieutenant owns its execution end-to-end; you stop
> and learn the outcome from a mention (landed SHA, or a CI-red relay you
> re-route on a *new* SHA).
>
> **M4-M9 below are still yours to run in the FALLBACK case — no lieutenant
> seated, or the operator tells you to publish directly** (e.g. your own §6a
> corpus route). The steps are identical; only the seat that runs them changes.
> Read M4-M9 as "the executor does X" — you when there is no lieutenant, the
> lieutenant otherwise. **M3a is never anyone else's**: even in the fallback you
> post the authorization before you publish, because the gate it carries is
> about whether a review is still expected, not about who runs the publisher.

Whether a thing *should* land, and where the cut goes, is `merge-policy.md`.
This file assumes that decision is made.

## M1 — Verify the Decision is `resolved`, read fresh from the object

```sh
# HTTP, your OWN credential. MCP list_decisions is NOT exhaustive.
# GET {API}/api/spaces/spc_4q7g0se87rgje/decisions?limit=20
```

`approved`, `proposed`, and `rejected` are not `resolved`. A Decision you
watched resolve earlier can be voided by an intervening publish — **re-read it
at merge time, never from memory.**

> ### M1 IS A PRESENCE PROOF. IT IS THREE-VALUED, AND THE THIRD VALUE IS NOT "ABSENT".
>
> **`list_decisions` is hard-capped at 100 rows and returns a sliding recency
> window.** Measured 2026-09-17 from 313 saved payloads: the last response
> exceeding 100 rows was `2026-09-05T14:35` (1036 objects); every call from
> `16:03` that day onward returns **exactly 100**. There is no error, no flag,
> no `has_more`. **A capped response is shaped exactly like a complete one.**
>
> **The cap applies AFTER the filter**, so a bucket's calendar reach is
> inversely proportional to how fast it fills:
>
>     status=proposed    n=0     under cap                     absence SOUND
>     status=approved    n=6     reaches 2026-07-03            absence SOUND
>     status=resolved    n=100   reaches ~9 days               absence UNSOUND
>     status=rejected    n=100   reaches ~40 days              absence UNSOUND
>     unfiltered         n=100   SHALLOWEST of all             absence UNSOUND
>
> ⇒ **Never "read it unfiltered to be safe."** Unfiltered spends its whole
> 100-row budget across every status at once and has the least reach of any
> query you can make. **The busiest bucket is the blindest, and `resolved` — the
> one this step reads — is the busiest.**
>
> **What saves this step is that M1 needs PRESENCE, not absence.** A recency cap
> removes rows; it can never invent one. So everything returned is real and a
> hit is always sound. Only the inference *"not in the list ⇒ does not exist"*
> is broken — and the window **emits its own floor**, so you can test it
> without knowing the cap exists:
>
>     FOUND                                          -> PASS
>     NOT FOUND, Decision resolved AFTER the floor   -> genuinely absent, FAIL soundly
>     NOT FOUND, resolved BEFORE the floor           -> INCONCLUSIVE: escalate,
>                                                       and never report it as absent
>
> **Read the floor — the oldest `created_at`/`resolved_at` in what came back —
> at gate time, and never carry it.** It slides. A memorised floor is the same
> defect as a memorised `origin/main`.
>
> **There is no by-id endpoint**, so the assembled read is all there is. In
> normal operation this step is conclusive: a candidate is routed minutes after
> its Decision resolves, which is deep inside the window. **The failure mode is
> old finished work routed late** — a re-route, a long-held branch, a survey
> that has been asked for five times. It fails CLOSED, refusing valid work
> rather than admitting invalid work, which is why it ran unnoticed from
> 2026-09-05.
>
> **Do NOT repair this by relaxing the acceptance criterion.** Re-keying M1 off
> `resolved_by`/`resolved_at` instead of the status string was proposed and
> **withdrawn as fail-open** on 2026-09-16: the same mis-key made by three seats
> is one defect with three instances, not a convention. M1's criterion is
> correct as written. **The defect was never the gate or its key — it was
> reporting an INCONCLUSIVE as an ABSENCE.**

> ### WHAT M1-M3 CANNOT SEE, SO YOU DO NOT MISREAD A LATER RED AS A GATE FAILURE
>
> This gate verifies **provenance and scope**: a resolved Decision, an exact
> SHA, a diff shape. **None of those is a compile, a test, or a review.**
>
> Measured 2026-09-17 on `fe7dc542b0fe29ce899ec575aabd5f8930e8901d`: M2 and M3
> were clean and every statement in them was true — 9 files, crates-only,
> correct merge-base, empty intersection, on origin — **and the tree called four
> functions that are defined nowhere in it.** `-p ken-runtime` compiled green
> because the call sites sit under `#[cfg(feature = "px8-ds-test-support")]` and
> that package's `default = []`, while CI activates the union every workspace
> member demands.
>
> ⇒ **When a routed candidate reds in CI, the routing gate did not fail.** Do
> not add build steps to M1-M3 and do not weaken `COORDINATION §12`, which is
> operator law. Withdraw the routing, say so by full SHA, and let the ring
> respin — the next candidate is a new SHA and carries nothing forward.

## M2 — Verify the exact SHA, and verify its SHAPE against the declared range

```sh
git fetch origin --prune && git cat-file -e <SHA>^{commit} && git rev-parse <SHA>
git log --oneline <BASE>..<SHA>          # commit count vs declared
git diff --shortstat <BASE>..<SHA>       # +/- vs declared
git diff --name-only <BASE>...<SHA>      # path count vs declared
```

**Never `--target HEAD`** — it dies with `src refspec refs/heads/HEAD does not
match any`. Always an explicit SHA.

> ### The heading used to say "exists on ORIGIN". It never does, and that is lawful.
>
> **`COORDINATION §14` forbids build seats from pushing**, so a ring candidate
> is a purely local object in the shared store at M2 — **the publisher is what
> pushes it**, at M5. `git branch -r --contains <SHA>` therefore returns empty
> on every correct handoff, and treating that as a blocker would stall every
> merge.
>
> ⇒ **What M2 can actually establish is that the object exists and its shape
> matches what the ring and the Architect declared.** Commit count, path count,
> and the `+/-` line, each against the declared range — not against `<SHA>^`.
> A shape mismatch here means the Decision approved something other than what
> you are about to publish, and it is far cheaper to catch now than at M6.
>
> **The standing warning that `git cat-file -e` passes on a purely local commit
> is still true** — it just is not a defect at this step. It matters when you
> are checking whether something has *landed*, which is M6's job and M6 uses
> blob identity for exactly that reason.

> ### BUT: if the wp/ branch DOES exist on origin, its CI history is not the candidate's
>
> The rule above says an empty `git branch -r --contains` is lawful. **The
> dangerous case is the opposite one — the branch exists, carries a pile of green
> check-runs, and its head is an ancestor the ring has since moved past.**
>
> Measured 2026-09-17, `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT`. The candidate was
> `b53bccda5`; the remote head was `fe7dc542b0fe`, **three commits behind**, and
> those three were the entire repair under review — a reverted re-export, the
> restored oracle, the documented drivers.
>
>     remote wp/ head   fe7dc542b0fe    26 check-runs
>     candidate         b53bccda5        0 check-runs
>
> The hazard is not an *absent* measurement — an absence announces itself. It is a
> **present** one about a different tree, and it reads as coverage.
>
> #### AND THE SECOND HALF, WHICH THE STEWARD WALKED INTO WHILE WRITING THIS
>
> **`total_count` is a ROW COUNT, not a verdict.** The Steward read that `26` and
> reported it, twice in the channel and once in the first draft of this very
> block, as *"26 check-runs, all green."* The actual breakdown:
>
>     fe7dc542b0fe    18 FAILURE / 8 success
>                     build + test, all 8 test shards, verify-realized-shard-union
>
> **The ancestor was not green. It was deeply red**, and the sentence claiming
> otherwise was written into a playbook block whose entire subject is not reading
> a green off the wrong object. Nothing in the API response says "green" — the
> conclusion lives per-row in `conclusion`, and a bare `total_count` is compatible
> with every row having failed.
>
> ⇒ **Never report a check-run figure without its conclusion split.** `26` is not
> a status. Print `Counter(r['conclusion'] for r in runs)` or name the failing
> leaves; see `a-surface-that-names-its-own-status-is-not-a-measurement-of-that-status`
> for why an aggregate cannot distinguish *failed* from *never ran*.
>
> ⇒ **Before reading any green off a wp/ branch, resolve what the remote head
> actually points at and compare it to the candidate SHA:**
>
>     git ls-remote --heads origin 'refs/heads/wp/<BRANCH>'
>     git rev-list --count <REMOTE_HEAD>..<CANDIDATE>     # 0 means they agree
>
> **A CI verdict belongs to a commit, never to a branch name.** This is the same
> exact-SHA discipline M1 applies to the Decision, applied to the check-runs.
>
> **Pushing the candidate yourself at M3 is permitted and is sometimes right** —
> it starts CI in parallel with the Decision instead of serially after it, which
> is worth doing when the local box cannot carry the build shape. **It is an
> optimization, not a repair, and it does not make an unpushed candidate a
> defect.** Verify a clean fast-forward over the current remote head first, and
> never force-push a ring's branch.

> ### A commit-count mismatch is a REVIEW defect first. POST IT.
>
> When the count you measure disagrees with the handback, the damage lands
> **before** the merge. A reviewer told *"one commit from `<BASE>`"* reasonably
> anchors on `<SHA>^..<SHA>`, and on a two-commit cut **the earlier deliverable
> is not truncated in that view — it is absent.** The review then comes back
> complete having never looked at half the candidate.
>
> Measured 2026-08-12: a combined `AC-1a`/`AC-1b` candidate was handed back as
> "one commit" when it was two. The strong form of the check is not the count
> but the identifier —
>
> ```sh
> git show <SHA> | grep -c <earlier-deliverable-test-name>   # 0
> ```
>
> — zero occurrences of `AC-1b`'s test name anywhere in the last commit.
>
> ⇒ **Correct the range to the reviewers as a post, before they anchor**, and
> state it explicitly as `<BASE>...<SHA>`. This is the same failure the M6 note
> below corrects for *merge verification*; nothing carried it to *review scope*,
> and the direction is identical — success reported over a smaller population
> than was declared, with no error and no short-list warning.

### M2a — READ THE COMMIT MESSAGES FOR SELF-DECLARED UNREADINESS

Shape is not the only thing the object DB will tell you. **Scan the candidate's
commit bodies, and treat any self-declared unreadiness as a claim you must
resolve against the tip before routing.**

```sh
git log --format='%h|%s' <BASE>..<SHA> -i \
  --grep='not merge-ready' --grep='not merge ready' \
  --grep='do not build on' --grep='do-not-merge'
```

**Do NOT add `--grep='WIP'` to that command.** It is the obvious term and it is
the wrong field. See the measurement below.

The predicate is a **self-assessment of merge-readiness in the negative**, not a
progress label. `WIP` says *"this is a checkpoint"*; `not merge-ready` says
*"this must not ship"*. Only the second is a claim you can resolve.

**A WIP marker on an intermediate commit is NOT by itself a defect, and you
must not treat it as one.** The publisher squashes, so the arc collapses to one
commit and intermediate states never reach `main`'s history. Rings develop
incrementally and label their checkpoints honestly; punishing that would buy
nothing and would teach seats to stop writing it down. **What you are looking
for is a marker whose stated CONDITION is still true at the tip** — a live
description of the candidate, wearing a historical commit's clothes.

So the check is one question per hit, not a veto: *is the thing this commit
says is unfinished, finished now?* Most answers are yes and cost a sentence.

> #### The noise and the signal are in DIFFERENT FIELDS. Measured, 2026-09-15.
>
> This step shipped with `--grep='WIP'` in it and that was wrong. Corrected
> after the Architect censused the disqualifying predicate over 2000 commits of
> `origin/main` (`evt_2ptnm0dyjx4kw`) and I re-ran both forms on the arc:
>
>     with    --grep='WIP'      11 hits
>     without --grep='WIP'       2 hits   <- a8430a8c2 and 2b8e2ba41
>     `WIP` in the SUBJECT line  9
>
> **Nine of the eleven were subject-line progress labels, and the true positive
> was not one of them.** `a8430a8c2`'s subject is confident — *"ABI-S6 HS18:
> certify generated Result path cuts"* — and the admission is the last sentence
> of a dense body. So the `WIP` term contributes only noise here, and a reviewer
> facing eleven hits of which nine are plainly ordinary would rationally dismiss
> the list, taking the true positive with it. **A check whose false-positive rate
> teaches you to skim it is worse than no check**, because it also supplies an
> alibi.
>
> The narrow predicate is also cheap to trust: the same 2000-commit census found
> **zero prior instances** of a self-declared not-merge-ready commit landing.
> This is a rare, cleanly greppable marker, which is what makes it a gate rather
> than a reform.
>
> #### `do-not-merge` IS HYPHENATED ON PURPOSE. Do not "fix" it to the spaced form.
>
> The spaced spelling `do not merge` is a **prefix of the operator's own
> merge-on-green boilerplate** — *"if CI reds, report it, do not merge past"* —
> so it matches that boilerplate by construction, on every candidate, forever.
>
> This step shipped with the spaced form and the Architect measured it
> (`evt_22zqt6d2ft4md`); I re-ran it over 3000 commits of `origin/main`:
>
>     spaced  `do not merge`      7 hits, 5 of them the boilerplate
>     hyphenated `do-not-merge`   3 hits, 0 of them the boilerplate
>     over the 36-commit arc      2 hits either way — both true positives kept
>
> **So the correction that removed nine noisy `WIP` hits had replaced them with
> five noisy boilerplate hits**, reproducing one level down the exact failure it
> was written to fix. The reasoning that shipped with it was also wrong, and is
> worth naming because it is a tempting mistake: it claimed the boilerplate was
> handled by *declining to search for* `merge past`. **Not searching for a phrase
> does not stop a shorter term from matching it.** What admits the boilerplate is
> the presence of `do not merge`, not the absence of anything.
>
> Both true positives survive the narrowing, which is the direction that had to
> be checked — a predicate narrowed until it drops a real hit is the worse error.
>
> Two residuals, neither a problem. The playbook's own commits match this step,
> because the file documents its own search terms; the step runs over a
> *candidate range* and never over the playbook's history. And the census is a
> measurement over this corpus, not a proof about the phrasing space: no one has
> established that some other spelling of a genuine do-not-merge instruction
> exists which only the spaced form would catch.

> #### Measured 2026-09-15, and it is why this step exists
>
> The 36-commit ABI-S6 D5b candidate `0f71ab5b9` carried **11 commits with such
> markers**, including one reading `DO NOT BUILD ON`. Ten were ordinary
> incremental checkpoints, resolved by the tip, exactly as above.
>
> The eleventh, `a8430a8c2` (arc commit 5 of 35), ended:
>
> > *native execution still reaches a controlled ResourceBodyResult pattern
> > trap (exit 1 vs interpreter 0); this commit is a WIP proof checkpoint, **not
> > merge-ready***
>
> **That sentence is a precise description of the CI failure that stopped the
> merge thirty commits later**, and it was true continuously in between. It
> passed an Architect design APPROVE, a QA CONFIRMED, this Steward's M1-M4, and
> a route to the publisher. Every gate read past it, mine included, because
> nothing in any gate's procedure looked at a commit body.
>
> The author did the right thing and wrote it down. **A record nobody reads is
> not a gate** — the same finding this arc spent the day establishing about
> documented-versus-checked invariants, pointed at process instead of code.
> `not merge-ready` in a commit body stopped nothing.
>
> The condition in that message was also **directly testable**: "exit 1 vs
> interpreter 0" names a differential anyone could run. The check is cheap
> because the honest marker usually tells you how to resolve it.

Route the hits to the ring rather than adjudicating them yourself — the ring
knows which are stale. What you own is asking, and recording the answer in the
Decision alongside the M2 numbers.

## M3 — Cited-source check

One command, not a judgment:

```sh
while IFS= read -r f; do
  git show origin/main:library/SOURCE-ATTESTATIONS | awk '{print $2}' \
    | grep -qxF "$f" && echo "CITED: $f"
done < <(git diff --name-only origin/main...<SHA>)
```

Hits route to the **Librarian, after the merge**. Never into the ring's frame.
Rationale and the already-red-`main` question are in
`release-and-handoff.md`, step 7.

## M3a — POST THE AUTHORIZATION. This is the irrevocable act.

**When a lieutenant is seated this is your LAST step, and it is the only one
that cannot be undone.** M1-M3 are measurements; you can redo a measurement.
**You cannot recall a `ROUTED:`.**

Post `ROUTED: <SHA>` mentioning the lieutenant, carrying the gates, the
resolved Decision, the base, and the self-verified diff scope **taken at the
merge-base, not at `origin/main`** (see M2).

**Before you post, ask the one question the other steps do not:**

> **Is a reviewer still expected on this candidate?**

**If yes, DO NOT POST `ROUTED:` AT ALL.** Post the SHA, the gates and the
scope, say plainly that it is **not authorized**, and post a bare
`ROUTED: <SHA>` once the review clears.

**A qualified routing (`ROUTED, HOLD FOR <reviewer>`) is NOT protection and
must not be used as such** — the executor is told to *"await the Steward's
ROUTED authorization"* (`moot.toml:562`), the unqualified concept, so a
qualifier is not a different token to the seat waiting on it. **Withholding the
authorization is a mechanism; adding words to a message is not.** Measured
2026-09-16: two review-post holds were correct and both landed anyway, while
withholding the authorization held a candidate that a qualifier would not have.

> **Why this is a numbered step and was not before.** This file's enforcement
> clause is *"Run every step"* — **keyed on the step as its unit.** The post had
> no step number, so it sat outside the only mechanism the file has: a Steward
> could complete every step the procedure contained without passing one that
> said *post now, and not while a review is expected.* **An enforcement
> mechanism keyed on a unit is blind to anything that is not one of those
> units** (Architect, `evt_2mks4ydn9ps1a`).

## M4 — Mint a token

**M4 onward is the LIEUTENANT's when one is seated** — you stop after M3a.

Agents hold no GitHub credential.

```sh
export GH_TOKEN="$(/workspaces/ken/.devcontainer/mint-gh-token.sh)"
```

## M5 — Run the publisher

**Executor step (the lieutenant, or you in the fallback case — see the split at
the top).** When a lieutenant is seated, you have already posted `ROUTED: <SHA>`
after M3; this is the lieutenant's to run, and you stop here until it confirms
the landed SHA or relays a CI-red. Everything below is written for whoever runs
the publisher.

```sh
scripts/scripted-pr-automerge.sh \
  --target <SHA> --title <pr-title> \
  (--description <text> | --description-file <path>) [--doc-only]
```

> ### BEFORE YOU PUBLISH, CHECK WHETHER A RUN IS IN PROGRESS ON `main`
>
> ```sh
> gh api "repos/swe-toolkit/ken/actions/runs?branch=main&per_page=1" \
>   --jq '.workflow_runs[0] | "\(.status) \(.head_sha)"'
> ```
>
> **If it says `in_progress`, wait.** `.github/workflows/ci.yml` keys its
> concurrency group on `github.event.pull_request.number || github.ref`. On a
> push there is no PR number, so the group collapses to the constant
> `refs/heads/main`, and `cancel-in-progress: true` means **your merge cancels
> the run verifying the commit before yours.**
>
> On a PR branch that supersede is correct — successive pushes are revisions of
> one candidate. **On `main` it is never correct**: successive pushes are
> distinct landed commits, each its own subject, each needing its own
> measurement.
>
> **A DOC-ONLY publish is the most destructive kind**, which is the opposite of
> how it reads. It cancels a full crates matrix and replaces it with a run whose
> heavy jobs are *skipped* — so the branch keeps reporting green while nothing
> executes. Measured 2026-09-16: `f4229a16` (crates) was cancelled 3m48s in by a
> docs push, and ten docs commits followed, each green, none running a shard.
>
> **This is a live workaround, not the fix.** The fix is a one-line change of
> `github.ref` to `github.sha`, which leaves the `pull_request` path provably
> untouched because that branch of the expression is unreachable there.

- **doc-only** — about two minutes; foreground is fine.
- **code** — `run_in_background: true`, always. It waits 581 to 718 seconds
  before its first poll, which exceeds a foreground tool timeout.

> ### A COMMENT-ONLY change qualifies as `--doc-only`, even in `.rs` files
>
> Operator, 2026-08-12. The discriminator is the **content of the diff**, not
> the file extension. A candidate that changes only comments is a doc change
> that happens to live in a code file, and paying a full CI poll for it is
> waste — measured on `e503ac73`, where a comment-only Runtime candidate spent
> ten-plus minutes in the code path.
>
> **Establish it mechanically before you pass the flag**, because "comment-only"
> is a claim about every hunk and the handback's word for it is not evidence:
>
> ```sh
> git diff -U0 <BASE>...<SHA> | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
>   | sed -E 's/^[+-]//' | grep -vE '^\s*(//|///|//!|\*|/\*)' | grep -vE '^\s*$'
> ```
>
> Empty output both directions means comment-only. Non-empty means it is a code
> merge, whatever the handoff called it.
>
> **One Rust-specific caveat.** A `///` comment can carry a **doctest**, so a
> comment-only diff that adds a fenced code block inside `///` adds a compiled,
> executed test. That is a code change wearing a comment's syntax. Check for an
> added fence before treating a `///` diff as doc-only.

> ### ON `doc-only`, RUN THE `§14a` READ SEPARATELY. ALWAYS.
>
> **`doc-only` decides WHICH CI MATRIX RUNS. `§14a` decides WHO MUST REVIEW.**
> Different questions over different criteria. A correct `doc-only` line
> sitting in your routing post **reads as evidence for the review question
> nobody asked it.**
>
> `§14a` names **`library/`** specifically — not "documentation", so a candidate
> can be `doc-only` to the classifier and still need the Architect's vote.
> Measured 2026-09-18 on `fefde16e`, a 162-line proof-obligation survey under
> `docs/program/`: `doc-only` to the classifier, outside `library/` to `§14a`.
> Raised by the Architect (`evt_2zeccsvspa4g1`).
>
> **THE GUARD IS ONE-DIRECTIONAL. Only `doc-only` can mislead.**
>
>     ON doc-only  ->  run the §14a read separately. ALWAYS.
>     ON full      ->  nothing owed; it cannot cause a skip.
>
> Reading `doc-only` as the predicate **skips a gate**. Reading `full` as the
> predicate costs at most a review nobody needed. That is half the work of "two
> reads, never one" and catches the same failures — and this step runs on every
> candidate, so the cheap version is the one that survives a hundred runs
> (Architect, `evt_6fa01xrvj9t3j`).
>
> **One divergent instance is enough, and only a divergent one counts.** Four
> cells, classifier against `§14a`:
>
>     doc-only x escalates      fefde16e   DIVERGENT -- the instance
>     full     x escalates                 concordant, both say heavy
>     doc-only x no-Architect              concordant, library-confined
>     full     x no-Architect              STRUCTURALLY EMPTY
>
> Library-confined implies `doc-only` by path, so the fourth cell is reachable
> only by the classifier being **wrong** about paths — an instrument failure,
> not a case. And a concordant row **cannot discriminate**: it is consistent
> with the classifier determining the review answer, so it corroborates
> nothing. **Do not add one to this evidence.** I did — `b3cacccd2`, cited as a
> `full`-CI mirror — and the Architect caught it before it landed. By path it is
> a single zero-`crates/` file, so the `full` half was never derivable from the
> diff, and the row sat in the concordant cell either way. **A supporting row
> inside a finding you authored gets adopted, not audited** — by its author most
> of all.
>
> **And the `docs/program/` exception is CONJUNCTIVE.** Verbatim from `§14a`:
>
>     the change is **currency or editorial**, and the Steward
>     **authorized the expansion when routing the WP**.
>
> Two conditions joined by "and". When you claim the exception, **state both.**
> The second is the one that goes unsaid, because it is a fact about **your own
> prior act** rather than about the diff in front of you — nothing in the
> candidate can remind you of it.
- **Never `git fetch` while it runs** — a lost ref-CAS reads as unverified.
- **Never pipe its output through `grep`** — block buffering swallows the poll
  lines.

**`resolved` is not the last gate. CI is.** Four candidates in one arc cleared
a `resolved` Decision and still failed CI.

The script creates the PR, waits and polls checks for non-doc changes, and runs
the publisher merge command. If GitHub blocks the merge it must stop and route
that fact; it must not pretend the publisher identity can self-approve.

### M5a — A RED that is not the candidate's: attribute it, then re-trigger on the SAME SHA

**The publisher stops on the first red shard by design and does not distinguish
a flake from a regression. That distinction is yours, and it is cheap.**

**Attribute before you tell anyone anything.** Three reads, all fast:

1. **Is the base green?** `gh run list --branch main --limit 5`. A red base is
   not the ring's bug.
2. **Can the candidate's paths reach the failing test?** Compare
   `git diff --name-only <BASE>...<SHA>` against what the test actually does.
3. **What do the sibling shards say?** Three of four green with the fourth
   failing on scratch-directory I/O is a runner fault, not a behaviour.

**Measured 2026-08-14 on `#2202`.** A one-path `ken-runtime` **test-control**
candidate reddened `library_documentation_gates.rs`'s synthetic-git fixture with
`git commit -m "filler 16"` → `error: bad tree object HEAD`. Shards 1, 2 and 4
passed; `main` was green. **The sixteenth filler commit failing after fifteen
succeeded is a write fault**, and no `ken-runtime` test edit can reach whether
git can write a tree object in a scratch repo.

> ### `gh run rerun --failed` IS REFUSED TO THE PUBLISHER IDENTITY
>
> `Resource not accessible by integration` — the App has no `actions:write`.
> **Tested, not assumed** (§7a: a capability question is answered by attempting
> it). Do not escalate for this permission before trying the route below.

**The route that works, and why it is the right one:** `.github/workflows/ci.yml`
triggers on `pull_request`, whose default types include **`reopened`**.

```sh
gh pr close <N> && sleep 5 && gh pr reopen <N>
gh pr view <N> --json headRefOid    # MUST be unchanged -- verify, don't assume
```

⇒ **A fresh run on the identical SHA.** That is the whole point: the Architect's
approval and the Decision bind an **exact SHA**, so republishing at a new SHA
would detach a live verdict and cost the ring a fresh Decision **for a flake**.
Never push an empty commit to "kick CI".

**Then re-run the publisher on the same `--target <SHA>`.** It finds the existing
PR rather than creating a second one.

> **Watch the stale-check hazard on the way back in.** The SHA now carries the
> old red check-runs *and* the new ones. Confirm `gh pr checks <N>` reports the
> **fresh** result before relaunching — a publisher that reads history aborts on
> a candidate that is actually green.

## M6 — Verify by blob identity, every changed path

Ancestry lies after a squash; phrase greps lie on wrapped lines.

```sh
git fetch origin --prune          # --prune is LOAD-BEARING, see below
# <BASE> is the cut's merge-base -- the base the ring declared, NOT <SHA>^.
for f in $(git diff --name-only <BASE>...<SHA>); do
  r=$(git rev-parse "origin/main:$f" 2>/dev/null); l=$(git rev-parse "<SHA>:$f")
  [ "$r" = "$l" ] && echo "MATCH  $f" || echo "DIFFER $f"
done
```

> ### `<SHA>^ <SHA>` IS WRONG AND SILENTLY UNDER-VERIFIES. Corrected 2026-08-10.
>
> This recipe said `git diff --name-only <SHA>^ <SHA>` until DS-9 `D1` caught
> it. **That range is the last commit only.** The cut was two commits —
> `2ef20dc5` added `Json.ken.md`, `6675ff54` added the signature assertions —
> so `6675ff54^..6675ff54` enumerated **one** of the two declared paths. The
> loop printed a single confident `MATCH` and the package itself went
> unverified.
>
> **The failure direction is the bad one: it reports success on a smaller
> population than you declared.** There is no error, no empty output, no
> DIFFER — just a short list that looks like a complete one. Any cut of more
> than one commit hits this, which is most of them.
>
> **Always enumerate from the declared merge-base**, the same `<BASE>...<SHA>`
> range the ring cited and the Architect reviewed, and **check the path count
> against the ring's declared scope.** If the loop prints fewer paths than the
> handoff named, the instrument is wrong, not the handoff.

The publisher squashes. `git reset --hard origin/main` afterwards —
`steward/work` is stale the instant any publish lands.

> ### `--prune` HERE is what stops the NEXT publish from this branch rejecting
>
> **Origin deletes the head branch the moment a PR merges** — which is exactly
> when this step runs. Your `refs/remotes/origin/wp/<ID>` is now stale, and the
> publisher's `--force-with-lease` compares against it, so the **next** publish
> from that branch dies before it creates a PR:
>
> ```
> ! [rejected]  wp/<ID> -> wp/<ID> (stale info)
> ```
>
> Under the accepted-partial policy a WP branch is published **repeatedly by
> construction**, so this fires on most multi-deliverable nodes. It has now
> fired **five times**.
>
> **Neither fetch you would otherwise run clears it, and both look reassuring:**
>
> | what you run | what it does |
> |---|---|
> | plain `git fetch origin` | succeeds, prints nothing relevant, **leaves the stale ref** |
> | `git fetch origin wp/<ID>` at M2 | fails `couldn't find remote ref`, **leaves the stale ref** |
>
> That second one is the trap worth naming: it means *the remote branch is gone
> **and** your tracking ref is now stale.* Reading only the first half — "gone,
> the publisher will recreate it" — is what preceded the fifth occurrence.
>
> ⇒ **Prune where the staleness is CREATED, not where it is felt.** This step
> already fetches after every merge, so `--prune` here is deterministic and
> costs nothing. Every earlier prevention was phrased as *notice the early
> warning*, and **a prevention that depends on noticing an incidental symptom
> is not a prevention** — all five occurrences were diagnosed only after the
> push rejected.
>
> `--prune` deletes only refs whose upstream is **gone**, so it cannot strand a
> branch another seat is still publishing from. Measured on the fifth
> occurrence: this exact line reaped the branch origin had just deleted, one
> command after the merge.
>
> **If you are reading this because a push already rejected:** do not re-run
> the publisher until `git rev-parse --verify refs/remotes/origin/wp/<ID>`
> **fails**. A successful fetch is not that check. The failed run dies at the
> push *before* creating a PR, so there is no orphaned PR to hunt.

> **Why blob identity and not a verification phrase grep.** A phrase must not
> span `**bold**` or `` `code` `` markers — and, the mode that fires every
> time, must not span a **line break**. In a corpus hard-wrapped at 80 columns,
> any phrase distinctive enough to prove identity is long enough to wrap. The
> two requirements are in direct tension.
>
> Measured on PR #955: four verification greps, **two came back empty on
> content that was byte-identical on `origin/main`.**
>
> **The dangerous direction is the false negative becoming a false positive.**
> An empty grep says "not landed", which is alarming and self-correcting. But
> the instinct it trains is to shorten the phrase until it matches, and a
> phrase short enough never to wrap is usually short enough to appear in prose
> that predates your change. **Never weaken a probe to make it pass. Replace
> the instrument.**
>
> **Blob identity is necessary, not sufficient — it proves the file landed,
> never that the file is right.** Keep the index post-condition below.

## M7 — Flip the node, regenerate the tracker

```sh
sed -i 's/^status: active$/status: merged/' docs/program/issues/<ID>.md
scripts/gen-progress.sh
```

Bundle both into your next publish.

### M7a — A node that OWNS ignored rows may not reach `merged` unowned

**Before flipping, ask whether this node owns any `#[ignore]`d row. If it does,
the flip must either clear those rows or name a successor owner — in the row's
own attribute, not only in a convo post.**

```sh
grep -rn '#\[ignore' crates/ | grep '"<ID>'
```

**Scan all of `crates/`, not `crates/*/tests/`.** Ignored rows live under `src/`
too — `ken-elaborator/src/compiler_driver.rs` and `ken-runtime/src/` both carry
them, and so does a nested `src/.../core/tests/`. A `crates/*/tests/` glob misses
every one of those directories.

That is not hypothetical, and it is the reason this paragraph exists. **The step
shipped with that glob, and the case it was written for sits in the gap it
leaves.** Measured at `34e1426c7`: the corrected command returns
`compiler_driver.rs:5404`, the founding case; the original returns zero for it.
Both return the same three `RT-SITEOP-CARRIED-WITNESS` rows, so the widening
loses nothing. A rule's founding case is the one it is never run against — run
this one against yours before you trust it.

Match on the node named at the **START** of the `#[ignore]` string. A node the
label merely *mentions* — to supersede, to inherit from, to cite — does not own
the row, and a substring join will hand you rows that belong to someone else.

If the grep returns rows and you cannot clear them in this merge, **the
successor owner goes into each row's attribute as part of this publish.** A
row whose label names a `merged` node is unroutable: it looks owned to every
census, and nothing will ever correct it, because the only thing that would
have is the node that just closed.

> **Why this step exists.** Measured 2026-09-18, against the operator's
> top-priority objective (clearing the ignored tests). Of the fourteen blocker
> rows in `crates/ken-cli/tests/`, **four named a `merged` owner** — three
> `RT-SITEOP-CARRIED-WITNESS`, one `RT-SUBCONTINUATION-LIFO-RELEASE-ORDER`. None
> was abandoned on purpose. Each was minted by an ordinary, correct merge that
> had no step at which anyone asked this question.
>
> **The mechanism is still live and the next instance is already visible:**
> `RT-COMPMATCH-TREE-SCRUTINEE` is `ready`, owns one row, and is in the queue.
> When it lands it will orphan that row the same way, and nothing in the
> federation will report it. **A census is a reading at a moment; this is the
> missing enforcement edge** (Architect `evt_7881e7b6wvf87`).
>
> The worst of the four shows what the gap costs: `px8ta:326` carries a label
> saying **"product defect, escalated"** and is two blockers under one
> `#[ignore]`, the second of which its own comment says is *"not owned by that
> node"*. An escalated product defect, an unowned second blocker, and a merged
> owner — reached without any single step being wrong.

**State the predicate when you report a count, because the cells move under
it:** `owner terminal (merged)` and `owner not releasable today (merged +
draft)` are different populations, and a node that is itself `draft` discharges
nothing.

#### What this step does NOT cover

This step reaches **node-owned rows** — rows whose label names the merging node
at its start. It does not reach the rows the CI sweep holds exempt in
`.github/ignored-test-exemptions.toml`, which are keyed by test path and carry a
`class` and a `readmission` condition rather than an owning node.

Those have the same defect one layer over: `verify_blocked_upstream_relations`
in `scripts/ci-ignored-sweep.py` checks that a row's readmission symbol *occurs
in its own `#[ignore]` reason*, never that the named relation is still absent.
`RT-CLOSURE-BOUNDARY-LANE` is `merged`, its exemption stands, and CI is green.
**Do not read M7a as covering the ignored-row population.** It covers the rows a
merge can orphan; the registry's rows expire on a condition no instrument
watches, and closing that is a `CI-IGNORED-SWEEP` successor, not this step.

Nor is the sweep's own selection label-keyed, so do not reason about it from
this step's grep: selection is **registry subtraction** — the nextest ignored
population minus the exemption registry (`expected_count`). `verify_lists`
reconstructs the nextest total from selected plus registry and raises, so an
ignored row that is neither selected nor registered is a red, not a silent drop.

## M8 — Compact the Adversary, then notify it, if the merge carries code

Two actions in that order, both required, both yours. A step, not a courtesy.

### M8a — Compact it FIRST. Nothing else in the fleet does.

**Operator, 2026-08-17: *"There is nothing that compacts adversary and the
adversary does not self-compact."*** `COORDINATION §15` assigns the job to the
seat itself, along with the other singletons. Measured, that does not happen —
so the assignment produced no compaction at all, and the Adversary is the one
seat that can climb indefinitely with no instrument pointed at it.

**Why the compaction belongs here and not on a schedule.** The Adversary is
event-driven with no WP pipeline, so the release path never hands it a
before-work gate. **This notification is the only work boundary it has** — and
it is the *complete* set of them, because its context grows only when it hunts
and it hunts only when notified. Compacting at M8a is therefore the same rule
as `compaction.md`'s *"always compact before new work"*, applied at the only
seam that exists for this seat.

```sh
moot compact adversary
tmux capture-pane -p -S -50 -t moot-adversary | grep -c Compacting   # confirm
```

**ctx unread.** Do not look at the number to decide, for the reason
`compaction.md` gives: every rationalization for skipping is a threshold.

> ### THE ONE EXEMPTION: A SEAT AT A FRESH-SESSION FLOOR. Measured at the first firing, 2026-08-17.
>
> **`moot compact` on a just-started session is not a cheap no-op — it is a
> LOSS.** A fresh seat holds exactly its orientation: playbook, memory scopes,
> role. Compaction summarizes that away, so the seat you then notify must
> re-orient before it can hunt. **A restart is strictly stronger than a
> compaction** — it discards rather than summarizes — so the goal of M8a is
> already met by construction.
>
> **The tell is not the ctx number, and this is why the rule below still says to
> ignore it.** `ctx 0%` alone does not distinguish "fresh" from "compacted an
> hour ago and idle since" — and the second case is also already at the floor, so
> both exempt. What you are looking for is the **session-start signature** in the
> pane: `Skills restored (<role>)`, the file-reference block from its orientation,
> and an empty composer. Read the pane, not the percentage.
>
> **This exemption is narrow and it is the only one.** It says *"there is nothing
> to compact"*, never *"the context is warm"*, *"they are only at N%"*, or *"I
> will catch it next merge" — those are the rationalizations `compaction.md`
> names, and they remain forbidden. If the seat has hunted since it started, it
> has context, and it gets compacted.
>
> **Log the skip in the M8b notification** so the next reader can see the step was
> evaluated rather than forgotten. A silently skipped step and a step that never
> existed look identical three merges later.

**Precondition, same as any other seat: quiescent, and owing nothing in
flight.** If it is mid-hunt on the previous merge, or holds a finding it has not
yet handed to you, compaction drops the obligation — do M8b now and take the
compaction at its next handoff. Never compact it mid-reasoning.

> ### THE ORDER CREATES A SILENCE, AND THIS STEP EXISTS TO REMOVE ONE
>
> **A convo mention does not wake a freshly compacted seat.** It returns to an
> empty composer and does not poll (`adversary.md`, §1). So compact-then-notify,
> run naively, produces a notified-and-asleep Adversary — **the exact failure
> M8 was created to fix in 2026-07-29, arrived at from the other direction.**
>
> The sequence is four beats, not two: **compact → confirm the drop → post M8b →
> rouse the pane.**
>
> ```sh
> tmux send-keys -t moot-adversary -l 'run get_recent_context and pick up event <evt_id>; re-orient per CLAUDE.md, then proceed'
> tmux send-keys -t moot-adversary Enter     # a SEPARATE call
> ```
>
> **Skipping the rouse is invisible from your side.** A seat that was never
> woken and a seat with nothing to report post identically: nothing.

### M8b — The notification

> ### NAME THE LANDED SQUASH, OR `merge-base...tip`. NEVER A BARE TIP SHA.
>
> **Measured 2026-08-13, Adversary finding on #2103.** The notification named
> `448604e1`, the pre-squash branch head. A reviewer who anchors on it runs
> `git show` and reads **one commit**:
>
> ```
> git show --stat 448604e1           ->   2 files, +44/-3
> git diff --stat b4d38b8a 448604e1  ->  10 files, +87/-62
> ```
>
> They see a two-line lexer edit and a test, **find none of the enum removals
> the notification described, and git gives them no error.** It reads as
> complete.
>
> **A branch head names a tree containing everything and a commit containing
> only the last step.** The reviewer wants the range; the archaeologist wants
> the squash; the bare SHA silently serves neither. **One clause fixes it** —
> name the squash that landed on `main`, or write the range explicitly.
>
> This is why the habit forms: the tip SHA is the object review votes are cast
> on, so it is the one in front of you. **The reviewing audience and the
> reading audience need different identifiers.** Give the reading audience
> theirs.

**Doc-only merges do not concern the Adversary** (operator, 2026-07-29). Do not
notify it for them. Frames, tracker flips, node registrations, counters, and
corpus edits are not its surface.

**A skipped M8 skips M8a with it, and that is correct, not a gap.** No
notification means no hunt means no growth, so a long doc-only run leaves the
seat uncompacted *and* idle. The thing to keep true is the pairing: **you never
notify it without having offered it a compaction first.**

> ### `--doc-only` STOPPED being this step's discriminator on 2026-08-12
>
> It used to read *"`--doc-only` on the publisher is exactly the discriminator
> — if you passed it, skip M8."* That was sound only while the flag meant
> "touches no code file." **The operator widened it that day to cover
> comment-only changes inside `.rs` files** (see M5), and the moment it widened,
> this step started silently skipping merges that land inside the Adversary's
> surface.
>
> **The direction of the failure is the bad one:** it suppresses a notification
> rather than sending a spurious one, and a seat that is never told is
> indistinguishable from a seat with nothing to report.
>
> **Ask the question directly instead of reading a flag:** did this candidate
> change any file under `crates/`? If yes, run M8 — even if you passed
> `--doc-only`. Comments in code are frequently where the safety reasoning
> lives; `units.rs:2889` was a comment bounding a lowering the ring itself
> called *"a wrong program rather than a missing one"* if armed as written.
>
> **The reusable shape:** a discriminator that piggybacks on another flag
> inherits every later widening of that flag, and nothing about the widening
> looks like a change to this step.

For a code merge, look the id up at post time (`scripts/moot-actor-id.sh
adversary`) and post:

- the merged SHA and the resulting `origin/main`;
- the code paths and the size (`git diff --shortstat <SHA>^ <SHA>`), so it can
  bound its pass;
- anything you already know is unhunted or excluded.

> **Measured 2026-07-29: this step did not exist and the seat ran blind.** The
> Adversary was *described* as "event-driven on merge notifications" with
> nobody assigned to send one. It hunted four unnotified merges off its own
> currency checks and filed two findings, one soundness-adjacent on the kernel
> gate. **A requirement living in a descriptive sentence is never discharged —
> it needs a numbered step.**
>
> **The loop has a second half: its reports do not appear in the space-level
> event read.** A 200-event scan returned zero adversary posts while both
> findings were live and fetchable by event id. Every watchdog tick, read
> `GET {API}/api/spaces/{sid}/threads` and open any thread with
> `unread_count_for_actor > 0`. Notifying it and never reading it back is the
> same silence.

## M9 — Close the loop with the ring

Then run the stay-one-release-ahead check (`../steward.md`, section 4): every
node whose `depends_on` names this one is `ready` with a shovel-ready frame
before you stop.

### M9a — Return your worktree to your home branch, after every merge

**Executor step — the "keep `steward/work` fresh" rule below, generalized to
whichever seat ran M4-M9.** Authoring the M7 close commit leaves your worktree
on a `wp/...` branch; if you never switch back, it rests **detached** on a
superseded close commit — a stale tip that misleads every "what has landed" read
and is the state in which the branch namespace accretes. Measured 2026-08-23:
1846 stale `wp/scripted-merge-*` refs had accumulated this way. (Detachment was
initially suspected of also blocking `moot compact <seat>`; a direct test
2026-08-23 falsified that — the hang persists with the worktree attached, so its
cause is separate and still open.)

```sh
git fetch origin --prune
git switch <home>/work        # steward/work (Steward) or lieutenant/work (lieutenant)
git reset --hard origin/main  # your tracker-sync commit already put the tracker on main; nothing durable is lost
git branch -D "wp/scripted-merge-$(git rev-parse --short <target-sha>)"  # publisher synthetic ref; disposable once M6 verified the landing
```

Rest on your home branch at `origin/main`, never on a `wp/...` tip. The synthetic
branch the publisher mints (`scripts/scripted-pr-automerge.sh`, ~line 279) is
disposable the moment M6 confirms the content on `main`; reaping it each run is
what keeps the ref namespace from accreting (1846 had accumulated by 2026-08-23).

## Routing your own corpus edits

Your operational docs — the progress tracker, `agent/` playbook and
`COORDINATION.md` edits — skip the spec-leader step and go straight to `main`.

1. **Commit on `steward/work`** when the working change belongs there.
2. **Route to a corpus branch off current `origin/main`:** `git fetch origin`;
   `git branch -f wp/steward-<slug> origin/main`; `git switch
   wp/steward-<slug>`; apply or cherry-pick the intended change. The branch
   starts as `origin/main` plus the routed change only, never a stale base.
3. **Append the tracker-sync commit before publication.** Pull the current
   progress file from `steward/work`, commit it if it differs, and treat the
   resulting branch tip as the PR SHA:

   ```sh
   git checkout steward/work -- docs/program/IMPLEMENTATION-PROGRESS.md
   git add docs/program/IMPLEMENTATION-PROGRESS.md
   git diff --cached --quiet || git commit -m "tracker: sync implementation progress"
   ```

   The `git diff --cached --quiet ||` guard is required: without it the commit
   fails when the tracker already matches. **This is the only copy of this
   procedure** — it also applies to every ring candidate you publish, so
   `origin/main` preserves the current progress file durably.

   > **`git checkout <ref> -- <path>` HERE IS SAFE ONLY BECAUSE YOU HAVE NOT
   > EDITED THE TRACKER ON THIS BRANCH. Do not generalize the idiom.**
   >
   > It is a checkout, not a merge: it takes the ref's blob wholesale and
   > **destroys your uncommitted work in that path**. Measured 2026-08-12 — the
   > Steward reached for this same line to carry a frame edit onto a corpus
   > branch and **reverted the edit it was trying to publish.**
   >
   > **The failure is silent in the worst direction.** `git status` comes back
   > empty, which reads as *"the switch carried everything cleanly"* rather than
   > *"your change is gone"*; the branch then publishes an empty or no-op
   > candidate, the publisher succeeds, blob identity passes on a file that never
   > changed, and you announce a correction that is not on `main`. **Every
   > post-condition in this file passes.**
   >
   > ⇒ **Carry your own edits by `cherry-pick`, or by re-making them on the
   > corpus branch.** Reserve this command for a path you have not touched, and
   > if you have already run it, the check is `grep` for the text you wrote —
   > not `git status`.
4. **Publish with the scripted publisher path** (M4 and M5) unless the operator
   routes otherwise.
5. **Sweep only after the merge is confirmed** — M6's blob-identity loop. The
   repository deletes remote head branches automatically; local cleanup is
   optional and must not delete a branch before `origin/main` is verified.

A multi-piece corpus change is one branch (`COORDINATION §14`). Width-check
markdown at 80 display columns (codepoints, not bytes) before routing.

## Corpus edits: the index post-condition

When a corpus change carries an index — a `README.md` table, a manifest, a
catalog — assert the post-condition, not the phrase. Blob identity proves the
file landed; it says nothing about whether the change landed **whole**.

**Sweep every scope directory in one command, never one at a time:**

```sh
# rows vs files, BOTH orphan directions, EVERY scope
for d in $(find agent/memory -type d | sort); do
  ls $d/*.md >/dev/null 2>&1 || continue; [ -f $d/README.md ] || { echo "$d NO README"; continue; }
  files=$(ls $d/*.md | xargs -n1 basename | sed 's/\.md$//' | grep -v '^README$' | sort)
  rows=$(grep -oE '\]\(([A-Za-z0-9._-]+)\.md\)' $d/README.md | sed 's/](//;s/\.md)//' | sort -u)
  echo "$d files=$(echo "$files"|grep -c .) rows=$(echo "$rows"|grep -c .)"
  comm -23 <(echo "$files") <(echo "$rows") | sed "s|^|  ORPHAN FILE: $d/|"
  comm -13 <(echo "$files") <(echo "$rows") | sed "s|^|  ORPHAN ROW:  $d/|"
done
```

Three defects this replaces, all measured 2026-07-26 on one run of the old
snippet, each invisible to the run that had it:

1. **It was run on the two scopes the role loads** and reported "8 orphans" as
   the population. The real count was 10. **A per-directory command invites a
   per-directory population, and the report says nothing about the scopes it
   never visited.**
2. **An ad-hoc lowercase-only character class** made two rows with capitals
   read as false orphans. The class must admit every filename shape actually on
   disk — derive it from the corpus. Note the direction: this over-reported
   while defect 1 under-reported, in the same run, so the totals partly
   cancelled and both looked plausible.
3. **A known non-lesson file reports as a permanent orphan.** Exempt those
   explicitly, because a standing false positive is how you learn to skim past
   real ones.

**`git checkout <ref> -- <path>` is a checkout, not a merge.** It takes the
ref's blob wholesale and destroys what was there. Applying two branches that
share a file this way silently keeps only the second. (2026-07-22: two adversary
memory branches both appending to one index gave `10 base + 3 = 13` where `10 +
5 = 15` was due; two lessons landed with no index row, both commits clean, no
warning.) To combine branches use `cherry-pick` or `merge` so conflicts are
raised, then run the post-condition regardless.

**Predict the number before you look:** `base + delta1 + delta2`. That one line
of arithmetic is the detector.

**Why a post-condition and not a mechanism guard.** The loss was first blamed
on git's silent union of disjoint hunks. Measured, that was false — those
branches conflict loudly, and the loss came from a command with no merge
semantics at all. **A guard keyed to a mechanism story fails when the story is
wrong.** A post-condition on the merged artifact catches unions, wholesale
takes, and bad conflict resolutions alike, without needing to know which
occurred.

## Corpus edits: check what you broadcast, not only what you committed

The artifact and the announcement fail independently, and it is the
announcement that reaches rings as binding instruction. Twice in one session a
correct landed artifact was published alongside prose carrying a false
mechanism — once to two build rings as the *reason* for a rule.

After any publish you also narrate to the fleet, re-read your own message
against the artifact and ask whether the explanation reproduces what actually
happened. An explanation that merely sounds consistent with a true rule is not
thereby true. **Watch especially for a clause whose function is to tell the
reader they need not look** — "you cannot lose X by accident", "errs in the
safe direction", "immaterial".

## Keep `steward/work` fresh against `origin/main`

`steward/work` is a working copy, not a durable log: it should always be
`origin/main` plus at most the current unpublished tracker delta. It drifts
into a stale tree when tracker commits pile up on a base that never advances
while other teams merge. The symptom is a worktree carrying a superseded layout,
a giant false `origin/main..HEAD` diff, and merge hazards — editing files
against that stale base silently reverts other teams' merged work if you route
the branch.

- **On resume, after any merge notification (yours or another team's), and
  before starting new corpus work:** `git fetch origin`; preserve the tip
  cheaply with `git branch -f preserved/steward-work-$(git rev-parse --short
  HEAD) HEAD`; then `git reset --hard origin/main`. Your last publish already
  put the tracker on `main` via the tracker-sync commit, so the reset loses
  nothing durable. **Re-derive** the current tracker block against `main`'s
  version rather than blind-carrying a stale copy.
- **Never `git rebase origin/main` a long-lived `steward/work`.** A squash-merge
  leaves the original branch commits dangling ahead of `origin/main` while
  their content is already merged, so a rebase replays already-landed commits
  into conflicts. Reset-to-`origin/main` plus re-apply is the robust move.
- The corpus-branch route already cuts from current `origin/main`, so a fresh
  `steward/work` is not required to publish — but a stale one misleads you
  about what has landed and is the root of phantom "unmerged work" scares.

## The squash-merge trap

After a squash-merge the *original* branch commits dangle **ahead** of
`origin/main` while their content is already merged. Such a branch is a stale
leftover, not unmerged work. Grep `origin/main` for the squash commit before
treating branch-ahead commits as lost or held; do not re-open or recover them.
**Branch-ahead does not imply unmerged.**
