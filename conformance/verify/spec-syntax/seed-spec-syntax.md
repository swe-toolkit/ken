# Verification spec syntax (V1) conformance — seed cases

Format: `../../README.md`. These pin **WS-V V1**: the spec-annotation syntax
(`requires`/`ensures`/refinements/`prove`/`law`), its **elaboration to core**,
and the **verification status model** — the W1 contract in the released
`spec/20-verification/21-spec-syntax.md`. Expected results are grounded in the
current Spec body, settled decisions, and first principles. The prototype is
not mounted, and no external reference material was consulted for this W1
conformance revision.

The layer is untrusted, so the cases pin the elaboration shape (the emitted
core), the honesty guard (`unknown` never reads `proved`), and scope guards
(`old`, `result`, Ω-typing). Every emitted proof is re-checked by the kernel
(`18 §4`); an elaborator bug is a wrong term, verdict, or diagnostic, not a
kernel soundness failure.

**W1 representation target.** A refinement `{x:A|φ}` elaborates to the checked
subset `Σ(x:A).φ` at `Type (max ℓ_A ℓ_φ)` (`21 §2`/§6.3, `34 §5`). Its
carrier is relevant, its Ω proof is proof-irrelevant, and the Σ is not
convertible to `A`. Introduction constructs a checked pair; forgetting inserts
`Proj1` without an obligation. No coercion descends under a type former or
binder. Written `requires` remains a separate Π proof argument. The Π and Σ
sort rules are landed (`13 §4`); **subset-Σ elaboration expectations are
[deferred — W5]** while the transitional carrier-only implementation remains.

**W1 honesty target.** `unknown ≢ proved` requires a checked certificate and
no reachable open obligation hole (`21 §5.4`). `postulates_reachable(env, p)`
traverses transparent dependencies; an audited contract axiom can remain visible
without demoting a checked proof. **The transitive-query expectation is
[deferred — W4]**; `trusted_base()` membership alone is not the honesty test.

**Deferred-tag convention.** A case may describe a settled model whose concrete
spelling or elaboration capability has not landed. Tag only that portion
`[deferred — <named capability or section>]`, state the landed behavior
separately, and never present the deferred portion as currently accepted. Here
`[deferred — W5]` names the subset-Σ elaboration target and `[deferred — W4]`
names the read-only reachability query. The tag names implementation
availability, not whether the governing design decision is open.

The first instance is disposition-tag spelling (`21 §5.5`): V1 delivers the
status **model** and concrete grammar for
`requires`/`ensures`/`{x:A|φ}`/`prove`/`law` only; the **disposition-tag clause
spelling** (`tested`/`assume`/`test`/`delegated`) stays **reserved**
(`OQ-syntax`, downstream of `../70-behavioral/`). Cases that would exercise that
spelling are tagged **`[deferred — §5.5, OQ-syntax]`** and assert the model,
never un-landed grammar. The former second instance was the deferred-model
expectation in each `old` scope row. Pre-state elaboration for block-space
cells has landed, so those two row-level deferrals are retired. This does not
retire separate `OQ-Space` concurrency or space-placement gaps.

Cases tagged **(soundness)** encode the honesty/scope commitments the whole
model
rests on (`21 §5.4`) and must never regress.

---

## A. Syntax, AST, and elaboration to core (`21 §6.1`–§6.3)

### verify/spec-syntax/requires-elaborates-to-pi-proof-arg
- spec: `21 §6.3` (elabFn), `§6.1`/§6.2 (grammar/AST); `16 §1.1` (`sort_pi`)
- given: `fn divide (n : Int) (d : Int) : Int requires Not (Equal Int d 0) = n / d`
- expect: **accepts**; **(a — emitted type)** the emitted core type is
  `(n:Int) → (d:Int) → (_ : Not (Equal Int d 0)) → Int` — the precondition is
  a **Π proof-argument** and the function type stays at **`Type`** (not Ω);
  **(b — emitted body)** the core body is `λ n. λ d. λ p. (n / d)` — the bare
  carrier, no proof paired in.
- why: §6.3 lowers `requires φ` to a Π proof-arg, `check`ed at Ω then assumed in
  the body. `sort_pi(Ω, Type) = Type` (`16 §1.1`, codomain-keyed) — an Ω
  **domain** does not collapse the function to a proposition. Structural
  (emitted core), not just accept: a bug dropping the proof-arg, or mis-keying
  `sort_pi` on the **domain** (→ Ω), changes the emitted type — green-vs-red on
  the core shape. The two conformance attachments divide these labelled halves
  and jointly cover the expectation; neither attachment alone establishes both.

