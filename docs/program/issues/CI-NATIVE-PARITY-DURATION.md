---
id: CI-NATIVE-PARITY-DURATION
title: "Rework the CI test suite to run faster, under the operator's 20-minute ceiling and toward the 10-minute target, by removing three measured serial floors: split checked_ih_generated_entry_confluence_and_route_mutations_reject (39 sequential subprocess mutations, 1299.983s in ONE scheduling unit) and its five siblings into per-case tests so nextest can schedule them; then partition native-rt-parity across runners; then rebalance the workspace shards. Splitting alone is necessary but NOT sufficient -- it takes the job from 25m to about 18m, because 4171.65 CPU-seconds on a 4-vCPU runner floors at 17.4m -- and partitioning is INERT until the split lands, because --partition cannot subdivide a single 1300s test. Separately, the ignored-row sweep EXECUTES 33 ignored rows for 10m and treats their failures as non-blocking findings. D1-D3 LANDED at c555f843a. D4 was RE-SCOPED 2026-08-28 after the verify ring measured its original short-circuit population EMPTY: no ignored row names an unbuilt capability, so the short-circuit half is INAPPLICABLE, and D4 instead builds a STALE-READMISSION DETECTOR -- the sweep reports a FINDING when an ignored row names a condition whose tracker node reads merged while the row stays ignored, which is the operator's own re-enablement concern already realized twice in this file. CHANNEL SEMANTICS RULED 2026-08-29 after the Architect rejected 7c607486: a stale row is an EXIT-ZERO routed finding naming every stale row, never a red gate (D4a measured 16 against the tree, so an enforced gate would red main on arrival); the instrument's OWN failures -- malformed, missing, invalid, census/registry mismatch -- exit nonzero and block. The flip to enforcement at zero population is a separate later increment needing its own release. The contradiction was the Steward's: that ruling was made in a convo thread and never landed here, so four operative passages still said FAIL. D4c is RECUT 2026-08-29 (Architect evt_2933nxk4x45d4) and its original scope -- repair the source-text ignore-reason parser -- is RETIRED as the wrong layer: source-derived discovery and association are REPLACED by the compiler-generated test descriptors, which supply BOTH the test identity and the decoded ignore_message, after five gate cycles proved that lexical masking plus a raw-line substring search cannot establish outer-attribute ownership. Compiler-owned inventory plus source-parsed reason does NOT close the class -- the attribute-ownership join is the defect, so the compiler must supply both halves. D4c is PARKED at rejected 1f26af0d8 and holds no seat; D5 splits the 574s test, which is exactly shard 1 entire excess over the mean. Behaviour-preserving throughout: the same 90 mutation cases with the same assertions and outcomes."
status: active
owner: verify
size: M
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Operator request 2026-08-28: 'diagnose the native-slow test and brief me on what we can do about it? ideally CI should be less than 10 minutes, but less than 20 is acceptable.' Then the operator's directing message the same day: 'split up checked_ih_generated_entry_confluence_and_route_mutations_reject so that it can be parallelized and/or sharded into separate jobs. Are ignored tests run, but not considered failures if they do fail? If so, add a short circuit quick end to the tests with a comment so that they are properly re-enabled when they are rearmed. let lane 2 finish its current wp, then bring up verify on lane 2 to rework CI tests to make them run faster.' That last clause is a ROSTER RULING and is recorded in steward/lanes.md. Steward diagnosis measured against completed main run 33192361977 at bb33dfb71e302a68377ffde8038f7dc8bd2c82ac -- the first fully completed main run since 31258f403. Steward-filed per COORDINATION section 2."
---

> # OPERATIVE AND TOPMOST — D4's DISCOVERY MECHANISM IS REPLACED. THE COMPILER
> # OWNS BOTH TEST IDENTITY AND IGNORE REASON. DO NOT REPAIR THE PYTHON LEXER.
>
> **Architect ruling `evt_2933nxk4x45d4`, 2026-08-29, on the Steward's mechanism
> disposition request `evt_7xa7h67epj47m`. It governs every D4 passage below.**
> Where anything further down describes source-text discovery, association, or
> reason decoding as the mechanism, THIS BANNER WINS and that passage states a
> retired design. Sections `D4a`, `D4c`, the 43-vs-39 residual ruling, and
> `AC-COMPILER-ORACLE` have each been rewritten in place; nothing was left to be
> read as still-live by anyone who reaches it first.
>
> **Five gate cycles were spent making a Python Rust lexer agree with rustc** —
> on comments, then `\bfn\b`, then `stringify!` and `macro_rules!` token trees.
> Each cycle closed the exhibited counterexample and the next found another
> spelling of one class: **lexical masking plus a raw-line `#[test]` substring
> search cannot establish outer-attribute ownership.** Robustly, that path ends
> at a full Rust parser written in Python.
>
> **The correction the Architect made to the Steward's diagnosis, and it is the
> load-bearing half:** compiler-owned INVENTORY plus source-parsed REASON does
> **not** close the class. Attaching a source reason to a compiler test id still
> requires deciding which outer attributes belong to which item — the exact join
> that failed on all four shapes. **The compiler must supply BOTH the identity
> and the decoded reason.** A half-migration keeps the defect.
>
> The current toolchain already exposes both, Architect-probed on `rustc
> 1.97.1` (descriptor log SHA-256
> `327e7330d5824817dc8a3eff00d1159f6051c8d464a71d2ea565667618985252`):
>
> ```sh
> RUSTC_BOOTSTRAP=1 "$test_binary" --list --format json -Z unstable-options
> ```
>
> emitting per descriptor: `name`, `ignore`, `ignore_message`, `source_path`,
> and `start_line`/`start_col`/`end_line`/`end_col`.
>
> **This closes the class structurally rather than case by case.** Comments,
> raw/cooked escapes, lifetimes, character literals, item boundaries, attribute
> order, macro expansion, `cfg`, and attribute-shaped tokens are all resolved by
> rustc *before* the descriptor exists. An ignored helper is not an ignored
> TEST and is correctly absent, rather than a census failure. A `cfg`-disabled
> test is absent from this profile; if another profile matters, **compile and
> list that profile** rather than pretending a static scan models `cfg`.
>
> **`AC-COMPILER-ORACLE` IS NOT "TRIVIALLY SATISFIED" NOW — that phrasing was the
> Steward's and the Architect struck it.** Shared derivation is not
> corroboration. The descriptor becomes the AUTHORITY; independence comes from
> the fixed known-answer fixture, exact nextest-versus-descriptor set equality,
> fixed registry authority, and compile-preserving mutations of the adapter's
> inputs.
>
> **D4c is PARKED at rejected `1f26af0d8e0c6df596ffc99b3f4e2162fab2fe27` and D5
> holds the implementer.** This recut is what D4c resumes onto; it is not a
> release. **The branch's own `docs/program/wp/CI-NATIVE-PARITY-DURATION-D4c.md`
> (blob `139ef45104e4a03991d147ea94aaf0afa5907102`) still carries the retired
> mechanism and MUST be rewritten to this ruling before any respin** — it exists
> only on the branch, so this node cannot correct it for you.

