# Authoring a frame: fixed inputs, audits, and acceptance criteria

Steward task procedure. Read at the point of use. Governing playbook:
`../steward.md`. The release sequence is in `release-and-handoff.md`.

The brief must pin every settled decision as a fixed input (cite `/spec` and
the OQ register — never leave a decided fork open for a lower-tier model to
relitigate), give a mandated deliverable outline where each section ends in a
concrete implementable choice rather than a survey, list testable acceptance
criteria, and state the do-not-reopen guardrails. This is the frame — scope,
acceptance, sequencing, settled-decision pinning — not the full spec.

**Never ship a frame without this clause:** *"treat every anchor as perishable;
if a fixed input turns out false against the landed code, say so and escalate —
do not quietly build around it."* A T1-authored frame is still wrong sometimes,
and that clause is the only thing between a bad pin and a ring confidently
building the wrong thing.

**Frame by objective and acceptance, and treat any current-implementation-state
claim as perishable.** A frame saying "seam X currently does Y, patch this
hole" can go stale between authoring and elaboration, and a stale *"what is
broken"* is worse than a stale *"what is done"*: it misdirects the team to
rebuild removed unsoundness. Prefer describing the goal and acceptance, and tag
any current-state claim *"verify against the landed code, not this line."*

## FRAME THE REPAIR, NOT A MEASUREMENT OF IT. Operator, 2026-08-15.

**Verbatim:** *"Roughly 50% of design discovery happens in implementation, and
there's often no way to understand the structure of a problem without trying to
solve it directly. The measure node, then repair node runs directly against
that. Measure after a failed repair, but first make the a priori best guess of
what the repair should be with the information you have."*

⇒ **The default frame states a best guess and asks the ring to build it.**
Discovery is expected to happen *inside* the attempt. A failed attempt is a
measurement, and a better one than a standalone probe, because it fails from the
inside with the structure in view.

**Do not split a repair into measure-then-repair to protect against
mis-framing.** Operator, same ruling: *"we already have a QA and architect review
process which will catch mis-framings. The basic process should stay simple for
the majority of the cases that work out as expected."* The split buys a guarantee
those reviews already provide and costs a full turn every time.

> **The failure this is correcting, because it is easy to repeat.** On the
> `RecursiveDescent` retirement a measure-first split was the correct **reaction**
> to one earlier wrong repair — and the Steward promoted it into standard
> process and wrote it into every frame. **Measured 2026-08-15: five consecutive
> measurement nodes merged in one day, zero residuals removed, and the counters
> flat** — while the one cell where a repair would have happened sat blocked and
> unattended. **Each step was individually justified; the sequence had no
> termination condition.**

**When a split IS still right** — keep it rare and name the reason in the frame:

- the two outcomes differ by roughly an **order of magnitude** in size, or one
  is unbounded (a spec change, a new abstraction);
- the attempt would be **destructive or hard to reverse**;
- the guess would consume a **scarce shared resource** to find out.

**Otherwise write the guess down explicitly** — as a claim a reviewer can attack
directly — bound the attempt to **one honest try plus a handback**, and say what
a blocked attempt should report. **A stated wrong guess is cheap; an unstated one
is what review cannot catch.**

**Corollary: do not park work on an absent ruling.** If a stop is waiting on a
disposition and nothing records it arriving, guess and attempt rather than wait.
Review is the backstop.

### THE SECTION ABOVE HAS NO DETECTOR, AND WITHOUT ONE IT DOES NOT FIRE.

**Measured 2026-09-18, this seat, against the section above.** The operator set
clearing the ignored rows as the top priority on 2026-09-17. Over the preceding
five days the runtime lane landed **thirteen** distinct nodes — census,
disposition, correction, re-attribution, coverage — and the selected population
went **15 to 15**. Then the next six framed WPs were checked and **not one had
the row going green as a deliverable**; four said so explicitly, in an `AC` or a
dedicated section, each with a defensible local reason.

**That is the 2026-08-15 failure at 2.6x, by the same seat, with this section
already written and loaded at every session start.** Reading it does not fire
it. Each frame's prohibition is argued on its own merits at authoring time, and
the ratio is invisible from inside any one of them — which is exactly what the
2026-08-15 note says and is still not enough to catch it.

