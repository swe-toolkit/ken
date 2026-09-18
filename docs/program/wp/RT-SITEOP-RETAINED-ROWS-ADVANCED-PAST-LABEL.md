# RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL — work package

**Owner: Team Runtime. Size M. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`823c4a67cbfe8ef3e8fccf06acd1fa4c4a13af97`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."* Covers ledger rows **1, 2 and 10**.

> **Every coordinate, count, line number and status in this frame is PERISHABLE
> and was measured at the base SHA above.** Re-ground each one against
> `origin/main` before acting on it. If a re-measurement disagrees with this
> document, the tree wins and the disagreement is a finding worth reporting, not
> a discrepancy to reconcile silently.

## 1. Objective

Three ignored rows carry a label naming a merged node whose closeout asserts
they stop at a refusal the ledger measured them **not** stopping at. Establish
what actually owns them.

**`D0` — the node's own gate — IS ALREADY ANSWERED. Do not re-run it.** It
returns `(ii)`, recovered from the tree by the Steward and written into
`docs/program/issues/RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL.md`. **Read
that section before anything else**; this frame starts after it.

## 2. Fixed inputs, measured at `823c4a67c` and at `a388dc06`

**The retained 16, recovered at `a388dc06`** (`RT-SITEOP-CARRIED-WITNESS`'s
merge commit) as the set carrying one byte-identical `#[ignore]` string:

    px7f_resource_native.rs                 2     rows 1, 2
    px7l_checked_host_recursive_bind.rs     2
    px7m_hostresult_computational_match.rs  2
    px8ta_oriented_subcontinuation.rs       2
    px8x_single_schema_observation.rs       1
    rt_escape_second_resource_native.rs     2     row 10 among these
    rt_parity_native.rs                     5
                                           16    matches the closeout's count

**Recovered with this exact command, and RE-RUN THIS ONE, not a command you
compose from the node name:**

    git grep -c 'RT-SITEOP-CARRIED-WITNESS D2' a388dc06

**The three rows still carry that label on `main` today, byte-identical.**

> ### THE OBVIOUS INSTRUMENT RETURNS 36 AND READS LIKE A FINE ANSWER
>
>     grep -c on the NODE NAME      36 matching lines across 9 files
>     grep -c on the ATTRIBUTE      16 rows across 7 files
>
> The node name appears about **twice per row** — once in a comment above the
> attribute — **and in two files that carry no such label at all**
> (`px4b_native_production.rs`, `px7p_constructor_field_composition.rs`). So a
> name-keyed count is larger, spans more files, and looks correct. **The
> attribute is the key; the name is a mention.**
>
> **Second trap, the path:** these rows are in `crates/ken-cli/tests/`, **not**
> `crates/ken-runtime/tests/`. A wrong-path run returns zero — and an empty
> result on a population you know is non-empty is the cheapest false-negative
> tell there is. Independently reproduced by the Architect, `evt_4j7ygjy4jyaxx`.

> **The membership test and the claim under test are the same string.** That
> string both marks a row retained and asserts the eliminated-not-callable
> refusal, so on its own it cannot separate *"retained, claim false"* from
> *"never retained, mislabelled."*
>
> **The count CORROBORATES; it does not decide.** Two compensating errors
> preserve a total — one row mislabelled in and one genuinely-retained row
> missing nets to sixteen and reproduces the observation exactly. **What carries
> `D0` is the per-line verification of rows 1, 2 and 10**, each at its own
> coordinate. Do not read "sixteen" as a proof and skip that.
>
> **This is a stated limit on the instrument, not a caveat to skip.** If your
> re-measurement returns anything but 16, `D0` is NOT answered and this frame's
> §1 is void: stop and route it back. **That direction is sound — a mismatch
> really does refute. Only the converse fails to carry.**

## 3. Deliverables

**`D1` — re-measure the three signatures at `main`.** The ledger's numbers are
from `04d4dd38a` and it says so; they are a correct record of that base, not of
`main`. Run the three rows, record what they do now.

