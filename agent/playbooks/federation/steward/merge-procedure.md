# Merging: the nine-step procedure and corpus git routing

Steward task procedure. Read at the point of use. Governing playbook:
`../steward.md`.

`COORDINATION §14` defines the gate; this is the mechanics. **Run every step.
None is conditional on how routine the merge feels.**

**M1-M3 run before you publish, M4-M5 publish, M6-M9 run after it lands.**

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

## M4 — Mint a token

Agents hold no GitHub credential.

```sh
export GH_TOKEN="$(/workspaces/ken/.devcontainer/mint-gh-token.sh)"
```

## M5 — Run the publisher

```sh
scripts/scripted-pr-automerge.sh \
  --target <SHA> --title <pr-title> \
  (--description <text> | --description-file <path>) [--doc-only]
```

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
- **Never `git fetch` while it runs** — a lost ref-CAS reads as unverified.
- **Never pipe its output through `grep`** — block buffering swallows the poll
  lines.

**`resolved` is not the last gate. CI is.** Four candidates in one arc cleared
a `resolved` Decision and still failed CI.

The script creates the PR, waits and polls checks for non-doc changes, and runs
the publisher merge command. If GitHub blocks the merge it must stop and route
that fact; it must not pretend the publisher identity can self-approve.

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

## M8 — Notify the Adversary, if the merge carries code

A step, not a courtesy.

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
