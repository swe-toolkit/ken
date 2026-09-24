//! Module namespacing, import resolution, and visibility (`33 §3-4`,
//! ES3-build) — a pure surface/elaboration-time layer.
//!
//! `module`/`import`/`pub` add **no kernel feature**: a `module M { … }`
//! block is an environment fragment whose declarations are renamed to their
//! fully-qualified surface names (`M.foo`) and elaborated through the exact
//! same `resolve::resolve_decl` → `elab::elaborate_rdecl_v1` pipeline as a
//! flat, unqualified program. The kernel `GlobalEnv`/`Σ` never sees a name —
//! only `GlobalId`s — so qualification is bookkeeping entirely local to the
//! `globals: HashMap<String, GlobalId>` surface layer plus the bookkeeping
//! in `ModuleState` below. Abstract export (`§4.2`) requires zero additional
//! mechanism: a `pub data T = MkT` registers `T` in the module's export
//! table but never `MkT` (constructors are never auto-exported), which IS
//! the existing opaque-constant discipline at the surface layer — a client
//! that never gets `MkT` into scope can't build or match it, exactly as if
//! `T` had been declared as a hand-written opaque constant.
//!
//! Pipeline per compilation unit (one `elaborate_*` call's `Vec<Decl>`):
//! rename (qualify decl-level names) → `resolve_decl` (lexical resolution plus
//! unit-local collision admission) → rewrite (qualify free `RCon`/pattern-ctor
//! references via the active import scope) → `elaborate_rdecl_v1` (unchanged).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::ast::{
    BoundaryHeader, CtorDecl, Decl, ExplicitDataCtor, ExportForm, Fixity, ImportItem, ImportKind,
    Type,
};
use crate::error::{ElabError, Span};
use crate::resolve::{
    self, RCtorDecl, RDecl, RDeclKind, RExplicitCtorDecl, RExpr, RInfixOperator, RMatchArm,
    RPatKind, RPattern, RPropIntro, RTelescopeEntry, RType,
};
use crate::ElabEnv;

/// Persistent cross-call module bookkeeping (lives on `ElabEnv`).
#[derive(Default, Clone)]
pub struct ModuleState {
    /// The anonymous boundary parsed from the active root source unit. This is
    /// the shared reader seam: admission consumes `admits`; the runner may
    /// independently consume `capabilities` after elaboration.
    boundary_header: Option<BoundaryHeader>,
    /// The root (unqualified, file-level) scope: accumulates selective-import
    /// bindings and top-level local names seen across separate
    /// `elaborate_decl`/`elaborate_file` calls, so a later call's bare
    /// references still see earlier imports/locals (a "file" is an implicit
    /// module, `33 §3.1`).
    root_scope: Scope,
    /// Qualified module path (`"M"`, `"M.N"`) → {bare `pub` name → canonical
    /// qualified name}. Populated whenever a `module { … }` block elaborates.
    /// Only `pub` names are recorded here — the export table IS the
    /// enforcement point for private-by-default (`§4.1`) and abstract
    /// export (`§4.2`): a name simply isn't here if it wasn't exported.
    exports: HashMap<String, HashMap<String, String>>,
    /// Canonical `prop` family → names of its actually elaborated intro helpers.
    /// Kept separate from exports so a private family can be explicitly
    /// re-exported without accidentally exposing data constructors or modules.
    prop_intros: HashMap<String, Vec<String>>,
    /// Direct parent → child edges recorded only when expanding an inline
    /// `module` declaration. Loaded file units have no such edge even when
    /// their dotted path shares a prefix with another loaded unit.
    inline_children: HashMap<String, HashSet<String>>,
    /// Completed file unit → its own expanded inline declaration paths.
    /// A global parent→child edge alone cannot prove which unit declared it.
    file_inline_paths: HashMap<String, HashSet<String>>,
    /// File unit → immutable public export tables of its root and the inline
    /// descendants it declared. A later in-memory owner may replace the
    /// global tables at these canonical spellings without changing this file.
    file_export_tables: FileExportTables,
    /// Checked IDs from exactly those file tables, captured before another
    /// unit can overwrite the same canonical spelling in `globals`.
    file_export_ids: HashMap<String, ProviderExportIds>,
    /// Current export table → its own paths and origin, replaced at the same
    /// site as `exports` (file-backed or in-memory, without a fixed priority).
    export_provenance: HashMap<String, ExportProvenance>,
    /// Plural resolver input for this run. N2 accepts exactly one populated
    /// root; retaining the list here makes later roots a data change.
    catalog_roots: Vec<PathBuf>,
    /// Successfully elaborated file units, keyed by dotted module path.
    loaded_units: HashMap<String, Vec<ken_kernel::GlobalId>>,
    /// Raw source and the one authoritative extraction for loaded literate
    /// units. Loading does not execute checked fences; an entry front end may
    /// request that separate document-check step after the module graph loads.
    loaded_literate_units: HashMap<String, (String, crate::literate::KenMdExtraction)>,
    /// Completed per-unit scopes. The loader does not install these as the
    /// ambient isolated-file scope; an entry document check installs only its
    /// selected unit so checked fences see the declarations they follow.
    loaded_unit_scopes: HashMap<String, Scope>,
    /// Units currently being discovered/elaborated, in entry-rooted edge order.
    active_imports: Vec<String>,
    /// Parent names in the closed prelude floor (`30-taxonomy §4`).
    prelude_names: HashSet<String>,
    /// Unshadowable bindings derived from the exact floor parents: those
    /// parent names plus only their kernel-recorded constructor names.
    prelude_binding_names: HashSet<String>,
    /// Compiler vocabulary captured before package source can add aliases.
    /// Includes native trusted names and constructors of the closed floor.
    strict_builtin_names: HashSet<String>,
    /// One roots-loader run cannot mix legacy and strict units: the loaded-unit
    /// cache records elaborated results rather than unresolved source.
    roots_resolution_mode: Option<ResolutionMode>,
}

type FileExportTables = HashMap<String, HashMap<String, HashMap<String, String>>>;
type ProviderExportIds = HashMap<String, HashMap<String, ken_kernel::GlobalId>>;

#[derive(Clone)]
struct ExportProvenance {
    inline_paths: HashSet<String>,
    file_root: Option<String>,
    /// Per-canonical-module public name → checked declaration identity,
    /// including owned inline descendants and selected facade identities.
    member_ids: ProviderExportIds,
}

/// The complete Ken-defined always-present type floor (`30-taxonomy §4`).
///
/// Strict resolution consults [`is_prelude_floor_name`] so the configured type
/// inventory has one source of truth. Its signature arm is independently
/// derived from every primitive declaration type by the realization controls;
/// `Nat` and `Pair` have separate internal-provision witnesses.
pub const PRELUDE_FLOOR_NAMES: [&str; 10] = [
    "Auth",
    "Bool",
    "Char",
    "List",
    "Nat",
    "Option",
    "Pair",
    "ResourceKind",
    "Result",
    "Utf8Error",
];

/// Checked bindings admitted with the exact compiler-bootstrap `Pair` type.
/// These are not type-floor members and do not increase its ten-member count.
pub const PRELUDE_COMPANION_BINDING_NAMES: [&str; 3] = ["mk_pair", "pair_fst", "pair_snd"];

pub fn is_prelude_floor_name(name: &str) -> bool {
    PRELUDE_FLOOR_NAMES.contains(&name)
}

fn term_mentions_global(term: &ken_kernel::Term, target: ken_kernel::GlobalId) -> bool {
    match term {
        ken_kernel::Term::Const { id, .. }
        | ken_kernel::Term::IndFormer { id, .. }
        | ken_kernel::Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        ken_kernel::Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions_global(child, target)),
    }
}

impl ModuleState {
    pub(crate) fn loaded_unit_count(&self) -> usize {
        self.loaded_units.len()
    }

    pub(crate) fn boundary_header(&self) -> Option<&BoundaryHeader> {
        self.boundary_header.as_ref()
    }

    pub(crate) fn install_prelude_floor(&mut self) {
        // `30-taxonomy §4` derives this exact closed set from the built-in
        // primitive signatures. Other definitions constructed in prelude.rs
        // are package-level conveniences, not unshadowable prelude members.
        self.prelude_names = PRELUDE_FLOOR_NAMES
            .into_iter()
            .map(str::to_string)
            .collect();
    }