⇒ **The section describes how to author one frame. It needs a check that reads
the population.** Run it at the watchdog tick, not at authoring time:

    for the objective the operator named, name its POPULATION and its
    CURRENT NUMBER. Then: of the frames released and queued against it,
    HOW MANY HAVE MOVING THAT NUMBER AS A DELIVERABLE?

**Zero is the alarm, and it is a reading on the frames, not on the teams.** A
lane can be highly productive, land every node correctly, pass every review, and
score zero here — that is the observed state, not a hypothetical. **Node
throughput and objective throughput are two numbers, and only one of them is the
priority.**

**Two traps in running the check, both hit on 2026-09-18:**

- **The number must come from the objective's own definition, not from a grep
  that resembles it.** A raw `git grep -c '#\[ignore' -- crates/` returns 28; the
  attribute count excluding `//` comment lines is 23; the sweep's selected
  population is 15. Three numbers, one subject. **Read the producer** — the
  ledger node states its selection rule and the cross-check that makes it
  falsifiable.
- **Two populations of equal size are not the same population.** The ken-cli
  `#[ignore]` attributes number 15 and the sweep's selected rows number 15, and
  they differ in membership at both ends: the selected set excludes a
  deliberately parked row and includes one row in `ken-runtime`. **A count that
  matches is not a set that matches**, and a headline built on the coincidence
  names nobody.

**When the check returns zero, the remedy is a kick, not a rule.** Find the
released node whose completion moves the number and kick that one next. On
2026-09-18 it existed, was `ready`, and was not what got kicked — the kick had
been chosen for base-freshness, which is an instrument about staleness and says
nothing about the objective.

## Before you pin a fixed input: five audits, every time

A fixed input is only as good as the substrate it stands on, and **grounding
the names is not grounding the obligations.**

## Audit (a): dependency-DAG check

If the WP introduces an abstraction an existing package will consume, draw the
load order and look for the cycle. **An abstraction module must never depend on
its clients:** home each instance with the carrier it is over, and make the
generic module define its own parameterized result and error carriers. The
moment it reaches for a client's concrete type, the cycle returns.

Measured on CC3: a `ByteCursor` instance homed in the new `Cursor` module *and*
`Cursor` ordered before the CAT-5 that declares its `Source`, giving
Cursor to CAT-5 to Decoder to Cursor. **The tell was wanting cosmetic
symmetry** — "both instances in one module" — and cosmetic symmetry created the
cycle.

## Audit (b): constructibility, for every promised carrier field

For each field pinned at a structural type (`Nat`, `List`, ...), ask whether
the landed primitive can actually produce it. Opaque primitives (`Int`,
`Bytes`, `String`) are constructible but not destructible — reading a length,
index, or size *out* of one is exactly the hop that does not exist.

Measured on CC3: `remaining : Nat` pinned over raw `List Bytes`, but
`bytes_length : Bytes -> Int` and no `Int -> Nat` bridge exists. The field was
unproducible. **Opaque representation boundaries are design constraints, not
implementation details.** The landed idiom is a proof-carrying cached-`Nat`
wrapper: carry the `Nat` and prove it agrees with the opaque length. Never
convert, and never mint the missing primitive — that is a TCB delta and it goes
to the operator, not into a build WP.

## Audit (b-prime): seam and ABI — can the landed interface carry the value?

(b) asks whether a primitive can *produce* a pinned type. This asks whether the
landed interface can *carry* it. **Trace the value end-to-end through every
seam it must cross, not just the one the design names.**

Measured on I-5: ADR-0017's central security property is *"check and use share
the resolved fd."* The seam at `authorizes` was correctly verified as pre-cut,
but the seam below it, `HostHandler`, speaks only `fs_*(&[u8])`, and
`fs_dispatch` hands the original path bytes to the handler after the check.
Check and use cannot share an fd. The design was coherent, the TCB verdict was
right, and it was unbuildable through the landed ABI. **The tell: a design that
names one seam as already pre-cut. Check the seams it did not name.**

## Audit (b-double-prime): genericization

When a WP makes an existing concrete path generic over a trait, "is the trait
public?" is the wrong question. The right one: **can the generic version
perform every step the concrete version performs?**

**The tell is greppable** — the concrete type's inherent methods that the call
path uses, which the trait does not declare:

```sh
rg 'concrete_host\.\w+\(' <the path>   # methods the path calls
rg '^\s*fn \w+' <the trait>            # methods the trait declares
# the DIFFERENCE is the gap: a blocker, not a detail
```

