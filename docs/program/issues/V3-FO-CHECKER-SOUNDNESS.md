---
id: V3-FO-CHECKER-SOUNDNESS
title: "Author FokDerivation, fok_derives and fok_classically_valid, and prove checker_soundness in Ken over the check_cert that now exists -- the first of the two theorems 23 section 4.4 requires before route FO may return proved"
status: active
owner: language
size: L
gate: none
depends_on: [V3-FO-KEN-LEVEL-CHECKER-AUTHORING]
blocks: []
github: null
origin: "Steward, 2026-08-16, discharging section 4e on the merge of V3-FO-KEN-LEVEL-CHECKER-AUTHORING (D4+D5, PR #2421). That node's own banned scope names this work: 'Proving embedding_adequacy or checker_soundness. Unfiled successors.' Every coordinate below re-verified against origin/main 5aae7a454 before filing. Steward-filed per COORDINATION section 2."
---

## Why this exists: the predecessor removed the last thing that was missing

`23 §4.4` forbids route FO from returning `proved` *"until both theorems are
kernel-checked in an approved home."* Until 2026-08-16 that precondition was
unreachable for a reason that had nothing to do with proving anything:
**the definitions the theorems are about did not exist.**

[[V3-FO-KEN-LEVEL-CHECKER-AUTHORING]] authored them.
`catalog/packages/Tooling/Verification/FoKripke.ken` now carries `FokIForm`,
`FokForm`, `FokQTerm`, `FokSequent`, `FokRule`, `FokCert`, `fok_embed`,
`fok_w_forces` and `fok_check_cert` (651 lines at `5aae7a454`), kernel-checked,
with a differential against the Rust reference checker and a measured
conversion cost.

⇒ **`checker_soundness` is now a theorem with a subject.** This node proves it.

## The statement, taken from the spec rather than restated

`spec/20-verification/23-prover.md:515-517`:

```
checker_soundness :
  (q : Form) -> (pi : Cert) ->
  check_cert q pi = True -> classically_valid q
```

and its two supporting definitions, `23 §4.3` at `:452-467`:

```
Derivation(Gamma => Delta) : Type   -- the indexed proof-tree family generated
                                    -- by exactly the same rules
Derives(s) : Omega := || Derivation(s) ||
classically_valid : Form -> Omega
classically_valid q := Derives([] => [q])
```

**`checker_soundness` ranges over every quoted target formula, not only an
`embed` result** (`:521-522`). Do not narrow it to the `embed` image.

## The home is ruled and it is not this node's to revisit

`docs/design/fo-route-theorem-home.md` §1, Architect ruling 2026-08-15:

> *"`embedding_adequacy` and `checker_soundness` are Ken theorems, proved and
> kernel-checked -- not postulated, and not implemented in Rust. No
> `declare_primitive`. No `declare_postulate`. No new kernel file."*

§3 of that doc records why: **route FO adds zero entries to `trusted_base()`**,
because every artifact it introduces is a definition or an inductive, and those
are re-checked rather than trusted. **That zero is the entire cost answer to the
placement question, and this node is where it is either kept or lost.**
Postulating the theorem is not a fallback — `23 §4.4` forbids the outcome it
would buy, and it would put a load-bearing metatheorem into the trusted base.

## Fixed inputs, measured at `origin/main` `5aae7a454`

| fact | value |
|---|---|
| `FoKripke.ken` | 651 lines, 6 inductives, 27 functions |
| `fok_check_cert` | `:650`, dispatches to `fok_check_tree ([] => [q]) pi` |
| the rule table the checker enforces | `fok_check_rule` `:572`, `fok_check_forall_right` `:550` |
| `FokRule` constructors | `:72` |
| `Omega` reachable from `.ken` surface | **yes** — `conformance/challenge/C2-proof-relevant-omega/sound-perm-count.ken` declares `data ... : Omega` |
| propositional truncation | **kernel-level yes** (`16 §6`, `‖A‖ : Omega`, `Trunc`/`TruncElim` in `17-conversion.md:154`, `18-judgments.md:174`); **surface reachability from `.ken` NOT established** |
| `Omega` in any catalog `.ken` | **zero** |
| truncation in any catalog `.ken` | **zero** |

