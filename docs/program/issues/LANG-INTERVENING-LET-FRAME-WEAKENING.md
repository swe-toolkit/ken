---
id: LANG-INTERVENING-LET-FRAME-WEAKENING
title: "An intervening let between an outer match's premise and a nested match reaches install_index_refinements and dies in refine_branch_goal with 'could not classify the branch goal: TypeMismatch' -- and the Architect refused 'orthogonal', because the reported found term carries LANG-CONVOY's own D1 signature and there is an influence path through RVar resolution"
status: active
owner: language
size: S
gate: none
depends_on: [LANG-CONVOY-MATCH-FIELD-PROVENANCE]
blocks: []
github: null
origin: "language-implementer's bounded section 5 witness attempt (evt_4n7wdytrehs23), routed for separate ownership by language-leader and language-qa. The Architect made the three-way attribution measurement a REQUIRED follow-up of his approval evt_5b3c38r3xrqm6, owned by this filing rather than by a new SHA. Steward-filed per COORDINATION section 2."
---

> # `D1` IS A REGRESSION CHECK ON A JUST-MERGED NODE. RUN IT FIRST, ALONE.
>
> The Architect approved [[LANG-CONVOY-MATCH-FIELD-PROVENANCE]] at
> `dac4d16af7584b68adbcb0ed45109dbd146cf3ba` **with this measurement outstanding**,
> and stated the branch condition himself:
>
> | outcome of `D1` | what it means |
> |---|---|
> | the failure is **invariant across all three** runs | genuinely independent of the merged predicate; this node is a clean pre-existing gap and `D2` proceeds |
> | it **fails only under the region set** | **it is that node's acceptance regression on a merged node, and it returns to the Architect immediately** |
>
> ⇒ **Do not start `D2` before `D1` is reported.** The second row is the reason
> this node exists at all, and it is cheap to settle.
>
> ### THE TWO-ROW TABLE HAS A THIRD CASE, AND IT IS ALREADY MEASURED
>
> **Adversary hunt on `f08388396` (`evt_537ca9ady72kg`), run at `origin/main`,
> probe and guard mutation both reverted, worktree clean.** An interleaved-`let`
> variant — annotated `k : Vec Nat m`, binding `xs` directly — was run under
> both guards:
>
> | guard | plain zip | this interleaved `let` |
> |---|---|---|
> | shipped region set | `Ok(g583)` | `KernelRejected TypeMismatch ((Dg574 Dg67) @9)` vs `@4` |
> | prohibited floor `abs_pos >= 3` | `Ok(g583)` | `KernelRejected NotTerminating("SCT: idempotent self-loop has no strictly-decreasing parameter")` |
>
> ⇒ **It fails under BOTH, in different classes.** The Architect's dichotomy —
> invariant across three, or fails only under the region set — **does not have a
> row for that**, so do not force this result into one. Under the shipped guard
> it carries the predecessor's own pre-remedy signature: same head, one de Bruijn
> index apart. **That is the original defect still standing on an interleaved
> shape**, not a new one.
>
> **And it is a DIFFERENT failure from the one this node was filed on.** Yours
> dies in `refine_branch_goal`; this one gets past that and dies in the kernel.
> ⇒ **Either this node covers a different interleaving, or there are two distinct
> interleaved-`let` failures at different depths.** The Adversary explicitly did
> not reconcile them, and naming that is what makes it reconcilable.
>
> ⇒ **The question `D1` answers is no longer "is it independent?" but "which
> interleavings does the region set cover?"** Both variants are inputs to it.

