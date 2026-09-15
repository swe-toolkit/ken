---
id: RT-DISCHARGE-ARM-SUBSTITUTES-PLAN-FOR-OBSERVATION
title: "Arm 1 of the constructor-identity discharge substitutes a PLANNED identity plus realization for an OBSERVED constructor identity at the returned word. It never consults `body.authorities` at any point -- its whole discharge condition is membership of the returned word in `realized_call_words`, and `independent_contract` is `emission.row.k_ret_identity()`, a planned value off the emission row. The plan is compared against the demand, the word is checked to be initialized, and NOTHING checks what the word actually holds. The two live symptoms are the two exits of this one rule, and ONE OF THEM IS THE GATE WORKING: repairing the emission refusal toward green converts a loud refusal into a silent miscompile. The closure makes the tree REDDER, and that red population is the deliverable."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-CONSTRUCTOR-AUTHORITY-DISCHARGE]
blocks: []
github: null
origin: "Architect ruling evt_20wvshayvhvxj (2026-09-15, the two-exits predicate, and the standing prohibition on repairing cause 2 toward green), completed by evt_4tb8wvsdc93f after the Architect ran its own gating premise check at source and found the mechanism sharper than the ruling had it. The Architect directed the filing and will vote the node. The reduction that produced the evidence is runtime-implementer's: a 139-line program reaching three distinct failures by single-line edits, plus a native-vs-interpreter harness showing identical effect traces with different exit status. Steward-verified at source on 0f71ab5b9267781ae1d91bc654011cad42b926af before filing; the base-tree measurement in the banner below is the Steward's and refutes nothing in the ruling."
---

> # EVERY COORDINATE IN THIS NODE IS ON AN UNLANDED CANDIDATE. NONE OF THIS CODE IS ON `main`.
>
> **Measured by the Steward at filing**, `origin/main` =
> `ea9f4a6151ae82291950e4e7ff185b08d770a960`, candidate =
> `0f71ab5b9267781ae1d91bc654011cad42b926af` (PR #3676, the `ABI-S6 D5b` arc,
> merge-base `1dec48f33cc0697569e9e8765874fe61269332f4`):
>
> | probe in `lowering/units.rs` | `origin/main` | `0f71ab5b9` |
> |---|---|---|
> | file length | 8445 lines | 12623 lines |
> | `independent_contract` | **0 hits** | 10 hits |
> | `realized_call_words` | **0 hits** | 11 hits |
> | `k_ret_identity` | **2 hits** | 3 hits |
> | the arm-1 conditional | **absent** | `:4474` |
>
> ⇒ **The entire mechanism this node is about exists only on the candidate.**
> `:4474` on `main` is unrelated, error-free code, and nothing warns a reader
> that they are in the wrong tree — implausibility of the answer is the only
> detector, and the wrong answer here is perfectly plausible. This is the
> failure mode `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` documents at length for
> its own coordinates; it applies verbatim here.
>
> **AND NAME-PRESENCE IS NOT A RELIABLE WRONG-TREE DETECTOR HERE, WHICH IS THE
> ROW MOST WORTH READING.** Two of the three names resolve to nothing on `main`
> and would catch a misplaced reader. **`k_ret_identity` does not** — it has two
> hits on `main`, in unrelated code. It is also the name that anchors where
> `independent_contract` comes from, so a reader checking the three names gets a
> true negative twice and a **false reassurance on the one that matters most**.
> (Architect, re-measured by the Steward; both readings agree.)
>
> **Lead with the symbol name. The line number never travels without its
> anchor.** Whoever discharges this re-measures at the tip they are on.

## What this is

`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`, the parent, states the invariant in its
own title: make the discharge **the authority's consumption at the consumer**,
so that *"a hand-off with no authority is a missing VALUE and a planner error,
never a missing list entry and **never a silent pass**."*

**Arm 1 is a silent pass on a hand-off with no authority.** This node is not
new scope. It is the parent's stated invariant, unmet by the parent's own
in-flight implementation, cut out so the repair and its measured fallout do not
disappear into an already-`L` node with a held dependent set.

## The mechanism, verbatim at `0f71ab5b9`

`crates/ken-runtime/src/cranelift_backend/lowering/units.rs:4474-4523`:

```rust
let proven = if body.independent_contract == Some(identity) {
    ...
    if realized_call_words.contains(&publication.returned_word) {
        discharge_sources.insert(publication.returned_word);
        true
    } else { false }
} else {
    let cuts = derive_certified_cuts(...)?;
    let proof = prove_forwarded_value(..., &body.authorities, ...)?;
    if proof.valid && proof.grounded { ... true } else { false }
};
```

