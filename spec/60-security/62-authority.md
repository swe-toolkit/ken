# Authority, capabilities, and least privilege

> Status: **Sec2 elaborated** — implementation-ready for Team Verify (WS-Sec).
> **Normative for the authority discipline**: no-ambient enforcement,
> in-type capability tokens, **monotone-downward attenuation**, **transitive
> revocation** (static contract plus §4's bounded OS-operation behavior;
> runtime mechanism deferred), and
> **statically-known audit points** (§1–§6, §H). The concrete surface
> *spelling* stays proposal-level (`OQ-syntax`). This is the **authority** half
> of Ken's tier-1 security story (ADR 0004); the **flow** half is
> `61-information-flow.md`, with which it composes (§6). **Settled inputs (do
> *not* reopen): `OQ-8a` DECIDED** — capabilities are first-class value tokens,
> distinct from logical `requires` (`../30-surface/36 §3`); **`OQ-Space`
> DECIDED** — shared-nothing message-passing over `space` cells. Section 4 pins
> the current implicit root space's OS-operation revocation behavior; general
> multi-space and cross-space realization remains deferred to `40-runtime`
> (`../30-surface/36 §4`).
>
> **Surface standing.** The security requirements in this chapter remain
> committed. Present-tense operation claims, however, extend only to the
> authority-indexed filesystem surface landed by `38 §1.3.1`. The bounded
> use-site authority predicate, a distinguishing pair of live authority sinks,
> and an authority-indexed network surface are committed targets that the
> current source language does not yet instantiate. Sections 7, 8, and H name
> those gaps and their exit conditions; example metanotation does not make them
> current Ken.
>
> **No new kernel rule.** No-ambient is the L5 **capability-passing
> translation** (`36 §2.5`: a `perform_E` is ordinary Π/λ over the `Cap E`
> value); attenuation rides the **landed refinement types**
> (`../30-surface/34 §5`, `../20-verification/21 §2`) as a kernel-re-checked
> obligation (§3); revocation combines the static lineage contract with §4's
> bounded runtime behavior, while its mechanism remains deferred; audit points
> are the **`Vis` nodes the type already declares** (§5). The kernel gains
> **nothing** — capabilities are ordinary typed values and the authority order
> is an ordinary `61 §2` lattice value (§2).
>
> **Perishable — pin against the *landed* L5/Sec1 capability machinery, not
> this banner.** Capabilities are already in the elaborator:
> `CapParam { name, effect }` + `cap_set`
> (`crates/ken-elaborator/src/effects/algebra.rs`) thread one `Cap E` per
> un-handled effect through an `EffectSig` (`effects/infer.rs`, `cap_params`);
> `DeclassifyCap { from, to }` with `is_valid` (`to ⊑ from` ∧ strict),
> `check_declassify`, and `check_declassify_in_delta`
> (`crates/ken-elaborator/src/ifc.rs`) is **already a monotone-downward,
> validity-checked, delta-audited capability** — the declassification special
> case of everything below. Sec2 **generalizes** this landed pattern to
> authority + attenuation; it does **not** re-implement it. Extends `36 §3`;
> ADR 0004 Decision 4.

## 1. No ambient authority

Ken has **no ambient authority**: there is no global `open`, no implicit
filesystem or network, no process-wide mutable singletons reachable from
anywhere. A computation can act on the world *only* with an authority it was
**explicitly given** (a capability) and *only* via an effect its type declares
(`../30-surface/36 §1`). A `view` with no effect row and no capability arguments
is, by its type, **inert** — it can compute, nothing else. This is the
structural precondition for every authority claim below.

**This is the L5 capability-passing translation (`36 §2.5`), not a new gate.** A
function of row `ρ` elaborates to take one `Cap E` parameter per **un-handled**
effect `E ∈ ρ_open` as a leading argument; a `perform_E op` is well-formed
**only if `Cap E` is in scope**. So a world-action with **no** matching `Cap E`
parameter elaborates to a term that **references an unbound capability** — which
the kernel rejects as ill-typed *and* the elaborator catches earlier as the
nicer **missing-capability** diagnostic (`36 §7.3` error class 2). The
no-ambient guarantee is therefore **kernel-backed at its core** (the `perform`'s
denotation needs the `Cap E` value; the value is a real Π binding, not an erased
index) — the elaborator supplies only the source-located diagnostic, not the
soundness. A no-cap/no-row `view` denotes to `ITree 𝟘 ⟦B⟧ ≅ ⟦B⟧`
(`36 §2.4`): no `Vis` node is constructible, so it is provably effect-free.

## 2. Capabilities are static, visible, and least

A **capability** is an unforgeable authority token a computation must hold to
perform the corresponding effect. Capability types are authority-indexed as
`Cap a`; the landed source-facing instance is the filesystem family in
`38 §1.3.1`. Declassification has its separate label-edge index (`61 §4`). No
corresponding authority-indexed network family is yet admitted. Per
`36 §2.5`/`§3`:

