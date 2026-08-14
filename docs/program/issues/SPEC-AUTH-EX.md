---
id: SPEC-AUTH-EX
title: "62-authority section 7 is the spec's only worked example of the authority discipline and almost none of it elaborates -- four stale axes rather than the three recorded, the fourth being a RESULT TYPE that changed shape, and two examples that cannot be respelled at all because v1 lacks the quantification one needs and moved the write boundary the other turns on"
status: merged
owner: spec-enclave
size: S
gate: none
depends_on: [SPEC-IDENT-BLESSED, CAT-CAPEX]
blocks: []
github: null
origin: "Measured by the Steward 2026-07-27 at origin/main e700b861 while discharging the CAT-CAPEX ordering question; not routed by any ring. Re-measured and FRAMED by the Steward 2026-08-14 at main a12f37b7, at which point both recorded holds were discharged -- SPEC-IDENT-BLESSED merged, and CAT-CAPEX (the exemplar this node's own scope note said should land first) merged. The re-measurement found a fourth stale axis the original census missed and corrected the size of the other three."
---

> # ARCHITECT RULING `evt_3eqh0vxc5v95p` -- THE HOLD IS LIFTED. READ THIS FIRST.
>
> **`62` already owns the mechanism. Do not build a new one.** The answer turns
> on `§H` and `§8`, which `D0` did not cite. He read the chapter at `ed65aa40`
> directly rather than work from the census.
>
> **The standing:** `62`'s *security requirements* stay committed and
> chapter-wide; its *operation-level* claims narrow to the FS axis, the only
> landed one; every Net-axis and bounded-quantification claim moves out of the
> present tense into a `§H` row plus a `§8 Deferred (named, `(oracle)`-tagged)`
> entry. **Nothing is deleted.** `§H` is already a graded status ledger and `§8`
> already separates Committed / Decided / Deferred.
>
> The chapter's own charter decides it -- `§H` opens by quoting `64 §4`,
> *"a verified language that over-claims is itself a security risk."*
> **Present-tense Net material is an over-claim by the chapter's own standard,
> in a security chapter, about security properties.**
>
> ## Record the CLASS per site. A flat unexpressible-sites table is wrong.
>
> **The three classes have different exit conditions**, and a flat list loses
> them so the next attempt re-derives them. Tag each site with its class and
> its unblocking condition.
>
> | class | sites | exit condition |
> |---|---|---|
> | **1. Net -- no landed surface at all** | `send`, `Cap_Net`, `:337`, `AC6 exfil` | a Net surface exists. The `§H` row **must name `38 §1.3.1`'s FS shape as the target**, so Net lands authority-indexed rather than re-litigated. Requirement stays committed -- precedent is `§8 OQ-Space`'s *"the security requirement is fixed regardless of runtime construct form"* |
> | **2. the use-site bound `a ⊑ authority c`** | `:173`, `§3.1` | bounded authority quantification exists |
> | **3. the `AC3` order-dual pair** | `§3.2`, `§7` | whichever of 1 or 2 first admits a distinguishing sink pair |
>
> ## `§H` IS CURRENTLY WRONG ABOUT THE LANDED SURFACE, and this is the big one
>
> **`§H` grades use-site sufficiency as *kernel-backed (refinement
> obligation)*** -- *"a sink refines its cap parameter `{c | a ⊑ authority c}`;
> each call emits the obligation."* **Landed sinks take an exact `Cap AFull`
> and `38 §1.3.1` provides no bounded authority quantification, so NO LANDED
> SINK EMITS THAT OBLIGATION.**
>
> ⇒ Re-grade that row to design-committed-but-not-instantiated, keeping the
> encoding as the pinned target. **This is not a currency nit: it is the
> chapter claiming a kernel-backed guarantee it does not currently have.**
>
> ## `AC3` -- retain as explicitly metatheoretic. DO NOT respell it.
>
> **This supersedes the read-path option this frame offered below; the
> Architect ruled it out and his reason is stronger than the one I gave.**
> `§3.2` establishes the kernel obligation is **direction-degenerate at meet**
> -- the canonical witness discharges both orientations by `⊑-refl`, so a
> backwards `⊑` still type-checks. **The pair is the sole net over a named
> soundness gap**, privilege escalation via inverted attenuation.
>
> Under a `writeFile` respell both halves reject on capability *type* before
> either sink comparison is reached, so **the pair goes red-red and stops
> distinguishing the orientation at all** -- while `§H` goes on claiming it is
> netted. **A conformance pair that is red for an unrelated reason is worse
> than an absent one, because the ledger still reads as covered.**
>
> ## ONE GREP IS REQUIRED BEFORE THE CENSUS LANDS -- it changes a disposition
>
> `§H` says the orientation is *"netted by the non-degenerate conformance pair
> (§3.2)"*. **Does the capabilities conformance corpus contain an EXECUTING
> pair that distinguishes the two `⊑` orientations at a sink, given landed
> sinks take exact `Cap AFull`?**
>
> - **yes** -- the row stands; mark `§7`'s block metatheoretic and move on.
> - **no** -- `§H` is claiming a live net that does not run, and the row
>   re-grades to *trusted / not currently netted*, as a named gap.
>
> **He explicitly declined to rule this blind. Do not assume either branch.**
>
> ## The durable principle
>
> **A spec chapter may be ahead of the implementation, but it must not be ahead
> of its own honesty ledger.** Present tense is reserved for what the landed
> surface provides; everything else is graded in `§H` and listed in `§8`.
> **`62` is not drifting -- it is UNGRADED, and grading is a mechanism it
> already has.**
>
> **`D1`-`D4` are unheld into this shape.** `AC-1`'s correction stands:
> `readFile`/`writeFile` are current and stay, including `:236` and `:386`.