### verify/spec-syntax/refined-domain-and-requires-higher-order-boundary
- spec: `21 §2`/§6.3; `22 §2.1`/§2.3; `39 §5.3`
- status: refined-domain and pair-introduction behavior is **deferred — W5**;
  `requires`-as-Π behavior is the landed portion.
- given: the declarations below and the four applications named below.
- expect: `refined_fn` has domain `Pos = Σ(n:Int).Not (Equal Int n 0)`;
  `requires_fn` instead has an `Int` domain followed by a separate Π proof
  argument. `refined_slot refined_fn n` accepts, while
  `refined_slot requires_fn n`, `plain_slot refined_fn`, and
  `plain_slot requires_fn` reject with type mismatch. In `refined_slot`,
  applying variable `f` to plain `n:Int` constructs a subset pair and emits
  exactly one call-site obligation `Not (Equal Int n 0)`; the body receives no
  hidden proof binder. The explicit `requires` call burden remains separate.
- why: a refined domain stays a Σ binder in a higher-order Π type; it is not
  normalized to an `Int` binder plus an implicit proof argument. A bug that
  erases the refinement makes the rejected function assignments type-compatible;
  a bug that invents a hidden binder changes the accepted refined-function
  shape and loses the caller's pair-introduction obligation.

The fixture declarations are:

```ken
def Pos = {x:Int | Not (Equal Int x 0)}
fn refined_fn (d : Pos) : Int = d
fn requires_fn (d : Int) : Int requires Not (Equal Int d 0) = d
fn plain_slot (f : (d : Int) → Int) : Int = f 1
fn refined_slot (f : (d : Pos) → Int) (n : Int) : Int = f n
```

The applications are `plain_slot refined_fn`, `plain_slot requires_fn`,
`refined_slot refined_fn n`, and `refined_slot requires_fn n`. The accepted
case is `refined_slot refined_fn n`; its body `f n` is the caller-introduction
site.

### verify/spec-syntax/requires-on-first-param-of-two
- spec: `21 §6.3` (preconditions become proof parameters), `39 §5.3`
  (parameter binding and scope)
- given: `fn f (n : Int) (d : Int) : Int requires IsTrue (leq_int 1 n) = d`
- expect: **accepts**; the elaborated type is equivalent up to binder names to
  `(n : Int) → (d : Int) → (_ : IsTrue (leq_int 1 n)) → Int`, and the
  elaborated body is equivalent to `λ n. λ d. λ _. d`. The `requires`-only
  declaration emits no definition-site obligation holes.
- why: inserting the precondition proof parameter must preserve both source
  bindings: `Positive` still applies to the first parameter `n`, while the
  body still returns the later parameter `d`. This is the minimal non-final
  positional case and is structural rather than acceptance-only.

### verify/spec-syntax/requires-on-middle-param-of-three
- spec: `21 §6.3` (preconditions become proof parameters), `39 §5.3`
  (parameter binding and scope)
- given: `fn g (a : Int) (b : Int) (c : Int) : Int requires IsTrue (leq_int 1 b) = a`
- expect: **accepts**; the elaborated type is equivalent up to binder names to
  `(a : Int) → (b : Int) → (c : Int) → (_ : IsTrue (leq_int 1 b)) → Int`, and
  the elaborated body is equivalent to `λ a. λ b. λ c. λ _. a`. The
  `requires`-only declaration emits no definition-site obligation holes.
- why: this independently covers an interior parameter with neighbours on both
  sides. The proof domain must keep referring to `b`, and the body must keep
  referring to the earlier `a`, after the proof parameter is introduced.
  The first-of-two case does not cover that interior-telescope shape.

### verify/spec-syntax/ensures-result-is-subset-sigma
- spec: `21 §6.3` (ordinary `fn` result), `§2`/§6.5; `22 §2.2`
- status: **deferred — W5 subset-Σ elaboration**
- given: `fn inc (n : Int) : Int ensures Equal Int result (n + 1) = n + 1`
- expect: the core result type is `Σ(r:Int).Equal Int r (n + 1)` at
  `Type 0`; the straight-line body checks as `Pair(n + 1, π)` at that type.
  Exactly one postcondition introduction site records the goal
  `Equal Int (n + 1) (n + 1)` with provenance for the `ensures` clause. `π` is
  checked evidence or the applied typed hole for that goal; it is never absent
  from the pair.
