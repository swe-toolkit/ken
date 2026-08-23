//! LANG-MOD-STRICT-RESOLUTION D0 buildability and migration census.
//!
//! D0 is measurement only: every root-loaded ambient fallback exercised here
//! still succeeds. No strict-mode carrier or enforcement exists in this file.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::{
    catalog_module_from_path, is_prelude_floor_name, PRELUDE_FLOOR_NAMES,
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

/// Durable D0 measurement: root-loaded and isolated entry paths are already
/// observably distinct, while both still retain the same ambient passthrough.
/// The type-only and inductive-constructor cases prove `RType::RCon` has direct
/// global fall-throughs in both `elab.rs` and `data.rs`, in addition to the
/// value `RExpr::RCon` seam.
#[test]
fn root_and_legacy_are_distinct_entries_with_all_fallbacks_still_live() {
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

/// Durable D0 measurement: the floor predicate is exactly the installer's
/// closed set, and a floor former plus kernel syntax elaborate in a root-loaded
/// unit without any user-declared ambient convenience.
#[test]
fn closed_floor_and_kernel_vocabulary_are_buildable_from_roots() {
    assert_eq!(PRELUDE_FLOOR_NAMES, ["Bool", "Char", "List"]);
    for name in PRELUDE_FLOOR_NAMES {
        assert!(is_prelude_floor_name(name));
    }
    for name in ["True", "Nat", "Int", "Ordering", "Equal"] {
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

/// Transition sentinel for WP-4: this is a behavioral migration census, not a
/// source-text census. Retire the exact rows when WP-4 adds explicit imports;
/// D1 must use the same strict floor and make every dependency vector empty.
#[test]
fn catalog_ambient_passthrough_migration_census() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = repo.join("catalog/packages");
    let mut census = Vec::new();
    let mut clean = Vec::new();
    let mut residuals = Vec::new();

    for path in source_leaves(&root) {
        let address = catalog_module_from_path(&path).expect("catalog leaf has a module address");
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
            "Capability.Console.Text".to_string(),
            [
                "IO", "IOError", "Result", "Stderr", "Stdout", "Unit", "write",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        (
            "Capability.Filesystem.Authority".to_string(),
            [
                "AFull",
                "Auth",
                "CreatePolicy",
                "FS",
                "FileError",
                "Result",
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
            ["Err", "ExitCode", "Failure", "Ok", "Result", "Success"]
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
                "Nat",
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
            "Core.Logic.EmptyDec".to_string(),
            ["Dec", "Empty", "Equal", "IsTrue", "No", "Proved", "Yes"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ),
    ];
    assert_eq!(
        census, expected,
        "WP-4 migration sentinel: ambient dependencies changed"
    );

    let library = repo.join("library");
    for path in source_leaves(&library) {
        assert!(
            catalog_module_from_path(&path).is_none(),
            "library guide remains on the isolated legacy path: {}",
            path.display()
        );
    }

    println!("D0 strict-floor clean = {clean:#?}");
    println!("D0 residuals = {residuals:#?}");
    assert!(
        !residuals.is_empty(),
        "baseline-red catalog units are an explicit D0 residual, not silently census-clean"
    );
}
