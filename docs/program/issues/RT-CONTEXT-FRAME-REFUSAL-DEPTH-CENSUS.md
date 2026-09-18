---
id: RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS
title: "MEASUREMENT NODE, nothing lands in crates/: how many refusals deep are the four RT-CARRIED-RESIDUAL-IH-ARITY rows, and is the stack finite. Three nodes in this series each closed one layer and each found the refusal at its own layer was CORRECT and the cause upstream, so a fourth blind repair is a fourth cycle to learn the same shape. The refuted predecessor forced the admission arm at core.rs:13577 to Ok(true) unconditionally -- strictly more permissive than any key, making the negative result an upper bound rather than a failed attempt -- and the rows still failed at agreeing_recursive_body_unit (core.rs:1230), a deliberate refusal carrying its own two-direction unit test. That was measured to its FIRST STOP AND NO FURTHER. Walk each row's stack, record every stop by file/function/message and every forced arm beside the result it produced, report depth PER ROW never summed, and verdict against the outcome menu: finite-repairable, finite-terminal (then these rows are correctly refused and the disposition is an exemption row plus a rewritten label, not a repair), not-bottomed-out at the six-layer budget, or the rows diverge."
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Steward, 2026-09-17, ruled in evt_339x9h7ys4pe9 on closing RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION as REFUTED: no fourth repair node on these rows until the queue of refusals is measured as a queue. Under the operator's standing L1 direction (2026-09-15, 'The other tests should be fixed'). The probe is reproducible from main and its procedure is published at evt_41nd1exp7vbb0 -- five anchors that each resolve exactly once in core.rs, so it applies with no line numbers; --test-threads=1 is load-bearing because parallel cargo test interleaves stderr and attributing a printed line to the header above it is not a measurement. Predecessor lineage: RT-CARRIED-RESIDUAL-IH-ARITY established the arity refusal is correct at all four sites and is a fallback symptom; the label-correction node established the labels asserted a mechanism and a cardinality comparison the code does not make; RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION was refuted on its own premise (RTPROBE-WRITE = 1 on all four rows, and keying by worker_body_origin readmits nothing)."
---

> # RESULT — DEPTH 3, ALL FOUR ROWS CONVERGE. READ THE EVIDENCE FILE.
>
> **`docs/program/evidence/rt-context-frame-refusal-depth-census.md`** is this
> node's deliverable and the only surface carrying its finding. Landed
> `9682927b7`; measured at `origin/main` `0298c51eb08df955c502a4fa2bfd17e315745e22`.
>
>     L1  recursive_position_captures_all_planner_recoverable   core.rs:13532
>     L2  agreeing_recursive_body_unit                          core.rs:1230
>     L3  resolve_context_capture_claim                         core.rs:9551
>
> **The stack is FINITE at depth 3.** `L1` and `L2` reject data that is PRESENT
> and are both correct; `L3` is an ABSENCE, and the technique terminates there
> on principle, not on budget. Verdict is `A` or `B` and **this node could not
> discriminate**; the discriminator is a planner-side question, now owned by
> **[[RT-CONTEXT-CAPTURE-CLAIM-ABSENCE]]**.
>
> > **THE `title:` AND `origin:` ABOVE DESCRIBE THIS NODE'S PREDECESSOR'S LIMIT,
> > NOT THIS NODE'S RESULT.** They say the stack was *"measured to its FIRST STOP
> > AND NO FURTHER"* — true of `RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION`,
> > false of this node, which walked two stops further. **The four rows'
> > `#[ignore]` labels carry the same stale "READMISSION CONDITION UNKNOWN"
> > wording** and are rewritten by the successor node. The Steward cut a wrong
> > node off these two surfaces on 2026-09-18 before reading the evidence file.
> > **A framing text is written before the result and is never updated by it.**
>
> # RELEASED 2026-09-17 (Steward). L1 lane.
>
> Frame: `docs/program/wp/RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS.md`.
>
> **NOTHING LANDS IN `crates/` FROM THIS NODE.** The probe is applied and
> reverted inside the turn; the durable artifact is the census.
>
> ## THE DISCIPLINE THIS NODE LIVES OR DIES BY
>
> **Forcing past a deliberate refusal is a MEASUREMENT TECHNIQUE, never a
> repair, and a forced arm is never evidence that the arm should be relaxed.**
> Every refusal forced past in this series has so far turned out to be
> **correct** — `agreeing_recursive_body_unit` carries a two-direction unit
> test asserting it refuses. **A green row reached by forcing is not a fixed
> row; it is a row whose remaining guards you disabled.** Record what you
> forced, at every layer, beside every result.
>
> ## OUTCOME `B` IS A REAL RESULT, NOT A FAILURE
>
> If the stack bottoms out at a refusal that is correct and terminal, **these
> rows are correctly refused and are not a defect.** The disposition is then an
> exemption row plus a rewritten label, not a repair. Report it as the finding
> it is.
>
> ## BUDGET
>
> **Six forced layers per row, or one turn, whichever comes first.** Hitting
> the cap is outcome `C` and is a result — it forecloses the fourth blind
> repair just as well as a bottom does. **Do not carry a half-walked stack into
> a second turn silently.**
>
> `scripts/ken-cargo`, scoped: `-p ken-runtime --lib` then `-p ken-cli --test
> <suite>`. **Never `--workspace`** (`COORDINATION §12`).
>
> Reviewers: runtime QA + Architect. Then Steward M1-M3a, lieutenant M4-M9.
