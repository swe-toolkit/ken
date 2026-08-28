---
id: V3-FO-SORTED-EIGENPARAMETER-DERIVATION
title: "Make ForallRight parameter-only and sorted on BOTH checker surfaces, replace arbitrary-term subst0 with typed instantiation, and restate FokDerivation and its reflection proofs in lockstep -- the atomic middle of the FO soundness repair"
status: active
owner: language
size: L
tier: T1
gate: none
depends_on: [CORE-FO-CHECK-TREE-SORT-VALIDATION]
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-08-27, cutting items 3, 4 and 5 of the repair envelope in Architect ruling evt_6hx31xvw9tqs2, which REJECTED the current FO checker/derivation/adequacy interface as a semantic soundness gate and ruled it not repairable by finishing the current proof. This node SUPERSEDES the relation that V3-FO-CHECKER-SOUNDNESS proved (that node stays merged -- its deliverables did land -- and carries a banner pointing here) and SUBSUMES the control obligation of V3-FO-SUBST-DEPTH-CONTROL, whose shallow oracle the ruling names as insufficient. It is ONE atomic increment because item 5 of the envelope requires the Rust checker/search, the Ken checker, the FokDerivation constructors and the reflection proofs to change TOGETHER. Steward-filed per COORDINATION section 2."
---

## Objective

Replace the permissive eigenterm rule on **every** surface at once: the Rust
checker and search, the Ken checker, the `FokDerivation` constructors, and the
reflection proofs that connect them.

**`ForallRight` accepts only a fresh parameter carrying the quantified binder's
exact sort**, and instantiation becomes **typed** rather than an arbitrary-term
`subst0`.

## THIS IS ONE ATOMIC INCREMENT. IT CANNOT BE SPLIT.

Repair envelope item 5, verbatim in force: **Rust checker/search + Ken checker +
`FokDerivation` constructors + reflection proofs change TOGETHER.**

**Why a partial state here is dishonest rather than merely incomplete.** The
reflection theorem's whole content is that checker acceptance implies the
derivation relation holds:

```
fok_prop_check_tree_sound_type :
  (expected : FokSequent) →
  Equal Bool (fok_check_tree expected pi) True → FokDerivation expected
```

(`FoKripke.ken:1553-1555`). Land **the corrected parameter-only `ForallRight`
checker rule from this node** without restating `FokDerivation`, and the theorem
still elaborates while asserting strictly less than it appears to. Restate
`FokDerivation` without that rule, and the reflection proof fails to elaborate.
**Either half alone leaves the tracker claiming a gate that is not there.**

> **This argument is about THIS node's transition, and does not condemn the
> predecessor** (Architect `evt_ayw7409xg5ty`). **External domain validation and
> a changed calculus rule are different transitions.**
> [[CORE-FO-CHECK-TREE-SORT-VALIDATION]] adds a fail-closed domain predicate over
> the existing representation: its obligation is **monotonic** — acceptance by
> the validated checker implies acceptance by the old structural checker — so it
> changes neither the calculus rule nor `FokDerivation`'s relation, and asserts
> no new semantic theorem. **This node changes the inference rule itself**, which
> is why it alone cannot be split. There is no intermediate state in which
both surfaces are honest, which is the Architect's own criterion for subsuming
into a single frame.

**The corrected relation must STATE the parameter-only sorted rule, not
re-encode the old checker result.** A `FokDerivation` constructor whose premise
is "the old checker accepted" is not a repair; it is the defect relocated.

**Reflection of checker acceptance is not evidence that the reflected rule is
semantically lawful.** This node delivers a corrected *relation*, not a
soundness result. The semantic claim is re-established downstream in
[[V3-FO-EMBEDDING-ADEQUACY]], and this frame must not be reviewed as if it
carried it.

## The defect, located

**Architect's statement of cause (`evt_6hx31xvw9tqs2`).** Both Rust and Ken give
`ForallRight` an arbitrary `QTerm`/`FokQTerm`; the guard checks only occurrence
in the conclusion, so a fresh `Bound(k)` passes as if it were a fresh parameter;
the shared untyped de Bruijn substitution then installs it across world **and**
object binders. **Freshness is not eigenparameter provenance, and a comment
saying the replacement is always a parameter is not an invariant at a public
checker boundary.**

**The Rust arm cannot distinguish the sorts even in principle**, measured at
`fo_kripke.rs:891-897`:

