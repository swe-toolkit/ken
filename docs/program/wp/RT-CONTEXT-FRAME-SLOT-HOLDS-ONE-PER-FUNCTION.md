# RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION — work package

**Owner: Team Runtime. Size M. Tier T1. Gate: none.**
**Implementation base: `origin/main` at `ef11485ddc0704f744c60d97fdc934537322418c`.**
**Cut fresh from `origin/main`; do not build on the original base.**

> **The base moved three times since the fixed inputs were measured and every
> cited coordinate survived all three.** §2 was measured at `89dc3b0e5`; all
> **nine** cited lines are byte-identical at `6acd40705` and again at
> `ef11485dd`, re-read line by line at each base rather than carried forward.
> The four rows' files are byte-identical too.
>
> `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` **has landed** (`ef11485dd`) and its
> landing diff touched **zero** files under `lowering/` — measured directly,
> not taken from its contention section.
>
> **Treat this line as dated, because it is.** A base SHA in prose is a claim
> with an expiry and no alarm on it. The check is two commands and it is yours
> to run at cut time, not to inherit:
>
>     git rev-parse origin/main
>     git show origin/main:<file> | sed -n '<line>p'

Operator priority, 2026-09-15: *"The other tests should be fixed."* Successor
to `RT-CARRIED-RESIDUAL-IH-ARITY`, which diagnosed this and was scoped to stop
at the diagnosis.

## 1. Objective

**Give `constructed_context_frame` a key, so one function can hold a frame per
worker body instead of only the last one written.** Readmit the four ignored
rows that fail because of it.

**This is the repair the diagnosis named. The diagnosis is not re-litigated** —
the arity refusal is correct at all four of its call sites and stays.

## 2. Fixed inputs, measured at `89dc3b0e5`

    crates/ken-runtime/src/cranelift_backend/lowering/

    mod.rs:1249    constructed_context_frame: Option<ConstructedContextFrame>
    mod.rs:1062    the struct's key discipline (quoted in §3)
    core.rs:10866  THE ONLY WRITE SITE
    core.rs:13577  READ -- admission query, the silent fallthrough
    calls.rs:986   READ -- operand gather, already matches the full triple

    sibling, same shape:
    mod.rs:1243    generated_context_captures: Option<GeneratedContextCaptures>
    units.rs:4028  its only write
    calls.rs:346   its only read, with the overclaiming message

> ### CENSUS CORRECTION: ONE OF THE `None` INITIALIZERS IS PRODUCTION CODE
>
> An earlier cut of this frame said *"six further occurrences are `None`
> test-fixture initializers"* and called the census an enumeration of the whole
> tree. **The count was right and the label was wrong.** `grep` returns 15
> occurrences of `constructed_context_frame`: 4 named above, 4 inside `#[ignore]`
> label strings, **6 test-fixture initializers, and one more that is neither.**
>
>     mod.rs:964     constructed_context_frame: None,    PRODUCTION
>
> **This is the per-function construction of `FunctionLocal`, and it is the site
> the repair actually changes** — `None` becomes an empty keyed container. An
> implementer given "six test-fixture initializers" looks in `core/tests/` and
> does not find it.
>
> **It also answers `AC-3` outright, so do not go looking for a new mechanism.**
> The field is initialized at `mod.rs:964` beside six sibling `BTreeMap::new()`
> fields under this comment:
>
> > *"Empty, never inherited. This is the map whose `ir::Value` keys alias
> > across functions; starting it empty per function is why the two structs are
> > separate types rather than one with a `reset()`."*
>
> ⇒ **Function-locality is already structural and already argued.** A keyed
> container placed at `mod.rs:964` inherits it exactly as `native_int_tags`
> does. `AC-3` is satisfied by *naming that construction site and showing the
> field is built there*, not by inventing a guard. **The failure `AC-3` is
> actually guarding against is a container hoisted OUT of `FunctionLocal`** —
> and that is visible as a diff that touches a different struct.

**The four rows** (from `RT-CARRIED-RESIDUAL-IH-ARITY`, landed at `762d24347`):

    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs

**Read their `#[ignore]` labels first.** They carry the measured cause and the
readmission condition. **One clause in them is wrong and §3 corrects it.**

## 3. THE KEY IS THE TRIPLE. GET THIS RIGHT BEFORE WRITING ANYTHING.

**The landed label says the rows readmit when the slot is *"keyed by
`worker_body_origin`"*. That key is insufficient and the struct's own doc
forbids it by name** (`mod.rs:1062-1066`):

> *"Keyed by the **complete planner-issued coordinate triple**, never by body
> origin alone. One function can reach two retargets over one body, and a frame
> consumed at the wrong one would be an arity-correct call carrying another
> occurrence's values."*

    THE KEY   (continuation_origin, recursive_position, worker_body_origin)