- why: `ensures` is the kernel subset-Σ result motive, not a bare carrier plus
  an unrelated side obligation. A carrier-only result with the same obligation
  would not produce `Pair(n + 1, π)` or the checked Σ type. This pins the
  elaboration shape, not merely the prover's final status.

### verify/spec-syntax/ensures-result-level-is-predicative-max
- spec: `21 §6.3`/§8; `22 §2.2`; `13 §4`
- status: surface refinement formation is **deferred — W5**. Kernel Σ sort
  formation remains live through `kernel/pi-sigma/`.
- given: in a context with `A : Type 0` and `P : A → Ω_1`, elaborate
  `fn keep (x : A) : A ensures P result = x`.
- expect: the core result type is `Σ(r:A).P r : Type 1`, since
  `max 0 1 = 1`; the body introduces `Pair(x,π)` with a proof site at Ω_1.
  The function result does not remain at `Type 0` merely because its carrier
  does.
- why: ordinary `ensures` uses the same predicative subset-Σ formation rule as
  a named refinement. This distinguishes the W1 result type from the old
  carrier-only result plus a separate obligation.

### verify/spec-syntax/named-refinement-is-subset-sigma
- spec: `21 §2`/§6.3/§8; `34 §5`; `13 §4`
- status: **deferred — W5 subset-Σ elaboration**
- given: `def Pos = { n : Int | IsTrue (leq_int 1 n) }` and
  `fn keep (n : Int) : Pos = n`
- expect: `Pos` unfolds transparently to
  `Σ(n:Int).IsTrue (leq_int 1 n)` at `Type 0`, not to `Int`. The body checks
  as `Pair(n, ?h n) : Pos`; its open goal is
  `IsTrue (leq_int 1 n)` in the function context, with `?h` present in
  `trusted_base()`.
- why: this observes the named type's kernel form and its proof-bearing
  introduction. The carrier encoding could emit the same goal while leaving the
  core type and value as `Int`; the asserted Sigma type and pair distinguish
  W1. The relevant carrier keeps the result in `Type`.

### verify/spec-syntax/refinement-type-level-is-predicative-max
- spec: `21 §2`/§8; `13 §4`; `12 §2`/§3
- status: **deferred — W5 subset-Σ elaboration**
- given: `A : Type 0`, `P : A → Ω_1`, `a : A`, and `p : P a`; elaborate
  `def Ref = {x:A | P x}` and check `Pair(a,p) : Ref`.
- expect: `Ref` unfolds to `Σ(x:A).P x : Type (max 0 1) = Type 1` — not
  `Type 0` and not Ω. The pair checks at `Ref`; `Proj1` has type `A`.
- why: the kernel's predicative Σ formation uses both the relevant carrier and
  the predicate level. The old carrier encoding would leave `Ref` at `Type 0`;
  the observed sort distinguishes the W1 representation from that behavior.

### verify/spec-syntax/refinement-to-refinement-uses-source-proof
- spec: `21 §2`/§6.3; `22 §2.1`; `16 §1.3`
- status: **deferred — W5 subset-Σ elaboration**
- given: in context `Γ₀ = (P : Int → Ω)`, define
  `Positive = {n:Int | P n}` and `Strong = {n:Int | And (P n) Top}`; check the
  expression `x` at `Positive` under `Γ = Γ₀, (x : Strong)`.
- expect: the core body is `Pair(Proj1 x, ?h P x) : Positive`. It records one
  target-introduction obligation `P (Proj1 x)`; the closed goal's context
  contains `P` and `Proj2 x : And (P (Proj1 x)) Top`. V3 can discharge it with
  `Proj1 (Proj2 x)`; otherwise the typed hole, closed over all of Γ, is applied
  to `P` and `x`. It may not erase the pair.
- why: target-first coercion forgets the source with `Proj1` and introduces the
  target subset under the source pair's checked `Proj2` hypothesis. This is not
  kernel subtyping or conversion of the two subset types. A carrier-erasing
  implementation would return `x` as an identity term instead.

