# RT-FNUNIT-RESULT-TOKEN — a broad starter shape fails result-token decode on the functionized lane

**A composite starter program reaching `FunctionizedUnits` fails with `native
result token 265 is not in the result table`. The wall predates the seed-closure
port and was masked by it; retiring `SeedClosureCall` made it reachable. This
node makes the shape work on the functionized lane.**

**Owner:** Team Runtime. **Branch:** `wp/RT-FNUNIT-RESULT-TOKEN`.
**Size:** M — **provisional, and §3 may overturn it before any code is written.**
**Risk:** medium — the failure is in result decoding, which every native return
crosses.

**Read `docs/program/16-recursive-descent-retirement.md` first.** This node exists
because of that campaign's Trap 2, and the frame does not repeat the traps.

---

## 1. Fixed inputs

Measured at `ddddb48d`, re-verified at `origin/main = 464cb446` (2026-08-08),
and **re-pinned by the Steward at `origin/main = 22fb3a61` (2026-08-11). One
of the three has MOVED.**

| path | blob at `464cb446` | blob at `22fb3a61` | |
|---|---|---|---|
| `crates/ken-runtime/src/cranelift_backend/surface.rs` | `99b9b507` | `99b9b507` | unchanged |
| `crates/ken-runtime/src/cranelift_backend/artifact/api/tests.rs` | `f96a0b0b` | **`7ea92d79`** | **MOVED, `+208/-9`** |
| `crates/ken-runtime/src/cranelift_backend/compiled.rs` | `31e5c149` | `31e5c149` | unchanged |

`RT-PRODUCER-MATCH-PORT` was in flight when this frame was written and has
since **merged**; it did not touch any of the three.

**Re-pin at pickup anyway.** These are recorded so the derivation below can be
checked against what changed — not so the numbers can be trusted.

### 1.1 What moved, and the new adjacency it creates for you

One commit moved that file: **`90ddcf1c`, `RT-DYNAMIC-ARM-SCALAR-MERGE c1`** —
typed role authority at the consumption boundary, admitting real packages to
the native lane.

**Your deliverable is intact.** The `nc22` row is still `#[ignore]`d and still
names this node as its owner, in the same words, and nothing in that commit
touches `NativeResultDecode`, the result table, or the decode path.

**But the instrument you would reach for now has two consumers it did not
have.** `nc22_program_with_body` went from **6 references to 8**. The two new
callers are **not** `nc22` tests:

- `a_package_backed_program_without_a_role_record_refuses_before_lowering`
- `the_synthetic_entrypoint_consumes_the_authority_it_is_given`

⇒ **Changing the shared program builder to make `nc22` green now perturbs two
tests belonging to a different node.** That is not a prohibition — it is the
thing to notice before you edit it, and the reason to prefer a change local to
the `nc22` row or a new builder over widening the shared one. If you do widen
it, those two are the controls that say whether you broke something.

**`RT-DYNAMIC-ARM-SCALAR-MERGE` is `active`.** Confirm its live state with your
leader before editing this file — the contention is real and current, not
historical.

## 2. What is known, and how it was established

- The failing row is `nc22_cranelift_agrees_with_runtime_ir_report_for_broad_starter_shapes`
  (`artifact/api/tests.rs`), currently `#[ignore]`d with this node named as owner.
- The error is `BackendFailure::NativeResultDecode { token }`, declared in
  `surface.rs` and rendered there by a single `write!`.
- **The port is NOT the cause, and this was measured rather than argued.**
  Flipping `nc22`'s callee from `RuntimeExpr::Closure` to
  `RuntimeExpr::LexicalClosure` — an arm live since [[RT-DECL-CLOSURE-PORT]] and
  untouched by [[RT-SEED-CALL-PORT]]'s `D2`/`D3` — reproduces the **identical**
  error. The shape was already unsupported on the functionized lane.

