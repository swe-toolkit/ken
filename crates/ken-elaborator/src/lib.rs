//! `ken-elaborator` — V0/V1/L1 surface elaborator (`docs/program/wp/V0-elaborator.md`,
//! `spec/20-verification/21-spec-syntax.md`, `spec/30-surface/35-numbers.md`).
//!
//! Pipeline: `lex → parse → resolve → elaborate → kernel-check`.
//!
//! V1 extensions: requires/ensures, obligation holes, honesty guard.
//! L1 extensions: numeric tower, literal defaulting, overflow obligations.
//! Clean-room: built from `/spec` and `/conformance` only.

// `standard_operators::StandardOperatorRole` is crate-internal BY CONTRACT,
// not by convention: the membership track widens it, and that widening is an
// ordinary internal edit only for as long as the type never reaches this
// crate's public surface. `private_interfaces` is the compiler check that
// says so -- a `pub fn` taking or returning the role type, or a `pub` field
// holding one, becomes a BUILD ERROR here rather than a silent escape that
// the widening seat discovers later.
#![deny(private_interfaces)]

mod ast;
pub mod bytes;
pub mod capabilities;
pub mod checked_core;
pub mod classes;
pub mod compiler_driver;
pub mod conversions;
pub mod data;
pub mod decimal_char;
pub mod diagnostics;
pub mod effects;
pub mod elab;
pub mod erasure;
pub mod error;
pub mod export;
pub mod extract;
pub mod fo_kripke;
pub mod foreign;
pub mod format;
pub mod ifc;
pub mod layout;
pub mod lexer;
pub mod literate;
pub mod lossless;
pub mod modules;
pub mod numbers;
pub mod parser;
pub mod prelude;
pub mod program_admission;
pub mod protocol;
pub mod prover;
pub mod resolve;
mod standard_operators;
pub mod strings;
pub mod temporal;
pub mod trace;
#[cfg(feature = "z3-process")]
mod z3_process;

#[cfg(test)]
extern crate self as ken_elaborator;
#[cfg(test)]
mod env_mark_rollback_tests;
#[cfg(test)]
mod r_layer_tests;
#[cfg(test)]
mod seal2_tests;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

use ken_kernel::{
    check as kernel_check, declare_postulate, env_mark, rollback_to_mark, Context,
    Decl as KernelDecl, EnvMark, GlobalEnv, GlobalId, Term,
};

pub use ast::{
    BinOp, BoundaryHeader, BoundaryKind, CapabilityDecl, ConstructorSignature,
    ConstructorSignatureArg, Decl, ExplicitDataCtor, ExportForm, Expr, ImportItem, ImportKind,
    LetBinding, RecursiveResultSelector, SpaceCell, SpaceOperation, Type,
};
pub use bytes::BytesEnv;
pub use classes::{ClassEnv, ClassInfo, ClassKind, ClassView, InstanceResolution, ProjectionView};
pub use diagnostics::{
    project_all, project_diagnostic, tv_and, tv_not, tv_or, tv_strict, Diagnostic, DiagnosticTag,
    FailureWitness, FormRef, HoleId, KripkeCountermodel, Region, SuggestedAction, ThirdValue,
    TypedHole, WorldId,
};
pub use elab::{ElabResult, Obligation, ObligationKind};
pub use error::{ArmDeadCause, ElabError, MissingPatternWitness, Span};
pub use export::{
    canonical_host_perform_signature_v1, canonical_l5_perform_signature_v1,
    canonical_perform_node_signature_v1, emit_checked_target_export, serialize_export,
    try_serialize_export, BehavioralExport, ExportError, GEntry, PEntry, PStatus,
    PerformNodeInventoryV1, PerformNodeSignatureV1, QEntry, ResourceLifetimeBindingPoint,
    ResourceLifetimeCorrelation, ResourceLifetimeObligation, ResourceLifetimePlan, TEntry,
    WardResourceLifetimeMonitor,
};
pub use extract::{
    render_open_obligations, v2_extract, ExtractionResult, ObligationId, ObligationTriple,
    ProvKind, Provenance,
};
pub use foreign::{
    trusted_base_delta, FfiRuntimeCheck, ForeignBinding, ForeignEnv, MarshalKind, MarshalSig,
};
pub use literate::{
    extract_ken_md, format_ken_md, validate_ken_md_fences, KenMdExtraction, KenMdFence,
    KenMdFenceRole,
};
pub use numbers::{int_lit_val, NumericEnv, NumericLitVal};
pub use prelude::PreludeEnv;
pub use protocol::{
    deserialize_atom_id, deserialize_formula_path, hole_id_string, obligation_id_string,
    project_obligation_status, project_wire_verdict, rollup_doc_status, round_trip,
    serialize_action, serialize_atom_id, serialize_countermodel, serialize_decomposition,
    serialize_diagnostic, serialize_document, serialize_formula_path, serialize_hole,
    serialize_obligation, serialize_slice, trusted_base_entry, validate_document, DocStatus,
    ObligationStatus, WireVerdict,
};
pub use prover::{
    attempt_d_with_int_assignment, attempt_obligation, attempt_obligation_with_catalog_handles,
    attempt_with_cert, classify, Countermodel, FormulaPath, FormulaStep, ProverResult, Route,
    StructuralRefutation, Verdict,
};
#[cfg(feature = "z3-process")]
pub use prover::{attempt_d_with_z3_process, Z3ProcessConfig};
pub use strings::NfcString;
pub use temporal::{
    closed, elaborate_temporal_expr, temporal_hoas_inductive_spec, temporal_inductive_spec, Pred,
    Temporal, TemporalExpr, TemporalObligation, Var,
};
pub use trace::{
    emit_trace_contract, serialize_trace_contract, try_emit_trace_contract, AssertionPoint,
    MonitorProjection, TraceContract, TraceContractError, TraceEvent,
};

