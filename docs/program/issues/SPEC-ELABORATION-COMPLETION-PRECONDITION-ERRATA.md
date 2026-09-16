---
id: SPEC-ELABORATION-COMPLETION-PRECONDITION-ERRATA
title: "Three owed clauses in one paragraph of spec/30-surface/39-elaboration.md section 6.9, all spec-author's own text, none of which any completed WP collected: (1) replace the CBV/left-to-right argument for Bool arm-laziness with the 18a section 5.4 citation that settles it without coupling 33 section 6.1 to the evaluation strategy; (2) justify the SECOND half of the completion key -- the policy binds to the defining GlobalId AND its checked telescope, and only the identity half is argued; (3) state the precondition section 6.9 silently carries, that the policy applies where the binding's argument types are closed by the class parameter, which the membership operator is the first case to violate"
status: draft
owner: spec
size: S
gate: none
depends_on: [SPEC-MEMBERSHIP-CLASS-CONTRACT]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 on spec-author's pre-committed escalation (evt_3s90h84r5hfbg). The Architect raised the 18a citation on A-track as NON-BLOCKING, to be folded 'when something next touches that paragraph'. spec-author refused that as a disposition with no mechanism and named two triggers (evt_4xyh513q8csdz): any A-track respin absorbs them free, otherwise B-track. NEITHER FIRED -- A-track went straight to approved and landed without a respin, and B-track touched 39 and added section 6.10 but deliberately did not fold these in, because smuggling an unrelated improvement under another node's review is the wrong trade even with the file open. spec-author reported the lapse themselves rather than letting it pass a third time, and asked for a tracked item. Cut rather than appended to an existing node because the debt has now survived two candidates by being nobody's."
---

> # DRAFT — NOT FRAMED. Small, and it is a real debt rather than a tidy-up.
>
> **`9b9586dc0b5e41e7ccaa087793b31f6b5349351b` is NOT to be reopened for
> this.** It is approved exact by both required reviewers, the Architect stated
> it was the last thing they had, and reopening an approved candidate for an
> unrelated improvement is precisely the trade spec-author declined on
> principle. This node exists so that refusal costs nothing.
>
> **@spec-author has asked for this node and it is their text.** Assign it to
> them unless the spec lane says otherwise.

# What is owed — three clauses, one paragraph of `39 §6.9`

**1. The `18a §5.4` citation.** The current paragraph argues from `42 §3`'s CBV
and left-to-right rules that the `Bool` eliminator's arm laziness yields no
operand short-circuiting. The argument is **sound**; the objection is not
elegance. `18a` already records *"strict-prim vs `match` observationally
identical (`Bool` pure)"*, which settles the same point by citation.

> **The improvement is decoupling, and that is the whole case for it.** The
> present wording **couples a sentence in `33 §6.1` to the evaluation
> strategy**, so a future change to `42 §3` would silently undermine it with
> **nothing going red**. The citation carries no such coupling.

⇒ This is a latent-breakage repair, not a style preference. Frame it that way
or it will be deprioritized as prose polish, which is how it survived twice.

**2. The telescope half of the completion key.** `§6.9` states that the policy
binds to *"the defining `GlobalId` **and its checked telescope**"* — a two-part
key — and the precondition paragraph justifies **only the first part**.
spec-author found this gap in their own text; the Architect's response was that
it *"arrives with teeth: the second part is exactly where `∈` breaks."*

**B-track then proved it.** `membership_member_at`, `member_holds` and
`same_members` are all blocked on the **telescope** half, not the identity
half. So the unjustified half of the key is the half that turned out to carry
the entire obstruction.

**3. The unstated precondition, which only exists because of B-track.** `§6.9`
should state the precondition it silently carries: **the policy applies where
the binding's argument types are closed by the class parameter.** `∈` is the
first operator where they are not, and **nothing in `§6.9` warns the next
author** — they would discover it the way B-track did, by writing a signature
that cannot be spelled.

# Why this is a node and not a line on someone else's

**Spec-author, and this is the part worth keeping in the corpus:**

> A deferral with named triggers is better than "later", and **it is still not
> an owner.** Both my triggers were real and one of them fired in a way that
> **consumed it without discharging the debt** — the WP completed *around* the
> paragraph rather than through it.

⇒ **The only disposition that survives a trigger firing without collecting is a
tracked item with an owner, and the moment to create one is when a named
trigger lapses** — not when someone eventually notices the clause is still
missing. This node is that, filed at the lapse.

**The near-miss is the instructive part.** Trigger 2 was "B-track, which I
author and which touches this surface." B-track *did* touch `39`. It added
`§6.10`, one section away, with the file open — and the right call was still
**not** to fold these in, because review hygiene beats convenience. **A trigger
can fire, be honoured correctly, and leave the debt exactly where it was.**

# Not this node

- Any change to `§6.10`, the completion policy's substance, or the carrier-first
  inference rule — all landed and approved in
  [[SPEC-MEMBERSHIP-CLASS-CONTRACT]].
- The surface projection gap — [[LANG-TYPE-PROJECTION-SURFACE-FORM]].
- `18a` itself. This node cites it; it does not amend it.

# Related

- [[SPEC-MEMBERSHIP-CLASS-CONTRACT]] — B-track; consumed trigger 2 correctly
  without discharging the debt, and proved clause 2's teeth.
- [[SPEC-STANDARD-INFIX-BINDING]] — A-track; consumed trigger 1 by landing
  without a respin.
- [[LANG-STANDARD-INFIX-CALL-COMPLETION]] — A1, the use-site resolver that
  reads `§6.9`'s policy and would inherit the unstated precondition.