### verify/spec-syntax/no-coercion-under-list
- spec: `21 §2`/§6.3; `13 §4`; `18 §3`
- status: **deferred — W5 subset-Σ elaboration**
- given: `def Five = { n : Int | Equal Int n 5 }`; terms
  `xs = Cons Int 5 (Nil Int)` and
  `ys = Cons Five (Pair(5, Refl)) (Nil Five)`. Check each at both
  `List Int` and `List Five`.
- expect: the cross-checks reject with kernel `TypeMismatch` at
  `List Int` versus `List Five`; each term checks at its own list type. The
  kernel does not coerce the element type through `List`.
- why: W1 allows only outermost, type-directed introduction and forgetting.
  No coercion descends through `List`; the kernel sees distinct element types.
  The old carrier encoding would accept the mismatched list types because
  `Five` erased to `Int`, so this is a true discriminator, not a source-shape
  check.

### verify/spec-syntax/lambda-at-refinement-introduces-proof
- spec: `21 §2`/§6.3; `22 §2.1`
- status: **deferred — W5 subset-Σ elaboration**
- given: `def SevenAtZero = { f : (x:Int) → Int | Equal Int (f 0) 7 }`;
  check the lambda `λ x. 5` at `SevenAtZero` and at the plain type
  `(x:Int) → Int`.
- expect: at `SevenAtZero`, the core type is a subset Σ and the checked term
  is `Pair(λ x.5, π)` with one goal
  `Equal Int ((λ x.5) 0) 7` (β-reducing to `Equal Int 5 7`) and its applied
  typed hole. At the plain Pi type the lambda needs no subset pair or
  introduction obligation.
- why: check-only lambdas first check at the carrier and then introduce the
  expected subset once at the check entry. The old carrier encoding accepts
  the same lambda without a proof component; this case distinguishes that
  behavior from the W1 pair-plus-obligation rule.

### verify/spec-syntax/non-omega-predicate-surface-error
- spec: `21 §4` (every spec prop `: Ω`), `§6.3` (check at Ω before any
  obligation)
- given: `fn f (n : Int) : Int requires (n + 1) = n` — the clause body
  `n + 1 : Int`, **not** a proposition
- expect: **rejected** as a **surface type error** at elaboration
  (`TypeMismatch`: expected `Ω`, found `Int`), **before** any obligation is
  formed; **not** a verification failure, **not** silently admitted.
- why: §6.3 `check`s every clause body at Ω; a non-Ω body is rejected at
  elaboration (`§4`, "a load-bearing guard"). **Absence-assertion gate** — guard
  named: the explicit `check(Γ, φ, Ω)` in `elabFn`/`elabType`. **Disconfirming
  check:** would a non-Ω `requires` also reject under the bug this targets
  (omitting the Ω-check)? **No** — that bug would *accept* it and form an
  obligation over a non-proposition. **Verdict-flips:**
  `requires IsTrue (leq_int 1 n)` (`: Ω`) accepts; `requires (n + 1)` rejects —
  pins the Ω-gate, not a coincidental reject.

### verify/spec-syntax/result-scope
- spec: `21 §4` (`result` only in `ensures`), `§6.3`
- given: (a) `fn g (n:Int) : Int ensures IsTrue (leq_int 0 result) = …`; (b)
  `fn h (n:Int) : Int requires IsTrue (leq_int 0 result) = …`
- expect: (a) **accepts** — `result` resolves to the return value (`: Int`) in
  the `ensures` scope; (b) **rejected** — `result` in a `requires` is an
  **unbound-name / scope error** (`result` not in scope outside `ensures`).
- why: §4 binds `result` only in `ensures` clauses. **Verdict-flips on scope:**
  the same identifier resolves in (a), is unbound in (b). **Absence-assertion:**
  guard = the binder scope (resolution adds `result` only when elaborating an
  `ensures`). Disconfirming: a bug scoping `result` globally would *accept* (b)
  — so the reject is guard-gated, not coincidental.

---

## B. `old`-capture scope guard (`21 §6.4`) — the flip pair

### verify/spec-syntax/old-resolves-in-space-op-ensures
- spec: `21 §6.4` (elabSpaceEnsures), `36 §4.3`; `16 §2` (`refl`)
- status: `old` resolution is landed; the result pair is **deferred — W5**.
- given: a **block-space** op `inc` over a cell `n : Int` with
  `ensures Equal Int n (old(n) + 1)` and residual row `F = 𝟘`
- expect (landed): `old(n)` resolves to the pre-state projection and bare `n`
  to the post-state projection. `run_state` returns `Ret (r, s_post)`.