- **Static + visible.** A capability is part of a function's type (a `CapParam`
  in its `EffectSig`), so a function's signature *is* its authority manifest:
  you can read, per function, exactly what it is permitted to touch — authority
  is checked **statically**, not by a runtime-only gate.
- **Least by default.** Because authority is never ambient, the default
  authority of any function is **none**; it holds exactly the capabilities its
  callers pass (`cap_params`, `36 §2.5`). The principle of least authority
  (PoLA) is the *path of least resistance*, not a discipline to remember.

### 2.1 The authority lattice (`Authority`)

A capability carries an **authority** — the *scope* of what it permits (for
example, filesystem paths, network hosts, a declassification edge, a quota, or
a validity window). The current source surface realizes the filesystem case as
`Cap a`; the other names used here remain semantic families, not source forms.
Authorities form a
**bounded lattice** `Authority` — the *same machinery* as the IFC label lattice
(`61 §2.1`), an **ordinary Ken value, not a kernel primitive**: a record of a
carrier plus `⊑`/`⊔`/`⊓`/`⊥`/`⊤` plus the lattice laws as `Ω`-valued
obligations (`../10-kernel/16 §1`), discharged once per instance.

```
authority : Cap E → Authority         -- the scope a capability confers; ⊑-comparable
⊥_auth = no authority (the least)      ⊤_auth = full authority for E (the most)
```

Data flows on the `61` lattice; **authority flows on this one**, and *more
authority is higher*: `a₁ ⊑ a₂` reads "`a₁` is **weaker than (or equal to)**
`a₂`." Attenuation moves **down** toward `⊥_auth` (§3). The concrete authority
lattice for each effect — which paths, which hosts, which edges — is a
**policy** supplied separately (`65`, ADR 0007), exactly as the IFC lattice's
instance is (`61 §2.2`): the discipline is **lattice-parametric**, the instance
is policy.

### 2.2 Unforgeability (the abstraction boundary)

`Cap E` is an **abstract (opaque) type**: user code has **no constructor** for
it. A capability value enters scope by exactly three privileged routes and no
other:

1. **Minted by a handler** — a handler is a capability provider (`36 §5`): it
   supplies `Cap E` to the body it interprets, at the authority the handler
   itself holds.
2. **Passed** — threaded as an ordinary Π parameter from a holder (`36 §2.5`).
3. **Attenuated by the trusted runner/host** — derived `⊑`-downward from a held
   capability and then supplied through an existing privileged route (§3).

Unforgeability is **load-bearing for §3**: monotone-downward attenuation guards
nothing if user code can *fabricate* a `⊤_auth` capability or invoke the raw
attenuation action itself. The opaque-type boundary (no public introduction
form; minting and attenuation confined to trusted handler/runner/runtime
machinery) makes the trusted attenuation route exclusive. This is an
**abstraction-boundary** property (§H) — the kernel rejects a user-side `Cap E`
construction because no constructor is in scope, while I-4 also requires the
raw names `attenuate` and `revoke` to be unbound in Ken (`38 §1.3.1`). Which
code is privileged to mint or derive a capability is a runner/module
discipline, not a kernel rule.

## 3. Attenuation — hand a child a strictly weaker token (the headline)

A trusted runner/host can derive a **weaker** capability from one already held —
**attenuation** — and **never a stronger one**. Semantically, attenuating parent
`c` by bound `w` produces a child `c'` only if `authority c' ⊑ authority c ⊓ w`.
This relation is **not** a Ken declaration or callable signature.

- Attenuation **narrows**: a smaller scope (one directory, not the filesystem;
  one host, not the network), a lower clearance, a tighter quota, a shorter
  validity window. The result authority is bounded by **both** the parent's
  authority **and** `w` — their meet `authority c ⊓ w` (`⊓-glb`, `61 §2.1`).
- A child therefore **cannot exceed** the authority its parent delegated, *by
  construction*. "This AI-generated helper must not reach the network beyond
  `api.example.com`" becomes a **compile-time fact**, not a code-review hope.

### 3.1 The encoding — a kernel-re-checked refinement obligation

The child exposed through an existing privileged capability path carries the
**landed refinement** (`34 §5`, `21 §2`): the **carrier `Cap E`** with predicate
`authority c' ⊑ authority c ⊓ w`. Supplying the child emits the obligation
`authority c' ⊑ authority c ⊓ w` (`22 §2.1`), discharged by the prover and
**re-checked by the kernel** (`23 §1`, `18 §4`). The trusted action establishes
`authority c' = authority c ⊓ w`, so the obligation is `(authority c ⊓ w) ⊑
(authority c ⊓ w)` — discharged by `⊑-refl`. This does not introduce a public
capability-producing wrapper (§4, `38 §1.3.1`).

