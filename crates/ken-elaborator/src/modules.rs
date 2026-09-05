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
    BoundaryHeader, CtorDecl, Decl, ExplicitDataCtor, ExportForm, ImportItem, ImportKind, Type,
};
use crate::error::{ElabError, Span};
use crate::resolve::{
    self, RCtorDecl, RDecl, RDeclKind, RExplicitCtorDecl, RExpr, RMatchArm, RPatKind, RPattern,
    RPropIntro, RTelescopeEntry, RType,
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
        Ok(())
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
    /// Alias prefixes from `import M as N` — `N` resolves to `M` when used
    /// as a qualifying prefix (`N.foo`).
    prefixes: HashMap<String, String>,
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

    fn bind_import(&mut self, bare: &str, qualified: &str, span: &Span) -> Result<(), ElabError> {
        if self.locals.contains(bare) {
            let local = self
                .bindings
                .get(bare)
                .cloned()
                .unwrap_or_else(|| bare.to_string());
            return Err(ElabError::AmbiguousReference {
                name: bare.to_string(),
                sources: vec![local, qualified.to_string()],
                span: span.clone(),
            });
        }
        match self.bindings.get(bare) {
            None => {
                self.bindings
                    .insert(bare.to_string(), qualified.to_string());
            }
            Some(existing) if existing == qualified => {}
            Some(existing) => {
                return Err(ElabError::AmbiguousReference {
                    name: bare.to_string(),
                    sources: vec![existing.clone(), qualified.to_string()],
                    span: span.clone(),
                });
            }
        }
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
    if let Some(dot) = name.rfind('.') {
        let (prefix_part, leaf) = (&name[..dot], &name[dot + 1..]);
        if let Some(q) = scope.bindings.get(prefix_part) {
            return Ok(format!("{q}.{leaf}"));
        }
        let canonical_module = scope
            .prefixes
            .get(prefix_part)
            .cloned()
            .unwrap_or_else(|| prefix_part.to_string());
        if let Some(pubmap) = exports.get(&canonical_module) {
            return pubmap
                .get(leaf)
                .cloned()
                .ok_or_else(|| ElabError::UnboundName {
                    name: name.to_string(),
                    span: span.clone(),
                });
        }
        if prefix_part.contains('.') {
            let resolved_prefix = resolve_ref(scope, exports, prefix_part, span)?;
            return Ok(format!("{resolved_prefix}.{leaf}"));
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
) -> Result<String, ElabError> {
    let subject_is_local = !subject.contains('.') && scope.locals.contains(subject);
    let subject = resolve_ref(scope, exports, subject, span)?;
    if !subject_is_local {
        if let Some(dot) = subject.rfind('.') {
            let (module, leaf) = (&subject[..dot], &subject[dot + 1..]);
            if let Some(pubmap) = exports.get(module) {
                let attached_key = format!("{leaf}::{proof_name}");
                return pubmap
                    .get(&attached_key)
                    .cloned()
                    .ok_or_else(|| ElabError::UnboundName {
                        name: format!("{subject}::{proof_name}"),
                        span: span.clone(),
                    });
            }
        }
    }
    Ok(format!("{subject}::{proof_name}"))
}

fn apply_import(
    scope: &mut Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    prelude_binding_names: &HashSet<String>,
    module: &str,
    kind: &ImportKind,
    span: &Span,
) -> Result<(), ElabError> {
    let pubmap = exports.get(module).ok_or_else(|| ElabError::UnboundName {
        name: module.to_string(),
        span: span.clone(),
    })?;
    match kind {
        ImportKind::Qualified => {
            scope
                .prefixes
                .insert(module.to_string(), module.to_string());
        }
        ImportKind::Aliased(alias) => {
            scope.prefixes.insert(alias.clone(), module.to_string());
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
                if prelude_binding_names.contains(bare) {
                    let same_canonical_identity = globals
                        .get(bare)
                        .zip(globals.get(q))
                        .is_some_and(|(installed, incoming)| installed == incoming);
                    if !same_canonical_identity {
                        return Err(ElabError::AmbiguousReference {
                            name: bare.to_string(),
                            sources: vec![format!("<prelude>.{bare}"), q.clone()],
                            span: span.clone(),
                        });
                    }
                }
                scope.bind_import(bare, q, span)?;
            }
        }
    }
    Ok(())
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

fn published_name(item: &ImportItem) -> &str {
    item.rename.as_deref().unwrap_or(&item.name)
}

fn apply_export(
    scope: &mut Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    exports_here: &mut HashMap<String, String>,
    form: &ExportForm,
    span: &Span,
) -> Result<(), ElabError> {
    match form {
        ExportForm::Facade { module, items } => {
            let pubmap = exports.get(module).ok_or_else(|| ElabError::UnboundName {
                name: module.clone(),
                span: span.clone(),
            })?;
            for item in items {
                let canonical = pubmap
                    .get(&item.name)
                    .ok_or_else(|| ElabError::UnboundName {
                        name: format!("{module}.{}", item.name),
                        span: span.clone(),
                    })?;
                let surface = published_name(item);
                publish_identity(exports_here, surface, canonical, span)?;
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
                publish_identity(exports_here, published_name(item), &canonical, span)?;
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

fn imported_module_paths(decls: &[Decl], out: &mut Vec<(String, Span)>) {
    for decl in decls {
        match decl.unwrap_pub() {
            Decl::ImportDecl { module, span, .. } => {
                out.push((module.clone(), span.clone()));
            }
            Decl::ExportDecl {
                form: ExportForm::Facade { module, .. },
                span,
            } => out.push((module.clone(), span.clone())),
            Decl::ModuleDecl { decls: inner, .. } => imported_module_paths(inner, out),
            _ => {}
        }
    }
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
        declared_module_paths(&decls, "", &mut local_modules);
        let mut imports = Vec::new();
        imported_module_paths(&decls, &mut imports);
        for (dependency, import_span) in imports {
            if local_modules.contains(&dependency)
                || elab.module_state.exports.contains_key(&dependency)
            {
                continue;
            }
            load_unit(elab, &dependency, &import_span, mode)?;
        }
        refresh_carried_instance_admission(elab);

        // Every loaded source unit is an ordinary orphan-check module. Assign
        // its id only after dependencies return so their current-module ids do
        // not leak into this unit's declarations.
        elab.class_env.next_module();

        let mut scope = Scope::with_mode(mode, elab.module_state.strict_builtin_names.clone());
        let mut unit_definitions = HashSet::new();
        let (results, exports) = expand_scope(
            elab,
            &decls,
            module,
            &mut scope,
            &mut unit_definitions,
            true,
        )?;
        let ids: Vec<ken_kernel::GlobalId> =
            results.into_iter().map(|result| result.def_id).collect();
        elab.module_state
            .exports
            .insert(module.to_string(), exports);
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
            let n = resolve_ref(scope, exports, &name, &span)?;
            RType::RCon(n, span)
        }
        RType::RVarTy(i, n, s) => RType::RVarTy(i, n, s),
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

fn rewrite_rexpr_inner(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    e: RExpr,
    kernel_head: Option<&'static str>,
) -> Result<RExpr, ElabError> {
    Ok(match e {
        RExpr::RCon(name, span) if kernel_head == Some(name.as_str()) => RExpr::RCon(name, span),
        RExpr::RCon(name, span) => {
            let n = resolve_ref(scope, exports, &name, &span)?;
            RExpr::RCon(n, span)
        }
        RExpr::RVar(i, n, s) => RExpr::RVar(i, n, s),
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
        RExpr::RApp(f, a, s) => RExpr::RApp(
            Box::new(rewrite_rexpr_inner(scope, exports, *f, kernel_head)?),
            Box::new(rewrite_rexpr(scope, exports, *a)?),
            s,
        ),
        RExpr::RLam(n, b, s) => RExpr::RLam(n, Box::new(rewrite_rexpr(scope, exports, *b)?), s),
        RExpr::RLet(x, ty, rhs, body, s) => RExpr::RLet(
            x,
            ty.map(|t| rewrite_rtype(scope, exports, t)).transpose()?,
            Box::new(rewrite_rexpr(scope, exports, *rhs)?),
            Box::new(rewrite_rexpr(scope, exports, *body)?),
            s,
        ),
        RExpr::RAsc(e, t, s) => RExpr::RAsc(
            Box::new(rewrite_rexpr(scope, exports, *e)?),
            Box::new(rewrite_rtype(scope, exports, *t)?),
            s,
        ),
        RExpr::ROld(e, s) => RExpr::ROld(Box::new(rewrite_rexpr(scope, exports, *e)?), s),
        RExpr::RCell(index, name, span) => RExpr::RCell(index, name, span),
        RExpr::RBecomes(index, name, value, span) => RExpr::RBecomes(
            index,
            name,
            Box::new(rewrite_rexpr(scope, exports, *value)?),
            span,
        ),
        RExpr::RNumLit(l, s) => RExpr::RNumLit(l, s),
        RExpr::RStr(v, s) => RExpr::RStr(v, s),
        RExpr::RCharLit(c, s) => RExpr::RCharLit(c, s),
        RExpr::RByteStr(v, s) => RExpr::RByteStr(v, s),
        RExpr::RBinOp(op, l, r, s) => RExpr::RBinOp(
            op,
            Box::new(rewrite_rexpr(scope, exports, *l)?),
            Box::new(rewrite_rexpr(scope, exports, *r)?),
            s,
        ),
        RExpr::RMatch {
            scrut,
            equation,
            arms,
            span,
        } => {
            let scrut = Box::new(rewrite_rexpr(scope, exports, *scrut)?);
            let arms = arms
                .into_iter()
                .map(|a| {
                    Ok(RMatchArm {
                        pat: rewrite_rpattern(scope, exports, a.pat)?,
                        body: rewrite_rexpr(scope, exports, a.body)?,
                        span: a.span,
                    })
                })
                .collect::<Result<Vec<_>, ElabError>>()?;
            RExpr::RMatch {
                scrut,
                equation,
                arms,
                span,
            }
        }
        RExpr::RIf {
            condition,
            then_branch,
            else_branch,
            span,
        } => RExpr::RIf {
            condition: Box::new(rewrite_rexpr(scope, exports, *condition)?),
            then_branch: Box::new(rewrite_rexpr(scope, exports, *then_branch)?),
            else_branch: Box::new(rewrite_rexpr(scope, exports, *else_branch)?),
            span,
        },
        RExpr::RPair(components, span) => RExpr::RPair(
            components
                .into_iter()
                .map(|component| rewrite_rexpr(scope, exports, component))
                .collect::<Result<Vec<_>, _>>()?,
            span,
        ),
        RExpr::RRecord { base, fields, span } => RExpr::RRecord {
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
        },
        RExpr::RPosProj(e, index, span) => {
            RExpr::RPosProj(Box::new(rewrite_rexpr(scope, exports, *e)?), index, span)
        }
        RExpr::RProj(e, field, s) => {
            RExpr::RProj(Box::new(rewrite_rexpr(scope, exports, *e)?), field, s)
        }
        RExpr::RPi(x, a, b, s) => RExpr::RPi(
            x,
            Box::new(rewrite_rtype(scope, exports, *a)?),
            Box::new(rewrite_rexpr(scope, exports, *b)?),
            s,
        ),
        RExpr::RArrow(a, b, s) => RExpr::RArrow(
            Box::new(rewrite_rexpr(scope, exports, *a)?),
            Box::new(rewrite_rexpr(scope, exports, *b)?),
            s,
        ),
        RExpr::RAttachedProofRef {
            subject,
            proof_name,
            span,
        } => RExpr::RCon(
            resolve_attached_ref(scope, exports, &subject, &proof_name, &span)?,
            span,
        ),
        RExpr::RTrunc(e, s) => RExpr::RTrunc(Box::new(rewrite_rexpr(scope, exports, *e)?), s),
    })
}

fn rewrite_rpattern(
    scope: &Scope,
    exports: &HashMap<String, HashMap<String, String>>,
    p: RPattern,
) -> Result<RPattern, ElabError> {
    let kind = match p.kind {
        RPatKind::Wild => RPatKind::Wild,
        RPatKind::Var(n) => RPatKind::Var(n),
        RPatKind::Ctor(name, subs) => {
            let n = resolve_ref(scope, exports, &name, &p.span)?;
            let subs = subs
                .into_iter()
                .map(|s| rewrite_rpattern(scope, exports, s))
                .collect::<Result<Vec<_>, ElabError>>()?;
            RPatKind::Ctor(n, subs)
        }
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
    }
    if let Some(fb) = &result.foreign_binding {
        elab.foreign_env.register(result.name.clone(), fb.clone());
        if !fb.effect_row.is_empty() {
            elab.effect_rows.insert(
                result.name.clone(),
                crate::effects::RowType::Concrete(fb.effect_row.clone()),
            );
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
) -> Result<crate::elab::ElabResult, ElabError> {
    crate::elab::check_surface_purity(rdecl, &elab.effect_rows, &elab.globals, &elab.class_env)?;
    let result = crate::elab::elaborate_rdecl_v1_with_effect_rows(
        &mut elab.env,
        &mut elab.globals,
        &mut elab.num_values,
        &elab.numeric_env,
        &mut elab.class_env,
        &elab.effect_rows,
        rdecl,
    )?;
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
        | Type::TSigma(_, _, _, _) => None,
    }
}

fn canonical_leaf(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SynthesizedDictionaryName {
    surface: String,
    canonical: String,
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
    })
}

fn resolved_named_type_head(ty: &RType) -> Option<&str> {
    match ty {
        RType::RCon(name, _) => Some(name),
        RType::RApp(function, _, _) | RType::RRefine(_, function, _, _) => {
            resolved_named_type_head(function)
        }
        RType::RVarTy(_, _, _)
        | RType::RUniv(_, _)
        | RType::RArr(_, _, _)
        | RType::REffectArr(_, _, _, _)
        | RType::RPi(_, _, _, _)
        | RType::RSigma(_, _, _, _) => None,
    }
}

fn resolved_synthesized_dictionary_name(rdecl: &RDecl) -> Option<SynthesizedDictionaryName> {
    let resolved_head = match &rdecl.kind {
        RDeclKind::InstanceDecl { head_type, .. } => resolved_named_type_head(head_type)?,
        RDeclKind::DeriveDecl { data_name } => data_name,
        _ => return None,
    };
    Some(SynthesizedDictionaryName {
        surface: format!(
            "{}_instance_{}",
            canonical_leaf(&rdecl.name),
            canonical_leaf(resolved_head)
        ),
        canonical: format!("{}_instance_{resolved_head}", rdecl.name),
    })
}

fn synthesized_dictionary_class_is_imported(decl: &Decl, rdecl: &RDecl, scope: &Scope) -> bool {
    let class_name = match decl {
        Decl::InstanceDecl { class_name, .. } | Decl::DeriveDecl { class_name, .. } => class_name,
        _ => return false,
    };
    if class_name.contains('.') {
        // Dotted class references resolve only through a loaded module's public
        // export table (qualified directly or through an alias prefix).
        return true;
    }
    !scope.locals.contains(class_name)
        && scope
            .bindings
            .get(class_name)
            .is_some_and(|canonical| canonical == &rdecl.name)
}

#[derive(Clone, Debug)]
struct PendingSynthesizedDictionary {
    name: SynthesizedDictionaryName,
    class_canonical: String,
    class_is_imported: bool,
    span: Span,
}

fn assert_synthesis_installed_identity(
    globals: &HashMap<String, ken_kernel::GlobalId>,
    result: &crate::elab::ElabResult,
    name: &SynthesizedDictionaryName,
    span: &Span,
) -> Result<(), ElabError> {
    let installed = globals.get(&name.canonical).copied();
    if installed == Some(result.def_id) {
        return Ok(());
    }
    Err(ElabError::Internal(format!(
        "synthesized dictionary `{}` did not preserve its installed canonical `{}` identity at {}..{}: installed={installed:?}, result={:?}",
        name.surface, name.canonical, span.start, span.end, result.def_id
    )))
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
        Decl::BoundaryDecl { .. } | Decl::ModuleDecl { .. } => DeclNamespaceEffect::NoBinding,
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
    exports: &HashMap<String, HashMap<String, String>>,
    globals: &HashMap<String, ken_kernel::GlobalId>,
    prelude_binding_names: &HashSet<String>,
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
        if matches!(inner, Decl::AttachedProofDecl { .. }) {
            continue;
        }
        let bare = inner.name().to_string();
        let qualified = if unqualified_local {
            bare.clone()
        } else {
            qualify(prefix, &bare)
        };
        scope.bind_local(&bare, &qualified, inner.span())?;
        match inner {
            Decl::DataDecl { ctors, .. } => {
                for ctor in ctors {
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
                    let qualified = qualify(prefix, name);
                    scope.bind_local(name, &qualified, span)?;
                }
            }
            _ => {}
        }
    }

    // Instance and derive declarations reference a class, but also produce one
    // ordinary global dictionary. A scope with no producer needs no import
    // replay; in particular, its same-file child modules are not expanded yet.
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
                    globals,
                    prelude_binding_names,
                    module,
                    kind,
                    span,
                ) {
                    match error {
                        // A same-file module is expanded only by the real
                        // ordered pass. Its synthesized aliases are installed
                        // just in time there; every other import error is also
                        // reproduced by that authoritative pass.
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
            }
        }
    }
    Ok(())
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

    prebind_scope_declarations(
        scope,
        decls,
        prefix,
        &elab.module_state.exports,
        &elab.globals,
        &elab.module_state.prelude_binding_names,
    )?;

    let mut ids = Vec::new();
    let mut exports_here: HashMap<String, String> = HashMap::new();
    let mut synthesized_dictionaries = Vec::new();
    let mut i = 0;
    while i < decls.len() {
        let decl = &decls[i];
        match decl {
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
                    &elab.globals,
                    &elab.module_state.prelude_binding_names,
                    module,
                    kind,
                    span,
                )?;
                i += 1;
            }
            Decl::ExportDecl { form, span } => {
                apply_export(
                    scope,
                    &elab.module_state.exports,
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
                    &mut child_scope,
                    unit_definitions,
                    false,
                )?;
                ids.extend(child_ids);
                elab.module_state
                    .exports
                    .insert(child_prefix, child_exports);
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
                ids.extend(crate::elab::elaborate_space_decl(elab, &resolved)?);
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
                    while e < decls.len() && is_recursive_candidate(decls[e].unwrap_pub()) {
                        e += 1;
                    }
                    e
                };
                let run = &decls[i..run_end];

                // Resolve + rewrite every run member up front — safe because
                // a run contains no import/module, so `scope`/`exports`
                // don't change across it; each member sees exactly the
                // state it would have seen processed alone at its position.
                let mut bare_names: Vec<String> = Vec::with_capacity(run.len());
                let mut rdecls: Vec<crate::resolve::RDecl> = Vec::with_capacity(run.len());
                for d in run {
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
                        let result = elaborate_checked(elab, rdecl)?;
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
                                &elab.globals,
                                &elab.class_env,
                            )?;
                        }
                        let results = crate::elab::elaborate_mutual_group(
                            &mut elab.env,
                            &mut elab.globals,
                            &mut elab.num_values,
                            &elab.numeric_env,
                            &elab.class_env,
                            &members,
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
                for (d, rdecl) in run.iter().zip(&rdecls) {
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
                            || run.iter().any(|candidate| {
                                candidate.is_pub() && candidate.unwrap_pub().name() == subject
                            });
                        if !subject_is_public {
                            return Err(ElabError::UnboundName {
                                name: subject.clone(),
                                span: inner.span().clone(),
                            });
                        }
                        publish_identity(
                            &mut exports_here,
                            &format!("{subject}::{proof_name}"),
                            &rdecl.name,
                            inner.span(),
                        )?;
                    } else {
                        publish_identity(
                            &mut exports_here,
                            inner.name(),
                            &rdecl.name,
                            inner.span(),
                        )?;
                    }
                }
                i = run_end;
            }
            other => {
                let is_pub = other.is_pub();
                let inner = other.unwrap_pub();
                if is_pub && !prefix.is_empty() {
                    if let Decl::DataDecl { name, span, .. } = inner {
                        // Abstract export (`33 §4.2`) applies only INSIDE a
                        // real `module { … }` (`prefix` non-empty) — there
                        // is no "outside" to hide from at the true file
                        // root (`prefix == ""`), exactly as a root-level
                        // `pub` on `View`/`Let`/`TypeAlias` is already
                        // inert there (its `exports_here` entry is
                        // produced but discarded by `expand_and_elaborate`
                        // as `_root_exports`). A `pub data T = MkT` at the
                        // top level must fall through to ordinary `data`
                        // elaboration below — `MkT` stays a real,
                        // constructible/matchable constructor, not a
                        // silently-stripped opaque constant with no
                        // client to protect.
                        //
                        // A `pub data T = …` exports the type name only —
                        // constructors are never `pub`-able in this
                        // surface, so the whole ctor set is always
                        // withheld. Rather than a real `Decl::Inductive`
                        // with hidden-but-present ctors, declare `T` as
                        // the kernel's EXISTING opaque constant (`11 §4`)
                        // directly: byte-identical to a hand-written
                        // `T : Type` postulate, no new `Decl` variant, no
                        // kernel "abstract" flag. The constructors are
                        // simply never registered anywhere (not in
                        // `globals`, not in any export table) —
                        // unconstructible and unmatchable, by every
                        // observer, kernel included.
                        let qualified = qualify(prefix, name);
                        resolve::check_no_definition_collision(
                            &qualified,
                            &qualified,
                            span,
                            Some(unit_definitions),
                        )?;
                        let ty = ken_kernel::Term::ty(ken_kernel::Level::Zero);
                        let id = ken_kernel::declare_postulate(
                            &mut elab.env,
                            qualified.clone(),
                            vec![],
                            ty,
                        )
                        .map_err(|e| ElabError::KernelRejected {
                            error: e,
                            span: span.clone(),
                        })?;
                        elab.globals.insert(qualified.clone(), id);
                        publish_identity(&mut exports_here, name, &qualified, span)?;
                        ids.push(crate::elab::ElabResult {
                            name: qualified,
                            def_id: id,
                            obligations: vec![],
                            foreign_binding: None,
                            temporal_obligations: vec![],
                            effect_row_type: None,
                        });
                        i += 1;
                        continue;
                    }
                }
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
                    let result = elaborate_checked(elab, &rdecl)?;
                    if is_pub {
                        if let Decl::AttachedProofDecl {
                            subject,
                            proof_name,
                            ..
                        } = inner
                        {
                            publish_identity(
                                &mut exports_here,
                                &format!("{subject}::{proof_name}"),
                                &result.name,
                                inner.span(),
                            )?;
                        } else {
                            // Only the decl's own qualified name is exported —
                            // never a `DataDecl`'s constructors (`33 §4.2`,
                            // abstract export: ctors are simply never entered
                            // into any export table, so a client can't bring
                            // them into scope by any import form).
                            publish_identity(&mut exports_here, &bare, &result.name, inner.span())?;
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
                    if let Some(name) = resolved_synthesized_dictionary_name(&rdecl) {
                        reject_prelude_binding(
                            &name.surface,
                            &name.canonical,
                            inner.span(),
                            &elab.module_state.prelude_binding_names,
                        )?;
                        scope.bind_local(&name.surface, &name.canonical, inner.span())?;
                    }
                    let result = elaborate_checked(elab, &rdecl)?;
                    if is_pub && matches!(inner, Decl::ClassDecl { .. }) {
                        publish_identity(
                            &mut exports_here,
                            inner.name(),
                            &result.name,
                            inner.span(),
                        )?;
                    }
                    if let Some(name) = resolved_synthesized_dictionary_name(&rdecl) {
                        assert_synthesis_installed_identity(
                            &elab.globals,
                            &result,
                            &name,
                            inner.span(),
                        )?;
                        synthesized_dictionaries.push(PendingSynthesizedDictionary {
                            name,
                            class_canonical: rdecl.name.clone(),
                            class_is_imported: synthesized_dictionary_class_is_imported(
                                inner, &rdecl, scope,
                            ),
                            span: inner.span().clone(),
                        });
                    }
                    ids.push(result);
                }
                i += 1;
            }
        }
    }

    // `export C` has the same interface effect as `pub class C`, and may occur
    // after an instance. Decide inherited dictionary visibility only after the
    // complete owner surface is known so the WIRE-derived export is independent
    // of declaration order. An imported class is already public by construction.
    for pending in synthesized_dictionaries {
        let class_is_exported = exports_here
            .values()
            .any(|canonical| canonical == &pending.class_canonical);
        if pending.class_is_imported || class_is_exported {
            publish_identity(
                &mut exports_here,
                &pending.name.surface,
                &pending.name.canonical,
                &pending.span,
            )?;
        }
    }
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
    let expanded = expand_scope(elab, decls, "", &mut scope, &mut unit_definitions, true);
    if direct_call {
        elab.class_env.current_package = previous_package;
        elab.class_env.direct_use_packages = previous_direct_use;
        elab.class_env.direct_use_instances = previous_direct_instances;
        elab.class_env.implicit_single_provider = previous_implicit_single_provider;
    }
    let (results, _root_exports) = expanded?;
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
    use super::{decl_namespace_effect, ConstructorNameSource, DeclNamespaceEffect};
    use crate::ast::{Decl, ExplicitDataCtor};
    use crate::error::Span;
    use crate::parser::parse_decls;

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