- expect (deferred — W5): with `F = 𝟘`, elaboration collapses the whole result
  type to `Σ(rs:Int × S).ψ(s_pre,rs)` and pairs the result with its proof.
  Substitution and record reduction discharge this `inc` example.
- why: the empty residual row collapses the general space rule to a pure
  state-transformer subset Σ, not a bare `(result, state)` carrier. The
  block-space operation installs distinct pre-state and post-state cell
  environments. Modifier-form `space proc` still has no cells or state
  environment and refuses `old` with `OldPreStateUnsupported`.

### verify/spec-syntax/space-ensures-residual-tree-allret
- spec: `21 §6.4`; `22 §2.2`; `36 §4.3`; `13 §4`
- status: **deferred — W5 subset-Σ space-result elaboration and AllRet**
- given: a space operation with residual row `F` and one `ensures ψ` clause;
  its residual tree is `t : ITree F (R × S)`. Use a residual operation whose
  response is `Bool`, with `t = Vis e (λ b. Ret (b, s_post))` and
  `ψ(b, s_post) = Equal Bool b True`. For the level control, take
  `P : X → Ω_0`, `x : X`, and `t_hi = Vis e_hi (λ a. Ret x)` with
  `E.Resp e_hi : Type 1`.
- expect (deferred — W5): the checked result type is
  `Σ(t : ITree F (R × S)).AllRet (λ rs.ψ(s_pre,rs)) t`; the proof is in one
  outer pair and one obligation, not a pair inside `ITree F` or one proof per
  user `Ret`. Its fold equations are
  `AllRet P (Ret rs) = P rs` and
  `AllRet P (Vis e k) = Π(r:E.Resp e).AllRet P (k r)`. On the sample tree the
  fold is `Π(b:Bool).Equal Bool b True`, so the False response leaves the
  obligation open; a tree whose every response returns `True` is the discharge
  control. `AllRet P t_hi` reduces to `Π(a:E.Resp e_hi).P x` at `Ω_1`; a result
  Σ with tree carrier at `Type ℓ_T` is at `Type (max ℓ_T 1)`. The output-type
  shape is W1; executing the helper awaits W5.
- why: the residual response is universal: checking only the first return
  would discharge the mixed tree incorrectly. The result's carrier remains the
  residual tree, and the `AllRet` proof is its outer subset-Σ component. Its
  Π level includes the response-domain level; this is predicative formation,
  not implicit cumulativity.

### verify/spec-syntax/old-out-of-scope-rejects (soundness)
- spec: `21 §6.4` (scope guard), `§4`, `36 §7.3`
- given: a **pure**
  `fn k (x : Int) : Int ensures Equal Int result (old(x) + 1) = x + 1`
- expect (landed): **rejected** as `UnboundName("old")` at scope resolution.
  With the same `old(…)` syntax in a block-space operation postcondition, the
  operation **accepts** and resolves `old` to its pre-state cell projection.
  The landed relation is therefore a true **reject/accept verdict flip**.
- why: spelling alone is not the guard. Pure code is resolver-rejected as
  `UnboundName("old")`; modifier-form `space proc` passes resolution but has no
  `space_pre_state`, so elaboration refuses it as `OldPreStateUnsupported`; a
  block-space operation passes resolution and installs the pre-state cell
  environment, so its contract accepts. Acceptance therefore requires both an
  admitting resolver scope and an installed block-space pre-state environment.
  The pure rejection proves the resolver scope remains live, while the modifier
  control proves declaration kind alone is insufficient. The pure/block pair
  retains the true reject/accept verdict flip.

---

## C. The verification status model + honesty guard (`21 §5`)

### verify/spec-syntax/proved-status-cert-checks-no-reachable-open-hole (soundness)
- spec: `21 §5.1` (verdict `proved`), `§5.4` (discriminator); `18 §4.5`/§5
- status: **deferred — W4 `postulates_reachable`**; certificate checking
  remains live.
- given: an `ensures` obligation `?h : φ` discharged by a certificate `p` with
  `Γ ⊢ p : φ` (a closed core proof term)
- expect: `check(env, Γ, p, φ)` accepts. **Deferred — W4:** the claim is
  `proved` only when `postulates_reachable(env, p)` reaches no open obligation
  hole. Audited contract axioms may remain visible in `trusted_base()` without
  demoting the checked proof.
