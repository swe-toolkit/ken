# `CI-IGNORED-SWEEP` — frame

Owner: **verify**. Size: **S**. Gate: none. Depends on: nothing.
Origin record: [`CI-IGNORED-SWEEP`](../issues/CI-IGNORED-SWEEP.md)

Ground: `origin/main` **`20c7a291`**, re-grounded at release. Every line number
and tool fact below was re-read at that ref.

**Re-grounding note, and it is evidence for this node rather than bookkeeping.**
The frame was first ground at `368ff87e`, where the population was **50**. It is
now **51**, and the delta is fully accounted in `1a`. The population moved twice
while this node sat `ready` — which is precisely the claim the node makes about
the population being unmonitored.

## 0. Posture

Nothing in the repo ever re-runs an ignored row. The suppressed population is
write-only: a row goes in, and no mechanism asks whether it still belongs.
When an owner node lands its repair, the `#[ignore]` persists — so the repair
ships with its own regression cover switched off.

**Treat every anchor as perishable. If a fixed input turns out false against the
landed code or the landed tooling, say so and escalate — do not quietly build
around it.**

**This node builds an instrument. The failure mode of an instrument is
reporting when it measured nothing**, which is the exact defect the two `px8f`
jobs already had. Section 3 is written against that, not against ordinary
correctness.

## 1. Fixed inputs

### 1a. The population is selected by attribute, not provenance — and it MOVES

**Pin the derivation, not the number.** The population is whatever this command
returns; the count below is its value at one instant, not a constant.

```sh
git grep -cE '^[[:space:]]*#\[ignore' <ref> -- crates/<crate>
```

Use the **anchored** form. The unanchored form also matches doc-comment lines
that merely mention the attribute, and inflates every file.

Value at `20c7a291`:

```
total 51 — ken-cli 33, ken-verify 10, ken-runtime 5, ken-interp 3
```

**The delta from the `368ff87e` grounding is fully accounted, and you should
not re-derive it:**

| crate | was | now | cause |
|---|---|---|---|
| `ken-cli` | 34 | 33 | `RT-CARRIER-BYTESPAN-OBSERVE` `D5` (`e0fc15c3`) repaired the `ConsoleWrite` byte-span seat and **un-ignored its row** in `tests/px4b_native_production.rs`. A correct repair-and-un-ignore by the owning node |
| `ken-runtime` | 3 | 5 | two rows added: `cranelift_backend/artifact/api/tests.rs:95` (`RT-FNUNIT-RESULT-TOKEN`) and `cranelift_backend/planning/static_transition.rs:16537` (the carried `KERNEL-NESTED-IND` control) |

**Two things follow, and they are the reason this section was rewritten.**

1. **A literal count is the wrong pin.** It moved twice in two days from
   ordinary, correct work. An `AC` or hard stop keyed on `50` would fire on
   healthy activity and train its reader to re-baseline, which `4` bans.
2. **The `ken-cli` un-ignore is the good-news event this node exists to
   route, and nothing reported it.** The owner repaired the seat and lifted its
   own row, which is the *correct* path — but it is visible here only because
   the Steward diffed two refs by hand at release time. That is the same
   luck-of-scope that `1g` records, in the healthy direction.

The node's original `46` was a scope error, not a miscount: it summed the
42-row authorized set and the four pre-existing `px4b` ignores, counting the
population this program **authored** rather than the population the mechanism
will **select**. A sweep selects on the attribute.

### 1b. Only 44 of the 51 have ever been measured, and the run is now stale

The one hand-run, at `7d204438`:

```
ken-cli    --no-fail-fast -- --ignored   ->  0 passed / 34 failed
ken-verify --no-fail-fast -- --ignored   ->  0 passed / 10 failed
```

That covered `ken-cli` and `ken-verify` only. **The five `ken-runtime` and
three `ken-interp` ignores were never in it**, so the over-annotation question
is open for those **eight**. Closing it is `D5`.

**Two caveats on the 44, both of which widen `D5` rather than the frame's
estimate of it:**

- The `ken-cli` half was `34` rows at `7d204438` and the crate now holds `33`,
  so that number does not describe today's tree either.
- `ken-runtime` grew from three rows to five *after* the hand-run, so two of the
  eight unmeasured rows did not exist when the gap was first described.

### 1c. Two independent classification axes, and rows can be in both

