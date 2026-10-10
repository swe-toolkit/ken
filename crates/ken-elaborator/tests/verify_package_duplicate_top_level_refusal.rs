//! Package-local root definitions stay unique across checked source units.
//! Promise class: durable invariant (spec 46 §3.2): a second top-level binding
//! cannot take an admitted declaration's stable spelling.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::ElabEnv;
use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerDriverError, CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources, compile_ken_source,
};
use ken_elaborator::error::ElabError;

const PKG: &str = "zz_adv_duplicate";
const ADMITTED: &str = "```ken\nconst base : Bool = True\nconst main : Bool = base\n```\n";
static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

fn selected_main() -> TargetSelector {
    TargetSelector::StableSymbol {
        package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
        symbol: StableSymbol::declaration(PKG, &[], "main"),
        kind: CompilerTargetKind::NonRuntime,
    }
}

fn compile_literate(
    src: &str,
) -> Result<ken_elaborator::compiler_driver::CompilerDriverOutput, CompilerDriverError> {
    compile_ken_source(PKG, CompilerSource::new("a.ken.md", src), selected_main())
}

fn assert_duplicate(error: CompilerDriverError, expected: &str) {
    match error {
        CompilerDriverError::Elaboration(ElabError::DuplicateDefinition { name, .. }) => {
            assert_eq!(name, expected);
        }
        other => panic!("expected DuplicateDefinition({expected}), got {other:?}"),
    }
}

/// MEASURED: both a fenced example and a later plain source refuse when
/// they take `base` or `main`; the previously accepted shadowing case changes
/// the admitted package's hash and even removes `main` from target selection.
/// CLAIMED: a package cannot rebind a prior top-level name across units.
/// THE GAP: a separate roots-entry test covers the non-driver module route.
#[test]
fn examples_and_plain_sources_refuse_duplicate_names() {
    for name in ["base", "main"] {
        let fence = format!("{ADMITTED}```ken example\nconst {name} : Bool = False\n```\n");
        assert_duplicate(compile_literate(&fence).unwrap_err(), name);
        let plain = compile_ken_package_sources(
            &CompilerManifest::new(PKG, Vec::new()),
            vec![
                CompilerSource::new(
                    "a.ken",
                    "const base : Bool = True\nconst main : Bool = base",
                ),
                CompilerSource::new("b.ken", format!("const {name} : Bool = False")),
            ],
            selected_main(),
        );
        assert_duplicate(plain.unwrap_err(), name);
    }
}

/// MEASURED: an unrelated checked example is excluded from the emitted
/// package and leaves its canonical semantic hash unchanged. CLAIMED:
/// unrelated example checking is still legal, without shifting admitted
/// declaration identity. THE GAP: this fixes the package-name-dependent
/// baseline vector for this one package, not every possible package name.
#[test]
fn unrelated_example_keeps_the_baseline_semantics_and_hash() {
    let baseline = compile_literate(ADMITTED)
        .expect("ordinary checked package")
        .package;
    let with_example = format!("{ADMITTED}```ken example\nconst zz_other : Bool = False\n```\n");
    let checked = compile_literate(&with_example)
        .expect("independent example remains valid")
        .package;
    assert_eq!(baseline.core_semantic_hash, 0x7064ef0807895040);
    assert_eq!(checked.core_semantic_hash, baseline.core_semantic_hash);
    assert_eq!(checked.artifact.semantic, baseline.artifact.semantic);
}

struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-package-duplicate-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create package-root fixture");
        Self(path)
    }
}
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// MEASURED: the loaded entry's scope resolves `base` to Entry.base, so
/// prebinding rejects a second `base` with AmbiguousReference while an
/// unrelated fresh example succeeds. CLAIMED: module-route checked fences
/// cannot rebind their own entry's declarations. THE GAP: this earlier scope
/// guard does not prove the package-route resolver's duplicate-set check.
#[test]
fn roots_entry_checked_fence_refuses_its_own_name() {
    let root = Root::new();
    fs::write(
        root.0.join("Entry.ken.md"),
        "```ken\nconst base : Bool = True\n```\n```ken example\nconst base : Bool = False\n```\n",
    )
    .unwrap();
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .unwrap();
    let error = env
        .execute_loaded_entry_checked_fences_v1("Entry")
        .unwrap_err();
    assert!(
        matches!(&error, ElabError::AmbiguousReference { name, sources, .. }
        if name == "base" && sources == &["Entry.base", "base"]),
        "entry scope must report the exact prebind collision, got {error:?}"
    );
}