Measured on I-6: `run_io<H>`, `HostHandler`, and `CaptureHost` were all
verified public and re-exported — all true — and a generic `run_program<H:
HostHandler>` was framed anyway. But the runner mints the program's capability
via `PosixHost::mint_fs_cap`, an inherent method, and `HostHandler` has no mint
operation at all. One line, and the whole WP was unbuildable as framed.

## Audit (b-triple-prime): the general form, expressibility

Architect synthesis, 2026-07-14. **This subsumes the genericization audit**,
which is its trait-shaped special case. Full statement:
`agent/memory/fleet/never-pin-a-shape-that-cannot-state-its-own-contract.md`.

> **Every obligation this shape must carry — where does it get written, and is
> that place inside the shape's own checked vocabulary, or a reach outside it?**

Three consecutive expensive near-misses were one failure. A shape is pinned — a
trait, a type, an effect op, a spec claim — that must carry an obligation. The
obligation is not false and not expensive to prove. It is **unsayable in the
shape as pinned**:

- **I-6** — the trait had nowhere to say *"any host mints its own-identity
  cap"*, so the obligation lived on the concrete type.
- **The primitive-`Op` erratum** — conversion had nowhere to say
  `byteLength "abc" = 3`, so the spec said it in prose the kernel cannot check.
- **I-8** — `monotonic_now : {Clock} -> Int` had nowhere to say *"this read is
  at least the last"*: two `Int`s, no handle relating them.

**Why it is invisible in a green diff.** Tests exercise values; the gap is in
the type, ABI, or relation surface. The values compile and pass because the
missing thing was never a value — it was a place to write a guarantee. No
amount of green touches it. **The audit is not "run the suite." It is: try to
write each obligation in the shape's own vocabulary and see if your pen has
anywhere to land.**

**The escape hatches are the doors it arrives through:** a comment, a
per-consumer `Axiom` (unbounded TCB), a new trusted primitive (TCB growth), a
caller-fabricated value that manufactures the missing binding, a concrete-only
method the generic path cannot reach. All say the same thing — the shape had no
home for the obligation, so we put it somewhere nothing checks or somewhere
that costs trust.

**The audit:** enumerate the shape's contract obligations; for each, name the
in-shape checkable home (term, type, handle, method) where it is written and
checked. Any obligation whose only discharge is a reach-outside is an
expressibility gap. Either extend the shape to give it a home — and if that
home grows the TCB, it is the operator's call — or descope it honestly. I-8
shipped wall-clock-only with no ordering law, because a wall clock genuinely
has none; **the absence of a law is the truthful statement, not a gap.**

**This is structurally the design pass's catch, not QA's.** The implementer
builds values and they compile; QA tests values and they pass. Only the design
pass asks whether the shape can express its contract. That is why the
design-review edge exists at all.

## Audit (c): corpus-oracle enumeration

If the WP adds a file to a globbed directory (`catalog/`, `examples/`,
`conformance/`), it must satisfy every corpus-wide oracle, and those live in
crates the WP never touches. Targeted per-crate validation cannot see them, so
they surface as red CI at publish — after review, after the merge Decision, the
most expensive place to find them.

```sh
rg 'collect\(.*catalog|examples/rosetta' crates/*/tests/
```