**Kernel-backed *when the authority is kernel-visible* — not a uniform claim
(the honest split).** A capability is a **real Π value** (`36 §2.5`), when its
`Authority` ranges over **kernel-visible values** — FS paths, `Net` hosts, a
quota, a window (common case) — `authority c'`/`⊑` are **real terms** and the
bound is a real `Ω` obligation the kernel **certifies**, a genuine difference
from Sec1's flow rules (which are *trusted*: IFC labels are erased, conformance
the sole net, `61 §H`/§9 N1). **The exception: the declassify capability**
`Cap_declassify[ℓ→ℓ']` (`61 §4`): its authority **is an IFC label edge**, and
IFC labels are **erased before the kernel** (`61 §3`/§9 N1), so *its* monotone
bound (`ℓ' ⊑ ℓ`) is **trusted-by-typing** — exactly the landed
elaborator check `DeclassifyCap.is_valid` (`ifc.rs`), **not** a kernel
obligation. The rule:
**real-value authority → kernel-backed obligation; label-mediated authority
(declassify) → trusted-by-typing**, mirroring `61 §H`. Filing a label-mediated
guarantee as kernel-certified over-claims (the `61 §9 N1` erasure boundary);
the safe, accurate split is the one above.

**Committed use-site dual; not a current source form.** A world-action that
needs *at least* authority `a` would declare its capability parameter refined:

```text
required capability: { c : Cap(E) | a ⊑ authority(c) }
```

This relation is retained as target metanotation, not as present Ken. Current
source-facing filesystem sinks instead require an exact
`Cap AFull` (`38 §1.3.1`); they do not quantify a lower bound and do not emit
`a ⊑ authority c` at a call. Once bounded authority quantification and a real
sufficiency-demanding sink are admitted, calling such a sink with `c''` must
emit the kernel-re-checked obligation `a ⊑ authority c''`. A capability is then
sufficient exactly when its authority is `⊒` the demand. This is the pinned
encoding for the committed use-site dual, not a claim that v1 instantiates it.

### 3.2 No amplification — assert the absence, and net the orientation

**There is no Ken operation that amplifies or attenuates authority.** No
`strengthen`, `amplify`, `attenuate`, `revoke`, or public `Cap` constructor or
producer is bound in the Ken environment (§2.2, §4, `38 §1.3.1`). The trusted
runner/host's only derivation action is the raw, `⊑`-bounded attenuation
relation above. **Soundness of "downward-only" is the conjunction of three
facts**: (a) the attenuation bound (§3.1, kernel-backed when the authority is
kernel-visible); (b) the **enumerated absence** of any source operation that can
produce or alter a capability; and (c) **unforgeability** (§2.2).

**The order-dual soundness net (`[Sec1-dual]` trap-class).** `⊑` on `Authority`
is a **direction** — getting it **backwards** (writing the bound or the
sufficiency check as `⊒`) **silently inverts** attenuation-weakens into
attenuation-strengthens, exactly the taint-axis orientation hazard of Sec1's
integrity / `@ct` axes (`61 §5a.1`/§H). And the kernel obligation **alone does
not net it**: the canonical witness `authority c' = authority c ⊓ w` discharges
**both** `authority c' ⊑ authority c ⊓ w` **and** reversed `authority c ⊓ w ⊑
authority c'` by `⊑-refl` — the bound is **direction-degenerate at meet**, so
a backwards rule still type-checks. The orientation is held **only** by a
**non-degenerate distinguishing pair** (conformance AC3) on **strict**
authorities (`authority c ⊓ w ⊏ authority c`):

- **weaker cap at a weak sink — ACCEPTS** (`authority c ⊓ w ⊑ authority c'` =
  refl): the child's reduced demand is met. *(Necessary, but degenerate alone —
  green under both orientations.)*
- **weaker cap at a sink demanding the parent's full authority — REJECTS**
  (`authority c ⋢ authority c'`, strict): the weakened cap is **insufficient**.
  *This is the net:* under a backwards `⊑` it would **wrongly accept** (a
  weakened cap passing a strong sink — privilege escalation). The pair flips
  green↔red on exactly the orientation bug; a single accept case cannot.

This same-cap/two-sink formulation is a **metatheoretic distinguishing
requirement**, not a pair of current Ken operations. It must not be respelled as
two `writeFile` calls: that source-facing gate accepts exactly `Cap AFull`, so
both arms would reject at the capability type before comparing demands.

