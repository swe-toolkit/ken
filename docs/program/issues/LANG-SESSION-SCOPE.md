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
`lang_membership_operator_surface.rs` were the Map-specific sites in that
increment. Both moved to public abstract-parameter wrappers in the
candidate with the Map fence. A separate door lies under
`map_build_acceptance.rs::mk_env`: its fixture calls
`mk_map_dependency_env_with_provider_owned`, which calls
`catalog_or::load_derived_importing_fixture_many`. That helper roots-loads
LawfulClasses and Derived, then calls `expose_module` on each
(`tests/support/catalog_or.rs:218-219`), copying every qualified global
under each module prefix into mutable flat `env.globals`. `mk_env` remains a
white-box unit for Map's own source, but its dependency fixture crosses the
module boundary.

**Cross-provider fixture door census** (Steward `evt_2amvp527b3kyr`,
Architect `evt_6zc6kc4be78qh`). The caller sweep used `origin/main`
`96ca72119be02dc84771f71d2c8e0cae8dfd192c` and searched `catalog/`,
`crates/`, `r_layer_tests/`, `examples/`, and `conformance/`. All helper
call sites below were in `crates/`; no helper or
`catalog_or::expose_module` calls appeared in the other roots. Nested
`src/r_layer_tests` and CLI tests were included through `crates/`.

`load_derived_importing_fixture_many`
(`tests/support/catalog_or.rs:210-227`) has 16 call sites in 13 test files.
Its single-import wrapper `load_derived_importing_fixture`
(`:229-233`) has 4 call sites in 4 files. The `_many` callers are:

- `tests/cat1_lawful_functors_package.rs`: 2
- `tests/cat5_parsing_package.rs`: 1
- `tests/cat_property_acceptance.rs`: 1
- `tests/cc1_nonempty_validation_acceptance.rs`: 1
- `tests/cc3_parsing_cursor_decoder_acceptance.rs`: 1
- `tests/cc4_diagnostic_core_acceptance.rs`: 1
- `tests/cc5_pretty_doc_acceptance.rs`: 1
- `tests/cc7_argparse_acceptance.rs`: 1
- `tests/cc8_env_config_decoder_acceptance.rs`: 1
- `tests/ds3_sum_combinators_acceptance.rs`: 2
- `tests/ds7_applicative_monad_acceptance.rs`: 1
- `tests/ds8_traversable_acceptance.rs`: 1
- `tests/either_catalog_package_acceptance.rs`: 2

The single-import wrapper callers are:

- `tests/cat_map_bool_and_owner.rs`: 1
- `tests/cc6a_process_arguments_exit_acceptance.rs`: 1
- `tests/es2_acceptance.rs`: 1
- `tests/map_build_acceptance.rs`: 1

The adjacent `load_derived_fixture` (`tests/support/catalog_or.rs:189-204`)
makes the same two exposures and has 14 call sites in 11 files:

- `src/r_layer_tests/ds1_empty_dec_acceptance.rs`: 1
- `tests/cat3_collections_package.rs`: 3
- `tests/cat_sort_insertion_sort_acceptance.rs`: 1
- `tests/cc2_text_codec_numeric_acceptance.rs`: 1
- `tests/compare_ord_lexicographic_acceptance.rs`: 1
- `tests/ds4_list_combinators_acceptance.rs`: 2
- `tests/ds6a_int_deceq_acceptance.rs`: 1
- `tests/es4_classes_acceptance.rs`: 1
- `tests/l3_strings_surface_acceptance.rs`: 1
- `tests/structural_deceq_acceptance.rs`: 1
- `tests/sub1_bytes_structural_view.rs`: 1

The shared `catalog_or::expose_module` has 38 call sites in 11 files,
plus those four calls inside `catalog_or.rs`. The external call sites are:

- `src/r_layer_tests/cat_tier_d_parsing_group_import.rs`: 1 wrapper call,
  reached at 8 local `expose` sites
- `tests/cat5_parsing_package.rs`: 4
- `tests/cat_bsearch_acceptance.rs`: 3
- `tests/cc1_nonempty_validation_acceptance.rs`: 1
- `tests/cc2_text_codec_numeric_acceptance.rs`: 1
- `tests/cc3_parsing_cursor_decoder_acceptance.rs`: 4
- `tests/cc4_diagnostic_core_acceptance.rs`: 4
- `tests/cc7_argparse_acceptance.rs`: 9
- `tests/cc8_env_config_decoder_acceptance.rs`: 8
- `tests/ds9_json_codec_acceptance.rs`: 2
- `tests/sub1b_uint8_deceq.rs`: 1

