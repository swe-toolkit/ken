# RT-PROCESS-EXIT-STATUS — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` at
`831e521e5187e28cdd7c24668bbafbaa0eec3e8d`.**

Operator priority, 2026-09-15: *"The other tests should be fixed."* Operator,
2026-09-17: *"Is L1 still working on clearing the ignored tests? That is the top
priority until it is done."* Covers ledger row **13**.

> **Every coordinate, count and status here is PERISHABLE and was measured at
> the base above.** Re-ground before acting. If a re-measurement disagrees, the
> tree wins and the disagreement is a finding, not a discrepancy to reconcile.

## 1. Objective

`docs/program/issues/RT-PROCESS-EXIT-STATUS.md` carries the measurement, the
superseded signature, the `D0`/`D1`/`D2` structure, the `D3` hard stop and the
bans. **Read it first; this frame does not restate it.** Where they differ, the
node governs — **with no exception. An earlier cut of this frame carved one out
in §3, claiming the node was wrong about `D1` step 1. The node was right and
this frame was wrong; see §3.**

## 2. Fixed inputs, measured at `831e521e5`

> **RE-VERIFIED AT `origin/main` `742bf929a` BEFORE RELEASE, AND AGAIN AT
> `70fe4beb5` AFTER (Steward, 2026-09-18).**
>
> **THE FIRST PASS'S CLAIM — *"every coordinate below still resolves"* — WAS
> FALSE, and the way it was false is the lesson.** `boundary.rs`'s two
> coordinates did not resolve, and had never resolved. The second pass caught
> it because it checked every coordinate BY TOKEN instead of stopping at the
> blob.
>
>     rt_escape   blob CHANGED  => forced me to look   => coordinates CORRECT
>     boundary    blob IDENTICAL => I stopped there    => coordinates WRONG
>     surface     blob IDENTICAL => I stopped there    => coordinates correct
>
> ⇒ **Blob identity proves the FILE did not change. It says nothing about
> whether my coordinates were ever right.** The file that changed got a token
> check and came out clean; the files that looked safe kept their errors,
> because "unchanged" was read as "verified". **A staleness instrument cannot
> answer a correctness question, and the files it clears are the ones nobody
> re-reads.**
>
> The frame sat unlanded while `main` advanced, so its inputs were re-measured
> rather than assumed:
>
>     boundary.rs       blob IDENTICAL at 831e521e5 and 742bf929a
>     surface.rs        blob IDENTICAL
>     rt_escape_second_resource_native.rs   blob CHANGED -- see below
>
> **The changed file does NOT move this frame, and the distinction is the
> useful part.** `589926845` re-attributed two `#[ignore]` strings in it, at
> `:653` and `:713`. Both are single-line replacements (`+1/-1` each), so
> **nothing below them shifts**, and neither row is this frame's. Confirmed at
> the token rather than inferred from the arithmetic: `:776` on `742bf929a` is
> `#[ignore = "RT-PROCESS-EXIT-STATUS: ..."` and `:777` is
> `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds`.
>
> ⇒ **A file-level blob comparison is a COARSE staleness instrument and it
> fires here as a false positive.** "The file changed" and "my coordinates
> moved" are different claims; the first is cheap and the second is the one
> that matters. Resolve the token before believing a blob diff has de-aimed
> you — and before paying to re-ground a frame that is fine.

**All verified by grepping for the token, not by reading a window.**

    crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs
      :1031   fn boundary_transfer_admissibility      encloses the FIRST site
      :1056   Lowered::StaticResponseDeferred         => Err(unsupported(
                                                           "StaticResponseDeferred", why))
      :1124   fn boundary_disposition                 encloses the SECOND site
      :1291   LoweredVariant::StaticResponseDeferred  => FailClosedForbidden { why }