**`calls.rs:986` already filters on all three.** That is independent
confirmation, from the other consumer, that the triple is the identity — and
that the single-field match at `core.rs:13577` is the narrow one.

> ### A BODY-ORIGIN-ONLY FIX IS WORSE THAN THE BUG
>
> Today: a loud refusal that stops the compile. With a too-narrow key: an
> **arity-correct call carrying another occurrence's values**, which does not
> stop. **Do not trade a refusal for a silent miscompile.**
>
> **This is not a hypothetical, and the precedent is in this tree.**
> `mod.rs:4432` records that the pre-`D6a` code decided the continuation-input
> suffix by comparing `generated_context_captures.worker_body_origin` against
> the binding's `body_origin` — and that this was **"blind here by
> construction: the two bindings share a body origin, so a body-origin
> comparison answers the same for both."** That comparison was removed and
> replaced by a carried field. **A body-origin-only key is a defect this
> subsystem has already had, diagnosed, and paid to remove.**

## 4. Deliverables

1. **`constructed_context_frame` keyed by the full triple**, function-local.
2. **The admission query at `core.rs:13577` distinguishes its two cases.**
   *No frame stashed* and *a frame stashed under a different key* are different
   facts and currently produce one silent fallthrough. **Whatever the repair,
   they must be separable** — by the key lookup succeeding or failing, not by
   reading a message.
3. **The four rows readmitted and passing**, resolved by file and line.
4. **The `#[ignore]` labels removed** on readmission. If any row stays ignored,
   its label is rewritten with the measured reason — **not left naming this
   node while the node is closed.**
5. **`.github/ignored-test-exemptions.toml` rows** for any row readmitted with
   an accepted red (`class` + `readmission` + `test_path`).
6. **`AC-5`'s finding on the sibling field, reported either way.**

## 5. Acceptance criteria

**AC-1 — the key is the triple, and the frame says why.** State the key you
used beside the change. **If you use anything narrower, the deliverable is a
written argument against `mod.rs:1062`'s explicit instruction, not silence.**

**AC-2 — a NEGATIVE control on the key, and it must lead.** Construct the
two-body case and show that a frame stored under one key is **not** returned
for the other. **A test that only shows both bodies now succeed cannot
distinguish a correct key from a container that returns the first thing it
holds.** The control comes first and must fail before the repair.

> #### THE KEY HAS THREE COMPONENTS. A CONTROL THAT VARIES ONE TESTS ONE.
>
> **State the denominator beside the control.** "A frame stored under one key is
> not returned for the other" is satisfied by varying **any single** component,
> and a control that varies only `worker_body_origin` **passes against a
> body-origin-only key** — the precise key `§3` forbids. It would be a control
> defined by the defect's own condition.
>
>     continuation_origin     varied?   __
>     recursive_position      varied?   __
>     worker_body_origin      varied?   __
>
> **Vary each independently, or name the component you could not vary and say
> what stops you.** A component you cannot construct a difference for is an
> honest result and a reportable one; silently testing one third of the key is
> not.
>
> #### HOLD THE CARDINALITIES EQUAL OR THEY DO THE DISCRIMINATING
>
> The admission predicate at `core.rs:13578-82` is **not** body-origin alone —
> it is body-origin **plus two cardinality equalities**:
>
>     frame.worker_body_origin == body_origin
>     frame.worker_captures.len() == captures
>     frame.context_captures.len() == claims.len()
>
> ⇒ **If your two bodies differ in capture count, the cardinality check refuses
> the wrong frame and the control passes with the key untouched.** The control
> would be green, correct, and blind. **Construct the two bodies with EQUAL
> capture and claim counts**, so the only thing that can separate them is the
> key, and say in the report that you did.

**AC-3 — entries never cross functions.** `mod.rs:1245-1248`: the operands are
`ir::Value`s of this `Function`. **Say how the repair keeps the container
function-local**, and give the check that would catch a cross-function leak.
A global map is unsound here, not merely untidy.

**AC-4 — admission stays a PERMISSION, not the authority.** `core.rs:13570-76`
records the two-layer structure: the consumer re-matches on the full key and
re-checks both cardinalities, **so a frame admitted wrongly still refuses at
the call.** That backstop is why widening admission is safe. **Show it is
intact after the change** — if the repair collapses the two layers into one,
it has removed the reason the widening was safe.

**AC-5 — MEASURE the sibling, do not assume it.** `generated_context_captures`
has the identical single-slot shape and its consumer's message claims *"no
continuation-input suffix for any body"*, which a one-slot container cannot
establish. **Determine whether a live row reaches that path with a frame
stashed for a different body.**

