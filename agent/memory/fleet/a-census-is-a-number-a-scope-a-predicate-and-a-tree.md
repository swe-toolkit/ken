---
name: a-census-is-a-number-a-scope-a-predicate-and-a-tree
description: A census or blast-radius claim states a number together with a scope, a predicate and a measurement tree, and only the number reads as a measurement. Six instances where the count was exact or nearly so and the scope (a path list, the diff, the control in view, a hand-named pair), the unstated predicate, or the tree it was taken at was the defect. Recount the boundary, name the predicate, re-run the enumeration on the tree it will land on.
metadata:
  type: feedback
---

# A census is a number, a scope, a predicate and a tree

A census claim reads as one fact: *"40 call sites across X and Y, unclassified
population zero"*, *"the two stale couplings"*, *"three of eighteen discard the
result"*, *"the tests in the blast radius"*. It is four fields:

| field | how it goes wrong | what to do |
|---|---|---|
| **number** | rarely: whoever wrote it already checked it | recount it, but do not stop there |
| **scope** | chosen by where the diagnosis started, by what the diff changed, or by the control in view | recount the boundary, not only the members inside it |
| **predicate** | unstated, so the count cannot be retaken; or one of two mechanisms | write down what was counted |
| **tree** | the census was right at the tree it was measured on | re-run it on the tree the work will land on |

The number is the half that has already been checked. **The scope reads as
context, not as a claim, and it is the axis nobody recounts.**

## Instances

- **Scope as a path list** (2026-08-14, `2ca91a3a`, `RT-TEST-SCRATCH-RAII`).
  Claim: 40 `std::env::temp_dir()` sites across `crates/ken-runtime/src/` and
  `crates/ken-cli/tests/`, classified 4 fixed-name + 18 guarded + 18 migrated,
  unclassified zero; a parallel `CARGO_TARGET_TMPDIR` census of 14. Both counts
  recounted exact. And `temp_dir()` had 73 occurrences in 35 files: 33 in 13
  files in four other crates, **28 with no drop guard**, one of them the node's
  own defect sentence word for word (a helper returning a bare `PathBuf` from a
  `pid`+`nanos`-suffixed `create_dir_all`, called by nine tests, cleaned only by
  a trailing statement on the success path). The two directories were where the
  diagnosis started.
- **Scope as the diff** (2026-08-17). A should-fix named "all 13 newly
  un-ignored tests retain a leading comment block that is now false". Measured:
  package `#[ignore]` 33 to 29 (4 un-ignored); stale owner-blocks above a
  running test, 5. The 13 was real and belonged to another population (the same
  notification's "13 pass; 16 carry a later refusal" over 29 governed rows). The
  fifth stale block sat above a test that was already running, so a repair
  scoped to "newly un-ignored" misses it by construction.
- **Scope as the control in view** (2026-08-14, `bc62216a`,
  `RT-LEXICAL-R3-FUSION-EMITTER`). A residual was filed against one control's
  two-root coverage. The candidate had added nine controls, each iterating the
  identical hard-coded two-element literal, with nine doc blocks reading "on
  both armed roots". The comment was the population definition for the whole
  control set.
- **Scope as a hand-named pair** (2026-08-17). A merge named two tests as "the
  tests in the blast radius" of a four-symbol deletion. Measured: 32 distinct
  functions with any reference (97 occurrences), 31 with code references (96).
  One named test only mentioned the symbols in a comment and compiled fine;
  thirty unnamed functions carried code references, including a shared helper
  that drags its callers with it. The merge's good sizing rule ("if the repair
  is too large, say so and stop") was calibrated against 6% of the surface.
- **Predicate unstated, two mechanisms** (2026-08-16). A blast-radius bound
  said "only three of eighteen discard the compile result". Measured:
  `set_selector_variant_exclusion(Some(…))` base 18, merged 16; `let _ =
  rt_run*` base 2, merged 2. An outcome is discarded explicitly (`let _ = …`)
  or structurally (a helper that returns trace data and never an outcome). The
  deleted tests used the second shape, so the explicit population was
  byte-unchanged by the removal. The count could not be retaken because the
  predicate was missing, not because effort was.
- **Tree** (2026-08-14, `f807d7c3`, `LANG-PRELUDE-ELABORATION-DEPTH`, the child
  of the tree `LANG-COMMENT-POPULATION-PARITY` was framed on). A ready node's D6
  said "repair the two stale attachment-totality couplings" and named both,
  correctly at `2ca91a3a`. The next candidate to land added an eleven-line
  `LOAD-BEARING SHAPE` comment documenting exactly the coupling D1+D2 dissolve,
  a third member.

## Rules

- **When asked to recount an absolute claim, recount the population boundary.**
  The tell is a scope written as a path list in the same sentence as the count;
  ask what selected those directories.
- **Grep the defect's signature, not its repair or its producer.** The
  directory prefix, format string or emitted name crosses directory boundaries
  a path-scoped sweep cannot, and finds second producers of one artifact. In the
  `temp_dir` case the reaper's own comment listed the leaking prefixes; one had
  two producers, one inside the census scope (migrated) and one outside
  (untouched).
- **When a fix's dependency reach equals the census scope, the two stop being
  independent checks.** The migration added `tempfile` to exactly the three
  crates the census covered, so "is the fix applied everywhere it can be?"
  answered yes. Measure the population with an instrument that does not know
  about the fix.
- **If a compensating control would absorb the difference, the claim is
  unfalsifiable by observation.** An age-gated reaper kept the volume from
  filling, so "zero unclassified" was never contradicted by symptoms. Check by
  enumeration.
- **For a text correction, the population is the predicate: every block that
  is now wrong.** The diff is how you found them, never the boundary. A number
  carried across populations keeps looking measured. Check a conjunctive
  description against each conjunct: "names an owner" and "asserts the program
  never executes" gave 5 and 1 above running tests, so a repair keyed on the
  conjunction would touch one file.
- **When a residual says "this control's coverage rests on X", grep for the
  population (the literal), not for the control.** The tell is an anonymous
  literal repeated N times with no `const` naming it: a third member needs N
  edits, and no pin has anything to assert against. Say precisely where a fact
  stops being data and becomes prose: here the same three-member population was
  data in the planning plane (an unarmed base test wrote the full array) and
  prose in the armed plane.
- **Separate comment references from code references before sizing.** The
  doc-only member is what a raw count over-includes; the shared helper is what
  a hand-listed pair under-includes. State the split's limits (block comments
  and string literals miscount either way) and read the one member the
  conclusion turns on. A correct instruction sized against the wrong population
  gives the wrong answer without anyone violating it, and a correction you just
  accepted from someone says nothing about their next claim.