> **`:1056` and `:1291` were CORRECTED 2026-09-18 from `:1058` and `:1292`.
> Nothing drifted — the blob is `da6211689` at `831e521e5` and at `main` alike,
> and the old numbers do not resolve at the frame's OWN base either.** They
> labelled each arm with a line inside it rather than the line it starts on.
>
> **The two wrong coordinates were exactly the two §3's argument rests on**,
> and everything else in this section — `:1031`, `:1124`, `surface.rs:485`,
> `surface.rs:253`, `rt_escape:755`, `:764`, `:776`, `:777` — verified exact at
> `main`. A frame's load-bearing citations are not the ones most likely to be
> right; they are the ones most often re-typed from prose while the incidental
> ones get pasted from a tool.
>
> ⇒ **Resolve these by the token, as this section already instructs.** The
> distinction matters beyond the typo: a DRIFTED coordinate says re-measure at
> your base, a WRONG one says the author's own base never supported it, so
> nothing resting on it is safe on its say-so. Here the CONTENT claim holds —
> the arm really is `Err(unsupported("StaticResponseDeferred", why))`.
>
> **THAT CLEARED THE COORDINATE AXIS ONLY, AND THE SENTENCE THAT USED TO SIT
> HERE — *"so §3 survives intact and only the numbers were off"* — OVERSOLD
> IT.** §3's argument was refuted hours later, on a different axis entirely:
> the tag it turned on is attached by the CONSUMER, so both arms render the
> same text (§3.1). **Both citations were right and the inference drawn from
> them was wrong.** Checking that a claim's coordinates resolve is not checking
> the claim; the clearance a re-measurement issues reaches exactly as far as
> what it measured.

    crates/ken-runtime/src/cranelift_backend/surface.rs
      :485    fn unsupported(construct, reason)       builds UnsupportedLowering
      :253    impl Display for UnsupportedLowering    write!("{}: {}",
                                                        construct, reason)

    crates/ken-cli/tests/rt_escape_second_resource_native.rs
      :755    the "Observed signature, exactly" block  carries the SUPERSEDED
                                                       ProcessExitStatus text
      :764    "It refuses at object emission, ..."     the clause to CHECK
      :776    #[ignore = "RT-PROCESS-EXIT-STATUS: ..."]  stale record #2
      :777    fn r2_cross_buffer_freeze_fails_closed_with_invalid_bounds

## 3. `D1.1` IS UNDECIDED. THIS FRAME PREVIOUSLY CLAIMED OTHERWISE AND WAS WRONG.

**The node's `D1` step 1 — "establish which of the two sites fired" — STANDS,
and it is real work.** An earlier cut of this section claimed it was
*"DISCHARGED BY CITATION"* on a construct-tag argument. **That argument is
refuted** (Architect, `evt_398hkbskdt1mx`; independently verified by the
Steward at the objects before it was acted on).

### 3.1 Why the tag does not discriminate

The refuted rule was: `:1056` prepends a construct tag and `:1291`
`FailClosedForbidden { why }` cannot, so a tagged signature means the
admissibility walk fired. **The tag is not attached at the disposition site. It
is attached by the CONSUMER, and every consumer attaches it:**

    aggregates.rs:1533, :2240, :2275, :2312   all four, identically
      BoundaryDisposition::FailClosedForbidden { why }
        => Err(unsupported(lowered_value_kind(v), why))

    mod.rs:13509
      Lowered::StaticResponseDeferred => "StaticResponseDeferred"

    surface.rs:219
      CraneliftBackendError::Unsupported(err)
        => write!(f, "unsupported runtime-IR lowering: {err}")

⇒ **The `:1291` path renders BYTE-IDENTICALLY to the `:1056` path** — same
wrapper, same tag, same `why`. **Nothing in the rendered message separates the
two sites.**

Name the enclosing items rather than the line numbers, which drift:

    :1056   boundary_transfer_admissibility   (:1031)   the admissibility walk
    :1291   boundary_disposition              (:1124)   the disposition table

> **`"runtime-IR lowering"` is NOT a third site.** It is the enclosing wrapper
> on every `Unsupported` rendering in the tree, which is why other rows record
> `BoundaryCarrier:`, `ContinuationSpecialization:`, `Closure:` and `Effect:`
> in that same slot. A signature decomposes as **wrapper + construct tag +
> `why`**. Reading the wrapper as a site name is the mistake that made this
> look like a three-way question; it is two-way.

### 3.2 THE QUESTION IS REOPENED, NOT ANSWERED THE OTHER WAY

**What was shown is that `:1291` CAN render identically. Nobody has shown this
row went through it.** Concluding "the disposition table" is the same
unsupported inference running backwards, and it is the sentence a reader in a
hurry will drop. **Neither site is established. Do not write either one into a
deliverable, a commit message or a node title until the probe below has run.**

### 3.3 THE INSTRUMENT: make the two paths differ in TEXT

The paths differ in call stack, not in rendered text, so the probe makes them
differ in text. **Its ending is stated before the run, and both endings are
reachable:**

    ACT       add a one-word marker to the `why` literal at boundary.rs:1058
              ONLY. Leave :1292 untouched.
    OBSERVE   re-run the row and read the signature CAPTURED FROM THE HELPER
              THREAD. The test thread's `unwrap()` wrapper measures nothing --
              the row's own comment says so.
    ENDS      marker present  => :1056, the admissibility walk
              marker absent   => :1291, the disposition table
    THEN      REVERT the marker.