**Read `mod.rs:4420-4440` FIRST — it is about this exact field and it changes
what you are looking for.** The sibling's body-origin comparison was already
found blind and taken out of the route decision; what remains at
`calls.rs:346` is a **residue** of a mechanism that was partly superseded, not
an untouched twin of `constructed_context_frame`. **That does not exonerate
it** — a residue can still be read on a path nobody re-examined — but it means
"same shape" is a claim about the container, not about the consumer, and the
consumer is where `AC-5` is decided.

    SAME DEFECT     a live row reaches it with a frame under another key.
                    REPORT IT; do not absorb it without coming back to the
                    Steward to re-size.

    REACHED, OK     a live row reaches it and the single slot is always
                    correct for that row. Say so, with what makes it safe
                    here and not at constructed_context_frame.

    NOT REACHED     no live row reaches the path at all. This is a DIFFERENT
                    answer from "reached and fine" -- it means the sibling is
                    UNMEASURED, not exonerated, and it must be reported that
                    way rather than as a clean bill.

**The message repair is owed in ALL THREE branches** — it is two lines and it
asserts a universal its container cannot support, whatever any row does.

> ### IF YOUR ANSWER IS NOT ON THIS LIST, THE LIST IS WRONG — SAY SO AND STOP.
>
> **Twice in this ledger-derived series a Steward outcome menu has been
> exhaustive over the wrong axis**, and both times the real answer sat in the
> gap: `RT-CARRIED-RESIDUAL-IH-ARITY`'s readmit/retire binary (the disposition
> was neither), and `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` §3's
> same/different-operation binary (both branches refuted by one run — see its
> §3a).
>
> **A menu that omits the outcome the work reaches pushes a good result toward
> mis-filing.** That is the frame author's defect, not the ring's. **Report the
> outcome you actually measured, name the branch it does not fit, and hand it
> back** — do not pick the nearest listed branch.

**AC-6 — no regression on `ken-runtime` and `ken-cli`.** **Report the delta's
three buckets, not the total:** red-on-both (pre-existing), green-on-main-red-
here (yours), and candidate-only with no `main` counterpart (**publish it even
at zero** — no subtraction shows it). Re-measure the baseline if `main` moves.

**AC-7 — the crate set is DERIVED to a fixpoint, not enumerated.** Compute the
transitive reverse-dependency closure of the touched crates from the manifests
and record it as derivation **output**. `cargo tree -i` closes transitively;
**omitting `--depth` is what makes it total**, and `-e normal,dev` is
load-bearing because the dev edges are in the test surface. **`cargo tree`
compiles nothing** — `COORDINATION §12` is not in play for the derivation.
**Never `--workspace` for builds.** **State the target selection beside any
green claim**: `cargo check` does not compile `#[cfg(test)]`.

## 6. What this node is NOT

- **Not a relaxation of the arity refusal.** `reject_carried_residual_arguments`
  is correct at all four call sites (`RT-CARRIED-RESIDUAL-IH-ARITY`, QA and
  Architect confirmed). The arity text is the fallback route's report; this
  repair stops control reaching that route.
- **Not a rework of `RT-CAPTURE-CONTEXT-FRAME-EMIT`'s design.** The write site
  and its placement argument stand. **Only the container's cardinality and key
  change.**
- **Not `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION`.** Same family of addressing
  defect, different subsystem, different rows, and that node's cause is not yet
  measured. **Do not fold.**
- **Not the other eleven ignored rows.**

## 7. Contention

`cranelift_backend/lowering/` — `ABI-S6-HS18-D5B-SUBSTRATE-PORT` is **parked**
(dead candidate; the recut is the Steward's and unstarted).
`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` **landed at `ef11485dd`** and is no
longer a contention question. Its landing diff was 3 files — two `ken-cli` test
annotations and its own frame — **zero under `lowering/`**, measured on the
diff rather than read off its contention section. **Re-measure at cut time
rather than inheriting this line:**

    git diff --name-only origin/main <candidate> | grep -c 'lowering/'
**`RT-CARRIED-RESIDUAL-IH-ARITY` is closed and touched only test annotations.**
No live contention at cut time. If this and node 2 run concurrently they are
separate branches and separate PRs.

## 8. Estimated tier: T1

**The key choice is soundness-adjacent.** A body-origin-only key produces an
arity-correct call carrying another occurrence's values — a silent miscompile
in place of a refusal. That is a judgment about identity in the lowering path,
not a transcription.

## 9. Sizing note

**M, and the reason is `AC-2` and `AC-5`, not the edit.** The container change
is small. Constructing the two-body negative control is the real work, and the
sibling measurement is a second consumer path. **If the negative control turns
out to be the whole turn, that is a complete turn** — post it with the key
argument and stop.
