//! CAT-MIGRATE-TIER-C-DATA-VALUE Vector closeout controls.
//!
//! Vector owns checked indexed families, operations, computation theorems,
//! and private map, lookup, and round-trip laws. Only Fin, FZero, and FSuc
//! are public. Its checked providers include
//! Combinators and Transport; the new private functions add no trust beyond
//! those providers.
//! `cat_vec_acceptance` retains the family-index, computation, and
//! impossible-call behavior obligations.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Level, Term};

const VECTOR: &str = "Data.Vector.Vector";
const TRANSPORT: &str = "Core.Logic.Transport";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load(module: &str) -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must isolated-roots-load: {error:?}"));
    (env, owned)
}

fn checked_vector_examples(names: &[&str]) -> (ElabEnv, Vec<GlobalId>) {
    let (mut env, owned) = load(VECTOR);
    let prefix = format!("{VECTOR}.");
    let before: BTreeMap<_, _> = env
        .globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| (name.clone(), *id))
        .collect();
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for name in names {
        assert!(
            !env.globals.contains_key(*name),
            "{name} must come from the fence"
        );
        assert!(!before.contains_key(&format!("{VECTOR}.{name}")));
    }
    env.execute_loaded_entry_checked_fences(VECTOR)
        .expect("Vector's owner-local checked examples must elaborate");
    let after: BTreeMap<_, _> = env
        .globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| (name.clone(), *id))
        .collect();
    assert_eq!(after, before, "fences cannot add Vector-owned names");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "fences cannot add trust"
    );
    for name in names {
        let id = *env
            .globals
            .get(*name)
            .unwrap_or_else(|| panic!("missing {name}"));
        assert!(
            !owned.contains(&id),
            "{name} is an example, not an owned law"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked transparent example"
        );
    }
    (env, owned)
}

fn expected_owned_names() -> BTreeSet<String> {
    [
        "FSuc",
        "FZero",
        "Fin",
        "VCons",
        "VNil",
        "Vec",
        "head",
        "head_vcons",
        "lookup",
        "lookup_fzero",
        "lookup_fsuc",
        "lookup_map",
        "lookup_zip_with",
        "map",
        "map_vcons",
        "map_vnil",
        "tail",
        "tail_vcons",
        "to_list",
        "to_list_length",
        "unzip",
        "unzip_zip",
        "unzip_zip_fst",
        "unzip_zip_snd",
        "vec_nil_case",
        "vec_nil_case_holds",
        "vec_zero_vnil",
        "vector_pair_cong",
        "vec_map_compose",
        "vec_map_identity",
        "zip_with",
        "zip_with_map",
        "zip_with_vcons",
        "zip_with_vnil",
        "zip",
        "zip_unzip",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn collect_references(term: &Term, references: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            references.insert(*id);
        }
        Term::Elim { fam, .. } => {
            references.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        collect_references(child, references);
    }
}

fn declaration_references(declaration: &Decl) -> BTreeSet<GlobalId> {
    let mut references = BTreeSet::new();
    match declaration {
        Decl::Transparent { ty, body, .. } => {
            collect_references(ty, &mut references);
            collect_references(body, &mut references);
        }
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            collect_references(ty, &mut references);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_references(term, &mut references);
            }
            for term in &inductive.indices {
                collect_references(term, &mut references);
            }
            collect_references(&inductive.former_type, &mut references);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_references(term, &mut references);
                }
                for term in &constructor.target_indices {
                    collect_references(term, &mut references);
                }
                collect_references(&constructor.type_, &mut references);
            }
        }
    }
    references
}

fn qualified_owned_names(env: &ElabEnv) -> BTreeSet<String> {
    let prefix = format!("{VECTOR}.");
    env.globals
        .keys()
        .filter_map(|name| name.strip_prefix(&prefix).map(str::to_owned))
        .collect()
}