- **Axis 1 — reason for ignoring:** `base-debt-awaiting-repair` versus
  `ignored-by-policy`. Four rows are policy and will answer *"yes, still
  belongs"* forever:

  | row | class |
  |---|---|
  | `crates/ken-runtime/src/boundary_value_clif.rs:7884` | **COST** — `"~142s of arena work; the fast instance at depth 3000 runs by default"` |
  | `crates/ken-interp/tests/l1_acceptance.rs:243` | **UNBUILT CAPABILITY**, and **assertion-free** — see `1c.1` |
  | `crates/ken-interp/tests/l1_acceptance.rs:285` | **UNBUILT CAPABILITY**, and **assertion-free** — see `1c.1` |
  | `crates/ken-interp/tests/l1_acceptance.rs:335` | **VACUOUS** — comment-only body; **passes** — see `1c.1` |

  **The cost row moved from `:7473` to `:7884` between grounding and release,
  with no change to the row itself.** That is the concrete justification for
  `D1`'s decision to key the registry on **test path** rather than `file:line`:
  had the registry existed and been keyed on position, it would already be
  stale, and it would have failed *toward exempting the wrong row*.

#### `1c.1` — THREE of these four assert nothing. Do NOT seed them as policy.

**Steward disposition of the `D5` hard stop, `evt_15argr23kn3rq`, 2026-08-09,
re-measured at `d75d8c48`.** The implementer found
`sec24_char_excludes_surrogates` **passing while ignored** and correctly stopped
— then correctly refused to call it a repair, because the body is a comment and
nothing else.

Measured across `l1_acceptance.rs`: of 17 tests, **four contain no `assert*` and
no `panic!`**, and **three of those four are exactly the three ignored rows
above**. `#[ignore]` has been the storage mechanism for unfinished tests.

⇒ **The `UNBUILT CAPABILITY` label on rows `243` and `285` is true and
insufficient.** Their bodies `unwrap()` an elaboration and check nothing, so on
the day their capability lands they go green **without ever testing the property
their names claim**. The `#[ignore]` is the only thing preventing a false green,
and it is held there by an unrelated fact that expires on repair.

**Consequences for `D1`, which override its "seed exactly the four rows"
instruction:**

- Seed **only the cost row** as an ordinary policy exemption. It is the one row
  whose exemption is unconditional and whose reason cannot expire.
- The three `l1_acceptance` rows are seeded in a **third class**,
  `placeholder-no-assertions`, which is **exempt from the sweep but NOT from
  scrutiny**: the registry entry must record *what re-admission requires*, which
  for these rows is **an assertion**, not a capability.
- **A registry entry that records only "exempt" is what this section forbids.**
  Permanently exempting a vacuous test is how it stops being looked at, which is
  the same write-only failure one level up.

**Out of scope here, and already filed:** the assertion-free rows themselves,
including `ac2_expected_type_overrides_default:110` — which is **live, green,
un-ignored, and counted as cover today**, and is therefore structurally
invisible to this sweep. That is [[CI-ASSERTIONLESS-L1]]. **This node does not
fix, un-ignore, or rewrite any of those bodies.**

- **Axis 2 — where the row dies:** some rows fail *upstream of their own
  property*, at an `expect` before any assertion runs.
  `RT-WORKER-FIXTURE-DECODE` and `RT-CARRIER-PRODUCER-OCCURRENCE` are both this
  shape. For them "still failing" is not the useful bit; "can the fixture even
  execute" is.

The axes are independent — a row can be base debt *and* die before its
assertion.

**A third shape, added at release, and it is not a new axis.** The carried
control at `static_transition.rs:16537` is base debt awaiting a repair, so
axis 1 classifies it correctly — but **its `#[ignore]` reason states a release
condition that is itself wrong.** `RT-BODY-OCCURRENCE-PROVENANCE` `D7` is open
precisely because that condition names a *merge event* rather than the
capability, and it has already gone true without the capability arriving.

⇒ **Do not treat an `#[ignore]` reason string as authority for anything except
routing.** `D4` may read the owner-node id out of it. Nothing in this node may
read a *release condition* out of it, and the sweep must not attempt to decide
whether a row is ready to be un-ignored — it reports, the owner decides.

### 1d. The cost row is not merely noise, it is a budget

`boundary_value_clif.rs:7884` costs about 142 seconds. **A sweep that re-runs
the whole ignored population pays that on every run, unbudgeted.** It must be
exempt from the first run, not after someone notices.

### 1e. The two venues use DIFFERENT tools, and this decides the shape

Measured at `20c7a291`:

- **CI runs nextest.** Main gate is `cargo nextest run --workspace --locked`
  (`.github/workflows/ci.yml:121`); the dedicated native jobs are at `:213`,
  `:262`, `:315`.
- **Locally, nextest is NOT INSTALLED.** `cargo nextest --version` reports
  `no such command: nextest`. Local agents run `scripts/ken-cargo test`, which
  wraps cargo/libtest under a machine-wide build lock.