```rust
Rule::ForallRight { right, eigen } => {
    let Some(quantified) = node.conclusion.delta.get(*right) else { return false; };
    let body = match quantified {
        Form::ForallWorld(b) | Form::ForallObj(b) => b,
        _ => return false,
    };
```

**One match arm serves both binder kinds.** The rule discards exactly the
distinction it would need in order to demand a world parameter under
`ForallWorld` and an object parameter under `ForallObj`. Splitting this arm is
the smallest honest statement of item 3.

**`subst0_form` is shared by the checker and the producer.** Its two callers are
`check_tree`'s `ForallRight` (validates) and `search`'s `ForallRight`
(constructs) — `fo_kripke.rs:944` via `subst_form_at:948`, called from `:891`
and `:1083`. **A depth or sort error is applied identically on both sides, so
the certificate the search builds is exactly the one the checker expects and
they cannot disagree.** Any control shaped *"discovery succeeds,
`find_certificate` returns `Some`, `check_cert` accepts"* is **structurally
incapable** of detecting this, at any depth. This was measured and filed on
[[V3-FO-SUBST-DEPTH-CONTROL]]; that node's oracle is the right shape and the
ruling names its depth as insufficient.

## Fixed inputs, measured at `f43ff910357075f976a0d4838353af36480495cc`

The ruling's target paths have **no delta** from `ef91b8225` through
`d8b640d77`, so these coordinates carry the ruling's base. **Re-measure before
starting** — line numbers move.

| coordinate | location |
|---|---|
| `Rule` enum | `crates/ken-elaborator/src/fo_kripke.rs:474` |
| `check_cert` | `fo_kripke.rs:861` |
| `check_tree` | `fo_kripke.rs:866` |
| `check_tree`'s `ForallRight` arm | `fo_kripke.rs:891` |
| `subst0_form` / `subst_form_at` | `fo_kripke.rs:944` / `:948` |
| `search`'s `ForallRight` | `fo_kripke.rs:1032`, eigen at `:1083` |
| `fok_subst0_form` | `catalog/packages/Tooling/Verification/FoKripke.ken:707` |
| `fok_check_tree` | `FoKripke.ken:798` |
| `fok_check_cert` | `FoKripke.ken:814` |
| `fok_derives` / `fok_classically_valid` | `FoKripke.ken:874` / `:876` |
| `data FokDerivation` | `FoKripke.ken:825` |
| `FokDerivForallWorldRight` / `FokDerivForallObjRight` | `FoKripke.ken:847` / `:858` |
| reflection type / proof | `FoKripke.ken:1553` / `:2262` |
| `fok_checker_soundness` | `FoKripke.ken:2268` |
| production fail-safe | `crates/ken-elaborator/src/prover.rs:562-604` |

**Evidence SHAs from the ruling** (do not re-derive): search log `fe3baf37...`,
certificate `db1bf51e...`, Ken counterexample `453f87eb...`, 64-MiB run
`6bbb7f82...`.

## Deliverables

**`D0` — the corrected rule, written down before it is built.** State the
parameter-only sorted `ForallRight` for both binder kinds, and the typed
instantiation's contract, as a rule the reader can check against `23 §4.3`.
**Say which existing theorem statements move and which do not.** Front-load
this: it decides whether the reflection proofs are an edit or a rewrite.

**`D1` — parameter-only sorted `ForallRight`, both surfaces.** Split the shared
match arm. A **bound variable must be unrepresentable in the eigen slot, or
explicitly rejected before substitution.** **Numeric identity is not
provenance** — a check that the eigen's index is unused does not establish that
it is a parameter.

**`D2` — typed parameter instantiation.** Replace arbitrary-term `subst0` on
this path with typed parameter instantiation, or an equivalently typed
capture-avoiding substitution. It must be coherent across **mixed world/object
binder contexts**, preserve inner binders, decrement only the **consumed outer
reference**, and reject out-of-scope and wrong-sort references. Apply to the
producer (`search`) and the checker together — they share the function today and
must remain in agreement.

**`D3` — restate `FokDerivation` and re-prove reflection.** The `ForallRight`
constructors carry the corrected parameter-only sorted premise. The reflection
proofs elaborate against the corrected relation. **The relation states the rule;
it does not cite the checker.**

**`D4` — state the reach honestly.** What the corrected relation now says, what
it does **not** say (it is not a semantic soundness result), and that route FO's
production verdict boundary is unchanged. A fragment reported as the whole is a
defect.

## Acceptance criteria