- **A count needs its predicate as a fourth field** beside base, tip and
  number, and a metric with two mechanisms needs it most.
- **A count in a deliverable ("the two", "both", "all three") is a census and
  carries its measurement tree.** Frames state a base for anchors, nobody
  restates it for a cleanup population, and a missed member survives silently
  where a stale line number fails loudly. When two nodes touch one file and one
  lands first, re-run the other's enumeration on the tree it will land on. No
  gate does this: QA verifies the candidate and the Architect reviews its diff.
- **When an artifact discloses a collision, check whether the disclosed case is
  the whole equivalence class.** An enumeration's doc named three entries that
  render as one placeholder; the same flattening happened one level up, because
  the label was built from two namespaces (a kernel audit name for one variant,
  a surface name for the rest), so an entry changing kind under one spelling was
  invisible too. A named blind spot reads as the blind spot. Check the
  strengthening is writable (here one `format!`, no API) and say which sub-case
  it closes.
- **Name the sweep you ran and the one you did not, in one sentence.** Asked
  whether anything else in five files rested on a fact recorded the same way,
  the sweep covered lines the candidate added; the unchanged 28k+ lines carried
  the same phrase family. A diff-scoped answer to a file-scoped question is a
  true answer to a different question.

## Findings that rode along with these instances

- **A trailing cleanup statement is not a drop guard.**
  `remove_dir_all(root).unwrap()` as a last statement runs only on the success
  path; `impl Drop` runs during unwind. A file-level `git grep -c
  remove_dir_all` cannot tell them apart. Grep `impl Drop` (or the guard type)
  and read the cleanup's position: one crate had 22 scratch sites, many
  `remove_dir_all`, and zero `impl Drop`.
- **Report the targets that came back clean, with the read that settled each.**
  A pass that only returns findings carries no information in its negatives.
  And check whether the sentence describing a correct mechanism is one clause
  narrower than the code: "only a failing identity assertion keeps the trees"
  while two assertions (the first a precondition) sat in the preserved scope.
- **When a residual is "undocumented", check whether it is worse.** A helper's
  doc said the scratch was a per-test directory with a fresh file; it was
  per-file, reused and overwritten. The sentence a future author reads just
  before triggering a hazard is the one nobody audits.
- **A cost estimate that defers a pin is falsified by writing the pin.** "Needs
  a new armed assertion, and the arming discipline is not something to loosen"
  deferred one instrument. Written: 34 lines, one 36-second build, nothing
  loosened (the RAII test arm was already used at twelve sites in the file, nine
  added by that candidate); as a third row on the existing control, about six
  lines. Read what each objection is about (building a harness, arming
  production), not what it concludes. A cost estimate is an unrun measurement
  that inherits its disposition's authority.
- **Run the premise even when you expect it to hold.** Arming all three roots
  reproduced the quoted refusal byte for byte, which turned "the comment might
  be wrong" into "nothing detects the boundary moving".
- **When a blocker's stated cause is a prohibition, removing the blocker is a
  claim the prohibition still holds.** Removed `#[ignore]` reasons blamed "the
  banned `Carried -> Lowered` inverse"; check the ban was not routed around.
- **Re-check your own predicate before reporting a count discrepancy.** One
  wording gave 1 against a claimed 13; the owner-line predicate gave 5.
- **Re-hunting a control for its logic does not census its plumbing.** Both
  explicit discards in the file sat in a control reviewed across three merges.
  Ask "what does this ignore?" deliberately. Name an anchor's actual strength:
  a record-versus-reconstruction assertion requiring exactly one recorded
  routing decision proves the compile reached that site, not that it completed;
  one `assert!` on the returned outcome closes it.
- **Removing beats re-homing when the premise was never established.**
  Assertions about structure in a compilation that never completed were never
  evidence; say so when a candidate deletes rather than tolerates them.

Related: [[an-enumeration-needs-a-proven-closure-not-a-better-grep]] (how to
make a count closed),
[[repairing-a-census-completeness-does-not-re-aim-its-subject]] (a complete
census answering a different question),
[[a-list-of-same-shape-guards-reads-as-complete-because-the-shape-is-what-made-them-findable]],
[[correcting-scope-must-sweep-whole-doc]],
[[text-whose-truth-has-an-expiry-sits-in-an-artifact-with-no-alarm]] (the
obsolete-warning half of the tree instance),
[[attack-an-impossibility-claim-at-module-scope-not-only-the-signature]] (the
"cannot re-derive" half of the tree instance),
[[no-instrument-exists-is-a-claim-about-the-space-you-enumerated]] (the
cost-estimate deferral),
[[my-reporting-scope-silently-became-my-measurement-scope]] (the unswept
remainder).
