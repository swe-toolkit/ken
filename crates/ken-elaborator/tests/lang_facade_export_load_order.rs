//! Facade provider selection under cold and caller-preloaded source graphs.
//! Durable invariant: spec 33 §§3.2–3.3, §4.3.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Term;

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str, files: &[(&str, &str)]) -> Self {
        let serial = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-facade-provider-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create root");
        for (file, source) in files {
            fs::write(path.join(file), source).expect("write fixture");
        }
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Verdict {
    Admitted,
    InlineProvider,
    FileProvider,
    UnboundFacade,
}

fn checked_body_id(env: &ElabEnv, name: &str) -> ken_kernel::GlobalId {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .expect("checked transparent consumer body");
    match body {
        Term::Const { id, .. } => id,
        other => panic!("{name} did not select a checked global: {other:?}"),
    }
}

fn observe(
    root: &FixtureRoot,
    entry: &str,
    a_source: &str,
    facade: &str,
    consumer: &str,
    inline_name: Option<&str>,
) -> Verdict {
    let mut env = ElabEnv::new().expect("fresh base environment per entry");
    let result = env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], entry);
    match result {
        Err(ElabError::UnboundName { name, span }) if name == "N" => {
            let at = a_source.find(facade).expect("facade in A source");
            assert_eq!(span.start, at, "the failure must be at A's facade");
            Verdict::UnboundFacade
        }
        Err(other) => panic!("entry {entry} failed at the wrong edge: {other:?}"),
        Ok(_) if entry == "A" => Verdict::Admitted,
        Ok(_) => {
            let selected = checked_body_id(&env, consumer);
            let file = env.globals.get("N.x").copied();
            let inline = inline_name.map(|name| env.globals[name]);
            if let Some(inline) = inline {
                assert_ne!(
                    file,
                    Some(inline),
                    "the two providers must have distinct IDs"
                );
            }
            if inline == Some(selected) {
                Verdict::InlineProvider
            } else if file == Some(selected) {
                Verdict::FileProvider
            } else {
                panic!("{consumer} used neither file nor inline provider: {selected:?}");
            }
        }
    }
}

const N: &str = "pub const x : Nat = Suc (Suc Zero)\n";
const D: &str = "import A (x)\npub const y : Nat = x\n";
const C1: &str = "import N\nimport D\n";
const C2: &str = "import D\nimport N\n";
const ENTRIES: [&str; 4] = ["A", "D", "C1", "C2"];

fn verdicts(label: &str, a: &str, d: &str, inline_name: Option<&str>) -> Vec<Verdict> {
    let root = FixtureRoot::new(
        label,
        &[
            ("N.ken", N),
            ("A.ken", a),
            ("D.ken", d),
            ("C1.ken", C1),
            ("C2.ken", C2),
        ],
    );
    ENTRIES
        .iter()
        .map(|entry| observe(&root, entry, a, "export N (x)", "D.y", inline_name))
        .collect()
}

/// Promise class: durable invariant. MEASURED: four fresh entries through
/// the same file graph, with the consumer's checked body ID compared against
/// both distinct providers. CLAIMED: an available inline N is the facade's
/// authority regardless of what the caller imported. THE GAP: the negative
/// order and file-positive tests below rule out always-inline selection.
#[test]
fn available_inline_facade_selects_its_checked_id_in_every_entry_order() {
    let a = "module N { pub const x : Nat = Zero }\nexport N (x)\n";
    assert_eq!(
        verdicts("available", a, D, Some("A.N.x")),
        vec![
            Verdict::Admitted,
            Verdict::InlineProvider,
            Verdict::InlineProvider,
            Verdict::InlineProvider,
        ]
    );
}

/// Promise class: durable invariant. MEASURED: all four entries reject the
/// same facade span and `UnboundName N`, including caller-first N loading.
/// CLAIMED: a declared but unavailable child cannot borrow the caller's
/// provider. THE GAP: the available and absolute controls independently
/// demonstrate that the harness can select both legitimate providers.
#[test]
fn later_inline_facade_rejects_before_expansion_in_every_entry_order() {
    let a = "export N (x)\nmodule N { pub const x : Nat = Zero }\n";
    assert_eq!(
        verdicts("later", a, D, Some("A.N.x")),
        vec![Verdict::UnboundFacade; 4]
    );
}

/// Promise class: durable invariant. MEASURED: a facade with no inline N
/// succeeds cold and selects the file N.x ID in all three consumers. CLAIMED:
/// an absolute facade keeps the import path's file provider, never an ambient
/// same-spelling export. THE GAP: the separate inline-positive case confirms
/// that a locally available child takes precedence when it actually exists.
#[test]
fn absolute_facade_selects_source_file_provider_cold_and_in_every_order() {
    assert_eq!(
        verdicts("absolute", "export N (x)\n", D, None),
        vec![
            Verdict::Admitted,
            Verdict::FileProvider,
            Verdict::FileProvider,
            Verdict::FileProvider,
        ]
    );
}

/// Promise class: durable invariant. MEASURED: D imports A as K and uses
/// `K.P.x` through P's actual public interface; the two distinct x IDs are
/// compared. CLAIMED: a nested facade publishes its selected child into P,
/// rather than requiring the unrelated `import A (x)` spelling. THE GAP: the
/// root-level tests above cover both declaration orders and cold selection.
#[test]
fn nested_facade_reaches_the_consumer_through_p_interface() {
    let a = "module P { module N { pub const x : Nat = Zero } export N (x) }\n";
    let d = "import A as K\npub const y : Nat = K.P.x\n";
    let root = FixtureRoot::new(
        "nested",
        &[
            ("N.ken", N),
            ("A.ken", a),
            ("D.ken", d),
            ("C1.ken", C1),
            ("C2.ken", C2),
        ],
    );
    for entry in ENTRIES {
        assert_eq!(
            observe(&root, entry, a, "export N (x)", "D.y", Some("A.P.N.x")),
            if entry == "A" {
                Verdict::Admitted
            } else {
                Verdict::InlineProvider
            },
            "nested entry {entry}"
        );
    }
}
