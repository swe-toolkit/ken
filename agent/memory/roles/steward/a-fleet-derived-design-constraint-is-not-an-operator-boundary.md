---
scope: roles/steward
---

# A fleet-derived design constraint is not an operator boundary to escalate

When a funded approach hits a wall, the wall is usually a set of constraints the
design has accumulated — "no representation change", "no runtime callable
object", "no new call/frame boundary". If breaking one of them is the only way
through, the reflex is to hand the operator a "which constraint may change?"
boundary decision. **Before you do: check who imposed each constraint.** A
constraint the operator never stated is the Architect's to revise, not the
operator's to bless.

MEASURED 2026-09-01 (composed-return (a)(i), HS#5). The Architect refuted the
in-SSA delayed-S application mechanism (the future response has no owner in the
frozen representation — the closed `RT-LIVE-K-FUTURE-INPUT-OWNERSHIP-D0`
negative), and framed the next move as "an operator boundary decision: name which
producer/control boundary may change." The Steward carried that to the operator
and **quoted the exclusion list to them as if it were their constraint set.** The
operator's correction: "I did not specify any of these constraints you quote,
only the general approach (compile-time SSA vs targeted lowering of the closure
into the runtime)." The exclusions were the fleet's own framing. There was no
operator decision to hold — the way-through was the Architect's and Research's to
find, and holding it for the operator's return cost a round trip and idled the
lane on a question that was never theirs.

**Why it happens:** a design-framing exclusion and an operator directive read
identically once both are written into a node as "the WP forbids X". The node
does not record provenance, so at escalation time every constraint looks
load-bearing and operator-owned. This is the derived-vs-inherited premise trap
(`ask-whether-a-load-bearing-premise-was-derived-or-merely-inherited`) wearing an
escalation hat.

**How to apply:** before routing a "which constraint may change" fork to the
operator, for each constraint name its SOURCE — an operator `evt_`/`dec_`, a spec
rule, or fleet design framing. Only constraints in the first two classes are the
operator's to relax. If the wall is entirely fleet-framing, there is no operator
decision: route the way-through to the Architect (and Research, if the operator
has asked for it or a stop-count triggers), treat the exclusions as revisable,
and keep the lane moving. What the operator owns is the level ABOVE the
exclusions — the general approach — and if a genuinely new operator-owned fork
(TCB growth, a spec commitment) surfaces during the redesign, THAT is what comes
back, not the constraint menu. Related:
[[an-out-of-band-merge-leaves-your-branch-on-a-reverting-base]] is a different
axis; the shared discipline is: name the source before you act on the claim.