- why: §5.1/§5.4 — a `proved` certificate is a core term that `check`
  validates and whose transitive dependencies contain no open obligation hole.
  This is the clean-certificate side; the refined-constant contrast follows.
  The cert is given, not prover-generated — V1 fixes the status model; the
  prover is V3.

### verify/spec-syntax/bogus-cert-not-proved (soundness)
- spec: `21 §5.1`, `§5.4`; `18 §4.5` (de Bruijn re-check); `../seed-verify.md`
  `verify/wrong-proof-rejected`
- given: the same goal `φ` and a **wrong** certificate `c` with `Γ ⊬ c : φ`
- expect: `check(env, Γ, c, φ)` **rejects** ⇒ verdict is **not `proved`** — the
  obligation stays an open hole (`unknown`), `φ` **remains** in
  `trusted_base()`.
- why: the kernel re-check is the soundness firewall around the untrusted prover
  (`18 §4`) — a wrong certificate cannot make `φ` `proved`. It contrasts with
  the clean-certificate case on the exact `check` result; transitive dependency
  honesty is tested separately by the W4 pair below.

### verify/spec-syntax/unknown-hole-distinct-from-proved (soundness)
- spec: `21 §5.4` (the honesty guard), `§5.1` (`unknown` = postulate); `18 §5`;
  `24 §2`
- status: the direct hole is a visible postulate; transitive status is
  **deferred — W4 `postulates_reachable`**.
- given: an `fn` with an `ensures φ` left **undischarged** — admitted with a
  typed hole `?h : φ`; the program type-checks and runs
- expect: the open hole's `GlobalId` appears in `trusted_base()`.
  **Deferred — W4:** it is reachable from the applied hole certificate and
  makes the claim `unknown`; a checked certificate with no reachable open
  obligation hole is `proved`. This own-hole case is only one arm of the
  transitive rule.
- why: §5.4 — the honesty discriminator is the checked certificate plus the
  kernel-side dependency query, not a V-layer flag. Membership of the goal
  itself is not a dependency test; the next case supplies the disconfirming
  transitive contrast. The read-only query is W4-deferred.

### verify/spec-syntax/proved-status-rejects-transitive-proj2-hole (soundness)
- spec: `21 §5.4`; `18 §5`; `24 §2`
- status: **deferred — W4 `postulates_reachable` query**
- given: a designated audited contract axiom `audit_top : Top` and an open
  obligation hole `?h : Top`. Define transparent
  `clean : Σ(n:Int).Top = Pair(0, audit_top)` and
  `refined : Σ(n:Int).Top = Pair(0, ?h)`. Let
  `p_clean = λ (_:Top).Proj2 clean` and
  `p_refined = λ (_:Top).Proj2 refined`; both target `Top → Top`.
- expect: both certificates pass `check`.
  `postulates_reachable(env, p_clean) = {id_audit}` and reaches no open hole,
  so `p_clean` is `proved`; the query for `p_refined` returns `{id_h}`, so
  `p_refined` is `unknown`. Both identities remain visible in `trusted_base()`.
- why: both certificates conclude `Top → Top`; the open hole has the distinct
  goal `Top` and is reached through the transparent constant's `Proj2`. No
  postulate carries the target goal, so a goal-local lookup misses the hole;
  demoting every reachable postulate misclassifies the audited-axiom control.
  This is the W4 discriminator for transitive proof honesty.

### verify/spec-syntax/disproved-distinct-from-unknown
- spec: `21 §5.1` (`disproved` = verification error), `§5.3` (projection),
  `24 §3`
- given: `fn f (n : Int) : Int ensures IsTrue (leq_int 1 result) = n` —
  false for `n ≤ 0`, with a **given** countermodel at that world (the prover is
  V3 — the countermodel is supplied here, as the certificate is in the
  `proved`/bogus-cert cases)
- expect: verdict **`disproved`** (carried evidence: the countermodel) — a hard
  **verification error** (`24 §3`) with **no epistemic status** (it is fixed,
  not exported); **distinct** from `unknown`: a `disproved` claim is an error to
  fix, an `unknown` claim leaves the program **running** with a hole.
- why: §5.3 — the projection maps `disproved` → *none* (a refuted claim is never
  shipped) and `unknown` → `unknown` (the program runs). The verdict trichotomy
  (`proved`/`disproved`/`unknown`) is **not** collapsible: `disproved ≠ unknown`
  (both "not proved", but one is an error, the other a running hole) —
  discriminates on the trichotomy, not merely "not proved".

