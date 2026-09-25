//! Catalog logic providers must resolve the new decision family by import,
//! not silently fall through to the still-registered prelude family.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

fn copy_catalog(source: &std::path::Path, destination: &std::path::Path) {
    fs::create_dir_all(destination).expect("create catalog mirror");
    for item in fs::read_dir(source).expect("read catalog source") {
        let item = item.expect("read catalog entry");
        let path = item.path();
        let target = destination.join(item.file_name());
        if path.is_dir() {
            copy_catalog(&path, &target);
        } else {
            fs::copy(&path, &target).expect("mirror catalog leaf");
        }
    }
}

impl Fixture {
    fn new(imports: &[&str]) -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ken-logic-prelude-move-{}-{id}",
            std::process::id()
        ));
        let catalog = root.join("catalog/packages");
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("catalog/packages");
        copy_catalog(&source, &catalog);
        let folder = catalog.join("Probe");
        fs::create_dir_all(&folder).expect("make fixture module");
        let source = format!(
            "import Core.Logic.EmptyDec ({})\n\
             fn yes_case (p : Top) : Dec Top = Yes Top p\n\
             fn no_case (f : Top → Empty) : Dec Top = No Top f\n\
             fn decision (d : Dec Top) : Bool = decide Top d\n",
            imports.join(", ")
        );
        fs::write(folder.join("Consumer.ken"), source).expect("write fixture module");
        Self(root)
    }

    fn ordered_search(imports: &[&str]) -> Self {
        let fixture = Self::new(&["Empty", "Dec", "Yes", "No", "decide"]);
        let path = fixture
            .0
            .join("catalog/packages/Algorithm/Searching/OrderedSearch.ken.md");
        let source = fs::read_to_string(&path).expect("read OrderedSearch");
        let original = "import Core.Logic.EmptyDec (Empty, Dec, Yes, No)";
        let (before, after) = source
            .split_once(original)
            .expect("derive the OrderedSearch import control from its checked source");
        let replacement = format!("import Core.Logic.EmptyDec ({})", imports.join(", "));
        fs::write(path, format!("{before}{replacement}{after}"))
            .expect("write OrderedSearch import control");
        fixture
    }

    fn check_module(&self, module: &str) -> Result<(), ElabError> {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_module_from_roots(&[self.0.join("catalog/packages")], module)
            .map(|_| ())
    }

    fn check(&self) -> Result<(), ElabError> {
        self.check_module("Probe.Consumer")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Promise class: durable invariant. MEASURED: one roots-loaded consumer
/// using catalog Empty/Dec/Yes/No/decide checks, but omitting each inductive
/// import individually reaches a kernel TypeMismatch. CLAIMED: the imported
/// family, constructors and refutation target share one checked identity,
/// rather than falling back to the prelude copies. THE GAP: this fixture pins
/// import resolution for these uses; the OrderedSearch consumer is checked
/// independently in the companion test.
#[test]
fn decision_family_imports_select_one_catalog_identity() {
    let all = ["Empty", "Dec", "Yes", "No", "decide"];
    Fixture::new(&all)
        .check()
        .expect("the imported decision family must check together");

    for omitted in ["Empty", "Dec", "Yes", "No"] {
        let names: Vec<_> = all
            .iter()
            .copied()
            .filter(|name| *name != omitted)
            .collect();
        let error = Fixture::new(&names)
            .check()
            .expect_err("omitting one family member must not fall through to the prelude");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "omitting {omitted} must fail on a kernel type mismatch, got {error:?}"
        );
    }
}

/// Promise class: durable invariant. MEASURED: the real OrderedSearch module
/// checks when all four catalog names are imported; changing only its import
/// list by deleting one name reaches a kernel TypeMismatch in each arm.
/// CLAIMED: its search uses one catalog decision family rather than silently
/// falling through to prelude identities. THE GAP: this pins four import
/// resolution edges, not search's operational or sortedness properties.
#[test]
fn ordered_search_import_removals_are_attributable() {
    let all = ["Empty", "Dec", "Yes", "No"];
    Fixture::ordered_search(&all)
        .check_module("Algorithm.Searching.OrderedSearch")
        .expect("OrderedSearch must check with all decision imports");

    for omitted in all {
        let names: Vec<_> = all
            .iter()
            .copied()
            .filter(|name| *name != omitted)
            .collect();
        let error = Fixture::ordered_search(&names)
            .check_module("Algorithm.Searching.OrderedSearch")
            .expect_err("only removing the selected import must reject OrderedSearch");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "OrderedSearch omitting {omitted} must fail with kernel TypeMismatch, got {error:?}"
        );
    }
}