> # RESCOPED 2026-08-14 AFTER `D0` HARD-STOPPED. Superseded in part by the
> # ruling above; kept because it records why the stop was correct.
>
> **`D0` fired stop condition 3 before any edit, which is the frame working.
> Two of the three reasons were defects in this frame, and both are fixed
> below.** Steward ruling `evt_4atyx2jgvmtae`, on spec-leader's stop
> `evt_43kamkngvr1qa`.
>
> **The four-respellable / two-unexpressible partition in this file is FALSE.**
> It measured the FS axis and generalised to the chapter. **`send` and
> `Cap_Net` appear nowhere in `catalog/` and nowhere in `38-ffi-io.md` -- they
> exist only inside `62` itself.** So `exfil` is a **third** unexpressible
> example, and **`:337` (`view send (c : Cap_Net) …`, `§6`) is a fifth
> unexpressible site this frame never enumerated.**
>
> ⇒ **The Net capability surface did not land in any form**, and the chapter's
> `§3.1` use-site bound `a ⊑ authority c` needs a quantification `38 §1.3.1`
> says v1 does not provide. **`62` is written in the present tense about a
> capability surface that only partly exists.**
>
> **A partial respell would make the chapter MORE internally inconsistent** --
> some examples current, some retired, and no marker telling a reader which.
>
> **The rescope:**
>
> 1. **`D0`'s census IS this node's durable deliverable.** Land the per-example
>    table across the four axes, plus every unexpressible site paired with the
>    normative sentence that refutes it: `:173`, `:337`, `sandbox`, the `AC3`
>    pair, `exfil`.
> 2. **`D1`-`D4` are HELD, not cancelled.**
> 3. **ONE question routes to the Architect** on the enclave's own edge, need
>    surfaced and vehicle left open: *what is `62`'s standing relative to the
>    landed surface?* **Not five questions, and no bundled mechanism** -- a
>    bundled mechanism anchors the owner and its rejection then reads as "the
>    need cannot be met".
>
> **Do not let the `AC3` pair quietly disappear while this is held.** `62 §3.2`
> names it as the net holding the order relation's orientation.

> ## BOTH RECORDED HOLDS ARE DISCHARGED. This node was `draft` for a reason
> ## that expired, and the expiry is why it is being released now.
>
> The file said *"the frame is not written and the enclave is building
> `SPEC-IDENT-BLESSED`"*, and its scope note recommended letting `CAT-CAPEX`
> land the checked exemplar **first** so that section 7 can cite something that
> provably elaborates instead of a code block nobody runs.
>
> **`SPEC-IDENT-BLESSED` is merged. `CAT-CAPEX` is merged.** The recommended
> sequencing is not merely unblocked -- it is now the available one, and `D3`
> below is exactly the move the old scope note reserved.
>
> `depends_on` records both, and both are `merged`, so this node is genuinely
> releasable rather than releasable-looking.

## What this is

**`spec/60-security/62-authority.md §7` ("Worked examples") is the spec's only
worked example of the authority discipline**, and the chapter doc 04 cites as
its authoritative prose artifact. Every code line in it is written in a surface
Ken no longer has.

