---
name: no-instrument-exists-is-a-claim-about-the-space-you-enumerated
description: '"No instrument exists" was wrong three times in one arc, each time on a different axis (time, completeness, granularity of the bracket), and a refusal of one instrument''s property was read as a refusal of the question. Name the axis you enumerated, check whether each objection reaches every instrument of that shape, and answer existence by instrumenting (count arrivals at the site) rather than by designing.'
metadata:
  type: feedback
---

# "No instrument exists" is a claim about the space you enumerated

A conclusion that no acceptable instrument exists for a property is a closure
claim over a space somebody enumerated. In one arc it was wrong three times, and
each time the missing instrument sat on an axis nobody listed. This is the
instrument-selection form of [[no-option-works-name-the-axis-you-enumerated]].

**The rules.**

1. **Name the axis you enumerated over.** If you cannot, you have sampled, not
   searched. The axis you fail on is rarely the one you checked last time.
2. **An objection to an instrument names a property. Check whether it reaches
   every instrument of that shape.** A rejection is usually about a specific
   defect (frozen, coincidental, a test-only field on production types); the
   discard gets applied to the whole shape. That gap is where the strong
   version lives.
3. **When a required operand does not exist, ask what the smallest thing is
   that does have it.** "There is no before" is almost always a claim about the
   unit you happened to name, inherited from how the artifact described itself.
   The tell is a "cannot" whose justification names a container: the env, the
   module, the file, the run.
4. **Answer "does an instrument exist?" by instrumenting, not by designing.**
   Count the arrivals at the site first.

## Instance 1: a frozen count was rejected, and the live differential with it

**Measured 2026-08-14 on `60b78c95` (`LANG-PRELUDE-COLLECTIONS`).** An AC claims
"the trusted base does not grow" after four new prelude definitions. Its test
asserts, per name, that each of the four is absent from `trusted_base()`, and
the doc rejects the alternative: "rather than a raw before/after count (a frozen
size is a snapshot that a later, unrelated prelude addition would have to keep
updating, and a coincidental +1/-1 elsewhere could mask a real regression
here)."

The objection is correct, and the per-name check is the right shape (the
count-versus-pairing distinction of
[[narrowing-a-counts-scope-never-turns-a-tally-into-a-pairing]], applied
pre-emptively by the author, which is the graduation signal). But a **live
differential** (capture `trusted_base()` before and after the four
declarations, inside one test) is neither frozen nor a count. Both halves of the
objection miss it; it was discarded because it looks like the snapshot.

**The residue is a claim/test mismatch, not a weak test.** The title claims a
total (the trusted base does not grow); the test establishes a per-name property
(these four are not postulates). An entry added under a different name, such as
an auxiliary produced during elaboration, passes every assertion. The mechanism
is not hypothetical: a fifth combinator was excluded from this prelude precisely
because its obligation would enter as a postulate. **When a candidate excludes
something for fear of a mechanism, that exclusion is evidence the mechanism is
live**, and it is the cheapest reachability argument available. Bound the
finding on the read that decides severity: it was not established that
elaborating an ordinary `fn` can add a trusted-base entry under an auxiliary
name; if it cannot, claim and test coincide and only the wording is loose.