**The two arms are asymmetric. Arm 2 proves. Arm 1 does not consult
`body.authorities` at any point.** Arm 2 passes the map into
`prove_forwarded_value`; arm 1's entire discharge condition is set membership of
`publication.returned_word` in `realized_call_words`.

**And `independent_contract` is not an observation either.** At `:6962`,
`let independent_contract = emission.row.k_ret_identity()` — a **planned**
identity read off the emission row.

⇒ **Arm 1 compares the plan against the demand, checks the word is initialized,
and never checks what the word actually holds.**

**`None` means the absence of evidence, not a not-applicable marker.** The
reported identity comes from `body.authorities.get(&publication.returned_word)`
(`:4576-4578`, and again in the details tuple built at `:4587`), which is `None`
exactly when **that word has no authority entry at all**. This was the premise
the ruling was gated on; the Architect read it at source and it confirms.

That is why the trapping carrier is exactly the one with no authority. Not a
coincidence — **the arm is only reachable without one.**

### Arm 1 is not a rare branch. It is the measured path, and arm 2 never runs.

**Independent corroboration, from a measurement taken for a different question**
(`RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`, 2026-09-14, venue finding
2): in the `WRITE_ALL` population, *"both bodies carrying an
`independent_contract` are demanded under exactly one identity, and that
identity **is** their own contract, **so arm 2 never runs**."*

⇒ Wherever the planned identity matches the demand, **arm 1 is the path taken
and the proving arm is not reached at all.**

**KEEP THE TWO HALVES OF THAT APART — they have different warrants and only one
of them is a measurement** (Architect, `evt_4eerxykb24edj`):

- **That arm 1 is taken whenever plan matches demand is a fact about the
  conditional at `:4474`.** It is read off the `if`/`else` and needs no
  measurement at all. As a statement of the *rule*, "the ordinary route for
  every body that carries a contract" is exactly right.
- **What `WRITE_ALL` adds is that the antecedent is actually satisfied in a real
  population**, so the branch is trafficked rather than hypothetical. That is
  its whole contribution. **It establishes no frequency anywhere else**, and
  read as a measurement rather than as the rule, "every body" would be an
  overclaim on a two-body sample.

A claim inherits the scope of the site you checked, not the scope you stated.
The measurement was made to answer whether two callers could collide on one
body; it happens to establish that arm 1 is trafficked, and it was not taken
with this node in view, which is what makes it worth citing.

## The two exits, and why one of them must be left alone

    plan == demand AND word realized   -> arm 1 discharges -> compiles -> traps at
                                          runtime if the actual value is not what
                                          the plan said                    (cause 1)

    plan != demand OR word unrealized  -> arm 1 declines, arm 2's real proof also
                                          fails -> missing -> emission refusal
                                                                           (cause 2)

> ### CAUSE 2 IS THE GATE WORKING. NOBODY REPAIRS IT TOWARD GREEN.
>
> **Architect, `evt_20wvshayvhvxj`, and it is restated here because this is the
> single most tempting action on this board.** Cause 2 is six of seventeen
> failing checks, it has a loud specific failure, and it has an obvious-looking
> repair. Removing the emission failure **without** adding an identity
> requirement converts a loud refusal into a silent miscompile.
>
> A seat that is woken, sees a red check, and reaches for the nearest fix lands
> here. runtime-implementer named this hazard unprompted while holding
> (`evt_6gwyey6pacv43`) and was right to hold.

## What the author already saw, and why the defect is one level above it

The comment at `:4480-4488` records the hazard:

> THIS ARM DISCHARGES TOO, AND ITS WORD IS THE SAME ONE THE OTHER ARM ROOTS AT
> ... a ledger keyed per word but populated from one arm promises coverage over
> a domain its population never covers.

That note is about a **ledger**, and commit `c0de78286` acted on it — *"D2 fold:
the discharge ledger covers ARM 1 as well."* Both are correct as far as they go.

⇒ **The defect is one level up from what they guard. It is not that arm 1's
discharges are under-recorded. It is that arm 1 discharges at all without
consulting authority.** Recording a fail-open faithfully leaves it a fail-open.

## D0 — THE POPULATION. Measurement first, and it does not land.

**Apply the closure on the candidate branch, run, and report what newly
refuses. Do not land this increment.**

Arm 1 must consult `body.authorities` for `publication.returned_word` and must
refuse when there is no entry, or when the entry's identity differs from the
demand. **Realization stays necessary and stops being sufficient.**

The deliverable is a **census**: how many, and which, hand-offs carry no
constructor authority. Report it as a population with identities, not as a pass
or fail.

