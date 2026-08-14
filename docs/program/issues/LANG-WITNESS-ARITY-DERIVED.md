---
id: LANG-WITNESS-ARITY-DERIVED
title: "`missing_pattern_witness` takes the constructor's arity as a caller-supplied parameter beside the id it names, so the two can disagree in principle -- and three of the four emitters have no test that inspects the witness, meaning a future divergence reds nothing; the arity is derivable from the id alone through an existing kernel API, which retires the class instead of testing it"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-MATCH-DIAGNOSTIC-PROSE]
blocks: []
github: null
origin: "Adversary hunt evt_4zx9xp7qkf6rm on fc9408ec, triaged by the Steward at main 96c95586. The hunt independently confirmed LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD's AC-1 by reverting the property, then found that only one of four emitters has arity-positive evidence. The Steward's re-check NARROWED the finding before filing -- see the disposition section, which is the reason this node's deliverable is a derivation change rather than the four fixtures the hunt ranked first."
---

## What this is

`crates/ken-elaborator/src/elab.rs:3222`:

```rust
fn missing_pattern_witness(cx: &ElabCtx, id: GlobalId, arity: usize) -> MissingPatternWitness {
    MissingPatternWitness { constructor: ctor_name(cx, id), arity }
}
```

**The name is looked up from `id`; the arity is whatever the caller passed.**
Nothing in the signature or the body ties them to the same constructor. The
witness renders `<name>` followed by `arity` wildcards, so a disagreement
prints a pattern of the wrong shape for the constructor it names -- and
`34 §4.1` requires the witness to *be* the unmatched pattern.

## THE HUNT'S FINDING, NARROWED -- READ THIS BEFORE THE DELIVERABLES

**The Adversary reported that at three sites "nothing distinguishes the
constructor's arity from an index count, a telescope length, or a sibling arm's
binder count." Measured at `96c95586`, that is too strong, and the correction
is what changes this node's shape.**

All four call sites pass the `.args.len()` of the same constructor whose `id`
they pass in the same call:

| site | id | arity expression |
|---|---|---|
| `elab.rs:1586` | `host_ctor.id` | `host_ctor.args.len()` |
| `elab.rs:2135` | `ctor.id` | `n` |
| `elab.rs:2314` | `ctor.id` | `n` |
| `elab.rs:8321` | `c0.id` | `c0.args.len()` |

**`:2135` and `:2314` are the same `n`** -- both sit in `check_match_dependent`
(from `:1959`), and `n` is bound once at `:2097` as `let n = ctor.args.len();`.

⇒ **The pairing is correct by construction at all four sites today, and the
hunt's "a wrong-but-derived arity renders the wrong number of wildcards" is not
a live defect.** For the arity to be wrong while the name is right, the named
constructor's own declared argument count would have to be wrong.

**What survives the correction, and it is worth a node:** the pairing is
correct **by convention at each call site**, not by construction of the API,
and **three of the four sites have no test that inspects the witness at all.**
The hunt measured that directly: `:2314` reached 11 times including
`ConsVector` at arity 3, `:8321` reached once with a zero-arity `Zero`, and
`:1586` / `:2135` never reached by any witness-asserting test. So a later
refactor that threads a different quantity into one of those calls -- an index
count, a telescope length -- **changes a user-visible diagnostic and reds
nothing.**

**That is a missing guard, not a present defect. State it that way in the
handback; do not report this node as fixing wrong output.**

## Why the remedy is a derivation, not fixtures

The hunt ranked "one arity-positive fixture per remaining reachable site"
first, noting `:8321` costs only a constructor swap. **That tests the
convention at the sites that exist.** It does not stop the next call site from
being added with a mismatched pair, and it pays four test-fixture costs to
guard an invariant that can be made unrepresentable once.

**`crates/ken-kernel/src/env.rs:495` already exposes what is needed:**

```rust
pub fn constructor(&self, id: GlobalId) -> Option<(&InductiveDecl, usize)>
```

It returns the owning `InductiveDecl` and the constructor's ordinal, from
which the declared argument count is reachable. ⇒ **`missing_pattern_witness`
can derive the arity from `id` and drop the parameter**, after which no caller
can supply a disagreeing one because there is no parameter to supply.

**This is `docs/PRINCIPLES.md`'s subsume-don't-proliferate applied to a guard:
retire the class rather than install four detectors for it.** The same move
the Architect noted on the parent merge, where an inline globals scan was
folded into the shared `ctor_name`.

**`D1` must confirm the lookup is available and total at the point of use
before committing to this shape.** It is stated here as the leading candidate
grounded in a real signature, not as a fixed input -- the call sites run during
elaboration and the id may not always be resolvable in the env at that moment.

## Deliverables

