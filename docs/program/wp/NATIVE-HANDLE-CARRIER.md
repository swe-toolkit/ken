# NATIVE-HANDLE-CARRIER — carry the sealed `BufferHandle` to native GREEN

**The elaborator half is done and preserved. The residual is one `ken-runtime`
primitive arm plus a real rebase, and landing it closes
[[PX8-F-CAP-41]] Phase 2 in the same merge.**

**Owner:** Team Runtime (`runtime-leader` + `runtime-implementer` +
`runtime-qa`). **Branch:** `wp/NATIVE-HANDLE-CARRIER`. **Size:** M.
**Risk:** medium — the code slice is S; the rebase is where this gets lost.

**Status:** Steward frame. **Amended 2026-08-17 — read the banner below before
any section.** The [[RT-NATIVE-FNSPLIT]] serialization
(`evt_1v37rgez26kmf`) is **spent**: that node is `merged`.

⭐ **On the Linux ABI I critical path.** `PX8` gates 15 of that program's 19
nodes; this is one of `PX8`'s three blockers.

---

> # AMENDED 2026-08-17 (Steward). THE INPUT REF CHANGED AND THE ARM MOVED FILES.
>
> **This frame was last edited `b5126f574` at 2026-07-29 15:09. The preserved
> candidate it should point at was cut at 16:08 — 59 minutes later.** Everything
> below was written before that candidate existed, so `§1`'s input ref and `§7`'s
> `D1`/`D2` name work that is already done. Measured at `origin/main =
> 2e7daa622`.
>
> ## What is already done, and where
>
> **`85dcee25` is the input. Not `c07e63c2`.** Per this node's own 07-29
> disposition (`evt_5mtkdft1nxmwp`), `85dcee25` carries **a completed,
> uncontested `D1` rebase** — `git range-diff` 3/3 `=`, no conflict, no side
> choice — **plus the `D2` identity arm**, and it validated at
> `rt_parity_native` **11 passed / 1 failed**.
>
> ⇒ **`D1` and `D2` of `§7` are DONE on that ref.** Do not rebase `c07e63c2`
> across 1580 commits to re-derive a merge that already exists clean.
>
> ## But `D2`'s one-line edit NO LONGER HAS A TARGET
>
> `85dcee25` adds `int_to_uint64_raw` to the identity arm at
> **`lowering/core.rs:9713`**. That arm is **no longer in `core.rs`.**
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] merged **2026-08-17** and relocated it
> to **`lowering/core/primitive.rs:206`**.
>
> | | |
> |---|---|
> | on `main`, is the primitive still absent from `crates/ken-runtime/src/`? | **yes, zero occurrences** — the frame still has its subject |
> | the elaborator half of `85dcee25` | carry it forward |
> | the one-line runtime arm | **re-derive at the new home; the preserved hunk will not apply** |
>
> **The whole `ken-runtime` residual is one line plus a 20-line test**
> (`85dcee25` vs its base: `core.rs` 1/1, `core/tests/values.rs` 20/0). `§9`'s
> "no concurrent `lowering/core.rs` edit" is spent with `RT-NATIVE-FNSPLIT`; the
> live contention is `lowering/mod.rs`, see the sequencing note below.
>
> ## A SECOND site now exists, and `AC-2`'s control does not point at it
>
> `primitive.rs:93-98` is a `scalar_kind` map that did not exist when `85dcee25`
> was cut. It maps the same `uint8_to_int | int_to_uint8_raw` pair to
> `Some("Int")`, and its own comment says a carried word in such a position **is
> projected through the emitted scalar helper.**
>
> ⛔ **Open question, and it is NOT the ring's to settle alone:** does
> `int_to_uint64_raw` need a `scalar_kind` entry, and does routing its operand
> through a scalar `Int` helper truncate the **Big** carrier that `AC-2` exists
> to protect? **`AC-2`'s control mutates the arm at `:206`. A truncation living
> in `scalar_kind` would survive that control untouched** — the control would
> pass and the defect would ship. Raise it with the coordinates before building.
>
> ## Sequencing — UNBLOCKED 2026-08-17. All five dependencies are `merged`.
>
> [[RT-SITEOP-CARRIED-WITNESS]] `D2` **merged** (PR #2557, exact `a388dc06`), so
> `RT-NATIVE-FNSPLIT`, `RT-JOIN-DISPOSITION`, `RT-DECL-CLOSURE-PORT`,
> `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` and `RT-SITEOP-CARRIED-WITNESS` are all
> `merged`. **This node is dispatchable.**
>
> ⚠ **`scalar_kind`'s emitted-scalar-helper route is adjacent mechanism to what
> `D2` just landed**, in the same subsystem — which is why `§7 D2a` matters more
> than usual and why it is a hard stop rather than a judgment call.
>
> **The 12th row is no longer this node's known blocker.** The 07-29 failure
> `fs_write_at_malformed_offset_narrows_to_invalid_offset` was owned by
> [[RT-DECL-CLOSURE-PORT]]'s `AC-1`, root cause `authority=RecursiveDescent`.
> That node is **`merged`**, and `RecursiveDescent` now has **zero occurrences**
> in `crates/ken-runtime/src/`. ⛔ **That is the mechanism's absence, not a green
> row — nobody has run it since.** Measure it first; do not assume either
> outcome.

---

## 1. ⭐ Premise correction — there is ONE input ref, not two

Both node files say to "fold `c07e63c2` with `f0eb65ce`." **Measured: there is
nothing to fold.** `f0eb65ce` is `c07e63c2`'s parent:

```
c07e63c2  NATIVE-HANDLE-CARRIER: preserve arbitrary-precision body literals
f0eb65ce  WIP PX8-F-CAP-41: seal capacity-carrying buffer handle
8ebe370a  PX8-F-CAP-41 Phase 1 (§38 fold)   <- merge-base with main
```

⇒ **Of those two, take `c07e63c2` alone.** It already carries the
handle/admission impl *and* the elaborator slice. ⛔ Do not attempt a merge of
the two; you would be merging a commit with its own ancestor.

> ### SUPERSEDED 2026-08-17 — A LATER ATTEMPT MADE A THIRD REF, AND IT IS THE INPUT.
>
> **This correction settled which of the two 07-23 refs to take, and it is still
> right about those two.** It is no longer the answer to *"what do I rebase"*:
> the node was picked up again on 07-29, hit hard stop #21, and preserved a
> candidate that is **on a divergent lineage from both.**

| ref | sha | date | what |
|---|---|---|---|
| `origin/preserved/native-handle-carrier-hs21-85dcee25` | `85dcee25` | 07-29 16:08 | **THE INPUT** — completed `D1` rebase + `D2` arm, 11/12 |
| `origin/preserved/native-handle-carrier-hs21-8bc7556a` | `8bc7556a` | 07-29 13:54 | superseded WIP of the same attempt |
| `origin/preserved/native-handle-carrier-c07e63c2` | `c07e63c2` | 07-23 15:13 | the elaborator slice, now upstream of the input |
| `origin/preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce` | `f0eb65ce` | 07-23 14:43 | `c07e63c2`'s parent, informational |

⛔⛔ **ALL FOUR ARE ON DIVERGENT LINEAGES — verified, none of the `hs21` pair is
an ancestor or descendant of `c07e63c2`.** So preserving the newest does **not**
subsume the rest, and a `git log` on one tells you nothing about another. Read
the table as four separate artifacts, not a history.

⚠ All are `preserved/*` refs, **not** live WP branches. Cut
`wp/NATIVE-HANDLE-CARRIER` from **`85dcee25`**; leave every preserved ref
untouched. ⛔ **Do not reset, delete, or repoint `85dcee25`** — it is the only
copy of a clean rebase, and the hazard is a handoff-gate hard reset, not storage.

---

## 2. What is already GREEN, and what remains

**Done in `c07e63c2` (Foundation, `ken-elaborator` only — no `ken-runtime`):**
the driver's `MissingClosureMetadata` collapse was de-erased and the true root
cause fixed — checked-core `BigInt` literals were being narrowed to `i64`, and
the CAP-41 fixture reaches `u64::MAX` through the checked `intToUInt64` bound.
Body-view, computational-IH census, and erasure are **GREEN**. Interp half is
GREEN on all four CAP-41 rows.

> ### THIS RESIDUAL IS CLOSED — 2026-08-18. The primitive is in the tree.
>
> The paragraph below is the measurement this node was opened on, and it is
> **history**. `D2'` landed the identity arm: `int_to_uint64_raw` is live at
> `crates/ken-runtime/src/cranelift_backend/lowering/core/primitive.rs:206`,
> extending the `uint8_to_int | int_to_uint8_raw` arm, with a Big-carrier test
> at `primitive/tests.rs:323`. Four occurrences under `crates/ken-runtime/src/`
> at `36ecc162c`, where there were zero for this node's whole life.
>
> **Do not re-run the grep below as a check on current state** — it now returns
> hits, and reading it as the frame intends would invert its meaning.

**The residual, as measured at `origin/main = 5404108a` and now closed:** the
fixture failed only at object emission —

```
int_to_uint64_raw is not in the supported native set
```

`grep -rn 'int_to_uint64_raw' crates/ken-runtime/src/` returned **nothing** at
that base. The primitive was absent from the native lowering entirely.

> ✅ **RESIDUAL RE-MEASURED AND STILL TRUE at `origin/main = 06cb2964`**
> (Steward, 2026-07-29 — supersedes the `dca1b793` and `5404108a` measurements).
> `int_to_uint64_raw` is **still absent** from `crates/ken-runtime/src/` — zero
> occurrences — and on `main` it appears only in `crates/ken-interp/src/eval.rs`,
> the interp half this frame already records as **GREEN**.
>
> ⭐⭐ **This re-measurement is worth more than the two before it.** Between them
> the **entire `RT-NATIVE-FNSPLIT` arc landed and closed** — `RT-FNSPLIT-RECUR-PORT`
> plus the Scale nodes rewrote `crates/ken-runtime/src/cranelift_backend/`
> wholesale (`lowering/core.rs` **+3899/−1022**, `lowering/mod.rs` **+3654/−156**).
> ⇒ **A rewrite of exactly the subsystem that owes this primitive did not build
> it.** The node still has its subject; no re-framing pass is owed.
>
> ✅ **Both input refs still resolve on `origin`** (checked, not assumed):
> `preserved/native-handle-carrier-c07e63c2` → `c07e63c2`, and
> `preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce` → `f0eb65ce`.
>
> ⚠ **The one number that rotted:** `§3` says `main` is **215** commits ahead of
> `8ebe370a`; it is now **303**. ⛔ Do not re-pin that figure — the **derivation**
> is the pin (`git rev-list --count 8ebe370a..origin/main`) and it grows with
> every merge. It moved in the direction that makes `§3`'s argument *stronger*,
> not weaker: the rebase is more of a deliverable now, not less.
>
> ⛔ **`§3`'s churn table is now an UNDERSTATEMENT, and that matters.** It was
> measured when the collision was elaborator-only. The FNSPLIT arc has since
> rewritten the native lowering you must add the primitive to, so `§4`'s native
> arm lands in a **different mechanism** than the one it was written against.
> ⇒ Re-derive `§4` against current `main` at pickup. The *ruling* stands; the
> code it points at does not.

---

## 3. ⛔ The rebase is a deliverable, not a preliminary

`c07e63c2` is based at `8ebe370a`. **`origin/main` is 215 commits ahead of
that**, and the collision is not incidental:

| file | main's churn since `8ebe370a` | the branch's own churn |
|---|---|---|
| `crates/ken-elaborator/src/prelude.rs` | +100 | +115 |
| `crates/ken-elaborator/src/erasure.rs` | +99 | +43 |
| `crates/ken-elaborator/src/compiler_driver.rs` | +25 | +30 |
| `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs` | +37 | +2 |

⇒ **All three production files of the elaborator slice were also edited on
`main`.** This is a genuine three-way merge over the exact lines the slice
changes, not a fast-forward.

> ### THESE NUMBERS ARE FOR A REBASE ALREADY DONE. Do not run it again.
>
> **Measured 2026-08-17 at `origin/main = 2e7daa622`.** The table describes
> rebasing `c07e63c2`, which `§1` no longer asks you to do. Both figures below
> are derivations, **never pins** — re-run them at pickup:
>
> ```sh
> git rev-list --count <base>..origin/main
> git diff --numstat <base>..origin/main -- <the four files>
> ```
>
> | from | commits behind `main` | `prelude.rs` | `erasure.rs` | `compiler_driver.rs` |
> |---|---|---|---|---|
> | `8ebe370a` (`c07e63c2`'s base) | **1580** | +391/-13 | +1564/-45 | +810/-108 |
> | `af056a78` (`85dcee25`'s base) | **1262** | +300/-4 | +1488/-22 | +783/-104 |
>
> **`§3`'s argument is now far stronger than when it was written, and its stated
> stake is a 7x understatement:** taking a side wholesale would revert **1580**
> commits' worth of landed work, not 215. `erasure.rs` alone moved from +99 to
> **+1564**.
>
> ⇒ **That is the reason to start from `85dcee25`, not a reason to redo the
> merge.** It already resolved this collision once with `range-diff` 3/3 `=` and
> no side choice. **The hazard in this section stays fully live** for advancing
> that ref the remaining 1262 commits — re-derive each hunk, and `AC-1`'s control
> applies unchanged.

⭐ **The failure mode this AC exists to catch:** a rebase that resolves a
`prelude.rs` conflict by taking the branch side wholesale **silently reverts
215 commits' worth of landed work in that file**, and every targeted test still
passes because the reverted work has its own tests elsewhere. ⛔ Do not resolve
conflicts by side-preference. Re-derive each hunk.

**Control (`AC-1`):** after the rebase, `git merge-tree origin/main <your-sha>`
and confirm every blob that `main` advanced in those four files survives with an
OID that reflects **both** changes — ⛔ not the branch's pre-rebase OID.

---

## 4. The native arm — Architect-ruled, `evt_7xrcjp0apb4f1`

⛔ **Settled inputs. Do not re-litigate.**

`int_to_uint64_raw` is **value identity**, ⛔ **NOT** a machine `i64 -> u64`
conversion. Ken's fixed-width carriers share the exact `Int` runtime
representation. The native arm must:

- require exactly one `Lowered::Int` argument;
- return **that same `Lowered::Int` unchanged** — including the native-Int tag
  sidecar and payload/arena slot;
- preserve `18446744073709551615` as the existing **Big signed-magnitude**
  value;
- leave range admission to the derived checked `intToUInt64` wrapper, which
  proves `0 <= n <= u64::MAX` before calling the raw cast.

**Extend the existing identity arm**, currently at
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs:6827`:

```rust
"uint8_to_int" | "int_to_uint8_raw" => {
    let [value]: [Lowered; 1] = lowered_args.try_into().map_err(...)?;
    let Lowered::Int { .. } = value else { return Err(unsupported(...)) };
    Ok(value)
}
```

⛔ **A Cranelift integer cast or an `i64` fast path truncates, wraps, or retags
the Big arm.** That is the named failure mode, and it is invisible to any test
whose operand fits in `i64`.

---

## 5. ⭐ Scope holds at UInt64 — but know what you are standing next to

Measured: the interpreter treats the **entire** representation-sharing cast
family as identity in one arm (`crates/ken-interp/src/eval.rs:1355-1369`,
`=> a.clone()`) — **22 members**. Native implements **2**.

⇒ **Every other member is a latent instance of this exact wall**, and the
diagnostic staircase will surface them one at a time.

⛔ **Do not generalize in this WP.** The Architect ruled the family
generalization **optional** and not required for CAP-41 GREEN. Adding a
wildcard over primitive names would ship 20 untested arms behind one test.
⭐ Record the 2-of-22 count in `D5` so the next WP is framed against a measured
surface rather than rediscovering it.

---

## 6. ⚠ Diagnostic-staircase contingency

`int_to_uint64_raw` is **not asserted to be the final gap.** This fixture has
revealed a new wall at every layer:

```
MissingClosureMetadata -> int_lit_outside_native_i64 -> int_to_uint64_raw -> ?
```

Acceptance is **"full two-engine oracle GREEN"**, not "the primitive was added."
Any further native gap the exact fixture hits is **surfaced and triaged**, never
worked around.

> ### THE `?` IS RESOLVED, AND THIS SECTION'S PREDICTION HELD EXACTLY.
>
> **Step 4 arrived on 2026-08-17 and it was the predicted class.** The fixture
> hit a **non-primitive effect gap** — not a primitive one, exactly as the
> paragraph below forecast:
>
> ```
> ... -> int_to_uint64_raw -> effect seat Argument(0) of FsReadFile
>                             needs BytesPointerLength, unobservable in CarriedWord
> ```
>
> **It was surfaced, routed, and is owned elsewhere.** The Architect ruled the
> gap is not this node's (`evt_559gymspqap8w`); it belongs to
> [[RT-SITEOP-CARRIED-WITNESS]], whose `D2` is in flight. ⇒ **Do not re-derive
> this wall and do not route it to the Steward a second time** — see `§11`.
>
> ⚠ **This does not assert step 4 is the last.** The section's discipline is
> unchanged and still applies to a step 5.

⭐ The Architect enumerated the checked closure's primitives — `leq_int`,
`and_bool`, `int_to_uint64_raw`, `sub_int`, `eq_int`, `add_int` — and native
already handles all but one. `Some`/`None`, handle construction/projection, and
result branching are constructor/control lowering, **not** primitives. ⇒ Expect
no further *primitive* gap; the retained stop condition is for a
**non-primitive constructor/effect** gap. ⛔ Do not pre-inflate scope on the
contingency.

---

## 7. Deliverables

> ### `D1` AND `D2` ARE DONE ON `85dcee25`. RESTATED BELOW — 2026-08-17.
>
> The two originals are struck through, not deleted, because their **controls**
> (`AC-1`, `AC-2`) still bind on the restated form.

- ~~**`D1`** — `c07e63c2` rebased onto current `origin/main`.~~ **DONE on
  `85dcee25`**: `range-diff` 3/3 `=`, no conflict, no side choice.
  ⇒ **`D1'`** — advance `85dcee25` the remaining **1262** commits to current
  `origin/main`, hunk-by-hunk, with `§3`'s stale-base check run and reported.
  **`AC-1`'s control applies unchanged.**
- ~~**`D2`** — the `int_to_uint64_raw` identity arm in `core.rs`.~~ **DONE on
  `85dcee25` at `core.rs:9713` — but that file no longer holds the arm.**
  ⇒ **`D2'`** — **re-derive** the one-line arm at its new home,
  `lowering/core/primitive.rs:206`, extending the landed
  `uint8_to_int | int_to_uint8_raw` arm. The preserved hunk will not apply;
  [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] relocated it on 2026-08-17.
- **`D2a`** — **resolve the `scalar_kind` question before building `D2'`.**
  `primitive.rs:93-98` maps the same pair to `Some("Int")` and projects a carried
  word through the emitted scalar helper. Report whether
  `int_to_uint64_raw` needs an entry and whether that path can truncate the
  **Big** carrier. ⛔ **A truncation there survives `AC-2`'s control untouched**,
  so this is a hard stop to the Steward/Architect if the answer is not plainly
  no, not a call to make inside `D2'`.
  - ✅ **ANSWERED 2026-08-17: a plain no, and DISCHARGED** (`evt_s5bhmq0n4yad`).
    No `scalar_kind` entry was added. The real sites are **Specialized**, and
    `Big` is `PersistentGround` while the scalar helpers require
    `ImmediateInt` — so the carried word cannot reach that path with a `Big`
    payload. ⭐ **The `Big` fast-path mutation reddens and was reverted**, which
    is what makes this a measurement rather than a reading of the map.
> ### `D0'` IS NOW THE NEXT CUT, AND IT RUNS BEFORE `D3` — 2026-08-18.
>
> **The missing port landed.** [[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]] is
> `merged` (`D1` `5bac56000`, `D2` `ca639b5ef`), and `§2`'s residual — the
> absence of `int_to_uint64_raw` from `crates/ken-runtime/` — is closed by this
> node's own landed `D2'`. Verified in the tree at `36ecc162c`:
> `recursive_position_unit_body` walks plain-`Match` arms at
> `lowering/core.rs:15910` before route 1 at `:15919`, and the identity arm is
> live at `lowering/core/primitive.rs:206`.
>
> ⇒ **`D3`, `D4` and `D5` were all written against a tree that could not run
> them.** They are unchanged in substance and they are no longer the next
> action; the deciding measurement is, and each of them forks on its result.
> **Full statement of `D0'` is in the node**, `docs/program/issues/`.

- ~~**`D0'`** — restore the four `cap41_*` Rust rows and run them.~~ **RAN
  2026-08-18** (`evt_2kdscqgge6x2p`): 0 passed / 4 failed, all at
  `ObjectEmission` with *"recursive position is outside its source
  constructor"*. `AC-5`'s row 0/1 with the identical refusal at **both**
  `7b8dad7df` and the tip. The restoration is uncommitted on
  `wp/NATIVE-HANDLE-CARRIER-D0`, base/tip `86049d660`, `+59` lines, **not a
  merge candidate.**
  ⛔ **The three-outcome fork this deliverable carried was DEFECTIVE and it was
  the Steward's.** It forked on the refusal **string**, and that string is
  produced at `7b8dad7df` too — a tree whose resolver (`:15668`) has **no Match
  branch at all**, only `return Ok(None)` at `:15680`. So it cannot separate
  "the port did not reach this population" from "the port reached it and
  something downstream fails the same way". **Not a hard stop.**
  ⇒ **`D0''`** — **attribute the `Err` to its call site.** One run, instruments
  already in the tree: report `entered` / `route1` / `match_arms_walked` **per
  governed program**, plus the static origin of the call raising `core.rs:15924`.
  `match_arms_walked >= 1` means the port reached this population and the
  blocker is downstream; `route1 >= 1` with the counter at zero is the genuine
  hard stop; **neither recorder firing means the `Err` is an unrelated aborting
  call and `D0'` measured nothing about this node's subject.** Full statement in
  the node.
  ⛔ **`AC-5`'s reading is routed to the Architect, not acted on.** The row is
  red at the base, so the ban's *regresses-an-already-GREEN-row* warrant is
  refuted — but **the ban still binds** until the Architect lifts it.
- **`D3`** — the four focused discriminators of `§8` (`AC-3`), before the full
  oracle. **Gated on `D0'`.**
- **`D4`** — the CAP-41 fixture carried to **full native GREEN**, and the full
  two-engine oracle: all four CAP-41 rows absolute GREEN on **both** engines.
  **Gated on `D0'`** — if `D0'` comes back green this collapses into an oracle
  pass, and if it comes back red its content is decided by the new refusal.
- **`D5`** — the Architect's six-axis matrix (a)–(f) discharged, plus the
  2-of-22 family count from `§5` and any further staircase gap encountered.

---

## 8. Acceptance criteria

- **`AC-1`** ⭐ **(the rebase, and the one most likely to be got wrong)** — no
  landed work is reverted. **Control:** `git merge-tree origin/main <sha>` shows
  the four `§3` files carrying **both** main's and the branch's changes. ⛔ A
  blob OID equal to the branch's pre-rebase OID on any of them fails this AC.

- **`AC-2`** ⭐ **(load-bearing)** — the arm is **identity, not a cast**.
  **Control:** mutate the new arm to a Cranelift `i64` cast (or an `i64`
  fast path) and show the `u64::MAX` discriminator **reddens**. ⛔ A test whose
  operand fits in `i64` cannot distinguish identity from truncation and does not
  discharge this AC — the control must be on the **Big** carrier.

- **`AC-3`** — the four focused discriminators hold:
  1. `intToUInt64 u64::MAX` reaches `Some` natively, preserving the exact Big
     value **and tag**;
  2. `intToUInt64 (u64::MAX + 1)` and `intToUInt64 (-1)` reach `None` — proving
     the checked **wrapper**, not the raw arm, owns admission;
  3. the native arm and the interpreter agree on representation identity, with
     no wrap/truncation mutation surviving;
  4. existing `UInt8` conversion behavior is unchanged.

- **`AC-4`** — the six-axis matrix: (a) normalized checked declaration body
  view with the underlying error lane visible; (b) computational-IH
  census/metadata consistency; (c) erasure of handle construction, match, and
  projections; (d) runtime constructor/value lowering; (e) unchanged raw
  `Resource Buffer` host request and wire ABI; (f) constructor and both
  projections **absent from the public name map**.

  ⛔⛔ **`(c)` and `(f)` are ABSENCE claims and each owes a POSITIVE CONTROL.**
  Every other axis is a positive or consistency claim that witnesses itself; an
  absence claim **passes for any reason**, including an instrument pointed at a
  layer or a phase where the thing is never present. ⇒ For each of `(c)` and
  `(f)`, show the check **reds** when the name/construct **is** there: for `(f)`,
  export one of the three names (or aim the census at a symbol known to be
  public) and show it is reported; for `(c)`, read the same view **before** the
  erasure stage and show the constructs appear. ⛔ Reporting "not found" without
  that is not a measurement of the exported surface — it is a measurement that
  the instrument found nothing.

  ⚠ This is campaign **Trap 3**
  (`docs/program/16-recursive-descent-retirement.md`), which rejected an
  otherwise-sound `RT-JOIN-DISPOSITION` candidate on 2026-07-29: a proof
  quantified over a **recorded population** ran over an empty list and passed,
  and it was silent precisely because **every control over it passed**.

- **`AC-5`** — the regression that made an honest partial inadmissible is
  repaired. **Control:** `fs_read_at_malformed_offset_narrows_to_invalid_offset`
  — a pre-existing two-bracket native read row that was GREEN before the API
  migration and then failed pre-execution — is **GREEN again**.

- **`AC-6`** — no collateral regression. **Control:** the landed `BufferSpan`
  product stays GREEN; malformed / stale / closed authority behavior is
  unchanged.

- **`AC-7`** — targeted green. **Control:** name the exact
  `scripts/ken-cargo test -p <crate>` invocations and pass counts. ⚠ A full
  `-p ken-interp` run is required if the reifier or value shape changes
  (attested `eval.rs` ⇒ OID-bump rider). ⛔ No `--workspace`
  (`COORDINATION §12`); workspace-green means **green in CI**.

---

## 9. ⛔ Banned scope

- ⛔ **No `spec/` edit.** `38-ffi-io.md` is LOCKED and the representation is
  normatively settled — a token-only handle cannot supply the raw resource
  public consumers need and would reopen the authority/ABI boundary Phase 1
  closed.
- ⛔ **No `conformance/` edit.** The four CAP-41 seed rows are the oracle, not
  the deliverable.
- ⛔ **No family generalization** over the other 20 cast members (`§5`).
- ⛔ **No honest partial.** The Architect ruled this out explicitly: the
  candidate *regresses* an already-GREEN native row (`AC-5`), so interp-only is
  not a landable state.
  - ⚠ **UNDER REVIEW 2026-08-17 — the ban STILL BINDS until the Architect says
    otherwise; this note is not a licence.** Its warrant, in the node's
    "Decisive regression evidence" paragraph, is that `AC-5`'s row failed with
    `MissingClosureMetadata` — **a cause that is fixed.** The node's own
    2026-07-29 HELD banner then records that row **GREEN** on preserved
    `85dcee25`. ⇒ On the record the candidate does not regress it; the refusal
    arrived from `main` in the rebase. ⛔ **Separate the conclusion from the
    warrant** — a discharged warrant does not by itself discharge the ruling,
    and the ruling is the Architect's to revisit. **The deciding measurement is
    the `AC-5` row at base `7b8dad7df` versus at the tip**, and it has not been
    run.
- ⛔ **No concurrent `lowering/core.rs` edit** while `RT-NATIVE-FNSPLIT` is
  live.

---

## 10. Contention — ⚠ read this before pinning anything

⛔ **`crates/ken-runtime/src/cranelift_backend/lowering/core.rs` WILL MOVE.**
Its blob at `origin/main = 5404108a` is
`2da09df89df2bc4c0792df999da3cae96506ec5e` and the identity arm is at `:6827` —
**both are recorded as provenance, not as pins.** `RT-NATIVE-FNSPLIT` owns an
indivisible continuation-partitioning change to that exact file and lands first.

⇒ ⭐ **Re-derive the arm's location at pickup.** Do not search for `:6827`;
search for `"uint8_to_int" | "int_to_uint8_raw"`.

> ### IT MOVED, AND IT LEFT THE FILE. This section called it right — 2026-08-17.
>
> **`RT-NATIVE-FNSPLIT` is `merged`, so its serialization is spent.** But the arm
> then moved *again*, and out of `core.rs` entirely:
>
> | | at `origin/main = 2e7daa622` |
> |---|---|
> | the identity arm | **`lowering/core/primitive.rs:206`** |
> | the `scalar_kind` map (new; see `§7 D2a`) | **`lowering/core/primitive.rs:93-98`** |
> | the same pair in the IR evaluator | `runtime_ir_evaluator.rs:1632` — **a third site; decide whether it is in scope, do not edit it reflexively** |
> | `core.rs` blob | `e173b6fc7df3f18c638354f72f012d8ba67414ac` — **provenance only** |
> | who relocated it | [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]], merged 2026-08-17 |
>
> ⭐ **The prescribed search string is what saved this, and it still works** —
> `command grep -rn 'uint8_to_int|int_to_uint8_raw' crates/ken-runtime/src/`
> finds all three sites across the relocation. **Searching by content survived a
> whole-module move that any line pin would have failed.** Keep doing that.
>
> **Live contention is now `lowering/mod.rs`, not `core.rs`** —
> [[RT-SITEOP-CARRIED-WITNESS]] `D2` holds it. Re-derive slot availability
> against that node, not against `RT-NATIVE-FNSPLIT`.

⚠ Re-derive build-slot availability too. `ken-cargo` blocks silently for up to
30 minutes on lock contention; `fuser -v /tmp/ken-build-locks/build.lock` names
the holder — ⛔ don't pipe it through `head`.

---

## 11. Hard stop

⛔ Route to the Steward if:

- the rebase produces a conflict in `prelude.rs`/`erasure.rs`/
  `compiler_driver.rs` you cannot resolve without choosing a side — ⭐ say which
  hunk and what the two sides assert; **or**
- the fixture hits a **non-primitive** constructor/effect native gap (`§6`)
  — ⚠ **amended 2026-08-17: this bullet FIRED and its gap is DISCHARGED.** The
  effect-seat gap is [[RT-SITEOP-CARRIED-WITNESS]]'s and is in flight; ⛔ do not
  route *that* one again. The bullet stays live for **any further** such gap;
  **or**
  - ⇒ ⛔ **IT FIRED AGAIN 2026-08-17 12:54, on a DIFFERENT gap, and that is
    the node's current state.** All four CAP-41 rows and the directly-run
    `AC-5` row refuse at `lowering/core.rs:2929`
    `reject_carried_residual_arguments` — *"a carried recursive hypothesis is
    an eliminated value, not a callable, so it takes no arguments, but the call
    provides 1"*. Handback `evt_s5bhmq0n4yad`, routed `evt_64w7h59bd91y8` /
    `evt_jzd8wsxk74dz`. **The guard is `main`'s** (`feab3cb56 RT-FNSPLIT-C1`;
    `core.rs` is byte-identical base-to-tip), so this is inherited, not
    introduced. Disposition is the Architect's; see the node's leading banner;
    **or**
- `§7 D2a`'s `scalar_kind` question does not come back a plain no — a truncation
  path that `AC-2`'s control cannot see is not yours to judge safe
  — ✅ **DISCHARGED 2026-08-17: it came back a plain no, with a reverted
  reddening mutation as the control. This bullet is spent**; **or**
- identity lowering turns out to be unsound for the Big carrier on the native
  path — that reopens the Architect's means ruling and is not yours to re-decide;
  **or**
- `AC-5`'s pre-existing row cannot be restored, which would mean the elaborator
  slice itself regressed something.

---

## 12. What landing this closes

> ## ⛔ UNPAIRED 2026-08-17 — DO NOT FLIP [[PX8-F-CAP-41]] ON THIS MERGE
>
> **Architect ruling `evt_13ax2j6e0jfq2`; Steward disposition.** The paragraph
> below is superseded. The pairing rested on the honest-partial ban, and that ban
> is discharged by the `--ignored` differential (`evt_6h59tq0zpe7dn`): the `AC-5`
> row refuses **identically** at base `7b8dad7df` and at tip `3d23f1182`.
>
> ⇒ **The partial may land. Phase 2 closure may not ride on it.** The four CAP-41
> rows still refuse at `lowering/core.rs:2929`, the gap is a **missing port**
> owned outside this node, and flipping Phase 2 would be a claim about the past
> while the rows it names refuse.
>
> ⛔ **`AC-5` remains OUTSTANDING.** The ban being stale is not the criterion
> being met.
>
> ⇒ **Flip [[NATIVE-HANDLE-CARRIER]] only**, and only to the extent its claimed
> ACs are voted at the exact SHA. The Phase 2 scope call is recorded on that
> node's leading banner.

⭐ **This merge closes BOTH [[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]]
Phase 2.** They are one deliverable — the carrier fix is meaningless without the
fixture it unblocks, and the fixture cannot land without the fix. Flip both
nodes on the same merge.

⇒ `PX8` then has two blockers left: [[PX8-WROTE-ABS]] (Verify, released) and
[[PX8-ERRID-SCOPE]] (Verify, behind [[PX8-ERRID-ALLOC]]).
