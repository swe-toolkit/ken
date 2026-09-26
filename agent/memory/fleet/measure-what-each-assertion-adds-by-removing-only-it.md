---
name: measure-what-each-assertion-adds-by-removing-only-it
description: An assertion's contribution is what reddens when only it is removed, not whether the test reddens. Ablate a repair's new assertion and re-run the named mutant; an aggregate summed over the qualifying subset restates the per-case checks; an operand pin on a guard-determined fixture restates the class check; a tuple can have one discriminating member. Ask each assertion what it alone can tell apart.
metadata:
  type: feedback
---

# Measure what each assertion adds by removing only it

A mutant that reddens the test says the suite catches it. It does not say
**which assertion** caught it, and a repair, an aggregate or a tuple member is
accountable for its own contribution. Four Adversary findings, 2026-08-15 to
2026-08-18, merged as one mechanism: ask of each assertion what it alone can
tell apart, and measure by removing only it.

## Ablate the new assertion, then re-run the mutant

**Measured 2026-08-18 on `8f09b122b`**, the repair for the finding that an
arrival bit partitions mutants by line position, not behaviour (*"the bit
records arrival, so an early return one line lower passes"*). The repair moved
the recorder one statement down, made it a counter, and asserted its value. The
named mutant reddened; the counter was not what reddened it. Wrap the new
assertion in `if std::env::var("...").is_err() { ... }` and run the same mutant
twice:

| run | verdict |
|---|---|
| mutant, new assertion present | red on the new assertion, `left: 0  right: 1` |
| mutant, new assertion ablated | still red, on a pre-existing behavioural assertion |

Net new discriminating power for the mutant the repair was built for: zero. One
env switch, one extra run. Do this before accepting any "the instrument now
discriminates" claim.

- **Re-run the original finding at the new offset.** The same early `return
  Ok(None)`, placed after the new counter and before the recursion, satisfied
  the new assertion. The blind region moved down by exactly the distance the
  recorder moved; everything the deliverable exists to do stayed below the last
  observation. A repair that relocates an observation reproduces its own bug at
  the new offset ([[a-fix-can-reproduce-its-own-bug-one-layer-up]]; the
  original finding is in
  [[a-recorder-witnesses-the-line-it-sits-on-not-the-mechanism]]).
- **Prove the entailment and state its limit.** The witness's refusal string had
  exactly one producer in `crates/`, inside a branch reachable only through the
  recursive call below the recorder, so *error text, branch entered, one entry*
  already entailed *counter >= 1*. The assertion is `== 1`; a mutant walking two
  arms would be caught by the exact count alone, and none was built. So the
  claim was *"on both mutants in this family the discrimination came from the
  error text"*, not *"the assertion is redundant"*.
- **Credit the halves that landed.** The same merge renamed the bit from a verb
  it never observed (`match_descent`) to what it records
  (`match_branch_entered`) and relabelled the kill switch as the recorder
  positive control it always was. Say so first; a finding that reads as "the
  whole deliverable failed" when two of three claims hold is discounted
  wholesale.

## An aggregate over the qualified subset restates the per-case checks

**Measured 2026-08-15 on `66715f9fb`**, fresh control machinery. The population
is selected at runtime (`NonZeroUsize::new(*arrivals)` qualifies a case), and
both emptiness holes were properly guarded. But both aggregate counters
incremented **inside** the qualifying `if let`:

| assertion | why it cannot fail independently |
|---|---|
| `aggregate_arrivals >= qualified_cases` | every summed case has `arrivals >= 1` because that is what qualified it |
| `aggregate_forwards == aggregate_arrivals` | the per-case equality already forces it over exactly the summed set |

An aggregate exists to catch a case that contributed without being counted, and
an aggregate over the counted subset can never see one. **The repair is two
lines moved**: sum over all cases including the complement; the values are
unchanged today and a complement case that started contributing would red.

- **Read an assertion's message for the word "derived"** (*"the aggregate
  derived from the qualified per-case rows disagrees"*): the author is saying
  the check is downstream of another. This was the third aggregate-restates-
  per-case in one subsystem.
- **Propose the move, not the deletion.** A vacuous assertion invites removal,
  which loses the slot where the real check goes. Ask what small change makes an
  entailed assertion independent before recommending retirement; the entailment
  is usually a scoping accident.
- **Name an improvement against the earlier defect.** The same validator
  recomputed its subject rather than taking the value under test as input, the
  property missing on a sibling re-derivation
  ([[a-pin-cannot-disagree-with-its-own-source]]). Saying so turns a past
  finding into a standard the ring can see itself meeting.

## An operand pin is redundant when the fixture's error class is guard-determined

**Measured 2026-08-17 on `000d69663` (ds5b).** A carve-out deleted two operand
assertions (`expected_dbg.contains("@9")`, `found_dbg.contains("@4")`), leaving
`KernelError::TypeMismatch { .. } => {}`. The obvious hypothesis, filed twice:
a same-class regression that moves the operands is now green. Two guard
mutations refuted it:

| mutation | result |
|---|---|
| `elab.rs:3067` skip forced always (`if true \|\| …`) | FAILED |
| `elab.rs:2251` region off-by-one (`..len().saturating_sub(1)`) | FAILED; the probe print never fired, so it left the `TypeMismatch` arm entirely |

Both changed the error **class**, not the operands. On this fixture the
rejection route is decided by the guard, so the operands cannot vary
independently of the class, and the operand pin was a second spelling of the
class check. The deletion was right.

- **Before filing "this deletion loses discriminating power", ask what the
  operand can vary with.** If every reachable perturbation that moves the
  operand also moves the class, the pin adds nothing and its brittleness is
  pure cost.
- **Mutate the guard; do not enumerate operand shapes.** The first pass reasoned
  about which `found` strings satisfy a substring conjunction, a reading of the
  encoding rather than a probe of it. Two guard mutations settled it in one
  build.
- **Report a refuted attack**, naming both mutations; it tells the next pass
  which approaches are spent.
- **Sweep the deletion's blast radius into prose.** A doc line still read
  *"(asserted by the control below)"* about operands no longer asserted, and an
  `other =>` panic still named *"convoy class @9 vs @4"*. A `panic!` stating a
  check that is not performed misdiagnoses at the one moment someone reads a
  failure.

## A tuple can have one discriminating member, and it is the blind one

**Measured 2026-08-15 on `00fad9da9`**, extending a carried finding: a boolean
asserted `false` is also its initialization value, so `false` conflates
"refused" with "never ran", and the rows pass with the validator removed.
Confirmed by mechanism: row creation and outcome recording are two calls, and
deleting the validator deletes only the second. **A default-valued assertion is
deletion-blind exactly when the record's creation is independent of the event
being recorded.**

| member | distinguishes the phase? |
|---|---|
| a count asserted `0` | no; its own message says a zero cannot tell which phase fired |
| a named-authority `true` | no; both phases emit the same named refusal |
| the flagged boolean | yes, and it is the blind one |

- **Ask of each tuple member what it could tell apart, not whether it is
  pinned.** A tuple reads as N checks and is often one check plus N-1 context
  values. Here the repair is to make the flagged member tri-state, not to add a
  fourth.
- **Scope the repair to the record's birth convention.** The row is born with
  three absence-valued fields (two `false`, one `None`); any future assertion on
  the siblings inherits the conflation. A later node's empty-scan fallback that
  could not tell "absent" from "slot missing" was the second two-state
  observation for a three-state question in the subsystem; when two nodes owe
  the same conversion, state it once as a convention.
- **Look for a second silent route to the default.** The recorder opens `let
  Some(index) = index else { return }`, so a `None` index is a no-op even when
  the validator runs. The `[row]` destructure catches "no row"; nothing catches
  "row created, outcome never recorded".
- **A mutation proof covers the direction it perturbs.** The ring's mutation
  flipped the flag to `true` (the admit direction); the delete direction had no
  observation. Name the uncovered direction, since "mutation-proved" reads as
  both.

Related: [[rank-a-controls-assertions-by-what-survives-a-redundancy-trim]] (the
durability question: which assertion a cleanup keeps),
[[a-mutation-that-reddens-does-not-confirm-which-detector-caught-it]], and
[[differential-verify-which-mechanism-is-the-net]].