- **There is no `.config/nextest.toml`** in the tree.

Consequences the deliverables must respect:

1. **The `1b` hand-run used libtest syntax (`-- --ignored`). It does not
   transfer to the CI job.** nextest selects ignored rows by its own flag.
   **Verify the exact spelling against the landed tool before pinning it in a
   workflow** — this frame deliberately does not name it, because I could not
   run nextest to check.
2. **The implementer cannot iterate on a nextest invocation locally.** Develop
   and validate the *selection logic* per-crate with `ken-cargo test -p <crate>`,
   and treat the workflow wiring as CI-verified only.
3. **`COORDINATION §12` forbids a local `--workspace` run.** The sweep is a CI
   job. Locally, per-crate only.

### 1f. Eight owner nodes are queued against this population

`RT-CARRIER-BYTESPAN-OBSERVE`, `RT-CARRIED-RESOURCE-SCALAR`,
`RT-CLOSURE-BOUNDARY-LANE`, `RT-COMPMATCH-TREE-SCRUTINEE`,
`RT-FRAME-MARKER-ONCE`, `RT-PROCESS-EXIT-STATUS`, `RT-WORKER-FIXTURE-DECODE`,
`RT-CARRIER-PRODUCER-OCCURRENCE`. Each will land a repair whose row nothing
currently un-ignores.

### 1g. It has already failed once, and luck caught it

`RT-SRCBODY-BIND-ORDER` `D11` ignored `px7o` on a false premise and **would have
switched off a working repair.** It was caught only because `D12` happened to
run an enumeration that included ignored rows. A normal verification run cannot
see this by construction: `D13`'s `120 passed / 0 failed / 34 ignored` is
*disjoint* from the population it suppresses.

## 2. Deliverables

**`D0` — pin the venue and confirm the tooling gap.**
Record the exact nextest invocation that selects only ignored rows, verified
against the landed tool in CI, and state plainly that the `1b` local syntax is
not it. If nextest cannot express the selection, say so — that is a finding,
and the fallback (a per-crate libtest job) is a re-sizing conversation, not a
longer turn.

**`D1` — build the policy-exemption carrier, and seed it.**

**Decided, do not relitigate:** the carrier is an **explicit checked-in
registry** of policy-exempt rows, keyed on **test path** (`crate::module::fn`),
not on `file:line`, which drifts.

Rationale, so the constraint is legible rather than aesthetic:

- **Do not parse the reason string for prose.** These four rows are
  distinguishable today only by wording a human chose. A sweep that greps for
  `"not yet in scope"` is one reword away from silently re-including a row.
- **Fail toward sweeping, never toward skipping.** An unregistered row gets
  swept. A base-debt row omitted from the registry therefore costs *noise*; a
  policy row omitted costs a missed regression. **Noise is self-correcting and
  a missed regression is the thing this node exists to prevent**, so the
  default must be "sweep it".
- Seed it per `1c` **as amended by `1c.1`**: the cost row from `1d` as an
  ordinary policy exemption, and the three `l1_acceptance` rows in the
  `placeholder-no-assertions` class, each recording that **an assertion** is
  what re-admission requires. Do not seed all four as plain policy — `1c.1` says
  why.

**`D2` — the sweep job itself.** Non-blocking **for findings**: a row that
starts passing is good news needing routing, not a red gate, and must not become
a fourth way for an unrelated candidate to be blocked. **Instrument failure
stays blocking** — see `AC-4a`, which is operative and was written after this
interaction blocked a real merge.

**`D3` — the positive control.** Un-ignore one known-failing row, observe the
sweep reports the change, restore it.

**`D4` — routing.** Name where a finding goes. The owner node id already in each
base-debt `#[ignore]` string is the natural address; say what happens for a row
whose id names no live node.

**`D5` — close the open over-annotation question for the eight unmeasured rows**
(five `ken-runtime`, three `ken-interp` from `1b`). Run them per-crate
locally. Expect the three `ken-interp` L1 rows to fail for unbuilt-capability
reasons, which is the `1c` policy class, not over-annotation.

## 3. Acceptance criteria

### `AC-1` — the sweep asserts a POSITIVE, never the absence of a failure token