**This is a throwaway probe. It must not ride in a candidate**, and the revert
is part of the deliverable, not a tidy-up afterwards.

### 3.4 The probe's two blindness hazards were checked BEFORE it runs

Both come back clean, so the instrument is sound in **both** directions
(Architect, `evt_35mdm2f7bmc02`):

**A shared constant would have made "marker present" vacuous.** If both sites
read one `const`, marking "the literal at `:1058`" marks both, the marker
appears whichever path fired, and the probe reports "shared site"
unconditionally — the always-green failure, invisible from the result. They are
**two independently written literals**, byte-identical in text, at separate
sites (`boundary.rs:1058` and `:1292`). Editing `:1058` does not touch `:1292`.
**Marker present is a real positive, not a tautology.**

**An open roster would have made "marker absent" ambiguous.** *Absent ⇒ `:1291`*
holds only if `:1291` is the sole OTHER way that sentence can reach the
signature; with a third emitter anywhere, absent would mean "not `:1056`" and
nothing more — a roster read as closed while it is open, which is the direction
that fails open. Census over the tree, keyed on the **sentence** rather than on
either site's name:

    grep -rn "can only enter its exact response owner" crates/ --include=*.rs
    boundary.rs:1058
    boundary.rs:1292
    (no others)

Population is exactly two. ⇒ **Absent ⇒ `:1291` is an ENTAILMENT, not an
inference**, and a negative result licenses naming the disposition table rather
than merely excluding the walk.

> **The bound on that census, so it is not read wider than it is.** It
> quantifies over **source text** in `crates/`, so it is blind to a sentence
> assembled at runtime from fragments or produced by a macro body; neither was
> looked for. It is a strong negative on the ordinary case and **not a proof of
> absence** — the same standing as any grep. **The probe is the oracle; this is
> only its precondition check.**

## 4. Deliverables

**`D0` — re-establish the premise at YOUR base. Run it first.** The node's
three outcomes `(a)`/`(b)`/`(c)` are unchanged and all three terminate.

> **`(c)` — the row passes — is LIVE, not a formality.**
> `[[RT-COMPOSED-RETURN-SSA-SPECIALIZATION]]` is **`merged`** (squash
> `ad9905a7e`, 2026-09-03) and reworked this exact mechanism; the ledger row
> was measured 2026-09-17, two weeks later, so a surviving refusal is not a
> pre-recut artefact. **But do not convert `(c)` into a hunt for a way to make
> it fail.** Report it and propose un-ignoring.

**`D1` — step 1 is NOT discharged. RUN THE §3.3 PROBE, then do step 2.** An
earlier cut of this frame discharged step 1 by citation; that argument is
refuted (§3.1) and the site is undecided in both directions (§3.2). The probe
is cheap and terminates. **Then** apply the landed
caller-consumption discriminator to this row's response:

    Deferred = P1 UNION P2      P1  absent-residual
                                P2  present-but-unconsumed placeholder
    discriminator = CALLER-CONSUMPTION
      Specialized  IFF  specializable AND the caller consumed it

    (i)  CONSUMED and specializable => the P2 leak. It should have classified
         Specialized and never reached the boundary. THE REPAIR IS IN CLASSIFY,
         NOT AT THE BOUNDARY. Attempt this first.
    (ii) NOT consumed => the Deferred is correct, the fail-closed is doing its
         job, and the defect is in the FIXTURE. Say plainly that the compiler
         was right.

**Do not re-derive that contract — it is landed. Fold and cite it.**

**`D2` — correct BOTH stale records in the same change**, `:755`'s block and
`:776`'s reason string, to what is observed rather than to this node's name,
and record the SHA measured at.

## 5. THE OBJECT-EMISSION CLAUSE: THIS FILE HAS ALREADY REFUTED IT ONCE

The node says the clause at `:764` — *"It refuses at object emission, so the
program never executes"* — is *"probably still true and must not be deleted
reflexively."* **Treat that as weaker than it sounds. Measured in this file:**

    :554   the SAME sentence, recorded REFUTED -- "the program emits,
           executes, exits 0", refuted POSITIVELY by an effect trace, on a
           row readmitted under RT-IGNORED-PASSING-ROWS
    :629   live, above the #[ignore] at :653
    :674   live, above the #[ignore] at :684
    :704   live, above the #[ignore] at :713
    :764   live, above THIS row's #[ignore] at :776

