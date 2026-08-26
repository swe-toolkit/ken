//! LANG-MOD-STRICT-RESOLUTION D0 buildability and migration census.
//!
//! D0 is measurement only: every root-loaded ambient fallback exercised here
//! still succeeds. No strict-mode carrier or enforcement exists in this file.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::{
    catalog_module_from_path, is_prelude_floor_name, PRELUDE_COMPANION_BINDING_NAMES,
    PRELUDE_FLOOR_NAMES,
};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Level, Term};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-strict-d0-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create D0 fixture root");
        Self(path)
    }

    fn write(&self, relative: &str, source: &str) {
        fs::write(self.0.join(relative), source).expect("write D0 fixture");
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// D1 transition sentinel: root-loaded and isolated entry paths are already
/// observably distinct, while D0 still retains the same ambient passthrough.
#[test]
fn root_and_legacy_are_distinct_entries_with_ambient_passthrough_still_live() {
    let root = FixtureRoot::new("mode-key");
    root.write("Value.ken", "const leaked : Int = ambient_value");
    root.write(
        "Type.ken",
        "fn identity (x : AmbientType) : AmbientType = x",
    );
    root.write("Data.ken", "data Boxed = Box AmbientType");

    let mut rooted = ElabEnv::new().expect("base environment");
    let int = Term::const_(rooted.globals["Int"], vec![]);
    let ambient_value = rooted
        .declare_postulate_raw("ambient_value", int)
        .expect("declare ambient value");
    rooted
        .declare_postulate_raw("AmbientType", Term::ty(Level::Zero))
        .expect("declare ambient type");

    for entry in ["Value", "Type", "Data"] {
        rooted
            .elaborate_module_from_roots(&[root.0.clone()], entry)
            .unwrap_or_else(|error| panic!("root-loaded {entry} retains D0 fallback: {error}"));
    }
    assert_eq!(rooted.loaded_module_count(), 3);
    let (_, body) = rooted
        .env
        .transparent_body(rooted.globals["Value.leaked"])
        .expect("root-loaded value is transparent");
    assert!(matches!(body, Term::Const { id, .. } if id == ambient_value));

    let mut legacy = ElabEnv::new().expect("base environment");
    let int = Term::const_(legacy.globals["Int"], vec![]);
    let legacy_ambient = legacy
        .declare_postulate_raw("ambient_value", int)
        .expect("declare legacy ambient value");
    legacy
        .declare_postulate_raw("AmbientType", Term::ty(Level::Zero))
        .expect("declare legacy ambient type");
    legacy
        .elaborate_file(
            "const leaked : Int = ambient_value \
             fn identity (x : AmbientType) : AmbientType = x \
             data Boxed = Box AmbientType",
        )
        .expect("isolated legacy path retains all ambient fallbacks");
    assert_eq!(legacy.loaded_module_count(), 0);
    let (_, body) = legacy
        .env
        .transparent_body(legacy.globals["leaked"])
        .expect("legacy value is transparent");
    assert!(matches!(body, Term::Const { id, .. } if id == legacy_ambient));
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum AmbientNameRoute {
    ExprConToGlobals,
    TypeConToElabGlobals,
    TypeConToDataGlobals,
    PatternCtorToGlobals,
    AttachedDeclSubjectToGlobals,
    AttachedExprSubjectToGlobals,
    ExportInScopeToGlobals,
    DeriveClassToClassEnv,
    DeriveDataToGlobals,
    InstanceClassToClassEnv,
    InstanceConstraintClassToClassEnv,
    ViewConstraintClassToClassEnv,
}

const COMPLETE_AMBIENT_NAME_ROUTES: [AmbientNameRoute; 12] = [
    AmbientNameRoute::ExprConToGlobals,
    AmbientNameRoute::TypeConToElabGlobals,
    AmbientNameRoute::TypeConToDataGlobals,
    AmbientNameRoute::PatternCtorToGlobals,
    AmbientNameRoute::AttachedDeclSubjectToGlobals,
    AmbientNameRoute::AttachedExprSubjectToGlobals,
    AmbientNameRoute::ExportInScopeToGlobals,
    AmbientNameRoute::DeriveClassToClassEnv,
    AmbientNameRoute::DeriveDataToGlobals,
    AmbientNameRoute::InstanceClassToClassEnv,
    AmbientNameRoute::InstanceConstraintClassToClassEnv,
    AmbientNameRoute::ViewConstraintClassToClassEnv,
];

/// D1 transition sentinel inventory of every resolved surface representation
/// that can reach arbitrary ambient declaration state during roots loading.
///
/// The inventory is by representation and consumer, rather than spelling:
/// `RExpr::RCon`, `RType::RCon`'s two consumers, `RPatKind::Ctor`, the attached
/// declaration/reference forms, and in-scope export all pass through the module
/// resolver before reaching `globals`; derive, instance, and both constraint
/// consumers retain class/data names and consult `class_env`/`globals` directly.
///
/// All other name-bearing fields are non-ambient by construction: declaration
/// and constructor names introduce bindings; `RVar`/`RVarTy`/pattern variables,
/// cells, and recursive-result names are lexically indexed; record/class field
/// and projection names are structural labels; qualified imports and facade
/// exports consult only module export tables; effect-row names, temporal atoms,
/// foreign symbols, and library names inhabit separate non-global namespaces.
#[test]
fn every_ambient_name_representation_reaches_its_current_route() {
    let mut observed = BTreeSet::new();

    let root = FixtureRoot::new("name-routes");
    root.write("Expr.ken", "const leaked : Int = ambient_value");
    root.write(
        "Type.ken",
        "fn identity (x : AmbientType) : AmbientType = x",
    );
    root.write("Data.ken", "data Boxed = Box AmbientType");
    root.write(
        "Pattern.ken",
        "fn inspect (x : Bool) : Bool = match x { AmbientTrue |-> True ; False |-> False }",
    );
    root.write("Export.ken", "export ambient_value");
    root.write(
        "AttachedDecl.ken",
        "proof rooted for ambient_id (x : Int) : Equal Int (ambient_id x) x = Refl",
    );
    root.write(
        "AttachedExpr.ken",
        "theorem consume (x : Int) : Equal Int (ambient_id x) x = ambient_id::stable x",
    );

    let mut globals = ElabEnv::new().expect("base environment");
    let int = Term::const_(globals.globals["Int"], vec![]);
    globals
        .declare_postulate_raw("ambient_value", int)
        .expect("declare ambient value");
    globals
        .declare_postulate_raw("AmbientType", Term::ty(Level::Zero))
        .expect("declare ambient type");
    let true_id = globals.globals["True"];
    globals.globals.insert("AmbientTrue".to_string(), true_id);
    globals
        .elaborate_file(
            "fn ambient_id (x : Int) : Int = x \
             proof stable for ambient_id (x : Int) : Equal Int (ambient_id x) x = Refl",
        )
        .expect("declare ambient attached-proof subject and proof");

    for (entry, routes) in [
        ("Expr", &[AmbientNameRoute::ExprConToGlobals][..]),
        ("Type", &[AmbientNameRoute::TypeConToElabGlobals][..]),
        ("Data", &[AmbientNameRoute::TypeConToDataGlobals][..]),
        ("Pattern", &[AmbientNameRoute::PatternCtorToGlobals][..]),
        ("Export", &[AmbientNameRoute::ExportInScopeToGlobals][..]),
        (
            "AttachedDecl",
            &[AmbientNameRoute::AttachedDeclSubjectToGlobals][..],
        ),
        (
            "AttachedExpr",
            &[AmbientNameRoute::AttachedExprSubjectToGlobals][..],
        ),
    ] {
        globals
            .elaborate_module_from_roots(&[root.0.clone()], entry)
            .unwrap_or_else(|error| panic!("{entry} ambient route remains live in D0: {error}"));
        observed.extend(routes.iter().copied());
    }

    let derive_root = FixtureRoot::new("derive-routes");
    derive_root.write("Entry.ken", "derive AmbientDerive for AmbientData");
    let mut derive = ElabEnv::new().expect("base environment");
    derive
        .elaborate_file("class AmbientDerive a {} data AmbientData = MkAmbientData")
        .expect("declare ambient derive class and data");
    derive
        .elaborate_module_from_roots(&[derive_root.0.clone()], "Entry")
        .expect("derive class/data names retain direct ambient routes in D0");
    observed.extend([
        AmbientNameRoute::DeriveClassToClassEnv,
        AmbientNameRoute::DeriveDataToGlobals,
    ]);

    let instance_root = FixtureRoot::new("instance-routes");
    instance_root.write(
        "Entry.ken",
        "data Local = MkLocal \
         instance AmbientInstance Local where (d : AmbientConstraint Local) {}",
    );
    let mut instance = ElabEnv::new().expect("base environment");
    instance
        .elaborate_file("class AmbientInstance a {} class AmbientConstraint a {}")
        .expect("declare ambient instance and constraint classes");
    instance
        .elaborate_module_from_roots(&[instance_root.0.clone()], "Entry")
        .expect("instance and prerequisite class names retain direct class routes in D0");
    observed.extend([
        AmbientNameRoute::InstanceClassToClassEnv,
        AmbientNameRoute::InstanceConstraintClassToClassEnv,
    ]);

    let view_root = FixtureRoot::new("view-constraint-route");
    view_root.write(
        "Provider.ken",
        "class AmbientView a {} instance AmbientView Bool {}",
    );
    view_root.write(
        "Entry.ken",
        "fn constrained (x : Bool) : Bool where AmbientView Bool = x",
    );
    let mut view = ElabEnv::new().expect("base environment");
    view.elaborate_module_from_roots(&[view_root.0.clone()], "Provider")
        .expect("load the ambient class/dictionary without importing it into Entry");
    view.elaborate_module_from_roots(&[view_root.0.clone()], "Entry")
        .expect("view constraint class retains direct class route in D0");
    observed.insert(AmbientNameRoute::ViewConstraintClassToClassEnv);

    assert_eq!(
        observed,
        COMPLETE_AMBIENT_NAME_ROUTES.into_iter().collect(),
        "every ambient-capable name representation has a live D0 route probe"
    );
}

/// Durable D0 measurement: the floor predicate is exactly the installer's
/// closed set, and a floor former plus kernel syntax elaborate in a root-loaded
/// unit without any user-declared ambient convenience.
#[test]
fn closed_floor_and_kernel_vocabulary_are_buildable_from_roots() {
    assert_eq!(
        PRELUDE_FLOOR_NAMES.as_slice(),
        [
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
        ]
        .as_slice()
    );
    for name in PRELUDE_FLOOR_NAMES {
        assert!(is_prelude_floor_name(name));
    }
    for name in [
        "True", "Int", "Ordering", "Equal", "Prod", "mk_pair", "pair_fst", "pair_snd",
    ] {
        assert!(!is_prelude_floor_name(name));
    }

    let root = FixtureRoot::new("floor");
    root.write(
        "Entry.ken",
        "fn bool_id (x : Bool) : Bool = x \
         data Identity (a : Type) : Type where { MkIdentity : a -> Identity a }",
    );
    let mut env = strict_floor_env(&BTreeSet::new());
    env.elaborate_module_from_roots(&[root.0.clone()], "Entry")
        .expect("closed floor member and kernel Type syntax resolve from roots");
}

fn source_leaves(root: &Path) -> Vec<PathBuf> {
    fn visit(dir: &Path, leaves: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).expect("read census directory") {
            let path = entry.expect("read census entry").path();
            if path.is_dir() {
                visit(&path, leaves);
            } else if path.to_string_lossy().ends_with(".ken")
                || path.to_string_lossy().ends_with(".ken.md")
            {
                leaves.push(path);
            }
        }
    }
    let mut leaves = Vec::new();
    visit(root, &mut leaves);
    leaves.sort();
    leaves
}

fn strict_floor_env(extra_names: &BTreeSet<String>) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    let mut admitted = env.prelude_env.native_trusted_base.clone();
    for name in PRELUDE_FLOOR_NAMES {
        let id = env.globals[name];
        admitted.insert(id);
        if let Some(inductive) = env.env.inductive(id) {
            admitted.extend(inductive.constructors.iter().map(|ctor| ctor.id));
        }
    }
    admitted.extend(PRELUDE_COMPANION_BINDING_NAMES.map(|name| env.globals[name]));
    env.globals
        .retain(|name, id| admitted.contains(id) || extra_names.contains(name));
    env
}

