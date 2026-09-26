---
name: a-route-admitted-on-its-callee-class-must-be-probed-with-a-continuation-that-does-more-work
description: A newly admitted native route verified only on its fixture's shape can silently drop work the fixture never had; vary the effect count after the selected arm's first effect, and run the base and a no-route control
metadata:
  type: feedback
---

# A route admitted on its callee class must be probed with a continuation that does more work

RT-SELECTED-PENDING-CALL-BUILD inc2 (912c44cf4) admitted a pending leaf on
one fact, its callee class (static response owner), and replaced the Vis arm
with a trap claimed unreachable because "the owner checks Ret". Every pin
used the fixture's own leaves, each of which returns Ret after one effect.

A one-line variant, `bind (flush) (\_. print_line ..)` in the selected arm,
was admitted, built, and ran natively with exit 0 while dropping the print.
The interpreter printed it. The base refused the same source, and the same
body with no runtime-selected Match agreed natively. So the route, not the
owner machinery, lost the work. Two prints in the arm instead hit a
"please report this compiler bug" planner invariant. The landed px7m err row
reaches users as that same error behind an `is_err()` pin.

**Why:** a trap or check "unreachable because X validates Y" is only as
broad as what X actually runs. If X runs less than the source continuation,
the check passes on a truncated result and nothing traps.

**How to apply:** when a merge admits a new native route, probe it against
the interpreter with the selected continuation doing MORE than the fixture:
a second effect, an effect after the inner bind, the sibling arm selected.
Run three columns: landed, merge-base, and a control without the new route.
Only that triple attributes a divergence to the merge. Check refused variants
through the production binary (`ken native-build`, test features off) to see
the diagnostic a user sees, not the admission row. See
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
