# WP frame — `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT`

Node: `docs/program/issues/ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT.md`.
Owner: **runtime**. Size **M**. Estimated capability tier **T1**.

> # THIS NODE UNBLOCKS INCREMENT A OF `ABI-S6-HS18-MAIN-BASED-CLOSURE`.
>
> Increment A holds the call sites and **none** of the definitions. It is at
> `bd3e1ff676444f48771d261371982a8f289ce26a` — Category A cleared, 21 errors
> down to 11, **every remaining error attributable to this node**. Increment A
> cannot reach green until this lands.

## 1. Objective

Port the **checked-IH post-call consumer machinery** — a capability `main` never
grew — from the tree that has it, onto a `main`-based line, so that the verifier
substance's call sites resolve.

**Named for the capability, not for the errors that exposed it** (Architect,
`evt_6mptkrtvysd8s`). The eleven compile errors are a symptom that located it;
they are not its scope.

### 1b. SYMPTOM INVENTORY — the populations this node's model failed to predict

**Seeded by the Steward 2026-09-16 on the Architect's ruling
`evt_7wm7z1t416y0p`. This section was missing from both the node and the frame,
so the running count lived only in one seat's working context — which is exactly
what a compaction discards.** Append here as they occur; do not keep them in a
thread.

**The predicate, and it is the point of the section** (Architect, same ruling):

> **The frame modelled this node as "what is ABSENT from `main`," and every
> surprise so far has been something PRESENT on `main` that CHANGES.**

⇒ **An absent-definitions census cannot see a shape change, by construction.**
It keys on names that are *not* in `main`; every surprise below is a name that
**is**. Widening that census (the seed of 5 closing to 29) made it complete
without re-aiming it — its *subject* is still absence. The shape change was
found by **reading**, not by the census, which is why the next one is the same
class unless a differently-subjected instrument runs. See `§2b` (D0b).

| # | population | how it was found |
|---|---|---|
| 1-3 | the first three, from `thr_6azxdz555c2qy` | appended by runtime-implementer |
| 4 | seed of 5 closes to **29 items** (130 reached, 59 zero-ref, minus 19 variants + 10 fields + 1 doc-comment token) | D0 census |
| 5 | `RequiredConsumerProjection` is **resident on `main` and reshaping** — struct at `main:1204`, enum at port `:1318`; 13 refs / 7 files | read, NOT the census |
| 6 | `Copy` **dropped** from the same derive; `Ord`/`PartialOrd` added | read, NOT the census |
| 7 | the three accessors go **TOTAL to PARTIAL** with unchanged signatures | read, NOT the census |
| 8 | ten variants live behind a **dev-dependency feature** invisible to every permitted local build | manifest read |

**Rows 5-8 were all found by reading and none by an instrument.** That is the
finding about the method, not about the tree.

## 2. D0 — the question this node answers BEFORE it ports anything

**D0. What is the transitive closure of machinery absent from `main` that the
consumer machinery itself reaches?**

**UNMEASURED.** The Architect measured the five seed symbols and explicitly
nothing beyond them:

> Whether the consumer machinery *itself* calls into anything else that is 0 on
> `main` — I measured the five symbols you named and nothing beyond them. If it
> has its own absent dependencies, we find a fourth population at exactly this
> point again — so the new node's first deliverable is the transitive census,
> not the port. **Do not let it be cut as "add these five things."**

**The census is deliverable one and it gates the port.** Do not begin porting
before D0 is answered and posted.

### 2a. D0 is already known to be non-trivial, and that is measured

