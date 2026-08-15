---
id: CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY
title: "Every postulate audit label is a canonical artifact-identity input: decide whether a semantic hash should encode label prose, and if not, migrate"
status: draft
owner: language
size: L
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-15, on the Architect's ruling evt_2q0bm3ez5aczd, which found the property from language's D6 probe (evt_3twtwsv7fhadh) and ruled it must NOT be filed against V3-FO-OBLIGATION-SIGNATURE-DISCOVERY D5. The ownership call was explicitly routed to the Steward. Steward-filed per COORDINATION section 2."
---

## The measured property

**Every `declare_postulate` audit label is a canonical input to artifact
identity** — including the long-standing `"prover unknown goal"`, and every other
postulate label anywhere in the system.

Measured by the language ring under `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` `D6`,
with an uncommitted and reverted `checked_core.rs` unit probe:

| fact | evidence |
|---|---|
| two `Decl::Opaque` values differing **only** in `name` hash differently | unequal `canonical_decl_bytes`, 166 vs 236 bytes, same `StableSymbolTable` |
| the name is serialized unconditionally | `encode_decl` (`crates/ken-elaborator/src/checked_core.rs:2324`) |
| admitted declarations reach the semantic bytes / core hash | `emit_package_from_env` (`crates/ken-elaborator/src/compiler_driver.rs:3081`), no Opaque exclusion |

`canonical_decl_bytes` is at `checked_core.rs:1278`.

## This is not a defect in the change that found it

**`D5` did not create the coupling — it is the first thing to have looked.**
The Architect ruled explicitly that filing it against `D5` would leave the
general fact undocumented and let the next person rediscover it the same way.
That is why this node exists and why it is scoped to the encoding, not to any
one label.

**The probe is also the reason this is known.** The Architect had read that the
label did not participate — `trusted_base_delta` is keyed on `StableSymbol`, and
`18 §4.2` calls the postulate name a non-positional audit label — and asked for a
measurement rather than shipping the reading. The measurement contradicted the
reading. **One supporting clause of `dec_3dv5462aen3g`'s resolution text is
therefore measured false**; the approval it supports stands and the object is
unchanged (`evt_2q0bm3ez5aczd` is the correction of record).

## The question this node exists to decide

**Should a canonical SEMANTIC hash encode a postulate's label prose?**

The Architect's reasoning, recorded as an argument on the merits and **not** as
an authorization:

> A postulate's semantics is its **type** — the proposition being assumed. Two
> packages that assume the same proposition under different label prose assume
> exactly the same thing, and should not be different artifacts.

Excluding labels from the hash would cost no audit value: the label still lives
in `trusted_base()` and `lookup()`, where a reviewer actually reads it. **Being
in the environment and being in the hash are different concerns, and `18 §5` asks
only for the first.**

### The sharpest form of the problem is a spec section number

The label approved in `D5` contains the citation `23-prover.md §4.4`. As encoded
today, **a spec section number is an artifact-identity input.** If that section
is ever renumbered, the choice is between:

- a label gone stale — degrading the audit surface `18 §5` exists to provide; or
- a label edit that **changes the hash of every package carrying such a hole.**

Neither is acceptable. **That a documentation-only edit can move an artifact hash
is the tell that the coupling is misplaced.**

### The tension is real and belongs to the encoding, not the wording

The Architect recorded it against his own approval: under `AC-8` he valued that
citation for making the label readable without prior knowledge, and still does.
Under the measured coupling, the same citation is a liability. **The audit goal
and the stability goal pull in opposite directions at exactly this point.** That
is a property of the encoding. Do not resolve it by rewording labels.

## Why this is not simply "remove labels from the hash"

An encoding change **touches artifact identity, invalidates existing hashes, and
plausibly reaches conformance.** It needs its own migration, and it was
explicitly not authorized inside the signature-discovery arc. Any deliverable
here must state what happens to already-emitted artifacts, and whether locked
conformance rows move.

## Deliverables

**`D0` — establish the population.** Which encoded fields are audit prose rather
than semantics, across every `Decl` variant, not only `Opaque`. **A census keyed
on `Opaque` answers a question about one constructor**, and the ruling above is
about a class.

**`D1` — the decision, posed as an attackable claim.** Whether label prose
belongs in the canonical semantic hash, argued against `18 §5`'s stated purpose
and against what conformance locks. **Not this ring's to decide alone** — it is
Architect and operator territory once artifact identity moves.

**`D2` — the migration, if `D1` rules for exclusion.** What happens to existing
hashes, which conformance rows move, and whether a compatibility path is needed.

## Acceptance criteria

**`AC-1`.** The property is demonstrated by a committed control, not by the
reverted probe that found it. The probe was correct and deliberately not landed;
a permanent claim needs a permanent test.

**`AC-2`.** Any change to canonical encoding is accompanied by an explicit
statement of which previously-emitted artifacts change identity.

**`AC-3`.** No audit value is lost. Whatever a reviewer can read in
`trusted_base()` today, they can still read afterwards.

**`AC-4`.** No-regression, in CI (`COORDINATION §12`).

## Status: FILED, OWNED, AND DELIBERATELY NOT RELEASED

**This node is not queued to a ring.** It is filed so the encoding question has a
home and a named owner, which is the condition the Architect placed on
`V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` `D1`-`D3` proceeding.

**Its existence is what unblocks that work; working it is not.** The operator's
two standing lanes are runtime's `RecursiveDescent` retirement and the z3
round-trip plus FO Kripke embedding. This is neither, so it waits — and a
correctly-filed node waiting is the accepted cost of a priority, not framing debt.

## What binds the FO ring meanwhile

The Architect's three constraints on `D1`-`D3` (`evt_2q0bm3ez5aczd`), recorded
here because they are consequences of this property:

1. **Treat every audit label as a frozen artifact-identity input, not as prose.**
2. **Introduce no further citation-bearing labels** until this question is
   settled — each one adds another spec-number-to-hash edge.
3. **Record at `emit_unknown_hole_fo_withheld`** that the do-not-reword
   instruction is load-bearing for artifact stability, not only for `AC-8`'s
   presentation property. A reader has no way to infer that from the current
   wording.

## Provenance

Language `D6`/`AC-9` probe `evt_3twtwsv7fhadh`; Architect ruling
`evt_2q0bm3ez5aczd`. The `D6` deliverable was itself added by the Steward from
the Architect's non-blocking item on the approved `D5` (`evt_241vfpwng5jym`),
which asked for a probe precisely because the answer had been read and not
measured.
