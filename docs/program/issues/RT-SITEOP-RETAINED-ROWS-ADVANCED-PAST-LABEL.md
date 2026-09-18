---
id: RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL
title: "RT-SITEOP-CARRIED-WITNESS merged claiming its 16 retained rows all stop at the LATER eliminated-not-callable refusal -- an advancing refusal, not two causes. Three of those rows were then measured stopping somewhere else entirely: rows 1 and 2 at UnclassifiedRuntimeTrap{terminal_value:-1}, row 10 at a typed-consumer-projection disagreement. The closeout's universal claim over the retained set is refuted on these three, which is why they read as owned and cannot be routed. Establish what actually owns them."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, from RT-IGNORED-FAILING-ROWS-INVENTORY's ledger (docs/program/evidence/rt-ignored-failing-rows-ledger.md), AC-5: 'Six of fifteen have an owning node that cannot close them as it stands', of which rows 1, 2 and 10 point at the merged RT-SITEOP-CARRIED-WITNESS. The ledger states the consequence -- 'A merged node with a stale label is the exact state that makes a row look owned and leaves it unroutable' -- and this node names the checkable claim underneath it. Operator directive 2026-09-15: 'The other tests should be fixed.' Steward-filed per COORDINATION section 2."
---

> # THE LABEL BEING STALE IS THE SYMPTOM. THE CLAIM UNDERNEATH IS CHECKABLE.
>
> It is tempting to file this as *"three rows carry an out-of-date label, go
> re-label them."* **That framing cannot be wrong and cannot be acted on.**
> Underneath it is a specific universal claim, made in a merged node's closeout,
> that three measurements contradict. **Resolve the claim and the labels follow;
> re-label first and the contradiction is erased without being understood.**

# The claim, and the three measurements against it

