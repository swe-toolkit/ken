# LANG-COMMENT-POPULATION-PARITY

Node: `docs/program/issues/LANG-COMMENT-POPULATION-PARITY.md`. Read it first —
it carries the defect, the repro, and why the three Adversary findings are one
node.

**Treat every anchor in this frame as perishable. If a fixed input turns out
false against the landed code, say so and escalate — do not quietly build
around it.** Line numbers below are anchors to re-find, never values to check;
each is qualified at the point of use with the tree it was read in.

## The measurement, at `origin/main = 2ca91a3a`

Read by the Steward at that SHA. Four facts, and the fourth changes what this
frame can ask for.

1. **`TriviaKind` has five variants and production consumes it through exactly
   two predicates, both two-way** — `is_doc_comment()` (`DocLineComment |
   DocBlockComment`) and `is_comment()` (`!Whitespace`), both in
   `crates/ken-elaborator/src/lossless.rs`, in the `impl TriviaKind` block.
2. **`attach_comments` filters on `kind.is_comment()`**; the B1 helper
   `assert_round_trip` filters on `item.kind == TriviaKind::LineComment`.
3. **Both predicates are private** — declared `fn`, not `pub fn`, on an
   otherwise-public type. An integration test in `tests/` cannot call either.
4. **`CommentKind` is `pub(crate)`** — declared in
   `crates/ken-elaborator/src/lexer.rs`, grep `enum CommentKind`. **An
   integration test in `crates/ken-elaborator/tests/` cannot name it.**

## The design call, front-loaded

**Fact 4 is why the landed pin claimed more than it established, and it is the
one thing to understand before writing any code here.**

`lang_trivia_kind_mapping_pin.rs` is an integration test. `CommentKind` is
`pub(crate)`. So that file **cannot write a direct assertion on the mapping at
all** — it cannot name the input type. It was obliged to pin the map
*through behavior*, and behavior sees the map only through two two-way
predicates, which collapse four arms onto two images.

⇒ **The four-arm claim was not an overstatement someone could have caught by
being more careful. It was unachievable from where the file sits.** Do not
attempt to repair it in place by adding more rows to that file; more behavioral
rows cannot separate a within-class pair, for the same reason the existing two
cannot.

**Three calls, taken by the Steward. Each is a public-surface or test-topology
choice, not a semantic one; the Architect reviews them on the merge Decision
like any other diff.**

**Call 1 — the arm-level pin lives in an in-crate `#[cfg(test)]` module in
`src/lossless.rs`, beside the `From` impl it pins.** That is the only location
from which all four arms are nameable without growing the public API. It
asserts the mapping directly, so it reddens under **all six** transpositions
rather than the two that happen to cross the class boundary.

*Rejected alternative: making `CommentKind` `pub`.* It is a surface-syntax
lexer internal; exporting it to buy one test's reach is a permanent API cost
for a problem that has a free local answer.

**Call 2 — `TriviaKind::is_comment` becomes `pub`.** The B1 helper must assert
against **the production predicate itself**, not a re-spelling of it. A
duplicated predicate that drifted is the entire mechanism of this defect;
replacing one copy with a differently-worded copy leaves the mechanism intact.
`TriviaKind` is already public and already imported by that test file, so this
adds one method to a type the test surface already sees.

*`is_doc_comment` stays private.* Nothing outside the crate needs it, and
widening exports beyond what a deliverable requires is how the next such
coupling gets built.

**Call 3 — D1 lands in the same candidate as D2, and D2 is verified first.**
Between widening the filter and installing the arm pin there is a window in
which nothing in the tree pins `Line` against `Block`. Since both are in one
candidate, the window is internal to the branch and costs nothing — but the
verification order is what proves the pin is real, so it is an AC and not a
suggestion.

## Deliverables

