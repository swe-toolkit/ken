# Obligation generation (V2) conformance — seed cases

Format: `../../README.md`. These pin **WS-V V2** — the verification-condition
extractor: turning a V1-spec'd program into the **obligation set** (each a
triple `⟨id, Γ ⊢ φ, provenance⟩`). Grounded in the W1 `21`/`22` subset-Σ
contract, the `18 §4`/§5 certificate API, the kernel eliminator (`14 §3`),
`match → elim_D` (`39 §2.6`), and first principles. Refined parameters remain
Σ domains; they are not normalized to a carrier binder plus a generated
`requires` proof argument. No external reference material was consulted for
this W1 conformance revision. (Partial-primitive grounding `35 §3`/`43 §2` is
not a forward reference.)

**The layer is ★★ (untrusted) — but read the backstop precisely (Architect).** A
V2 bug never breaks **kernel** soundness: the kernel re-checks every *supplied*
certificate (`18 §4`), so a **spurious** obligation is over-conservatism (a
false
`unknown`) and a *bad* cert is kernel-rejected. But a **missed** obligation — a
burden the extractor **never emits** — is **not** caught downstream: the V1 §5.4
honesty guard (`trusted_base()`) catches **generated-but-undischarged** holes,
not
sites that were never turned into holes, so a never-emitted burden reads as
`proved` though unproven. **Completeness-of-extraction is therefore the
*verification*-soundness linchpin, backstopped by nothing but the absent-clause
scan (§2.5)** — "all obligations discharged ⇒ correct" is only as strong as the
guarantee that **no burden was silently skipped**. So these cases pin
**completeness of extraction** (every burden site → its obligation; the
absent-clause scan audits that no burden is silently skipped *and* no trivial
clause over-skipped — the load-bearing safeguard, not a nicety) and **honest
provenance** (each obligation traces to its source clause + has a stable id).

**V2 consumes the checked subset-Σ form (`22 §1.1`).** V1 emits a kernel-
checked `Pair(a,π)` at `Σ(x:A).φ`; an open introduction hole is the applied
proof component, not a free-standing fact detached from the value. V2 associates
the marked proof site with its obligation and extracts it without skipping the
pair. The existing `sort_sigma` rule places a relevant carrier with an Ω proof
in `Type (max ℓ_A ℓ_φ)` (`13 §4`); V2 reads that checked core type rather than
re-encoding the refinement.