/// Internal identities generated while elaborating surface `space` blocks.
///
/// The container is visible so structural tests can classify every `ElabEnv`
/// namespace, but its maps are private and cannot act as source namespaces.
#[derive(Default)]
pub struct SpaceElaborationMetadata {
    initial_states: HashMap<String, GlobalId>,
}

/// The surface-level elaboration environment.
pub struct ElabEnv {
    pub env: GlobalEnv,
    pub globals: HashMap<String, GlobalId>,
    /// Elaborator-side ownership of kernel declarations without a source name.
    /// A qualified declaration or fixed prelude stage owns each allocation.
    pub(crate) decl_owner: BTreeMap<GlobalId, (String, u32)>,
    owner_ordinals: HashMap<String, u32>,
    /// Caller-visible `requires` arities keyed by checked declaration identity.
    pub(crate) preconditions: HashMap<GlobalId, (usize, usize)>,
    /// Numeric literal values keyed by their opaque-postulate GlobalId.
    /// Accumulated during elaboration; copied to `EvalStore.num_values` for eval.
    pub num_values: HashMap<GlobalId, NumericLitVal>,
    /// Declared user-operator fixities keyed only by canonical kernel identity.
    /// A missing entry means the normative default `infixl 9`.
    pub fixities: HashMap<GlobalId, ast::Fixity>,
    /// First declaration site for the conflict diagnostic. This parallel
    /// attribution map is not consulted for association. It is public so
    /// structural tests can exhaustively classify every `ElabEnv` namespace.
    pub fixity_spans: HashMap<GlobalId, Span>,
    /// First declaration site of each constructor spelling, for the
    /// duplicate-constructor-spelling diagnostic
    /// (`LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD`). Populated when a
    /// constructor is inserted into `globals`; consulted before a colliding
    /// insert to report the earlier site. Public so structural tests can
    /// classify every `ElabEnv` namespace.
    pub ctor_decl_spans: HashMap<String, Span>,
    /// The numeric tower (registered op ids, dispatch tables).
    pub numeric_env: NumericEnv,

