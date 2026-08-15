---
id: V3-FO-OBLIGATION-SIGNATURE-DISCOVERY
title: "Decide and build how an incoming obligation is matched to an FO slice signature, so route FO's public entry point can reach the embedding at all"
status: active
owner: language
size: L
gate: none
depends_on: [V3-FO-KRIPKE-SLICE]
blocks: []
github: null
origin: "Steward, 2026-08-15. Filed as the named home for the public-route residual cut out of V3-FO-KRIPKE-SLICE D5/AC-5 (disposition in that node's leading banner). Architect finding evt_30mehjtecaazy; QA block on e0474679c upheld. The load-bearing mechanism was re-derived by the Steward from crates/ at e0474679c before filing. Steward-filed per COORDINATION section 2."
---

## The gap, stated as a fact about the tree

**Route FO is built and unreachable.** `attempt_fo` (`prover.rs:545` at
`e0474679c`) opens with:

```rust
let sig = crate::fo_kripke::declare_fo_slice_signature(env);
attempt_fo_with_signature(env, ctx, phi, phi_closed, &sig)
```

`declare_fo_slice_signature` calls `declare_postulate` and `declare_inductive`,
**which mint fresh `GlobalId`s on every call.** It is not idempotent, not even
against the same `env`: two calls yield two disjoint signatures.

⇒ **The signature `attempt_fo` quotes against is unreachable by any caller**, so
`quote_fo` refuses every externally-constructed term and the route falls through
to the unchanged IPC fallback. **Every real obligation, always.** The slice's
quotation, embedding, certificate search, and checker are correct and no program
can reach them.

**Two consequences worth separating.** The capability gap is that route FO does
nothing new in production. The measurement gap is that any probe driven through
`attempt_fo` is **inert** — it returns `Unknown` by quotation refusal rather than
by the theorem-home boundary, and would return exactly the same with the slice
deleted.

## The decision this node exists to make, and it is not the ring's

The missing step is **deciding which signature an incoming obligation belongs
to.** That is a design call with its own review, per `evt_30mehjtecaazy`, and it
must be made before anything is built.

> ### CORRECTED. The old prohibition was RIGHT ON SITE, WRONG ON REASON.
>
> **This block previously said the obligation must not be allowed to determine its
> own signature, because `embed` would then be computed over caller-chosen
> predicates. The Architect re-derived that against `23 §4.4` and it is wrong**
> (`evt_4v2j0e05t5ew9`). It was my restatement of his earlier warning, inherited
> rather than re-derived — the same failure this node's own frame warns about.
>
> **`embedding_adequacy` is universally quantified over `Sigma`, `C`, `rho`, and
> `f`.** A caller-chosen `Sigma` is therefore **inside** the theorem's statement,
> not outside it. So obligation-derived selection is **not per se unsound**, and
> the old sentence forbade something the theorem already covers.
>
> ⇒ **A rule built to the wrong reason over-constrains in one direction while
> leaving the actual hole open.** That is what happened here, and it is why the
> correction changes what this node builds rather than merely how it is worded.
>
> **The real hazard is QUOTATION PRESERVATION**, the obligation `§4.4` states one
> paragraph earlier: if `quote_fo(o) = Accepted(problem Sigma C rho f)` then
> `denote C rho f` must be the `Pi`-closed proposition of the **original**
> obligation `o`, up to definitional equality. **The danger is not that the caller
> picks the vocabulary — it is that quotation accepts a `(Sigma, C, rho, f)` whose
> `denote` is a DIFFERENT proposition from the one asked, and then discharges `o`
> with a proof of that other thing.**

### The lawful rule: forbid UNVERIFIED selection, not obligation-derived selection

**An obligation MAY determine which of its own `GlobalId`s fill the sort and
predicate roles.** Four conjuncts, and the third is the one that carries it:

1. **Role assignment is a deterministic function of the obligation's own syntax**,
   total-or-refusing. **No ambient declaration-order matching, no environment
   scan, no "most recent postulate of the right shape."** That prohibition stands.
2. **Declaration shapes are validated:** the sort is a declared type, the
   predicate is `A -> Omega` at that sort. **Necessary, not sufficient.**
3. **Quotation ESTABLISHES PRESERVATION and refuses otherwise:** `denote C rho f`
   is definitionally the `Pi`-closed proposition of `o`. **This must be
   discharged, not assumed.** If it cannot be established for a given obligation,
   the outcome is `Unknown` — **refusing is always available and always safe.**
4. **`embed` is applied to the very `Sigma` quotation verified**, not to one
   re-derived downstream. **Two derivations of "the same" signature is exactly the
   shape that failed on the runtime candidate earlier today** — planner and
   lowering each computed a key and nothing proved they agreed.

> ### CONJUNCT 3 IS *THE* ATTACKABLE CLAIM. The four are not equals.
>
> **Conjuncts 1, 2 and 4 fail LOUDLY.** A non-deterministic matcher, a wrong
> declaration shape, a re-derived `Sigma` — each produces a refusal or a visible
> disagreement.
>
> **Conjunct 3 is the only one whose failure is SILENT.** Quotation accepts, the
> certificate checks, the verdict looks honest, and **the proposition discharged
> is not the one asked.**
>
> ⇒ **Whoever attacks this should spend nearly all of their effort on conjunct 3.**
> Stated here rather than left to be inferred from ordering
> (Architect rider, `evt_2t61wgk7pp896`).

> ### "ADOPTION" IS NOW A LAWFUL WORD, AND ONLY IN VIRTUE OF CONJUNCT 3.
>
> The matcher **may adopt the obligation's own identities BECAUSE preservation is
> established — never instead of establishing it.**
>
> **Adoption without established preservation is exactly the cheap repair that got
> the predecessor blocked**, now wearing a sanctioned term. Pinned here so the
> word cannot later be read as sanctioning the unverified version by someone
> reading the vocabulary and not the conjunct.

> ### THE CONSISTENCY SAFEGUARD IS IN THE TYPE, NOT IN OWNERSHIP.
>
> A caller cannot escape by choosing a `Sigma` whose `K(Sigma)` is inconsistent.
> If `K(Sigma)` were unsatisfiable, `classically_valid(embed Sigma f)` would hold
> vacuously for **every** `f`, and adequacy would then force `denote C rho f` for
> every `f` — which is false. **So adequacy is not provable for arbitrary
> `Sigma`.** What makes it provable is that its statement demands an actual
> `C : Carriers Sigma` and `rho : AtomEnv Sigma C`. **That is a model**, quotation
> must produce one, and **an obligation cannot conjure a carrier interpretation
> for an inconsistent theory.**
>
> ⇒ **Spend this node's effort on conjunct 3 and on producing `C`/`rho` honestly.
> Do not spend it on signature-ownership machinery**, which buys provenance you do
> not need at the cost of the reachability you do.

> **Why "prover-owned, pre-registered" was rejected, since it is the intuitive
> answer:** it is **too strong** — a real obligation's sort and predicate are the
> user program's `GlobalId`s, a pre-registered signature holds prover-minted ones,
> and those are never equal. It converts fresh-per-call unreachability into
> fresh-per-registration unreachability and **fails `D3` by construction.** It is
> also **too weak** on the half that matters: ownership is a provenance property,
> preservation is a semantic one, and you can own `Sigma` and still quote `o` into
> an `f` whose `denote` is not `o`.

## THIS NODE REMOVES THE FIRST OF TWO GATES THAT ARE MASKING EACH OTHER

**Architect condition on the `D0` ruling, `evt_7r29t8139t4p2`. Read this before
`D0`, not at `D3`.**

Route FO cannot return `proved` today for **two independent reasons**:

1. **Quotation refuses every real obligation**, so nothing reaches the boundary.
2. **`23-prover.md §4.4` forbids `proved`** until `embedding_adequacy` and
   `checker_soundness` are kernel-checked in an approved home — an Architect and
   operator item, still unsettled.

**Gate 1 is currently doing all the visible work, and gate 2 has never been under
load.** This node removes gate 1. The moment quotation starts succeeding, **`§4.4`
becomes the only thing between an accepted certificate and `proved`** — and it
becomes load-bearing for the first time inside a node whose subject is signature
matching, reviewed by people reasoning about signatures rather than about theorem
homes.

⇒ **That is exactly the shape in which a fail-safe gets refactored past by
someone who does not know it is the last one.**

**Two conditions follow, and they bind:**

- **`D0` must state the `§4.4` interaction explicitly as part of the soundness
  obligation**, not only the signature-matching rule.
- **`AC-2`'s mutation must be aimed at the `§4.4` gate, not at quotation.** A
  discrimination test that only proves quotation started working leaves the
  second gate untested at the precise moment it becomes the only one.

## Deliverables

**`D0` — RE-POSED, 2026-08-15.** The first `D0` was ruled not accepted
(`evt_4v2j0e05t5ew9`) because it was built to the superseded reason above.

**Re-pose it as the four conjuncts, with conjunct 3 as the attackable claim** —
that is the one an adversary should be invited to break, **because its failure
yields a wrong proof rather than a refusal.** Still state the `§4.4` interaction
explicitly, per the two-gate section.

> **The Architect ruled the SHAPE, not the mechanism.** How preservation is
> established is this ring's. **If it turns out preservation cannot be established
> without something he has banned, come back and say so rather than routing
> around it.**
>
> **And he asked for disagreement over deference:** he derived the adequacy
> quantification and the model-witness argument from `23 §4.2`/`§4.4` at ruling
> time rather than from the earlier thread. **If you read those sections and reach
> a different conclusion, that is wanted.**

**`D0` IS NOW ACCEPTED** (`evt_2t61wgk7pp896`): all four conjuncts carried
faithfully, the model-witness safeguard stated as structural rather than as
machinery, and the `§4.4` interaction and `AC-2` aim correct.

⇒ **Sequence: `D5` first (ungated), then `D1`-`D3`.** The Architect released
`D1`-`D3` to begin after `D5` closes.

**`D1` — the discovery mechanism**, built to whatever `D0` rules. Recognition
must be by a checked structural property, not by ambient state and not by
declaration order.

**`D2` — `attempt_fo` reaches the boundary on a real obligation.** The public
route quotes an externally-constructed positive control, finds its certificate,
and arrives at the `23 §4.4` boundary.

**`D3` — the inherited residual, discharged.** The `AC-5` fail-safe is
demonstrated **through the public `attempt_fo`**, not through
`attempt_fo_with_signature`: an obligation whose certificate `check_cert`
genuinely accepts yields `Unknown`, never `Proved`. **Assert the acceptance as a
precondition** — without it the test passes via the IPC fallback and measures
nothing.

**`D4` — a refusal that is still a refusal.** An obligation outside the slice
fragment is refused by `quote_fo` as before. Discovery must not widen what
quotes.

**`D5` — the distinguishing audit label. NOT GATED ON `D0`; start here.**
Today both exits converge on `emit_unknown_hole`, which declares a postulate
labelled `"prover unknown goal"` (`prover.rs:743-752`) carrying nothing else. So
**an obligation whose certificate was ACCEPTED and withheld pending `§4.4` is
indistinguishable in `trusted_base()` from a goal nothing could establish.**

Give them distinct audit labels. **The two states are different facts and only
one of them is a limitation of Ken's logic.**

