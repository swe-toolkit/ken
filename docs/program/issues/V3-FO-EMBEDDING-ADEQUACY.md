---
id: V3-FO-EMBEDDING-ADEQUACY
title: "Author the embedding and prove embedding_adequacy (classically_valid of the translated form implies the source form) — the SECOND of the two theorems 23 section 4.4 requires before route FO may return proved"
status: active
owner: language
size: L
gate: none
depends_on: [V3-FO-KEN-LEVEL-CHECKER-AUTHORING, V3-FO-CHECKER-SOUNDNESS]
blocks: []
github: null
origin: "Steward, 2026-08-22, discharging the framing debt surfaced when the FO D0 fork was routed to the spec enclave. V3-FO-CHECKER-SOUNDNESS is the FIRST of the two 23 section 4.4 theorems (merged); this node is the SECOND. The enclave D0 ruling (spec-leader evt_2enqgkgqwd2g5, from spec-author evt_3kefqcayzajq9) directed that this node be cut AFTER D0 landed, on the structural assumption, so it does not race ahead and silently assume a (b)/(c) kernel premise. Framed to ready 2026-08-22 as the interim lane-2 WP after checker-soundness completed; all coordinates measured at origin/main 6842689b. Steward-filed per COORDINATION section 2. RECUT by the Steward 2026-08-27 at origin/main b76943684, before release, without an operator or Architect ruling because nothing about the objective changed: D1 had LANDED (771eec449, 87f26d0d2, 215b88071, 1308e9ea0, 5ef0f0983; Architect-approved, Decision dec_7f4k3whvy9n8 resolved) while this node still read status ready with D1 listed as work and its artifacts declared ABSENT with zero occurrences. Re-measured every fixed input; every Ken-side line number had moved and the absence claim was false. The releasable remainder is D2+D3 only. Rust-side coordinates (fo_kripke.rs Carriers:500 AtomEnv:508 denote:517; prover.rs attempt_with_cert:316 attempt_fo_with_signature:574 emit_unknown_hole_fo_withheld:800) all re-verified UNCHANGED."
---

> # FRAMED 2026-08-22 — the second route-FO `proved` theorem, on the structural arm
>
> Route FO may return `proved` only once BOTH `23 §4.4` theorems hold. The first,
> `checker_soundness`, is merged ([[V3-FO-CHECKER-SOUNDNESS]], origin/main
> `ba20b4810`). This node is the second, `embedding_adequacy`. **It authors the
> denotation side and proves adequacy; it does NOT wire `attempt_fo` or emit
> `proved` — that verdict-flip is a separate downstream node** (see AC-3 and the
> `attempt_fo` fixed input).

