//! `Tooling.Testing.Property` canonical-list reuse and behavior acceptance.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId, Term};

const PROPERTY_KEN_MD: &str =
    include_str!("../../../catalog/packages/Tooling/Testing/Property.ken.md");

fn property_dependency_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_derived_importing_fixture_many(&mut env, &["length", "map", "reverse"]);
    for name in ["length", "map", "reverse"] {
        assert!(
            !env.globals.contains_key(name),
            "the fixture must withhold flat {name} so Property's selector is observable"
        );
    }
    env
}

fn leading_pi_count(term: &Term) -> usize {
    let mut count = 0;
    let mut current = term;
    while let Term::Pi(_, body) = current {
        count += 1;
        current = body;
    }
    count
}

fn saturated_provider_occurrences(term: &Term, provider: GlobalId, arity: usize) -> usize {
    let mut head = term;
    let mut arguments = 0;
    while let Term::App(function, _) = head {
        arguments += 1;
        head = function;
    }
    let here = usize::from(
        arguments == arity && matches!(head, Term::Const { id, .. } if *id == provider),
    );
    here + term
        .children()
        .into_iter()
        .map(|child| saturated_provider_occurrences(child, provider, arity))
        .sum::<usize>()
}

fn transparent_property_bodies_with_saturated_provider_occurrence(
    env: &ElabEnv,
    provider: GlobalId,
    property_globals: &BTreeSet<GlobalId>,
) -> BTreeSet<String> {
    let provider_type = match env.env.lookup(provider) {
        Some(Decl::Transparent { ty, .. }) => ty,
        other => panic!("Derived length must be transparent, got {other:?}"),
    };
    let arity = leading_pi_count(provider_type);
    assert!(arity > 0, "Derived length must have a function type");

    env.globals
        .iter()
        .filter_map(|(local, id)| {
            if !property_globals.contains(id) {
                return None;
            }
            let (_, body) = env.env.transparent_body(*id)?;
            (saturated_provider_occurrences(&body, provider, arity) > 0).then(|| local.to_owned())
        })
        .collect()
}