> **This is the instrument the `§4.4` theorem-home decision needs**, per
> `evt_55yxjym31ktxz`: the precise number that would let that decision be made
> against evidence rather than in the abstract — how often route FO would
> actually discharge a real obligation — **is generated at that line and thrown
> away there.**
>
> **It is buildable before `D0`'s ruling** and should not wait on it. The
> accepted-certificate path already reaches `emit_unknown_hole` at
> `attempt_fo_with_signature`, so the label is meaningful and testable at the
> helper level today. `D1`-`D3` remain gated; this one is not.

> ### THE LABEL RECORDS PROVENANCE, NOT STRENGTH. This is a constraint, not a nuance.
>
> **A `trusted_base()` entry labelled "checked, withheld pending `§4.4`" is
> exactly as assumed as one labelled "prover unknown goal".** The postulate is
> admitted either way and the goal is taken on faith either way. **Nothing about
> the certificate makes the entry weaker as an assumption** — the entire reason it
> is withheld is that the theorems licensing that step have no approved home. If
> they had one, it would not be a postulate at all.
>
> **The hazard is specific and likely:** a hopeful-sounding label invites a reader
> to treat the entry as nearly-proved and to discount it when auditing what Ken
> assumes. **That would make the audit surface worse than the undifferentiated
> label it replaces**, because today's label at least does not overstate.
>
> ⇒ **Neither state may be presentable as a lesser assumption.** Distinguish
> provenance without grading strength, and say in the code that the label is
> provenance only.
>
> **The same conflation to avoid when the number arrives:** a large count of
> withheld entries is evidence that route FO would discharge real work. It is
> **not** evidence that those obligations are closer to proved. Both are true at
> once. (Architect, `evt_7h0jnhhwtrah5`.)

**`D6` — measure whether the audit label reaches a hash. GATES `D1`-`D3`.**
Added 2026-08-15 from the Architect's non-blocking item on the approved `D5`
(`evt_241vfpwng5jym`), which asked for it before `D1`-`D3` rather than after.

**Today nothing in production reaches either label.** `D1`-`D3` are exactly what
makes them appear in real elaborations for the first time. **If the label
participates in any canonical or content-addressed encoding, then its wording is
an artifact-stability concern and not only an audit-honesty one** — and `D5`'s
"do not reword this label" doc comment silently changes from good practice into
a hard constraint with a different owner.

**The Architect's reading, offered as a hypothesis and explicitly not as the
answer:** `trusted_base_delta` is keyed on `StableSymbol`, and `18 §4.2` calls
the postulate name a non-positional audit label, which he takes to mean not
identity-bearing. **He read that; he did not measure it**, and said so while
naming the shape he had already been wrong in once that day — a structural
reading trusted over a probe.

⇒ **Probe it. Report either outcome.** If the label does reach a hash, say so
plainly: that moves ownership of future wording changes, and `D5`'s doc comment
needs to say which constraint it is enforcing.

