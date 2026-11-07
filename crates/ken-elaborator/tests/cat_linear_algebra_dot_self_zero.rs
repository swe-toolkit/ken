//! NatMatrix self-dot definiteness: spec/50-stdlib/65-linear-algebra.md §14.
//! Promise class: durable invariant. A generic public client uses the exact
//! curried theorem; separate closed controls distinguish a zero self-dot from
//! a nonzero one, with each false Nat premise checked in a fresh environment.
//! The independent conformance seed is authored and reviewed separately.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const ARITH: &str = "Data.Numeric.Nat.Arithmetic";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix root-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix (dot, vector_zero, vector_zero_of_dot_self_zero)

theorem client_definite (n : Nat) (v : Vec Nat n)
    : Equal Nat (dot n v v) Zero → Equal (Vec Nat n) v (vector_zero n) =
  vector_zero_of_dot_self_zero n v
"#;

const CLOSED_PREFIX: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix (dot, vector_zero, vector_zero_of_dot_self_zero)

const two : Nat = Suc (Suc Zero)
const zeros : Vec Nat two = VCons Nat (Suc Zero) Zero (VCons Nat Zero Zero (VNil Nat))
const bumped : Vec Nat two = VCons Nat (Suc Zero) Zero (VCons Nat Zero (Suc Zero) (VNil Nat))
const empty : Vec Nat Zero = VNil Nat

theorem zeros_self : Equal Nat (dot two zeros zeros) Zero = Proved
theorem zeros_is_zero : Equal (Vec Nat two) zeros (vector_zero two) =
  vector_zero_of_dot_self_zero two zeros zeros_self
theorem zeros_are_zero : Equal (Vec Nat two) zeros (vector_zero two) = Refl
theorem bumped_self : Equal Nat (dot two bumped bumped) (Suc Zero) = Proved
theorem empty_self : Equal Nat (dot Zero empty empty) Zero = Proved
theorem empty_is_zero : Equal (Vec Nat Zero) empty (vector_zero Zero) =
  vector_zero_of_dot_self_zero Zero empty empty_self
"#;

const REFUSE_LAW: &str = r#"
theorem bumped_is_zero : Equal (Vec Nat two) bumped (vector_zero two) =
  vector_zero_of_dot_self_zero two bumped Proved
"#;

const REFUSE_PREMISE: &str = r#"
theorem bumped_self_zero : Equal Nat (dot two bumped bumped) Zero = Proved
"#;

/// MEASURED: root-loading NatMatrix after its providers changes no trusted
/// GlobalId identity; the new law is owned, Transparent and usable at its
/// exact generic public type. CLAIMED: the proof adds no trust and can be
/// consumed at arbitrary indexed lengths. THE GAP: an accepted type alone
/// does not show the premise is inhabited; closed controls below do.
#[test]
fn self_dot_zero_public_law_is_owned_transparent_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Vector.Vector",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix checks against its providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted GlobalId identities: providers={} matrix={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "the matrix law adds no trusted GlobalId");

    let id = env.globals[&format!("{MATRIX}.vector_zero_of_dot_self_zero")];
    assert!(owned.contains(&id), "the public law is owned by NatMatrix");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public law is kernel-checked, not opaque"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("selective import checks the exact curried generic law");
}

/// MEASURED: the true zero and nonzero self-dots check with `Proved`, the
/// public law applies to zero vectors at lengths two and zero, and each
/// false zero premise rejects at its own declaration in a fresh checker.
/// CLAIMED: zero self-dot characterizes zero vectors without accepting a
/// nonzero component. THE GAP: these finite fixtures do not replace the
/// generic checked proof and public client above.
#[test]
fn zero_and_nonzero_self_dots_distinguish_two_false_nat_premises() {
    let mut accepted = loaded_matrix();
    accepted
        .elaborate_file(CLOSED_PREFIX)
        .expect("true zero, nonzero and empty-vector controls check");
    for name in [
        "zeros_self",
        "zeros_is_zero",
        "zeros_are_zero",
        "bumped_self",
        "empty_self",
        "empty_is_zero",
    ] {
        let id = accepted.globals[name];
        assert!(
            matches!(accepted.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked client, not an assumption"
        );
    }
    for (name, source) in [
        ("bumped_is_zero", REFUSE_LAW),
        ("bumped_self_zero", REFUSE_PREMISE),
    ] {
        let parsed = parse_decls(source).expect("isolated false theorem parses");
        let [SourceDecl::TheoremDecl { span, .. }] = parsed.as_slice() else {
            panic!("{name} must be one false theorem");
        };
        let mut env = loaded_matrix();
        env.elaborate_file(CLOSED_PREFIX)
            .expect("fresh accepted same-input control checks before refusal");
        match env.elaborate_file(source) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span: failure,
            }) if failure.start >= span.start
                && failure.end <= span.end
                && failure.start < failure.end => {}
            other => panic!("{name} must refuse its false zero premise: {other:?}"),
        }
    }
}

/// MEASURED: the deleted NatMatrix names are absent; both new attached
/// identities are Arithmetic-owned, Transparent, and unchanged when Matrix
/// loads; importing the remaining private square step returns UnboundName.
/// CLAIMED: zero-sum inversion is reused from Arithmetic, not re-exported
/// or privately duplicated by NatMatrix. THE GAP: these identity checks do
/// not prove every proof body uses that binding; deleting either new law
/// must also break both downstream package checks (AC-3).
#[test]
fn zero_sum_laws_belong_to_arithmetic_and_square_step_stays_private() {
    let mut env = ElabEnv::new().expect("compiler base");
    let arithmetic_owned = env
        .elaborate_module_from_roots(&[root()], ARITH)
        .expect("Arithmetic root-loads before NatMatrix");
    let laws = [
        ("add_eq_zero_left", "eq_zero_l"),
        ("add_eq_zero_right", "eq_zero_r"),
    ];
    let ids = laws.map(|(_, proof)| {
        let name = format!("{ARITH}.add::{proof}");
        let id = env.globals[&name];
        assert!(
            arithmetic_owned.contains(&id),
            "{name} belongs to Arithmetic"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked proof, not an assumption"
        );
        id
    });
    let matrix_owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix checks with imported Arithmetic proofs");
    env.elaborate_file(&format!("import {ARITH} (add)\n"))
        .expect("selective add import also resolves attached proofs");
    for ((old_name, proof), id) in laws.into_iter().zip(ids) {
        assert!(
            !env.globals.contains_key(&format!("{MATRIX}.{old_name}")),
            "the former NatMatrix helper must be absent"
        );
        assert_eq!(
            env.globals[&format!("{ARITH}.add::{proof}")],
            id,
            "the imported proof retains Arithmetic's GlobalId"
        );
        assert!(
            !matrix_owned.contains(&id),
            "NatMatrix does not own {proof}"
        );
    }
    match env.elaborate_file(&format!("import {MATRIX} (mul_self_eq_zero)\n")) {
        Err(ElabError::UnboundName { name, .. })
            if name == format!("{MATRIX}.mul_self_eq_zero") => {}
        other => panic!("private square step must refuse import: {other:?}"),
    }
}