    pub(crate) fn capture_strict_builtin_names(
        &mut self,
        env: &ken_kernel::GlobalEnv,
        globals: &HashMap<String, ken_kernel::GlobalId>,
        native_trusted_base: &std::collections::BTreeSet<ken_kernel::GlobalId>,
    ) -> Result<(), ElabError> {
        let floor_formers: HashSet<_> = PRELUDE_FLOOR_NAMES
            .iter()
            .map(|name| {
                globals.get(*name).copied().ok_or_else(|| {
                    ElabError::Internal(format!(
                        "prelude type-floor member `{name}` has no pre-source identity"
                    ))
                })
            })
            .collect::<Result<_, _>>()?;
        let pair_id = globals.get("Pair").copied().ok_or_else(|| {
            ElabError::Internal(
                "prelude type-floor member `Pair` has no pre-source identity".to_string(),
            )
        })?;

        let companion_bindings = PRELUDE_COMPANION_BINDING_NAMES
            .iter()
            .map(|name| {
                let id = globals.get(*name).copied().ok_or_else(|| {
                    ElabError::Internal(format!(
                        "prelude Pair companion `{name}` has no pre-source identity"
                    ))
                })?;
                if !matches!(env.lookup(id), Some(ken_kernel::Decl::Transparent { .. })) {
                    return Err(ElabError::Internal(format!(
                        "prelude Pair companion `{name}` is not checked-transparent"
                    )));
                }
                let (_, ty) = env.const_type(id).ok_or_else(|| {
                    ElabError::Internal(format!(
                        "prelude Pair companion `{name}` is not a checked constant"
                    ))
                })?;
                if !term_mentions_global(&ty, pair_id) {
                    return Err(ElabError::Internal(format!(
                        "prelude Pair companion `{name}` is not keyed to the exact Pair identity"
                    )));
                }
                Ok((name.to_string(), id))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // A fixed proof constant is neither a type-floor member nor a
        // postulate. Validate its kernel identity before reserving its name.
        require_fixed_proved_identity(env, globals)?;

        self.prelude_binding_names = self.prelude_names.clone();
        self.strict_builtin_names = globals
            .iter()
            .filter_map(|(name, id)| {
                let floor_constructor = env
                    .constructor(*id)
                    .is_some_and(|(parent, _)| floor_formers.contains(&parent.id));
                if floor_constructor {
                    self.prelude_binding_names.insert(name.clone());
                }
                (native_trusted_base.contains(id) || floor_constructor).then_some(name.clone())
            })
            .collect();
        for (name, _) in companion_bindings {
            self.prelude_binding_names.insert(name.clone());
            self.strict_builtin_names.insert(name);
        }
        self.prelude_binding_names.insert("Proved".to_string());
        self.strict_builtin_names.insert("Proved".to_string());
        Ok(())
    }
}

fn require_fixed_proved_identity(
    env: &ken_kernel::GlobalEnv,
    globals: &HashMap<String, ken_kernel::GlobalId>,
) -> Result<(), ElabError> {
    let expected = env.tt_id();
    match globals.get("Proved").copied() {
        Some(actual) if actual == expected => Ok(()),
        actual => Err(ElabError::Internal(format!(
            "Proved identity mismatch: expected fixed kernel tt {expected:?}, found {actual:?}"
        ))),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ResolutionMode {
    #[default]
    Legacy,
    Strict,
}

/// Per-scope bare-name resolution: selective-import bindings plus this scope's
/// own local declarations. A top-level
/// local/import collision is fail-closed regardless of source order (`33
/// §3.3`); narrower lexical binders remain innermost-wins.
#[derive(Default, Clone)]
struct Scope {
    mode: ResolutionMode,
    /// Compiler-installed vocabulary captured before package source can speak.
    /// This excludes later ambient postulates while preserving primitive names.
    kernel_names: HashSet<String>,
    bindings: HashMap<String, String>,
    /// Bare names bound by a top-level LOCAL declaration in this scope.
    locals: std::collections::HashSet<String>,
    /// Qualified imports, aliases, and inline descendants of a locally
    /// declared or imported owner. Every canonical target is backed by an
    /// actual inline-declaration edge; loaded exports alone grant no prefix.
    prefixes: HashMap<String, String>,
    /// The exact export table selected by a file-backed import at each surface
    /// prefix. A later memory owner may replace `exports[canonical]` without
    /// changing a previously imported file's public interface.
    file_prefix_exports: HashMap<String, HashMap<String, String>>,
    /// Fully spelled imported member → identity selected by that provider.
    /// Bare selective aliases are separate: their spelling has no prefix.
    qualified_ids: HashMap<String, ken_kernel::GlobalId>,
    binding_ids: HashMap<String, ken_kernel::GlobalId>,
    /// Export-name ID overrides for facade/in-scope republishing. A canonical
    /// spelling alone can no longer recover the imported provider's ID.
    exported_ids: HashMap<String, ken_kernel::GlobalId>,
    /// Locals declared by the current expansion, even if a previous unit
    /// already minted this canonical spelling. Cleared after the ordered pass.
    current_local_names: HashSet<String>,
    /// An `export Local` may precede Local's checked declaration. Delay its
    /// ID until that declaration finishes, then check any competing facade.
    pending_local_exports: HashMap<String, (String, Span)>,
    /// A proof declared in THIS scope may be attached to an imported subject.
    /// Its own declaration, not the subject's provider, grants the local
    /// `proof p for subject` selector (including same-unit SCC references).
    local_attached_proofs: HashSet<String>,
    /// Names mentioned by a facade export remain deliberately unavailable to
    /// the body unless a separate import/local binding supplies them. Keeping
    /// this negative fact makes the normative facade-vs-binding failure an
    /// attributed surface `UnboundName` rather than a later global miss.
    facade_only: HashSet<String>,
}

impl Scope {
    fn with_mode(mode: ResolutionMode, kernel_names: HashSet<String>) -> Self {
        Self {
            mode,
            kernel_names,
            ..Self::default()
        }
    }

    fn bind_import(
        &mut self,
        globals: &HashMap<String, ken_kernel::GlobalId>,
        bare: &str,
        qualified: &str,
        selected_id: ken_kernel::GlobalId,
        span: &Span,
    ) -> Result<(), ElabError> {
        if self.locals.contains(bare) {
            let local_binding = self.bindings.get(bare).cloned();
            let local_id = match local_binding.as_deref() {
                Some(local) => globals.get(local),
                None => globals.get(bare),
            };
            if local_id.is_some_and(|local| *local == selected_id) {
                return Ok(());
            }
            let local_source = local_binding
                .or_else(|| local_id.map(|id| format!("{id:?}")))
                .ok_or_else(|| {
                    ElabError::Internal(format!(
                        "local import collision for `{bare}` has neither a canonical \
                         binding nor a resolved identity"
                    ))
                })?;
            return Err(ElabError::AmbiguousReference {
                name: bare.to_string(),
                sources: vec![local_source, qualified.to_string()],
                span: span.clone(),
            });
        }
        match self.bindings.get(bare) {
            None => {
                self.bindings
                    .insert(bare.to_string(), qualified.to_string());
            }
            Some(existing) if existing == qualified => {
                if let Some(old) = self.binding_ids.get(bare) {
                    if *old != selected_id {
                        return Err(ElabError::AmbiguousReference {
                            name: bare.to_string(),
                            sources: vec![format!("{existing} {old:?}"), format!("{qualified} {selected_id:?}")],
                            span: span.clone(),
                        });
                    }
                }
            }
            Some(existing) => {
                return Err(ElabError::AmbiguousReference {
                    name: bare.to_string(),
                    sources: vec![existing.clone(), qualified.to_string()],
                    span: span.clone(),
                });
            }
        }
        self.binding_ids.insert(bare.to_string(), selected_id);
        Ok(())
    }

    /// Bind a top-level local, rejecting any import installed by an earlier
    /// elaboration call. Same-file locals are pre-collected before imports, so
    /// `bind_import` supplies the symmetric arm.
    fn bind_local(&mut self, bare: &str, qualified: &str, span: &Span) -> Result<(), ElabError> {
        if let Some(binding) = self.bindings.get(bare) {
            if binding != qualified {
                let mut sources = vec![binding.clone()];
                sources.push(qualified.to_string());
                return Err(ElabError::AmbiguousReference {
                    name: bare.to_string(),
                    sources,
                    span: span.clone(),
                });
            }
        }
        self.locals.insert(bare.to_string());
        self.bindings
            .insert(bare.to_string(), qualified.to_string());
        Ok(())
    }
}

fn qualify(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", prefix, name)
    }
}

/// Resolve a (possibly dotted) surface name reference to its canonical
/// qualified form, using the active `scope` for bare names and `exports`
/// for qualified (`M.foo`) references. Legacy mode returns an untracked name
/// unchanged, preserving the pre-existing flat `cx.globals` fallback. Strict
/// mode instead fails closed unless the name is scope-bound, compiler
/// vocabulary, or a member of the closed prelude floor.
fn is_unshadowable_kernel_name(name: &str) -> bool {
    matches!(
        name,
        "Omega" | crate::resolve::SUGAR_REFL | crate::resolve::SUGAR_AXIOM
    )
}

fn resolve_ref(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    name: &str,
    span: &Span,
) -> Result<String, ElabError> {
    if let Some((subject, proof)) = name.split_once("::") {
        if !subject.contains('.') {
            if scope.qualified_ids.contains_key(name)
                || scope.local_attached_proofs.contains(name)
            {
                if let Some(canonical) = scope.bindings.get(subject) {
                    return Ok(format!("{canonical}::{proof}"));
                }
            }
            if scope.mode == ResolutionMode::Strict && scope.bindings.contains_key(subject) {
                return Err(ElabError::UnboundName {
                    name: name.to_string(),
                    span: span.clone(),
                });
            }
        }
    }
    if let Some(dot) = name.rfind('.') {
        let (prefix_part, leaf) = (&name[..dot], &name[dot + 1..]);
        if let Some(q) = scope.bindings.get(prefix_part) {
            // Selective imports of a family grant only checked public
            // selectors. A canonical spelling formed by appending an
            // arbitrary leaf could otherwise borrow an ambient provider's
            // private member after that provider overwrote `globals`.
            if scope.locals.contains(prefix_part) || scope.qualified_ids.contains_key(name) {
                return Ok(format!("{q}.{leaf}"));
            }
            return Err(ElabError::UnboundName {
                name: name.to_string(),
                span: span.clone(),
            });
        }
        if let Some(canonical_module) = scope.prefixes.get(prefix_part) {
            return scope
                .file_prefix_exports
                .get(prefix_part)
                .or_else(|| exports.get(canonical_module))
                .and_then(|pubmap| pubmap.get(leaf))
                .cloned()
                .ok_or_else(|| ElabError::UnboundName {
                    name: name.to_string(),
                    span: span.clone(),
                });
        }
        // A `prop` intro is a selector on an exported family, not an inline
        // module child. Its exact entry is minted only from an elaborated
        // PropDecl (including re-exports), under the authorized module prefix.
        // Never infer a child module from a shorter imported prefix: its
        // private leaves still need that child's own public export map.
        if let Some((module_part, family)) = prefix_part.rsplit_once('.') {
            if let Some(canonical_module) = scope.prefixes.get(module_part) {
                if let Some(pubmap) = scope
                    .file_prefix_exports
                    .get(module_part)
                    .or_else(|| exports.get(canonical_module))
                {
                    if let (Some(canonical_family), Some(canonical_intro)) =
                        (pubmap.get(family), pubmap.get(&format!("{family}.{leaf}")))
                    {
                        if canonical_intro == &format!("{canonical_family}.{leaf}") {
                            return Ok(canonical_intro.clone());
                        }
                    }
                }
            }
        }
        Err(ElabError::UnboundName {
            name: name.to_string(),
            span: span.clone(),
        })
    } else {
        match scope.bindings.get(name) {
            Some(q) => Ok(q.clone()),
            None if scope.facade_only.contains(name) => Err(ElabError::UnboundName {
                name: name.to_string(),
                span: span.clone(),
            }),
            None if scope.mode == ResolutionMode::Strict
                && !is_prelude_floor_name(name)
                && !scope.kernel_names.contains(name)
                && !is_unshadowable_kernel_name(name) =>
            {
                Err(ElabError::UnboundName {
                    name: name.to_string(),
                    span: span.clone(),
                })
            }
            None => Ok(name.to_string()),
        }
    }
}

/// Resolve a class-environment reference through the unit's class namespace.
/// Class-bearing declarations do not carry an RCon, so this is their single
/// strict choke, parallel to [`resolve_ref`] for globals-routed forms.
/// Return the selected provider's checked identity with its display name.
/// Locals and unresolved SCC members have no ID yet and keep the existing
/// string route until they are checked; imported members never re-lookup one.
fn resolve_checked_ref(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    name: &str,
    span: &Span,
) -> Result<(String, Option<ken_kernel::GlobalId>), ElabError> {
    let canonical = resolve_ref(scope, exports, name, span)?;
    let selected = scope
        .qualified_ids
        .get(name)
        .or_else(|| scope.binding_ids.get(name))
        .copied();
    Ok((canonical, selected))
}

fn resolve_class_ref(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    name: &str,
    span: &Span,
) -> Result<String, ElabError> {
    resolve_ref(scope, exports, name, span)
}

fn resolve_attached_ref(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    subject: &str,
    proof_name: &str,
    span: &Span,
) -> Result<(String, Option<ken_kernel::GlobalId>), ElabError> {
    let subject_is_local = !subject.contains('.') && scope.locals.contains(subject);
    let canonical_subject = resolve_ref(scope, exports, subject, span)?;
    let selected = format!("{subject}::{proof_name}");
    if let Some(id) = scope.qualified_ids.get(&selected) {
        // Only an actually exported attached proof is entered under this
        // subject selector. Its ID is selected by the subject's provider,
        // never by the process-global canonical spelling.
        let canonical = if selected.contains('.') {
            resolve_ref(scope, exports, &selected, span)?
        } else {
            format!("{canonical_subject}::{proof_name}")
        };
        return Ok((canonical, Some(*id)));
    }
    if subject_is_local || scope.local_attached_proofs.contains(&selected) {
        return Ok((format!("{canonical_subject}::{proof_name}"), None));
    }
    // D0's legacy ambient path remains available only when the subject is
    // genuinely untracked. An imported subject has a selected provider, so
    // a missing checked proof must never borrow ambient global history.
    if scope.mode == ResolutionMode::Legacy
        && !scope.bindings.contains_key(subject)
        && !subject.contains('.')
    {
        return Ok((format!("{canonical_subject}::{proof_name}"), None));
    }
    Err(ElabError::UnboundName {
        name: format!("{canonical_subject}::{proof_name}"),
        span: span.clone(),
    })
}

/// A bare import in a unit is either its own declared child (already
/// expanded or still unavailable) or an absolute module name. Global export
/// tables and edges owned by another unit do not classify the import.
enum InlineImport {
    Available(String),
    Unavailable,
    Absolute,
}

fn lexical_inline_import(
    prefix: &str,
    file_root: Option<&str>,
    unit_inline_modules: &HashSet<String>,
    ordered_inline_modules: &HashSet<String>,
    module: &str,
    inline_children: &HashMap<String, HashSet<String>>,
) -> InlineImport {
    if let Some(canonical) = declared_inline_import(prefix, file_root, module, unit_inline_modules)
    {
        // An in-memory unit's empty-prefix root has no parent→child edge in
        // inline_children. Its own ordered expansion is the provenance for
        // that root child; nested children still need their recorded edge.
        let own_edge = match canonical.rsplit_once('.') {
            Some((parent, _)) => inline_children
                .get(parent)
                .is_some_and(|children| children.contains(&canonical)),
            None => file_root.is_none(),
        };
        if ordered_inline_modules.contains(&canonical) && own_edge {
            InlineImport::Available(canonical)
        } else {
            InlineImport::Unavailable
        }
    } else {
        InlineImport::Absolute
    }
}

/// Grant only descendants whose edges were produced by actual inline module
/// declarations. `surface` may be an alias or a sibling-relative import;
/// `canonical` is the exact path used by the child's export table.
fn authorize_inline_descendants(
    scope: &mut Scope,
    inline_children: &HashMap<String, HashSet<String>>,
    owned_paths: &HashSet<String>,
    file_tables: Option<&HashMap<String, HashMap<String, String>>>,
    member_ids: Option<&ProviderExportIds>,
    canonical: &str,
    surface: &str,
) {
    let mut pending = vec![(canonical.to_string(), surface.to_string())];
    while let Some((parent, alias)) = pending.pop() {
        if let Some(children) = inline_children.get(&parent) {
            for child in children {
                // The shared graph only says an edge was declared somewhere.
                // Every traversed edge must belong to this particular owner
                // unit, not just have the same canonical spelling.
                if !owned_paths.contains(child) {
                    continue;
                }
                let leaf = child
                    .strip_prefix(&parent)
                    .and_then(|rest| rest.strip_prefix('.'))
                    .expect("inline declaration edge must name a direct child");
                let surface_child = qualify(&alias, leaf);
                scope.prefixes.insert(surface_child.clone(), child.clone());
                if let Some(ids) = member_ids {
                    bind_provider_members(scope, &surface_child, child, ids);
                }
                if let Some(tables) = file_tables {
                    let pubmap = tables
                        .get(child)
                        .expect("a file-owned inline descendant has an export snapshot");
                    scope
                        .file_prefix_exports
                        .insert(surface_child.clone(), pubmap.clone());
                }
                pending.push((child.clone(), surface_child));
            }
        }
    }
}

fn apply_import(
    scope: &mut Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    inline_children: &HashMap<String, HashSet<String>>,
    file_inline_paths: &HashMap<String, HashSet<String>>,
    file_export_tables: &FileExportTables,
    file_export_ids: &HashMap<String, ProviderExportIds>,
    export_provenance: &HashMap<String, ExportProvenance>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    prelude_binding_names: &HashSet<String>,
    owner: &str,
    file_root: Option<&str>,
    unit_inline_modules: &HashSet<String>,
    ordered_inline_modules: &HashSet<String>,
    module: &str,
    kind: &ImportKind,
    span: &Span,
) -> Result<(), ElabError> {
    let own_paths;
    let (canonical, authorized_paths, provider_file) = match lexical_inline_import(
        owner,
        file_root,
        unit_inline_modules,
        ordered_inline_modules,
        module,
        inline_children,
    ) {
        InlineImport::Available(path) => {
            own_paths = unit_inline_modules
                .intersection(ordered_inline_modules)
                .cloned()
                .collect::<HashSet<_>>();
            (path, Some(&own_paths), None)
        }
        InlineImport::Unavailable => {
            return Err(ElabError::UnboundName {
                name: module.to_string(),
                span: span.clone(),
            });
        }
        InlineImport::Absolute => {
            // Roots imports select their explicit file, while in-memory
            // imports select the current export table and its paired origin.
            // Neither kind wins merely by having a map at this spelling.
            if file_root.is_some() {
                (
                    module.to_string(),
                    file_inline_paths.get(module),
                    Some(module),
                )
            } else {
                let provenance = export_provenance.get(module);
                (
                    module.to_string(),
                    provenance.map(|owner| &owner.inline_paths),
                    provenance.and_then(|owner| owner.file_root.as_deref()),
                )
            }
        }
    };
    let file_tables = provider_file.and_then(|root| file_export_tables.get(root));
    let pubmap = match provider_file {
        Some(_) => file_tables.and_then(|tables| tables.get(&canonical)),
        None => exports.get(&canonical),
    }
    .ok_or_else(|| ElabError::UnboundName {
        name: module.to_string(),
        span: span.clone(),
    })?;
    let selected_ids = match provider_file {
        Some(file) => file_export_ids.get(file),
        None => export_provenance.get(&canonical).map(|owner| &owner.member_ids),
    }
    .ok_or_else(|| ElabError::Internal(format!(
        "selected import provider '{canonical}' has no checked export identities"
    )))?;
    let provider_members = selected_ids.get(&canonical).ok_or_else(|| {
        ElabError::Internal(format!("selected provider '{canonical}' has no checked member IDs"))
    })?;
    for leaf in pubmap.keys() {
        if !provider_members.contains_key(leaf) {
            return Err(ElabError::Internal(format!(
                "public member '{canonical}.{leaf}' has no checked ID in its selected provider"
            )));
        }
    }
    match kind {
        ImportKind::Qualified | ImportKind::Aliased(_) => {
            let surface = match kind {
                ImportKind::Qualified => module,
                ImportKind::Aliased(alias) => alias,
                ImportKind::Selective(_) => unreachable!(),
            };
            // Re-importing the same surface name replaces its authority.
            // Root scopes survive across in-memory source calls, so stale
            // descendant aliases from a prior provider must be withdrawn.
            let descendant_prefix = format!("{surface}.");
            scope
                .prefixes
                .retain(|alias, _| alias != surface && !alias.starts_with(&descendant_prefix));
            scope
                .file_prefix_exports
                .retain(|alias, _| alias != surface && !alias.starts_with(&descendant_prefix));
            scope
                .qualified_ids
                .retain(|name, _| !name.starts_with(&descendant_prefix));
            scope
                .prefixes
                .insert(surface.to_string(), canonical.clone());
            bind_provider_members(scope, surface, &canonical, selected_ids);
            if let Some(tables) = file_tables {
                scope
                    .file_prefix_exports
                    .insert(surface.to_string(), pubmap.clone());
                if let Some(paths) = authorized_paths {
                    authorize_inline_descendants(
                        scope,
                        inline_children,
                        paths,
                        Some(tables),
                        Some(selected_ids),
                        &canonical,
                        surface,
                    );
                }
            } else if let Some(paths) = authorized_paths {
                authorize_inline_descendants(
                    scope,
                    inline_children,
                    paths,
                    None,
                    Some(selected_ids),
                    &canonical,
                    surface,
                );
            }
        }
        ImportKind::Selective(names) => {
            for item in names {
                let q = pubmap
                    .get(&item.name)
                    .ok_or_else(|| ElabError::UnboundName {
                        name: format!("{}.{}", module, item.name),
                        span: span.clone(),
                    })?;
                let bare = item.rename.as_deref().unwrap_or(&item.name);
                let selected_id = provider_members[&item.name];
                if prelude_binding_names.contains(bare) {
                    let same_canonical_identity = globals
                        .get(bare)
                        .is_some_and(|installed| *installed == selected_id);
                    if !same_canonical_identity {
                        return Err(ElabError::AmbiguousReference {
                            name: bare.to_string(),
                            sources: vec![format!("<prelude>.{bare}"), q.clone()],
                            span: span.clone(),
                        });
                    }
                }
                scope.bind_import(globals, bare, q, selected_id, span)?;
                let selector_prefix = format!("{}.", item.name);
                let proof_prefix = format!("{}::", item.name);
                for (member, id) in provider_members {
                    if pubmap.contains_key(member) {
                        if let Some(selector) = member.strip_prefix(&selector_prefix) {
                            scope.qualified_ids.insert(format!("{bare}.{selector}"), *id);
                        }
                        if let Some(proof) = member.strip_prefix(&proof_prefix) {
                            scope.qualified_ids.insert(format!("{bare}::{proof}"), *id);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Layer 3 — certify the standard-operator home the moment its own export
/// table is complete (`33 §6.1`, `39 §6.9`).
///
/// **Called from BOTH module-elaboration paths on purpose.** A module reaches
/// its export table by two routes — a loaded source unit and an inline
/// `module M { … }` — and they insert into `module_state.exports` at two
/// different sites. Hooking only the first is the defect this function exists
/// to prevent: an inline home would elaborate entirely uncertified, and every
/// negative case would pass while proving nothing. That is not hypothetical;
/// it is what the AC-9 cases caught on the first run.
fn certify_standard_operator_home(
    elab: &mut ElabEnv,
    module: &str,
    at: Option<&Span>,
) -> Result<(), ElabError> {
    if !crate::standard_operators::is_standard_operator_home(module) {
        return Ok(());
    }
    // An inline `module M { … }` knows its own span; a loaded source unit is
    // the whole file and has none to offer. Carry the real one where it
    // exists rather than synthesising a coordinate that points nowhere.
    let span = at.cloned().unwrap_or_else(|| Span::new(0, 0));
    let bool_id = elab.numeric_env.bool_id;
    let certified = crate::standard_operators::certify_roles(
        &elab.env,
        &elab.module_state.exports,
        &elab.globals,
        crate::standard_operators::STANDARD_OPERATOR_HOME,
        bool_id,
        &span,
    )?;

    // `33 §6.1`'s standard fixities, installed onto the identities just
    // certified rather than declared in surface source.
    //
    // This is the only mechanism available, and the measurement is in the WP
    // thread: a fixity target must be DEFINED in the declaring module
    // (`scope.locals`), and a fixity target must be a SYMBOLIC operator, so
    // `infix 4 ord_leq_at` at the defining module is unsayable and
    // `infix 4 ≤` at the facade is refused as not-local. The only expressible
    // declaration is one that also DEFINES the glyph -- which mints a second
    // `GlobalId` and is exactly what `§6.9` forbids.
    //
    // Keying on the identity is what `§6` already requires: fixity is "a
    // property of the operator's canonical identity, not of any surface path
    // or alias that reaches it", so it travels with import and re-export for
    // free. `fixities` is `GlobalId`-keyed, so nothing further is needed to
    // make that travel happen.
    for (role, id) in &certified {
        elab.fixities.insert(*id, role.fixity());
    }
    elab.standard_operators = certified;
    Ok(())
}

fn checked_export_ids(
    scope: &mut Scope,
    exports: &HashMap<String, String>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
) -> Result<HashMap<String, ken_kernel::GlobalId>, ElabError> {
    for (surface, (canonical, span)) in std::mem::take(&mut scope.pending_local_exports) {
        let id = globals.get(&canonical).copied().ok_or_else(|| ElabError::UnboundName {
            name: canonical.clone(),
            span: span.clone(),
        })?;
        if let Some(existing_id) = scope.exported_ids.get(&surface) {
            if *existing_id != id {
                return Err(ElabError::ReExportCollision {
                    surface_name: surface.clone(),
                    existing: exports[&surface].clone(),
                    incoming: canonical,
                    span,
                });
            }
        }
        scope.exported_ids.insert(surface, id);
    }
    exports
        .iter()
        .map(|(surface, canonical)| {
            scope
                .exported_ids
                .get(surface)
                .copied()
                .or_else(|| globals.get(canonical).copied())
                .map(|id| (surface.clone(), id))
                .ok_or_else(|| ElabError::Internal(format!(
                    "published member '{surface}' at '{canonical}' has no checked ID"
                )))
        })
        .collect()
}

fn bind_provider_members(
    scope: &mut Scope,
    surface: &str,
    canonical: &str,
    member_ids: &ProviderExportIds,
) {
    if let Some(members) = member_ids.get(canonical) {
        for (leaf, id) in members {
            scope.qualified_ids.insert(qualify(surface, leaf), *id);
        }
    }
}

fn publish_identity(
    exports_here: &mut HashMap<String, String>,
    surface_name: &str,
    canonical: &str,
    span: &Span,
) -> Result<(), ElabError> {
    match exports_here.get(surface_name) {
        None => {
            exports_here.insert(surface_name.to_string(), canonical.to_string());
            Ok(())
        }
        Some(existing) if existing == canonical => Ok(()),
        Some(existing) => Err(ElabError::ReExportCollision {
            surface_name: surface_name.to_string(),
            existing: existing.clone(),
            incoming: canonical.to_string(),
            span: span.clone(),
        }),
    }
}

/// Publish only checked helpers belonging to the exact exported `prop` family.
/// A rename changes the visible family path, never the helper's canonical ID.
fn publish_checked_identity(
    scope: &mut Scope,
    exports_here: &mut HashMap<String, String>,
    surface: &str,
    canonical: &str,
    id: ken_kernel::GlobalId,
    span: &Span,
) -> Result<(), ElabError> {
    if let Some(existing_id) = scope.exported_ids.get(surface) {
        if *existing_id != id {
            return Err(ElabError::ReExportCollision {
                surface_name: surface.to_string(),
                existing: exports_here[surface].clone(),
                incoming: canonical.to_string(),
                span: span.clone(),
            });
        }
    }
    publish_identity(exports_here, surface, canonical, span)?;
    scope.exported_ids.insert(surface.to_string(), id);
    Ok(())
}

fn publish_family_intros(
    scope: &mut Scope,
    exports_here: &mut HashMap<String, String>,
    prop_intros: &HashMap<String, Vec<String>>,
    surface_family: &str,
    canonical_family: &str,
    source_family: &str,
    source_members: Option<&HashMap<String, ken_kernel::GlobalId>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    span: &Span,
) -> Result<(), ElabError> {
    if let Some(intros) = prop_intros.get(canonical_family) {
        for intro in intros {
            let surface = format!("{surface_family}.{intro}");
            let canonical = format!("{canonical_family}.{intro}");
            let id = match source_members {
                Some(members) => members.get(&format!("{source_family}.{intro}")).copied(),
                None => globals.get(&canonical).copied(),
            }
            .ok_or_else(|| ElabError::UnboundName {
                name: surface.clone(),
                span: span.clone(),
            })?;
            publish_checked_identity(scope, exports_here, &surface, &canonical, id, span)?;
        }
    }
    Ok(())
}

fn published_name(item: &ImportItem) -> &str {
    item.rename.as_deref().unwrap_or(&item.name)
}

fn apply_export(
    scope: &mut Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    file_export_tables: &FileExportTables,
    file_export_ids: &HashMap<String, ProviderExportIds>,
    export_provenance: &HashMap<String, ExportProvenance>,
    selected_file: Option<&str>,
    prop_intros: &HashMap<String, Vec<String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    exports_here: &mut HashMap<String, String>,
    form: &ExportForm,
    span: &Span,
) -> Result<(), ElabError> {
    match form {
        ExportForm::Facade { module, items } => {
            let pubmap = match selected_file {
                Some(file) => file_export_tables
                    .get(file)
                    .and_then(|tables| tables.get(module)),
                None => exports.get(module),
            }
            .ok_or_else(|| ElabError::UnboundName {
                name: module.clone(),
                span: span.clone(),
            })?;
            let source_members = match selected_file {
                Some(file) => file_export_ids.get(file),
                None => export_provenance.get(module).map(|owner| &owner.member_ids),
            }
            .and_then(|ids| ids.get(module))
            .ok_or_else(|| ElabError::Internal(format!(
                "facade provider '{module}' has no checked export identities"
            )))?;
            for item in items {
                let canonical = pubmap
                    .get(&item.name)
                    .ok_or_else(|| ElabError::UnboundName {
                        name: format!("{module}.{}", item.name),
                        span: span.clone(),
                    })?;
                let surface = published_name(item);
                let selected_id = source_members.get(&item.name).copied().ok_or_else(|| {
                    ElabError::Internal(format!(
                        "facade member '{module}.{}' has no checked provider ID",
                        item.name
                    ))
                })?;
                publish_checked_identity(scope, exports_here, surface, canonical, selected_id, span)?;
                publish_family_intros(
                    scope, exports_here, prop_intros, surface, canonical,
                    &item.name, Some(source_members), globals, span,
                )?;
                for name in [item.name.as_str(), surface] {
                    if !scope.bindings.contains_key(name) && !globals.contains_key(name) {
                        scope.facade_only.insert(name.to_string());
                    }
                }
            }
        }
        ExportForm::InScope { items } => {
            for item in items {
                let had_scope_binding = scope.bindings.contains_key(&item.name);
                let canonical = resolve_ref(scope, exports, &item.name, span)?;
                if !had_scope_binding && !globals.contains_key(&canonical) {
                    return Err(ElabError::UnboundName {
                        name: item.name.clone(),
                        span: span.clone(),
                    });
                }
                let surface = published_name(item);
                if scope.current_local_names.contains(&item.name) {
                    // The same unit can export a local before it is checked.
                    // Even if another unit already owns this canonical name,
                    // that ambient ID is not the forward local's identity.
                    publish_identity(exports_here, surface, &canonical, span)?;
                    scope.pending_local_exports.insert(
                        surface.to_string(), (canonical, span.clone()),
                    );
                    continue;
                }
                let selected_id = scope
                    .qualified_ids
                    .get(&item.name)
                    .or_else(|| scope.binding_ids.get(&item.name))
                    .copied()
                    .or_else(|| {
                        // Locals checked earlier in this unit have their ID
                        // in `globals`. An imported binding must already have
                        // its selected ID; the mutable table is not evidence.
                        (scope.locals.contains(&item.name)
                            || (!had_scope_binding && !item.name.contains('.')))
                            .then(|| globals.get(&canonical).copied())
                            .flatten()
                    })
                    .ok_or_else(|| ElabError::UnboundName {
                        name: item.name.clone(),
                        span: span.clone(),
                    })?;
                publish_checked_identity(
                    scope, exports_here, surface, &canonical, selected_id, span,
                )?;
                let selected_members = (scope.qualified_ids.contains_key(&item.name)
                    || scope.binding_ids.contains_key(&item.name))
                    .then(|| scope.qualified_ids.clone());
                let source_members = selected_members.as_ref();
                publish_family_intros(
                    scope, exports_here, prop_intros, surface, &canonical,
                    &item.name, source_members, globals, span,
                )?;
            }
        }
    }
    Ok(())
}

fn declared_module_paths(decls: &[Decl], prefix: &str, out: &mut HashSet<String>) {
    for decl in decls {
        if let Decl::ModuleDecl {
            name, decls: inner, ..
        } = decl.unwrap_pub()
        {
            let path = qualify(prefix, name);
            out.insert(path.clone());
            declared_module_paths(inner, &path, out);
        }
    }
}

fn imported_module_paths(decls: &[Decl], owner: &str, out: &mut Vec<(String, String, Span)>) {
    for decl in decls {
        match decl.unwrap_pub() {
            Decl::ImportDecl { module, span, .. } => {
                out.push((module.clone(), owner.to_string(), span.clone()));
            }
            Decl::ExportDecl {
                form: ExportForm::Facade { module, .. },
                span,
            } => out.push((module.clone(), owner.to_string(), span.clone())),
            Decl::ModuleDecl {
                name, decls: inner, ..
            } => imported_module_paths(inner, &qualify(owner, name), out),
            _ => {}
        }
    }
}

fn declared_inline_import(
    owner: &str,
    file_root: Option<&str>,
    module: &str,
    declared: &HashSet<String>,
) -> Option<String> {
    if module.contains('.') {
        return None;
    }
    let mut current = owner;
    loop {
        // The empty prefix is the lexical root of a direct in-memory unit,
        // but never an ancestor of a loaded file unit's dotted path.
        if current.is_empty() && file_root.is_some() {
            break;
        }
        let canonical = qualify(current, module);
        if declared.contains(&canonical) {
            return Some(canonical);
        }
        if current.is_empty() || file_root == Some(current) {
            break;
        }
        current = current.rsplit_once('.').map_or("", |(parent, _)| parent);
    }
    None
}

fn admission_boundary(decls: &[Decl]) -> Result<Option<(BoundaryHeader, Span)>, ElabError> {
    let mut found = None;
    for (index, decl) in decls.iter().enumerate() {
        if let Decl::BoundaryDecl {
            kind,
            admits,
            capabilities,
            allow_root_execution,
            span,
        } = decl.unwrap_pub()
        {
            if decl.is_pub() || index != 0 || found.is_some() {
                return Err(ElabError::ParseError {
                    msg:
                        "an anonymous program/package boundary must be the single first file header"
                            .to_string(),
                    span: span.clone(),
                });
            }
            found = Some((
                BoundaryHeader {
                    kind: *kind,
                    admits: admits.clone(),
                    capabilities: capabilities.clone(),
                    allow_root_execution: *allow_root_execution,
                },
                span.clone(),
            ));
        }
    }
    Ok(found)
}

fn valid_module_component(component: &str) -> bool {
    let mut chars = component.chars();
    chars.next().is_some_and(|first| first.is_ascii_uppercase())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'')
}

/// The single catalog root and dotted module entry named by a source path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogModulePath {
    pub root: PathBuf,
    pub entry: String,
}

/// Invert the catalog-root path mapping used by [`source_path`].
///
/// The nearest `catalog/packages` ancestor is the root. Paths outside such a
/// root, paths without a source extension, and paths containing an invalid
/// module component are not catalog module paths and return `None` so callers
/// can retain their isolated-file behavior.
pub fn catalog_module_from_path(path: &Path) -> Option<CatalogModulePath> {
    let parent = path.parent()?;
    let root = parent.ancestors().find(|ancestor| {
        ancestor.file_name().is_some_and(|name| name == "packages")
            && ancestor
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|name| name == "catalog")
    })?;
    let relative = path.strip_prefix(root).ok()?;
    let mut components = relative.iter().collect::<Vec<_>>();
    let leaf = components.pop()?.to_str()?;
    let leaf = leaf
        .strip_suffix(".ken.md")
        .or_else(|| leaf.strip_suffix(".ken"))?;

    let mut module = components
        .into_iter()
        .map(|component| component.to_str())
        .collect::<Option<Vec<_>>>()?;
    module.push(leaf);
    if module.is_empty() || !module.iter().copied().all(valid_module_component) {
        return None;
    }
    Some(CatalogModulePath {
        root: root.to_path_buf(),
        entry: module.join("."),
    })
}

fn source_path(root: &Path, module: &str, span: &Span) -> Result<PathBuf, ElabError> {
    if module.is_empty() || !module.split('.').all(valid_module_component) {
        return Err(ElabError::ParseError {
            msg: format!("invalid dotted module path '{module}'"),
            span: span.clone(),
        });
    }
    let mut stem = root.to_path_buf();
    for component in module.split('.') {
        stem.push(component);
    }

    // The strict bijection makes a path position a leaf or a directory, never
    // both. It also permits exactly one source spelling for the leaf.
    let ken = stem.with_extension("ken");
    let ken_md = stem.with_extension("ken.md");
    let existing: Vec<PathBuf> = [ken, ken_md]
        .into_iter()
        .filter(|path| path.is_file())
        .collect();
    if stem.is_dir() && !existing.is_empty() {
        return Err(ElabError::ParseError {
            msg: format!("module path '{module}' is both a source leaf and a directory"),
            span: span.clone(),
        });
    }
    match existing.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(ElabError::UnboundName {
            name: module.to_string(),
            span: span.clone(),
        }),
        _ => Err(ElabError::ParseError {
            msg: format!("module path '{module}' has both .ken and .ken.md source leaves"),
            span: span.clone(),
        }),
    }
}

struct ParsedUnit {
    decls: Vec<Decl>,
    literate: Option<(String, crate::literate::KenMdExtraction)>,
}

fn parse_unit_source(path: &Path, span: &Span) -> Result<ParsedUnit, ElabError> {
    let source = std::fs::read_to_string(path).map_err(|error| ElabError::ParseError {
        msg: format!("failed to read module source '{}': {error}", path.display()),
        span: span.clone(),
    })?;
    if path.to_string_lossy().ends_with(".ken.md") {
        let extracted = crate::literate::extract_ken_md(&source)?;
        crate::literate::validate_ken_md_fences(&extracted)?;
        let decls = crate::parser::parse_decls(&extracted.source)?;
        Ok(ParsedUnit {
            decls,
            literate: Some((source, extracted)),
        })
    } else {
        Ok(ParsedUnit {
            decls: crate::parser::parse_decls(&source)?,
            literate: None,
        })
    }
}

fn refresh_carried_instance_admission(elab: &mut ElabEnv) {
    let Some(admitted) = elab.class_env.direct_use_packages.as_ref() else {
        return;
    };
    let public_identities: HashSet<&str> = admitted
        .iter()
        .filter_map(|package| elab.module_state.exports.get(package))
        .flat_map(|pubmap| pubmap.values().map(String::as_str))
        .collect();
    let carried: Vec<ken_kernel::GlobalId> = elab
        .class_env
        .instances
        .iter()
        .filter_map(|((class_name, head_name), info)| {
            (public_identities.contains(class_name.as_str())
                || public_identities.contains(head_name.as_str()))
            .then_some(info.instance_id)
        })
        .collect();
    elab.class_env.direct_use_instances.extend(carried);
}

/// Load one file unit through the active-stack gate. Import edges are
/// discovered before `expand_scope`, so a cyclic unit is rejected before any
/// of that unit's declarations are admitted to the flat kernel environment.
fn load_unit(
    elab: &mut ElabEnv,
    module: &str,
    span: &Span,
    mode: ResolutionMode,
) -> Result<Vec<ken_kernel::GlobalId>, ElabError> {
    if let Some(start) = elab
        .module_state
        .active_imports
        .iter()
        .position(|active| active == module)
    {
        let mut cycle = elab.module_state.active_imports[start..].to_vec();
        cycle.push(module.to_string());
        return Err(ElabError::ImportCycle {
            cycle,
            span: span.clone(),
        });
    }
    if let Some(ids) = elab.module_state.loaded_units.get(module) {
        return Ok(ids.clone());
    }

    let root = elab
        .module_state
        .catalog_roots
        .first()
        .expect("N2 root count is checked at the public entry point")
        .clone();
    let path = source_path(&root, module, span)?;
    let ParsedUnit { decls, literate } = parse_unit_source(&path, span)?;

    let previous_package = elab.class_env.current_package.clone();
    let previous_direct_use = elab.class_env.direct_use_packages.clone();
    let previous_direct_instances = elab.class_env.direct_use_instances.clone();
    let previous_implicit_single_provider = elab.class_env.implicit_single_provider;
    let boundary = admission_boundary(&decls)?;
    let has_boundary = boundary.is_some();
    let root_unit = previous_package.is_none() && previous_direct_use.is_none();
    elab.class_env.current_package = Some(module.to_string());
    elab.class_env.direct_use_packages = match &boundary {
        Some((header, _)) => Some(
            header
                .admits
                .clone()
                .unwrap_or_default()
                .into_iter()
                .collect(),
        ),
        None if previous_package.is_none() && previous_direct_use.is_none() => Some(HashSet::new()),
        None => previous_direct_use.clone(),
    };
    if has_boundary || root_unit {
        elab.class_env.direct_use_instances.clear();
    }
    elab.class_env.implicit_single_provider = if has_boundary {
        false
    } else if previous_package.is_none() && previous_direct_use.is_none() {
        true
    } else {
        previous_implicit_single_provider
    };

    elab.module_state.active_imports.push(module.to_string());
    let result = (|| {
        let mut local_modules = HashSet::new();
        declared_module_paths(&decls, module, &mut local_modules);
        let mut imports = Vec::new();
        imported_module_paths(&decls, module, &mut imports);
        for (dependency, owner, import_span) in imports {
            // A same-unit child is never an absolute file dependency, even if
            // declared later: the ordered pass will reject its premature use
            // at the import. An unrelated global export is not a file load.
            if declared_inline_import(&owner, Some(module), &dependency, &local_modules).is_none() {
                load_unit(elab, &dependency, &import_span, mode)?;
            }
        }
        refresh_carried_instance_admission(elab);

        // Every loaded source unit is an ordinary orphan-check module. Assign
        // its id only after dependencies return so their current-module ids do
        // not leak into this unit's declarations.
        elab.class_env.next_module();

        let mut scope = Scope::with_mode(mode, elab.module_state.strict_builtin_names.clone());
        let mut unit_definitions = HashSet::new();
        let mut ordered_inline_modules = HashSet::new();
        let (results, exports) = expand_scope(
            elab,
            &decls,
            module,
            &local_modules,
            &mut ordered_inline_modules,
            &mut scope,
            &mut unit_definitions,
            true,
        )?;
        let ids: Vec<ken_kernel::GlobalId> =
            results.into_iter().map(|result| result.def_id).collect();
        // A file's descendants are those it declared AND expanded, not
        // similarly spelled edges retained from another source unit. Keep
        // the current import provenance paired with this export-table write;
        // the file-specific path remains cached for explicit roots imports.
        let owned_paths: HashSet<String> = local_modules
            .intersection(&ordered_inline_modules)
            .cloned()
            .collect();
        let mut file_tables: HashMap<String, HashMap<String, String>> = owned_paths
            .iter()
            .filter_map(|path| {
                elab.module_state
                    .exports
                    .get(path)
                    .map(|pubmap| (path.clone(), pubmap.clone()))
            })
            .collect();
        file_tables.insert(module.to_string(), exports.clone());
        let mut member_ids: ProviderExportIds = owned_paths
            .iter()
            .filter_map(|path| {
                elab.module_state
                    .export_provenance
                    .get(path)
                    .and_then(|owner| owner.member_ids.get(path))
                    .map(|ids| (path.clone(), ids.clone()))
            })
            .collect();
        member_ids.insert(
            module.to_string(),
            checked_export_ids(&mut scope, &exports, &elab.globals)?,
        );
        elab.module_state
            .file_export_tables
            .insert(module.to_string(), file_tables);
        elab.module_state
            .file_export_ids
            .insert(module.to_string(), member_ids.clone());
        elab.module_state
            .file_inline_paths
            .insert(module.to_string(), owned_paths.clone());
        elab.module_state.export_provenance.insert(
            module.to_string(),
            ExportProvenance {
                inline_paths: owned_paths,
                file_root: Some(module.to_string()),
                member_ids,
            },
        );
        elab.module_state
            .exports
            .insert(module.to_string(), exports);

        certify_standard_operator_home(elab, module, None)?;
        elab.module_state
            .loaded_unit_scopes
            .insert(module.to_string(), scope);
        Ok(ids)
    })();
    let popped = elab.module_state.active_imports.pop();
    debug_assert_eq!(popped.as_deref(), Some(module));
    elab.class_env.current_package = previous_package;
    elab.class_env.direct_use_packages = previous_direct_use;
    elab.class_env.direct_use_instances = previous_direct_instances;
    elab.class_env.implicit_single_provider = previous_implicit_single_provider;

    let ids = result?;
    if root_unit {
        elab.module_state.boundary_header = boundary.map(|(header, _)| header);
    }
    elab.module_state
        .loaded_units
        .insert(module.to_string(), ids.clone());
    if let Some(literate) = literate {
        elab.module_state
            .loaded_literate_units
            .insert(module.to_string(), literate);
    }
    Ok(ids)
}

/// Execute the document-check obligations for one already-loaded entry unit.
///
/// Dependency loading never calls this function. A front end calls it only for
/// the dotted module selected as its entry, preserving the isolated `.ken.md`
/// contract without turning checked fences into part of a module's interface.
pub(crate) fn execute_loaded_entry_checked_fences(
    elab: &mut ElabEnv,
    entry: &str,
) -> Result<(), ElabError> {
    if !elab.module_state.loaded_units.contains_key(entry) {
        return Err(ElabError::Internal(format!(
            "cannot check fences for unloaded module entry '{entry}'"
        )));
    }
    let Some((source, extracted)) = elab.module_state.loaded_literate_units.get(entry).cloned()
    else {
        return Ok(());
    };
    let scope = elab
        .module_state
        .loaded_unit_scopes
        .get(entry)
        .cloned()
        .ok_or_else(|| {
            ElabError::Internal(format!(
                "loaded module entry '{entry}' has no completed scope"
            ))
        })?;
    elab.module_state.root_scope = scope;
    elab.execute_ken_md_checked_fences(&source, &extracted)
}

/// Plural-root entry point for the N2 in-repo loader (`33 §3.2`).
pub fn elaborate_module_from_roots(
    elab: &mut ElabEnv,
    roots: &[PathBuf],
    entry: &str,
) -> Result<Vec<ken_kernel::GlobalId>, ElabError> {
    elaborate_module_from_roots_with_mode(elab, roots, entry, ResolutionMode::Legacy)
}

/// Opt-in strict roots entry. WP-4 will move the real catalog caller to this
/// entry only after its dependency census has been migrated.
pub fn elaborate_module_from_roots_strict(
    elab: &mut ElabEnv,
    roots: &[PathBuf],
    entry: &str,
) -> Result<Vec<ken_kernel::GlobalId>, ElabError> {
    elaborate_module_from_roots_with_mode(elab, roots, entry, ResolutionMode::Strict)
}

fn elaborate_module_from_roots_with_mode(
    elab: &mut ElabEnv,
    roots: &[PathBuf],
    entry: &str,
    mode: ResolutionMode,
) -> Result<Vec<ken_kernel::GlobalId>, ElabError> {
    // The public globals map may change after the pre-source capture. Recheck
    // before loading or reusing any strict unit; do not heal a forged map.
    if mode == ResolutionMode::Strict {
        require_fixed_proved_identity(&elab.env, &elab.globals)?;
    }
    if roots.len() != 1 {
        return Err(ElabError::ParseError {
            msg: format!(
                "N2 requires exactly one populated catalog root, found {}",
                roots.len()
            ),
            span: Span::zero(),
        });
    }
    if elab.module_state.catalog_roots.is_empty() {
        elab.module_state.catalog_roots = roots.to_vec();
        elab.module_state.roots_resolution_mode = Some(mode);
    } else if elab.module_state.catalog_roots != roots {
        return Err(ElabError::ParseError {
            msg: "catalog roots cannot change during one elaboration run".to_string(),
            span: Span::zero(),
        });
    } else if elab.module_state.roots_resolution_mode != Some(mode) {
        return Err(ElabError::Internal(
            "roots resolution mode cannot change during one elaboration run".to_string(),
        ));
    }
    load_unit(elab, entry, &Span::zero(), mode)
}

/// Rename the declared name(s) of a raw surface `Decl` to their fully
/// qualified form (`prefix.name`), leaving every reference *inside* the
/// decl's body/type/etc untouched (those are qualified later, post-resolve,
/// by `rewrite_rdecl`). Only decl kinds with a single ordinary declared
/// name participate in module qualification (`view`/`let`/`data`/`type`);
/// classes/instances/laws/foreign/temporal/prove decls are elaborated
/// unqualified even inside a module (out of this WP's scope — no seed case
/// exercises them nested).
fn qualify_decl_name(decl: &Decl, prefix: &str) -> Decl {
    match decl {
        Decl::ViewDecl {
            keyword,
            name,
            params,
            ret_ty,
            requires,
            ensures,
            constraints,
            visits,
            body,
            is_space_op,
            span,
        } => Decl::ViewDecl {
            keyword: *keyword,
            name: qualify(prefix, name),
            params: params.clone(),
            ret_ty: ret_ty.clone(),
            requires: requires.clone(),
            ensures: ensures.clone(),
            constraints: constraints.clone(),
            visits: visits.clone(),
            body: body.clone(),
            is_space_op: *is_space_op,
            span: span.clone(),
        },
        Decl::SpaceDecl {
            name,
            cells,
            operations,
            span,
        } => Decl::SpaceDecl {
            name: qualify(prefix, name),
            cells: cells.clone(),
            operations: operations.clone(),
            span: span.clone(),
        },
        Decl::LetDecl {
            name,
            ty,
            val,
            span,
        } => Decl::LetDecl {
            name: qualify(prefix, name),
            ty: ty.clone(),
            val: val.clone(),
            span: span.clone(),
        },
        Decl::DataDecl {
            name,
            type_params,
            ctors,
            span,
        } => Decl::DataDecl {
            name: qualify(prefix, name),
            type_params: type_params.clone(),
            ctors: ctors
                .iter()
                .map(|c| CtorDecl {
                    name: qualify(prefix, &c.name),
                    args: c.args.clone(),
                    field_labels: c.field_labels.clone(),
                    span: c.span.clone(),
                })
                .collect(),
            span: span.clone(),
        },
        Decl::ExplicitDataDecl {
            name,
            params,
            family,
            ctors,
            span,
        } => Decl::ExplicitDataDecl {
            name: qualify(prefix, name),
            params: params.clone(),
            family: family.clone(),
            ctors: ctors
                .iter()
                .map(|c| match c {
                    ExplicitDataCtor::Simple(c) => ExplicitDataCtor::Simple(CtorDecl {
                        name: qualify(prefix, &c.name),
                        args: c.args.clone(),
                        field_labels: c.field_labels.clone(),
                        span: c.span.clone(),
                    }),
                    ExplicitDataCtor::Signature {
                        name,
                        signature,
                        span,
                    } => ExplicitDataCtor::Signature {
                        name: qualify(prefix, name),
                        signature: signature.clone(),
                        span: span.clone(),
                    },
                })
                .collect(),
            span: span.clone(),
        },
        Decl::TypeAlias { name, ty, span } => Decl::TypeAlias {
            name: qualify(prefix, name),
            ty: ty.clone(),
            span: span.clone(),
        },
        Decl::PropDecl {
            name,
            params,
            ret_ty,
            intros,
            span,
        } => Decl::PropDecl {
            name: qualify(prefix, name),
            params: params.clone(),
            ret_ty: ret_ty.clone(),
            intros: intros.clone(),
            span: span.clone(),
        },
        Decl::TheoremDecl {
            name,
            params,
            theorem,
            body,
            span,
        } => Decl::TheoremDecl {
            name: qualify(prefix, name),
            params: params.clone(),
            theorem: theorem.clone(),
            body: body.clone(),
            span: span.clone(),
        },
        Decl::AxiomDecl {
            name,
            theorem,
            span,
        } => Decl::AxiomDecl {
            name: qualify(prefix, name),
            theorem: theorem.clone(),
            span: span.clone(),
        },
        Decl::AttachedProofDecl {
            proof_name,
            subject,
            params,
            theorem,
            body,
            span,
        } => Decl::AttachedProofDecl {
            proof_name: proof_name.clone(),
            subject: subject.clone(),
            params: params.clone(),
            theorem: theorem.clone(),
            body: body.clone(),
            span: span.clone(),
        },
        other => other.clone(),
    }
}

fn rtype_kernel_head(ty: &RType) -> Option<&'static str> {
    let mut cursor = ty;
    let mut arity = 0;
    while let RType::RApp(function, _, _) = cursor {
        arity += 1;
        cursor = function;
    }
    matches!(cursor, RType::RCon(name, _) if name == crate::resolve::SUGAR_EQ && arity == 3)
        .then_some(crate::resolve::SUGAR_EQ)
}

fn rewrite_rtype(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    ty: RType,
) -> Result<RType, ElabError> {
    let kernel_head = rtype_kernel_head(&ty);
    rewrite_rtype_inner(scope, exports, ty, kernel_head)
}

fn rewrite_rtype_inner(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    ty: RType,
    kernel_head: Option<&'static str>,
) -> Result<RType, ElabError> {
    Ok(match ty {
        RType::RCon(name, span) if kernel_head == Some(name.as_str()) => RType::RCon(name, span),
        RType::RCon(name, span) => {
            let (name, selected) = resolve_checked_ref(scope, exports, &name, &span)?;
            match selected {
                Some(id) => RType::RCheckedGlobal { name, id, span },
                None => RType::RCon(name, span),
            }
        }
        RType::RCheckedGlobal { name, id, span } => RType::RCheckedGlobal { name, id, span },
        RType::RVarTy(i, n, s) => RType::RVarTy(i, n, s),
        RType::RPatternAliasTy(slot, name, span) => RType::RPatternAliasTy(slot, name, span),
        RType::RUniv(l, s) => RType::RUniv(l, s),
        RType::RPi(x, a, b, s) => RType::RPi(
            x,
            Box::new(rewrite_rtype(scope, exports, *a)?),
            Box::new(rewrite_rtype(scope, exports, *b)?),
            s,
        ),
        RType::RSigma(x, a, b, s) => RType::RSigma(
            x,
            Box::new(rewrite_rtype(scope, exports, *a)?),
            Box::new(rewrite_rtype(scope, exports, *b)?),
            s,
        ),
        RType::RArr(a, b, s) => RType::RArr(
            Box::new(rewrite_rtype(scope, exports, *a)?),
            Box::new(rewrite_rtype(scope, exports, *b)?),
            s,
        ),
        RType::REffectArr(a, row, b, s) => RType::REffectArr(
            Box::new(rewrite_rtype(scope, exports, *a)?),
            row,
            Box::new(rewrite_rtype(scope, exports, *b)?),
            s,
        ),
        RType::RRefine(x, a, phi, s) => RType::RRefine(
            x,
            Box::new(rewrite_rtype(scope, exports, *a)?),
            Box::new(rewrite_rexpr(scope, exports, *phi)?),
            s,
        ),
        RType::RApp(f, a, s) => RType::RApp(
            Box::new(rewrite_rtype_inner(scope, exports, *f, kernel_head)?),
            Box::new(rewrite_rtype(scope, exports, *a)?),
            s,
        ),
        RType::RTrunc(a, s) => {
            RType::RTrunc(Box::new(rewrite_rtype(scope, exports, *a)?), s)
        }
        // Rewrite the base through the EXPRESSION rewriter, as `RRefine` does
        // with its predicate: the projected object can name an imported
        // binding, so skipping it would leave an unrewritten reference that
        // fails to resolve only in a module context.
        RType::RProj(base, field, s) => {
            RType::RProj(Box::new(rewrite_rexpr(scope, exports, *base)?), field, s)
        }
    })
}

fn rexpr_kernel_head(expr: &RExpr) -> Option<&'static str> {
    let mut cursor = expr;
    let mut arity = 0;
    while let RExpr::RApp(function, _, _) = cursor {
        arity += 1;
        cursor = function;
    }
    let RExpr::RCon(name, _) = cursor else {
        return None;
    };
    match (name.as_str(), arity) {
        (crate::resolve::SUGAR_ABSURD, 1) => Some(crate::resolve::SUGAR_ABSURD),
        (crate::resolve::SUGAR_TRUNC_INTRO, 1) => Some(crate::resolve::SUGAR_TRUNC_INTRO),
        (crate::resolve::SUGAR_J, 3) => Some(crate::resolve::SUGAR_J),
        (crate::resolve::SUGAR_EQ, 3) => Some(crate::resolve::SUGAR_EQ),
        (crate::resolve::SUGAR_ELIM_TRUNC, 3) => Some(crate::resolve::SUGAR_ELIM_TRUNC),
        _ => None,
    }
}

fn rewrite_rexpr(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    e: RExpr,
) -> Result<RExpr, ElabError> {
    let kernel_head = rexpr_kernel_head(&e);
    rewrite_rexpr_inner(scope, exports, e, kernel_head)
}

/// Keep per-variant temporaries out of the recursive dispatcher's stack frame.
/// Each closure monomorphizes to its own non-inlined call, so adding a large
/// `RExpr` arm no longer taxes every level of every unrelated descent.
#[inline(never)]
fn rewrite_rexpr_arm(
    rewrite: impl FnOnce() -> Result<RExpr, ElabError>,
) -> Result<RExpr, ElabError> {
    rewrite()
}

fn rewrite_rexpr_inner(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    e: RExpr,
    kernel_head: Option<&'static str>,
) -> Result<RExpr, ElabError> {
    Ok(match e {
        RExpr::RCon(name, span) if kernel_head == Some(name.as_str()) => RExpr::RCon(name, span),
        RExpr::RCon(name, span) => {
            let (name, selected) = resolve_checked_ref(scope, exports, &name, &span)?;
            match selected {
                Some(id) => RExpr::RCheckedGlobal { name, id, span },
                None => RExpr::RCon(name, span),
            }
        }
        RExpr::RCheckedGlobal { name, id, span } => RExpr::RCheckedGlobal { name, id, span },
        RExpr::RVar(i, n, s) => RExpr::RVar(i, n, s),
        RExpr::RPatternAlias(slot, n, s) => RExpr::RPatternAlias(slot, n, s),
        RExpr::RRecursiveResult {
            selector,
            index,
            name,
            binding_span,
            span,
        } => RExpr::RRecursiveResult {
            selector,
            index,
            name,
            binding_span,
            span,
        },
        RExpr::RUniv(l, s) => RExpr::RUniv(l, s),
        RExpr::RApp(f, a, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RApp(
                Box::new(rewrite_rexpr_inner(scope, exports, *f, kernel_head)?),
                Box::new(rewrite_rexpr(scope, exports, *a)?),
                s,
            ))
        })?,
        RExpr::RLam(n, b, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RLam(
                n,
                Box::new(rewrite_rexpr(scope, exports, *b)?),
                s,
            ))
        })?,
        RExpr::RLet(x, ty, rhs, body, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RLet(
                x,
                ty.map(|t| rewrite_rtype(scope, exports, t)).transpose()?,
                Box::new(rewrite_rexpr(scope, exports, *rhs)?),
                Box::new(rewrite_rexpr(scope, exports, *body)?),
                s,
            ))
        })?,
        RExpr::RAsc(e, t, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RAsc(
                Box::new(rewrite_rexpr(scope, exports, *e)?),
                Box::new(rewrite_rtype(scope, exports, *t)?),
                s,
            ))
        })?,
        RExpr::ROld(e, s) => {
            rewrite_rexpr_arm(|| Ok(RExpr::ROld(Box::new(rewrite_rexpr(scope, exports, *e)?), s)))?
        }
        RExpr::RCell(index, name, span) => RExpr::RCell(index, name, span),
        RExpr::RBecomes(index, name, value, span) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RBecomes(
                index,
                name,
                Box::new(rewrite_rexpr(scope, exports, *value)?),
                span,
            ))
        })?,
        RExpr::RNumLit(l, s) => RExpr::RNumLit(l, s),
        RExpr::RStr(v, s) => RExpr::RStr(v, s),
        RExpr::RCharLit(c, s) => RExpr::RCharLit(c, s),
        RExpr::RByteStr(v, s) => RExpr::RByteStr(v, s),
        RExpr::RBinOp(op, l, r, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RBinOp(
                op,
                Box::new(rewrite_rexpr(scope, exports, *l)?),
                Box::new(rewrite_rexpr(scope, exports, *r)?),
                s,
            ))
        })?,
        // THE CROSS-MODULE REMAP, and omitting it would have been INVISIBLE
        // rather than a compile error (Architect, `evt_2y0a3j5yjznn2`). This
        // function is a REWRITE over every node: a variant left out does not
        // fail to compile, it silently stops having its operands remapped, and
        // an operator crossing an import would then complete differently from
        // the same operator in its home module -- which no single-module
        // fixture can see.
        //
        // The IDENTITY is deliberately not remapped. A `GlobalId` is already
        // canonical -- `33 §4.3` says an export republishes the existing one
        // and never mints another -- so there is nothing here for a module
        // boundary to rewrite. That is the whole reason this node carries the
        // identity rather than the glyph: the glyph WOULD have needed
        // remapping, and forgetting it is the invisible failure above.
        RExpr::RStandardOp {
            op,
            lhs,
            rhs,
            span,
        } => rewrite_rexpr_arm(|| {
            Ok(RExpr::RStandardOp {
                op,
                lhs: Box::new(rewrite_rexpr(scope, exports, *lhs)?),
                rhs: Box::new(rewrite_rexpr(scope, exports, *rhs)?),
                span,
            })
        })?,
        RExpr::RInfixSpine {
            operands,
            operators,
            span,
        } => rewrite_rexpr_arm(|| {
            Ok(RExpr::RInfixSpine {
                operands: operands
                    .into_iter()
                    .map(|operand| rewrite_rexpr(scope, exports, operand))
                    .collect::<Result<Vec<_>, _>>()?,
                operators: operators
                    .into_iter()
                    .map(|operator| match operator {
                        RInfixOperator::Builtin(op, span) => Ok(RInfixOperator::Builtin(op, span)),
                        RInfixOperator::User(name, span) => {
                            let (name, selected) = resolve_checked_ref(scope, exports, &name, &span)?;
                            Ok(match selected {
                                Some(id) => RInfixOperator::CheckedUser(name, id, span),
                                None => RInfixOperator::User(name, span),
                            })
                        }
                        RInfixOperator::CheckedUser(name, id, span) => {
                            Ok(RInfixOperator::CheckedUser(name, id, span))
                        }
                    })
                    .collect::<Result<Vec<_>, ElabError>>()?,
                span,
            })
        })?,
        RExpr::RMatch {
            scrut,
            equation,
            arms,
            span,
        } => rewrite_rexpr_arm(|| {
            let scrut = Box::new(rewrite_rexpr(scope, exports, *scrut)?);
            // Keep recursive match descent free of per-arm iterator frames. The
            // iterator/Result collection adds a chain of adapter frames
            // for every nested source match; a finite checked program must not
            // become unresolvable merely because the prelude gained declarations.
            let mut rewritten_arms = Vec::with_capacity(arms.len());
            for arm in arms {
                rewritten_arms.push(RMatchArm {
                    pat: rewrite_rpattern(scope, exports, arm.pat)?,
                    guard: arm
                        .guard
                        .map(|guard| rewrite_rexpr(scope, exports, guard))
                        .transpose()?,
                    body: rewrite_rexpr(scope, exports, arm.body)?,
                    span: arm.span,
                });
            }
            let arms = rewritten_arms;
            Ok(RExpr::RMatch {
                scrut,
                equation,
                arms,
                span,
            })
        })?,
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            span,
        } => rewrite_rexpr_arm(|| {
            Ok(RExpr::RIf {
                condition: Box::new(rewrite_rexpr(scope, exports, *condition)?),
                then_branch: Box::new(rewrite_rexpr(scope, exports, *then_branch)?),
                else_branch: Box::new(rewrite_rexpr(scope, exports, *else_branch)?),
                span,
            })
        })?,
        RExpr::RPair(components, span) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RPair(
                components
                    .into_iter()
                    .map(|component| rewrite_rexpr(scope, exports, component))
                    .collect::<Result<Vec<_>, _>>()?,
                span,
            ))
        })?,
        RExpr::RRecord { base, fields, span } => rewrite_rexpr_arm(|| {
            Ok(RExpr::RRecord {
                base: base
                    .map(|base| rewrite_rexpr(scope, exports, *base).map(Box::new))
                    .transpose()?,
                fields: fields
                    .into_iter()
                    .map(|(name, value, name_span)| {
                        Ok((name, rewrite_rexpr(scope, exports, value)?, name_span))
                    })
                    .collect::<Result<Vec<_>, ElabError>>()?,
                span,
            })
        })?,
        RExpr::RPosProj(e, index, span) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RPosProj(
                Box::new(rewrite_rexpr(scope, exports, *e)?),
                index,
                span,
            ))
        })?,
        RExpr::RProj(e, field, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RProj(
                Box::new(rewrite_rexpr(scope, exports, *e)?),
                field,
                s,
            ))
        })?,
        RExpr::RPi(x, a, b, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RPi(
                x,
                Box::new(rewrite_rtype(scope, exports, *a)?),
                Box::new(rewrite_rexpr(scope, exports, *b)?),
                s,
            ))
        })?,
        RExpr::RArrow(a, b, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RArrow(
                Box::new(rewrite_rexpr(scope, exports, *a)?),
                Box::new(rewrite_rexpr(scope, exports, *b)?),
                s,
            ))
        })?,
        RExpr::RAttachedProofRef {
            subject,
            proof_name,
            span,
        } => rewrite_rexpr_arm(|| {
            let (name, selected) =
                resolve_attached_ref(scope, exports, &subject, &proof_name, &span)?;
            Ok(match selected {
                Some(id) => RExpr::RCheckedGlobal { name, id, span },
                None => RExpr::RCon(name, span),
            })
        })?,
        RExpr::RTrunc(e, s) => rewrite_rexpr_arm(|| {
            Ok(RExpr::RTrunc(
                Box::new(rewrite_rexpr(scope, exports, *e)?),
                s,
            ))
        })?,
    })
}