fn ambient_dependencies(root: &Path, entry: &str) -> Result<Vec<String>, String> {
    let full_initial = ElabEnv::new().map_err(|error| error.to_string())?;
    let available = full_initial
        .globals
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut admitted = BTreeSet::new();

    for _ in 0..available.len() {
        let mut env = strict_floor_env(&admitted);
        match env.elaborate_module_from_roots(&[root.to_path_buf()], entry) {
            Ok(_) => return Ok(admitted.into_iter().collect()),
            Err(ElabError::UnresolvedCon { name, .. }) if available.contains(&name) => {
                if !admitted.insert(name.clone()) {
                    return Err(format!("repeated unresolved convenience `{name}`"));
                }
            }
            Err(error) => return Err(format!("{error}")),
        }
    }
    Err("ambient dependency census exceeded the initial global inventory".to_string())
}

/// Transition sentinel for catalog migration: this is a behavioral census,
/// not a source-text census. The ten-type floor intentionally removes its
/// families and constructors from these residual dependency vectors; every
/// remaining name still requires an explicit provider migration.
#[test]
fn catalog_ambient_passthrough_migration_census() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = repo.join("catalog/packages");
    let mut discovered = BTreeSet::new();
    let mut census = Vec::new();
    let mut clean = Vec::new();
    let mut residuals = Vec::new();

    for path in source_leaves(&root) {
        let address = catalog_module_from_path(&path).expect("catalog leaf has a module address");
        discovered.insert(address.entry.clone());
        let mut baseline = ElabEnv::new().expect("base environment");
        if let Err(error) = baseline
            .elaborate_module_from_roots(std::slice::from_ref(&address.root), &address.entry)
        {
            residuals.push((address.entry, format!("baseline: {error}")));
            continue;
        }
        match ambient_dependencies(&address.root, &address.entry) {
            Ok(names) if !names.is_empty() => census.push((address.entry, names)),
            Ok(_) => clean.push(address.entry),
            Err(error) => residuals.push((address.entry, error)),
        }
    }

    let expected = vec![
        (
            "Algorithm.Numeric.Gcd".to_string(),
            [
                "And",
                "Bottom",
                "Equal",
                "Prop",
                "Proved",
                "and_fst",
                "and_intro",
                "and_snd",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.Console.Text".to_string(),
            ["IO", "IOError", "Stderr", "Stdout", "Unit", "write"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Capability.Diagnostics.Core".to_string(),
            ["Bottom", "Equal", "Prop", "Top"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Capability.Filesystem.Authority".to_string(),
            [
                "CreatePolicy",
                "FS",
                "FileError",
                "Unit",
                "read_bytes",
                "write_file",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.Filesystem.Errors".to_string(),
            [
                "AlreadyExists",
                "BrokenPipe",
                "CapabilityDenied",
                "FileError",
                "IOError",
                "Interrupted",
                "InvalidInput",
                "IsDirectory",
                "MkFileError",
                "NotDirectory",
                "NotEmpty",
                "NotFound",
                "Other",
                "PermissionDenied",
                "Unsupported",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.Process.Environment".to_string(),
            ["Equal", "MkProcessInput", "ProcessInput", "Prod"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Capability.Process.Exit".to_string(),
            ["ExitCode", "Failure", "Success"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Capability.Process.WorkingDirectory".to_string(),
            ["Equal", "MkProcessInput", "ProcessInput"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Capability.System.Buffer".to_string(),
            [
                "BufferSpan",
                "BufferWindow",
                "Equal",
                "MkBufferWindow",
                "TransferCount",
                "buffer_nat_add",
                "buffer_span_budget",
                "buffer_span_length",
                "transfer_count_int",
                "transfer_count_nat",
                "transfer_count_positive",
                "transfer_count_positive_prop",
                "transfer_count_remaining",
                "transfer_count_request_budget",
                "transfer_count_request_budget::bounded",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.System.Resource".to_string(),
            [
                "ResourceBodyErr",
                "ResourceBodyOk",
                "ResourceBodyResult",
                "ResourceBracketBodyAndReleaseError",
                "ResourceBracketBodyError",
                "ResourceBracketOk",
                "ResourceBracketReleaseError",
                "ResourceBracketResult",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.Time.WallClock".to_string(),
            ["Instant", "MkInstant"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Core.Classes.LawfulClasses".to_string(),
            [
                "And",
                "Bottom",
                "Equal",
                "Prop",
                "Proved",
                "and_fst",
                "and_intro",
                "and_snd",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Core.Logic.Compare".to_string(),
            ["And", "Equal", "Proved", "and_intro"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Core.Logic.EmptyDec".to_string(),
            ["Dec", "Empty", "Equal", "IsTrue", "No", "Proved", "Yes"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Data.Collections.Deque".to_string(),
            ["Equal"].into_iter().map(str::to_string).collect(),
        ),
        (
            "Data.Collections.Derived".to_string(),
            [
                "And",
                "Equal",
                "Prop",
                "Proved",
                "Top",
                "Unit",
                "and_intro",
                "and_snd",
                "eqChar",
                "is_sorted",
                "leqChar",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Data.Numeric.Nat.Arithmetic".to_string(),
            ["Equal", "Proved"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Data.Numeric.Nat.Order".to_string(),
            [
                "And",
                "Bottom",
                "Equal",
                "Prop",
                "Proved",
                "and_fst",
                "and_intro",
                "and_snd",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Data.Sums.Combinators".to_string(),
            ["Equal", "Proved"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Data.Vector.Vector".to_string(),
            ["Equal", "Proved"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
        (
            "Tooling.Testing.Property".to_string(),
            ["MkUnit", "Unit"].into_iter().map(str::to_string).collect(),
        ),
        (
            "Tooling.Verification.FoKripke".to_string(),
            ["Bottom", "Equal", "Proved"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
    ];
    assert_eq!(
        census, expected,
        "WP-4 migration sentinel: ambient dependencies changed"
    );

    let ambient = census
        .iter()
        .map(|(entry, _)| entry.clone())
        .collect::<BTreeSet<_>>();
    let clean = clean.into_iter().collect::<BTreeSet<_>>();
    let residual_names = residuals
        .iter()
        .map(|(entry, _)| entry.clone())
        .collect::<BTreeSet<_>>();
    let expected_clean = [
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Tooling.Verification.ProofErasureBoundaryChecker",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let expected_residuals = [
        "Algorithm.Searching.OrderedSearch",
        "Algorithm.Sorting.InsertionSort",
        "Application.CommandLine.ArgParse",
        "Application.Configuration.Decoder",
        "Application.Input.Schema",
        "Capability.Diagnostics.Render",
        "Capability.Filesystem.Path.Posix",
        "Capability.Formatting.Doc",
        "Capability.Parsing.Cursor",
        "Capability.Parsing.Decoder",
        "Capability.Parsing.Numeric",
        "Capability.Parsing.Parsing",
        "Capability.Process.Arguments",
        "Capability.System.IO",
        "Core.Classes.EffectfulClasses",
        "Core.Classes.LawfulFunctors",
        "Data.Binary.BytesKeys",
        "Data.Collections.Map",
        "Data.Collections.NonEmpty",
        "Data.Serialization.Json",
        "Data.Sums.Validation",
        "Data.Text.Codec",
        "Data.Text.StringBijection",
        "Data.Text.StringKeys",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    assert_eq!(clean, expected_clean, "WP-4 strict-floor-clean sentinel");
    assert_eq!(
        residual_names, expected_residuals,
        "WP-4 baseline-red residual sentinel"
    );
    assert!(ambient.is_disjoint(&clean));
    assert!(ambient.is_disjoint(&residual_names));
    assert!(clean.is_disjoint(&residual_names));
    let partition = ambient
        .union(&clean)
        .cloned()
        .collect::<BTreeSet<_>>()
        .union(&residual_names)
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        partition, discovered,
        "ambient, clean, and residual sets exhaust discovered catalog addresses"
    );

    let library = repo.join("library");
    for path in source_leaves(&library) {
        assert!(
            catalog_module_from_path(&path).is_none(),
            "library guide remains on the isolated legacy path: {}",
            path.display()
        );
    }

    println!("D0 residuals = {residuals:#?}");
}