`RT-SITEOP-CARRIED-WITNESS` merged 2026-08-17 (PR #2557, exact `a388dc06`).
Its closeout, verbatim:

> ### THE 29 ROWS DID NOT SPLIT. §9's HARD STOP DID NOT FIRE.
>
> **13 un-ignored and passing, 16 retained** carrying the *later*
> eliminated-not-callable refusal. That is an **advancing refusal** — the port
> succeeded and those rows reached the next wall — **not two causes.**

**That is a universal claim over the retained 16**, and it is a good one: it
says the port worked and the survivors are all standing at one new wall. It is
also the claim that makes those rows look owned.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661` by the failing-rows
ledger**, three of the rows carrying this node's label stop somewhere else:

    row  identity                                      observed first refusal
     1   px7f_resource_native ::                       UnclassifiedRuntimeTrap
         linked_public_right_denial_preserves_          { terminal_value: -1 }
         exact_masks
     2   px7f_resource_native ::                       same as row 1
         linked_public_second_release_is_closed_
         and_the_handle_closes_once
    10   rt_escape_second_resource_native ::           source-specific inheritances
         escaped_buffer_used_by_fanning_host_op_       at one generated entry
         matches_interpreter                           disagree on their typed
                                                       consumer projection,
                                                       including the fresh-result
                                                       route

**None of the three is the eliminated-not-callable refusal.** The ledger's
`label agrees?` column records this independently for all three, each as *"label
predicts the BoundaryCarrier arity refusal."*

## THE CLOSEOUT KNEW, AND CLASSIFIED IT NON-BLOCKING

This is not a discrepancy that went unnoticed. The same closeout, in its
should-fix list, batched by the Architect and explicitly not recut:

> 1. **All 13 un-ignored tests keep a leading comment block that is now false**
>    … **The still-ignored 16 have a smaller version: their *"Observed
>    signature, exactly"* is no longer observed.**

⇒ **The merged node recorded that the retained rows' documented signatures had
stopped matching reality, and filed it as a non-blocking documentation nit —
"a smaller version" of a comment-block problem.**

> **A MONTH LATER THAT NIT IS WHY THREE ROWS CANNOT BE ROUTED.** The
> classification was defensible at merge time: nothing was broken, no test
> changed behaviour, and the fix is a text edit. **What it missed is that a
> row's recorded signature is not documentation — it is the only routing key
> anyone has.** When it goes stale the row does not look unowned, which would
> be discoverable; it looks owned by a node that will never close it.
>
> **This is the finding worth carrying out of this node even if `D0` returns
> (i):** *stale prose is cosmetic; a stale IDENTIFIER is a routing defect, and
> the two look identical in a should-fix list.*

# `D0` — WHICH OF THE TWO READINGS IS TRUE, AND THEY ARE NOT THE SAME NODE

**Do not start anywhere else. The rest of the work is different work under each.**

    (i)  these three rows were NEVER in the retained 16
         => the closeout's claim is intact and was always about a set these
            rows are not in. They are mis-attributed, and the question is what
            they were ever doing under this label. CHEAPEST, and it is a
            records question, not a runtime one.
    (ii) they WERE in the retained 16
         => the closeout's universal claim is REFUTED, the retained set did
            split after all, and `§9`'s hard stop -- which the closeout records
            as not having fired -- fired later and silently. That is a finding
            about the merged node, not about these rows.

**The discriminator is the retained-16 membership list as it stood at
`a388dc06`**, not as anyone remembers it.

> ### MEASURED BY THE STEWARD WHILE FILING: THE LIST DOES NOT EXIST
>
> Neither `docs/program/issues/RT-SITEOP-CARRIED-WITNESS.md` nor
> `docs/program/wp/RT-SITEOP-CARRIED-WITNESS.md` **names any of the three rows,
> and neither enumerates the 16.** Checked by searching both artifacts for each
> row identity: zero hits in either, for all three.
>
> ⇒ **`D0` as posed may be undecidable from the records**, and that is a real
> result rather than a failure to look. It does not make `D0` skippable: the
> membership may still be recoverable from the tree at `a388dc06` — which rows
> carried an `#[ignore]` then, and which of those this node's `D1a`/`D2`
> re-credited. **Recover it from the tree or declare it unrecoverable; do not
> infer it from the label, which is the thing under suspicion.**

## `D0` IS ANSWERED. IT RETURNS `(ii)`. RECOVERED FROM THE TREE, 2026-09-18.

**The artifacts do not name the 16; the tree does.** Steward, measured at
`a388dc06` itself:

    git grep -c 'RT-SITEOP-CARRIED-WITNESS D2' a388dc06

    px7f_resource_native.rs               2      <- rows 1 and 2
    px7l_checked_host_recursive_bind.rs   2
    px7m_hostresult_computational_match.rs 2
    px8ta_oriented_subcontinuation.rs     2
    px8x_single_schema_observation.rs     1
    rt_escape_second_resource_native.rs   2      <- row 10 among these
    rt_parity_native.rs                   5
    ------------------------------------------
                                         16

**Sixteen, exactly — and the closeout says sixteen.** The label is one
byte-identical string on all of them:

    #[ignore = "RT-SITEOP-CARRIED-WITNESS D2: the carried SiteOperand port
     succeeds; this row next refuses because a carried recursive hypothesis is
     an eliminated value, not a callable, but the call provides 1"]

⇒ **The retained 16 is recoverable as the set carrying that string**, the count
matches the closeout independently of the string, and **rows 1, 2 and 10 are
inside it.** They were never mis-attributed.

**Each of the three verified at its own line, not inferred from a per-file
count.** The count says `px7f` holds two label-carriers; it does not say *which*
two, and `px7f` holds a third `#[ignore]` under a different node
(`RT-CARRIED-RESOURCE-SCALAR`, `:268`) that a count cannot exclude:

    a388dc06 px7f_resource_native.rs:311            -> row 1, fn at :312
    a388dc06 px7f_resource_native.rs:345            -> row 2, fn at :346
    a388dc06 rt_escape_second_resource_native.rs    -> row 10, fn at :663

`rt_escape`'s other label-carrier is `escape_resource_plus_plain_matches_
interpreter` (fn at `:591`), which is **not** in the current ignored set — one
of the six retained rows that have since left it.

> **The per-file count was the instrument in this section's first revision, and
> it could not have decided this.** *Two label-carriers in `px7f`* is consistent
> with row 2 being the row under a different label. **A count establishes how
> many, never which** — and the file's third `#[ignore]` is exactly the case
> that makes the difference observable.

> **THE MEMBERSHIP TEST AND THE CLAIM UNDER TEST ARE THE SAME STRING, AND THAT
> IS THE ONE WEAKNESS HERE.** The label both marks a row retained *and* asserts
> the eliminated-not-callable refusal. So this measurement cannot distinguish
> *"retained, and the claim about it is false"* from *"never retained, and
> mislabelled."* **What breaks the tie is the count**: sixteen label-carriers
> against a closeout that independently says sixteen. Had the count come back
> 14 or 19, this would not have settled `D0` and the honest report would have
> been that it remains open.

⇒ `(ii)` HOLDS: **the closeout's universal claim over the retained 16 is
refuted**, the retained set did split, and `§9`'s hard stop did not fire when
it should have. Per this node's own `D0` text, *"that is a finding about the
merged node, not about these rows"* — and **the other 13 retained rows inherit
the doubt**, which is the consequence `D0` was ordered first to expose.

**The labels are still on `main` today, byte-identical**, on rows the ledger
measured stopping somewhere else entirely.

> **WHY THIS ORDER.** Under (i) nothing is wrong with the merged node and the
> repair is bookkeeping. Under (ii) a merged node's central claim is false and
> **the other 13 retained rows inherit the doubt** — this stops being about
> three rows. **Sizing the repair before `D0` sizes it for the wrong node.**

# `D1` — re-measure the three signatures at current `main`

The ledger's numbers are from `04d4dd38a` and it says so. **They are a correct
record of that base, not of `main`**, and this node must not inherit them as
current. Re-run the three rows and record what they do now.

**Rows 1 and 2's signature is the one to watch.** `UnclassifiedRuntimeTrap
{ terminal_value: -1 }` is **not a diagnosis** — it is the absence of one. A
trap that reached the classifier and was not classified tells you the row
failed and nothing about why.

⇒ **Do not frame a repair against an unclassified trap.** The first deliverable
for rows 1 and 2 is a *classification*: what is the criterion function that
should have matched, and why did it not? Verify the membership from the
criterion, never from the value.

# `D2` — decide ownership per signature, and do not fold on a guess

Rows 1 and 2 share a signature. Row 10 does not. **That is two signatures and
it is evidence about first stops, not about causes** — the ledger is explicit
that eight signatures is neither a floor of eight repairs nor a ceiling on
shared causes.

⇒ **Fold them only with an argument, and split them only with one.** Either
direction taken for free is the same error. If the answer is that each needs
its own node, say that; if one node covers all three, state what it is that
they share.

# What must not happen

- **Do not re-label the rows to make them consistent.** The inconsistency is
  the evidence. A row pointing at a node that does not describe it is
  discoverable; a row pointing at a plausible node nobody checked is not.
- **Do not reopen or amend `RT-SITEOP-CARRIED-WITNESS`.** It is merged and its
  `D2` landed. If (ii) holds, the finding belongs in a new node that cites it —
  a merged node's record of what it believed at merge time stays as it was.
- **Do not treat "the port succeeded" as in question.** It is not. 13 rows
  un-ignored and passing is a measured positive result and nothing here touches
  it. **What is in question is the account of where the survivors stopped**,
  which is a different claim in the same sentence.
- **Do not un-ignore any of the three to see what happens.** Two of them
  currently produce an unclassified trap; un-ignoring converts three parked
  rows into three red ones and buys one bit you can get from a targeted run.

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that measured these three
  and flagged them unroutable. **Merged; read
  `docs/program/evidence/rt-ignored-failing-rows-ledger.md`, not the node** —
  the node is the commissioning document and the ledger is the result.
- [[RT-SITEOP-CARRIED-WITNESS]] — the merged node whose closeout carries the
  claim under test. **Cited, not reopened.**
- [[RT-CARRIER-BYTESPAN-OBSERVE]] — the predecessor the 29 rows were credited
  to before `D1a`/`D2` re-credited them. Relevant only if `D0` returns (i).