The generic `expose_module` inventory does not claim that every call targets
LawfulClasses or Derived, or that each caller consumes every private name.
The mechanism sweep also found a separate CC6a `expose_module_aliases` for
Capability modules and an unused local Deque `expose_module` definition;
neither is a caller of `catalog_or::expose_module`. This census changes no
acceptance criteria.

**Per-name `catalog_or` door classification** (Architect
`evt_2pwssda6rb5rx`, amended `evt_1cgncc9j8w2n2`; Steward
`evt_6x7jz7hz6kg9d`, corrected `evt_dj5m6f5x4eke`). The independent sweep
on routed d3 `d3e7beaa8fc487a8da32d4ffe63017caf4b03c66` counted all 76
invocations: 16 `_many`, four single wrappers, 14 Derived fixtures, 38
external `expose_module` calls and four internal calls. The eight local
reach sites of the tier-D generic wrapper are separately enumerated.
The initial per-invocation consumed-name table, with each source file:line,
exposed module, and reached names, is `/tmp/lang-session-catalog-or-per-call.tsv`
(SHA-256 `267d11ee8c5178e46ec5c44f2d952a9f481af58165efbc31e53c036b3c0d9b18`;
convo `evt_ssydn1ky0jtd`). A later lexical audit corrected only the
already-public names in the deferred `cat5_parsing_package.rs:77` row:
`/tmp/lang-session-catalog-or-per-call-audited.tsv` (SHA-256
`87c9bfb2a2809ee63d738e6d14b9ce9e7c502a6a214e4dee14dbb2f02f22f683`;
convo `evt_1250608g5rrcy`). The original Architect-reviewed table remains
unchanged. The correction adds existing-public Parsing names, removes the
unread `span_origin` from that row, and changes no private row, count, or
item-1 scope. A second, no-op-door and dynamic-host-loop audit added six
reached-name groups at already counted invocations, not new calls or pins:
private LC `compare_bool_cases` reached through DS1's helper; public LC
String-key names read dynamically in CC2; public LC `Eq`, `DecEq`, `Ord`, and
`bool_or` read dynamically in ES4; the public Doc `render` used by CC7's
help-growth snippet; and public Derived
`bytes_nat_length` used in SUB1's Ken expression. The separate additive
`/tmp/lang-session-catalog-or-additive.tsv` (SHA-256
`9ab059f0642689e3318094d80e2bb820f1b05b4189cc8ae52153bd7b7fe72b06`;
following discovery `evt_4vyxgzndatjen`) records each test function and site,
plane, and checked-ID route. Neither the ruled 76-row original nor the
49-pin table was overwritten.

The table distinguishes `W`, a public flat alias removed by a fixture to
prove an import; `P`, an existing-public name read through a flat alias
without a client import; `H`, a host flat lookup; and `I`, a private checked
Ken name, constructor, or generated instance. Host-only class-registry
observations are not flat reads. The four internal exposure calls inherit
only the names reached by their 34 Derived/many/single callers, not the
whole copied module. Existing catalog source imports, qualified host reads,
Map's own local `fold`/`insert`/`total_leq_nat`, and InsertionSort's local
`sort` are not attributed to the copied Derived or LawfulClasses aliases.

Private `I` providers and reaching callers, retaining the table's exact
per-row names rather than broadening an interface:

- LawfulClasses: `compare_ord_lexicographic_acceptance.rs:19` reaches
  `compare_raw` and five attached proofs; `ds1_empty_dec_acceptance.rs:379`,
  `cat_sort_insertion_sort_acceptance.rs:22`,
  `cat_bsearch_acceptance.rs:179`, `ds6a_int_deceq_acceptance.rs:24`,
  `structural_deceq_acceptance.rs:15`, `es4_classes_acceptance.rs:96`,
  `cc2_text_codec_numeric_acceptance.rs:46`, and `sub1b_uint8_deceq.rs:21`
  reach the named, unexported `Ord`/`Eq`/`DecEq` dictionaries recorded in
  the table. Generated dictionary identity is checked by loader-owned ID,
  never by an `*_instance_*` spelling alone.
