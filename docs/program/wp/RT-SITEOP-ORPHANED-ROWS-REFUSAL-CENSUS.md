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

## Which phase a candidate is in — decidable from its ROSTER alone

**AC-7 and AC-11 mandate opposite file rosters, so a reviewer must be able to
tell which one governs a given candidate without knowing what anyone intended.
This rule is what makes that decidable, and it is a SEQUENCING requirement, not
a definition:**

> **The label edit lands in its OWN candidate, carrying no other AC's
> deliverable. Every candidate before it is an investigation candidate.**

A reviewer classifies from the roster and nothing else: a roster containing an
`#[ignore]` attribute change is the closeout candidate and AC-11 governs it;
any other roster is an investigation candidate and AC-7 governs it. **It also
makes "`full` CI, never doc-only" follow from the shape** — the closeout
candidate is the only one that touches `crates/` — rather than being a separate
instruction someone has to remember.

> **Added by Steward respin 2026-09-19, before the Architect's vote, because
> the first version of this amendment left the boundary undefined and the
> runtime-implementer produced the collision case rather than a worry**
> (`evt_4zen8fkcjcp2c`). The terms "investigation" and "closeout" occurred five
> times across AC-7 and AC-11 and **zero times anywhere else in the frame**:
> each AC asserted which phase it governed and neither said how to tell.
>
> **The collision was the natural way to do the work, which is what made it
> serious.** `AC-11`'s string *is* `AC-6`'s output — the successor written into
> the attribute is the per-signature ownership proposal in the row's own words —
> so the obvious candidate carries both, and is then an investigation candidate
> by AC-6 and the closeout by AC-11. Two controls, both keyed on the roster,
> demanding opposite rosters. **A reviewer checking AC-7 blocks it; a reviewer
> checking AC-11 blocks its absence; both are reading the frame correctly.**
>
> **Defining the phase by "which AC it delivers" does not close it** — the
> collision is exactly a candidate delivering an investigative AC *and* the
> close obligation. **Defining it by the roster alone would be circular**, since
> AC-11 already defines the closeout roster, which makes AC-11 self-satisfying
> and leaves AC-7 no grip. Requiring the edit to land ALONE is what breaks the
> circularity: it constrains sequencing, and the roster then reports the phase
> as a consequence.

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
  written out for each grouping. **Its population is the EIGHT rows whose own
  `#[ignore]` text names no live node, not this node's three** (enumerated
  below). For each row **outside AC-11's closeout roster**, the proposal names
  **which live node is expected to apply the correction** — a ROUTING statement
  and nothing more. *(Control: the proposal has an entry for each of the eight,
  and each of the five outside the roster names an owning node that is `ready`,
  `active` or `draft` at the candidate's base. An entry that proposes mechanism
  content for a row this census did not measure fails this AC.)*

  > **Added by Steward amendment 2026-09-19, on the owner-liveness residue
  > measurement (`evt_73fa9v0ca03ve`). The residue is EMPTY — every one of the
  > 14 selected rows has a live owner — and that is exactly why the population
  > is eight.** The defect is not orphaning. It is that **8 of 14 rows' own
  > text names nothing live**, so ownership is real but recoverable only by
  > opening 126 live node frames and matching coordinates. A reader starts at
  > the row, and the row is a dead end.
  >
  >     THIS NODE (its 3, the closeout roster)
  >       px7f_resource_native.rs:314
  >       px7f_resource_native.rs:348
  >       rt_escape_second_resource_native.rs:684
  >
  >     RT-CONTEXT-CAPTURE-CLAIM-ABSENCE  [ready]
  >       px7l_checked_host_recursive_bind.rs:163
  >       px7l_checked_host_recursive_bind.rs:241
  >       px7m_hostresult_computational_match.rs:163
  >       px7m_hostresult_computational_match.rs:206
  >
  >     RT-BRACKET-RELEASE-ORDER-PARITY   [ready]
  >       px8ta_oriented_subcontinuation.rs:326
  >
  > The right-hand column is the arm-2 result, verified at each owning frame
  > rather than by grep — the first three are this node's own adoptees, the next
  > four are fenced by `file:line` at `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE`'s frame,
  > and `px8ta:326` is named by test name in
  > `RT-BRACKET-RELEASE-ORDER-PARITY`'s own title.
  >
  > **WHY ROUTING ONLY, AND WHY THIS IS A CONSTRAINT RATHER THAN A PREFERENCE.**
  > Raised by the runtime-implementer (`evt_70s2ffwwyn3b6`), endorsed as a
  > standing constraint by the Architect (`evt_1b6zpr83b4cr8`). A routing
  > statement is backed by the arm-2 measurement above and is writable here. A
  > **mechanism string** for `px7l`/`px7m`/`px8ta` would not be: this census
  > measured `px7f` ×2 and `rt_escape`, and those rows' mechanisms are the other
  > two nodes' findings — for `px7m:206` the likely correct answer is that the
  > refusal is RIGHT, which is `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE`'s conclusion to
  > reach. **A successor string authored from someone else's unfinished
  > measurement is authoritative-looking and unbacked, and a reader who greps
  > finds a hit and stops looking — strictly worse than the dead end it
  > replaces,** and the same label rot this node exists to clean up. It is also
  > what `AC-12` exists to catch, so manufacturing it here would defeat the check
  > one AC below.
  >
  > **AC-6 RECORDS the expectation; it does not make anything CHECK it.** The
  > five rows outside the closeout roster are corrected by their own nodes, whose
  > reviewers open **their** frames, not this one. The obligation therefore also
  > lands in each owning node's frame, the way `AC-11` sits in this one — see
  > `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE` and `RT-BRACKET-RELEASE-ORDER-PARITY`.
  > Recording and enforcing are two halves and **neither does the other's work**.