The orientation nevertheless has an executing runtime net. The real FS read
driver calls the production authority gate with demand `APartial`. Two
READ-bearing witnesses drive that gate over the same existing
`three-lines.txt`: `fs_driver_build_capability_acceptance.rs`'s
`r1_sufficient_cap_reads_fixture` supplies `APartial`, while
`fs_driver_build_acceptance.rs`'s `positive_read_returns_exact_fixture_bytes`
supplies `AFull`; both accept. Reversing the check from
`required ⊑ available` to `available ⊑ required` preserves the equal
`APartial` arm but wrongly rejects the `AFull` arm (`AFull ⋢ APartial`). Thus
the headline bound remains kernel-backed where its authority is kernel-visible
and its orientation is **trusted but conformance-netted** by an executing sink
pair; the block in §7 remains the canonical metatheoretic explanation of the
hazard.

## 4. Revocation — transitive, fail-closed, and bounded at runtime

Authority is **revocable**: a delegated capability can be withdrawn, and
**everything attenuated from it** is withdrawn with it. The trusted runner/host
maintains a non-Ken-visible revocation identity for each grant. Copying a
capability preserves that identity. The raw management action `attenuate`
creates a child identity linked to its parent; raw `revoke` closes the selected
identity and every descendant, to any depth, but not its parent or siblings.
A resource acquired under that authority, and any duplicate of that resource,
remains governed by the same lineage unless a future explicit reauthorization
establishes a different sponsor. Consuming a resource token therefore cannot
bypass revocation.

These raw actions are **management semantics, not Ken terms**. Neither
`attenuate` nor `revoke` is a Ken global, capability constructor, effect
producer, or capability-producing wrapper. They are absent from the Ken name
environment, as is every public `Cap` constructor or producer. Existing
`ProgramCaps`, `readFile`, and `writeFile` remain the source-facing capability
path (`38 §1.3.1`). A program observes revocation only when a later, existing
capability-consuming operation is denied; it cannot invoke or directly inspect
either management action or the identity they govern.

### 4.1 One denial identity, two public projections

Revocation has one semantic denial identity with two exact, type-local public
projections. The existing result families remain distinct:

- a path/capability operation returns
  `MkFileError <operation> <path> Revoked`, where `Revoked` is a new `IOError`
  cause beside and distinct from `CapabilityDenied`;
- a resource-token operation returns the nullary constructor
  `ResourceError.Revoked`, distinct from `Closed`, `MalformedResource`,
  `RightNotHeld`, `ResourceKindMismatch`, and `ResourceHostIO _`.

The runtime/host mapping preserves this correspondence. It must not collapse
either projection into `CapabilityDenied`,
`ResourceHostIO CapabilityDenied`, `Closed`, malformed capability/resource,
stale-generation, `RightNotHeld`, or a host I/O error. The discriminator applies
when revocation is the reason an otherwise well-formed, live,
sufficiently-righted operation is refused; this supplies a non-degenerate
control for each neighbouring denial without choosing precedence for an input
that is invalid in several independent ways.

### 4.2 Admission and settlement

**Admission is the linearization point.** Admission succeeds only while the
addressed identity and every ancestor are live. It separates exactly two
observable outcomes:

- **revoke before admission:** the capability-consuming operation returns the
  appropriate `Revoked` projection from §4.1, and no guarded OS backend
  operation occurs;
- **admission before revoke:** the admitted operation may finish and returns its
  real result, whether success or its actual non-revocation error. A later
  revoke does not rewrite that result to `Revoked`, and a side effect may already
  have committed.

Revocation promises **neither rollback nor cancellation**. Cancellation is a
separate operation and cannot be inferred from revocation. Revocation closes
new admissions immediately, while an already-admitted operation settles
normally. An owned OS resource is settled only after all such operations finish;
its close success or `ReleaseFailed` outcome remains recorded exactly once under
ADR 0021's resource identity and settlement discipline. Settlement failure does
not reopen authority.

### 4.3 Honest runtime boundary

This section closes only the **current OS-operation runtime face** for Ken's
implicit root execution space. It does **not** claim general runtime realization
of surface `space` (`36 §4`), separate runtime spaces, cross-space forwarders,
transport, cross-space or distributed revocation, or distributed isolation.
`44 §3` already realizes the memory/reclamation projection of each surface
`space` as a store `Space`; this contract neither denies that realization nor
claims its missing state/effect, authority, isolation, or transport projections
are delivered.

The runtime representation and isolation argument remain a `40-runtime` ADR
choice. A controlling space cell, forwarder, validity index, or region lifetime
is not normative here. Whatever mechanism is chosen must preserve the lineage,
descendant closure, admission boundary, two `Revoked` projections, and
settlement observations above.

## 5. Audit at trust boundaries — statically known

Authority exercised across a trust boundary is **auditable**, and the audit
points are **static**:

- A trust boundary — a `space` edge, FFI (`38 §3`), a **declassification**
  (`61 §4`), a capability **delegation** — is **exactly** a `Vis` node the
  function's type declares (`36 §3.1`: every authority-relevant act is a `Vis`
  node; nothing effectful hides between nodes). So the set of audit points is
  **recoverable from the type**, and an **un-audited boundary effect is
  impossible**: you cannot perform an effect the row did not declare (`36 §1.4`
  escape check), and a no-row `view` is inert (§1). The **statically-known**
  property is therefore kernel-backed by the row discipline; what each record
  *contains* — *what* authority, *by whom*, *what* effect — is the audit-record
  shape this chapter fixes.