- Derived: `cat3_collections_package.rs:135` invokes private list
  structural/sort/lens names; `ds4_list_combinators_acceptance.rs:23`
  invokes `range`, `range_length`, `zip`, `zip_length`;
  `es4_classes_acceptance.rs:96` invokes the Derived comparator-indexed
  `Perm`, distinct from the prelude's; `l3_strings_surface_acceptance.rs:34`
  invokes `concat`, `slice`, `char_at`, `eq`, `compare`; and
  `cc2_text_codec_numeric_acceptance.rs:46` invokes `compare_char`.
  `cat3_collections_package.rs:245` instead reads public `map`/`filter`.
- Diagnostics.Core: `cat5_parsing_package.rs:52` reaches the private
  `MkSourceId` constructor. `cc4_diagnostic_core_acceptance.rs:44` reaches
  `MkSourceId`, `environment_origin`, and `config_key_origin`.
- Cursor: `cc3_parsing_cursor_decoder_acceptance.rs:57` reaches private
  `arg_cursor_*`, `arg_location_origin*`, and `MkArgCursor` through host
  observations; `cc4_diagnostic_core_acceptance.rs:54` also invokes
  `arg_location_origin` in Ken.
- Decoder: `cc3_parsing_cursor_decoder_acceptance.rs:67` observes private
  combinator/fuel helpers and `DecoderZeroProgress` in its host list;
  `ds9_json_codec_acceptance.rs:592` invokes `decoder_map` in Ken.
- Parsing: `cc3_parsing_cursor_decoder_acceptance.rs:77` and
  `cc4_diagnostic_core_acceptance.rs:74` host-read private cursor, span,
  and grammar helpers; `cat5_parsing_package.rs:77` invokes private
  `bool_expr_eq` and `syntax_leaf` in Ken.
- EffectfulClasses: `cc1_nonempty_validation_acceptance.rs:293` tests
  private `Monad` by a `NoInstance` negative probe.
- Json: `ds9_json_codec_acceptance.rs:549` invokes private `json_size`.

Public `P` without an explicit route is a stop, not an export request.
It includes CAT3/DS4 plain list snippets, CAT-BSEARCH's three exposed
providers, CC3/CC4/CAT5 plain clients, DS9's Json/Decoder snippets, and
Forge's `Diagnostic`, `Doc`, `diagnostic_to_doc` from CC7/CC8. The 20
`_many`/single fixtures remove only their listed public Derived aliases;
LawfulFunctors fixtures also withhold four public `bool_and` aliases. Four
host controls assert the flat public LC `leq_nat` matches its loader-owned
ID. Per-test `W` and `leq_nat` host controls still measure the door and
remain until deletion; the shared fixture's removal control is converted
in item 1 to assert absence after removal, without asserting that the door
exposed an alias first (Architect `evt_12nc3hq6s2pzk`).

**Stop-9 pin census.** The 49-row test/function/file:line/property,
promise-class-annotation and route census is
`/tmp/lang-session-catalog-or-pin-census.tsv` (SHA-256
`c59d0325023228105ff7aba69350354b6effa56cd7b1a2e6bd37a7bada28213f`;
convo `evt_6ffstwbv51jmg`). It covers LawfulClasses (six), Derived
(thirteen), Diagnostics.Core (four), Cursor (five), Decoder (four), Parsing
(five), EffectfulClasses (seven), Json (four), and the cross-catalog ledger
(one). The routes include exact public/loader-visible inventories, private
import refusals, trust closure, owned class/instance populations, all-owned
checked bodies/references, and named-list pins. Twenty-three older tests
have no `Promise class` annotation; the table says `UNLABELLED`, not an
invented quotation. The `cc3:364`, `cc4:210`, and `cat5:307` named lists
and `lang_mod_catalog_evidence_frontier.rs:759` ledger retain their measured
promises. The Derived test-2 four-alias residual remains separate.

