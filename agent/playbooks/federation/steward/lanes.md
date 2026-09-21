# Live lane roster

Current operational state only. The operator owns lane count, ordering, and
objectives. The Steward updates this file only when one of those fields changes
and cites the ruling that changed it.

Hard limit: 80 lines. Replace stale text; never append history. Do not record
completed work, review transcripts, measurements, explanations, prior states, or
superseded instructions. Git and the WP thread are the history.

## Live roster direction

These rulings remain operative and are retained verbatim.

- **2026-08-22:** "playbook = stable discipline, this file = mutable roster."
- **2026-08-25:** "there are three lanes authorized right now. language (lane
  2) was unblocking rt on priority, and when that was done should have unblocked
  foundation (lane 3) with module/import."
- **2026-09-05:** "btw the trial is over. 3 lanes works with some contention.
  retire the idea that there is still a trial running."

## Live objective direction

- **2026-09-12, Route B:** L2's next deliverable is
  `LANG-ACTIVE-PREMISE-KERNEL-VIEW`, the contextual kernel-query boundary split
  out of `LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS`. **Discharged — that node is
  merged.**
- **2026-09-19, "g then remeasure":** selects option (g) on the A1 re-baseline
  fork and directs that the measurement follow the repair. **DISCHARGED — both
  halves landed at `6a36cfbdd`.** L2 has no live operator objective; see its
  row for the standing direction I sequenced it against.

## Catalog proof direction

- **2026-09-13:** "The proofs are not done. A catalog package is not finished
  until its proofs are complete. Declaring that they are tested computation is
  less valuable than proven correct behavior."
- **2026-09-13:** "computational tests are not part of the package, so a reader
  cannot trust them as intrinsics, but must believe that the implementation of
  both the package and its tests was done correctly. This is inherently
  inferior and weakens the promise that ken strives to make."
- **2026-09-13:** "schedule the proof backfill before extending the catalog."

## Runtime and streamline direction

- **2026-09-17:** "Is L1 still working on clearing the ignored tests? That is
  the top priority until it is done."
- **2026-09-19:** "It is too slow. You need to streamline the process and focus
  on forward movement, not management."

The 2026-09-19 ruling means frames stay short; investigation is the first step
of a repair, not a separate report node; label and wording hygiene do not enter
a lane.

## Authorized roster

Three lanes, in priority order:

1. Runtime
2. Language
3. Foundation

Finished accepted work still routes immediately, regardless of lane. No other
ring starts without an operator lane change.

## Current state

| Lane | Ring | Objective | Active WP | Next WP | Blocker / next action |
|---|---|---|---|---|---|
| L1 | runtime | Clear the selected ignored runtime rows | `RT-GRAFTED-SPINE-CONTROL-GRAPH` -- RELEASED and kicked `evt_1aeb3rv27sx6b`, thread `thr_4smw1pgkegngv`, base `dcbc24c33`. Behavior-inert precursor, L, T1, own hard-stop counter. `RT-PLANNER-KRET-GRAFTED-SPINE` stays PARKED at hard stop 15 (`evt_7bff93f1zg2jh`); `RT-DUPLICATED-RESPONSE-BLOCK` stays `active` on its remaining rows after the partial landed `ea117ce04` | `RT-PLANNER-KRET-GRAFTED-SPINE` -- ONE final atomic D0+D1+D2 attempt once the precursor lands, then park again. Counts stay 15/16; the one-attempt bound is NOT spent on the precursor | **The ruling's boundary symbols are CANDIDATE-SIDE.** `ReleaseEmissionSite`, `claim_word`, `PendingReleaseContextEdge`, `ReleasePlacementLedger` and `record_finished_function` occur ZERO times under `crates/` on main and exist only on rejected `7b03f258c`; the precursor introduces its own function/body scoping type rather than importing one. The kick was posted 90s before the ring compaction and DROPPED; re-delivered at `evt_3zgvt29s3ajsm`. Confirm the leader reaches Working -- a post is not a pickup. |
| L2 | language | Language surface. The operator's 2026-08-25 "unblock foundation with module/import" direction is DISCHARGED -- all 13 `LANG-MOD-*` members AND `CAT-GCD-REFACTOR` are `merged`; the root `LANG-MODULE-IMPORT-SYSTEM` sits `draft` only because a campaign root is never itself released | `LANG-FOREIGN-NAME-FORMAT-CHARS` -- `active`, RELEASED `evt_5kq717yjahm9d` on the amended frame landed `df6f94640`. Realizes normative `spec/30-surface/31-lexical.md §1f`, zero enforcement in `crates/` | `LANG-R-LAYER-EXPORT-RETRACTION` -- FRAMED `ready` in this commit, S, T2. Kick when FORMAT-CHARS closes; Architect review NOT required, the design fork is already ruled | **FORMAT-CHARS: the ruled mechanism is the crate-private `ValidatedSource` capability** -- NOT a `Lexer::new` pending-error arm, NOT per-context checks. `db550884` approval state is DEAD; §1a count stays 0. My "no `kenfmt` change" boundary was MY defect, withdrawn. U+00AD in `constructors.rs` (L1-owned) and `ds-campaign-judgment-log.md` stays OUT of scope. **Two language nodes read as live and are NOT:** `LANG-MATCH-PATTERN-FORMS-ABSENT` has all six slices `merged` and `LANG-ATOM-START-CLASSIFICATION-CLOSURE` is a bookkeeping flip owed on `evt_1n4k31gecsfty`, not work. Neither is a successor. |
| L3 | foundation | Complete catalog proof backfill before catalog extension | `CAT-PARSING-NUMERIC-LAWS` -- kicked `evt_188hq5vf8t38e`, thread `thr_7g2559s7fnpac`, base `83b2f9f0f`, implementer holding `wp/CAT-PARSING-NUMERIC-LAWS` | `CAT-CONFIGURATION-DECODER-LAWS` -- FRAMED `draft` in this commit, M, T1. Flip `ready` and kick when NUMERIC-LAWS closes | **The successor was MEASURED, not taken from the survey.** The survey proposes four parallel targets and names none of them as the dangerous one; measurement found that `env_config_validation` DISCARDS the validation's own `Valid` values and recomputes them via a second independent traversal whose `None` branch emits an empty-`Bytes` placeholder. Nothing connects `Valid` to lookup success, so a missing required field could decode to empty bytes. Its precursor `CAT-SCHEMA-LAWS` merged `47d770216` and published `schema_validate_fields::valid_coverage`, which is the lever. **If the agreement is FALSE that is a product defect and returns to me before any repair.** |

## Update rule

Retain exactly one value for each table field. An update deletes the old value.
Link the WP issue or thread instead of copying detail here. Verify a row against
its issue file and current `origin/main` before acting. Correct stale state in
the next product-attached update; never publish a standalone currency commit.