- **AC-7** — **on every INVESTIGATION candidate** (see *Which phase a candidate
  is in*, above — decide it from the roster, never from intent) no row is
  un-ignored, no row is re-labelled, and `RT-SITEOP-CARRIED-WITNESS` is neither
  reopened nor amended.
  *(Control: the candidate's file roster contains no change to any `#[ignore]`
  attribute and no change under
  `docs/program/issues/RT-SITEOP-CARRIED-WITNESS.md`. A file roster discharges
  this; prose cannot.)*

  > **SCOPED TO THE INVESTIGATION CANDIDATES BY STEWARD AMENDMENT, 2026-09-19,
  > BECAUSE AS WRITTEN IT FORBADE THIS NODE'S OWN STATED PRODUCT.** The issue
  > title says *"the first deliverable is a LABEL CORRECTION, which is
  > re-labelling and not clearing"*; unscoped, AC-7 said *"no row is
  > re-labelled"* and its control was the **absence of exactly that edit**. The
  > two could not both be satisfied by one candidate, so the node could not
  > close correctly. Raised by the runtime-implementer at `evt_3ds0ek6a218z7`.
  >
  > **AC-7's intent survives intact and is the right fence:** a measurement node
  > must not make an orphan disappear by quietly rewriting its label mid-census,
  > which is the same fence AC-0's control states (*"a label that has become
  > accurate is reported and not repaired"*). What was wrong was the **scope**,
  > not the rule — it reached the closeout, where the label correction is not a
  > repair-in-disguise but the node's actual deliverable. **AC-11 is where that
  > edit now belongs**, and the two ACs are complementary rather than in
  > tension: AC-7 bars the edit while measuring, AC-11 requires it at close.
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
- **AC-11 — CLOSE OBLIGATION. This node does not reach `merged` until each of
  its three rows either CLEARS or carries its successor owner at the START of
  its own `#[ignore]` string.** The successor is whatever this census's own
  measurement determined; if the answer is that no live node owns the row, the
  attribute says that in those words, naming this census as the node that
  established it. *(Control: the closeout candidate's file roster **contains**
  changes to `crates/ken-cli/tests/px7f_resource_native.rs` and
  `crates/ken-cli/tests/rt_escape_second_resource_native.rs`. This is the exact
  edit AC-7 bars on the investigation candidates and requires here — check the
  roster, not the prose. Because it touches `crates/`, this candidate is
  **`full` CI, never doc-only**.)*

  > **Added by Steward amendment 2026-09-19. This AC exists because the
  > federation's guard against exactly this failure returns ZERO here, and the
  > zero is guaranteed rather than informative.** `M7a` asks whether any row
  > names the node that is merging. These three rows begin
  > `#[ignore = "RT-SITEOP-CARRIED-WITNESS D2: …` — they name the node that
  > **merged**, not the one that is merging — so M7a discharges green at this
  > node's close and all three rows stay orphaned. Measured by the
  > runtime-implementer with both controls: the closing node's ID returns 0, the
  > predecessor's returns 3, a nonexistent ID returns 0. **The instrument works;
  > the population it searches cannot contain a hit for an adoption node.**
  >
  > **That is the whole reason this node exists, recurring at its own close.**
  > The census was created because three rows named a merged owner and no live
  > node would ever correct them. Closing it under a label-keyed guard would
  > hand back the identical state, and the step that would have caught it is the
  > one whose node just closed. `M7a` has since been given a second arm — check
  > the closing node's own frame for row coordinates, not only the label grep —
  > and **this AC is the same obligation written where the ring will read it**,
  > because a step in the Steward's merge procedure is not visible to the seat
  > authoring the candidate.
  >
  > **Who writes the string:** the ring, not the lieutenant. The reason text
  > needs the census's own findings, which the publisher does not hold.