    /// The standard-operator identities layer 3 certified against the
    /// standard-operator home's export table (`33 §6.1`, `39 §6.9`).
    ///
    /// Empty when the program does not provide the home. **An occurrence is
    /// then NOT refused naming the role, and this comment used to say it
    /// was.** An empty map certifies nothing, so each occurrence is left as an
    /// ordinary under-applied application and caught downstream by the kernel
    /// check; the residual on `reduce_resolved_operator`'s non-certified arm
    /// carries the detail.
    ///
    /// **Not `pub`**: the role type is crate-internal by contract, and
    /// `#![deny(private_interfaces)]` above is what holds that rather than
    /// this comment.
    pub(crate) standard_operators:
        HashMap<standard_operators::StandardOperatorRole, GlobalId>,
    /// The Bytes layer (L6): type ids, I/O effect row registry (`38 §1`, `41`).
    pub bytes_env: BytesEnv,
    /// The foreign FFI layer (L7): binding registry (`38 §2–§4`).
    pub foreign_env: ForeignEnv,
    /// Surface effect rows for already-elaborated definitions. SURF-1 D2 uses
    /// this to release a callee's declared row at a resolved call site.
    pub effect_rows: HashMap<String, effects::RowType>,
    /// Immutable row evidence for checked imported identities. The spelling
    /// table above may later be overwritten by another provider with the
    /// same canonical name; purity inference must use the selected ID.
    pub(crate) effect_rows_by_id: HashMap<ken_kernel::GlobalId, effects::RowType>,
    /// Generated initial-state definitions for surface `space` blocks.
    ///
    /// These are elaboration metadata, not source-visible `Space.initial`
    /// members. Tests and later lowering stages may inspect them through
    /// `space_initial_state`; source name resolution may not.
    pub space_metadata: SpaceElaborationMetadata,
    /// The L3 prelude: collection inductives + Ω constants (`37`).
    pub prelude_env: PreludeEnv,
    /// The Lc typeclass environment: class/instance registry + structural
    /// postulates (`RecordNil`, `record_nil_val`). Initialized in `empty()`.
    pub class_env: ClassEnv,
    /// Successful implicit-resolution provenance, in source order.
    ///
    /// **A SIBLING OF `class_env`, not a field of it, and the split is the
    /// point.** `33 §6.2` instance search is a lookup against a registry fixed
    /// before any body elaborates, so the resolution DECISION is a pure
    /// function of `&ClassEnv`. This is the one thing the resolver writes --
    /// an append-only OUTPUT with zero production readers -- and while it
    /// lived inside `ClassEnv` the whole registry had to be borrowed mutably
    /// to record it, which put dictionary resolution out of reach of an
    /// expression site.
    ///
    /// Keeping it inside and borrowing the field alone does not work: `&*env`
    /// and `&mut env.field` overlap as places. Two fields of `ElabEnv` are
    /// disjoint by direct projection, which is why the SINK moves rather than
    /// the borrow (Architect, `evt_qgdv6h7s3fyn`).
    ///
    /// **ONE OWNER, APPENDED IN ELABORATION ORDER.** `classes.rs` documented
    /// "in source order" and tests index `resolutions[0]`/`[1]`, so this must
    /// not become per-declaration.
    pub resolution_provenance: Vec<classes::InstanceResolution>,
    /// Module/import/visibility bookkeeping (`33 §3-4`, ES3-build) —
    /// persists the file-level (root) import scope and every elaborated
    /// module's `pub` export table across separate `elaborate_*` calls.
    /// Purely a surface-layer concern: never touches `env`/`Σ`.
    pub module_state: modules::ModuleState,
    /// Identity-keyed, elaborator-only refinement introduction contracts.
    pub(crate) refinement_facts: elab::RefinementFacts,
}

/// Compile-time inventory tripwire for every field carried by [`ElabEnv`].
///
/// This function only acknowledges that a field exists. It does not classify
/// any namespace and therefore does not establish that the SEAL-2 producer walk
/// is complete. `seal2_tests::support::enumerate_producer_types` remains the
/// obligation that performs that classification.
#[allow(dead_code)]
fn acknowledge_elab_env_field_inventory(env: &ElabEnv) {
    let ElabEnv {
        env: _,
        globals: _,
        decl_owner: _,
        owner_ordinals: _,
        preconditions: _,
        num_values: _,
        fixities: _,
        fixity_spans: _,
        ctor_decl_spans: _,
        numeric_env: _,
        standard_operators: _,
        bytes_env: _,
        foreign_env: _,
        effect_rows: _,
        effect_rows_by_id: _,
        space_metadata: _,
        prelude_env: _,
        class_env: _,
        resolution_provenance: _,
        module_state: _,
        refinement_facts: _,
    } = env;
}

// GlobalEnv appends declarations in dependency order (`decls()`), and mark
// rollback removes a suffix. Count the live suffix rather than numeric IDs:
// constructor IDs share the allocator but are not standalone Decl entries.
fn record_decl_owners(
    env: &GlobalEnv,
    before: usize,
    owner: &str,
    owners: &mut BTreeMap<GlobalId, (String, u32)>,
    ordinals: &mut HashMap<String, u32>,
) {
    for decl in env.decls().skip(before) {
        if owners.contains_key(&decl.id()) {
            continue;
        }
        let ordinal = ordinals.entry(owner.to_owned()).or_insert(0);
        owners.insert(decl.id(), (owner.to_owned(), *ordinal));
        *ordinal = ordinal.checked_add(1).expect("owner ordinal exhausted");
    }
}

