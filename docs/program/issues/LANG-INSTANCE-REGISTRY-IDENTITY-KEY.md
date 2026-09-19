---
id: LANG-INSTANCE-REGISTRY-IDENTITY-KEY
title: "Re-key the class-instance registry on IDENTITY (GlobalId) rather than on surface type NAME. classes.rs:202 declares `pub instances: HashMap<(String, String), InstanceInfo>`, registration builds the key from surface syntax, and any elaborate-side consumer holding an identity but not a name must reverse-lookup id -> name, for which NOTHING GUARANTEES a well-defined answer. D0 gates this node: exhibit a GlobalId whose reconstructed name is absent or resolves back to a different id, or close it. Reaches coherence, the orphan check, module re-export and the derive path, so it is deliberately NOT paid by LANG-STANDARD-INFIX-CALL-COMPLETION."
status: draft
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "language-leader, 2026-09-19 (evt_4qnyda4jn97vh), routed to the Steward on the Architect's instruction as the second overflow item from LANG-STANDARD-INFIX-CALL-COMPLETION's D1b, found while building the completion adapter's <=/>= dictionary resolution. The Architect ruled the real closure is NOT A1's to pay because it reaches coherence, the orphan check, module re-export and the derive path; A1 instead carries a bounded linear scan with an explicit ambiguity refusal, which the Architect ruled sound for A1's scope and explicitly NOT a fix. A1 is not blocked on this. Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c."
---

# What is verified, and on which tree

