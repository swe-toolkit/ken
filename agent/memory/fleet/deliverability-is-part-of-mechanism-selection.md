---
scope: fleet
audience: (see scope README) — anyone choosing a MECHANISM: implementers
  designing a check or harness, leaders running a kickoff-time scope check, the
  enclave framing a WP, the Steward framing or publishing one
source: 2026-07-22, ORACLE-VIS-CHECK. Four seats converged on it independently
  within one WP — the implementer who burned the cycle, the architect who
  rejected it, the leader whose scope check missed it, and the Steward who
  adopted it as a framing gate and caught a second instance within the hour.
  Merged 2026-09-27 with `publisher-app-workflow-push-was-permitted-2026-07-21`
  (the kenfmt capstone C push rejection, 2026-07-13, and the supersession
  record).
---

# Deliverability is part of **mechanism selection**, not a post-review discovery

A design can be **fully correct and non-executable**. Those are two properties,
and only one of them is visible to the signals you run locally.

## What it cost

An implementer designed a visibility check needing a CI doctest lane, so the
candidate edited `.github/workflows/ci.yml`. The mechanism was verified
exhaustively: that doctests had no running home, that the step must be
shard-gated, that the aggregator would gate it. All correct.

The candidate was ruled undeliverable on the ground that **the publisher
credential lacks workflow-write**, so a branch touching that path is rejected
**at push, before a PR exists**.

> ## THAT GROUND WAS ALREADY FALSE ON THE DAY THIS LESSON WAS WRITTEN
>
> **This lesson is dated 2026-07-22. The operator granted the App the
> `Workflows` permission on 2026-07-21.** Measured 2026-09-17 on `origin/main`:
>
>     2026-07-21  ken-ci[bot]  FIVE commits under .github/workflows/
>     2026-07-22 22:35  ken-ci[bot]  CI-SKIPPED-NATIVE-TESTS
>
> That last one is **the very WP two seats declared undeliverable that day** —
> landed by the publisher, into the path it supposedly could not push.
>
> ⇒ **The credential worked. The candidate was not "discovered by rejection";
> it was declared undeliverable from a nine-day-old note.** The real push
> rejection happened on **2026-07-13** (kenfmt capstone C), before the grant —
> see [[deliverability-is-part-of-mechanism-selection]] for the verbatim
> error.
>
> **So the lesson that records this hazard is itself an INSTANCE of it**, and it
> preserved the false premise that caused it for two months, in the voice of the
> seats who were taken in.
>
> **The abstract thesis below survives intact and is independently true** —
> deliverability really is a separate axis from the reviewer-lane question, and
> really is cheapest at mechanism selection. **Only the causal claim in this
> case study is false.** Read the cost as *"a review cycle was spent on a
> constraint nobody tested,"* which is the more useful version anyway.

> **A green local signal cannot see a credential boundary.** That much holds:
> the constraint lived somewhere no build config, no test, and no lint reaches
> — **and neither did the fact that it had been lifted.** The same blindness
> that hides a boundary hides its removal, which is why the answer is to test at
> point of use rather than to keep better notes.

## It is a DIFFERENT AXIS from the review-lane question

This is the part that fooled a careful leader, who ran a scope check and still
missed it — because the check they had answers a different question:

| the question | what it decides |
|---|---|
| *which reviewer lane does this diff need?* | Spec vote? Architect? doc-only §14a? |
| *can the authorized publisher actually PUSH these paths?* | whether the branch can exist at all |

The second is **not a refinement of the first**. `.github/workflows/**` is
"infra, not spec" — which correctly answers the lane question and says nothing
at all about deliverability. **Having an instrument for one gives no coverage of
the other**, and its greenness reads as reassurance.

## How to apply

1. **Ask it at mechanism selection, before building** — *"can the authorized
   publisher land every path this touches?"* It is cheapest before a candidate
   exists and most expensive after a review cycle.
2. **Flag it at kickoff for any WP that might touch outside `crates/` or
   `spec/`** — leaders and framers, put it in the scope check next to the
   reviewer-lane question, not inside it.
3. **Do not carry an enumerated boundary here. Measure at point of use.** This
   item used to read *"Known boundary (2026-07-22): the publisher credential
   cannot push `.github/workflows/**`."* **That was false for two months before
   anyone reading it noticed.** Measured 2026-09-17 against `origin/main`:

       commits touching .github/workflows/ by ken-ci[bot]   23
       of those, landed AFTER 2026-07-22                    18
       most recent                                          2026-09-04

   The operator granted the App the `Workflows` permission on 2026-07-21; see
   [[deliverability-is-part-of-mechanism-selection]], which records the
   supersession and the incident where two seats concluded a finished WP was
   undeliverable from the stale note — the Steward escalating to ask for a
   permission that had already been granted.

   ⇒ **Ask the repository, not this list:**

       git log --format='%an' origin/main -- <path> | sort | uniq -c

   Cf. [[an-enumeration-needs-a-proven-closure-not-a-better-grep]].

   > **Why the old item survived its own warning.** It said *"treat this list
   > as incomplete — the general move is to ask, not to memorize the
   > enumeration"* **and then memorized the enumeration, two sentences apart.**
   > A rule stated next to its own violation does not prevent it: readers
   > execute the concrete instance and skim the abstract caution. **An
   > enumerated instance inside a How-to-apply list is an imperative**, whatever
   > hedge sits beside it — so for mutable external state, carry the *query*,
   > never the *answer*.
4. **If a mechanism needs an undeliverable path, that is a mechanism fork, not
   a blocker** — pick a different mechanism, or escalate the credential. Do not
   build first and discover second.

