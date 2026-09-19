---
id: LANG-R-LAYER-EXPORT-RETRACTION
title: "Retract ken-elaborator's R layer from the crate's public surface. RExpr, RDecl, RDeclKind, RType, 15 further public types that carry them in their fields, 2 structs in classes.rs and 9 functions are public as ITEMS (the unit of retraction is item visibility -- `pub(crate)` on the items closes the path with the `pub mod` declarations untouched; deleting the `pub use` re-export narrows nothing) and measured at ZERO external consumers -- an internal IR that was never meant to be public. The retraction was built and compiles; the only obstruction is that six of the nine functions are consumed by this crate's own INTEGRATION tests, which are external to the lib target and so break under pub(crate). Architect's design ruling: move those tests into the crate as unit tests, NOT a #[doc(hidden)] pub seam."
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

# FIRST: THE UNIT OF RETRACTION IS THE ITEM'S VISIBILITY, NOT THE RE-EXPORT

**EVERY COORDINATE IN THIS NODE CARRIES ITS TREE. Read no line number here
without the SHA beside it** — this file's coordinates moved twice in one hour
while it was being written, and two seats disagreed about them while both were
correct.

    main d9d8d692f              branch a8f0873b0 (A1, 7 commits above base)
      :41  pub mod resolve;       :50  pub mod resolve;
      :104 pub use resolve::{..}  :114 pub use resolve::{..}
      (:114 is blank)             (:104 is mid-import-list)

The re-export names **four** items on both trees — `RDecl, RDeclKind, RExpr,
RType` — not 26. **The "26 items at lib.rs:114" reading is a compression that
happened in relay**: the Architect measured at `a8f0873b0` and said so; the
filing that reached the Steward carried the coordinate without the tree.

## What holds: deleting the `pub use` narrows NOTHING

`pub mod resolve;` plus a `pub` item means `ken_elaborator::resolve::RExpr`
resolves with or without the re-export. **So a candidate that deletes the
re-export, re-runs the consumer census, gets zero, and reports the surface
closed would be GREEN AND WRONG.**

⇒ **The census cannot distinguish "I closed it" from "nobody was using it
anyway." It returns zero in both worlds.** That trap survives review because
every number in it is correct, and avoiding it is the point of this section.

## What the unit actually is: ITEM visibility

**`pub(crate) enum RExpr` inside `pub mod resolve;` is not nameable outside the
crate.** A public enclosing module does not re-export a crate-private item;
effective visibility is the item's, not the path's. **The `pub use` deletion is
a CONSEQUENCE, not the mechanism — you cannot re-export an item you have just
made crate-private.**

> ### A STEWARD CLAIM WAS WRONG HERE AND THE FALSIFIER WAS ALREADY BUILT.
>
> An earlier revision of this node said *"the unit of retraction is the `pub
> mod` declarations."* **It is not.** That reading points an implementer at 34
> module declarations — a crate-wide restructure — when the same retraction is
> reachable as `pub(crate)` on roughly thirty items across two files, which is
> what was already built and compiled.
>
> **The measurement that settles it, from the implementer's own (i) tree:** they
> set the four types `pub(crate)`, left `pub mod resolve;` untouched, and the
> compiler produced a cascade — 19 further public types, 2 structs in
> `classes.rs`, 9 public functions, each now exposing a crate-private type.
>
>     If pub(crate) did NOT narrow inside a pub mod, that build emits ZERO
>     new errors. It emitted about thirty. The cascade IS the compiler
>     demonstrating that the item is the operative unit.
>
> Architect, `evt_jyxbsc56dgvn`, contradicting the Steward directly rather than
> hedging, on the grounds that the falsifier was on the table. Correct, and the
> scope consequence is the reason it mattered: **this is how a 5x scope growth
> re-enters through a successor node's own text after being kept out of the
> predecessor.**

> ### THE POSTURE IS CRATE-WIDE. RECORDED, NOT FOLDED IN, AND NOT A PRECONDITION.
>
> **Measured at `d9d8d692f`: 34 of `ken-elaborator`'s 36 module declarations are
> `pub mod`**, with only `ast` and `z3_process` private. (The A1 branch reads 34
> public and 3 private — it adds `mod standard_operators;`, the one module this
> ring deliberately made private, which is how the whole question arose.)
>
> **The R layer is one symptom of a crate-wide default, not a local slip.**
>
> **This node's scope is the R layer as filed. It is NOT widened to 34 modules.**
> It is recorded because a reader who narrows four roots and measures zero would
> otherwise conclude the surface was closed. **And under item visibility the two
> are INDEPENDENT — the posture question is not a precondition for this
> retraction and blocks nothing.** Whether it earns its own node is a Steward
> call that has NOT been made and must not be inferred from this paragraph.

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