The Architect's sequence is eight increments: (1) test-only public routes,
authenticated host observations, generated-instance `where` resolution,
and a controlled EC negative; (2) LC private compare laws in owner examples;
(3) Derived private families in owner examples, split if needed; (4)
Diagnostics.Core/Cursor private observations; (5) Json `json_size` owner
example; (6) Decoder `decoder_map` only if public composition cannot retain
the probe's property; (7) Parsing private examples; (8) the final door
removal, remaining W/`leq_nat` controls, and Derived test-2 disposition.
Every increment returns with exact checked text and rechecks file collisions.
The item-1 migration unit is a whole test function (Architect
`evt_6pa7fhxkbpnrq`): a function with any later private Ken obligation or
per-test door control defers atomically to its latest required increment.
Every changed function must survive a restored, disposable no-op
`expose_module` mutation; a new companion test cannot stand in for it.
CC1's private `Monad` negative cannot yet meet the item-1 EC guard:
`provider_owned_id` finds no qualified `Core.Classes.EffectfulClasses.Monad`
global in that roots-loaded environment. The candidate restores the original
negative untouched; it claims no EC migration or production registry seam.
The guarded spelling-view alternative and its lawful-instance mutation
remain a later exact-text obligation (Architect `evt_78z6k087mxve7`).
Decoder item 6 waits for CAT-DECODER-RECURSIVE-SUCCEEDS; Parsing item 7
waits for BYTES D3. Because BYTES D3 also edits
`tests/cat5_parsing_package.rs` and
`src/r_layer_tests/cat_tier_d_parsing_group_import.rs`, all rows in those
files move to item 7; the cat5:31 W/H controls remain final item 8.
Item 1 does not touch either file. No acceptance criterion or private door
is changed by this factual record.

**Map flat residual (Architect `evt_42yfw3pmj8t7s`, Steward
`evt_72qaxq9bb9er3`).** `mk_env` is retained as an authorized white-box
compilation context for Map's own source, which it elaborates as one unit.
That classification does not cover its dependency fixture: the separate
LawfulClasses and Derived units are flattened by the `catalog_or` helper
in the census above. The historical
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

**Direct-writer census, after item 1** (Steward `evt_6mytrrnr0h1ms`,
Architect `evt_e25945tdk1gd`). At landed item-1 base
`701e13472e057467f195fa69bccbee98d786326a`, a lexical scan of all
`crates/**/*.rs` for the Rust token sequence `globals . insert (` or
`globals . extend (` (arbitrary whitespace, comments and string literals
excluded) found **157 source sites**: 94 production `insert` calls, 55 test
`insert` calls and eight test `extend` calls. The roots included every crate's
integration tests, `ken-elaborator/src/r_layer_tests`,
`src/seal2_tests`, and inline `#[cfg(test)]` modules in `src/elab.rs` and
`src/modules.rs`. No other crate had a site. An independent physical-line
grep found 144 apparent sites; five were comment/string lookalikes and 18
real calls split the receiver from `.insert` across lines, yielding 157.
Each row below is one syntactic writer; loop fan-out and calls through the
shared helper are not extra sites.
`ke/` abbreviates `crates/ken-elaborator/`, and `ki/` abbreviates
`crates/ken-interp/` **in these two tables only**. The independently generated
scan is `/tmp/lang-direct-writer-population.tsv` (SHA-256
`da5b17f203e34ef05db460d7d5eb714f6e58c35556cf9b3e9eb4465ce5df3c68`);
the classified 157-site ledger is
`/tmp/lang-direct-writer-classification.tsv` (SHA-256
`de939dd2838bb3e9014a05349bf65677b0f1caa0e7b187da814bfe69b21d8feb`).
These are additive: the ruled 76-call and 49-pin tables above are unchanged.

`R` denotes a resolution-enabling test writer: it installs, or was written to
install, a flat identity used by Ken or by a host observer. Its row states the
actual provider, the current reach boundary and the later migration item.
`C` denotes a kept control, including literal forgery, intentionally ambient
negative/positive fixtures, and host-only synthetic mirrors that do **not**
supply a Ken import. Calling the latter "forgery" would overstate the
mechanism. A green no-op at an R site means only the stated test or suite
passed **with every other alias route still enabled**. It does not authorize
leaving the writer behind or deleting its consumer. Synthetic test-local
kernel declarations and private TCB escape discriminators have no catalog
provider to import; their item-8 rows require an exact disposition, not a
fabricated public route.

