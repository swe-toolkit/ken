//! CAT-VEC length-indexed-vector acceptance.
//!
//! Public names are normative compatibility vectors. Family indices, generic
//! result types, impossible-call rejection, computation proofs, roots loading,
//! and zero trust drift are durable invariants.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Context, Decl, GlobalId, KernelError, Term};

const MODULE: &str = "Data.Vector.Vector";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn roots_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Data.Vector.Vector must load from its canonical catalog path");
    env
}

fn checked_vector_examples(names: &[&str]) -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Vector must roots-load through its real provider closure");
    let prefix = format!("{MODULE}.");
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
        assert!(!before.contains_key(&format!("{MODULE}.{name}")));
    }
    env.execute_loaded_entry_checked_fences(MODULE)
        .expect("Vector owner-local examples must check");
    let after: BTreeMap<_, _> = env
        .globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| (name.clone(), *id))
        .collect();
    assert_eq!(
        after, before,
        "examples must not add Vector-owned declarations"
    );
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "examples must not add trust"
    );
    for name in names {
        let id = *env
            .globals
            .get(*name)
            .unwrap_or_else(|| panic!("missing {name}"));
        assert!(
            !owned.contains(&id),
            "{name} is an example, not a package declaration"
        );
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    (env, owned)
}

fn body_references(term: &Term, id: GlobalId) -> bool {
    matches!(term, Term::Const { id: found, .. } if *found == id)
        || term
            .children()
            .iter()
            .any(|child| body_references(child, id))
}

fn assert_example_references(env: &ElabEnv, owned: &[GlobalId], example: &str, operation: &str) {
    let id = env.globals[&format!("{MODULE}.{operation}")];
    assert!(
        owned.contains(&id),
        "{operation} must be checked Vector-owned"
    );
    let example_id = env.globals[example];
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(example_id) else {
        panic!("{example} must have a checked transparent body");
    };
    assert!(
        body_references(ty, id) || body_references(body, id),
        "{example} must mention Vector's {operation}"
    );
}

#[test]
fn roots_loader_registers_the_indexed_public_surface() {
    let env = roots_env();
    for name in [
        "Vec",
        "VNil",
        "VCons",
        "Fin",
        "FZero",
        "FSuc",
        "head",
        "tail",
        "map",
        "zip_with",
        "lookup",
        "head_vcons",
        "tail_vcons",
        "map_vnil",
        "zip_with_vnil",
        "lookup_fzero",
    ] {
        let qualified = format!("{MODULE}.{name}");
        assert!(
            env.globals.contains_key(&qualified),
            "`{qualified}` must be a real kernel-checked global"
        );
    }

    let vec_id = env.globals[&format!("{MODULE}.Vec")];
    let vec_decl = env
        .env
        .inductive(vec_id)
        .expect("Vec must be an inductive family");
    assert_eq!(vec_decl.params.len(), 1, "element type is Vec's parameter");
    assert_eq!(vec_decl.indices.len(), 1, "length is Vec's sole index");
    assert_eq!(vec_decl.constructors.len(), 2);
    assert_eq!(vec_decl.constructors[0].target_indices.len(), 1);
    assert_eq!(vec_decl.constructors[1].target_indices.len(), 1);
    assert_eq!(vec_decl.constructors[1].args.len(), 3);

    let fin_id = env.globals[&format!("{MODULE}.Fin")];
    let fin_decl = env
        .env
        .inductive(fin_id)
        .expect("Fin must be an inductive family");
    assert!(fin_decl.params.is_empty());
    assert_eq!(fin_decl.indices.len(), 1, "bound is Fin's sole index");
    assert_eq!(fin_decl.constructors.len(), 2);
    assert!(
        fin_decl
            .constructors
            .iter()
            .all(|constructor| constructor.target_indices.len() == 1),
        "every Fin constructor must refine its bound index"
    );
}