> **THIS INCREMENT MAKES THE TREE REDDER AND THAT IS THE POINT.** Cause 1's
> population converts from compile-and-trap into refuse-at-emission, so the
> refusal count goes **up**. Architect: *"expect the closure to surface further
> red, and that surfacing is the deliverable."*
>
> ⇒ **Do not write a workspace-green acceptance criterion on D0, and do not
> read its red as a regression.** The standing operator rule that red CI is a
> priority to fix, not to merge past, is about unexplained red. This red is the
> instrument's output, enumerated in advance. It is also the reason D0 does not
> land: a measured population is what tells us whether the upstream repair is
> small or is an arc.

## D1 — the closure proper, scoped by D0's answer

The arm-1 repair above, landed, **plus** whatever D0 shows is needed upstream to
make constructor authority actually present at the words that now refuse. D1 is
deliberately not sized here: **D0's population is what sizes it**, and sizing it
now would be the guess this node exists to replace.

## Acceptance

**AC-1. The refusal is OBSERVED, not argued for.** An input on which the
repaired arm 1 refuses, run and recorded. The parent arc has one node already
stalled for want of exactly this (`RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP`'s
`D0`), so it is stated first here.

**This node is not at that risk and the difference is worth naming.** Its
witnesses already exist: runtime-implementer's reduction is a 139-line program
reaching three distinct failures by single-line edits, with `d1/d3/d4.ken` and a
native-vs-interpreter harness in the scratchpad. A node with a witness in hand
is a check; a node without one is a documented invariant.

**AND THE WITNESS MAY NOT BE MANUFACTURED BY SYNTHESIZING AN IDENTITY.**
`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE:333-335` forbids it in terms, and the
Architect **extended that prohibition to controls** as a ruling
(`evt_5gws0pnssfqch`) rather than leaving it to be read across. This is exactly
what stalled `RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`: a mutation
could not produce its shape without synthesizing an identity, so the shape
stayed unreachable by any permitted instrument.

The governing line is that node's `:332` — *"if either is unavailable in
principle at some site, THAT IS THE FINDING."* **An in-principle unavailability
is a result to report, not an obstacle to engineer around.** If the repaired
arm 1 cannot be made to refuse by permitted means, say so and stop; do not
reach for a synthesized identity to close AC-1.

**AC-2. The census is a population, not a verdict.** Each newly-refusing
hand-off named with its target, its demanded identity, and what
`body.authorities` held at its returned word. A count alone does not satisfy
this — the distribution is what sizes `D1`.

**AC-3. Cause 2's six checks are NOT made green, and the diff is checked for
it.** State explicitly that no change removes an emission refusal without
adding an identity requirement. This is an AC because it is the failure mode the
ruling exists to prevent.

**AC-4. Arm 1 and arm 2 are shown to agree on what evidence they require.** The
asymmetry is the defect; a repair that leaves arm 1 checking something weaker
than arm 2 has relocated it. Name what each arm requires, side by side.

**AC-5. Each missing entry is classified into ONE of the four cases in the
fence below, and ONLY case (c) folds into this node.** A report of the form
*"not a differing identity, therefore this node's"* does not satisfy this — that
is the elimination the fence exists to prevent.

## THE FENCE: the six `rt_parity_native` shards are NOT established as this node's

**Architect, explicitly: the attachment is by error signature today, and a
signature that reads identically is not a mechanism identity.** Do not fold them
into this node's population before the read.

### The first filing's two-way fork was NOT A PARTITION. This replaces it.

**Architect, `evt_4eerxykb24edj`, correcting its own discriminator spec; every
claim below re-verified at source by the Steward at `0f71ab5b9`.**

The refusal loop at `:4569-4586` runs over **every** missing entry before any
details are built, and it `continue`s in three cases — no staged body (`:4570`),
no publication (`:4573`), and **`None` authority (`:4576`)**. Only a **present**
authority reaches the mismatch arm at `:4579`.

⇒ **THE DIFFERING-IDENTITY OUTCOME IS FORECLOSED BY THE SIGNATURE THAT SELECTED
THESE SHARDS.** An artifact reading *"the finished generated-Result proof graph
is not closed"* (`:4626`) has already run that loop to completion, so **no**
missing entry in it has a present-but-differing authority. The old fork's second
branch cannot coexist with the error string used to pick the six shards. It was
not merely distinguishable from the artifact — it was excluded before anyone
opened one.

**The population the signature actually admits is four cases:**

    (a) no staged body for the target      -> DROPPED from details by the
                                              filter_map at :4587, so
                                              details.len() < missing.len()
    (b) body has no publication            -> details field 3 is None
    (c) authority at returned word is None -> details field 4 is None
                                              <- THE ONLY CASE THAT FOLDS IN
    (d) authority PRESENT and its identity -> details field 4 is
        EQUALS the demand, yet the check      Some((word, identity)) with
        is still missing                      identity == the demanded one

