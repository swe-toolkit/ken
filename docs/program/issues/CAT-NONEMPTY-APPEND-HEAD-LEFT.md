---
id: CAT-NONEMPTY-APPEND-HEAD-LEFT
title: "`nonempty_append` publishes no law about its head, so a client holding only the three authorized selectors cannot establish `nonempty_head (nonempty_append xs ys) = nonempty_head xs`. Measured: the equality is not definitional under an abstract carrier, and the carrier constructor is outside a selector-list import even though `NonEmpty` is `pub data`. Add one `pub proof head_left for nonempty_append` in the package that owns the type. Its subject is already `pub fn`, and an attached `pub proof` is measured to travel with the function selector, so no client import or selector list changes."
status: merged
owner: foundation
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-20, at origin/main 1bd3ad8f15bdfee72e6fb16ce7c187191bb6d415. Cut out of CAT-SCHEMA-LAWS' second measured boundary (foundation-implementer evt_78ten32r28fm, foundation-leader evt_45rwsndm1cdgm): after D0 succeeded, the first-rejection law's rejected-head/Invalid-tail arm required this equality and could reach it neither definitionally (Refl: not convertible) nor structurally (UnresolvedCon NonEmptyCons). CAT-SCHEMA-LAWS was rescoped in place to its two elaborating laws rather than held; the first-rejection law is deferred behind this node. Steward-filed per COORDINATION section 2."
---

# Publish the head-of-append law where the type lives

## Settled inputs -- measured at `1bd3ad8f1`. Do not re-derive any of these.

**The carrier is public and the subject is already `pub fn`.**
`Data/Collections/NonEmpty.ken.md:39` is
`pub data NonEmpty a = NonEmptyCons a (List a)`; `:65` is
`pub fn nonempty_append`. The ring's `UnresolvedCon { name: "NonEmptyCons" }`
was an import-scope result in the client, not a privacy property of this
package: the client's authorized selector list carries three names -- `NonEmpty`,
`nonempty_append`, `nonempty_cons` -- and the
constructor is not among them.

**An attached `pub proof` travels with the function selector, and the live
example is inside this same file.** `NonEmpty.ken.md:37` imports only the
function `list_append` from `Data.Collections.Derived`, and the body of this
package's own `assoc` proof then consumes `list_append::assoc`.
`Derived.ken.md:85` and `:227` publish that as `pub fn list_append` plus
`pub proof assoc for list_append`. This is one observed example, which is why
AC-2 below measures the consumption rather than assuming it.

**The definitional content is already there.** `nonempty_append` (`:65`)
destructures both arguments and rebuilds with the FIRST argument's head:
`NonEmptyCons a x (list_append a rest (Cons a y more))`. `nonempty_head`
(`:45`) returns that head. So the law needs a case split on both arguments and
no transport; the split is available here and is exactly what a
selector-holding client cannot perform.

## Deliverable

One attached public law on the existing function. No new function, carrier,
import, instance, primitive, or export-list entry.

    pub proof head_left for nonempty_append
      : nonempty_head a (nonempty_append a xs ys) = nonempty_head a xs

State the GENERAL law, not the client's specialization. The specialization the
deferred first-rejection law needs follows from it by unfolding, because
`nonempty_head a (nonempty_cons a x Nil)` reduces to `x` definitionally.
Proving only the specialization would publish a weaker law and leave the next
client to re-derive it.

## Acceptance criteria

**AC-1 -- the general law is the one proved, and a wrong-side mutation REDs.**
Mutate the conclusion's right-hand side to `nonempty_head a ys` and the proof
MUST go red. A law that still checks under that swap is not this law.

**AC-2 -- client reachability is MEASURED, not assumed.** From
`Application/Input/Schema.ken.md`, with its existing three-selector `NonEmpty`
import BYTE-UNCHANGED, `nonempty_append::head_left` must resolve. Report the
exact diagnostic if it does not, and stop.

**This AC exists because the settled input above is an inference from a single
observed example.** The Steward's claim that an attached `pub proof` travels
with the function selector is measured once, in one direction, in one package.
It is the same class of unmeasured expectation that broke CAT-SCHEMA-LAWS' own
premise. Measure it here rather than discovering it in the next client.

**AC-3 -- report the public-surface census delta for
`Data.Collections.NonEmpty`.** Adding an attached law to an
already-public function should publish no new name. Report the added and
removed name counts rather than asserting them; a non-zero added-name count is
a Steward stop.

## Stop conditions

Hand back rather than work around if either holds:

- **The general law is not provable with the authorized means.** Report the
  diagnostic, then prove the client's specialization instead and say so
  plainly. Do not add a helper function, a carrier, an import, or a postulate
  to reach the general form.
- **AC-2 fails.** Then attached proofs do not travel with a function selector,
  the settled input above is wrong, and this node is the wrong shape. That is a
  frame amendment and it is the Steward's to author. Do not widen the client's
  selector list to compensate.

## Not this node

- Publishing this package's existing `proof assoc for nonempty_append`
  (`:95`), which is plain `proof` on a `pub fn` subject. That gap is real
  against the catalog-proof objective and belongs to this package's own laws
  node, not to a precursor cut to unblock a client.
- Any edit to `Application/Input/Schema.ken.md`, including its import list.
- The deferred first-rejection law itself, which returns to CAT-SCHEMA-LAWS as
  a named successor once this lands.
