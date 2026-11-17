//! Trace is invariant under exchanging matrix product factors: spec 65 §19.
//! Promise class: durable invariant. Open checked application and closed
//! rectangular/square clients separate trace equality from matrix equality.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const LAW: &str = "trace_product_comm";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
    for provider in [
        "Data.Vector.Vector",
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
}

fn loaded_matrix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix root-loads with checked product trace proof");
    env
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut terms = vec![body];
    while let Some(term) = terms.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        terms.extend(term.children());
    }
    false
}

const GENERIC: &str = r#"import Data.Vector.Vector (Vec)

import Algorithm.LinearAlgebra.NatMatrix (matrix_product, trace, trace_product_comm)

theorem generic_trace_product_comm
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat m) n)
    : Equal Nat (trace m (matrix_product m n m a b)) (trace n (matrix_product n m n b a)) =
  trace_product_comm m n a b
"#;

const ACCEPTED: &str = r#"import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_product, trace, trace_product_comm)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const nine : Nat = Suc (Suc (Suc (Suc five)))

fn vec2 (p : Nat) (q : Nat) : Vec Nat two = VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

const a : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one (vec3 one two Zero) (VCons (Vec Nat three) Zero (vec3 Zero one three) (VNil (Vec Nat three)))

const b : Vec (Vec Nat two) three =
  VCons
    (Vec Nat two)
    two
    (vec2 one Zero)
    (VCons (Vec Nat two) one (vec2 two one) (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two))))

const c : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (vec2 one one) (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two)))

const d : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (vec2 one Zero) (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))

theorem rect_left_trace : Equal Nat (trace two (matrix_product two three two a b)) nine = Proved

theorem rect_right_trace : Equal Nat (trace three (matrix_product three two three b a)) nine = Proved

theorem rect_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two a b)
        (VCons (Vec Nat two) one (vec2 five two) (VCons (Vec Nat two) Zero (vec2 two four) (VNil (Vec Nat two)))) =
  Refl

theorem rect_law : Equal Nat (trace two (matrix_product two three two a b)) (trace three (matrix_product three two three b a)) =
  trace_product_comm two three a b

theorem square_left_trace : Equal Nat (trace two (matrix_product two two two c d)) three = Proved

theorem square_right_trace : Equal Nat (trace two (matrix_product two two two d c)) three = Proved

theorem square_law : Equal Nat (trace two (matrix_product two two two c d)) (trace two (matrix_product two two two d c)) =
  trace_product_comm two two c d

theorem square_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two c d)
        (VCons (Vec Nat two) one (vec2 two one) (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))) =
  Refl

theorem square_right_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two d c)
        (VCons (Vec Nat two) one (vec2 one one) (VCons (Vec Nat two) Zero (vec2 one two) (VNil (Vec Nat two)))) =
  Refl
"#;

const FALSE_PRODUCT_COMM: &str = r#"theorem square_products_commute
    : Equal (Vec (Vec Nat two) two) (matrix_product two two two c d) (matrix_product two two two d c) =
  Refl
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §19, Architect AC-0.
/// PROMISE CLASS: durable invariant. MEASURED: three providers load first;
/// the exact trusted GlobalId set is unchanged by loading NatMatrix. The
/// public law has an owner-local Transparent body, private helpers reject
/// import, and the selective open client uses the exact checked owner Const.
/// CLAIMED: rectangular trace-product commutativity has a reusable generic
/// proof at zero new trust. THE GAP: inherited provider trust remains, while
/// finite fixtures cannot stand in for the open theorem.
#[test]
fn provider_first_trust_owner_visibility_and_generic_law() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("ruled trace_product_comm and helpers root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        after, before,
        "trace product law adds no trusted GlobalId identity"
    );
    let qualified = format!("{MATRIX}.{LAW}");
    let owner = *env
        .globals
        .get(&qualified)
        .expect("new checked law identity exists");
    assert!(owned.contains(&owner), "law belongs to NatMatrix");
    assert!(
        matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
        "public law has checked Transparent body"
    );
    for helper in [
        "row_times_columns",
        "row_dot_sum",
        "trace_row_dot_sum",
        "drop_column_columns_cons",
        "row_dot_sum_comm",
        "dot_zero_width",
        "row_dot_sum_zero_width",
        "row_dot_sum_prepend",
        "row_dot_sum_transpose",
    ] {
        let id = *env
            .globals
            .get(&format!("{MATRIX}.{helper}"))
            .unwrap_or_else(|| panic!("{helper} must belong to NatMatrix"));
        assert!(owned.contains(&id), "{helper} is local to NatMatrix");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{helper} has a checked private body"
        );
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{MATRIX}.{helper}") => {}
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
    env.elaborate_file(GENERIC)
        .expect("open rectangular law selectively imports and checks");
    let (_, body) = env
        .env
        .transparent_body(env.globals["generic_trace_product_comm"])
        .expect("open application has a checked body");
    assert!(
        body_contains_const(&body, owner),
        "open application must use the exact NatMatrix checked owner Const"
    );
}

/// SOURCE: spec 65 §19, AC-2 rectangular and square accepted cases.
/// PROMISE CLASS: durable invariant with closed normative value controls.
/// MEASURED: full accepted fixture checks left/right rectangular traces
/// both equal nine, both distinct square products as written, their traces
/// both equal three, and both public law applications. The 2×2 rectangular
/// left product [[5,2],[2,4]] checks by Refl, distinct from its 3×3 right.
/// CLAIMED: exchanging factors preserves their traces across unequal and
/// equal dimensions without asserting matrix-product commutativity.
/// THE GAP: the values alone cannot establish the generic theorem.
#[test]
fn rectangular_and_square_values_and_law_applications_check() {
    loaded_matrix()
        .elaborate_file(ACCEPTED)
        .expect("complete rectangular and square accepted clients check");
}

/// SOURCE: spec 65 §19, AC-2 same-form false matrix commutativity.
/// PROMISE CLASS: durable invariant with an isolated false stronger claim.
/// MEASURED: after a fresh complete accepted prefix, X alone refuses with
/// ElabError::TypeMismatch and the exact Refl-shaped reason at its own Refl
/// byte span inside its sole theorem, while both product values checked.
/// CLAIMED: trace equality does not imply equality of the two matrix values.
/// THE GAP: these closed matrices discriminate the false strengthening, not
/// every possible consequence of the premise-free trace theorem.
#[test]
fn square_product_commutation_refuses_at_own_refl_after_fresh_prefix() {
    let mut env = loaded_matrix();
    env.elaborate_file(ACCEPTED)
        .expect("fresh complete accepted prefix checks before X");
    let parsed = parse_decls(FALSE_PRODUCT_COMM).expect("isolated X theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one refused product equality theorem");
    };
    assert_eq!(name, "square_products_commute");
    let token = FALSE_PRODUCT_COMM
        .rfind("Refl")
        .expect("X terminal Refl exists");
    match env
        .elaborate_file(FALSE_PRODUCT_COMM)
        .expect_err("false product equality must refuse")
    {
        ElabError::TypeMismatch { reason, span }
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= claim.end
                && span.start == token
                && span.end == token + "Refl".len()
                && FALSE_PRODUCT_COMM.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("X must refuse at only its own Refl: {other:?}"),
    }
}