- **Declassification (`61 §4`) is a capability whose use is audited.** Each
  `declassify` is a recorded event at a trust boundary, and the declassification
  authority a dependency holds appears in its **`trusted_base_delta`** (`63`,
  `25 §3`) — the landed `check_declassify_in_delta` (`ifc.rs`) is exactly this
  check. A package that downgrades secrets **cannot hide it**.

**Static face vs runtime face (the same split as §4).** Sec2 delivers the
*static* audit surface — the boundary set is type-determined, and declassify's
every-use audit point is a static site. The *runtime emission* of records
(serialization, tamper-evidence / append-only log) is a runtime/`Ward` concern,
`(oracle)`-tagged here — named, not absorbed.

## 6. Relationship to effects and flow — authority + flow compose

Authority and flow must compose: a capability **gates an effect**, and the sink
that capability opens **carries a clearance label** (`61 §3`, data may flow
only `⊑` that clearance). The landed operation-level instance is the
authority-indexed filesystem surface in `38 §1.3.1`. The following network
arrow states the committed composition rule but is deferred target
metanotation: current Ken has neither an authority-indexed network capability
nor `send`.

```text
send(capability for Net authority a, socket at κ, message at ℓ)
  is admitted only if the capability is present and ℓ ⊔ pc ⊑ κ
```

When that surface exists, **both** concessions are required and independent:
dropping the capability is a missing-capability error (§1); dropping the flow
check is an `IFC-FLOW` error (`61 §3.1`). Authority does not buy clearance, and
clearance does not buy authority. The security requirement is committed now;
the network operation-level instance is the named `AUTH-NET-SURFACE` gap.

## 7. Worked examples

### 7.1 Surface census

This is the durable D0 census for the examples below and the two normative
sites that share their assumptions. It separates four independent axes:
definition keyword, authority index or quantification, operation, and
effect-indexed result shape. A target spelling in this table is not an
authorization to present it as current Ken.

Three classes govern the non-current rows:

- `AUTH-NET-SURFACE`: no authority-indexed network capability, operation, or
  result surface is admitted. It exits when a Net surface lands with the
  authority-indexed capability/operation/result association already used by
  the filesystem surface in `38 §1.3.1`.
- `AUTH-BOUNDED-SINK`: v1 has no bounded authority quantification and no live
  sink that emits `a ⊑ authority c`. It exits when both the quantification and
  a real sink obligation are admitted.
- `AUTH-AC3-ORIENTATION`: §7's same-cap/two-sink spelling is unavailable until
  a live operation admits those two non-degenerate demands. Its soundness
  purpose is already covered: the executing FS read pair accepts both
  `APartial` and `AFull` under the correct order, while a reversed `⊑` rejects
  only the `AFull` arm, so the orientation ledger remains conformance-netted.

| Site or example | Definition keyword | Authority index or quantification | Operation | Effect-indexed result | Standing |
|---|---|---|---|---|---|
| §3.1 target signature (issue anchor `:173`) | `proc` target | bounded `{c : Cap a \| demand ⊑ authority c}` unavailable | sufficiency-demanding FS sink unavailable | must use the sink's `FS a (Result …)` association | `AUTH-BOUNDED-SINK`; design-committed, not instantiated |
| §6 network arrow (issue anchor `:337`) | `proc` target | authority-indexed Net cap unavailable | `send` unavailable | no Net result association is admitted | `AUTH-NET-SURFACE`; deferred `(oracle)` |
| no-ambient family: `classify`, `save`, `save_bad` | `fn` for pure classify; `proc` for both saves | none for classify; `Cap AFull` for save; save_bad omits it | none for classify; source-facing `writeFile` for both saves | pure `Tag`; both saves use `FS AFull (Result FileError Unit)` | current positive and deliberate no-cap negative below |
| `sandbox` | `proc` target | lower-bounded child quantification unavailable | real lower-bound sink unavailable | would follow the admitted FS sink association | `AUTH-BOUNDED-SINK`; metatheoretic target |
| AC3 order-dual pair | no standalone definition | same attenuated cap under two demands unavailable | two real sinks unavailable | accept/reject observations, not a source result annotation | `AUTH-AC3-ORIENTATION`; metatheoretic form, orientation net live |
| three management negatives | no definition | held capability only | names remain deliberately unbound | none | current negative contract |
| `use_child` | `proc` | `Cap APartial` | current `readFile APartial c_child path` | `FS APartial (Result FileError Bytes)` | current source spelling below |
| `exfil` | `proc` target | authority-indexed Net cap unavailable | `send` unavailable | no Net result association is admitted | `AUTH-NET-SURFACE`; deferred `(oracle)` |