> **MEASURED:** the job asserts a suppressed-row **count** and the exit status.
> **CLAIMED:** the sweep ran and observed the population.
>
> **The count is asserted against `1a`'s derivation, not against a literal
> baked into the job.** The population legitimately moves — it moved twice
> between grounding and release. So the assertion is *"the number of rows the
> sweep selected equals the number the anchored grep finds at the commit under
> test, minus the registry"*, which stays true across healthy activity and
> still fails when the sweep selects nothing. **A hard-coded `51` would be a
> stale pin within days and would train its reader to re-baseline**, which
> section 4 bans.
> **THE GAP:** a job that selects zero tests and exits 0 satisfies a required
> check while carrying no signal. **That exact defect already shipped here** —
> two `px8f` jobs went to zero selection, and the `build-test` aggregator
> (`ci.yml:333`, result loop `:344-360`) tests only `[ "$result" != "success" ]`.
> A check for "no failure token in the output" passes identically when nothing
> ran. Note both `px8f` jobs still carry `--no-tests=pass` (`:213`, `:262`),
> so zero selection is *by construction* not an error there.

### `AC-2` — the sweep is proved live by mutation, not by its colour

> **MEASURED:** with one known-failing row un-ignored, the sweep's report
> **changes**; restore it and the report returns.
> **CLAIMED:** the sweep reaches the suppressed population.
> **THE GAP:** a green sweep is exactly what a sweep that never ran produces.
> Record the observed before/after counts, not "control passed". Say which row
> you used.

### `AC-3` — the registry cannot deselect anything from the MAIN gate

> **MEASURED:** temporarily add a **live, passing** test to the policy registry;
> the main `cargo nextest run --workspace --locked` job still runs it. Remove it.
> **CLAIMED:** the exemption mechanism is scoped to the sweep.
> **THE GAP:** if the registry is implemented as a default nextest profile or a
> repo-wide filterset, it silently narrows every run, including the required
> gate. **A mechanism that can exempt a row from the sweep must be structurally
> unable to exempt it from the gate**, and this control is what distinguishes
> the two. If it fires, the carrier is in the wrong place — move it, do not
> add a warning.

### `AC-4` — non-blocking is demonstrated, not asserted

> **MEASURED:** with the sweep reporting a newly-passing row, the required
> `build + test` check is still green.
> **CLAIMED:** the sweep cannot block an unrelated candidate.
> **THE GAP:** "we set it non-blocking" is a claim about intent. The
> aggregator's wiring is the thing that decides, and it already reads job
> results.
>
> **Concretely:** `build-test` (`ci.yml:333`) declares
> `needs: [test-shard, native-write-partition, native-buffer, native-rt-parity]`
> at `:336` and re-checks each result at `:344-360`. **The sweep job must not
> appear in that `needs:` list, and must not be added to the result loop.**
> Those two edits are the whole of what "blocking" means here, so the control
> is a diff check on `:336` and `:344-360` plus the observed green.
>
> Read the comment at `ci.yml:158-167` before touching any of this — it records
> the inverse hazard, a job whose result stops being inspected while `needs:`
> still names it.

#### `AC-4a` — the EXIT STATUS must separate a broken instrument from a finding

**Steward decision, 2026-08-09, on verify-leader's proposal `evt_6azs3wbbe5551`,
after PR #1714 was blocked by this exact interaction. This is operative.**

`AC-4` as written is satisfiable while the defect it exists to prevent still
occurs. Measured: the sweep job sits outside `build-test`'s `needs:` and result
loop, so the required aggregator is untouched and `AC-4`'s control passes — **and
a red sweep still blocked a merge**, because the publisher path fails on *any*
failing check. That is section 4's banned "fourth way for an unrelated candidate
to be blocked", reaching the repository through a gate `AC-4` never modelled.

**Three outcomes, and the exit status alone must distinguish them**, because the
publisher sees nothing else. A summary the publisher cannot read does not count.

| outcome | exit | blocks a merge |
|---|---|---|
| **Instrument failure** — the sweep could not run, or could not be trusted: bad filterset, selection error, a registry entry that does not resolve against the nextest listing, a listing/count mismatch | **non-zero** | **yes, and that is correct** |
| **Finding** — the sweep completed and one or more ignored rows now pass | **zero**, with notices and a summary | **never** |
| **Nominal** — the sweep completed and every selected row still fails | **zero** | no |

**Why instrument failure must stay blocking, rather than being softened along
with the rest.** Section 0: the failure mode of an instrument is reporting when
it measured nothing. A sweep that cannot fail is that defect. `AC-1` already
forbids asserting the absence of a failure token; making every outcome exit zero
would be the same mistake at the job level. **The non-blocking requirement is
about findings, not about the instrument's own health**, and the frame did not
previously say so.

**One case verify-leader's proposal did not name, ruled here: a registry entry
that resolves to no test is INSTRUMENT FAILURE, not a finding, and not
silently skippable.** The corrective mechanism resolves each `test_path` against
the nextest listing; if that resolution fails, the exemption set no longer
describes reality and a rename could silently carry an exemption onto the wrong
row — or off the intended one. `D1`'s stated principle is **fail toward
sweeping**, and an unresolvable exemption is the one case where quietly dropping
the entry would instead fail toward *exempting*. Make it loud.

