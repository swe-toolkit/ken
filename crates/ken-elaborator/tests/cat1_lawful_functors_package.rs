//! CAT-1 package checks for the landed lawful-functors source.
//! This loads the real package files through the production elaborator path.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;
use ken_kernel::Term;

const LAWFUL_FUNCTORS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulFunctors.ken.md");

fn mk_env_with_lawful_functors() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env construction failed");
    // LawfulFunctors declares real imports for Derived's list append, LC's
    // canonical Bool conjunction family, and Transport. Keep the imported
    // aliases out of the flat fixture so those selective imports remain the
    // only route by which the package resolves them.
    catalog_or::load_core_logic_compare(&mut env);
    let (lawful_owned, _) =
        catalog_or::load_derived_importing_fixture_many(&mut env, &["list_append"]);
    catalog_or::withhold_lc_bool_and_flat_aliases(&mut env, &lawful_owned);
    catalog_or::load_function_combinators(&mut env);
    env.elaborate_ken_md_file(LAWFUL_FUNCTORS_KEN_MD)
        .expect("catalog/packages/Core/Classes/LawfulFunctors.ken.md must elaborate");
    env
}

/// Promise class: durable checked-provider identity invariant.
/// MEASURED: forging only a flat alias preserves the loader-owned Transport
/// operation; forging the qualified key to a distinct checked Compare ID
/// makes the fixture alias exposure refuse. CLAIMED: fixture alias wiring
/// cannot turn an unrelated provider into Transport by mutable spelling.
/// THE GAP: this guards the three names exposed by the shared helper, not
/// arbitrary aliases created by other test-local fixtures.
#[test]
fn transport_fixture_aliases_reject_forged_qualified_provider_key() {
    let mut env = ElabEnv::new().expect("base environment");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    let canonical = catalog_or::provider_owned_id(
        &env, &transport_owned, "Core.Logic.Transport", "cong",
    ).expect("Transport must own checked cong");
    let foreign = env.globals["Core.Logic.Compare.pair_compare"];
    assert_ne!(canonical, foreign);
    env.globals.insert("cong".to_owned(), foreign);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    assert_eq!(env.globals["cong"], canonical);

    env.globals.insert("Core.Logic.Transport.cong".to_owned(), foreign);
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    })).expect_err("forged qualified cong must fail Transport ownership");
    let message = error.downcast_ref::<String>().map(String::as_str)
        .or_else(|| error.downcast_ref::<&str>().copied())
        .expect("Transport refusal must report a string");
    assert!(message.contains("does not own"), "unexpected refusal: {message}");
}

/// Promise class: durable checked-provider identity invariant.
/// MEASURED: a checked Compare ID cannot become a canonical LawfulClasses
/// Bool helper by forging the qualified key before the fixture withholds its
/// flat alias. CLAIMED: the LC fixture reads the provider's owned population.
/// THE GAP: this pins bool_and; the shared loop checks its other three names.
#[test]
fn lawful_fixture_withhold_rejects_forged_qualified_provider_key() {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_compare(&mut env);
    let (lawful_owned, _) =
        catalog_or::load_derived_importing_fixture_many(&mut env, &["list_append"]);
    let canonical = catalog_or::provider_owned_id(
        &env, &lawful_owned, "Core.Classes.LawfulClasses", "bool_and",
    ).expect("LawfulClasses must own checked bool_and");
    let foreign = env.globals["Core.Logic.Compare.pair_compare"];
    assert_ne!(canonical, foreign);
    env.globals.insert("bool_and".to_owned(), foreign);
    assert_eq!(
        catalog_or::provider_owned_id(
            &env, &lawful_owned, "Core.Classes.LawfulClasses", "bool_and",
        ),
        Ok(canonical),
    );
    env.globals.insert("Core.Classes.LawfulClasses.bool_and".to_owned(), foreign);
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        catalog_or::withhold_lc_bool_and_flat_aliases(&mut env, &lawful_owned);
    })).expect_err("forged qualified bool_and must fail LC ownership");
    let message = error.downcast_ref::<String>().map(String::as_str)
        .or_else(|| error.downcast_ref::<&str>().copied())
        .expect("LawfulClasses refusal must report a string");
    assert!(message.contains("does not own"), "unexpected refusal: {message}");
}

