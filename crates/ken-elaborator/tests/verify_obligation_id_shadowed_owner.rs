//! VERIFY-OBLIGATION-ID-SHADOWED-OWNER. Durable invariant: a retained checked
//! owner's premise is reported under that owner's stable obligation identity.
//! Spec: 20-verification/22 §1, §2.3; 40-runtime/46 §3.2.

use std::collections::BTreeSet;

use ken_elaborator::checked_core::{
    canonical_symbol_bytes, canonical_term_bytes, CheckedCorePackage, StableSymbol,
    StableSymbolTable, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::ElabEnv;
use ken_kernel::Term;

const PKG: &str = "verify_shadowed_obligation_owner";
const PRE: &str = "class Pick carrier { sel : carrier -> Int }\nconst need1 : Int requires Equal Int 0 1 = 0\nconst need2 : Int requires Equal Int 0 2 = 0\nconst k : Nat = Zero\n";
const INSTANCE_A: &str =
    "data Foo : Type where { MkOld : Foo }\ninstance Pick Foo { sel = \\x. need1 }\n";
const INSTANCE_B: &str =
    "data Foo : Type where { MkNew : Foo }\ninstance Pick Foo { sel = \\x. need2 }\n";
const USER_A: &str = "const x : Int = need1\nconst y : Int = x\n";
const USER_B: &str = "const x : Int = need2\n";
const CLASS_A: &str =
    "data Foo : Type where { MkFoo : Foo }\ninstance Pick Foo { sel = \\x. need1 }\n";
const CLASS_B: &str =
    "class Pick carrier { sel : carrier -> Int }\ninstance Pick Foo { sel = \\x. need2 }\n";

fn decl(name: &str) -> StableSymbol {
    StableSymbol::declaration(PKG, &[], name)
}

fn compile(sources: &[(&str, &str)]) -> CheckedCorePackage {
    let inputs = std::iter::once(CompilerSource::new("src/c.ken", PRE))
        .chain(
            sources
                .iter()
                .map(|(path, source)| CompilerSource::new(*path, *source)),
        )
        .collect();
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, vec![]),
        inputs,
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.into()]),
            symbol: decl("k"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("both owners admit and produce a checked package")
    .package
}

