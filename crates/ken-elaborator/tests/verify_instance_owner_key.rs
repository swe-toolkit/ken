//! Instance-owned allocations and obligations at the checked-core package
//! boundary (spec/40-runtime/46-checked-core-package.md §3.2).

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Level, Term};

const PKG: &str = "zz_adv_ownerkey";
const C: &str =
    "class Lbl carrier { label : String }\ndata A = MkA\ndata B = MkB\nconst k : Nat = Zero\n";
const A: &str = "instance Lbl A { label = \"aaa\" }\n";
const B: &str = "instance Lbl B { label = \"bbb\" }\n";
const C_HOLE: &str = "class Lbl carrier { label : String ; mark : carrier -> Int }\ndata A = MkA\ndata B = MkB\nconst need : Int requires Equal Int 0 1 = 0\nconst k : Nat = Zero\n";
const A_HOLE: &str = "instance Lbl A { label = \"aaa\" ; mark = \\x. need }\n";
const B_HOLE: &str = "instance Lbl B { label = \"bbb\" ; mark = \\x. need }\n";
const STRUCT_CLASS: &str = "class Lbl carrier { label : String }\nconst k : Nat = Zero\n";
const STRUCT_NAT: &str = "instance Lbl (Nat -> Nat) { label = \"aaa\" }\n";
const STRUCT_BOOL: &str = "instance Lbl (Bool -> Bool) { label = \"bbb\" }\n";
const STRUCT_CLASS_HOLE: &str = "class Lbl carrier { label : String ; mark : carrier -> Int }\nconst need : Int requires Equal Int 0 1 = 0\nconst k : Nat = Zero\n";
const STRUCT_NAT_HOLE: &str = "instance Lbl (Nat -> Nat) { label = \"aaa\" ; mark = \\x. need }\n";
const STRUCT_BOOL_HOLE: &str =
    "instance Lbl (Bool -> Bool) { label = \"bbb\" ; mark = \\x. need }\n";

fn decl(name: &str) -> StableSymbol {
    StableSymbol::declaration(PKG, &[], name)
}

fn package(sources: &[(&str, &str)]) -> CheckedCorePackage {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, vec![]),
        sources
            .iter()
            .map(|(path, text)| CompilerSource::new(*path, *text))
            .collect(),
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.into()]),
            symbol: decl("k"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("named-head instance source admits and emits")
    .package
}

/// Promise class: durable invariant. MEASURED: actual emitted semantic maps,
/// package hash and B-owned literal/dictionary entries across c,a,b, c,b,a
/// and c,b. CLAIMED: unrelated same-class instances cannot rename another
/// instance's unnamed checked declarations. THE GAP: these heads are named;
/// the structural-head sibling below covers admitted arrows separately.
#[test]
fn named_head_literals_have_instance_owned_keys_in_both_orders() {
    let ab = package(&[("src/c.ken", C), ("src/a.ken", A), ("src/b.ken", B)]);
    let ba = package(&[("src/c.ken", C), ("src/b.ken", B), ("src/a.ken", A)]);
    let b = package(&[("src/c.ken", C), ("src/b.ken", B)]);
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
    let owner_a = decl("Lbl_instance_A#0");
    let owner_b = decl("Lbl_instance_B#0");
    for semantic in [&ab.artifact.semantic, &ba.artifact.semantic] {
        assert!(semantic.symbols.contains(&owner_a));
        assert!(semantic.symbols.contains(&owner_b));
        assert!(semantic.declarations.contains_key(&decl("Lbl_instance_A")));
        assert!(semantic.declarations.contains_key(&decl("Lbl_instance_B")));
    }
    assert!(b.artifact.semantic.symbols.contains(&owner_b));
    let dictionary = decl("Lbl_instance_B");
    assert_eq!(
        ab.artifact.semantic.declarations.get(&dictionary),
        b.artifact.semantic.declarations.get(&dictionary),
        "B's checked dictionary must not change when A is added first",
    );
}

