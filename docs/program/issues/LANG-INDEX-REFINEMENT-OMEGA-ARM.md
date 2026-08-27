---
id: LANG-INDEX-REFINEMENT-OMEGA-ARM
title: "Make dependent-match index refinement sort-general: of five classifier decisions in elab.rs, widen the re-indexed-position helper, the branch goal, and the hidden-result outer-binding prefilter to Type union Omega, transporting Omega-classified types by a direct J arm alongside the existing Type-plus-Cast arm, while both index-type classifiers stay Type-only"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-08-27, discharging the predecessor the Architect assigned across TWO CUMULATIVE rulings: the detailed mechanism ruling evt_1wnk1ek4s8sgj and the concise status clarification evt_pw69nxgxn99j (thread thr_1czkntcjrvvz9). The first is why the outer prefilter, the real-consumer controls, and the direction mutation exist; the second confirms the route is viable and no additional discriminator is needed. NEITHER SUPERSEDES THE OTHER. V3-FO-EMBEDDING-ADEQUACY D2 hard-stopped immediately on release (implementer hard stop evt_5fxgv9eeqm68f, leader route evt_74q124wnb3zaf): eliminating the proof-indexed FokDerivation with index-dependent Omega evidence fails elaboration with ElabError::Internal(\"index refinement: ... not classified by a Type universe, found Omega0\"). The Steward routed it to the Architect (evt_nrvb2atg0xay) without diagnosing it. The first cut of this frame was BLOCKED by the Architect (evt_367papv4k57kk) for a three-decision census that missed the hidden-result outer-binding prefilter, for citing the two rulings as replacement rather than cumulative provenance, for swapped hard-stop authorship, for dropping the ruled real-consumer and direction controls, and for discharging a byte-identity claim with suite greenness. This is the recut. All fixed inputs measured at origin/main 6a37b92c7ce02edf0e73be7306776253ca68e8c4."
---

> # WHAT THE ARCHITECT RULED — evt_1wnk1ek4s8sgj + evt_pw69nxgxn99j, CUMULATIVE
>
> **Read both. Neither replaces the other.** `evt_1wnk1ek4s8sgj` is the detailed
> mechanism ruling and is the source of the outer-prefilter requirement, the
> real-consumer controls, and the direction mutation. `evt_pw69nxgxn99j` is a
> concise status clarification confirming the route. A frame citing only the
> second loses three requirements — that is what got the first cut blocked.
>
> Three arms were on the table when the hard stop was routed. The rulings
> **reject two and do not need the third**:
>
> - **Not prohibited elimination out of Omega.** D2 first eliminates
>   `‖FokDerivation s‖` into the Omega-valued denotation; its method receives
>   `d : FokDerivation s : Type` and eliminates that Type-inductive into an
>   Omega motive. Both steps are permitted (K4 permits eliminating INTO an
>   Omega motive). The Steward's `16 §1.4` flag was the right check to run and
>   it comes back clean.
> - **Not a re-representation fork and not a TCB-capability question.**
>   Existing kernel `J` already transports index-dependent Omega evidence and
>   kernel-checks the result. No new kernel capability is needed, so no
>   operator TCB call arises.
> - **No additional discriminator is needed.** The evidence object already
>   isolates the defect.
>
> **The defect is the elaborator's repeated Type-only assumption in index
> refinement.** Both re-indexed constructor positions and branch-goal
> restoration must classify over `Type ∪ Ω` — and so must the hidden-result
> outer-binding prefilter that decides which positions ever reach them.
>
> **Explicitly NOT authorized by this node:** any kernel change, any trust
> change, any `FokDerivation` change, any change to V3-FO D1, and any change to
> the released structural premise or to `fok_classically_valid`'s validity
> statement. `FoKripke.ken` stays byte-untouched. This node is elaborator-only
> and sort-general — it is not about FO. FO is only the consumer that exposed
> it.

> # WHAT IS HELD BEHIND THIS NODE
>
> The Language ring stays held on `V3-FO-EMBEDDING-ADEQUACY` D2 until **both**
> increments of this predecessor land through publisher CI **and the Steward
> explicitly re-releases FO**. Landing this node authorizes nothing downstream
> on its own.
>
> Evidence commit `3f687a460f4399bd1204a03ca8cbb57cad75eb92` (tree
> `15f2d977e`, one test path, `+126`, blob `c9eefe4e5`, 2/2 passing;
> independently reproduced by the Architect) remains **held transition
> evidence, not a D2 candidate**. Its `Probe` is a fixed input to this node
> (AC-PROBE), not a deliverable of it.

## Fixed inputs

Measured at exact `origin/main 6a37b92c7ce02edf0e73be7306776253ca68e8c4`,
tree `27ba416f97336f302be67930a9241526d2992f96`.

| path | blob | role |
|---|---|---|
| `crates/ken-elaborator/src/elab.rs` | `e43be39f51ede05335170e934009aa74d196600e` | the whole change surface (10079 lines) |
| `crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs` | `db992f637717d9e65f5a5830c5a5730f5cbe06e8` | the owning acceptance suite (622 lines) |

### The census is FIVE classifier decisions, not three

| # | decision | lines | classifies | disposition |
|---|---|---|---|---|
| 1 | `try_reindex_cast` | 3234-3245 | a re-indexed position's type `cur_ty` | **WIDENS** |
| 2 | `refine_branch_goal` | 3304-3315 | the refined branch goal `candidate` | **WIDENS** |
| 3 | hidden-result outer-binding **prefilter** | 3392-3405 | each outer binding's type, before `try_reindex_cast` at 3406 | **WIDENS, and delegates** |
| 4 | hidden-result matched/index type | 3360-3372 | `index_ty` in `install_hidden_result_variable_refinements` | **stays Type-only** |
| 5 | capability-2 sibling-convoy index type | 3504-3515 | the index type `peel_ty` | **stays Type-only** |

**Decision 3 is why "all four `try_reindex_cast` call sites route through the
one function" is behaviorally FALSE**, and the first cut of this frame asserted
exactly that. `install_hidden_result_variable_refinements` runs

```rust
if !matches!(whnf(/* classifier of outer_ty */), Term::Type(_)) {
    continue;
}
```

at `elab.rs:3392-3405`, immediately before the `try_reindex_cast` call at 3406.
A lawful Omega-classified outer binding is **silently skipped** there, so
widening the central helper alone leaves this caller's positions unrefined with
no error and no diagnostic. Widen the prefilter to an explicit `Type | Omega`
admission and let the central helper make the arm choice; do not duplicate the
arm logic in the caller.

**Neither index-type check widens.** Decisions 4 and 5 classify an *index*
type, and an index inhabits a Type. They keep failing closed, and
`AC-NO-INDEX-WIDENING` pins that both still fire.

Supporting coordinates, all unchanged by this node unless stated:

- `build_index_type_cong` (`elab.rs:3171`) — builds
  `e : Eq (Type l) cur_ty new_ty` by `J` and returns `(e, new_ty)`. It is the
  **Type arm** and stays exactly as it is.
- `try_reindex_cast` call sites: `elab.rs:1670`, `3406`, `3482`, `3542`.
- `refine_branch_goal` call site: `elab.rs:2244`; its restoration loop is
  `elab.rs:2260-2262`, which currently applies each `(src, tgt, e)` triple in
  reverse as `Term::Cast`.
- `Term::Omega(Level)` (`crates/ken-kernel/src/term.rs:237`), constructor
  `Term::omega` (`term.rs:362`), `Debug` renders as `Ω{level}` (`term.rs:454`)
  — which is why the observed hard stop reads `found Ω0`.

## Objective

Dependent-match index refinement currently assumes every type it re-indexes is
classified by a `Type` universe. Where that assumption holds it builds a type
equation `e : Eq (Type l) cur_ty cur_ty[new/old]` and wraps the value in a
`Cast`. Where the type is Omega-classified it either raises
`ElabError::Internal` (decisions 1 and 2) or silently skips the position
(decision 3).

Make the refinement **sort-general**: classify over `Type ∪ Ω` at decisions 1,
2 and 3, keep the existing Type-plus-`Cast` arm byte-identical, and add a
direct-`J` arm for Omega. Any other classifier still fails closed.

## Authorized mechanism

### 1. The Omega arm, verbatim from the ruling

For an Omega-classified `cur_ty` at level `l`, do **not** build a type equation
and cast. Transport the inhabitant directly with `J` under the motive

```text
λ y (_ : Eq index_ty old_idx y). cur_ty[y/old_idx]
```

ascribed at

```text
Π (y : index_ty). Π (_ : Eq index_ty old_idx y). Ω l
```

based at the value itself (`base = value : cur_ty`, since `cur_ty[old/old]`
is `cur_ty`), applied to `h : Eq index_ty old_idx new_idx`. `J` then yields
the transported inhabitant at `cur_ty[new_idx/old_idx]` with no `Cast` in the
result.

The motive is a bare `Lam` and `infer_j` calls `infer` on it directly, so the
ascription is mandatory, exactly as `build_index_type_cong` and `build_sym`
already document.

**Orientation is load-bearing and is not proved by arm selection.** The
old/new generalization direction must be the one the ruling states; the
direct-`J` prototype's reversed direction already reddened the used-evidence
positive. AC-DIRECTION is the control.

### 2. Decision 1 — re-indexed positions (`try_reindex_cast`)

Replace the `match whnf(...) { Term::Type(level) => level, other => Err }` with
a three-way classification:

- `Term::Type(l)` — unchanged: call `build_index_type_cong` and wrap in
  `Term::Cast`. The emitted core must be byte-identical to today's.
- `Term::Omega(l)` — build the direct-`J` transport of §1 and return it as the
  refined value. There is no `Cast` node on this path.
- anything else — `ElabError::Internal`, naming the actual classifier.

Return type is unchanged (`Option<(Term, Term)>`): the pair is still
(refined value, refined type). Only how the value is built differs.

### 3. Decision 3 — the hidden-result outer-binding prefilter

Widen `elab.rs:3392-3405` from `matches!(..., Term::Type(_))` to an explicit
`Term::Type(_) | Term::Omega(_)` admission, then delegate to `try_reindex_cast`
as it already does. Do not inline the arm choice here.

**What happens to a classifier outside `Type | Omega` at this decision is NOT
settled by this section** — it is an open question the increment must resolve
with evidence under **AC-DECISION-3-DEFAULT**. Preserving today's silent
`continue` is the conservative default and the starting assumption, but it is
an assumption, not a ruling: this frame does not authorize converting the
existing skip into a new error, and it does not authorize widening past
`Type | Omega` either. Resolve which, and say so.

**The requirement is that a lawful Omega position stops being silently
skipped.** AC-PREFILTER measures that with a witness, not by reading the
widened `matches!`.

### 4. Decision 2 — branch-goal restoration (`refine_branch_goal`)

Same three-way classification on `candidate`'s classifier. The consequence is
larger here because the restoration is deferred: `refine_branch_goal` returns
`casts: Vec<(Term, Term, Term)>` and the caller replays them in reverse as
`Term::Cast` at `elab.rs:2260-2262`. An untagged triple cannot express which
arm restored it.

**Producer-time classification must survive to the consumer as a private
Type-Cast/Omega-J restoration variant** — one entry per refinement step,
tagged, carrying exactly the ingredients its own arm needs. The caller
dispatches on the tag and applies the matching form, still innermost-first.
Re-classifying an untagged triple at replay time duplicates authority and is
explicitly not intended (Architect, call 2).

### 5. Fail-closed discipline

Every classifier that is neither `Type` nor `Omega` continues to raise
`ElabError::Internal` at decisions 1 and 2. Do not widen to a `_ =>` accept,
and do not widen decisions 4 or 5.

### 6. What produces the soundness

Nothing here is postulated. The Omega arm's result is an ordinary kernel `J`
application, so the branch's core term is kernel-checkable exactly as the Type
arm's is. AC-CORE-KERNEL-CHECKS is the gate that this actually holds rather
than merely elaborating.

## Deliverables

**Two independently landable increments. Each lands with its OWN evidence —
D1's controls may NOT be deferred to D2 or to a later shared test deliverable**
(Architect, call 3). Size each for the one-hour turn target.

> **THE COMPLETE ACCEPTANCE-CRITERIA SET BELOW BINDS BOTH INCREMENTS.** Every
> AC applies to each separately landing candidate, including the global scope
> constraints AC-KERNEL-UNCHANGED, AC-BLAST-RADIUS and AC-NO-REGRESSION. The
> per-increment lists name the **mechanism-dependent local discharges** that
> increment must produce on top of that; they are **not** a licence to omit a
> global AC, and an AC absent from a list is still binding. An AC that names
> per-decision halves is owned by whichever increment changes that decision.

- **D1 — decisions 1 and 3.** `try_reindex_cast` classifies over `Type ∪ Ω`
  with the direct-`J` Omega arm; the hidden-result outer-binding prefilter
  admits `Type | Omega` and delegates. **Local discharges:**
  AC-OMEGA-REINDEX-POSITION; AC-PREFILTER; AC-DECISION-3-DEFAULT;
  AC-J-MOTIVE-EXACT for the decision-1 constructor; AC-FAIL-CLOSED **decision-1
  half**; AC-MUTATION classes **(a), (b) and (c)** — all three are D1
  properties, since they inject at `try_reindex_cast`, at its motive
  ascription, and at the prefilter; AC-DIRECTION; AC-CORE-KERNEL-CHECKS;
  AC-TYPE-ARM-UNCHANGED; AC-NO-INDEX-WIDENING; AC-REAL-CONSUMER.
- **D2 — decision 2.** `refine_branch_goal` classifies over `Type ∪ Ω`; the
  returned restoration becomes a tagged Type-Cast/Omega-J plan and
  `elab.rs:2244`'s caller dispatches on it. **Local discharges:**
  AC-OMEGA-BRANCH-GOAL; AC-PLAN-TYPED; AC-FAIL-CLOSED **decision-2 half**;
  AC-D2-MOTIVE-PROVENANCE (which supplies D2's own exact-motive discharge and
  its own collapse / drop-ascription mutations, or proves reuse instead);
  AC-PROBE; and its own re-run of AC-CORE-KERNEL-CHECKS,
  AC-TYPE-ARM-UNCHANGED and AC-DIRECTION.

Tests land in
`crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs`
(the owning suite) unless an AC names another home.

## Acceptance criteria

**AC-OMEGA-REINDEX-POSITION.** A dependent match whose peeled field or convoyed
sibling has an Omega-classified type mentioning the refined index elaborates.
The control is that the same fixture on the base blob
`e43be39f51ede05335170e934009aa74d196600e` fails with
`index refinement: re-indexed position is not classified by a Type universe`.
Cite both outcomes; a passing test alone does not show the arm is what made it
pass.

**AC-PREFILTER.** A lawful Omega-classified **outer binding** reached through
`install_hidden_result_variable_refinements` is refined rather than skipped.
The two-sided control is the distinguishing one for this decision: on the base
blob the fixture must show the position **silently unrefined with no error**,
not an `ElabError`. A witness is required — reading the widened `matches!` does
not satisfy this, because the defect it guards is a silent skip.

**AC-OMEGA-BRANCH-GOAL.** A branch whose own checking goal is Omega-classified
and mentions the un-refined outer index elaborates, and its result is restored
to the original `expected_here`. Same two-sided control, against
`index refinement: branch goal is not classified by a Type universe`.

**AC-REAL-CONSUMER.** The minimal `Probe` does not establish the real family's
recursive path, so it is not sufficient on its own. Required, per
`evt_1wnk1ek4s8sgj`:

- real `FokDerivation` matches over **all four constructors**, under **both**
  Type and Omega motives;
- a **recursive Omega theorem** that consumes the exact recursive child paths;
- the existing ordinary indexed-Type, unindexed `FokCert`, and
  truncation-into-Omega positives preserved.

These are **test consumers**. `FoKripke.ken` stays byte-untouched, and no
FO-side production artifact may be edited to satisfy this.

**AC-PROBE.** The `Probe` from held evidence `3f687a460` is a fixed input: its
**source program stays unmodified**, while its transition expectation flips
from rejection to acceptance. The held expect-error harness cannot itself pass
unmodified once the capability lands — that flip is the expected result, not a
violation. **It does not require D2 of V3-FO-EMBEDDING-ADEQUACY.** If the
source program itself needs editing to pass, that is a finding to report, not a
licence to edit it.

**AC-J-MOTIVE-EXACT.** Assert the Omega arm's constructed motive against the
built `Term`, not against source text: body
`λ y (_ : Eq index_ty old_idx y). cur_ty[y/old_idx]`, ascription
`Π(y:index_ty). Π(_ : Eq index_ty old_idx y). Ω l`, base the untransported
value, scrutinee `h`. A test that only checks the arm was reached does not
satisfy this.

**AC-DIRECTION.** A mutation that **reverses the old/new generalization
direction** must red. This is a distinct control from arm selection and from
inferability: collapsing Omega to Type proves only that the arm is selected,
and dropping the ascription proves only that the motive is inferable. Neither
proves transport orientation. The direct-`J` prototype's reversed direction
already reddened the used-evidence positive, so this control is known to
discriminate.

**AC-CORE-KERNEL-CHECKS.** The core term produced through the Omega arm passes
kernel checking. This is the soundness gate; elaboration succeeding is not it.

**AC-TYPE-ARM-UNCHANGED.** Every existing Type-classified refinement produces a
**byte-identical** core term to the base blob, discharged by an **actual
exact-base-versus-candidate `Term` / transparent-body differential** over the
existing Type fixtures, in matching deterministic environments.

**Suite greenness does not discharge this.** The `ds5b` suite asserts
elaboration, evaluation and error classes; it never compares emitted core
bytes, so reading it as byte-identity evidence measures something the AC does
not claim. Keep the unchanged suite as **regression** evidence and do not
promote it. An expected-output edit in that file remains a failure of this AC
rather than a test update.

**AC-NO-INDEX-WIDENING.** Decisions 4 (`elab.rs:3360-3372`) and 5
(`elab.rs:3504-3515`) are unchanged and still Type-only. Pin **both** with
witnesses whose *index type* is Omega-classified: they must still fail closed
with `result refinement: matched type is not classified by Type` and
`index refinement: index type is not classified by a Type universe`
respectively. A read of the unchanged source does not satisfy this — each site
has to be shown still firing.

**AC-FAIL-CLOSED.** A classifier that is neither `Type` nor `Omega` still
raises `ElabError::Internal`, and the message names the actual classifier
found. Needs a real witness, not an inspection of the fallthrough arm. **Two
halves, each owned by the increment that changes its decision:** the
**decision-1 half** at `try_reindex_cast` lands with D1; the **decision-2 half**
at `refine_branch_goal` lands with D2. Decision 3's default is a separate
question and is governed by AC-DECISION-3-DEFAULT, not by this AC.

**AC-DECISION-3-DEFAULT.** Decision 3's behaviour for a classifier outside
`Type | Omega` must be **resolved with evidence, not left to prose.** As
written, §3 says today's silent `continue` is preserved there, but AC-PREFILTER
measures only the Omega case and AC-FAIL-CLOSED covers only decisions 1 and 2 —
so accepting bare `_`, or converting the old skip into an error, would fail
nothing. Resolve it one of two ways and say which:

- **If a non-sort is reachable** after a successful `kernel_infer` at that site,
  supply a witness that reaches it and show the silent skip preserved.
- **If well-formed context formation closes the successful population to
  exactly `Type | Omega`**, state that closure argument explicitly and mark the
  `_ => continue` branch **defensive and unreachable**. Do not manufacture a
  production witness for an unreachable state.

**In either case, bare `_` admission at decision 3 is an explicit
NON-DISCHARGE.** Widening the prefilter to accept anything beyond
`Type | Omega` fails this AC regardless of suite colour. Do not resolve this by
guessing which branch is true; establish it.

**AC-D2-MOTIVE-PROVENANCE.** D1's exact-motive pin proves nothing about a
**separately built** delayed-restoration `J` arm, so D2 must discharge its own
motive provenance. Either:

- **prove reuse** — D2's Omega restoration routes through the *same* exact
  already-pinned constructor as decision 1, via a structurally unique call, and
  D2 proves that use rather than asserting it (this is the preferred shape,
  mirroring how `refine_branch_goal` and `try_reindex_cast` already share
  `build_index_type_cong` on the Type arm); or
- **carry its own controls** — D2 supplies its own AC-J-MOTIVE-EXACT against
  the built `Term` plus its own collapse-to-Type and drop-ascription mutations
  at the branch-goal restoration producer *and* consumer.

Choosing the second because the first was not attempted is a reportable
finding, not a free choice.

**AC-PLAN-TYPED.** Mutation control on D2: a mutation that applies the
Type-Cast restoration to an Omega-tagged plan entry must be **caught by the
suite**, not merely produce a different term. Report the mutation by its
injection point, not by its effect.

**AC-MUTATION.** Three mutation classes minimum, hashed logs, each named by
injection point: (a) collapse the Omega arm to the Type arm at
`try_reindex_cast`; (b) drop the ascription from that Omega motive; (c) restore
the prefilter at `elab.rs:3392-3405` to `Term::Type(_)` only.

**All three inject into decision 1 or decision 3, so all three are D1
properties and land with D1** — they are not deferrable to D2 or to a later
shared deliverable. AC-DIRECTION supplies a fourth, also D1's. **D2's mutation
obligations are its own** and come from AC-PLAN-TYPED and
AC-D2-MOTIVE-PROVENANCE; D1's (a) and (b) do not cover the separately built
restoration arm.

All must red. A mutation that stays green is a gap in the suite and is
reportable as a finding.

**AC-KERNEL-UNCHANGED.** Zero diff under `crates/ken-kernel/`. No new kernel
capability, no trust delta, no `FokDerivation` change, no V3-FO D1 change, no
change to the released structural premise, `FoKripke.ken` byte-untouched.

**AC-BLAST-RADIUS.** `try_reindex_cast`, `refine_branch_goal` and
`install_hidden_result_variable_refinements` are shared surfaces reached by
every dependent match, not only by FO. Name the suites the change can reach and
gate on them, `ds5b_dependent_match_refinement_acceptance` and
`surface_def_refinement` at minimum. If the census finds a third, gate on it too
and say so.

**AC-NO-REGRESSION.** Green in **CI** — the full-workspace build, the `--locked`
gate, and the conformance suite run on GitHub, never on the box. Local work is
targeted only, via `scripts/ken-cargo -p ken-elaborator` (or `--test <name>`).
Never `--workspace` locally.

## Reviewers

- **Architect** — required. It ruled the mechanism (`evt_1wnk1ek4s8sgj` +
  `evt_pw69nxgxn99j`) and blocked the first cut of this frame
  (`evt_367papv4k57kk`); the motive, the orientation, the five-decision census
  and the fail-closed boundary are its call.
- **language-qa** — the mutation evidence and the two-sided controls,
  particularly AC-PREFILTER's silent-skip base outcome and
  AC-TYPE-ARM-UNCHANGED's differential.
- **Adversary** — after landing, on the merged SHA.

## Capability tier

**T1.** The mechanism is handed down precisely, but the tagged restoration plan
is a design call, and the soundness turns on an argument about which motive is
admissible at which sort and in which direction — not on a mechanical diff.

## Sequencing

Predecessor to `V3-FO-EMBEDDING-ADEQUACY` D2, which is held. When **both**
increments land, the Steward owes an **explicit** re-release of the FO node;
landing does not release it.
