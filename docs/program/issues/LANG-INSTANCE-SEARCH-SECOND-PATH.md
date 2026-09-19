---
id: LANG-INSTANCE-SEARCH-SECOND-PATH
title: "`ClassEnv::instance_search` (classes.rs:331) is a second reader of the instance registry with TWO production call sites in `projected_instance_id` (elab.rs:10090, :10101), and three doc comments (prelude.rs:21, :23, resolve.rs:165) describe it as the mechanism the `where`-clause path uses. D0 gates the node's size and its claim: classify those two calls as a RESOLUTION that reaches elaboration or a READ that classifies projection purity. If they resolve, main carries two live selection paths and this is a defect node; if they read, it is three stale doc comments plus a visibility question. The filing's premise -- zero production callers, live only in tests -- is measured FALSE and is not what this node rests on."
status: draft
owner: language
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "language-leader, 2026-09-19 (evt_6mbj5ebssp9ca), routed to the Steward out of LANG-STANDARD-INFIX-CALL-COMPLETION's AC-3 discharge on the Architect's instruction, as pre-existing work that predates A1 and discharges independently of AC-3. Filed with a CORRECTED premise: see the first section. Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c."
---

# FIRST: THE FILING'S PREMISE IS FALSE, AND THE CORRECTION MAKES THE NODE SHARPER

**Filed as:** *"pub, zero production callers, live only in
`tests/lc_acceptance.rs`"* — a second dispatch path **waiting for a caller**.

**Measured by the Steward at `origin/main` `eadba7e82`:**

    crates/ken-elaborator/src/classes.rs:331   pub fn instance_search
    crates/ken-elaborator/src/elab.rs:10090    ctx.class_env.instance_search(..)
    crates/ken-elaborator/src/elab.rs:10101    ctx.class_env.instance_search(..)

Both are inside `fn projected_instance_id` (`elab.rs:10083`), called at
`elab.rs:10126`. All in `src/`, all at module top level.

**The one thing that could have made them test-only was checked:** the nearest
preceding `#[cfg(test)]` is at `:7824` and **closes at `:7829`** — a five-line
block, nowhere near. `ProjectionPurityCtx` occurs only in `elab.rs` and no part
of the chain is test-gated.

⇒ **TWO PRODUCTION CALL SITES, NOT ZERO.** And the other half is wrong in the
opposite direction: test uses span `cat1_constructor_classes`,
`cat1_lawful_functors_package`, `cat_bool_pub_export`, `cat_bytes_keys_closeout`,
`cc1_nonempty_validation_acceptance`, `ds1_empty_dec_acceptance`,
`l3b_acceptance`, `record_decl_form` and `lc_acceptance` — not one file.

> ### WHAT THIS REFUTES, AND WHAT IT SHARPENS.
>
> **Refuted:** "dead code waiting for a caller." There is nothing to wait for.
>
> **NOT refuted — sharpened:** "second dispatch path." A path with live
> production callers is not latent. **Whether that is a defect is exactly what
> `D0` asks**, and it is a better question than the one the node was filed with:
> a dead path is a tidiness item, a live second selector is a soundness-adjacent
> one.

# D0, and it gates both this node's SIZE and its CLAIM

    Classify elab.rs:10090 and :10101 at a NAMED SHA.

    Does `projected_instance_id` RESOLVE a dictionary that reaches
    elaboration, or READ the registry to classify projection purity?

    RESOLVES -> main carries two live selection paths. Defect node,
                size grows, and it is no longer `S`.
    READS    -> the node is three stale doc comments plus a visibility
                question. Smaller than filed.

**Do not guess this from the name.** `projected_instance_id` returns
`Option<GlobalId>` inside a **purity** context, which is consistent with either
reading. The Steward measured call SITES and deliberately did not classify what
they DO — **a census of call sites is not a census of behaviour**, and this
session has already spent two numbers that were complete, correct, and cited for
a question they did not answer.

# The three doc comments are CONFIRMED, and are a finding either way

    prelude.rs:21   "via the Lc-landed `instance_search` (`classes.rs:91`)"
    prelude.rs:23   "checked against `instance_search` before the body
                     elaborates, emitting `NoInstance` on failure"
    resolve.rs:165  "checked against `instance_search` in `elaborate_rdecl_v1`"

All three describe `instance_search` as the mechanism the `where`-clause path
uses, **without qualification**. Whether that is accurate today is `D0`'s
answer; that they assert it unconditionally is true now.

**Note `prelude.rs:21` cites `classes.rs:91`. The function is at `:331`.** That
citation is stale on its face and is the cheapest available evidence that this
prose has not been re-grounded in a long time.

# This node and AC-3 are the SAME MEASUREMENT

`[[LANG-STANDARD-INFIX-CALL-COMPLETION]]`'s AC-3 was amended 2026-09-19 to the
property *"ONE FUNCTION PERFORMS INSTANCE SELECTION; every entry point
contributes a registry KEY and REFUSALS, never a resolution,"* discharged by
enumerating every `ClassEnv::instances` read that yields a dictionary instance
and showing exactly one sits on a production path.

**That census IS `D0`.** Whoever runs either runs both.

> **Which is the argument for the amended AC rather than a coincidence.** The
> struck wording — *"name the single resolver entry point"* — has no vocabulary
> for a second READER that may or may not be a selector, so it could not have
> asked this question at all. The rewrite is what makes `D0` expressible.

**They still do not fold.** AC-3 is an acceptance criterion on a released WP;
this is a pre-existing defect that predates A1 and discharges independently
(Architect, via `evt_6mbj5ebssp9ca`). Running one census satisfies both, and
neither blocks the other.

# The constraint, interrogated

**Grounded, CONDITIONALLY on `D0`.** If two production paths select instances,
`docs/PRINCIPLES.md` §8 applies directly — instance selection decides which
dictionary a term elaborates against, and two selectors that can disagree is
silent divergence. **If `D0` returns READS, the grounding is different and
weaker:** three doc comments asserting a mechanism the code does not use, which
is an honesty-of-the-record issue, real but small.

**NOT grounded, and must not be written in:** not a TCB argument
(`PRINCIPLES.md` §5 puts the elaborator outside the trust root), and not a
safety-of-`main` argument. Nothing is red.

# Why this is `draft`

**QUEUED by priority, and additionally gated on its own `D0`.** L1 (clearing the
ignored tests) is the operator's top priority as of 2026-09-17; the language
lane's objective is `LANG-MODULE-IMPORT-SYSTEM`. Not released until the Steward
releases it. Pre-existing, no contention, and it blocks nothing.

**Size `S` is provisional and is `D0`'s to overturn** — it is sized for the
READS branch. If `D0` returns RESOLVES, re-size before starting rather than
absorbing the growth silently.