impl ElabEnv {
    pub fn empty() -> Result<Self, ElabError> {
        let mut env = GlobalEnv::new();
        let mut globals = HashMap::new();
        let mut ctor_decl_spans = HashMap::new();
        let mut decl_owner = BTreeMap::new();
        let mut owner_ordinals = HashMap::new();
        // `Bool` is pre-registered here (real `data Bool = True | False`, ES2 —
        // demotes the former opaque `declare_postulate` so `Bool` is
        // matchable data; `reg_ty!("Bool")` in `register_numeric_env` reuses
        // this GlobalId) so downstream code using `ElabEnv::empty` gets a
        // consistent GlobalId. Declared via the raw inductive machinery
        // (`data.rs::elab_data_decl`, not `elaborate_decl`) since the full
        // `ElabEnv` doesn't exist yet at this point in construction.
        let true_ctor = resolve::RCtorDecl {
            name: "True".into(),
            args: vec![],
            field_labels: None,
            span: Span::zero(),
        };
        let false_ctor = resolve::RCtorDecl {
            name: "False".into(),
            args: vec![],
            field_labels: None,
            span: Span::zero(),
        };
        data::elab_data_decl(
            &mut env,
            &mut globals,
            &mut ctor_decl_spans,
            "Bool",
            &[],
            &[true_ctor, false_ctor],
            &Span::zero(),
        )?;
        record_decl_owners(
            &env,
            0,
            "prelude::bool",
            &mut decl_owner,
            &mut owner_ordinals,
        );
        let before_numeric = env.decls().count();
        let numeric_env = numbers::register_numeric_env(&mut env, &mut globals)
            .map_err(|e| ElabError::Internal(format!("numeric tower init failed: {}", e)))?;
        record_decl_owners(
            &env,
            before_numeric,
            "prelude::numeric",
            &mut decl_owner,
            &mut owner_ordinals,
        );
        let before_bytes = env.decls().count();
        let bytes_env = bytes::register_bytes_env(&mut env, &mut globals)
            .map_err(|e| ElabError::Internal(format!("bytes layer init failed: {}", e)))?;
        record_decl_owners(
            &env,
            before_bytes,
            "prelude::bytes",
            &mut decl_owner,
            &mut owner_ordinals,
        );
        // Effect rows are populated only from elaborated declarations. An
        // independent seed would let producer-binding tests stay green after
        // the real declaration lost its `visits` row.
        let effect_rows = HashMap::new();
        let mut elab = Self {
            env,
            globals,
            decl_owner,
            owner_ordinals,
            preconditions: HashMap::new(),
            num_values: HashMap::new(),
            fixities: HashMap::new(),
            fixity_spans: HashMap::new(),
            ctor_decl_spans,
            numeric_env,
            standard_operators: HashMap::new(),
            bytes_env,
            foreign_env: foreign::ForeignEnv::empty(),
            effect_rows,
            effect_rows_by_id: HashMap::new(),
            space_metadata: SpaceElaborationMetadata::default(),
            // placeholder; `register_prelude` fills it (and needs `&mut self`).
            prelude_env: prelude::empty_prelude_env(),
            // placeholder; replaced after prelude registration below.
            class_env: classes::ClassEnv::sentinel(),
            resolution_provenance: Vec::new(),
            module_state: modules::ModuleState::default(),
            refinement_facts: elab::RefinementFacts::default(),
        };
        // L3 prelude: Peano `Nat` (replaces the placeholder postulate) + the
        // collection inductives + Ω constants (`37`). Registered via the landed
        // `data` / postulate machinery — no new kernel rule.
        let registered_prelude =
            elab.with_owner("prelude::register".into(), prelude::register_prelude)?;
        elab.prelude_env = registered_prelude;
        // Safe Bytes ops return the prelude's `Option`/`Result` sums, so their
        // primitive signatures are installed only after those sums exist.
        elab.with_owner("prelude::safe_bytes".into(), |elab| {
            bytes::register_safe_bytes_ops(&mut elab.env, &mut elab.globals, &mut elab.bytes_env)
        })?;
        // Lc typeclass env: pre-declare RecordNil + record_nil_val (`33 §5`).
        let class_env = elab.with_owner("prelude::classes".into(), |elab| {
            elab::init_class_env(&mut elab.env, &mut elab.globals)
        })?;
        elab.class_env = class_env;
        // `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — capture the immutable
        // pre-source trusted base.
        //
        // ⛔ **The authority boundary is the end of `ElabEnv::empty()`**, not
        // any single call inside it. What makes the capture sound is the
        // constructor's contract: every compiler declaration stage
        // (`register_prelude`, `register_safe_bytes_ops`, `init_class_env`)
        // has run, and the constructor has not yet returned an environment
        // that any package source could elaborate into. So the roster is
        // exactly "trusted before the package could speak".
        //
        // The neighbouring `install_prelude_floor()` is NOT that boundary and
        // must not be cited as one: it installs the unshadowable *name* floor
        // only, and says nothing about `Σ`'s trusted base. The adjacency is
        // where the line sits, not why it is correct.
        //
        // ⚠ Capturing at the end of `register_prelude` instead is WRONG, and
        // was measured: it misses the targets the two later prelude stages
        // declare (`bytes_at`, `bytes_decode`, `bytes_list_roundtrip`,
        // `RecordNil`, ...), leaving a roster strictly smaller than a clean
        // package's own tuple targets -- so every real package would be
        // refused for entries no user wrote.
        //
        // Adding a compiler initializer that declares trusted entries AFTER
        // this line silently shrinks the roster the same way. The control in
        // `prelude.rs` derives roster/tuple equality as a SET relation on a
        // freshly built environment, so such an addition reddens there.
        elab.prelude_env.native_trusted_base = elab.env.trusted_base().into_iter().collect();
        elab.module_state.install_prelude_floor();
        elab.module_state.capture_strict_builtin_names(
            &elab.env,
            &elab.globals,
            &elab.prelude_env.native_trusted_base,
        )?;
        Ok(elab)
    }

