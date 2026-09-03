//! LANG-MOD-CATALOG-COMPLETENESS RECUT #3 evidence-frontier census.
//!
//! This is an executable transition report, not resolver input and not a
//! catalog-completeness claim. It observes the unchanged real loaders and
//! records exactly where legacy identity evidence is unavailable.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use ken_elaborator::modules::{
    catalog_module_from_path, PRELUDE_COMPANION_BINDING_NAMES, PRELUDE_FLOOR_NAMES,
};
use ken_elaborator::{literate, parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, Span};
use ken_kernel::{
    ConstructorDecl, Decl, GlobalId, InductiveDecl, KernelError, Level, ParameterPolarity,
    PrimReduction, Term,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PipelineStage {
    Parsing,
    Resolution,
    Elaboration,
    Kernel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LoaderObservation {
    Succeeded,
    Refused { stage: PipelineStage },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum IdentityPrerequisite {
    CanonicalHome {
        identity: GlobalId,
        current_names: Vec<String>,
    },
    ProviderInterfaceAndImport {
        identity: GlobalId,
        defining_module: String,
        provider_interface_available: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LegacyIdentityEvidence {
    ExactLedger {
        resolved_globals: BTreeSet<GlobalId>,
        identity_grounded_prerequisites: BTreeSet<IdentityPrerequisite>,
    },
    Unavailable {
        failing_stage: PipelineStage,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UnitCensusRow {
    canonical_module: String,
    parsed_direct_imports: BTreeSet<String>,
    strict_observation: LoaderObservation,
    legacy_observation: LoaderObservation,
    legacy_identity_evidence: LegacyIdentityEvidence,
}

#[derive(Clone)]
struct DiscoveredUnit {
    path: PathBuf,
    direct_imports: BTreeSet<String>,
    selective_import_bindings: BTreeMap<String, String>,
    public_targets: BTreeSet<String>,
}

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn parse_source(path: &Path) -> Vec<SurfaceDecl> {
    let source = fs::read_to_string(path).expect("catalog source must be readable UTF-8");
    if path.to_string_lossy().ends_with(".ken.md") {
        let extracted = literate::extract_ken_md(&source).expect("literate extraction");
        literate::validate_ken_md_fences(&extracted).expect("valid literate fences");
        parser::parse_decls(&extracted.source).expect("catalog Ken must parse")
    } else {
        parser::parse_decls(&source).expect("catalog Ken must parse")
    }
}

fn collect_imports(
    decls: &[SurfaceDecl],
    out: &mut BTreeSet<String>,
    selective_bindings: &mut BTreeMap<String, String>,
) {
    for decl in decls {
        match decl.unwrap_pub() {
            SurfaceDecl::ImportDecl { module, kind, .. } => {
                out.insert(module.clone());
                if let ken_elaborator::ImportKind::Selective(items) = kind {
                    for item in items {
                        selective_bindings.insert(
                            item.rename.clone().unwrap_or_else(|| item.name.clone()),
                            module.clone(),
                        );
                    }
                }
            }
            SurfaceDecl::ExportDecl {
                form: ExportForm::Facade { module, .. },
                ..
            } => {
                out.insert(module.clone());
            }
            SurfaceDecl::ModuleDecl { decls, .. } => {
                collect_imports(decls, out, selective_bindings)
            }
            _ => {}
        }
    }
}

fn collect_public_targets(
    module: &str,
    decls: &[SurfaceDecl],
    selective_scope: &mut BTreeMap<String, String>,
    out: &mut BTreeSet<String>,
) {
    for decl in decls {
        match decl {
            SurfaceDecl::Pub(inner) => {
                let name = inner.name();
                if !name.is_empty() {
                    let target = format!("{module}.{name}");
                    selective_scope.insert(name.to_string(), target.clone());
                    out.insert(target);
                }
            }
            SurfaceDecl::ImportDecl {
                module: provider,
                kind: ken_elaborator::ImportKind::Selective(items),
                ..
            } => {
                for item in items {
                    selective_scope.insert(
                        item.rename.clone().unwrap_or_else(|| item.name.clone()),
                        format!("{provider}.{}", item.name),
                    );
                }
            }
            SurfaceDecl::ExportDecl { form, .. } => match form {
                ExportForm::Facade {
                    module: provider,
                    items,
                } => {
                    for item in items {
                        out.insert(format!("{provider}.{}", item.name));
                    }
                }
                ExportForm::InScope { items } => {
                    for item in items {
                        let target = selective_scope
                            .get(&item.name)
                            .cloned()
                            .unwrap_or_else(|| format!("{module}.{}", item.name));
                        out.insert(target);
                    }
                }
            },
            SurfaceDecl::ModuleDecl { .. } => {}
            other => {
                let name = other.name();
                if !name.is_empty() {
                    selective_scope
                        .entry(name.to_string())
                        .or_insert_with(|| format!("{module}.{name}"));
                }
            }
        }
    }
}

fn discover_units(root: &Path) -> BTreeMap<String, DiscoveredUnit> {
    fn walk(root: &Path, dir: &Path, units: &mut BTreeMap<String, DiscoveredUnit>) {
        for entry in fs::read_dir(dir).expect("read populated catalog directory") {
            let path = entry.expect("read catalog entry").path();
            if path.is_dir() {
                walk(root, &path, units);
                continue;
            }
            if !path.to_string_lossy().ends_with(".ken")
                && !path.to_string_lossy().ends_with(".ken.md")
            {
                continue;
            }
            let canonical = catalog_module_from_path(&path)
                .expect("every discovered source must have a canonical module path");
            assert_eq!(
                canonical.root, root,
                "discovery and loader root must coincide"
            );
            let decls = parse_source(&path);
            let mut direct_imports = BTreeSet::new();
            let mut selective_import_bindings = BTreeMap::new();
            collect_imports(&decls, &mut direct_imports, &mut selective_import_bindings);
            let mut public_targets = BTreeSet::new();
            collect_public_targets(
                &canonical.entry,
                &decls,
                &mut BTreeMap::new(),
                &mut public_targets,
            );
            let previous = units.insert(
                canonical.entry,
                DiscoveredUnit {
                    path,
                    direct_imports,
                    selective_import_bindings,
                    public_targets,
                },
            );
            assert!(
                previous.is_none(),
                "canonical module identity must be unique"
            );
        }
    }

    let mut units = BTreeMap::new();
    walk(root, root, &mut units);
    units
}

fn failure_stage(error: &ElabError) -> Option<PipelineStage> {
    match error {
        ElabError::ParseError { .. }
        | ElabError::NonAsciiIdentifierCharacter { .. }
        | ElabError::InvalidEscape { .. }
        | ElabError::ForeignNameControlCharacter { .. } => Some(PipelineStage::Parsing),
        ElabError::UnboundName { .. }
        | ElabError::UnresolvedCon { .. }
        | ElabError::DuplicateDefinition { .. }
        | ElabError::ImportCycle { .. }
        | ElabError::AmbiguousReference { .. } => Some(PipelineStage::Resolution),
        ElabError::KernelRejected { .. } => Some(PipelineStage::Kernel),
        ElabError::TypeMismatch { .. }
        | ElabError::LambdaVsNonFunction { .. }
        | ElabError::NotAFunction { .. }
        | ElabError::LevelConflict { .. } => Some(PipelineStage::Elaboration),
        _ => None,
    }
}

fn observe_strict(root: &Path, module: &str) -> LoaderObservation {
    let mut env = ElabEnv::new().expect("base environment");
    match env.elaborate_module_from_roots_strict(&[root.to_path_buf()], module) {
        Ok(_) => LoaderObservation::Succeeded,
        Err(error) => LoaderObservation::Refused {
            stage: failure_stage(&error)
                .unwrap_or_else(|| panic!("strict refusal has unrecorded typed stage: {error:?}")),
        },
    }
}

fn observe_legacy(root: &Path, module: &str) -> LoaderObservation {
    let mut env = ElabEnv::new().expect("base environment");
    match env.elaborate_module_from_roots(&[root.to_path_buf()], module) {
        Ok(_) => LoaderObservation::Succeeded,
        Err(error) => LoaderObservation::Refused {
            stage: failure_stage(&error)
                .unwrap_or_else(|| panic!("legacy refusal has unrecorded typed stage: {error:?}")),
        },
    }
}

fn collect_term_globals(term: &Term, out: &mut BTreeSet<GlobalId>) {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        match term {
            Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
                out.insert(*id);
            }
            Term::Elim { fam, .. } => {
                out.insert(*fam);
            }
            _ => {}
        }
        pending.extend(term.children());
    }
}

fn collect_decl_globals(decl: &Decl, out: &mut BTreeSet<GlobalId>) {
    match decl {
        Decl::Transparent { ty, body, .. } => {
            collect_term_globals(ty, out);
            collect_term_globals(body, out);
        }
        Decl::Opaque { ty, .. } => {
            collect_term_globals(ty, out);
        }
        Decl::Primitive { ty, .. } => {
            collect_term_globals(ty, out);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_term_globals(term, out);
            }
            for term in &inductive.indices {
                collect_term_globals(term, out);
            }
            collect_term_globals(&inductive.former_type, out);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_term_globals(term, out);
                }
                for term in &constructor.target_indices {
                    collect_term_globals(term, out);
                }
                collect_term_globals(&constructor.type_, out);
            }
        }
    }
}

fn names_by_id(env: &ElabEnv) -> BTreeMap<GlobalId, Vec<String>> {
    let mut names = BTreeMap::<GlobalId, Vec<String>>::new();
    for (name, id) in &env.globals {
        names.entry(*id).or_default().push(name.clone());
    }
    for values in names.values_mut() {
        values.sort();
        values.dedup();
    }
    names
}

fn provider_interface_available(unit: &DiscoveredUnit, identity_names: &[String]) -> bool {
    unit.public_targets
        .iter()
        .any(|target| identity_names.contains(target))
}

fn exact_public_target_identity(env: &ElabEnv, target: &str) -> Option<GlobalId> {
    env.globals.get(target).copied().or_else(|| {
        let surface_name = target.rsplit('.').next()?;
        env.globals.get(surface_name).copied()
    })
}

fn defining_module(name: &str, modules_longest_first: &[String]) -> Option<String> {
    modules_longest_first
        .iter()
        .find(|module| name.starts_with(&format!("{module}.")))
        .cloned()
}

fn strict_available_ids(base: &ElabEnv) -> BTreeSet<GlobalId> {
    let trusted: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    let floor_formers: BTreeSet<_> = PRELUDE_FLOOR_NAMES
        .iter()
        .filter_map(|name| base.globals.get(*name).copied())
        .collect();
    let mut available = trusted;
    available.extend(floor_formers.iter().copied());
    available.extend(PRELUDE_COMPANION_BINDING_NAMES.map(|name| base.globals[name]));
    for id in base.globals.values().copied() {
        if base
            .env
            .constructor(id)
            .is_some_and(|(parent, _)| floor_formers.contains(&parent.id))
        {
            available.insert(id);
        }
    }
    available
}

#[derive(Debug)]
struct CompilerInternalBaseSupportAuthority {
    support_ids: BTreeSet<GlobalId>,
    named_ids: BTreeSet<GlobalId>,
}

impl CompilerInternalBaseSupportAuthority {
    fn from_base(base: &ElabEnv, base_names: &BTreeMap<GlobalId, Vec<String>>) -> Self {
        Self {
            support_ids: base
                .env
                .declarations()
                .iter()
                .map(Decl::id)
                .filter(|identity| !base_names.contains_key(identity))
                .collect(),
            named_ids: base_names.keys().copied().collect(),
        }
    }

    fn contains(&self, identity: GlobalId) -> bool {
        self.support_ids.contains(&identity)
    }

    fn exact_ids(&self) -> &BTreeSet<GlobalId> {
        &self.support_ids
    }
}

fn legacy_evidence(
    root: &Path,
    module: &str,
    unit: &DiscoveredUnit,
    all_units: &BTreeMap<String, DiscoveredUnit>,
) -> LegacyIdentityEvidence {
    let mut env = ElabEnv::new().expect("base environment");
    let strict_available = strict_available_ids(&env);
    let base_names = names_by_id(&env);
    let base_support = CompilerInternalBaseSupportAuthority::from_base(&env, &base_names);
    let entry_ids = match env.elaborate_module_from_roots(&[root.to_path_buf()], module) {
        Ok(ids) => ids,
        Err(error) => {
            return LegacyIdentityEvidence::Unavailable {
                failing_stage: failure_stage(&error).unwrap_or_else(|| {
                    panic!("legacy failure has unrecorded typed stage for {module}: {error:?}")
                }),
            };
        }
    };

    let names = names_by_id(&env);
    let entry_start = entry_ids.iter().map(|id| id.0).min();
    let mut owned = BTreeSet::from_iter(entry_ids.iter().copied());
    for (id, identity_names) in &names {
        if identity_names
            .iter()
            .any(|name| name.starts_with(&format!("{module}.")))
        {
            owned.insert(*id);
        }
    }
    for declaration in env.env.declarations() {
        if let Decl::Opaque { id, name, .. } = declaration {
            if name.starts_with(&format!("{module}.")) {
                owned.insert(*id);
            }
        }
    }

    let mut resolved = BTreeSet::new();
    if entry_ids.is_empty() {
        for target in &unit.public_targets {
            resolved.insert(
                exact_public_target_identity(&env, target).unwrap_or_else(|| {
                    panic!("successful legacy unit {module} has no exact public target {target}")
                }),
            );
        }
    }
    if let Some(entry_start) = entry_start {
        for declaration in env
            .env
            .declarations()
            .iter()
            .filter(|declaration| declaration.id().0 >= entry_start)
        {
            collect_decl_globals(declaration, &mut resolved);
        }
    }

    let mut modules_longest_first: Vec<_> = all_units.keys().cloned().collect();
    modules_longest_first.sort_by_key(|name| std::cmp::Reverse(name.len()));
    let mut prerequisites = BTreeSet::new();

    for identity in resolved.iter().copied() {
        // Dependencies finish before the root unit starts, and Sigma is
        // append-only. The root's first returned declaration therefore bounds
        // every later helper/proof identity owned by that root. Earlier private
        // `axiom` identities and generated opaque law fields are instead owned
        // by their kernel audit label even when absent from the surface globals
        // map.
        if owned.contains(&identity)
            || entry_start.is_some_and(|start| identity.0 >= start)
            || strict_available.contains(&identity)
            || base_support.contains(identity)
            || matches!(
                env.env.lookup(identity),
                Some(Decl::Primitive {
                    reduction: PrimReduction::Literal,
                    ..
                })
            )
        {
            continue;
        }
        let identity_names = names.get(&identity).cloned().unwrap_or_default();
        if identity_names.iter().any(|name| {
            unit.selective_import_bindings
                .get(name)
                .is_some_and(|provider| unit.direct_imports.contains(provider))
        }) {
            continue;
        }
        let provider = identity_names
            .iter()
            .filter_map(|name| defining_module(name, &modules_longest_first))
            .next();
        if let Some(provider) = provider {
            if unit.direct_imports.contains(&provider) {
                continue;
            }
            let provider_interface_available = provider_interface_available(
                &all_units[&provider],
                names.get(&identity).map(Vec::as_slice).unwrap_or(&[]),
            );
            prerequisites.insert(IdentityPrerequisite::ProviderInterfaceAndImport {
                identity,
                defining_module: provider,
                provider_interface_available,
            });
            continue;
        }
        let native_names = base_names.get(&identity).cloned().unwrap_or_default();
        if !native_names.is_empty() {
            prerequisites.insert(IdentityPrerequisite::CanonicalHome {
                identity,
                current_names: native_names,
            });
            continue;
        }
        panic!(
            "ExactLedger row {module} contains unclassified identity {identity:?} with names {identity_names:?}, declaration {:?}",
            env.env.lookup(identity)
        );
    }

    LegacyIdentityEvidence::ExactLedger {
        resolved_globals: resolved,
        identity_grounded_prerequisites: prerequisites,
    }
}

fn build_census() -> (BTreeMap<String, DiscoveredUnit>, Vec<UnitCensusRow>) {
    let root = catalog_root();
    let units = discover_units(&root);
    let strict_observations: BTreeMap<_, _> = units
        .keys()
        .map(|module| (module.clone(), observe_strict(&root, module)))
        .collect();
    let legacy_observations: BTreeMap<_, _> = units
        .keys()
        .map(|module| (module.clone(), observe_legacy(&root, module)))
        .collect();
    let mut rows = Vec::with_capacity(units.len());
    for (module, unit) in &units {
        let evidence = legacy_evidence(&root, module, unit, &units);
        rows.push(UnitCensusRow {
            canonical_module: module.clone(),
            parsed_direct_imports: unit.direct_imports.clone(),
            strict_observation: strict_observations[module].clone(),
            legacy_observation: legacy_observations[module].clone(),
            legacy_identity_evidence: evidence,
        });
    }
    (units, rows)
}

fn collect_census() -> &'static (BTreeMap<String, DiscoveredUnit>, Vec<UnitCensusRow>) {
    static CENSUS: OnceLock<(BTreeMap<String, DiscoveredUnit>, Vec<UnitCensusRow>)> =
        OnceLock::new();
    CENSUS.get_or_init(build_census)
}

fn validate_population(
    discovered: &BTreeSet<String>,
    rows: &[UnitCensusRow],
) -> Result<(), String> {
    let row_modules: BTreeSet<_> = rows
        .iter()
        .map(|row| row.canonical_module.clone())
        .collect();
    if row_modules.len() != rows.len() {
        return Err("duplicate canonical census row".to_string());
    }
    if &row_modules != discovered {
        return Err("census population differs from real loader discovery".to_string());
    }
    Ok(())
}

fn validate_evidence_frontier(rows: &[UnitCensusRow]) -> Result<(), String> {
    let strict_by_module: BTreeMap<_, _> = rows
        .iter()
        .map(|row| (row.canonical_module.as_str(), &row.strict_observation))
        .collect();
    for row in rows {
        match (&row.legacy_observation, &row.legacy_identity_evidence) {
            (LoaderObservation::Succeeded, LegacyIdentityEvidence::ExactLedger { .. }) => {}
            (
                LoaderObservation::Refused { stage },
                LegacyIdentityEvidence::Unavailable { failing_stage },
            ) if stage == failing_stage => {}
            _ => {
                return Err(format!(
                    "legacy evidence does not match the independently observed legacy outcome: {}",
                    row.canonical_module
                ));
            }
        }

        let imported_refusal = row.parsed_direct_imports.iter().any(|dependency| {
            matches!(
                strict_by_module.get(dependency.as_str()),
                Some(LoaderObservation::Refused { .. })
            )
        });
        match (&row.strict_observation, &row.legacy_identity_evidence) {
            (
                LoaderObservation::Succeeded,
                LegacyIdentityEvidence::ExactLedger {
                    resolved_globals: _,
                    identity_grounded_prerequisites,
                },
            ) => {
                if !identity_grounded_prerequisites.is_empty() {
                    return Err(format!(
                        "strict success retains an unmet identity prerequisite: {}",
                        row.canonical_module
                    ));
                }
            }
            (
                LoaderObservation::Refused { .. },
                LegacyIdentityEvidence::ExactLedger {
                    resolved_globals: _,
                    identity_grounded_prerequisites,
                },
            ) => {
                if identity_grounded_prerequisites.is_empty() && !imported_refusal {
                    return Err(format!(
                        "strict refusal is unexplained by exact identities or a parsed refused dependency: {}",
                        row.canonical_module
                    ));
                }
            }
            (_, LegacyIdentityEvidence::Unavailable { .. }) => {}
        }
    }
    Ok(())
}

/// Promise class: durable invariant.
///
/// MEASURED: real discovery, parsing, strict roots loading, and legacy roots
/// loading construct one row per canonical unit, with exact core identities only
/// after successful legacy elaboration. CLAIMED: this is the current evidence
/// frontier and nothing more. THE GAP: unavailable rows intentionally name no
/// provider or migration and prevent every completeness claim.
#[test]
fn catalog_evidence_frontier_is_closed_and_honest() {
    let root = catalog_root();
    let before: BTreeMap<_, _> = discover_units(&root)
        .into_iter()
        .map(|(module, unit)| (module, fs::read(&unit.path).expect("source bytes")))
        .collect();
    let (units, rows) = build_census();
    let discovered: BTreeSet<_> = units.keys().cloned().collect();
    validate_population(&discovered, &rows).expect("one row per discovered canonical unit");

    for row in &rows {
        assert_eq!(
            row.parsed_direct_imports, units[&row.canonical_module].direct_imports,
            "row imports must be the parsed loader edges"
        );
        println!("{row:#?}");
    }

    validate_evidence_frontier(&rows)
        .expect("every ExactLedger strict observation must be identity-explained");

    let after: BTreeMap<_, _> = discover_units(&root)
        .into_iter()
        .map(|(module, unit)| (module, fs::read(&unit.path).expect("source bytes")))
        .collect();
    assert_eq!(
        before, after,
        "the census must leave every source byte-stable"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: independent known-answer declarations place distinct identities
/// in Transparent, Opaque, Primitive, and every stored Inductive term field,
/// including nested children and the non-child `Elim.fam`; the collector returns
/// their exact set. CLAIMED: the ExactLedger walk reaches every identity-bearing
/// declaration position rather than only transparent signatures and bodies. THE
/// GAP: declaration ownership is separately established by the real-loader
/// append-order boundary in the census test.
#[test]
fn global_ledger_collector_reaches_every_decl_variant_and_inductive_field() {
    let in_type = GlobalId(10_001);
    let body_former = GlobalId(10_002);
    let body_constructor = GlobalId(10_003);
    let elim_family = GlobalId(10_004);
    let opaque_type = GlobalId(10_005);
    let primitive_type = GlobalId(10_006);
    let inductive_param = GlobalId(10_007);
    let inductive_index = GlobalId(10_008);
    let former_type = GlobalId(10_009);
    let constructor_arg = GlobalId(10_010);
    let target_index = GlobalId(10_011);
    let constructor_type = GlobalId(10_012);
    let declarations = [
        Decl::Transparent {
            id: GlobalId(10_000),
            level_params: vec![],
            ty: Term::const_(in_type, vec![]),
            body: Term::pair(
                Term::indformer(body_former, vec![]),
                Term::pair(
                    Term::constructor(body_constructor, vec![]),
                    Term::Elim {
                        fam: elim_family,
                        level_args: vec![],
                        params: vec![],
                        motive: Box::new(Term::ty(Level::zero())),
                        methods: vec![],
                        indices: vec![],
                        scrut: Box::new(Term::var(0)),
                    },
                ),
            ),
        },
        Decl::Opaque {
            id: GlobalId(10_100),
            name: "fixture opaque".to_string(),
            level_params: vec![],
            ty: Term::const_(opaque_type, vec![]),
        },
        Decl::Primitive {
            id: GlobalId(10_101),
            level_params: vec![],
            ty: Term::const_(primitive_type, vec![]),
            reduction: PrimReduction::Literal,
        },
        Decl::Inductive(InductiveDecl {
            id: GlobalId(10_102),
            level_params: vec![],
            params: vec![Term::const_(inductive_param, vec![])],
            parameter_polarities: vec![ParameterPolarity::StrictlyPositive],
            indices: vec![Term::const_(inductive_index, vec![])],
            level: Level::zero(),
            constructors: vec![ConstructorDecl {
                id: GlobalId(10_103),
                args: vec![Term::const_(constructor_arg, vec![])],
                target_indices: vec![Term::const_(target_index, vec![])],
                type_: Term::const_(constructor_type, vec![]),
                recursive_positions: vec![],
            }],
            former_type: Term::const_(former_type, vec![]),
        }),
    ];
    let mut found = BTreeSet::new();
    for declaration in &declarations {
        collect_decl_globals(declaration, &mut found);
    }
    assert_eq!(
        found,
        BTreeSet::from([
            in_type,
            body_former,
            body_constructor,
            elim_family,
            opaque_type,
            primitive_type,
            inductive_param,
            inductive_index,
            former_type,
            constructor_arg,
            target_index,
            constructor_type,
        ])
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: the population validator compares identities, not counts, and
/// independently rejects an omitted row and a duplicate row. CLAIMED: neither
/// equal cardinality nor row multiplicity can masquerade as population closure.
/// THE GAP: canonical path validity remains production `catalog_module_from_path`'s
/// responsibility and is exercised by the real-discovery test above.
#[test]
fn population_pin_rejects_omission_replacement_and_duplicate() {
    let (_, rows) = collect_census();
    let discovered: BTreeSet<_> = rows
        .iter()
        .map(|row| row.canonical_module.clone())
        .collect();
    assert!(validate_population(&discovered, &rows).is_ok());

    let mut omitted = (*rows).clone();
    omitted.pop();
    assert!(validate_population(&discovered, &omitted).is_err());

    let mut replaced = (*rows).clone();
    replaced[0].canonical_module = "Synthetic.NotDiscovered".to_string();
    assert!(validate_population(&discovered, &replaced).is_err());

    let mut duplicated = (*rows).clone();
    duplicated.push(rows[0].clone());
    assert!(validate_population(&discovered, &duplicated).is_err());
}

/// Promise class: durable invariant.
///
/// MEASURED: a parsed known-answer source separates a public interface target
/// from a private local, and records both an import and a facade-export loader
/// edge while retaining the selective binding's provider. CLAIMED: provider
/// reconciliation is based on parsed interface/edge structure, not a source
/// grep or a matching spelling. THE GAP: identity equality itself is supplied
/// by the successful legacy core and exercised by the real census.
#[test]
fn parsed_provider_interfaces_and_loader_edges_have_known_answer() {
    let provider_decls = parser::parse_decls(
        "const hidden : Bool = True\nconst visible : Bool = True\nexport visible\n",
    )
    .expect("provider fixture parses");
    let mut public_targets = BTreeSet::new();
    collect_public_targets(
        "Fixture.Provider",
        &provider_decls,
        &mut BTreeMap::new(),
        &mut public_targets,
    );
    let provider = DiscoveredUnit {
        path: PathBuf::new(),
        direct_imports: BTreeSet::new(),
        selective_import_bindings: BTreeMap::new(),
        public_targets,
    };
    assert!(provider_interface_available(
        &provider,
        &["Fixture.Provider.visible".to_string()]
    ));
    assert!(!provider_interface_available(
        &provider,
        &["Fixture.Provider.hidden".to_string()]
    ));

    let consumer_decls = parser::parse_decls(
        "import Fixture.Provider (visible as local)\nexport Fixture.Other (thing)\n",
    )
    .expect("consumer fixture parses");
    let mut edges = BTreeSet::new();
    let mut bindings = BTreeMap::new();
    collect_imports(&consumer_decls, &mut edges, &mut bindings);
    assert_eq!(
        edges,
        BTreeSet::from(["Fixture.Other".to_string(), "Fixture.Provider".to_string(),])
    );
    assert_eq!(
        bindings,
        BTreeMap::from([("local".to_string(), "Fixture.Provider".to_string())])
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: a successful export-only facade returns no root declarations, yet
/// its parsed public target resolves to the provider's exact loader identity.
/// CLAIMED: an empty successful root core remains representable as ExactLedger
/// evidence rather than panicking or inventing a declaration threshold. THE GAP:
/// the fixture establishes one exact facade edge, while real-population closure
/// remains the census test's responsibility.
#[test]
fn successful_empty_facade_has_exact_interface_identity_evidence() {
    let nonce = format!(
        "ken-catalog-empty-facade-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    );
    let root = std::env::temp_dir()
        .join(nonce)
        .join("catalog")
        .join("packages");
    fs::create_dir_all(&root).expect("create fixture root");
    fs::write(
        root.join("Provider.ken"),
        "const visible : Bool = True\nexport visible\n",
    )
    .expect("write provider fixture");
    fs::write(root.join("Facade.ken"), "export Provider (visible)\n")
        .expect("write facade fixture");

    assert_eq!(
        observe_strict(&root, "Facade"),
        LoaderObservation::Succeeded
    );
    let units = discover_units(&root);
    let evidence = legacy_evidence(&root, "Facade", &units["Facade"], &units);

    let mut expected_env = ElabEnv::new().expect("base environment");
    let root_ids = expected_env
        .elaborate_module_from_roots(&[root.clone()], "Facade")
        .expect("legacy facade loading succeeds");
    assert!(
        root_ids.is_empty(),
        "the facade fixture must exercise a successful empty root core"
    );
    let provider_identity = expected_env.globals["Provider.visible"];
    assert_eq!(
        evidence,
        LegacyIdentityEvidence::ExactLedger {
            resolved_globals: BTreeSet::from([provider_identity]),
            identity_grounded_prerequisites: BTreeSet::new(),
        }
    );

    let fixture_top = root
        .parent()
        .and_then(Path::parent)
        .expect("fixture has temporary top");
    fs::remove_dir_all(fixture_top).expect("remove fixture tree");
}

/// Promise class: durable invariant.
///
/// MEASURED: the constructed authority equals the complete set of pre-source
/// declaration identities lacking public base names, with no extra members; a
/// foreign identity at the numeric boundary and an unnamed identity numerically
/// between real-looking identities are both absent. CLAIMED: prerequisite
/// classification uses provenance-bearing membership, never numeric allocation
/// position. THE GAP: named strict/floor authority and literal identities are
/// separate explicit classification arms in the real census.
#[test]
fn compiler_internal_base_support_is_an_exact_allowed_inventory() {
    let base = ElabEnv::new().expect("base environment");
    let names = names_by_id(&base);
    let support = CompilerInternalBaseSupportAuthority::from_base(&base, &names);
    let expected: BTreeSet<_> = base
        .env
        .declarations()
        .iter()
        .map(Decl::id)
        .filter(|identity| !names.contains_key(identity))
        .collect();
    assert!(!expected.is_empty(), "fixture needs unnamed base support");
    assert_eq!(
        support.exact_ids(),
        &expected,
        "authority must be exactly the unnamed base-declaration inventory"
    );

    for declaration in base.env.declarations() {
        assert_eq!(
            support.contains(declaration.id()),
            !names.contains_key(&declaration.id()),
            "inventory membership must equal unnamed base-declaration provenance"
        );
    }
    let foreign = GlobalId(base.env.next_global_id().0);
    assert!(base.env.lookup(foreign).is_none());
    assert!(!support.contains(foreign));

    let named = GlobalId(40);
    let inside_interval_nonmember = GlobalId(41);
    let unnamed_support = GlobalId(42);
    let synthetic = CompilerInternalBaseSupportAuthority {
        support_ids: BTreeSet::from([unnamed_support]),
        named_ids: BTreeSet::from([named]),
    };
    assert!(synthetic.contains(unnamed_support));
    assert!(
        inside_interval_nonmember.0 < unnamed_support.0 + 1
            && !synthetic.named_ids.contains(&inside_interval_nonmember)
            && !synthetic.contains(inside_interval_nonmember),
        "an unnamed numeric in-range nonmember must not inherit base-support provenance"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: a known-answer temporary catalog root discovers both supported
/// source spellings and preserves a nested canonical identity while ignoring a
/// non-source file. CLAIMED: the population producer reaches the complete file
/// spelling surface it claims. THE GAP: uniqueness and equality against the real
/// populated root are exercised separately by the census and population pin.
#[test]
fn discovery_known_answer_covers_both_source_spellings() {
    let nonce = format!(
        "ken-catalog-census-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    );
    let root = std::env::temp_dir()
        .join(nonce)
        .join("catalog")
        .join("packages");
    fs::create_dir_all(root.join("Nested")).expect("create fixture root");
    fs::write(root.join("A.ken"), "const a : Bool = True\n").expect("write .ken fixture");
    fs::write(
        root.join("Nested/B.ken.md"),
        "# B\n\n```ken\nconst b : Bool = True\n```\n",
    )
    .expect("write .ken.md fixture");
    fs::write(root.join("Ignored.txt"), "not Ken\n").expect("write noise fixture");

    let discovered = discover_units(&root);
    assert_eq!(
        discovered.keys().cloned().collect::<BTreeSet<_>>(),
        BTreeSet::from(["A".to_string(), "Nested.B".to_string()])
    );

    let fixture_top = root
        .parent()
        .and_then(Path::parent)
        .expect("fixture has temporary top");
    fs::remove_dir_all(fixture_top).expect("remove fixture tree");
}

/// Promise class: durable invariant.
///
/// MEASURED: an independent legacy observation requires exact success/evidence
/// and refusal/stage agreement, while the Strict observation can vary
/// independently on an unavailable row. CLAIMED: a failed legacy load cannot be
/// enriched into guessed dependency evidence. THE GAP: the diagnostic payload
/// is deliberately excluded because it has no identity authority.
#[test]
fn evidence_status_is_typed_and_independent_from_strict_observation() {
    assert_eq!(
        failure_stage(&ElabError::ParseError {
            msg: "fixture".to_string(),
            span: Span::zero(),
        }),
        Some(PipelineStage::Parsing)
    );
    assert_eq!(
        failure_stage(&ElabError::UnresolvedCon {
            name: "fixture".to_string(),
            span: Span::zero(),
        }),
        Some(PipelineStage::Resolution)
    );
    assert_eq!(
        failure_stage(&ElabError::TypeMismatch {
            span: Span::zero(),
            reason: "fixture".to_string(),
        }),
        Some(PipelineStage::Elaboration)
    );
    assert_eq!(
        failure_stage(&ElabError::KernelRejected {
            error: KernelError::K2ReservedFormer,
            span: Span::zero(),
        }),
        Some(PipelineStage::Kernel)
    );

    let unavailable = UnitCensusRow {
        canonical_module: "Fixture.Unavailable".to_string(),
        parsed_direct_imports: BTreeSet::new(),
        strict_observation: LoaderObservation::Refused {
            stage: PipelineStage::Resolution,
        },
        legacy_observation: LoaderObservation::Refused {
            stage: PipelineStage::Resolution,
        },
        legacy_identity_evidence: LegacyIdentityEvidence::Unavailable {
            failing_stage: PipelineStage::Resolution,
        },
    };

    let mut forged = vec![unavailable.clone()];
    forged[0].legacy_identity_evidence = LegacyIdentityEvidence::ExactLedger {
        resolved_globals: BTreeSet::new(),
        identity_grounded_prerequisites: BTreeSet::from([
            IdentityPrerequisite::ProviderInterfaceAndImport {
                identity: GlobalId(0),
                defining_module: "Invented.Provider".to_string(),
                provider_interface_available: true,
            },
        ]),
    };
    assert!(
        validate_evidence_frontier(&forged).is_err(),
        "a failed legacy load cannot be relabelled as a nonempty ExactLedger"
    );

    let mut wrong_stage = vec![unavailable.clone()];
    wrong_stage[0].legacy_identity_evidence = LegacyIdentityEvidence::Unavailable {
        failing_stage: PipelineStage::Kernel,
    };
    assert!(
        validate_evidence_frontier(&wrong_stage).is_err(),
        "unavailable evidence must retain the independently observed failure stage"
    );

    let mut missing_success_ledger = vec![unavailable.clone()];
    missing_success_ledger[0].legacy_observation = LoaderObservation::Succeeded;
    assert!(
        validate_evidence_frontier(&missing_success_ledger).is_err(),
        "a successful legacy load must carry an ExactLedger"
    );

    let mut orthogonal = vec![unavailable];
    orthogonal[0].strict_observation = LoaderObservation::Succeeded;
    assert!(
        validate_evidence_frontier(&orthogonal).is_ok(),
        "strict outcome must not manufacture legacy identity evidence"
    );
}
