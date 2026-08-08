# RT-CONTINUATION-EDGE-DISPOSITION — `D0`, the candidate census

Base: `main` `6be73d207c10c10a363be5e012fe9888dfde5882`. Frame blob
`ff032c2847c69a515c46af30b4a52692477b5501`, node blob
`55e5df6bda098b046e566d3575dbea34bc279f51`, both read from the worktree at that
base. The predecessor's record is on the base as blob
`744a181d545f0fa9c65a6a49d75cfa991e2f0993` — the amended one, carrying the full
option-3 chronology.

Census only. **No production code changed**: the instruments were reverted
before this record was written and `crates/` is byte-identical to the base,
tree `4c2bc579c52046040db81c81fea3fb5d545523d2` on both sides. Coordinates are
cited by grep-able phrase.

> ## THE HEADLINE, AND IT IS ABOUT `D2` RATHER THAN `D1`
>
> **The closeout sees 210 of 637 candidates. It never sees the other 427.**
>
> `D2` as framed requires *"an exact, disjoint disposition for every
> candidate"* **first**, and then derives the call-obligation subset. Measured,
> the population that reaches a closeout is **a third of the candidate
> population**, because a ledger only closes over candidates whose owner
> function actually reaches closeout. Requiring a disposition for *every*
> candidate is therefore not a check added at the existing seat — it is a change
> to **which population that seat ranges over**.
>
> That is adjacent to §4's named hard stop rather than clearly inside it, so I
> am reporting it rather than ruling it.
>
> **And `AC-7` does not clear on this corpus.** Details in §4: the only
> `InlineNoCall` members a closeout actually checks are **three of this
> campaign's own controls**, all in refusing compiles.

## 1. Method

Six events, recorded per candidate, env-gated (`KEN_CED_D0_CENSUS`) rather than
`#[cfg(test)]`-only so one whole-suite run produces the population:

| event | seat |
|---|---|
| `PLANNED` | the planner's continuation-call issuance, in the fixed point |
| `BINDING` | where the deferred constructor environment installs a `StaticWorker` binding at a recursive position |
| `DIRECT` | `claim_and_call_continuation`, **on a successful claim/emit only** |
| `COMPOSED_VERIFIED` | admission to the verified composed population, after every clause of `verify_recorded_composed_discharges` |
| `BRIDGE_BYPASSED` / `BRIDGE_COMPLETE` | the heterogeneous-deforestation bridge, and its scope completing `Ok` |
| `CLOSE_CHECKED` | every identity `ContinuationClaimLedger::close` actually ranges over |

Plus `COMPILE_OK` / `COMPILE_ERR` per compile.

**Three instrument corrections, each of which changed the answer.** They are
recorded because the uncorrected versions were individually plausible and all
three under-reported.

1. **One prebuilt line per record, written with a single `write_all`.**
   `writeln!` issues a write per format fragment; the first run came back
   interleaved across libtest threads and unparseable.
2. **The key carries a per-compile epoch.** Without it a control that runs its
   fixture unmutated and then under a mutation seam collapses into **one**
   member, and a disposition produced only by the mutated arm is
   indistinguishable from one production reaches. Adding the epoch took the
   population from 402 to **634** and `InlineNoCall` from 10 to **21**.
3. **The compile outcome is recorded at the funnel, not at an entry.** The first
   version sat on `compile_expr_into_object_module` and saw **164** compiles; the
   corpus runs **624**. Every compile reaching lowering by another entry was
   silently counted as *not* `OK`. This is the same defect as
   *an instrument at one consumer is blind to the others*, made again by me on
   the node after I wrote it down.

## 2. The partition

**637 candidates**, one disposition each, **zero orphans** — every candidate
that entered lowering has a disposition, and the 219 with no compile at all are
plan-only tests that never enter it.

| class | all | closeout checked it | compile `OK` |
|---|---|---|---|
| `DIRECT` | 193 | 141 | 136 |
| `COMPOSED` | 43 | 32 | 29 |
| `BOTH` | **0** | — | — |
| `INLINE_NO_CALL` | 21 | **3** | 2 |
| `BRIDGE_INCOMPLETE` | 25 | 0 | 0 |
| `PLANNED_ONLY` | 355 | 34 | 53 |

`BRIDGE_INCOMPLETE` is a candidate whose bridge scope was entered and did **not**
complete — the compile failed inside it. It is deliberately **not** folded into
`InlineNoCall`: the frame settles that disposition only on a scope that
completes successfully, and conflating the two would manufacture members.

**`BOTH` is empty**, which is the disjointness the existing law already
enforces, measured rather than assumed.

## 3. Closeout visibility, which is the finding

**210 of 637 candidates are checked by a closeout. 427 are not.**

A candidate is checked only when its owner's ledger reaches `close`. So
`CLOSE_CHECKED` splits every class, including the healthy ones — **52 `DIRECT`
and 11 `COMPOSED` candidates are discharged at seats no closeout ever ranges
over.**

That is not a defect in the law. It is what makes `D2`'s ordering a bigger
change than it reads: *"require an exact disposition for every candidate"*
cannot be satisfied at a seat that structurally sees a third of them.

