# RT-HOST-RESPONSE-ROUTE-KEY-COLLISION — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at `fe649049f2419be5e8c9d58547baeeb27cb06ddd`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Second
repair node cut from the `RT-IGNORED-FAILING-ROWS-INVENTORY` ledger.

## 1. Objective

Decide whether the `two host response cases claim one operation constructor`
invariant is **too strong**, and disposition the four ignored rows that trip
it — readmitted if the invariant is wrong, retired with evidence if the
invariant is right and the tests assert something false.

**Both outcomes close this node.** An invariant that correctly refuses four
bad tests is as good a result as a key that needed widening, and it is
reported the same way.

## 2. Fixed inputs, measured

**The invariant:**

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

in a `BTreeMap<RuntimeSymbol, HostResponseRoute>` keyed on `case.constructor`
alone, whose value carries `operation` as a field.

**The four rows, by FILE and LINE** (at `c041de7c3`) — see the issue node's
warning about the name collision with `px7o_heterogeneous_eliminator_frames.rs`:

    rt_escape_second_resource_native.rs:654   escaped_resource_used_by_fanning_host_op_...
    rt_escape_second_resource_native.rs:714   nat_fanout_escaped_resource_matches_interpreter
    px7n_nested_computational_eliminator.rs:150  nested_ok_payload_reaches_both_real_executors
    px7n_nested_computational_eliminator.rs:171  nested_err_payload_reaches_both_real_executors

**Clean baseline, measured on `main` at `c041de7c3`:**

    ken-cargo test -p ken-runtime --lib    1035 passed   0 FAILED   2 ignored

**You are changing planner code those 1035 tests exercise.** The baseline is
clean, so a red appearing during this repair is yours and cannot be
pre-existing debt.

## 3. THE MEASUREMENT THAT DECIDES IT — RUN THIS FIRST

**For each of the four rows: do the two colliding cases carry the SAME
operation or DIFFERENT operations?**

    DIFFERENT   the key is too narrow. It omits what the value already
                carries, and the invariant is reporting a duplicate that is
                not one. Repair is the key.

    SAME        the invariant is RIGHT. Two cases genuinely claim one
                constructor for one operation, the tests are asserting
                something false, and the repair is to the tests -- or they
                retire.

**Do not write the repair before this comes back.** The key-too-narrow reading
is the Steward's hypothesis from reading the code, **not a measurement**, and
it is exactly the kind of reading that is refuted by one run. If it comes back
`SAME`, discard it.

**Report the answer per row, not in aggregate.** Four rows can split across the
two outcomes, and a summary would hide that.

## 4. Deliverables

1. The §3 measurement, per row, with the operations named.
2. The repair at whichever unit §3 selects — the key, or the tests.
3. **The four rows dispositioned by name**: readmitted-and-passing, or retired
   with the evidence that shows the assertion false.
4. If a row is readmitted with an accepted red, it needs a row in
   `.github/ignored-test-exemptions.toml` (`class` + `readmission` +
   `test_path`).
5. **The two stale `#[ignore]` labels are corrected or removed.** They name
   mechanisms the run does not exhibit (`RT-CLOSURE-BOUNDARY-LANE`'s durable
   lane, `RT-FRAME-MARKER-ONCE`'s frame marker) and cite base `21fd46dc`.
   **Leaving a wrong label on a readmitted row re-seeds the defect this whole
   program exists to clear.**

## 5. Acceptance criteria

**AC-1 — four rows, four named dispositions**, resolved by file:line rather
than by symbol. A count is not an answer.

**AC-2 — no regression on the `ken-runtime` lib suite** against the
`1035 / 0 / 2` baseline in §2. **Report the delta's three buckets, not the
total:** red-on-both (pre-existing), green-on-main-red-here (yours), and
candidate-only with no `main` counterpart (which no subtraction shows —
publish it even at zero). Re-measure the baseline if `main` moves under you.

**AC-3 — the crate set is DERIVED, not named.** Compute the
reverse-dependency closure over the touched set **to a fixpoint**,
mechanically, and test that set. **Never `--workspace`** (`COORDINATION §12`).
**State the target selection beside the claim** — `cargo check` does not
compile `#[cfg(test)]`, so a green `check` is not evidence that any test built.

**AC-4 — if the key changes, the widened key is justified against the
invariant's PURPOSE, not against these four rows.** The invariant exists to
catch a real ambiguity. **Say what it still catches after the change**, and
give one case that must still be refused. A widened key that refuses nothing
has deleted the check rather than corrected it.

**AC-5 — `S6` and `S8` are checked against the outcome, and NOT pulled in.**
Both are single rows carrying different planner-invariant messages from this
same subsystem. **Say whether your cause explains them.** If it does, that is a
finding for the Steward to re-cut on — **do not absorb them into this node**;
if it does not, say that too.

## 6. What this node is NOT

- **Not the other eleven ignored rows.**
- **Not a rewrite of the static-transition planner.** The scope is one
  invariant and the four rows that trip it.
- **Not an endorsement of the key-too-narrow reading.** §3 decides it.

## 7. Contention

`crates/ken-runtime/src/cranelift_backend/planning/` — `ABI-S6-HS18-D5B-
SUBSTRATE-PORT` is **parked** (dead candidate, recut is the Steward's,
unstarted). `RT-CARRIED-RESIDUAL-IH-ARITY` touches
`cranelift_backend/lowering/`, a **different** subtree. **No live contention at
cut time.** If both this and `RT-CARRIED-RESIDUAL-IH-ARITY` run concurrently,
they are separate branches and separate PRs.

## 8. Estimated tier: T1

**Deciding whether an invariant is too strong is a soundness-adjacent call**,
not a transcription. The wrong answer either deletes a real check or retires
four tests that were right.