**Two-sided completeness (the spine of these cases).** The extractor must
**emit**
at every burden site (§2.1–§2.4) **and** must not emit where there is no burden
—
but each no-emit position is **explicitly guarded** (§2.5), so a *missing* guard
(a silently-dropped clause) is detectable. Every no-emit case below names its
**guard** and passes the **disconfirming check** ("would an obligation wrongly
appear / vanish here under the targeted bug?").

Cases tagged **(soundness)** encode the completeness/honesty commitments the
verdict model rests on (`22 §2.5`, `21 §5.4`) and must never regress.

---

## A. Extraction completeness — the four sources (`22 §2`)

### verify/obligations/refinement-introduction-emits-phi
- spec: `22 §2.1`; `21 §2` (subset-Σ introduction)
- given: `def Pos = { n : Int | IsTrue (leq_int 1 n) }`; a function
  `fn keep (n : Int) : Pos = n`
- expect: the core body is `Pair(n, ?h n) : Σ(n:Int).IsTrue (leq_int 1 n)`.
  It records **one** open introduction goal
  `⟨id, Γ ⊢ IsTrue (leq_int 1 n), prov⟩`; the applied typed hole is the pair's
  proof component and appears in the trusted base. Provenance points to the
  introduction site.
- why: §2.1 constructs a checked pair at the subset type and records the
  substituted proposition. A carrier-only body with a detached obligation
  misses the kernel-checkable proof component; a free `n`-goal or no goal is
  also wrong. The reverse forgetful direction emits no obligation and uses
  `Proj1` (`forgetful-coercion-emits-nothing`).

### verify/obligations/postcondition-emits-substituted-goal
- spec: `22 §2.2`; `21 §6.3`
- given: `fn inc (n : Int) : Int ensures Equal Int result (n + 1) = n + 1` —
  a **straight-line** body
- expect: the core result type is `Σ(r:Int).Equal Int r (n + 1)` and the body
  introduces `Pair(n + 1, π)`. It records **one** goal
  `⟨id, Γ,(n:Int) ⊢ Equal Int (n + 1) (n + 1), prov⟩` — `ψ[b/result]` with
  `result` replaced by `b = n + 1`; `π` is checked evidence or the applied
  typed hole for that introduction.
- why: §2.2 — for a straight-line body the subset-Σ result motive degenerates
  to one leaf introduction. The goal mentions the body, not a free `result`;
  the checked pair carries the proof. A branchy or recursive body introduces
  one pair per leaf under that branch's path conditions and induction
  hypotheses, not one over-the-body obligation (`§3`/§4; see
  `conditional-branch-adds-boolean-equation` and
  `recursive-fn-per-ctor-obligation-with-ih`).

### verify/obligations/precondition-obligation-at-call-not-in-body (soundness)
- spec: `22 §2.3`, `§2.5.2`
- given: `fn safe_div (n:Int) (d:Int) : Int` with
  `requires Not (Equal Int d 0) = n / d`, and a caller
  `fn use (x:Int) : Int = safe_div x 2`
- expect: the precondition `Not (Equal Int d 0)` yields an obligation **at the
  call** in `use` — `⟨id, Γ_call ⊢ Not (Equal Int 2 0), prov(call)⟩` (the caller
  meets it); **inside** `safe_div`'s body it yields **no** obligation — the
  proposition is an **assumption** in `Γ`.
- why: §2.3 — the precondition is the **caller's** burden, discharged at the
  call; the callee assumes it. **Verdict/structural flip on placement:** the
  obligation appears at the call (`Not (Equal Int 2 0)`) and is **absent**
  (present as a Γ-assumption) in the body.
  **Absence-assertion (no body obligation)** — guard:
  a precondition enters `Γ` at the body top (§3), never the function's own goal.
  **Disconfirming:** would the body also carry no
  `Not (Equal Int d 0)` obligation under the bug that re-obligates the
  precondition? **No** — that bug emits a **spurious**
  body goal the callee cannot discharge (it has no proof its own argument is
  nonzero). The asymmetry **is** the contract.

### verify/obligations/partial-primitive-emits-nonzero-obligation
- spec: `22 §2.4`; `35 §3` (Int div/mod by zero = obligation); `43 §2`
- given: (a) an unrefined division `n / d` on `Int` with possibly-zero `d`;
  (b) `fn f (d : {x:Int | Not (Equal Int x 0)}) : Int = 1 / d`; (c) the same
  body with `d : Int requires Not (Equal Int d 0)`.
- expect: (a) emits one non-zero obligation
  `⟨id, Γ ⊢ Not (Equal Int d 0), prov(op)⟩`. In (b), the operation uses
  `Proj1 d` and `Proj2 d` discharges its non-zero side condition; in (c), the
  written `requires` supplies a separate Π proof argument. Neither control
  emits a new PartialPrim obligation.
- why: §2.4 — a partial primitive (`/`, `%` on `Int`) emits its side condition
  (`35 §3`: "possibly-zero is an obligation, not a silent trap";
  cross-referenced from `43 §2`). **Verdict-flips:** possibly-zero divisor →
  non-zero obligation; refined/assumed-nonzero divisor → no new obligation.
  (`Int` is arbitrary-precision, so the partial case is **div-by-zero**, not
  overflow; fixed-width overflow is the analogous side-condition for sized
  types.)

### verify/obligations/prove-and-law-emit-one-obligation-per-goal
- spec: `22 §2.4` (degenerate `prove`/`law`); `21 §3`
- given: (a) `prove add_comm : (a b : Int) → Equal Int (a + b) (b + a)`; (b)
  `law Monoid (M) { assoc : … ; unit_l : … ; unit_r : … }`
- expect: (a) **one** obligation
  `⟨id, Γ_binders ⊢ Equal Int (a + b) (b + a), prov⟩` (no
  body — the degenerate case); (b) **one obligation per field** (3 here).
- why: §2.4 — `prove` is the degenerate (bodyless) obligation; `law` emits one
  per field. Structural: the obligation **count** (1 / 3) + each prop. A bug
  merging the `law` fields into one obligation, or dropping the bodyless
  `prove`, flips the count.

---

## B. The absent-clause scan — guarded no-emit + the counter-rule (`22 §2.5`)

### verify/obligations/refined-param-is-sigma-domain (soundness)
- spec: `21 §2`/§6.3; `22 §2.1`/§2.3/§3
- given: `fn head (xs : { l : List A | Not (Equal (List A) l (Nil A)) }) : A = …`
  and a caller that supplies plain `xs : List A`
- expect: the function binder is
  `xs : Σ(l:List A).Not (Equal (List A) l (Nil A))`; its proof assumption is
  the checked term `Proj2 xs`, not a generated Π binder. The declaration emits
  no definition-site introduction obligation. The caller introduces a pair
  and owes `Not (Equal (List A) xs (Nil A))` at that call. Supplying an already
  refined value reuses its checked `Proj2` without a new proof obligation.
- why: a refined parameter is a real Σ domain. It is not normalized to a
  carrier binder plus an implicit `requires`; only written `requires` creates
  a separate Π proof argument. The no-definition-site-obligation guard remains
  because the parameter itself is not an introduction site; the call is.

### verify/obligations/body-requires-assumed-not-reobligated
- spec: `22 §2.5.2`, `§3`
- given: inside `safe_div`'s body (above), the precondition
  `requires Not (Equal Int d 0)`
- expect: `Not (Equal Int d 0)` is **assumed** through its Π proof argument at
  the top of the body and is **not** re-emitted as an obligation of `safe_div`.
- why: §2.5.2 — a precondition, once inside the body, is an assumption (the
  caller owes it, §2.3). **Absence-assertion** — guard: a precondition enters
  `Γ` at the body top, never the function's own goal. **Disconfirming:** would
  the body lack this obligation under the bug that re-obligates it? **No** —
  that bug emits a spurious unprovable body goal. Pairs with
  `precondition-obligation-at-call-not-in-body` (the two halves of one
  asymmetry).

### verify/obligations/present-cert-yields-zero-new-obligations (soundness)
- spec: `22 §2.5.3`; `21 §5.4` (discharged hole leaves `trusted_base()`);
  `18 §4.5`
- given: `prove p : φ := <a valid certificate term>` — a `prove` whose
  discharging term is already supplied and `check`s
- expect: **zero new** obligations — the certificate **is** the discharge; the
  hole is **retired** (not re-emitted), and `φ` does **not** appear in
  `trusted_base()`.
- why: §2.5.3 — a site whose certificate is already present yields no open
  obligation (proving = hole-filling, and this hole is filled).
  **Absence-assertion** — guard: a discharged hole is not in the open set
  (`21 §5.4` — the goal leaves `trusted_base()`). **Disconfirming:** would an
  **un**discharged `prove φ` also yield zero obligations? **No** — without a
  cert it emits the open obligation and `φ` **stays** in `trusted_base()` (the
  `prove-and-law` case). Zero-vs-one obligations flips on cert-presence. (Ties
  the honesty guard `21 §5.4` to extraction.)

### verify/obligations/forgetful-coercion-emits-nothing
- spec: `22 §2.1`/§2.5.4; `21 §6.3`
- given: a value `p : Pos` (`= Σ(n:Int).IsTrue (leq_int 1 n)`) used where
  plain `Int` is expected — the forgetful direction `{x:A|φ} ≤ A`
- expect: **no** obligation; the core coercion is `Proj1 p : Int`, not the
  identity term. The checked proof component is forgotten without evaluation.
- why: forgetting is free of proof obligations but preserves the fact that
  `Pos` is a distinct Σ type. The introduction direction constructs a pair and
  owes `φ[a]` (`refinement-introduction-emits-phi`); the two directions differ
  by `Pair` versus `Proj1`, not by subtyping.

### verify/obligations/trivial-clause-still-emits-obligation (soundness)
- spec: `22 §2.5` (the completeness counter-rule); acceptance `§8` / frame `§1`
- given: (a) `fn f (n : Int) : Int ensures Equal Int result result = n` (a
  **trivially-true** postcondition); (b)
  `fn g (n : Int) : Int ensures IsTrue (leq_int 0 result) = n * n` (a
  **real-burden** postcondition) — both **straight-line** bodies (one obligation
  each, isolating the trivial-vs-real axis from path-sensitivity)
- expect: **both** emit a postcondition obligation — (a)
  `⟨id, Γ ⊢ Equal Int b_f b_f, prov⟩` (provable, discharged trivially by
  `refl`); (b) `⟨id, Γ ⊢ IsTrue (leq_int 0 (n * n)), prov⟩` (a real burden).
  **Neither** yields *no* obligation.
- why: §2.5 counter-rule — emission is keyed on the **clause's presence**, never
  a triviality heuristic; a trivially-true clause yields its **provable**
  obligation, not no obligation. **The absent-clause discriminating property:**
  a real-burden and a trivial clause both **emit** distinct obligations.
  **Disconfirming:** would (a) emit no obligation under a "skip the
  obviously-true ones" optimization? **Yes** — and that *is* the bug: a clause
  mis-judged trivial is silently dropped, reading as "verified." Green
  (obligation emitted, discharged by `refl`) vs red (no obligation — a missed
  check), on emission keyed to clause presence. The load-bearing completeness
  audit.

### verify/obligations/exhaustive-traversal-no-silent-skip (soundness)
- spec: `22 §2.5` (the exhaustiveness property, Architect-required), `§5`
- given: a core construct at a position the extractor has **no explicit rule
  for** — neither an emit site (§2.1–§2.4) nor an explicit Γ-extension (§3) nor
  an explicitly-guarded no-emit (§2.5) — e.g. a future burden-bearing core form
  added without an extraction rule
- expect: the extractor surfaces a **visible gap** — an **error** (or a
  conservative emit) — **never** a silent recurse-past. The traversal is
  **exhaustive by construction**: every core form is an emit site, an explicit
  Γ-extension, or an explicitly-guarded no-emit; there is **no** catch-all
  `_ ⇒ skip`.
- why: completeness-of-extraction is the **sole** backstop of verification
  soundness (preamble) — a missed obligation is *not* caught downstream — so the
  one thing that must never happen is a burden silently no-emitted. **Structural
  / absence assertion (the guard is exhaustiveness by construction, not a value
  flip):** adding a burden-bearing core form **without** an emit rule must be a
  **compile/visible failure**, not a silent miss. This is not a value-verdict
  (no program exhibits it today — the core `Term` set is fixed); it is a
  property of the **extractor's shape** — assert that the traversal `match` has
  **no catch-all `_ ⇒ skip`** (which would silently swallow a future variant),
  so an unmatched construct is emit-or-error. **Disconfirming:** would a future
  burden-bearing variant be silently skipped under a catch-all skip? **Yes** —
  that is exactly the bug this forbids; the property is green (visible gap /
  error) vs red (silent vanish). The normative form of §2.5's "every no-emit is
  an **explicit guarded skip**, so a missing clause is a **visible gap, not a
  silent drop**."

---

## C. Path-sensitive hypothesis accumulation — the context Γ (`22 §3`)

### verify/obligations/match-branch-gamma-carries-scrutinee-equation (soundness)
- spec: `22 §3` (match constructor equation), `§4`; `39 §2.6` (match→elim_D)
- given: `fn f (xs : List Int) : Int ensures P result = …` whose body is
  `match xs { nil → e0 ; cons y ys → e1 }` where the `cons`-branch goal
  discharges only by knowing the scrutinee shape
- expect: in the `cons y ys` branch, the obligation's `Γ` carries the
  **scrutinee equation** `(_ : Eq (List Int) xs (cons y ys))` with its checked
  convoy term and binds fields `y, ys`; the nil branch carries its equation
  with term evidence too. Each branch introduces its result as a checked pair
  at the postcondition's subset-Σ motive.
- why: §3 — a case split adds, per branch, the constructor equation and its
  checked convoy evidence. The leaf's subset-Σ proof can apply its obligation
  hole to that term. An unchecked proposition in `Γ` cannot be used as a proof;
  dropping either the equation or its term yields a false `unknown`.

### verify/obligations/let-binding-adds-equation-to-gamma
- spec: `22 §3` (let-equation)
- given: a body `let m := n + 1 in …` with a downstream obligation that
  discharges only via `Equal Int m (n + 1)`
- expect: the obligation's `Γ` carries `(m : Int)` and the equation
  `(_ : Equal Int m (n + 1))`.
- why: §3 — `let x := e` adds `x` and (where informative) the equation
  `Equal A x e`, so later obligations may rewrite by the binding. Structural
  flip: the obligation needing `Equal Int m (n + 1)` is provable with the
  let-equation in `Γ`,
  unprovable without it (too-weak `Γ` → false unknown).

### verify/obligations/conditional-branch-adds-boolean-equation
- spec: `22 §3` (conditional)
- given: `fn f (n : Int) : Int` with
  `ensures IsTrue (leq_int 0 result) = if leq_int 0 n then n else 0`
- expect: the then-branch obligation's `Γ` carries
  `(_ : Equal Bool (leq_int 0 n) true)` with its checked convoy term; the else
  branch carries `(_ : Equal Bool (leq_int 0 n) false)` with its term. Each
  branch's result is a checked pair at the postcondition subset-Σ type.
- why: §3 — `if c` adds `Equal Bool c true` / `false` per branch (elaborated
  `elim_Bool`) together with checked convoy terms. The `then` proof component
  can use that term to discharge `IsTrue (leq_int 0 n)`; a proposition in `Γ`
  without evidence cannot inhabit the subset pair's proof component.

### verify/obligations/non-direct-requires-carries-into-partialprim-telescope
- spec: `21 §1` (requires premise); `22 §2.4` (PartialPrim); `22 §3`
  (body context); `35 §3.1` (Int `/` and `%`).
- given: for each of `/` and `%`, a declaration without a caller:
  `fn f (n : Int) (d : Int) : Int requires Equal Int d 5 = n / d`, and
  the same declaration with `%` in place of `/`.
- expect: one operation-site `PartialPrim` obligation per operator. Its
  `goal_closed` has the telescope
  `Π (n : Int). Π (d : Int). Π (_ : Eq Int d 5). NonZeroDivisor d`.
  The `Eq Int d 5` binder follows both parameter binders and appears at
  parameter depth; `NonZeroDivisor d` remains the goal under that binder.
- why: `Equal Int d 5` is an Ω-valued, non-direct `requires` premise. It enters
  the body context but does not convert to `NonZeroDivisor d`, so each
  operation site emits one obligation. **Structural flip:** correct closure
  contains the `Eq Int d 5` binder; dropping `requires` from `close_goal`
  leaves it out.

---

## D. Body-as-motive — induction surfaced from `elim_D` (`22 §4`)

### verify/obligations/recursive-fn-per-ctor-obligation-with-ih (soundness)
- spec: `22 §4`; `14 §3` (eliminator); `39 §2.6`
- given: `def NonNeg = { n : Int | IsTrue (leq_int 0 n) }`; a recursive
  `fn sum (xs : List NonNeg) : Int` with
  `ensures IsTrue (leq_int 0 result) = …` and body
  `match xs { nil → 0 ; cons y ys → y + sum ys }`
- expect: the extractor emits per-constructor obligations from the `elim_D`
  motive `M z = Σ(r:Int).IsTrue (leq_int 0 r)`. The nil branch introduces a
  pair and records `IsTrue (leq_int 0 0)`. In the cons branch, the result pair's
  obligation is `IsTrue (leq_int 0 (Proj1 y + Proj1 (sum ys)))`; the context
  contains `Proj2 y` and the induction hypothesis
  `M ys : Σ(r:Int).IsTrue (leq_int 0 r)`, whose `Proj2` is evidence for the
  recursive result.
- why: §4 — the dependent eliminator gives each constructor method the IH for
  its recursive fields (`M zᵢ`); V2 reads these from the elaborator's
  `match → elim_D` compilation and adds them to `Γ` (it does not synthesize an
  induction principle). The `cons` proof can use `Proj2 y` and the proof
  projection of `M ys`; dropping either yields a false `unknown`. The result
  pair is the motive itself, not a carrier plus a separate postcondition.

### verify/obligations/nonrecursive-degenerate-no-induction-hypothesis
- spec: `22 §4` (degenerate motive), `21 §2`/§6.3
- given: `def NonNeg = { n : Int | IsTrue (leq_int 0 n) }`; a non-recursive
  `fn double (n : NonNeg) : Int ensures IsTrue (leq_int n result) = n + n`
- expect: the obligation `IsTrue (leq_int (Proj1 n) (Proj1 n + Proj1 n))` is
  emitted with no induction hypothesis in `Γ`. The refined parameter binder is
  a subset Σ, and its proof assumption is `Proj2 n`; the straight-line result
  introduces one subset pair.
- why: §4 — non-recursive functions are the degenerate motive (no recursive
  fields ⇒ no IH); the same machinery covers both, no special-casing. The
  explicit projections distinguish the Sigma parameter from its carrier.
  With `recursive-fn-…-with-IH`, the pair pins that an IH appears iff there is
  a recursive field.

---

## E. The acceptance flagship (`22 §8`)

### verify/obligations/inductive-postcond-hole-localization (soundness)
- spec: `22 §8` (acceptance), `§5`, `§6`; `18 §4.5`; `21 §5.4`
- given: the recursive `sum` (above) with its per-constructor obligations
  **supplied with valid proofs**; then the **same** with the `cons`-branch proof
  **removed**
- expect: (a) with all proofs — every obligation's certificate `check`s in the
  kernel (`18 §4.5`); the definition is **fully verified** (empty open set;
  nothing from it in `trusted_base()`). (b) with the `cons` proof removed — a
  **single, precisely-located** open hole at the `cons`-branch obligation
  `IsTrue (leq_int 0 (y + sum ys))` (status `unknown`); its goal **appears** in
  `trusted_base()`; the other obligations stay `proved`.
- why: §8 — the flagship end-to-end. **Structural/verdict flip:** all-proofs →
  fully verified; one-proof-removed → exactly **one** localized hole (its
  `Γ ⊢ φ` identity + provenance pin *which*), the rest unaffected.
  **Disconfirming:** does removing a needed proof leave a vague/global failure,
  or a precisely-located hole? §8 requires the latter — the hole carries its
  `Γ ⊢ φ` and provenance, and only that obligation flips to `unknown`. Pins the
  honesty guard (`21 §5.4`) end-to-end at the obligation grain.

---

## F. Interface, provenance, and regression

### verify/obligations/provenance-and-stable-ids
- spec: `22 §1` (stable ids + provenance), `§6`; `24 §6`
- given: a multi-clause definition; then an **unrelated** edit elsewhere (e.g.
  renaming a local in a *different* function)
- expect: each obligation carries `provenance` = source span + responsible
  clause (`requires`/`ensures`/refinement/`prove`); the obligation `id`s are
  **stable** across the unrelated edit (so a verification run diffs cleanly).
- why: §1/§6 — ids are stable across edits unrelated to the clause (`24 §6`),
  and provenance traces each obligation to its clause. Structural: the id of an
  untouched clause's obligation is unchanged after the unrelated edit; a bug
  keying ids on global position (not clause identity) flips them.

### verify/obligations/all-ret-bind-composes-evidence
- spec: `21 §6.4`; `22 §2.2`; `36 §4.3`
- status: **deferred — W5 AllRet declarations**
- given: `t : ITree F X`, `k : X → ITree F Y`, predicates
  `P : X → Ω` and `Q : Y → Ω`, evidence `h : AllRet P t`, and a checked
  continuation proof `step : Π(x:X).P x → AllRet Q (k x)`.
- expect: the checked Ken lemma `all_ret_bind` produces evidence of
  `AllRet Q (bind t k)`. A `Ret x` control reduces to `step x h`; a `Vis` case
  retains the universal response premise. With closed proof inputs, loading the
  declaration adds no `trusted_base()` delta.
- why: the consumer composes the outer `AllRet` proof through residual effects;
  it is a checked `elim_ITree` theorem in W5, not an elaborator coercion. The
  deferred case observes its proof behavior and trust delta when the declaration
  lands; W1's space result type is tested separately in
  `space-ensures-residual-tree-allret`.

### verify/obligations/v2-extracts-subset-sigma-proof-site (soundness)
- spec: `22 §1.1`/§5; `21 §2`/§6.3; `13 §4`
- given: `def Pos = {x:Int | IsTrue (leq_int 1 x)}` and
  `fn keep (n:Int) : Pos = n`, with checked core
  `Pair(n, ?h n) : Σ(x:Int).IsTrue (leq_int 1 x)`
- expect: V2 associates exactly one obligation
  `Γ ⊢ IsTrue (leq_int 1 n)` with the pair's proof site and source provenance.
  It does not skip the `Pair`, detach the hole from its proof component, or
  invent a second obligation. The subset type forms in
  `Type (max ℓ_A ℓ_φ)`; the proof component's Ω sort does not collapse it.
- why: V2 walks V1's checked core subset-Σ form. The kernel re-checks the pair;
  V2 supplies the completeness and provenance net for the applied hole inside
  it. The old carrier encoding could produce a free obligation with no
  proof-bearing core pair, so the paired observation discriminates the W1
  producer/consumer contract.

### verify/obligations/non-spec-program-empty-obligation-set (soundness)
- spec: `22 §8` (regression), `§6`; `21 §6.2`
- given: a non-spec program — no `requires`/`ensures`/refinement/`prove`/`law`,
  no partial primitive — e.g. `fn id (A : Type) (x : A) : A = x`
- expect: the **empty** obligation set; V1/V0 elaboration of the program is
  **unchanged**.
- why: §8 — a program with no proof burden yields no obligations; V2 adds
  nothing to a spec-free program (the extractor's emit clauses never fire).
  **Regression guard.** Mirrors `../spec-syntax/seed-spec-syntax.md`
  `v0-unchanged-for-non-spec-programs` (V1 side): spec-free → unchanged
  elaboration (V1) → empty obligation set (V2).

---

## Coverage map (acceptance, `22 §8` / frame `§1`–§5)

- **#1 extraction completeness** — `refinement-introduction-emits-phi`,
  `postcondition-emits-substituted-goal`,
  `precondition-obligation-at-call-not-in-body`,
  `partial-primitive-emits-nonzero-obligation`,
  `prove-and-law-emit-one-obligation-per-goal`; the **absent-clause scan**
  (`refined-param-is-sigma-domain`,
  `body-requires-assumed-not-reobligated`,
  `present-cert-yields-zero-new-obligations`,
  `forgetful-coercion-emits-nothing`) + the counter-rule
  (`trivial-clause-still-emits-obligation`) + the exhaustiveness backstop
  (`exhaustive-traversal-no-silent-skip` — the sole verification-soundness
  safeguard).
- **#2 Ω + context correctness** —
  `match-branch-gamma-carries-scrutinee-equation`,
  `let-binding-adds-equation-to-gamma`,
  `conditional-branch-adds-boolean-equation`,
  `non-direct-requires-carries-into-partialprim-telescope`, + body-as-motive
  (`recursive-fn-per-ctor-obligation-with-ih`,
  `nonrecursive-degenerate-no-induction-hypothesis`).
- **#3 subset-Σ extraction** — `v2-extracts-subset-sigma-proof-site`.
- **W5-deferred** — `all-ret-bind-composes-evidence` (checked Ken theorem,
  no kernel/trust change).
- **#4 V2→V3 interface** — `inductive-postcond-hole-localization` (the
  obligation set → per-obligation verdict), `provenance-and-stable-ids`.
- **#5 no regression** — `non-spec-program-empty-obligation-set`.

Build-sequencing: W1's V1 contract supplies checked subset-Σ terms and marked
proof sites; V2 consumes that interface (`21 §7`) with the `18 §4`/§5
certificate API and the `match → elim_D` elaboration (`39 §2.6`). The kernel's
`sort_sigma` rule is landed (`13 §4`); the obligation goals stay in Ω by
construction (`22 §7`: substitution preserves Ω, `11 §5`).
