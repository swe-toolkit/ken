# RT-CARRIED-RESIDUAL-IH-ARITY — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at `fe649049f2419be5e8c9d58547baeeb27cb06ddd`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* This is the
first repair node cut from the `RT-IGNORED-FAILING-ROWS-INVENTORY` ledger.

## 1. Objective

Close the `BoundaryCarrier` refusal *"a carried recursive hypothesis is an
eliminated value, not a callable, so it takes no arguments, but the call
provides N"* for the four ignored rows that carry it, and **readmit those four
rows** — or, if the refusal is correct and the rows are wrong, say so with the
same evidence and retire them instead.

**Both outcomes close this node.** See §4.

## 2. Fixed inputs, measured

**The four rows, at `c041de7c3` (the ledger's base):**

    px7l_checked_host_recursive_bind.rs:153  delayed_capturing_generic_bind_...
    px7l_checked_host_recursive_bind.rs:220  runtime_selected_non_unit_response_...
    px7m_hostresult_computational_match.rs:153  dynamic_ok_payload_selects_...
    px7m_hostresult_computational_match.rs:185  dynamic_err_payload_selects_...

All four under `crates/ken-cli/tests/`. All four `#[ignore]` labels are
**byte-identical** and name this mechanism.

**The refusal, at `fe649049f`:**

    core.rs:3052   fn reject_carried_residual_arguments(arguments: usize)

    called from    core.rs:3101   core.rs:6063   core.rs:16133
                   source.rs:5016

**The pointer left by the previous seat:** `aggregates.rs:3340-3356`. It rules
this refusal out of scope for the carried SITE-OPERAND projector and says
*"widening this dispatch cannot close it and must not try."* **Read it first.**

**Cold-lowering witness:** `rt_allocate_stage`,
`rt_cold_lowering_path_enumeration.rs:156`, dispositioned `Completes` at `:547`.

**Clean baseline, measured on `main` at `c041de7c3`:**

    ken-cargo test -p ken-runtime --lib    1035 passed   0 FAILED   2 ignored

**You are changing lowering code those 1035 tests exercise.** The baseline is
clean, so any red appearing during this repair is yours and cannot be
pre-existing debt. That is the whole reason the number is in the ledger.

## 3. The design call — make it FIRST and write it down

**Is a carried recursive hypothesis non-callable at all five call sites, or
only at the site these four rows reach?**

    fix at the definition   changes all five paths
    fix at one call site    changes one

**Answer this before writing the repair, and record the answer with its
evidence in the handover.** A fix at the shared function is a claim about
fan-in; **the claim has to be checked, not inherited from where the helper
happens to live.** If you cannot determine it for all five, fix the site you
measured and say which sites you did not clear — a scoped fix with a named
residue beats a choke-point fix with an unchecked one.

**This is the reasoning content of the node.** The edit that follows is small.

## 4. Deliverables

1. The design call of §3, written down with its evidence.
2. The repair, at whichever unit §3 selects.
3. **The four rows readmitted** — `#[ignore]` removed, passing — **or** retired
   with the measurement that shows the refusal is correct and the rows are
   asserting something false. State which, per row.
4. If any row is readmitted with an accepted red, it needs a row in
   `.github/ignored-test-exemptions.toml` (`class` + `readmission` +
   `test_path`) — **an accepted red names what would readmit it.**

## 5. Acceptance criteria

**AC-1 — the four rows are dispositioned, per row, by name.** Each is
readmitted-and-passing or retired-with-evidence. **Four outcomes, four names.
A count is not an answer here** — the rows sit in two files and a per-file
summary hides a row.

**AC-2 — the `ken-runtime` lib suite does not regress.**

    ken-cargo test -p ken-runtime --lib

against the `1035 passed / 0 FAILED / 2 ignored` baseline in §2. **Report the
delta and its three buckets, not the total:** red-on-both (pre-existing),
green-on-main-red-here (yours), and **candidate-only with no `main`
counterpart** (which no subtraction can show — publish it even at zero).

**AC-2's control:** the baseline is `main`'s, not a number you carry forward.
Re-measure it if `main` moves under you.

**AC-3 — the crate set is DERIVED, not named.** This node touches
`ken-runtime` and `ken-cli`. Compute the reverse-dependency closure over the
touched set **to a fixpoint**, mechanically, and test that set. **Never
`--workspace`** (`COORDINATION §12`) — a derived list written down is still a
targeted build. **State the target selection beside the claim**
(`--lib`/`--tests`/`--all-targets`): `cargo check` does not compile
`#[cfg(test)]`, and a green `check` is not evidence that any test built.

> **AC-3 exists because this exact criterion was got wrong three times on
> `ABI-S6-HS18-D5B-SUBSTRATE-PORT` this week** — a hand-named single crate,
> then a hand-named four, then a hand-chosen one-hop closure. **The population
> was a default nobody selected each time.** Do not enumerate it.

**AC-4 — the `aggregates.rs:3340-3356` scope comment is reconciled.** It
asserts this refusal *"does not become reachable or unreachable by anything
this projector does."* If your repair makes that false, **update it**; if it
stays true, say so. **A comment that names the next layer is load-bearing and
goes stale silently.**

**AC-5 — `rt_allocate_stage`'s enumeration disposition is checked.** It is the
recorded witness. If it moves, that is a result; if it does not, that is also a
result. **Name which.**

## 6. What this node is NOT

- **Not the other eleven ignored rows.** `S2` (4 rows, planner-invariant) is a
  separate cut; `S3` (2 rows, `UnclassifiedRuntimeTrap`) is not routable yet
  because its signature names no mechanism; five singletons are queued.
- **Not an investigation into why these rows still fail.** The mechanism is
  named in the label, in the code, and in §2.
- **Not `RT-SITEOP-CARRIED-WITNESS` reopened.** That node landed its
  deliverable and these labels record it succeeding.

## 7. Contention

`crates/ken-runtime/src/cranelift_backend/lowering/` is touched by
`ABI-S6-HS18-D5B-SUBSTRATE-PORT`, which is **parked** — its candidate is dead
and its recut is the Steward's, unstarted. **No live contention at cut time.**
If D5B's recut is released while this is in flight, it comes back to the
Steward to sequence; do not resolve it in the ring.

## 8. Estimated tier: T1

Not for the size of the edit. **The design call in §3 decides a property of
five code paths from evidence about one**, and the wrong answer changes four
paths nobody measured. That is reasoning, not transcription.