The current filesystem shapes below follow the checked exemplar at
`catalog/packages/Capability/Filesystem/Authority.ken.md`, while staying at
the source-facing `readFile`/`writeFile` tier fixed by `38 §1.3.1`.

```ken
-- No ambient authority: a no-cap/no-row fn is inert by its type.
fn classify (x : Record) : Tag = …

-- A world-action REQUIRES the capability + the declared effect.
proc save
      (c : Cap AFull) (p : Bytes) (policy : CreatePolicy) (d : Bytes)
    : FS AFull (Result FileError Unit)
    visits [FS] =
  writeFile c p policy d

proc save_bad
      (p : Bytes) (policy : CreatePolicy) (d : Bytes)
    : FS AFull (Result FileError Unit)
    visits [FS] =
  writeFile missing_capability p policy d
  -- REJECTED: missing_capability is unbound; no FS capability is ambient.
```

The `sandbox` example is a metatheoretic `AUTH-BOUNDED-SINK` target, not Ken
source. A trusted host starts with `c : Cap(A_parent)` and supplies
`c_tmp : Cap(A_tmp)` such that
`A_tmp ⊑ A_parent ⊓ authority-for("/tmp")`; the child can reach exactly the
delegated `/tmp` scope. Ken cannot yet state the required lower-bounded
capability parameter because `38 §1.3.1` expressly leaves bounded authority
quantification out of v1. This preserves the host-side reading settled for the
chapter without fabricating a source spelling.

The AC3 order-dual pair is likewise **metatheoretic**, never a respelled read or
write program:

```text
same attenuated c_tmp at sink demanding /tmp       -> ACCEPTS
same attenuated c_tmp at sink demanding parent     -> REJECTS
```

The second verdict must flip to acceptance if `⊑` is implemented backwards;
that is the non-degenerate orientation discriminator. Landed `writeFile` takes
exactly `Cap AFull`, so spelling these as two writes would reject both at the
capability type before either demand and would destroy the discriminator.

The negative management examples remain current name-resolution claims:

```ken
-- No public production or management action: all three names are absent.
attenuate c AFull   -- REJECTED: UnboundName (I-4)
revoke c            -- REJECTED: UnboundName (§4)
strengthen c AFull  -- REJECTED: UnboundName (§3.2)
```

Revocation is observed through the current source-facing filesystem API:

```ken
-- If the trusted host revoked c_child or an ancestor before admission, the
-- call returns MkFileError ReadFile path Revoked and performs no backend read.
proc use_child (c_child : Cap APartial) (path : Bytes)
    : FS APartial (Result FileError Bytes)
    visits [FS] =
  readFile APartial c_child path
```

Finally, `exfil` is a deferred `(oracle)` `AUTH-NET-SURFACE` example, not Ken
source. It requires an authority-indexed Net capability and a Net sink at
`Public`; sending `Bytes @ Secret` must be rejected by `61`'s `L-SINK` even
when the capability is present. Omitting either the capability concession or
the flow concession must reject independently. This retains AC6 without
inventing the absent Net operation or result surface.

A CISO reads these and sees no-ambient confinement, least authority,
non-amplifiable delegation, and audited boundaries in the typed surface, plus
transitive, fail-visible revocation at the runtime boundary. The static controls
over the landed filesystem surface are enforced by construction. The bounded
sink and Net controls remain committed requirements with the explicit gaps
above; §4 names the separate runtime-trusted guarantee without presenting it as
a kernel theorem.

## H. Honest limits — kernel-backed vs trusted vs deferred

Per `64 §4`: **a verified language that over-claims is itself a security risk.**
Ken states its authority boundaries exactly. **None of this enlarges the trusted
kernel** — capabilities are ordinary Π (`36 §2.5`), the authority order is
an ordinary lattice (§2.1), and real-value attenuation bounds are `21 §2`
obligations re-checked by the *same* small kernel (the declassify-edge bound
excepted — it is over erased labels, §3.1) (ADR 0004 Decision 3, ADR 0001).