**The examples' argument is sound and is not what this node changes.**
No-ambient confinement, least authority, non-amplifiable delegation, the
order-dual `AC3` pair, the `AC6` authority-plus-flow composition, and the three
`UnboundName` management names are all correct and all worth preserving.

## THE CORRECTION THAT SIZES THIS NODE: none of the axes is a rename

**The earlier census recorded three axes and presented them as spellings.**
Re-measured at `main` `a12f37b7`, there are **four**, and **not one of them is
a substitution.** This matters because the failure direction is the bad one: a
sweep that renames every occurrence produces code that is still wrong, and
wrong in ways that no longer look stale.

**First, get the LAYER right, because it decides every other answer.** The
landed surface has two tiers, pinned at `38 §1.3.1`:

- **the source-facing capability API** -- `readFile`, `writeFile`. `62`'s own
  line `236` names these as *"the source-facing capability path"*.
- **the raw authority-polymorphic producers** they wrap -- `read_bytes`,
  `write_file`. The `CAT-CAPEX` exemplar is written at **this** tier.

> **Section 7 is written at the SOURCE-FACING tier and must stay there.** It
> already says `readFile`. **Do not respell it toward the exemplar's tier** --
> copy the exemplar's *shape* (indexed `Cap`, `FS`-indexed result), not its
> operation names. A frame that said "`readFile` is stale, use `read_bytes`"
> would be measuring `catalog/` and inferring about the surface; `readFile` is
> current, and `38 §1.3.1` is its authority.

| axis | section 7 writes | landed source-facing surface | why a rename does not do it |
|---|---|---|---|
| definition keyword | `view` | `const` / `fn` / `proc` | `view` split by role. Effectful examples are `proc`; a pure classifier is `fn`. **Per-example judgment**, not a global replace |
| capability type | `Cap_FS` | `Cap : Auth -> Type0` | the type became **authority-indexed**. Every occurrence must choose an index (`Cap AFull`, `Cap APartial`) or quantify (`(a : Auth) (cap : Cap a)`). There is no index-free spelling to rename to |
| FS write | `write_at c p d` | `writeFile : Cap AFull -> Bytes -> CreatePolicy -> Bytes -> FS AFull (Result FileError Unit)` | three arguments to four; `Path` became `Bytes`; a `CreatePolicy` argument appeared; and the capability is pinned to **`AFull` specifically**, not polymorphic |
| result type | `: Result FileError Bytes` | `: FS a (Result FileError Bytes)` | **the SHAPE of the type moved, not the names in it.** No name-level check can see this axis, and section 7 gets it wrong even on the line whose operation is already current |

**`readFile` is NOT stale.** `readFile APartial c_child path` is correct in
name, arity, and index. **That single correct line is why the chapter reads as
current**, and it is the reason the axis list has to be per-example rather than
per-identifier.

> **The reason section 7 reads as current is that it is HALF-MIGRATED.** The
> `use_child` example says `readFile APartial c_child path` -- a current
> operation at a current authority index -- inside a `view` signature that
> takes `Cap_FS` and declares an un-indexed result. **A reader who
> spot-checks one identifier finds a current one.** Do not take any single
> current-looking name as evidence that its example is current.

## Fixed inputs, measured at `main` `a12f37b7`

Re-derive at your base and state the SHA you read.

**The stale chapter.** `spec/60-security/62-authority.md`, section 7 at line
`350`. Corpus counts in that file: `Cap_FS` **8**, `write_at` **5**,
`write_file` **0**.

**The landed exemplar, which is the copy source and the cite target.**
`catalog/packages/Capability/Filesystem/Authority.ken.md`, 54 lines, delivered
by [[CAT-CAPEX]]:

```ken
proc capability_read
      (a : Auth) (cap : Cap a) (path : Bytes)
    : FS a (Result FileError Bytes)
    visits [FS] =
  read_bytes a cap path

proc full_authority_write
      (cap : Cap AFull) (path : Bytes) (policy : CreatePolicy) (contents : Bytes)
    : FS AFull (Result FileError Unit)
    visits [FS] =
  write_file AFull cap path policy contents
```

**This file is checked.** That is the whole point of citing it: it is the one
capability surface in the corpus that provably elaborates.

## TWO EXAMPLES ARE NOT RESPELLABLE, AND ONE OF THEM IS THE CHAPTER'S OWN
## SOUNDNESS NET. This is the part that is not currency work.

**Both are refuted by a normative sentence elsewhere in the corpus, not by an
inference of mine. Cite the sentence, do not re-derive the judgment.**