> ## `D1` IS DELIVERED AND MERGED. `fe7be8386`, 2026-08-15. READ THIS BEFORE `D2`.
>
> **Result: invariant.** Byte-for-byte the same `ElabError::Internal`, same site
> (`refine_branch_goal`, `elab.rs:2913-2917`), same operands at **all three**
> points — base `43bd0d597`, the prohibited floor, and the shipped region set.
> ⇒ **A clean pre-existing gap. The predecessor's regression trigger did not
> fire and is closed.**
>
> **The landed pin is non-vacuous**, and that is what makes it a measurement: it
> requires the `Internal` variant, the `could not classify the branch goal` site,
> **and both exact operands** (`expected: Dg67`, `found: ((Dg574 Dg67) @8)`). A
> different failure cannot pass it. Expected `Nat` against found `Vec Nat @8` is
> a **sort** mismatch where an index was expected — the frame/weakening shape
> this node is named for.
>
> ### THREE THINGS `D2`/`D3` OWE THAT `D1` DID NOT COVER
>
> **1. CONVERT the landed test, do not delete it (Architect, blocking on `D2`).**
> When the repair lands this test reds and will read as the fixer's bug. **Turn
> it into the positive** — the same program asserted to elaborate. Deleting it
> erases the only durable record that the attribution was performed.
>
> **2. Note that `@8` is context-shape-dependent** (Architect, non-blocking). It
> is an absolute de Bruijn position, so an unrelated edit to `vec_env()` or the
> prelude reds this spuriously. **One line in the doc comment** saying a shifted
> index means *re-measure*, not *regression*. Fold into `D2`'s touch.
>
> **3. `D1`'s reconciliation half and `D4` were added to this node AFTER it was
> released, and the ring never saw them.** That is mine, not theirs — they
> delivered exactly what was published. **Both are still owed** and they are the
> same program: reconcile the Adversary's variant against the filed repro, and
> land the discriminating fixture. **The two variants are now known to differ** —
> the Adversary's dies in the kernel under both guards in *different classes*,
> this one dies in `refine_branch_goal` *identically* under all three. ⇒ **There
> are two distinct interleaved-`let` failures, not one**, and `D2` should say
> which it is localizing.

## Why "orthogonal" was refused, and it is not a formality

The implementer reported the failure as *"apparently unrelated"* and hedged it
correctly. The **disposition** rested on that orthogonality, so the Architect
tested it and found the argument insufficient in a specific way worth carrying:

`refine_branch_goal` (`elab.rs:2880-2942`) reads **neither** `match_field_regions`
**nor** `var_refinements` — no site for either falls in that range. **That is a
function-local argument, and the property needed is reachability.** The influence
path he traced:

> region skip → fewer entries in `var_refinements` → **which term an `RVar`
> resolves to changes** (`elab.rs:3403` — a `Cast`-wrapped alias versus the bare
> `Var`) → that term flows into a nested match's scrutinee/indices → **which are
> arguments to `refine_branch_goal`.**

Capability 2 inserts into the map only (`:3020`, `:3080`) and never into
`cx.ctx`, so context **types** are untouched; the path runs through
*resolution*, not through the context. Narrow, but real.

**And the prior is wrong-signed.** The reported `found` term is
`((Dg574 Dg67) @N)` — the same shape as the predecessor's own `D1` signature,
`((Dg574 Dg67) @9)` versus `@4`. That is evidence of the **same defect family**
(an index/frame disagreement reached by a different path), not of an unrelated
gap. **Frame the investigation on that prior, not on independence.**

## Fixed inputs