**Test writers: complete per-site disposition and disposable result.** For each
R writer, the source expression was separately replaced by a typed no-op;
its named baseline test passed 1/1 (22 distinct baseline targets), the
mutated target executed exactly one test, and the writer was restored to its
pre-probe SHA-256 before the next run. Sixteen R sites failed 0/1, eight
passed 1/1. `R` does not mean a migration has landed. `C` sites remain
unchanged: some are negative/positive controls; others are host-only mirrors,
not Ken import bypasses. None is a proposed migration probe.
The per-site result ledger is `/tmp/lang-direct-writer-probe-results.tsv`
(SHA-256 `b6ccb89464f8ebc1a96e2d37807437b23b08eb385eaa6144f6b3f84d6cdea3b3`);
`/tmp/lang-direct-writer-probe-<probe>.log` holds the exact command output.
The first four attempted probes were invalid compiler failures caused by
leaving `env.` before the stub, not by the writer. CAT3-575 and CC6b42 used
`env./* per-writer no-op */ None::<ken_kernel::GlobalId>` (E0609: no field
`None`); CAT5-48 and DEQUE-38 used `env./* per-writer no-op */ ()`
(unexpected `(`, E0618: expected function). None ran a test. Their logs remain
at `/tmp/lang-direct-writer-probe-<probe>-invalid-build.log`; each was rerun
with a compile-preserving no-op after removing the receiver. They are not in
the valid 24-run result count. All source blobs were restored and the final
worktree diff contains only this issue appendix.