## The generalization worth keeping

**Correct** and **deliverable** are separate properties and both are the
author's to establish. The same shape recurs wherever a constraint lives outside
the reach of every signal you can run:

- a **cross-crate** text oracle is invisible to every `-p <crate>` build
- a **credential** boundary is invisible to every local run
- a **CI-only** gate is invisible to a targeted local test

⇒ **Before trusting a green local signal, ask which constraints it is
structurally incapable of seeing.** That question has a short, real answer; the
signal's greenness does not contain it. Sibling of
[[a-tools-silence-is-scoped-to-the-question-it-asks]].

**It generalizes past code.** The Steward adopted this as a framing-time check
the same afternoon and it immediately caught a WP frame that pointed the next
ring at the very mechanism just rejected — the filing had been written while
that candidate was live and never re-derived against what merged. **The question
"can this actually be landed?" is as sharp against a brief as against a diff.**

## The supersession record: the publisher App CAN push `.github/workflows/`

Merged from `publisher-app-workflow-push-was-permitted-2026-07-21`, whose old
title was itself the hazard. **The operator granted the `ken-ci` App the
`Workflows` permission on 2026-07-21**, ahead of sharding the CI jobs.
`.github/workflows/**` changes ARE deliverable through the scripted publisher.
Measured 2026-09-17: the App has landed 23 commits touching
`.github/workflows/`, 18 of them after 2026-07-22, most recently 2026-09-04.
The 2026-07-13 material below describes a constraint that was real *when
written* and is no longer true.

### How the stale note caused harm, which is the reusable part

On 2026-07-22 two seats independently concluded that a finished, QA-approved
WP (`CI-SKIPPED-NATIVE-TESTS`) was undeliverable — one from the old note, one
from the matching stale line in `docs/ops/runbook-gh-identities.md`. The
Steward escalated to the operator and **asked them to grant a permission they
had already granted.**

The disconfirming evidence was in the repository the whole time: three commits
under `.github/workflows/ci.yml` authored `ken-ci[bot]`. `verify-leader` FOUND
them, and did the right thing — held the documented claim and the
contradicting observation side by side and said *"I can't tell which; flagging
that I don't know rather than guessing."* The Steward did the wrong thing:
explained the observation away as "probably operator-pushed" and acted on the
nine-day-old note.

⇒ **A recorded constraint about MUTABLE EXTERNAL STATE — credentials, scopes,
quotas, infra — is evidence about the past, not a fact about the present.** It
carries a timestamp whether or not one is written down. When an observation
contradicts such a record, **the record is what needs re-verifying, not the
observation that needs explaining away.**

⇒ **The generalization is not "keep notes fresh"** — you cannot notice that a
note went stale, because nothing changes when it does. It is: **for mutable
external state, TEST AT POINT OF USE.** A push either works or it doesn't, and
finding out costs one command. Never escalate a capability claim you have not
tried.

### Historical: the 2026-07-13 rejection (before the grant)

kenfmt capstone C was Architect+QA APPROVED (`91ea984d`) and
honesty-gate-clean, but the scripted publisher
(`scripts/scripted-pr-automerge.sh`) failed at the **branch push** — not CI,
not review:

```
! [remote rejected] wp/kenfmt-c-capstone -> wp/kenfmt-c-capstone
  (refusing to allow a GitHub App to create or update workflow
   `.github/workflows/ci.yml` without `workflows` permission)
error: failed to push some refs
```

The text below is the 2026-07-13 note verbatim. It was true when written
and the permission it describes was granted on 2026-07-21; read its
present tense as of that date.

The publisher GitHub App's token **lacks the `workflows` permission**, so it
cannot create or update **any** file under `.github/workflows/`. The push is
rejected **before** the PR is created — so nothing lands, no PR, `origin/main`
untouched (clean failure, no partial state to unwind). This is independent of the
change's correctness; a one-line CI-step addition is enough to make the whole
branch unpushable via the publisher — which is the **only** GitHub-touching path
(agents' direct `gh` is not authed).

**The rule / preferred design:** enforce CI gates as **workspace tests**, not as
new workflow-file steps. A gate expressed as a `#[test]` that reads the target
files from disk and asserts the property (e.g. read each corpus file, assert
`format(file) == file` / `ken fmt --check` clean, naming any offender) is run by
the **existing** `cargo test --workspace --locked` CI step — so it enforces
day-one and everywhere cargo test runs, **and** the candidate touches no
`.github/workflows/`, so the publisher can push it. This is both more robust
(broader enforcement) and publishable. Frame CI-gate WPs this way from the start.

**How to apply:**
- **Framing:** when a WP's acceptance includes "wire a strict gate into CI,"
  specify it as a workspace test, NOT a `.github/workflows/*.yml` edit. Call this
  out in the frame's guardrails.
- **Honesty-gate:** before publishing, check whether the candidate touches
  `.github/workflows/` (`git diff --name-only <base> <cand> -- '.github/**'`). If
  it does, expect a push rejection — route a gate-relocation re-spin *before*
  attempting the publish, don't burn the failed push.
- **If a real workflow change is genuinely required** (rare), that is an
  **operator action** (grant the App `workflows` permission, or the operator
  lands the workflow file manually) — and it is **security-sensitive**
  (workflow-write = supply-chain surface), so escalate it as an operator decision
  rather than working around it.

Sibling to [[scripted-publisher-target-is-head-branch-never-main]] and the
publisher discipline in the steward skill (scripted publisher path). The
publisher is mechanical and permission-bounded; design merges to stay inside
its permission envelope, and measure that envelope at point of use.