**Measured by the Steward at `origin/main` `b2d186ef7`.** The filing carried
coordinates without a tree, so they are re-grounded here rather than relayed.

    VERIFIED VERBATIM, main b2d186ef7
      crates/ken-elaborator/src/classes.rs:202
        pub instances: HashMap<(String, String), InstanceInfo>,

    VERIFIED VERBATIM, main b2d186ef7
      crates/ken-elaborator/src/elab.rs:757     the name -> id resolution
        RType::RCon(name, span) => {
            if name == "Omega" { return Ok(Term::omega(Level::Zero)); }
            let id = cx.globals.get(name).copied()
                .ok_or_else(|| ElabError::UnresolvedCon { ... })?;

The filing cited `:776-786`, which is correct on branch `c96e0c20f` and is the
same arm eighteen lines later; on `main` it is `:757`. **An earlier revision of
this node recorded it as "does not resolve on main". That was the Steward
looking at one range and concluding rather than searching, and it was wrong.**

# THE PREMISE: AN ABSENCE OF GUARANTEE, NOT AN EXHIBITED COLLISION

**The Architect's claim was** *"nothing makes that injective, so id -> name is
a choice among candidates"* — **an ABSENCE-OF-GUARANTEE claim.** An earlier
revision of this node rendered it as *"name -> id isn't injective"*, **an
EXISTENCE claim.** Those need different evidence and the modality was
strengthened in transit by the Steward, not by the Architect.

> **This is the same relay family as a dropped SHA and a worse member of it.**
> A dropped qualifier is visible as a gap. **A strengthened modality reads as a
> cleaner finding** — nothing in the stronger sentence looks like it is missing
> anything. Architect, `evt_4mfatt232egep`.

**The strong claim is kept as the node's GATE, deliberately.** A node titled
IDENTITY-KEY that rests on "no guarantee was written down" is a tidiness change
wearing a soundness title. **Exhibit the defect or close the node.**

    D0, and it gates the node's existence.
    At a NAMED SHA, exhibit a GlobalId whose reconstructed surface name is
    either (a) ABSENT -- no name resolves to it -- or (b) UNFAITHFUL -- the
    name it would be reconstructed under resolves back to a DIFFERENT id.
    Either one breaks the reverse lookup. Absent BOTH, close this node.

> ### THE DIRECTION MATTERS AND AN EARLIER REVISION HAD IT BACKWARDS.
>
> That revision asked whether *"two distinct `GlobalId`s carry the same surface
> name"*. That is only half the hazard, and on its own it is not even the
> dangerous half. `cx.globals` is keyed BY NAME, so it holds at most one id per
> name:
>
>     two NAMES -> one ID      id -> name has several candidates. Ambiguous.
>     two IDS -> one NAME      the map holds ONE. The shadowed id reconstructs
>                              to a name that resolves to the OTHER id -- a key
>                              that looks valid and points elsewhere.
>
> **The second is the one that produces a wrong dictionary silently**, which is
> the whole reason this node is not cosmetic. D0 above covers both.

**Circumstantial support, NOT a substitute for D0:** `classes.rs:208` carries
`global_modules: HashMap<GlobalId, u32>`, mapping each id to its declaring
module, and `[[LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD]]` exists as a node.
Both are consistent with names colliding across modules. **Consistent-with is
not measured**, and a reader must not promote this paragraph to the finding.

# D0 DOES NOT GATE A1. DO NOT RECORD A1 AS BLOCKED ON IT.

**The conclusion stands. The ARGUMENT for it was withdrawn and replaced**
(Architect, `evt_zfwss6hz79ct`, amending `evt_4mfatt232egep`).

    WITHDRAWN   "the scan consults only globals.get(name), never the reverse,
                so it has no injectivity premise to be wrong about."
                FALSE. The scan assumes globals[head_name] TODAY is the
                binding head_name had at REGISTRATION time -- a mutable-state
                premise, cited rather than tested.
    STANDS      A1 confirms the carrier's identity IN CORE after the kernel
                infer, so NEITHER answer to D0 can reach it.

> ### THE REFRAMED D0 BROKE THE ARCHITECT'S OWN MECHANISM, NOT JUST THIS NODE.
>
> Under the **two IDS -> one NAME** direction the original scan is unsound:
>
>     entry registered as ("Ord","Foo") when Foo meant T1
>     globals["Foo"] now resolves to T2  (a later Foo shadowed it)
>     scan with head_id = id(T1)  ->  no match  ->  NoInstance, fails closed
>     scan with head_id = id(T2)  ->  MATCHES   ->  hands T1's DICTIONARY
>                                                   to a T2 carrier
>
> **Silent, wrong, and the two-match ambiguity arm never fires because there is
> only ONE match.** The detector is blind to the case by construction.
>
> **The amended mechanism, which is what gets built:** scan finds the
> candidate; require the class to be carrier-parameterised (`projection
> .head_param.is_some()`), else refuse; then after `kernel_infer_raw`, require
> the head `GlobalId` of the class type's argument to equal the carrier's head
> `GlobalId`, else refuse naming both identities. **One comparison on a term
> the resolver already computes.** It demotes the NAME from a decision to a
> hint, and a wrong hint fails closed at a core-level check.

**Why the two directions are not symmetric — state it this way, it is
load-bearing beyond this node.** `cx.globals` is keyed BY NAME and holds at
most one id per name. So:

    two NAMES -> one ID    an AMBIGUITY. Detectable: both candidates are
                           still present to be counted.
    two IDS -> one NAME    a SUBSTITUTION. NOT detectable, because the map
                           has already forgotten the other one.

**And if D0 returns INJECTIVE, the Architect's rejection of the synthesised
`RType` still stands — on its second reason, not its first.** Recovering a
type's identity from a name you reconstructed *while already holding the
identity* is the defect independently of whether that reconstruction happens to
be unique today; the scan is also the cheaper lookup, over this class's
instances rather than all of `globals`. ⇒ **D0 changes this node's existence and
changes nothing about the A1 ruling. The two are not coupled.**

> **If D0 returns INJECTIVE, say so and close this node.** That is a real
> outcome, not a failed turn, and it is cheaper to discover first than after a
> re-keying is built. The `(String, String)` key would still be re-keyable on
> other grounds, but not on THIS one, and the node's title would be wrong.

# The constraint, interrogated

**Grounded** if D0 returns non-injective: a consumer that must choose among
candidates to recover a key is deciding instance selection by a rule nobody
wrote down, and instance selection is a soundness-adjacent surface — it decides
which dictionary a term elaborates against. `PRINCIPLES.md` §8 (be honest about
the boundary; prefer loud refusal over silent degradation) is the relevant
commitment, and A1's chosen workaround already honours it.

**NOT grounded, and must not be written in:** this is not a TCB argument.
`PRINCIPLES.md` §5 places the elaborator outside the trust root — the kernel
re-checks. A wrong dictionary is a wrong elaboration, caught or not by the
kernel depending on whether it changes the checked term; that is worth fixing
on its own terms without claiming a trust-root defect.

# What A1 does instead, and why it is not this fix

A **bounded linear scan with an explicit ambiguity refusal**, on the Architect's
ruling. Sound for A1's scope. **It is explicitly not a fix**, and this node must
not be closed by pointing at it — a local refusal at one call site does not
re-key a registry that four other paths read.

# Blast radius, from the Architect

    coherence           module re-export
    the orphan check    the derive path

**This is why it is not A1's to pay**, and why it is `M`/`T1` rather than a
small mechanical change: the re-key is easy, and the four consumers' behaviour
under a changed key is the actual work.

# Contention: one file overlaps an already-filed node

`[[LANG-R-LAYER-EXPORT-RETRACTION]]` names `InstanceInfo` and
`InstanceConstraintInfo` in `classes.rs` among the items whose visibility it
narrows. **Different axes — that node changes visibility, this one changes a
key type — but the same file.** Whichever runs second re-grounds its
coordinates; neither blocks the other, and they must not be folded.

# Why this is `draft`

**QUEUED by priority, not unframed**, and additionally **gated on its own D0**.
L1 (clearing the ignored tests) is the operator's top priority as of
2026-09-17, and the language lane's objective is `LANG-MODULE-IMPORT-SYSTEM`.
Not released, not to be started until the Steward releases it. The leader
confirms A1 is not blocked on it.
