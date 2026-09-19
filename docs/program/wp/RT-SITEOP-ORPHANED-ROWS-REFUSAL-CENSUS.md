# RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS — frame

**Owner:** runtime ring. **Size:** M. **Estimated capability tier: T1.**
**Node:** `docs/program/issues/RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS.md` — read
it first; this frame governs, the node carries the provenance and the four
conflicting accounts in full.

**Treat every anchor in this frame as perishable. If a fixed input turns out
false against the landed code, say so and escalate — do not quietly build
around it.**

## Objective

Three ignored rows name a merged node in their `#[ignore]` strings, so no live
node owns their next repair, and **four separate accounts of what they do
disagree with each other and with the ledger.** Establish what each row
actually refuses with at your own base, and decide ownership per signature.

**This node may clear nothing, and that is an acceptable outcome.** If the
rows' labels are simply stale, the deliverable is a label correction. An
un-ignore is a **result** here, never an acceptance criterion.

## The three rows — cited by name, because the numbers have already moved

    crates/ken-cli/tests/px7f_resource_native.rs
      fn linked_public_right_denial_preserves_exact_masks              (row 1)
      fn linked_public_second_release_is_closed_and_the_handle_closes_once
                                                                       (row 2)
    crates/ken-cli/tests/rt_escape_second_resource_native.rs
      fn escaped_buffer_used_by_fanning_host_op_matches_interpreter    (row 10)

**Do not carry a line number for these rows.** Three different pairs are in
circulation for rows 1 and 2 — `:311`/`:345` (at `a388dc06`), `:314`/`:348`
(the `#[ignore]` attribute lines), `:315`/`:349` (the `fn` lines). Every one of
them was correct about something. `git grep -n "fn <name>"` at your base.

All three `#[ignore]` strings are byte-identical, and **that is the error this
node exists because of, not evidence of a shared cause** — one node wrote all
three. None of the three appears in `.github/ignored-test-exemptions.toml`, so
all three are live members of the swept population that the operator's
ignored-test objective counts.

## `AC-0` — re-establish the premise at YOUR base, before anything else

**Name your own base SHA and measure there.** Do not use any SHA quoted in the
node or this frame as the tree you check against; all of them are stale by
construction.

Run the three rows targeted (`scripts/ken-cargo`, `-p ken-cli --test <name>`,
never `--workspace`) and paste the output. Record for each row:

1. Does it still carry an `#[ignore]` attribute, and is the string still the
   byte-identical one the node records?
2. What does it actually do when run — which phase does it stop in, and with
   what?

**Stop conditions, both reportable outcomes:**

- **If any row now passes**, stop on that row and report it. Do not repair
  forward. A passing row goes to the readmission path
  (`RT-IGNORED-PASSING-ROWS-DISPOSITION`'s discipline: readmit on a **mutation**
  that reds the row, never on the green), not to this census.
- **If the ledger's verdict no longer holds**, say so and stop. The premise of
  this node is that these rows' labels do not describe them; a row whose label
  has become accurate is out of scope and is a finding.

## `D0` — partition on PHASE first, then capture each stop's SITE

**Partition before you diagnose.** The ledger measured two different signatures
and that is evidence about **first stops**, not about causes:

- **rows 1 and 2** — `UnclassifiedRuntimeTrap { terminal_value: -1 }`. They
  COMPILE, LINK and RUN. The value comes from a live process.
- **row 10** — refuses at ObjectEmission inside the Cranelift static transition
  planner. **No artifact is produced and nothing runs.**

**That is a different layer, not a different symptom.** Do not fold row 10 with
rows 1 and 2 on the strength of a shared node or a shared label.

**For each row, capture the SITE of its stop, not just its signature.** The
node's founding error was treating identical refusal text as identical
behaviour; a sentinel value carries no mechanism.

## `D1` — rows 1 and 2: the first deliverable is a CLASSIFICATION, not a repair

**`UnclassifiedRuntimeTrap { terminal_value: -1 }` is not a diagnosis — it is
the absence of one.** A trap that reached the classifier and was not classified
tells you the row failed and nothing about why.

⇒ **Do not frame or build a repair against an unclassified trap.** Deliver
instead: **what is the criterion function that should have matched this trap,
and why did it not?** Verify the membership from the **criterion**, never from
the value.

**This is the one place a measurement-before-repair split is justified on this
node**, and the reason is in the frame rather than left to judgment: the repair
is genuinely unbounded until the trap is classified, so a guess would consume a
T1 turn to find out. Everywhere else on this node, build the best guess.