### `sandbox` (`:366`) -- the refinement needs a quantification v1 does not have

```
    (c_tmp : { c' : Cap_FS | authority c' ⊑ authority c ⊓ only_dir "/tmp" })
```

**`only_dir` occurs exactly once in the entire `spec/` + `catalog/` corpus: on
that line.** Nothing defines it; the example invents it. **`authority` is not
Ken vocabulary either** -- `62 §3.2` says flatly *"There is no Ken operation
that amplifies or attenuates authority"*, and the `⊑`-bounded relation is the
trusted runner/host's raw derivation.

**And the decisive sentence is `38 §1.3.1`:** *"A static lower bound on `a`
would require bounded authority quantification, **which v1 does not provide**;
that is a v2 option rather than an implied v1 guarantee."* That is precisely
what this refinement type is. ⇒ **The example is not stale, it is
unexpressible**, and no respelling reaches it.

### the `AC3` order-dual pair (`:369`-`:373`) -- the write boundary MOVED

The pair writes the same attenuated `c_tmp` to two sinks and turns on the
verdict flipping **at the sink**. Under the landed surface it cannot: `38
§1.3.1` pins `writeFile : Cap AFull -> …` and states *"a program holding only
`Cap APartial` cannot apply `writeFile`, so the attempt is ill-typed before the
driver runs."*

⇒ **Both halves of the pair fail to typecheck at the capability, so neither
reaches the sink comparison the pair exists to demonstrate.** The dynamic
authority floor now lives on the **read** path -- *"the driver's
`CapabilityDenied` result remains defense in depth for writes and is the
primary authority floor for reads."*

**This one matters far beyond tidiness.** `62 §3.2` names this pair as **the
net** that holds the `⊑` orientation, because the kernel bound alone is
direction-degenerate at the meet. **A repair that quietly drops it deletes the
chapter's own argument for why a backwards `⊑` would be caught.**

### The fork, and it is not the ring's to take

Each example has the same two shapes available: **mark the block explicitly as
host-side or metatheoretic** and stop presenting it as Ken, or **re-express it
in the landed mechanism** -- the authority index for `sandbox` (`Cap AFull`
handed on as `Cap APartial`, which is what `CAT-CAPEX` demonstrates), and the
read path for `AC3`, where the dynamic floor actually is.

**Name both readings per example and take neither on your own authority.** The
`AC3` choice in particular changes what the chapter claims its soundness net
is, which is an Architect question.

## Deliverables

**`D0` -- enumerate every code line in section 7 against the four axes, before
changing any of them.** One row per example: which keyword it needs, which
authority index (or quantification), which operation name, and whether its
declared result type gains an `FS` index. **The enumeration is the deliverable
that makes the rest mechanical**, and it is where the half-migrated lines get
caught.

**`D1` -- respell every example that CAN be respelled**, against the landed
exemplar rather than against memory of the old surface. Preserve the argument
of each example exactly; this changes how the code is written and never what it
demonstrates.

**`D2` -- the two unexpressible examples: report the fork, do not resolve it.**
One paragraph each for `sandbox` and for the `AC3` pair, naming both readings
and which you would take. **`COORDINATION §6` governs**: if `38 §1.3.1` plus
`62 §3.2` already determine the answer, resolve it and cite what determined it
rather than escalating a settled question. **The `AC3` half is the one most
likely to be a genuine fork**, because taking either shape changes what the
chapter claims its orientation net is.

**`D3` -- section 7 cites the landed exemplar.** This is the move the node's
original scope note reserved for after `CAT-CAPEX`, and it is now available.
A worked example that points at a checked artifact is worth more than one that
merely parses, because the citation goes stale loudly.

**`D4` -- sweep the rest of the chapter, and note that one site is NORMATIVE.**
The counts above are file-wide, not section-7-wide. Measured: **5 of the 8
`Cap_FS` and 4 of the 5 `write_at` are inside section 7**, so the residue is
exactly four sites, all pinned here:

| line | what is there | disposition |
|---|---|---|
| `:68` | `Cap_FS`, `Cap_Net`, `Cap_declassify[l->l']` as an inline list of capability names | prose naming, respell |
| `:83` | *"paths for `Cap_FS`, a set of hosts for `Cap_Net`"* | prose naming, respell |
| `:173` | `view write_at (c : { c : Cap_FS \| a ⊑ authority c }) (p : Path) … visits [FS]` | **a retired-surface code line inside normative text**, carrying the retired keyword, the retired type, the retired operation, AND the same host-side `authority` predicate as the `sandbox` example |