> **Rows 1 and 2's signature is the one to watch.** `UnclassifiedRuntimeTrap
> { terminal_value: -1 }` is **not a diagnosis — it is the absence of one.** A
> trap that reached the classifier and was not classified tells you the row
> failed and nothing about why.
>
> ⇒ **Do not frame a repair against an unclassified trap.** The deliverable for
> rows 1 and 2 is a **classification**: name the criterion function that should
> have matched and establish why it did not. **Verify the membership from the
> criterion, never from the value.**

**`D2` — decide ownership per signature.** Rows 1 and 2 share a signature; row
10 does not. That is evidence about **first stops**, not about causes.

⇒ **Fold them only with an argument, and split them only with one.** Either
direction taken for free is the same error. If each needs its own node, say so;
if one node covers all three, state what they share.

**`D3` — bound the inherited doubt, and report the bound as a number.**

`(ii)` means the closeout's universal claim over the retained 16 is refuted, so
**the other 13 retained rows inherit the doubt.** That is the consequence `D0`
was ordered first to expose, and it is this node's to bound — not to repair.

    for each of the other 13: does its CURRENT first refusal match the
    eliminated-not-callable claim the retained label makes about it?

**Much of this discharges from existing evidence rather than from runs.** The
ledger already carries current signatures for every one of the 16 that is still
`#[ignore]`d, and six of the 16 are no longer in the ignore set at all. **Use
the ledger where it covers a row; run only what it does not.** State per row
which of the two you used.

> **This is a CENSUS, not a repair program.** Do not open nodes for what it
> finds. Hand back a table and let it be routed. A `D3` that turns into repairs
> is this node growing past M in flight, which is the Steward's to re-cut.

## 4. Acceptance criteria

**AC-1 — `D0` is cited, not redone.** The deliverable states `(ii)` and its
basis. **Control:** re-measuring the label count at `a388dc06` must return 16.
A different number invalidates `D0` and this WP stops.

**AC-2 — rows 1 and 2 get a classification, not a signature.** Naming
`UnclassifiedRuntimeTrap` as the cause fails this AC; it is what the row prints
when nothing classified it. **Control:** the criterion function is named and
the reason it did not match is stated.

**AC-3 — the fold/split decision in `D2` carries its argument.** A grouping
asserted without one fails, in either direction.

**AC-4 — `D3` reports a number and its method per row.** "Ledger" or "ran it"
beside each of the 13. **Control:** the 16 partition exactly — still-ignored
plus no-longer-ignored must sum to 16 with no row unexplained.

**AC-5 — no row-closure claim, and no re-labelling.** See §5.

**AC-6 — no-regression, green in CI.** Per `COORDINATION §12`: targeted local
runs only (`scripts/ken-cargo`, `-p` or `--test`), **never `--workspace`**.
Workspace-green means green in CI, not on this box.

## 5. What must not happen

- **Do not re-label the rows to make them consistent.** The inconsistency is
  the evidence. A row pointing at a node that does not describe it is
  discoverable; a row pointing at a plausible node nobody checked is not.
- **Do not reopen or amend `RT-SITEOP-CARRIED-WITNESS`.** It is merged and its
  `D2` landed. Under `(ii)` the finding belongs in a node that cites it — **a
  merged node's record of what it believed at merge time stays as it was.**
- **Do not treat "the port succeeded" as in question.** It is not. Thirteen rows
  un-ignored and passing is a measured positive result and nothing here touches
  it. **What is in question is the account of where the survivors stopped**,
  which is a different claim in the same sentence.
- **Do not un-ignore any of the three to see what happens.** Two currently
  produce an unclassified trap; un-ignoring converts three parked rows into
  three red ones to buy one bit a targeted run already gives you.
- **No row closes here.** This node establishes ownership; it repairs nothing.

## 6. Contention

**Clean at this base.** This WP writes `docs/program/` only — a census and an
ownership decision, no `crates/` edit. Its reads touch `px7f_resource_native.rs`
and `rt_escape_second_resource_native.rs`.

**One neighbour to watch:** `RT-DUPLICATED-RESPONSE-BLOCK` is in flight and
reads `rt_escape_second_resource_native.rs` for rows 11 and 12. **Reads do not
contend**, and this node writes no test file, so the two are compatible. If
`D3` ever wants to edit a label, it does not — see §5.