### What is established about `-1`, and what is NOT

**Established (Architect, `evt_7v4gb9y06ft6t`).** Five conditions hold in order
— trace decoded, all three of plan / target-ABI / host-effect-ABI hashes
matched, exit code present, no child-recorded `terminal_error` — **before**
`trace.terminal_value` is consulted. So `-1` is a value the child **wrote**
after a clean hash-validated handshake. It is **not** a fallback, not a
missing-trace sentinel, and not the OS exit status. The producer set collapses
to one: `ken_nc23_entrypoint`'s return (`STARTER_ENTRY_SYMBOL`).

**Established (runtime ring).** The native C stub keys its message on the
**value** — `if (value == -1) fputs("ken native trap: malformed borrowed
process input")` — so **the stderr line cannot discriminate**; it is a lookup
on an integer. Cite that branch **by the `value == -1` branch, never by line**
(the number has already moved from `:2221` to `:2303`). `-1` is one of four
reserved sentinels (`-1`..`-4`), with planned trap tokens at `<= -511`, so the
negative band is partitioned by contract and not free space.

**NOT established — the crux, and it decides the routing either way:**

> Rows 1 and 2 declare `proc main (_input : ProcessInput)`. The parameter is
> underscore-prefixed and **unused**. Does ingress validation run for a
> parameter the program never consumes?

If it does, an unused `_input` can still fail closed at `-1` and the borrowed
route is live. If it does not, these rows cannot be reaching `-1` that way and
they need their own account. **Answer this before proposing an owner.**

### Two named candidate mechanisms — and rows 1 and 2 have NOT got an owner

Write this in these words in your handback: **rows 1 and 2 have a named
candidate owner and not an owner.**

- **(a) The borrowed ingress.**
  `lowering/core/tests/effects.rs::borrowed_ingress_malformed_metadata_fails_closed`
  asserts `-1` for three malformed shapes, so `-1` is the ingress **fail-closed**
  value by design. **WEAKENED 2026-09-19:**
  `RT-BORROWED-INPUT-CARRIER-DURABILITY` was **closed REFUTED** with zero live
  witnesses (`4d0958988`) — the only rows ever measured to take a
  BorrowedOpaque-capture-to-`-1` path are un-ignored and return the computed
  value. **Note what that does and does not say:** those rows **consume** their
  `ProcessInput` (`walk (seed input) True`), so they say nothing about an
  **unused** ingress. The candidate is weakened, not eliminated.
- **(b) The rows' own stated reason.** Their `#[ignore]` string says *"a carried
  recursive hypothesis is an eliminated value, not a callable, but the call
  provides 1"* — a second, concrete candidate mechanism their own label names.
  **It is also demonstrably wrong in one respect**: it predicts a refusal, and
  these rows run. Treat it as a lead, not as an account.

### Search space for (a) — and the guard that makes a zero usable

If you pursue the borrowed route, `BorrowedOpaque`'s **EIGHT** non-test runtime
files, **SIX** of them under `cranelift_backend/lowering/`:

    crates/ken-runtime/src/boundary_value.rs
    crates/ken-runtime/src/boundary_value_clif.rs
    crates/ken-runtime/src/cranelift_backend/lowering/aggregates.rs
    crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs
    crates/ken-runtime/src/cranelift_backend/lowering/core/primitive.rs
    crates/ken-runtime/src/cranelift_backend/lowering/effects.rs
    crates/ken-runtime/src/cranelift_backend/lowering/joins.rs
    crates/ken-runtime/src/cranelift_backend/lowering/source.rs

**This count is a claim, not a label.** It was published as "seven … and the
five under lowering" and corrected to eight and six (`evt_5kwg9ha672eb4`). **If
your own `git grep -l BorrowedOpaque -- crates/ken-runtime/src` returns a
different set, the frame is wrong and your measurement wins** — say so.

