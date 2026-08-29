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
    catalog_or::load_derived_importing_fixture_many(&mut env, &["length", "reverse"]);
    for name in ["length", "reverse"] {
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
