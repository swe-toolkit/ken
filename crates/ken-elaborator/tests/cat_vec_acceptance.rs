//! CAT-VEC length-indexed-vector acceptance.
//!
//! Public names are normative compatibility vectors. Family indices, generic
//! result types, impossible-call rejection, computation proofs, roots loading,
//! and zero trust drift are durable invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

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

fn internal_vector_fixture_env() -> ElabEnv {
    let mut env = roots_env();
    // Existing fixtures inspect Vector's private operations. Bind aliases only
    // in this synthetic test scope; external clients still use real imports.
    let prefix = format!("{MODULE}.");
    let aliases: Vec<_> = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            name.strip_prefix(&prefix)
                .map(|suffix| (suffix.to_owned(), *id))
        })
        .collect();
    env.globals.extend(aliases);
    env
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
    let mut env = internal_vector_fixture_env();
    env.elaborate_file(
        "fn cat_vec_head (a : Type) (n : Nat) (xs : Vec a (Suc n)) : a = \
           head a n xs\n\
         fn cat_vec_tail (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Vec a n = \
           tail a n xs\n\
         fn cat_vec_map \
             (a : Type) (b : Type) (n : Nat) \
             (f : a -> b) (xs : Vec a n) \
           : Vec b n = map a b n f xs\n\
         fn cat_vec_zip_with \
             (a : Type) (b : Type) (c : Type) (n : Nat) \
             (f : a -> b -> c) (xs : Vec a n) (ys : Vec b n) \
           : Vec c n = zip_with a b c n f xs ys\n\
         fn cat_vec_lookup \
             (a : Type) (n : Nat) (xs : Vec a n) (i : Fin n) \
           : a = lookup a n xs i\n\
         fn cat_vec_not (x : Bool) : Bool = \
           match x { True |-> False; False |-> True }\n\
         fn cat_vec_and (x : Bool) (y : Bool) : Bool = \
           match x { True |-> y; False |-> False }\n\
         theorem cat_vec_lookup_second : \
           Equal Bool \
             (lookup Bool (Suc (Suc Zero)) \
               (VCons Bool (Suc Zero) True \
                 (VCons Bool Zero False (VNil Bool))) \
               (FSuc (Suc Zero) (FZero Zero))) \
             False = Proved\n\
         theorem cat_vec_map_second : \
           Equal Bool \
             (lookup Bool (Suc (Suc Zero)) \
               (map Bool Bool (Suc (Suc Zero)) cat_vec_not \
                 (VCons Bool (Suc Zero) True \
                   (VCons Bool Zero False (VNil Bool)))) \
               (FSuc (Suc Zero) (FZero Zero))) \
             True = Proved\n\
         theorem cat_vec_zip_second : \
           Equal Bool \
             (lookup Bool (Suc (Suc Zero)) \
               (zip_with Bool Bool Bool (Suc (Suc Zero)) cat_vec_and \
                 (VCons Bool (Suc Zero) False \
                   (VCons Bool Zero True (VNil Bool))) \
                 (VCons Bool (Suc Zero) True \
                   (VCons Bool Zero False (VNil Bool)))) \
               (FSuc (Suc Zero) (FZero Zero))) \
             False = Proved",
    )
    .expect("generic indexed APIs and concrete computations must kernel-check");
}