⇒ **The last two rows are why `D0` exists and why it may hard-stop.**
Everything this node needs is specified and kernel-supported; whether it is
*writable in a `.ken` file today* is an unanswered question. **A YES is answered
by elaborating a file. A NO is answered by the grammar** — see the closeout;
this sentence originally prescribed elaboration for both, and that is wrong in
the direction that matters, because the NO is the answer this node got.

> **`D0` ANSWERED THE TRUNCATION ROW AND IT IS A NO — read the closeout below
> before acting on the row above.** The row stands as written because it records
> what was known at filing; the measured answer is that truncation is
> **kernel-complete and surface-unwritable**, and the gap is a missing token
> rather than a missing capability. [[LANG-TRUNCATION-SURFACE-SYNTAX]] carries
> it.

## Deliverables

**`D0` — the buildability probe. A HARD STOP HERE IS A COMPLETE RESULT.**
Elaborate a minimal file establishing three things independently:

1. an inductive **indexed by** `FokSequent` (`FokDerivation : FokSequent -> Type`
   or whatever the surface admits) declares and kernel-checks;
2. `‖A‖` — propositional truncation — is reachable from `.ken` surface
   syntax, and `Derives(s) : Omega := ‖ FokDerivation s ‖` is writable;
3. a proof term can eliminate a `Equal Bool b True` hypothesis by cases on `b`.

**Report each of the three separately.** A single "it works" or "it does not"
loses the information that decides the cut. If (2) fails, say what the surface
*does* admit — the node then becomes a surface-gap report plus a filing, and
that is a genuine result, not a failure.

**`D1` — author `FokDerivation`, `fok_derives`, `fok_classically_valid`.**
`FokDerivation` is generated by **exactly the rules `fok_check_rule` checks** —
same rule set, same premise counts, same freshness side conditions.

> **`D0` split this deliverable in half. BOTH HALVES ARE NOW BLOCKED, on two
> INDEPENDENT defects. Neither unblocks the other.**
>
> - **`D1a` — `FokDerivation` itself. BLOCKED** on
>   [[LANG-CTOR-PREMISE-ELABORATION-DIVERGES]]. `D0` part (1) proved the
>   *indexed-family* form elaborates, and that remains true — **what diverges is
>   a constructor premise applying a recursive function to a telescope-bound
>   variable**, which every `FokDerivation` constructor needs in order to
>   transcribe `fok_check_rule`'s guards. Measured at `D1a` (`8d6d7d545`).
>   **The frame was subsequently located in `ken-kernel` — the strict-positivity
>   check normalizes every constructor argument to a full normal form before
>   testing it (`inductive.rs:97`). This blocker is kernel-owned and
>   TCB-resident, and unblocking it is an operator ring decision, not this
>   ring's.** Details and epistemic status in the linked node.
> - **`D1b` — `fok_derives` and `fok_classically_valid`. BLOCKED** on
>   [[LANG-TRUNCATION-SURFACE-SYNTAX]]. `fok_derives s := ‖ FokDerivation s ‖`
>   is the only line in this node that needs the missing spelling, and
>   `fok_classically_valid` is defined through it.
>
> ⇒ **Do not read this node as one blocker away from proceeding.** `D0` part (1)
> passing is what made `D1a` look dispatchable; it was, on the evidence
> available, and the divergence was found by attempting it. **`D2` is
> unaffected and remains dispatchable** — ordinary `fn`/`theorem` work, which
> `D1a`'s control cells show elaborates fine.

**`D2` — the Bool-inversion infrastructure.** `fok_check_rule` is built from
`fok_and`/`fok_or` (`:263`/`:256`), so every step of the soundness proof needs
`Equal Bool (fok_and a b) True -> Equal Bool a True` and its partner, plus the
`fok_or` disjunction eliminator. **This is where the proof actually lives**;
front-loading it is what makes `D3` and `D4` one-hour increments rather than one
impossible one.

**`D3` — `fok_checker_soundness` for the PROPOSITIONAL fragment.** `FokInit`,
`FokImpRight`, `FokImpLeft` and whatever else in `FokRule` mentions no
quantifier. A proved, kernel-checked theorem over a named sub-fragment is a
releasable increment and a real result.