/// Promise class: durable invariant. MEASURED: two live method-call holes
/// survive as two distinct stable obligation IDs with the right dictionary
/// origins and trust keys in both source orders; B's entries also agree with
/// c,b alone. CLAIMED: a class cannot overwrite another instance's reported
/// premise. THE GAP: the two instances share one class and one open call each;
/// the structural-head sibling below is independent; multiple calls per
/// method are not covered.
#[test]
fn named_head_requires_holes_keep_both_obligations_and_trust_keys() {
    let ab = package(&[
        ("src/c.ken", C_HOLE),
        ("src/a.ken", A_HOLE),
        ("src/b.ken", B_HOLE),
    ]);
    let ba = package(&[
        ("src/c.ken", C_HOLE),
        ("src/b.ken", B_HOLE),
        ("src/a.ken", A_HOLE),
    ]);
    let b = package(&[("src/c.ken", C_HOLE), ("src/b.ken", B_HOLE)]);
    for (order, semantic) in [
        ("c,a,b", &ab.artifact.semantic),
        ("c,b,a", &ba.artifact.semantic),
    ] {
        assert_eq!(
            semantic.obligations.len(),
            2,
            "{order}: both live obligations"
        );
        assert_eq!(
            semantic.obligation_metadata.len(),
            2,
            "{order}: both origins"
        );
        for head in ["A", "B"] {
            let owner = format!("Lbl_instance_{head}");
            let obligation = StableSymbol::obligation(format!("{owner}.requires.0"));
            let hole = decl(&format!("{owner}#1"));
            assert!(semantic.obligations.contains_key(&obligation));
            assert_eq!(
                semantic.obligation_metadata[&obligation].origin,
                decl(&owner)
            );
            assert!(semantic.trusted_base_delta.contains_key(&hole));
        }
    }
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
    let semantic = &ab.artifact.semantic;
    let obligation = StableSymbol::obligation("Lbl_instance_B.requires.0");
    let hole = decl("Lbl_instance_B#1");
    assert_eq!(
        semantic.obligations.get(&obligation),
        b.artifact.semantic.obligations.get(&obligation)
    );
    assert_eq!(
        semantic.obligation_metadata.get(&obligation),
        b.artifact.semantic.obligation_metadata.get(&obligation)
    );
    assert_eq!(
        semantic.trusted_base_delta.get(&hole),
        b.artifact.semantic.trusted_base_delta.get(&hole)
    );
}

/// Promise class: durable invariant. MEASURED: `derive E for A` and an
/// explicit `instance E B` each return the canonical dictionary identity as
/// their checked result, even though both resolve the same class name.
/// CLAIMED: both dictionary-producing declaration kinds use one identity
/// source. THE GAP: this derives an empty-class record, with no method hole.
#[test]
fn derive_and_explicit_instance_results_use_their_dictionary_identities() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env
        .elaborate_file_v1(
            "class E a { }\ndata A = MkA\ndata B = MkB\nderive E for A\ninstance E B { }\n",
        )
        .expect("derive and explicit instance elaborate");
    for owner in ["E_instance_A", "E_instance_B"] {
        let id = env.globals[owner];
        assert_eq!(
            results
                .iter()
                .find(|result| result.def_id == id)
                .unwrap()
                .name,
            owner,
        );
    }
}

/// Promise class: transition sentinel until module scoping admits these
/// unqualified declaration kinds across different modules. MEASURED: their
/// same-spelling pairs hit typed `DuplicateDefinition` before `with_owner`.
/// CLAIMED: today these five kinds cannot produce an admitted shared owner.
/// THE GAP: when module scoping starts admitting such a pair, retire this
/// refusal and give each admitted declaration its own qualified owner.
#[test]
fn other_unqualified_kinds_refuse_same_spelling_in_two_modules() {
    for (name, declaration) in [
        ("D", "class D a { }"),
        ("L", "law L (v) { witness : Top }"),
        ("f", "foreign f : Int -> Int = \"f\" \"fixture\" pure"),
        ("t", "temporal t { always True }"),
        ("p", "prove p : Top"),
    ] {
        let source = format!("module M {{ {declaration} }}\nmodule N {{ {declaration} }}\n");
        let error = ElabEnv::new()
            .unwrap()
            .elaborate_file_v1(&source)
            .expect_err("same unqualified name must not be admitted twice");
        assert!(
            matches!(error, ElabError::DuplicateDefinition { name: ref duplicate, .. } if duplicate == name),
            "{name}: {error:?}"
        );
    }
}

