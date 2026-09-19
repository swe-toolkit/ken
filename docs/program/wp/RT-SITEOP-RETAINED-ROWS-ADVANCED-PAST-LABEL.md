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

## 7. Measured outcome

**Measured at `56e43920de1a29682d1eceab365676499d32cd51`** on
`wp/RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL-census`. **Writes
`docs/program/` only; zero `crates/` edits; no row re-labelled and no row
closed.**

### 7.1 `AC-1` — `D0` cited, and its control re-run rather than inherited

    git grep -c 'RT-SITEOP-CARRIED-WITNESS D2' a388dc06     16 rows / 7 files

**Sixteen. `D0`'s answer `(ii)` stands and this WP proceeds.** The per-line
membership of the three subject rows was re-derived at their own coordinates
rather than taken from the count, because a count establishes how many and
never which:

    a388dc06 px7f_resource_native.rs:311  -> fn :312 row 1
    a388dc06 px7f_resource_native.rs:345  -> fn :346 row 2
    a388dc06 rt_escape_second_resource_native.rs:662 -> fn :663 row 10
    (:590 -> fn :591 is the file's OTHER carrier, escape_resource_plus_plain)

**Both documented traps reproduce.** The name-keyed count returns **36 across
9 files** — confirmed — against the attribute's 16 across 7.

> **The 36 has an UNSTATED SELECTOR, and re-deriving it cost a detour.**
> Unscoped, the name-keyed count is **65 across 19 files**; it is 36/9 only
> when restricted to `crates/ken-cli/tests/`. Both numbers are correct and the
> frame states neither scope. **A number published without its selector cannot
> be re-derived**, and the reader who gets 65 has no way to tell whether they
> or the frame are wrong. Recorded so the next reader does not repeat it.

### 7.2 THE FRAME'S OWN COMMAND IS NOT A ROW-COUNTER AT THIS BASE

**The frame says re-run its exact command and not one you compose. That is
right at `a388dc06` and it silently changes meaning at `56e43920d`.**

    key                                    a388dc06        56e43920d
    'RT-SITEOP-CARRIED-WITNESS D2'         16 lines        12 lines
    '#[ignore = "RT-SITEOP-CARRIED-WITNESS D2'  16 rows      3 rows

At `a388dc06` the string occurs exactly **once per row**, so lines and rows
coincide and `grep -c` is a valid row counter. At this base the same string
also appears in **comments** and inside **other nodes' label prose**, so the
identical command returns 12 and a reader reports twelve rows where there are
**three**.

⇒ **This is the frame's own name-versus-attribute trap, one level in: not a
wrong key, but a right key whose line-to-row ratio is a property of the tree
it was calibrated on.** I reported 12 rows before catching it. **Count rows on
the attribute; a bare string count is a line count wherever the string can be
mentioned.**

### 7.3 `D1` — the three signatures re-measured at this base

Run singly, `--ignored --test-threads=1`, `--no-fail-fast` passed to **cargo**
and not to libtest (it is rejected after the `--`, and the run then executes
nothing while still exiting non-zero).

    row 1   linked_public_right_denial_preserves_exact_masks
            UnclassifiedRuntimeTrap { terminal_value: -1 }
    row 2   linked_public_second_release_is_closed_and_the_handle_closes_once
            UnclassifiedRuntimeTrap { terminal_value: -1 }
    row 10  escaped_buffer_used_by_fanning_host_op_matches_interpreter
            source-specific inheritances at one generated entry disagree on
            their typed consumer projection, including the fresh-result route

**All three still fail, all three still carry the byte-identical retained
label (md5 verified equal to `a388dc06`'s), and none of the three stops at the
eliminated-not-callable refusal that label asserts.** Every signature is
unchanged from the ledger's `04d4dd38a` reading, so the ledger's record of
these three is still current — established by re-running, not assumed.

### 7.4 `AC-2` — rows 1 and 2 CLASSIFIED, not merely signed

**The criterion function is `decode_signed_root_trap`
(`crates/ken-runtime/src/object_linker_packaging.rs:256`)**, which delegates
membership to **`root_trap_catalog_index` (`cranelift_backend/compiled.rs:56`)**.
A planned root trap is encoded as `-((identity << 8) | 0xff)`:

    ROOT_TRAP_TOKEN_TAG    0xff      the low byte must be exactly 0xff
    ROOT_TRAP_TOKEN_SHIFT  8         identity is the high bits, 1-based

**Evaluated on this row's value rather than reasoned about.** For
`terminal_value = -1`, magnitude is `1`:

    terminal_value >= 0 ?      false   passes the sign guard
    1 & 0xff == 0xff ?         FALSE   <- fails HERE. The tag byte is 0x01.
    1 >> 8 != 0 ?              false   would fail the identity guard too

⇒ **`-1` is not a malformed trap token. It is not a trap token at all** — the
smallest legal one is `-511`. So `UnclassifiedRuntimeTrap` is **not a
classifier failure and not a missing catalog entry**: the classifier is correct
and its refusal is accurate. **The artifact built, linked, ran, and terminated
at `-1` through a path that never entered the planned-trap encoding.**

**That is the classification `AC-2` asks for, and it relocates the question.**
No trap-catalog node owns rows 1 and 2, because there is no trap. What owns
them is whatever terminates the child outside the trap channel.

### 7.5 `D2` — SPLIT, and the argument is the layer, not the signature

**Rows 1 and 2 route together; row 10 routes separately.**

**The split argument is not that the signatures differ** — the frame is right
that first stops are not causes. It is that the two groups fail **at different
layers, and one of them never produces an artifact at all**:

    rows 1, 2   COMPILE and LINK and RUN. The failure is a terminal value
                from a live process.
    row 10      refuses at ObjectEmission inside the Cranelift static
                transition planner. No artifact is produced, nothing runs.

A single cause would have to explain both a planner-invariant violation before
emission and a non-trap runtime termination after it. **Nothing in the evidence
connects them, and they are not even in the same execution.**

**The fold of rows 1 and 2 is the weaker half and is declared provisional.**
They share a signature that is *the absence of a diagnosis*, and two rows
sharing a non-diagnosis share nothing — that is the frame's own warning and it
binds this direction too. What they actually share, measured: same file, same
`linked public` resource path, same assertion site (`px7f:36`), and the same
classification in 7.4 (terminates outside the trap channel). **That is a
mechanism-level commonality rather than a shared symptom, which is why the fold
is proposed at all** — but one cause is not established, and the thing that
would refute it is a cause for one that does not apply to the other.

### 7.6 `D3` — the inherited doubt, bounded as a number

**The 16 partition exactly, two independent ways, and the two agree.**

By ignore-state at this base, each row resolved at its own coordinate:

    3   still ignored, carrying THIS label      rows 1, 2, 10 (the subjects)
    4   still ignored, RE-ATTRIBUTED to
        RT-CONTEXT-FRAME-LABEL-CORRECTION       px7l x2, px7m x2
    1   still ignored, BY DESIGN                px8ta, "focused native
                                                resource-cost row; run outside
                                                default suite"
    8   no longer ignored                       px8ta x1, px8x x1,
                                                rt_escape x1, rt_parity x5
    --
    16

By ledger coverage: **7 covered, 9 absent = 16**, and the 7 covered are exactly
the 7 still defect-ignored. **Two partitions built from different sources
landing on the same split is the closure `AC-4` asks for.**

> **DISAGREEMENT WITH §3, REPORTED AS THE FRAME ASKS.** §3 states *"six of the
> 16 are no longer in the ignore set at all."* At this base it is **eight**
> un-ignored, or **nine** if the by-design row is counted as having left the
> defect population. Six is not reproducible here by either reading.
>
> **This is almost certainly the frame's number going stale rather than being
> wrong when written** — §3's anchors are `823c4a67c` / `a388dc06`-era and the
> frame says so itself. It is recorded because §3 also says a disagreement is a
> finding rather than something to reconcile silently, and because the number
> is load-bearing: a reader who takes "six" and subtracts gets ten rows still
> ignored, where there are eight, of which only seven are defect rows.

**Does each of the other 13 match the eliminated-not-callable claim?**
Observation from the ledger where it covers a row; the comparison against the
claim is mine, because the ledger's own `label agrees?` column compares against
each row's *then-current* label, which for the re-attributed four is no longer
the retained one.

    MATCHES  1   delayed_capturing_generic_bind_agrees_across_real_executors
                 "BoundaryCarrier: a carried recursive hypothesis is an
                  eliminated value, not a callable ... but the call provides 1"
                 -- verbatim the retained claim.            method: ledger
    REFUTES  3   runtime_selected_non_unit_response, dynamic_ok_payload,
                 dynamic_err_payload -- "an ordering assertion inside one
                 engine".                                   method: ledger
    MOOT     9   no current refusal to compare: 8 un-ignored and in the
                 default suite, 1 by-design and registered at
                 .github/ignored-test-exemptions.toml:42 under its NEW name.
                 method: tree state + registry, not a local run

⇒ **The bound, and it is sharper than "13 inherit the doubt": across the full
retained 16, the closeout's universal claim is 1 CONFIRMED, 6 REFUTED, 9 MOOT.**
It is not uniformly wrong — it is true of exactly one row that still has a
refusal, and false of every other row that does.

### 7.7 Two rows cannot be tracked by NAME across the interval

`AC-4`'s partition is stated as binary — still-ignored plus no-longer-ignored.
**The population is ternary**, and a residual bucket is what caught it: two of
the 16 have no function of that name at this base.

    px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges
      -> px8ds_real_same_depth_path_runs_exact_edges          RENAMED
    linked_route_exposes_real_ordered_bindings_and_filters_reserved_input
      -> linked_route_exposes_real_ordered_role_labelled_bindings  RENAMED

**Both are renames, not deletions**, and resolving them moved one row into
each of the other buckets — so the binary partition is recoverable, but only
after the third bucket is admitted. **A name-keyed census without a residual
would have reported these two as gone and still summed to 16**, because the
two errors are in opposite directions. Note also that the first rename **drops
`rejects_flat_order_and` from the name**, which is a narrowing of what the test
claims to cover; that is outside this census and is not pursued here.

### 7.8 What this does not deliver

**No row is closed, no row is re-labelled, and `RT-SITEOP-CARRIED-WITNESS` is
cited and not amended.** `D2` hands back a routing proposal, not nodes — per
§3, opening nodes for what a census finds is this WP growing past M in flight.
The cause of the `-1` termination in rows 1 and 2 is **named as the open
question and not investigated**; 7.4 establishes only that it is not a trap.
