//! Checked-core identity collision at a named instance's importable dictionary
//! name (spec/30-surface/33-declarations.md §5, 46-checked-core-package §3.2).

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerDriverError, CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
use ken_elaborator::{ElabEnv, ElabError};

const PKG: &str = "zz_adv_ownerkey";
const R1_PRE: &str = "class A carrier { la : carrier -> Int }\nclass A_instance_B carrier { lb : carrier -> Int }\ndata B_instance_C : Type where { MkBC : B_instance_C }\ndata C : Type where { MkC : C }\nconst need : Int requires Equal Int 0 1 = 0\nconst k : Nat = Zero\n";
const R1_A: &str = "instance A B_instance_C { la = \\x. need }\n";
const R1_B: &str = "instance A_instance_B C { lb = \\x. need }\n";
const R1_B_DISTINCT: &str = "instance A_instance_B B_instance_C { lb = \\x. need }\n";
const R2_PRE: &str = "class Lbl carrier { mark : carrier -> Int }\ndata A = MkA\nconst need : Int requires Equal Int 0 1 = 0\nconst k : Nat = Zero\n";
const R2_A: &str = "instance Lbl A { mark = \\x. need }\n";
const R2_B: &str = "const Lbl_instance_A : Int = need\n";
const R2_B_DISTINCT: &str = "const Lbl_instance_A_extra : Int = need\n";

fn decl(name: &str) -> StableSymbol {
    StableSymbol::declaration(PKG, &[], name)
}

fn compile(sources: &[(&str, String)]) -> Result<CheckedCorePackage, CompilerDriverError> {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, vec![]),
        sources
            .iter()
            .map(|(path, source)| CompilerSource::new(*path, source.as_str()))
            .collect(),
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.into()]),
            symbol: decl("k"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .map(|output| output.package)
}

fn sources(pre: &str, first: &str, second: &str, layout: u8) -> Vec<(&'static str, String)> {
    match layout {
        1 => vec![("src/c.ken", format!("{pre}{first}{second}"))],
        2 => vec![
            ("src/c.ken", format!("{pre}{first}")),
            ("src/other.ken", second.into()),
        ],
        3 => vec![
            ("src/c.ken", pre.into()),
            ("src/a.ken", first.into()),
            ("src/b.ken", second.into()),
        ],
        _ => unreachable!("only one-, two- and three-file layouts"),
    }
}