> # OPERATIVE — D1-D3 LANDED; D4 IS RE-SCOPED TO A STALE-READMISSION DETECTOR
>
> **Steward ruling, 2026-08-28, on the verify ring's D4 hard stop
> `evt_7qmmeqkd21c6y`. This banner governs the remaining work. Every passage
> below describing D4 as a short-circuit states the ORIGINAL design, not the
> authorized one.**
>
> **D1+D2+D3 MERGED at `c555f843a686ae30ddadc160e52bbed8de381d87`** — both
> candidate paths blob-verified, Adversary M8 hunt CLEAN (`evt_7q9kmjszk8naz`,
> the mutation-mode census byte-identical at 96 tokens). Measured wall clock
> `28m55s` -> `18m51s` on run `33215344963`.
>
> **`AC-DURATION-MEASURED` IS NOT DISCHARGED BY THAT PAIR.** The two runs sat on
> different runner hardware, so `28m55s` -> `18m51s` is an INFERENCE about the
> increment, not the comparable-hardware measurement the criterion asks for.
> Report it from comparable hardware, and state the caveat rather than dropping
> it.
>
> ## D4's ORIGINAL POPULATION IS EMPTY, AND THE RING PROVED IT
>
> The verify ring re-grounded the population, found it empty, reset
> `wp/CI-NATIVE-PARITY-DURATION-D4` byte-clean, and cut no candidate. It
> declined to invent a capability or preserve a stale one. That was correct and
> is the reason this node advances instead of shipping a control that cannot
> fail.
>
> Six `#[ignore]` rows exist in `crates/ken-cli/tests/rt_parity_native.rs`. Four
> name only "post-M6 runtime parity debt" — an unfalsifiable condition D4
> forbids short-circuiting. The two that name a real condition name one that is
> already **built**. **So the short-circuit half of D4 is INAPPLICABLE**, and
> that is a measured outcome recorded with its evidence, not a failed
> deliverable.
>
> ## THE EMPTY POPULATION IS THE OPERATOR'S STATED FAILURE, ALREADY REALIZED TWICE
>
> The operator asked that rows be *"properly re-enabled when they are rearmed"*.
> Two rows name conditions that ARE rearmed and were never re-enabled or
> re-pointed: `:675` names `RT-SITEOP-CARRIED-WITNESS`, `:2039` names
> `RT-CLOSURE-BOUNDARY-LANE`, and every tracker node either row cites reads
> `status: merged` (`RT-SITEOP-CARRIED-WITNESS`,
> `RT-CARRIED-IH-DISPATCH-SITEOP`, `RT-CLOSURE-BOUNDARY-LANE`,
> `RT-CLOSURE-BOUNDARY-RESIDUAL` — verified by the Steward on `origin/main`,
> independently of the ring's census).
>
> **A row whose blocker landed and whose ignore stayed is exactly the drift the
> short-circuit existed to prevent. It is not hypothetical in this file; it is
> the file's measured state, twice.**
>
> ⇒ **D4 becomes a STALE-READMISSION DETECTOR.** The sweep REPORTS A FINDING when
> an ignored row names a condition whose tracker node reads `merged` while the
> row remains ignored. (**"FAILS" here was superseded** by the channel-semantics
> ruling in the OPERATIVE banner below: a stale row is an exit-zero finding; only
> the instrument's own failures are enforced.) The population is non-empty, which
> is what makes the mutation
> criterion truthfully exercisable — the ring's blocking objection to the
> original framing, and it was right.
>
> ## CORRECTION TO MY OWN RULING: THE POPULATION IS NOT "EXACTLY TWO"
>
> **I ruled the population was exactly the two `rt_parity_native.rs` rows. Then I
> opened `.github/ignored-test-exemptions.toml` and found a THIRD instance
> sitting in the registry itself:** the entry for
> `ken-elaborator::compiler_driver::tests::gate_4a_preparation_and_full_build_are_one_transaction`
> carries `readmission = "RT-CLOSURE-BOUNDARY-LANE"`, and that node is `merged`.
>
> **So the drift is not confined to the file the ring was looking at, and my
> count came from the same narrow view.** Two consequences, and the second is the
> one that shapes the deliverable:
>
> 1. **The condition is recorded in TWO places** — the registry's `readmission`
>    field and the row's own comment / `#[ignore = "..."]` reason. A detector
>    reading one source under-covers. Enumerating the true population is D4's
>    FIRST measurement, not a number this frame hands down.
> 2. **The `readmission` field is POLYMORPHIC and most of it is not a node id.**
>    Of the six registry entries, one names a tracker node, one names a relation
>    symbol (`TermJReduction`), and four are free prose ("not applicable: …",
>    "after L-classes expose Int.toInt64, …"). **A rule that fails on any
>    unresolvable string reds the whole existing registry on day one; a rule that
>    silently passes them can be disabled by a typo.** That fork is real, it is
>    not pre-decided here, and it is `AC-STALE-READMISSION`'s first obligation.

> # OPERATIVE — CHANNEL SEMANTICS RULED. A STALE ROW IS A FINDING, NOT A RED
> # GATE. THE INSTRUMENT'S OWN FAILURES STAY ENFORCED.
>
> **Steward ruling, 2026-08-29, after the Architect rejected `7c607486` at
> `evt_bfg568nhmpth` and verify-leader correctly held rather than choosing the
> semantics itself (`evt_1cd8x1dmhrv8q`).**
>
> **THE CONTRADICTION WAS MINE AND IT WAS IN THE FRAME, NOT IN THE RING'S
> READING.** I ruled the non-blocking channel IN A CONVO THREAD and never landed
> it here, so this file went on saying "the sweep FAILS" in four operative
> places — the `title:`, the D4 re-scope banner, the D4 deliverable, and
> `AC-STALE-READMISSION`'s positive arm — while the ring built the channel I had
> described in the thread. **A thread ruling cannot override an operative frame,
> and the ring was right to refuse to choose.** This is the same defect already
> recorded on the runtime node: a frame passage that survives the ruling
> invalidating it does not read as stale, it reads as authoritative. Landing it
> here is the correction; the thread post is not.
>
> **THE RULING — two channels, and the split is the existing one at
> `.github/workflows/ci.yml:130-132`, not a new mechanism:**
>
> 1. **A STALE ROW IS A FINDING.** The sweep exits ZERO, routes the finding, and
>    NAMES every stale row with the node status it read for each. It does not
>    red the gate. A count without the row names is not actionable and does not
>    satisfy this.
> 2. **AN INSTRUMENT FAILURE IS ENFORCED.** Malformed, missing, invalid,
>    unresolvable-where-resolution-was-claimed, and census/registry mismatch exit
>    NONZERO and block. **The detector must not be able to fail silently** — a
>    broken instrument that passes is worse than no instrument, because it reads
>    as coverage.
>
> **WHY FINDINGS AND NOT ENFORCEMENT, stated so nobody has to re-derive it:**
> D4a measured **16 stale rows** against the tree as it stands. Landing an
> enforced gate would red `main` on its first run over a population that
> predates the instrument, halting all three lanes to report a backlog the
> instrument was built to make visible. **That is the gate being wrong on
> arrival, not uptime being protected.** The correct shape is to measure first
> and enforce once the population is drained.
>
> **THE RATCHET IS PART OF THIS RULING, NOT A LATER NICETY.** "Non-blocking" is
> a starting position with a stated end, or the detector never bites. When the
> stale population reaches ZERO, the finding channel flips to enforced. That
> flip is its own increment and needs its own release from me — it is NOT
> authorized here, and it must not be pre-wired behind a flag or a threshold
> that trips on its own.
>
> **`AC-SHORTCIRCUIT-ENFORCED` is unaffected** — it is retired as INAPPLICABLE
> with evidence, and this ruling does not revive it.

**This node is lane 2, owned by verify.** The operator ruled: *"let lane 2
finish its current wp, then bring up verify on lane 2 to rework CI tests to make
them run faster."*

**THE RELEASE CONDITION IS MET (2026-08-28).**
`V3-FO-SORTED-EIGENPARAMETER-DERIVATION` LANDED at `114a6c105` — all eleven
candidate paths blob-verified against `origin/main` by the Steward, CI run
`33207199378` green, tracker closeout at `5fe12514b`. Lane 2's ring therefore
changes from language to **verify**, and this node is flipped `active` and
RELEASED.

**This is a ring change WITHIN lane 2, not a fourth lane.** The three-lane
roster — runtime / language(->verify) / foundation — is unchanged, and the z3
integration campaign queues behind this node.

Landing alone still releases nothing; the Steward's explicit release does, and
it accompanies this flip.

> **CONTENTION RE-MEASURED AT RELEASE, not trusted from this frame's earlier
> reading.** This node edits `crates/ken-cli/tests/rt_parity_native.rs`. Lane
> 1's D3 node (`RT-RESULT-CONTINUATION-BINDING-PROVENANCE`) adds observations to
> the SAME file — but D3 is FROZEN at HS13, and its only authorized next turn is
> a D0 measurement that lands NO production. **There is no live writer on that
> file today.** If D0 returns YES and a later D3 release issues while this node
> is still in flight, re-measure and hard-stop rather than merging across the
> two.

## Model-capability estimate (steward.md §4h): T2 — mechanical

The diagnosis is settled below and the arithmetic is measured, not argued. What
remains is a behaviour-preserving restructure of a test file, two workflow matrix
edits, and — after the D4 re-scope — a status-keyed stale-readmission check in
the existing sweep script. The review turns on differential faithfulness — same
cases, same assertions, same outcomes — not on an argument.

## Fixed inputs — measured at `bb33dfb71`, run `33192361977`

Do not re-derive these; cite them. Every number is from the completed main run's
own logs.

Run wall-clock `16:56:49Z` to `17:25:44Z` = **28m55s**. Critical path:

| job | Build | Test |
|---|---|---|
| `native-slow (rt_parity_native)` | 1m | **24m** |
| `test shard 1/4` | 1m | **18m** |
| `test shard 4/4` | — | 12m |
| `test shard 2/4` | — | 12m |
| `test shard 3/4` | — | 11m |
| `ignored-row sweep` (non-blocking findings) | — | 10m |
| everything else | — | 6m or less |

**The Build step is 1m. The whole problem is the Test step.** A faster runner or
a better dependency cache buys nothing here.

`rt_parity_native` nextest summary: `Summary [1435.857s] 15 tests run: 15 passed
(12 slow), 6 skipped`. Per-test, the top of the distribution:

| seconds | test |
|---|---|
| **1299.983** | `checked_ih_generated_entry_confluence_and_route_mutations_reject` |
| 656.759 | `checked_ih_continuation_inheritance_mutations_bite_their_own_arms` |
| 623.268 | `checked_ih_generated_entry_capsule_mutations_reject` |
| 339.805 | `checked_ih_generated_entry_admission_population_mutations_reject` |
| 270.219 | `d1_route_control_full_program_mutations_are_fail_closed` |
| 258.421 | `checked_ih_generated_entry_per_arrival_operation_mutations_break_equality` |
| 136.884 and below | the remaining nine tests |

Total CPU across all 15: **4171.65s**. Wall: **1435.86s**. Effective
parallelism **2.9x** on a 4-vCPU `ubuntu-latest` runner.

## The diagnosis: three independent floors

**Floor 1 — the longest single test, which is the one the operator named.**
`nextest` schedules at `#[test]` granularity. The six slowest tests are each a
`for` loop over a case array that spawns one isolated child process per case via
`std::env::current_exe()`:

| line | test | cases |
|---|---|---|
| 1370 | `..._per_arrival_operation_mutations_break_equality` | 6 |
| 1445 | `..._admission_population_mutations_reject` | 8 |
| **1588** | **`checked_ih_generated_entry_confluence_and_route_mutations_reject`** | **39** |
| 1748 | `..._capsule_mutations_reject` | 15 |
| 1990 | `..._continuation_inheritance_mutations_bite_their_own_arms` | 15 |
| 2151 | `d1_route_control_full_program_mutations_are_fail_closed` | 7 |

**90 subprocess children, and each loop is ONE scheduling unit on ONE core.**
The named 39-case loop costs 1299.983s — 33.3s per child — and no other test can
use the cores it leaves idle. **Wall time cannot go below 1300s while that test
exists**, whatever else is done.

**Floor 2 — total CPU against runner width, and this is what makes splitting
insufficient on its own.** 4171.65s of work on 4 vCPUs floors at **1042.9s =
17.4m**, plus the 1m Build. So:

- Split alone: 25m job → **about 18m**. Under the 20m ceiling, not near 10m.
- **`--partition` is what reaches the target — and it is INERT before the
  split.** Partitioning 15 tests cannot subdivide a 1300s test; that test lands
  whole on one shard and re-floors the job at ~22m. **After** the split the
  population is ~99 tests of roughly even cost, and 3 partitions give
  ~1043/3 ≈ 350s ≈ 6m + 1m Build = **about 7m**.

**⇒ D1 is the enabler and D2 is the payoff. Neither delivers the target alone,
and D2 measured before D1 reads as no improvement.** Do not conclude from a flat
D2 measurement that partitioning does not work.

> ## PACKAGING: D1 + D2 (+ D3 if clean) LAND AS **ONE** CANDIDATE
>
> **Steward sequencing ruling, 2026-08-28, after the operator made this node the
> fleet's first priority.** The split-before-partition constraint above is a
> **MEASUREMENT** ordering and it is unchanged: perform the split, measure it,
> then apply the partition and measure again, and report BOTH numbers.
>
> **It is not a packaging instruction, and reading it as one is expensive.**
> Every publish cycle costs a full CI run — about 29 minutes today — so shipping
> D1 and D2 as separate candidates spends roughly an hour of pure latency to
> honour an ordering that is already satisfied inside a single turn. The
> measurement discipline lives in the turn; the packaging does not have to.
>
> **D3 belongs in the same candidate when it is clean, and it is not optional
> polish.** `native-slow` is 24m and `test shard 1/4` is 18m, so **D1 alone
> lands the run at roughly 19m — inside the operator's 20m ceiling by about one
> minute, with no margin**, because shard 1/4 simply becomes the new critical
> path. D1+D2+D3 is what reaches the 10m target.
>
> **This does NOT relax the behaviour-preserving requirement, which is the whole
> review.** Same 90 mutation cases, same assertions, same outcomes. A larger
> candidate makes that review bigger, not weaker. **A split that turns out not
> to be behaviour-preserving is a HARD STOP to the Steward, never a scope
> widening**, and a candidate that cannot show differential faithfulness does
> not land regardless of what it does to the clock.

**Floor 3 — shard imbalance, independent of the above.** `test shard 1/4` is 18m
against 11m for shard 3/4. `--partition count:N/4` assigns by test, not by
duration, so the split is uneven by construction. The aggregate `build + test`
gate reads `needs.test-shard.result` for the whole matrix, so the shard count can
change without touching branch protection — the stated design intent of the
comment at `.github/workflows/ci.yml:383`.

## The ignored-row question, answered

**The operator asked whether ignored tests run but are not treated as failures.
They do, and they are not.** Measured, not inferred:

- `.github/workflows/ci.yml:157` runs `cargo nextest run --workspace --locked
  --run-ignored=only --no-fail-fast`, so ignored rows **are executed**.
- It is wrapped in `set +e` and its exit status is passed to
  `scripts/ci-ignored-sweep.py report`, which routes findings and exits zero.
  The job name says so: `ignored-row sweep (findings non-blocking)`.
- On run `33192361977` that sweep ran **33 rows**, most of which FAILED, at
  roughly 40-75s each. That is the 10m job.

So the operator's premise holds — and this half of the section is measurement
that survives the re-scope unchanged.

> **THE REMEDY BELOW IS THE ORIGINAL D4 DESIGN AND IS NOT WHAT IS AUTHORIZED.**
> Its population was measured EMPTY; see the operative banner at the top. Read
> the next two paragraphs as the reasoning that produced the registry mechanism,
> which the re-scoped D4 still builds ON — not as a live instruction to
> short-circuit anything.

**One consequence had to be stated before it was built:** a bare short-circuit
makes the sweep report a row as passing, and the sweep's whole purpose is to
notice when an ignored row *starts* passing so it can be re-armed.
Short-circuiting rows without telling the sweep would convert a live instrument
into one that always reads green — the failure mode this program keeps paying
for.

**So the short-circuit was to be registered, not silent.** The mechanism already
exists: `.github/ignored-test-exemptions.toml` carries a `class` and a
`readmission` field per row, and `ci-ignored-sweep.py verify-row-claims` already
enforces the registry against the tree. That `readmission` string is exactly the
"properly re-enabled when they are rearmed" hook the operator asked for.

**THAT MECHANISM IS WHERE THE RE-SCOPED D4 BUILDS.** The stale-readmission
detector is a check inside the SAME `verify-row-claims` path, reading the same
`readmission` field, and failing the sweep when the condition that field names
resolves to a tracker node whose `status:` is `merged`. **The operator's hook is
delivered — it just fires on the drift that actually exists in this tree rather
than on a population that does not.**

## Deliverables

**D1 — split the mutation loops into per-case tests.**
`checked_ih_generated_entry_confluence_and_route_mutations_reject` is REQUIRED
and is the operator's named target; its five siblings are the same defect and
are in scope. Each of the 90 cases becomes its own `#[test]`, preserving the
existing parent/child subprocess isolation unchanged. The child-side dispatch
(`assert_*_child`, the `KEN_RT_*_CHILD` environment variables, the `--exact`
re-invocation) is NOT redesigned; only the parent side stops looping.

**D2 — partition `native-rt-parity` across runners.** Add a shard matrix to the
job at `.github/workflows/ci.yml:325` on the same `--partition count:N/M` form
the workspace lane already uses, with `fail-fast: false` for the reason the
existing matrix comment gives. Pick N from the post-D1 measurement, not in
advance.

**D3 — rebalance the workspace shard count.** Raise the `shard:` matrix at
`.github/workflows/ci.yml:46` so no shard's Test step exceeds the target. Keep
the `Doctests` step conditional on a single shard.

**D4 — stale-readmission detector for ignored rows whose blocker has landed.**
RE-SCOPED 2026-08-28; see the operative banner at the top of this node.

The sweep gains a check that REPORTS A FINDING when an ignored row names a
condition whose tracker node reads `status: merged` while the row remains
ignored. **Exit zero, routed, and every stale row named** — see the
channel-semantics ruling in the OPERATIVE banner at the top of this node. The
instrument's OWN failures (malformed, missing, invalid, census/registry
mismatch) exit nonzero and block.

**D4a — enumerate the population first, across BOTH authorities.** A readmission
condition is recorded in two places and they do not agree on shape: the
`readmission` field in `.github/ignored-test-exemptions.toml`, and the row's own
ignore reason. **The row's reason is the COMPILER-DECODED `ignore_message` from
the test descriptor, NOT the source `#[ignore = "..."]` literal and NOT an
adjacent comment** — see the topmost banner. An adjacent comment is not an
authority at all under the ruled mechanism; where D4a's original text treated
comment prose as a recorded condition, that is retired. Deliver a table of every
ignored row, both recorded conditions, whether each resolves to a
`docs/program/issues/<ID>.md` file, and that node's `status:` when it does.
**Three instances are known to the Steward and the enumeration must find at
least these three, or it is under-covering:**
`crates/ken-cli/tests/rt_parity_native.rs:675` (`RT-SITEOP-CARRIED-WITNESS`),
`:2039` (`RT-CLOSURE-BOUNDARY-LANE`), and the registry entry for
`ken-elaborator::compiler_driver::tests::gate_4a_preparation_and_full_build_are_one_transaction`
(`readmission = "RT-CLOSURE-BOUNDARY-LANE"`). **This is a floor, not the
roster** — I found the third by opening the registry after ruling there were
two, so treat my list as a control on your census rather than as its answer.

**D4b — build the check, keyed on TRACKER NODE STATUS, never on matching label
text.** The verify ring raised this and it is adopted: a string match over
comment prose fires on any sentence mentioning a node id, including one that
denies the row is blocked on it, and it stops working the moment someone rewords
a comment. Resolve the cited id to `docs/program/issues/<ID>.md` and read its
`status:` field.

**D4c — REPLACE SOURCE-DERIVED DISCOVERY AND ASSOCIATION WITH THE
COMPILER-GENERATED TEST DESCRIPTORS. RECUT 2026-08-29 on Architect ruling
`evt_2933nxk4x45d4`; PARKED at rejected `1f26af0d8`, not released.**

The original D4c scope — repair the Rust literal parser, add declared-condition
extraction, add three literal-form controls — **is RETIRED IN FULL.** It was a
correct description of the symptoms and the wrong layer. Do not implement it,
and do not treat the retired bullets as a floor to also satisfy.

**Required mechanism, all six steps:**

1. **Keep the existing authoritative nextest listing** as the suite enumerator:

   ```sh
   cargo nextest list --workspace --locked --run-ignored=only \
     --message-format json > ignored-row-all.json
   ```

   Its suite records already carry `package-name`, `binary-id`, `binary-name`,
   `binary-path`, `cwd`, and full test names.

2. **For each listed suite, invoke that exact `binary-path` in its `cwd`** with
   the libtest JSON discovery command in the topmost banner. Parse only complete
   `event=discovered` records. **Identity is `(package-name, binary-name,
   descriptor.name)`; the reason is `descriptor.ignore_message` on
   `ignore=true`.**

3. **Reconcile EXACT SETS, per suite and package — never counts:**

   ```text
   nextest ignored identities == compiler-descriptor ignored identities
   ```

   Missing suite, duplicate identity, malformed JSON, missing required field,
   empty ignore reason, or any set difference is an INSTRUMENT FAILURE and exits
   nonzero. **Counts may be printed only as summaries of already-equal sets** —
   a count is a summary here, never a check.

4. **Resolve every exemption registry `test_path` to exactly one compiler
   identity**, as the current nextest-side resolver already does. Apply
   `readmission_kind` unchanged: `tracker-node` requires the compiled
   `ignore_message` to BEGIN with the exact `TRACKER-ID:` declaration and agree
   with the registry row; `relation-symbol` requires the compiled message to
   contain the exact bounded symbol; `free-prose` invents no tracker resolution.
   Tracker status lookup and the exit-zero named stale-row findings are
   UNCHANGED. Invalid, missing, or unresolvable claimed authority stays blocking.

5. **Source paths and spans are DIAGNOSTICS ONLY.** Source text is no longer an
   inventory, an association oracle, or a reason decoder.

6. **DELETE the source mechanism rather than leaving it as a second authority:**
   `rust_code_mask`, Rust literal decoding, `ignore_attributes`,
   `has_adjacent_test_attribute`, source attribute counts, and every
   count-equality rule derived from them. **Two authorities recreate the exact
   disagreement D4c exists to retire** — leaving the old path in place as a
   cross-check reintroduces the defect under a new name.

**The unstable-interface boundary, and its three hard limits.** `--format json
-Z unstable-options` is an unstable libtest observation surface, acceptable here
ONLY under all three: `RUSTC_BOOTSTRAP=1` is set on the already-built test-binary
LISTING SUBPROCESS ONLY — never on cargo/rustc compilation and never exported to
the job; **no tests execute in that subprocess**; and schema drift or removal
**fails closed** as an instrument error. This adds no Ken product or TCB
assumption and is strictly smaller than maintaining a second Rust parser in
Python. **If that narrow interface becomes unavailable, STOP and re-route the
mechanism to the Steward — do not fall back to keyword enumeration or another
masked-line parser.**

**The 45-vs-39 reconciliation the Steward owed here is DISCHARGED BY THE RECUT,
not by an answer.** It was arithmetic over source `#[ignore` grep hits against
source-parsed rows, and neither quantity exists under the ruled mechanism. Step
3's set equality is the invariant that replaces it, and it is stronger: it
compares identities rather than reconciling counts.

> ## THE 43-vs-39 RESIDUAL RULING IS RETIRED (superseded 2026-08-29)
>
> **Both quantities were artifacts of the source-parsing mechanism and neither
> exists under the ruled one.** "43 function-associated records" was the output
> of the association step that the Architect has now removed, and "39 syntactic
> attributes" was the source attribute count that D4c step 6 DELETES. A
> reconciliation between two retired numbers is not a deliverable, and demanding
> it would send the ring back into the layer the ruling vacated.
>
> **The general requirement it was protecting SURVIVES and is now stronger.** It
> was: *the instrument must not be able to be silently inconsistent with itself.*
> That is realized by **D4c step 3's exact set equality** between the nextest
> ignored identities and the compiler-descriptor ignored identities, enforced
> nonzero and blocking. Set equality over identities is a strictly better form of
> the same invariant than reconciling two counts, because it names WHICH row
> disagrees rather than only that a total differs.
>
> **One measurement lesson from it is worth carrying** and does not depend on the
> retired mechanism: `cfg` profile is part of a population's identity. Under the
> new mechanism this is handled structurally rather than by attribution — a
> `cfg`-disabled test is simply absent from the descriptor population for that
> profile, and if another profile matters you **compile and list that profile**.

**THE UNRESOLVABLE-CONDITION RULE IS A FORK, AND D4a's TABLE DECIDES IT — do not
pick one here.** Four of the six registry entries are free prose and one names a
relation symbol rather than a node, so **failing on every non-resolving string
reds the existing registry immediately**, while **passing them silently lets a
typo disable the detector**. Bring the measured table and a recommendation to
the Steward; a third option (for example, a per-entry declaration of which kind
of condition the field holds) is admissible if the table supports it. **Choosing
one arm without exhibiting the table is the defect, not choosing the arm I would
not have chosen.**

**THE REMEDY IS RE-POINTING OR RE-ENABLING, CHOSEN BY A HUMAN OR THE OWNING
RING. THE SWEEP NEVER RE-ENABLES A ROW ITSELF.** Automatic readmission is
FORBIDDEN and the reason is concrete: `:675`'s comment documents that the row
refuses NEXT for a different reason (a carried recursive hypothesis is an
eliminated value, not a callable), so un-ignoring it would simply red. **A
landed blocker means the comment is stale, not that the row passes.** The
detector's output is a named, actionable failure; the repair is a human edit.

**The four generic rows stay UNTOUCHED and are out of D4's scope.** "post-M6
runtime parity debt" names no node, so the detector cannot evaluate it and must
not guess. Determining what actually blocks those four is runtime's knowledge,
not verify's, and inventing a plausible node id for them would manufacture
exactly the false pairing this node already rejected once.

**PACKAGING (superseding the earlier "D1 may land alone" split, and stated here
because this is where the packaging instruction actually lives): D1 and D2 are
ONE candidate, and D3 joins them when it is clean.** See the packaging ruling
under "The diagnosis" for the reasoning — each publish costs a full CI run, and
D1 alone leaves only a one-minute margin under the 20m ceiling.

**D2 must still never be MEASURED before D1** — that constraint is about
measurement order inside the turn and is untouched by the packaging change.

**D4 remains independently packageable** and may land in either order relative
to D1-D3; it is the ignored-row sweep work and shares no file with them.

**D5 — SPLIT THE 574-SECOND TEST. This is the next serial floor and it is the
largest single item left.** Framed by the Steward 2026-08-29. Independently
packageable; shares no file with D4.

Fixed inputs, measured at `origin/main`
`ac9b681e1f5a684b40a2da8b9ac0c0d19a13b2fc`:

- `crates/ken-cli/tests/rt_cold_lowering_path_enumeration.rs`, blob
  `13c1a96f269d6345b39270ef5432d8337f27877b`;
- `every_rt_parity_entry_reaches_its_expected_terminal_state` at `:575` — ONE
  `#[test]` looping over `ENTRIES` and calling `ken_cli::build_native_program`
  once per entry, in-process, each in its own `tempfile::tempdir()`;
- `ENTRIES` at `:492` — **11 names**; `EXPECTED` — **11 rows**, one per entry;
- the coverage guard `the_expectation_table_covers_exactly_the_population` at
  `:564`, asserting `EXPECTED == ENTRIES` as sets;
- **574s, 68% of shard 1.** `--partition count:N/M` balances by test COUNT and
  never subdivides a single test, so **sharding cannot touch this** — the same
  reason partitioning was inert before D1.

**The arithmetic that makes this worth doing:** 574s over 11 entries is roughly
52s each. Eleven schedulable units on a 4-vCPU runner is about three waves,
call it ~160s against 574s. That is an estimate from the split alone and the
real number is `AC-D5-DURATION`'s to measure, not this frame's to assert.

**DESIGN JUDGMENT, FRONT-LOADED — three things a naive per-entry split breaks,
and the second is the one that would pass review:**

1. **The unconditional aggregate report at `:595-600`.** It prints the whole
   population pass or fail, and the code says why: *"A refusal set that is only
   visible when the assertion happens to fail is not a report."* Eleven
   independent tests each print one line and the population-level view is gone.
   Preserve it — either a separate aggregate reporting step, or per-entry lines
   plus something that emits the population summary. **Do not drop it and do not
   make it conditional on failure.**
2. **A per-case split introduces a THIRD roster, and that is the failure mode.**
   Today two rosters are cross-checked: `ENTRIES` against `EXPECTED`. After the
   split there is also the set of entries that actually have a generated test.
   **An entry can sit in `ENTRIES` and in `EXPECTED`, satisfy the existing
   coverage guard, and never run** — silently untested, with every check green.
   The existing guard cannot see this because it compares the two rosters that
   still agree.
3. **Prefer plain in-process `#[test]`s per entry.** Each entry already builds
   in its own tempdir with no shared mutable state, so the subprocess isolation
   of `generated_entry_case!` (`rt_parity_native.rs:1362`) is not obviously
   needed here and costs a re-exec per case. **If a measured cross-entry
   interference does require the subprocess pattern, the Adversary's latent on
   it applies**: the plain variant asserts only child `status.success()` with no
   marker, so its non-vacuity depends on the macro living at CRATE ROOT for
   `--exact` to resolve. Name which pattern you used and why.

**Capability tier: T2.** This is a behaviour-preserving mechanical split with a
landed precedent in D1, not a design problem. Size S/M.

## Acceptance criteria

- **`AC-CASE-FAITHFUL`.** Every one of the 90 cases survives with its mode
  string, its expected outcome, and its assertions byte-faithful to the loop body
  it came from. **No case is dropped, merged, renamed in a way that changes what
  it selects, or given a weaker assertion.** A case count of 90 before and after
  is necessary and NOT sufficient — the pairing must be exhibited. An increment
  that speeds CI up and loses a mutation case has made the suite worse, not
  faster.
- **`AC-CHILD-MECHANISM-UNCHANGED`.** The `assert_*_child` functions and the
  `KEN_RT_*_CHILD` environment protocol are preserved. The child half of each
  test is not restructured.
- **`AC-NO-NEW-SKIPS`.** The `6 tests skipped` population in `rt_parity_native`
  is unchanged by D1-D3. A test that stops running also stops failing, so a
  faster job with more skips is a regression measured as an improvement.
- **`AC-SHORTCIRCUIT-ENFORCED` — INAPPLICABLE, and that is a measured result.**

  > **RETIRED 2026-08-28 with evidence, not dropped.** Its population is empty:
  > no `#[ignore]` row in `rt_parity_native.rs` names an unbuilt capability. Four
  > name only unfalsifiable "post-M6 debt"; the other two name conditions that
  > are built. **Retiring it is what the measurement licenses. Satisfying it
  > would have required inventing a capability**, which the verify ring refused
  > and was right to refuse. Recorded here rather than deleted so that a later
  > reader can see the criterion was answered rather than skipped.

- **`AC-STALE-READMISSION` — the detector fires on the real population, and its
  power is proved BY MUTATION on BOTH sides.** For D4:
  - **Positive.** Against the tree as it stands, the sweep REPORTS A FINDING —
    exit zero, routed — and NAMES every stale row with the node status it read
    for each. A detector that reports a count without naming the rows is not
    actionable.

    > **TWO CORRECTIONS TO THIS ARM AS ORIGINALLY WRITTEN, both mine.**
    > **(a) "FAILS" is superseded** by the channel-semantics ruling in the
    > OPERATIVE banner at the top of this node: a stale row is a finding, not a
    > red gate; only the instrument's own failures are enforced.
    > **(b) "both rows" IS NOT THE ROSTER.** It named
    > `rt_parity_native.rs:675` / `RT-SITEOP-CARRIED-WITNESS` and `:2039` /
    > `RT-CLOSURE-BOUNDARY-LANE`, and D4a measured **16**. Those two were a
    > FLOOR I could see from a narrow view, exactly as this node's earlier
    > "exactly two" correction warned. **Satisfy this arm against the measured
    > population, never against the two rows named here** — an enumerated roster
    > in an AC is what lets a detector look complete while under-covering.
  - **Negative, and this is the arm that has to be exhibited.** Mutate the
    STATUS OPERAND, not the row: point a row's comment at a node whose `status:`
    is not `merged`, and the sweep must PASS for that row. **This is the arm the
    original framing could not exercise, which is precisely why it was
    re-scoped** — over an empty population `AC-B` could only have been satisfied
    by a control that cannot fail.
  - **Both sources covered.** The detector must evaluate conditions recorded in
    the registry's `readmission` field AND in the row's own comment. Prove it by
    mutation on EACH source independently — the known registry instance
    (`gate_4a_preparation_and_full_build_are_one_transaction`) and a
    comment-only instance must each be able to fire ALONE. A detector that
    passes because the other source happened to catch the same node is not
    covering both.
  - **Unresolvable condition.** Whichever arm of the D4a fork is ruled, exhibit
    the behaviour by mutation and show it does not silently disable the
    detector. If the ruling is that non-resolving strings pass, then a
    node-id-shaped condition that resolves to NOTHING must still be
    distinguishable from deliberate free prose, or a typo is an off switch.
  - **The four generic rows must NOT fire** — proved, not assumed. They name no
    node, and a detector that reaches for them is guessing.

  **Do not satisfy this AC by matching label text.** A control keyed on prose
  passes for a reason unrelated to the property, and this program has twice paid
  for a control that could not fail.

- **`AC-COMPILER-ORACLE` (D4) — RECUT 2026-08-29. The compiler descriptor is the
  AUTHORITY, not a thing the instrument is compared against.** The sweep no
  longer holds a model of Rust to prove correct; it holds an ADAPTER over
  `(nextest suite listing, libtest JSON descriptors)`. What must be proved is
  that the adapter reads those inputs faithfully and fails closed when they
  disagree.

  > **DO NOT RESTATE THIS AS "TRIVIALLY SATISFIED BECAUSE THE INSTRUMENT AND
  > ORACLE NOW SHARE A SOURCE."** That phrasing was the Steward's and the
  > Architect struck it: **shared derivation is not corroboration.** Independence
  > comes from four places, and every one of them must be exhibited — the fixed
  > known-answer fixture, the exact nextest-versus-descriptor set equality,
  > the independently fixed registry authority, and compile-preserving mutations
  > of the adapter's inputs.

  Required, and each case must travel the **whole path** — suite listing,
  descriptor discovery, set reconciliation, registry resolution, verifier
  enforcement — **never a direct call into an internal helper**:

  - **Retain the exact positives** and require that each COMPILES and that its
    **compiler descriptor** carries the independently fixed full test identity
    and decoded reason, after which fixed registry/tracker authority and the
    full verifier must pass: multiline-nested comment, same-line comment,
    zero-fence raw, compiled `\u{2_d}`, hashed raw, escapes, continuations, and
    either attribute order.
  - **Retain the exact `#[test] fn decoy` negative**, and add the two class
    controls the Architect measured: `const _: &str = stringify!(#[test]);` and
    a one-line `macro_rules!` transcriber containing `#[test]`, each followed by
    an ignored helper. Rustc, nextest, and the descriptor inventory must all be
    EMPTY for the helper, and the verifier must not associate it.
  - **Add a macro-generated POSITIVE ignored test.** This is the case that
    proves the mechanism FOLLOWS compiler expansion rather than merely excluding
    macro-shaped decoys — an exclusion-only control set would pass a mechanism
    that silently drops every macro-generated test.
  - **Keep the wrong-node mutation:** changing independently fixed registry
    authority to `RT-WRONG` must red.
  - **Replace the four custom invalid-Rust-literal parser diagnostics with
    compiler negatives paired to valid controls.** An invalid escape must fail
    the rustc/nextest BUILD before any inventory exists; **the sweep must not
    reimplement rustc merely to issue its own wording.** Malformed descriptor
    JSON and registry/descriptor disagreement still exercise
    `ignored-sweep instrument error:` and exit 2.
  - **Mutation-prove the adapter in BOTH directions:** remove one compiler
    descriptor while holding nextest fixed, and change one ignored flag or
    reason while holding nextest and the registry fixed. Each must red on an
    exact identity/reason diagnostic, not a count mismatch.

  > **A CONTROL THAT CALLS THE CANDIDATE'S OWN PARSER IS A SELF-ORACLE, AND A
  > SELF-ORACLE CANNOT DETECT A DEFECT IN THE THING IT ORACLES.** It agrees with
  > the implementation by construction, so it stays green across exactly the
  > divergences it appears to be testing. This is the shared predicate of every
  > D4c rejection to date: the defects were in the lexer, the adjacency walk and
  > the escape grammar, and the controls invoked `ignore_attributes`,
  > `ignored_test_reasons`, `current_tracker_nodes` and
  > `parse_rust_string_literal` directly. A green focused suite therefore
  > certified neither the token boundary nor the cooked grammar (Architect
  > `evt_50xbb17j0btt6`), and the suite was green on every rejected candidate.
  >
  > **THE FRAMING DEBT IS THE STEWARD'S.** "Use the compiler as the oracle" has
  > lived only in successive rejection prose for four cycles and was never a
  > criterion, so each respin could satisfy the frame and fail the gate. **This
  > is the same shape as lane 1's `AC-EXCLUSIVE`** — a requirement that
  > originates in a gate's respin list and is never folded back lives where
  > nobody reads it, and the ring fails it repeatedly without ever being wrong
  > about the frame.

  > **THE SELF-ORACLE WARNING STILL BINDS, AND THE RECUT DOES NOT DISCHARGE
  > IT.** It now applies to the ADAPTER: a control that calls the adapter's own
  > reconciliation helper instead of running the whole path is the same
  > self-oracle in a new layer. The retired lexer is gone; the way to certify
  > by construction is not.

  **The lexical cases this AC used to enumerate are RETIRED with the mechanism.**
  Apostrophes versus lifetimes, block-comment spacers, backslash-continuation,
  `\'`, `\0`, and the hex/Unicode/raw escape grammar are all resolved by rustc
  before a descriptor exists, so they are no longer the instrument's questions
  to get right. **Do not carry that list forward as a checklist to also satisfy**
  — several of its members are now unreachable by construction, and a control
  that still exercises them is testing a code path D4c step 6 deletes.

  The cases named in the bullets above are the REQUIRED controls, and they are a
  floor rather than a roster: **satisfy the predicate, which is that the adapter
  reads its two inputs faithfully and fails closed on any disagreement.** An
  enumerated list is finishable, and a finished checklist is exactly what let the
  previous controls look complete across five rejected candidates.

- **`AC-DURATION-MEASURED`.** Report the post-increment `native-slow
  (rt_parity_native)` Test-step duration, the `ignored-row sweep` duration, and
  the run wall-clock **from a completed CI run**, against the 24m / 10m / 28m55s
  baselines above. A local timing is not evidence — the target is a property of
  the CI runner. **Report the number you get, including if it misses the
  target**; the arithmetic predicts about 18m after D1 alone, and that prediction
  being met is the deliverable, not a shortfall.

  > **NOT DISCHARGED BY D1-D3's LANDING.** The measured pair — `28m55s` on run
  > `33192361977`, `18m51s` on run `33215344963` — sat on DIFFERENT runner
  > hardware, so it supports an inference about the increment and is not the
  > comparable-hardware measurement this criterion asks for. Report from
  > comparable hardware and state the caveat; a number reported without it
  > overclaims.
- **`AC-D5-SCHEDULABLE` (D5).** After the split, nextest lists the 11 entries as
  SEPARATE test items and schedules them independently. Prove it by listing
  them, not by asserting it — the whole point of the increment is the scheduling
  unit count, and that is directly observable.
- **`AC-D5-INVOCATION-CENSUS` (D5) — the third roster must be checked, and this
  is the criterion the split exists to not fail silently.** A per-case split
  makes the set of entries that HAVE a generated test into a third roster
  alongside `ENTRIES` and `EXPECTED`. **An entry present in both existing
  rosters but missing a test satisfies
  `the_expectation_table_covers_exactly_the_population` and never runs.**
  Deliver a check that FAILS when an entry is added to `ENTRIES` without a
  corresponding test, and exhibit that failure by actually adding one and
  showing the red, then removing it.

  > **PREDICATE FORM, NOT A ROSTER.** State the check as "every member of
  > `ENTRIES` has a generated test", never as a list of the 11 current names. An
  > enumerated roster is satisfied by editing the roster, which is the same
  > unfalsifiability this node already corrected once in
  > `AC-STALE-READMISSION`'s positive arm. **The count 11 in the D5 deliverable
  > is a fixed input measured at a SHA, not the criterion.**
- **`AC-D5-BEHAVIOUR-IDENTICAL` (D5).** Same 11 dispositions with the same
  pass/fail outcomes, and the mismatch text preserved verbatim — including the
  `retired_by` guidance ("its blocker is retired by X; if that landed, move this
  row to `Disposition::Completes`") and the "refuses for a DIFFERENT reason"
  message. That text is the actionable half of a failure and a split that keeps
  the assertion while dropping the guidance has lost the deliverable.
- **`AC-D5-REPORT-PRESERVED` (D5).** The population-level report is still
  emitted UNCONDITIONALLY, pass or fail. Control: force one entry to mismatch
  and show the full population still prints, not only the failing row.
- **`AC-D5-DURATION` (D5).** Report shard 1's duration before and after **from
  completed CI runs on COMPARABLE RUNNER HARDWARE**, and state the hardware.
  **Do not repeat `AC-DURATION-MEASURED`'s defect** — that pair crossed
  different runners, which is why it is still undischarged. If comparable
  hardware is not available, say so and report the number as an inference
  rather than a measurement.
- **`AC-AFFECTED-CLOSURE`.** Cover every target that loads any module whose
  CLOSURE this increment changes, diff-touched or not. This is not a relaxation
  of the targeted-build rule: what changes is which targets count as affected,
  never how many crates build at once. This criterion has now cost three lanes a
  red merge.

## Banned scope

- **Do not delete, `#[ignore]`, or conditionally skip any mutation case** to hit
  a duration number. The cases are the suite's discriminating power.
- **Do not weaken an assertion** because a split test makes it awkward to reach.
- **Do not restructure the child-side dispatch**, and do not replace the
  subprocess isolation with in-process execution. That isolation is why these
  mutations are observable at all.
- **Do not re-enable, un-`#[ignore]`, or edit any mutation row's body under D4.**
  The detector REPORTS; a human or the owning ring repairs. `:675` refuses next
  for an unrelated reason and would red if simply re-enabled.
- **Do not key the detector on comment TEXT.** Resolve the cited node id and read
  its `status:` field. A prose match is not the property.
- **Do not keep the source mechanism as a second authority or a cross-check**
  (D4c step 6). `rust_code_mask`, Rust literal decoding, `ignore_attributes`,
  `has_adjacent_test_attribute` and every count-equality rule derived from them
  are DELETED, not retained beside the descriptor path. Two authorities recreate
  the exact disagreement D4c exists to retire, and a cross-check is the most
  persuasive way to reintroduce it.
- **Do not repair the Python Rust lexer again, and do not add another item
  keyword, mask, or adjacency heuristic.** Five cycles closed five spellings of
  one class. If the descriptor interface is unavailable, **STOP and re-route to
  the Steward** — a fallback to keyword enumeration is banned, not merely
  discouraged.
- **Do not set `RUSTC_BOOTSTRAP=1` on cargo/rustc COMPILATION, and do not export
  it to the job.** It is permitted only on the already-built test-binary listing
  subprocess. **No tests execute in that subprocess**, and schema drift or
  removal of the unstable interface must FAIL CLOSED as an instrument error.
- **Do not reimplement rustc to issue the sweep's own wording** for invalid Rust
  literals. An invalid escape fails the build before any inventory exists; that
  is the diagnostic.
- **Do not assign a node id to the four generic "post-M6 runtime parity debt"
  rows.** Their real blockers are runtime's to name. Guessing one manufactures
  the false pairing this node already rejected.
- **Do not short-circuit an ignored row whose readmission condition is unnamed**,
  and do not short-circuit one merely because it currently fails. (Retained from
  the original D4; the short-circuit half is now INAPPLICABLE, but the ban still
  binds anyone who revives it.)
- **Do not touch `concurrency:` / `cancel-in-progress`** at
  `.github/workflows/ci.yml:17-19`. Separate operator decision (below), not in
  scope.
- **Do not change what the sharded lane excludes** at `ci.yml:123`. The three
  native binaries stay in their own jobs.

## Contention — real, and the reason the release condition matters

`crates/ken-cli/tests/rt_parity_native.rs` is the file the FROZEN
`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` chain will eventually add
observations to, and `RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS` (`draft`)
cites `rt_parity_native.rs:1149` directly.

At filing time lane 1 was stopped at HARD STOP 12 with the Architect holding for
a Research advisory, and `runtime-implementer` reported the branch free and the
tree unchanged at `bb33dfb71` with no commit, candidate, or QA — so the file was
uncontended then. That window was not guaranteed to survive to release.

**MEASURED AT RELEASE (2026-08-28): still uncontended, and the stop count has
moved on — lane 1 is now at HARD STOP 13** (`evt_59t7b49m41z8m`), which froze
D3 to a D0-only return-boundary measurement that lands NO production. The
release-condition block at the top of this node carries the live statement; the
paragraph above is filing-time history.

**Re-measure the contention at release time, not from this paragraph.** If the D3
chain has resumed and is editing this file, hard-stop to the Steward rather than
resolving a merge across the two; the sequencing call is the Steward's.

## Reviewers

**Verify QA AND the Architect, both on the exact implementation candidate SHA.**

**The Architect IS a required reviewer.** An earlier version of this section said
otherwise on the grounds that "this is not the M-series" — that is not the gate
predicate, and the claim conflicted with federation law: **a merge Decision
requires the Architect always.** The `docs/program/` editorial exception covers
this Steward-owned FRAME route; it does not cover the implementation candidate,
which touches `crates/` and `.github/workflows/`. Corrected at the Architect's
block `evt_60hd0s0sn3kxw`; the defect was the Steward's.

The review turns on **differential faithfulness** — the same 90 mutation cases,
the same assertions, the same outcomes — and on the workflow changes not
weakening what `main` is gated on.

A finding that a mutation case cannot be split without changing what it observes
is a **HARD STOP to the Steward**, never something to resolve by weakening the
case. Larger packaging does not relax this: one candidate makes the differential
review bigger, not looser.

## Out of scope, and recorded here anyway

**`cancel-in-progress` blinds `main`, and it is an operator decision.**
`.github/workflows/ci.yml:17-19` groups on `github.ref` with
`cancel-in-progress: true`, so every push to `main` kills `main`'s previous
in-flight run. **A cancelled run did not fail and did not pass.** Combined with a
28-minute CI, any landing cadence faster than about 28 minutes leaves `main`
permanently unverified — which is what happened between `31258f403` and
`bb33dfb71`, several of those cancellations caused by the Steward's own doc
routes.

Shortening CI narrows that window, which is why it is recorded alongside this
work. **It does not close it**, and whether `main` should carry
`cancel-in-progress` at all is the operator's call, not this node's.