**`AC-1` — RECONCILED 2026-08-28 to the parameter-only representation
(language-leader `evt_k7x6nmxy4ydy` / `evt_6t3hrfekx34dw`, Steward-owned
reconciliation). The literal prior wording is SUPERSEDED and is quoted below so
the change is auditable.**

> Prior wording: *"The reproduced false embedded certificate is REJECTED by BOTH
> checker surfaces. This is the refuting certificate from `evt_2yh515wg0mczy`,
> reaching the checker through `fok_embed`'s image — not a hand-built term in the
> excess."*

**WHY IT COULD NOT BE MET AS WRITTEN, and this is not the ring falling short.**
The prior wording demands a **checker verdict** on the historical exploit. That
exploit's two object eigen steps (`Bound5`/`Bound3`) are **unrepresentable in the
released parameter-only representation**, and the historical certificate hash
mechanism is not in-tree. So the certificate cannot be constructed to be judged,
and the demanded verdict is unreachable — **not because the repair is weak, but
because the repair is STRONGER than the criterion anticipated.** The exploit is
now refused at CONSTRUCTION rather than by a checker verdict. **An AC that
requires a rejection verdict is not satisfied by unrepresentability, and would
have forced reintroducing legacy representation solely to feed a dead exploit
back in.** The language-leader ruled against that and the ruling stands.

**THE HISTORICAL PROVENANCE, recorded exactly — provenance, NOT a
reproducibility claim.** The refutation that motivated this repair
(`evt_2yh515wg0mczy`, base `ef91b8225`) was a certificate for the `fok_embed`
image of the non-valid source form `forall x : A. forall y : A. P x -> P y`. Its
recorded certificate hash was
`db1bf51e9434307d587fbf9cd565af1343cbd877831ff2477f857d5a740779a8`, and the
recorded 14-step tree instantiated the two OBJECT quantifiers with BOUND
references (`ForallR Bound5`, `ForallR Bound3`) into outer WORLD binders — the
"invent an object-sort inhabitant" exploit. Under the released parameter-only
representation the eigen is a parameter index, so that exact tree is
UNREPRESENTABLE. **Nothing may be asserted to follow from recomputing the hash.**

**The criterion is therefore stated as a PREDICATE over the exploit's
nonexistence, at both levels. All three parts are required:**

1. **Constructor-level unrepresentability, pinned on BOTH surfaces
   concretely.** A bound object eigen has no constructor encoding in Rust
   `Rule::ForallRight { eigen: usize }` or Ken `FokForallRight Nat Nat`. On the
   Ken surface this is pinned BEHAVIOURALLY: a `FokQTerm` eigen is a **type
   error, not a checker `False`.** This is a statement about the datatype, not
   about a search outcome.
2. **Embed-image and search nonexistence on BOTH surfaces.** No term in
   `fok_embed`'s image, and no term the corrected search reaches, derives the
   false conclusion — measured on the Rust surface and the Ken surface
   separately, never one standing in for the other. **Demonstrated by RUNNING
   the decision procedure on the exact source form's genuine `fok_embed` image,
   not argued.**
3. **The recorded historical hash is retained as PROVENANCE ONLY**, per the block
   above.

**`AC-1-POWER` — the nonexistence claims must be shown to have power, and this
is the load-bearing half.** Parts 1 and 2 are both NEGATIVE claims, and a
misconfigured search or an over-broad unrepresentability argument satisfies them
vacuously while proving nothing. **A control that cannot fail is not weaker
evidence; it is none.** Required, by measurement rather than by argument:

- The corrected search MUST FIND a planted, genuinely representable witness of
  comparable shape. A search that returns nothing on every input has not shown
  the exploit is absent — it has shown the search is inert.
- The unrepresentability argument MUST be specific to the historical eigen
  steps: exhibit a near-miss that IS representable and IS refused downstream.
  **An argument that would equally prove some lawful certificate unrepresentable
  is refuting itself, not the exploit.**

> **`AC-1-POWER` IS WHY THIS AC WAS NOT SIMPLY TAKEN FROM THE RING'S OWN
> DRAFT.** The candidate `04ad4379c` proposed its own AC-1 reconciliation
> carrying the concrete provenance and both surface pins above — which is why
> they are now here — but it dropped the anti-vacuity half and reverted `AC-6`'s
> pointer. **"Demonstrated by running the decision procedure" proves the
> procedure RAN; it does not prove the procedure CAN FIND ANYTHING.** A search
> misconfigured to return nothing on every input satisfies part 2 perfectly and
> establishes nothing at all. That gap is exactly the class this program has paid
> for repeatedly, so the running requirement is kept AND the planted-witness
> requirement is kept beside it. The named control
> `crates/ken-elaborator/tests/v3_fo_sorted_eigenparameter_refuting_cert.rs` is
> where both belong.