/// Promise class: durable invariant (33 §5 and 46 §3.2). MEASURED: the
/// emitted dictionary declarations and literal keys for two admitted arrow
/// heads survive either order, and B's dictionary is unchanged when A arrives.
/// CLAIMED: different structural heads own different stable declaration and
/// allocation identities. THE GAP: this fixes two arrow heads of one class,
/// not every nested type constructor or identical-head property duplicates.
#[test]
fn structural_head_literals_have_distinct_owners_in_both_orders() {
    let ab = package(&[
        ("src/c.ken", STRUCT_CLASS),
        ("src/a.ken", STRUCT_NAT),
        ("src/b.ken", STRUCT_BOOL),
    ]);
    let ba = package(&[
        ("src/c.ken", STRUCT_CLASS),
        ("src/b.ken", STRUCT_BOOL),
        ("src/a.ken", STRUCT_NAT),
    ]);
    let b = package(&[("src/c.ken", STRUCT_CLASS), ("src/b.ken", STRUCT_BOOL)]);
    let nat = "Lbl_instance_(arr Nat Nat)";
    let bool_head = "Lbl_instance_(arr Bool Bool)";
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
    for semantic in [&ab.artifact.semantic, &ba.artifact.semantic] {
        for owner in [nat, bool_head] {
            assert!(semantic.declarations.contains_key(&decl(owner)), "{owner}");
            assert!(semantic.symbols.contains(&decl(&format!("{owner}#0"))));
        }
        assert!(!semantic.declarations.contains_key(&decl("Lbl#2")));
        assert!(!semantic.declarations.contains_key(&decl("Lbl_instance_->")));
    }
    assert_eq!(
        ab.artifact.semantic.declarations.get(&decl(bool_head)),
        b.artifact.semantic.declarations.get(&decl(bool_head)),
        "B's checked dictionary must not move when A is inserted first",
    );
}

/// Promise class: durable invariant (22 §1 and 46 §3.2). MEASURED: two live
/// requires holes for distinct structural heads produce two stable clause IDs,
/// two origins and two trust entries in both orders, matching B alone.
/// CLAIMED: a structural instance owns its own reported premise. THE GAP:
/// one class with one open method call per instance does not cover every
/// instance field or property-class duplicate.
#[test]
fn structural_head_requires_holes_keep_both_obligations_and_trust_keys() {
    let ab = package(&[
        ("src/c.ken", STRUCT_CLASS_HOLE),
        ("src/a.ken", STRUCT_NAT_HOLE),
        ("src/b.ken", STRUCT_BOOL_HOLE),
    ]);
    let ba = package(&[
        ("src/c.ken", STRUCT_CLASS_HOLE),
        ("src/b.ken", STRUCT_BOOL_HOLE),
        ("src/a.ken", STRUCT_NAT_HOLE),
    ]);
    let b = package(&[
        ("src/c.ken", STRUCT_CLASS_HOLE),
        ("src/b.ken", STRUCT_BOOL_HOLE),
    ]);
    for (order, semantic) in [
        ("c,a,b", &ab.artifact.semantic),
        ("c,b,a", &ba.artifact.semantic),
    ] {
        assert_eq!(
            semantic.obligations.len(),
            2,
            "{order}: both live obligations"
        );
        assert_eq!(
            semantic.obligation_metadata.len(),
            2,
            "{order}: both origins"
        );
        for owner in ["Lbl_instance_(arr Nat Nat)", "Lbl_instance_(arr Bool Bool)"] {
            let obligation = StableSymbol::obligation(format!("{owner}.requires.0"));
            let hole = decl(&format!("{owner}#1"));
            assert!(semantic.declarations.contains_key(&decl(owner)));
            assert!(semantic.obligations.contains_key(&obligation));
            assert_eq!(
                semantic.obligation_metadata[&obligation].origin,
                decl(owner)
            );
            assert!(semantic.trusted_base_delta.contains_key(&hole));
        }
    }
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
    let obligation = StableSymbol::obligation("Lbl_instance_(arr Bool Bool).requires.0");
    let hole = decl("Lbl_instance_(arr Bool Bool)#1");
    assert_eq!(
        ab.artifact.semantic.obligations.get(&obligation),
        b.artifact.semantic.obligations.get(&obligation)
    );
    assert_eq!(
        ab.artifact.semantic.obligation_metadata.get(&obligation),
        b.artifact.semantic.obligation_metadata.get(&obligation)
    );
    assert_eq!(
        ab.artifact.semantic.trusted_base_delta.get(&hole),
        b.artifact.semantic.trusted_base_delta.get(&hole)
    );
}

