//! VERIFY-OBLIGATION-ID-SHADOWED-OWNER package boundary: a second owner of
//! one top-level name is refused before emission. The two-owner obligation
//! identity property is tested at the emitter seam in compiler_driver.rs.
//! Spec: 20-verification/22 §1, §2.3; 40-runtime/46 §3.2.

use ken_elaborator::checked_core::{
    canonical_term_bytes, CheckedCorePackage, StableSymbol, StableSymbolTable, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerDriverError, CompilerManifest, CompilerSource,
    CompilerTargetKind, TargetSelector,
};
use ken_elaborator::{ElabEnv, ElabError};
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

fn compile(sources: &[(&str, &str)]) -> Result<CheckedCorePackage, CompilerDriverError> {
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
    .map(|output| output.package)
}

fn assert_package_refuses_rebound_name(sources: &[(&str, &str)], expected: &str) {
    let error = compile(sources).expect_err("a second package owner must not produce a package");
    assert!(
        matches!(
            error,
            CompilerDriverError::Elaboration(ElabError::DuplicateDefinition { ref name, .. })
                if name == expected
        ),
        "package must refuse DuplicateDefinition({expected}), got {error:?}"
    );
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

/// AC-1d; promise class: durable invariant. MEASURED: a second file cannot
/// rebind `Foo` in either order; the driver returns the typed refusal and no
/// checked package. CLAIMED: package-order changes cannot erase an owner's
/// instance premise. THE GAP: the emitter's legal two-owner state is tested
/// separately in compiler_driver.rs, not reached by package compilation.
#[test]
fn rebound_instances_refuse_second_package_owner_in_both_orders() {
    assert_package_refuses_rebound_name(
        &[("src/a.ken", INSTANCE_A), ("src/b.ken", INSTANCE_B)],
        "Foo",
    );
    assert_package_refuses_rebound_name(
        &[("src/b.ken", INSTANCE_B), ("src/a.ken", INSTANCE_A)],
        "Foo",
    );
}

/// AC-1d; promise class: durable invariant. MEASURED: the second file's
/// `Foo` cannot displace the first dictionary owner reached by `z`; no
/// checked package is emitted. CLAIMED: a package cannot silently discard
/// that owner's premise. THE GAP: the retained reference itself is asserted
/// in the separate emitter-state test rather than in a refused package.
#[test]
fn retained_dictionary_use_refuses_second_package_owner() {
    let a = format!("{INSTANCE_A}const z : Int where Pick Foo = d.sel MkOld\n");
    assert_package_refuses_rebound_name(&[("src/a.ken", &a), ("src/b.ken", INSTANCE_B)], "Foo");
}

/// AC-1d; promise class: durable invariant. MEASURED: `USER_B` cannot
/// rebind `x` after `USER_A` defines `y`; no checked package is emitted.
/// CLAIMED: a later package file cannot hide a retained user's obligation.
/// THE GAP: `y`'s retained checked identity is pinned at the emitter seam.
#[test]
fn retained_user_shadow_refuses_second_package_owner() {
    assert_package_refuses_rebound_name(&[("src/a.ken", USER_A), ("src/b.ken", USER_B)], "x");
}

/// AC-1d; promise class: durable invariant. MEASURED: `CLASS_B` cannot
/// rebind the earlier `Pick` class, so no checked package is emitted.
/// CLAIMED: an instance premise cannot be silently lost by class shadowing.
/// THE GAP: whether class rebinds in a session is tested separately.
#[test]
fn rebound_class_refuses_second_package_owner() {
    assert_package_refuses_rebound_name(&[("src/a.ken", CLASS_A), ("src/b.ken", CLASS_B)], "Pick");
}

/// AC-2; promise class: durable invariant. MEASURED: each single-file
/// witness still has exactly its expected source predicate, ID and origin.
/// CLAIMED: disambiguation activates only for two checked owners of one name.
/// THE GAP: existing predecessor suites pin unrelated identity families.
#[test]
fn single_owner_controls_keep_unadorned_obligation_ids() {
    for (source, expected, other) in [(INSTANCE_A, 1, 2), (INSTANCE_B, 2, 1)] {
        let package = compile(&[("src/one.ken", source)])
            .expect("one owner must still produce a checked package");
        assert_eq!(package.artifact.semantic.obligations.len(), 1);
        assert_owner_goal(&package, "Pick_instance_Foo", expected, other);
    }
}
