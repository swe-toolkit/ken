---
id: CI-IGNORED-SWEEP-EXEMPTION-LIVENESS
title: "The ignored-sweep registry checks that a row CITES its blocker, never that the blocker is still open -- so RT-CLOSURE-BOUNDARY-LANE is merged, its exemption stands, CI is green, and a readmittable test is invisible; plus the label lexer cannot read 2 of the 22 labels and the job prints counts without names"
status: draft
owner: verify
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Measured 2026-09-18 by the Steward from the producer (scripts/ci-ignored-sweep.py, .github/ignored-test-exemptions.toml) at origin/main 34e1426c7, after the Architect located the sweep job and read its selection line. The lexer defect is the Architect's (evt_cx8ce8w3qf5), reproduced independently here. Steward-filed per COORDINATION section 2 (agents cannot create tracked work). QUEUED, NOT RELEASED: verify is not one of the three lanes in steward/lanes.md, and the release call is the operator's -- put to Pat at evt_55gys113fz4wx."
---

> ## FILED AND QUEUED. NOT RELEASED. THE LANE CALL IS THE OPERATOR'S.
>
> Verify is not one of the three active lanes (`steward/lanes.md`: runtime,
> language, foundation), so this node is filed and queued behind them. **It is
> nonetheless the instrument that measures the operator's stated top-priority
> objective**, which is the tension, and it is the operator's to resolve rather
> than the Steward's. Asked at `evt_55gys113fz4wx`; nothing is blocked on the
> answer.

## What the sweep actually does, read from the producer

**Do not reason about this job from a grep or from arithmetic.** Both were tried
on 2026-09-18 and both produced a wrong mechanism that happened to match the
right numbers.

    selected = nextest ignored-only population MINUS the exemption registry
               (scripts/ci-ignored-sweep.py, expected_count)

    registry = .github/ignored-test-exemptions.toml
               schema-validated: every row carries exactly
               {test_path, class, readmission}, all non-empty,
               test_path package-qualified, no duplicates,
               class from a closed set

Selection is **registry subtraction**. It is not keyed on the row's label, and
nothing about the `#[ignore]` text decides which population a row lands in.

**The job already fails closed on the arithmetic.** `verify_lists` reconstructs
the nextest total from selected plus registry and raises `SweepError` on a
mismatch, so **an ignored row that is neither selected nor registered is a red
CI, not a silent drop.** A new unowned `#[ignore]` cannot slip past.

    22   ignored rows in the tree        nextest ground truth
    14   rows the sweep runs
     8   rows registry-exempt

These figures are time-varying and are stated with their predicate deliberately.
The `CI-IGNORED-SWEEP` frame records the population moving twice while that node
sat `ready`. **Quote the predicate with the count, or do not quote the count.**

## Deliverable 1 — read the attribute with a real lexer

`IGNORE_REASON_RE` models a Rust string literal as `"[^"]+"` on a single line.
Rust string literals are not that: they may contain escaped quotes and they may
span lines with a trailing backslash. Both forms are in the tree.

Measured at `c2b4854cb`, and reproduced independently by the Steward:

    attribute-form rows scanned: 22   matched 20   UNMATCHED 2
      ken-cli/tests/px7m_hostresult_computational_match.rs:206   escaped quote
      ken-kernel/tests/recursive_head_totality_d0.rs:411         line continuation

`ignored_test_reasons` records only matching lines, so an unparseable label
produces **no entry at all**.

**Scope this correctly — the lexer is not the selection mechanism.** Because
selection is registry subtraction, a parse failure cannot move a row between
populations. The lexer gates `verify_blocked_upstream_relations` only, and there
it is fail-closed: zero matches raises. What it produces is a **misleading red**:

    registry test_path '...' resolves to 0 source #[ignore = "..."] reasons;
    expected exactly one

which reads as *"this test has no `#[ignore]` attribute."* It has one. The
message names the registry and the source, and the fault is in neither.

**The fail-open arm is occupied today, not latent.**
`recursive_head_totality_d0.rs:411` is in the registry by name, classed
`policy-cost` — a class `verify_blocked_upstream_relations` never consults. So
there is a registered row in the tree right now whose label the sweep cannot
read, sitting in the class that never looks. Nothing is wrong today only because
the two conditions have to hold together, and both do.

## Deliverable 2 — emit the selected identities beside the counts

The job prints counts and no names:

    ignored sweep selection: 14 selected of 22 ignored-only matches
    from 4016 total discovered tests; nextest ground truth and
    registry subtraction agree