> #### CORRECTED 2026-08-08 — this frame previously named FIVE producers and the
> #### enumeration was wrong when written, not stale
>
> It said: *"Its producers are five sites in `cranelift_backend/compiled.rs` —
> `:135`, `:168`, `:194`, `:197`, `:200` — each an `ok_or_else` on a failed
> lookup."* **`compiled.rs` raises it at eight sites, and the blob is identical
> at `ddddb48d` and `464cb446`** — nothing moved; the count was simply short.
>
> **The omission is not arbitrary. The one site the error message actually
> names — the `Table` arm's `result_table.get(&token)` miss, the only literal
> result-table lookup of the eight — is among the three that were missing.**
>
> ```sh
> git grep -n 'NativeResultDecode' -- crates/ken-runtime/src/cranelift_backend/compiled.rs
> ```
>
> **The authoritative arm-by-arm table is [[RT-WORKER-FIXTURE-DECODE]] §1e, and
> it is not repeated here** — that frame had all eight correct on 2026-08-07,
> a day before this one contradicted it with five. One table, one place to
> maintain. Read §1e, then come back.
>
> Two consequences, both binding on `D2`:
>
> 1. **"each an `ok_or_else` on a failed lookup" is false for two of the eight.**
>    An unconditional `return Err` on a reached path is not a lookup miss — it
>    says the decoder was *selected* wrongly, or the artifact declared it returns
>    only a trap. Those route to **production**, not registration, so the false
>    uniformity biased `D2`'s own discrimination toward one of its two answers.
> 2. **The message does not localize the fault.** One variant, one `write!`, eight
>    raise sites — *every* one of them prints `native result token N is not in the
>    result table`, and that wording is literally accurate for exactly one. **Do
>    not infer the site from the text.** Discriminate by which `ResultDecoder`
>    arm was selected for `nc22`.

**Discounted evidence, recorded so nobody re-counts it:** an earlier smaller
record-returning probe failed on both arms with a *different* error
(`BoundaryCarrier` unsupported). It does not attribute this stop.

## 3. THE SCOPING ANSWER — `D1` IS DISCHARGED, AND IT FALSIFIED THIS SECTION

> ### `D1` ANSWERED AT `be54d47f` AND THE STEWARD'S PREMISE WAS WRONG.
> ### `M` STANDS. Proceed to `D2`.
>
> **Read this section as the corrected state.** The three claims it used to
> make are struck below with what replaced them, because two of them were
> load-bearing on `AC-2` and a reader who skipped to the ACs would still be
> working from them.

**What survives, unchanged.** `nc22` is a **single composite program**, not a
loop over shapes — one nested `Let` / `Call{callee: Closure}` / `Match` /
`Construct` / `Record` / `If` tree, where "broad starter shapes" names breadth
*within* one program. It remains live and still reproduces
`NativeResultDecode` token `265`.

### The three struck claims

**STRUCK — *"it is the only one of 21 `nc` fixtures carrying a `Call` whose
callee is a `Closure` or `LexicalClosure`."*** False. `nc5_seed_examples`
carries `closure-capture-application` with `callee: RuntimeExpr::Closure` and
`captures: ["decl:fixture::Local::y"]` at `crates/ken-runtime/src/ir.rs:1084`.
It is a `pub fn` in `ir.rs` — **production, not a test fixture.**

**The specific defect was scope, not staleness.** The claim was true *within*
`crates/ken-runtime/src/cranelift_backend/artifact/api/tests.rs`, which still
holds exactly one `callee: Closure` at `:106`. **One file was measured and
"the corpus" was written.** A scoped measurement stated at unscoped width reads
as a far stronger claim than it is, and this one carried two false conclusions
out of it.

**STRUCK — *"family width is UNESTABLISHABLE from this corpus."*** False. There
are two instances of the closure-call shape, and comparing them establishes the
axis without authoring anything.

**STRUCK — *"the corpus has ZERO live coverage of this shape in either
direction."*** False as stated. `nc5` is green on `FunctionizedUnits` under two
existing committed corpus controls. **See `AC-1a` — this one is struck
conditionally, and the condition is not yet discharged.**

### The axis is RETURN SHAPE, and the closure call was never the wall

| fixture | returns | on `FunctionizedUnits` |
|---|---|---|
| `nc5` closure-capture-application | `Int 7` | **green** |
| `nc22` | `Record { ok: Bool(true), value: Int(7) }` | fails, token `265` |

