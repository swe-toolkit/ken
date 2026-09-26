---
name: a-contains-assertion-on-a-message-excludes-less-than-it-reads
description: A pin over rendered message text has four independent axes, and a `contains` fragment is weak on each - right-hand anchoring (`"expected: Dg67"` also matches `Dg670`), fragment versus exact equality (a synonym slips through), hardcoded versus computed identifiers, and add versus remove (a positive `contains` cannot guard a phrasing a ruling deleted). Grade each axis separately, and census the whole file before filing one instance.
metadata:
  type: feedback
---

# A `contains` assertion on a message excludes less than it reads

Refusal-message pins are everywhere in this repo, and most are
`msg.contains(<fragment>)`. A fragment states what must be present. It says
little about what else may be there, where the match ends, or whether an
identifier inside it is the right one. Grade a message pin on four axes, each
measured separately:

| axis | strict | loose |
|---|---|---|
| **anchoring** | the fragment ends at a token edge | an open right edge matches a longer token |
| **wording** | exact equality — any rewording reds | fragment — a synonym slips through |
| **identifiers** | computed from the run — id churn cannot break it | hardcoded — renumbering reds a true row |
| **direction** | positive and negative clauses | positive only — a removed phrasing can come back |

## Anchoring: the right-hand boundary

**Measured 2026-08-15 on `beb31566b`**, a measurement-landed-as-a-test whose
author called it *"non-vacuous by construction … a different failure cannot pass
it."* Three `msg.contains(..)` assertions over one formatted error:

| assertion | pattern | anchored? |
|---|---|---|
| `found` | `"found: ((Dg574 Dg67) @8)"` | yes — the trailing `)` stops `@80)` matching |
| `expected` | `"expected: Dg67"` | **no** — matches `expected: Dg670, …` |

The assertion whose stated job is *"a different expected operand means this is
NOT the failure this pins"* can pass on a different expected operand. The
`found` pattern is anchored **by accident** of ending on punctuation, which is
why the asymmetry within one test is the tell. ⇒ **When several patterns match
the same rendered format, compare their right-hand boundaries**; the one that
stops at a token edge is safe and the others were written the same way with
different luck. One character fixes it: add the delimiter the format already
emits.

**Bound the reachability rather than asserting it.** The prelude registered
~452 globals and ids already ran past `Dg574`, so four-digit ids beginning
`Dg67` were about a hundred globals away — not constructible then, entirely
ordinary to reach.

**Two recorded hazards can be one event pointing opposite ways.** The node also
recorded that an absolute de Bruijn position in the same message means an
unrelated prelude edit reds the test spuriously. The same prelude growth that
shifts that index mints the ids that make the unanchored pattern match wrongly:
**a loud spurious red on one assertion and a silent false green on another,
simultaneously.** The loud one gets investigated; the maintainer re-measuring
after it is looking at a sibling assertion that has quietly stopped
discriminating. When a node records two hazards, check whether one trigger
fires both, and in which directions.

**Check the message-as-site claim every time.** A pin that claims to fix *"the
classification site"* while matching only message text holds only if the text
is site-unique (here it was: one production emitter, the other occurrence being
the assertion). One grep; this check came back clean twice and found a real gap
once, and a message emitted at two sites turns a "site pin" into a "message
pin" with no other change.

### Census the file before filing one instance

**Measured 2026-08-15 on `7b11bbd84`.** The Steward's notification named one
unanchored substring and the Adversary had named another on the previous node.
Censusing every `contains(..)` operand assertion in the file:

| pattern | anchored? | who named it |
|---|---|---|
| `"expected: Dg67"` | no | the Adversary, carried on the **previous** node |
| `"found: ((… ) @8)"` | yes — by a trailing `)` | — |
| `"@9"` | no | **nobody** |
| `"@4"` | no | the Steward, carried on **this** node |

Three of four unanchored and the fourth anchored by accident: a **file-level
convention**, not N slips, carried in pieces on two nodes with a third instance
named by nobody. ⇒ **When a defect has a sibling on another node's carry list,
census the file before filing**; the scoping question is whether the successor
fixes the convention or the instances, and only a census can pose it. Any
reviewer naming a site rather than a class leaves the class scoped to whoever
reads their sentence (the Adversary was on the wrong side of this two weeks
earlier: it named one slice-index site and the Architect showed it was the
best-invariant member of three). File-level conventions cluster, because both
come from copying a neighbour without its context: the same file had three
artifacts resting on one constructor being index-impossible at a refined type,
with the warning written on one of them.