/// MEASURED: an entry's fresh checked example still elaborates after the
/// module-specific seed is installed. CLAIMED: the seed rejects collisions,
/// not every checked fence. THE GAP: the companion base row above is the
/// rejecting half of the same entry surface.
#[test]
fn roots_entry_checked_fence_accepts_fresh_name() {
    let root = Root::new();
    fs::write(root.0.join("Entry.ken.md"),
        "```ken\nconst base : Bool = True\n```\n```ken example\nconst zz_fresh : Bool = False\n```\n").unwrap();
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .unwrap();
    let examples = env
        .execute_loaded_entry_checked_fences_v1("Entry")
        .expect("fresh example name must remain legal after entry seeding");
    assert_eq!(examples.len(), 1);
    assert_eq!(examples[0].name, "zz_fresh");
}

/// MEASURED: the loaded entry's constructor binding is visible in the
/// checked fence scope, so a duplicate constructor is rejected by prebinding.
/// CLAIMED: module-route checked fences cannot take an entry's constructor
/// spelling. THE GAP: the package-route resolver pin below is independent.
#[test]
fn roots_entry_checked_fence_refuses_own_constructor() {
    let root = Root::new();
    fs::write(
        root.0.join("Entry.ken.md"),
        "```ken\ndata U = ZzBase | ZzOther\n```\n```ken example\ndata T = ZzBase | ZzQ\n```\n",
    )
    .unwrap();
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .unwrap();
    let error = env
        .execute_loaded_entry_checked_fences_v1("Entry")
        .unwrap_err();
    assert!(
        matches!(&error, ElabError::AmbiguousReference { name, sources, .. }
        if name == "ZzBase" && sources == &["Entry.ZzBase", "ZzBase"]),
        "entry constructor's scope must refuse its own spelling, got {error:?}"
    );
}

/// MEASURED: a failed negative fence does not enter the later example's
/// definition set. CLAIMED: failed-unit names are not committed to package
/// identity. THE GAP: an elaboration error may leave other environment state;
/// this tests only the resolver name set's subsequent admission behavior.
#[test]
fn rejected_fence_does_not_reserve_its_failed_name() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_ken_md_file(
        "```ken\nconst base : Bool = True\n```\n```ken reject\nconst zz_retry : Bool = unbound\n```\n```ken example\nconst zz_retry : Bool = False\n```\n",
    ).expect("failed reject must not reserve zz_retry against the checked example");
}

/// MEASURED: a second source family cannot bind a constructor already
/// admitted by an earlier family, while distinct constructor names remain
/// legal. CLAIMED: constructor names occupy the same package definition set.
/// THE GAP: the earlier prelude collision is separately guarded by the
/// existing DuplicateConstructorSpelling test; this uses unique spellings.
#[test]
fn example_repeated_constructor_name_is_refused() {
    let mut env = ElabEnv::new().unwrap();
    let error = env
        .elaborate_ken_md_file(
            "```ken\ndata U = ZzBase | ZzOther\n```\n```ken example\ndata T = ZzBase | ZzQ\n```\n",
        )
        .unwrap_err();
    assert!(
        matches!(&error, ElabError::DuplicateDefinition { name, .. } if name == "ZzBase"),
        "expected the resolver's package-local constructor collision, got {error:?}"
    );
}

#[test]
fn uppercase_const_constructor_collision() {
    // Baseline syntax accepted `const ZzK : Bool = True` on ce4e72b33.
    let mut env = ElabEnv::new().unwrap();
    let example = "```ken\nconst ZzK : Bool = True\n```\n```ken example\ndata T = ZzK | ZzR\n```\n";
    let error = env.elaborate_ken_md_file(example).unwrap_err();
    assert!(
        matches!(&error, ElabError::DuplicateDefinition { name, .. } if name == "ZzK"),
        "uppercase const and constructor must share the definition set, got {error:?}"
    );
}