---

## D. Epistemic projection + deferred disposition tags (`21 §5.2`/§5.3/§5.5)

### verify/spec-syntax/epistemic-projection-distinct
- spec: `21 §5.2`/§5.3 (the projection), `§5.5` (deferred tag grammar)
- status: the transitive `proved`/`unknown` discriminator is **deferred — W4**.
- given: the four epistemic statuses of `21 §5.2`, each by carried evidence
- expect (model): the projection (`§5.3`) pins them **distinct** —
  prove+`proved` → **proved** (checked certificate, no reachable open hole;
  W4); prove+`unknown` → **unknown** (reachable typed hole; W4);
  `test`/`assume` → **tested** (runtime/test + generator obligation); `delegate`
  → **delegated** (temporal-logic export, model-checking obligation). The
  verifier **never returns** `tested` or `delegated` — they are author-chosen
  annotations that **bypass the prover** (`§5.3`).
- expect (deferred): the transitive `proved` vs `unknown` distinction is
  **[deferred — W4]** (via the query cases in group C); the
  **`tested`/`delegated` clause spelling** is **`[deferred — §5.5, OQ-syntax]`**
  (`assume`/`test` reserved, grammar downstream of `../70-behavioral/`) — this
  case asserts the **model**, not un-landed grammar.
- why: §5.3/§5.5 — V1 delivers the status **model** (four meanings + projection
  + honesty guard) and concrete grammar only for the proof-disposition forms;
  the disposition-tag spelling is reserved. Tagging the deferred sub-part keeps
  the case from asserting grammar that does not yet exist.

---

## E. The V1→V2 interface (`21 §7`)

### verify/spec-syntax/obligation-hole-set-exposed-to-v2
- spec: `21 §7` (the interface), `§6.3`/§6.5; `22 §2.1`/§2.2/§3
- status: subset-Σ telescope/result shapes are **deferred — W5**.
- given: `def NonZero = {d:Int | Not (Equal Int d 0)}`;
  `fn f (n : Int) (m : NonZero) : Int` with `requires IsTrue (leq_int 1 n)`,
  postconditions `IsTrue (leq_int n result)` and `IsTrue (leq_int 0 result)`,
  and body `n + m`; callers `fn use (n:Int) (m:Int):Int = f n m` and
  `fn use_refined (n:Int) (m:NonZero):Int = f n m`.
- expect: the core telescope keeps `m` at
  `Σ(d:Int).Not (Equal Int d 0)`; the written `requires` is a separate Π
  proof argument. The result type is
  `Σ(r:Int).(IsTrue (leq_int n r) ∧ IsTrue (leq_int 0 r))`, with one proof
  component holding both per-clause proofs. Exactly two `ensures` obligations
  are recorded at `f`; each `Γ` contains `Proj2 m` and the written precondition
  once, in scope. The body forgets `m` through `Proj1`. At `use`, plain `m`
  creates a pair-introduction obligation; `use_refined` reuses its checked
  proof. Both callers separately owe the written `requires` proof for `n`.
  The `f n m` result is a subset-Σ pair; `use` forgets it with `Proj1` to return
  `Int`. Each site retains source provenance.
- why: V1 hands V2 a kernel-checkable subset-Σ term with obligation sites
  marked; V2 adds path-sensitive hypotheses and extracts each marked proof
  site, not a carrier-only side table. The shape distinguishes a Σ domain from
  a generated `requires` binder and keeps both `ensures` proofs in one result
  pair. Pins `21 §9` acceptance #4 (V2-ready).

---

## F. Goals — `prove` / `law` (`21 §3`, §6.3)

### verify/spec-syntax/prove-goal-obligation-and-postulate-binding (soundness)
- spec: `21 §3` (`prove`), `§6.3` (elabProve), `§5.4`; `11 §4`
- given: `prove add_comm : (a b : Int) → Equal Int (a + b) (b + a)`, before
  and after discharge
- expect: **before discharge** — a standalone obligation hole `?h : φ` is
  emitted; `add_comm` is bound as a **postulate** of `φ` (usable as a proof
  term), and `φ` **appears** in `trusted_base()` (status `unknown`). **After
  discharge** by a certificate `p` with `Γ ⊢ p : φ` that `check`s — the
  postulate is **retired**, `add_comm ↦ p`, and `φ` leaves `trusted_base()`.
  **Deferred — W4:** `add_comm` is `proved` only if
  `postulates_reachable(env, p)` reaches no open obligation hole.
