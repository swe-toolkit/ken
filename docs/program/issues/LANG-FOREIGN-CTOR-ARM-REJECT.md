---
id: LANG-FOREIGN-CTOR-ARM-REJECT
title: "a match arm naming a constructor of a DIFFERENT inductive family reaches match compilation instead of being rejected as a constructor/type mismatch, so a Nat match with a List.Nil arm is diagnosed by whatever the coverage machinery happens to conclude about it rather than by the mismatch that is actually present"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-REACHABILITY-SUBSUMING-ARMS]
blocks: []
github: null
origin: "Architect-ruled finding raised to the Steward by language-leader at evt_736qmrew9ymdp as outside the LANG-REACHABILITY-SUBSUMING-ARMS recut and needing its own node. Observed panic/empty-winner evidence evt_69dkk9q8hn3ye; Architect disposition evt_1abn5x4rnxzyb. Steward-filed per COORDINATION §2, and filed rather than folded because the leader is right that it is a different defect: the recut repairs what the coverage machinery REPORTS, and this is about a program that should never reach that machinery."
---

> ## THE `depends_on` EDGE IS BOTH PREMISE AND CONTENTION HERE
>
> **Premise:** the interim `NoInhabitants` diagnostic that currently keeps the
> foreign-constructor case from being worse **is not on `main`** — it arrives
> with [[LANG-REACHABILITY-SUBSUMING-ARMS]]. Measuring this node's "before"
> against `main` therefore measures a different program from the one the fix
> lands on.
>
> **Contention:** both nodes edit `crates/ken-elaborator/src/elab.rs` and
> Language runs one node at a time.
>
> ⇒ **`D0` re-measures at the delivered base and does not inherit the numbers
> below.** This is the same discipline that saved the `D2k` route selection on
> the Runtime lane: a probe specified against the wrong base answers correctly
> and selects the wrong branch.

## What this is

**An otherwise-complete `Nat` match with a `List.Nil` arm reaches reachability
construction rather than being rejected as a constructor/type mismatch.**

The arm names a real constructor. It is simply a constructor of **another
family**. Nothing between the parser and match compilation asks whether the
constructor the arm names belongs to the scrutinee's type, so the program
arrives at machinery whose entire job presumes it does.

## Why this is its own node and not part of the recut

**The recut repairs what the coverage machinery reports. This is about a
program that should never reach that machinery.** The distinction matters
because the two remedies do not overlap: a better reachability diagnostic still
describes a coverage fact about a match that is not well-typed, and rejecting
the arm removes the question instead of answering it better.

**The interim diagnostic is TRUE, and that is exactly why it is dangerous to
leave.** `NoInhabitants` is not wrong about the program it is handed — a
`List.Nil` arm genuinely contributes no `Nat` inhabitants. It is a correct
answer to a question nobody asked, in place of the answer to the one that
matters. ⇒ **A user who wrote `List.Nil` for `Nat.Zero` is told something true
about inhabitants and nothing about the typo.**

## Fixed inputs, measured at `main` `ffe0e91d`

Re-derive at your base; see the block at the top of this file for why that is
not optional here.

Every site is `crates/ken-elaborator/src/`.

| site | what is there |
|---|---|
| `error.rs:88` | the existing doc phrasing *"The authority is not a constructor of the family's authority type"* — **the mismatch this node needs is already expressible somewhere in this file's vocabulary.** Establish whether that variant is reachable from a match arm, or names a different authority |
| `error.rs:208` | `ExhaustivenessError { missing, span }`, rendered at `:540` |
| `error.rs:214` | `ReachabilityError { span }`, rendered at `:547` |

**`NoInhabitants` does not occur in `crates/ken-elaborator/src/*.rs` at this
SHA.** That is not a contradiction of the leader's report — it is the interim
diagnostic riding in on the predecessor. **Do not conclude the report is stale;
conclude your base is not `main`.**

## Deliverables

**`D0` — measure the current behaviour at YOUR base, and report it raw.** For a
`Nat` match with a `List.Nil` arm: what is emitted, from which site, and does
anything panic or produce an empty winner. The leader's evidence
(`evt_69dkk9q8hn3ye`) records a panic/empty-winner observation; **say whether
you reproduce it, and if you do not, say that plainly** — the predecessor may
have already converted it.

**`D1` — reject the foreign constructor before match compilation.** The
rejection must name **both** the constructor that was written and the type that
was expected. A message naming only one of the two is the diagnostic this node
exists to replace.

**`D2` — establish whether an existing error variant carries this**, starting
from `error.rs:88`. **Prefer reusing it over adding one.** If a new variant is
genuinely needed, say why the existing one does not fit, in one sentence.

**`D3` — a control that reds if the arm is accepted again**, asserting on the
rendered message rather than only on "an error occurred". The failure this node
repairs is *the wrong error*, so a control that accepts any error cannot see it.

## Acceptance criteria

**`AC-1` — the foreign-constructor arm is rejected before match compilation**,
with a message naming the written constructor and the expected type.

**`AC-2` — no coverage or reachability diagnostic is emitted for that program.**
The point is that the question is removed, not answered better. **A delivery
that produces a nicer `NoInhabitants` has not addressed the node.**

**`AC-3` — well-typed matches are unchanged.** Every currently-accepted program
elaborates identically and every currently-emitted coverage diagnostic is
byte-identical. **This node changes which programs reach the machinery, never
what the machinery concludes.**

**`AC-4` — the predecessor's controls stay green on the same derivation.**

**`AC-5` — no-regression, in CI.** `COORDINATION §12`; build and test targeted,
`-p ken-elaborator`.

## Stop conditions -- return to the Steward, do not decide

1. **The check cannot be placed before match compilation** without reaching a
   surface the predecessor owns. Report the site and what is missing.
2. **`D0` finds the case already rejected** at your base. That is a good
   outcome — report it and this node closes as resolved-without-landing rather
   than being repaired into existence.
3. **A new error variant looks unavoidable and it is not obviously local.**
   Adding vocabulary to `ElabError` is a design call; route it rather than
   picking a shape.

## Sizing

**`S`.** One check, one message, one control. **`D2` is the part that can
grow** — if the existing variant turns out to be load-bearing elsewhere and
reusing it would change another diagnostic, stop and report; that is a re-cut,
not an overrun.

## Not this node

- **Not [[LANG-REACHABILITY-SUBSUMING-ARMS]].** That node owns the reachability
  payload and its emission sites. **Sequence this after it, never
  concurrently** — same file.
- **Not [[LANG-WITNESS-DIAGNOSTIC-STRICTNESS]]**, which is the exhaustiveness
  witness path. Three Language nodes now share `elab.rs`; they run in a line.
- **Not a general type-mismatch audit of pattern position.** One shape: an arm
  naming a constructor of a different inductive family.
