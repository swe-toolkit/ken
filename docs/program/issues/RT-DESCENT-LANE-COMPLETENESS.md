---
id: RT-DESCENT-LANE-COMPLETENESS
title: "Is the functionized lane a complete replacement for RecursiveDescent, or has it been carrying only the ported subset? D2c refused NINE programs the retiring lane compiles, across FOUR independent constructs -- a pattern, not a missing case, so this is a lane-completeness question and not a port"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_7qtgrtwv76vke, 2026-08-16, on runtime-leader's corroboration and construct inventory evt_6bvnv6t4teech: the D2c reds are four distinct refusing constructs across nine programs, the artifact hypothesis is closed, and the successor is a lane-completeness node rather than a missing port. Node cut assigned to the Steward in that ruling; D1 soundness retained by the Architect. Steward-filed per COORDINATION section 2."
---

Frame: `docs/program/wp/RT-DESCENT-LANE-COMPLETENESS.md`. Read it before
pulling anything here — this node's shape is the whole point of it.

## Why this node exists

[[RT-DESCENT-RETIRE]]'s `D2c` rerouted `select_body_emission_authority` to never
return `BodyEmissionAuthority::RecursiveDescent`, deleting nothing. It reded 17
of 943. Fourteen were inside the frozen `D2b` set; **nine of those fourteen are
the surviving lane refusing a program the retiring lane compiles, across four
independent constructs.**

**Four separate representability gaps is a pattern, not an omission.** The
question is therefore not *"add the missing case"* but **whether the
functionized-units lane is a complete replacement for `RecursiveDescent`, or has
been carrying only the ported subset.**

## The artifact hypothesis is CLOSED

The identical `UnsupportedLowering` / `StaticWorkerBinding` — same constructor
origin 36, same static worker field 0, same origin 35, same recognition 2 —
reproduces at **untouched base `c98f72ba8`** through the **pre-existing**
exclusion mechanism, touching no production code (runtime-leader,
`evt_6bvnv6t4teech`).

⇒ **Two independent instruments, one of which does not involve `D2c`'s edit at
all. The finding is about the lane, not about the reroute.** That was the one
way this could have been an artifact and it is now excluded.

**The uncomfortable half: the evidence predates `D2c` entirely.** The exclusion
mechanism was a complete differential instrument the whole time; the sentinel
ran the functionized route, held the answer, and discarded it. `D4` bounds how
far that shape spread.

## The four constructs

| construct | n |
|---|---|
| `ComputationalMatch` / in-flight non-transferable activation | 4 |
| `StaticWorkerBinding` | 2 |
| Backend `Module` / missing recursive-position-1 worker projection | 2 |
| Backend `PlannerInvariant` / missing affine checked-root authority | 1 |

Exact test names are in the frame, section 3. A further **five** reds assert the
retiring lane's own control, lifecycle or route state with no program refusing;
those are `D6` rewrites in the predecessor and **stay gated behind the nine**.