⇒ **The node is sharper, not bigger.** `D2` identifies `nc22`'s actual
`ResultDecoder` arm; only then does `AC-2` decide whether the uninstantiated
Bool / bare-constructor / Boundary cells need authored fixtures or a report.
**No re-cut. `M` stands.**

**`D1` did its job.** It was written as a gate that could resize the node, and
it returned "the sizing survives, for a reason nobody had." That is a better
outcome than agreement, and it is why the deliverable exists.

## 4. Deliverables

- **`D1` — DISCHARGED at `be54d47f`.** Evidence-only, `+195/-0`, one new record
  path, crates untouched. It falsified this frame's corpus premise and
  established the return-shape axis; see §3. `M` stands and no re-cut is owed.
  **Its residual is `AC-1a`, which is `D2`'s to discharge.**
- **`D2` — DISCHARGED, 2026-08-11**, with disposable instrumentation and no
  candidate edit. **The answer, and it retires two of this frame's own
  framings:**

  `nc22` reaches `compiled.rs`'s **`Boundary` decoder and raises at its `_ =>`
  arm.** Token `265` is a **well-formed `InvocationAggregate` tag (Record)** —
  **not** an unrecognized tag, and **not** a `Table` lookup.

  **STRUCK — *"whether the gap is the token's PRODUCTION or its
  REGISTRATION."*** It is neither. The tag is produced correctly and is
  well-formed; **there is simply no arm to receive it.** The production/
  registration fork was the wrong pair of alternatives to offer.

  **STRUCK — *"if the arm is `TrapOnly` or an unrecognized `Boundary` tag, the
  fault is upstream of the decode table entirely and §7's hard stop is likely
  live."*** The arm **is** the `Boundary` `_ =>`, but the tag is well-formed,
  so the fault is **not** upstream and **§7 does not fire.** This frame paired
  "the `_ =>` arm was reached" with "the tag is malformed" as though they
  travelled together. They do not, and the distinction is the whole finding.

  `AC-1a` is discharged **positive**: `nc5` reaches the same `Boundary`
  decoder's native call by name and emits `ImmediateInt 7`, so it is a genuine
  lane control rather than a fixture that is green because the path was never
  reached.

  **What survives unchanged, and it did its job:** *establish the site by
  observation, not by the message.* All eight producers render one text
  accurate for exactly one of them, and `nc22` is now a **confirmed instance of
  the seven where it is false** — see `D3`.

- **`D3` — the repair. TWO parts.** Cut ruled by the Steward 2026-08-11
  (`evt_5fn8nb9s03785`).

  **`D3a` — the decode arm.** An `InvocationAggregate` decode path.
  **The mechanism is the Architect's ruling, not this frame's and not the
  leader's** — the question routed to it is whether the correct shape is *a
  specific `InvocationAggregate` arm with the `_ =>` refusal RETAINED for the
  remaining six tags.* **Do not build ahead of that ruling.** `AC-4` binds
  hardest here and is quoted in the ruling request: the repair lands in one of
  the two arms this frame names as dangerous, and "add a correct arm" versus
  "widen `_ =>` until it stops complaining" are **indistinguishable in a green
  test.**

  **`D3b` — the error message, and it needs no design input.** The current text
  *"native result token N is not in the result table"* is **false for this
  case**: `265` is a well-formed tag reaching an unhandled arm, not a table
  miss. **A message that misdirects the next reader to the table is a defect on
  the decode surface and is this node's to fix.** This part can be staged
  before the Architect rules.

  **Out of scope: the six other unhandled `Boundary` tags.** Name them in the
  handback; do not handle them.
- **`D4` — un-skip `nc22` and prove it green on the functionized lane.**
  **This node closes on the row running, not on the skip being tidied.**

## 5. Acceptance criteria

- **`AC-1` — `nc22` runs green on `FunctionizedUnits`**, with its `#[ignore]` and
  the owner reference removed. **Seen to fail before it passes** — this row has
  been dark, so a green with no demonstrated red is not evidence the repair did
  anything.