    /// Attribute every surviving kernel declaration admitted in this scope to
    /// its qualified source owner. A nested scope keeps its own attribution.
    pub(crate) fn with_owner<T>(&mut self, owner: String, f: impl FnOnce(&mut Self) -> T) -> T {
        let before = self.env.decls().count();
        let result = f(self);
        record_decl_owners(
            &self.env,
            before,
            &owner,
            &mut self.decl_owner,
            &mut self.owner_ordinals,
        );
        result
    }

    /// Create an environment with pre-declared `Nat`, `Bool`, and the full numeric tower.
    pub fn new() -> Result<Self, ElabError> {
        Self::empty()
    }

    /// Return the generated initial-state definition for a surface `space`.
    ///
    /// This accessor does not install a source-level member in `globals`.
    pub fn space_initial_state(&self, space: &str) -> Option<GlobalId> {
        self.space_metadata.initial_states.get(space).copied()
    }

    /// Declare a postulate `name : ty_term` in the environment.
    ///
    /// Used by tests to pre-declare types, predicates, and propositions needed
    /// for conformance test setup.
    pub fn declare_postulate_raw(&mut self, name: &str, ty: Term) -> Result<GlobalId, ElabError> {
        let id = declare_postulate(&mut self.env, name.to_string(), vec![], ty)
            .map_err(|e| ElabError::Internal(format!("declare_postulate failed: {}", e)))?;
        self.globals.insert(name.to_string(), id);
        Ok(id)
    }

    /// Elaborate a single V0/V1/L1 declaration from source.
    ///
    /// On success the declaration is registered in `self.env`.
    pub fn elaborate_decl(&mut self, src: &str) -> Result<GlobalId, ElabError> {
        self.elaborate_decl_v1(src).map(|result| result.def_id)
    }

    /// Elaborate a V1/L1 declaration, returning obligations alongside the id.
    pub fn elaborate_decl_v1(&mut self, src: &str) -> Result<ElabResult, ElabError> {
        let decls = parser::parse_decls(src)?;
        if decls.len() != 1 {
            return Err(ElabError::ParseError {
                msg: format!("expected exactly one declaration, found {}", decls.len()),
                span: Span::zero(),
            });
        }
        let results = modules::expand_and_elaborate(self, &decls)?;
        results.into_iter().last().ok_or_else(|| {
            ElabError::Internal("declaration produced no definition (bare import?)".into())
        })
    }

    /// Elaborate one declaration, returning every result it expands to
    /// (a `module { ... }` block yields one per inner declaration).
    pub fn elaborate_decl_results_v1(
        &mut self,
        src: &str,
    ) -> Result<Vec<ElabResult>, ElabError> {
        let decls = parser::parse_decls(src)?;
        if decls.len() != 1 {
            return Err(ElabError::ParseError {
                msg: format!("expected exactly one declaration, found {}", decls.len()),
                span: Span::zero(),
            });
        }
        modules::expand_and_elaborate(self, &decls)
    }

    /// Elaborate zero or more declarations from source, in order.
    ///
    /// Each declaration is elaborated and registered in `self.env` before the
    /// next is processed, so later declarations may refer to earlier ones.
    /// `module`/`import`/`export`/`pub` (`33 §3-4`) are resolved away here —
    /// they contribute zero or more `GlobalId`s (a bare `import` contributes
    /// none; a `module { … }` block contributes one per inner decl) but
    /// never a kernel-visible module concept. Returns the `GlobalId` of
    /// every successfully elaborated declaration.
    pub fn elaborate_file(&mut self, src: &str) -> Result<Vec<GlobalId>, ElabError> {
        self.elaborate_file_v1(src)
            .map(|results| results.into_iter().map(|result| result.def_id).collect())
    }

