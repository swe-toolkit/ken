---
id: LANG-R-LAYER-EXPORT-RETRACTION
title: "Retract ken-elaborator's R layer from the crate's public surface. RExpr, RDecl, RDeclKind, RType, 15 further public types that carry them in their fields, 2 structs in classes.rs and 9 functions are reachable on the public surface via `pub mod resolve`/`elab`/`data`/`classes` (NOT via the `pub use` at lib.rs:104, whose deletion narrows nothing) and measured at ZERO external consumers -- an internal IR that was never meant to be public. The retraction was built and compiles; the only obstruction is that six of the nine functions are consumed by this crate's own INTEGRATION tests, which are external to the lib target and so break under pub(crate). Architect's design ruling: move those tests into the crate as unit tests, NOT a #[doc(hidden)] pub seam."
status: draft
owner: language
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "language-leader, 2026-09-19 (evt_fwp33xw5pnsd), filed to the Steward as a scope overflow from LANG-STANDARD-INFIX-CALL-COMPLETION's A1 design fork and REVERSED BACK OUT on the Architect's call rather than folded into that WP. It surfaced because building A1's D1 completion adapter needed RExpr::RStandardOp to carry data, which needed a type-visibility fix, which cascaded into the whole export surface. A1 is now building a different design that does not touch this. The leader's stated reason for filing is that the measurement should not evaporate, and that is the reason this node exists. Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c before the node was created."
---

# FIRST: THE FILED MECHANISM IS WRONG, AND THE FILED FIX WOULD NOT HAVE WORKED

**Steward, 2026-09-19, re-measured at `origin/main` `d9d8d692f` before creating
this node. The zero-consumers measurement HOLDS. The mechanism does not.**

    FILED       "exported via `pub use resolve::{...}` at lib.rs:114"
    MEASURED    lib.rs:114 is a BLANK LINE.
                The re-export is at :104 and names FOUR items, not 26:
                  pub use resolve::{RDecl, RDeclKind, RExpr, RType};

**And that line is not the mechanism.** `lib.rs:41` declares `pub mod resolve;`.
**So `ken_elaborator::resolve::RExpr` resolves whether or not the `pub use`
exists** — deleting the re-export narrows nothing. The same holds for the rest:

    9 functions   pub mod elab (:21), pub mod resolve (:41), pub mod data (:17)
    2 structs     pub mod classes (:14)  -- InstanceInfo at classes.rs:147,
                  InstanceConstraintInfo at :183

⇒ **The unit of retraction is the `pub mod` declarations, not a re-export
line.** A candidate that deletes `:104` and reports the surface closed would be
green, wrong, and very hard to catch afterwards.

> ### THE POSTURE IS CRATE-WIDE, AND THAT IS RECORDED HERE, NOT FOLDED IN.
>
> **Measured: 34 of `ken-elaborator`'s 36 module declarations are `pub mod`.**
> Only `ast` and `z3_process` are private. **The R layer is one symptom of a
> crate-wide default, not a local slip.**
>
> **This node's scope is the R layer as filed. It is NOT widened to 34 modules**
> — that is a different node, a different size, and the Architect's design
> ruling covers the R-layer test relocation, not a crate-wide audit. It is
> written down because the next reader would otherwise retract four roots,
> measure zero external consumers, and reasonably conclude the surface was
> closed. **Whether the crate-wide posture is worth its own node is a Steward
> call that has NOT been made and must not be inferred from this paragraph.**

# The measurement, and the instrument that makes its zero readable

The resolver's **R layer** is reachable on the crate's public surface:

    RExpr, RDecl, RDeclKind, RType
    + 15 further public types whose FIELDS carry them --
      RCtorDecl, RMatchArm, RSpaceCell, RSpaceOperation, RTelescopeEntry,
      RClassField, RRecordField, RPropIntro, RInstanceConstraint, RPattern, ...
    + 2 structs in classes.rs -- InstanceInfo, InstanceConstraintInfo
    + 9 functions -- elaborate_rdecl, elaborate_rexpr, resolve_decl,
      resolve_decls, resolve_expr_standalone, surface_declared_row_type,
      check_surface_purity, elab_data_decl, elab_explicit_data_decl