#[test]
fn generic_operations_preserve_length_and_concrete_computations_hold() {
    let (env, owned) = checked_vector_examples(&[
        "vec_example_head",
        "vec_example_tail",
        "vec_example_map",
        "vec_example_zip_with",
        "vec_example_lookup",
        "vec_example_not",
        "vec_example_and",
        "vec_example_lookup_second",
        "vec_example_map_second",
        "vec_example_zip_second",
    ]);
    for (example, operation) in [
        ("vec_example_head", "head"),
        ("vec_example_tail", "tail"),
        ("vec_example_map", "map"),
        ("vec_example_zip_with", "zip_with"),
        ("vec_example_lookup", "lookup"),
        ("vec_example_lookup_second", "lookup"),
        ("vec_example_map_second", "map"),
        ("vec_example_zip_second", "zip_with"),
    ] {
        assert_example_references(&env, &owned, example, operation);
    }
}

#[test]
fn empty_and_out_of_bounds_calls_are_rejected_by_their_indices() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Vector must roots-load with its real checked identities");
    // The two public-index negatives still go through elaborated source.
    env.elaborate_file("import Data.Vector.Vector (Vec, VNil, VCons, Fin, FZero, zip_with)")
        .expect("the public Vector constructors and operation must import");
    for (label, source) in [
        (
            "constructor of Fin Zero",
            "const cat_vec_bad_fin : Fin Zero = FZero Zero",
        ),
        (
            "zip vectors at unequal lengths",
            "const cat_vec_bad_zip : Vec Bool (Suc Zero) = \
               zip_with Bool Bool Bool (Suc Zero) (\\x.\\y.x) \
                 (VCons Bool Zero True (VNil Bool)) \
                 (VCons Bool (Suc Zero) True \
                   (VCons Bool Zero False (VNil Bool)))",
        ),
    ] {
        let error = match env.elaborate_decl(source) {
            Ok(_) => panic!("{label} unexpectedly elaborated"),
            Err(error) => error,
        };
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "{label} must fail as a kernel type mismatch, got {error:?}"
        );
    }

    // `head` is Vector-private: apply its checked identity directly so the
    // exact kernel index refusal stays measured (no alias, no reject fence).
    let head = env.globals["Data.Vector.Vector.head"];
    assert!(
        owned.contains(&head),
        "head must be Vector's checked private operation"
    );
    let vnil = env.globals["Data.Vector.Vector.VNil"];
    let vcons = env.globals["Data.Vector.Vector.VCons"];
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    let zero = Term::constructor(env.globals["Zero"], vec![]);
    let nil_bool = Term::app(Term::constructor(vnil, vec![]), bool_ty.clone());
    let apply_head = |xs: Term| {
        Term::app(
            Term::app(
                Term::app(Term::const_(head, vec![]), bool_ty.clone()),
                zero.clone(),
            ),
            xs,
        )
    };
    let one = Term::app(
        Term::app(
            Term::app(
                Term::app(Term::constructor(vcons, vec![]), bool_ty.clone()),
                zero.clone(),
            ),
            Term::constructor(env.globals["True"], vec![]),
        ),
        nil_bool.clone(),
    );
    let ctx = Context::new();
    let ok = ken_kernel::infer(&env.env, &ctx, &apply_head(one))
        .expect("the same builder must type head on a length-one vector");
    assert!(ken_kernel::convert_type(&env.env, &ctx, &ok, &bool_ty));
    match ken_kernel::infer(&env.env, &ctx, &apply_head(nil_bool)) {
        Err(KernelError::TypeMismatch { .. }) => {}
        other => panic!("head on an empty vector must be a kernel index mismatch, got {other:?}"),
    }
}