#[test]
fn lawful_functors_package_elaborates_with_parametric_list_monoid() {
    let env = mk_env_with_lawful_functors();

    let instance_id = env
        .globals
        .get("Monoid_instance_List")
        .copied()
        .expect("Monoid (List a) should register by bare List head");
    let (_, ty) = env
        .env
        .const_type(instance_id)
        .expect("registered instance should have a kernel type");

    assert!(
        matches!(ty, Term::Pi(_, _)),
        "Monoid (List a) should elaborate as a Pi-typed generic dictionary, got {ty:?}"
    );
    assert!(
        env.class_env.instance_search("Monoid", "List").is_some(),
        "coherence key should be Monoid/List, not a closed element type"
    );

    for (class, head, global) in [
        ("Functor", "List", "Functor_instance_List"),
        ("Functor", "Option", "Functor_instance_Option"),
        ("Foldable", "List", "Foldable_instance_List"),
        ("Foldable", "Option", "Foldable_instance_Option"),
    ] {
        let instance_id = env
            .globals
            .get(global)
            .copied()
            .unwrap_or_else(|| panic!("{global} should be registered"));
        env.env
            .const_type(instance_id)
            .unwrap_or_else(|| panic!("{global} should have a kernel type"));
        assert!(
            env.class_env.instance_search(class, head).is_some(),
            "coherence key should include {class}/{head}"
        );
    }
}

#[test]
fn lawful_functors_source_cites_landed_laws_without_axiom() {
    let compact = LAWFUL_FUNCTORS_KEN_MD
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        compact.contains("instance Monoid (List a)"),
        "package must use the parametric List Monoid instance head"
    );
    assert!(
        !LAWFUL_FUNCTORS_KEN_MD.contains("instance Monoid (List Nat)"),
        "package must not leave the old closed List Nat Monoid instance"
    );
    assert!(
        compact.contains("assoc = proof assoc for list_append a;")
            && compact.contains("left_unit = proof left_unit for list_append a;")
            && compact.contains("right_unit = proof right_unit for list_append a"),
        "law fields should cite the existing generic List proofs"
    );
    assert!(
        !LAWFUL_FUNCTORS_KEN_MD.contains("= Axiom"),
        "lawful-functors package must not fill laws with Axiom"
    );
    assert!(
        compact.contains("class Functor (f : Type → Type)")
            && compact.contains("id_law : (a : Type) → (x : f a)")
            && compact.contains("fusion_law : (a : Type) → (b : Type) → (c : Type)")
            && compact.contains("(g : b → c) → (h : a → b) → (x : f a)"),
        "Functor should use the settled single pointwise law fields"
    );
    assert!(
        !LAWFUL_FUNCTORS_KEN_MD.contains("map_id")
            && !LAWFUL_FUNCTORS_KEN_MD.contains("map_comp")
            && !LAWFUL_FUNCTORS_KEN_MD.contains("pointfree"),
        "Functor should not add a point-free duplicate law surface"
    );
    assert!(
        LAWFUL_FUNCTORS_KEN_MD.contains("instance Functor List")
            && LAWFUL_FUNCTORS_KEN_MD.contains("instance Functor Option")
            && LAWFUL_FUNCTORS_KEN_MD.contains("instance Foldable List")
            && LAWFUL_FUNCTORS_KEN_MD.contains("instance Foldable Option"),
        "D3 should provide List and Option Functor/Foldable instances"
    );
    assert!(
        LAWFUL_FUNCTORS_KEN_MD.contains("fold_map_coherence")
            && LAWFUL_FUNCTORS_KEN_MD.contains("fold_map_step")
            && LAWFUL_FUNCTORS_KEN_MD.contains("foldr_to_list"),
        "Foldable should pin fold_map through the selected Monoid and to_list reconstruction laws"
    );
}
