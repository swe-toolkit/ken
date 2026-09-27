---
id: LANG-SESSION-SCOPE
title: "Give the incremental entry points (elaborate_decl, the REPL, harness declare_postulate_raw and tests' globals.insert) a per-session scope that resolution reads as locals, separate from the prelude's root scope, so no source-visible name depends on the flat globals table outside the prelude; L2-2 of the minimal-prelude program"
status: active
owner: language
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect program design evt_4s5he6tnf3xs4, L2-2: the incremental API and harness postulates bind into a per-session scope read as locals, the one legitimate source-visible use of non-prelude globals. Population: LANG-PRELUDE-FLOOR-FIFTEEN AC-0 (d). Also carries the Architect's ownership note from LANG-QUALIFIED-CONSTRUCTOR-PRIVACY D0 (evt_1nrnwakvjy2gj). Serves the operator's 2026-09-25 one-resolution-mode ruling. Steward-filed per COORDINATION section 2."
---

# Session scope for incremental and harness units

## Objective

A name bound by an incremental or harness entry point lives in a
per-session scope that resolution reads as a local. Outside the prelude,
nothing a source unit can see depends on the flat `globals` table. That is
the last source-visible use of `globals` the L2 flip has to replace.

## Settled inputs -- Architect `evt_4s5he6tnf3xs4`; to measure at the base

- **Population.** `LANG-PRELUDE-FLOOR-FIFTEEN` AC-0 (d) (merged) lists every
  non-source path into `globals`: `declare_postulate_raw` (`lib.rs:379`),
  sequential `elaborate_decl` (`lib.rs:389`), REPL sessions, and tests'
  `globals.insert`. Re-measure it at the base before building, and reconcile
  it against the D0 list.
- **Behaviour is unchanged under the current resolution mode.** Every
  existing incremental, REPL and harness suite still resolves the same names
  to the same `GlobalId`s.
- **Ownership (Architect note on `evt_1nrnwakvjy2gj`).** Ambient user code
  counts as the owner of every prelude family because it shares the
  prelude's root scope: the `owner_member` test at `modules.rs:730` reads the
  root scope's `checked_local_ids`. With a session scope, a session unit is
  not the prelude's scope. The Architect rules at D0 whether the session
  scope also removes that ownership, or whether that waits for the flip.
- **Out of scope.** The floor additions for keyed names, the flip that
  deletes legacy mode and the census, and the no-binding rule.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`ElabEnv` holds a session scope distinct from the prelude's root scope. The
incremental API, the REPL and the harness entry points bind into it, and
resolution reads it as locals ahead of any fall-through to `globals`. Tests
that installed names with `globals.insert` use the session entry point
instead. No kernel, `trusted_base()` or spec change.

## Acceptance

- **AC-0 (D0).** The re-measured population, as (entry point, file, count),
  and the Architect's ruling on the form and on ownership.
- **AC-1 (behavioural).** With the `globals` fall-through disabled in
  scratch for session-bound names only, the incremental, REPL and harness
  suites stay green, which shows they now resolve through the session scope.
  On base the same scratch change reds them. Report the red set as the
  measurement.
- **AC-2 (identity).** A name bound in a session resolves to the exact
  `GlobalId` its binding produced, in expression, pattern and type position.
  A later session binding of the same spelling does not change an earlier
  unit's checked reference.
- **AC-3 (controls).** The strict-resolution census is byte-unchanged. The
  prelude's hidden constructors stay unreachable
  (`lang_qualified_constructor_privacy`). Targeted builds only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Recut: one provenance-tagged session map (Architect `evt_1gz54y177tm3c`)

This supersedes the per-reader session tables of the withdrawn candidates
`914d7f616` and `07f62fe6a`. Build from current `origin/main`. Same WP and
objective; size M, tier T1.

- **R1. Representation.** `ModuleState` holds one `session` map from name to
  `{ id, provenance: Local | Alias | Import { qualified } }`, plus
  `session_prefixes`. At unit start the unit's `Scope` tables are rebuilt
  from it; only a successful unit commit folds that unit's own checked
  locals and imports into it. A failed unit commits nothing. The only
  writers are `commit_unit` and `bind_session_alias`, and every match on
  the provenance is total.