> # D1 BUILD HANDOFF — Architect-ruled 2026-08-23 (authoritative build spec)
>
> D0 landed as a genuine representability hard stop (probe `4d5fd8cee`,
> QA-approved evt_2ty00dbtgy1xv): the plain `FokIForm` atom (bare `Nat`, no scope
> proof) has no total denotation, and the two invention workarounds were correctly
> refused. The spec enclave concurred on the fix's semantic faithfulness
> (spec-author evt_2q0tkxtca67e2 / spec-leader evt_447d5qssxrftr: scope-indexed
> quotation/denotation commutation, zero new trust), and the Architect ruled the
> component design (evt_4s0p5m234544h + D1 handoff evt_pxmzdg6kq6as; `§4.4:531`
> places the representation with the Architect). This section is the authoritative
> D1 build; it supersedes the abstract D-sequence below where they differ.
>
> COMPONENT DESIGN (all NEW/additive; nothing landed is re-cut):
> 1. `IForm Sigma n` — intrinsically-scoped source form. Atom variable = bounded
>    index `i : Fin n` (n = object-binder depth), NOT a bare `Nat`. Five
>    constructors mirroring the landed `FokIForm` (Bottom, Atom, Or, Imp, Forall),
>    scope-indexed. This IS the spec's `IForm Sigma`.
> 2. `denote_at` (internal): interprets `IForm Sigma n` under a length-n carrier
>    env; atom arm = total `Fin` lookup (no Option, no default, no invented
>    Bottom); Forall extends the env (n -> n+1); proof-hypothesis Imp introduces NO
>    object slot (n unchanged). Bottom / truncated Or / Pi into Omega / carrier Pi
>    into Omega = the four green D0 axes. Public `denote C rho f` derives from it.
> 3. `erase_n : IForm Sigma n -> FokIForm` — constructor-homomorphic, maps
>    `i : Fin n` to the same de Bruijn `Nat`, changes neither binder order nor atom
>    identity. Used ONLY on the target-embedding leg.
> 4. `embed Sigma f := fok_embed (erase_n f)` — reuses the landed `fok_embed`
>    verbatim.
> 5. `embedding_adequacy` statement (`§4.4:498-503`): `(Sigma)(C)(rho)(f : IForm
>    Sigma) -> classically_valid (embed Sigma f) -> denote C rho f`.
>
> THE TWO LEGS (spec-author's refinement, load-bearing): the DENOTE leg consumes
> the intrinsic `f` directly (`denote_at`, no erase); the EMBED leg uses erase
> (`embed = fok_embed . erase_n`). Adequacy consumes intrinsic `f`; the landed
> `fok_embed` + checker operate only on its forgetful image. Quotation-then-
> intrinsic-denotation is the identity up to kernel conversion.
>
> REUSED UNTOUCHED (a fold must NOT touch these): `check_cert`,
> `checker_soundness`, `FokForm`, `FokCert`, `fok_embed`, `fok_w_forces`
> (`FoKripke.ken:165-199`). The discharge composition (`§4.4:515-521`) type-checks
> with `checker_soundness` reused verbatim on `embed Sigma f : Form`.
>
> D1 SLICE (Steward's cut, per the Architect's recommended boundary): D1 = the
> apparatus (`IForm Sigma` + `denote_at`/`denote` + `erase_n` + `embed`) + the
> `embedding_adequacy` STATEMENT + the quotation-preservation STATEMENT, all
> zero-trust, with the D0 probe axes carried in as durable controls ON THE
> INTRINSIC FORM (the four passing axes carry over directly; the atom axis is
> re-expressed on `IForm Sigma n` — total-by-construction, the D0-refused
> workarounds stay refused). The D0 probe `4d5fd8cee` is ABSORBED INTO the D1
> branch (rebased/carried), NOT merged standalone, so no plain-form atom assertion
> lands only to be reshaped. The two PROOFS (adequacy: `classically_valid embed ->
> denote`; quotation-preservation: `denote` = the Pi-closed obligation up to
> conversion) follow as D2/D3. `§4.5`'s first slice (one sort A, one unary P, five
> source forms) is exactly this grammar.
>
> D1 ACs (sourced-from-source + the enclave's zero-trust conditions):
> - `denote` TOTAL BY CONSTRUCTION — atom carrier value admitted from the `Fin`
>   index into the length-n env; NO default, NO Bottom-on-None, NO invented carrier.
> - ZERO trusted-base delta (before==after) on every new declaration.
> - Connectives per `§4.4:489` / `16 §1.3` (the four green D0 axes).
> - SCT green by direct-subterm descent (scope index n->n+1 at Forall rides a
>   strictly-smaller subform).
> - Discharge composition type-checks with `checker_soundness` reused verbatim.
> - GATE: this does NOT license an FO `proved` verdict — held behind the
>   kernel-checked `embedding_adequacy` AND quotation-preservation TERMS landing
>   (later deliverables; the verdict-flip stays a separate downstream node, AC-3).
>
> Reviewers: Architect (required soundness reviewer — checker reuse untouched,
> `denote` total-by-construction, zero-trust, discharge composition sound),
> language-QA, Adversary (over-accept hunt in parallel). Capability tier: T1.

## Objective

Author a Ken-level embedding-denotation apparatus (`Carriers`, `AtomEnv`,
`denote`) and prove `embedding_adequacy`: that classical validity of the embedded
form implies the source form's denotation. This is the validity-to-Ken direction
`23 §4.4` needs for the discharge; the unused converse is out of scope (spec
`:520-521`).

## The statement, taken from the spec rather than restated

`spec/20-verification/23-prover.md:508-513`:

```
embedding_adequacy :
  (Sigma : Signature) ->
  (C : Carriers Sigma) ->
  (rho : AtomEnv Sigma C) ->
  (f : IForm Sigma) ->
  classically_valid (embed Sigma f) -> denote C rho f
```

Supporting definitions: `denote : (C : Carriers Sigma) -> AtomEnv Sigma C ->
IForm Sigma -> Omega` (`:496`, "interpret the `IForm` constructors by Ken's
connectives"); `classically_valid q := Derives([] => [q])` (`:465-466`);
`Derives(s) : Omega := || Derivation(s) ||` (`:458`, a truncation);
`Carriers Sigma` / `AtomEnv Sigma C` (`:356`, `:491-493`). The discharge
composition is `sound ... := embedding_adequacy ... (checker_soundness ...)`
(`:526-531`); trust boundary at `:539-544`.

## Why it is the cleaner of the two, and forces no kernel arm

The spec enclave's D0 analysis (spec-author `evt_3kefqcayzajq9`) established that
adequacy is **structural induction on the source `IForm` over the translation
clauses** — no certificate, and no rotation-prone second argument like the one
that made `checker_soundness` the SCT-expressibility crux. It needs no SCT and
forces no kernel `size_rel` change. Under the arm-(a) ruling
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]) each theorem resolves the same way —
structural elaboration — so this node does NOT depend on the SCT rotation fix
and does not gate on a kernel arm.