/// The oracle is the source predicate `Equal Int 0 n`, assembled independently
/// of the package's obligation id and owner metadata. A complete canonical
/// subterm is required, not a coincidental single byte or a minted name.
fn predicate_fragment(value: i64) -> Vec<u8> {
    let env = ElabEnv::new().expect("checked Int and Equal prelude");
    let mut symbols = StableSymbolTable::new();
    for name in ["Int", "Equal"] {
        symbols.insert_global(env.globals[name], decl(name));
    }
    let predicate = Term::app(
        Term::app(
            Term::app(
                Term::const_(env.globals["Equal"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ),
            Term::IntLit(0.into()),
        ),
        Term::IntLit(value.into()),
    );
    canonical_term_bytes(&predicate, &symbols).expect("hand-built checked predicate")
}

fn contains(goal: &[u8], needle: &[u8]) -> bool {
    goal.windows(needle.len())
        .any(|fragment| fragment == needle)
}

fn assert_owner_goal(
    package: &CheckedCorePackage,
    owner: &str,
    expected_value: i64,
    other_value: i64,
) {
    let obligation = StableSymbol::obligation(format!("{owner}.requires.0"));
    let semantic = &package.artifact.semantic;
    let goal = semantic
        .obligations
        .get(&obligation)
        .expect("owned premise present");
    let expected = predicate_fragment(expected_value);
    let other = predicate_fragment(other_value);
    assert!(
        contains(goal, &expected),
        "{obligation} must carry Equal Int 0 {expected_value}"
    );
    assert!(
        !contains(goal, &other),
        "{obligation} must not carry Equal Int 0 {other_value}"
    );
    assert_eq!(
        semantic.obligation_metadata[&obligation].origin,
        decl(owner)
    );
}

fn assert_retained_reference(package: &CheckedCorePackage, user: &str, owner: &str) {
    let semantic = &package.artifact.semantic;
    let body = &semantic.declarations[&decl(user)];
    let checked_owner = canonical_symbol_bytes(&decl(owner));
    assert!(
        contains(body, &checked_owner),
        "{user} must retain a checked reference to {owner}"
    );
}

fn assert_two_owners(package: &CheckedCorePackage, name: &str, old: i64, new: i64) {
    let semantic = &package.artifact.semantic;
    let old_owner = format!("{name}#1");
    let expected: BTreeSet<_> = [
        StableSymbol::obligation(format!("{old_owner}.requires.0")),
        StableSymbol::obligation(format!("{name}.requires.0")),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        semantic
            .obligations
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected
    );
    assert_eq!(
        semantic
            .obligation_metadata
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected
    );
    assert_owner_goal(package, &old_owner, old, new);
    assert_owner_goal(package, name, new, old);
}

/// AC-1; promise class: durable invariant. MEASURED: two source orders keep
/// both checked owner ids, each with the independently pinned `Equal Int 0 n`
/// goal and matching origin. CLAIMED: a lawful rebind cannot erase a premise.
/// THE GAP: order-dependent `#1` naming is accepted only because the source
/// predicate is tested separately; identical id sets do not imply goal parity.
#[test]
fn rebound_instances_keep_two_source_premises_in_both_orders() {
    let ab = compile(&[("src/a.ken", INSTANCE_A), ("src/b.ken", INSTANCE_B)]);
    let ba = compile(&[("src/b.ken", INSTANCE_B), ("src/a.ken", INSTANCE_A)]);
    assert_two_owners(&ab, "Pick_instance_Foo", 1, 2);
    assert_two_owners(&ba, "Pick_instance_Foo", 2, 1);
}

/// AC-1; promise class: durable invariant. MEASURED: the old dictionary is
/// retained by `z`, and both old/new owned premises survive the later rebind.
/// CLAIMED: preservation is by checked owner, not by the flat dictionary key.
/// THE GAP: `z` is a retained declaration, not an execution assertion.
#[test]
fn retained_dictionary_use_keeps_its_premise_after_rebind() {
    let a = format!("{INSTANCE_A}const z : Int where Pick Foo = d.sel MkOld\n");
    let package = compile(&[("src/a.ken", &a), ("src/b.ken", INSTANCE_B)]);
    assert_retained_reference(&package, "z", "Pick_instance_Foo#1");
    assert_two_owners(&package, "Pick_instance_Foo", 1, 2);
}

/// AC-1; promise class: durable invariant. MEASURED: retained `y` and two
/// `x` premises remain distinct after a user-spelled shadow. CLAIMED: the
/// collision protection applies equally to non-dictionary source owners.
/// THE GAP: this is one user-declaration kind, beside the instance controls.
#[test]
fn retained_user_shadow_use_keeps_both_source_premises() {
    let package = compile(&[("src/a.ken", USER_A), ("src/b.ken", USER_B)]);
    assert_retained_reference(&package, "y", "x#1");
    assert_two_owners(&package, "x", 1, 2);
}

/// AC-1; promise class: durable invariant. MEASURED: rebound class owner
/// still yields two distinct dictionary goals with checked origins. CLAIMED:
/// the key follows the checked instance owner even after class rebinding.
/// THE GAP: no assertion about other class method or inheritance semantics.
#[test]
fn rebound_class_keeps_both_instance_premises() {
    let package = compile(&[("src/a.ken", CLASS_A), ("src/b.ken", CLASS_B)]);
    assert_two_owners(&package, "Pick_instance_Foo", 1, 2);
}

/// AC-2; promise class: durable invariant. MEASURED: each single-file
/// witness still has exactly its expected source predicate, ID and origin.
/// CLAIMED: disambiguation activates only for two checked owners of one name.
/// THE GAP: existing predecessor suites pin unrelated identity families.
#[test]
fn single_owner_controls_keep_unadorned_obligation_ids() {
    for (source, expected, other) in [(INSTANCE_A, 1, 2), (INSTANCE_B, 2, 1)] {
        let package = compile(&[("src/one.ken", source)]);
        assert_eq!(package.artifact.semantic.obligations.len(), 1);
        assert_owner_goal(&package, "Pick_instance_Foo", expected, other);
    }
}