Name each one in the ACs. "The formatter gate" is rarely the only one. (CC3:
AC6 named `ken_fmt.rs` and missed `kenfmt_c_capstone.rs`, giving red CI on a WP
that had passed QA, the Architect, and the author's own honesty gate.)

**When one of those oracles is a frozen baseline table, do not re-baseline it
to make the build pass.** A file created after the frame has no honest
pre-frame value, so the row you add is fabricated and its check is vacuous
forever. Re-scope the oracle to its own historical set and let a live-anchored
property cover new files — confirming that live net exists first, or you trade
a rubber stamp for a hole.

## Audit (c-prime): cite prose sites by grep-able phrase

A coordinate is a time-sensitive operand.

**Promoted 2026-08-08 from `RT-CONTSPEC-LEDGER`, and it binds the frame author
first because that is where the defect originated.**

**The rule: cite by a grep-able phrase with no number.** Reach for `path:line`
only when no phrase is stable, and then only at **the SHA you are handing off**.

> **Do not weaken this to "always attach a base to your coordinate." That
> repair PASSES on the case that produced the rule and still fails.**
>
> That frame qualified its coordinates **three ways** — *"All anchors are in
> `…/planning/static_transition.rs`"*, *"Measured by the Steward at
> `0fd9f6e8…`"*, and *"read every line number as an anchor to re-find, never as
> a value to check."* All three sat in one section. `D4` cited `:4729` and
> `:6304` **ninety lines below it** and inherited none of them in practice.
>
> ⇒ **Distance defeats every qualifier equally.** A qualification that lives in
> a different section does not travel to the citation. If a reader must scroll
> to learn which file and which tree a number belongs to, it is unqualified at
> the point of use.
>
> **And the base it named was true and irrelevant** — `0fd9f6e8` was one commit
> behind the `5da614ba` the node was released at, so the coordinates were
> correctly measured against **a tree no reader of that frame would ever check
> out.** That is worse than an unqualified number, for the reason the Adversary
> gave about the wrong filename: **it carries visible evidence of care.**

Downstream those coordinates acquired the wrong filename, and an auditor
checking whether `D4` discharged would have opened live unrelated production
code and concluded the deliverable never landed.

⇒ **A citation pointing at a real thing that is not the thing is worse than one
pointing at nothing.** A bad path 404s and gets fixed; a plausible-but-wrong
path gets believed, and it fails in the direction of "the work is missing" when
the work is present.

**Both numbers were also stale on the day they were written** — grepped at one
`main` and published in a frame based three merges later. Ask the frame's own
standing question of the coordinate: **which state was this measured in?** We
routinely ask it of a before/after figure and forget that a line number is the
same kind of operand — **and one destroyed by the very change the deliverable
performs.** A frame whose job is editing those lines has guaranteed its own
coordinates are wrong by the time anyone checks them. That is the spent-oracle
trap wearing a different hat.

**The phrase is the durable instrument.** It survives the edit; the number does
not. Where a number genuinely helps a reader navigate, write it as *"at
`<sha>`, around `path:NNNN`"* — an anchor to re-find, never a value to check.

## Audit (d): reuse must be proved behaviorally, not structurally

When a WP is framed as a specialization of landed substrate ("consume CC1-CC6,
do not rebuild them"), **the ordered shared-`ElabEnv` harness makes reuse look
true even when it is false.** Loading a dependency is not using it. A package
can declare the landed `Decoder` — so it appears in the closure and every
import check passes — and then shadow it with a private byte loop. A green
suite hides this perfectly.

**The AC must be behavioral: the landed abstraction must be driven.** The
Architect's phrase is the test to keep — *genuinely driven, not
declared-then-shadowed.* Press the mechanism, not the imports, and write the AC
so a reviewer can tell the difference.

## Audit (e): after a disposition is refuted, sweep the frame BY PHRASE

**Promoted here from the watchdog tick prompt, 2026-08-17**, which had no room
left and is the wrong home: this fires when a ruling lands, not on a timer.

When a disposition a frame rests on is refuted, **the correction is not the AC
alone.** Sweep the whole artifact by the dead phrase: title, lede, deliverables,
every AC, and the tracker node's one-liner. **A reader consults the title
INSTEAD of the ACs**, so a title still asserting a refuted disposition
out-argues a corrected criterion two screens down.

**Measured: five dispositions died on one node in one arc, and the frame still
asserted every one of them.** Grep the dead phrase and prove zero live hits —
the phrase, not the conclusion, because the conclusion gets paraphrased and a
paraphrase survives a grep keyed on your own wording.

## Audit (f): a CITED coordinate is not a VERIFIED one, including the Architect's

**Promoted here from the watchdog tick prompt, 2026-08-17**, for the same reason
as audit (e): it fires when you frame, not on a timer, and the tick had no room.

**Locate the symbol yourself before framing on it.** Repeatedly a cited site
named a function that did not do what was attributed to it — and the citation
came from the Architect as often as from anywhere else, so seniority is not the
filter. Open the file, read the function, confirm it does the thing the frame is
about to require of it.

**Two forms this takes, both measured:**

- **The coordinate is right and the attribution is wrong.** The line exists, the
  function exists, and its behaviour is one level above or below what the frame
  needs. Route 1 of `resolve_recursive_unit_body` is the clean example: its doc
  sentence describes the recursive *position*, the code at that site turns on the
  *scrutinee*, and a frame quoting the doc as warrant for the code would have been
  wrong while quoting accurately.
- **The coordinate decays.** A `path:NNNN` is destroyed by the very edit the
  deliverable performs — see audit (c-prime). Cite by grep-able phrase.

## Frame patterns by WP type

Each of these was promoted after the shape cost a merge or a red `main`.

**A contract or boundary WP that cites in-flight builds** must make the merge
Decision a hard gate on those builds being green and merged. Cite the gates,
never restate their verdicts, so the audited contract equals the surface the
code exposes the day it lands. Never freeze a transient pre-build code state.
K-api's freeze-gate held the contract open exactly long enough to catch a
reversed quotient-respect `cast` direction, and released the instant they
converged.

**A capability-gate un-stage or reopen frame** must require the author to
re-verify that each un-staged net's per-branch obligation falls *within* the
landed capability's power — not merely that the capability merged. The K4/K5
arc proved this twice: an un-stage wrote "K4 landed, therefore the laws are
provable" flat, name-matching the merged capability, but the
concrete-`Eq`-conclusion laws escaped K4 into K5. State the conclusion-shape
axis as a hard AC: for each un-staged net, show its obligation reduces within
the landed capability; if any reduces to a further primitive, it stays gated on
that next capability.

**A frame that adds a new kernel `Term` variant** must enumerate *every*
soundness-relevant exhaustive walker that needs the new arm, not just one. K5's
AC6 named the termination walker (`sct.rs::collect_calls`) but not the
trust-accounting walker (`foreign.rs::collect_consts_in_tb`), whose omission
undercounts a postulate in `trusted_base_delta` and launders trust surface. It
surfaced as CI-red mid-merge. Enumerate the walker set as a hard AC — at
minimum termination and trust-accounting, plus subst, conv, and children — each
with the arm and, for the soundness-relevant ones, a neuter-the-arm flip test.

**A frame for a kernel reduction or completeness change** (whnf, iota,
`eq_reduce`) must require full-workspace-green validation and must not assert a
kernel-only diff. A sound completeness change makes an already-reducing path
reduce more completely, forcing migration of every downstream proof term that
was riding the old incompleteness — and those live both in the crate and in
shipped `catalog/packages/` proofs. K7's frame asserted "the `ken-kernel` diff
is the only diff", the build validated `-p ken-kernel` (153 green), and
`lawful_classes.ken` rode the same incompleteness, giving red `main` and an
Architect hold. So the frame must: (i) distinguish the soundness surface
(kernel-only, legitimately asserted) from the landing unit (workspace-wide);
(ii) make the no-regression AC **workspace-green in CI**, never a local `cargo
test --workspace` — local agents build and test only the touched crate,
`COORDINATION §12`; (iii) state up front that downstream proofs migrate
land-together in one workspace-green unit.

**A WP built on a cross-repo or external handoff** must name the epistemic
boundary: mark which facts are locally verifiable and which are
externally-sourced-and-trusted, and route confirmation to the cross-repo owner
rather than leaving the author to launder an unverifiable citation into a
normative spec. A *narrowing* of a co-owned contract is ambiguous by
construction — new divergence versus catching up to the counterparty's
finalized contract — so it routes to the cross-repo owner, never asserted
settled. The principle underneath: **Ken classifies epistemic status, never the
counterparty's mechanism.**

## THE LAST SWEEP BEFORE YOU RELEASE: every prose requirement must be an AC

**Measured 2026-08-13, on both sides of one handoff in one afternoon.**

> **A deliverable stated in prose and never carried into an AC is a deliverable
> the frame cannot check. A ruling's constraint that never becomes an AC is a
> constraint the build cannot fail.**

`RT-4B-ENUMERATION-INPUT-SIZE` said in its D2, in plain words, *"drive the same
real `C2_MIXED_SOURCE` through `compile_native_program_sources`."* Its five ACs
then constrained where the count came from, what stayed identical, which channel
carried it, that a mutation redded it, and that the artifact stated its
ambiguity limit. **Not one said which witness.**

⇒ An in-crate implementation on D2j fixtures satisfied **every** acceptance
criterion while answering a different question — and two of those fixtures were
**perturbed so the mechanism could not fire**, so the increment reported a
negative control's designed outcome as a discovery. Nothing in the frame could
have caught it, and the Steward published the reading.

The Architect's half was identical from the reviewer's side: of four constraints
in his 4b envelope, the two that became ACs were honoured exactly and the two
that stayed prose were silently not met.

### The detector, which is the part worth more than the rule

**The constraints that resist becoming ACs resist because they are HARD — and
hard is exactly where a frame asks for something its own fixed inputs forbid.**

So their absence is not an oversight to tidy up; it is a **signal to re-check
the frame against its own fixed inputs.** In this case the unenforced
requirement was unenforceable: the observation is `#[cfg(test)]` inside
`ken-runtime`, and the named witness drives through `ken-elaborator`, which
links a build where those calls do not exist. **The frame asked for something
impossible, and the missing AC was the tell.**

### Do this, mechanically, before you flip a node `ready`

1. **Read your own deliverables and hard stops for any requirement stated as
   prose.** Especially *which witness*, *which population*, *which build*.
2. **For each: is it in an AC?** If not, either write the AC or delete the
   prose. **Never leave it as prose you are hoping someone honours.**
3. **If it resists becoming an AC, stop and ask why** — that is the detector
   firing, and the answer is usually that a fixed input forbids it.
4. **Sweep the whole frame when you add a constraint late.** A frame that
   contradicts itself gets read at whichever half is convenient. Adding the
   four-row answer table to `RT-4B-C2-REACHABILITY` left a `D1` two paragraphs
   above still saying *"report yes or no"*; the convenient half was the one that
   loses the distinction.
5. **After you change a thing, grep the diff you are about to commit for
   references TO the thing you changed.** Not the whole frame — the diff. A
   frame refers to its own parts by description (*"the SHA in the header
   above"*, *"the table in D2"*, *"the count in §3"*), and those references are
   invisible from the edit site.

   > Measured 2026-09-17 on `LANG-ATOM-START-CLASSIFICATION-CLOSURE`. The header
   > was rewritten from a release-time SHA to *"name your own base"* — correctly,
   > because a pinned SHA decays between release and the cut. `AC-0` twenty lines
   > down still said *"and not at the SHA in the header above if `main` has moved
   > since"*, now pointing at a header that no longer names one. **The edit was
   > right and it orphaned a pointer to itself.** One `git diff | grep -n
   > 'header above\|above\|below'` would have caught it; the Architect caught it
   > instead.
   >
   > **The general form: a reference by DESCRIPTION does not break when its
   > target changes — it silently starts describing something else.** A reference
   > by name breaks loudly. Prefer names, and grep for descriptions.

## AC-0: THE FIRST AC MUST BE ABLE TO REFUTE THE FRAME'S OWN PREMISE

**Ask of every frame: which AC fails if the defect does not exist?** If the
answer is "none", the frame is a set of controls on a repair and cannot report
that there was nothing to repair.

**Write AC-0 before AC-1.** It re-establishes the frame's premise **at the
implementer's own base**, by re-running the frame's own probes, with the output
pasted. It carries a **stop condition**: if the premise does not reproduce,
stop and report that — do not proceed to AC-1 and do not repair forward.

> **Measured on `RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION`, closed REFUTED
> 2026-09-17 with nothing landed.** Ten acceptance criteria, every one a control
> on the repair. The premise — that a single slot is overwritten per
> construction — was false on all four rows (`RTPROBE-WRITE = 1`, written exactly
> once per compile). **None of the ten could have failed, because none of them
> was about the premise.** The ring measured it in one turn; the frame had had no
> way to ask.

**Three properties, or it is not AC-0:**

1. **It re-measures at the implementer's base, not at the framing SHA.** A
   premise measured at framing time is a claim about a tree that has since
   moved. Do not pin a base SHA in the frame for AC-0 to check against — tell
   the implementer to name their own base and measure there.
2. **Its probes are the frame's own**, quoted with expected output, so the
   implementer runs the same instrument that produced the claim rather than
   inventing one that agrees.
3. **A null result is a REPORTABLE OUTCOME, stated as such.** If the frame does
   not say "premise does not reproduce" is a result, it will be read as a
   failure to reproduce — and the implementer will look harder instead of
   reporting.

**AC-0 is cheapest exactly where it is most likely to fire:** a frame whose
premise was established by careful measurement the night before. Having spent
the effort establishing that the defect is real, *"what if it isn't"* is the
question you are least able to ask.

## A COMMISSIONED MEASUREMENT CARRIES ITS TERMINATING OBSERVATION, IN THE FRAME

**Any deliverable or AC that commissions a measurement whose outcome you do not
already know must state, beside the method, what observation ends it — and what
the other outcome looks like.** Not in the kickoff, not in a memory lesson: in
the frame. The rule binds at specification time, and the frame — rather than a
lesson whoever writes the frame may happen to recall — is where specifications
get written.

**Two clauses, and the second is the one that gets skipped:**

1. The observation is written **before** the measurement runs.
2. **It must be able to come out either way.** An observation that cannot fail,
   or that fires on the correct case as readily as the defective one, is not a
   terminating observation — it is a checkable sentence.

**The tell that you have written clause 1 without clause 2:** you can state the
observation without knowing which way it will go, *and you already know*. If you
can predict the result from the frame alone, it discriminates nothing.
**Checkability is not discrimination.**

> **Measured on `RT-DUPLICATED-RESPONSE-BLOCK`, 2026-09-18, both branches of one
> D0.** Outcome (2) opened *"every `static_origin` is visited exactly once"* —
> true by construction, since the table is a `Vec` indexed by origin with a
> single writer that refuses a second entry. Outcome (1) required *"two distinct
> `static_origin` values carry byte-identical case rosters at a constant
> offset"* — which is what a correct compiler emits whenever it inlines two call
> sites of one proc. **One could not fail; the other could not distinguish.**
> The decision was carried entirely by a control the frame had listed as an AC,
> and that control separated one-copy from two-copy, never legitimate from
> defective. The frame demanded a terminating observation and supplied two
> non-instances of one.

**Where this does not apply:** an AC whose outcome is known and whose job is to
detect drift — no-regression, baseline match, a scope or contention check. Those
need a control, not a terminating observation. Reserve this for the steps that
exist to find something out.

## Authoring acceptance criteria

Load `pin-a-property` (`agent/playbooks/tools/pin-a-property.md`) before
writing or amending an AC. You write the ACs the ring discharges mechanically,
so a badly phrased one costs a review round at build-team scale.

**Name the property first; the artifact is downstream of it.** Three of this
project's framing defects were one shape: a requirement stated in terms of the
artifact most recently looked at — a *population* requirement as a struct
change, an *authority* requirement as a call count, a *module-boundary*
requirement as a spelling class.

- **Require the per-pin evasion attempt in the frame itself.** A per-candidate
  reminder gets satisfied by the most salient control and silently skips the
  rest.

  > **And it must land as an AC, not as a hazards note.** Measured on
  > `RT-FNSPLIT-B2O` while this rule was already written: the requirement went
  > into the issue file under a heading reading *"Standing hazards for whoever
  > builds this."* The implementer ran one evasion attempt of several, and ran
  > the rest only after the same sentence arrived in a message — immediately
  > finding a real overclaim. **The paragraph was not wrong, not unread, and
  > not unclear. It was in a section whose grammatical mood is advice.** ACs
  > get discharged because something checks them; hazards get noted.
  >
  > **A sentence in a frame that tells someone to do something is an AC.** Give
  > it a per-pin enumeration (never "each pin" as a quantifier the reader
  > resolves), a named positive control that would fire if the work were
  > skipped, and a place to record the result per pin. If you cannot name the
  > control, you have stated a hope, not a requirement.
  >
  > **Audit your own frames before release:** read every advisory section and
  > ask which sentences are actually obligations. Those are the unguarded ones
  > by construction.

- **Ask which mechanism already enforces the property** before demanding a
  detector. The compiler is a legitimate answer and usually the strongest;
  never specify a test for something the language already refuses.
- **Give the honest answer a cell.** If an AC list has nowhere to record
  *"guarded by review, not by CI,"* it will be recorded as *"guarded."* State
  every residual arm.
- **When an AC is defeated repeatedly, diagnose before narrowing.** Ask what
  the defeats share; a granularity error is cheap to test and common. **A
  defeat count never licenses "unenforceable"** — that conclusion weakens a
  gate, so it must be demonstrated by building the candidate mechanism and
  showing it cannot work.
- **Narrowing an AC is a frame amendment, and it is yours to author and
  publish.** An amendment that is not on a fetchable ref has not happened; the
  ring reads the frame, not your message.