> **This amendment does NOT reopen the "released node" banner elsewhere in this
> file, and the distinction matters.** That banner forbids FOLDING IN A NEW
> CRITERION after release — new obligations belong to a successor. This is not a
> new obligation: it is the reconciliation of an EXISTING criterion that this
> node's own representation change made unsatisfiable. **A frame that asks for
> something its own increment made impossible is a frame defect, and repairing it
> is the Steward's job at any point in the lifecycle.** The obligation is not
> widened — parts 1-3 are what the prior wording was reaching for, stated so they
> can actually be met.

**`AC-2`. Lawful parameter certificates are still accepted AND still reflect.**
Both halves, pinned per quantifier rule. Acceptance without reflection, or
reflection without acceptance, is a broken repair.

**`AC-3`. Independent wrong-sort mutations each preserve refusal**, measured
separately: a **world eigen into an object binder**; an **object eigen into a
world binder**; **malformed atomic argument roles**; **out-of-scope bound
references**. Name each mutation by its **injection point**, not by its effect —
two sites share one English description.

**`AC-4`. Direct substitution oracles over at least TWO NESTED BINDERS in BOTH
mixed orders**, distinguishing the **consumed**, the **inner**, and the
**strictly-higher** bound references. **The existing shallow control is
insufficient and this AC exists to replace it** — a mutant diverges from correct
only at the second nested quantifier below a substitution point, so a corpus
that never nests two deep is blind by construction. **The oracle is direct on
the substitution function; a route through search-then-check cannot discriminate
here at any depth** (see the shared-caller argument above).

**`AC-5`. Compile-preserving mutations redden their controls**, each named by
injection point: removing parameter-only admission; collapsing the sort check;
corrupting mixed-binder depth.

**`AC-6`. Positive controls show refusal is not caused by an unrelated malformed
tree.** For each refusal in `AC-3`, and for the representable near-miss required
by `AC-1-POWER`, a near-identical tree differing only
in the mutated coordinate is accepted. Without this, the refusal ACs are
satisfied by a checker that rejects everything.

**`AC-7`. `fok_checker_soundness` elaborates over the CORRECTED relation**, with
no premise citing checker acceptance as the derivation's content.

**`AC-8`. No `proved` for FO.** `attempt_fo_with_signature` continues to return
`emit_unknown_hole_fo_withheld` — the audited `Unknown`. **This node does not
move the verdict boundary**, and moving it is out of scope even if the corrected
relation looks sufficient.

**`AC-9`. Zero new entries in `trusted_base()`.** No `declare_primitive`, no
`declare_postulate`, no new kernel file, no trusted axiom.

**`AC-10`.** No-regression, in CI (`COORDINATION §12`).

**`AC-FRESHNESS-ISOLATED` — the D1a obj-case freshness control is currently
MASKED, and this node owns the surface that masks it.** From Adversary advisory
hunt `evt_e9106h8ysr47` on respin `99a0b548`, **re-verified by the Steward
against that exact object before folding**:

- The WORLD control (`:226`) uses body `FokAccess (FokQParameter Zero)
  (FokQBound Zero)`. `Access` is World/World, so the stale eigen `Param0` is
  well-sorted for a `ForallWorld` eigen, sort-validation PASSES, and the `False`
  verdict comes from **freshness alone**. Break freshness and this reddens. It is
  a genuine pin.
- The OBJ control (`:236`) uses body `FokForcingP (FokQParameter Zero)
  (FokQBound Zero)`. **`ForcingP` is World/Object, so `Param0` sits in its WORLD
  slot while `ForallObj` requires an OBJECT eigen.** The shared parameter
  environment therefore rejects on a World-vs-Object conflict, and
  `validate(False) && structural` is `False` **independent of freshness**.
  ⇒ **Delete or break the structural freshness check and the obj test STAYS
  GREEN.** It does not test what its name says.

**Required:** after the relation is corrected, the obj-case freshness assertion
must be ISOLATED — inject the stale-eigen fault on a WELL-SORTED **Object**
non-fresh parameter, mirroring the clean world case, so that only freshness can
reject. **Prove it by mutation: break the freshness check and the obj control
must RED.**