**(d) is the live fork and neither the ruling nor the first filing named it.**
It says the returned word carries the *correct observed* constructor authority
and the proof still did not close — which points at arm 2's
`prove_forwarded_value` failing `valid && grounded`, or at the certificate match
at `:4560-4564`. **It does not point at arm 1's fail-open at all**, so those
shards are not this node's. (a) and (b) are neither mechanism; each is its own
small finding about the staged set.

> **WHY THIS IS NOT A TIDY-UP.** A reader given the old two-branch fork who
> confirms *"not a differing identity"* concludes *"therefore outcome one, folds
> in"* — and thereby routes (d), (a) and (b) into this node **silently, by
> elimination against a split that never covered them.** The criterion would
> have been satisfied correctly and still landed in the wrong column.
>
> Architect, naming its own lesson against its own spec: *an exhaustive split
> over one variable is not exhaustive over the thing you care about.*

**How to classify, per missing entry, from the artifact alone.** Field 4 of the
details tuple is `authorities.get(returned_word).map(|a| (a.word, a.identity))`,
so it is printed:

    details.len() vs missing.len()   -> (a)
    field 3 is None                  -> (b)
    field 4 is None                  -> (c)
    field 4 is Some, identity == demanded -> (d)

`body.independent_contract == Some(identity)` is still worth reporting per
entry, but it is now the **second** question, not the first.

**So the discriminator is a read, not an experiment** — the original point, and
it survives the correction intact.

(The Architect's ruling cited the mismatch arm as `:4579-4585` and the authority
read as `:4603-4607`. The first resolves exactly; the second is a field inside
the details tuple begun at `:4587`. Both name the same code — recorded because a
coordinate off by a few lines is the kind of thing a later reader silently
repairs in the wrong direction. The Architect has taken both corrections.)

## Relationship to `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` — adjacent, not duplicate

Both are fail-opens in the same discharge path, at **different sites**, and the
distinction decides where a repair goes:

| | that node | this node |
|---|---|---|
| site | the `proof.sources` loop and its `grounds.is_empty()` sink | arm 1 of the `proven` conditional |
| mechanism | a foreign identity is **detected** and silently discarded | authority is **never consulted** |
| relation to arm 2 | inside the proof arm 2 runs | bypasses arm 2 entirely |
| witness | none found; `D0` is blocked on one | in hand |

**When arm 1 is taken, `prove_forwarded_value` never runs**, so that node's sink
is unreachable on this path. They are not two views of one defect and neither
repair implies the other. Both nodes' repairs point the same direction —
refuse where the code currently proceeds — which is a reason to read them
together, not a reason to merge them.

**A consequence for that node's stalled `D0`, stated because it cuts against
its current framing.** Arm 1 is taken whenever the plan matches the demand — the
rule, read off the conditional — so the population that reaches arm 2's proof at
all is smaller than a reader of that node would assume, and its witness hunt is
correspondingly harder for a reason that has nothing to do with foreign
identities. **How much smaller is not measured**, here or anywhere: the
`WRITE_ALL` reading establishes that arm 1 is trafficked, not how much traffic
arm 2 retains. Whoever picks that node up should measure its reachable
population before spending more on the witness. **Not a finding of this node;
a pointer, and it is unverified in that direction.**

Third sibling, distinct again: `RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY`
asks whether the *ledger* has a net — whether two callers can demand different
identities of one body. That is a reachability question about the ledger's
purpose, not about what arm 1 consults. It supplies this node with evidence
(above) and takes none from it.

## Why `draft` and not released

**The `rt_parity` discriminator decides this node's population**, and
runtime-implementer is running it now. Releasing before it lands invites the
ring to fold six checks in on a signature match, which is the one thing the
Architect fenced.

**Release condition:** the discriminator result is posted. Nothing else gates
it — `D0` does not depend on that read, only its scope does — so this is a short
hold, not a queue.

The ring is not idle behind this: the discriminator is the implementer's task,
and it is the input.

## Contention

`crates/ken-runtime/src/cranelift_backend/lowering/units.rs`, on the `ABI-S6
D5b` candidate branch. **That is the same file and the same branch the D5b arc
is respinning on**, so `D0` is a branch-local measurement that must not be
released as a parallel candidate against it. Sequence it with the D5b ring, not
beside it.

`D2`, `R1`, `R2` and entry-18 of the parent node are held pending D5b
stabilization; this node joins that set rather than jumping it.