- why: §6.3/§5.4 — `prove` is the degenerate (no function-body) obligation;
  proving is hole-filling, and the goal's own postulate is retired on discharge.
  The transitive dependency check for `proved` is W4-deferred and is covered by
  the `postulates_reachable` contrast above.

### verify/spec-syntax/law-all-omega-fields-is-proposition
- spec: `21 §3` (`law`), `§8`; `16 §1.3` (conjunction in Ω)
- given: `law Monoid (M) { assoc : … ; unit_l : … ; unit_r : … }` with each
  field a proposition (`: Ω`)
- expect: **accepts**; each field `check`s at Ω, and the all-Ω bundle is a
  **conjunction** landing in **Ω** (the bundle is itself a proposition) — the
  sound both-components-Ω case (`16 §1.3`).
- why: a `law` of all-Ω fields is `Σ`-of-Ω-into-Ω, which lands in Ω because
  both components are proof-irrelevant. Contrast the refinement subset Σ:
  its carrier is relevant, so it stays in `Type` (`13 §4`, `21 §2`). The
  paired conformance cases ensure the two sort rules remain distinct.

---

## G. Regression — V0 unchanged (`21 §6.2`)

### verify/spec-syntax/v0-unchanged-for-non-spec-programs (soundness)
- spec: `21 §6.2` (no regression), `§9` acceptance #5; `39 §5` (V0)
- given: a non-spec program, e.g. `fn id (A : Type) (x : A) : A = x` — no
  `requires`/`ensures`/refinement/`prove`/`law`
- expect: parses to **exactly** the V0 AST (the `ViewDecl`'s
  `requires`/`ensures` lists empty, no `TRefine`/`ProveDecl`/`LawDecl`),
  elaborates identically to V0, and the kernel-checked core term is
  **unchanged** from V0.
- why: §6.2 — V1 adds spec-carrying fields and new variants, but a spec-free
  program exercises **none** of them; its elaboration must be byte-identical to
  V0 (`39 §5`). Regression guard: any V1 change that altered non-spec
  elaboration flips this. Mirrors `../../kernel/judgments/seed-judgments.md`
  `judgments/k1-k2-judgments-still-green`.

---

## Coverage map (acceptance, `21 §9`)

- **#1 syntax + elaboration to core** — `requires-elaborates-to-pi-proof-arg`,
  `refined-domain-and-requires-higher-order-boundary`,
  `ensures-result-is-subset-sigma`,
  `ensures-result-level-is-predicative-max`,
  `named-refinement-is-subset-sigma`,
  `refinement-type-level-is-predicative-max`,
  `refinement-to-refinement-uses-source-proof`, `no-coercion-under-list`,
  `lambda-at-refinement-introduces-proof`,
  `non-omega-predicate-surface-error`, `result-scope`.
- **#2 four-way status honest** —
  `proved-status-cert-checks-no-reachable-open-hole`, `bogus-cert-not-proved`,
  `unknown-hole-distinct-from-proved`,
  `proved-status-rejects-transitive-proj2-hole`,
  `disproved-distinct-from-unknown`, `epistemic-projection-distinct`.
- **#3 space contracts** — `old-out-of-scope-rejects` produces
  `UnboundName("old")` in pure code; `old-resolves-in-space-op-ensures` pins
  pre/post-state resolution now and the `F = 𝟘` subset-Σ collapse
  **deferred — W5**.
  `space-ensures-residual-tree-allret` pins the outer residual-tree Σ; its
  Bool-response fold observation is deferred to W5. Modifier-form `space proc`
  has no cells and remains refused with `OldPreStateUnsupported`.
- **#4 V2-ready** — `obligation-hole-set-exposed-to-v2`.
- **#5 no regression** — `v0-unchanged-for-non-spec-programs`.
- **goals** — `prove-goal-obligation-and-postulate-binding`,
  `law-all-omega-fields-is-proposition`.

Build-sequencing: the W1/W5 target elaborates refinements and ordinary
`ensures` results to kernel subset-Σ values, using the existing
`Sigma`/`Pair`/`Proj` constructors and landed `sort_sigma` (`13 §4`). The
kernel checks each pair; the elaborator records and discharges the introduction
obligation or applies a visible typed hole. W5-deferred cases pin this target;
they do not claim that transitional carrier-only lowering already emits pairs.