> # `D5` IS NOT ANSWERED. THE ROUTE THAT REACHES THE SELECTOR IS `ken native-build`.
>
> **SUPERSEDES the block that stood here, which read *"`D5` ANSWERED: YES — and
> it exposes a population defect."* That heading was wrong in its answer and
> right in its alarm.** Architect `evt_74f5ppk3tnh1q` proposed it, the runtime
> ring refuted its premise (`evt_6w6sbwrb8k1zq`), the Steward verified the
> refutation in the tree (`evt_46h0cskzqkjgy`), and the Architect confirmed it
> and supplied the real route (`evt_1krepwa0fsf4n`).
>
> ### WHAT WAS REFUTED: both examples run the INTERPRETER, so they select nothing
>
> `crates/ken-cli/tests/rosetta.rs` spawns `CARGO_BIN_EXE_ken run`; `run_file`
> reaches `ken_cli::run_program`, declared at `crates/ken-cli/src/lib.rs:64` as
> **generic over `ken_interp::HostHandler`**. `select_body_emission_authority`
> has its sole production call inside the **cranelift backend**, and the
> `units.rs` no-worker guard is likewise native-only.
>
> ⇒ **Neither example selects any `BodyEmissionAuthority`, because neither
> enters the path where authorities exist. Their greenness is evidence about
> NEITHER lane.** The two-outcome table that stood here presupposed a selection
> and died at that shared premise; **both of its arms were wrong for one
> reason.**
>
> ### WHAT HOLDS UNCHANGED, and it was never the disputed part
>
> **A two-recursive-position constructor is source-admissible, and two are IN
> THIS REPOSITORY, RUNNING IN CI TODAY:**
>
> | file | declaration | recursive positions |
> |---|---|---|
> | `examples/rosetta/tree-traversal/tree-traversal.ken` | `data Tree = Leaf \| Node Tree Char Tree` | **`[0, 2]`** |
> | `examples/rosetta/letter-frequency/letter-frequency.ken` | `data Tree k v = Leaf \| Node (Tree k v) k v (Tree k v)` | **`[0, 3]`** |
>
> **A binary tree. The most ordinary inductive type there is.** Neither is a
> fixture and neither is skipped: `crates/ken-cli/tests/rosetta.rs` runs each
> example through the **real `ken` binary as a subprocess** (`CARGO_BIN_EXE_ken`,
> `Command::new(ken_bin())`); `tree-traversal` is in `NEEDS_COLLECTIONS`; and
> `oracle_for` requires every dir to declare exactly one oracle — `expected` or
> `KNOWN-GAP.md`, **never a silent skip. Both dirs carry `expected`**, so each is
> asserted to compile *and* produce output.
>
> ⇒ **The declaration shape construct 3 cannot build an IH prefix for is
> source-admissible and in-tree.** What is NOT established is that anything
> carries it *into the selector* — that is exactly what `D5` still asks.
>
> ### THE ROUTE THAT DOES REACH NATIVE LOWERING: `ken native-build`
>
> **Architect `evt_1krepwa0fsf4n`. Every hop below located by the Steward in the
> tree, not transcribed** — the chain is unbroken:
>
> | hop | coordinate |
> |---|---|
> | subcommand dispatch | `crates/ken-cli/src/main.rs:51` to `native_build_file` at `:81` |
> | CLI entry | `ken_cli::build_native_program`, `crates/ken-cli/src/lib.rs:21` |
> | elaborator driver, **takes real Ken source text** | `compile_native_program_sources`, `crates/ken-elaborator/src/compiler_driver.rs:2524` |
> | runtime packaging | `build_bound_process_starter_executable_artifact`, `object_linker_packaging.rs:879` |
> | into cranelift | `emit_bound_process_program_object_with_cranelift`, `object_linker_packaging.rs:937` |
> | the selector | sole production call of `select_body_emission_authority` |
>
> ⇒ **Ken does NOT have a native backend that no source program reaches, and
> "fixture-only" is NOT structurally true for every construct.** That was the
> larger of the two possibilities the reframed question opened, and it is
> closed. It is worth stating positively.
>
> ### THE POPULATION DEFECT STANDS — on a corrected example, and it is BIGGER
>
> **The claim was right; the example demonstrating it was wrong.** The Rosetta
> pair does not exercise the mechanism, so it could not show the exclusion
> mattered. **The `native-build` corpus does, and it is excluded by the same
> scoping.**
>
> **Measured by the Steward, not cited:** `build_native_program` is called from
> **18 test files and 36 call sites under `crates/ken-cli/tests/`** — among them
> `px4b_native_production` (11 calls), `px7l_checked_host_recursive_bind`,
> `px7o_heterogeneous_eliminator_frames`, `px7p_constructor_field_composition`,
> `px8ta_oriented_subcontinuation`, `px8l_recursive_decl_native`,
> `mrc_4a_cross_crate_census`, `mrc_4a1_child_transport`. **This is larger than
> the seven files the ruling names.**
>
> ⇒ **`D1`'s 805 selector arrivals and `D2c`'s 943/0/4 were measured over `-p
> ken-runtime --lib`. Every one of those 36 call sites is OUTSIDE it.** The
> census that concluded "28 arrivals, all fixture-only" was taken over a
> population that structurally excludes **the real-source corpus that actually
> reaches the selector.**
>
> **Whether any of them selects `RecursiveDescent` is UNMEASURED.** That is the
> live question, and it is one command to answer.
>
> **This is not a criticism of the ring: the frame scoped it that way.**
>
> ### `D5`, RESTATED SO THE REAL PATH CAN PRODUCE IT
>
> > Compile a Ken source program declaring a **two-recursive-position
> > constructor** through `build_native_program`, and observe whether it reaches
> > the no-worker guard in `units.rs` — the `backend_module` error reading *"the
> > selected case has a recursive position {position} that the continuation
> > specialization projects no worker for"*.
>
> **Two cautions, both from the Architect, both binding on whoever runs it:**
> `tree-traversal.ken` is in `NEEDS_COLLECTIONS`, so the prelude must be
> prepended **exactly as `rosetta.rs` does**; and `native-build` requires a
> checked `Program I main`, so **a build failure is evidence only if it is
> attributed to the tree shape rather than to the harness. A refusal that is
> not attributed is not a result.**
>
> **`D1`'s four verdicts are UNCHANGED by any of this.** Construct 3 remains
> **missing port**. What `D5` moves is whether the gap is **reachable** — the
> blocked-versus-recorded-gap question, not the correctness one.