fn qualified_owned_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    let prefix = format!("{VECTOR}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

/// Promise class: transition sentinel for the owned declarations in this
/// Vector increment. Retire or rebaseline at the next separately authorized
/// Vector declaration extension; this inventory is not a permanent API promise.
/// MEASURED: ordinary isolated roots loading installs these checked
/// Vector identities, returns only identities from that population, and
/// executes every checked fence, then retains the same qualified name and ID
/// populations. Provider-closure trust is unchanged by Vector. CLAIMED: the
/// checked examples add no further module-owned declaration or local trust. THE GAP:
/// constructors are not separate loader results; the qualified environment
/// inventory closes that population, not bare fence-local helper globals.
#[test]
fn vector_owned_inventory_transition_sentinel_and_zero_local_trust() {
    let mut provider_only = ElabEnv::new().expect("provider environment");
    for provider in ["Core.Function.Combinators", "Core.Logic.Transport"] {
        provider_only
            .elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} must roots-load: {error:?}"));
    }
    let trust_before: BTreeSet<_> = provider_only.env.trusted_base().into_iter().collect();
    provider_only
        .elaborate_module_from_roots(&[catalog_root()], VECTOR)
        .expect("Vector must roots-load over the existing provider closure");
    let trust_after: BTreeSet<_> = provider_only.env.trusted_base().into_iter().collect();
    eprintln!(
        "Vector full-closure trust: before {}, after {}",
        trust_before.len(),
        trust_after.len()
    );
    let (mut via_vector, loader_results) = load(VECTOR);
    assert_eq!(
        qualified_owned_names(&via_vector),
        expected_owned_names(),
        "Vector owned declaration inventory changed"
    );
    let owned_ids = qualified_owned_ids(&via_vector);
    assert!(
        loader_results.iter().all(|id| owned_ids.contains(id)),
        "every Vector loader result must belong to its qualified identity population"
    );
    assert_eq!(
        trust_after, trust_before,
        "Vector must add no trust beyond its checked provider closure"
    );
    let pre_fence_trust: BTreeSet<_> = via_vector.env.trusted_base().into_iter().collect();
    via_vector
        .execute_loaded_entry_checked_fences(VECTOR)
        .expect("Vector Definition and every checked fence must elaborate");
    assert_eq!(
        via_vector
            .env
            .trusted_base()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        pre_fence_trust,
        "Vector checked fences must not add trust"
    );
    assert_eq!(
        qualified_owned_names(&via_vector),
        expected_owned_names(),
        "Vector checked fences must not grow the qualified declaration inventory"
    );
    assert_eq!(
        qualified_owned_ids(&via_vector),
        owned_ids,
        "Vector checked fences must not change owned declaration identities"
    );
}

/// Promise class: transition sentinel for Vector's checked dependency edge;
/// retire or rebaseline at the next separately authorized provider change.
/// MEASURED: roots-loaded Vector's checked declarations refer to the canonical
/// prelude, Combinators and Transport global IDs, and preserve
/// the prelude's canonical identities. CLAIMED: Vector adds no shadow provider
/// for those dependencies. THE GAP: references in compiled declarations do not
/// pin imports not reached from a checked declaration; this is a transition
/// sentinel for the actually used provider closure.
#[test]
fn vector_uses_canonical_checked_provider_identities() {
    let base = ElabEnv::new().expect("base environment");
    let (via_vector, _) = load(VECTOR);
    let owned_ids = qualified_owned_ids(&via_vector);
    let mut external = BTreeSet::new();
    for id in &owned_ids {
        if let Some(declaration) = via_vector.env.lookup(*id) {
            external.extend(declaration_references(declaration));
        }
    }
    for id in &owned_ids {
        external.remove(id);
    }
    let mut expected_external = [
        "Top", "Proved", "Nat", "Zero", "Suc", "List", "Nil", "Cons", "Equal", "Prop", "Pair",
        "mk_pair", "pair_fst", "pair_snd",
    ]
    .into_iter()
    .map(|name| base.globals[name])
    .collect::<BTreeSet<_>>();
    for name in [
        "Core.Function.Combinators.comp",
        "Core.Function.Combinators.idf",
        "Core.Logic.Transport.cong",
        "Core.Logic.Transport.sym",
        "Core.Logic.Transport.trans",
    ] {
        expected_external.insert(via_vector.globals[name]);
    }
    expected_external.insert(via_vector.globals["Data.Collections.List.length"]);
    assert_eq!(
        external, expected_external,
        "Vector's checked external identity inventory changed"
    );
    for name in [
        "Top", "Proved", "Nat", "Zero", "Suc", "List", "Nil", "Cons", "Equal", "Prop", "Pair",
        "mk_pair", "pair_fst", "pair_snd",
    ] {
        assert_eq!(
            via_vector.globals[name], base.globals[name],
            "Vector must retain the compiler's canonical `{name}` identity"
        );
    }
    assert!(
        !base.globals.contains_key("map"),
        "map is not a prelude name"
    );
    let mut alongside_derived = ElabEnv::new().expect("base environment");
    alongside_derived
        .elaborate_module_from_roots(&[catalog_root()], "Data.Collections.Derived")
        .expect("Derived.map must roots-load");
    let derived_map = alongside_derived.globals["Data.Collections.Derived.map"];
    alongside_derived
        .elaborate_module_from_roots(&[catalog_root()], VECTOR)
        .expect("Vector's local map must load beside Derived.map");
    assert_ne!(
        derived_map,
        alongside_derived.globals[&format!("{VECTOR}.map")],
        "Derived.map and Vector's local map must remain distinct identities"
    );
    assert_eq!(
        alongside_derived.globals["Data.Collections.Derived.map"], derived_map,
        "Vector's local map must not overwrite the checked Derived provider"
    );
}