> ### `D6` IS DISCHARGED AND THE ANSWER IS YES. The reading was wrong.
>
> **The label reaches the hash.** Two `Decl::Opaque` values differing only in
> `name` produced unequal `canonical_decl_bytes` — 166 vs 236 bytes under the
> same `StableSymbolTable`. `encode_decl` serializes the Opaque name
> unconditionally; `emit_package_from_env` serializes admitted declarations into
> the canonical semantic bytes with no Opaque exclusion. Probe
> `evt_3twtwsv7fhadh`, uncommitted and reverted, with `D5` restored byte-exact.
>
> **This is a pre-existing general property, not a `D5` defect, and the Architect
> ruled it must not be filed as one** (`evt_2q0bm3ez5aczd`). Every
> `declare_postulate` audit label is already a canonical input, including the
> unchanged `"prover unknown goal"`. `D5` is the first thing to have looked.
> **Home: [[CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY]]**, filed and owned so this
> question is not carried inside a signature-discovery arc.
>
> **`D5`'s approval stands and its object is unchanged.** One supporting clause
> of `dec_3dv5462aen3g`'s resolution text — that the label does not reach a hash
> — is measured false; the verdict is not. The FO-withheld label reaches no
> artifact today because nothing admits a prover hole into an emitted package
> through route FO, so `D5` as approved changes no existing hash. **It is future
> reachability that makes this live, which is what `D1`-`D3` create.**
>
> ### `D5` MERGED at `e6d7e30f8` (PR #2342), blob-verified 2/2 from `320ef7e6c`.
>
> ### `D1`-`D3` NOW WAIT ON [[V3-FO-QUOTE-GUARD-FAIL-CLOSED]] — Steward sequencing
>
> That node is size `S`, is `ready`, and fixes a **known fail-open guard** in
> `quote_fo`: `mentions_var0` misclassifies `Pair` as a binder and defaults ten
> subterm-carrying constructors to `false`. It runs **before** `quote_iform`'s
> refusal, on unvalidated input.
>
> **`D1`-`D3` are precisely the work that puts live obligations through it.**
> Fixing the guard first is free while `D1`-`D3` have not started and stops being
> free afterwards. **The earlier instruction that the quote-guard node must not
> jump this one is reversed** — it was written when this node's `D5` was the
> ungated starting point, and `D5` has landed.
>
> ### `D1`-`D3` ARE ALSO BOUNDED BY THE ENCODING PROPERTY, NOT BLOCKED BY IT
>
> Three constraints, from the same ruling. They are not gates and nothing is owed
> back before starting.
>
> 1. **Every audit label is a frozen artifact-identity input, not prose.**
> 2. **No further citation-bearing labels** until the encoding question is
>    settled. Each one adds another spec-number-to-hash edge, and the `§4.4`
>    citation already in the approved label is the sharpest form of the problem:
>    **a renumbered spec section would force a choice between a stale label and
>    a hash change across every package carrying such a hole.**
> 3. **Record it at `emit_unknown_hole_fo_withheld`** — one sentence saying the
>    do-not-reword instruction is load-bearing for artifact stability as well as
>    for `AC-8`'s presentation property. The current wording gives a reader no
>    way to infer the second reason.
>
> **The probe method is the transferable part.** The Architect asked for a
> measurement instead of shipping his reading, and the measurement contradicted
> it. `AC-9` is why this is known rather than assumed.

## Acceptance criteria

**`AC-1`.** No signature-selection rule is landed that `D0`'s ruling did not
authorize. **If the work seems to require deciding it, that is the handback.**

**`AC-2`.** `D3`'s public-route test fails if the boundary is removed.
**Demonstrate the discrimination by mutation** — an inert probe is what produced
this node, and a second one would not be caught by the same reasoning twice.

**Aim the mutation at the `§4.4` gate, not at quotation** (Architect condition,
`evt_7r29t8139t4p2`). The term under test must be one that **now quotes
successfully and carries an accepted certificate**, and it must still yield
`Unknown`. A mutation that only shows quotation started working proves gate 1 and
leaves gate 2 — by then the only gate — unmeasured.

**`AC-3`.** Route FO does not return `proved`. The `23 §4.4` reservation stands
until both theorems are kernel-checked in an approved home, and **this node does
not discharge it.**

**`AC-4`.** No new kernel primitive and no trusted axiom.

**`AC-5`.** The slice fragment is not widened. More sorts, predicates,
connectives, or `Cert` constructors remain a later node.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

**`AC-7`.** `D5`'s two audit labels are distinguishable **from the trusted-base
entries alone — via `trusted_base()` plus `env.lookup` — without knowing which
route produced the entry.** **Demonstrate both states**: one obligation whose
certificate was accepted and withheld, one nothing could establish. A label only
readable by someone who already knows which path ran is not the instrument `§4.4`
needs.

