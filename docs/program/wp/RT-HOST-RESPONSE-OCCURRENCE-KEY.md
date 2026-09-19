# RT-HOST-RESPONSE-OCCURRENCE-KEY — work package

- **Node:** `[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]`
- **Owner:** runtime
- **Size:** M
- **Tier:** T1 (section 8)
- **Depends on:** `[[RT-DUPLICATED-RESPONSE-BLOCK]]` — must land first
- **Branch:** `wp/RT-HOST-RESPONSE-OCCURRENCE-KEY-occurrence-key`

## 1. Objective

Re-key the static-transition host-response route map on the **occurrence**, so
that N legitimate instantiations each contributing one response handler stop
colliding, while two response-handling sites **within one occurrence** claiming
one operation constructor still refuse.

This is the sole route to clearing all four failing-ignored rows that stop at
`two host response cases claim one operation constructor` — four of the
fourteen failing-ignored rows on `main`, and the largest single-repair block in
that ledger.

## 2. Fixed inputs, measured

All code coordinates read at `origin/main`
`2f046723e71be210df86666042fd061dae86b746`. Row coordinates are the `#[ignore]`
**attribute** line.

**The four rows**, each run alone with `--exact` by the runtime-implementer at
`60df2cfd2`, current refusal text read rather than carried — four for four,
byte-identical:

    px7n:149        two host response cases claim one operation constructor
    px7n:170        same
    rt_escape:653   same
    rt_escape:713   same

**The producer** —
`crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs:1246-1287`:

```rust
fn host_response_routes(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<RuntimeSymbol, HostResponseRoute>, CraneliftBackendError> {
    let mut routes = BTreeMap::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Match { cases, .. } = occurrence.expr else { continue };
        for (alternative, case) in cases.iter().enumerate() {
            ...
            if routes.insert(case.constructor.clone(), route).is_some() {
                return Err(planner_error(
                    "two host response cases claim one operation constructor",  // :1281
                ));
            }
```

**The use site** — same file, `:1289-1310`:

```rust
fn selected_host_response_route(
    plan: &StaticTransitionPlan<'_>,
    operation_origin: StaticOriginId,
    routes: &BTreeMap<RuntimeSymbol, HostResponseRoute>,
) -> Result<Option<(HostResponseRoute, StaticOriginId)>, CraneliftBackendError> {
    let mut selected = None;
    let mut pending = vec![operation_origin];
    while let Some(origin) = pending.pop() {
        if let RuntimeExpr::Construct { constructor, .. } = plan.planned_occurrence_expr(origin)? {
            if let Some(route) = routes.get(constructor).copied() {     // :1298
                if selected.is_some() {
                    return Err(planner_error(
                        "one Vis operation subtree selects more than one host response producer",  // :1301
                    ));
                }
```

**Three facts from that pair, and they set the shape of the whole WP:**

1. **The producer already holds the occurrence.** It iterates
   `plan.source_occurrences` and has `occurrence.static_origin` in hand at the
   insert. Widening the key on this side is available, not invented.
2. **The use site does not.** It receives an `operation_origin` and walks
   children. It has no occurrence and no way to name one without something new.
   **This is where section 3a's "pairing N `Vis` sites to N handlers is planner
   work" actually lives**, and it is localized to one function.
3. **A use-site refusal for ambiguity ALREADY EXISTS at `:1301`.** Any claim
   that a use-site check must be built from nothing is false.

**`source_occurrences`** —
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:570`:

```rust
source_occurrences: Vec<Option<PlannedOccurrence<'src>>>,
```

indexed by `StaticNodeId`, and `record_source_occurrence` refuses a second
occurrence at one index (`occurrences.rs`, `"static origin was given more than
one source occurrence"`). Occurrence identity is therefore already unique and
already a plan-level object.

**The predecessor's 9.5 measurement, which bounds this WP** — the deferral
repair was built, measured, and reverted before commit. It moves the refusal
from construction to use and leaves the key global and the map lossy: **58
route overwrites per run**, each silently discarding one instantiation's
`effect` / `producer_call` / `response` origins. It splits the four rows 2/2
and closes neither half.

## 3. THE DESIGN JUDGMENT, FRONT-LOADED

**The repair is a key widening on the producer and a lookup relation at the use
site. Those are two different difficulties and only the second is open.**

Do not start by writing the key type. Start by establishing whether the use
site can name the occurrence its `Vis` operation subtree belongs to. That is
D0, and the answer decides whether this WP is a bounded edit to two functions
or a plan-structure addition.

### 3a. WHAT MUST NOT BE REBUILT

**Deferral is not re-keying.** Section 9.5 moved *when* the map is checked.
This node changes *what the map can hold*. A repair that leaves the key global
and adds a use-site-only refusal has rebuilt 9.5, and 9.5 is measured not to
close these rows. Refused on arrival.

**Tuple-widening on `operation` is dead.** Both colliding entries already carry
the same operation; across all three measured programs every colliding
constructor agrees on its operation, 29 of 29. Adding `operation` to the key
changes nothing. Refuted at section 3a of the predecessor and not re-openable
here.