    /// Elaborate a file while retaining verification obligations, including
    /// those emitted by block-space operation contracts.
    pub fn elaborate_file_v1(&mut self, src: &str) -> Result<Vec<ElabResult>, ElabError> {
        let decls = parser::parse_decls(src)?;
        modules::expand_and_elaborate(self, &decls)
    }

    /// Elaborate the in-repo compilation unit named by `entry` under the
    /// plural catalog-root input (`33 §3.2`, ADR 0014 MRES-1/2/3a).
    ///
    /// Returns only declarations from the entry unit; dependencies remain in
    /// [`Self::elaborate_module_from_roots_v1`]. N2 populates exactly one root.
    /// The plural slice is the stable API shape; multi-root precedence remains
    /// deliberately deferred.
    pub fn elaborate_module_from_roots(
        &mut self,
        roots: &[PathBuf],
        entry: &str,
    ) -> Result<Vec<GlobalId>, ElabError> {
        modules::elaborate_module_from_roots(self, roots, entry)
    }

    /// Elaborate a roots-loaded unit and return the V1 results of its whole
    /// dependency closure, whether loaded by this call or cached from an earlier
    /// one, dependency-first and deduplicated by definition id.
    pub fn elaborate_module_from_roots_v1(
        &mut self,
        roots: &[PathBuf],
        entry: &str,
    ) -> Result<Vec<ElabResult>, ElabError> {
        modules::elaborate_module_from_roots_v1(self, roots, entry)
    }

    /// Elaborate a roots-loaded unit with strict bare-name resolution.
    ///
    /// This is opt-in until WP-4 migrates the real catalog dependency census;
    /// [`Self::elaborate_module_from_roots`] retains its legacy behavior.
    pub fn elaborate_module_from_roots_strict(
        &mut self,
        roots: &[PathBuf],
        entry: &str,
    ) -> Result<Vec<GlobalId>, ElabError> {
        modules::elaborate_module_from_roots_strict(self, roots, entry)
    }

    /// Execute checked-fence obligations for an already-loaded dotted entry.
    ///
    /// The roots loader records but never executes literate document roles.
    /// Calling this after a successful roots load preserves the directly-checked
    /// entry's `ken reject`/`ken example` contract without executing roles from
    /// imported dependency documents. Plain `.ken` entries are a no-op.
    pub fn execute_loaded_entry_checked_fences(&mut self, entry: &str) -> Result<(), ElabError> {
        self.execute_loaded_entry_checked_fences_v1(entry).map(|_| ())
    }

    /// Execute a loaded entry's checked fences and retain open obligations
    /// from its `ken example` fences. Rejected `ken reject` fences contribute
    /// no result.
    pub fn execute_loaded_entry_checked_fences_v1(
        &mut self,
        entry: &str,
    ) -> Result<Vec<ElabResult>, ElabError> {
        modules::execute_loaded_entry_checked_fences_v1(self, entry)
    }

    /// Number of successfully loaded cross-file units in this elaboration run.
    /// Exposed so acceptance tests and drivers can verify at-most-once loading.
    pub fn loaded_module_count(&self) -> usize {
        self.module_state.loaded_unit_count()
    }

    /// Read the anonymous boundary declared by the root source unit.
    ///
    /// The two optional clauses remain independent in the returned AST. This
    /// accessor performs no capability minting; Runtime owns that later step.
    pub fn boundary_header(&self) -> Option<&BoundaryHeader> {
        self.module_state.boundary_header()
    }

    /// Elaborate a single `.ken.md` source artifact.
    ///
    /// The Markdown extractor is a read-boundary transform only: it preserves
    /// byte offsets into the original artifact by blanking prose, validates
    /// each compiled fence independently, then reuses the ordinary file
    /// parser/elaborator path on the full blank-preserved buffer.
    ///
    /// After the module elaborates, every `` ```ken reject `` block is
    /// checked to still fail to elaborate (an unexpected success means the
    /// negative example has gone stale) and every `` ```ken example `` block
    /// is checked to elaborate (`catalog-literate-fence-roles` §4.6). Both
    /// checks run **in document order against this same, module-seeded
    /// `self`** — a deliberate V1 simplification: a later checked block may
    /// observe declarations an earlier one introduced, and neither role
    /// forks/rolls back env state.
    pub fn elaborate_ken_md_file(&mut self, src: &str) -> Result<Vec<GlobalId>, ElabError> {
        let (declarations, _examples, _example_ids) = self.elaborate_ken_md_file_parts(src)?;
        Ok(declarations
            .into_iter()
            .map(|result| result.def_id)
            .collect())
    }