- **R2. Tiers.** Resolution inside a unit is current-unit, then session.
  Distinct ids: session `Local` or `Import` against a later import is
  `AmbiguousReference`; a later local shadows a session `Local` or `Alias`
  but collides with a session `Import`; a later import replaces a session
  `Alias`. The same id is always idempotent.
- **R3. Visibility first.** `bind_session_name` refuses any id that is not
  a pub export of its owner, and `catalog_or::expose_module` iterates the
  export table, not a `globals` prefix scan.
- **R4. Standalone expressions.** In `rewrite_standalone` only, a fully
  qualified `M.x` left unresolved resolves through `exports[M][x]`; a
  non-exported `M.x` is `UnboundName`. Source units keep 33 §3.2 strictly.
- **R5. Children.** Loaded-module fences run in a discarded clone of the
  module scope and never assign `session_scope`.
- **Pins,** each a pair on a shared input with a mutation shown red then
  restored: one per R2 row (mutation: route `Alias` through `bind_local`);
  visibility (Map's non-pub `leq_nat` refused, LC's pub one accepted;
  mutation: restore the prefix scan); standalone (`Core.Logic.Or.Or`
  resolves, a non-pub `M.x` and an unimported source-unit `M.x` do not;
  mutation: drop the export fallback); fence isolation (mutation: reassign
  `session_scope`); and a failed unit leaves the session map byte-equal.
- **Census.** Grep every write to `session` and `session_prefixes`. Name
  and migrate every red R5 causes in the targeted suites, or report it as a
  stop.
- **Handoff evidence (targeted suites only; operator 2026-09-26: no full
  local runs, CI is the whole-repo gate).** The suites the pins live in,
  the five suites of the two CI reds (`purity_keywords`,
  `cat_map_bool_and_owner`, `lang_instance_search_second_path`,
  `lang_membership_operator_surface`, `v3_fo_embedding_adequacy_d1`),
  `lang_session_scope`, and `-p ken-interp --test px8p_checked_buffer`, each
  with the absolute result at the candidate and at its merge-base.
- **Retained:** C1-C3, the D0 pins, the attached-proof rows, the three
  `914d7f616` rows and the `07f62fe6a` pair controls. Stop if any retained
  pin needs a changed expectation rather than a changed mechanism.

## Stop-6 amendment: no door; one disposition per name (Architect `evt_76m3hykd9h4tz`)

Rescoped in place. This supersedes the stop-5 door (inventory line 5).

- **Door withdrawn.** Delete `bind_session_module_private_alias` and the
  `expose_module_private` wrappers. A test's direct `globals.insert` /
  `globals.extend` that lets Ken source resolve a name is the same bypass
  and gets the same treatment.
- **One class per name.** Every P row, door user and source-visible direct
  writer is exactly one of:
  - **E**, a missing export: add `pub` in the provider;
  - **I**, internal: rewrite the probe against the public surface, or move
    the law into the provider. A constructor of an abstractly exported type
    is always I;
  - **H**, a host flat read: key the Rust lookup on the qualified identity.

  The class defaults are in the ruling.
- **Scope added.** `catalog/` `pub` edits for E names land as commit 1, a
  straight-ancestor prefix that may be cut and landed alone (Architect gate).
  They must be green at base on the targets they touch. Commit 2 onward, in
  `crates/`: delete the door, migrate the I and H rows, and point export
  assertions at the module interface (`file_export_ids`).
- **Prefix topology** (Steward, 2026-09-26): the prefix is its own branch,
  `wp/LANG-SESSION-SCOPE-catalog-prefix`, cut from `origin/main`. It is an
  atomic pair: the catalog/ E edits plus, under crates/, only the
  expected-set additions for those names in exact public-interface
  inventory tests (`published_module_surfaces` callers and any similar
  helper). The additions are bounded by the Architect's conditions
  (`evt_tz8x9csmx18t`): additions only, exactly the E set, exactness kept,
  the CLAIMED lines stay true. After the prefix lands, the crates/
  remainder is replayed onto a fresh branch from main, and `2f5577e7f`
  stays as history.
- **Disposition table:** one row per name, giving its class and a
  one-phrase citation. The Architect reviews it with the `catalog/` diff.