- **`AC-1a` — DISCHARGED POSITIVE, 2026-08-11.** `nc5` reaches the same
  `Boundary` decoder's native call **by name** and emits `ImmediateInt 7`. ⇒
  **It is a genuine control on the functionized lane**, not a fixture that is
  green because the path was never reached, and §3's third struck claim stays
  struck. The stop this AC carried did not fire.

  **Kept below as landed text, because the reasoning is the reusable part.**

  Establish that `nc5` REACHES NATIVE EMISSION, at `file:line`, or record that
  it does not. Added by the Steward 2026-08-11 on `D1`'s own caveat, promoted
  from a note because it is load-bearing: **the residual census `D1` ran
  observes IR only.**

  Everything `nc5` contributes rests on it being **green on
  `FunctionizedUnits`**. If it never reaches native emission, it is green for a
  reason that has nothing to do with the lane, and three things follow: it is
  **not** a control on the functionized lane; §3's third struck claim becomes
  substantially true again; and `AC-2`'s coverage decision would be taken
  against a fixture that cannot observe the wall move in **either** direction.

  ⇒ **A control that passes because the path was never reached is worse than an
  absent one, because it reads as coverage.** Either answer discharges this AC.
  **If `nc5` does not reach native emission, stop and return to the Steward** —
  that changes what `AC-2` is deciding, and the coverage call may not rest on
  it.

- **`AC-2` — RESOLVED by `D2`, without authoring a fixture family.** The
  enumerable family is **`Boundary` tags**, not program shapes:
  `ImmediateInt`/`Bool`/`PersistentGround` are **handled**;
  `InvocationAggregate` — **Record and bare constructor together, one cell** —
  falls through; **six other tags remain unhandled.**

  **That collapses the coverage question.** Bool is already handled, and Record
  and bare-constructor share a single tag cell, so there is no family to author
  fixtures for. **No proliferation, no re-cut, `M` stands.**

  **What this AC still requires of the handback:** name the six unhandled tags.
  They are out of scope to fix and in scope to enumerate — an unnamed remainder
  is how a narrowed surface reads as a complete one.
- **`AC-3` (no-regression).** Workspace green **in CI** — never a local
  `--workspace` run (`COORDINATION §12`).
- **`AC-4` — the decode surface stays fail-closed, on the arm the repair
  actually touches.** A value the decoder genuinely cannot interpret must still
  raise `NativeResultDecode` rather than being defaulted, silently mapped, or
  widened away. **Making the error disappear is the failure mode, not the fix.**

  **State the guard on the path `nc22`'s value takes**, named by the
  `ResultDecoder` arm `D2` identified — not on the table lookup by default.
  Seven of the eight producers are not table lookups, so an `AC-4` discharged
  against `result_table` **passes vacuously** for a repair that widened the
  `Boundary` `_ =>` arm or the `TrapOnly` arm. Those two are the dangerous ones:
  each is an unconditional refusal on a reached path, and the cheap way to make
  either stop firing is to accept what it was built to reject.

## 6. Banned scope

- **Adjusting `nc22` to pass** — narrowing its assertions, changing its shape, or
  re-routing it off the functionized lane.
- **Weakening the `NativeResultDecode` refusal.** See `AC-4`.
- **Retiring any residual class or touching the selector or the
  `RecursiveDescent` lane** — those are the campaign's nodes.
- **Absorbing a re-scope.** `D1` has answered and the node is **not** mis-sized,
  so this is no longer about `D1`. It still binds: if `AC-1a` comes back saying
  `nc5` does not reach native emission, or `D2` finds the coverage question is
  wider than the return-shape axis, that is a Steward recut and not something
  to work through.

## 7. Hard stop

Stop and return the seam if the repair requires changing what a native result
token *is* for callers other than this shape, or if `D2` finds the gap is in
token **production** inside emitted code rather than in the decode table — that
is a different layer and likely a different node.

## 8. Why this blocks `RT-DESCENT-RETIRE`

[[RT-DESCENT-RETIRE]] **deletes the `RecursiveDescent` emission lane.** This shape
is currently supported only there. If it is still unsupported on the functionized
lane when that deletion lands, **the retirement silently narrows what Ken can
compile** — and with `nc22` skipped, no row in the corpus would report it.
