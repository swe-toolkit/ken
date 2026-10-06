//! CAT-MIGRATE-TIER-C-DATA-VALUE Vector closeout controls.
//!
//! Vector owns checked indexed families, operations, computation theorems,
//! and private map, lookup, and round-trip laws. Its checked providers include
//! Combinators, Transport, and Derived.length; the new private functions
//! add no trust beyond those providers.
//! `cat_vec_acceptance` retains the family-index, computation, and
//! impossible-call behavior obligations.

use std::collections::BTreeSet;
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

fn with_private_names() -> ElabEnv {
    let (mut env, _) = load(VECTOR);
    let prefix = format!("{VECTOR}.");
    let aliases: Vec<_> = env
        .globals
        .iter()
        .filter_map(|(qualified, id)| {
            qualified
                .strip_prefix(&prefix)
                .map(|local| (local.to_owned(), *id))
        })
        .collect();
    env.globals.extend(aliases);
    env
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
    for provider in [
        "Core.Function.Combinators",
        "Core.Logic.Transport",
        "Data.Collections.Derived",
    ] {
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
/// prelude, Combinators, Transport and Derived.length global IDs, and preserve
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
        "Data.Collections.Derived.length",
    ] {
        expected_external.insert(via_vector.globals[name]);
    }
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

/// MEASURED: a known public Transport item succeeds through the same selective
/// import path, while every direct Vector name (including the three new
/// operations) rejects with its exact qualified `UnboundName`. CLAIMED:
/// Vector's loader-visible catalog inventory remains private. THE GAP: none;
/// the exact owned inventory supplies the complete direct-name population.
#[test]
fn vector_loader_visible_inventory_is_empty() {
    let mut positive = ElabEnv::new().expect("base environment");
    positive
        .elaborate_module_from_roots(&[catalog_root()], TRANSPORT)
        .expect("Transport positive-control provider must roots-load");
    positive
        .elaborate_file(&format!(
            "import {TRANSPORT} (cong as vector_closeout_public_control)"
        ))
        .expect("the selective-import positive control must succeed");

    let (mut env, _) = load(VECTOR);
    for (index, surface) in expected_owned_names().iter().enumerate() {
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
/// lengths, functions and vectors. THE GAP: the client pins the proposition
/// and transparent proof, not its particular proof structure. An intact
/// statement with a broken operation is separately tested by kernel check.
#[test]
fn lookup_zip_with_has_exact_checked_generic_proposition() {
    let mut env = with_private_names();
    let id = env.globals[&format!("{VECTOR}.lookup_zip_with")];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("lookup law must roots-load")
    else {
        panic!("lookup_zip_with must be a checked transparent theorem");
    };
    let actual = ty.clone();
    env.elaborate_file(
        "theorem checked_lookup_zip_with_contract
           (a : Type) (b : Type) (c : Type) (n : Nat)
           (f : a → b → c) (xs : Vec a n) (ys : Vec b n) (i : Fin n)
         : Equal c
             (lookup c n (zip_with a b c n f xs ys) i)
             (f (lookup a n xs i) (lookup b n ys i)) =
           lookup_zip_with a b c n f xs ys i",
    )
    .expect("the generic consumer must type-check against the lookup law");
    let witness_id = env.globals["checked_lookup_zip_with_contract"];
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
/// is an independent spelling of the law; the kernel checks the application,
/// while raw type equality catches any binder or endpoint change.
#[test]
fn zip_with_map_has_exact_checked_pointwise_proposition() {
    let mut env = with_private_names();
    let name = format!("{VECTOR}.zip_with_map");
    let id = env.globals[&name];
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("law must roots-load") else {
        panic!("{name} must be a checked transparent theorem, not an assumption");
    };
    let actual = ty.clone();
    env.elaborate_file(
        "theorem pointwise_vector_naturality_contract
           (a : Type) (a2 : Type) (b : Type) (b2 : Type) (c : Type) (n : Nat)
           (g : a → b) (h : a2 → b2) (f : b → b2 → c)
           (k : a → a2 → c)
           (hk : (u : a) → (v : a2) → Equal c (k u v) (f (g u) (h v)))
           (xs : Vec a n) (ys : Vec a2 n)
         : Equal (Vec c n)
             (zip_with b b2 c n f (map a b n g xs) (map a2 b2 n h ys))
             (zip_with a a2 c n k xs ys) =
           zip_with_map a a2 b b2 c n g h f k hk xs ys",
    )
    .expect("the stated proposition must be inhabited by the Vector law");
    let witness_id = env.globals["pointwise_vector_naturality_contract"];
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
/// discriminates the two orders, not every possible function pair.
#[test]
fn swapped_maps_reject_on_distinct_boolean_functions() {
    let mut env = with_private_names();
    env.elaborate_file(
        "fn pointwise_g (x : Bool) : Bool = x
         fn pointwise_h (x : Bool) : Bool =
           match x { True ↦ False; False ↦ True }
         fn pointwise_f (x : Bool) (y : Bool) : Bool = y
         fn pointwise_k (x : Bool) (y : Bool) : Bool =
           pointwise_f (pointwise_g x) (pointwise_h y)
         theorem pointwise_hk (u : Bool) (v : Bool)
           : Equal Bool (pointwise_k u v)
               (pointwise_f (pointwise_g u) (pointwise_h v)) = Refl
         theorem mapped_g_true : Equal Bool (pointwise_g True) True = Proved
         theorem mapped_h_true : Equal Bool (pointwise_h True) False = Proved
         theorem unswapped_concrete :
           Equal (Vec Bool (Suc Zero))
             (zip_with Bool Bool Bool (Suc Zero) pointwise_f
               (map Bool Bool (Suc Zero) pointwise_g (VCons Bool Zero True (VNil Bool)))
               (map Bool Bool (Suc Zero) pointwise_h (VCons Bool Zero True (VNil Bool))))
             (zip_with Bool Bool Bool (Suc Zero) pointwise_k
               (VCons Bool Zero True (VNil Bool))
               (VCons Bool Zero True (VNil Bool))) =
           zip_with_map Bool Bool Bool Bool Bool (Suc Zero)
             pointwise_g pointwise_h pointwise_f pointwise_k pointwise_hk
             (VCons Bool Zero True (VNil Bool))
             (VCons Bool Zero True (VNil Bool))",
    )
    .expect("the original order and distinguishing functions must check");
    let bad = "theorem swapped_concrete :
           Equal (Vec Bool (Suc Zero))
             (zip_with Bool Bool Bool (Suc Zero) pointwise_f
               (map Bool Bool (Suc Zero) pointwise_h (VCons Bool Zero True (VNil Bool)))
               (map Bool Bool (Suc Zero) pointwise_g (VCons Bool Zero True (VNil Bool))))
             (zip_with Bool Bool Bool (Suc Zero) pointwise_k
               (VCons Bool Zero True (VNil Bool))
               (VCons Bool Zero True (VNil Bool))) = Proved";
    let error = env
        .elaborate_file(bad)
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
