---
id: V3-FO-TOP-REFUSAL-ENFORCEMENT
title: "Give the sort-candidate criterion an enforcement: a test asserting quote_iform refuses top_id with UnsupportedTermShape, so an IForm::Top arm reds at the same moment the collector needs updating -- and soften the D2 claim that the criterion forces that update"
status: active
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
> today **only** because `quote_iform` refuses `⊤` with `UnsupportedTermShape`.
>
> **That refusal is an unasserted property of the current tree.** When
> `IForm::Top` lands, `quote_iform` gains a second bare-`Const` arm and
> `⊤ -> q` reproduces the over-collection `V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT`
> repaired, **on a different constant.** The criterion describes that coupling;
> **no test observes it.**

> ### THE LANDED COMMENT SAYS "COLLECTOR AND QUOTER REFUSE TOGETHER". FALSE.
> ### Adversary `evt_dnaz2tj1jvyz`. Verified by the Steward at `fo_kripke.rs:278`.
>
> ```rust
> if level_args.is_empty() && *id != env.bottom_id() {
>     sort_ids.insert(*id);        // top_id IS inserted
> }
> ```
>
> **The guard excludes `bottom_id` and nothing else, so the collector does NOT
> refuse `⊤` — it collects it as a spurious sort candidate**, and the refusal
> comes from conjunct 1's ambiguity check. **That is the over-collection path,
> exactly as `⊥` behaved before `V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT`.**
>
> ⇒ **The conclusion is right and the stated mechanism is not.** `⊤` costs
> nothing **because `⊤` is not quotable**, so the obligation it would have lost
> was going to be refused anyway. **`⊥` was quotable. That is the whole
> difference**, and it is the same under/over-collection distinction the
> surrounding paragraphs spend their length establishing.
>
> **Why this is not pedantry:** a reader who takes *"both refuse together"*
> literally concludes **`top_id` is not collected** — false today — **and that
> is precisely the premise they would reason from when adding `Top`.**

## `D0` — assert the refusal, so the coupling reds

**A test asserting `quote_iform` refuses `top_id` with `UnsupportedTermShape`.**

**Its value is entirely in when it FAILS.** The moment someone adds an
`IForm::Top` arm, this test reds — **at the same moment the collector criterion
needs updating**, and in the same file. That converts the comment's advisory
sentence into a tripwire.

> ### THE ALARM IS ON THE QUOTER; THE OBLIGATION IS IN THE COLLECTOR.
> ### Adversary `evt_dnaz2tj1jvyz`, and the fix is in the AC, not the mechanism.
>
> **The trigger is exactly right** — adding an `IForm::Top` arm reds this test
> at the moment it matters. **But the test's SUBJECT is `quote_iform` while the
> obligation is in `collect_signature_candidates`.**
>
> An author who has just written `Const{top_id} => Ok(IForm::Top)` sees an
> assertion reading *"quote_iform must refuse top_id"* fail, and **the natural
> reading is "correct — I just made it accept `Top`; this assertion is
> obsolete."** ⇒ **The cheapest resolution is to delete the red**, and the
> collector is never opened.
>
> ⇒ **That converts a silent future defect into a LOUD MISROUTED one.** Better,
> and still not the enforcement the word *"forces"* promises.
>
> **An alarm that does not name the obligation it guards is resolved by
> silencing it.** That is what `AC-2` exists to prevent.

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

**`AC-2`. THE FAILURE MESSAGE NAMES THE COLLECTOR OBLIGATION.** When this test
reds, its message must tell the author **what to go fix and where** — in
substance: *"if you are adding a quoter arm for this constant,
`collect_signature_candidates` must exclude it under `D2`'s criterion."*

> **Check this by reading the failure text, not the source.** Run the mutation
> from `AC-1`, **read what the test harness prints**, and ask whether an author
> who has never seen this node would open `collect_signature_candidates` on the
> strength of it. **If the message only says the quoter should have refused,
> the honest response to it is to delete the test** — and the defect returns
> silently. **This AC, not the assertion, is what makes the node worth doing.**

**`AC-2a`. Correct the "both refuse together" clause in `D2`'s comment.**
Verified false at `fo_kripke.rs:278`: the guard is `*id != env.bottom_id()`
alone, so **`⊤` IS collected** and its refusal comes from conjunct 1's
ambiguity check — the over-collection path. State it as: *`⊤` is still
collected as a spurious sort candidate and still refuses on ambiguity, but it
costs nothing because `quote_iform` refuses `⊤` anyway, so no quotable
obligation is lost; `⊥` was quotable, and that is the difference.*

**`AC-3`. `top_id` is still NOT excluded from sort candidates.** This node adds
an assertion about the quoter; it does not change the collector. **Excluding
`top_id` now fits a future tree rather than this one** and is barred by
`V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT` `AC-2` and by
`V3-FO-SUBST-DEPTH-CONTROL` `AC-7`.

**`AC-4`. The collector's doc comment and the test name each other.** The
coupling is the deliverable; a test that enforces it silently, from a file the
collector never mentions, leaves the next editor exactly where they are today.

**`AC-5`. No `IForm::Top` arm is added**, no slice widening, no FO `Proved`, no
new kernel primitive, no trusted axiom. **This node prepares for `Top`; it does
not begin it.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only — `-p ken-elaborator`, never `--workspace`.

## Banned scope

- **Adding `IForm::Top`** or any slice widening.
- **Excluding `top_id`** from sort candidates — see `AC-3`.
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
