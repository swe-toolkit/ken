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
const VECTOR_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Vector/Vector.ken.md");

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

fn direct_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let extracted = ken_elaborator::literate::extract_ken_md(VECTOR_KEN_MD)
        .expect("Vector literate source must extract");
    env.elaborate_file(&extracted.source)
        .expect("Vector source must elaborate and kernel-check");
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
    let mut env = direct_env();
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
    let mut env = direct_env();

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

#[test]
fn entry_adds_no_trusted_declarations() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let extracted = ken_elaborator::literate::extract_ken_md(VECTOR_KEN_MD)
        .expect("Vector literate source must extract");
    env.elaborate_file(&extracted.source)
        .expect("Vector source must elaborate and kernel-check");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Vector must add zero trusted declarations");
}
