# RT-CONTINUATION-EDGE-DISPOSITION — `D0` correction, and the hard stop measured

Base: `main` `6be73d20`. Parent: `e93afb06`, the `D0` census, held as evidence.
`crates/` byte-identical to base, tree `4c2bc579` on both sides; every
instrument reverted before this was written.

> ## THE CORRECTION, AND IT IS MINE
>
> **`D0` claimed the closeout has a per-owner lifetime. It does not.** I wrote
> that a candidate is checked *"only when its owner's ledger reaches close"* and
> read 427 unchecked candidates as healthy members sitting outside an otherwise
> successful closeout's authority. **There is no per-owner closeout to sit
> outside of.**
>
> Verified at the seat rather than taken on report: in the
> `BodyEmissionAuthority::FunctionizedUnits` arm,
> `open_continuation_claim_ledger` opens **one artifact-wide ledger** before
> `define_unit_bodies`; the same ledger spans continuation bodies, generated
> contexts and the root adapter; `close_continuation_claim_ledger` runs once
> after all of them. The comment at that seat says so in as many words —
> *"THE ONE LEDGER'S LIFETIME, opened and closed HERE rather than inside any
> single definition pass"*, with *"deliberately no per-pass partial close and no
> second mirrored ledger."*
>
> ⇒ **`CLOSE_CHECKED = false` means the compilation never reached a successful
> artifact closure, or never selected `FunctionizedUnits`.** It says nothing
> about an owner-local candidate's authority.
>
> **The 427 number is not the traversal hard stop, and my §3 causal sentence is
> withdrawn.** The counts stand as observations; the explanation attached to
> them was wrong.
>
> **How I got it wrong is the part worth keeping.** I measured a real
> correlation — unchecked candidates cluster in failing compiles — and supplied
> a mechanism for it from nothing. The census could not see the ledger's
> lifetime, because I never instrumented `open`; one grep at the seat would have
> refuted it before I wrote the headline.

## 1. The corrected denominator

**637 stands as the observational superpopulation** and the §2 partition of the
parent record is unchanged. What changes is what it is a denominator *for*.

`D2`'s production quantifier is narrower and exact: **every activated binding
candidate in one selected, successful `FunctionizedUnits` artifact, settled once
before that artifact closes.** Three classes in the census are therefore **not**
obligations that a successful functionized close must visit:

- plan-only rows, which never enter lowering (219 candidates with no compile);
- candidates in compilations already returning `Err`;
- candidates in plans compiled under the non-selected `RecursiveDescent`
  authority.

The 52 `DIRECT` and 11 `COMPOSED` rows I highlighted remain useful census
observations. **They do not show omitted healthy members of a successful
artifact ledger**, and the parent record should not be read as claiming they do.

## 2. The named measurement could not be taken on the `px8j` candidate, and why

The ruling named the `px8j` candidate target for the four axes. **That candidate
is not in the population the measurement is about.**

Measured with an instrument at the `FunctionizedUnits` arm: compiling the `px8j`
witness produces **no `AUTHORITY` record at all** — it does not select
`FunctionizedUnits`.

**The absence is real, not a plumbing failure.** A `PROBE_ALIVE` positive
control written from the probe itself does appear in the same file on the same
run, so the env-gated instrument and its path are working and the arm is simply
never entered. Without that control, "no record" would have been equally
consistent with the variable never reaching the test binary — the shape that has
cost this campaign a false negative before.

⇒ This also **corroborates the ruling independently**: `px8j`'s candidate is
`CLOSE_CHECKED = false` because its compile does not select the functionized
arm, which is exactly the reading the ruling substituted for mine.

## 3. The four axes, on a witness that IS in the population

The three `InlineNoCall` members a closeout does check — `sar_d3`, `ccr_d3`,
`coc_d3` — select the arm and reach it. Measured on each:

| axis | result |
|---|---|
| selected authority | **`FunctionizedUnits`**, recorded twice per test (the retained and activated runs) |
| `UnitBundle` membership and declared `FuncId` | **declared** — implied constructively by the next row, since `define_function` is reached only for a bundle-declared id |
| definition at the real `define_function` seat | **defined**, once, as `funcid43`, recorded adjacent to `define_function` inside `define_continuation_bodies` |
| ABI descriptor reachability | **not directly instrumented.** Definition through the bundle entails a resolved descriptor, but that is an inference, and it is the one axis I am not reporting as measured |

**No `ARTIFACT_CLOSED` record on any of the three** — the compiles refuse at the
closeout, which is the missing disposition and not a reachability failure.

## 4. Hard stop: UNFIRED, and not cleared

**The prospective `InlineNoCall` target is declared and defined through the
already-selected `UnitBundle`, inside the selected artifact, before the closeout
refuses.** Reaching it requires **no post-lowering call-graph rebuild and no
planner traversal-contract change** — the existing pipeline already gets there.
The only thing that fails is the closeout equality, which is the disposition
question this node exists to answer.

**Unfired, not cleared**, and the gap is named: ABI descriptor reachability is
inferred from definition rather than observed, so a witness that is declared and
defined but whose descriptor is unreachable would not have been caught by what I
ran.

## 5. `AC-7` remains open, and it SPLITS across `D1` and `D2`

> **AMENDED 2026-08-09 to the corrected phase contract** (ruling relayed at
> `evt_5swtqzvd30hb0`). This section previously said `D1` owes a
> *"binding-installed, closeout-checked, **successful** `InlineNoCall`
> witness."* **That last word was wrong and it named an impossible
> deliverable.**

The measurement in the parent record is unchanged: no member has a binding, a
closeout, and a successful compile; the three closeout-visible members are this
campaign's own controls in refusing compiles; and the two in successful compiles
are **non-selected-authority** rows, which `D2`'s quantifier excludes by name.

**Why the fourth clause cannot be `D1`'s.** `ContinuationClaimLedger::close`
takes exact set equality over a `planned` set seeded at `open` from the **full**
`continuation_calls()` population. An `InlineNoCall` candidate is in that
population by construction and is by definition neither emitted nor composed, so
a closeout that checks it **must** refuse. Making it compile requires either
keeping it out of `planned` — the planner-side exclusion the Architect withdrew
at `evt_dakdkqk4wbg6` — or taking the equality over a derived subset, which is
`D2`.

⇒ **The split:**

| phase | owes |
|---|---|
| `D1` | opaque candidate minting; binding installation; exact `InlineNoCall` settlement, only after the deferred bridge succeeds; a real selected `FunctionizedUnits` artifact reaching the existing close; and **the exact pre-`D2` missing-call refusal** |
| `D2` | the ordered closeout — one-disposition/disjointness first, then derive the `DirectCall ∪ ComposedCall` subset, then the **unchanged** exact equality — which converts that same witness to compile-`OK` |

**`D1`'s witness must REFUSE, and must not claim compile success.** A green `D1`
witness is a **`D1` defect**, not a success: it would mean `D2` had been done
early, the exact law had been weakened, or planner-side exclusion had returned.
The refusal is the deliverable, and it is the oracle that tells those three
apart from a correct `D1`.

## 6. Not done

No size. No `D1` representation and no witness authoring. No `D2` closeout edit.
No node fork. No `AC-7` claim. `ContinuationClaimLedger::close`, finished-CLIF
direct and composed verification, the both-sets refusal, the `composed` feed and
the empty resume are untouched; the instruments were observation-only and are
reverted. `issues/`, the tracker, rows 1-5 and the five landed repairs are
untouched, and neither predecessor's accepted work is reopened.

## 7. Suite

`scripts/ken-cargo test -p ken-runtime --lib`: **815 passed, 6 failed, 4
ignored**, unchanged from the parent. All six are the pre-existing environmental
`object_linker_packaging` link-and-run reds; `crates/` is byte-identical to the
base, so nothing here can have caused them.