/// Promise class: transition sentinel for Vector's exported-name boundary.
/// MEASURED: the three public Fin names import through the roots loader and
/// checked clients refer to Vector's exact family and constructor IDs; every
/// other name in the complete owned inventory rejects with qualified
/// `UnboundName`. CLAIMED: only the bounded-index provider is publicly
/// accessible. THE GAP: the owned-name inventory must be rebaselined for any
/// future separately authorized Vector declaration extension.
#[test]
fn vector_exports_only_checked_fin_family_and_constructors() {
    let mut positive = ElabEnv::new().expect("base environment");
    positive
        .elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport positive-control provider must roots-load");
    positive
        .elaborate_file(&format!(
            "import {TRANSPORT} (cong as vector_closeout_public_control)"
        ))
        .expect("the independent selective-import control must succeed");

    let (mut env, _) = load(VECTOR);
    let owned = expected_owned_names();
    let public = BTreeSet::from(
        [
            "Fin",
            "FZero",
            "FSuc",
            "Vec",
            "VNil",
            "VCons",
            "map",
            "zip_with",
            "vec_zero_vnil",
        ]
        .map(str::to_owned),
    );
    assert!(
        public.is_subset(&owned),
        "the public names must be in Vector's complete owned inventory"
    );
    env.elaborate_file(&format!(
        "import {VECTOR} (Fin as bounded_index, FZero as first_index, FSuc as later_index)"
    ))
    .expect("all three bounded-index names must import together");
    env.elaborate_file(
        "const checked_first : bounded_index (Suc Zero) = first_index Zero
         const checked_later : bounded_index (Suc (Suc Zero)) =
           later_index (Suc Zero) (first_index Zero)",
    )
    .expect("a fresh client must use all three imported names");
    let first = declaration_references(
        env.env
            .lookup(env.globals["checked_first"])
            .expect("checked first-index client"),
    );
    let later = declaration_references(
        env.env
            .lookup(env.globals["checked_later"])
            .expect("checked successor-index client"),
    );
    for (surface, references) in [("Fin", &first), ("FZero", &first), ("FSuc", &later)] {
        assert!(
            references.contains(&env.globals[&format!("{VECTOR}.{surface}")]),
            "{surface} must refer to Vector's checked identity"
        );
    }
    for (index, surface) in owned.difference(&public).enumerate() {
        let source = format!("import {VECTOR} ({surface} as vector_private_{index})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{VECTOR}.{surface}"));
            }
            Err(other) => {
                panic!("Vector import of {surface} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("Vector unexpectedly published {surface}"),
        }
    }
}

/// Promise class: durable invariant for the generic bounded-lookup law.
/// MEASURED: roots loading installs a transparent `lookup_zip_with`; a fresh
/// client independently spells its full generic Π proposition and applies the
/// checked law at that type. The client's raw checked type must equal the
/// provider's raw type. CLAIMED: lookup after zipping with `f` at the same
/// `Fin n` equals `f` of *both* input lookups, at arbitrary element types,
/// lengths, functions and vectors. THE GAP: the contract is now spelled in
/// the provider's example fence and elaborated separately from the law;
/// raw type equality checks binders and endpoints, not proof structure.
#[test]
fn lookup_zip_with_has_exact_checked_generic_proposition() {
    let (env, _) = checked_vector_examples(&["vec_example_lookup_zip_with_contract"]);
    let id = env.globals[&format!("{VECTOR}.lookup_zip_with")];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("lookup law must roots-load")
    else {
        panic!("lookup_zip_with must be a checked transparent theorem");
    };
    let actual = ty.clone();
    let witness_id = env.globals["vec_example_lookup_zip_with_contract"];
    let Decl::Transparent { ty: expected, .. } = env
        .env
        .lookup(witness_id)
        .expect("checked client must load")
    else {
        panic!("lookup client must carry a checked proof");
    };
    assert_eq!(
        &actual, expected,
        "lookup_zip_with's raw proposition changed"
    );
}

/// Promise class: durable invariant for the checked pointwise naturality law.
/// MEASURED: a fresh roots-loaded Vector has a transparent `zip_with_map` with
/// exactly the independently elaborated thirteen-binder raw Π proposition;
/// a client type-checks the unmodified proof against that proposition.
/// CLAIMED: the proof relates mapped zip to zip with `k` for arbitrary element
/// types, lengths, vectors and the pointwise head witness, not a reflexive
/// filler or an equality with swapped maps. THE GAP: the test-owned signature
/// is now spelled in the provider's example fence, separately elaborated
/// from the law; raw type equality catches any binder or endpoint change.
#[test]
fn zip_with_map_has_exact_checked_pointwise_proposition() {
    let (env, _) = checked_vector_examples(&["vec_example_zip_with_map_contract"]);
    let name = format!("{VECTOR}.zip_with_map");
    let id = env.globals[&name];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("law must roots-load") else {
        panic!("{name} must be a checked transparent theorem, not an assumption");
    };
    let actual = ty.clone();
    let witness_id = env.globals["vec_example_zip_with_map_contract"];
    let Decl::Transparent { ty: expected, .. } = env
        .env
        .lookup(witness_id)
        .expect("contract witness must kernel-check")
    else {
        panic!("the consumer's contract witness must be transparent");
    };
    assert_eq!(
        &actual, expected,
        "zip_with_map's raw checked pointwise proposition changed"
    );
}