**D1 — widen the B1 comment population to the production predicate.** In
`crates/ken-elaborator/tests/kenfmt_b1_lossless.rs`, `assert_round_trip`'s
`comment_count` filter becomes `item.kind.is_comment()`, calling the predicate
made public in D3. Change nothing else about the helper, and do not touch its
assertion message in this deliverable — D5 owns that.

**D2 — install the arm-level pin, in-crate.** A `#[cfg(test)]` module in
`crates/ken-elaborator/src/lossless.rs` asserting all four arms of
`From<CommentKind> for TriviaKind` directly, one assertion per arm.

**D3 — make `TriviaKind::is_comment` public.** One `pub`. Leave
`is_doc_comment` private.

**D4 — a fixture control for the widened population.** A test that drives
`assert_round_trip` over a source containing at least one block comment and at
least one doc comment, so the helper's two sides are exercised on a population
that is not all `LineComment`. This is the control that would have caught the
defect; it must live in the fixture set, not in `catalog/`.

**D5 — correct the pin file's claim to what it establishes.** In
`crates/ken-elaborator/tests/lang_trivia_kind_mapping_pin.rs`, replace the
four-arm header claim with what behavior can establish from an integration
test — class membership — and point to D2's in-crate module as the home of the
arm-level claim. **State in that header why the split exists** (`CommentKind`
is `pub(crate)`, so this file cannot name the arms), so the next author does
not re-widen the claim here.

**D6 — repair the stale attachment-totality couplings. THERE ARE THREE, and
the third landed after this frame was measured.** **Editing the message text is
D6's, not D1's** — kept separate so the diff attributes.

1. The helper's own assertion message in `kenfmt_b1_lossless.rs`, which names
   attachment totality for a check that is about kind mapping.
2. The residual section in `docs/program/issues/LANG-TRIVIA-KIND-MAPPING-PIN.md`
   — record that the coupling is gone and which deliverable removed it.
3. **The `LOAD-BEARING SHAPE` comment block above
   `leading_trailing_and_interstitial_comments_have_stable_unique_homes` in
   `kenfmt_b1_lossless.rs`.** Added by `LANG-PRELUDE-ELABORATION-DEPTH` as its
   carried addendum, landed at `f807d7c3`.

> ### AMENDED 2026-08-14, AFTER THE KICK. Item 3 and the paragraph below are
> ### the amendment; the ring must read `D6` as three items, not two.
>
> **This frame enumerated two couplings because it was measured at `2ca91a3a`,
> and both were visible there.** `f807d7c3` is that tree's child and it added
> an eleven-line block asserting the fixture's shape is load-bearing for the
> pin's `Line` arm. **The enumeration was complete when written and incomplete
> against the tree it lands on.**
>
> **The Contention section already said "`D6` updates the comment it added."
> That was not enough and the gap is instructive:** a reconciliation stated in
> a section whose grammatical mood is advisory does not bind a deliverable that
> enumerates its own members. A reader discharging `D6` works the list. **This
> frame's own rule — a sentence that tells someone to do something is an AC —
> applied to the Contention section and I did not apply it.**
>
> **Why item 3 is not bookkeeping.** Every clause of that block is false after
> `D1`+`D2`: the `Line` arm will be pinned in-crate and will not depend on the
> fixture; `D5` rewrites the header it cites; and `D2` reds under all six
> transpositions where the block says *"nothing reds."* **The last clause is
> the live hazard — *"do not change this configuration without updating that
> file too"* survives as a prohibition on a fixture that is now free, addressed
> to exactly the author `D4` invites into that file.** A warning outlives its
> cause silently and reads as a live constraint.
>
> **And the block states an impossibility that is measured false.** It says the
> pin file *"cannot re-derive this fixture."* An in-file `--` row appended to
> `d2` **does** red under a `Line`/`DocLine` transposition — run, then reverted.
> The cross-file coupling was a **scope decision, not an impossibility**, and
> the comment states it in the strongest available form, which is the one
> nobody re-checks. If any of that block survives `D6`, it must say *why* the
> arm was not re-derived there — superseded by `D2` — never that it could not
> be.
>
> **Do not let this collapse into the design call above.** `CommentKind` being
> `pub(crate)` makes the **map** unassertable from an integration test, and no
> number of behavioural rows separates a **within-class** pair. Behaviour
> separates the four **cross-class** transpositions perfectly well. **The
> impossibility is real for the map and false for the `Line` arm's behavioural
> pin**; the two sit one sentence apart and the landed comment merges them.

