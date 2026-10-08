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
/// without a loud error. THE GAP: a later user-spelled or constructor binding
/// is outside this guard; successor residual rows below make that explicit.
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
                    continue; // Later user-spelled const: successor residual, not a refusal.
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

/// Promise class: transition sentinel until VERIFY-GLOBALS-IDENTITY-CHECKED-
/// INSERT changes user-spelled insertion. MEASURED: the reverse R2 order is
/// still admitted in each file layout with only one obligation, an ordinal
/// dictionary fallback, and the pre-guard core hash. CLAIMED: this residual
/// is visible, not a success of the minted-key guard. THE GAP: the other
/// order now refuses and has no post-repair hash to compare; the historical
/// `ba` hash differs (evt_15k31thzp9j03). Retire at successor landing.
#[test]
fn successor_residual_user_const_after_minted_dictionary_still_overwrites() {
    for layout in 1..=3 {
        let package = compile(&sources(R2_PRE, R2_A, R2_B, layout))
            .expect("later user-spelled const still overwrites minted binding");
        let semantic = &package.artifact.semantic;
        assert_eq!(semantic.obligations.len(), 1, "{layout} file(s)");
        assert_eq!(semantic.obligation_metadata.len(), 1, "{layout} file(s)");
        assert!(semantic.declarations.contains_key(&decl("Lbl_instance_A")));
        assert!(
            semantic
                .declarations
                .contains_key(&decl("Lbl_instance_A#1"))
        );
        assert_eq!(package.core_semantic_hash, 0x6329_57b6_b846_ba73);
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
/// second) is the separate successor residual, not a protected path.
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

/// Promise class: transition sentinel until VERIFY-GLOBALS-IDENTITY-CHECKED-
/// INSERT routes later user-spelled insertions. MEASURED: a class, data
/// declaration or const checked after a named dictionary still replaces that
/// flat globals key. CLAIMED: this is the remaining insertion-side defect.
/// THE GAP: same-spelling constructors were measured separately in AC-0.
#[test]
fn successor_residual_user_declarations_after_dictionary_still_rebind() {
    let pre = "class E a { }\ndata A = MkA\n";
    for (dictionary, other) in [
        ("instance E A { }", "class E_instance_A b { }"),
        ("instance E A { }", "data E_instance_A = MkE"),
        ("derive E for A", "const E_instance_A : Int = 0"),
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        let first = env
            .elaborate_file_v1(&format!("{pre}{dictionary}\n"))
            .expect("dictionary initially admitted");
        let old_id = first.last().expect("dictionary result").def_id;
        assert_eq!(env.globals["E_instance_A"], old_id);
        let later = env
            .elaborate_decl_v1(other)
            .expect("later user-spelled declaration currently admits");
        assert_ne!(later.def_id, old_id);
        assert_eq!(env.globals["E_instance_A"], later.def_id);
    }
}

/// Promise class: durable invariant (33 §5). MEASURED: on the same env a
/// second `data Foo` with a different checked ID admits, while a second
/// `instance Pick Foo` for that rebound head hits the minted dictionary key
/// and refuses with the typed error. CLAIMED: lawful user shadowing remains
/// possible without letting a newly minted identity overwrite old checked
/// dictionaries. THE GAP: a later user declaration still overwrites a
/// dictionary, routed to the successor rather than this guard.
#[test]
fn rebound_user_data_admits_but_rebound_head_instance_refuses() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file_v1(
        "class Pick carrier { selected : Bool }\ndata Foo : Type where { MkOldFoo : Foo }\ninstance Pick Foo { selected = True }\n",
    )
    .expect("old class, data and instance check");
    let old_head = env.globals["Foo"];
    let old_dictionary = env.globals["Pick_instance_Foo"];
    let new_head = env
        .elaborate_decl("data Foo : Type where { MkNewFoo : Foo }")
        .expect("later user-spelled `data Foo` is lawful shadowing");
    assert_ne!(old_head, new_head);
    assert_eq!(env.globals["Foo"], new_head);
    let error = env
        .elaborate_decl("instance Pick Foo { selected = False }")
        .expect_err("same minted dictionary identity cannot replace old instance");
    assert!(matches!(
        error,
        ElabError::DeclarationIdentityCollision {
            ref identity,
            ref first,
            ref second,
            ..
        } if identity == "Pick_instance_Foo" && first == "instance Pick Foo" && second == "instance Pick Foo"
    ));
    assert_eq!(env.globals["Pick_instance_Foo"], old_dictionary);
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