## `D1` DELIVERED. THE HEADLINE QUESTION HAS AN ANSWER, AND IT IS "NO".

**Architect `evt_5cxzxp4b6q31v`, grounded at base `c98f72ba8`** (no `crates/`
drift to `main` at `c9cbd1f5a`, checked by diff; machinery read, not error
strings; fixtures read, not reports).

**Four verdicts and they do NOT answer alike — TWO correct semantics, TWO
missing port.** ⇒ **The functionized lane is NOT a complete replacement. It has
been carrying only the ported subset in two respects.**

### The discriminator, which is the reusable part

> **Does the refusal make a claim about the PROGRAM'S DENOTATION, or about the
> COMPILER'S OWN BOOKKEEPING?**

Constructs 1 and 2 say what a value *is* and why it cannot exist across a
boundary. Constructs 3 and 4 name a compiler structure that is **absent**.
**That line, not the severity of the message, is where correct-semantics and
missing-port separate.**

| # | construct | verdict |
|---|---|---|
| 1 | `ComputationalMatch` / in-flight non-transferable activation (4) | **CORRECT SEMANTICS** — an in-flight activation is control state live only for that activation; transferring it across a durable boundary names a frame that need not exist. Independently corroborated: merged `RT-LEXICAL-R3-FUSION-EMITTER` already carries it as a ratified refusal. |
| 2 | `StaticWorkerBinding` (2) | **CORRECT SEMANTICS** — the sibling arm of the same walk as construct 1: **one law at two callable kinds.** Descent compiled it with **zero boundary crossings**, i.e. by never materializing the aggregate. **Porting it would mean giving closures a durable lane — a semantic change to Ken, not a port.** |
| 3 | Backend `Module` / missing recursive-position-1 worker projection (2) | **MISSING PORT.** An internal error, not a typed refusal; the code says `D6a` deliberately does not generalize to a multi-worker population. Trigger is a **binary-tree fold**. ⇒ [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] |
| 4 | `PlannerInvariant` / no affine checked-root authority (1) | **MISSING PORT.** Descent mints the token unconditionally; functionized mints it only in the root unit, so a terminal answer emitted in a non-root unit finds `None`. **An unrouted token, not a statement about the program.** ⇒ [[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]] |

### `D5` — THE QUESTION NOBODY ASKED, and it can still flip this node

