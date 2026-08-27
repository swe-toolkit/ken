---
id: LANG-INDEX-REFINEMENT-OMEGA-ARM
title: "Make dependent-match index refinement sort-general: classify re-indexed positions and branch goals over Type union Omega, transporting Omega-classified types by a direct J arm alongside the existing Type-plus-Cast arm"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-08-27, discharging the predecessor the Architect assigned in evt_pw69nxgxn99j (thread thr_1czkntcjrvvz9), which supersedes the earlier evt_1wnk1ek4s8sgj framing. V3-FO-EMBEDDING-ADEQUACY D2 hard-stopped immediately on release (language-implementer evt_74q124wnb3zaf, leader evt_5fxgv9eeqm68f): eliminating the proof-indexed FokDerivation with index-dependent Omega evidence fails elaboration with ElabError::Internal(\"index refinement: ... not classified by a Type universe, found Omega0\"). The Steward routed it to the Architect (evt_nrvb2atg0xay) without diagnosing it. The Architect ruled the route VIABLE and located the defect in the elaborator, not the kernel and not the premise. All fixed inputs below measured at origin/main 6a37b92c7ce02edf0e73be7306776253ca68e8c4."
---

> # WHAT THE ARCHITECT RULED — evt_pw69nxgxn99j, authoritative
>
> Three arms were on the table when the hard stop was routed. The ruling
> **rejects two and does not need the third**:
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
> restoration must classify over `Type ∪ Ω`.
>
> **Explicitly NOT authorized by this node:** any kernel change, any trust
> change, any `FokDerivation` change, any change to V3-FO D1, and any change to
> the released structural premise or to `fok_classically_valid`'s validity
> statement. This node is elaborator-only and sort-general — it is not about
> FO. FO is only the consumer that exposed it.

> # WHAT IS HELD BEHIND THIS NODE
>
> The Language ring stays held on `V3-FO-EMBEDDING-ADEQUACY` D2 until this
> predecessor is framed, implemented, reviewed, landed through publisher CI,
> **and explicitly re-released**. Landing this node authorizes nothing
> downstream on its own.
>
> Evidence commit `3f687a460f4399bd1204a03ca8cbb57cad75eb92` (tree
> `15f2d977e`, one test path, `+126`, blob `c9eefe4e5`, 2/2 passing;
> independently reproduced by the Architect) remains **held transition
> evidence, not a D2 candidate**. Its minimal `Probe` is a fixed input to this
> node (AC-PROBE), not a deliverable of it.

## Fixed inputs

Measured at exact `origin/main 6a37b92c7ce02edf0e73be7306776253ca68e8c4`,
tree `27ba416f97336f302be67930a9241526d2992f96`.

| path | blob | role |
|---|---|---|
| `crates/ken-elaborator/src/elab.rs` | `e43be39f51ede05335170e934009aa74d196600e` | the whole change surface (10079 lines) |
| `crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs` | `db992f637717d9e65f5a5830c5a5730f5cbe06e8` | the owning acceptance suite (622 lines) |

The three Type-only classification sites in `elab.rs`, and their disposition:

| site | lines | classifies | disposition |
|---|---|---|---|
| `try_reindex_cast` | 3234-3245 | a re-indexed position's type `cur_ty` | **WIDEN (D1)** |
| `refine_branch_goal` | 3304-3315 | the refined branch goal `candidate` | **WIDEN (D2)** |
| capability-2 sibling convoy | 3504-3515 | the *index* type `peel_ty` | **OUT OF SCOPE — stays Type-only** |

The third site is **not** named by the ruling and is not widened here. An
index inhabits a Type; an Omega-classified index type is a different question
that no evidence has raised. It keeps failing closed, and AC-NO-INDEX-WIDENING
pins that it still does.

Supporting coordinates, all unchanged by this node unless stated:

- `build_index_type_cong` (`elab.rs:3171`) — builds
  `e : Eq (Type l) cur_ty new_ty` by `J` and returns `(e, new_ty)`. It is the
  **Type arm** and stays exactly as it is.
- `try_reindex_cast` call sites: `elab.rs:1670`, `3408`, `3482`, `3542`. All
  four route through the one function, so D1 is a single-point widening that
  covers every caller.
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
`Cast`. Where the type is Omega-classified it does not fall back — it raises
`ElabError::Internal` and elaboration stops.

Make the refinement **sort-general**: classify over `Type ∪ Ω`, keep the
existing Type-plus-`Cast` arm byte-identical, and add a direct-`J` arm for
Omega. Any other classifier still fails closed.

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

### 2. D1 — re-indexed positions (`try_reindex_cast`)

Replace the `match whnf(...) { Term::Type(level) => level, other => Err }` with
a three-way classification:

- `Term::Type(l)` — unchanged: call `build_index_type_cong` and wrap in
  `Term::Cast`. The emitted core must be byte-identical to today's.
- `Term::Omega(l)` — build the direct-`J` transport of §1 and return it as the
  refined value. There is no `Cast` node on this path.
- anything else — `ElabError::Internal`, naming the actual classifier.

Return type is unchanged (`Option<(Term, Term)>`): the pair is still
(refined value, refined type). Only how the value is built differs.

### 3. D2 — branch-goal restoration (`refine_branch_goal`)

Same three-way classification on `candidate`'s classifier. The consequence is
larger here because the restoration is deferred: `refine_branch_goal` returns
`casts: Vec<(Term, Term, Term)>` and the caller replays them in reverse as
`Term::Cast` at `elab.rs:2260-2262`. An untagged triple cannot express which
arm restored it.

**Replace the untagged triple with a typed restoration plan** — one entry per
refinement step, tagged Type-Cast or Omega-J, carrying exactly the ingredients
its own arm needs. The caller dispatches on the tag and applies the matching
form, still innermost-first. This is the ruling's "the corresponding typed
Type-Cast/Omega-J plan"; an untagged vector that the caller re-classifies at
application time does not satisfy it, because it re-derives at the consumer
what the producer already knew.

### 4. Fail-closed discipline

Every classifier that is neither `Type` nor `Omega` continues to raise
`ElabError::Internal`. Do not widen to a `_ =>` accept, and do not widen the
capability-2 index-type site (fixed inputs, third row).

### 5. What produces the soundness

Nothing here is postulated. The Omega arm's result is an ordinary kernel `J`
application, so the branch's core term is kernel-checkable exactly as the Type
arm's is. AC-CORE-KERNEL-CHECKS is the gate that this actually holds rather
than merely elaborating.

## Deliverables

**D1 and D2 are each independently landable.** Sequence them as two increments
so each reaches a releasable state or a genuine hard stop inside about an hour.

- **D1.** `try_reindex_cast` classifies over `Type ∪ Ω`; Omega takes the
  direct-`J` arm. Covers all four call sites at once.
- **D2.** `refine_branch_goal` classifies over `Type ∪ Ω`; the returned
  restoration becomes a typed Type-Cast/Omega-J plan and `elab.rs:2244`'s
  caller dispatches on it.
- **D3.** Tests for both, in
  `crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs`
  (the owning suite), plus the negative and mutation evidence the ACs below
  require.

## Acceptance criteria

**AC-OMEGA-REINDEX-POSITION.** A dependent match whose peeled field or convoyed
sibling has an Omega-classified type mentioning the refined index elaborates.
The control is that the same fixture on the base blob
`e43be39f51ede05335170e934009aa74d196600e` fails with
`index refinement: re-indexed position is not classified by a Type universe`.
Cite both outcomes; a passing test alone does not show the arm is what made it
pass.

**AC-OMEGA-BRANCH-GOAL.** A branch whose own checking goal is Omega-classified
and mentions the un-refined outer index elaborates, and its result is restored
to the original `expected_here`. Same two-sided control, against
`index refinement: branch goal is not classified by a Type universe`.

**AC-PROBE.** The minimal `Probe` from held evidence `3f687a460` elaborates
under this node's change, unmodified. This is a fixed input, so reproduce it
rather than re-inventing it. **It does not require D2 of
V3-FO-EMBEDDING-ADEQUACY, and no FO-side artifact may be touched to satisfy
it.** If the probe needs editing to pass, that is a finding to report, not a
licence to edit it.

**AC-J-MOTIVE-EXACT.** Assert the Omega arm's constructed motive against the
built `Term`, not against source text: body
`λ y (_ : Eq index_ty old_idx y). cur_ty[y/old_idx]`, ascription
`Π(y:index_ty). Π(_ : Eq index_ty old_idx y). Ω l`, base the untransported
value, scrutinee `h`. A test that only checks the arm was reached does not
satisfy this.

**AC-CORE-KERNEL-CHECKS.** The core term produced through the Omega arm passes
kernel checking. This is the soundness gate; elaboration succeeding is not it.

**AC-TYPE-ARM-UNCHANGED.** Every existing Type-classified refinement produces a
**byte-identical** core term to the base blob. The control is the full
`ds5b_dependent_match_refinement_acceptance` suite green with **no edits to any
expected output**. An expected-output edit in that file is a failure of this
AC, not a test update — say so if one seems needed.

**AC-NO-INDEX-WIDENING.** The capability-2 index-type site (`elab.rs:3504-3515`)
is unchanged and still Type-only. Pin it with a witness whose *index type* is
Omega-classified: it must still fail closed with
`index refinement: index type is not classified by a Type universe`. A read of
the unchanged source does not satisfy this — the site has to be shown still
firing.

**AC-FAIL-CLOSED.** A classifier that is neither `Type` nor `Omega` still
raises `ElabError::Internal` at both widened sites, and the message names the
actual classifier found. Needs a real witness, not an inspection of the
fallthrough arm.

**AC-PLAN-TYPED.** Mutation control on D2: a mutation that applies the
Type-Cast restoration to an Omega-tagged plan entry must be **caught by the
suite**, not merely produce a different term. Report the mutation by its
injection point, not by its effect.

**AC-MUTATION.** Two mutation classes, hashed logs, each named by injection
point: (a) collapse the Omega arm to the Type arm at `try_reindex_cast`;
(b) drop the ascription from the Omega motive. Both must red. A mutation that
stays green is a gap in the suite and is reportable as a finding.

**AC-KERNEL-UNCHANGED.** Zero diff under `crates/ken-kernel/`. No new kernel
capability, no trust delta, no `FokDerivation` change, no V3-FO D1 change, no
change to the released structural premise.

**AC-BLAST-RADIUS.** `try_reindex_cast` and `refine_branch_goal` are shared
surfaces reached by every dependent match, not only by FO. Name the suites the
change can reach and gate on them, `ds5b_dependent_match_refinement_acceptance`
and `surface_def_refinement` at minimum. If the census finds a third, gate on
it too and say so.

**AC-NO-REGRESSION.** Green in **CI** — the full-workspace build, the `--locked`
gate, and the conformance suite run on GitHub, never on the box. Local work is
targeted only, via `scripts/ken-cargo -p ken-elaborator` (or `--test <name>`).
Never `--workspace` locally.

## Reviewers

- **Architect** — required. It ruled the mechanism (evt_pw69nxgxn99j); the
  motive, the typed plan, and the fail-closed boundary are its call.
- **language-qa** — the mutation evidence and the two-sided controls.
- **Adversary** — after landing, on the merged SHA.

## Capability tier

**T1.** The mechanism is handed down precisely, but the typed restoration plan
is a design call, and the soundness turns on an argument about which motive is
admissible at which sort — not on a mechanical diff.

## Sequencing

Predecessor to `V3-FO-EMBEDDING-ADEQUACY` D2, which is held. When this lands,
the Steward owes an **explicit** re-release of the FO node; landing does not
release it.