| Aspect | Status | Detail |
|---|---|---|
| No ambient authority — a `perform_E` needs `Cap E` in scope | **kernel-backed** | the cap is a real Π parameter (`36 §2.5`); a world-action with no matching cap denotes to an unbound reference the kernel rejects (§1). The elaborator adds only the source-located **missing-capability** diagnostic |
| Least by default — a function holds exactly the caps it is passed | **kernel-backed** | same mechanism — using an un-passed capability is an unbound reference; default authority is `∅` |
| Attenuation **monotone bound** `authority c' ⊑ authority c ⊓ w` (real-value authority) | **kernel-backed (refinement obligation) — but direction-degenerate** | a `34 §5`/`21 §2` obligation, kernel-re-checked (§3.1) — *stronger* than Sec1's erased flow rules. **Yet** the meet-witness discharges both `⊑` orientations by refl, so the orientation remains trusted. The production FS read gate's executing `APartial`/`AFull` accept pair becomes accept/reject under a reversed check and conformance-nets that orientation (§3.2) |
| Attenuation bound of the **declassify** cap `ℓ' ⊑ ℓ` | **trusted-by-typing** | its authority is an **IFC label edge** and labels are erased before the kernel (`61 §3`/§9 N1), so the bound is the landed elaborator check `DeclassifyCap.is_valid`, **not** a kernel obligation — exactly Sec1's erased-label posture (§3.1) |
| Use-site **sufficiency** `a ⊑ authority c` | **design-committed; not instantiated** | the pinned target refines a sink's cap parameter `{c | a ⊑ authority c}`, but current sinks take exact `Cap AFull`; v1 has no bounded authority quantification and no landed call emits this obligation (`AUTH-BOUNDED-SINK`, §3.1) |
| **No amplification / source attenuation** | **trusted by enumerated absence** | no `strengthen`/`amplify`/`attenuate`/`revoke` or public `Cap` constructor/producer exists — there is nothing to call; conformance asserts the positive wrappers plus this complete absence (§3.2, §4), which the kernel cannot witness |
| **Unforgeability** of `Cap E` | **abstraction-boundary** | `Cap E` is opaque; minting and raw management are confined to trusted handler/runner/runtime machinery. The kernel rejects a user-side construction (no constructor in scope); I-4 separately nets the absence of every producer/management name (§2.2) |
| Revocation **lineage + bounded OS-operation behavior** | **runtime-trusted contract; mechanism deferred** | raw management is not Ken-callable; descendant closure, admission linearization, the two distinct `Revoked` projections, and settlement are normative for the current implicit root space (§4) |
| Revocation **mechanism + general space realization** | **deferred → `40-runtime` / `OQ-Space`** | representation and isolation argument are ADR-owned; no general multi-space, cross-space, transport, or distributed claim (§4.3) |
| Audit points **statically known** | **kernel-backed** | the boundary set = the `Vis` nodes the type declares; an un-audited declared effect is impossible (`36 §1.4`, §5) |
| Audit-record **emission** (log, tamper-evidence) | **deferred → runtime / `Ward`** | the static surface is fixed; runtime serialization is `(oracle)`-tagged (§5) |
| Authority + flow **compose** | **committed; Net operation instance deferred** | capability and flow concessions remain independent requirements. The operation-level capability surface currently lands only for FS (`38 §1.3.1`); the Net form must follow that authority-indexed capability/operation/result association (`AUTH-NET-SURFACE`, §6) |
| The **policy** (which paths/hosts/edges an authority lattice has) | **assumed** | a wrong policy ⇒ a wrong guarantee — the `64 §4.1` spec≠intent analog; the policy (`65`) is the human-reviewed boundary, exactly as for IFC (`61 §H`) |

**The Sec2 vs Sec1 contrast (worth stating, the design payoff).** Sec1's IFC
labels are **erased** before the kernel, so its flow rules are *trusted* and the
conformance corpus is the **sole** net (`61 §9 N1`). Sec2's capabilities are
**real values**, so an instantiated attenuation *bound* over kernel-visible
authority is a kernel-backed obligation. The use-site sufficiency encoding is
committed but not yet instantiated, and therefore supplies no current
kernel-backed guarantee. What remains trusted or deferred is **narrower and
named**: the *orientation* of `⊑` (degenerate at meet, conformance-netted by the
executing FS read pair),
the **absence**
of amplification, **unforgeability** (abstraction boundary), the **declassify**
cap's bound (over erased labels → trusted-by-typing, the one Sec1-style
exception, §3.1), §4's runtime-trusted revocation contract and deferred
mechanism, and runtime audit emission (`40-runtime`/`Ward`).

## 8. What is committed vs. open

- **Committed:** no ambient authority (§1); static + visible capabilities, least
  by default (§2); **attenuation** monotone-downward with a **kernel-re-checked
  bound** (§3); **no public producing, attenuating, revoking, or amplifying
  operation** (§3.2/§4); transitive revocation plus the bounded OS-operation
  contract for the implicit root space (§4); statically-known **boundary audit**
  (§5); capabilities **gate effects and compose with clearance** (§6).
- **Decided (`OQ-8a`):** capabilities are first-class tokens, handler-or-row
  supplied, attenuable/revocable/audited, distinct from logical `requires`
  (`36 §3`).
- **Decided (`OQ-Space`):** shared-nothing message-passing over encapsulated,
  non-aliased `space` cells (`36 §4`). Section 4 pins the current implicit root
  space's OS-operation revocation behavior without choosing a runtime
  representation. General multi-space/cross-space realization remains deferred
  to `40-runtime`. The *security requirement* — attenuable, revocable, audited,
  least — is fixed regardless of runtime construct form.