**`D4` — extend to the quantifier rules.** `FokForallRight` and its
eigenparameter freshness are the hard case — `fok_check_forall_right` `:550` is
the only non-structural check in the checker, and it is the only place a
`Bound`/`Parameter` distinction is semantic.

**`D5` — state the theorem's reach honestly.** Which rules are covered, which
are not, and whether the composed discharge in `23 §4.4` is now one theorem
short or two. **A partial fragment is a result; a fragment reported as the whole
is a defect.**

## `D0` closeout — merged, and the blocker is one layer lower than it looked

**Candidate `c367ceb13`, PR #2424, Decision `dec_2dvn8t1xxhwdh`.** QA
`evt_z2p3b0zkn1ga`, Architect `evt_38f22rwkq90ry`. One new test file, `+232/-0`,
purely additive. All three parts were **probed** by elaborating `.ken` and
reported separately as `AC-5` required. **Parts (1) and (3) are established by
that probe. Part (2) — the hard stop — is not, and is established by the grammar
instead; see below before citing it.**

| part | result |
|---|---|
| (1) inductive indexed by `FokSequent` | **PASS** — `34 §2`'s indexed-family form takes a user-defined index, not only a prelude type. Zero `trusted_base()` delta, pinned before and after |
| (2) `‖A‖` writable in `.ken` | **FAIL — the hard stop.** `‖…‖` dies in the lexer, `||…||` in the parser at `Pipe`, bare `Trunc (…)` at name resolution |
| (3) `Equal Bool b True` elimination | **PASS in one form only** — the hypothesis must sit in the return type's Pi-chain. The naive pre-bound form is kernel-rejected: `34 §3.3`'s per-branch refinement covers the branch's *result type*, not other context bindings |

### Part (2)'s conclusion is right and its stated method cannot reach it

**The node required each part to be established *"by actual `.ken` elaboration,
not by reading the kernel or elaborator source."* That is correct for parts (1)
and (3) and backwards for part (2)** (Adversary `evt_5m089b44vzr32`, re-run by
the Steward).

Parts (1) and (3) are **positive** existence claims: elaborating the thing
proves it is writable, and reading source could not. **Part (2) is a negative
one**, and no number of failed spellings establishes it — the probe tried two,
and two failures are consistent with a third succeeding. **It is also the hard
stop, the result the entire node turns on.**

What does establish it is the grammar: `lexer.rs` contains no occurrence of
`trunc` in any form, `parser.rs`'s only hits are the English words in two
unrelated doc comments, and all six `Trunc` references in `elab.rs` are
structural traversals with no surface-form mapping.

⇒ **A method adopted because it is stronger than reading source is weaker than
reading source for the one claim that is negative — and the file applied it
uniformly to all three.** The verdict stands; the grounding moved.
[[LANG-TRUNCATION-SURFACE-SYNTAX]] carries the grammar reads, because its whole
premise is that negative.

### The blocker, in the form that routes it

> **`Term::Trunc` exists and is kernel-typed; no surface syntax or elaboration
> rule reaches it.**

**This is a missing spelling, not a missing capability**, and the distinction
decides both the owner and the size. "The language lacks truncation" routes to
Kernel, touches the TCB, and needs a soundness-sensitive addition — which
`AC-1`, `AC-3` and `AC-4` forbid. "An existing former has no spelling" routes to
Language, touches no TCB, and adds no primitive, postulate, axiom, or
`trusted_base()` entry. **Under the second reading the constraint is not even in
tension with this node's own ACs.**

The probe's own evidence is what selects the narrower reading: failing at
lexer, parser and resolution, with the kernel layer never reached, is the
signature of a missing token.

**The impredicative encoding is closed and does not need re-treading.**
`‖A‖ := (P : Omega) -> (A -> P) -> P` does not typecheck, because `Ω` is
predicative (`ken-kernel/src/lib.rs:23`) so that `Pi` does not land back in `Ω`.

### `D3`/`D4` are not affected, and `D2` is dispatchable