| Site | Method | Class | Provider or control reason | Route/item | Probe, single target |
|---|---|---|---|---|---|
| `ke/src/elab.rs:19590` | `insert` | R | synthetic checked ShadowSibling for Ken expression, not catalog | 8: local-fixture ruling | ELAB-19590: RED 0/1 |
| `ke/src/elab.rs:19888` | `insert` | R | synthetic raw recursiveOwner for elaborator RExpr, not catalog | 8: local-fixture ruling | ELAB-19888: RED 0/1 |
| `ke/src/modules.rs:4679` | `insert` | C (keep) | host-only extra True alias tests private-roster identity sweep | keep | not mutated |
| `ke/src/modules.rs:4744` | `insert` | C (keep) | forged Proved floor ID must be refused | keep | not mutated |
| `ke/src/modules.rs:4864` | `insert` | C (keep) | forged Proved during strict roots load must be refused | keep | not mutated |
| `ke/src/modules.rs:7363` | `insert` | C (keep) | forged attached-proof key tests file proof selection | keep | not mutated |
| `ke/src/r_layer_tests/cc6b_path_posix_acceptance.rs:42` | `insert` | R | public Capability.Filesystem.Path.Posix; host bare lookup after source import | 8: host-identity rewrite | CC6b42: RED 0/1 |
| `ke/src/seal2_tests/producer_closure.rs:642` | `insert` | C (keep) | synthetic unknown ID tests producer-closure refusal | keep | not mutated |
| `ke/tests/cat1_lawful_functors_package.rs:45` | `insert` | C (keep) | flat cong forged, checked Transport owner must prevail | keep | not mutated |
| `ke/tests/cat1_lawful_functors_package.rs:49` | `insert` | C (keep) | qualified cong forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cat1_lawful_functors_package.rs:75` | `insert` | C (keep) | flat bool_and forged, checked LC owner must prevail | keep | not mutated |
| `ke/tests/cat1_lawful_functors_package.rs:82` | `insert` | C (keep) | qualified bool_and forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cat3_collections_package.rs:575` | `insert` | R | public Data.Numeric.Nat.Order.min renamed cat3_canonical_min for Ken proof | 3: CAT3 whole function | CAT3-575: RED 0/1 |
| `ke/tests/cat5_parsing_package.rs:48` | `extend` | R | Core.Classes.LawfulClasses aliases copied after Derived; CAT5 source boundary | 7: CAT5 after BYTES D3; W/H in 8 | CAT5-48: GREEN 1/1 |
| `ke/tests/cat5_parsing_package.rs:464` | `insert` | C (keep) | flat SourceId forged, Diagnostics owner must prevail | keep | not mutated |
| `ke/tests/cat5_parsing_package.rs:469` | `insert` | C (keep) | qualified Diagnostics SourceId forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cat_deque_acceptance.rs:38` | `extend` | R | Data.Collections.Deque flat helper currently has zero callers | 8: retire dormant fixture | DEQUE-38: GREEN 1/1 |
| `ke/tests/cat_deque_acceptance.rs:141` | `insert` | C (keep) | flat Deque names forged, owned-ID host reads must prevail | keep | not mutated |
| `ke/tests/cat_deque_acceptance.rs:181` | `insert` | C (keep) | qualified Deque.pushFront forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cat_derived_filter_membership_law.rs:52` | `insert` | R | Derived.filter/mem and LC.IsTrue/bool_and; four aliases of Derived test 2 | 8: Derived test-2 disposition | DFILTER-52: RED 0/1 |
| `ke/tests/cat_derived_sort_laws.rs:45` | `insert` | R | Derived count/Perm/insert/sort/laws plus LC IsTrue/bool_or/leq_nat | 3: Derived private sort laws | DSORT-45: RED 0/1 |
| `ke/tests/cat_map_bool_and_owner.rs:213` | `insert` | C (keep) | Map-qualified host observation rebinds Map-owned IDs; no Ken client read | keep | not mutated |
| `ke/tests/cat_vec_acceptance.rs:41` | `extend` | R | Data.Vector.Vector public/private operations in Ken test source | 8: Vector owner/public split | VEC-41: RED 0/1 |
| `ke/tests/cc2_text_codec_numeric_acceptance.rs:42` | `extend` | R | Core.Classes.LawfulClasses only, NOT Derived; checked String keys in CC2 | 2: LC group; door joint control in 8 | CC2-42: GREEN 1/1 |
| `ke/tests/cc3_parsing_cursor_decoder_acceptance.rs:40` | `extend` | R | Core.Classes.LawfulClasses aliases in CC3 Cursor/Decoder/Diagnostics fixture | 4/6/7 consumers; 8 shared W/H | CC3-40: GREEN 1/1 |
| `ke/tests/cc4_diagnostic_core_acceptance.rs:41` | `extend` | R | Core.Classes.LawfulClasses aliases in CC4 Diagnostics/Cursor/Parsing fixture | 4/7 consumers; 8 shared W/H | CC4-41: GREEN 1/1 |
| `ke/tests/cc5_pretty_doc_acceptance.rs:324` | `insert` | C (keep) | flat render forged, checked Doc owner must prevail | keep | not mutated |
| `ke/tests/cc5_pretty_doc_acceptance.rs:326` | `insert` | C (keep) | qualified Doc.render forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cc6a_process_arguments_exit_acceptance.rs:59` | `extend` | R | Capability.Process.Arguments flat aliases; checked host read | 8: Process fixture | CC6a-59: RED 0/1 |
| `ke/tests/cc7_argparse_acceptance.rs:368` | `insert` | C (keep) | flat render forged, checked Doc owner must prevail | keep | not mutated |
| `ke/tests/cc7_argparse_acceptance.rs:373` | `insert` | C (keep) | qualified Doc.render forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/cc8_env_config_decoder_acceptance.rs:393` | `insert` | C (keep) | flat render forged, checked Doc owner must prevail | keep | not mutated |
| `ke/tests/cc8_env_config_decoder_acceptance.rs:398` | `insert` | C (keep) | qualified Doc.render forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/es2_acceptance.rs:310` | `insert` | C (keep) | flat Tree forged, checked Map owner must prevail | keep | not mutated |
| `ke/tests/es2_acceptance.rs:312` | `insert` | C (keep) | flat Map operations forged, checked Map owner must prevail | keep | not mutated |
| `ke/tests/k3_literal_char_view.rs:19` | `insert` | R | public Data.Collections.Derived.map in checked ASCII Ken source | 3: Derived public import | K3-19: RED 0/1 |
| `ke/tests/kernel_conv_congruence_closure.rs:43` | `insert` | R | test-local kernel defs installed by install_def, then Ken fixtures read names | 8: local-fixture ruling | KCONV-43: RED 0/1 |
| `ke/tests/l3_strings_surface_acceptance.rs:36` | `insert` | R | public Data.Numeric.Nat.Order.sub renamed l3_canonical_nat_sub for Ken | 3: L3 whole function | L3-36: RED 0/1 |
| `ke/tests/l3a_acceptance.rs:311` | `insert` | R | public Data.Collections.Derived.map for map_id Ken source | 3: Derived public import | L3a-311: RED 0/1 |
| `ke/tests/lang_mod_strict_resolution_d0.rs:180` | `insert` | C (keep) | deliberate AmbientTrue control: D0 ambient path remains accepted | keep | not mutated |
| `ke/tests/lang_mod_strict_resolution_d1.rs:60` | `insert` | C (keep) | deliberate AmbientTrue control: strict D1 path refuses ambient name | keep | not mutated |
| `ke/tests/lang_prelude_collections.rs:33` | `insert` | R | public Data.Collections.Derived.map/filter for Ken List client | 3: Derived public imports | PRELUDE-33: RED 0/1 |
| `ke/tests/lang_prelude_floor_fifteen.rs:280` | `insert` | C (keep) | forged Top floor ID must be refused | keep | not mutated |
| `ke/tests/lang_qualified_constructors.rs:407` | `insert` | C (keep) | forged ResourceKind.Buffer key must not redirect checked constructor | keep | not mutated |
| `ke/tests/lang_qualified_constructors.rs:450` | `insert` | C (keep) | forged ResourceKind.Buffer key must not create bare member | keep | not mutated |
| `ke/tests/lang_qualified_constructors.rs:491` | `insert` | C (keep) | forged ResourceKind.Buffer key must not capture user type name | keep | not mutated |
| `ke/tests/lang_qualified_constructors.rs:525` | `insert` | C (keep) | forged ResourceKind.Buffer key must not redirect checked imported family | keep | not mutated |
| `ke/tests/lang_qualified_constructors.rs:629` | `insert` | C (keep) | forged Colour.Red key must not redirect checked constructor | keep | not mutated |
| `ke/tests/lang_standard_infix_call_completion.rs:715` | `insert` | C (keep) | forged Bar key must not redirect Foo instance head | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:62` | `insert` | C (keep) | flat Map keys forged, owner check must prevail | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:67` | `insert` | C (keep) | qualified Map keys forged, owned-ID guard rejects | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:72` | `insert` | C (keep) | restore Map keys after forgery for next case | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:293` | `insert` | C (keep) | Map-qualified host binding mirror, no Ken client reads it | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:376` | `insert` | C (keep) | Map-qualified host binding mirror, no Ken client reads it | keep | not mutated |
| `ke/tests/map_build_acceptance.rs:1206` | `insert` | C (keep) | forged succ host key must not spoof structural Map check | keep | not mutated |
| `ke/tests/px8f_buffer_io_surface.rs:374` | `insert` | C (keep) | escape a sealed constructor in host to test closure detector | keep | not mutated |
| `ke/tests/px8f_buffer_io_surface.rs:391` | `insert` | C (keep) | unknown host ID must make closure detector reject | keep | not mutated |
| `ke/tests/support/catalog_or.rs:166` | `insert` | R | Core.Logic.OrdResult/Compare qualified providers copied to flat test scope | 2 compare laws; 8 shared fixture | SUP166: RED 0/1 |
| `ke/tests/support/catalog_or.rs:186` | `extend` | R | catalog_or::expose_module alias door; 76 family invocations classified earlier | 8: remove private door | SUP186: GREEN 1/1 |
| `ke/tests/support/catalog_or.rs:282` | `insert` | R | public LC.bool_and family restored after LawfulFunctors import control | 8: shared fixture | SUP282: GREEN 1/1 |
| `ke/tests/support/catalog_or.rs:318` | `insert` | R | public Core.Logic.Transport cong/sym/trans flat fixture aliases | 8: shared fixture | SUP318: GREEN 1/1 |
| `ki/tests/px8p_checked_buffer.rs:16` | `insert` | R | private prelude resource_release ID fed to Ken escape-discriminator source | 8: private TCB fixture ruling | PX8P-16: RED 0/1 |
| `ki/tests/px8p_checked_buffer.rs:20` | `insert` | R | private prelude buffer_resource ID fed to Ken escape-discriminator source | 8: private TCB fixture ruling | PX8P-20: RED 0/1 |