## Acceptance criteria

**AC-1 — the widened filter is exercised, not merely written.** D4's fixture
must contain a comment of a kind the old filter excluded, and the ring must
report the comment count the helper now sees versus what it saw before the
change, for that fixture. A number that does not move means the fixture did not
reach the widened path.

**AC-2 — positive control on D1, run and reported.** Revert D1's filter to
`item.kind == TriviaKind::LineComment`, leaving D4's fixture in place, and
confirm D4 **reddens**. Restore. Report the failing assertion. A D4 that stays
green under the reverted filter is not a control for this defect.

**AC-3 — the arm pin reddens under all six transpositions, each run
separately.** For each of the six pairs from `{Line, DocLine, Block, DocBlock}`
— including both within-class pairs, `Line`/`Block` and `DocLine`/`DocBlock` —
transpose that pair's two arms in the `From` impl, run D2's module, confirm
red, restore. **Report six rows: pair, red or green, and the assertion that
fired.** This is the enumeration, not "each pair" as a quantifier for the
reader to resolve; a row per pair, or the AC is not met.

**AC-4 — verification order, recorded.** AC-3 must be run and reported
**before** D1's filter change is committed on the branch, and the report must
say so. This is what establishes that the arm pin stands on its own rather than
riding the filter it replaces.

**AC-5 — no behavior change.** `-p ken-elaborator` green, and the `From` impl,
`attach_comments`, and both predicates' bodies are byte-identical to
`2ca91a3a` apart from D3's single `pub`. Report the diff of
`src/lossless.rs` restricted to those items.

**AC-6 — the pin file's header no longer claims more than the file
establishes.** After D5, quote the header's claim sentence in the handback and
name which of the two artifacts — this file or D2's module — carries the
arm-level claim.

**AC-6a — no surviving text asserts a coupling this node dissolved.** After D6,
grep `crates/ken-elaborator/` for the three cited artifacts and report, one row
each, what the text now says. **The specific thing that must not survive
anywhere is a prohibition on changing a fixture's configuration, or a claim
that the pin file cannot re-derive an arm.** Both are false after `D2`, and
both currently read as live constraints on the file `D4` sends an author into.

**AC-7 — no-regression, in CI.** Green in CI on the candidate. Do **not** run
a local `--workspace` build; the venue is CI (`COORDINATION §12`).

## Contention

**None expected.** `LANG-PRELUDE-ELABORATION-DEPTH` is the other Language node
in flight and touches `crates/ken-elaborator/tests/kenfmt_b1_lossless.rs` for
**one comment line at its `-- trailing` fixture**, as its carried addendum.

**That addendum and this node's D1 both touch that file, and this is the
overlap to know about rather than to avoid.** The addendum records that the
fixture's shape is load-bearing for the pin's `Line` arm — a claim **this node
retires**, since after D2 the `Line` arm is pinned in-crate and no longer
depends on that fixture's configuration. Whichever lands second reconciles: if
`LANG-PRELUDE-ELABORATION-DEPTH` lands first, D6 updates the comment it added;
if this node lands first, say so in the handback so the addendum is not written
against a premise this node removed.

**Not a blocking dependency.** The two do not contend on any assertion, and
neither ordering costs a rebase beyond one comment line.

## Not this node

No change to `attach_comments`, the `From` impl, the placement heuristic, or
any catalog source. No new corpus oracle. No widening of exports beyond D3's
single `pub`.
