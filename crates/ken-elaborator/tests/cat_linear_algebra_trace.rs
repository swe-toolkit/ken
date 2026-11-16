//! Square-matrix trace: spec/50-stdlib/65-linear-algebra.md §18.
//! Promise class: durable invariant. Open law applications and kernel-checked
//! bodies establish generic obligations; closed independent values and false
//! same-form claims distinguish diagonal trace from row sum and addition.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> (
    ElabEnv,
    BTreeSet<GlobalId>,
    BTreeSet<GlobalId>,
    Vec<GlobalId>,
) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Data.Vector.Vector",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix roots-loads with checked trace proofs");
    let after = env.env.trusted_base().into_iter().collect();
    (env, before, after, owned)
}

fn has_const(body: &Term, owner: GlobalId) -> bool {
    let mut terms = vec![body];
    while let Some(term) = terms.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        terms.extend(term.children());
    }
    false
}

const GENERIC: &str = r#"
import Data.Vector.Vector (Vec)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

theorem generic_trace_add
      (n : Nat) (a : Vec (Vec Nat n) n) (b : Vec (Vec Nat n) n)
    : Equal Nat (trace n (matrix_add n n a b)) (add (trace n a) (trace n b)) =
  trace_add n a b

theorem generic_trace_scale
      (n : Nat) (x : Nat) (a : Vec (Vec Nat n) n)
    : Equal Nat (trace n (matrix_scale n n x a)) (mul x (trace n a)) =
  trace_scale n x a

theorem generic_trace_transpose
      (n : Nat) (a : Vec (Vec Nat n) n)
    : Equal Nat (trace n (transpose n n a)) (trace n a) =
  trace_transpose n a

theorem generic_trace_identity (n : Nat) : Equal Nat (trace n (identity n)) n = trace_identity n
"#;

const ACCEPTED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = Suc seven

const nine : Nat = Suc eight

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

fn mat3 (r0 : Vec Nat three) (r1 : Vec Nat three) (r2 : Vec Nat three) : Vec (Vec Nat three) three =
  VCons (Vec Nat three) two r0 (VCons (Vec Nat three) one r1 (VCons (Vec Nat three) Zero r2 (VNil (Vec Nat three))))

const a : Vec (Vec Nat three) three = mat3 (vec3 one two three) (vec3 four five six) (vec3 seven eight nine)

const b : Vec (Vec Nat three) three = mat3 (vec3 two Zero one) (vec3 one three Zero) (vec3 Zero one four)

theorem trace_a_value : Equal Nat (trace three a) (add six nine) = Proved

theorem trace_b_value : Equal Nat (trace three b) nine = Proved

theorem first_row_sum_value : Equal Nat (add (add one two) three) six = Proved

theorem trace_add_closed
    : Equal Nat (trace three (matrix_add three three a b)) (add (trace three a) (trace three b)) =
  trace_add three a b

theorem trace_scale_closed
    : Equal Nat (trace three (matrix_scale three three three a)) (mul three (trace three a)) =
  trace_scale three three a

theorem trace_transpose_closed : Equal Nat (trace three (transpose three three a)) (trace three a) =
  trace_transpose three a

theorem trace_identity_closed : Equal Nat (trace three (identity three)) three = trace_identity three

theorem trace_scale_value : Equal Nat (trace three (matrix_scale three three three a)) (mul three (add six nine)) = Proved
"#;

const X_ROW: &str =
    "theorem trace_first_row : Equal Nat (trace three a) (add (add one two) three) = Proved\n";
const X_SCALE: &str = "theorem trace_scale_as_add : Equal Nat (trace three (matrix_scale three three three a)) (add three (trace three a)) = Proved\n";

/// SOURCE: spec 65 §18. PROMISE CLASS: durable invariant.
/// MEASURED: the same exact trusted GlobalId set precedes and follows the
/// matrix module; all five new declarations are owner-local and Transparent;
/// all nine private helpers reject selective import by qualified UnboundName;
/// four generic clients apply the laws at open dimensions and their checked
/// bodies refer to the corresponding exact owner Const in this environment.
/// CLAIMED: public generic trace laws are checked without adding local trust.
/// THE GAP: inherited provider trust remains; closed clients alone do not
/// establish the laws for arbitrary indexed matrices.
#[test]
fn trace_laws_are_public_generic_checked_and_add_no_trust() {
    let (mut env, before, after, owned) = loaded();
    assert_eq!(after, before, "trace adds no trusted GlobalId identity");
    for name in [
        "trace",
        "trace_add",
        "trace_scale",
        "trace_transpose",
        "trace_identity",
    ] {
        let qualified = format!("{MATRIX}.{name}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "{qualified} belongs to NatMatrix");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} has a checked body"
        );
    }
    for helper in [
        "drop_column_add",
        "drop_column_scale",
        "transpose_column_split",
        "drop_column_prepend",
        "drop_column_prepend_zero",
        "drop_column",
        "first_column",
        "row_tail",
        "row_head",
    ] {
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{MATRIX}.{helper}") => (),
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
    env.elaborate_file(GENERIC)
        .expect("open law types check on selective import");
    for (client, law) in [
        ("generic_trace_add", "trace_add"),
        ("generic_trace_scale", "trace_scale"),
        ("generic_trace_transpose", "trace_transpose"),
        ("generic_trace_identity", "trace_identity"),
    ] {
        let law_id = env.globals[&format!("{MATRIX}.{law}")];
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("checked client");
        assert!(
            has_const(&body, law_id),
            "{client} must use {law}'s owner Const"
        );
    }
}

/// SOURCE: spec 65 §18. PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted 3×3 fixture checks independent diagonal,
/// other-matrix, first-row and scaled values along with four law instances.
/// Each false claim is tried alone after a freshly checked whole accepted
/// prefix, and refuses with exactly KernelRejected(TypeMismatch) at its own
/// theorem, enclosing its Proved proof. CLAIMED: row sum and additive scaling
/// are distinguishable from diagonal trace and multiplicative scaling.
/// THE GAP: the finite values do not replace the generic checked proofs.
#[test]
fn accepted_diagonal_values_and_laws_refuse_two_false_own_claims() {
    let (mut env, _, _, _) = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("full accepted trace client checks");
    for name in [
        "trace_a_value",
        "trace_b_value",
        "first_row_sum_value",
        "trace_add_closed",
        "trace_scale_closed",
        "trace_transpose_closed",
        "trace_identity_closed",
        "trace_scale_value",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked accepted theorem"
        );
    }
    for (name, row) in [("trace_first_row", X_ROW), ("trace_scale_as_add", X_SCALE)] {
        let decls = parse_decls(row).expect("false claim parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            span: claim,
            ..
        }] = decls.as_slice()
        else {
            panic!("{name} must be exactly one theorem");
        };
        assert_eq!(actual, name);
        let (mut fresh, _, _, _) = loaded();
        fresh
            .elaborate_file(ACCEPTED)
            .unwrap_or_else(|error| panic!("{name} fresh accepted prefix: {error:?}"));
        let span = match fresh
            .elaborate_file(row)
            .expect_err("false theorem must refuse")
        {
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            } => span,
            other => panic!("{name} must refuse at own proof with exact kind: {other:?}"),
        };
        assert!(
            claim.start <= span.start && span.start < span.end && span.end <= claim.end,
            "{name} must refuse inside its own theorem: {span:?} vs {claim:?}"
        );
        let terminal = row.rfind("Proved").expect("terminal in claim");
        assert!(
            span.start <= terminal && terminal + "Proved".len() <= span.end,
            "{name} error must enclose its own Proved: {span:?}"
        );
    }
}