/// Promise class: durable invariant (33 §5; checked-core §3.2). MEASURED:
/// six R1 rows and three const-before-instance R2 rows refuse with the same
/// typed, sorted declaration descriptions across three file layouts. CLAIMED:
/// an elaborator-minted name cannot replace an unrelated checked identity
/// without a loud error. THE GAP: reverse-order user-spelled insertions and
/// constructors have separate controls in this suite and the successor.
#[test]
fn minted_named_identity_refuses_all_nine_collision_rows() {
    let mut failures = Vec::new();
    let mut checked = 0;
    for (case, pre, a, b, identity, expected_first, expected_second) in [
        (
            "R1",
            R1_PRE,
            R1_A,
            R1_B,
            "A_instance_B_instance_C",
            "instance A B_instance_C",
            "instance A_instance_B C",
        ),
        (
            "R2",
            R2_PRE,
            R2_A,
            R2_B,
            "Lbl_instance_A",
            "const Lbl_instance_A",
            "instance Lbl A",
        ),
    ] {
        for layout in 1..=3 {
            for (order, first_source, second_source) in [("ab", a, b), ("ba", b, a)] {
                if case == "R2" && order == "ab" {
                    continue; // The opposite R2 order is tested separately below.
                }
                checked += 1;
                let label = format!("{case}: {layout} file(s), {order}");
                match compile(&sources(pre, first_source, second_source, layout)) {
                    Err(CompilerDriverError::Elaboration(
                        ElabError::DeclarationIdentityCollision {
                            identity: found,
                            first,
                            second,
                            ..
                        },
                    )) if found == identity
                        && first == expected_first
                        && second == expected_second => {}
                    Ok(package) => failures.push(format!(
                        "{label}: admitted with {} obligations, #1 fallback={}, hash={:#x}",
                        package.artifact.semantic.obligations.len(),
                        package
                            .artifact
                            .semantic
                            .declarations
                            .contains_key(&decl(identity))
                            && package
                                .artifact
                                .semantic
                                .declarations
                                .contains_key(&decl(&format!("{identity}#1"))),
                        package.core_semantic_hash
                    )),
                    Err(error) => failures.push(format!("{label}: wrong error {error:?}")),
                }
            }
        }
    }
    assert_eq!(
        checked, 9,
        "six R1 rows and three R2 rows must be exercised"
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Promise class: durable invariant (33 §5; checked-core §3.2). MEASURED:
/// a user-spelled const following a checked dictionary refuses in one-, two-
/// and three-file layouts, naming both identities. CLAIMED: reversing R2
/// cannot silently displace the dictionary's flat key. THE GAP: only the
/// const-after-instance direction is tested here; the predecessor's nine
/// rows separately cover the opposite direction and R1.
#[test]
fn user_const_after_minted_dictionary_refuses_in_every_layout() {
    for layout in 1..=3 {
        match compile(&sources(R2_PRE, R2_A, R2_B, layout)) {
            Err(CompilerDriverError::Elaboration(
                ElabError::DeclarationIdentityCollision { identity, first, second, .. },
            )) => {
                assert_eq!(identity, "Lbl_instance_A", "{layout} file(s)");
                assert_eq!(first, "const Lbl_instance_A", "{layout} file(s)");
                assert_eq!(second, "instance Lbl A", "{layout} file(s)");
            }
            other => panic!("{layout} file(s): wrong result {other:?}"),
        }
    }
}

/// Promise class: durable invariant (33 §5; checked-core §3.2). MEASURED:
/// changing the second head to a distinct canonical dictionary name leaves
/// two independently owned premises and identical semantic maps/hashes in
/// both declaration orders. CLAIMED: refusal is about actual identity-key
/// collisions, not blanket rejection of two live instance obligations. THE
/// GAP: one non-colliding pair and one method-call premise per dictionary.
#[test]
fn distinct_named_instance_identities_keep_both_obligations_and_hash() {
    let ab = compile(&sources(R1_PRE, R1_A, R1_B_DISTINCT, 3)).expect("ab admits");
    let ba = compile(&sources(R1_PRE, R1_B_DISTINCT, R1_A, 3)).expect("ba admits");
    let owners = [
        "A_instance_B_instance_C",
        "A_instance_B_instance_B_instance_C",
    ];
    for package in [&ab, &ba] {
        let semantic = &package.artifact.semantic;
        assert_eq!(semantic.obligations.len(), 2);
        assert_eq!(semantic.obligation_metadata.len(), 2);
        for owner in owners {
            assert!(semantic.declarations.contains_key(&decl(owner)));
            assert!(
                !semantic
                    .declarations
                    .contains_key(&decl(&format!("{owner}#1")))
            );
            let obligation = StableSymbol::obligation(format!("{owner}.requires.0"));
            assert!(semantic.obligations.contains_key(&obligation));
            assert_eq!(
                semantic.obligation_metadata[&obligation].origin,
                decl(owner)
            );
        }
    }
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
}

/// Promise class: durable invariant (33 §5). MEASURED: an instance or
/// `derive` cannot mint its dictionary identity onto a preexisting class,
/// data declaration or const. CLAIMED: the canonical importable binding is
/// exclusive when minted. THE GAP: the reverse order (user-spelled binding
/// second) is asserted in a separate test below.
#[test]
fn minted_dictionary_refuses_a_prior_class_type_or_const_identity() {
    let pre = "class E a { }\ndata A = MkA\n";
    for (dictionary, other, expected_other) in [
        (
            "instance E A { }\n",
            "class E_instance_A b { }\n",
            "class E_instance_A",
        ),
        (
            "instance E A { }\n",
            "data E_instance_A = MkE\n",
            "data E_instance_A",
        ),
        (
            "derive E for A\n",
            "const E_instance_A : Int = 0\n",
            "const E_instance_A",
        ),
    ] {
        let expected_dictionary = if dictionary.starts_with("derive") {
            "derive E for A"
        } else {
            "instance E A"
        };
        let mut expected = [expected_dictionary, expected_other];
        expected.sort();
        let mut env = ElabEnv::new().expect("prelude");
        let error = env
            .elaborate_file_v1(&format!("{pre}{other}{dictionary}"))
            .expect_err("the minted dictionary cannot replace a checked user binding");
        match error {
            ElabError::DeclarationIdentityCollision {
                identity,
                first,
                second,
                ..
            } => {
                assert_eq!(identity, "E_instance_A");
                assert_eq!([first.as_str(), second.as_str()], expected);
            }
            other => panic!("wrong error {other:?}"),
        }
    }
}

/// Promise class: durable invariant (33 §5). MEASURED: a later class, data
/// former or const cannot displace a minted dictionary and leaves the prior
/// checked key bound. CLAIMED: declaration identity is independent of which
/// source kind comes second. THE GAP: constructor, law-field and space-op
/// collisions are exercised by the successor's independent suite.
#[test]
fn user_declarations_after_dictionary_refuse_and_preserve_prior_key() {
    let pre = "class E a { }\ndata A = MkA\n";
    for (dictionary, other, expected_dictionary, expected_other) in [
        ("instance E A { }", "class E_instance_A b { }", "instance E A", "class E_instance_A"),
        ("instance E A { }", "data E_instance_A = MkE", "instance E A", "data E_instance_A"),
        ("derive E for A", "const E_instance_A : Int = 0", "derive E for A", "const E_instance_A"),
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        let first = env
            .elaborate_file_v1(&format!("{pre}{dictionary}\n"))
            .expect("dictionary initially admitted");
        let old_id = first.last().expect("dictionary result").def_id;
        assert_eq!(env.globals["E_instance_A"], old_id);
        let error = env.elaborate_decl_v1(other)
            .expect_err("a user-spelled declaration may not replace a minted dictionary");
        let mut expected = [expected_dictionary, expected_other];
        expected.sort();
        match error {
            ElabError::DeclarationIdentityCollision { identity, first, second, .. } => {
                assert_eq!(identity, "E_instance_A");
                assert_eq!([first.as_str(), second.as_str()], expected);
            }
            other => panic!("wrong error {other:?}"),
        }
        assert_eq!(env.globals["E_instance_A"], old_id);
    }
}

/// Promise class: durable invariant (33 §5.1). MEASURED: multiple checked
/// dictionaries with exactly the same class/head key remain admitted when
/// the class is proof-irrelevant, including either order of `derive` and an
/// explicit instance. CLAIMED: the collision guard preserves the sole
/// permitted same-key exemption. THE GAP: this is a property class with an
/// empty record; structure-class same-key overlap is covered separately.
#[test]
fn property_class_same_head_duplicates_remain_admitted() {
    for declarations in [
        "instance E A { }\ninstance E A { }\n",
        "derive E for A\ninstance E A { }\n",
        "instance E A { }\nderive E for A\n",
    ] {
        let mut env = ken_elaborator::ElabEnv::new().expect("prelude");
        let results = env
            .elaborate_file_v1(&format!("class E a {{ }}\ndata A = MkA\n{declarations}"))
            .expect("proof-irrelevant duplicate of the same checked instance key");
        assert_eq!(
            results
                .iter()
                .filter(|result| result.name == "E_instance_A")
                .count(),
            2
        );
        assert!(env.globals.contains_key("E_instance_A"));
    }
}

/// Promise class: durable invariant (33 §5; checked-core §3.2). MEASURED:
/// changing the user const spelling to a distinct identity preserves its
/// premise beside the instance's premise with no ordinal dictionary fallback
/// and an order-independent hash. CLAIMED: users retain a legal non-colliding
/// importable binding. THE GAP: this uses a `const`, not every declaration
/// kind that can take a dictionary spelling.
#[test]
fn distinct_const_and_instance_identities_keep_both_obligations_and_hash() {
    let ab = compile(&sources(R2_PRE, R2_A, R2_B_DISTINCT, 3)).expect("ab admits");
    let ba = compile(&sources(R2_PRE, R2_B_DISTINCT, R2_A, 3)).expect("ba admits");
    for package in [&ab, &ba] {
        let semantic = &package.artifact.semantic;
        assert_eq!(semantic.obligations.len(), 2);
        assert_eq!(semantic.obligation_metadata.len(), 2);
        assert!(semantic.declarations.contains_key(&decl("Lbl_instance_A")));
        assert!(
            semantic
                .declarations
                .contains_key(&decl("Lbl_instance_A_extra"))
        );
        assert!(
            !semantic
                .declarations
                .contains_key(&decl("Lbl_instance_A#1"))
        );
        for owner in ["Lbl_instance_A", "Lbl_instance_A_extra"] {
            let obligation = StableSymbol::obligation(format!("{owner}.requires.0"));
            assert!(semantic.obligations.contains_key(&obligation));
            assert_eq!(
                semantic.obligation_metadata[&obligation].origin,
                decl(owner)
            );
        }
    }
    assert_eq!(ab.artifact.semantic, ba.artifact.semantic);
    assert_eq!(ab.core_semantic_hash, ba.core_semantic_hash);
}