    /// Elaborate a `.ken.md` artifact and retain results from its declarations
    /// and every successful `ken example` fence.
    pub fn elaborate_ken_md_file_v1(
        &mut self,
        src: &str,
    ) -> Result<Vec<ElabResult>, ElabError> {
        let (mut declarations, examples, _example_ids) =
            self.elaborate_ken_md_file_parts(src)?;
        declarations.extend(examples);
        Ok(declarations)
    }

    pub(crate) fn elaborate_ken_md_file_parts(
        &mut self,
        src: &str,
    ) -> Result<(Vec<ElabResult>, Vec<ElabResult>, Vec<GlobalId>), ElabError> {
        fn declaration_ids(env: &GlobalEnv) -> HashSet<GlobalId> {
            let mut ids = HashSet::new();
            for decl in env.decls() {
                ids.insert(decl.id());
                if let KernelDecl::Inductive(inductive) = decl {
                    ids.extend(inductive.constructors.iter().map(|constructor| constructor.id));
                }
            }
            ids
        }

        let extracted = literate::extract_ken_md(src)?;
        literate::validate_ken_md_fences(&extracted)?;
        let decls = parser::parse_decls(&extracted.source)?;
        let declarations = modules::expand_and_elaborate(self, &decls)?;
        let before_examples = declaration_ids(&self.env);
        let examples = self.execute_ken_md_checked_fences_v1(src, &extracted)?;
        let example_ids = declaration_ids(&self.env)
            .difference(&before_examples)
            .copied()
            .collect();
        Ok((declarations, examples, example_ids))
    }

    /// Execute one literate entry's checked-but-not-tangled fence roles.
    ///
    /// The extraction carries byte ranges into `src`; callers reuse the
    /// extraction that produced the entry's compiled source. This is separate
    /// from module loading because dependency documents do not execute roles.
    pub fn execute_ken_md_checked_fences(
        &mut self,
        src: &str,
        extracted: &literate::KenMdExtraction,
    ) -> Result<(), ElabError> {
        self.execute_ken_md_checked_fences_v1(src, extracted)
            .map(|_| ())
    }

    /// Execute checked literate fences and return results from successful
    /// `ken example` fences. `ken reject` ranges are checked and excluded.
    pub fn execute_ken_md_checked_fences_v1(
        &mut self,
        src: &str,
        extracted: &literate::KenMdExtraction,
    ) -> Result<Vec<ElabResult>, ElabError> {
        for range in &extracted.reject_ranges {
            if self.elaborate_file(&src[range.clone()]).is_ok() {
                return Err(ElabError::ParseError {
                    msg: "a 'ken reject' block unexpectedly elaborated: the negative example \
                          is stale and no longer demonstrates a rejection"
                        .to_string(),
                    span: Span::new(range.start, range.end),
                });
            }
        }
        let mut results = Vec::new();
        for range in &extracted.example_ranges {
            let example_results = self
                .elaborate_file_v1(&src[range.clone()])
                .map_err(|_| ElabError::ParseError {
                    msg: "a 'ken example' block failed to elaborate".to_string(),
                    span: Span::new(range.start, range.end),
                })?;
            results.extend(example_results);
        }
        Ok(results)
    }

    /// Try to discharge an obligation hole with a certificate term.
    ///
    /// `cert` is a CLOSED term (no free variables) of type `closed_goal`.
    /// The closed goal is checked before the kernel's checked-body admission
    /// verifies the hole's declared type, SCT, and transparent escape paths.
    /// Only then is the postulate removed from `trusted_base()`. Returns
    /// `true` if the discharge succeeded.
    pub fn discharge_hole(&mut self, obl: &Obligation, cert: Term) -> bool {
        // Kernel-check the certificate against the closed goal
        if kernel_check(&self.env, &Context::new(), &cert, &obl.goal_closed).is_err() {
            return false;
        }
        // A circular certificate must not retire its own trusted assumption.
        ken_kernel::check::admit_bodies(&mut self.env, &[(obl.hole_id, cert)]).is_ok()
    }

    /// Returns `true` if `hole_id` is still in `trusted_base()` (status = `unknown`).
    pub fn is_open_hole(&self, hole_id: GlobalId) -> bool {
        self.env.trusted_base().contains(&hole_id)
    }