The five-symbol seed **was already short by one before this node was framed.**
`constructor_identity` surfaced only when Category A was attempted, and its
resolution took two corrections:

    constructor_identity, domain crates/**              main: 4 refs, 1 def
    constructor_identity, domain crates/ken-runtime/**  main: 0 refs
      positive control cranelift_backend, ken-runtime, main:  2239

`main` carries an unrelated free function of that name in
`crates/ken-elaborator/src/erasure.rs:6872`. **A census over `crates/**` reports
a false hit.** In `ken-runtime` the name is a *local variable*, and the actual
dependency is the method that produces it:

    b601e2ec7  lowering/core.rs:14247
      let constructor_identity = self.static_transition_plan
                                     .constructor_symbol_identity(origin)?;

**That method is PRESENT on `main`** — same receiver, byte-identical signature
and body (`closure.rs:1202`, `impl<'src> StaticTransitionPlan<'src>`). The
apparent third `impl` at `b601e2ec7` is a **string literal** in an
allowed-inventory test (`lowering/core/tests/mod.rs:1249`), not a definition.

⇒ **`constructor_symbol_identity` has NO Category B content, and the sixth
error's cause is UNESTABLISHED.** It is not an absent symbol and not an absent
impl. **Do not carry a third repair shape into the port on this example** —
resolve it in the census. What the sixth error proves is only that the seed list
was short and a compile found what five greps did not.

> **Why the retracted claim matters more than the correction.** A `fn +NAME`
> grep counted a **quoted string** as a definition. That is the same class as
> the `constructor_identity = 70` substring count in §2a — **an instrument that
> passed a reach control and was wrong on SPECIFICITY, in the inflating
> direction, both times.**
>
> ⇒ **A GREP KEY THAT MATCHES TEXT CANNOT DISTINGUISH A DEFINITION FROM A
> MENTION OF ONE** — doc comment, test fixture, inventory assertion,
> error-message string. The remedy is not a better regex; it is **resolving what
> encloses each hit before the count means anything.** `AC-ENCLOSING-UNIT`
> below exists for this.

**AC-ENCLOSING-UNIT.** A count that distinguishes definitions from references
is not reported until its matches have been **opened** and their enclosing
construct identified.

> **Control — re-run the census's own definition key and open every hit.** A hit
> inside a string literal, a comment, a macro body, or a test fixture is a
> **mention**, not a definition, and must not be counted as one. Three separate
> census errors today were caught this way and none by a control.

### 2b. D0b — REQUIREMENT 0. Runs before any port work, and gates it as D0 did.

**Added 2026-09-16 (Steward) on the Architect's ruling `evt_7wm7z1t416y0p`.**

> **For every symbol the port and `main` SHARE in the domain, diff the
> declaration.**

**This is the complement of D0's query, and the two have different subjects.**
D0 asks *what is absent from `main`* — it keys on names that are not there.
D0b asks *what is present in both and DIFFERS* — it keys on names that are.

⇒ **No amount of completing D0 can find a shape change.** Widening the seed from
5 to 29 made D0 complete and left it aimed at the wrong question. Every
surprise in `§1b` rows 5-8 is a shared symbol whose declaration differs, and
every one was found by **reading** rather than by an instrument. D0b is the only
instrument in this node that can return that class as a **measurement**.

**Acceptance for D0b, same standard as D0:** the domain pinned, both refs named,
a positive control on every zero, and the **enclosing construct** resolved for
each hit rather than the line reported. Report it before porting.

*Control that it is aimed correctly:* D0b must return `RequiredConsumerProjection`
(struct to enum) and its `Copy` drop. **Those are known members.** A D0b that
does not return them is measuring the wrong set, and that is a stronger check
than a positive control on a symbol nobody disputes — it fails on the exact
class the instrument exists to catch.

## 3. Fixed inputs, measured at named refs

Steward-measured 2026-09-16 at `origin/main`
`a75a470106248514a53a989987449b49c62834b8`, each with a live positive control.

### 3a. The seed — absent from `main`, present at the port source

    symbol                                        main   b601e2ec7 (defs/refs)
    CheckedIhPostCallConsumerStep                    0        1 / 20
    CheckedIhPostCallConsumer                        0        1 / 40
    checked_ih_generated_context_result_contract     0        1 /  5
    static_response_forwarded_result_identity        0        1 /  2
    checked_post_call_consumer_frame                 0        1 /  2

      domain           crates/ken-runtime/**
      port source      b601e2ec78989d32222b34b9ec68e01844c6d70b
      positive control cranelift_backend   main 2239 / b601e2ec7 2350

**Exactly one definition site per symbol, all under `planning/**`.** This is a
**port from a named tree**, not an authoring task — and the frame says so
because *"port these definitions"* and *"write this machinery"* are very
different sizes and the second is the wrong one.

### 3b. THIS NODE CONVERTS A LIVE TYPE. It is not only an addition.

> **REWRITTEN 2026-09-16 (Steward) on the Architect's ruling
> `evt_7wm7z1t416y0p`. The previous text was REFUTED BY MEASUREMENT and is
> replaced, not annotated.** It read:
>
> > *"`StaticTransitionPlan` is on `main` at 291 references. It is not a missing
> > layer. Absent is: one type family (`CheckedIhPostCallConsumer{,Step}`, 48
> > refs at base), two accessors on that existing type, and one on `Lowering`."*
>
> **Three things in it are false.** `RequiredConsumerProjection` is not absent —
> it is resident and changing shape. Two of the "accessors" are not additions —
> `source()`, `body_origin()` and `eliminator_origin()` already exist on `main`
> and are being **re-expressed**. And "not a missing layer" was written as a
> *bound* and has been **functioning as a floor**: it is the sentence that made
> every subsequent surprise surprising.

**`StaticTransitionPlan` is on `main`** at 291 references, and that part stands.

**What this node does is not only "add what is absent."** At least three
resident things change, measured at `origin/main` `432d36254` against port
`b601e2ec7`:

    RequiredConsumerProjection   struct (main :1204)  ->  enum (port :1318)
                                 13 references across 7 files
                                 main :6349 constructs it as a struct literal

    its derive                   Copy DROPPED; Ord/PartialOrd added
                                 (carry Ord only with a named consumer — see R3)

    source() / body_origin() /   TOTAL on main  ->  PARTIAL at the port,
    eliminator_origin()          signatures UNCHANGED

**The population is NOT restated to a final number here, deliberately.** D0
measured 29 items, but D0's subject is *absence* and `§2b`'s D0b is outstanding
and is expected to move the count. **Writing "29" as the scope now would repeat
this section's original defect in a new number** — a figure that reads as a
bound and functions as a floor. The bound is D0 plus D0b, and it is not closed.

⇒ **Do not size this node from a count of absent definitions.** A struct-to-enum
conversion of a live type touches every construction and every field access on
`main`, and the totality change touches call sites that the compiler will not
flag at all (`§5`, R1).

### 3c. `b601e2ec7` IS EVIDENCE, NOT A BASE

> Read it, port from it, and **never make it an ancestor of a candidate.** The
> candidate bases on `origin/main`, per the operator's 2026-09-16 direction
> ("do not build on an unmerged commit"). A port whose provenance is a tree is
> still a `main`-based candidate.

Identical in force to the rule the parent node applies to its two `preserve/`
refs.

## 4. Deliverables

1. **The transitive census** (D0), posted before any port work begins, with its
   ref and domain pinned and a positive control on every zero.
2. **The ported definitions**, based on `origin/main`.
3. **A disposition for the sixth error**, whose cause is unestablished.
   `constructor_symbol_identity` is **not** it — that method is on `main`,
   byte-identical (§2a) — so this deliverable is to find what the sixth error
   actually is, not to port a symbol already believed absent.
4. A statement of what the census found **beyond** the seed, by name, including
   "nothing" if that is the answer — stated as a measurement, not a silence.

## 5. Acceptance criteria, each with its control

**AC-CENSUS-IS-A-PREDICATE.** The census's scope is stated as a predicate —
*every symbol whose fix requires machinery absent from `main` in the
`ken-runtime` domain* — and its closure is asserted over that predicate, **never
as "the listed symbols are ported."**

> **Control — read the exit criterion and ask whether a NEW symbol would
> violate it.** If the criterion can be satisfied by a tree that still fails to
> compile on a twelfth symbol, it is an enumeration wearing a predicate's
> clothes and it fails this AC. **A list cannot report being incomplete**, and
> this list already proved it by being short by one.

**AC-DOMAIN-PINNED.** Every census measurement names **both** its ref and its
path domain.

> **Control — `constructor_identity` is the regression case.** Re-run the
> census's own command for it over `crates/**` and over
> `crates/ken-runtime/**`. The two disagree (4 vs 0). A census that reports one
> number for it without a domain fails this AC even if the number is the right
> one, because **an unanchored figure is unfalsifiable rather than false.**

**AC-CONTROL-ON-EVERY-ZERO.** No zero is reported without a positive control
returning non-zero from the same instrument, ref and domain.

> **Control — the zeros in §3a are only measurements because `cranelift_backend`
> returns 2239/2350 beside them.** A census with no live positive result cannot
> distinguish "absent" from "my pattern was wrong."

**AC-EVERY-MEMBER-MEASURED.** Every symbol the census *reports on* has its own
measurement. A member that entered the list from a compiler diagnostic, a
grep of error text, or an inference is **not** measured, and the report must
not cover it with a collective claim.

> **Control — compare the census's MEMBER list against its MEASUREMENT list by
> name and by cardinality.** They must be the same set. If the report says "all
> N are absent from `main`" while N-1 rows carry counts, it fails.
>
> **This AC exists because that is exactly what happened** (runtime-implementer,
> `evt_5bcs52n4x7rdj`, self-reported): five symbols were measured with a pinned
> domain, per-file counts and a positive control; `constructor_identity`
> entered the list **from the error message text** and was never measured at
> all; and the summary sentence — *"all 11 are Category B, all 0 on main"* —
> covered six symbols on evidence for five.
>
> ⇒ **AN UNMEASURED MEMBER INSIDE A MEASURED LIST INHERITS THE LIST'S
> CREDIBILITY.** Nothing in the presentation distinguishes the row that was
> measured from the row that was assumed. `AC-CONTROL-ON-EVERY-ZERO` does not
> catch this: a census can control every zero it reports and still carry a
> member it never measured.

**AC-WORD-BOUNDARY-OR-JUSTIFY.** Symbol counts use word-boundary matching
(`\bNAME\b`), or the report states why substring matching is correct for that
symbol.

> **Control — `constructor_identity` on `main` in `crates/ken-runtime/src/`:**
>
>     substring grep    70
>     word boundary      0
>
> The 70 are longer identifiers — `case_constructor_identity` (34),
> `synthesized_constructor_identity` (17), and others. **A substring census
> reports 70 and concludes the symbol is PRESENT on `main`, which is the exact
> inverse of the truth**, and it fails in the direction that silently removes a
> symbol from this node's scope. The symbol survived only because nobody ran the
> careless instrument on it.

**AC-NOT-A-BASE.** `b601e2ec7` is not an ancestor of the candidate.

```sh
git merge-base --is-ancestor b601e2ec78989d32222b34b9ec68e01844c6d70b HEAD \
  && { echo "FAIL: port source is an ancestor of the candidate"; exit 1; }
echo "OK: port source is evidence, not a base"
```

**AC-NO-CAPABILITY-FLIP.** The refused `MappingAcquireFile` capability grant does
not ride along. The preserved checkpoint carries it across five coordinated
sites and it is out of scope here exactly as in the parent.

> **Control — grep the candidate's diff for `MappingAcquireFile` and expect
> zero.** Read the parent frame `§4` before touching either tree.

**AC-NO-INVENTED-DEFAULTS.** No absent dependency is discharged by substituting
a value that merely compiles.

> **Control — `OwnedContext.result_contract` is the worked example.** It could
> be set `None` today and **would build**, silently disabling the identity check
> the field carries. The correct disposition, already taken by the implementer,
> is to write the real expression and let the file **fail on a named dependency
> rather than pass on an invention.** A candidate that "fixes" that back fails
> this AC.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run.
Local verification is targeted only, through `scripts/ken-cargo`, scoped
`-p ken-runtime`.

> **Build under memory pressure: use `-j 1`.** Two default-parallelism
> `-p ken-runtime` builds were OOM-killed at ~5.5 GB available on 2026-09-16;
> the same build at `-j 1` succeeded at the same headroom. See §7.

## 6. Contention

`crates/ken-runtime/src/cranelift_backend/planning/**` and `lowering/**`.

**Increment A of the parent node is live in the same files** at
`bd3e1ff676444f48771d261371982a8f289ce26a`. That is not a conflict to resolve —
it is the reason this node is sequenced first. **The runtime ring holds
increment A until this lands**, and its own Category A work is complete, so
there is no concurrent authoring in those paths.

## 7. The box: what is measured, and the cost class this node MOVES INTO

**There is no `-j` recommendation here, and the earlier one is withdrawn.**
An earlier revision of this frame recorded `-j 1/2/3 build, -j 6 kills` as a
measured bound. **It was not a bound.** Instrumented (@runtime-implementer,
`evt_7y3efwmhjbxea`):

                         -j 1        -j 3
    rustc processes         1           1
    max threads/rustc       4           4
    peak summed RSS     960 MB      948 MB
    crates compiled         1           1
    terminus            11 errors   11 errors

**`-j 1` and `-j 3` are not two points on a curve; they are the same measurement
taken twice.** A crate that fails type-check never reaches LLVM codegen, so
there were no codegen threads for the jobserver to bound and no CGUs to
distribute. **The flag was not the variable.**

⇒ **BEFORE ARGUING WHICH KNOB GOVERNS A PHASE, VERIFY THE RUN REACHED THAT
PHASE.** Two seats spent three exchanges arguing which knob governs codegen
parallelism in a build that does no codegen, and neither asked. That is the
durable lesson; the numbers are not.

### 7a. What is actually known

- **The ABORTING shape** — what every seat hitting these eleven errors runs —
  costs about **950 MB in one rustc process and is independent of `-j`.** There
  is no `-j` decision to make while the tree aborts at type-check.
- **The `-j 6` kills are real events that remain UNATTRIBUTED to the flag.** The
  work differed (multi-crate, cold-ish, after large edits), not just the setting.
- **Codegen and link are UNMEASURED at every `-j`.** One attempt to force
  codegen on a single green crate (`cargo rustc -p ken-host --lib -- -C
  opt-level=3`) was **killed for system memory pressure at `-j 1`.**

### 7b. THE RISK THIS NODE CARRIES, and it is the reason §7 still exists

**@runtime-leader's forward implication (`evt_6jfc6j2z1s7aa`), and it is a
sizing risk rather than a build tip.** Increment A currently aborts at
type-check. **The whole point of this node is to make it stop aborting.** So the
moment this port lands, increment A moves:

    ABORTING shape     ~950 MB, one process, -j-independent      SURVIVABLE
    COMPLETING shape   codegen + link, UNMEASURED at every -j,
                       and one green crate was already killed
                       at -j 1                                   UNKNOWN

⇒ **This node is currently priced on the wrong cost class.** The box may not fit
the very build that closes the parent WP, **regardless of `-j`** — and nothing
measured so far bears on that, because everything measured so far was an
eight-second frontend abort.

**The implementer must surface this as a hard stop rather than absorb it.** If
the first build that compiles through gets killed, that is **not** a `-j`
problem to tune around and **not** the implementer's to solve: it is a box
capacity question for the Steward and the operator. Say so and stop.

**`COORDINATION §12` is untouched by all of this.** Targeted builds only,
through `scripts/ken-cargo`, `-p ken-runtime`; workspace/locked/conformance in
CI. What changed is that the throughput cost once claimed for a `-j 1`
constraint is withdrawn along with the constraint.

## 8. Not this node

- Increment A's eleven Category-A errors. Complete, and not this node's.
- `MappingAcquireFile`. Refused, and out of scope in both nodes.
- Increments B (Q1 resume-exit repair) and C (amendment 8's consumer
  relocation). Both stay in the parent.