**Control.** Force each of the three outcomes and record the observed exit
status for each. A test that only exercises the nominal path does not discharge
this row — the two interesting outcomes are the ones that have never run.

### `AC-5` — bounded wall-clock, cost row exempt from run one

> **MEASURED:** the cost row at `boundary_value_clif.rs:7884` is in the registry
> from `D1`, and the job's observed duration is recorded.
> **CLAIMED:** the sweep is affordable to run standing.
> **THE GAP:** ~142 seconds is not visible in a pass/fail bit, and an
> instrument nobody wants to run is an instrument that gets disabled.

### `AC-6` — no regression

Green in CI. Per `COORDINATION §12` this means CI, **not** a local
`--workspace` run. Locally, per-crate only.

## 4. Banned scope

- **Do not un-ignore any row as part of this node.** The eight owner nodes own
  their repairs. This node builds the instrument that reports on them; it does
  not do their work, and a row that starts passing is routed, not fixed here.
- **Do not put policy exemptions in a default nextest profile or any repo-wide
  filterset.** See `AC-3`. If exempting a row from the sweep can also exempt it
  from the required gate, the carrier is wrong.
- **Do not classify by grepping the reason prose.** `D1` decides this; a
  substring match on wording is the one form explicitly excluded.
- **Do not make the sweep a required check.** `D2` and `AC-4`.
- **Do not re-baseline the expected count to whatever the first run produces.**
  The count is derived from `1a`'s anchored grep at the commit under test, and
  **a delta always has a nameable cause** — `1a` names both of the two that have
  happened so far. If the sweep's selection disagrees with the grep and you
  cannot say which commit moved it and why, that is a finding to report, not a
  number to adjust.

## 5. Hard stop

Stop and report rather than proceeding if:

- nextest cannot express ignored-only selection (`D0`), or the CI job cannot be
  wired non-blocking without touching the aggregator's contract.
- `AC-3` fires — the registry can narrow the main gate. That is a mechanism
  question, not a tuning exercise.
- `D5` finds a row that **passes** while ignored. Report it to the Steward with
  the row and its owner node; do not un-ignore it here.

  **This stop has fired once and its disposition is recorded in `1c.1` — that
  row is settled, do not re-stop on it.** It also taught the distinction the
  stop's original wording missed: a passing ignored row is **either**
  over-annotation (a live repair with its cover switched off, `1g`'s shape)
  **or** vacuity (a body that asserts nothing and passes for free). They look
  identical from the exit status and have opposite dispositions — the first
  wants un-ignoring by its owner, the second must stay ignored until someone
  writes the assertion. **Check the body before classifying.**
- The sweep's selected-row count disagrees with `1a`'s anchored grep at the
  commit under test **and you cannot attribute the difference to a specific
  commit.** An accounted delta is normal and is not a hard stop; an unaccounted
  one means the selection logic and the grep disagree about what the population
  is, which is the whole mechanism.

Per the one-hour turn target, a genuine hard stop is a good outcome.

## 6. Contention

Touches `.github/workflows/ci.yml` and adds a registry file.

**Re-grounded at release, `20c7a291`, and this is the basis on which the node is
being released while Runtime works.** Measured, not assumed:

- The two live Runtime candidates, `de1434cd` and `bb9fad0a`, touch **zero**
  `ci.yml` paths. Their scope is `crates/ken-cli/tests/`,
  `crates/ken-runtime/src/cranelift_backend.rs`, and
  `.../lowering/core.rs`.
- This node adds **no `crates/` change at all.** It reads other nodes' ignored
  rows; it does not modify them, and `4` forbids un-ignoring any of them.

⇒ **The intersection with in-flight Runtime work is empty**, so the
single-threaded posture is satisfied rather than waived. That posture exists to
prevent contention, and the operator's standing doc-track exception is granted
on exactly this basis — contention-free-ness, not priority.

**This is a Steward sequencing call and it is re-derivable, so re-derive it
rather than trusting this paragraph.** A disjointness claim of the Steward's has
died twice on the Runtime chain, both times because the *other* lane's repair
moved into a file the claim had checked earlier. Before starting, re-run the
check against the then-current tree:

```sh
git diff --name-only $(git merge-base origin/main <candidate>) <candidate> | grep -c 'ci\.yml'
```

If any in-flight candidate touches `ci.yml`, **stop and return to the Steward**;
do not sequence around it yourself.
