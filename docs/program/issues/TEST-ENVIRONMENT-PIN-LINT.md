---
id: TEST-ENVIRONMENT-PIN-LINT
title: "Tests pin values that are functions of the whole compiler prelude (absolute GlobalId and vis_origin numbers, absolute semantic-hash literals, whole-environment member counts), so every lawful prelude extension breaks pins outside the extending WP and they surface one CI round at a time: PX10 hit three such stops. Re-key the latent pins by role or relative equality, and add a test-source lint over the three value shapes with an allowed inventory and a seeded-positive control"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: [PX10-SPAWN-PLAN-WAIT]
blocks: []
github: null
origin: "Architect pin-form ruling evt_3w6sxjzdgjqgq item 4 (PX10 §1a stop 3, Research advisory evt_12pb0ckfez8wg): a lint, not an API guard, as a separate WP. L2 (Language + Verify) is the authorized lane; the next L1 and L2 WPs both grow the prelude. Steward-filed per COORDINATION section 2."
---

# Tests do not pin the whole prelude

## Objective

No test under `crates/` asserts a value that changes under a lawful
prelude extension unless that value is the row's subject. A lint test
enumerates every such value in one local run, and it fails on a new
one.

## Settled inputs (Architect sweep of `666c9a81f`, evt_3w6sxjzdgjqgq)

- **The predicate.** A pin's expected value is a function of the whole
  upstream population, not of the property the row claims (PX10 §1b
  entries 1 to 5).
- **The latent rows, at least these** (Architect correction
  evt_4qb287akz98n6). They are green on PX10's tree but break on the next
  prelude growth. PX10 refreshed the first two files' literals rather
  than re-keying them:
  - `crates/ken-cli/tests/px7f_resource_native.rs`: both protocol table
    rows (`664 … 641` and `695 … 684 … 603`, from `:566`) and the
    `owner_origin, 695` assert at `:805`;
  - `crates/ken-cli/tests/rt_escape_second_resource_native.rs:1381`,
    `:1386` and `:1395` (the owner triples and set, the `== 1353`
    branch, and the text at `:1362` and `:1411`);
  - `crates/ken-elaborator/tests/cat_symbolic_algebra_derivative.rs:96`
    (`before.len(), 114`, an absolute trust count).
- **The re-key forms** (Architect, items 1 and 2):
  - look up an id by the operation or role it serves, in the same
    environment, with an exactly-one assertion;
  - compare hashes relatively, and drop the literal;
  - for counts, pin the subject's members by name or the delta across
    the change.
- **The three sweep patterns** are in evt_3w6sxjzdgjqgq, with their hit
  counts on `666c9a81f`: ids 42, hashes 12, counts 9. Pattern (a) misses
  a bare numeric tuple in a table row with no id keyword nearby, which
  is how the px7f and rt_escape rows escaped it.
- **The right form exists.** `rt_parity_native.rs`
  (`key_shape_without_origin_ordinals`) renames each `StaticOriginId(N)`
  to `source#k`.
  - Lawful hits include stored-artifact vectors
    (`checked_core.rs:8391-8392`), algorithm test vectors, spec-anchored
    counts (`lang_prelude_floor_console_char.rs:147`), and hand-built
    synthetic `GlobalId(N)` fixtures.
  - `git grep -P` with a `'crates/*/tests'` pathspec matches no files.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One verify branch, cut after PX10 lands:

1. Every hit the lint reports on the post-PX10 tree that is not an
   allowed entry, re-keyed by the ruled forms. The lint's output sets
   this population, not the list above.
2. A lint test that runs the three patterns over test sources. Every
   hit is either absent or listed in an allowed inventory with one
   reason class: stored-artifact vector, algorithm test vector,
   spec-anchored count, or synthetic fixture id.

## Acceptance

- **AC-1 (seeded positives).** The lint's own test feeds it three
  texts, and it must report all three:
  - the pre-respin text of PX10's two red rows,
    `rt_performing_underived.rs:102` and
    `verify_package_duplicate_top_level_refusal.rs:130` on `42ba094f3`;
  - a bare numeric tuple in a table row with no id keyword, PX10's
    `DOUBLE_RELEASE, 695,` row from `px7f_resource_native.rs:566`.
- **AC-2 (environment invariance).** Each re-keyed row is green on two
  trees whose ids differ: the candidate tip, and main before PX10
  (`42ba094f3`) with only the re-keyed test edits applied.
- **AC-3 (mutation, QA).** Adding back one refreshed literal from
  PX10's diff, such as `owner_origin, 695`, turns the lint red, naming
  that file and line.

## Stop conditions

- A hit whose reason class needs a spec or conformance ruling.
- A re-key that needs production code to expose a lookup it lacks.
- A kernel or trust change.

Checks: 6 fired: the latent population is the Architect's sweep on
`666c9a81f` as corrected by evt_4qb287akz98n6, and `git diff 0fdc10a5d
666c9a81f` shows PX10 refreshing the first two rows' literals (618 to
695 at `px7f_resource_native.rs`). 8 fired: AC-3's mutant restores a
literal that is green on its own tree, so only the lint can turn it
red. A lint validated only on keyword-adjacent positives would pass
with the bare-tuple blind spot, so AC-1 seeds that shape.
