---
scope: enclave
audience: (see scope README)
source: private memory `wp-frame-stale-vs-landed-kernel`
---

# A WP frame's description of the kernel can be stale vs the landed kernel

Two forms of one rule: a frame's account of the code is a claim to re-verify
against the landed code. Its "current state" can lag the kernel (below), and
its pseudocode is not the mechanism that shipped (last section).

A **Steward/WP frame** is authored at a point in time, and the **kernel moves
under it** — especially via Architect follow-up soundness fixes between the
frame's writing and the WP's pickup. So the frame's description of *"what is
currently deferred / stuck / a soundness-TODO"* is **not** authoritative about
the present kernel. Before elaborating "the fix," **reconstruct each seam's
CURRENT state from the landed code** (`grep`/read the actual functions), not the
frame's prose.

K2c-series-2 (3 obs-reduction seams, `16`/`17`): the frame said seam 1
(`cast_at_inductive`) *"rebuilds the constructor but keeps the family-index
value, wrapping in Cast"* and seam 3 (`check_respect`) *"raw-well-forms the
respect proof for non-Ω targets."* **Both stale.** A prior Architect decision
(`dec_7xpn5ywf4ebfw`) had already (a) **removed** the index keep-and-wrap as
**unsound** — `subst_index` was a no-op that left the reduct ill-typed because
constructor arg types mention *earlier args*, not family indices — leaving seam
1 **cleanly stuck**; and (b) **hard-rejected** non-Ω quotient elim, closing the
seam-3 hole. Elaborating from the frame would have told the build team to
**restore the exact removed unsoundness**. The catch came only from reading
`obs.rs`/`check.rs` (a parallel Explore recon), whose in-code comments cited the
superseding decision. The Steward then reconciled his own frame, naming it *"the
inverse-L5 stale-prose trap, applied to my own artifact."*

**Why:** this is the **mirror** of spec claim kernel admittance vs staging
(there a *spec chapter* ran **ahead** of the implemented kernel; here a *WP
frame* ran **behind** it) and a sibling of conformance reconcile inherits spec
metatheory bugs (match-the-artifact ≠ match-the-truth). The common root: any
**secondary artifact** describing the kernel (sibling spec chapter, WP frame,
conformance seed, a paraphrase) is a **claim to re-verify against the code**,
never a citation to build on — and the gap is largest where the code was
recently changed by a soundness fix the artifact predates. A stale "what's
broken" is worse than a stale "what's done": it actively misdirects toward
re-introducing the removed bug.

**How to apply:** at pickup of any kernel/spec-completion WP, (1) for each
deliverable, **read the named function(s) in the landed kernel** and confirm the
frame's "current state" matches — diff the frame's claim against the code's
actual fallback (stuck? reject? wrong-accept?) and its comments (look for a
superseding `dec_*`); (2) if they disagree, **flag it to spec-leader + Architect
as a scope checkpoint before authoring** (the deliverable usually stands; the
*starting point* and the do-not-restore hazard change) — bring the proposed
corrected rule, not just the discrepancy; (3) write the spec so the build team
**grounds on the code, not the frame** (and route a frame-doc reconcile so the
next reader isn't misled). A parallel Explore agent quoting the stubs verbatim
is the cheap way to get the ground truth. Extends the verify-against-the-kernel
discipline (COORDINATION §7) to the WP frame itself.

## Frame pseudocode diverges from the landed mechanism

Merged from `frame-pseudocode-diverges-from-landed-mechanism` (source: case-eq
WP and proof-vocabulary WP, two consecutive recurrences).

When authoring a normative §-mechanism statement (soundness/admission/
elaboration prose), the WP frame's **own pseudocode is not the producer** —
even when it was authored as a "normative admission algorithm." Implementers
often deliver the same *observable property* by a different *mechanism*,
and the spec is bound to the mechanism that ships.

**Concrete recurrence (2 in a row):**
- case-eq: the frame/ruling sketched a materialized `Or_N` dichotomy; the
  landed elaborator built a direct eliminator-with-equation-motive.
  Conformance validation caught it; the mechanism prose was re-grounded on
  the producer.
- proof-vocabulary: the frame's Phase-1 pseudocode said a **"scope-wide
  signatures-first" pre-pass delivers forward references.** The landed
  code has **no such pass** — forward refs come from **dependency-ordered
  SCC processing** (callee component before caller; condensation edges =
  union of all members' out-edges), and signatures-first is only the
  *within-recursive-component* step. Re-grounding on the producer flipped
  the wording. (Currency check, 2026-07-28: the SCC-order function and the
  mutual-group elaborator both still exist — grep `scc_dependency_order` in
  `crates/ken-elaborator/src/modules.rs` and `elaborate_mutual_group` in
  `crates/ken-elaborator/src/elab.rs`.)

**Why:** a frame's pseudocode is a design-intent artifact written *before*
implementation; the team may satisfy the AC by another route. Reflecting
the pseudocode verbatim puts a mechanism in normative prose that the
producer never runs — a fidelity bug that conformance validation will catch
(or worse, won't).

**How to apply:** before committing a mechanism claim, read the *landed*
function(s) named in the WP and describe what the code literally does —
the control flow, the ordering, what delivers each property — not what the
frame's pseudocode says. Cite the function, not the frame. When a code
candidate gets a repair (e.g. an ordering fix), re-ground against the exact
QA/Architect-cleared head — the mechanism prose may need to change with
it. See [[mechanism-citation-needs-own-empirical-probe]].
