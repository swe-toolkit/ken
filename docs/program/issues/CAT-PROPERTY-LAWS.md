---
id: CAT-PROPERTY-LAWS
title: "check_samples stops at the first false predicate, and that FIRST is the whole claim -- yet nothing in the package states it: give the runner soundness, success completeness, and a first-counterexample characterization whose strict-ordering premise is shown to be load-bearing, over the existing Gen list and error-biased Result, with no new trust"
status: merged
owner: foundation
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Selected and framed by the Steward 2026-09-20 as L3's successor while CAT-PROCESS-ARGUMENTS-LAWS was in full CI. Chosen after measuring the alternatives rather than reading the survey's one-line recommendations: CAT-CONSOLE-TEXT-LAWS was REJECTED as next because Capability/Console/Text.ken.md's four helpers ARE their own definitions, so the reducible half is refl and the only non-trivial claim (exactly one newline) runs through bytes_encode and list_char_to_string, which that same survey classifies as opaque/kernel-Neutral TCB exports -- that node is vacuous or blocked on a TCB-contract question, not a Ken proof gap. Property is the smallest remaining gap that is genuine Ken structural induction with every precursor landed. All current-code facts measured at origin/main 085192c5c6a9eb4c5c584677cd121097fa2c9b03."
---

# Property runner laws

`Tooling/Testing/Property.ken.md` ships a finite-sample property runner whose
entire behavioral guarantee is three executable `Bool` constants and one
elaborator acceptance test. The package says so itself (`:212`): the witnesses
"are executable `Bool` computations rather than proof terms." Give the runner
the general laws over the representation it already ships.

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## Settled inputs -- measured at `085192c5c`. Do not re-derive.

**The function under proof is four lines of ordinary structural recursion**
(`:47`):

```ken
fn check_samples (a : Type) (samples : List a) (predicate : a → Bool) : Result a Unit =
  match samples {
    Nil ↦ Ok a Unit MkUnit;
    Cons sample rest ↦
      match predicate sample {
        True ↦ check_samples a rest predicate;
        False ↦ Err a Unit sample
      }
  }
```

`check` (`:57`) routes through `Gen`: `check a g p = check_samples a
(gen_samples a g) p`, with `data Gen a = MkGen (List a)` (`:35`),
`gen_from_list` (`:37`) and `gen_samples` (`:39`) a transparent one-field
wrapper pair.

**THE CLAIM IS "FIRST", AND "FIRST" IS WHAT NOTHING STATES.** That `Err`
carries *a* failing sample is weak and nearly free. That it carries the
*earliest* failing sample is the runner's actual contract, and it is the part
a proof can get wrong while still checking. AC-2 exists solely to force that.

**The ordering vocabulary already exists in the package.** `property_nat_lt`
(`:123`) is a local structural `Nat` strict-less-than on the existing
definitions. Use it. Do not add a comparison, and do not import one.

**The lookup precursors are landed, which is why this node is selected now.**
`Data/Collections/Derived.ken.md` exports `pub fn nth` (`:91`) with two
attached public laws, `nth::some_below_length` (`:177`) and
`nth::at_or_beyond_is_none` (`:191`). State every element claim through
`nth a i xs = Some a v` rather than inventing an "all elements" predicate.

**Widening the existing import line is AUTHORIZED, and this is a ruling, not a
guess.** `:33` already reads `import Data.Collections.Derived (length)`;
extending it to include `nth` draws on landed `pub` exports of a module this
package already depends on. That is a widening of an existing edge, not a new
module edge. Do not add a second import line for a different module.

**The package has NO public surface: zero occurrences of `pub`.** So attached
proofs here are private, and private proof machinery matches this package's
posture BY CONSTRUCTION -- established by measuring this package, not carried
from any earlier node. Do not add `pub` to anything. Do not create a public
surface where none exists.