### 3b. THE ACCEPTANCE BAR — TWO CLAUSES (clause 3 added and STRUCK 2026-09-19)

    MUST STILL REFUSE   two response-handling sites within ONE occurrence
                        claiming one operation constructor
    MUST NOT REFUSE     N legitimate instantiations each contributing one
                        response handler

**A repair that cannot still refuse the first has relaxed the invariant, not
re-keyed it.** A naive `(occurrence, constructor)` key satisfies the second
half automatically and the first half only if the producer-side insert check is
retained per occurrence. Retaining it is not optional.

> ### THIS COPY IS CORRECT BY COINCIDENCE, AND THAT IS WHY IT CARRIES A MARKER.
>
> **Between 2026-09-18 and 2026-09-19 the bar was THREE clauses** — a
> `MUST NOT MIS-ROUTE` clause added by `evt_4eghtvj2fhpz0` and struck by
> `evt_7sj5xmgcxwk5f`. **This copy was never updated in either direction**, so
> it went stale, then became accurate again without anyone touching it.
>
> **Do not delete this marker as redundant once the copies agree.** Its job is
> not to flag a discrepancy — there is none. Its job is to stop the inference
> that the bar was ALWAYS two clauses. Clause 3 was set deliberately, tested
> against a four-row census, and struck on the measured finding that its
> subject does not denote. **Two agreeing copies with no history erase that
> record**, and the next reader to ask "was the consumer side ever checked?"
> would have to redo the census to find out that it was.
>
> The three-clause copy in `issues/RT-HOST-RESPONSE-OCCURRENCE-KEY.md` was the
> one that read as wrong and was corrected 2026-09-19. **This one read as right
> and would not have been** — the Architect flagged it as the more dangerous of
> the two for exactly that reason, and they are correct.

### 3c. READ SECTION 3a OF THE PREDECESSOR TO THE END

It carries a closure **and** a live assignment. The closure — *"CLOSED BY 9.2
AND 9.3 ... Do not re-measure this from this paragraph"* — attaches to the
**peer question** (are the two entries peers, or is one a prelude occurrence).
It does not close the occurrence-keying assignment three paragraphs above it.
**The ban's reason was narrower than the text it sits in.**

## 4. Deliverables

**D0 — ESTABLISH WHETHER THE USE SITE CAN NAME ITS OCCURRENCE. Run this
first; it decides the shape of D1.**

At `selected_host_response_route`, determine whether the occurrence containing
the intended handler is derivable from `operation_origin` using relations the
plan already carries.

> **TERMINATING OBSERVATION, written before the run, and it can come out either
> way.**
>
>     (a) DERIVABLE -- operation_origin resolves to exactly one occurrence, and
>         that occurrence is the one whose handler the Vis site intends.
>         => the repair is bounded to these two functions. Section 3a's
>            "planner work" was an overstatement; say so explicitly.
>
>     (b) NOT DERIVABLE -- the Vis site's operation subtree and its intended
>         handler are not connected by any relation the plan carries.
>         => the body of this WP is constructing and threading that relation.
>            Name the SMALLEST one that suffices and what must carry it.
>
>     (c) DERIVABLE BUT MANY-TO-MANY -- one operation subtree resolves to more
>         than one candidate occurrence.
>         => that is an ANSWER, not a rung. Report the multiplicity measured,
>            and D1 must then decide by a rule, not by first-match.
>
> **(c) is stated because it is the outcome that looks like a failed
> measurement and is not one.** Do not collapse it into (b).

**D1 — The repair.** Key on the occurrence, retain the per-occurrence insert
refusal, and resolve at the use site by whatever D0 established.

**D2 — Run the four rows in production.** No suppression, no probe. Record what
each row does: passes, or stops on a named next mechanism.

**D3 — The `#[ignore]` labels.** For any row still red, rewrite its readmission
condition. The attribution halves of all four current labels are measured sound
and are not to be touched; only the readmission halves are stale.

**D4 — The `RT-COMPMATCH-TREE-SCRUTINEE` acceptance observation.** See AC-4.

## 5. Acceptance criteria

**AC-0 — The bar in 3b, both halves, each with its own test.** A test that two
response-handling sites within one occurrence claiming one constructor still
produce a refusal, and a test that N legitimate instantiations do not. The
first is the one that a passing suite can silently lose; it gets an explicit
test, not an inference from the four rows going green.

**AC-1 — The four rows, per row, with the current refusal text read rather
than carried.** Row coordinates are the `#[ignore]` attribute line
(149 / 170 / 653 / 713). A row that moves to a different refusal is a result,
not a failure — record which.

**AC-2 — NO OVERWRITE PATH REMAINS.** Instrument the landed repair and assert
that the number of route overwrites is **zero** on all four programs. The
predecessor measured 58 per run under the deferral repair.

