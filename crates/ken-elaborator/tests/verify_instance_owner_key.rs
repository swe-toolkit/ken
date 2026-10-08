//! Instance-owned allocations and obligations at the checked-core package
//! boundary (spec/40-runtime/46-checked-core-package.md §3.2).

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
use ken_elaborator::{ElabEnv, ElabError};

const PKG: &str = "zz_adv_ownerkey";
const C: &str =
    "class Lbl carrier { label : String }\ndata A = MkA\ndata B = MkB\nconst k : Nat = Zero\n";
const A: &str = "instance Lbl A { label = \"aaa\" }\n";
const B: &str = "instance Lbl B { label = \"bbb\" }\n";
const C_HOLE: &str = "class Lbl carrier { label : String ; mark : carrier -> Int }\ndata A = MkA\ndata B = MkB\nconst need : Int requires Equal Int 0 1 = 0\nconst k : Nat = Zero\n";
const A_HOLE: &str = "instance Lbl A { label = \"aaa\" ; mark = \\x. need }\n";
const B_HOLE: &str = "instance Lbl B { label = \"bbb\" ; mark = \\x. need }\n";

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
/// structural heads are a separate successor WP, not covered by this claim.
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
/// this does not cover structural-head instances or multiple calls per method.
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
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
    let semantic = &ab.artifact.semantic;
    assert_eq!(semantic.obligations.len(), 2);
    assert_eq!(semantic.obligation_metadata.len(), 2);
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
        assert!(ba.artifact.semantic.trusted_base_delta.contains_key(&hole));
        if head == "B" {
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
    }
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

/// Promise class: transition sentinel for the successor
/// VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY. MEASURED: two distinct admitted
/// arrow heads keep their pre-WP class-owned fallback symbol and shared
/// synthesized spelling. CLAIMED: this named-head WP does not silently
/// rename structural-head declarations. THE GAP: the successor must retire
/// this sentinel when it gives structural heads distinct canonical names.
#[test]
fn structural_head_instances_keep_their_existing_symbols() {
    const PKG_STRUCT: &str = "zz_owner_struct";
    let source = "class Lbl carrier { label : String }\ninstance Lbl (Nat -> Nat) { label = \"x\" }\ninstance Lbl (Bool -> Bool) { label = \"y\" }\nconst k : Nat = Zero\n";
    let package = compile_ken_package_sources(
        &CompilerManifest::new(PKG_STRUCT, vec![]),
        vec![CompilerSource::new("src/c.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG_STRUCT.into()]),
            symbol: StableSymbol::declaration(PKG_STRUCT, &[], "k"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("structural heads are admitted before successor WP")
    .package;
    let sym = |name| StableSymbol::declaration(PKG_STRUCT, &[], name);
    let symbols = &package.artifact.semantic.symbols;
    for name in ["Lbl", "Lbl#1", "Lbl#2", "Lbl#3", "Lbl_instance_->"] {
        assert!(symbols.contains(&sym(name)), "missing unchanged {name}");
    }
    assert!(
        package
            .artifact
            .semantic
            .declarations
            .contains_key(&sym("Lbl#2"))
    );
    assert!(
        package
            .artifact
            .semantic
            .declarations
            .contains_key(&sym("Lbl_instance_->"))
    );
    assert!(!symbols.contains(&sym("Lbl_instance_->#0")));
}