    pub(crate) fn with_env_mark_rollback<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, ElabError>,
    ) -> Result<T, ElabError> {
        let mark = env_mark(&self.env);
        let provenance_len = self.resolution_provenance.len();
        match operation(self) {
            Ok(value) => Ok(value),
            Err(error) => match self.rollback_env_mark(mark) {
                Ok(()) => {
                    self.resolution_provenance.truncate(provenance_len);
                    Err(error)
                }
                Err(rollback_error) => Err(ElabError::Internal(format!(
                    "operation failed ({error:?}); environment rollback failed ({rollback_error:?})"
                ))),
            },
        }
    }

    fn rollback_env_mark(&mut self, mark: EnvMark) -> Result<(), ElabError> {
        let removed = rollback_to_mark(&mut self.env, mark).map_err(|error| {
            ElabError::Internal(format!("environment rollback failed: {error:?}"))
        })?;
        let mut removed_ids = HashSet::new();
        for declaration in &removed {
            removed_ids.insert(declaration.id());
            if let KernelDecl::Inductive(inductive) = declaration {
                removed_ids.extend(inductive.constructors.iter().map(|ctor| ctor.id));
            }
        }
        if removed_ids.is_empty() {
            return Ok(());
        }

        let mut removed_names: HashSet<String> = removed
            .iter()
            .filter_map(|declaration| match declaration {
                KernelDecl::Opaque { name, .. } => Some(name.clone()),
                _ => None,
            })
            .collect();
        let ElabEnv {
            env: _,
            globals,
            decl_owner,
            owner_ordinals,
            preconditions,
            num_values,
            fixities,
            fixity_spans,
            ctor_decl_spans,
            numeric_env,
            standard_operators,
            bytes_env: _,
            foreign_env,
            effect_rows,
            effect_rows_by_id,
            space_metadata,
            prelude_env,
            class_env,
            resolution_provenance: _,
            module_state,
            refinement_facts,
        } = self;
        removed_names.extend(
            globals
                .iter()
                .filter_map(|(name, id)| removed_ids.contains(id).then_some(name.clone())),
        );
        globals.retain(|_, id| !removed_ids.contains(id));
        decl_owner.retain(|id, _| !removed_ids.contains(id));
        owner_ordinals.clear();
        for (_, (owner, ordinal)) in decl_owner.iter() {
            let next = owner_ordinals.entry(owner.clone()).or_insert(0);
            *next = (*next).max(ordinal.checked_add(1).expect("owner ordinal exhausted"));
        }
        preconditions.retain(|id, _| !removed_ids.contains(id));
        num_values.retain(|id, _| !removed_ids.contains(id));
        fixities.retain(|id, _| !removed_ids.contains(id));
        fixity_spans.retain(|id, _| !removed_ids.contains(id));
        ctor_decl_spans.retain(|name, _| !removed_names.contains(name));
        numeric_env.scrub_global_ids(&removed_ids);
        standard_operators.retain(|_, id| !removed_ids.contains(id));
        foreign_env.scrub_global_ids(&removed_ids);
        effect_rows.retain(|name, _| !removed_names.contains(name));
        effect_rows_by_id.retain(|id, _| !removed_ids.contains(id));
        space_metadata
            .initial_states
            .retain(|_, id| !removed_ids.contains(id));
        prelude_env
            .native_trusted_base
            .retain(|id| !removed_ids.contains(id));
        class_env.scrub_global_ids(&removed_ids);
        module_state.scrub_global_ids(&removed_ids);
        refinement_facts.scrub_global_ids(&removed_ids);
        Ok(())
    }

    /// Elaborate a standalone expression from source.
    pub fn elaborate_expr(
        &mut self,
        owner_label: impl Into<String>,
        src: &str,
    ) -> Result<(Term, Term), ElabError> {
        let expr = parser::parse_expr(src)?;
        let rexpr = resolve::resolve_expr_standalone(&expr)?;
        let owner_label = owner_label.into();
        self.with_owner(owner_label.clone(), |env| {
            env.with_env_mark_rollback(|env| {
                elab::elaborate_rexpr(
                    &mut env.env,
                    &env.globals,
                    &env.preconditions,
                    &mut env.num_values,
                    &env.numeric_env,
                    &env.refinement_facts,
                    owner_label,
                    &rexpr,
                )
            })
        })
    }

    pub fn kernel_version(&self) -> &'static str {
        ken_kernel::version()
    }
}

impl Default for ElabEnv {
    fn default() -> Self {
        Self::new().expect("base environment predeclaration failed")
    }
}

pub fn kernel_version() -> &'static str {
    ken_kernel::version()
}