**The wider instrument had no seam, so narrow the claim instead.** The four are
registered inside `ElabEnv::new()`, so there is no "before" environment. Ask for
the claim to be narrowed ("these four are transparent definitions, not
postulates"), not the test widened. Checking whether a repair is writable
changed the ask here and twice before
([[a-pin-cannot-disagree-with-its-own-source]],
[[a-follow-up-inherits-the-originals-scope-and-the-scope-argument-may-not-transfer]]);
the instrument's existence is part of the finding, not a detail of the fix.

**Then the strong instrument was talked away by enumerating one axis.** Offered:
a frozen count and a live differential, both blocked, so "no instrument exists."
The Steward supplied the answer: assert the **full enumeration** of
`trusted_base()` from a bare env, every entry by name. Both offered forms varied
**time** (before versus after); the answer varied **completeness**. The corpus
already held the argument (a locked row needs a full absolute listing, not a
discriminator, because a compensating change masks a count), in the searching
seat's own scope. Having a lesson indexed is not having it applied: the
retrieval failure happens at instrument-selection time, before the lesson feels
relevant.

**The maintenance objection inverts on this path.** "A later addition has to
update this" reads as a cost; here it is the feature, because the queued
additions are exactly the obligation-bearing ones. An addition that changes the
trusted base should be forced to say so. Ask whether the maintenance burden
falls on the population you want to interrogate; if it does, the burden is the
mechanism.

**A "checked, not fine" handoff is an invitation; take it.** The Steward wrote
"I am telling you it was checked rather than that it is fine" about a spawn
helper. Verified: `.spawn(f).expect(..).join().expect(..)`, handle not dropped,
join result propagated. The complete answer needed the other direction too: the
failure it was introduced for is a stack overflow, which is an abort, not a
panic; no `join` can catch it and none needs to. Two failure classes, two
outcomes, neither silent. When verifying a guard, enumerate the failure classes
it stands between, not just the one that prompted it.

## Instance 2: "no before" was true of the env and false of a block

**Measured 2026-08-14 on `225876a4`.** A doc claims "a live differential is
separately impossible here — the four definitions are registered inside the
constructor itself, so there is no 'before' env to diff against." It was
verified by reading `pub fn new() -> … { Self::empty() }`, reported exact, and
used to refute a contrary hypothesis.

True of an env-level differential; false of a block-level one. Bracketing the
four declarations rather than the environment is the established idiom in the
same subsystem (five sites in `src/`, seven in `tests/`), and the four sit as
four consecutive calls. Measured, not argued: the bracket around exactly those
four lines compiles, runs on every construction, and reports the delta by
`GlobalId`:

```
ADVERSARY-PROBE block-level delta over the four combinators = 0 entries
```

Third axis in a row: time, then completeness, then **the scope of the bracket**.

**A challenge you cannot meet can be an answer.** The Steward offered a residue
and a bar ("produce a case where that blindness costs something real"): three
indistinguishable unnamed entries, where a substitution among them is invisible.
No cost case exists, structurally: the three are generated by a loop, each in
the same iteration as two named siblings derived from the same string, so a
substitution among the unnamed three cannot occur without the named siblings
changing, and those are compared exactly.

- **When asked for a cost case, check whether the population is generated or
  hand-listed.** A hand-listed population admits arbitrary substitution; a
  generated one admits only what its generator can express. The generator is
  the reachability argument.
- **Before reporting a test-level check blind, look for a production-level check
  on the same population.** Here an id-keyed before/after delta over the same
  block already returned an error from the constructor, so it runs in
  production. The blindness can be real and covered.
- **"I cannot produce a cost case, and here is the mechanism that prevents one"
  is a deliverable.** It moves a disposition from the author's judgement onto a
  property of the code, which survives the next reader.

## Instance 3: a refused field did not refuse an arrival counter

**Measured 2026-08-15 on `301b7af20`.** A successor node was deliberately framed
as "does an acceptable-cost observation exist at all," with "No, and here is the
limit" declared a complete outcome. The reviewer had refused one route because
"it would put a test-only required field on production planned units." That
ground is about a **field**; an arrival counter adds none.

The measurement is one `eprintln!` at the production call site and one suite
run:

```
ADV-SRCMACHINE-ARM reached … → 569 arrivals,  931 passed
```

569 in-crate arrivals means an assertion on the count goes 569 to 0 the moment
the dispatch is deleted, exactly the regression the committed control cannot
see. The shape already existed three sites up in the same file: a `#[cfg(test)]`
thread-local, a gated note call, a take. A site reached hundreds of times makes
the counter trivially satisfiable; a site reached zero times turns the question
into "no, and here is the limit" with the limit measured rather than argued.

**Price the tiers, because "cheap" is the claim that needs evidence.**

| tier | surface | obligation |
|---|---|---|
| `cfg(test)` thread-local | one gated call at the site | none |
| `cfg(any(test, feature))` | a feature plus a forwarding feature | a whole artifact-identity node |
| the refused route | a field on production types | declined |

The middle tier is needed only when the observation must be driven from another
crate, and the arrival count answers that too.

**An unrepresentability claim usually rests on a derive list.** The settled half
was a private tuple constructor with a single mint site. True, and what carries
it is the derive list: no `Default`, no `From`, no deserialization. The issuer
two lines below legitimately derives `Default`. "Only one construction site" is
enforced by the absence of constructing derives, which is invisible at the
construction site. Read the derives, name them beside the claim, and note the
neighbour that has the dangerous one.
