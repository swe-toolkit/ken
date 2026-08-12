---
name: a-rejected-aggregate-recurs-inside-the-key-that-repaired-it
description: A count-vs-pairing rejection repaired by adding a key leaves the aggregate intact INSIDE each key whenever one key admits several members — and the repair's own doc is where that admission is written down, because it is the justification for keying
---

# A rejected aggregate recurs inside the key that repaired it

**Measured 2026-08-12 on `45addeaf` (`D2k-1b-i`).**

An Architect rejection (`dec_2xxj1zrwmgjdb`) killed a conservation ledger whose
`note_consuming_call` incremented **one compile-wide scalar**, closing on
`consuming_calls >= entries.len()`. The stated reason is the reusable sentence:

> **A count over a population is not a pairing within it.**

The repair keyed every relation by the planner's
`child_static_origin(owner, position)` — genuinely occurrence-keyed, and its
control set proves the cross-field case refuses at the point of call. **Then
the entry's own doc says this, as the justification for keying rather than
listing:**

> *the same static occurrence can be lowered more than once … Those are two
> transports of one occurrence, and **each owes its own consumption**.*

And `close()` checks `entry.consumptions != entry.rebinds`. **Within one key
that is an aggregate over that key's transports** — the rejected relation, at
the next granularity down. At `rebinds = 2`, one transport consumed twice and
its sibling dropped closes green at `2 == 2`.

## The rule

⇒ **When a count-vs-pairing rejection is repaired by introducing a key, ask
whether one key can hold several members.** If it can, the aggregate is intact
inside the key and only its scope shrank. Keying converts *"a count over the
whole compile"* into *"a count per occurrence"*, which is strictly better and
is **not** the pairing the rejection demanded.

⚠ **The admission is in the repair's own doc, and it is load-bearing there.**
This is not a fact you have to hunt: the author must explain why the structure
is a `BTreeMap` keyed by X rather than a `Vec` of X, and **the only reason to
key is that the same X arrives more than once.** So the sentence justifying the
repair is the sentence establishing that the repair is incomplete. Read the
justification for the data structure, not just the check over it.

⇒ Generalises past ledgers: any `HashMap<K, Count>` replacing a bare `Count`,
any per-file tally replacing a per-run one, any per-request quota replacing a
global one. **Ask what the multiplicity of K is.** If K is unique by
construction, the repair is complete and saying so is worth one sentence; if
the doc says K repeats, it is not.

## The control set's near miss is the tell

The committed control has six rows and its row 3 is *"two transports of one
occurrence, one consumption"* — `rebinds = 2, consumptions = 1`, asserted to
refuse. **The author reached the multi-transport case and tested the direction
the representation can see.** The unsound direction is `rebinds = 2,
consumptions = 2` paid by one transport, and the entry holds two `usize`s, so
**no fact in the structure could separate them.**

⇒ **That is not a missing row — it is the strongest claim the representation
supports**, so filing it as *"add a row"* would be wrong and refutable. File it
as the gap between the **doc's stated requirement** and the **check's
granularity**, which needs no reachability argument
([[a-detector-that-re-derives-its-mechanisms-lookup-is-blind-where-the-two-disagree]]
— bound a fidelity finding to fidelity).

⚠ **A control set that reaches the hard case and tests its visible half is the
most convincing thing you will read all day**, because the row is *present* and
*green* and names the right scenario. Ask of each row which direction it moves
the quantity in, and whether the opposite direction is representable.

## Verify a self-retiring premise's SIBLINGS were carried, one by one

The other half of this pass was my own `1b-i0` finding coming due: one premise
was self-retiring (a `never constructed` warning the compiler announces) and
four went false silently and in place. **All four were re-derived, each quoting
the licence the armed producer deleted.**

⇒ **Check the population you named, element by element, and say you did** —
the ring's care is exactly what makes sampling feel safe here
([[a-confirming-first-instance-is-when-the-sample-size-matters-most]] in the
flattering direction: three cleared premises make the fourth feel cleared).

⚠ **And check the premise family OUTSIDE the merge's declared paths**, because
[[a-pin-built-from-your-finding-inherits-your-enumeration]] — my five became the
repair's population. Three other files carried *"nothing constructs"* licences;
none was about this arm. **A same-phrase, different-subject hit is the common
case, so the sweep is cheap and its result is usually "no residue" — run it
anyway, and report the population you checked rather than the finding you
didn't make.**

## An unqualified clause can be true of one arm and false of its sibling

Same merge: *"It runs before emission of the root answer, which is what makes
the refusal lawful rather than late"* — unqualified, over a close called from
**two** arms of one `match`. True of the second; in the first, the root adapter
is emitted thirty-three lines above the close, deliberately and for a good
reason stated locally.

⇒ **Count the call sites of anything a doc comment describes positionally**
(*before*, *after*, *ahead of*, *at the end of*). A position is a property of a
call site, and one function called from two places has two positions.

⚠ **Rank it correctly: the safety did not rest on the clause** — the same doc's
next sentence carries it. **A wrong reason beside a right one** is a low-severity
finding with a one-qualifier repair, and saying so is what keeps it from reading
as an alarm ([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]).