/// Promise class: durable invariant (33 §5). MEASURED: admissible dependent,
/// sigma, effect-arrow, application, bound-variable, universe, and truncation
/// heads use tagged canonical globals keys and checked result names. CLAIMED:
/// these head constructors use the resolved whole-head rendering, not a leaf
/// or an allocation counter. THE GAP: one fixture per selected constructor
/// cannot exhaust all nesting patterns or import qualification variants.
#[test]
fn admitted_structural_forms_use_tagged_global_and_result_names() {
    for (source, owner) in [
        (
            "class E carrier { label : String }\ninstance E ((x : Nat) -> Nat) { label = \"p\" }\n",
            "E_instance_(pi Nat Nat)",
        ),
        (
            "class E carrier { label : String }\ninstance E ((x : Nat) × Nat) { label = \"s\" }\n",
            "E_instance_(sigma Nat Nat)",
        ),
        (
            "class E carrier { label : String }\ninstance E (Nat ->[IO] Bool) { label = \"e\" }\n",
            "E_instance_(arr Nat Bool)",
        ),
        (
            "class E carrier { label : String }\ninstance E (List Nat -> List Bool) { label = \"a\" }\n",
            "E_instance_(arr (app List Nat) (app List Bool))",
        ),
        (
            "class E carrier { label : String }\ninstance E (a -> List a) { label = \"v\" }\n",
            "E_instance_(arr (var 0) (app List (var 0)))",
        ),
        (
            "class E (carrier : Type 1) { }\ninstance E Type { }\n",
            "E_instance_(type)",
        ),
        (
            "class E (p : Omega) { }\ninstance E ‖Bool‖ { }\n",
            "E_instance_(trunc Bool)",
        ),
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        let results = env
            .elaborate_file_v1(source)
            .unwrap_or_else(|error| panic!("{owner}: {error:?}"));
        let id = env.globals[owner];
        assert!(
            results
                .iter()
                .any(|result| result.def_id == id && result.name == owner),
            "{owner}"
        );
    }
}

/// Promise class: durable invariant (33 §5). MEASURED: two property-class
/// instances with one identical structural head remain admitted, while a
/// structure class refuses an overlapping arrow/Pi pair before registration.
/// CLAIMED: the symbol-reuse guard preserves canonical coherence behavior.
/// THE GAP: these are one duplicate and one overlap; a deliberate adversarial
/// non-injective renderer is required to exercise the guard's refusal arm.
#[test]
fn structural_symbol_reuse_preserves_property_and_structure_coherence() {
    let mut property = ElabEnv::new().expect("prelude");
    property
        .elaborate_file_v1(
            "class E carrier { }\ninstance E (Nat -> Nat) { }\ninstance E (Nat -> Nat) { }\n",
        )
        .expect("property-class duplicates remain admitted");
    assert!(property.globals.contains_key("E_instance_(arr Nat Nat)"));

    let error = ElabEnv::new()
        .expect("prelude")
        .elaborate_file_v1(
            "class E carrier { label : String }\ninstance E (Nat -> Nat) { label = \"x\" }\ninstance E ((x : Nat) -> Nat) { label = \"y\" }\n",
        )
        .expect_err("structure-class overlap must refuse before registration");
    assert!(matches!(error, ElabError::OverlappingInstances { class, .. } if class == "E"));
}

/// Promise class: durable invariant (46 §3.2). MEASURED: a preexisting
/// distinct globals binding at a rendered structural name refuses instead of
/// silently being replaced by a new instance. CLAIMED: any future collision
/// at that binding is loud. THE GAP: this raw setup does not demonstrate a
/// source-spellable collision; structural names contain parentheses.
#[test]
fn structural_symbol_reuse_guard_refuses_a_different_existing_global() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("class E carrier { }")
        .expect("property class");
    let name = "E_instance_(arr Nat Nat)";
    let previous = env
        .declare_postulate_raw(name, Term::ty(Level::Zero))
        .expect("preexisting distinct binding");
    let error = env
        .elaborate_decl("instance E (Nat -> Nat) { }")
        .expect_err("a distinct global cannot be overwritten");
    assert!(
        matches!(error, ElabError::Internal(ref message) if message == &format!("instance symbol `{name}` already names a different instance head"))
    );
    assert_eq!(env.globals[name], previous);
}
