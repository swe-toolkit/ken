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
- **Remainder scope** (Steward `evt_6222gyxzjgv1g`): `catalog/` edits are
  allowed only to move an I-class law into its own provider as an
  unexported theorem: no new `pub`, no interface widening, no TCB change.
  The Architect rules the remedy per row (`evt_31hre3qw20sk`). The
  remainder lands as straight-ancestor increments on one branch, each
  migrating one provider group off the door while the door exists; the
  last deletes the door.
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
8. durable exact-inventory guard omits a publication route (`ExportDecl`)
   that its own claim covers — keyed on declaration form.
9. provider-side remedy (unexported theorems) conflicts with
   owned-declaration inventory pins the ruling did not enumerate — keyed on
   per-provider test pins of mixed promise class.

**Predicate for lines 7-9** (Architect `evt_byvjqbge371e`, stop 9): a
provider's contract lives in scattered per-provider test pins of mixed
property (public set, exports, owned set, privacy probes, trust, body
populations) and promise class (durable, transition sentinel, snapshot),
and each ruling was issued before the affected provider's pins were
enumerated. Closure: before any further remedy ruling, one mechanical pin
census for every provider the remaining increments touch (provider,
test::fn and file:line, property, quoted promise class, routes queried),
grepped over every root and by mechanism; the Architect rules remedies
against that table. The census is the first step of the next increment,
not a separate node.

**Stop-9 ruling** (Architect `evt_677gx0f0xb6jc`, on Research
`evt_rh4mcmpk097x`): a catalog provider's contract is its public surface,
exports, loader-visible inventory and trust closure; every pin asserting
those stays byte-identical, and moving one is a stop. Owned-set and
all-owned-body population pins may move by addition only, in the increment
that adds the unexported provider theorems, with the prescribed doc-comment
sentence. Named-list body pins do not move; an id- or position-keyed pin
that would move is a stop. The census goes to the Architect for one-pass
confirmation before any provider edit.

10. H rule keyed on qualified identity applied to class declarations, whose
    globals and class-env keys are bare spellings — keyed on the declaration
    plane (class vs value).

**Stop-10 route** (Architect `evt_ax2kbwvatm8f`): the class H rows select
from Derived's owned id population via `class_by_id` and the owner name,
exactly one match, with no new `(module,name) → GlobalId` accessor. The
spelling-keyed class registration itself is
`LANG-CLASS-IDENTITY-BY-CHECKED-ID`, not this WP.

11. H function host reads keyed on qualified spelling in the mutable flat
    `globals` table — keyed on spelling.

**Predicate for lines 10-11 and closure** (Architect `evt_6qvbhqywfmbne`,
stop 11): a provider identity looked up by spelling in a mutable flat table.
Every H read goes through one helper, `provider_owned_id` in
`tests/support/catalog_or.rs`, which finds a candidate by qualified spelling
and checks it against the provider's owned-id population from the real
loader. No H row reads `env.globals` directly for a provider identity; a
third instance means the closure was bypassed. The forged-alias pins forge
both the flat and the qualified key. The cross-unit inline-path clash is
`SPEC-MODULE-PATH-SINGLE-OWNER`, not this WP.

12. prescribed a theorem form and an `= Refl` terminal in words, never
    checked; the type-position lambda does not parse, and the closed Bool
    equality needs `Proved`. Keyed on: form not written in the code's
    vocabulary.

**Predicate for lines 10-12 and closure** (Architect `evt_7gjadvjt4bsgw`,
stop 12): each ruled remedy prescribed a form or plane in words that was
never written and checked in the code's own vocabulary first (Check 4 on the
ruling side). Closure for the rest of this WP: the Architect rules only on
implementer-supplied, checked text, meaning the exact declaration or test plus
the targeted command that elaborates it, green or with its exact error.

**Stop-12 ruling** (Architect `evt_7x0ar6g90f0tp`, on Research
`evt_676fs7226bmyp`): a closed ground-type theorem uses the terminal its
reduced proposition needs (`Proved` for a closed Bool equality), never `Refl`
by default. No helper `fn` enters Vector's owned inventory. Route (iii): one
`ken example` fence in Vector.ken.md holds fence-local Bool helpers with
distinct spellings and the closed map/zip theorems. It must (a) leave the
owned-name and owned-id sets unchanged when measured after the fences run,
(b) redden the closeout when one expected value changes, and (c) use no
prelude or catalog spelling. If (iii) fails any of these, the rows fall back
to H: they stay test-local, with no Vector provider edit, and the measured
failure is recorded here. A (iii) failure is not a new stop.

**Deque door-use census (Deque increment).** A grep across `crates/*/tests`
plus `crates/*/src/r_layer_tests` found one Deque exposure call site before
migration: `cat_deque_acceptance::loaded_env_with_owned` copied 28 qualified
private names into bare globals. Five tests reached that helper, but only two
source probes consumed the private bare names: the four generic law
applications (I4) and five concrete order observations (I5). After moving
I4/I5 to the checked Deque-local example fence, there are zero Deque
exposure call sites and zero source consumers of those aliases. The host
forgery controls (E1/E2) and Derived occurrence check (H1) use checked
owner IDs; the local exposure helper definition remains unused until the
final door-deletion increment. No `r_layer_tests` consumer used Deque aliases.