## Fixed inputs, RE-MEASURED at origin/main `b76943684` (2026-08-27)

> **The 2026-08-22 coordinates were measured at `6842689b` and are SUPERSEDED.**
> Every Ken-side line number had moved, and the absence claim below had been
> falsified by this node's own D1 landing. Re-measured by the Steward before
> release. Do not read the `6842689b` figures out of the `origin:` line.

**Reuse (already merged — do not re-author):**

| artifact | location | role |
|---|---|---|
| `FokIForm` | `FoKripke.ken:28` | the source slice (Bottom/atom/or/imp/forall) |
| `fok_embed` | `FoKripke.ken:300` | the embedding `K(Sigma) => forall w. w \|= f` |
| `fok_w_forces` | `FoKripke.ken:267` | per-world classical forcing translation |
| `FokDerivation` `:825`, `fok_derives`, `fok_classically_valid` `:877` | `FoKripke.ken` | the reflected proof-tree family + `classically_valid` |
| `fok_checker_soundness` | `FoKripke.ken:2268` | the FIRST theorem — the discharge composes with it |
| `TruncBar` (`‖…‖` / `\|\|…\|\|`) | lexer `:112`, parser `:2636` | truncation surface token — usable; `classically_valid` uses it |

**ALSO ALREADY LANDED — this node's own `D1`, merged 2026-08-23. DO NOT
RE-AUTHOR ANY OF IT:**

| artifact | location |
|---|---|
| `FokCarriers` | `FoKripke.ken:44` |
| `fok_sort_a` | `FoKripke.ken:48` |
| `FokAtomEnv` | `FoKripke.ken:59` |
| `fok_denote_at` | `FoKripke.ken:306` |
| `fok_denote` | `FoKripke.ken:332` |
| `fok_embedding_adequacy_statement` | `FoKripke.ken:881` |
| the D1 differential | `crates/ken-elaborator/tests/v3_fo_embedding_adequacy_d1.rs` |

**Still ABSENT, and the only thing this node has left to author:** the
kernel-checked PROOF `fok_embedding_adequacy`. Zero occurrences outside
`fok_embedding_adequacy_statement`. `FoKripke.ken:879-880` says so in its own
words — "The D1 adequacy statement. Its kernel-checked proof is the following
increment; this proposition adds no assumption or trusted declaration."

**The Rust reference to mirror (NOT to invent a semantics):**
`crates/ken-elaborator/src/fo_kripke.rs:517`
`denote(env, sig, f) -> Term`, five arms (`:518-540`): `Bottom -> bottom`;
`Atom(IVar i) -> P (Var i)`; `Or(p,q) -> Term::Trunc(...)`; `Imp(p,q) ->
Term::pi(...)`; `Forall(p) -> Term::pi(sort_a, ...)`. `Carriers`/`AtomEnv` are
Rust structs at `:500`/`:508`. The module doc (`:16-19`, `:514-516`) names
`embedding_adequacy` as "the reserved boundary, not built here" — this node
crosses it. **A differential harness exists** for the executable mirrors
(`tests/v3_fo_ken_level_checker_authoring.rs`: Ken `fok_embed`/`fok_check_cert`
vs the Rust `embed`/`check_cert`). The `denote` differential that D1 owed now
EXISTS, at `tests/v3_fo_embedding_adequacy_d1.rs`; it is the landed `AC-2`
control and it is an input to D2, not work to redo.

**`attempt_fo` gating** (`crates/ken-elaborator/src/prover.rs`): the `proved`
verdict is withheld UNCONDITIONALLY (`attempt_fo_with_signature:574-604`) — even
when quotation, `find_certificate`, and `check_cert` all succeed it returns an
`Unknown` hole (`:584-597`, `emit_unknown_hole_fo_withheld:800`), citing the
reserved `§4.4` home. **This node does NOT touch `prover.rs`.** Flipping the
verdict (replace the withhold at `:597` with the composed discharge term through
`attempt_with_cert:316`) is a separate downstream node that depends on BOTH
theorems having an approved kernel-checked home.