**External consumers, measured: ZERO.** No Rust source anywhere outside
`crates/ken-elaborator` touches any of it.

> ### THE ZERO IS READABLE BECAUSE THE INSTRUMENT WAS CONTROLLED. SAY SO.
>
> A search returning nothing is a legal reading on a broken query, and it agrees
> with the hypothesis, so nothing in the output flags it. **The leader ran a
> positive control: `GlobalId` outside `ken-kernel`** — the same instrument, on
> a symbol known to have real external use, returning a large non-zero. **They
> reported 175 files; I measure 174 at `d9d8d692f`.** The one-file difference is
> immaterial to what the control establishes and is recorded rather than
> reconciled — the control's job is to be non-zero, not to be a stable count.
>
> **Re-measured independently here:** `RExpr` outside `crates/ken-elaborator`
> returns 0 files, and `ken_elaborator::resolve` / `ken_elaborator::elab` as
> import paths return 0 hits outside the crate.
>
> ⇒ **The query could have produced a hit and did not.** That is what separates
> this from a false zero, and it is why this node can be sized from the
> measurement instead of re-deriving it. **Whoever takes this re-measures anyway
> (coordinates decay), and re-runs the control when they do.**

# The constraint, and why it is grounded

**Interrogated per `steward.md` §4c before this node was created.** The claim
"an internal IR should never have been public" is an aesthetic preference on
its own. What grounds it is `docs/PRINCIPLES.md:18-45`, the binding
initial-development block:

> *"There is no backwards compatibility to preserve and no existing use to
> protect ... The only thing a parallel old form can buy is compatibility, and
> there is no one to be compatible with ... bought for a beneficiary that does
> not exist."*

**A public API is a compatibility commitment.** Its entire value is to external
consumers, this one has none, and it is therefore paid for by a beneficiary
that does not exist — while constraining every future refactor of the resolver's
internal representation. That is the principle's own reasoning, applied to a
visibility surface rather than to a retained `V1`.

**What does NOT ground it, and must not be written into this node:** this is
**not** a TCB argument. `PRINCIPLES.md` §5 places the elaborator explicitly
*outside* the trust root — it "produces certificates the kernel re-checks."
Retracting these exports does not shrink the TCB and no acceptance criterion
here may claim it does.

# What is already built

**The retraction compiles.** The implementer had it working before the scope
reversal. Everything succeeds except one thing:

    six of the nine functions are consumed by this crate's own INTEGRATION
    tests, which are EXTERNAL to the lib target -- so pub(crate) breaks them.

# The design ruling, already made

**Architect: move those integration tests into the crate as unit tests.**

**Explicitly rejected: a `#[doc(hidden)] pub` seam.** That is a public surface
asked not to look like one — **the same failure shape as the export it would be
covering up**, and it would leave the measured defect in place while removing
the evidence of it.

# Why this is `draft`

**`draft` here is a QUEUED marker, not an unframed one.** The frame is
complete, the measurement is controlled, and the design fork is already ruled;
nothing about it is provisional, and a team pulling it would not find its
premise false.

It is `draft` because of priority, not readiness. L1 (clearing the ignored
tests) is the operator's top priority as of 2026-09-17, and the language lane's
own objective is `LANG-MODULE-IMPORT-SYSTEM`, which unblocks lane 3. **This node
is not released and is not to be started until the Steward releases it**
(`steward.md` §0, filings queue behind the lanes). The leader confirmed it
blocks nothing and that A1's current design does not touch it.

**Tier T2, deliberately.** The work is behaviour-preserving — a visibility
narrowing plus a test relocation, with the design already decided. It does not
need a T1 seat, and standing it on one is the over-provisioning `steward.md`
§4h exists to make visible before the kick rather than after the invoice.

# The one hazard for whoever takes it

**`pub(crate)` is not the only possible answer and the node does not presume
it.** Some of the 26 items may want to be fully private, some `pub(crate)`, and
the 15 field-carrying types are only public *because* they are reachable from
public fields — narrowing the four roots may make several of them fall out
automatically rather than needing individual treatment. **Measure what is still
reachable after the roots are narrowed before hand-editing 26 declarations.**