> ### CORRECTED 2026-08-15. The old wording said "`trusted_base()` output alone".
>
> **`trusted_base()` returns `Vec<GlobalId>`** (`ken-kernel/src/env.rs:568`) — the
> label is not in it. And `declare_postulate` calls `fresh_id()` unconditionally,
> so **the two holes were always distinct there**; the candidate's own control
> proves it with `assert_ne!(accepted_id, ordinary_id)` before reading any label.
>
> ⇒ **What the label buys is SEMANTIC distinguishability after `env.lookup`** — a
> reader learns *why* the entries differ, not *that* they differ. Both are worth
> having and only the second already existed.
>
> **This was a Steward error in the AC, not a defect in the work.** The wording
> was reviewed and approved by QA and the Architect, and the delivered test does
> the right thing — its doc says the names are recovered via `env.lookup`. **The
> `prover.rs` comment was written to the AC, so it inherited the error**; fixing
> it is folded into the Architect's constraint-3 comment edit rather than filed
> separately. Adversary hunt `evt_38p85xzh3tge`, re-derived against the tree by
> the Steward before acceptance.
>
> **The reusable shape: an AC that names the wrong surface propagates into the
> code comment written to satisfy it**, and every reviewer downstream checks the
> code against the AC rather than the AC against the tree.

**`AC-8`.** **Neither label presents its entry as a lesser assumption**, per the
provenance-not-strength constraint above. Both are postulates admitted on faith
and the labels distinguish where the entry came from, never how nearly-proved it
is. **A reviewer auditing what Ken assumes must not be invited to discount
either one.**

> **`AC-8` is REVIEW-GATED, not testable, and that is deliberate.** Whether a
> label invites a reader to discount an entry is a judgment about presentation
> and no mutation settles it. Every other criterion here demands demonstration by
> measurement; **this one does not, and a green suite does not discharge it.**
>
> **The Architect's standard, stated in advance so it is knowable rather than
> invented at review time** (`evt_5n9p6qeqhf368`):
>
> > Can a reader who does not already know the answer tell the two states apart
> > **and** come away treating both as fully assumed? **A label that discriminates
> > but reads as reassuring fails, and so does one that is honest but unreadable
> > without prior knowledge.**
>
> ⇒ **Name the CAUSE of the withholding, not the status of the obligation.** The
> missing theorem home is the fact; the certificate is not. **The exact wording is
> the implementer's to propose** — the Architect declined to write the string so
> that a real proposal gets reviewed rather than his phrasing being built to.

**`AC-9`.** `D6`'s answer is established **by a probe, not by a citation.**
Changing the label text and observing whether any canonical or content-addressed
value moves is a measurement; re-reading `18 §4.2` and concluding it is
non-positional is the reading that already exists and is what `D6` was raised to
check. **Report the outcome either way** — "the label does not reach a hash" is
a result, and so is the opposite.

## Adversary hunt on the landed `D5` range: dispositions

`evt_38p85xzh3tge`, on `320ef7e6c...c94f0319a`. Recorded here so none of it is
re-surfaced; each item was re-derived against the tree before acceptance.

**1. The `prover.rs:590` rationale names the wrong surface. CONFIRMED.**
Folded into the Architect's constraint-3 comment edit — one edit fixes the
surface claim and records the artifact-identity constraint. The comment should
also point at [[CORE-AUDIT-LABELS-ARE-ARTIFACT-IDENTITY]], because a reader who
learns the label is an audit convenience will not expect it to move artifact
bytes. See the `AC-7` correction above for the origin of the error.

**2. Two identities, opposite answers, and the comment invites conflating them.**
The same string **does not** participate in *declaration* identity — `fresh_id()`
is unconditional and `ax2_named_postulate_inertness.rs` pins *"labels do not
participate in identity"* — and **does** participate in *artifact* identity, per
`D6`. **Both statements are true and they are about different identities.** The
replacement comment must say which.

