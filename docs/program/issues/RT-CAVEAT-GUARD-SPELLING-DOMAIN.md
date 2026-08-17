---
id: RT-CAVEAT-GUARD-SPELLING-DOMAIN
title: "The census caveat's guard pins one attribute SPELLING while its own rationale clause covers three -- 18 test-gated regions under any(test, feature) are uncounted, and the guard is blind to more arriving"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CENSUS-CAVEAT-GUARD, RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING]
blocks: []
github: null
origin: "Adversary hunt evt_6dxxrgvd0w5zs (2026-08-17) on the RT-CENSUS-CAVEAT-GUARD landing be25ea6a2. Steward-filed per COORDINATION section 2, with two corrections to the finding recorded below."
---

> # HELD 2026-08-17 — THIS NODE'S PREMISE IS NOW IN QUESTION. Do not start it.
>
> **Gated behind [[RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING]]'s `D3`.**
>
> A later Adversary hunt (`evt_12x7wnwfbfbr`, on `ca639b5ef`) measured something
> this node assumes away: **the census the guard's caveat annotates carries
> `#[cfg(any())]`** (`control.rs:9233`) and is therefore **compiled out**.
> Steward-verified by reading the attribute; the Adversary confirmed it by
> planting `compile_error!` in the body and compiling clean, with a positive
> control in the live test beside it.
>
> ⇒ **The clause below — *"the census still errs toward a false red, never a
> false green"* — cannot be true as stated.** A census that does not compile errs
> toward **nothing**. The direction claim I recorded as surviving untouched does
> not survive this.
>
> **What that does to this node.** Widening the guard from one spelling to the
> full test-gating domain is only worth doing **if the guard protects something
> compiled.** Right now it does not, so the census in the table below is accurate
> and the *work it implies* rests on a constraint that may not be real (§4c).
>
> **`RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING` `D3` decides which:** revive
> the census, retire the guard with its caveat, or re-key it across all six
> spellings. **Only the third outcome leaves this node with work**, and then it
> is likely subsumed rather than run separately. Re-read this block before
> releasing it.

> # THIS DOES NOT RECLASSIFY THE LANDING, AND IT IS NOT A REGRESSION.
>
> `RT-CENSUS-CAVEAT-GUARD` corrected a caveat magnitude from **22** to **322**
> and replaced an existence check with a discriminating count guard. **That is
> an improvement and it stands.** The direction claim also survives untouched:
> more uncounted test regions means the census still errs toward a **false red,
> never a false green.**
>
> **This node exists because the same defect class survives one refinement
> down, at a smaller scale.**

## The verified census

Reproduced independently by the Steward at landed squash `be25ea6a2`, over
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs`:

| spelling | count | counted by the guard? |
|---|---|---|
| `#[cfg(test)]` | **322** | yes |
| `#[cfg(any(test, feature = "px8-ds-test-support"))]` | 12 | **no** |
| `#[cfg(any(test, feature = "r3-4b-observation"))]` | 6 | **no** |
| `#[cfg(not(test))]` | 21 | no — correctly, it is production-only |
| `#[cfg(feature = "px8-ds-test-support")]` | 3 | no — see the second correction |
| `#[cfg(not(feature = "px8-ds-test-support"))]` | 1 | no |

A raw regex finds **325** `#[cfg(test)]` occurrences against the guard's 322.
**The three extras are comment mentions** — `core.rs:85`, `:2030`, `:11348`.
⇒ **The solo-line requirement is what filters them, so it is a virtue**, and the
"obvious" broader regex would have destroyed it. Do not widen by dropping it.

## The defect, stated precisely

**The guard's selector is exact for the sentence it pins. It is narrower than
the sentence's own rationale clause.**

The caveat reads: *"does not partition out `core.rs`'s 322 inline
`#[cfg(test)]` regions, **so a call added inside one would be counted as
production**."*

- **The selector clause names `#[cfg(test)]`** — and there are exactly 322. On
  its own literal terms the guard is correct.
- **The rationale clause is what makes the caveat load-bearing**, and it is
  true of **340** regions: the 18 `any(test, …)` attributes gate real code that
  is active under `cargo test`, and a call inside one is counted as production
  in exactly the same way.

⇒ **The number is right for what the sentence says and wrong for what the
sentence is for.**

## Why this is the node's own failure class, one level down

