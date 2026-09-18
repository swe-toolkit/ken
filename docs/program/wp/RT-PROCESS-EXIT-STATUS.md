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
node governs — **except on the one point in §3 below, which the node gets
wrong and this frame corrects with a measurement.**

## 2. Fixed inputs, measured at `831e521e5`

> **RE-VERIFIED AT `origin/main` `742bf929a` BEFORE RELEASE (Steward,
> 2026-09-18). Every coordinate below still resolves. Do not re-derive this.**
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
      :1058   Lowered::StaticResponseDeferred         => Err(unsupported(
                                                           "StaticResponseDeferred", why))
      :1124   fn boundary_disposition                 encloses the SECOND site
      :1292   LoweredVariant::StaticResponseDeferred  => FailClosedForbidden { why }

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

## 3. `D1.1` IS ALREADY DECIDED, AND THE NODE SAYS IT IS NOT

**The node's `D1` step 1 is "establish which of the two sites fired", on this
stated reason:**

> *"Both carry the same `why` text, so the ledger's signature alone does not
> say which one fired."*

**Both do carry the same `why`. Only one PREPENDS A CONSTRUCT TAG, and the
ledger's recorded signature has the tag.**

    :1058   Err(unsupported("StaticResponseDeferred", why))
            => UnsupportedLowering { construct, reason }
            => Display renders  "{construct}: {reason}"
            => "StaticResponseDeferred: a deferred host response is compiler
                control and can only enter its exact response owner"

    :1292   FailClosedForbidden { why }
            => a BoundaryDisposition variant, destructured `{ .. }` at its use
               sites. NO construct tag, so it cannot render that prefix.

**The ledger's row-13 signature is that string, tag included.** ⇒ **The refusal
came from `:1058`, the `Lowered::StaticResponseDeferred` arm of
`boundary_transfer_admissibility`.** The admissibility walk fired; the
disposition arm did not.

> **Why the node concluded otherwise, because the shape recurs.** It compared
> the two `why` strings, found them identical, and read that as the signature
> being ambiguous. **The signature is not the `why`. It is `construct: why`,
> and the `construct` half is exactly the discriminator.** A comparison on the
> shared substring answered a question about the whole string — and came back
> "indistinguishable", which reads as a finding rather than as a gap.
>
> ⇒ **Confirm this at your base before relying on it** (re-read the `Display`
> impl; a `{construct}` dropped from the format string retires the
> discriminator silently). **If it holds, `D1` step 1 is DISCHARGED BY
> CITATION** — record it and spend the turn on step 2, which is the real work.
> **If it does not hold, that is a finding: say so and do the enumeration.**

## 4. Deliverables

**`D0` — re-establish the premise at YOUR base. Run it first.** The node's
three outcomes `(a)`/`(b)`/`(c)` are unchanged and all three terminate.

> **`(c)` — the row passes — is LIVE, not a formality.**
> `[[RT-COMPOSED-RETURN-SSA-SPECIALIZATION]]` is **`merged`** (squash
> `ad9905a7e`, 2026-09-03) and reworked this exact mechanism; the ledger row
> was measured 2026-09-17, two weeks later, so a surviving refusal is not a
> pre-recut artefact. **But do not convert `(c)` into a hunt for a way to make
> it fail.** Report it and propose un-ignoring.

**`D1` — step 1 discharged per §3; do step 2.** Apply the landed
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

**AC-1 — the firing site is named with its evidence.** Either §3's tag argument
is confirmed and cited, or it is refuted and the enumeration is done.
**Control:** an answer resting on the `why` text alone fails this AC in either
direction — the `why` is shared and cannot discriminate.

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