⇒ **Four live copies of a sentence this same file has already shown to be
false once, positively, about one of its own rows.** It is a template that was
pasted, not four independent observations. **Check it against your `D0` run and
say which; do not carry it forward because it is plausible.**

## 6. Acceptance criteria

**AC-0 — `D0` is run at YOUR named base**, the verbatim refusal is pasted into
the handback, and the base SHA is stated. **A `(b)` or `(c)` outcome discharges
this node's measurement obligation and STOPS the work — that is a pass, not a
failure.**

**AC-1 — the firing site is named with the PROBE's evidence, or reported
undecided.** Run §3.3 and report the signature **as captured from the helper
thread**, plus which ending it hit. **Control: an answer resting on the
rendered message alone fails this AC in either direction** — the two sites
render byte-identically (§3.1), so any argument from the signature's text is
refuted before it is made, including one that concludes `:1291`.

**A reverted probe is a passing probe.** Verify the marker is gone: the diff
handed back must contain no edit to `boundary.rs`.

**If the probe cannot be run** — the row will not reach the refusal at your
base, or the helper-thread signature cannot be captured — **that is a hard
stop to report, not a licence to fall back on the text.** Say which of the two
it was.

**AC-2 — the caller-consumption verdict is stated with its evidence**, not its
expectation.

**AC-3 — under `(i)`, the diff touches NO fail-closed arm in `boundary.rs`.**
If it does, that is `D3`'s hard stop to the Architect, not a judgment call.

**AC-4 — under `(ii)`, one sentence stating the compiler's refusal was
correct**, written where the row is, so the same shape is not re-filed as a
compiler defect.

**AC-5 — both stale records corrected, each carrying its measured-at SHA.** A
grep for `Persistent referent lifetime` and for `planned NoReferent` returns
**zero live hits** outside a block explicitly labelled superseded.

**AC-6 — the row is DIFFERENTIAL.** If un-ignored,
`assert_native_matches_interpreter` **and** both per-engine `InvalidBounds`
assertions pass. **A native-only green does not satisfy this**, and the row's
name advertises none of the four differential patterns — the ledger caught that
by body after a name-pattern pass had excluded it.

**AC-7 — the `:764` clause is dispositioned**, per §5.

**AC-8 — no-regression, green in CI.** `COORDINATION §12`: targeted local runs
only (`scripts/ken-cargo`, `--test rt_escape_second_resource_native`), **never
`--workspace`**.

## 7. What must not happen

- **Do not rename the `id`.** It is cited elsewhere; the title carries the
  correction.
- **Do not repair at the boundary because that is where the message came
  from.** The message names where the value was *caught*, not where it was
  *made*. Under `(i)` the defect is upstream in classify.
- **Do not inherit the "fits no released owner" judgment** at `:767-769`. It
  was true of the superseded signature and is false of this one — the Deferred
  mechanism has a merged owner.
- **Do not touch the other three rows in this file.** See §8.

## 8. Contention — ONE FILE, FOUR IGNORED ROWS, THREE OWNING NODES

**This is the sharpest contention in the row-clearing program and it is not
visible from the node.** Measured at this base:

    crates/ken-cli/tests/rt_escape_second_resource_native.rs
      :653   RT-HOST-RESPONSE-ROUTE-KEY-COLLISION   readmits on the node IN FLIGHT
      :684   RT-SITEOP-CARRIED-WITNESS D2           ledger row 10, owned by
                                                    RT-SITEOP-RETAINED-ROWS-...
      :713   RT-HOST-RESPONSE-ROUTE-KEY-COLLISION   readmits on the node IN FLIGHT
      :776   RT-PROCESS-EXIT-STATUS                 THIS ROW, ledger row 13

- **`RT-DUPLICATED-RESPONSE-BLOCK` is in flight and edits `:653` and `:713`**
  in this file. **Its candidate's entire `crates/` content is `#[ignore]`
  attribute lines**, so a concurrent edit here is a direct line-level conflict
  risk on one file.
- **`RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL` reads `:684`** and writes
  `docs/program/` only. **Reads do not contend.**

⇒ **Check with the Steward before starting.** This WP touches exactly one line
of this file (`:776`) plus the block above it; that is small enough to sequence
rather than to race. **Do not edit any `#[ignore]` in this file other than
`:776`.**

**`boundary.rs` is clean** — no other live node names it.