**`D0` -- re-derive the four sites and the two shared bindings at your base.**
Report the call sites, each arity expression, and confirm or refute that
`:2135` and `:2314` share `n` from `:2097`. **An item this frame got wrong is a
finding.** In particular, if any site's arity is *not* the named constructor's
own `.args.len()`, that is a live defect and it outranks everything below --
stop and report it.

**`D1` -- derive the arity from `id` inside `missing_pattern_witness` and
delete the parameter.** All four call sites lose their arity argument. **If
`env.constructor(id)` is not reachable or not total at any of the four sites,
stop and report which site and why** -- the fallback is `D2`'s controls alone,
and that is a legitimate outcome of this node, not a failure of it.

**`D2` -- a control that reds if a witness's arity stops matching its
constructor.** Under `D1` this is a single test that the derived arity equals
the declaration's for a constructor of arity >= 1, since the mismatch is
otherwise unconstructible. **If `D1` was refuted, this becomes the per-site
fixtures instead** -- and then `AC-3`'s per-site requirement is the one that
binds.

**`D3` -- close the census bound the hunt explicitly left unrun.** It stopped a
whole-crate `-p ken-elaborator` build to stay inside `COORDINATION §12` and
recorded that `:1586` / `:2135` are unreached *by witness-asserting tests*,
which does not distinguish "never exercised anywhere". **Settle it with
targeted runs -- named test binaries, not a crate-wide or workspace build.**
If it cannot be settled targeted, say so and leave it open; the finding does
not depend on it.

## Acceptance criteria

**`AC-1` -- after `D1`, a mismatched name/arity pair is unrepresentable at the
call sites, and the node says so by pointing at the signature.** A parameter
that still exists and is merely always passed correctly does not discharge
this.

**`AC-2` -- the rendered diagnostic is unchanged for every currently-exercised
case.** This node changes where the arity comes from, not what it is. The
existing arity-positive control and the zero-arity `l2_acceptance` rows must
pass unmodified. **If any expected string changes, that is a defect this node
found -- report it, do not update the expectation.**

**`AC-3` -- the guard is stated per site.** Either the derivation covers all
four (name the four) or a control covers each remaining one (name each). **"The
shared helper is tested" is not sufficient if the parameter survived** -- that
is exactly the shape that left three sites unwitnessed.

**`AC-4` -- no behaviour change beyond the arity's provenance.** No emission
site moves, no checker logic, no payload field added or removed beyond the
deleted parameter. If a fix appears to need one, stop and report.

**`AC-5` -- the `D3` census is reported with its method**, including "not
settled targeted" if that is the answer. **No `--workspace` run, in any
deliverable, for any reason** (`COORDINATION §12`).

**`AC-6` -- no-regression, in CI.** The venue is CI, never a local
`--workspace` run. Build targeted, `-p ken-elaborator`.

## Sizing

**`S`, and it should be well under the hour if `D1` holds.** One helper, four
call sites, one control. **`D1`'s reachability check is the only thing that can
surprise you**; if the env lookup is not total at an emission site, hand that
back rather than threading a new parameter to work around it -- reintroducing a
caller-supplied arity is the defect this node exists to remove.

## Adversary disposition, recorded here rather than replied

**Hunt `evt_4zx9xp7qkf6rm`. Confirmed defect, narrowed, filed as this node.**

- **Its `AC-1` re-verification is accepted in full** and is the stronger half
  of the report: it reverted `Display` to a name-only render, got exactly the
  old `missing constructor 'ConsVector'`, and observed that **only one test in
  the crate reddened** -- a universal probe establishing that one test sees the
  arity property.
- **Its per-site reachability table is accepted** and is the evidence `D3`
  extends.
- **Its causal claim is narrowed**, per the measurement above: the four sites
  pair id and arity from the same constructor, so there is no live wrong-arity
  output. The real gap is the absent guard. **This is recorded so the "three
  emitters emit wrong arities" framing is not re-surfaced from the hunt text by
  a later reader.**
- **Its ranked remedy is not taken.** Four fixtures test a convention; deleting
  the parameter retires it. If `D1` refutes the derivation, the fixtures are
  the fallback and `AC-3` binds.
- **The bound it declined to run was declined correctly.** Killing a
  whole-crate build to stay inside `COORDINATION §12` is the rule working, and
  the finding stands without it.

## Not this node

- **Not a general `ElabError` diagnostic-quality pass.** One helper and its
  callers.
- **Not the prose and citations** -- that is [[LANG-MATCH-DIAGNOSTIC-PROSE]],
  in flight on the same file. This node does not reword the message, the
  `Display`, or any doc comment beyond what the deleted parameter requires.
- **Not the ordinal pairing at `:1586`.** That site pairs
  `support_decl.constructors[ordinal]` with `host.constructors[ordinal]`; if
  those families could diverge in order or count, the *constructor itself*
  would be wrong and its name and arity would be wrong together. That is a
  different defect with a different witness. **If you see evidence of it, report
  it -- do not absorb it.**
- **Not an amendment to `34 §4.1`.**