| input | pin |
|---|---|
| the program | a fresh `let k` bound to a `Vec Nat n` value (**not** an already-refined alias), interleaved between an outer match's premise computation and a nested match, consumed by a further nested match or the recursive call |
| the error | `index refinement: could not classify the branch goal: TypeMismatch`, raised at `crates/ken-elaborator/src/elab.rs:2913-2917` |
| the observation | a temporary `try_reindex_cast` operand trace showed `k`'s weakened raw type and the middle match's `b2` **disagreeing on which absolute position they name** |
| the three bases | `43bd0d597` (predecessor's merge-base), the floor mutation `if abs_pos >= 3`, and the shipped region set |
| `try_reindex_cast` | `elab.rs:2830` — returns `Ok(None)` when `subst_term_generalize(cur_ty, old_idx, new_idx) == cur_ty`, i.e. when `cur_ty` does not depend on `old_idx` at all |

**Re-derive at your candidate base.** The predecessor moved `elab.rs`; these line
numbers are from its review, not from your tree.

## Deliverables

**`D1` — the three-way attribution, and nothing else until it is reported.**
Run the failing program at each of the three bases above. Report the exact error
(or its absence) for each, with the run.

**`D1` now has a second half, and it is one read: reconcile the two variants.**
Put your repro beside the Adversary's (`k : Vec Nat m` bound to `xs` directly,
between the outer arm and the nested match) and say whether they are the same
shape. **That decides whether this node has one failure or two**, and it is the
one thing the hunt explicitly did not do. If they differ, say what differs —
the annotation, the bound term, or the position — because that is the variable
the coverage question turns on.

> ### `D4` MERGED 2026-08-15 at `7b11bbd84`, PR #2301, exact `956d86921`.
> ### `D1` LANDED EARLIER at `beb31566b` (PR #2282, 03:49:57Z). NODE STAYS
> ### `active` — `D2` and `D3` are deferred, not done.
>
> `dec_7r12dsg9py2a4` resolved 08:31:56Z; QA `evt_7qxcwvx2vnrnx`, Architect
> `evt_41nczz7c68t5y`. Declared base `a737d8c9b`; **one path enumerated from
> it, blob MATCH** (`e0f71ee19`), `+90/-0`, no production, spec or conformance
> delta. **Two non-blocking Architect findings carry to `D2`** — recorded in
> full further down; neither is a reason to recut, and the first discharges a
> standing note of his.
>
> **`D1` had already merged when its publish was requested**, and that is worth
> keeping rather than filing away. `fe7be838` is not an ancestor of `main` and
> never will be — **a squash rewrites the commit, so a landed branch head reads
> as owed forever.** Both the leader's and QA's statuses said "awaiting merge
> routing" while the content sat in `main`, and two seats reading the same
> unlanded-looking head is not corroboration. **The blob test settles it in one
> command and belongs before the publish, not after:**
> `git diff --quiet origin/main <head> -- <declared paths>`.

**`D4` — the discriminating fixture, folded in here rather than filed
separately.** The Adversary's variant is the first program measured to
distinguish the shipped region set from the prohibited floor, and
[[LANG-CONVOY-MATCH-FIELD-PROVENANCE]]'s `AC-1` gap is exactly one such fixture
wide. **Land it asserting that the two guards' rejection CLASSES differ** —
type-index versus termination — not that either succeeds. Neither does.

> **Do not assert "the region set is correct here".** Both guards reject this
> program. The claim the fixture can carry is that they fail differently and
> that the floor's failure reaches the termination gate. **A fixture asserting
> more than was measured is how this node's predecessor got recut twice.**

**`D5` — one character on the landed `D1` pin. Take it with `D4`; same file.**

**Adversary hunt on `beb31566b` (`evt_1xk8hj29qn2js`), re-checked against
`origin/main` by the Steward.** The `D1` assertions are asymmetric, and only
one of them is anchored:

| line | assertion | anchored? |
|---|---|---|
| `:514` | `msg.contains("found: ((Dg574 Dg67) @8)")` | **yes** — the trailing `)` means `@80)` cannot match |
| `:507` | `msg.contains("expected: Dg67")` | **NO** — also matches `expected: Dg670,` … `Dg679,` |

The message is `format!("… could not classify the branch goal: {e:?}")` over a
`KernelError`, so the render is `TypeMismatch { expected: <term>, found: <term> }`
— **there is a comma after `expected`'s value.** ⇒ **`"expected: Dg67,"`.**