`checker_soundness`'s conclusion is `classically_valid q : Omega`, which is
where `D1b` needs truncation. **The proof steps do not.** `D2` is the
`fok_and`/`fok_or` Bool-inversion infrastructure over functions that already
exist in `FoKripke.ken` (`:263`/`:256`) — no truncation, no `FokDerivation`.
`D0` part (3) established the signature shape every one of those lemmas must be
written in, so `D2` is derisked rather than blocked.

⇒ **This node is not stalled. `D2` is dispatchable.**

> **This paragraph originally read *"`D1a` and `D2` are both dispatchable"*, and
> `D1a` was dispatched on it. It hard-stopped**: authoring `FokDerivation`
> diverges during elaboration, in the kernel's strict-positivity check
> ([[LANG-CTOR-PREMISE-ELABORATION-DIVERGES]]).
>
> **The claim was correct on the evidence and the evidence was insufficient.**
> `D0` part (1) established that an *indexed family over a user-defined index*
> elaborates — it did not exercise a **constructor premise**, which is a
> different axis and the one that diverges. **A probe establishes the shapes it
> probed, and `D1a` needed a shape `D0` never wrote.**
>
> ⇒ `D2` survives this correction on stronger ground than `D1a` did: `D0` part
> (3) exercised the exact `fn`/`theorem` signature shape `D2` needs, and `D1a`'s
> control cells independently confirm that path terminates.

## Acceptance criteria

**`AC-1`. Zero new entries in `trusted_base()`.** No `declare_primitive`, no
`declare_postulate`, no new kernel file, no trusted axiom. This is
`fo-route-theorem-home.md` §3's headline result and the reason the home was
ruled as it was. **A candidate that admits the theorem has not delivered a
weaker version of this node — it has destroyed the node's entire value.**

**`AC-2`. `FokDerivation`'s rules and `fok_check_rule`'s checks are the SAME
rules, established by a CONTROL, not by a reading.**

> **This is the load-bearing criterion and the one most likely to be discharged
> by inspection.** If the two drift, `checker_soundness` is a true theorem about
> a calculus the checker does not implement, and the discharge path is unsound
> in exactly the direction that matters — **the proof still kernel-checks.**
>
> **Reading the two side by side is not a control.** The predecessor node
> produced four separate instances of a property that held because of what the
> corpus happened to contain, with nothing making it hold; **two of the four
> were invisible to three readers and fell to a mutation.** Both times the
> reasoned safety argument was wrong rather than merely unproven.
>
> ⇒ **Mutate a rule in one artifact and show the other side reds.** Per rule,
> not once. If a mutation passes, that rule pair is unprotected and saying why
> is part of this AC.

**`AC-3`. No FO `Proved` verdict, on any basis.** `23 §4.4` needs **both**
theorems; `embedding_adequacy` is not in this node and is not started. The
reservation is untouched. **`attempt_fo` is not edited by this node at all.**

**`AC-4`. `fok_check_cert` and its callees are NOT changed to make the proof go
through.** The checker is the merged, differentially-validated artifact. If the
proof genuinely cannot be completed against it, **that is a finding about the
checker and it routes back** — it is not a licence to edit the subject of the
theorem until the theorem holds.

**`AC-5`. A hard stop is a complete result**, at `D0` or anywhere after it.
Report what was established, what blocked, and what the next node would need.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Targeted local validation
only — `-p ken-elaborator`, never `--workspace`.

## Banned scope

- **`embedding_adequacy`, `denote`, `Carriers`, `AtomEnv`.** The other theorem
  and its semantic apparatus. It is the successor to this node, not part of it.

  > **Correction, measured 2026-08-16: a `denote` DOES exist — in Rust**, at
  > `ken-elaborator/src/fo_kripke.rs`, whose doc names `embedding_adequacy` as
  > *"the reserved boundary, not built here."* What does not exist is a
  > **Ken-level** `denote`. This node and its predecessor both said "`denote`
  > does not exist" without that qualifier. **The distinction sizes the
  > successor**: it has a Rust reference to mirror and differentially validate,
  > exactly as `fok_check_cert` mirrored the Rust checker — not a semantics to
  > invent.
- **Emitting `proved` for FO.**
- **Changing the Rust reference checker** (`fo_kripke.rs`). It is the reference
  the predecessor's differential is against.