**`:173` is the site to handle first and most carefully.** It is not in section
7, it is a full stale signature rather than a mention, and it sits in the
attenuation-bound material that `AC-6` forbids you to move. **If respelling it
would change what the bound asserts, that is stop condition 3, not an edit** --
and it is the likeliest place in this node for that to happen.

**A repair that leaves the chapter internally inconsistent has moved the defect
rather than closed it.**

## Acceptance criteria

**`AC-1` -- no RETIRED name survives in the chapter.** `write_at`, `Cap_FS`,
and every `view` used as a definition keyword are absent. **`readFile` and
`writeFile` are CURRENT and must REMAIN**, including the existing correct
occurrences at `:236` and `:386`. **State the post-change counts**, so the
claim is checkable rather than asserted.

> **Corrected 2026-08-14 after `D0` hard-stopped.** This AC previously listed
> `readFile` among the names required to be absent, contradicting the same
> frame's own axis table and `38 §1.3.1`. **The analysis was fixed and the
> enforcement was not** -- an AC states a rule in outcome language, so a
> vocabulary sweep of the prose does not reach it. Found by spec-leader at
> `evt_43kamkngvr1qa`.

**`AC-2` -- every respelled example is valid against the landed surface**, in
arity, in authority index, and in **result type shape**. The `FS`-indexed
result is the one that a name-level check cannot see and is therefore the one
to state explicitly.

**`AC-3` -- every example's argument survives, including the two that cannot
be respelled.** No-ambient confinement, least authority, non-amplifiable
delegation, the `AC3` order-dual pair, `AC6`, and the three `UnboundName` names
all still read. **A repair that deletes an example to avoid the problem has
failed this.** If an argument can only be kept as explicitly-labelled host-side
or metatheoretic text, keep it that way and say so -- **silently dropping the
`AC3` pair would delete the chapter's own argument for why a backwards `⊑`
gets caught**, which is a soundness-documentation regression, not a tidy.

**`AC-4` -- both forks are reported and neither is silently taken.** If the
delivered text picks a shape, it says which and cites what settled it.

**`AC-5` -- section 7 cites the landed exemplar by path.**

**`AC-6` -- no normative content moves.** This node changes worked examples and
their surrounding prose. **Section 3.2's attenuation bound, section 4's
revocation semantics, and the orientation net are untouched** -- if a respelling
appears to require changing one of them, that is a stop, not an edit.

## Stop conditions -- return to the Steward, do not decide

1. **The `sandbox` refinement needs a surface Ken cannot express** and neither
   shape above is clearly right. Report both readings; this is an Architect
   question about what the spec is claiming, not a spelling question.
2. **An example turns out to be semantically wrong**, not merely stale. The
   node is scoped to currency, and a wrong claim in the spec's only authority
   example is a different and larger thing.
3. **`D4`'s sweep reaches normative text.** If a retired name is load-bearing
   in section 3.2 or section 4 rather than illustrative, stop -- `AC-6` forbids
   the edit and the finding is worth more than the sweep.
4. **The landed exemplar itself looks wrong** while you are copying from it.
   That is a `catalog/` finding and it routes, rather than being worked around
   in the spec.

## Sizing

**`S`, and it is `S` only because `D2` REPORTS rather than resolves.** Six
examples, four axes, four residual sites, one citation, two forks named.

**`D2` is the part that can grow, and the `AC3` half is where.** If re-expressing
the order-dual pair turns out to need a normative statement the chapter does not
currently make -- about where the authority floor lives now that writes are
statically gated and reads dynamically gated -- **that is a re-cut, not an
overrun.** Stop and say so.

> **The honest read of this node's own history:** it was filed as *"a
> surface-currency repair, not a rewrite"*, and that is true of four of the six
> examples and false of two. **Do not let the `S` label carry the old claim
> into the work.**

## Not this node

- **Not [[CAT-CAPEX]]**, merged, which delivered the checked exemplar this node
  cites. That work is `catalog/`-only and Ergo does not edit `spec/`.
- **Not [[ABI-REVOKE]]**, which also touches this chapter but owns the
  revocation ABI rather than the worked examples. It is `draft`; **sequence
  the two if it is ever released.**
- **Not a rewrite of the authority discipline.** The argument is sound. This is
  a currency repair whose only surprise is that its axes are structural rather
  than lexical.
