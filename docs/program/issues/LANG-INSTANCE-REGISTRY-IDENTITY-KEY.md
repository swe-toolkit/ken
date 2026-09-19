---
id: LANG-INSTANCE-REGISTRY-IDENTITY-KEY
title: "Re-key the class-instance registry on IDENTITY (GlobalId) rather than on surface type NAME. classes.rs:202 declares `pub instances: HashMap<(String, String), InstanceInfo>`, registration builds the key from surface syntax, and any elaborate-side consumer holding an identity but not a name must reverse-lookup id -> name -- which is a choice among candidates rather than a fact if name -> id is not injective. Reaches coherence, the orphan check, module re-export and the derive path, so it is deliberately NOT paid by LANG-STANDARD-INFIX-CALL-COMPLETION."
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

    NOT VERIFIED HERE -- coordinate does not resolve on main
      "elab_type's own resolution at elab.rs:776-786"
      On main b2d186ef7 that range is the Constructor / IndFormer / const_
      dispatch on an ALREADY-RESOLVED `id`. It is id -> Term, not name -> id.
      The cited range is almost certainly branch-relative (A1's branch carries
      seven-plus commits above the merge-base and every line in elab.rs has
      moved).

# THE PREMISE IS THE UNVERIFIED HALF. RE-GROUND IT FIRST.

**The whole finding rests on `name -> id` NOT being injective.** If names are
globally unique, an `id -> name` reverse lookup is a total function, the
registry key is merely inconvenient rather than unsound, and this node is a
tidiness change instead of a correctness one. **That claim is the one thing the
Steward could not verify**, and it is reported as unverified rather than
inherited.

    D0, and it gates everything else in this node.
    Establish, at a NAMED SHA, whether two distinct GlobalIds can carry the
    same surface type name in scope at one registry -- and exhibit one if so.

**Circumstantial support, NOT a substitute for D0:** `classes.rs:208` carries
`global_modules: HashMap<GlobalId, u32>`, mapping each id to its declaring
module, and `[[LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD]]` exists as a node.
Both are consistent with names colliding across modules. **Consistent-with is
not measured**, and a reader must not promote this paragraph to the finding.

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