**A one-armed control over two disjoint outcome classes is complete.** The same
fixture pinned *"shipped guard versus prohibited alternative"* with the
prohibited arm in prose only (running it needs production code the node
forbids). The first read — the "versus" half is a comment — was wrong: the two
outcomes are different **error classes**, so reintroducing the alternative fails
the fixture's own `match` arm with *"got a different error"*. Before filing
"this control only covers one side", check whether the sides are
distinguishable by the shape the assertion already destructures.

## Wording: fragment versus exact is the choice a ruling skips

**Measured 2026-08-17 on a `+1/-0` clause.** A pin forbade one phrasing of a
refuted claim; a synonym passes. Ruled not a defect because *"string matching
is the only available predicate here."* Both halves measured: the forbidden
phrase reds the pin; the synonym passes all clauses. The limitation is real, and
the ruling's premise does not reach its conclusion. The distinction is not
string versus semantic, it is **fragment versus exact**: the sibling pin in the
same file used full equality and is immune to synonyms. The fragment pin used
`contains` because its message interpolates runtime values, and every one was
knowable to the test — including an id **returned by the very call the test
makes and discarded with `.expect(...)`**. Binding it makes exact matching
constructible, and the gap vanishes by construction.

⇒ **When a ruling says a weak predicate is the only one available, ask what the
neighbouring assertion does.** State the stronger predicate's cost in the same
breath — exact equality reds on any message edit, including benign rewording —
and note that the sibling already accepts it, or the reply will be that the cost
is why it was rejected. **Documenting a gap and closing it are different
dispositions**: a follow-on dispatched to record the limitation reads as
settled, so check whether it was removable before the recording lands.

## Identifiers: a separate axis from wording

**Measured 2026-08-17**, after a pin was rewritten to assert a full message
reconstructed from its own interpolations; mutation verified it (the synonym
that passed the fragment guard an hour earlier now reds — closed by
construction, not documented). A control can be strict on one axis and loose on
the other, so *"has it backwards"* is rarely the right verdict: a sibling so
described was **exact on wording** and hardcoded only on identifiers — one axis
wrong. Measure both axes before accepting a characterization, because the
remedy's size depends on which is wrong.

**Check whether the remedy transfers.** The rewritten pin builds a synthetic
fixture and knows its root, so the plan API recovers everything. The sibling
compiles real fixtures, hardcodes interior origins, and gets its rows through a
helper returning only strings: same shape, materially harder work. And accept a
correction to your own citation precisely: the sibling pin cited as proof that
a stronger predicate existed compared a **constant** message with no
interpolation — the idiom transferred, the difficulty did not. A premise can
fall on the half you checked while your example was about the other half; name
which half carried the argument (here, that the id was recoverable).

## Direction: a restoration pin guards the clauses added, not the one removed

**Measured 2026-08-17** on the last artifact standing before a lane deletion. A
refusal message had two clauses restored after an earlier finding, and a pin
asserted both: `if reason.contains("<clause A>") && reason.contains("<clause B>")`.
A wording correction became an assertion — the strongest outcome such a finding
can have; say so. But the message existed because a **third** phrasing was
**deleted** by a ruling for inviting a refuted reading, and that phrasing can be
re-added beside both pinned clauses with every assertion passing.

⇒ **When a pin encodes a wording ruling, check whether the ruling was ADD or
REMOVE.** A removal needs a negative assertion; one `&& !reason.contains(...)`
closes it. A pin that becomes the sole assertion of a text inherits every
question the retired ones answered, including the ones it was not built for.
When a pin is the last one, **reproduce its discrimination claim against
production yourself**: remove one refusal, then disable the other, and require
one red with the other left meaningful, both directions. And check its stated
purpose mechanically: both pins were re-homed to survive an enum's deletion, so
grep both bodies for the enum and its helper — absence is the deliverable.

See also
[[anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count]]
(the same anchoring failure in a census pattern) and
[[an-assertion-message-is-an-output-so-fire-it-and-check-it-names-the-cause]].
