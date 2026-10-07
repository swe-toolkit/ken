---
id: LANG-NAMED-REFINEMENT-TYPE-ARGUMENT
title: "A conversion that holds only by unfolding a named refinement to its carrier is accepted with zero obligations: List Five admits Cons Int six and List Char admits 55296. Refuse the nested introduction by running the ruled polarity guard over elaborator-recorded refined types keyed on the refinement's checked identity at each introduction site, not over the spelling of a delta-transparent Const in core terms"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION, LANG-MATCH-RESULT-REFINEMENT-IDENTITY]
blocks: []
github: null
origin: "F-E: separate finding the Architect's TYPE-POSITION D0 ruling evt_5am0p8wy7vc9j asked to be measured; measured on c49297983 by the language implementer (evt_2tgrvgpr76kwm), boundary confirmed by the Architect (evt_19x6v93jytdy4). Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A conversion that unfolds a named refinement where nothing introduces it is refused

## Objective

A conversion that holds only by unfolding a named refinement, at a position
where nothing introduces its predicate, is refused. A value that really has
the refined type, such as the prelude's `List Char`, stays legal, and the
corpus keeps its base results.

## Settled inputs (Architect recut `evt_5rpfj17mr3n9n`, research `evt_629xjmwz1wd5z`)

- **The shared predicate.** All three stops read refinement presence off the
  spelling of a delta-transparent `Const` head (`def Char = {c:Int |
  isScalar c}`). Under the carrier encoding (`34-data-match.md:14-16`),
  `Char` is `Int` in core after delta, so unfolding or substitution creates
  or erases the spelling independently of the checked type. The guard's
  algorithm is right; its input is the wrong representation.
- **Retained as proved.** The safety rule: safe iff the last step is
  `EqCarrier`, or no `Arg`/`Sig` and an odd `Dom` count (ruling
  `evt_2f9avjknr1xga`, census `evt_1t4qxd37d8a06`). The forgery refusals,
  the Char instance transports (`Dom`, `cod.dom`, the `Equal` carrier), the
  `RefinementView.project` and `countOf` introductions, LANG-MATCH's checked
  leaves, the ten integration tests and the WIP pins.
- **The WIP.** `b76c4f278` on `68bd8ad73` is held, not a candidate. Its pins
  and tests carry forward; its spelling guard at the conversion choke point
  (`unaligned expected-side named refinement ...`, `elab.rs` ~11807 on the
  WIP) is what gets replaced. It refuses 23 accepted files in each
  population (`evt_15fxy29zfx7n`), which become the regression set.
- **Fan-in.** The seven non-test `emit_refinement_introduction` callers and
  the `unify_types` callers (`evt_50b544fv42ney`), plus LANG-MATCH's
  checked-leaf route.
- Kernel conversion stays pure core. `forgets` (F-H) is out of scope:
  `LANG-NAMED-REFINED-BINDER-FIRST-CLASS`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).** The language
   implementer's pending diagnostic (`evt_6dd42xpajec4n`) is its first input.
   - (a) At each fan-in site, whether the elaborator still holds the resolved
     surface type (`RRefine`, `RApp`, `RCon`) or only the kernel `Term`.
   - (b) For each site lacking it, the minimal carriage: a side table over
     binders and terms keyed by checked identity, never by spelling.
   - The Architect rules the representation at D0.
2. **The closure.** The `ConvStep { Dom, Cod, Arg, Sig, EqCarrier }` walk
   runs over elaborator-recorded refined types, with the named refinement
   carried as the refinement definition's `GlobalId` through
   `refinement_root`, never as a `Const` in a core term. It compares expected
   and actual refined types at each introduction site, before core
   conversion.

## Acceptance

Base is current `origin/main`; compare every row against it.

- **AC-1 (refused; base accepts with 0 open).** The `List Five` row and its
  `5` twin; `probe`'s argument; `Option Five`; `record Bag { xs = Cons Int
  six .. }`; P1, P2, P3's tail `t`, P8, P9, P10, P11, P12; `List (G Five)`
  with `const G : Type → Type = λa. List a` and value `Nil (List Int)`; `M.Five`;
  forged `List Char` (`Cons Int 55296 (Nil Int)`) and forged `Option Char`;
  the indexed-match branch variable and the non-dependent `match` rows of
  stop 2.
- **AC-2 (controls; accepted, obligation counts unchanged).**
  - `ElabEnv::new()` and `intToChar`; `LawfulClasses` `Ord.Char` and
    `DecEq.Char`; Derived's `RefinementView`;
    `examples/rosetta/letter-frequency`.
  - P13 gives 1; `use5 six` gives 1; `record Box { value = six }` gives 1;
    `Cons Five five (Nil Five)` gives 1; `const c : Char = 55296` gives 1;
    `Some Char n` under `inRangeBool` is unchanged.
  - **Corpus parity is the gate.** The 81 roots (76 accepted, 32 open) and
    the 55 packages (55 accepted, 20 open) match base file by file, with
    zero newly refused files. The 23 files of `evt_15fxy29zfx7n` pass.
- **AC-3 (mutation, QA).** Re-pointed at the new input: M-arg (treat `Arg`
  as safe) re-admits `List Five`; M-polarity (treat an even `Dom` count as
  safe) re-admits P1; M-carrier (drop the `EqCarrier` exemption) refuses
  `LawfulClasses`; M-transport (skip the transported return) needs a
  reaching row, or records "not expressible" with the attempt.

## Stop conditions

- Any newly refused corpus, catalog, prelude or example program: stop with
  the row. Do not respell.
- A fan-in site whose refined type cannot be carried by checked identity.
- Any kernel, `trusted_base()` or spec change. The core-subset-Σ alternative
  is an open decision outside this WP.

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. position-keyed refusal of a named refinement (type argument, expression, codomain) refused the prelude's `Char` under `List`/`Option`; keyed on syntactic position
2. nested-introduction guard keyed on elaborator check routes; match compilation checks leaves against an inferred or δ-simplified substitute, so the refinement never reaches `check` — keyed on the type a leaf is checked against
3. after leaves are checked against the as-written result, the conversion guard sees an expected-side `Char` at Arg with no aligned value-side refinement. Keyed on which side of a conversion still spells the refinement-rooted `Const`.
```