- **AC-12 — the closeout candidate's review RECONCILES each attribute's
  successor string against the approved `AC-6` proposal.** Not *"does the row
  name a successor"* — `AC-11` already checks that — but **is the successor it
  names the one `AC-6` established, for that signature.** *(Control: the review
  quotes, per row, the `AC-6` grouping it came from and the string as written.
  A reviewer who checks only that a successor is present fails this AC.)*

  > **Named by the Architect at `evt_3z4e7gdv92hd2` as the obligation the phase
  > split creates, and it is the price of the rule above.** Requiring the label
  > edit to land alone puts `AC-6`'s proposal in one candidate and the attribute
  > strings **whose content is that proposal** in another. **That is a
  > transcription across a candidate boundary, and transcription drifts.**
  >
  > Without this check the frame would have traded an undecidable phase boundary
  > for a silent divergence between what was reasoned and what got written into
  > the durable label — and **the durable label is the artifact that outlives
  > both candidates.** It is what the next census reads, long after the proposal
  > justifying it has scrolled away. **A label that disagrees with its own
  > proposal is indistinguishable from a correct one at the point of use**,
  > which is precisely the failure family this census exists to fix.

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

## Measured outcome — increment 1 (AC-0, AC-1, AC-2, AC-5; AC-3 bounded)

**Base: `5fe2b9bd4aef626cf1fabfe1b00dc8dca886e2fa`**, re-anchored so
`merge-base == origin/main`. **No row un-ignored, no row re-labelled,
`RT-SITEOP-CARRIED-WITNESS` neither reopened nor amended. Docs-only.**

### `AC-0` — premise re-established at this base. Neither stop condition fired.

Rows located by symbol, per `AC-10`; numbers are anchors to re-find, not values
to check.

    px7f_resource_native.rs
      fn linked_public_right_denial_preserves_exact_masks              ~:315
      fn linked_public_second_release_is_closed_and_the_handle_closes_once ~:349
    rt_escape_second_resource_native.rs
      fn escaped_buffer_used_by_fanning_host_op_matches_interpreter    ~:685

All three still carry an `#[ignore]`, and the three strings are **one distinct
value** (md5-equal). None appears in `.github/ignored-test-exemptions.toml`, so
all three remain live members of the swept population. **No row passes** and
**the ledger's verdict still holds**, so neither reportable stop condition is
triggered.

### `AC-1` — phase and stop site, partitioned on the instrument's own field

    rows 1, 2   RUNTIME.  The artifact builds, links and runs.
                "linked PX7-F child emits its canonical observation:
                 UnclassifiedRuntimeTrap { terminal_value: -1 }"
                scripts/ken-cargo test -p ken-cli --test px7f_resource_native
                  --no-fail-fast -- --ignored --test-threads=1
                -> 0 passed; 2 failed

    row 10      COMPILE.  ObjectLinkerPackagingError
                  { stage: ObjectEmission, field: "checked_process_object" }
                reason: native static transition planner invariant failed;
                  source-specific inheritances at one generated entry disagree
                  on their typed consumer projection, including the
                  fresh-result route
                -> no artifact is produced and nothing runs

**The partition is measured, not asserted:** the layer difference is carried by
the packaging error's own `stage` field, not inferred from the signatures
differing.

### `AC-2` — rows 1 and 2 classified from the CRITERION, not the value

The criterion is `decode_signed_root_trap`, which delegates membership to
`root_trap_catalog_index`. A planned root trap is encoded
`-((identity << 8) | 0xff)`, with the tag in the low byte and a 1-based
identity above it. Evaluated on this row's value:

    terminal_value -1  ->  magnitude 1
      1 & 0xff == 0xff ?   FALSE   <- fails HERE, the tag clause
      1 >> 8 != 0 ?        false   (would fail the identity clause too)
    smallest legal token   -511

