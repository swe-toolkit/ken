---
id: VERIFY-PACKAGE-DUPLICATE-TOP-LEVEL-REFUSAL
title: "A ken example fence, or a later plain source, may redefine an admitted top-level name: the duplicate check is per elaboration unit, and stable_symbols_for_env names ids by spelling from the flat globals table, so the admitted declaration falls to the owner#ordinal fallback, core_semantic_hash moves, and a redefined target becomes unselectable. Refuse a duplicate top-level name across units of one package"
status: active
owner: verify
size: S
tier: T1
gate: architect
depends_on: [VERIFY-PACKAGE-EXAMPLE-BINDING-SCOPE]
blocks: []
github: null
origin: "Adversary finding evt_3vjznkjfzawy7 on e1b609597 (spec/40-runtime/46-checked-core-package.md section 3.2). Pre-existing; latent in the catalog (0 clashes in 60 .ken.md files). Steward-filed per COORDINATION section 2."
---

# A package has one declaration per top-level name

## Objective

A top-level name that is already bound in the package is refused with
`DuplicateDefinition`. This holds whether the second binding is in an
example fence or in a later source. Admitted stable symbols and
`core_semantic_hash` do not depend on example names.

## Settled inputs (Adversary `evt_3vjznkjfzawy7`, at `e1b609597`)

- **The check is per unit.** `DuplicateDefinition` uses `unit_definitions`
  (`resolve.rs:1026-1031`). Each example fence is its own
  `elaborate_file_v1` unit (`lib.rs:701-708`).
- **Naming is by spelling.** `stable_symbols_for_env` names ids from
  `env.globals` (`compiler_driver.rs:4197-4202`), and the displaced id
  takes the `owner#ordinal` fallback (`:4219-4227`).
- **Repro** (`compile_ken_source`, NonRuntime, target `main`):
  - `const base : Bool = True` and `const main : Bool = base` give hash
    `48e43379ae5466ab`. An unrelated example leaves the hash unchanged.
  - An example `const base : Bool = False` gives hash `7b980c10618353f4`
    and decls `[base#0, main]`.
  - An example `const main : Bool = False` gives `MissingTarget { main }`.
- **Plain duplicates.** Within one source, across fences, the duplicate is
  refused. Two plain package sources that each define `base` are not
  refused; they give decls `[base, base#0, main]`.
- **The pin.** `denotation_excludes_example_only_checked_string_literal`
  (`compiler_driver.rs` ~:7990) holds only because its example name is
  fresh.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect rules the seam at D0. The options are a package-wide
duplicate check in the driver, or a check in the resolver over the
session's bound names. The repair refuses the example and cross-source
duplicates, and stable-symbol naming no longer lets a later binding take
an admitted declaration's spelling.

- **Ruled closure (`evt_5nxr5z4pgf6ke`).** The package's name set is
  the names declared by elaboration entered after the end of
  `ElabEnv::empty()`, the existing "before the package could speak"
  boundary: `package_definitions` is cleared there. A census gate counts
  post-boundary `expand_and_elaborate` callers by kind over the AC-3
  runs; any caller outside root units, `.ken.md` parts, example and
  reject fences and test `elaborate_decl*` is a stop to the Architect,
  and the only case for per-declaration provenance.
- **A REPL session is not a package (`evt_7rjpnjvnrxfjm`).** Interactive
  re-entry keeps its documented shadowing policy. `ElabEnv` gains
  `elaborate_session_decl_results_v1`, which neither seeds nor records
  the package set, and `do_def` in `crates/ken-cli/src/repl.rs` (one
  added path) calls it. Duplicates inside one declaration are still
  refused.

## Acceptance

- **AC-1.** Each repro row with a shadowing example, and the two-source
  plain duplicate, is refused with `DuplicateDefinition` naming the name.
  The module route already refuses at the base through the loaded
  entry's `root_scope` prebinding (`evt_3c4aa4cbvgbx6`); these rows are
  controls. The checked-fence executor passes `DuplicateDefinition` and
  `AmbiguousReference` through unchanged, so the shadowing row gives
  `AmbiguousReference { base, sources: [Entry.base, base] }`, the
  constructor row (`data U = ZzBase | ZzOther`, then an example
  `data T = ZzBase | ZzQ`) gives the same for `ZzBase`, and a fresh name
  `zz_fresh` stays Ok. Consumers asserting the generic example error are
  migrated or are a stop.
- **AC-1a (test consumers, PR #4648 red, `evt_1za9gv23ehmzd`).** A test
  that re-admits a fixed name on one env is migrated to distinct names,
  never to the session entry (inventory `evt_t9hn8e3qwg1x`: 0
  session-style). Two paths join the scope:
  `ken-elaborator/tests/l3_strings_roundtrip_acceptance.rs` indexes
  `t_ac1`, `t_ac2`, `t_ac3` and `t_ac3_guard` per corpus row (suite 9/9),
  and `ken-interp/tests/omega_erasure_cache_rollback.rs` gives its second
  declaration a fresh name and keeps the same-`GlobalId` assertion (1/1).
- **AC-2 (control).** The unrelated-example row keeps hash
  `48e43379ae5466ab`, and the existing example and package-route rows keep
  their results. `rt_dasm_d1b_role_a_role_authority` is green with its
  original `DuplicateConstructorSpelling`, and a fresh `ElabEnv::new()`
  has an empty `package_definitions`.
- **AC-3.** A catalog and corpus census at the base shows no package newly
  refused. Any hit is the stop below.
- **AC-4 (mutation, QA).** Restoring the per-unit-only check reddens the
  package-route rows of AC-1.
- **AC-R1.** `repl_redefinition_shadows_the_previous_binding` is green:
  a re-entered name gets a new `GlobalId` and the old one is not
  retracted. Reverting only `do_def`'s call reddens it.
- **AC-R2.** The package-route ACs, the constructor guard, the
  prelude-boundary and rollback units and the REPL rollback test stay
  green.
- **AC-R3.** Every other production session-style caller is reported by
  name with its re-entry policy (for example `ken-verify`'s
  `scenario.rs`). One that documents shadowing without the session entry
  is a stop to the Architect.

## Stop conditions

- A catalog or corpus package is newly refused: stop with the list.
- The repair needs a kernel, trust or spec change.

## Hard-stop inventory

- **§1a:** 3 (`evt_1mk5h1xd5zaz`, `evt_3c4aa4cbvgbx6`,
  `evt_189f55fagz339`). Research advisory `evt_4rzm6r59rnbb8`; the next
  research hold is at the sixth.
- **1.** Module-route seed keyed on the loader's module-qualified
  namespace (`Entry.base`) while checked fences resolve and check in the
  root namespace (`base`). Ruled: strip exactly the `entry.` prefix when
  seeding, keep unprefixed members verbatim, and stop on any member with
  another module path.
- **2.** Module-route hook keyed on the resolver's definition set, while
  the loaded entry's `root_scope` prebinding refuses the redefinition
  first (`AmbiguousReference`); the D0 probe bypassed `root_scope`, so it
  measured a configuration the fence never runs. Ruled: delete the
  module-route hook and the entry-1 projection; prebind precedence is
  unchanged.
- **3.** `package_definitions` populated by prelude registration, which
  passes the same `expand_and_elaborate` funnel as package sources;
  keyed on the funnel, not on the package boundary.
- **§1b at entry 3: YES** (`evt_189f55fagz339`). Package membership was
  inferred from the resolver funnel a name passes through, not from the
  boundary that defines the package. Closure: anchor the set at the end
  of `ElabEnv::empty()` (above).