- **Adding sort validation** — [[CORE-FO-CHECK-TREE-SORT-VALIDATION]]'s, and
  folding it in here breaks the predecessor's differential by design.
- **Widening the slice** beyond `23 §4.5`.

## Four things the predecessor leaves loaded. Read them before `D0`.

**1. `23 §4.4`'s own discharge line prescribes a proof term this kernel
rejects.** The spec writes `sound Sigma C rho f pi (refl True)` at `:531`, and
`fo-route-theorem-home.md:110` repeats it. **`D4` measured that
`Equal Bool True True = Refl` is rejected** — *"Refl expects an Eq-shaped
goal"* — because equality at an inductive type reduces observationally past the
`Eq` shape before `Refl`'s check runs. The working term is **`Proved`**, the
prelude's `Top`-introduction constant (`prelude.rs:899`), on the prelude's own
established idiom.

⇒ **The discharge shape is sound and the spec's spelling of it is wrong.** This
node does not fix the spec — that is the enclave's — but it must not inherit
`refl True` as though it worked. **Recorded here so the next author does not
rediscover it by failing.**

**2. The conversion cost does NOT gate this node, and it will look like it
does.** `D5` measured kernel conversion of `fok_check_cert` at four to five
orders of magnitude above the Rust reference, super-linear, capped at
`imp_chain` depth 8. **That is the cost of the `ok` argument at USE.** Proving
`checker_soundness` is induction over the structure of `FokCert`; it does not
evaluate `fok_check_cert` on anything. **Do not size this node off `D5`'s
numbers, and do not report a slow proof as confirming them.**

**3. The test serializer is protected on all three axes, and ONE of the three is
protected only by accident.** `rule_source` and `qterm_source` have dedicated
pins from `D4`. `sequent_source`'s `gamma`/`delta` order **is measured and does
red** (Adversary `evt_10dxqtbsf4tcw`, re-run at `04af417d2`) — **but the
protection comes from the two pins built for the other axes**, not from anything
aimed at it.

⇒ **If you touch the serializer, a change that keeps both other pins green can
still silently unprotect `gamma`/`delta`.** A dedicated pin is warranted; a
re-measurement is not.

> **The predecessor recorded this axis as UNMEASURED for several hours, in the
> node, in the merge notification, and in the Architect's own resolution, while
> the measurement already existed.** The claim about it was discarded for sharing
> provenance with a refuted sibling. **Shared provenance is grounds for
> re-checking, never for concluding** — and the re-check had already been run.

**4. The SCT checker will bite this proof.** It traces a structural decrease
only through a direct pattern match feeding the recursive call, not through an
intervening non-recursive helper's return value — hit twice already, recorded
at [[LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN]] and at `FoKripke.ken:306-312`. **A
mutual induction over `FokDerivation` is exactly the shape that trips it.** The
working remedy is to inline the guard at each call site. Budget for it rather
than discovering it.

## Sequencing

**`ready` at filing.** `depends_on` is [[V3-FO-KEN-LEVEL-CHECKER-AUTHORING]],
merged 2026-08-16 (PR #2421). Nothing else gates it: the home is ruled
(`fo-route-theorem-home.md` §1), the spec gap is closed
([[V3-KRIPKE-THEORY-CLOSURE]], PR #2323), the statements are fixed at
`23 §4.4`, and gate 1 — `attempt_fo`'s per-call signature minting — was removed
by [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].

⇒ **`23 §4.4` is now the only thing between an accepted certificate and
`proved`, and this node is the first of its two halves.**

**This is lane 2 under the operator's 2026-08-15 two-lane directive** — the FO
Kripke embedding half. It does not contend with lane 1 (runtime,
`RecursiveDescent`) or with verify's [[V3-Z3-EMISSION-CONTROL]].

**Nothing waits on it.** No Ken program in this repository produces an
FO-quotable obligation ([[V3-FO-CONVERSION-LOAD-MEASURED]] `D1`), so this gates
nothing today. **That is a statement about urgency, not about readiness** — the
predecessor's Sequencing records the two being conflated once, and lane 2 sat
idle across the gap.