⇒ **`-1` is not a malformed trap token. It is not a trap token at all.** The
classifier is correct and its refusal is accurate, so `UnclassifiedRuntimeTrap`
is neither a classifier failure nor a missing catalog entry — **and no
trap-catalog node owns these rows.**

**The stub cannot corroborate or refute this.** Its diagnostic is keyed on the
value — the `value == -1` branch in the process starter's C stub — so the
stderr line `ken native trap: malformed borrowed process input` is a lookup on
an integer and carries no mechanism.

### `AC-5` — search space re-derived, and the frame is right

    git grep -l BorrowedOpaque -- 'crates/ken-runtime/src/*' | grep -v test
    -> 8 files, 6 under cranelift_backend/lowering/

**Confirmed at eight and six.** The frame's count stands.

### `AC-3` — BOUNDED, NOT ANSWERED

The question — does ingress validation run for a `proc main (_input :
ProcessInput)` the program never consumes — is **not** answered by this
increment. What is delivered is its boundary.

    CLOSED  ken-runtime, non-test references to PROCESS_INPUT_CONSTRUCTOR:
            the constant, one NativeProcessSymbols field, and
            native_process_input_value -- which CONSTRUCTS the value.
            Nothing builds a destructure.
    CLOSED  the elaborator: eight non-test process_input_constructor sites,
            all field plumbing or admission-time resolution
            (program_admission's get("MkProcessInput")). None builds a Match.
    CLOSED  the entry-wrapping route in lowering/core.rs -- EXAMINED in
            increment 1a. All ten sites are the SAME value THREADED, not
            consumed: `staged_process_input: Option<&RuntimeValue>` in four
            signature positions and the matching forwarding arguments, ending
            in compile_expr_into_module_with_root_projection, which forwards
            it onward again. No Match, no destructure, no unwrap of the
            Option anywhere in the file. core.rs carries the staged value; it
            does not validate it.
    LOCATED the forward target, in increment 1b: core.rs hands the value to
            `super::units::define_unit_bodies`, in
            cranelift_backend/lowering/units.rs -- a file named by NEITHER
            enumeration (not in the eight BorrowedOpaque files, not in the
            six entry-wrap files).
    OPEN    whether units.rs CONSUMES or forwards again, plus artifact/api.rs,
            artifact/mod.rs and platform_runtime_support.rs.

> ### THE VALUE IS RENAMED AT THE CALL BOUNDARY, AND THAT IS WHY THE TRAIL
> ### READS AS ENDING
>
>     core.rs    staged_process_input: Option<&RuntimeValue>
>     units.rs   staged_root_value:    Option<&RuntimeValue>   <- same value
>
> `grep staged_process_input` over `units.rs` returns **zero**, and the file is
> 8,445 lines, so the zero is entirely plausible and entirely wrong. It was
> caught only because the call site provably passes the value, so the callee
> must receive it under some name — implausibility, not a better key.
>
> ⇒ **Any enumeration of this value keyed on `staged_process_input` stops at
> `core.rs` by construction.** Follow the parameter across each call boundary
> and re-read the callee's signature for its local name; do not carry one name
> across a hop. This is the same defect class as `AC-4`'s residual-versus-
> emission guard, one layer along: there the value may never be written, here
> it is written under a name the query does not contain.

**The remaining routes are handed over as the predicate that generated them,
not as a list**, so the next reader can re-run the enumeration and learn
whether it is still the same size:

    git grep -ln 'staged_process_input\|STARTER_ENTRY_SYMBOL' \
      -- 'crates/*/src/*' | grep -v test
        cranelift_backend/artifact/api.rs
        cranelift_backend/artifact/mod.rs
        cranelift_backend/lowering/core.rs        <- 10 sites, the entry wrap
        native_process_entrypoint.rs
        object_linker_packaging.rs
        platform_runtime_support.rs

    grep -c 'staged_process_input\|STARTER_ENTRY_SYMBOL' \
      crates/ken-runtime/src/cranelift_backend/lowering/core.rs
        -> 10

### TWO INSTRUMENT FAILURES OF THIS INCREMENT, recorded beside the finding

Both are recorded here rather than in a scratchpad because each fails toward
*"nothing here"*, and a false zero that agrees with the reader is the one
nobody re-runs.

**1. A `#[cfg(test)]`-position filter returned a false zero over ten sites.**
I filtered production as *"lines before the first `#[cfg(test)]`"*. That is
sound for a file whose `mod tests` sits at the bottom. **In `core.rs` the first
`#[cfg(test)]` is at line 26 and decorates a `use`** — a test-only import, not
a module boundary — so the filter classified **16,461 of 16,487 lines as test**
and returned an **empty set over a population of ten.** The zero was the
instrument. ⇒ **Do not reuse that filter.** Resolve the enclosing item, or grep
the whole file and classify each hit.

**2. An ingress destructure read out of a test module.** The `Match` on the
process-input constructor with `binders: 3` and a `PatternMatchFailure` default
sits inside `native_process_entrypoint.rs`'s `mod tests`. Read as production it
would have answered `AC-3` from a test harness's own entry construction.

**And a limit on the counter-witness carried in from the sibling node.**
`abi_s6_mapping_surface_native` declares `proc main (_input : ProcessInput)`,
runs genuinely native and passes with no `-1`. That refutes *"proc plus an
unused `_input` inevitably yields `-1`"* and is **silent on `AC-3`**: a pass is
equally consistent with validation not running and with validation running and
succeeding on a well-formed input. It also needs re-running — `abi_s6` is the
one file that moved between the base it was measured at and this one.

### `AC-9`

**Rows 1 and 2 have a named candidate owner and not an owner.** `AC-2` is
discharged and `AC-3` is not, so no owner is named here.

### Not delivered

`AC-4` is discharged only for the zeros this increment actually produced;
`AC-6`'s fold-or-split argument, and any routing proposal, await `AC-3`.

### Increment 1c — a published pointer CORRECTED, and the launch ingress located

**`AC-3` is still not answered.** This increment fixes a wrong pointer I
published and replaces it with a measured one.

**THE CORRECTION.** I posted, and wrote into the handover, *"start at
`NativeProcessInput` in `object_linker_packaging` — the launch producer."*
**That is wrong.** Measured:

    object_linker_packaging.rs   4502 lines, `mod tests {` opens at :2398
    its only NativeProcessInput  :3044 -- INSIDE mod tests
    production constructions     ZERO

I grepped a symbol, took its single hit, and named it the launch producer
without resolving its enclosing item. **Same failure as the two already
recorded above**, committed while writing the section that warns about them.
Every `NativeProcessInput {` in `native_process_entrypoint.rs` is likewise at
`:576+` against a `mod tests` opening at `:399`, so **`NativeProcessInput` has
no production constructor in either file.**

**THE LAUNCH INGRESS, LOCATED — `boundary_activation.rs`, production by
measurement** (`mod tests` opens at `:530`; all sites below are above it):

    :432  fn bind_process_frame(process_input: *const c_void,
                                host_context: *mut c_void, capability: u64)
    :440  boxes it into GeneratedRootIngressV1 { process_input, ... }
    :512  pub struct GeneratedRootIngressV1 -- #[repr(C)], the frame the
          generated adapter receives

The source names it in its own terms: *"The launch ingress is Rust-owned and
C-opaque"*, and the field is *"the borrowed process-input value the launcher
built"*.

**What `bind_process_frame` does with it:** its only guard is
`if self.finished || !self.is_published() { return None; }` — a lifecycle
check on the activation, **not on the value**. The pointer is boxed and handed
to generated code unexamined. **No destructure, no arity check, no
constructor check at the boundary.**

⇒ **Bounded reading, and it is NOT `AC-3`:** the launch ingress does not
validate the process input; it transfers a pointer. Whether anything validates
it therefore depends on **what the generated code does with the frame** when
the program declares an unused `_input` — which is lowering/emission work and
is where `AC-3` now sits. That is the next question and it has not been asked.

**Note for whoever asks it:** the field's own word is **"borrowed"**, which
connects this path to candidate (a) rather than away from it. Candidate (a) is
still not eliminated and this increment does not bear on it either way.

## `AC-3` — ANSWERED. Ingress validation RUNS. Candidate (a) stays live.

Measured at `0e88e3167ec23ec18fe7c31b467c1d08707cbd83`, read through
`git show <sha>:<path>` throughout, so every coordinate below names the tree it
was read on.

**The answer, in one sentence:** a `proc main (_input : ProcessInput)` that
never consumes the parameter still gets a nonzero check emitted against the
borrowed process-input pointer, because the emission is gated on the **compile
lane**, not on the body's use of the parameter.

### Both rows were written down BEFORE the run

Pre-registered, per the fleet rule that a predicate evaluated only under the
reading you are trying to confirm is not a discriminator:

| reading | what the emitter would have to look like |
|---|---|
| candidate (a) LIVE | the load and its check are emitted from a condition that does not mention the body — a lane flag, a signature role, a plan slot |
| candidate (a) ELIMINATED | the load and its check are emitted from the parameter's **use site**, so an unconsumed `_input` emits neither |

These are different code shapes and only one of them is present, so the
observation separates them rather than confirming one of them.

**And the surface trap was pre-registered too.** "The emitted code contains no
reference to the process input" is produced by candidate (a) being eliminated
**and** by the validation living in the C stub instead. So both surfaces were
measured, not just the emitter.

### The chain, by symbol

    emit_bound_process_program_object_with_cranelift   artifact/api.rs
        passes the process-mode argument as a LITERAL true
        (at 0e88e3167, around crates/ken-runtime/src/cranelift_backend/
         artifact/api.rs:412)

    define_root_adapter                                lowering/units.rs
        pub(super), column 0, production: the only two column-0 `mod`
        openings in that file are `captured_environment_bijection` and
        `admitted_matches_role_sequence`, both far below it
        gates on process-mode twice, and on nothing else:
          - refuses "process root has no declared role-keyed ingress slot"
            for the ProcessInput and Capability roles
          - loads ROOT_INGRESS_PROCESS_INPUT off the ingress and calls
            Lowering::require_nonzero on it

    Lowering::require_nonzero                          lowering/mod.rs
        invalid branch emits `iconst(I64, -1)` then `return_`

    the C stub ladder                                  object_linker_packaging
        `if (value == -1) fputs("ken native trap: malformed borrowed
         process input")`

**The load-bearing link is the first one.** The process-mode argument is a
literal at each call site — `true` in the bound-process-program entry, `false`
in the seed and ordinary-program entries — and it appears nowhere else in the
workspace. It is therefore a property of **which compile entry the caller
chose**, fixed before the body is examined at all. It cannot be a function of
whether the body consumes the parameter.

### What this settles, and what it does not

**Settles `AC-3`:** ingress validation runs for an unconsumed `_input`.
Candidate (a) is not merely un-eliminated; it is affirmatively reached.

**Does NOT settle the attribution of rows 1 and 2's `-1`**, and the reason is
a closure argument rather than a gap in effort. Enumerating the producers
rather than the occurrences: `iconst(types::I64, -1)` followed by `return_`
appears at **ten production sites** in the lowering tree — three in
`effects.rs`, four in `joins.rs`, three in `mod.rs`. Only one of the three in
`mod.rs` is `require_nonzero`.

Of those ten, exactly one is emitted **directly into the entry adapter**, so
only that one's `-1` reaches the stub as the entry symbol's return without
further translation. Whether the other nine propagate unchanged is **not
established by what was run here**, and is not claimed.

### The finding this turned up, which is larger than the crux

`define_root_adapter` emits **four** `require_nonzero` calls into the entry
adapter, and all four return the identical `-1`:

| checked value | emitted under |
|---|---|
| the native int arena | unconditionally, every lane |
| the boundary arena | unconditionally, every lane |
| the borrowed process input | process mode only |
| the host dispatch context | process mode only |

The stub prints **"malformed borrowed process input"** for all four, because
its first rung is a fixed label on an integer. **Two of the four are arena
checks that have nothing to do with the process input and fire in every lane,
including the lanes that pass process-mode false.**

⇒ The stderr line was already known not to discriminate. What is new is the
size of what it fails to discriminate: within the entry adapter alone the
channel has four members, and the label names one of them.

### And the earlier `-1` result is now EXPLAINED rather than merely stated

`AC-2` established from the criterion function that `-1` is not a planned trap
token at all: magnitude 1, and `1 & 0xff` is not `0xff`, so it fails the tag
clause before anything else is consulted. That was derived from
`root_trap_catalog_index` without knowing where `-1` comes from.

It comes from here. `require_nonzero` builds its refusal by hand —
`iconst`, `return_` — and never goes near the trap-token encoding. **`-1` is
not a malformed trap token; it belongs to a second refusal channel that shares
the return register with the trap channel.** The two results agree, and the
second is the mechanism for the first.

### `AC-9`

`AC-3` is answered and `AC-2` is classified, so the `AC-9` condition is met on
its face. It is **not** taken here, and the wording stands: **rows 1 and 2 have
a named candidate owner and not an owner.** `AC-3` answered the direction the
crux was posed to test — it kept candidate (a) alive rather than eliminating
it — and a candidate that survives its own elimination test is exactly the one
that gets promoted to owner silently, which is the failure `AC-9` was written
against. Naming the owner additionally requires attributing the `-1` to one of
the four adapter checks, and the paragraph above says why that is open.