> **This AC is a guard on AC-4 and not only on AC-1.** AC-4's reasoning assumes
> the re-key removed overwriting entirely. If any overwriting path survives,
> **AC-4 does not discriminate and must say so instead of reporting a
> witness.**

**AC-3 — 9.5 is not rebuilt.** The landed key is not global. State the key's
type and show the insert check is per occurrence.

**AC-4 — THE `rt_escape` PRODUCTION OBSERVATION.** With AC-2 green, run
`rt_escape:653` and `:713` in production and record what they stop on.

> **TERMINATING OBSERVATION, three outcomes, all producible:**
>
>     stops on the ComputationalMatch tree-producing-scrutinee refusal
>        -> the mode-B result was about the PROGRAM, not about the
>           suppression. Report as a production observation.
>     stops on something else
>        -> the mode-B result was a suppression artifact. Say so; the
>           candidate dies.
>     row passes
>        -> report it. Nothing behind :1279 on this row at all.
>
> **AND THE INDEPENDENCE CAVEAT RIDES WITH IT, or the first outcome reports two
> witnesses where there may be one.** `:713` is `:653` plus two procs;
> `after_file_escape`, `handle_outer` and `main` are byte-identical between
> them and `read_body` differs by four lines. If the refusal arises in shared
> identical code, **the two rows are one observation reported twice.**
>
> **DO NOT COMMISSION "record which proc" HERE. THAT EVIDENCE CANNOT BE
> PRODUCED.** Measured by the runtime-implementer 2026-09-18: the programs are
> **inlined** — only `main` survives in `declaration_occurrences` — and there
> are **no source spans** anywhere under `cranelift_backend/planning/`. An
> origin cannot be attributed to a proc. Occurrence-keying does not change
> this: the rows will run in production and still be inlined and still
> span-less.
>
> ⇒ **State the replication count as `one-or-two, UNDETERMINED`, with the
> topology measurement attached, and do not treat that as a gap in the
> acceptance pass.** What IS settled and carries forward: the refusal is not at
> two separable sites. `second_read` and `after_read` exist only in `:713`, yet
> the two root-to-node ancestor chains agree in expression kind at all 16
> positions and in depth, over trees verified to be genuine trees (zero nodes
> with more than one parent in either).
>
> Settling one-versus-two needs either a shared-proc perturbation with a
> control establishing the perturbed program still reaches the same site, or
> origin-to-declaration attribution in the planner. **The second is a node, not
> a probe, and neither is owned here.** Price one only if AC-4 lands on its
> first outcome and the count is then load-bearing for something.
>
> **Bound:** this observes the FIRST mechanism behind `:1279`. It is not an
> inventory of what is behind it. Whether `[[RT-CLOSURE-BOUNDARY-LANE]]` sits
> below it remains UNDETERMINED and is not resolved by this AC.

**AC-5 — `[[RT-FRAME-MARKER-ONCE]]` is fenced as the mechanism BEHIND `:1279`,
not beside it.** If the `px7n` rows reach the frame-marker refusal after this
repair, that is `px7n`'s next stop and this node does not own it. Do not
absorb it, and do not record the `px7n` rows as having moved owner while they
are still red.

**AC-6 — No regression.** Green in CI, per `COORDINATION` section 12. Local
runs are targeted only.

## 6. What this WP is NOT

- Not `[[RT-FRAME-MARKER-ONCE]]`, which is `px7n`'s next stop.
- Not `[[RT-CLOSURE-BOUNDARY-LANE]]`, which stays UNDETERMINED.
- Not `[[RT-COMPMATCH-TREE-SCRUTINEE]]`. AC-4 hands that node a production
  observation it currently reaches only under suppression. That is an upside,
  not a dependency: row 14's own `D0` is unchanged and that node is not blocked
  on this one.
- **Not a campaign-text edit.** `docs/program/16-recursive-descent-retirement.md:439`
  carries a withdrawal that is routed and must land as written. If AC-4 lands
  on its first outcome, that text is edited THEN, by a separate deliverable,
  citing a production measurement. A candidate is not a witness.

## 7. Contention

**This candidate must land AFTER `142220494`** (`[[RT-DUPLICATED-RESPONSE-BLOCK]]`),
which this node's frontmatter depends on and whose node file this one links.
Both candidates regenerate `docs/program/IMPLEMENTATION-PROGRESS.md`, whose
`Last generated` line is a guaranteed one-line textual conflict. The generator
is `scripts/gen-progress.sh` and CI checks the committed file against its
output, so the regeneration is not optional and must be run against the main
that already carries `142220494`.

Code paths (`responses.rs`, and whatever D0 selects): intersection with the
routed-and-unlanded candidate set is to be re-measured at release, not
inherited from this line.

## 8. Estimated tier: T1

D0 is a structural question about plan relations with three live outcomes; D1
is a soundness-bearing invariant change that must preserve a refusal while
removing a different one. The diff is small and its review turns entirely on an
argument. Not a mechanical port.

## 9. Measured outcome

To be written by the implementing ring.