/// Promise class: durable invariant for an unswapped positive and swapped
/// negative use of the pointwise law.
/// MEASURED: both mapped inputs are Bool, with `g True = True` and
/// `h True = False`; the unswapped law is inhabited, while the swapped
/// concrete claim is kernel-rejected. CLAIMED: `g` and `h` are not
/// interchangeable in the pointwise law. THE GAP: this one-vector witness
/// discriminates the two orders, not every possible function pair. The
/// contract is now spelled in the provider's example fence and elaborated
/// separately from the law; checked bodies cite the exact owned operations.
#[test]
fn swapped_maps_reject_on_distinct_boolean_functions() {
    let (mut env, owned) = checked_vector_examples(&[
        "vec_example_pointwise_g",
        "vec_example_pointwise_h",
        "vec_example_pointwise_f",
        "vec_example_pointwise_k",
        "vec_example_pointwise_hk",
        "vec_example_unswapped_concrete",
        "vec_example_swapped_head",
        "vec_example_pointwise_head",
        "vec_example_swapped_head_true",
        "vec_example_pointwise_head_false",
    ]);
    let swapped = env.globals["vec_example_swapped_head"];
    let positive = env.globals["vec_example_unswapped_concrete"];
    for (example, required) in [
        (swapped, &["zip_with", "map", "lookup"][..]),
        (positive, &["zip_with_map"][..]),
    ] {
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(example) else {
            panic!("Vector example must have a checked body");
        };
        let mut references = BTreeSet::new();
        collect_references(body, &mut references);
        for name in required {
            let id = env.globals[&format!("{VECTOR}.{name}")];
            assert!(owned.contains(&id), "{name} must belong to Vector");
            assert!(
                references.contains(&id),
                "example body must cite checked {name}"
            );
        }
    }
    let error = env
        .elaborate_file(
            "theorem vector_swapped_concrete : Equal Bool \
             vec_example_swapped_head vec_example_pointwise_head = Proved",
        )
        .expect_err("swapped maps must not prove the same proposition");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: ken_kernel::KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "swapped map statement must fail by kernel type mismatch: {error:?}"
    );
}

/// Promise class: durable invariant for the exact private proof obligation.
/// MEASURED: the real roots loader installs a transparent checked theorem whose
/// raw type is three binders followed by the exact `Equal (Vec a n)` endpoints,
/// with canonical `map`/`idf` identities and de Bruijn-bound arguments.
/// CLAIMED: the checked proof is about applying Vector's map to the public
/// identity function, not merely about an equality that happens to reduce.
/// THE GAP: this pins the stated raw theorem type, not an arbitrary equivalent
/// rewriting; the authored law has precisely this contract.
#[test]
fn vec_map_identity_raw_checked_proposition_is_not_reflexive_filler() {
    let (env, _) = load(VECTOR);
    let name = format!("{VECTOR}.vec_map_identity");
    let id = env.globals[&name];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("identity law must be loaded")
    else {
        panic!("{name} must be a checked transparent proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let vec_at = |a: Term, n: Term| {
        Term::app(
            Term::app(
                Term::indformer(env.globals[&format!("{VECTOR}.Vec")], vec![]),
                a,
            ),
            n,
        )
    };
    let mapped = Term::app(
        Term::app(
            Term::app(
                Term::app(
                    Term::app(global(&format!("{VECTOR}.map")), Term::var(2)),
                    Term::var(2),
                ),
                Term::var(1),
            ),
            Term::app(global("Core.Function.Combinators.idf"), Term::var(2)),
        ),
        Term::var(0),
    );
    let proposition = Term::app(
        Term::app(
            Term::app(global("Equal"), vec_at(Term::var(2), Term::var(1))),
            mapped,
        ),
        Term::var(0),
    );
    let expected = Term::pi(
        Term::ty(Level::Zero),
        Term::pi(
            Term::indformer(env.globals["Nat"], vec![]),
            Term::pi(vec_at(Term::var(1), Term::var(0)), proposition),
        ),
    );
    assert_eq!(
        ty, &expected,
        "the checked map identity law changed its exact raw proposition"
    );
}
