---
id: RT-AUTHORITY-CONTRACT-MISDESCRIBES-ITS-POPULATION
title: "generated_constructor_authorities is NOT restricted to generated-context constructors -- a user's Result::Ok acquires an authority exactly like a generated-context one -- but the field name, its doc comment, and the registrar's own error string all say generated-context. The doc comment is the artifact the entire exclude-or-refuse obligation derives from, so a faithful derivation from it transmits the misdescription with full authority. That is what happened. (This title said 'ranges over EVERY carried constructor' until the Architect measured both writers: exit_failure returns early and the other registrar is gated, so the universal is false at a measured site while the finding is untouched.)"
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Found by runtime-implementer's guard read at evt_wtmmwstk76ze and confirmed independently at source by the Architect at evt_53p7m3s2f6t8q (2026-09-15), which brought it to the Steward because it changes a filed node's premise. Architect named the contract comment as a separate candidate defect and explicitly did not route it: 'I am naming that, not routing it, same fence as the trap message.' Steward-filed per COORDINATION section 2."
---

## The fact, measured

`crates/ken-runtime/src/cranelift_backend/lowering/core.rs:14363` at
`b0a7c2945`:

    pub(super) fn transfer_constructor_operands(
        &mut self, builder, origin: StaticOriginId,
        constructor: &str, args: &[LoweringOperand],
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        if constructor == self.process_symbols.exit_failure { ... early return ... }
        let constructor_identity =
            self.static_transition_plan.constructor_symbol_identity(origin)?;
        let identity = constructor_identity.tag_abi_word()?;
        ...
        self.emit_carrier_store_tag_id(builder, word, identity)?;
        ...
        self.register_generated_constructor_authority(constructor_identity, word)?;

It takes a constructor **by name** plus an origin, derives the identity from the
static transition plan, and its only early return is the process `exit_failure`
special case. **There is no generated-context gate between the entry and the
registrar.**

⇒ **`generated_constructor_authorities` is NOT restricted to generated-context
constructors.** A user's `Result::Ok` acquires an authority exactly as a
generated-context one does. That is the whole finding, and every deliverable
below rests on it.

**The population stated exactly, because the obvious stronger reading is false
at a measured site.** It is *every carried constructor reaching the registrar
through `transfer_constructor_operands`*, **minus** the `exit_failure` case
carrying a single `Carried` argument — which returns early, above — **plus** the
synthesized-identity constructors from the other registrar.

**Do not restate this as "ranges over every carried constructor."** This node
said exactly that until the Architect measured both writers and corrected it
(`evt_2k1de0ch6d7qa`, sites at `b0a7c2945`). The universal is refuted by the
early return that this node's own code quote shows three lines above it — the
node stated its counterexample and then wrote the claim it refutes. Nothing in
D0-D3 or AC-1/AC-2 needs the universal; they need only that the population is
wider than generated-context and contains ordinary user constructors. **A
criterion built on the universal would be falsifiable by a reader who checks
it**, and would look like the premise collapsing when the argument never rested
there.

**The two writers do not share a population.** The other registrar
(`calls.rs:1895`) is gated on `Lowered::Constructor { synthesized_identity:
Some(..) }` — and this path constructs its probe with `synthesized_identity:
None`.

## Three signals say otherwise, and all three are wrong about it

    the field NAME        generated_constructor_authorities
    its DOC COMMENT       "an identity different from a generated function's
                           demanded Result"
    the registrar's ERROR "one generated-context Result word acquired
                           incompatible terminal authorities"

Name, prose, and diagnostic all agree with each other and disagree with the
code. **Mutual corroboration among artifacts that share an author is not
evidence**; it is one claim counted three times.

## Why the doc comment is the load-bearing half

`lowering/mod.rs:1314-1318` is the artifact the **entire exclude-or-refuse
obligation derives from**:

> the finished proof must either exclude its path or refuse, and it must never
> relabel the word

That prose describes the field's population as generated-function demanded
Results. **It is not describing the population; it is describing a special case
of it.**

⇒ **A faithful derivation from a contract that misdescribes its own population
transmits the defect with full authority.** The consumer cannot detect it,
because the derivation is correct — the premise is what is wrong, and the
premise reads like law. `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP`, the
Architect's rulings, and the arc's framing all inherited "generated-context"
from this prose rather than from the code.

## Deliverables

- **D0.** The doc comment at `mod.rs:1314-1318` states the population the field
  actually has. If the obligation is meant to hold only for a **subset**, the
  comment says which subset and what distinguishes it — a contract whose scope
  is narrower than its field's must say so at the field.
- **D1.** The registrar's error string stops asserting "generated-context" over
  a population that is not generated-context, or the code gates so that it is.
  **Say which of those two the repair does.**
- **D2.** The field name is assessed against its population. **Renaming is not
  automatically correct** — if the population is genuinely everything carried,
  the name is the thing that is wrong; if the obligation is genuinely
  generated-context-only, the *gate* is what is missing. Do not rename to
  paper over an absent gate.
- **D3.** Whether the population **widened under** the contract is established
  from history, not assumed. The Architect explicitly did not claim the prose
  was wrong when written.

## Acceptance criteria

- **AC-1.** The population is stated where a reader of the contract meets it,
  and a reader who derives an obligation from that prose derives a true one.
- **AC-2.** Name, doc comment, and error string agree **with the code**, not
  merely with each other. State the check that establishes this rather than
  asserting agreement.
- **AC-3.** D3's history question is answered with a citation, or recorded as
  unestablished with the reason. "Probably widened" is not an answer.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## What this does to RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP

**It changes that node's premise. FOLDED there as of `48de11fe5`** — it is item
1 of that node's "HELD AGAINST THE NEXT TOUCH" section, carrying the corrected
population statement above. The backstop node's D0 requires a witness carrying a
"foreign identity" — a producer authority whose identity differs from the
demanded one, evaluated at `:3424` against `body.authorities`, which is this map.

Because the map is **not restricted to generated-context constructors**, an
ordinary user constructor whose identity simply is not the demanded one
satisfies that predicate. So D0's first conjunct admits benign instances.

That is the mirror of the empty-search problem already recorded against D0: an
**empty** witness search would be misread as establishing no witness exists, and
a **non-empty** one is now equally misreadable, because the predicate no longer
separates a generated-context anomaly from an ordinary non-demanded
constructor. **Both of D0's conjuncts have now been mischaracterized by a
name** — the first by this map, the second by `funcid60` being the only body
reaching the proof path.

The backstop node is `draft` and deliberately unreleased, so nothing can be
picked up on the false premise. This is held against its next touch rather than
forced into an amendment cycle.

**Not affected:** the four-arms-one-sink finding. It stands on the loop's exit
discipline, which is independent of how wide the authority population is.

## Bounds

- **Not established that any benign instance actually reaches the proof path**
  in a real compile. That is a measurement and nobody has taken it.
- **Not asserted that the contract prose was wrong when written.** The map's
  population may have widened under it — which is D3.

## Related

- `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` — the node whose premise this
  changes; held against its next touch.
- `RESULT-NAMES-TWO-ARTIFACTS` — the same defect class in the record rather
  than at a field, and see its scope question: this instance is in **code**,
  which a doc sweep would not reach.
- `RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION` — same day, same shape: an
  artifact that names a thing without naming the population it ranges over.
