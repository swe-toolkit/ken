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
*writable in a `.ken` file today* is an unanswered question, and it is answered
by elaborating a file, not by reading the kernel.

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
  and its semantic apparatus. It needs `denote`, which does not exist; it is the
  successor to this node, not part of it.
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

**3. `sequent_source`'s `gamma`/`delta` field order is UNMEASURED.** The
Architect's `D3` claim that eleven empty-`gamma` cases protect it was made by
the same read-the-values method that the `Init` claim was refuted by, and the
Architect withdrew it as unmeasured (`evt_2v5t7ekqzhqee`). Two pins landed
(`rule_source`, `qterm_source`); **no pin covers `sequent_source`.** Any work
here that reuses the test serializer inherits an open axis.

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