## Deliverables

> **THE RELEASABLE REMAINDER IS `D2` + `D3`. `D0` AND `D1` HAVE BOTH LANDED.**
> Start at `D2`. `D0` landed as the representability hard stop (probe
> `4d5fd8cee`), already folded into the component design above. `D1` landed
> 2026-08-23 across `771eec449`, `87f26d0d2`, `215b88071`, `1308e9ea0`,
> `5ef0f0983` — Architect-approved, Decision `dec_7f4k3whvy9n8` resolved. Their
> text is retained below as the record of what was built and on what terms; it
> is NOT an assignment. Re-authoring any of it is a scope defect, and the
> "confirmed ABSENT" claim it was written against is now false — see the
> re-measured fixed inputs.

**`D0` (LANDED) — the buildability probe. A HARD STOP HERE IS A COMPLETE RESULT.**
Elaborate a minimal file establishing, independently and reported separately:

1. `Carriers Sigma` and `AtomEnv Sigma C` are writable as Ken data (the
   `Signature`-indexed families the spec names), kernel-checked;
2. a Ken `denote : Carriers -> AtomEnv -> FokIForm -> Omega` mirroring the Rust
   five arms elaborates — in particular the `Or` arm's truncation (`‖…‖`)
   and the `Imp`/`Forall` arms' `Pi` into `Omega`;
3. the recursion in (2) over `FokIForm` passes SCT with the guard **inlined at
   each recursive call site**, not factored through a non-recursive helper.

**Report each separately.** If an arm is not structurally expressible (e.g. a
`denote` clause whose termination is non-structural), that is the binding return
condition below — a complete result, returned to the enclave + Architect, never a
silent kernel measure change.

**`D1` (LANDED) — author `Carriers`, `AtomEnv`, `denote`, and the `denote`
differential.** Mirror the Rust `fo_kripke.rs:517` five arms exactly. Extend the
differential harness with a `denote` differential row (render Rust `denote`
results to Ken source, convert, compare) — this is the `AC-2` control for the
interpretation. Delivered as `FokCarriers`/`FokAtomEnv`/`fok_denote_at`/
`fok_denote` plus `fok_embedding_adequacy_statement`, with the differential at
`tests/v3_fo_embedding_adequacy_d1.rs`. Consume these; do not rebuild them.

**`D2` (THE WORK) — prove `embedding_adequacy` by structural induction on
`FokIForm`.** Prove the ALREADY-LANDED proposition `fok_embedding_adequacy_statement`
(`FoKripke.ken:881`) as written — do NOT restate it. It reads

```
fok_classically_valid (fok_scoped_embed sigma f) -> fok_denote sigma c rho f
```

over `(sigma : FokSignature) (c : FokCarriers sigma) (rho : FokAtomEnv sigma c)
(f : FokScopedIForm sigma Zero)`. If the proof needs the proposition restated,
that is a hard stop to the Architect, not a licence to edit the landed
statement: D1's approval bound that exact form and `AC-1` still forbids adding
an assumption or trusted declaration to it. The `Imp` and `Forall` arms are the
non-trivial cases (as `ForallRight` was for checker-soundness), and the proof
composes with `fok_checker_soundness` (`:2268`).

> **D2 partition — Architect interface ruling requested at the D2 candidate.**
> If D2 splits into a propositional fragment (Bottom/Atom/Or/Imp) plus the
> quantifier arm (Forall), it MUST follow the checker-soundness D3/D4 shape the
> Architect ruled: a **structural full-tree `Bool` predicate over `FokIForm`**
> carried as a hypothesis, removed by the quantifier increment — NOT a parallel
> restricted `IForm` type. One `denote`, one adequacy relation, split by an
> ordinary hypothesis. Reviewed against that interface, not a restricted-type
> variant.

**`D3` — state the theorem's reach honestly.** Which `IForm` arms are covered,
whether route FO is now exactly one (downstream) wiring step from `proved`, and
that `attempt_fo` remains untouched. A partial fragment is a result; a fragment
reported as the whole is a defect.

## Acceptance criteria

**`AC-1`. Zero new entries in `trusted_base()`.** No `declare_primitive`, no
`declare_postulate`, no new kernel file, no trusted axiom. This is the entire
value of proving rather than postulating (`fo-route-theorem-home.md §3`); a
candidate that admits the theorem has destroyed the node's value.