**3. The FO label is a verbatim prefix-extension of the ordinary one. RULED: THE
PREFIX STANDS**, and not as an accepted limitation (Architect,
`evt_38w0vqh6rba1h`). The hazard is real; **the remedy was the wrong one.**

> ### THE TWO OPTIONS FAIL IN OPPOSITE DIRECTIONS, and that settles it
>
> | option | what a naive `contains("prover unknown goal")` does |
> |---|---|
> | **shared prefix, as landed** | matches **both** — an audit of what a package assumes **OVER-includes** |
> | **distinct leading text** (the proposed fix) | matches only the ordinary ones — the audit **UNDER-reports the trusted base** |
>
> **These are not two flavours of one bug.** Over-reporting assumptions is a safe
> failure: you review more than you must. **Under-reporting the trusted base is
> the dangerous one** — you conclude a package assumes less than it does, and the
> silently dropped entries are exactly those where a certificate was found, which
> is where a reader is most likely to have been reassured already.
>
> ⇒ **The proposed fix takes a hazard that currently fails safe and inverts it to
> fail unsafe.** The prefix is not a cost paid for `AC-8`'s sake; it is
> independently correct on audit-failure direction, and the two goals agree.

> ### THE DURABLE PART: HOW TO WRITE AN AUDIT AGAINST THESE LABELS
>
> Written here rather than left in a thread, because the failure mode is someone
> reaching for `starts_with` and silently under-reporting.
>
> - **Enumerating everything a package assumes:** match the **shared prefix** and
>   include both. Both are postulates admitted on faith.
> - **Separating the two causes:** use the **FO-specific token** (`theorem-home`,
>   or the section citation) **or exact equality** with `"prover unknown goal"`.
>   **Never a prefix.**
>
> **The third option asked for already exists in the landed string** — all three
> queries are expressible and none requires weakening another. The candidate's own
> test uses the precise form, asserting the FO name contains `theorem-home`. The
> finding assumed prefix matching was the only tool; the distinction is not
> dissolved, only unavailable to one sloppy query.

**3a. The Architect's `AC-8` basis was wrong in the same way `AC-7` was.** He
reasoned from *"in any `trusted_base()` listing both entries lead with..."* —
there is no listing of names, only ids a reader looks up. **His conclusion
survives on the assembled view, which is the only view a reader ever has**, but
the stated basis does not. Recorded because he asked that the corrected reason
stand rather than the original.

**4. The `§4.4` gate cannot erode; watch for a SECOND exit instead.** It is an
unconditional `return` inside the `check_cert`-success branch, so there is no
condition for `D1`-`D3` to weaken. **The real hazard is a second
accepted-certificate exit that does not route through it** — a structural check
on the `D1`-`D3` diff. Carry this into `D1`-`D3` review.

**5. A `check_cert`-false result falls silently through to `attempt_ipc`. NOT A
DEFECT.** Correct as safety — never trust an unchecked certificate. But
`find_certificate` is documented as a **decision procedure**, so a `Some(cert)`
that `check_cert` rejects is a **searcher/checker disagreement that nothing
observes.** Recorded as a known blind spot; worth a counter if the two ever grow
apart, and not work for this node.

## Banned scope

- **Deciding the theorem home.** Still an Architect and operator call, still not
  this ring's, and it is not unblocked by this node.
- **Widening the slice.** `§4.5`'s bounds are unchanged.
- **Returning `proved` for FO.** `AC-3`.

## What this node does not settle

**It does not make route FO return `proved`**, and landing it changes no
verdict from `Unknown` to `Proved`. It makes the slice *reachable*, which is the
precondition for the theorem-home work to matter at all.

## Provenance

`V3-FO-KRIPKE-SLICE`'s disposition banner; Architect finding
`evt_30mehjtecaazy`; QA block on `e0474679c`. `attempt_fo` and
`declare_fo_slice_signature` read at `e0474679c` by the Steward, who re-derived
the non-idempotence rather than inheriting it.