That line was read three different ways in one hour, and the membership question
it raises — *are the sweep's 14 the same rows as the ledger's 14?* — could not be
settled from it. It was then re-asked by hand, twice, by two different readers
reconstructing the selection from the script.

⇒ **Emit the selected test identities.** A membership question that the
instrument answers every run stops being a question. This is the one change here
that removes future work rather than adding a check.

## Deliverable 3 — liveness on the readmission condition

`verify_blocked_upstream_relations` checks that a row's readmission symbol
**occurs in its own `#[ignore]` reason**. It never checks that the named relation
is still absent.

    docs/program/issues/RT-CLOSURE-BOUNDARY-LANE.md
    status: merged

**The blocker is merged, the readmission condition is satisfied, the exemption
stands, and CI is green.** `ken-elaborator/src/compiler_driver.rs:5404` is
readmittable today and no instrument in the tree will say so. The registry's own
header calls the class *temporary*; nothing makes it expire.

**Three outcomes, all decided. Red on unresolvable, never a skip:**

    symbol resolves, node not merged   ->  green
    symbol resolves, node merged       ->  RED: readmittable, exemption expired
    symbol resolves to no node         ->  RED: unresolvable readmission

The direction matters at birth, and the reason is the Architect's: **a skip
installs the liveness check with a blind spot on half of the only class it
applies to** — which is the very failure this node exists to close, reproduced
inside its own repair.

**Do not key the resolution on `RELATION_SYMBOL_RE`.** That regex
(`^[A-Za-z][A-Za-z0-9_-]*$`) is the symbol-versus-prose test and the schema
already enforces it on this class, so **both** rows match it:
`RT-CLOSURE-BOUNDARY-LANE` and `TermJReduction` are equally symbol-shaped.
Keying on it returns "both fine" and checks nothing. The needed discrimination
is resolution against `docs/program/issues/`, which is a new read.

### `TermJReduction` goes red on day one, and that is the point

There is no issue file matching `TermJ*` in `docs/program/issues/`; its only
occurrence anywhere in the tracker is a citation inside
`CI-NATIVE-PARITY-DURATION`. It is a registry row waiting on something **that was
never given a node** — the orphan predicate in a third dress: not orphaned by a
merge, not unowned by omission, unowned because nothing was ever cut.

This node does not decide it. **Landing deliverable 3 decides it**, by making the
row red until someone either cuts a node or reclassifies it. Cheap now, while the
class has two members.

## Scope fence — what this node does NOT cover

    blocked-upstream-relation  2   symbol, checkable    <- THIS NODE
    policy-cost                3   "not applicable: ..."   CORRECTLY permanent
    placeholder-no-assertions  3   "after <capability>, assert <property>"

**Three rows are correctly permanent.** Their reason is that a standing gate
covers the same lowering; there is no future event that readmits them, so a
liveness check on them would be a check with no satisfying condition. They are
not a gap.

**Three rows expire on an event nothing watches, and they are the real
residue.** `CI-IGNORED-SWEEP` frame:127 already measured what they are: of 17
tests in `l1_acceptance.rs`, four contain no `assert*` and no `panic!`, and three
of those four are exactly these rows. Their `#[ignore]` is the only thing
preventing a false green on the day their capability lands.

⇒ **A row whose readmission condition is prose cannot be expired by any
instrument**, and the three rows in that condition are the three whose bodies
assert nothing. Making those conditions machine-checkable is a separate call and
is **not** decided here.

**And do not read this node as covering the ignored-row population.** It reaches
the registry. `merge-procedure.md` M7a reaches the complementary set — rows whose
label names the merging node at its start, which a merge can orphan. Neither
covers the other.

## Acceptance

1. The label reader handles escaped quotes and line continuations; the two rows
   above parse. **Assert the parsed count equals the attribute-form count** — a
   reader that silently drops rows is the defect being fixed, so the AC must be a
   measurement, not an example.
2. The sweep prints the selected identities. Verified by reading them out of a
   CI run, not by reading the code.
3. A `blocked-upstream-relation` row whose node is `merged` is red. **Verified by
   direction**: construct or observe both a green case and a red case, because a
   check that never fires and a check that cannot fire are the same output.
4. A `blocked-upstream-relation` row whose symbol resolves to no node is red.
   `TermJReduction` is the standing live case and needs no fixture.
5. No-regression in CI. Not a local `--workspace` run (`COORDINATION` section 12).

**Estimated capability tier: T2.** Three bounded edits to one script with the
mechanism already measured and written down. The reasoning in this node is the
part that was hard; executing it is not.