| predicate | population | narrowing |
|---|---|---|
| v1 `*line ==` (blocked) | 105 | column-0 only |
| v2 `line.trim() ==` (landed) | 322 | one spelling only |
| the rationale clause | 340 | — |

**Each refinement fixed the previous narrowing and introduced a smaller one.**

**And the guard cannot see the gap grow.** Adding an `any(test, …)` region
leaves the count at 322, nothing reds, and the caveat's magnitude drifts
again — **the exact failure this guard was built to stop.**

## `AC-2`'s predicate arm is the right instrument and could not have caught this

Recorded because it is the durable lesson, not as a criticism of the arm.

**Narrowing an existing predicate only explores subsets of the set already
chosen.** The predicate mutation that caught v2 was sound and necessary, and it
is structurally incapable of finding a spelling nobody enumerated.
**Widening is the untested direction**, and the remaining 18 live there.

## TWO CORRECTIONS TO THE ORIGINATING FINDING. Read both before scoping.

**Correction 1 — the finding says *"the population the caveat's own sentence
names is 340."* The sentence as landed names `#[cfg(test)]` regions, and there
are 322 of them.** The gap is between the **selector clause** and the
**rationale clause**, not between the sentence and the count. This matters for
the fix: **`D1` may legitimately be a sentence edit rather than a predicate
edit** (see the fork below), and the finding's phrasing forecloses that.

**Correction 2 — the finding's own enumeration is a selector over chosen
spellings, and it missed one.** Three bare
`#[cfg(feature = "px8-ds-test-support")]` attributes appear in the file and are
absent from its table. `crates/ken-runtime/Cargo.toml` has `default = []`, so
that feature is **off** under a plain `cargo test` and those three regions are
inactive — which is a defensible exclusion, but **the finding never states it,
and an unstated exclusion is how the 18 went missing in the first place.**
**Attribute every hit to its cfg profile before counting it**, and say which
profile the population is defined against.

## `D0` — decide what the caveat is a claim ABOUT. One line either way.

The two options are not equivalent and the choice is the owner's.

1. **Widen the predicate to the rationale**: admit `test` and `any(test, …)`,
   **exclude `not(test)`**, and re-derive the magnitude. Then the guard pins
   what the caveat is for.
2. **Narrow the sentence to the selector**: state that the caveat's magnitude
   covers the bare `#[cfg(test)]` spelling only, and that other test-gated
   spellings are unpartitioned and uncounted. Then the guard pins what the
   caveat says.

**Option 2 is a legitimate discharge**, not a cop-out — but it must say the
uncounted population exists, or the next reader inherits the same false
completeness.

## The trap in the obvious fix

**`contains("test")` is wrong.** It sweeps in the 21 `#[cfg(not(test))]`
attributes, which are **production-only** — the precise inversion. The
predicate must admit `test` and `any(test, …)` **while excluding `not(test)`**:
three spellings, not a substring.

## Acceptance criteria

- **`AC-1`.** `D0` is decided and the site says which reading it took.
- **`AC-2` — the population is defined against a named cfg profile.** State
  which features are on. A count of "test-gated regions" is meaningless without
  it, per correction 2.
- **`AC-3` — if option 1: both mutation arms, and the widening one is new.**
  A predicate mutation that *narrows* reds; a constant mutation reds. **Neither
  alone establishes the counted set is the right set.**
- **`AC-4` — the solo-line requirement survives.** It is what excludes the
  three comment mentions; a fix that reintroduces them has regressed.
- **`AC-5` — direction and no-partition scope preserved.** False-red-never-
  false-green stays; only the domain and magnitude may move.

## Banned scope

- **Reclassifying the `RT-CENSUS-CAVEAT-GUARD` landing.** It is an improvement
  and it stands.
- **Touching the identifier census algorithm** or the retired `#[cfg(any())]`
  body. Same boundary the predecessor held.
- **Claiming a regression.** No behaviour changed; this is a magnitude and
  domain defect in an honest-limit caveat.

## Sequencing

**Not scheduled ahead of anything.** It blocks nothing and the operator's run
order stands: `RT-CALL-EDGE-EXECUTABILITY-AXIS`, then
`RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL`, then
[[RT-BACKEND-SPLIT-CENSUS]].

**It does NOT need to land before the census.** The census's `AC-2` already
requires every lexical count to declare its domain and to say what its pattern
cannot see, which is exactly this defect stated generally — so inventory 4
records the guard's real property rather than freezing a false completeness.