**`AC-2`. The Ken `denote` and the Rust `denote` reference are the SAME
interpretation, established by a CONTROL, not a reading.** Mutate a `denote` arm
in one and show the differential reds. Per arm. If a mutation passes, that arm is
unprotected and saying why is part of this AC. (The checker-soundness campaign
twice had a safety argument that was wrong rather than merely unproven; a reading
is not a control.)

**`AC-3`. No FO `Proved` verdict, on any basis, and `prover.rs`/`attempt_fo` is
NOT edited by this node.** `23 §4.4` needs both theorems in an approved home AND
a downstream verdict-flip node; none of that is here. The reservation is
untouched.

**`AC-4`. The Rust reference (`fo_kripke.rs`) is NOT changed to make the proof go
through.** If adequacy genuinely cannot be proved against the reference `denote`,
that is a finding about the reference and it routes back — not a licence to edit
the subject of the theorem.

**`AC-5`. A hard stop is a complete result**, at `D0` or after. Report what was
established, what blocked, and what the next node would need.

**`AC-6`. No-regression, in CI (`COORDINATION §12`).** Targeted local validation
only — `-p ken-elaborator`, never `--workspace`.

**Required reviewers.** language-QA + Architect (soundness oracle — the Ken
`denote` faithfully mirrors the reference, the induction is structural, zero
`trusted_base` delta) + the Adversary (invention-in-costume: a `denote` arm that
silently diverges from the Rust reference, a fabricated `Carriers`/`AtomEnv`
shape, a restricted-`IForm` partition smuggled past the D2 interface).

## Banned scope

- **Emitting `proved` for FO / editing `attempt_fo` or `prover.rs`.** The
  verdict-flip is the downstream node, gated on both theorems' approved home.
- **Changing the Rust reference** (`fo_kripke.rs`) — it is the differential's
  reference, exactly as it was for the checker.
- **The converse direction** (`denote -> classically_valid`) — spec `:520-521`
  says only validity-to-Ken is needed; the converse is not in this contract.
- **Widening the `IForm` slice** beyond `23 §4.5`.
- **A kernel `size_rel` / SCT change.** If termination is non-structural, return
  to the enclave + Architect (the binding condition below), never a silent kernel
  measure change.

## Lessons the sibling node paid for — read before `D0`

- **SCT traces a decrease only through a DIRECT match feeding the recursive
  call**, not through a non-recursive helper's return (`FoKripke.ken:305-313`).
  A `denote` recursion over `FokIForm` is exactly that shape — inline the guard
  at each call site. Budget for it.
- **Discharge `Equal Bool _ True/False` with `Proved`, not `refl True`.** The
  spec sketch writes `refl True` (`:531`) but the kernel-checked idiom in this
  file is `Proved` (e.g. `:698`, `:727`); absurd hypotheses use `absurd h`.
- **Derive the operand fact you need from the CALL PATH, not the shape of the
  connective.** The checker-soundness campaign twice cut the wrong-polarity
  Bool-inversion lemma from the connective's shape; the correct one came from
  where the checker consumes it.

## Return condition (binding, from the enclave)

If authoring the embedding-denotation reaches a named sub-structure whose
termination is genuinely non-structural — not a subterm descent of any derived
inductive, which would contradict `23 §4` fixing the reflective apparatus as
structural — that returns to the spec enclave + Architect as a new fork, never a
silent fall into a kernel measure change.

## Sequencing

**Interim lane-2 WP after `checker_soundness`.** Released `ready` 2026-08-22 as
the low-contention FO completion while the operator's next language priority
([[LANG-MODULE-IMPORT-SYSTEM]]) is still being framed with the enclave. Per the
operator's finish-then-switch directive, the language ring finishes its
then-current WP and switches to module/import ahead of surface-syntax the moment
the enclave hands back the spec-surface.

**Contention: LOW.** Touches `catalog/packages/Tooling/Verification/FoKripke.ken`
plus `crates/ken-elaborator/tests/` — proof/catalog work, not the contended
lexer/parser/elaborator-core the surface-syntax backlog would move, and not the
lane-1 runtime crates. Coordinate a merge window only if a runtime WP is in
flight against `ken-elaborator` tests.

**Capability tier: T1.** A soundness-bearing theorem proved against a reference
— structural induction over the denotation, reviewed on the argument, not a
mechanical diff.

**Nothing consumes FO `proved` today** ([[V3-FO-CONVERSION-LOAD-MEASURED]] `D1`:
no Ken program in this repo produces an FO-quotable obligation), so this gates
nothing urgently. It completes the FO route's theorem obligations; the
verdict-flip remains a distinct downstream node.