The following production writes are **not test fixture aliases**. Their exact
sites are counted to keep the test/production boundary explicit. Each line
number is a distinct `insert`; production elaboration, checked prelude
registration, and the `checked_core` reverse ID→symbol map are not muted or
scheduled for this test-only WP. The list is exhaustive for these two
mechanisms across `crates/` at the measured base.

| Production file | Exact `insert` line numbers | Role |
|---|---|---|
| `ke/src/bytes.rs` | `57`, `62`, `88`, `103`, `118`, `145`, `194`, `237`, `248`, `274`, `294` | built-in Bytes/String and checked conversions |
| `ke/src/checked_core.rs` | `140` | reverse GlobalId→symbol map, not a Ken spelling map |
| `ke/src/conversions.rs` | `134`, `198`, `340` | production conversion registration |
| `ke/src/data.rs` | `99`, `114`, `298`, `311` | data and constructor registration |
| `ke/src/decimal_char.rs` | `133` | decimal-character primitive registration |
| `ke/src/elab.rs` | `12224`, `12329`, `12338`, `12408`, `12545`, `13039`, `13141`, `13424`, `13715`, `13812`, `13898`, `14040`, `14411`, `14575`, `14614`, `14669`, `14764`, `15020`, `15079`, `15098` | checked source declaration and instance registration |
| `ke/src/foreign.rs` | `176` | foreign declaration registration |
| `ke/src/lib.rs` | `382` | ElabEnv raw declaration registration |
| `ke/src/modules.rs` | `269` | production module-qualified binding registration |
| `ke/src/numbers.rs` | `288`, `304`, `320`, `336`, `405`, `406`, `485`, `489`, `492` | numeric and Bool primitive registration |
| `ke/src/prelude.rs` | `722`, `723`, `724`, `781`, `782`, `783`, `787`, `788`, `789`, `793`, `797`, `802`, `821`, `837`, `854`, `870`, `882`, `914`, `932`, `977`, `996`, `1015`, `1032`, `1042`, `1132`, `1137`, `1156`, `1180`, `1210`, `1229`, `1248`, `1479`, `1487`, `1489`, `1521`, `1565`, `1760`, `1843`, `2129`, `2164`, `2939`, `2946` | kernel/prelude floor bootstrap |