#[test]
fn empty_and_out_of_bounds_calls_are_rejected_by_their_indices() {
    let mut env = internal_vector_fixture_env();

    for (label, source) in [
        (
            "head on empty vector",
            "const cat_vec_bad_head : Bool = head Bool Zero (VNil Bool)",
        ),
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
}

/// Promise class: durable invariant. From an actually roots-loaded Vector,
/// independently restate and apply the three private, checked generic laws.
/// MEASURED: the elaborator/kernel checks every quantified statement and its
/// proof application, not a source spelling. CLAIMED: list length equals the
/// index and both zip/unzip inverse directions hold for arbitrary element
/// types, length and vectors. THE GAP: the proof clients use private test
/// aliases because the package does not export its Vec family or operations;
/// the original checked declarations are independently verified by roots load.
#[test]
fn bridge_laws_check_at_independent_generic_client_types() {
    let mut env = internal_vector_fixture_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(
        r#"
import Data.Collections.Derived (length)
fn bridge_as_list (a : Type) (n : Nat) (xs : Vec a n) : List a = to_list a n xs
fn bridge_zip (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n)
  : Vec (Pair a b) n = zip a b n xs ys
fn bridge_unzip (a : Type) (b : Type) (n : Nat) (ps : Vec (Pair a b) n)
  : Pair (Vec a n) (Vec b n) = unzip a b n ps

theorem bridge_length (a : Type) (n : Nat) (xs : Vec a n)
  : Equal Nat (length a (to_list a n xs)) n =
  to_list_length a n xs

theorem bridge_unzip_zip
    (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n)
  : Equal (Pair (Vec a n) (Vec b n))
      (unzip a b n (zip a b n xs ys))
      (mk_pair (Vec a n) (Vec b n) xs ys) =
  unzip_zip a b n xs ys

theorem bridge_zip_unzip (a : Type) (b : Type) (n : Nat) (ps : Vec (Pair a b) n)
  : Equal (Vec (Pair a b) n)
      (zip a b n
        (pair_fst (Vec a n) (Vec b n) (unzip a b n ps))
        (pair_snd (Vec a n) (Vec b n) (unzip a b n ps)))
      ps =
  zip_unzip a b n ps
"#,
    )
    .expect("three generic bridge laws must inhabit independent client types");
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "generic law clients must not add a trusted assumption"
    );
}

/// Promise class: durable invariant. Closed vectors at two distinct lengths
/// carry their actual heads, and unzip preserves two unequal Bool components.
/// MEASURED: checked concrete computations; CLAIMED: `to_list` keeps order,
/// `zip` pairs position-wise and `unzip` does not exchange pair projections.
/// THE GAP: concrete examples cover these witnesses, not arbitrary inputs;
/// the generic client above carries the universally quantified proof.
#[test]
fn vector_bridge_preserves_concrete_content_and_component_order() {
    let mut env = internal_vector_fixture_env();
    env.elaborate_file(
        r#"
theorem bridge_empty_list : Equal (List Bool) (to_list Bool Zero (VNil Bool)) (Nil Bool) =
  Proved

theorem bridge_two_list
  : Equal (List Bool)
      (to_list Bool (Suc (Suc Zero))
        (VCons Bool (Suc Zero) True (VCons Bool Zero False (VNil Bool))))
      (Cons Bool True (Cons Bool False (Nil Bool))) =
  Refl

theorem bridge_paired_head
  : Equal (List (Pair Bool Bool))
      (to_list (Pair Bool Bool) (Suc Zero)
        (zip Bool Bool (Suc Zero)
          (VCons Bool Zero True (VNil Bool))
          (VCons Bool Zero False (VNil Bool))))
      (Cons (Pair Bool Bool) (mk_pair Bool Bool True False) (Nil (Pair Bool Bool))) =
  Refl

theorem bridge_unzipped_left
  : Equal (Vec Bool (Suc Zero))
      (pair_fst (Vec Bool (Suc Zero)) (Vec Bool (Suc Zero))
        (unzip Bool Bool (Suc Zero)
          (VCons (Pair Bool Bool) Zero (mk_pair Bool Bool True False)
            (VNil (Pair Bool Bool)))))
      (VCons Bool Zero True (VNil Bool)) =
  Refl

theorem bridge_unzipped_right
  : Equal (Vec Bool (Suc Zero))
      (pair_snd (Vec Bool (Suc Zero)) (Vec Bool (Suc Zero))
        (unzip Bool Bool (Suc Zero)
          (VCons (Pair Bool Bool) Zero (mk_pair Bool Bool True False)
            (VNil (Pair Bool Bool)))))
      (VCons Bool Zero False (VNil Bool)) =
  Refl
"#,
    )
    .expect("closed head order and the two unequal projections must check");
}

#[test]
fn entry_adds_no_trusted_declarations_beyond_its_providers() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Function.Combinators",
        "Core.Logic.Transport",
        "Data.Collections.Derived",
    ] {
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