- **R rows** (ds4, es4, lang_qualified_constructors, map_build,
  px8f_buffer_io_surface): for each, a first-cause diagnosis (which
  spelling resolved to which identity, at base and at the candidate), then
  a repair under the settled rule. A row returns to the Architect only if
  its repair needs a design choice.
- **Retained expectations** change only for I-class export assertions and
  door-routed probes. This narrows the "changed expectation" stop above; it
  does not remove it.
- **Acceptance:**
  - a grep over `crates/` for the door's names returns nothing;
  - every export assertion reads the interface;
  - the 11 P targets are green;
  - the 5 R rows are diagnosed;
  - direct-writer neuter controls are recorded (candidate acceptance).

## Stop conditions

- Any kernel, `trusted_base()` or spec change, or a changed census row.
- A session-bound name would resolve to a different `GlobalId` than on base.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)

1. A prior-session attached-proof selector resolves by spelling
   (`resolve_attached_ref` -> `RCon` -> `globals`) -- keyed on a selector
   form (`subject::proof`) that is never in `current_local_names` or
   `checked_local_ids` (Architect `evt_5wgr0t36kk780`: record it by checked
   id in the one session ledger, gated by `private_ids`).
2. Three consumers bypass the session ledger: the prop-intro selector
   (never captured), a nested child scope (ledger not propagated), and the
   `InScope` export (reads `globals`) -- keyed on per-reader ID selection
   (Architect `evt_3pv9v8ed0v7we`).

**Shared predicate (Architect `evt_3pv9v8ed0v7we`):** ID selection is
implemented per reader and capture per producer form, with no single
function every consumer calls and no single capture every producer passes
through. The closure is one total-match capture at the sealed root (C1), one
`select_checked_id` that every reader calls with the private gate inside it
(C2), and child scopes inheriting the session ledger read-only (C3). Each
consumer in the handoff census is routed through `select_checked_id`, named
as a non-`globals` carry, or left to the flip.

3. Session selection is fed by writers that are never retracted (Architect
   `evt_5xcdk1h0f4w90`, stop 3, the `914d7f616` CI red; count published
   late at `evt_51bwgshs12vks`).
4. Standalone expressions resolve a fully qualified path through
   source-unit rules, so `Core.Logic.Or.Or` is unbound; and a harness alias
   for Map's non-pub `leq_nat` enters as a current-unit local and collides
   with a later import -- keyed on session entries with no provenance and a
   visibility check after tiering (Architect `evt_51bwgshs12vks`,
   `evt_1gz54y177tm3c`, stop 4, the `07f62fe6a` CI red).

**Predicate for lines 3 and 4** (`evt_51bwgshs12vks`): the persistent
session scope is written by paths that do not own it, and readers cannot
tell an entry's provenance, so current-unit rules apply to it. Lines 1 and 2
are the same predicate from the reader side. The recut above closes it.

5. Retained membership fixtures construct Map-private carriers and proofs
   through harness session exposure -- keyed on a defining-module privilege
   the harness took implicitly through the public-alias door (Architect
   `evt_7sqyq6xbj7fqb`, stop 5). Same predicate as lines 3-4: a writer
   that does not own what it writes. Closure: a named harness door,
   `bind_session_module_private_alias` plus `expose_module_private`, that
   resolves only the defining module's own members and writes `Alias`
   provenance; `bind_session_name` stays public-exports only.

6. The leak masked three classes (missing export in the provider,
   internal probe, host flat read), plus 5 independent resolution
   regressions. Closure: withdraw the door and give each name one
   disposition by rule (Architect `evt_1wgwkd3tdsy0b` stop 6, ruled
   `evt_76m3hykd9h4tz`; census `evt_1rbv3tktxzb78`, `evt_7xjvddha9cg0v`).
7. E defaults (empty-interface) issued over modules whose closeouts assert
   a durable empty surface -- keyed on the consumer-used name set instead of
   the provider's declared contract (Architect `evt_4pv21jmabcjcn`, stop 7;
   Vector and Deque rows become I; sweep by mechanism).

**Map carry, now inside this WP:** Map's `Tree` constructors, `empty` and
`Ordered` are dispositioned E or I by the stop-6 rule. Any remaining
question about a public route to a type (for example `SourceId`) goes to
the Steward as a Foundation API question.