**No-op attribution and the live-door confounder.** Source-restored full-suite
controls show CAT5:48 21/21, CC2:42 5/5, CC3:40 6/6, CC4:41 7/7, DEQUE:38
6/6, shared LC restore SUP282 7/7 and Transport SUP318 7/7 with **only that
writer** no-opped. Those seven green results do not prove public imports
suffice: the live `catalog_or::expose_module` still supplies other flat
aliases. Compare-ord with shared SUP186 no-opped was 1/4: only the already
migrated Pair/List function passed; `compare_raw`, the structural-law test and
the non-Bool list test failed on `compare_raw`, `pair_ord_leq`, and
`list_ord_leq`. All eight baselines passed completely and all were restored.
The exact full-suite results are
`/tmp/lang-direct-writer-full-results.tsv` (SHA-256
`38f79ca12bac71080e5ce7bd52b14ca60974e7323788a7e20c09ebbe4fb15cac`).

A separate **coupled** discriminator no-opped each LC local `extend` together
with the shared `expose_module` writer, one CC2/CC3/CC4/CAT5 binary at a
time. Baselines passed 5/5, 6/6, 7/7, 21/21 respectively; coupled runs
passed 4/5, 0/6, 3/7, 0/21. CC2's failure was explicitly
`UnresolvedCon compare_char` from **Derived**, not proof that its local
LawfulClasses writer was needed. Other failures include the per-test
`leq_nat` controls and deferred private Ken names. This compound probe
measures fallback reachability, not the independent necessity of an LC
writer; both subject files were restored byte-identically each time.
`/tmp/lang-direct-writer-coupled-results.tsv` (SHA-256
`053ca7bea8f0630e86ec10b51f7c7241fbbc5ac3cc93892b1a92158b65de99f8`)
and its `/tmp/lang-direct-writer-coupled-<probe>.log` files retain the results.

Decoder provider work landed at `00ffedcfd`; item 6 no longer waits for
that landing, but still waits for item 5's probe. The CAT5/tier-D item-7
file collision still waits for BYTES D3. No item 6 or 7 work begins here.

The first-cause single-site reds include public Ken names
`cat3_canonical_min`, Derived `count`/`map`/`filter`, Vector `Vec`, and private
PX8p prelude names as `UnresolvedCon`, plus a CC6b missing host `MkPath` and
a CC6a missing host `process_arguments`. The Derived test-2 no-op first hits
its preexisting flat host `filter` assertion; **that failure alone does not
prove** the subsequent Ken `mem`/`IsTrue`/`bool_and` resolutions. Each synthetic
kernel fixture red is local source/RExpr name resolution, not a catalog export
failure. Item 2 remains unstarted. Before item 8 can delete the door, its
owner must disposition every R row together with the per-test W/H controls,
without deleting the C controls or confusing a local synthetic declaration
with an importable catalog export. No acceptance criterion changes here.