> ### THE TELL WAS THE ASYMMETRY INSIDE ONE TEST, NOT THE VALUES
>
> `found` is anchored **by accident** — it happens to end on `)`. Nobody chose
> it. ⇒ **When two assertions of the same kind sit side by side and one is
> tighter, the loose one is the defect, and you can see it without knowing
> anything about the domain.**
>
> ### AND IT COMPOSES WITH THE `@8` HAZARD INTO ONE EVENT WITH TWO SIGNS
>
> The recorded hazard is that prelude growth shifts `@8` and reds `found`
> spuriously. **The same growth mints the four-digit ids that make
> `expected: Dg67` match wrongly.** One ordinary prelude edit therefore produces
> **a loud spurious red and a silent false green at the same time.**
>
> ⇒ **The loud half gets investigated and the silent half does not** — and a
> maintainer re-measuring after the red would be reading an `expected`
> assertion that had quietly stopped discriminating. **Anchoring the comma
> removes the silent half and leaves only the hazard already documented.**
>
> **The site pin is sound and needs nothing.** `msg.contains("could not classify
> the branch goal")` is a text match, not a location, so it holds only if the
> text is site-unique — and it is: **one production emitter** (`elab.rs:2915`),
> the only other occurrence in `crates/` being the assertion itself.

**`D2` — conditional on `D1` reading "invariant".** Locate the disagreement: two
things name an absolute position and do not agree on it. Say which two, and
which one is wrong. **A diagnosis is the deliverable; the repair is sized after
it.**

**`D3` — the reachability question, answered rather than assumed.** The
Architect's influence path is a *possible* route, traced by reading. Establish
whether it is the **actual** route here — if `RVar` resolution is not involved
in this failure, say so with the evidence, because that materially narrows `D2`.

> ### `D2` CARRIES TWO ARCHITECT FINDINGS FROM THE `D4` APPROVAL
> ### `evt_41nczz7c68t5y` / `dec_7r12dsg9py2a4`, both non-blocking, recorded
> ### here because `D2` is deferred and a finding left in a thread is lost.
>
> **Neither is a reason to hold or recut anything**, and the Architect said so
> explicitly: nothing in either can change program behaviour. **Both are `D2`
> work, and the first one discharges a standing note of his.**
>
> **Finding 1 — the positional literals add nothing to the discrimination they
> appear to make.** `D4`'s control separates the shipped region set from the
> prohibited positional floor **entirely by error class** — floor gives
> `NotTerminating`, which lands in the `other =>` arm. The `contains("@9")` and
> `contains("@4")` assertions contribute **no** discriminating power on top of
> that, while carrying full exposure to any unrelated binder-structure shift.
> **And `contains("@4")` also matches `@40` and `@43`** — so the literal is
> simultaneously **too strong against position and too weak against
> neighbours.**
>
> **The positive form is already written in this same file**, in the convoy
> `D1` doc: *"expected and found were the SAME HEAD, differing only in the
> trailing de Bruijn index."* ⇒ **Convert BOTH pins in one `D2` edit** —
> `D4`'s `@9`/`@4` and `D1`'s `@8`: assert the shared head, assert the two
> renderings differ, and **keep the literals in the doc comment as the measured
> instance** rather than as assertions.
>
> **Finding 2 — the fixture inherits an index-impossibility dependency without
> its warning.** The inner match `w` has **one arm against `Vec`'s two
> constructors**, and elaborates only while `VNil` is index-impossible at
> `Vec Nat (S m)` (`34 §4.3`). **This file already documents, for the sibling
> fixture, that a regression there surfaces as `ExhaustivenessError`** — a
> different route entirely. This fixture carries the dependency and not the
> warning, and its `other =>` message says a different error means
> *"guard-dependent behaviour needs re-measuring"*, **which would misdirect the
> next reader straight past the real cause.** Name the alternative route in the
> message, or restate the conjunction.