fn lit_to_eval(value: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match value {
        NumericLitVal::Int(n) => EvalVal::from(n.clone()),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        store
            .num_values
            .insert(*id, lit_to_eval(value, mkdecimalpair_id));
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_global(env: &ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let id = env.globals[name];
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("`{name}` must be transparent, got {other:?}"),
    }
}

fn assert_true(env: &ElabEnv, value: EvalVal, label: &str) {
    let expected = env.globals["True"];
    assert!(
        matches!(value, EvalVal::Ctor { id, ref args, .. } if id == expected && args.is_empty()),
        "{label} must evaluate to True, got {value:?}"
    );
}

fn boolean_list(env: &ElabEnv, value: EvalVal) -> Vec<bool> {
    let mut current = value;
    let mut result = Vec::new();
    loop {
        match current {
            EvalVal::Ctor { id, .. } if id == env.prelude_env.nil_id => return result,
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.cons_id => {
                let head = match &args[1] {
                    EvalVal::Ctor { id, .. } if *id == env.numeric_env.bool_true_id => true,
                    EvalVal::Ctor { id, .. } if *id == env.numeric_env.bool_false_id => false,
                    other => panic!("expected a Boolean list head, got {other:?}"),
                };
                result.push(head);
                current = args[2].clone();
            }
            other => panic!("expected a Boolean List constructor chain, got {other:?}"),
        }
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: real roots loading selects checked `Derived.map` for Property's
/// `gen_map` body, and the flat import fixture withholds `map` until Property's
/// selective import restores that exact provider identity. Nil and nonidentity
/// Cons examples compute the expected lists. CLAIMED: Property uses the one
/// canonical derived List operation, not a private recursive replacement.
/// THE GAP: a differently named, behaviorally equivalent unused helper is not
/// ruled out by this identity check; catalog inventory review owns that case.
#[test]
fn property_uses_derived_map_identity_for_gen_map() {
    let mut roots_env = ElabEnv::new().expect("base env");
    assert!(!roots_env.globals.contains_key("map"));
    roots_env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Tooling.Testing.Property")
        .expect("Property must roots-load through its real dependency closure");
    let derived_map = roots_env.globals["Data.Collections.Derived.map"];
    assert!(roots_env.env.transparent_body(derived_map).is_some());
    assert!(!roots_env.env.trusted_base().contains(&derived_map));
    assert!(!roots_env
        .globals
        .contains_key("Tooling.Testing.Property.gen_map_list"));
    assert!(!roots_env
        .globals
        .contains_key("Tooling.Testing.Property.map"));

    let provider_type = match roots_env.env.lookup(derived_map) {
        Some(Decl::Transparent { ty, .. }) => ty,
        other => panic!("Derived.map must be transparent, got {other:?}"),
    };
    let provider_arity = leading_pi_count(provider_type);
    let gen_map = roots_env.globals["Tooling.Testing.Property.gen_map"];
    let gen_map_body = match roots_env.env.lookup(gen_map) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("Property.gen_map must remain transparent, got {other:?}"),
    };
    assert_eq!(
        saturated_provider_occurrences(gen_map_body, derived_map, provider_arity),
        1,
        "Property.gen_map must contain one saturated Derived.map application"
    );

    let mut env = property_dependency_env();
    assert!(!env.globals.contains_key("map"));
    let flat_derived_map = env.globals["Data.Collections.Derived.map"];
    env.elaborate_ken_md_file(PROPERTY_KEN_MD)
        .expect("Property must elaborate through its selective Derived.map import");
    assert!(
        !env.globals.contains_key("map"),
        "Property's selective import must stay module-local in the flat fixture"
    );
    let flat_gen_map = env.globals["gen_map"];
    let flat_body = match env.env.lookup(flat_gen_map) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("flat Property.gen_map must be transparent, got {other:?}"),
    };
    assert_eq!(
        saturated_provider_occurrences(flat_body, flat_derived_map, provider_arity),
        1,
        "flat Property.gen_map must use the same exact Derived.map identity"
    );
    env.elaborate_file(
        "fn cat_derived_property_flip (x : Bool) : Bool = \
           match x { False ↦ True; True ↦ False }\n\
         const cat_derived_property_map_nil : List Bool = \
           gen_samples Bool \
             (gen_map Bool Bool cat_derived_property_flip \
               (gen_from_list Bool (Nil Bool)))\n\
         const cat_derived_property_map_recursive_cons : List Bool = \
           gen_samples Bool \
             (gen_map Bool Bool cat_derived_property_flip \
               (gen_from_list Bool \
                 (Cons Bool True (Cons Bool False (Nil Bool)))))",
    )
    .expect("gen_map must retain nondegenerate Nil/Cons behavior");

    let mut store = make_store(&env);
    for (name, expected) in [
        ("cat_derived_property_map_nil", vec![]),
        ("cat_derived_property_map_recursive_cons", vec![false, true]),
    ] {
        let value = eval_global(&env, &mut store, name);
        assert_eq!(
            boolean_list(&env, value),
            expected,
            "{name} must preserve Derived.map's recursive behavior"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: among transparent declarations introduced by the real Property
/// source, exactly `byte_cursor_remaining` contains a saturated application
/// spine headed by the exact Derived `length` identity. The retired local name
/// is absent, qualified-name trust does not grow, the real import resolves with
/// the flat alias withheld, and an available unimported sibling stays unresolved.
///
/// CLAIMED: Property has the elaboration-visible shape of the requested
/// canonical `length` migration and its selective import is load-bearing.
///
/// THE GAP: occurrence is a syntactic predicate. It does not prove evaluation,
/// reachability, result provenance, or the absence of dead padding or differently
/// named local computation. An unused extra source selector that installs no
/// loader binding is observationally outside this loader-visible pin. Concrete
/// witness evaluation and the separate reuse census own the other obligations.
#[test]
fn property_length_occurrence_and_selective_import_are_pinned() {
    let mut env = property_dependency_env();
    let before_globals: BTreeSet<_> = env.globals.values().copied().collect();
    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(PROPERTY_KEN_MD)
        .expect("Property must elaborate through its selective length import");

    assert!(
        !env.globals.contains_key("property_list_length"),
        "the retired package-local property_list_length declaration must be absent"
    );
    assert!(
        !env.globals.contains_key("reverse"),
        "Property's selective import must not install the omitted reverse binding"
    );
    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let new_trust_names: BTreeSet<_> = after_trust
        .difference(&before_trust)
        .map(|id| match env.env.lookup(*id) {
            Some(Decl::Opaque { name, .. }) => name.clone(),
            other => panic!("new trusted entry {id:?} must be named and opaque, got {other:?}"),
        })
        .collect();
    assert!(
        new_trust_names.is_empty(),
        "Property must add zero consumer-local qualified-name trust, got {new_trust_names:?}"
    );

    let property_globals: BTreeSet<_> = env
        .globals
        .values()
        .copied()
        .filter(|id| !before_globals.contains(id))
        .collect();
    let provider = env.globals["Data.Collections.Derived.length"];
    assert_eq!(
        transparent_property_bodies_with_saturated_provider_occurrence(
            &env,
            provider,
            &property_globals,
        ),
        BTreeSet::from(["byte_cursor_remaining".to_owned()]),
        "the saturated exact length-provider occurrence population must match"
    );

    let mut omitted = ElabEnv::empty().expect("prelude bootstrap");
    omitted
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("Derived provider must roots-load");
    let error = omitted
        .elaborate_file(
            "import Data.Collections.Derived (length)\n\
             fn property_length_negative (xs : List Bool) : List Bool = reverse Bool xs",
        )
        .expect_err("available but unimported reverse must not resolve");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "reverse"),
        "the non-import control must fail at the omitted binding, got {error:?}"
    );
}

/// Promise class: normative compatibility vectors.
///
/// MEASURED: the package's three existing finite-sample witnesses evaluate to
/// `True` through the real provider-loaded source after the migration.
/// CLAIMED: success, first-counterexample ordering, and the live cursor-progress
/// discriminator retain their concrete behavior. THE GAP: these fixed samples
/// do not prove the predicates for inputs outside the package's finite generators.
#[test]
fn property_finite_sample_witnesses_retain_behavior() {
    let mut env = property_dependency_env();
    env.elaborate_ken_md_file(PROPERTY_KEN_MD)
        .expect("Property must elaborate through its selective length import");
    let mut store = make_store(&env);

    for witness in [
        "first_counterexample_witness",
        "cursor_progress_witness",
        "cursor_stuck_counterexample_witness",
    ] {
        let value = eval_global(&env, &mut store, witness);
        assert_true(&env, value, witness);
    }
}