## 4. `AC-7` — `InlineNoCall` does not have a clean independent member

All 21 members, cross-tabulated, and the two columns do not overlap:

| binding installed | closeout checked | compile | n |
|---|---|---|---|
| yes | yes | `ERR` | 3 |
| yes | no | `ERR` | 16 |
| no | no | `OK` | 2 |

**There is no member that has a binding, is checked by a closeout, and
compiles.** That cell — the one the successor's representation exists to create
— is **empty today**, which is the defect stated as a population rather than as
an argument.

**The three closeout-visible members are `ccr_d3`, `coc_d3` and `sar_d3`** —
this campaign's own controls, each arming the `#[cfg(test)]` activation seam,
each in a refusing compile. They are honest witnesses that the class exists and
currently refuses. They are **not** independent members.

**The two members in successful compiles prove nothing about the law.** Both —
`d8d_the_composed_binding_site_is_live_...` and
`px8j_all_three_producer_paths_reach_real_consumers` — have `CLOSE_CHECKED =
false`. Their compiles do not succeed because the candidate was discharged;
they succeed because **no closeout ever looked at it.** Asserting anything about
`InlineNoCall` over this pair would be exactly the vacuity the frame's trap-3
box names, and I am recording them as excluded rather than counting them.

⇒ **`AC-7` cannot be discharged from the corpus as it stands.** The class is
real — 21 members reach it — but every one is either a campaign control or
invisible to the law. **A `D3` control over `InlineNoCall` needs a witness
authored for it**, and that authoring is a `D1` deliverable, not something `D0`
can find.

## 5. Committed controls, excluded and named

The `InlineNoCall` and `BRIDGE_INCOMPLETE` classes are wholly test-owned, so
every member is named in §4 and below rather than summarised. `BRIDGE_INCOMPLETE`
members come from `d8e` (both variants), `d8f_the_remaining_checked_marker_
refusals`, `d8m_the_checked_bridge_refuses_every_way_...`, and
`d8p_preserves_the_refusals_the_projection_could_have_weakened`. Nine
`InlineNoCall` members come from `d8j_the_composed_authority_is_discharged_once_
after_the_call` alone, across nine compile epochs — which is why the epoch
correction in §1 mattered.

## 6. The hard stop — measured in part, and the part I could not close

**Not closed, and I am not going to imply otherwise.** §4's `declaration /
definition / ABI reachability` measurement for a prospective `InlineNoCall`
candidate is incomplete: `b2f_last_unit_emission()` reports `(0, 0)` after a
successful compile of the `px8j` witness, so it is not observing the emission
path this question needs, and I stopped rather than substitute a different
number for the one that was asked for.

**What is measured**: the `px8j` witness plans **3** continuation calls and
carries **5** emittable units, and it compiles `OK` with one of those three
candidates settling `InlineNoCall`. So a specialization interned for a candidate
that is never called already exists inside a **successful** compile on this
base. That is evidence against the strong form of the hard stop — an uncalled
unit is not, by itself, something the current tree refuses — but it is **not**
evidence that no call-graph rebuild is required, because I have not shown that
unit is declared and defined rather than merely planned.

**The exact remaining measurement**, so the next turn does not re-derive it:
take an `InlineNoCall` candidate's `target`, and check whether that
specialization appears in the declared bundle and in the defined set, observed
at the emission seat rather than through `b2f_last_unit_emission`.

## 7. Sizing input

**I am not proposing a number**, and unlike the predecessor's `D0` this is not
because a fork is open — it is because two of the three inputs a size would rest
on are the ones §3 and §4 just moved:

- `D2`'s "disposition for every candidate" is a **427-candidate visibility gap**,
  not a check at an existing seat;
- `D3`'s five mutations need an `InlineNoCall` witness that **does not exist**
  and must be authored under `D1`;
- and §6's hard-stop measurement is open.

Sizing across those would be sizing the node twice, which is the error the
predecessor's record warned about and the frame repeats in its own `TBD` box.

## 8. Untouched

`ContinuationClaimLedger::close`, finished-CLIF direct and composed
verification, the both-sets refusal, the `composed` feed and the empty resume
are unchanged — the `CLOSE_CHECKED` instrument was an observation inside `close`
and is reverted. No `D1` representation, no `D2` closeout, no `D3` mutation. No
`#[ignore]` added; `issues/`, the tracker, rows 1-5 and the five landed repairs
untouched. The predecessor's accepted `D0`/`D1` and
[[RT-SPECIALIZED-ACTIVE-RESUME]]'s accepted partial are not reopened.

## 9. Suite

`scripts/ken-cargo test -p ken-runtime --lib`: **815 passed, 6 failed, 4
ignored** at this base, with and without the instrument armed.

**All six failures are pre-existing and environmental, not this candidate's.**
They are `object_linker_packaging::tests` link-and-run tests, failing at
`fs::read(&trace_path).unwrap()` with `Os { code: 2, NotFound }` — a linked
fixture that runs and writes no trace. `crates/` is byte-identical to the base,
so nothing here can have caused them, and the workspace gate is CI's.