> ### `D2` IS WIDENED TO ALL FOUR OPERAND ASSERTIONS IN THE FILE, INCLUDING
> ### `D1`'s. Adversary hunt `evt_399y8ys1ftnee`, ACCEPTED. Steward, 2026-08-15.
>
> **Finding 1 above named `contains("@4")`. It is one of three unanchored
> substrings in this file, and the third one sits on the predecessor's node.**
> Re-verified against the tree at `7b11bbd84`, not taken from the report:
>
> | line | assertion | anchored |
> |---|---|---|
> | `:507` | `contains("expected: Dg67")` | no — also matches `Dg670` |
> | `:514` | `contains("found: ((Dg574 Dg67) @8)")` | yes, by the trailing `)` |
> | `:599` | `contains("@9")` | no — also matches `@90`, `@93` |
> | `:604` | `contains("@4")` | no — also matches `@40`, `@43` |
>
> **`D4`'s hunk is `@@ -527,0 +528,90 @@`**, so `:599` and `:604` are `D4`'s and
> `:507`/`:514` are `D1`'s, landed at `beb31566b`. ⇒ **One file-level anchoring
> convention was being split across two nodes**, with `D2` holding two of the
> three live sites. **`D2` takes all four.** Leaving `:507` for whoever reopens
> `D1` means it is repaired by nobody, since `D1` is merged and closed to work.
>
> **The two findings do NOT stack, and applying both to one site is the error to
> avoid.** Finding 1's remedy **deletes** `:599` and `:604` — the error class
> already discriminates, so an anchored `"@9)"` is a brittle literal that still
> buys nothing. **Anchoring is the live repair only at `:507`**, where `Dg67` is
> a **name**, not a positional level, and Finding 1's argument does not reach it.
> So: convert `:514`/`:599`/`:604` per Finding 1; anchor `:507` per this one.
>
> **Two things the hunt confirms, recorded so they are not re-litigated.**
> First, the obvious criticism of this fixture — that the prohibited-floor arm
> is prose and therefore not a control — **does not land**: the two outcomes are
> disjoint error classes (`TypeMismatch` vs `NotTerminating`), so a floor
> reintroduction fails the fixture's own `match` arm. **A one-armed control over
> two disjoint error classes is a complete control**; do not "complete" it.
> Second, Finding 2's index-impossibility dependency has **three** dependent
> artifacts in this file with the warning written once (`evt_1e17vjqkdsg13`) ⇒
> state it as a file-level property, not a per-fixture comment.

## Acceptance criteria

**`AC-1`.** `D1` is reported with three runs and three results. **A `D1` that
reasons about what the three bases would do fails this** — that is precisely the
error (asserting a mechanism's behaviour from a reading) that produced the
predecessor's recut, twice.

**`AC-2`.** No repair lands before `D2`'s diagnosis is stated. A fix whose
rationale is "this makes the error go away" is not accepted here.

**`AC-3`.** The predecessor's shipped region-stack mechanism is **unchanged**
unless `D1` fires the second row. If it does, stop and return to the Architect —
**do not repair a merged node's regression on the ring's authority.**

**`AC-4`.** No widening to other `install_index_refinements` consumers beyond
what `D2`'s diagnosis names. The predecessor banned that scope and the ban is
what kept this finding honest.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

**`AC-6`.** `D2` leaves **all four** operand assertions in
`ds5b_dependent_match_refinement_acceptance.rs` correct by the table above —
`:507` anchored, `:514`/`:599`/`:604` converted to the head-plus-difference
form. **Three of four is a fail**, and so is anchoring a literal that Finding 1
says to delete. **This does not breach `AC-4`:** that ban is on widening to
other `install_index_refinements` **consumers**, and this is one test file's
assertion convention with no production reach.

## Why this earns a slot

**It is the closeout of a merge, not new work.** The Architect approved on
soundness with this measurement explicitly deferred to a filed node, and his
reason for not blocking was that the measurement is cheap and the predecessor's
frame forbade the repair. **Leaving it unfiled would convert "cheap and
deferred" into "never run."**

**A finding that lives only in the prose of a test about something else is lost
the first time someone greps for it** — his words, and the reason this is a node
rather than a doc comment.