fn rewrite_rpattern(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    p: RPattern,
) -> Result<RPattern, ElabError> {
    let kind = match p.kind {
        RPatKind::Wild => RPatKind::Wild,
        RPatKind::Var(n, slot) => RPatKind::Var(n, slot),
        RPatKind::Ctor(name, subs) => {
            let (name, selected) = resolve_checked_ref(scope, exports, &name, &p.span)?;
            let subs = subs
                .into_iter()
                .map(|s| rewrite_rpattern(scope, exports, s))
                .collect::<Result<Vec<_>, ElabError>>()?;
            match selected {
                Some(id) => RPatKind::CheckedCtor(name, id, subs),
                None => RPatKind::Ctor(name, subs),
            }
        }
        RPatKind::CheckedCtor(name, id, subs) => RPatKind::CheckedCtor(name, id, subs),
        RPatKind::Tuple(components) => RPatKind::Tuple(
            components
                .into_iter()
                .map(|component| rewrite_rpattern(scope, exports, component))
                .collect::<Result<Vec<_>, ElabError>>()?,
        ),
        RPatKind::Record(fields) => RPatKind::Record(
            fields
                .into_iter()
                .map(|mut field| {
                    field.pattern = rewrite_rpattern(scope, exports, field.pattern)?;
                    Ok(field)
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
        ),
        RPatKind::As(inner, alias, slot) => RPatKind::As(
            Box::new(rewrite_rpattern(scope, exports, *inner)?),
            alias,
            slot,
        ),
        RPatKind::Or(alternatives) => RPatKind::Or(
            alternatives
                .into_iter()
                .map(|alternative| rewrite_rpattern(scope, exports, alternative))
                .collect::<Result<Vec<_>, ElabError>>()?,
        ),
        RPatKind::Literal(literal, slot) => RPatKind::Literal(literal, slot),
    };
    Ok(RPattern { kind, span: p.span })
}

fn rewrite_rdecl(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    rdecl: RDecl,
) -> Result<RDecl, ElabError> {
    // Instance and derive declarations carry their class reference in the
    // declaration-name slot rather than an RCon. Route that direct class
    // family through the same scope/floor decision as every other bare name.
    let direct_class_name = if matches!(
        &rdecl.kind,
        RDeclKind::InstanceDecl { .. } | RDeclKind::DeriveDecl { .. }
    ) {
        Some(resolve_class_ref(scope, exports, &rdecl.name, &rdecl.span)?)
    } else {
        None
    };
    let ty = rdecl
        .ty
        .map(|t| rewrite_rtype(scope, exports, t))
        .transpose()?;
    let body = rewrite_rexpr(scope, exports, rdecl.body)?;
    let requires = rdecl
        .requires
        .into_iter()
        .map(|e| rewrite_rexpr(scope, exports, e))
        .collect::<Result<Vec<_>, ElabError>>()?;
    let ensures = rdecl
        .ensures
        .into_iter()
        .map(|e| rewrite_rexpr(scope, exports, e))
        .collect::<Result<Vec<_>, ElabError>>()?;
    let kind = match rdecl.kind {
        RDeclKind::View {
            keyword,
            is_space_op,
            constraints,
            visits,
        } => RDeclKind::View {
            keyword,
            is_space_op,
            constraints: constraints
                .into_iter()
                .map(|constraint| {
                    Ok(crate::resolve::RInstanceConstraint {
                        class_name: resolve_class_ref(
                            scope,
                            exports,
                            &constraint.class_name,
                            &rdecl.span,
                        )?,
                        head_type: rewrite_rtype(scope, exports, constraint.head_type)?,
                        binder: constraint.binder,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
            visits,
        },
        RDeclKind::Let => RDeclKind::Let,
        RDeclKind::Prove => RDeclKind::Prove,
        RDeclKind::Prop { intros } => RDeclKind::Prop {
            intros: intros
                .into_iter()
                .map(|intro| {
                    Ok(RPropIntro {
                        name: intro.name,
                        ty: rewrite_rtype(scope, exports, intro.ty)?,
                        span: intro.span,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::Theorem => RDeclKind::Theorem,
        RDeclKind::AttachedProof {
            subject,
            proof_name,
        } => RDeclKind::AttachedProof {
            subject: resolve_ref(scope, exports, &subject, &rdecl.span)?,
            proof_name,
        },
        RDeclKind::Law { param, fields } => RDeclKind::Law {
            param,
            fields: fields
                .into_iter()
                .map(|(n, e)| Ok((n, rewrite_rexpr(scope, exports, e)?)))
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::DataDecl { type_params, ctors } => RDeclKind::DataDecl {
            type_params,
            ctors: ctors
                .into_iter()
                .map(|c| {
                    Ok(RCtorDecl {
                        name: c.name,
                        args: c
                            .args
                            .into_iter()
                            .map(|t| rewrite_rtype(scope, exports, t))
                            .collect::<Result<Vec<_>, ElabError>>()?,
                        field_labels: c.field_labels,
                        span: c.span,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::ExplicitDataDecl {
            params,
            indices,
            level,
            ctors,
        } => {
            let rewrite_entry = |entry: RTelescopeEntry| -> Result<RTelescopeEntry, ElabError> {
                Ok(RTelescopeEntry {
                    name: entry.name,
                    ty: rewrite_rtype(scope, exports, entry.ty)?,
                    span: entry.span,
                })
            };
            RDeclKind::ExplicitDataDecl {
                params: params
                    .into_iter()
                    .map(rewrite_entry)
                    .collect::<Result<Vec<_>, ElabError>>()?,
                indices: indices
                    .into_iter()
                    .map(rewrite_entry)
                    .collect::<Result<Vec<_>, ElabError>>()?,
                level,
                ctors: ctors
                    .into_iter()
                    .map(|c| {
                        Ok(RExplicitCtorDecl {
                            name: c.name,
                            args: c
                                .args
                                .into_iter()
                                .map(rewrite_entry)
                                .collect::<Result<Vec<_>, ElabError>>()?,
                            result: c
                                .result
                                .map(|t| rewrite_rtype(scope, exports, t))
                                .transpose()?,
                            span: c.span,
                        })
                    })
                    .collect::<Result<Vec<_>, ElabError>>()?,
            }
        }
        RDeclKind::TypeAlias { ty } => RDeclKind::TypeAlias {
            ty: rewrite_rtype(scope, exports, ty)?,
        },
        RDeclKind::Foreign {
            symbol,
            library,
            is_pure,
            visits,
        } => RDeclKind::Foreign {
            symbol,
            library,
            is_pure,
            visits,
        },
        RDeclKind::Temporal { formula, source } => RDeclKind::Temporal { formula, source },
        RDeclKind::RecordDecl { fields } => RDeclKind::RecordDecl {
            fields: fields
                .into_iter()
                .map(|field| {
                    Ok(crate::resolve::RRecordField {
                        name: field.name,
                        ty: rewrite_rtype(scope, exports, field.ty)?,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::ClassDecl {
            param,
            param_kind,
            fields,
        } => RDeclKind::ClassDecl {
            param,
            param_kind: param_kind
                .map(|t| rewrite_rtype(scope, exports, t))
                .transpose()?,
            fields: fields
                .into_iter()
                .map(|f| {
                    Ok(crate::resolve::RClassField {
                        purity: f.purity,
                        name: f.name,
                        ty: rewrite_rtype(scope, exports, f.ty)?,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::InstanceDecl {
            head_params,
            head_type,
            constraints,
            fields,
        } => RDeclKind::InstanceDecl {
            head_params,
            head_type: rewrite_rtype(scope, exports, head_type)?,
            constraints: constraints
                .into_iter()
                .map(|constraint| {
                    Ok(crate::resolve::RInstanceConstraint {
                        class_name: resolve_class_ref(
                            scope,
                            exports,
                            &constraint.class_name,
                            &rdecl.span,
                        )?,
                        head_type: rewrite_rtype(scope, exports, constraint.head_type)?,
                        binder: constraint.binder,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?,
            fields: fields
                .into_iter()
                .map(|(n, e)| Ok((n, rewrite_rexpr(scope, exports, e)?)))
                .collect::<Result<Vec<_>, ElabError>>()?,
        },
        RDeclKind::DeriveDecl { data_name } => RDeclKind::DeriveDecl {
            data_name: resolve_ref(scope, exports, &data_name, &rdecl.span)?,
        },
    };
    let name = match &kind {
        RDeclKind::AttachedProof {
            subject,
            proof_name,
        } => format!("{subject}::{proof_name}"),
        RDeclKind::InstanceDecl { .. } | RDeclKind::DeriveDecl { .. } => {
            direct_class_name.expect("direct class declaration name is resolved above")
        }
        _ => rdecl.name,
    };
    Ok(RDecl {
        name,
        ty,
        body,
        requires,
        ensures,
        span: rdecl.span,
        contains_infix_spine: rdecl.contains_infix_spine,
        kind,
    })
}

/// Does this (unwrapped) decl kind participate in module-local-name
/// shadowing / qualification (`view`/`let`/`data`/`type`)? Classes,
/// instances, laws, foreign bindings, temporal obligations, and `prove`
/// obligations are elaborated unqualified even inside a `module { … }`
/// block (out of this WP's scope).
fn is_qualifiable(decl: &Decl) -> bool {
    matches!(
        decl,
        Decl::ViewDecl { .. }
            | Decl::SpaceDecl { .. }
            | Decl::LetDecl { .. }
            | Decl::PropDecl { .. }
            | Decl::TheoremDecl { .. }
            | Decl::AxiomDecl { .. }
            | Decl::AttachedProofDecl { .. }
            | Decl::DataDecl { .. }
            | Decl::ExplicitDataDecl { .. }
            | Decl::TypeAlias { .. }
    )
}

fn is_recursive_candidate(decl: &Decl) -> bool {
    matches!(
        decl,
        Decl::ViewDecl { .. }
            | Decl::LetDecl { .. }
            | Decl::TheoremDecl { .. }
            | Decl::AxiomDecl { .. }
            | Decl::AttachedProofDecl { .. }
    )
}

fn register_effect_row(elab: &mut ElabEnv, result: &crate::elab::ElabResult) {
    if let Some(row) = &result.effect_row_type {
        elab.effect_rows.insert(result.name.clone(), row.clone());
        elab.effect_rows_by_id.insert(result.def_id, row.clone());
    }
    if let Some(fb) = &result.foreign_binding {
        elab.foreign_env.register(result.name.clone(), fb.clone());
        if !fb.effect_row.is_empty() {
            let row = crate::effects::RowType::Concrete(fb.effect_row.clone());
            elab.effect_rows.insert(result.name.clone(), row.clone());
            elab.effect_rows_by_id.insert(result.def_id, row);
        }
    }
}

fn register_declared_effect_row(
    elab: &mut ElabEnv,
    rdecl: &crate::resolve::RDecl,
) -> Result<(), ElabError> {
    if let Some(row) = crate::elab::surface_declared_row_type(rdecl)? {
        elab.effect_rows.insert(rdecl.name.clone(), row);
    }
    Ok(())
}

fn elaborate_checked(
    elab: &mut ElabEnv,
    rdecl: &crate::resolve::RDecl,
    declared_fixity: Option<&PendingFixity>,
) -> Result<crate::elab::ElabResult, ElabError> {
    if declared_fixity.is_none() && !rdecl.contains_infix_spine {
        elaborate_checked_spine_free(elab, rdecl)
    } else {
        elaborate_checked_with_fixity(elab, rdecl, declared_fixity)
    }
}

#[inline(never)]
fn elaborate_checked_spine_free(
    elab: &mut ElabEnv,
    rdecl: &crate::resolve::RDecl,
) -> Result<crate::elab::ElabResult, ElabError> {
    crate::elab::check_surface_purity(
        rdecl, &elab.effect_rows, &elab.effect_rows_by_id, &elab.globals, &elab.class_env,
    )?;
    let standard_operators_here = elab.standard_operators.clone();
    let result = crate::elab::elaborate_rdecl_v1_with_effect_rows(
        &mut elab.env,
        &mut elab.globals,
        &mut elab.num_values,
        &elab.numeric_env,
        &mut elab.class_env,
        &mut elab.resolution_provenance,
        &standard_operators_here,
        &crate::elab::CheckedEffectRows::new(&elab.effect_rows, &elab.effect_rows_by_id),
        &mut elab.fixities,
        &mut elab.fixity_spans,
        &mut elab.ctor_decl_spans,
        None,
        rdecl,
    )?;
    register_effect_row(elab, &result);
    Ok(result)
}

#[inline(never)]
fn elaborate_checked_with_fixity(
    elab: &mut ElabEnv,
    rdecl: &crate::resolve::RDecl,
    declared_fixity: Option<&PendingFixity>,
) -> Result<crate::elab::ElabResult, ElabError> {
    crate::elab::check_surface_purity(
        rdecl, &elab.effect_rows, &elab.effect_rows_by_id, &elab.globals, &elab.class_env,
    )?;
    let standard_operators_here = elab.standard_operators.clone();
    let result = crate::elab::elaborate_rdecl_v1_with_effect_rows(
        &mut elab.env,
        &mut elab.globals,
        &mut elab.num_values,
        &elab.numeric_env,
        &mut elab.class_env,
        &mut elab.resolution_provenance,
        &standard_operators_here,
        &crate::elab::CheckedEffectRows::new(&elab.effect_rows, &elab.effect_rows_by_id),
        &mut elab.fixities,
        &mut elab.fixity_spans,
        &mut elab.ctor_decl_spans,
        declared_fixity.map(|pending| (pending.fixity, pending.declaration_span.clone())),
        rdecl,
    )?;
    if let Some(pending) = declared_fixity {
        install_declared_fixity(elab, &result.name, result.def_id, pending)?;
    }
    register_effect_row(elab, &result);
    Ok(result)
}

fn resolve_scoped_decl(
    decl: &Decl,
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    unit_definitions: &mut HashSet<String>,
) -> Result<RDecl, ElabError> {
    let attached_name = if let Decl::AttachedProofDecl {
        subject,
        proof_name,
        span,
        ..
    } = decl
    {
        Some(format!(
            "{}::{proof_name}",
            resolve_ref(scope, exports, subject, span)?
        ))
    } else {
        None
    };
    let rdecl = resolve::resolve_decl_in_unit(decl, unit_definitions, attached_name.as_deref())?;
    rewrite_rdecl(scope, exports, rdecl)
}

fn reject_prelude_binding(
    bare: &str,
    qualified: &str,
    span: &Span,
    prelude_binding_names: &HashSet<String>,
) -> Result<(), ElabError> {
    if prelude_binding_names.contains(bare) {
        return Err(ElabError::AmbiguousReference {
            name: bare.to_string(),
            sources: vec![format!("<prelude>.{bare}"), qualified.to_string()],
            span: span.clone(),
        });
    }
    Ok(())
}

/// Namespace effects of a parsed declaration, independent of whether its
/// spelling is module-qualifiable. This match is intentionally exhaustive and
/// has no wildcard: adding a declaration form forces an explicit collision-
/// population decision.
enum DeclNamespaceEffect<'a> {
    TopLevelName {
        name: &'a str,
        span: &'a Span,
    },
    ConstructorNames {
        parent: &'a str,
        parent_span: &'a Span,
        constructors: ConstructorNameSource<'a>,
    },
    QualifiedIdentity {
        subject: &'a str,
        proof_name: &'a str,
        span: &'a Span,
    },
    ReferenceWithSynthesizedDictionary {
        class_name: &'a str,
        head_name: &'a str,
        span: &'a Span,
    },
    ReferenceOnly,
    NoBinding,
}

enum ConstructorNameSource<'a> {
    Simple(&'a [CtorDecl]),
    Explicit(&'a [ExplicitDataCtor]),
}

fn named_type_head(ty: &Type) -> Option<&str> {
    match ty {
        Type::TCon(name, _) | Type::TVar(name, _) => Some(name),
        Type::TApp(function, _, _) | Type::TRefine(_, function, _, _) => named_type_head(function),
        Type::TUniv(_, _)
        | Type::TArr(_, _, _)
        | Type::TEffectArr(_, _, _, _)
        | Type::TPi(_, _, _, _)
        | Type::TSigma(_, _, _, _)
        | Type::TTrunc(_, _)
        // A projection names no type head -- see `head_type_name`.
        | Type::TProj(_, _, _) => None,
    }
}

fn canonical_leaf(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SynthesizedDictionaryName {
    surface: String,
    canonical: String,
    class_canonical: String,
}

fn synthesized_dictionary_name(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    class_name: &str,
    head_name: &str,
    span: &Span,
) -> Result<SynthesizedDictionaryName, ElabError> {
    let resolved_class = resolve_class_ref(scope, exports, class_name, span)?;
    let resolved_head = resolve_ref(scope, exports, head_name, span)?;
    Ok(SynthesizedDictionaryName {
        surface: format!(
            "{}_instance_{}",
            canonical_leaf(&resolved_class),
            canonical_leaf(&resolved_head)
        ),
        canonical: format!("{resolved_class}_instance_{resolved_head}"),
        class_canonical: resolved_class,
    })
}

fn synthesized_dictionary_class_is_imported(
    scope: &Scope,
    source_class: &str,
    resolved_class: &str,
) -> bool {
    source_class.contains('.')
        || (!scope.locals.contains(source_class)
            && scope
                .bindings
                .get(source_class)
                .is_some_and(|canonical| canonical == resolved_class))
}

fn synthesized_dictionary_class_is_exported(
    decls: &[Decl],
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    resolved_class: &str,
) -> bool {
    decls.iter().any(|decl| match decl {
        Decl::Pub(inner) if matches!(inner.as_ref(), Decl::ClassDecl { .. }) => {
            resolve_class_ref(scope, exports, inner.name(), inner.span())
                .is_ok_and(|canonical| canonical == resolved_class)
        }
        Decl::ExportDecl {
            form: ExportForm::InScope { items },
            span,
        } => items.iter().any(|item| {
            resolve_ref(scope, exports, &item.name, span)
                .is_ok_and(|canonical| canonical == resolved_class)
        }),
        _ => false,
    })
}

fn decl_namespace_effect(decl: &Decl) -> DeclNamespaceEffect<'_> {
    match decl {
        Decl::Pub(inner) => decl_namespace_effect(inner),
        Decl::ViewDecl { name, span, .. }
        | Decl::SpaceDecl { name, span, .. }
        | Decl::LetDecl { name, span, .. }
        | Decl::ProveDecl { name, span, .. }
        | Decl::PropDecl { name, span, .. }
        | Decl::TheoremDecl { name, span, .. }
        | Decl::AxiomDecl { name, span, .. }
        | Decl::LawDecl { name, span, .. }
        | Decl::TypeAlias { name, span, .. }
        | Decl::ForeignDecl { name, span, .. }
        | Decl::TemporalDecl { name, span, .. }
        | Decl::RecordDecl { name, span, .. }
        | Decl::ClassDecl { name, span, .. } => DeclNamespaceEffect::TopLevelName { name, span },
        Decl::DataDecl {
            name, ctors, span, ..
        } => DeclNamespaceEffect::ConstructorNames {
            parent: name,
            parent_span: span,
            constructors: ConstructorNameSource::Simple(ctors),
        },
        Decl::ExplicitDataDecl {
            name, ctors, span, ..
        } => DeclNamespaceEffect::ConstructorNames {
            parent: name,
            parent_span: span,
            constructors: ConstructorNameSource::Explicit(ctors),
        },
        Decl::AttachedProofDecl {
            proof_name,
            subject,
            span,
            ..
        } => DeclNamespaceEffect::QualifiedIdentity {
            subject,
            proof_name,
            span,
        },
        Decl::InstanceDecl {
            class_name,
            head_type,
            span,
            ..
        } => match named_type_head(head_type) {
            Some(head_name) => DeclNamespaceEffect::ReferenceWithSynthesizedDictionary {
                class_name,
                head_name,
                span,
            },
            None => DeclNamespaceEffect::ReferenceOnly,
        },
        Decl::DeriveDecl {
            class_name,
            data_name,
            span,
        } => DeclNamespaceEffect::ReferenceWithSynthesizedDictionary {
            class_name,
            head_name: data_name,
            span,
        },
        Decl::ImportDecl { .. } | Decl::ExportDecl { .. } => DeclNamespaceEffect::ReferenceOnly,
        Decl::BoundaryDecl { .. } | Decl::FixityDecl { .. } | Decl::ModuleDecl { .. } => {
            DeclNamespaceEffect::NoBinding
        }
    }
}

fn reject_decl_prelude_bindings(
    decl: &Decl,
    prefix: &str,
    prelude_binding_names: &HashSet<String>,
) -> Result<(), ElabError> {
    let reject = |bare: &str, span: &Span| {
        reject_prelude_binding(bare, &qualify(prefix, bare), span, prelude_binding_names)
    };
    match decl_namespace_effect(decl) {
        DeclNamespaceEffect::TopLevelName { name, span } => reject(name, span),
        DeclNamespaceEffect::ConstructorNames {
            parent,
            parent_span,
            constructors,
        } => {
            reject(parent, parent_span)?;
            match constructors {
                ConstructorNameSource::Simple(constructors) => {
                    for constructor in constructors {
                        reject(&constructor.name, &constructor.span)?;
                    }
                }
                ConstructorNameSource::Explicit(constructors) => {
                    for constructor in constructors {
                        let (name, span) = match constructor {
                            ExplicitDataCtor::Simple(constructor) => {
                                (&constructor.name, &constructor.span)
                            }
                            ExplicitDataCtor::Signature { name, span, .. } => (name, span),
                        };
                        reject(name, span)?;
                    }
                }
            }
            Ok(())
        }
        DeclNamespaceEffect::QualifiedIdentity {
            subject,
            proof_name,
            span,
        } => {
            let identity = format!("{subject}::{proof_name}");
            reject_prelude_binding(
                &identity,
                &qualify(prefix, &identity),
                span,
                prelude_binding_names,
            )
        }
        DeclNamespaceEffect::ReferenceWithSynthesizedDictionary {
            class_name,
            head_name,
            span,
        } => {
            let surface = format!(
                "{}_instance_{}",
                canonical_leaf(class_name),
                canonical_leaf(head_name)
            );
            reject_prelude_binding(
                &surface,
                &qualify(prefix, &surface),
                span,
                prelude_binding_names,
            )
        }
        DeclNamespaceEffect::ReferenceOnly | DeclNamespaceEffect::NoBinding => Ok(()),
    }
}

fn prebind_scope_declarations(
    scope: &mut Scope,
    decls: &[Decl],
    prefix: &str,
    file_root: Option<&str>,
    unit_inline_modules: &HashSet<String>,
    ordered_inline_modules: &HashSet<String>,
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    prelude_binding_names: &HashSet<String>,
    inline_children: &HashMap<String, HashSet<String>>,
    file_inline_paths: &HashMap<String, HashSet<String>>,
    file_export_tables: &FileExportTables,
    file_export_ids: &HashMap<String, ProviderExportIds>,
    export_provenance: &HashMap<String, ExportProvenance>,
    exports_here: &mut HashMap<String, String>,
) -> Result<(), ElabError> {
    // Collision population follows declaration namespace effects, not the
    // separate qualification taxonomy. Reject the whole scope before binding
    // or elaborating any declaration so refusal cannot allocate or replace a
    // canonical global.
    for decl in decls {
        reject_decl_prelude_bindings(decl, prefix, prelude_binding_names)?;
    }

    // Collect locals before imports so collisions are source-order independent.
    // This persistent module scope is separate from `expand_scope`'s recursive
    // application-spine frames, so local constructor/class binding is identical
    // in legacy and strict resolution without spending the legacy stack budget.
    for decl in decls {
        let inner = decl.unwrap_pub();
        let unqualified_local = matches!(inner, Decl::ClassDecl { .. });
        if !is_qualifiable(inner) && !unqualified_local {
            continue;
        }
        if let Decl::AttachedProofDecl { subject, proof_name, .. } = inner {
            scope.local_attached_proofs.insert(format!("{subject}::{proof_name}"));
            continue;
        }
        let bare = inner.name().to_string();
        scope.current_local_names.insert(bare.clone());
        let qualified = if unqualified_local {
            bare.clone()
        } else {
            qualify(prefix, &bare)
        };
        scope.bind_local(&bare, &qualified, inner.span())?;
        match inner {
            Decl::DataDecl { ctors, .. } => {
                for ctor in ctors {
                    scope.current_local_names.insert(ctor.name.clone());
                    let qualified = qualify(prefix, &ctor.name);
                    scope.bind_local(&ctor.name, &qualified, &ctor.span)?;
                }
            }
            Decl::ExplicitDataDecl { ctors, .. } => {
                for ctor in ctors {
                    let (name, span) = match ctor {
                        ExplicitDataCtor::Simple(ctor) => (&ctor.name, &ctor.span),
                        ExplicitDataCtor::Signature { name, span, .. } => (name, span),
                    };
                    scope.current_local_names.insert(name.clone());
                    let qualified = qualify(prefix, name);
                    scope.bind_local(name, &qualified, span)?;
                }
            }
            _ => {}
        }
    }

    prebind_synthesized_dictionaries(
        scope,
        decls,
        prefix,
        file_root,
        unit_inline_modules,
        ordered_inline_modules,
        exports,
        globals,
        prelude_binding_names,
        inline_children,
        file_inline_paths,
        file_export_tables,
        file_export_ids,
        export_provenance,
        exports_here,
    )
}

fn prebind_synthesized_dictionaries(
    scope: &mut Scope,
    decls: &[Decl],
    prefix: &str,
    file_root: Option<&str>,
    unit_inline_modules: &HashSet<String>,
    ordered_inline_modules: &HashSet<String>,
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    prelude_binding_names: &HashSet<String>,
    inline_children: &HashMap<String, HashSet<String>>,
    file_inline_paths: &HashMap<String, HashSet<String>>,
    file_export_tables: &FileExportTables,
    file_export_ids: &HashMap<String, ProviderExportIds>,
    export_provenance: &HashMap<String, ExportProvenance>,
    exports_here: &mut HashMap<String, String>,
) -> Result<(), ElabError> {
    // Instance and derive declarations reference a class, but also produce one
    // ordinary global dictionary. A scope with no producer needs no import
    // replay, whether before or after its same-file children expand.
    let has_synthesized_dictionary = decls.iter().any(|decl| {
        matches!(
            decl_namespace_effect(decl.unwrap_pub()),
            DeclNamespaceEffect::ReferenceWithSynthesizedDictionary { .. }
        )
    });
    if !has_synthesized_dictionary {
        return Ok(());
    }

    // Replay only the textual import effects in a throwaway scope so that the
    // dictionary alias is planned from the same resolved class/head names that
    // synthesis will use, without changing the real ordered import pass below.
    let mut synthesis_scope = scope.clone();
    for decl in decls {
        let inner = decl.unwrap_pub();
        match inner {
            Decl::ImportDecl { module, kind, span } => {
                if let Err(error) = apply_import(
                    &mut synthesis_scope,
                    exports,
                    inline_children,
                    file_inline_paths,
                    file_export_tables,
                    file_export_ids,
                    export_provenance,
                    globals,
                    prelude_binding_names,
                    prefix,
                    file_root,
                    unit_inline_modules,
                    ordered_inline_modules,
                    module,
                    kind,
                    span,
                ) {
                    match error {
                        // A same-file module is expanded only by the real
                        // ordered pass. Leave unavailable-import diagnostics to
                        // that authoritative pass rather than fabricating a
                        // synthesis identity from unresolved names.
                        ElabError::UnboundName { .. } => {}
                        other => return Err(other),
                    }
                }
            }
            _ => {
                let DeclNamespaceEffect::ReferenceWithSynthesizedDictionary {
                    class_name,
                    head_name,
                    span,
                } = decl_namespace_effect(inner)
                else {
                    continue;
                };
                let name = match synthesized_dictionary_name(
                    &synthesis_scope,
                    exports,
                    class_name,
                    head_name,
                    span,
                ) {
                    Ok(name) => name,
                    // The real ordered pass owns unresolved-name diagnostics;
                    // it may first make a same-file module export available.
                    Err(ElabError::UnboundName { .. }) => continue,
                    Err(other) => return Err(other),
                };
                reject_prelude_binding(
                    &name.surface,
                    &name.canonical,
                    span,
                    prelude_binding_names,
                )?;
                scope.bind_local(&name.surface, &name.canonical, span)?;
                synthesis_scope.bind_local(&name.surface, &name.canonical, span)?;
                let class_is_imported = synthesized_dictionary_class_is_imported(
                    &synthesis_scope,
                    class_name,
                    &name.class_canonical,
                );
                let class_is_exported = synthesized_dictionary_class_is_exported(
                    decls,
                    &synthesis_scope,
                    exports,
                    &name.class_canonical,
                );
                if class_is_imported || class_is_exported {
                    // Visibility is a property of the complete parsed owner
                    // interface, so it can be planned beside local prebinding
                    // without extending `expand_scope`'s legacy stack frame.
                    publish_identity(exports_here, &name.surface, &name.canonical, span)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct PendingFixity {
    /// Pre-admission coordinate produced by ordinary module resolution. It is
    /// never consulted for associativity; once the declaration has an id, the
    /// program `GlobalId -> Fixity` table is the sole carrier.
    canonical_name: String,
    source_operator: String,
    fixity: Fixity,
    declaration_span: Span,
}

fn pending_fixity_for<'a>(
    pending: &'a [PendingFixity],
    canonical_name: &str,
) -> Option<&'a PendingFixity> {
    pending
        .iter()
        .find(|candidate| candidate.canonical_name == canonical_name)
}

fn conflicting_fixity(
    operator: &str,
    first: Fixity,
    first_span: &Span,
    second: Fixity,
    second_span: &Span,
) -> ElabError {
    ElabError::ConflictingFixity {
        operator: operator.to_string(),
        first,
        second,
        first_span: first_span.clone(),
        second_span: second_span.clone(),
    }
}

fn install_declared_fixity(
    elab: &mut ElabEnv,
    operator: &str,
    id: ken_kernel::GlobalId,
    pending: &PendingFixity,
) -> Result<(), ElabError> {
    match elab.fixities.get(&id).copied() {
        None => {
            elab.fixities.insert(id, pending.fixity);
            elab.fixity_spans
                .insert(id, pending.declaration_span.clone());
            Ok(())
        }
        Some(existing) if existing == pending.fixity => Ok(()),
        Some(existing) => Err(conflicting_fixity(
            operator,
            existing,
            elab.fixity_spans
                .get(&id)
                .unwrap_or(&pending.declaration_span),
            pending.fixity,
            &pending.declaration_span,
        )),
    }
}

/// Collect this complete module's declarations before any body is associated.
/// Canonical names are only pre-identity staging coordinates: every fixity
/// consultation uses the `GlobalId` table after the target is admitted.
fn collect_scope_fixities(
    elab: &mut ElabEnv,
    decls: &[Decl],
    scope: &Scope,
) -> Result<Vec<PendingFixity>, ElabError> {
    let mut pending: Vec<PendingFixity> = Vec::new();
    for decl in decls {
        let Decl::FixityDecl {
            fixity,
            operator,
            operator_span,
            span,
        } = decl.unwrap_pub()
        else {
            continue;
        };
        if !scope.locals.contains(operator) {
            return Err(ElabError::FixityTargetNotLocal {
                operator: operator.clone(),
                span: operator_span.clone(),
            });
        }
        let canonical = scope
            .bindings
            .get(operator)
            .expect("a local binding has one canonical spelling")
            .clone();
        let candidate = PendingFixity {
            canonical_name: canonical.clone(),
            source_operator: operator.clone(),
            fixity: *fixity,
            declaration_span: span.clone(),
        };
        if let Some(first) = pending_fixity_for(&pending, &canonical) {
            if first.fixity != candidate.fixity {
                return Err(conflicting_fixity(
                    operator,
                    first.fixity,
                    &first.declaration_span,
                    candidate.fixity,
                    &candidate.declaration_span,
                ));
            }
            continue;
        }
        pending.push(candidate);
    }

    // A local operator declared by THIS unit has not received its ID yet.
    // Another already-loaded unit can have the same canonical spelling:
    // installing this unit's fixity onto that ambient ID would both mutate
    // the unrelated provider and turn a legal collision into a conflict.
    let new_operators: HashSet<&str> = decls
        .iter()
        .filter_map(|decl| match decl.unwrap_pub() {
            Decl::ViewDecl { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    // Validate already-admitted local targets (a fixity-only later source
    // call) before mutating the program table. New targets wait for admission.
    for candidate in &pending {
        if new_operators.contains(candidate.source_operator.as_str()) {
            continue;
        }
        let Some(id) = elab.globals.get(&candidate.canonical_name).copied() else {
            continue;
        };
        if let Some(existing) = elab.fixities.get(&id).copied() {
            if existing != candidate.fixity {
                return Err(conflicting_fixity(
                    &candidate.source_operator,
                    existing,
                    elab.fixity_spans
                        .get(&id)
                        .unwrap_or(&candidate.declaration_span),
                    candidate.fixity,
                    &candidate.declaration_span,
                ));
            }
        }
    }
    for candidate in &pending {
        if new_operators.contains(candidate.source_operator.as_str()) {
            continue;
        }
        if let Some(id) = elab.globals.get(&candidate.canonical_name).copied() {
            install_declared_fixity(elab, &candidate.source_operator, id, candidate)?;
        }
    }
    Ok(pending)
}

#[inline(never)]
fn elaborate_resolved_space(
    elab: &mut ElabEnv,
    resolved: &crate::resolve::RSpaceDecl,
) -> Result<Vec<crate::elab::ElabResult>, ElabError> {
    // Cloned rather than borrowed, following the same shape as the two
    // `standard_operators_here` sites above: `elab` is `&mut` here and the
    // reassociation pass needs an immutable view of the certified map.
    let standard_operators_here = elab.standard_operators.clone();
    let associated = crate::elab::reassociate_space_decl(
        resolved,
        &elab.globals,
        &elab.fixities,
        Some(&standard_operators_here),
    )?;
    crate::elab::elaborate_space_decl(elab, associated.as_deref().unwrap_or(resolved))
}

#[inline(never)]
fn elaborate_mutual_group_with_fixities(
    elab: &mut ElabEnv,
    members: &[crate::resolve::RDecl],
    declared_fixities: &[PendingFixity],
) -> Result<Vec<crate::elab::ElabResult>, ElabError> {
    let member_fixities = members
        .iter()
        .map(|member| {
            pending_fixity_for(declared_fixities, &member.name)
                .map(|pending| (pending.fixity, pending.declaration_span.clone()))
        })
        .collect::<Vec<_>>();
    // Snapshot: the call borrows `elab.env`/`elab.globals` mutably, so the
    // certified map cannot be handed over as a live field borrow.
    let standard_operators_for_group = elab.standard_operators.clone();
    crate::elab::elaborate_mutual_group(
        &mut elab.env,
        &mut elab.globals,
        &mut elab.num_values,
        &elab.numeric_env,
        &elab.class_env,
        &mut elab.resolution_provenance,
        &standard_operators_for_group,
        &mut elab.fixities,
        &mut elab.fixity_spans,
        &member_fixities,
        members,
    )
}

/// Expand and elaborate a compilation unit's raw decls (one `elaborate_*`
/// call's `Vec<Decl>`) at nesting `prefix` ("" at the file root), threading
/// `scope` (built fresh for a `module { … }` block; the persisted root
/// scope at the top level) and returning every produced `GlobalId` in
/// order, plus this scope's own `pub` export table.
fn expand_scope(
    elab: &mut ElabEnv,
    decls: &[Decl],
    prefix: &str,
    unit_inline_modules: &HashSet<String>,
    ordered_inline_modules: &mut HashSet<String>,
    scope: &mut Scope,
    unit_definitions: &mut HashSet<String>,
    allow_boundary: bool,
) -> Result<(Vec<crate::elab::ElabResult>, HashMap<String, String>), ElabError> {
    // P1 defines only private block spaces at the true file root. Reject the
    // syntactically accepted wider placements before qualification/resolution
    // can turn the unsupported surface into an internal error.
    for decl in decls {
        if !matches!(decl.unwrap_pub(), Decl::SpaceDecl { .. }) {
            continue;
        }
        let placement = if decl.is_pub() {
            Some("public")
        } else if !prefix.is_empty() {
            Some("nested")
        } else {
            None
        };
        if let Some(placement) = placement {
            return Err(ElabError::UnsupportedSpacePlacement {
                placement: placement.to_string(),
                span: decl.span().clone(),
            });
        }
    }

    let mut exports_here: HashMap<String, String> = HashMap::new();
    scope.exported_ids.clear();
    prebind_scope_declarations(
        scope,
        decls,
        prefix,
        elab.module_state.active_imports.last().map(String::as_str),
        unit_inline_modules,
        ordered_inline_modules,
        &elab.module_state.exports,
        &elab.globals,
        &elab.module_state.prelude_binding_names,
        &elab.module_state.inline_children,
        &elab.module_state.file_inline_paths,
        &elab.module_state.file_export_tables,
        &elab.module_state.file_export_ids,
        &elab.module_state.export_provenance,
        &mut exports_here,
    )?;
    let declared_fixities = collect_scope_fixities(elab, decls, scope)?;

    let mut ids = Vec::new();
    let mut i = 0;
    while i < decls.len() {
        let decl = &decls[i];
        match decl {
            Decl::FixityDecl { .. } => {
                // Metadata was collected for this complete scope before any
                // body was reassociated. It emits no elaboration result.
                i += 1;
            }
            Decl::BoundaryDecl { span, .. } => {
                if !allow_boundary || i != 0 {
                    return Err(ElabError::ParseError {
                        msg: "program/package boundary is only valid as the first file header"
                            .to_string(),
                        span: span.clone(),
                    });
                }
                i += 1;
            }
            // Imports are applied HERE, in textual order, so `import M`
            // sees `M`'s export table only once `module M { … }` has
            // actually been expanded — which happens earlier in this same
            // ordered pass if `M` is a sibling defined above (the normal
            // case; a module must be declared before it's imported).
            Decl::ImportDecl { module, kind, span } => {
                apply_import(
                    scope,
                    &elab.module_state.exports,
                    &elab.module_state.inline_children,
                    &elab.module_state.file_inline_paths,
                    &elab.module_state.file_export_tables,
                    &elab.module_state.file_export_ids,
                    &elab.module_state.export_provenance,
                    &elab.globals,
                    &elab.module_state.prelude_binding_names,
                    prefix,
                    elab.module_state.active_imports.last().map(String::as_str),
                    unit_inline_modules,
                    ordered_inline_modules,
                    module,
                    kind,
                    span,
                )?;
                i += 1;
            }
            Decl::ExportDecl { form, span } => {
                let file_root = elab.module_state.active_imports.last().map(String::as_str);
                let selected_file = match form {
                    ExportForm::Facade { module, .. }
                        if file_root.is_some()
                            && declared_inline_import(
                                prefix,
                                file_root,
                                module,
                                unit_inline_modules,
                            )
                            .is_none() =>
                    {
                        Some(module.as_str())
                    }
                    ExportForm::Facade { module, .. } if file_root.is_none() => elab
                        .module_state
                        .export_provenance
                        .get(module)
                        .and_then(|owner| owner.file_root.as_deref()),
                    _ => None,
                };
                apply_export(
                    scope,
                    &elab.module_state.exports,
                    &elab.module_state.file_export_tables,
                    &elab.module_state.file_export_ids,
                    &elab.module_state.export_provenance,
                    selected_file,
                    &elab.module_state.prop_intros,
                    &elab.globals,
                    &mut exports_here,
                    form,
                    span,
                )?;
                i += 1;
            }
            Decl::ModuleDecl {
                name,
                decls: inner,
                span: _,
            } => {
                let child_prefix = qualify(prefix, name);
                let mut child_scope = Scope::with_mode(scope.mode, scope.kernel_names.clone());
                let (child_ids, child_exports) = expand_scope(
                    elab,
                    inner,
                    &child_prefix,
                    unit_inline_modules,
                    ordered_inline_modules,
                    &mut child_scope,
                    unit_definitions,
                    false,
                )?;
                ids.extend(child_ids);
                // THIS DISCHARGES A CONTRACT, NOT A SYMPTOM.
                //
                // `local_prebinding_preserves_legacy_map_union_stack_budget`
                // promises that persistent local declaration bindings must not
                // enlarge `expand_scope`'s long-lived legacy frame. An earlier
                // form of this arm bound the decl's span and cloned
                // `child_prefix` so both could outlive the recursive call --
                // small in magnitude, and a true violation of exactly that.
                //
                // MEASURED: removing it does NOT fix the overflow that test
                // reports. The overflow comes from the `RStandardOp` descent in
                // `rewrite_rexpr_inner`, which is a DIFFERENT FRAME and which
                // correctness requires. So this edit is a no-op for the red and
                // the fix for the contract, and those are not the same job.
                // Keep it for the second reason: once the budget is
                // re-baselined, a total-stack pin can no longer see 8 bytes of
                // creep, and nothing else is watching this frame.
                //
                // `certify_standard_operator_home` no-ops for every module but
                // one, so gate it here rather than charge every recursion level
                // for the comparison: test the name, move `child_prefix` into
                // the export table as the pre-A1 code did, and materialise the
                // span from `decl` only on the path that consumes it. `decls`
                // outlives the call, so the span never has to cross it.
                let is_standard_operator_home =
                    crate::standard_operators::is_standard_operator_home(&child_prefix);
                if !prefix.is_empty() {
                    elab.module_state
                        .inline_children
                        .entry(prefix.to_string())
                        .or_default()
                        .insert(child_prefix.clone());
                }
                // Record availability only after this unit has completed the
                // child's ordered expansion, never from another unit's edge.
                ordered_inline_modules.insert(child_prefix.clone());
                // Intersect this source unit's declarations with what its
                // ordered pass actually expanded. The global edge map can
                // still contain same-spelling children from another unit.
                let owned_paths: HashSet<String> = unit_inline_modules
                    .intersection(ordered_inline_modules)
                    .cloned()
                    .collect();
                // Install provider provenance at the same site as its export
                // table, for file-backed and in-memory inline owners alike.
                let mut member_ids: ProviderExportIds = owned_paths
                    .iter()
                    .filter_map(|path| {
                        elab.module_state
                            .export_provenance
                            .get(path)
                            .and_then(|owner| owner.member_ids.get(path))
                            .map(|ids| (path.clone(), ids.clone()))
                    })
                    .collect();
                member_ids.insert(
                    child_prefix.clone(),
                    checked_export_ids(&mut child_scope, &child_exports, &elab.globals)?,
                );
                elab.module_state.export_provenance.insert(
                    child_prefix.clone(),
                    ExportProvenance {
                        inline_paths: owned_paths.clone(),
                        file_root: elab.module_state.active_imports.last().cloned(),
                        member_ids,
                    },
                );
                elab.module_state
                    .exports
                    .insert(child_prefix.clone(), child_exports);
                // The defining scope owns its declared child without an
                // import. Every grandchild needs this unit's own edge.
                scope
                    .prefixes
                    .insert(child_prefix.clone(), child_prefix.clone());
                let selected_ids = &elab.module_state.export_provenance[&child_prefix].member_ids;
                bind_provider_members(scope, &child_prefix, &child_prefix, selected_ids);
                authorize_inline_descendants(
                    scope,
                    &elab.module_state.inline_children,
                    &owned_paths,
                    None,
                    Some(selected_ids),
                    &child_prefix,
                    &child_prefix,
                );
                // The initial prebind ran before any same-unit child could
                // expand. Replay dictionary aliases now that this earlier
                // child has a real export table and an ordered owner edge.
                // Later children remain absent from ordered_inline_modules,
                // so this cannot borrow a caller's identically named export.
                prebind_synthesized_dictionaries(
                    scope,
                    decls,
                    prefix,
                    elab.module_state.active_imports.last().map(String::as_str),
                    unit_inline_modules,
                    ordered_inline_modules,
                    &elab.module_state.exports,
                    &elab.globals,
                    &elab.module_state.prelude_binding_names,
                    &elab.module_state.inline_children,
                    &elab.module_state.file_inline_paths,
                    &elab.module_state.file_export_tables,
                    &elab.module_state.file_export_ids,
                    &elab.module_state.export_provenance,
                    &mut exports_here,
                )?;
                if is_standard_operator_home {
                    certify_standard_operator_home(
                        elab,
                        crate::standard_operators::STANDARD_OPERATOR_HOME,
                        Some(decl.span()),
                    )?;
                }
                i += 1;
            }
            Decl::SpaceDecl {
                name,
                cells,
                operations,
                span,
            } => {
                let qualified_name = qualify(prefix, name);
                resolve::check_no_definition_collision(
                    name,
                    &qualified_name,
                    span,
                    Some(unit_definitions),
                )?;
                for operation in operations {
                    let operation_name = format!("{qualified_name}.{}", operation.name);
                    resolve::check_no_definition_collision(
                        &operation.name,
                        &operation_name,
                        &operation.span,
                        Some(unit_definitions),
                    )?;
                }
                let resolved =
                    resolve::resolve_space_decl(&qualified_name, cells, operations, span)?;
                ids.extend(elaborate_resolved_space(elab, &resolved)?);
                i += 1;
            }
            // A maximal run of non-`pub` definitions — auto-grouped by
            // call-graph SCC (`33 §1`: "All
            // top-level definitions are mutually recursive within a module
            // if the SCT check accepts the group"). A run with no actual
            // cycle degenerates to today's one-decl-at-a-time path, member
            // by member, byte-identical (AC3).
            _ if is_recursive_candidate(decl.unwrap_pub()) => {
                let run_end = {
                    let mut e = i;
                    while e < decls.len()
                        && (is_recursive_candidate(decls[e].unwrap_pub())
                            || matches!(decls[e].unwrap_pub(), Decl::FixityDecl { .. }))
                    {
                        e += 1;
                    }
                    e
                };
                let run = &decls[i..run_end];
                let run_member_count = run
                    .iter()
                    .filter(|candidate| is_recursive_candidate(candidate.unwrap_pub()))
                    .count();

                // Resolve + rewrite every run member up front — safe because
                // a run contains no import/module, so `scope`/`exports`
                // don't change across it; each member sees exactly the
                // state it would have seen processed alone at its position.
                let mut bare_names: Vec<String> = Vec::with_capacity(run_member_count);
                let mut rdecls: Vec<crate::resolve::RDecl> = Vec::with_capacity(run_member_count);
                for d in run
                    .iter()
                    .filter(|candidate| is_recursive_candidate(candidate.unwrap_pub()))
                {
                    let inner = d.unwrap_pub();
                    let renamed = qualify_decl_name(inner, prefix);
                    let rdecl = resolve_scoped_decl(
                        &renamed,
                        scope,
                        &elab.module_state.exports,
                        unit_definitions,
                    )?;
                    bare_names.push(rdecl.name.clone());
                    rdecls.push(rdecl);
                }

                // Call graph: edge a -> b iff a's body mentions b's bare
                // name (over-approximates on shadowing — safe, only ever
                // makes an SCC too LARGE, never misses a real cycle).
                let n = rdecls.len();
                let adj: Vec<Vec<usize>> = (0..n)
                    .map(|a| {
                        (0..n)
                            .filter(|&b| {
                                crate::elab::rexpr_mentions_name(&rdecls[a].body, &bare_names[b])
                                    || rdecls[a].ty.as_ref().is_some_and(|ty| {
                                        crate::elab::rtype_mentions_name(ty, &bare_names[b])
                                    })
                            })
                            .collect()
                    })
                    .collect();
                let sccs = scc_membership(&adj);

                // Process the SCC condensation dependency-first: a caller's
                // body is checked only after every acyclic callee body is
                // available for delta reduction.  The signature pre-pass in
                // a recursive SCC still admits every member before any body.
                let mut consumed = vec![false; n];
                for k in scc_dependency_order(&adj, &sccs) {
                    if consumed[k] {
                        continue;
                    }
                    let scc = &sccs[k];
                    for &m in scc {
                        consumed[m] = true;
                    }
                    // Existing singleton view/let recursion has its own
                    // spec-aware elaboration path.  Self edges are newly
                    // routed through the group/SCT seam only for proof
                    // declarations; multi-member SCCs remain shared.
                    let recursive = scc.len() > 1
                        || (adj[k].contains(&k)
                            && matches!(
                                rdecls[k].kind,
                                RDeclKind::Theorem | RDeclKind::AttachedProof { .. }
                            ));
                    if !recursive {
                        let rdecl = &rdecls[k];
                        let result = elaborate_checked(
                            elab,
                            rdecl,
                            pending_fixity_for(&declared_fixities, &rdecl.name),
                        )?;
                        ids.push(result);
                    } else {
                        let members: Vec<crate::resolve::RDecl> =
                            scc.iter().map(|&m| rdecls[m].clone()).collect();
                        let has_proof = members.iter().any(|rdecl| {
                            matches!(
                                rdecl.kind,
                                RDeclKind::Theorem | RDeclKind::AttachedProof { .. }
                            )
                        });
                        let has_computational = members.iter().any(|rdecl| {
                            matches!(rdecl.kind, RDeclKind::Let | RDeclKind::View { .. })
                        });
                        if has_proof && has_computational {
                            return Err(ElabError::TypeMismatch {
                                span: members[0].span.clone(),
                                reason: "mixed fn/const and proof recursive cycle is not supported"
                                    .to_string(),
                            });
                        }
                        let mut group_effect_rows = elab.effect_rows.clone();
                        for rdecl in &members {
                            if let Some(row) = crate::elab::surface_declared_row_type(rdecl)? {
                                group_effect_rows.insert(rdecl.name.clone(), row);
                            }
                        }
                        // Eligibility guard: the new group path only covers
                        // the plain V0 view/let shape (matches the existing
                        // singleton recursive-const rule) — a mutual member
                        // needing requires/ensures/where/refinement-return
                        // is out of this WP's scope; fail clearly rather
                        // than silently dropping its obligation.
                        for rdecl in &members {
                            let simple_kind = matches!(
                                &rdecl.kind,
                                RDeclKind::Let
                                    | RDeclKind::Theorem
                                    | RDeclKind::AttachedProof { .. }
                            ) || matches!(
                                &rdecl.kind,
                                RDeclKind::View { constraints, is_space_op, .. }
                                    if constraints.is_empty() && !is_space_op
                            );
                            let has_refine_return = rdecl
                                .ty
                                .as_ref()
                                .and_then(|ty| crate::elab::innermost_refine_pred(ty))
                                .is_some();
                            if !simple_kind
                                || !rdecl.requires.is_empty()
                                || !rdecl.ensures.is_empty()
                                || has_refine_return
                            {
                                return Err(ElabError::Internal(format!(
                                    "mutual recursion is only supported for plain recursive \
                                     definitions (no requires/ensures/where-constraints/\
                                     refinement-return); '{}' does not qualify",
                                    rdecl.name
                                )));
                            }
                            crate::elab::check_surface_purity(
                                rdecl,
                                &group_effect_rows,
                                &elab.effect_rows_by_id,
                                &elab.globals,
                                &elab.class_env,
                            )?;
                        }
                        let results = elaborate_mutual_group_with_fixities(
                            elab,
                            &members,
                            &declared_fixities,
                        )?;
                        for (rdecl, result) in members.iter().zip(results) {
                            register_effect_row(elab, &result);
                            register_declared_effect_row(elab, rdecl)?;
                            ids.push(result);
                        }
                    }
                }
                // Public definitions participate in the same scope-wide
                // admission run; publish their already-elaborated canonical
                // names only after the run succeeds, preserving the module
                // export boundary while allowing forward references.
                for (d, rdecl) in run
                    .iter()
                    .filter(|candidate| is_recursive_candidate(candidate.unwrap_pub()))
                    .zip(&rdecls)
                {
                    if !d.is_pub() {
                        continue;
                    }
                    let inner = d.unwrap_pub();
                    if let Decl::AttachedProofDecl {
                        subject,
                        proof_name,
                        ..
                    } = inner
                    {
                        let subject_is_public = exports_here.contains_key(subject)
                            || run
                                .iter()
                                .filter(|candidate| {
                                    is_recursive_candidate(candidate.unwrap_pub())
                                })
                                .any(|candidate| {
                                    candidate.is_pub() && candidate.unwrap_pub().name() == subject
                                });
                        if !subject_is_public {
                            return Err(ElabError::UnboundName {
                                name: subject.clone(),
                                span: inner.span().clone(),
                            });
                        }
                        let checked_id = elab.globals.get(&rdecl.name).copied().ok_or_else(|| {
                            ElabError::Internal(format!(
                                "recursive public proof '{}' has no checked ID",
                                rdecl.name
                            ))
                        })?;
                        publish_checked_identity(
                            scope,
                            &mut exports_here,
                            &format!("{subject}::{proof_name}"),
                            &rdecl.name,
                            checked_id,
                            inner.span(),
                        )?;
                    } else {
                        let checked_id = elab.globals.get(&rdecl.name).copied().ok_or_else(|| {
                            ElabError::Internal(format!(
                                "recursive public declaration '{}' has no checked ID",
                                rdecl.name
                            ))
                        })?;
                        publish_checked_identity(
                            scope,
                            &mut exports_here,
                            inner.name(),
                            &rdecl.name,
                            checked_id,
                            inner.span(),
                        )?;
                    }
                }
                i = run_end;
            }
            other => {
                let is_pub = other.is_pub();
                let inner = other.unwrap_pub();
                if is_qualifiable(inner) {
                    let bare = inner.name().to_string();
                    if is_pub {
                        if let Decl::AttachedProofDecl {
                            subject,
                            proof_name,
                            ..
                        } = inner
                        {
                            if !exports_here.contains_key(subject) {
                                return Err(ElabError::UnboundName {
                                    name: subject.clone(),
                                    span: inner.span().clone(),
                                });
                            }
                            if exports_here.contains_key(&format!("{subject}::{proof_name}")) {
                                return Err(ElabError::TypeMismatch {
                                    span: inner.span().clone(),
                                    reason: format!(
                                        "duplicate public attached proof '{}::{}'",
                                        subject, proof_name
                                    ),
                                });
                            }
                        }
                    }
                    let renamed = qualify_decl_name(inner, prefix);
                    let rdecl = resolve_scoped_decl(
                        &renamed,
                        scope,
                        &elab.module_state.exports,
                        unit_definitions,
                    )?;
                    let result = elaborate_checked(
                        elab,
                        &rdecl,
                        pending_fixity_for(&declared_fixities, &rdecl.name),
                    )?;
                    if let Decl::PropDecl { intros, .. } = inner {
                        let mut checked_intros = Vec::with_capacity(intros.len());
                        for intro in intros {
                            let canonical_intro = format!("{}.{}", result.name, intro.name);
                            if !elab.globals.contains_key(&canonical_intro) {
                                return Err(ElabError::Internal(format!(
                                    "prop intro '{canonical_intro}' did not elaborate"
                                )));
                            }
                            checked_intros.push(intro.name.clone());
                        }
                        elab.module_state
                            .prop_intros
                            .insert(result.name.clone(), checked_intros);
                    }
                    if is_pub {
                        if let Decl::AttachedProofDecl {
                            subject,
                            proof_name,
                            ..
                        } = inner
                        {
                            publish_checked_identity(
                                scope,
                                &mut exports_here,
                                &format!("{subject}::{proof_name}"),
                                &result.name,
                                result.def_id,
                                inner.span(),
                            )?;
                        } else {
                            // Only the decl's own qualified name is exported —
                            // never a `DataDecl`'s constructors (`33 §4.2`,
                            // abstract export: ctors are simply never entered
                            // into any export table, so a client can't bring
                            // them into scope by any import form).
                            publish_checked_identity(
                                scope, &mut exports_here, &bare, &result.name,
                                result.def_id, inner.span(),
                            )?;
                            publish_family_intros(
                                scope,
                                &mut exports_here,
                                &elab.module_state.prop_intros,
                                &bare,
                                &result.name,
                                &bare,
                                None,
                                &elab.globals,
                                inner.span(),
                            )?;
                        }
                    }
                    ids.push(result);
                } else {
                    // Not module-qualifiable (class/instance/law/foreign/
                    // temporal/prove) — elaborate unchanged, unqualified.
                    let rdecl = resolve_scoped_decl(
                        inner,
                        scope,
                        &elab.module_state.exports,
                        unit_definitions,
                    )?;
                    let result = elaborate_checked(
                        elab,
                        &rdecl,
                        pending_fixity_for(&declared_fixities, &rdecl.name),
                    )?;
                    if is_pub && matches!(inner, Decl::ClassDecl { .. }) {
                        publish_checked_identity(
                            scope, &mut exports_here, inner.name(),
                            &result.name, result.def_id, inner.span(),
                        )?;
                    }
                    ids.push(result);
                }
                i += 1;
            }
        }
    }

    // Forward in-scope exports are emitted only after their local family is
    // checked, so prop selectors cannot be lost merely because `export P`
    // preceded the declaration of `P` in this source unit.
    let forward_exports = scope.pending_local_exports.clone();
    for (surface, (canonical, span)) in forward_exports {
        publish_family_intros(
            scope,
            &mut exports_here,
            &elab.module_state.prop_intros,
            &surface,
            &canonical,
            &surface,
            None,
            &elab.globals,
            &span,
        )?;
    }
    scope.current_local_names.clear();
    Ok((ids, exports_here))
}

/// Strongly-connected-component membership for a small directed call graph
/// (`adj[i]` = out-edges from `i`, i.e. "`i`'s body mentions `j`"). Returns,
/// per node, the sorted list of node indices in its SCC (always includes the
/// node itself). O(n^3) — fine for a same-scope call graph (one source
/// file's mutually-recursive group), not sized for a whole-program graph.
fn scc_membership(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut reach: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for (i, reach_i) in reach.iter_mut().enumerate() {
        let mut stack = adj[i].clone();
        let mut seen = vec![false; n];
        while let Some(j) = stack.pop() {
            if seen[j] {
                continue;
            }
            seen[j] = true;
            reach_i[j] = true;
            for &k in &adj[j] {
                if !seen[k] {
                    stack.push(k);
                }
            }
        }
    }
    (0..n)
        .map(|i| {
            let mut members: Vec<usize> = (0..n)
                .filter(|&j| j == i || (reach[i][j] && reach[j][i]))
                .collect();
            members.sort_unstable();
            members
        })
        .collect()
}

/// Return one representative per SCC in dependency-first order.  An edge
/// `a -> b` means that `a`'s body uses `b`, so `b` must be elaborated first.
/// Members of an SCC are still elaborated together by the SCT path.
fn scc_dependency_order(adj: &[Vec<usize>], sccs: &[Vec<usize>]) -> Vec<usize> {
    let mut representatives = Vec::new();
    for (node, scc) in sccs.iter().enumerate() {
        if scc[0] == node {
            representatives.push(node);
        }
    }
    let mut order = Vec::new();
    let mut seen = vec![false; adj.len()];
    fn visit(
        node: usize,
        adj: &[Vec<usize>],
        sccs: &[Vec<usize>],
        seen: &mut [bool],
        order: &mut Vec<usize>,
    ) {
        let rep = sccs[node][0];
        if seen[rep] {
            return;
        }
        seen[rep] = true;
        // Condensation edges are the union of every member's edges.  Looking
        // only at the representative skips dependencies mentioned solely by
        // a later member of a mutual SCC.
        for &member in &sccs[rep] {
            for &dep in &adj[member] {
                visit(dep, adj, sccs, seen, order);
            }
        }
        order.push(rep);
    }
    for node in representatives {
        visit(node, adj, sccs, &mut seen, &mut order);
    }
    order
}

/// Entry point: expand + elaborate one `elaborate_*` call's raw decls
/// against the persisted root scope (the file-level implicit module,
/// `33 §3.1`), returning every produced `ElabResult` in order.
pub fn expand_and_elaborate(
    elab: &mut ElabEnv,
    decls: &[Decl],
) -> Result<Vec<crate::elab::ElabResult>, ElabError> {
    let boundary = admission_boundary(decls)?;
    let direct_call = boundary.is_some() && elab.class_env.current_package.is_none();
    let previous_package = elab.class_env.current_package.clone();
    let previous_direct_use = elab.class_env.direct_use_packages.clone();
    let previous_direct_instances = elab.class_env.direct_use_instances.clone();
    let previous_implicit_single_provider = elab.class_env.implicit_single_provider;
    if direct_call {
        let admitted = boundary
            .as_ref()
            .and_then(|(header, _)| header.admits.clone())
            .unwrap_or_default()
            .into_iter()
            .collect();
        elab.class_env.current_package = Some("<root>".to_string());
        elab.class_env.direct_use_packages = Some(admitted);
        elab.class_env.direct_use_instances.clear();
        elab.class_env.implicit_single_provider = false;
    }
    let mut scope = elab.module_state.root_scope.clone();
    let mut unit_definitions = HashSet::new();
    let mut local_modules = HashSet::new();
    declared_module_paths(decls, "", &mut local_modules);
    let mut ordered_inline_modules = HashSet::new();
    let expanded = expand_scope(
        elab,
        decls,
        "",
        &local_modules,
        &mut ordered_inline_modules,
        &mut scope,
        &mut unit_definitions,
        true,
    );
    if direct_call {
        elab.class_env.current_package = previous_package;
        elab.class_env.direct_use_packages = previous_direct_use;
        elab.class_env.direct_use_instances = previous_direct_instances;
        elab.class_env.implicit_single_provider = previous_implicit_single_provider;
    }
    let (results, root_exports) = expanded?;
    // In-memory root exports are not a named module interface, but any
    // forward local selected for export still owes checked-ID reconciliation
    // before this persistent scope is reused by the next source call.
    checked_export_ids(&mut scope, &root_exports, &elab.globals)?;
    // Root exports are not a named public interface. Keep imported scope
    // bindings across calls, but not this call's export-ID collision ledger.
    scope.exported_ids.clear();
    if direct_call {
        if let Some((header, header_span)) = &boundary {
            let main_span = decls
                .iter()
                .find(|decl| decl.name() == "main")
                .map(|decl| decl.span().clone());
            let main_uses_fs = main_span.is_some()
                && elab
                    .effect_rows
                    .get("main")
                    .is_some_and(|row| row.concrete_effects().contains("FS"));
            let declares_fs = header
                .capabilities
                .as_ref()
                .is_some_and(|caps| caps.iter().any(|cap| cap.family == "FS"));
            if header.kind == crate::ast::BoundaryKind::Program && main_uses_fs && !declares_fs {
                return Err(ElabError::MissingCapability {
                    effect: "FS".to_string(),
                    span: main_span.unwrap_or_else(|| header_span.clone()),
                });
            }
        }
    }
    if direct_call {
        elab.module_state.boundary_header = boundary.map(|(header, _)| header);
    }
    elab.module_state.root_scope = scope;
    Ok(results)
}

#[cfg(test)]
mod namespace_effect_tests {
    use std::collections::{BTreeSet, HashMap};
    use std::fs;
    use std::path::PathBuf;

    use super::{decl_namespace_effect, ConstructorNameSource, DeclNamespaceEffect, Scope};
    use crate::ast::{Decl, ExplicitDataCtor};
    use crate::error::{ElabError, Span};
    use crate::parser::parse_decls;
    use crate::ElabEnv;
    use ken_kernel::GlobalId;

    /// Promise class: durable invariant (spec 16 §1.4; 33 §3.3).
    ///
    /// MEASURED: the default Proved name denotes the kernel's fixed tt_id,
    /// which is absent from the assumption ledger; replacing that spelling
    /// with a distinct checked ID makes the strict roster refuse it.
    /// CLAIMED: only this fixed introduction may gain strict compiler-name
    /// admission, not arbitrary global names. THE GAP: the facade/import
    /// control in the Pair suite separately checks the selected identity.
    #[test]
    fn strict_proved_name_requires_exact_kernel_intro_identity() {
        let mut env = ElabEnv::new().expect("base environment");
        let proved = env.globals["Proved"];
        assert_eq!(proved, env.env.tt_id());
        let native: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
        assert!(
            !native.contains(&proved),
            "fixed prelude tt adds no assumption"
        );
        assert!(env.module_state.strict_builtin_names.contains("Proved"));

        let other = env.globals["True"];
        assert_ne!(proved, other, "the control must move the identity");
        env.globals.insert("Proved".to_string(), other);
        match env
            .module_state
            .capture_strict_builtin_names(&env.env, &env.globals, &native)
        {
            Err(ElabError::Internal(message)) => {
                assert!(message.contains("Proved"), "wrong guard: {message}");
                assert!(message.contains("tt"), "must name fixed intro: {message}");
            }
            other => panic!("forged Proved identity must be refused: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 16 §1.4; 33 §3.3).
    ///
    /// MEASURED: bare Proved declarations at roots, under public inline
    /// modules, and as data constructors collide with the installed binding
    /// before the kernel allocates an id, in both legacy and strict units.
    /// CLAIMED: source declarations cannot replace the fixed tt introduction.
    /// THE GAP: public-map mutation bypasses source prebinding and is checked
    /// independently by the strict-entry control below.
    #[test]
    fn proved_source_redeclarations_fail_at_prebind_without_global_allocation() {
        let mut baseline = ElabEnv::new().expect("base environment");
        let expected_next = baseline.env.fresh_id();
        for (label, source, strict) in [
            ("bare-legacy", "const Proved : Nat = Zero", false),
            (
                "nested-public-legacy",
                "module M { pub const Proved : Nat = Zero }",
                false,
            ),
            ("constructor-legacy", "data Token = Proved", false),
            ("bare-strict", "const Proved : Nat = Zero", true),
            (
                "nested-public-strict",
                "module M { pub const Proved : Nat = Zero }",
                true,
            ),
            ("constructor-strict", "data Token = Proved", true),
        ] {
            let mut env = ElabEnv::new().expect("base environment");
            let tt = env.env.tt_id();
            let result = if strict {
                let root = tempfile::tempdir().expect("temporary strict module root");
                fs::write(root.path().join("Entry.ken"), source).expect("write prebind-clash unit");
                env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
            } else {
                env.elaborate_file(source)
            };
            match result {
                Err(ElabError::AmbiguousReference { name, sources, .. }) => {
                    assert_eq!(name, "Proved", "{label}");
                    assert!(sources.contains(&"<prelude>.Proved".to_string()), "{label}");
                    assert_eq!(sources.len(), 2, "{label}");
                }
                Err(other) => panic!("{label}: expected prebind clash, got {other:?}"),
                Ok(_) => panic!("{label}: source redefined fixed Proved"),
            }
            assert_eq!(env.globals["Proved"], tt, "{label}");
            assert_eq!(env.env.fresh_id(), expected_next, "{label}: allocated id");
        }

        let mut lexical = ElabEnv::new().expect("base environment");
        lexical
            .elaborate_file("fn keep (Proved : Nat) : Nat = Zero")
            .expect("a narrow lexical binder is not a global redeclaration");
        assert_eq!(lexical.globals["Proved"], lexical.env.tt_id());
    }

    /// Promise class: durable invariant (spec 16 §1.4; 33 §§3.2–3.3).
    ///
    /// MEASURED: a distinct checked, same-Top proof substituted into the
    /// public globals map is refused at every strict roots entry, both cold
    /// and with both units cached. No loader state changes on refusal.
    /// CLAIMED: neither a file facade nor a cache can launder a postcapture
    /// non-tt Proved into strict source. THE GAP: source declarations are
    /// blocked separately by the prebind-reservation test above.
    #[test]
    fn strict_roots_refuses_checked_same_top_proved_forgery_before_load_or_reuse() {
        let root = tempfile::tempdir().expect("temporary strict module root");
        fs::write(root.path().join("ProofTerms.ken"), "export Proved")
            .expect("write real proof facade");
        fs::write(
            root.path().join("Entry.ken"),
            "import ProofTerms (Proved)\n\
             theorem selected_intro : Eq Bool True True = Proved",
        )
        .expect("write strict consumer");
        let roots = [root.path().to_path_buf()];
        for reuse in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            let trusted_before = env.env.trusted_base();
            env.elaborate_file("theorem alternate_intro : Top = Proved")
                .expect("distinct Top proof is checked, not postulated");
            assert_eq!(env.env.trusted_base(), trusted_before);
            let alternate = env.globals["alternate_intro"];
            let tt = env.env.tt_id();
            assert_ne!(alternate, tt);
            let (_, alternate_ty) = env.env.const_type(alternate).expect("checked proof type");
            assert_eq!(
                alternate_ty,
                ken_kernel::Term::const_(env.env.top_id(), vec![])
            );
            if reuse {
                env.elaborate_module_from_roots_strict(&roots, "Entry")
                    .expect("unmodified real facade and consumer elaborate");
                assert_eq!(env.module_state.exports["ProofTerms"]["Proved"], "Proved");
                let (_, body) = env
                    .env
                    .transparent_body(env.globals["Entry.selected_intro"])
                    .expect("selected proof has a checked body");
                match body {
                    ken_kernel::Term::Const { id, .. } => assert_eq!(id, tt),
                    other => panic!("facade selected a non-tt body: {other:?}"),
                }
            }
            let loaded_before = env.module_state.loaded_units.clone();
            let roots_before = env.module_state.catalog_roots.clone();
            assert_eq!(
                env.globals.insert("Proved".to_string(), alternate),
                Some(tt)
            );
            match env.elaborate_module_from_roots_strict(&roots, "Entry") {
                Err(ElabError::Internal(message)) => {
                    assert!(message.contains("Proved identity mismatch"), "{message}");
                    assert!(message.contains(&format!("{tt:?}")), "{message}");
                    assert!(message.contains(&format!("{alternate:?}")), "{message}");
                }
                Err(other) => panic!("reuse={reuse}: wrong refusal: {other:?}"),
                Ok(_) => panic!("reuse={reuse}: strict roots laundered non-tt Proved"),
            }
            assert_eq!(env.globals["Proved"], alternate, "must not heal caller map");
            assert_eq!(env.module_state.loaded_units, loaded_before);
            assert_eq!(env.module_state.catalog_roots, roots_before);
        }

        let mut missing = ElabEnv::new().expect("base environment");
        missing.globals.remove("Proved");
        match missing.elaborate_module_from_roots_strict(&roots, "Entry") {
            Err(ElabError::Internal(message)) => {
                assert!(message.contains("Proved identity mismatch"), "{message}");
                assert!(message.contains("None"), "missing identity: {message}");
            }
            Err(other) => panic!("missing Proved must fail at identity gate: {other:?}"),
            Ok(_) => panic!("missing Proved passed strict roots entry"),
        }
        assert!(missing.module_state.catalog_roots.is_empty());
        assert!(missing.module_state.loaded_units.is_empty());
    }

    #[derive(Debug, PartialEq, Eq)]
    enum OwnedNamespaceEffect {
        TopLevelName {
            name: String,
            span: Span,
        },
        ConstructorNames {
            parent: String,
            parent_span: Span,
            constructors: Vec<(String, Span)>,
        },
        QualifiedIdentity {
            subject: String,
            proof_name: String,
            span: Span,
        },
        ReferenceWithSynthesizedDictionary {
            class_name: String,
            head_name: String,
            span: Span,
        },
        ReferenceOnly,
        NoBinding,
    }

    fn owned_namespace_effect(decl: &Decl) -> OwnedNamespaceEffect {
        match decl_namespace_effect(decl) {
            DeclNamespaceEffect::TopLevelName { name, span } => {
                OwnedNamespaceEffect::TopLevelName {
                    name: name.to_string(),
                    span: span.clone(),
                }
            }
            DeclNamespaceEffect::ConstructorNames {
                parent,
                parent_span,
                constructors,
            } => {
                let constructors = match constructors {
                    ConstructorNameSource::Simple(constructors) => constructors
                        .iter()
                        .map(|constructor| (constructor.name.clone(), constructor.span.clone()))
                        .collect(),
                    ConstructorNameSource::Explicit(constructors) => constructors
                        .iter()
                        .map(|constructor| match constructor {
                            ExplicitDataCtor::Simple(constructor) => {
                                (constructor.name.clone(), constructor.span.clone())
                            }
                            ExplicitDataCtor::Signature { name, span, .. } => {
                                (name.clone(), span.clone())
                            }
                        })
                        .collect(),
                };
                OwnedNamespaceEffect::ConstructorNames {
                    parent: parent.to_string(),
                    parent_span: parent_span.clone(),
                    constructors,
                }
            }
            DeclNamespaceEffect::QualifiedIdentity {
                subject,
                proof_name,
                span,
            } => OwnedNamespaceEffect::QualifiedIdentity {
                subject: subject.to_string(),
                proof_name: proof_name.to_string(),
                span: span.clone(),
            },
            DeclNamespaceEffect::ReferenceWithSynthesizedDictionary {
                class_name,
                head_name,
                span,
            } => OwnedNamespaceEffect::ReferenceWithSynthesizedDictionary {
                class_name: class_name.to_string(),
                head_name: head_name.to_string(),
                span: span.clone(),
            },
            DeclNamespaceEffect::ReferenceOnly => OwnedNamespaceEffect::ReferenceOnly,
            DeclNamespaceEffect::NoBinding => OwnedNamespaceEffect::NoBinding,
        }
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: an ambient local and a qualified import that resolve to one
    /// `GlobalId` leave the private binding table byte-for-byte unchanged.
    /// CLAIMED: importing a second route to one declaration is a no-op. THE
    /// GAP: the distinct-identity control below proves this is identity
    /// equality rather than an unconditional local-name escape.
    #[test]
    fn ambient_local_import_of_same_identity_is_a_binding_noop() {
        let shared = GlobalId(90_001);
        let globals = HashMap::from([
            ("Owner.item".to_string(), shared),
            ("Provider.item".to_string(), shared),
        ]);
        let mut scope = Scope::default();
        scope
            .bind_local("item", "Owner.item", &Span::new(0, 4))
            .expect("the local producer installs both local and canonical binding state");
        let before = scope.bindings.clone();
        assert_eq!(before.get("item").map(String::as_str), Some("Owner.item"));

        scope
            .bind_import(&globals, "item", "Provider.item", shared, &Span::new(10, 20))
            .expect("two routes to one resolved identity must be idempotent");

        assert_eq!(
            scope.bindings, before,
            "the identity escape must not install or replace the canonical local binding"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: the same ambient-local shape with two distinct `GlobalId`s
    /// returns the exact ambiguity variant and reports the canonical local
    /// source beside the qualified source. CLAIMED: the escape cannot admit a real
    /// collision. THE GAP: both ids are asserted distinct before the refusal.
    #[test]
    fn ambient_local_import_of_distinct_identity_refuses_with_honest_sources() {
        let local_id = GlobalId(90_001);
        let imported_id = GlobalId(90_002);
        assert_ne!(
            local_id, imported_id,
            "the refusal fixture must be non-degenerate"
        );
        let globals = HashMap::from([
            ("Owner.item".to_string(), local_id),
            ("Provider.item".to_string(), imported_id),
        ]);
        let mut scope = Scope::default();
        scope
            .bind_local("item", "Owner.item", &Span::new(0, 4))
            .expect("the local producer installs both local and canonical binding state");

        match scope.bind_import(&globals, "item", "Provider.item", imported_id, &Span::new(10, 20)) {
            Err(ElabError::AmbiguousReference { name, sources, .. }) => {
                assert_eq!(name, "item");
                assert_eq!(
                    sources,
                    vec!["Owner.item".to_string(), "Provider.item".to_string()],
                    "the canonical local source must replace the old bare-name fallback"
                );
            }
            other => panic!("distinct identities must remain ambiguous, got {other:?}"),
        }
    }

    fn env_with_ambient_item_and_facade() -> (ElabEnv, GlobalId) {
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file(
            "const item : Nat = Zero \
             module Provider { export item }",
        )
        .expect("establish an ambient local and a facade route to its identity");
        let item = env.globals["item"];
        assert_eq!(
            env.module_state
                .root_scope
                .bindings
                .get("item")
                .map(String::as_str),
            Some("item")
        );
        assert_eq!(
            env.module_state.exports["Provider"]
                .get("item")
                .map(String::as_str),
            Some("item")
        );
        (env, item)
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: an actual `.ken` selective import reaches `apply_import`,
    /// accepts a facade route to the ambient local's existing `GlobalId`, and
    /// preserves the complete root binding table. CLAIMED: the identity escape
    /// is reachable through its production caller. THE GAP: the replay and
    /// source-order variants below drive the two other ordered-pass shapes.
    #[test]
    fn apply_import_accepts_an_ambient_local_at_the_same_identity() {
        let (mut env, item) = env_with_ambient_item_and_facade();
        let before = env.module_state.root_scope.bindings.clone();

        env.elaborate_file("import Provider (item)")
            .expect("the second route to the ambient identity must elaborate");

        assert_eq!(env.globals["item"], item);
        assert_eq!(
            env.module_state.root_scope.bindings, before,
            "apply_import must leave the ambient binding table unchanged"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: adding an instance makes the prebinding synthesis replay run,
    /// and the same-identity import still elaborates before the dictionary is
    /// produced. CLAIMED: replay reaches the identity escape without turning a
    /// no-op import into an ambiguity. THE GAP: the dictionary-presence check
    /// proves the fixture did not return before instance elaboration.
    #[test]
    fn apply_import_accepts_the_same_identity_during_instance_replay() {
        let (mut env, item) = env_with_ambient_item_and_facade();
        let before_item_binding = env.module_state.root_scope.bindings["item"].clone();

        env.elaborate_file(
            "class Marker a {} \
             import Provider (item) \
             instance Marker Nat {}",
        )
        .expect("synthesis replay and ordered import both accept the same identity");

        assert_eq!(env.globals["item"], item);
        assert_eq!(
            env.module_state.root_scope.bindings["item"],
            before_item_binding
        );
        assert!(
            env.globals.contains_key("Marker_instance_Nat"),
            "the instance must elaborate after the replayed import"
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: `apply_import` accepts the facade route when its import is
    /// textually above a local redeclaration prebound for the complete scope,
    /// and the later declaration allocates a fresh identity. CLAIMED: ordered
    /// application does not mistake the prebound same-identity local for a
    /// distinct source. THE GAP: the changed id proves the later local ran.
    #[test]
    fn apply_import_accepts_the_same_identity_above_the_local() {
        let (mut env, item_before) = env_with_ambient_item_and_facade();

        env.elaborate_file(
            "import Provider (item) \
             const item : Nat = Zero",
        )
        .expect("an import above the prebound local must accept its ambient identity");

        assert_ne!(
            env.globals["item"], item_before,
            "the local below the import must still elaborate"
        );
        assert_eq!(
            env.module_state
                .root_scope
                .bindings
                .get("item")
                .map(String::as_str),
            Some("item")
        );
    }

    /// Promise class: normative compatibility vector.
    ///
    /// MEASURED: the actual private resolver export table after roots-loading
    /// the catalog package. CLAIMED: PriorityQueue has exactly its specified
    /// six-name public API, including every direct export and re-export. THE
    /// GAP: integration tests separately prove those identities are usable and
    /// the selected private names refuse at the client boundary.
    #[test]
    fn priority_queue_actual_export_table_is_exactly_the_six_name_api() {
        let mut env = ElabEnv::new().expect("base environment");
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
        env.elaborate_module_from_roots(&[root], "Data.Collections.PriorityQueue")
            .expect("PriorityQueue must elaborate from its catalog roots");
        let exports = env
            .module_state
            .exports
            .get("Data.Collections.PriorityQueue")
            .expect("loaded module must have its actual resolver export table");
        let observed = exports.keys().map(String::as_str).collect::<BTreeSet<_>>();
        assert_eq!(
            observed,
            BTreeSet::from([
                "PriorityQueue",
                "empty",
                "find_min",
                "insert",
                "merge",
                "pop_min",
            ])
        );
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: a real facade module's private export table records a renamed
    /// re-export under its public alias. CLAIMED: the owner-local closure seam
    /// observes renamed re-exports rather than only direct declarations or
    /// original spellings. THE GAP: the catalog assertion above applies that
    /// seam to PriorityQueue's authoritative package.
    #[test]
    fn actual_export_table_includes_renamed_reexports() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::write(
            root.path().join("ExportSource.ken"),
            "pub const original : Nat = Zero\n",
        )
        .expect("write export source");
        fs::write(
            root.path().join("RenamedFacade.ken"),
            "export ExportSource (original as renamed)\n",
        )
        .expect("write renamed facade");

        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&[root.path().to_path_buf()], "RenamedFacade")
            .expect("renamed facade must elaborate");
        let exports = env
            .module_state
            .exports
            .get("RenamedFacade")
            .expect("loaded facade must have its actual resolver export table");
        assert_eq!(
            exports.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["renamed"])
        );
    }

    fn inline_owner_root(source: &str) -> tempfile::TempDir {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::write(root.path().join("A.ken"), source).expect("write inline owner unit");
        root
    }

    const INLINE_OWNER: &str = "module N {\n\
         pub const x : Nat = Zero\n\
         const s : Nat = Suc Zero\n\
         const inside : Nat = s\n\
         }\n\
         pub const top : Nat = Zero\n";

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: the roots loader admits A's own `A.N.x` and the inline
    /// sibling P's `import N` / `N.x`, without loading a decoy `N.ken`.
    /// CLAIMED: declaration provenance, not a discoverable file or ambient
    /// exports, grants the public child name. THE GAP: the B-unit negatives
    /// below exclude the alternative of simply opening all loaded children.
    #[test]
    fn inline_child_owner_and_sibling_import_use_declaration_provenance() {
        let root = inline_owner_root(
            "module N { pub const x : Nat = Zero\n\
             const s : Nat = Suc Zero\n\
             const inside : Nat = s }\n\
             const own : Nat = A.N.x\n\
             module P { import N\npub const sibling : Nat = N.x }\n\
             pub const top : Nat = Zero\n",
        );
        fs::write(root.path().join("N.ken"), "pub const x : Nat = Suc Zero\n")
            .expect("write a distinct file-backed decoy N");
        let mut env = ElabEnv::new().expect("base environment");
        let trust_before = env.env.trusted_base();
        env.elaborate_module_from_roots(&[root.path().to_path_buf()], "A")
            .expect("owner and inline sibling must use A.N.x");
        for name in ["A.N.x", "A.N.inside", "A.own", "A.P.sibling"] {
            assert!(env.globals.contains_key(name), "missing checked {name}");
        }
        assert!(
            !env.module_state.loaded_units.contains_key("N"),
            "inline N must not load the distinct N.ken"
        );
        assert_eq!(env.env.trusted_base(), trust_before);
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.2).
    ///
    /// MEASURED: a `Data.Foo` file unit's bare `import N` resolves the actual
    /// file `N.ken` even after a separate inline `Data.N` has been elaborated;
    /// both cold and preloaded N cases retain N.x's exact GlobalId in the body.
    /// CLAIMED: a loaded unit's lexical ancestor walk stops at that unit root,
    /// never capturing a similarly spelled child of the `Data` namespace.
    /// THE GAP: the existing inline-owner/sibling controls show that clipping
    /// the file walk does not forbid genuine same-unit inline children.
    #[test]
    fn file_unit_import_cannot_capture_external_inline_ancestor() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::create_dir_all(root.path().join("Data")).expect("create file-unit directory");
        fs::write(root.path().join("N.ken"), "pub const x : Nat = Suc Zero\n")
            .expect("write file-backed N");
        fs::write(
            root.path().join("Data/Foo.ken"),
            "import N\npub const observed : Nat = N.x\n",
        )
        .expect("write Data.Foo file unit");
        fs::write(
            root.path().join("Data/Owned.ken"),
            "module N { pub const x : Nat = Zero }\n\
             module P { import N\npub const selected : Nat = N.x }\n",
        )
        .expect("write actual same-unit inline sibling");
        let roots = [root.path().to_path_buf()];
        for preload_n in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            env.elaborate_file("module Data { module N { pub const x : Nat = Zero } }")
                .expect("separate inline Data.N");
            if preload_n {
                env.elaborate_module_from_roots(&roots, "N")
                    .expect("preload the distinct file-backed N");
            }
            env.elaborate_module_from_roots(&roots, "Data.Foo")
                .expect("Data.Foo must import the file-backed N");
            assert!(env.module_state.loaded_units.contains_key("N"));
            assert_ne!(env.globals["N.x"], env.globals["Data.N.x"]);
            let (_, body) = env
                .env
                .transparent_body(env.globals["Data.Foo.observed"])
                .expect("observed is transparent");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, env.globals["N.x"]),
                other => panic!("observed must reference N.x, got {other:?}"),
            }
            env.elaborate_module_from_roots(&roots, "Data.Owned")
                .expect("same-unit sibling import must remain relative");
            let (_, sibling_body) = env
                .env
                .transparent_body(env.globals["Data.Owned.P.selected"])
                .expect("sibling selector is transparent");
            assert_ne!(env.globals["Data.Owned.N.x"], env.globals["N.x"]);
            match sibling_body {
                ken_kernel::Term::Const { id, .. } => {
                    assert_eq!(id, env.globals["Data.Owned.N.x"])
                }
                other => panic!("same-unit sibling must reference its own N.x, got {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.2).
    ///
    /// MEASURED: a previously elaborated inline `Data.Foo.N` cannot satisfy
    /// the later loaded `Data/Foo.ken` unit's bare `import N` at its own root;
    /// cold and preloaded N orders both select the file-backed N.x identity.
    /// CLAIMED: even a same-spelling root edge belongs to its declaring unit,
    /// not a later file with the same qualified name. THE GAP: the positive
    /// file-owned N case below establishes that a legitimate root child works.
    #[test]
    fn file_unit_root_import_requires_its_own_inline_declaration() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::create_dir_all(root.path().join("Data")).expect("create file-unit directory");
        fs::write(root.path().join("N.ken"), "pub const x : Nat = Suc Zero\n")
            .expect("write file-backed N");
        fs::write(
            root.path().join("Data/Foo.ken"),
            "import N\npub const observed : Nat = N.x\n",
        )
        .expect("write loaded Data.Foo unit");
        let roots = [root.path().to_path_buf()];
        for preload_n in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            env.elaborate_file(
                "module Data { module Foo { module N { pub const x : Nat = Zero } } }",
            )
            .expect("separately declared inline Data.Foo.N");
            if preload_n {
                env.elaborate_module_from_roots(&roots, "N")
                    .expect("preload actual file-backed N");
            }
            env.elaborate_module_from_roots(&roots, "Data.Foo")
                .expect("load the real Data/Foo.ken despite earlier inline name");
            assert!(env.module_state.loaded_units.contains_key("N"));
            assert_ne!(env.globals["Data.Foo.N.x"], env.globals["N.x"]);
            let (_, body) = env
                .env
                .transparent_body(env.globals["Data.Foo.observed"])
                .expect("observed is transparent");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, env.globals["N.x"]),
                other => panic!("observed must reference file-backed N.x, got {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.2).
    ///
    /// MEASURED: the same file unit explicitly declares N before its import,
    /// and its own N.x is selected instead of a separately loaded file N.x.
    /// CLAIMED: the file-unit floor does not forbid genuinely declared inline
    /// children. THE GAP: the negative case above supplies the distinct
    /// provider against which declaration ownership must discriminate.
    #[test]
    fn file_unit_root_import_keeps_its_declared_inline_child() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::create_dir_all(root.path().join("Data")).expect("create file-unit directory");
        fs::write(root.path().join("N.ken"), "pub const x : Nat = Suc Zero\n")
            .expect("write file-backed decoy N");
        fs::write(
            root.path().join("Data/Foo.ken"),
            "module N { pub const x : Nat = Zero }\n\
             import N\npub const observed : Nat = N.x\n",
        )
        .expect("write file unit with own inline N");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&[root.path().to_path_buf()], "Data.Foo")
            .expect("real same-unit inline child must be importable");
        assert!(!env.module_state.loaded_units.contains_key("N"));
        let (_, body) = env
            .env
            .transparent_body(env.globals["Data.Foo.observed"])
            .expect("observed is transparent");
        match body {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, env.globals["Data.Foo.N.x"]),
            other => panic!("observed must reference locally declared N.x, got {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: distinct B file units accept `A.N.x` after `import A` and
    /// `K.N.x` after `import A as K`; alias-only `A.N.x` rejects by name.
    /// CLAIMED: only the bound surface prefix reaches the actual inline child.
    /// THE GAP: selective and no-import controls below rule out ambient access.
    #[test]
    fn imported_owner_grants_only_its_bound_inline_child_path() {
        let root = inline_owner_root(INLINE_OWNER);
        let roots = [root.path().to_path_buf()];
        for source in [
            "import A\nconst observed : Nat = A.N.x\n",
            "import A as K\nconst observed : Nat = K.N.x\n",
        ] {
            fs::write(root.path().join("B.ken"), source).expect("write B client");
            let mut env = ElabEnv::new().expect("base environment");
            env.elaborate_module_from_roots(&roots, "B")
                .unwrap_or_else(|error| panic!("B should access the child: {error:?}"));
            assert!(env.globals.contains_key("B.observed"));
            assert!(env.globals.contains_key("A.N.x"));
        }
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst denied : Nat = A.N.x\n",
        )
        .expect("write B with the unbound original prefix");
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&roots, "B") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.N.x"),
            other => panic!("alias-only import must not grant A.N.x: {other:?}"),
        }
        assert!(env.globals.contains_key("A.N.x"));
    }

    /// Promise class: durable invariant (spec 33 §§3.3, 4.1).
    ///
    /// MEASURED: owner, sibling and B consumer refuse private s at the
    /// exact child path after x was registered as a public child export.
    /// CLAIMED: qualified authority never bypasses the child export table.
    /// THE GAP: the independent positive owner/sibling/B tests above prove
    /// that the same paths reach public x; a refused SCC need not commit it.
    #[test]
    fn inline_child_private_leaf_refuses_in_owner_sibling_and_importer() {
        let root = inline_owner_root(&format!(
            "{INLINE_OWNER}const public_control : Nat = A.N.x\n\
             const denied : Nat = A.N.s\n"
        ));
        let roots = [root.path().to_path_buf()];
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&roots, "A") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.N.s"),
            other => panic!("A must not name private child leaf: {other:?}"),
        }
        assert!(env.globals.contains_key("A.N.x"));

        fs::write(
            root.path().join("A.ken"),
            format!(
                "{INLINE_OWNER}module P {{ import N\n\
                 pub const public_control : Nat = N.x\n\
                 const denied : Nat = N.s }}\n"
            ),
        )
        .expect("write private sibling fixture");
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&roots, "A") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "N.s"),
            other => panic!("P must not name private child leaf: {other:?}"),
        }
        assert!(env.globals.contains_key("A.N.x"));

        fs::write(root.path().join("A.ken"), INLINE_OWNER).expect("restore public A");
        for (source, private_name) in [
            (
                "import A\nconst public_control : Nat = A.N.x\n\
                 const denied : Nat = A.N.s\n",
                "A.N.s",
            ),
            (
                "import A as K\nconst public_control : Nat = K.N.x\n\
                 const denied : Nat = K.N.s\n",
                "K.N.s",
            ),
        ] {
            fs::write(root.path().join("B.ken"), source).expect("write B privacy client");
            let mut env = ElabEnv::new().expect("base environment");
            match env.elaborate_module_from_roots(&roots, "B") {
                Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, private_name),
                other => panic!("B must not name private child leaf: {other:?}"),
            }
            assert!(env.globals.contains_key("A.N.x"));
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.2–3.3).
    ///
    /// MEASURED: selective A(top) admits the bare top, but not A.N.x;
    /// previously loaded A also grants no import-free A.N.x in B. CLAIMED:
    /// no imported-owner prefix means no nested module authority. THE GAP:
    /// the qualified and aliased variants above establish that x exists.
    #[test]
    fn selective_or_loaded_owner_does_not_grant_inline_child_access() {
        let root = inline_owner_root(INLINE_OWNER);
        let roots = [root.path().to_path_buf()];
        fs::write(
            root.path().join("B.ken"),
            "import A (top)\nconst public_control : Nat = top\n",
        )
        .expect("write successful selective B client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&roots, "B")
            .expect("selective public top must resolve without nested access");
        assert!(env.globals.contains_key("B.public_control"));

        fs::write(
            root.path().join("B.ken"),
            "import A (top)\nconst public_control : Nat = top\n\
             const denied : Nat = A.N.x\n",
        )
        .expect("write selective B client");
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&roots, "B") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.N.x"),
            other => panic!("selection must not grant nested access: {other:?}"),
        }
        assert!(env.globals.contains_key("A.N.x"));

        fs::write(root.path().join("B.ken"), "const denied : Nat = A.N.x\n")
            .expect("write unimported B client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&roots, "A")
            .expect("A must finish before checking the B cache boundary");
        match env.elaborate_module_from_roots(&roots, "B") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.N.x"),
            other => panic!("loaded A must not grant nested access: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §3.2 declaration order).
    ///
    /// MEASURED: P's `import N` before A declares inline N rejects at N.
    /// CLAIMED: loader knowledge of a later child does not make the import
    /// available early. THE GAP: the succeeding ordered P fixture above
    /// proves that refusal is caused by order, not by missing N altogether.
    #[test]
    fn inline_sibling_import_requires_prior_declaration() {
        let root = inline_owner_root(
            "module P { import N\nconst premature : Nat = N.x }\n\
             module N { pub const x : Nat = Zero }\n",
        );
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&[root.path().to_path_buf()], "A") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "N"),
            other => panic!("import of a later inline sibling must reject: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.2–3.3).
    ///
    /// MEASURED: a later A.N never becomes available at P's import, whether
    /// C loaded file N first, A was the entry, or C loads N after A. CLAIMED:
    /// dependency pre-scan, textual resolution and caller order agree at the
    /// exact import site. THE GAP: the distinct bodies and IDs below rule out
    /// a passing error caused by a missing or interchangeable N provider.
    #[test]
    fn later_same_unit_sibling_rejects_regardless_of_file_import_order() {
        const A: &str = "module P { import N\npub const premature : Nat = N.x }\n\
                         module N { pub const x : Nat = Zero }\n";
        let root = inline_owner_root(A);
        fs::write(
            root.path().join("N.ken"),
            "pub const x : Nat = Suc (Suc Zero)\n",
        )
        .expect("write distinct file-backed N");
        fs::write(
            root.path().join("C1.ken"),
            "import N\nimport A\npub const ready : Nat = Zero\n",
        )
        .expect("write file-first caller");
        fs::write(
            root.path().join("C2.ken"),
            "import A\nimport N\npub const ready : Nat = Zero\n",
        )
        .expect("write file-last caller");
        let roots = [root.path().to_path_buf()];
        let mut results = Vec::new();
        for entry in ["C1", "A", "C2"] {
            let mut env = ElabEnv::new().expect("base environment");
            let result = env.elaborate_module_from_roots(&roots, entry);
            let file_id = env.globals.get("N.x").copied();
            let inline_id = env.globals.get("A.N.x").copied();
            let body_id = env.globals.get("A.P.premature").and_then(|id| {
                let (_, body) = env.env.transparent_body(*id)?;
                match body {
                    ken_kernel::Term::Const { id, .. } => Some(id),
                    other => panic!("premature body must reference N.x: {other:?}"),
                }
            });
            eprintln!(
                "load-order probe: entry={entry} result={result:?} \
                 file={file_id:?} inline={inline_id:?} body={body_id:?}"
            );
            results.push((entry, result, file_id, inline_id, body_id));
        }
        for (entry, result, file_id, inline_id, body_id) in results {
            if entry == "C1" && result.is_ok() {
                assert_eq!(body_id, file_id, "C1 must use file N, not inline A.N");
                assert_ne!(file_id, inline_id, "distinct provider identities");
            }
            match result {
                Err(ElabError::UnboundName { name, span }) => {
                    assert_eq!(name, "N", "entry {entry} must reject at import N");
                    let import_start = A.find("import N").unwrap();
                    assert_eq!(span.start, import_start);
                    assert_eq!(span.end, import_start + "import N".len());
                }
                other => panic!("entry {entry} must reject later A.N at import: {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §3.3 per-unit closure).
    ///
    /// MEASURED: an external A.N.decoy was checked before loading A.ken, but
    /// neither that edge nor a cold run makes A.P's premature import valid.
    /// CLAIMED: an ordered edge must originate in this file unit, not merely
    /// exist globally or appear in the unit's all-declarations pre-scan.
    /// THE GAP: the preloaded run records its actual external GlobalId and
    /// body, distinguishing a wrong provider from an unrelated late failure.
    #[test]
    fn external_inline_edge_cannot_predeclare_later_same_unit_sibling() {
        const A: &str = "module P { import N\npub const bound : Nat = N.decoy }\n\
                         module N { pub const x : Nat = Zero }\n";
        const A_WITH_REPLAY: &str = "module P { class Marker a {}\n\
                         import N\ninstance Marker Nat {}\n\
                         pub const bound : Nat = N.decoy }\n\
                         module N { pub const x : Nat = Zero }\n";
        let root = inline_owner_root(A);
        let roots = [root.path().to_path_buf()];
        let mut results = Vec::new();
        for (preload_inline, with_replay) in
            [(false, false), (true, false), (false, true), (true, true)]
        {
            let source = if with_replay { A_WITH_REPLAY } else { A };
            fs::write(root.path().join("A.ken"), source).expect("write A unit for replay arm");
            let mut env = ElabEnv::new().expect("base environment");
            if preload_inline {
                env.elaborate_file("module A { module N { pub const decoy : Nat = Suc Zero } }")
                    .expect("independent inline A.N.decoy");
            }
            let external_id = env.globals.get("A.N.decoy").copied();
            let result = env.elaborate_module_from_roots(&roots, "A");
            let body_id = env.globals.get("A.P.bound").and_then(|id| {
                let (_, body) = env.env.transparent_body(*id)?;
                match body {
                    ken_kernel::Term::Const { id, .. } => Some(id),
                    other => panic!("bound body must reference decoy: {other:?}"),
                }
            });
            eprintln!(
                "external-edge probe: preload={preload_inline} replay={with_replay} \
                 result={result:?} external={external_id:?} body={body_id:?}"
            );
            results.push((
                preload_inline,
                with_replay,
                source,
                result,
                external_id,
                body_id,
            ));
        }
        for (preload_inline, with_replay, source, result, external_id, body_id) in results {
            if preload_inline && result.is_ok() {
                assert_eq!(body_id, external_id, "base must bind the external decoy");
            }
            match result {
                Err(ElabError::UnboundName { name, span }) => {
                    assert_eq!(
                        name, "N",
                        "preload {preload_inline} replay {with_replay} must reject at import"
                    );
                    let import_start = source.find("import N").unwrap();
                    assert_eq!(span.start, import_start);
                    assert_eq!(span.end, import_start + "import N".len());
                }
                other => panic!(
                    "preload {preload_inline} replay {with_replay} must reject later A.N: {other:?}"
                ),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: at the empty-prefix in-memory unit root, `import N` before
    /// its own `module N` rejects at that import both cold and after a separate
    /// unit installed `N.external`. CLAIMED: a later local root child is not
    /// an absolute import of ambient N. THE GAP: when the preloaded call
    /// succeeds, the checked body/ID below identifies the wrong provider;
    /// the ordered-root positive below proves the same syntax can work.
    #[test]
    fn in_memory_root_import_refuses_later_child_even_when_external_n_is_loaded() {
        const SOURCE: &str = "import N\nconst borrowed : Nat = N.external\n\
                              module N { pub const own : Nat = Zero }\n";
        for preload_external in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            if preload_external {
                env.elaborate_file("module N { pub const external : Nat = Suc Zero }")
                    .expect("independent earlier in-memory unit");
            }
            let external_id = env.globals.get("N.external").copied();
            let result = env.elaborate_file(SOURCE);
            let own_id = env.globals.get("N.own").copied();
            let borrowed_id = env.globals.get("borrowed").and_then(|id| {
                let (_, body) = env.env.transparent_body(*id)?;
                match body {
                    ken_kernel::Term::Const { id, .. } => Some(id),
                    other => panic!("borrowed must be a global selector: {other:?}"),
                }
            });
            eprintln!(
                "in-memory root probe: preload={preload_external} result={result:?} \
                 external={external_id:?} own={own_id:?} borrowed={borrowed_id:?}"
            );
            if result.is_ok() {
                assert_eq!(
                    borrowed_id, external_id,
                    "wrong provider is the earlier unit"
                );
                assert_ne!(
                    own_id, external_id,
                    "own declaration is a distinct identity"
                );
            }
            match result {
                Err(ElabError::UnboundName { name, span }) => {
                    assert_eq!(name, "N");
                    assert_eq!((span.start, span.end), (0, "import N".len()));
                }
                other => panic!(
                    "later root child must reject regardless of preload {preload_external}: {other:?}"
                ),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: `module N` expanded before an in-memory import at the root
    /// or in sibling P selects this unit's N.own, not the other unit's
    /// preloaded N.external. CLAIMED: the empty-root declaration order, not
    /// an unconditional refusal or ambient selection, governs both scopes.
    /// THE GAP: the paired later-child case supplies the negative boundary.
    #[test]
    fn in_memory_root_import_keeps_earlier_declared_child() {
        const ROOT: &str = "module N { pub const own : Nat = Zero }\n\
                            import N\nconst selected : Nat = N.own\n";
        const SIBLING: &str = "module N { pub const own : Nat = Zero }\n\
                               module P { import N\npub const selected : Nat = N.own }\n";
        for (source, selected_name) in [(ROOT, "selected"), (SIBLING, "P.selected")] {
            for preload_external in [false, true] {
                let mut env = ElabEnv::new().expect("base environment");
                if preload_external {
                    env.elaborate_file("module N { pub const external : Nat = Suc Zero }")
                        .expect("independent earlier in-memory unit");
                }
                env.elaborate_file(source)
                    .expect("earlier same-unit root child must remain importable");
                let own_id = env.globals["N.own"];
                if preload_external {
                    assert_ne!(own_id, env.globals["N.external"]);
                }
                let (_, body) = env
                    .env
                    .transparent_body(env.globals[selected_name])
                    .expect("selected is transparent");
                match body {
                    ken_kernel::Term::Const { id, .. } => assert_eq!(id, own_id),
                    other => panic!("{selected_name} must use this unit's N.own: {other:?}"),
                }
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: an earlier A.N's ordered edge is selected over an actual
    /// N.ken, regardless of whether the file was preloaded; the transparent
    /// body contains the exact checked inline GlobalId. CLAIMED: rejecting
    /// later children does not disable legitimate prior sibling imports.
    /// THE GAP: the two failing-order fixtures above supply the opposite arm.
    #[test]
    fn earlier_same_unit_sibling_wins_over_file_import_in_both_orders() {
        let root = inline_owner_root(
            "module N { pub const x : Nat = Zero }\n\
             module P { import N\npub const selected : Nat = N.x }\n",
        );
        fs::write(
            root.path().join("N.ken"),
            "pub const x : Nat = Suc (Suc Zero)\n",
        )
        .expect("write file-backed decoy N");
        let roots = [root.path().to_path_buf()];
        for preload_file_n in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            if preload_file_n {
                env.elaborate_module_from_roots(&roots, "N")
                    .expect("preload independent N.ken");
            }
            env.elaborate_module_from_roots(&roots, "A")
                .expect("prior same-unit inline A.N is available");
            let inline_id = env.globals["A.N.x"];
            if preload_file_n {
                assert_ne!(inline_id, env.globals["N.x"]);
            } else {
                assert!(!env.module_state.loaded_units.contains_key("N"));
            }
            let (_, body) = env
                .env
                .transparent_body(env.globals["A.P.selected"])
                .expect("selected is transparent");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, inline_id),
                other => panic!("selected must use inline A.N.x: {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.2–3.3).
    ///
    /// MEASURED: an unrelated inline N provides only decoy; A's `import N`
    /// still loads catalog N.ken and its checked N.x. CLAIMED: an ambient
    /// export-table hit is not file-import authority. THE GAP: the earlier
    /// same-unit case above proves this is not a ban on inline modules.
    #[test]
    fn file_import_uses_catalog_source_despite_unrelated_inline_name() {
        let root = inline_owner_root("import N\npub const selected : Nat = N.x\n");
        fs::write(root.path().join("N.ken"), "pub const x : Nat = Suc Zero\n")
            .expect("write catalog source for N");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file("module N { pub const decoy : Nat = Zero }")
            .expect("independent inline N is checked");
        let unrelated_id = env.globals["N.decoy"];
        let roots = [root.path().to_path_buf()];
        let result = env.elaborate_module_from_roots(&roots, "A");
        eprintln!("absolute file-import probe: result={result:?} unrelated={unrelated_id:?}");
        result.expect("A must import the actual N.ken even with ambient inline N");
        let file_id = env.globals["N.x"];
        assert_ne!(unrelated_id, file_id);
        assert!(env.module_state.loaded_units.contains_key("N"));
        let (_, body) = env
            .env
            .transparent_body(env.globals["A.selected"])
            .expect("selected is transparent");
        match body {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file_id),
            other => panic!("selected must use catalog N.x: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §3.3 per-unit closure).
    ///
    /// MEASURED: two unimported spellings in inline sibling P reject even
    /// though A already declared N. CLAIMED: only A's own scope inherits its
    /// declaration authority; P must import N. THE GAP: the positive sibling
    /// import test above keeps this refusal from being a missing-child test.
    #[test]
    fn inline_sibling_does_not_inherit_owner_declaration_access() {
        for reference in ["N.x", "A.N.x"] {
            let root = inline_owner_root(&format!(
                "{INLINE_OWNER}module P {{ const denied : Nat = {reference} }}\n"
            ));
            let mut env = ElabEnv::new().expect("base environment");
            match env.elaborate_module_from_roots(&[root.path().to_path_buf()], "A") {
                Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, reference),
                other => panic!("P must import N to name {reference}: {other:?}"),
            }
            assert!(env.globals.contains_key("A.N.x"));
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a file A's own N is importable by P, but P cannot name
    /// N.Q.leak unless this file also declared Q, cold or after an unrelated
    /// in-memory A.N.Q.leak has been checked. The refusal is at the reference
    /// and the existing external id cannot become the checked borrowed body.
    /// CLAIMED: every descendant edge needs this file unit's provenance.
    /// THE GAP: the paired own-Q positive and file-import-owner negative below
    /// distinguish a real edge from an ambient same-spelling edge.
    #[test]
    fn file_unit_refuses_external_inline_grandchild_cold_and_preloaded() {
        let root = inline_owner_root(
            "module N { pub const own : Nat = Zero }\n\
             module P { import N\n\
             pub const borrowed : Nat = N.Q.leak }\n",
        );
        let source = fs::read_to_string(root.path().join("A.ken")).expect("read fixture source");
        for preload in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            let external = if preload {
                env.elaborate_file(
                    "module A { module N { module Q { pub const leak : Nat = Suc Zero } } }",
                )
                .expect("unrelated in-memory descendant");
                Some(env.globals["A.N.Q.leak"])
            } else {
                None
            };
            let error = env
                .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
                .expect_err("another unit's grandchild is not this file's declaration");
            let at = source
                .find("N.Q.leak")
                .expect("one attempted grandchild reference");
            match error {
                ElabError::UnboundName { name, span } => {
                    assert_eq!(name, "N.Q.leak", "preload={preload}");
                    assert_eq!((span.start, span.end), (at, at + "N.Q.leak".len()));
                }
                other => panic!("preload={preload}: wrong refusal: {other:?}"),
            }
            assert!(env.globals.contains_key("A.N.own"));
            assert_eq!(env.globals.get("A.N.Q.leak").copied(), external);
            assert!(!env.globals.contains_key("A.P.borrowed"));
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: when A.ken actually declares N.Q, P's import N authorizes
    /// N.Q.leak and binds the new file's checked id, not an earlier in-memory
    /// identity with the same spelling. CLAIMED: the per-edge gate preserves
    /// transitive authority for genuine same-unit descendants. THE GAP: the
    /// missing-Q negative above rejects the other provenance at this path.
    #[test]
    fn file_unit_own_inline_grandchild_selects_its_checked_identity() {
        let root = inline_owner_root(
            "module N { pub const own : Nat = Zero\n\
             module Q { pub const leak : Nat = Zero } }\n\
             module P { import N\n\
             pub const borrowed : Nat = N.Q.leak }\n",
        );
        for preload in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            let unrelated = if preload {
                env.elaborate_file(
                    "module A { module N { module Q { pub const leak : Nat = Suc Zero } } }",
                )
                .expect("unrelated in-memory descendant");
                Some(env.globals["A.N.Q.leak"])
            } else {
                None
            };
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
                .expect("file-owned N.Q must remain accessible through imported N");
            let own = env.globals["A.N.Q.leak"];
            if let Some(unrelated) = unrelated {
                assert_ne!(own, unrelated, "file Q must have a new checked identity");
            }
            let (_, borrowed) = env
                .env
                .transparent_body(env.globals["A.P.borrowed"])
                .expect("P's public body is checked");
            match borrowed {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, own),
                other => panic!("own grandchild body did not select Q.leak: {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a separate in-memory import of a completed owner can still
    /// reach the owner's genuinely declared grandchild under an alias.
    /// CLAIMED: the provenance net covers the in-memory module boundary too,
    /// rather than excluding all descendants once the source call returns.
    /// THE GAP: the file-unit negatives above exclude unrelated provenance.
    #[test]
    fn in_memory_import_preserves_its_own_inline_grandchild() {
        let mut env = ElabEnv::new().expect("base environment");
        let trust_before = env.env.trusted_base();
        env.elaborate_file("module A { module N { module Q { pub const x : Nat = Zero } } }")
            .expect("first in-memory unit declares A.N.Q");
        let own = env.globals["A.N.Q.x"];
        env.elaborate_file("import A as K\nconst observed : Nat = K.N.Q.x")
            .expect("later in-memory import retains A's declared grandchild");
        let (_, body) = env
            .env
            .transparent_body(env.globals["observed"])
            .expect("imported grandchild has a checked body");
        match body {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, own),
            other => panic!("owner import lost its exact grandchild: {other:?}"),
        }
        assert_eq!(env.env.trusted_base(), trust_before);
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: after a file A with N.Q.leak is loaded, a new in-memory
    /// A with only N.memory replaces A's export provider. An in-memory import
    /// can select that memory leaf, but cannot borrow the old file Q edge.
    /// The file-root import B still selects the file-owned Q despite the
    /// unrelated in-memory owner. CLAIMED: the import's provenance belongs
    /// to its selected export provider, not a preferred map of the same name.
    /// THE GAP: the inverse source-order case below checks the file provider.
    #[test]
    fn in_memory_import_uses_current_owner_not_stale_file_provenance() {
        let root = inline_owner_root("module N { module Q { pub const leak : Nat = Zero } }\n");
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst file_selected : Nat = K.N.Q.leak\n",
        )
        .expect("write separate strict file-import client");
        let mut env = ElabEnv::new().expect("base environment");
        let trust_before = env.env.trusted_base();
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("load file A and its checked Q leaf");
        let old_file = env.globals["A.N.Q.leak"];
        env.elaborate_file("import A as K\nconst earlier_file : Nat = K.N.Q.leak")
            .expect("first import actually named the file-owned descendant");
        let (_, earlier) = env
            .env
            .transparent_body(env.globals["earlier_file"])
            .expect("first import has a checked body");
        match earlier {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, old_file),
            other => panic!("first import did not select file Q: {other:?}"),
        }
        env.elaborate_file("module A { module N { pub const memory : Nat = Suc Zero } }")
            .expect("independent in-memory owner takes A's export slot");
        let memory = env.globals["A.N.memory"];
        env.elaborate_file("import A as K\nconst memory_selected : Nat = K.N.memory")
            .expect("memory owner's own N leaf is importable");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["memory_selected"])
            .expect("memory selection has a checked body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, memory),
            other => panic!("memory owner was not selected: {other:?}"),
        }
        let source = "import A as K\nconst selected : Nat = K.N.Q.leak";
        let error = env
            .elaborate_file(source)
            .expect_err("memory A did not declare file A's Q edge");
        let at = source.find("K.N.Q.leak").expect("one denied reference");
        match error {
            ElabError::UnboundName { name, span } => {
                assert_eq!(name, "K.N.Q.leak");
                assert_eq!((span.start, span.end), (at, at + "K.N.Q.leak".len()));
            }
            other => panic!("wrong refusal for absent memory Q: {other:?}"),
        }
        assert!(!env.globals.contains_key("selected"));
        assert_eq!(env.globals["A.N.Q.leak"], old_file);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file B imports the actual file A, not the in-memory A");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["B.file_selected"])
            .expect("B's checked body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, old_file),
            other => panic!("file-root import lost the file-owned Q: {other:?}"),
        }
        assert_eq!(env.env.trusted_base(), trust_before);
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: B's strict import of A still selects A.ken's public root
    /// leaf after an in-memory A replaces the global export table. It refuses
    /// a memory-only root leaf at the exact selector. CLAIMED: file-backed
    /// imports use that file's public export table as well as its inline
    /// provenance; the current global table is not a substitute for either.
    /// THE GAP: the in-memory-owner test above chooses the opposite provider.
    #[test]
    fn file_root_import_uses_its_own_exports_after_memory_shadow() {
        let root = inline_owner_root("pub const file_only : Nat = Zero\n");
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst selected : Nat = K.file_only\n",
        )
        .expect("write positive file importer");
        let c_source = "import A as K\nconst denied : Nat = K.memory_only\n";
        fs::write(root.path().join("C.ken"), c_source).expect("write negative file importer");
        fs::write(
            root.path().join("D.ken"),
            "import A (file_only)\nconst selected : Nat = file_only\n",
        )
        .expect("write selective file importer");
        fs::write(
            root.path().join("E.ken"),
            "import A (memory_only)\nconst denied : Nat = memory_only\n",
        )
        .expect("write forbidden selective importer");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("load file A");
        let file = env.globals["A.file_only"];
        env.elaborate_file("module A { pub const memory_only : Nat = Suc Zero }")
            .expect("later memory A has a disjoint root export");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file B must import A.ken's original public root leaf");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["B.selected"])
            .expect("file selection has a checked body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file),
            other => panic!("file import lost its own leaf: {other:?}"),
        }
        let error = env
            .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "C")
            .expect_err("C must not borrow A's memory-only root leaf");
        let at = c_source
            .find("K.memory_only")
            .expect("one denied reference");
        match error {
            ElabError::UnboundName { name, span } => {
                assert_eq!(name, "K.memory_only");
                assert_eq!((span.start, span.end), (at, at + "K.memory_only".len()));
            }
            other => panic!("wrong refusal for memory-only root leaf: {other:?}"),
        }
        assert!(!env.globals.contains_key("C.denied"));
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "D")
            .expect("selective import selects A.ken's original public leaf");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["D.selected"])
            .expect("selective file import has a checked body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file),
            other => panic!("selective file import lost its own leaf: {other:?}"),
        }
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "E") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.memory_only"),
            other => panic!("selective file import borrowed memory owner: {other:?}"),
        }
        assert!(!env.globals.contains_key("E.denied"));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a later memory owner redeclares the exact A.N.Q module
    /// path with different public leaves. A file-root client still sees the
    /// original file Q export table, not memory Q's exported leaf.
    /// CLAIMED: every descendant's public table shares the same selected
    /// file provider as its authorizing inline edge. THE GAP: the root-only
    /// case above separately checks the file root's own public interface.
    #[test]
    fn file_import_keeps_descendant_exports_from_selected_file_provider() {
        let root =
            inline_owner_root("module N { module Q { pub const file_only : Nat = Zero } }\n");
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst selected : Nat = K.N.Q.file_only\n",
        )
        .expect("write file descendant client");
        let c_source = "import A as K\nconst denied : Nat = K.N.Q.memory_only\n";
        fs::write(root.path().join("C.ken"), c_source).expect("write denied client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("load file A.Q");
        let file = env.globals["A.N.Q.file_only"];
        env.elaborate_file(
            "module A { module N { module Q { pub const memory_only : Nat = Suc Zero } } }",
        )
        .expect("memory owner replaces every shared module table");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file B retains A.ken's nested export identity");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["B.selected"])
            .expect("file selected body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file),
            other => panic!("file descendant export was not selected: {other:?}"),
        }
        let error = env
            .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "C")
            .expect_err("file import cannot select memory Q's public leaf");
        let at = c_source
            .find("K.N.Q.memory_only")
            .expect("one denied reference");
        match error {
            ElabError::UnboundName { name, span } => {
                assert_eq!(name, "K.N.Q.memory_only");
                assert_eq!((span.start, span.end), (at, at + "K.N.Q.memory_only".len()));
            }
            other => panic!("wrong descendant export refusal: {other:?}"),
        }
        assert!(!env.globals.contains_key("C.denied"));
    }

    /// Promise class: durable invariant (spec 33 §3.2).
    ///
    /// MEASURED: file B's facade export selects A.ken's public leaf even
    /// after an in-memory A replaces the global export table; it cannot
    /// republish A's memory-only leaf. CLAIMED: facade file dependencies use
    /// the same file-identity boundary as ordinary file imports.
    /// THE GAP: the direct-import control above pins the qualified path.
    #[test]
    fn file_facade_uses_source_file_exports_not_memory_shadow() {
        let root = inline_owner_root("pub const file_only : Nat = Zero\n");
        fs::write(root.path().join("B.ken"), "export A (file_only)\n").expect("write file facade");
        fs::write(
            root.path().join("Good.ken"),
            "import B as K\nconst selected : Nat = K.file_only\n",
        )
        .expect("write facade consumer");
        fs::write(root.path().join("C.ken"), "export A (memory_only)\n")
            .expect("write forbidden facade");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("load file A");
        let file = env.globals["A.file_only"];
        env.elaborate_file("module A { pub const memory_only : Nat = Suc Zero }")
            .expect("later in-memory A has different exports");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Good")
            .expect("facade B must republish the file A leaf");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["Good.selected"])
            .expect("facade consumer has a checked body");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file),
            other => panic!("facade did not select file A: {other:?}"),
        }
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "C") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.memory_only"),
            other => panic!("file facade cannot republish memory owner: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: file A first declares Q, then in-memory A redeclares Q.
    /// Import from memory must bind its newer checked Q identity, not the
    /// same-spelling cached file identity. CLAIMED: source-order changes the
    /// selected provider but cannot silently switch to a stale provenance.
    /// THE GAP: the missing-Q case above detects over-authorization.
    #[test]
    fn in_memory_q_after_file_selects_its_own_checked_identity() {
        let root = inline_owner_root("module N { module Q { pub const leak : Nat = Zero } }\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file A owns its Q");
        let file = env.globals["A.N.Q.leak"];
        env.elaborate_file(
            "module A { module N { module Q { pub const leak : Nat = Suc Zero } } }",
        )
        .expect("later in-memory A owns a distinct Q");
        let memory = env.globals["A.N.Q.leak"];
        assert_ne!(file, memory);
        env.elaborate_file("import A as K\nconst selected : Nat = K.N.Q.leak")
            .expect("import chooses the current memory provider");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["selected"])
            .expect("checked selection");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, memory),
            other => panic!("did not select memory-owned Q: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a memory A with N.memory precedes a file A with N.Q.
    /// The later import selects file-owned Q and cannot borrow memory-only
    /// leaves. CLAIMED: provider/export pairing tracks replacement in both
    /// directions, instead of unconditionally preferring in-memory paths.
    /// THE GAP: the file-first memory-second negative above probes the inverse.
    #[test]
    fn file_q_after_memory_owner_selects_file_provenance() {
        let root = inline_owner_root("module N { module Q { pub const leak : Nat = Zero } }\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file("module A { module N { pub const memory : Nat = Suc Zero } }")
            .expect("first memory owner");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("later file owner");
        let file = env.globals["A.N.Q.leak"];
        env.elaborate_file("import A as K\nconst selected : Nat = K.N.Q.leak")
            .expect("import chooses current file provider");
        let (_, selected) = env
            .env
            .transparent_body(env.globals["selected"])
            .expect("checked selection");
        match selected {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file),
            other => panic!("did not select file-owned Q: {other:?}"),
        }
        let source = "import A as K\nconst denied : Nat = K.N.memory";
        let error = env
            .elaborate_file(source)
            .expect_err("file A did not declare the memory-only leaf");
        let at = source.find("K.N.memory").expect("one denied reference");
        match error {
            ElabError::UnboundName { name, span } => {
                assert_eq!(name, "K.N.memory");
                assert_eq!((span.start, span.end), (at, at + "K.N.memory".len()));
            }
            other => panic!("wrong refusal for memory-only leaf: {other:?}"),
        }
        assert!(!env.globals.contains_key("denied"));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a local public `A.leak` and a facade-selected file
    /// `A.leak` share their canonical spelling but not their checked ID, and
    /// compete for the SAME public surface in one interface. CLAIMED: facade
    /// collision compares checked IDs, not only strings. THE GAP: the file
    /// identity is recorded before the competing local is elaborated.
    #[test]
    fn facade_reexport_refuses_two_ids_under_one_identical_canonical_name() {
        let root = inline_owner_root("pub const leak : Nat = Zero\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file provider checked");
        let file_id = env.globals["A.leak"];
        match env.elaborate_file("module A { pub const leak : Nat = Suc Zero export A (leak) }") {
            Err(ElabError::ReExportCollision { surface_name, existing, incoming, .. }) => {
                assert_eq!(surface_name, "leak");
                assert_eq!(existing, "A.leak");
                assert_eq!(incoming, "A.leak");
            }
            other => panic!("two IDs under one public spelling must clash: {other:?}"),
        }
        assert_ne!(file_id, env.globals["A.leak"]);
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: an explicit local export BEFORE its declaration selects the
    /// new local's ID rather than the old file declaration's ID at the same
    /// canonical spelling. CLAIMED: prebound locals are paired only once
    /// checked. THE GAP: an earlier file ID exists and differs, so an eager
    /// ambient lookup would select the wrong provider and fail this assertion.
    #[test]
    fn forward_local_export_waits_for_its_checked_id() {
        let root = inline_owner_root("pub const leak : Nat = Zero\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file provider checked");
        let old = env.globals["A.leak"];
        env.elaborate_file("module A { export leak\nconst leak : Nat = Suc Zero }")
            .expect("forward in-scope export resolves after local check");
        let local = env.globals["A.leak"];
        assert_ne!(old, local);
        env.elaborate_file("import A as M\nconst selected : Nat = M.leak")
            .expect("memory client selects the forward-published local");
        let (_, body) = env.env.transparent_body(env.globals["selected"])
            .expect("selected local has a checked body");
        match body {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, local),
            other => panic!("forward export selected non-local body: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: exporting a prop family before its declaration still
    /// publishes exactly its checked intro selector to a later strict file
    /// import. CLAIMED: delayed ID reconciliation carries intros as well as
    /// the family. THE GAP: an intro omitted by the early table would fail
    /// at the import site, before the checked proof can be constructed.
    #[test]
    fn forward_prop_export_publishes_its_checked_intro() {
        let root = inline_owner_root(
            "export HasProof\nprop HasProof (a : Type) : Omega where { intro : HasProof a }\n",
        );
        fs::write(
            root.path().join("B.ken"),
            "import A as K\ntheorem selected (a : Type) : K.HasProof a = K.HasProof.intro a\n",
        )
        .expect("write forward-prop client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("forward family export carries the checked intro");
        let file_intro = env.globals["A.HasProof.intro"];
        let (_, body) = env.env.transparent_body(env.globals["B.selected"])
            .expect("checked intro proof has a body");
        let ken_kernel::Term::Lam(_, body) = body else {
            panic!("selected is not a parameterized proof: {body:?}");
        };
        let ken_kernel::Term::App(head, _) = *body else {
            panic!("selected did not apply its intro: {body:?}");
        };
        assert!(matches!(head.as_ref(),
            ken_kernel::Term::Const { id, .. }
            | ken_kernel::Term::Constructor { id, .. } if *id == file_intro));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a forward local and a facade to the file owner publish one
    /// surface/canonical spelling but two IDs. CLAIMED: delayed local-ID
    /// reconciliation cannot let a later facade conceal a real collision.
    /// THE GAP: an earlier `pub` would exercise eager collision only.
    #[test]
    fn forward_local_and_file_facade_collide_when_ids_differ() {
        let root = inline_owner_root("pub const leak : Nat = Zero\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file provider checked");
        let old = env.globals["A.leak"];
        match env.elaborate_file(
            "module A { export leak\nexport A (leak)\nconst leak : Nat = Suc Zero }",
        ) {
            Err(ElabError::ReExportCollision { surface_name, existing, incoming, .. }) => {
                assert_eq!(surface_name, "leak");
                assert_eq!(existing, "A.leak");
                assert_eq!(incoming, "A.leak");
            }
            other => panic!("forward export and facade must clash by ID: {other:?}"),
        }
        assert_ne!(old, env.globals["A.leak"]);
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a file owner and a later or earlier in-memory owner mint
    /// distinct checked declarations at the SAME canonical leaf. File B's
    /// imported alias must select the file's GlobalId even if the mutable
    /// process-global spelling currently names the in-memory GlobalId; an
    /// independent memory import must select the in-memory ID. CLAIMED:
    /// source-to-resolved references carry the chosen provider's identity.
    /// THE GAP: descendant/export checks alone cannot catch an ID re-lookup.
    #[test]
    fn file_import_retains_checked_leaf_identity_across_same_name_memory_owner() {
        let root = inline_owner_root(
            "module N { module Q { pub const leak : Nat = Zero } }\n",
        );
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst selected : Nat = K.N.Q.leak\n",
        )
        .expect("write file-backed client");
        for memory_first in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            let memory_source =
                "module A { module N { module Q { pub const leak : Nat = Suc Zero } } }";
            let first_memory_id = if memory_first {
                env.elaborate_file(memory_source)
                    .expect("first in-memory A has a checked leaf");
                Some(env.globals["A.N.Q.leak"])
            } else {
                None
            };
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
                .expect("load file A's checked leaf");
            let file_id = env.globals["A.N.Q.leak"];
            let memory_id = if memory_first {
                first_memory_id
            } else {
                env.elaborate_file(memory_source)
                    .expect("later in-memory A has a distinct checked leaf");
                Some(env.globals["A.N.Q.leak"])
            };
            assert_ne!(file_id, memory_id.expect("both owners are checked"));
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
                .expect("B imports file A even with the memory A in process");
            let (_, body) = env
                .env
                .transparent_body(env.globals["B.selected"])
                .expect("checked B selection");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, file_id, "memory_first={memory_first}"),
                other => panic!("file import selected non-constant: {other:?}"),
            }
            if !memory_first {
                env.elaborate_file("import A as M\nconst memory_selected : Nat = M.N.Q.leak")
                    .expect("separate in-memory importer selects its current owner");
                let (_, body) = env
                    .env
                    .transparent_body(env.globals["memory_selected"])
                    .expect("checked memory selection");
                match body {
                    ken_kernel::Term::Const { id, .. } => assert_eq!(id, memory_id.unwrap()),
                    other => panic!("memory import selected non-constant: {other:?}"),
                }
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a file-owned inductive former and an in-memory former share
    /// one canonical name but not one checked ID. A strict imported annotation
    /// in both the domain and result type retains the selected file ID.
    /// CLAIMED: type elaboration consumes the same provider identity as term
    /// elaboration. THE GAP: asserting both Pi branches prevents a body-only
    /// repair from being mistaken for a type-position repair.
    #[test]
    fn file_import_retains_checked_type_identity_across_same_name_memory_owner() {
        let root = inline_owner_root("pub data T = MkT\n");
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nfn selected (x : K.T) : K.T = x\n",
        )
        .expect("write file-backed type-position client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file A declares an inductive former");
        let file = env.globals["A.T"];
        env.elaborate_file("module A { pub data T = Other }")
            .expect("in-memory A declares a distinct same-spelling former");
        let memory = env.globals["A.T"];
        assert_ne!(file, memory);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("checked type positions select file A");
        let (_, checked_type) = env.env.const_type(env.globals["B.selected"])
            .expect("selected declaration has a checked type");
        match checked_type {
            ken_kernel::Term::Pi(domain, codomain) => {
                assert_eq!(*domain, ken_kernel::Term::IndFormer { id: file, level_args: vec![] });
                assert_eq!(*codomain, ken_kernel::Term::IndFormer { id: file, level_args: vec![] });
            }
            other => panic!("expected selected file Pi, got {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: explicit constructor export at a file owner stays distinct
    /// from a later same-spelling non-constructor declaration in memory.
    /// CLAIMED: an imported constructor pattern uses its selected checked ID.
    /// THE GAP: the matching term and type both select the same file family.
    #[test]
    fn file_import_constructor_pattern_keeps_provider_checked_identity() {
        let root = inline_owner_root("pub data T = MkT\nexport MkT\n");
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nfn matched (x : K.T) : Nat = match x { K.MkT ↦ Zero }\n",
        )
        .expect("write constructor-pattern client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file A explicitly exports its constructor");
        let file_ctor = env.globals["A.MkT"];
        let file_former = env.globals["A.T"];
        env.elaborate_file("module A { pub const MkT : Nat = Zero }")
            .expect("memory A owns distinct same-spelling non-constructor");
        assert_ne!(file_ctor, env.globals["A.MkT"]);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("imported file pattern selects the checked constructor");
        assert!(env.env.constructor(file_ctor).is_some());
        let (_, checked_type) = env.env.const_type(env.globals["B.matched"])
            .expect("matched has a checked function type");
        match checked_type {
            ken_kernel::Term::Pi(domain, _) => {
                assert_eq!(*domain, ken_kernel::Term::IndFormer { id: file_former, level_args: vec![] });
            }
            other => panic!("matched was not a function: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: a selective file binding and a later in-memory binding
    /// contest the SAME bare name, despite identical canonical strings.
    /// CLAIMED: ID inequality rejects true selective clashes, while a repeated
    /// import of the SAME selected ID remains idempotent. THE GAP: a qualified
    /// file-only use would not exercise this unqualified binding collision.
    #[test]
    fn selective_import_distinguishes_same_spelling_providers_and_same_idempotence() {
        let root = inline_owner_root("pub const leak : Nat = Zero\n");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file provider checked");
        let file_id = env.globals["A.leak"];
        env.elaborate_file("import A (leak)\nconst same : Nat = leak")
            .expect("selective file binding checked");
        env.elaborate_file("import A (leak)\nconst again : Nat = leak")
            .expect("repeat of one checked ID is idempotent");
        for name in ["same", "again"] {
            let (_, body) = env.env.transparent_body(env.globals[name]).expect("checked body");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, file_id),
                other => panic!("{name} did not select file ID: {other:?}"),
            }
        }
        env.elaborate_file("module A { pub const leak : Nat = Suc Zero }")
            .expect("second in-memory provider checked");
        assert_ne!(env.globals["A.leak"], file_id);
        match env.elaborate_file("import A (leak)\nconst wrong : Nat = leak") {
            Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "leak"),
            other => panic!("distinct IDs on one bare selective name must clash: {other:?}"),
        }
        assert!(!env.globals.contains_key("wrong"));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: both the facade and in-scope re-export of file A's leaf
    /// preserve the file's checked ID after memory A owns the same name.
    /// CLAIMED: a second module interface republishes the provider identity,
    /// not its canonical spelling's current mutable lookup. THE GAP: both
    /// facade and in-scope paths are independently read in checked bodies.
    #[test]
    fn file_reexports_preserve_provider_id_through_facade_and_in_scope() {
        let root = inline_owner_root("pub const leak : Nat = Zero\n");
        for (name, source) in [
            ("P.ken", "export A (leak as facade)\n"),
            ("Q.ken", "import A (leak)\nexport leak as facade\n"),
            ("B.ken", "import P as PF\nimport Q as QS\nconst from_facade : Nat = PF.facade\nconst from_in_scope : Nat = QS.facade\n"),
        ] {
            fs::write(root.path().join(name), source).expect("write provider-ID relay");
        }
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file A checked");
        let file_id = env.globals["A.leak"];
        env.elaborate_file("module A { pub const leak : Nat = Suc Zero }")
            .expect("memory A checked");
        assert_ne!(file_id, env.globals["A.leak"]);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file re-exports carry the original checked provider");
        for name in ["B.from_facade", "B.from_in_scope"] {
            let (_, body) = env.env.transparent_body(env.globals[name]).expect("checked body");
            match body {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, file_id, "{name}"),
                other => panic!("{name} did not select file leaf: {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: file prop intro helpers are used through direct alias,
    /// facade, and in-scope selective family paths after a memory declaration
    /// overwrites the same canonical intro spelling. CLAIMED: selectors carry
    /// their owning file family's checked helper ID. THE GAP: both checked
    /// body and type must originate from the file, not the memory impostor.
    #[test]
    fn file_prop_intro_reexports_retain_checked_helper_identity() {
        let root = inline_owner_root(
            "pub prop HasProof (a : Type) : Omega where { intro : HasProof a }\n",
        );
        for (name, source) in [
            ("P.ken", "export A (HasProof as Proof)\n"),
            ("Q.ken", "import A (HasProof)\nexport HasProof as Claim\n"),
            ("B.ken", "import A as K\nimport P as F\nimport Q as I\ntheorem direct (a : Type) : K.HasProof a = K.HasProof.intro a\ntheorem facade (a : Type) : F.Proof a = F.Proof.intro a\ntheorem in_scope (a : Type) : I.Claim a = I.Claim.intro a\n"),
        ] {
            fs::write(root.path().join(name), source).expect("write prop selector relay");
        }
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file prop family checked");
        let file_intro = env.globals["A.HasProof.intro"];
        env.elaborate_file("module A { module HasProof { pub const intro : Nat = Zero } }")
            .expect("distinct memory owner checks a same-spelling intro");
        assert_ne!(file_intro, env.globals["A.HasProof.intro"]);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file family selectors keep their owning intro");
        for name in ["B.direct", "B.facade", "B.in_scope"] {
            let (_, body) = env.env.transparent_body(env.globals[name]).expect("checked proof");
            match body {
                ken_kernel::Term::Lam(_, body) => match *body {
                    ken_kernel::Term::App(head, _) => match *head {
                        ken_kernel::Term::Const { id, .. }
                        | ken_kernel::Term::Constructor { id, .. } => assert_eq!(id, file_intro, "{name}"),
                        other => panic!("{name}: wrong prop intro head {other:?}"),
                    },
                    other => panic!("{name}: wrong prop intro body {other:?}"),
                },
                other => panic!("{name}: expected one-parameter proof, got {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: file A's effectful checked `proc` and memory A's pure
    /// same-spelling `const` differ in declared row. A pure strict file
    /// importer must fail purity while an FS-qualified importer succeeds.
    /// CLAIMED: imported checked identity selects both the body and its row.
    /// THE GAP: memory A's ambient spelling is pure, so name-keyed row lookup
    /// would admit the wrong source; the positive rules out blanket rejection.
    #[test]
    fn file_import_uses_checked_provider_effect_row_under_name_collision() {
        let root = inline_owner_root("pub proc f : Nat visits [FS] = Zero\n");
        for (name, source) in [
            ("Bad.ken", "import A as K\nconst bad : Nat = K.f\n"),
            ("Good.ken", "import A as K\nproc good : Nat visits [FS] = K.f\n"),
        ] {
            fs::write(root.path().join(name), source).expect("write effect-row client");
        }
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("effectful file provider checked");
        let file_id = env.globals["A.f"];
        env.elaborate_file("module A { pub const f : Nat = Zero }")
            .expect("pure memory provider checked");
        assert_ne!(file_id, env.globals["A.f"]);
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Bad") {
            Err(ElabError::TypeMismatch { reason, .. }) => {
                assert!(reason.contains("false purity"), "wrong refusal: {reason}");
            }
            other => panic!("pure file client must reject file FS row: {other:?}"),
        }
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Good")
            .expect("declaring file FS row permits the same selected binding");
        let (_, body) = env.env.transparent_body(env.globals["Good.good"])
            .expect("effect-compatible client has checked body");
        match body {
            ken_kernel::Term::Const { id, .. } => assert_eq!(id, file_id),
            other => panic!("effectful client selected the wrong body: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: selective import of a real exported prop family does not
    /// grant an arbitrary same-spelling private selector from another unit;
    /// the authorized `.intro` positive is exercised by the preceding test.
    /// CLAIMED: a family selector is checked against its provider's public
    /// member table. THE GAP: the error and span prove this is name refusal,
    /// not a later type mismatch or failed import.
    #[test]
    fn selective_file_family_refuses_an_external_private_selector() {
        let root = inline_owner_root(
            "pub prop HasProof (a : Type) : Omega where { intro : HasProof a }\n",
        );
        let bad = "import A (HasProof)\nconst denied : Nat = HasProof.private\n";
        fs::write(root.path().join("B.ken"), bad).expect("write private selector client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file family and intro checked");
        env.elaborate_file("module A { module HasProof { pub const private : Nat = Zero } }")
            .expect("unrelated memory owner checks its leaf");
        assert!(env.globals.contains_key("A.HasProof.private"));
        let at = bad.find("HasProof.private").expect("single private selector");
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B") {
            Err(ElabError::UnboundName { name, span }) => {
                assert_eq!(name, "HasProof.private");
                assert_eq!((span.start, span.end), (at, at + "HasProof.private".len()));
            }
            other => panic!("private family selector must not resolve: {other:?}"),
        }
        assert!(!env.globals.contains_key("B.denied"));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3, §8.2).
    ///
    /// MEASURED: `proof same for K.id` selects the file owner's checked
    /// attached proof even when the mutable global-name map is pointed to an
    /// independently checked proof ID at that canonical spelling. A second
    /// source proof of the SAME subject and proof name is refused as a duplicate,
    /// so this map perturbation exercises the downstream consumer seam directly.
    /// CLAIMED: attached selectors carry a checked provider ID. THE GAP:
    /// memory's extra proof is checked, not an invented/unknown ID.
    #[test]
    fn file_attached_proof_selector_retains_checked_provider_id() {
        let root = inline_owner_root(
            "pub fn id (x : Nat) : Nat = x\n\
             pub proof same for id (x : Nat) : Eq Nat (id x) x = Refl\n",
        );
        fs::write(
            root.path().join("B.ken"),
            "import A as K\ntheorem selected (x : Nat) : Eq Nat (K.id x) x = (proof same for K.id) x\n",
        )
        .expect("write file-backed attached-proof client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file attached proof checked");
        let file_proof = env.globals["A.id::same"];
        env.elaborate_file(
            "module A { pub fn id (x : Nat) : Nat = x\n\
             pub proof extra for id (x : Nat) : Eq Nat (id x) x = Refl }",
        )
        .expect("memory owner checks a different attached proof");
        let memory_proof = env.globals["A.id::extra"];
        assert_ne!(file_proof, memory_proof);
        assert_eq!(env.globals.insert("A.id::same".to_string(), memory_proof), Some(file_proof));
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file attached proof selector uses file's checked ID");
        let (_, body) = env.env.transparent_body(env.globals["B.selected"])
            .expect("proof client has checked body");
        let ken_kernel::Term::Lam(_, body) = body else {
            panic!("selected should bind one argument: {body:?}");
        };
        let ken_kernel::Term::App(head, _) = *body else {
            panic!("selected should apply file proof: {body:?}");
        };
        assert!(matches!(head.as_ref(), ken_kernel::Term::Const { id, .. } if *id == file_proof));

        fs::write(
            root.path().join("Selective.ken"),
            "import A (id)\ntheorem selected (x : Nat) : Eq Nat (id x) x = id::same x\n",
        )
        .expect("write selective attached-proof client");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Selective")
            .expect("selective subject retains the checked attached-proof ID");
        let (_, body) = env.env.transparent_body(env.globals["Selective.selected"])
            .expect("selective proof client has a checked body");
        let ken_kernel::Term::Lam(_, body) = body else {
            panic!("selective proof should bind one argument: {body:?}");
        };
        let ken_kernel::Term::App(head, _) = *body else {
            panic!("selective proof should apply checked helper: {body:?}");
        };
        assert!(matches!(head.as_ref(), ken_kernel::Term::Const { id, .. } if *id == file_proof));

        fs::write(
            root.path().join("Bad.ken"),
            "import A as K\ntheorem denied (x : Nat) : Eq Nat (K.id x) x = (proof extra for K.id) x\n",
        )
        .expect("write memory-only attached-proof negative");
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Bad") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.id::extra"),
            other => panic!("file import cannot borrow memory-only proof: {other:?}"),
        }
        fs::write(
            root.path().join("BadDirect.ken"),
            "import A (id)\ntheorem denied (x : Nat) : Eq Nat (id x) x = id::extra x\n",
        )
        .expect("write memory-only selective selector negative");
        match env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "BadDirect") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.id::extra"),
            other => panic!("selective file subject cannot borrow memory proof: {other:?}"),
        }
        env.elaborate_file(
            "import A as M\ntheorem memory_selected (x : Nat) : Eq Nat (M.id x) x = (proof extra for M.id) x",
        )
        .expect("independent memory importer selects its own attached proof");
        let (_, body) = env.env.transparent_body(env.globals["memory_selected"])
            .expect("memory proof has a checked body");
        let ken_kernel::Term::Lam(_, body) = body else {
            panic!("memory proof should bind one argument: {body:?}");
        };
        let ken_kernel::Term::App(head, _) = *body else {
            panic!("memory proof should apply selected helper: {body:?}");
        };
        assert!(matches!(head.as_ref(), ken_kernel::Term::Const { id, .. } if *id == memory_proof));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3, §6).
    ///
    /// MEASURED: a selectively imported symbolic operator keeps the file
    /// provider's checked ID and right fixity when the memory provider has
    /// the same canonical name and left fixity. CLAIMED: both reassociation
    /// and operator elaboration consume the selected ID. THE GAP: the exact
    /// right-nested checked term distinguishes either wrong fixity or ID.
    #[test]
    fn file_operator_import_retains_checked_id_and_fixity() {
        let root = inline_owner_root(
            "pub fn <+> (a : Nat) (b : Nat) : Nat = a\ninfixr 5 <+>\n",
        );
        fs::write(
            root.path().join("B.ken"),
            "import A (<+>)\nfn selected (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c\n",
        )
        .expect("write operator client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("file operator and fixity checked");
        let file_op = env.globals["A.<+>"];
        env.elaborate_file(
            "module A { pub fn <+> (a : Nat) (b : Nat) : Nat = b\ninfixl 5 <+> }",
        )
        .expect("memory operator with left fixity checked");
        let memory_op = env.globals["A.<+>"];
        assert_ne!(file_op, memory_op);
        assert_eq!(env.fixities[&file_op].associativity, crate::ast::FixityAssoc::Right);
        assert_eq!(env.fixities[&memory_op].associativity, crate::ast::FixityAssoc::Left);
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
            .expect("file operator wins over the unrelated memory identity");
        let (_, body) = env.env.transparent_body(env.globals["B.selected"])
            .expect("operator client has checked body");
        let mut expr = &body;
        for _ in 0..3 {
            let ken_kernel::Term::Lam(_, next) = expr else {
                panic!("expected three parameters, got {expr:?}");
            };
            expr = next;
        }
        fn applied(term: &ken_kernel::Term, file_op: GlobalId) -> (&ken_kernel::Term, &ken_kernel::Term) {
            let ken_kernel::Term::App(first, rhs) = term else {
                panic!("expected saturated infix application: {term:?}");
            };
            let ken_kernel::Term::App(op, lhs) = first.as_ref() else {
                panic!("expected operator application: {first:?}");
            };
            assert!(matches!(op.as_ref(), ken_kernel::Term::Const { id, .. } if *id == file_op));
            (lhs.as_ref(), rhs.as_ref())
        }
        let (a, rest) = applied(expr, file_op);
        assert_eq!(*a, ken_kernel::Term::Var(2));
        let (b, c) = applied(rest, file_op);
        assert_eq!(*b, ken_kernel::Term::Var(1));
        assert_eq!(*c, ken_kernel::Term::Var(0));
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: B imports file A as K and reaches A's real N.own, but not
    /// the independently checked in-memory A.N.Q.leak, cold or preloaded.
    /// CLAIMED: imported file owners carry their own inline edge provenance,
    /// not process-global edges. THE GAP: the own-Q positive above ensures
    /// imported modules are not simply denied all descendants.
    #[test]
    fn imported_file_owner_refuses_an_external_same_spelling_grandchild() {
        let root = inline_owner_root("module N { pub const own : Nat = Zero }\n");
        fs::write(
            root.path().join("Good.ken"),
            "import A as K\nconst selected : Nat = K.N.own\n",
        )
        .expect("write positive imported-owner client");
        let b_source = "import A as K\nconst denied : Nat = K.N.Q.leak\n";
        fs::write(root.path().join("B.ken"), b_source)
            .expect("write negative imported-owner client");
        for preload in [false, true] {
            let mut env = ElabEnv::new().expect("base environment");
            let external = if preload {
                env.elaborate_file(
                    "module A { module N { module Q { pub const leak : Nat = Suc Zero } } }",
                )
                .expect("unrelated in-memory descendant");
                Some(env.globals["A.N.Q.leak"])
            } else {
                None
            };
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Good")
                .expect("file A's own N is reachable through a separate strict file import");
            let (_, selected) = env
                .env
                .transparent_body(env.globals["Good.selected"])
                .expect("positive import has a checked body");
            match selected {
                ken_kernel::Term::Const { id, .. } => assert_eq!(id, env.globals["A.N.own"]),
                other => panic!("file A's child body was not selected: {other:?}"),
            }
            let error = env
                .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "B")
                .expect_err("file A must not re-export an external inline grandchild");
            let at = b_source
                .find("K.N.Q.leak")
                .expect("one denied grandchild reference");
            match error {
                ElabError::UnboundName { name, span } => {
                    assert_eq!(name, "K.N.Q.leak", "preload={preload}");
                    assert_eq!((span.start, span.end), (at, at + "K.N.Q.leak".len()));
                }
                other => panic!("preload={preload}: wrong refusal: {other:?}"),
            }
            assert_eq!(env.globals.get("A.N.Q.leak").copied(), external);
            assert!(!env.globals.contains_key("B.denied"));
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3, 4.1).
    ///
    /// MEASURED: A and P name a grandchild through declaration / import,
    /// alias K reaches it from B, and its private leaf refuses at K.N.Q.s.
    /// CLAIMED: authorization follows every inline declaration edge and
    /// checks the terminal child's own public export map. THE GAP: the
    /// disjoint file-backed control below excludes dotted-spelling inference.
    #[test]
    fn inline_grandchild_preserves_transitive_authority_and_privacy() {
        let root = inline_owner_root(
            "module N { module Q { pub const x : Nat = Zero\n\
             const s : Nat = Suc Zero } }\n\
             const own : Nat = A.N.Q.x\n\
             module P { import N\nconst sibling : Nat = N.Q.x }\n\
             pub const top : Nat = Zero\n",
        );
        let roots = [root.path().to_path_buf()];
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst observed : Nat = K.N.Q.x\n",
        )
        .expect("write aliased grandchild client");
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&roots, "B")
            .expect("every declared inline edge must be available through K");
        for name in ["A.N.Q.x", "A.own", "A.P.sibling", "B.observed"] {
            assert!(env.globals.contains_key(name), "missing {name}");
        }
        fs::write(
            root.path().join("B.ken"),
            "import A as K\nconst denied : Nat = K.N.Q.s\n",
        )
        .expect("write private grandchild client");
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&roots, "B") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "K.N.Q.s"),
            other => panic!("private grandchild must not escape: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §§3.1–3.3).
    ///
    /// MEASURED: an in-memory import of inline A can reach A.top, but not
    /// the independently file-loaded A.N.x. CLAIMED: a file-backed child
    /// never becomes an inline descendant through shared dotted spelling.
    /// THE GAP: the positive A.top import proves A's owner path works.
    #[test]
    fn file_backed_child_never_counts_as_inline_descendant_of_imported_owner() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::create_dir(root.path().join("A")).expect("write A directory");
        fs::write(root.path().join("A/N.ken"), "pub const x : Nat = Zero\n")
            .expect("write independent file-backed A.N");
        let roots = [root.path().to_path_buf()];
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file("module A { pub const top : Nat = Zero }")
            .expect("create unrelated inline owner");
        env.elaborate_module_from_roots(&roots, "A.N")
            .expect("load the real file-backed child");
        env.elaborate_file("import A\nconst public_control : Nat = A.top\n")
            .expect("A.top is accessible through the imported owner");
        match env.elaborate_file("const denied : Nat = A.N.x\n") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "A.N.x"),
            other => panic!("a file-backed A.N must not become inline: {other:?}"),
        }
        assert!(env.globals.contains_key("A.top"));
        assert!(env.globals.contains_key("public_control"));
        assert!(env.globals.contains_key("A.N.x"));
    }

    /// Promise class: durable invariant (spec 33 §3.2, §3.3).
    ///
    /// MEASURED: the roots loader returns exact `UnboundName` for both exported
    /// `M.foo` and `M.bar` when A imports only `M (foo)`. CLAIMED: selective
    /// imports must not grant qualified access. THE GAP: the positive
    /// qualified-import controls below show this is not a missing export.
    #[test]
    fn selective_import_does_not_grant_qualified_access() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::write(
            root.path().join("M.ken"),
            "pub const foo : Nat = Zero\npub const bar : Nat = Zero\n",
        )
        .expect("write exported provider");
        for qualified in ["M.bar", "M.foo"] {
            fs::write(
                root.path().join("A.ken"),
                format!("import M (foo)\nconst probe : Nat = {qualified}\n"),
            )
            .expect("write selective consumer");
            let mut env = ElabEnv::new().expect("base environment");
            match env.elaborate_module_from_roots(&[root.path().to_path_buf()], "A") {
                Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, qualified),
                other => panic!("selective import must not grant {qualified}: {other:?}"),
            }
        }
    }

    /// Promise class: durable invariant (spec 33 §3.3 per-unit closure).
    ///
    /// MEASURED: a sibling imports and loads M before A refers to `M.bar`.
    /// CLAIMED: the loader cache is not ambient import authority for A. THE
    /// GAP: the sibling must successfully elaborate before A is checked.
    #[test]
    fn sibling_loaded_module_does_not_grant_qualified_access() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::write(root.path().join("M.ken"), "pub const bar : Nat = Zero\n")
            .expect("write exported provider");
        fs::write(
            root.path().join("Sibling.ken"),
            "import M\nconst seen : Nat = M.bar\n",
        )
        .expect("write sibling importer");
        fs::write(root.path().join("A.ken"), "const probe : Nat = M.bar\n")
            .expect("write unimported consumer");
        let mut env = ElabEnv::new().expect("base environment");
        let roots = [root.path().to_path_buf()];
        env.elaborate_module_from_roots(&roots, "Sibling")
            .expect("sibling must load M and use its qualified export");
        match env.elaborate_module_from_roots(&roots, "A") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "M.bar"),
            other => panic!("sibling-loaded M must not grant A access: {other:?}"),
        }
    }

    /// Promise class: durable invariant (spec 33 §3.2).
    ///
    /// MEASURED: the same exported foo is usable via each authorized import
    /// shape at the roots-loader boundary. CLAIMED: only qualified and aliased
    /// imports grant dotted access, while selection binds the bare name. THE
    /// GAP: AC-1's identical-path negative fixtures discriminate the grant.
    #[test]
    fn qualified_alias_and_selective_imports_bind_their_distinct_access_paths() {
        let root = tempfile::tempdir().expect("temporary module root");
        fs::write(root.path().join("M.ken"), "pub const foo : Nat = Zero\n")
            .expect("write exported provider");
        for (module, source) in [
            ("Qualified", "import M\nconst probe : Nat = M.foo\n"),
            ("Aliased", "import M as N\nconst probe : Nat = N.foo\n"),
            ("Selective", "import M (foo)\nconst probe : Nat = foo\n"),
        ] {
            fs::write(root.path().join(format!("{module}.ken")), source).expect("write consumer");
            let mut env = ElabEnv::new().expect("base environment");
            env.elaborate_module_from_roots(&[root.path().to_path_buf()], module)
                .unwrap_or_else(|error| panic!("{module} should elaborate: {error:?}"));
        }
        fs::write(
            root.path().join("AliasOriginal.ken"),
            "import M as N\nconst probe : Nat = M.foo\n",
        )
        .expect("write alias-only consumer using the unbound original prefix");
        let mut env = ElabEnv::new().expect("base environment");
        match env.elaborate_module_from_roots(&[root.path().to_path_buf()], "AliasOriginal") {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "M.foo"),
            other => panic!("alias must not grant the original prefix: {other:?}"),
        }
    }

    /// `Pub` is a transparent namespace-effect wrapper for the complete
    /// one-level constructible-leaf population. This table intentionally
    /// includes parser-ineligible public forms because the law is about the
    /// constructible AST, not only today's `pub_eligibility` subset.
    ///
    /// The table is manually maintained: the production classifier is
    /// compile-time exhaustive, but adding a non-`Pub` `Decl` leaf also
    /// requires adding its representative here.
    #[test]
    fn pub_wrapper_preserves_complete_owned_leaf_effects() {
        let leaf_sources = [
            ("boundary", "program"),
            ("view", "const local_view : Bool = True"),
            ("space", "space LocalSpace { mut cell : Int = 0 }"),
            ("let", "let local_let : Bool = True"),
            ("prove", "prove local_prove : Bool"),
            ("prop", "prop LocalProp : Omega where { intro : LocalProp }"),
            ("theorem", "theorem local_theorem : Bool = True"),
            ("axiom", "axiom local_axiom : Bool"),
            (
                "attached-proof",
                "proof local_proof for local_subject : Bool = True",
            ),
            ("law", "law LocalLaw (x) { field : Bool }"),
            ("data", "data LocalData = LocalCtor"),
            (
                "explicit-data",
                "data ExplicitData : Type where { ExplicitCtor : ExplicitData }",
            ),
            ("type-alias", "def LocalAlias = Bool"),
            (
                "foreign",
                "foreign local_foreign : Int = \"probe\" \"libc.so\" pure",
            ),
            ("temporal", "temporal local_temporal { always True }"),
            ("record", "record LocalRecord { field : Bool }"),
            ("class", "class LocalClass { field : Bool }"),
            ("instance", "instance LocalClass Bool { field = True }"),
            ("derive", "derive LocalClass for LocalData"),
            ("module", "module LocalModule {}"),
            ("import", "import LocalModule"),
            ("export", "export local_view"),
        ];

        for (label, source) in leaf_sources {
            let mut declarations = parse_decls(source)
                .unwrap_or_else(|error| panic!("{label} leaf must parse: {error}"));
            assert_eq!(
                declarations.len(),
                1,
                "{label} fixture must construct exactly one leaf"
            );
            let leaf = declarations.pop().expect("one checked declaration");
            assert!(
                !matches!(leaf, Decl::Pub(_)),
                "{label} fixture must be a non-Pub leaf"
            );
            let expected = owned_namespace_effect(&leaf);
            let wrapped = Decl::Pub(Box::new(leaf));
            let actual = owned_namespace_effect(&wrapped);
            assert_eq!(actual, expected, "Pub changed the complete {label} effect");
        }
    }
}