/// Promise class: durable invariant. From an actually roots-loaded Vector,
/// independently restate and apply both private, checked inverse laws.
/// MEASURED: the elaborator/kernel checks every quantified statement and its
/// proof application, not a source spelling. CLAIMED: both zip/unzip inverse
/// directions hold for arbitrary element types, length and vectors. THE GAP:
/// the contracts now live in Vector's checked example fence, separate from the
/// provider's law declarations; references and trust are checked after loading.
#[test]
fn bridge_laws_check_at_independent_generic_client_types() {
    let (env, owned) = checked_vector_examples(&[
        "vec_example_bridge_as_list",
        "vec_example_bridge_zip",
        "vec_example_bridge_unzip",
        "vec_example_bridge_unzip_zip",
        "vec_example_bridge_zip_unzip",
    ]);
    for (example, operation) in [
        ("vec_example_bridge_as_list", "to_list"),
        ("vec_example_bridge_zip", "zip"),
        ("vec_example_bridge_unzip", "unzip"),
        ("vec_example_bridge_unzip_zip", "unzip_zip"),
        ("vec_example_bridge_zip_unzip", "zip_unzip"),
    ] {
        assert_example_references(&env, &owned, example, operation);
    }
}

/// Promise class: durable invariant. Closed vectors at two distinct lengths
/// carry their actual heads, and unzip preserves two unequal Bool components.
/// MEASURED: checked concrete computations; CLAIMED: `to_list` keeps order,
/// `zip` pairs position-wise and `unzip` does not exchange pair projections.
/// THE GAP: concrete examples cover these witnesses, not arbitrary inputs;
/// the generic client above carries the universally quantified proof.
#[test]
fn vector_bridge_preserves_concrete_content_and_component_order() {
    let (env, owned) = checked_vector_examples(&[
        "vec_example_bridge_empty_list",
        "vec_example_bridge_two_list",
        "vec_example_bridge_paired_head",
        "vec_example_bridge_unzipped_left",
        "vec_example_bridge_unzipped_right",
    ]);
    for (example, operation) in [
        ("vec_example_bridge_empty_list", "to_list"),
        ("vec_example_bridge_two_list", "to_list"),
        ("vec_example_bridge_paired_head", "zip"),
        ("vec_example_bridge_unzipped_left", "unzip"),
        ("vec_example_bridge_unzipped_right", "unzip"),
    ] {
        assert_example_references(&env, &owned, example, operation);
    }
}

/// Promise class: durable invariant. For identical component types, swapping
/// the unequal left/right outputs is a well-typed but false inverse claim.
/// MEASURED: the owner-local generic inverse checks, and the independently
/// checked Boolean heads of the unequal projections are True and False; a
/// client claiming equality is kernel-refused. CLAIMED: the two components of
/// `unzip` are not interchangeable. THE GAP: this concrete witness does not
/// prove the all-type inverse; the checked generic proof above carries that.
#[test]
fn swapped_unzip_components_are_rejected_for_unequal_bool_values() {
    let (mut env, owned) = checked_vector_examples(&[
        "vec_example_bridge_left",
        "vec_example_bridge_right",
        "vec_example_bridge_unswapped",
        "vec_example_unzip_left_head",
        "vec_example_unzip_right_head",
        "vec_example_unzip_left_true",
        "vec_example_unzip_right_false",
    ]);
    assert_example_references(&env, &owned, "vec_example_bridge_unswapped", "unzip_zip");
    assert_example_references(&env, &owned, "vec_example_unzip_left_head", "unzip");
    assert_example_references(&env, &owned, "vec_example_unzip_right_head", "unzip");
    let error = env
        .elaborate_decl(
            "theorem bridge_swapped : Equal Bool \
             vec_example_unzip_left_head vec_example_unzip_right_head = Proved",
        )
        .expect_err("swapped unequal component values must not be equal");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "swapped result must fail by kernel type mismatch: {error:?}"
    );
}

#[test]
fn entry_adds_no_trusted_declarations_beyond_its_providers() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in ["Core.Function.Combinators", "Core.Logic.Transport"] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Vector must roots-load after its providers");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "Vector must add no trust beyond its providers"
    );
}