> **THE HAZARD THIS NODE SPECIFICALLY CREATES, and why the obligation lives here
> rather than in its own node.** This increment makes `ForallRight`
> **parameter-only AND SORTED on both surfaces**. Once eigenparameters are sorted
> by construction, the ill-sorted shape the obj control currently relies on may
> become **unconstructible** — at which point the control does not merely stay
> masked, it can become VACUOUS while still compiling and passing. **A control
> that cannot fail is not weaker evidence; it is none.** Face this deliberately
> while restating the fixtures in lockstep; do not discover it afterwards.
>
> **Not filed as a separate node deliberately.** This node rewrites the checker,
> `FokDerivation`, and the reflection proofs — and therefore these fixtures — in
> one atomic increment. A competing node over the same file would contend with
> it and be superseded before it ran (`docs/PRINCIPLES.md`: subsume, do not
> proliferate). **The fold was safe because this node was still `draft` and
> unreleased when it was made** (2026-08-28, before this node was released); the
> same fold into a frozen, mid-flight node is the defect `AC-DERIVE` was recut to
> remove. **That window is now closed — this node is released, so any further
> criterion belongs to its successor or to a fresh node, not folded in here.**
>
> **LATENT, not a live defect, and not a reason to hold anything.** Freshness IS
> still pinned today by the world case. The Adversary classed it minor,
> non-blocking, with no action strictly required, and its verdict on the
> `99a0b548` repair was that the repair is CORRECT — a legitimate well-sorted
> rewrite of a fixture that was only accidentally valid before validation
> existed, **not** a dodge of an over-rejection bug.

## Banned scope

- **Re-establishing `embedding_adequacy`.** Item 6 of the envelope, and it is
  the successor [[V3-FO-EMBEDDING-ADEQUACY]]. Adequacy may be re-established
  **only** over the corrected relation, which does not exist until this lands.
- **Returning `proved` for FO, or wiring the verdict boundary.** `AC-8`.
- **Sort/scope validation of the checker's own domain.** That is the
  predecessor [[CORE-FO-CHECK-TREE-SORT-VALIDATION]] and must be in before this
  starts.
- **Widening the slice.** Unchanged.
- **The retained two-index limitation.** `LANG-INDEX-REFINEMENT-OMEGA-ARM` left
  a bounded two-index goal-restoration case unsupported (Architect
  `evt_7wbrfyvwv5517`); the supported transition is the single-index branch-goal
  witness. A multi-index need is a **hard stop to Steward and Architect**, not
  an elaborator repair from this node.

## Sequencing

**Second of three, and the one that cannot be subdivided.**
[[CORE-FO-CHECK-TREE-SORT-VALIDATION]] lands first (items 1-2, strictly
stricter, claims nothing); this node lands as one increment (items 3-5); then
[[V3-FO-EMBEDDING-ADEQUACY]] re-establishes the semantic claim (item 6).

**Sizing note, and it is not an acceptance criterion.** This is `L` and it will
not fit the one-hour turn. That is accepted: the alternative is a split the
Architect's item 5 forbids. Expect hard stops rather than partial landings, and
route them to the Steward.

## Relationship to the two nodes it supersedes

**[[V3-FO-CHECKER-SOUNDNESS]] stays `merged`.** Its deliverables did land, and
flipping it would make the tracker claim a landing that happened did not. What
it proved is a **structural reflection theorem for the relation it was given**,
and that relation is the one now rejected. It carries a banner pointing here.

**[[V3-FO-SUBST-DEPTH-CONTROL]] stays `merged`**, and its control obligation is
**subsumed by `AC-4`** above rather than re-cut as its own node. Its finding
(the shared producer/checker function, and the blindness of end-to-end rows) is
load-bearing here and is restated above so this frame stands alone.

## Provenance

Architect rejection ruling `evt_6hx31xvw9tqs2` (base `ef91b8225`, tree
`19e0543a4ac006b24b256a038e25e83f29894162`), items 3, 4 and 5 of its repair
envelope and its required-controls list; refuting certificate
`evt_2yh515wg0mczy` (language-implementer, exact base `ef91b8225`); Steward
disposition `evt_55w8hgwbc053r`; the shared-caller and depth-blindness
measurement on [[V3-FO-SUBST-DEPTH-CONTROL]] (Adversary `evt_d1wy8d6kytpw`).