> **ASK:** what does `ken_nc23_entrypoint` return when a borrowed input fails
> to carry?
> **NOT:** where is `-1` written?
> **AND:** a zero result from an emission-site search is **NOT evidence of
> absence** here. These values may be **residuals**, not emissions — the
> runtime's own unbound-identity test documents the siblings as `-3, // the
> pre-fold dynamic-constructor residual` and `-4, // the pre-fold root
> generated-unit collapse`. If `-1` is likewise what a fold leaves behind
> rather than what a site writes, **no emission site exists and the grep
> returns zero forever** while the mechanism sits in plain view. **Report such a
> zero as a query that could not hit, never as a finding.**

## `D2` — decide ownership per signature, and do not fold on a guess

**Fold rows only with an argument, and split them only with one.** Either
direction taken for free is the same error. If each needs its own node, say
that. If one node covers more than one row, state what it is that they share —
a **mechanism**, never a symptom, a value, or a label.

Row 10's ObjectEmission layer is already a stated reason to split it from rows
1 and 2. That split is the frame's position, not a finding; **if you can refute
it, do.**

`D2` hands back a **routing proposal**. It does not create nodes, close nodes,
or re-label rows.

## Acceptance criteria

- **AC-0** — the premise is re-established at your own named base, with output
  pasted, and both stop conditions above are honoured. *(Control: a row that
  passes, or a label that has become accurate, is reported and not repaired.)*
- **AC-1** — each of the three rows has its **phase** and its **stop site**
  recorded separately, with the instrument that produced each. *(Control: the
  answer for row 10 must be at a different layer from rows 1 and 2, or the
  partition is asserted rather than measured.)*
- **AC-2** — rows 1 and 2 have a **classification**: the criterion function that
  should have matched the trap is named, and the reason it did not match is
  stated. *(Control: the classification is derived from the criterion function,
  never from the terminal value.)*
- **AC-3** — the `_input`-unused crux is answered yes or no, with the evidence.
  *(Terminating observation, and it can come out either way: if ingress
  validation runs for an unconsumed parameter, candidate (a) stays live; if it
  does not, candidate (a) is eliminated for these rows.)*
- **AC-4** — any emission-site search that returns zero is reported **with the
  residual-versus-emission guard stated**, so the zero is auditable. *(Control:
  a bare "not found" fails this AC.)*
- **AC-5** — the file count in this frame's search space was re-derived, and
  either confirmed at eight/six or corrected with your own measurement.
- **AC-6** — a per-signature ownership proposal, with the fold-or-split argument
  written out for each grouping.
- **AC-7** — no row is un-ignored, no row is re-labelled, and
  `RT-SITEOP-CARRIED-WITNESS` is neither reopened nor amended. *(Control: the
  candidate's file roster contains no change to any `#[ignore]` attribute and no
  change under `docs/program/issues/RT-SITEOP-CARRIED-WITNESS.md`. A file roster
  discharges this; prose cannot.)*
- **AC-8** — no regression. **Green in CI, never a local `--workspace` run**
  (`COORDINATION §12`).
- **AC-9** — the handback states, in these words, **"rows 1 and 2 have a named
  candidate owner and not an owner"**, unless `AC-3` eliminated one candidate and
  `AC-2` classified the trap — in which case it names the owner and says which
  evidence closed it. *(Control: a handback that names an owner without an
  `AC-2` classification and an `AC-3` answer fails this AC. This exists because
  a candidate that fits is the one that gets promoted to owner silently.)*
- **AC-10** — every code citation in the deliverable is by **grep-able phrase or
  symbol name**, not by `path:line`. *(Control: three different line pairs are
  already in circulation for rows 1 and 2 and each was correct about something,
  so a reviewer cannot tell a stale coordinate from a wrong one. Where a number
  genuinely helps navigation, write it as "at `<sha>`, around `path:NNNN`" — an
  anchor to re-find, never a value to check.)*

## What must not happen

- **Do not re-label the rows to make them consistent.** The inconsistency is the
  evidence. A row pointing at a node that does not describe it is discoverable;
  a row pointing at a plausible node nobody checked is not.
- **Do not reopen or amend `RT-SITEOP-CARRIED-WITNESS`.** It is merged. If a
  finding bears on it, the finding belongs in a new node that cites it — a
  merged node's record of what it believed at merge time stays as it was.
- **Do not un-ignore any of the three to see what happens.** Two of them produce
  an unclassified trap; un-ignoring converts parked rows into red ones and buys
  one bit you can get from a targeted run.
- **Do not treat "the port succeeded" as in question.** It is not. Thirteen rows
  un-ignored and passing is a measured positive result. What is in question is
  the account of **where the survivors stopped**.
- **Do not infer a shared cause from the shared label.**

## Sequencing

No blocking dependencies. `RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL` is the
sibling census and its `D1` supplies the "classify, don't repair" instruction
this frame's `D1` carries. Read the ledger, not its commissioning node:
`docs/program/evidence/rt-ignored-failing-rows-ledger.md`.