13. assigned Map's stored-vs-fresh comparator ground case to the
    membership-operator client and ruled a public-only re-expression, without
    checking the public surface could construct a witness. Keyed on:
    ownership of a property read off the test file's location, not the
    declaring module.

**Stop-13 ruling** (Architect `evt_3ry3d7c5pptj9`, then `evt_4xtksh29xcm9k`):
the ground stored-vs-fresh witnesses are Map's own instance semantics
(`Map.ken.md:216-238`) and join the accepted 32-line slice in Map's fence;
the client keeps only the `∈` surface facts through a public abstract
wrapper. The accepted discriminating mutation is a fresh ascending
comparator at the observation site. A stored ascending view of the same tree
is kernel-refused at the lawful `Ord` record and at the `Ordered` witness, so
the stored view cannot be falsified without trust. Next trigger 15.

**Map carry, now inside this WP:** Map's `Tree` constructors, `empty` and
`Ordered` are dispositioned E or I by the stop-6 rule. Any remaining
question about a public route to a type (for example `SourceId`) goes to
the Steward as a Foundation API question.

**Map ground-witness evidence (Architect `evt_4xtksh29xcm9k`, stop 13).**
The Map-owned example fence carries four generic laws plus a lawful
stored-descending versus fresh-ascending comparison on one concrete tree.
The old import-first proposal failed 0/1 with `AmbiguousReference leq_nat`:
Map's private `leq_nat` and the imported LawfulClasses `leq_nat` collided.
The accepted import-free replacement passed the real roots/fence test 1/1.

Changing only the view dictionary's `leq` to the ascending projection
failed 0/1 with `KernelRejected TypeMismatch` at the lawful `Ord` record;
it never reached the named ground theorem. Changing all dictionary laws
to the lawful ascending orientation instead failed 0/1 with
`KernelRejected TypeMismatch` at the tree's `Ordered` witness; it also
never reached the named ground theorem. These two earlier refusals guard
the construction boundary, not the ground theorem's liveness. Replacing
the stored-view observation with a fresh ascending `member` call on the
same descending tree failed 0/1 exactly at
`map_example_stored_comparator_finds_the_key`; restoring the observation
made the roots/fence test pass 1/1. None of the three mutations added
trust, widened Map's public surface, or changed its imports.

**Map door-use census.** The two `expose_module` sites in
`lang_membership_operator_surface.rs` were the only Map door users in this
increment; both move to public abstract-parameter wrappers in the same
candidate as the Map fence. The older `map_build_acceptance.rs::mk_env`
fixture elaborates the complete Map source as a flat legacy unit; it is a
separate census residual, not a third door user, and stays unchanged.

**Map flat residual (Architect `evt_42yfw3pmj8t7s`, Steward
`evt_72qaxq9bb9er3`).** `mk_env` is retained as an authorized white-box
compilation context: it elaborates Map's own text as one unit, not through
a door into a separately loaded unit. The historical
`cat_rel_reachable_within_has_exact_fuel_recurrence` failure is the named
regression witness. On a future session-scope recut, any change in the
`GlobalId` that bare `fold` selects inside that flat Map unit is a measured-ID
stop to the Architect. The selected ID in the withdrawn `2f5577e7f` failure
was not measured; it must not be inferred from `LambdaVsNonFunction`.

**Derived private-law cut (Architect `evt_23jbdb2ztb02v`, Steward
`evt_74kdvrmck3ywr`).** The two generic applications of private
`mem_filter` and `mem_filter_sound` move to a checked Derived-owner example
without exposing either law. Derived test 2
`compatibility_premise_distinguishes_true_and_false_instances` retains 4
direct aliases (filter, mem, IsTrue, bool_and) pending an exact checked
disposition before door deletion.

**Map Membership instance-coverage carry** (Steward `evt_1r4894xhbzs8q`,
Architect `evt_2e6zz0bme4fa2` and `evt_2bv0b64p745s7`). On landed Map
`b6402c71c`, the ground `Membership` observations called the two adapter
helpers directly rather than the instance `member` bodies. Independent
use-site-`Ord` rewrites of `OrderedKeyMembership` and
`RelationEdgeMembership` left those old observations green; the parent's
stored-comparator witness failed. Candidate SHA
`554401812722abe66340d89ca0e5404952e894da` adds two positive `ken example`
observations through carrier-typed `Membership` dictionaries, reaching
distinct loader-owned instances. Each use-site-`Ord` rewrite reddened its
corresponding new theorem with `KernelRejected TypeMismatch`. These rows
discriminate stored from use-site
comparator only: both return `True`, so an instance that ignores its view and
returns constant `True` is outside their scope. Existing absent-query rows
still use helpers. This is a carry item, not a new node or acceptance-criteria
change.