**`0/12` DOES NOT BOUND CONSTRUCT 3.** That measurement was taken over the
twelve `LexicalCallArgumentRecursor` renderings and argues from kernel
definition admission rejecting `Application(Lambda, ...)` in infer position —
**an argument about the lexical-call-argument shape.**

**Construct 3's mechanism is a different shape:** a limit on
`ComputationalMatch` cases with more than one `recursive_positions` entry. The
row-3 **fixture** is unreachable; **the single-worker limit is a general
property of the continuation specialization.**

⇒ **`D5`: is there any source-admissible program with a match case carrying two
recursive positions?**

**Routed to the Architect.** It is a frontend-admission question the runtime
ring cannot answer alone, and **it is the one input that could still flip this
node from RECORDED GAP to BLOCKED.** `D2`'s recorded-gap disposition on the nine
measured programs is **not** reopened.

**This is foreclosed shortcut 1 arriving for real: the fixture is how it was
found, not the extent of what was found.**

## Deliverables — THREE OF FOUR ARE ALREADY IN

**`D2`, `D3` and `D4` were delivered pre-frame** by runtime-leader
(`evt_2fmjv69z5bg2g`) at exact
`3c9b8bbd5fae09859d6e330f8ac0a17b40fe1f68` — a **different SHA from this node's
base `c98f72ba8`**. No candidate or instrumentation remains; `D2c` untouched.

| id | owner | state |
|---|---|---|
| **`D1`** | **Architect** | **OPEN — the only one.** Per construct: is the refusal **correct semantics** or a **missing port**? **Four verdicts, not one**; they may not answer alike. Soundness question, not decided by the ring as engineering. |
| **`D2`** | runtime ring | **DELIVERED.** All nine map byte-for-byte to five hash-tagged lexical fixture renderings, all fixture-only under merged [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]] ⇒ **zero source-reachable**. Recorded-gap input, **not** a soundness verdict. The mapping was established for all nine, not inherited from the sentinel. |
| **`D3`** | runtime ring | **DELIVERED as input.** **All nine** overlap an explicitly claimed merged-node population, but those records **do not claim complete `FunctionizedUnits` emission** — their dispositions are source-unreachable asserts/invariants or a preserved refusal. Ownership correctly left undecided. |
| **`D4`** | runtime ring | **DELIVERED, and it is not marginal.** `owner` and `multiplicity` each run five expressions and **every functionized compile aborts** (row 1 `PlannerInvariant`; rows 4/5 `StaticWorkerBinding`) **while their trace assertions stay green. Zero completed functionized runs.** |

### My `D3`-FIRST SEQUENCING WAS WRONG, and the frame is amended

I put `D3` first behind a hard stop on any hit, assuming an overlap is decidable
**independently of `D1`**. It is not. **The hit is universal — nine of nine — so
the stop would have fired on everything and stalled the node.** And whether an
overlap is an **erratum** is exactly `D1`'s verdict: a *preserved refusal*
disposition is **accurate** if that construct's refusal is correct semantics and
**false** if it is a missing port.

⇒ **`D3` is not an independent gate; it is a CONSEQUENCE of `D1`, per
construct.** `AC-5`'s hard stop is withdrawn in the frame. The ring supplying
the input and declining the ownership call was the right boundary.

## Standing

- **`D2c` stays UNPUBLISHED and unrebased** at
  `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`. Its base `c98f72ba8` is what the
  pin is measured against.
- **No Runtime implementation is authorized by this node.** It measures and
  adjudicates; it deletes and ports nothing.
- **[[RT-DESCENT-RETIRE]]'s `D3`-`D8` stay gated**, and **no `D6` re-home is
  lawful** while this node is open.
- **Two shortcuts are foreclosed** — *"fixture-only so it doesn't count"* and
  *"`RecursiveDescent` compiled it, so port it."* Both are argued in the frame,
  section 6; neither may be assumed from the error text.
