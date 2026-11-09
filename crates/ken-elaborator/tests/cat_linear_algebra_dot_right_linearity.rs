//! Right-argument dot-product linearity: Spec 65 §15.
//! Promise class: durable invariant. The closed CV seed is separately owned.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix (vector_add, vector_scale, dot, dot_distrib_right, dot_scale_right)

theorem client_distrib_right (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
    : Equal Nat (dot n xs (vector_add n ys zs)) (add (dot n xs ys) (dot n xs zs)) =
  dot_distrib_right n xs ys zs

theorem client_scale_right (n : Nat) (factor : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    : Equal Nat (dot n xs (vector_scale n factor ys)) (mul factor (dot n xs ys)) =
  dot_scale_right n factor xs ys
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §15 and the Architect AC-0
/// public-law types. PROMISE CLASS: durable invariant.
/// MEASURED: the root-loaded package owns both checked Transparent laws;
/// its trusted GlobalId set exactly equals the provider baseline. A selective-
/// import client consumes both public laws at arbitrary indexed dimensions
/// and checks their complete second-argument result types.
/// CLAIMED: dot is publicly additive and homogeneous in its second argument
/// without any new assumption, complementing the first-argument laws.
/// THE GAP: this open client does not check finite independent values or a
/// refused false equality; the separately owned CV seed must supply them.
#[test]
fn public_right_dot_laws_are_checked_generic_and_zero_trust() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Data.Vector.Vector",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix proofs check after provider roots");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§15 trusted GlobalId identity sets: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§15 adds no trusted GlobalId identity");
    for law in ["dot_distrib_right", "dot_scale_right"] {
        let id = env.globals[&format!("{MATRIX}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to NatMatrix");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} has a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both selectively imported public laws have exact generic types");
}
