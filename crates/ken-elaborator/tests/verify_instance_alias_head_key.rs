//! Alias-headed instance identity, refinement boundaries, and module ownership.
//! The overlap checks assert their exact refusal, not merely any error.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::{ElabEnv, ElabError};

static FIXTURE_SERIAL: AtomicU64 = AtomicU64::new(0);

struct Roots(PathBuf);
impl Roots {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "verify-instance-alias-{}-{}",
            std::process::id(),
            FIXTURE_SERIAL.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir_all(&path).expect("create strict roots fixture");
        Self(path)
    }
    fn write(&self, module: &str, source: &str) {
        fs::write(self.0.join(format!("{module}.ken")), source).expect("write module");
    }
}
impl Drop for Roots {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const CLASS: &str = "class E carrier { label : String }\n";

fn overlapping(source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(source)
        .expect_err("same unfolded head must overlap");
    assert!(
        matches!(error, ElabError::OverlappingInstances { ref class, .. } if class == "E"),
        "expected an identity-keyed E overlap, got {error:?}"
    );
}

/// Promise class: durable identity and resolution invariant (spec 39 §6.1).
#[test]
fn plain_alias_and_named_head_overlap_in_both_orders() {
    // MEASURED: either order refuses specifically at registration. CLAIMED:
    // aliases of a named head cannot create a second structure instance.
    // THE GAP: the succeeding single-instance resolution row checks that the
    // shared key is also used at lookup rather than only at registration.
    for (first, second) in [
        (
            "instance E N2 { label = \"alias\" }",
            "instance E Nat { label = \"plain\" }",
        ),
        (
            "instance E Nat { label = \"plain\" }",
            "instance E N2 { label = \"alias\" }",
        ),
    ] {
        overlapping(&format!("{CLASS} def N2 = Nat\n{first}\n{second}\n"));
    }
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{CLASS} def N2 = Nat\ninstance E N2 {{ label = \"alias\" }}\n"
    ))
    .expect("alias dictionary admits with its source spelling");
    let dictionary = env.globals["E_instance_N2"];
    assert_ne!(dictionary, env.globals["N2"]);
    env.resolution_provenance.clear();
    env.elaborate_decl("const selected : String where E Nat = d.label")
        .expect("the target spelling selects the alias's dictionary");
    assert_eq!(
        env.resolution_provenance
            .last()
            .expect("one resolution")
            .instance_id,
        dictionary
    );
    assert!(!env.globals.contains_key("E_instance_Nat"));
}

/// Promise class: durable structural overlap invariant (spec 39 §6.1).
#[test]
fn aliases_of_structural_heads_share_one_key() {
    for (alias, concrete) in [("F", "(Nat -> Nat)"), ("(N2 -> Nat)", "(Nat -> Nat)")] {
        for (first, second) in [(alias, concrete), (concrete, alias)] {
            overlapping(&format!(
                "{CLASS} def N2 = Nat\ndef F = Nat -> Nat\n\
                 instance E {first} {{ label = \"one\" }}\n\
                 instance E {second} {{ label = \"two\" }}"
            ));
        }
    }
}

/// Promise class: durable parameterized head invariant (spec 39 §6.1).
#[test]
fn nullary_alias_of_an_applied_type_function_keeps_the_named_constructor_key() {
    // Pair's transparent type-function body is structural Sigma, but the
    // parameterized instance key is Pair itself in both declarations.
    overlapping(&format!(
        "{CLASS} def PIB = Pair Int Bool\n\
         instance E (Pair a b) {{ label = \"generic\" }}\n\
         instance E PIB {{ label = \"alias\" }}"
    ));
}

/// Promise class: durable refinement identity invariant (spec 18a §5.9.1).
#[test]
fn named_refinement_stays_distinct_and_its_alias_shares_its_root() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{CLASS} instance E Char {{ label = \"char\" }}\n\
         instance E Int {{ label = \"int\" }}\ndef C2 = Char"
    ))
    .expect("named refinement Char must not overlap carrier Int");
    assert_ne!(
        env.globals["E_instance_Char"],
        env.globals["E_instance_Int"]
    );
    let error = env
        .elaborate_decl("instance E C2 { label = \"alias\" }")
        .expect_err("an alias of Char must share Char's refinement root");
    assert!(
        matches!(error, ElabError::OverlappingInstances { ref class, .. } if class == "E"),
        "expected refinement-root overlap, got {error:?}"
    );
}

/// Promise class: durable checked dictionary selection invariant (spec 39 §6.2).
#[test]
fn cross_module_alias_instance_resolves_at_imported_named_head() {
    let roots = Roots::new();
    roots.write(
        "A",
        "pub class E carrier { label : String } \
                      pub data T = MkT \
                      def T2 = T \
                      instance E T2 { label = \"alias\" }",
    );
    roots.write(
        "B",
        "import A (E, T) \
                      const selected : String where E T = d.label",
    );
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots_strict(&[roots.0.clone()], "B")
        .expect("B must resolve A's alias-headed dictionary via T");
    let dictionary = env.globals["E_instance_A.T2"];
    assert!(env
        .resolution_provenance
        .iter()
        .any(|entry| entry.instance_id == dictionary));
    assert!(env.globals.contains_key("B.selected"));
}

/// Promise class: durable orphan ownership invariant (spec 33 §5.3).
#[test]
fn owner_alias_instance_of_imported_class_admits() {
    let roots = Roots::new();
    roots.write("C", "pub class E carrier { label : String }");
    roots.write(
        "A",
        "import C (E) pub data T = MkT def T2 = T \
                      instance E T2 { label = \"owner\" }",
    );
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots_strict(&[roots.0.clone()], "A")
        .expect("A owns T, so an instance spelled through A's alias T2 is not an orphan");
}

// AC-2 (d'): the other module cannot gain T's ownership just by naming T2.
/// Promise class: durable orphan refusal invariant (spec 33 §5.3).
#[test]
fn imported_alias_cannot_evade_orphan_rule() {
    let roots = Roots::new();
    roots.write(
        "A",
        "pub class E carrier { label : String } pub data T = MkT",
    );
    roots.write(
        "B",
        "import A (E, T) def T2 = T \
                      instance E T2 { label = \"orphan\" }",
    );
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_module_from_roots_strict(&[roots.0.clone()], "B")
        .expect_err("B owns neither class E nor unfolded head T");
    assert!(
        matches!(error, ElabError::OrphanInstance { ref class, .. } if class == "E"),
        "expected orphan refusal at the unfolded head, got {error:?}"
    );
}
