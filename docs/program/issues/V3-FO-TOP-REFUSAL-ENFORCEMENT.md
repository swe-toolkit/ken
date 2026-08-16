---
id: V3-FO-TOP-REFUSAL-ENFORCEMENT
title: "Give the sort-candidate criterion an enforcement: a test asserting quote_iform refuses top_id with UnsupportedTermShape, so an IForm::Top arm reds at the same moment the collector needs updating -- and soften the D2 claim that the criterion forces that update"
status: ready
owner: language
size: XS
gate: none
depends_on: [V3-FO-SUBST-DEPTH-CONTROL]
blocks: []
github: null
origin: "Steward, 2026-08-16, on the Architect's named successor to V3-FO-SUBST-DEPTH-CONTROL D2, routed by language-leader at evt_7a12mnz4398p0. The Architect approved the D2-D3 candidate unchanged and named this as its successor rather than a fold. Steward-filed per COORDINATION section 2."
---

## The defect: a criterion that informs but does not enforce

**`V3-FO-SUBST-DEPTH-CONTROL` `D2` landed the right repair** — the
sort-candidate exclusion is stated as a **criterion** (*exclude any `Const` that
`quote_iform` recognizes as a formula in its own right*) rather than as the
constant `bottom_id`, and it records that the premise is **contingent** on this
slice's `IForm` having no `Top`.

**Its one overclaim:** the comment says the criterion *"is what **forces** the
collector update at that moment."* **It does not force anything.** It informs a
future editor who reads it. **Nothing reds if `IForm::Top` is added and the
collector is left alone**, which is exactly the failure the criterion was
written to prevent.

> ### THE PREMISE IS LOAD-BEARING AND CURRENTLY UNPINNED.
>
> `env.top_id()` exists and `⊤` is a bare `Const` too. It needs no exclusion
> today **only** because `quote_iform` refuses `⊤` with
> `UnsupportedTermShape` — **collector and quoter refuse together, so there is
> no disagreement to over-collect into.**
>
> **That refusal is an unasserted property of the current tree.** When
> `IForm::Top` lands, `quote_iform` gains a second bare-`Const` arm and
> `⊤ -> q` reproduces the over-collection `V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT`
> repaired, **on a different constant.** The criterion describes that coupling;
> **no test observes it.**

## `D0` — assert the refusal, so the coupling reds

**A test asserting `quote_iform` refuses `top_id` with `UnsupportedTermShape`.**

**Its value is entirely in when it FAILS.** The moment someone adds an
`IForm::Top` arm, this test reds — **at the same moment the collector criterion
needs updating**, and in the same file. That converts the comment's advisory
sentence into a tripwire.

> **Write the test so its failure message says what to do.** A red here is not
> a bug in the new `Top` arm; it is the signal that
> `collect_signature_candidates` now needs `top_id` excluded. **A future author
> who reds this test and simply deletes it has reintroduced the exact defect** —
> say so in the test's own doc comment, not only in the frame.

## `D1` — soften `D2`'s "forces" to what is true

**Replace the claim that the criterion *forces* the collector update.** Once
`D0` exists, the accurate statement is that **the criterion names the coupling
and `D0`'s test enforces it** — cite the test by name from the collector's doc
comment, so a reader of either lands on the other.

**Before `D0` lands the honest word is "informs".** Do not leave "forces"
standing on the strength of a test that is not yet written.

## Acceptance criteria

**`AC-1`. `D0`'s test fails if `quote_iform` stops refusing `top_id`.**
Demonstrate by mutation — give `quote_iform` a `top_id` arm, show the test red,
revert. **A green assertion nobody has seen fail is not an enforcement**, which
is the whole subject of this node.

**`AC-2`. `top_id` is still NOT excluded from sort candidates.** This node adds
an assertion about the quoter; it does not change the collector. **Excluding
`top_id` now fits a future tree rather than this one** and is barred by
`V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT` `AC-2` and by
`V3-FO-SUBST-DEPTH-CONTROL` `AC-7`.

**`AC-3`. The collector's doc comment and the test name each other.** The
coupling is the deliverable; a test that enforces it silently, from a file the
collector never mentions, leaves the next editor exactly where they are today.

**`AC-4`. No `IForm::Top` arm is added**, no slice widening, no FO `Proved`, no
new kernel primitive, no trusted axiom. **This node prepares for `Top`; it does
not begin it.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-elaborator`, never `--workspace`.

## Banned scope

- **Adding `IForm::Top`** or any slice widening.
- **Excluding `top_id`** from sort candidates — see `AC-2`.
- **Re-litigating `D2`'s criterion.** It is right; only its enforcement claim
  is overstated.
- **Touching `subst_form_at` or the `D0`/`D3` oracle rows.** Different subject,
  and they are merged.

## Sequencing

**Lane 2, `XS`.** Independent of the FO Kripke embedding's forward work and
cheap. **The Architect approved the `D2`-`D3` candidate unchanged and named
this as its successor rather than a fold**, so nothing here reopens what landed.

## Provenance

Architect's named successor, routed by `language-leader` at
`evt_7a12mnz4398p0`; the contingent-`Top` premise it addresses was itself folded
into `V3-FO-SUBST-DEPTH-CONTROL` `D2` from Adversary `evt_44194ewx0dxa`, whose
derivation — *"the DERIVATION is durable; the LIST is not"* — is the reason the
criterion was written as a criterion in the first place.