- **Deferred (named, `(oracle)`-tagged):** the implementation and isolation
  argument for §4's bounded contract, general space realization, and runtime
  audit-record **emission** (§5) — `40-runtime`/`Ward`, riding the contracts this
  chapter pins.
- **Deferred (`AUTH-NET-SURFACE`, `(oracle)`):** the authority-indexed Net
  capability, operation, and result surface. It must follow the landed FS
  association in `38 §1.3.1`; the gap exits when that Net surface exists.
- **Deferred (`AUTH-BOUNDED-SINK`, `(oracle)`):** the pinned use-site predicate
  `a ⊑ authority c`. The gap exits when bounded authority quantification exists
  and a real sink uses it to emit the obligation.

## 9. What Team Verify must deliver here (Sec2)

The Sec2 deliverable is the elaboration below, made impl-ready. Each item is a
concrete, codeable section; an implementer builds from these and the kernel
re-checks the emitted core (the elaborator is **not** in the TCB, `36 §7`).
Section 4's ABI-REVOKE runtime additions remain Runtime-owned; recording them
here does not reassign their implementation to Team Verify:

1. **No-ambient enforcement** — the capability-passing translation gate (§1,
   building on landed `CapParam`/`cap_set`): a world-action requires its `Cap E`
   parameter + declared row; a no-cap/no-row `view` is inert. *AC1.*
2. **Capability tokens in the type** — `Cap E` as an opaque token, the
   signature-as-manifest, least-by-default (§2), and the `Authority` lattice +
   `⊑` order as a `61 §2` lattice value (§2.1). *AC2.*
3. **Attenuation** — the trusted raw derivation supplies a child satisfying
   `{c' | authority c' ⊑ authority c ⊓ w}` as a kernel-re-checked refinement
   obligation (§3.1), while the Ken environment exposes no capability producer,
   `attenuate`, `revoke`, or amplifying operation (§3.2/§4). *AC3 — the
   headline.*
4. **Revocation split** — Sec2 supplies the non-Ken-visible raw-management
   boundary and lineage contract. ABI-REVOKE supplies the Runtime-owned two
   public `Revoked` projections, admission linearization, and settlement for the
   bounded implicit-root OS-operation face (§4). The mechanism, ADR isolation
   argument, and general space realization remain `40-runtime` work. *AC4.*
5. **Audit points** — the static boundary set + the audit-record shape;
   declassification every-use-audited and in `trusted_base_delta` (§5). *AC5.*
6. **Authority + flow composition** — a capability gates an effect **and** the
   sink carries a clearance label; dropping either rejects (§6). *AC6.*

### Level reconciliation (the soundness check — before the Architect handoff)

The authority constructs add **no new level rule** — only instances of existing
formation (`36 §7.4`, `61 §9`):

| Construct | Level | Rule |
|---|---|---|
| `Cap E` (capability token) | `Type ℓ_op` | a value type (`36 §2.5`); opaque (§2.2), no new former |
| `Authority` (carrier + ops record) | `Type (suc ℓ)` | record / Σ-Form (`13 §2`), laws at `Ω` — a `61 §2.1` lattice value |
| `authority : Cap E → Authority` | ordinary Π | a projection; no new rule |
| `{ c' : Cap E \| authority c' ⊑ authority c ⊓ w }` | `level(Cap E) = ℓ_op` | refinement = carrier + obligation (`21 §2`, `34 §5`); **predicative** (`12 §2`), **non-cumulative** (`12 §3`), same level as the carrier — adds no Σ over `Ω` |
| `authority c' ⊑ authority c ⊓ w` | `Ω` | an ordinary `Ω`-valued obligation (`22 §1`, `16 §1`) |
| raw attenuation/revocation identity | non-Ken-visible runtime state | no source former, global, constructor, producer, or new kernel rule (§4) |

Every level is the **predicative `max`** of its parts (`12 §2`), non-cumulative
(`12 §3`); the elaborator emits explicit levels and the kernel re-checks them
(`12 §4`). The authority discipline is impredicative nowhere — it reuses
Π/Σ/inductive/refinement, adding **no new level rule**.

Conformance: `../../conformance/security/capabilities/` — AC1–AC6 with
**discriminating** cases (COORDINATION §7; every live negative case flips on
the bug it targets). AC3 retains the metatheoretic order-dual requirement:
weaker-accepts / stronger-rejects on the same cap shape at non-degenerate live
sinks, never a synthetic flag. That illustrative form remains metatheoretic.
The production FS read gate supplies the live net. The independent executing
witnesses named in §3.2 accept `APartial` and `AFull` for the same read and
existing path; a reversed `⊑` preserves the equal `APartial` arm but rejects
the `AFull` arm.