**The package's own witness gives the laws ground truth to hit.**
`reject_every_byte_sample` (`:193`) rejects everything, `gen_bytes` (`:86`) has
`empty_byte_sample` (`:195`) first, and `first_counterexample_witness` (`:199`)
asserts `check Bytes gen_bytes reject_every_byte_sample` fails with exactly
that empty sample -- index 0.

**The external evidence being replaced** is
`crates/ken-elaborator/tests/cat_property_acceptance.rs::property_finite_sample_witnesses_retain_behavior`.
That test stays; it stops being the guarantee.

## Deliverable

Attached proofs on the existing `check_samples` and `check`:

- **soundness** -- from `check_samples a xs p = Ok a Unit MkUnit`, and
  `nth a i xs = Some a v` for any `i`, conclude `p v = True`.
- **success completeness** -- from the hypothesis that every `i`, `v` with
  `nth a i xs = Some a v` satisfies `p v = True`, conclude
  `check_samples a xs p = Ok a Unit MkUnit`.
- **first-counterexample** -- from `check_samples a xs p = Err a Unit x`,
  recover an index `i` with `nth a i xs = Some a x`, `p x = False`, AND that
  every earlier element passed: for all `j`, `v`, from
  `property_nat_lt j i = True` and `nth a j xs = Some a v`, conclude
  `p v = True`.
- **runner coherence** -- `check a (gen_from_list a xs) p = check_samples a xs p`
  and `gen_samples a (gen_from_list a xs) = xs`.

The index recovery may use a predicative proposition-level existential
eliminator. That SHAPE is precedented; no earlier node's authorization is
carried here, and the posture ruling above is what licenses private helpers.

No new function, carrier, module, instance, primitive, postulate, `Axiom`, or
trusted entry. `trusted_base()` delta stays zero (`:254`). No edit to any
other package.

## Acceptance criteria

**AC-1 -- soundness and completeness are separately load-bearing.** Mutation,
restored byte-exact: change soundness's conclusion from `p v = True` to
`p v = False` and it must RED. Report the diagnostic.

**AC-2 -- the strict-ordering premise must be SHOWN to carry weight.** Delete
the `property_nat_lt j i = True` premise from the first-counterexample law and
re-check. It MUST RED. If the law still checks without that premise, it is not
characterizing the FIRST counterexample -- it is stating the far weaker "some
element failed", and that is a FAILURE of this node, not a pass. Report the
result either way; this AC is the only thing separating the two.

**AC-3 -- instantiate at the package's own witness.** Derive, through the
first-counterexample law and not by computation, that
`check Bytes gen_bytes reject_every_byte_sample` yields `Err` at index 0 with
`empty_byte_sample`. A law that cannot reach the package's existing witness
does not satisfy this.

**AC-4 -- report the census delta exactly.** Re-run
`catalog_ambient_passthrough_migration_census`
(`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`). The
`Tooling.Testing.Property` row currently holds exactly 15 ambient names: `And`,
`Bottom`, `Equal`, `MkUnit`, `Prop`, `Proved`, `Top`, `Unit`, `and_fst`,
`and_intro`, `and_snd`, `eqChar`, `is_sorted`, `leqChar`, `map`. That row MAY
move -- report its exact added and removed names and why. Any OTHER row moving
is a Steward stop, not a mirroring update.

## Stop condition

Hand back rather than work around if either holds:

- a law cannot be stated without adding `pub`, a new module import, or a new
  production function. Do not restate `check_samples` in a helper to make the
  induction go through.
- the first-counterexample law cannot be stated over `property_nat_lt` and
  `nth` as they are landed. Report what is missing; a new ordering or lookup
  primitive is a different node.

## Not this node

- The cursor slice (`ByteCursor`, `cursor_progress`, the stuck-advance mutant)
  and its two witnesses. Adjacent, separately surveyed, and not the runner gap.
- `gen_map` functor laws or any `Gen` algebra beyond the coherence equation.
- Replacing or weakening `cat_property_acceptance.rs`.
- Generalizing beyond finite sample lists to any generator notion.
