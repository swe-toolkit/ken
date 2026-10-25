//! Dimension-safe natural matrices: spec/50-stdlib/65-linear-algebra.md §§1–3.
//! Every closed assertion and the dimension refusal derive from
//! conformance/stdlib/linear-algebra/seed-nat-matrix.md. Promise class:
//! durable invariant (including the seed's explicitly fixed numeric oracles).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const VECTOR: &str = "Data.Vector.Vector";
const SHIPPED_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-nat-matrix.md"
));

fn shipped_seed_sources() -> Vec<&'static str> {
    extract_ken_md(SHIPPED_SEED)
        .expect("shipped linear-algebra seed Ken fences parse")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SHIPPED_SEED[fence.body_range])
        .collect()
}

// Match a type-position application by its checked operator, not by its
// whitespace, source line or the names of this seed's concrete matrices.
fn applied_type<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TVar(found, _) | SourceType::TCon(found, _) if found == name => Some(args),
        _ => None,
    }
}

// A closed false Equal normalizes to Bottom, so conversion alone cannot
// distinguish this product law from an unrelated false equality. Compare
// parsed proposition trees, ignoring spans and whitespace; derive both
// expected trees from the positive control's own product operands.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported shape in the closed product seed: {other:?}"),
    }
}

fn proposition_shape(proposition: &str) -> String {
    let source = format!("theorem expected_product_relation : {proposition} = Refl");
    let decls = parse_decls(&source).expect("derived product relation parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = decls.as_slice() else {
        panic!("derived product relation is one theorem");
    };
    type_shape(theorem)
}

// Build the reversed and unreversed statements from the positive control's
// transpose-of-product operands. The shape comparison pins the proposition;
// the checker separately proves the reversed case and refuses the false one.
fn assert_paired_order_goals(shared: &str, refused: &str) {
    let positive_decls = parse_decls(shared).expect("shared order-control Ken parses");
    let Some(SourceDecl::TheoremDecl {
        theorem: positive_goal,
        body: Expr::ECon(proof, _),
        ..
    }) = positive_decls.last()
    else {
        panic!("square same-input control must end in a theorem proof");
    };
    assert_eq!(proof, "Refl", "positive control must carry its proof");
    let equality = applied_type(positive_goal, "Equal").expect("control states Equal");
    let [carrier, lhs, _] = equality.as_slice() else {
        panic!("control has one carrier and two endpoints");
    };
    let transpose = applied_type(lhs, "transpose").expect("control transposes a product");
    let [m, p, product] = transpose.as_slice() else {
        panic!("control's transpose has its two dimensions and operand");
    };
    let factors = applied_type(product, "matrix_product").expect("transpose wraps a product");
    let [_, n, _, a, b] = factors.as_slice() else {
        panic!("control's product has its three dimensions and two factors");
    };
    let snippet = |ty: &SourceType| &shared[ty.span().start..ty.span().end];
    let rhs =
        |left: &SourceType, right: &SourceType| {
            format!(
            "matrix_product ({}) ({}) ({}) (transpose ({}) ({}) ({})) (transpose ({}) ({}) ({}))",
            snippet(p), snippet(n), snippet(m),
            snippet(n), snippet(p), snippet(left),
            snippet(m), snippet(n), snippet(right)
        )
        };
    let relation = |left: &SourceType, right: &SourceType| {
        format!(
            "Equal ({}) ({}) ({})",
            snippet(carrier),
            snippet(lhs),
            rhs(left, right)
        )
    };
    assert_eq!(
        type_shape(positive_goal),
        proposition_shape(&relation(b, a)),
        "positive seed theorem must state reversed factor order"
    );
    let negative_decls = parse_decls(refused).expect("refusal Ken parses");
    let [SourceDecl::TheoremDecl {
        theorem: negative_goal,
        body: Expr::ECon(negative_proof, _),
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("refused block must be one theorem with a final proof");
    };
    assert_eq!(negative_proof, "Refl", "negative claim must use Refl");
    assert_eq!(
        type_shape(negative_goal),
        proposition_shape(&relation(a, b)),
        "refused seed theorem must state unreversed factor order on the same pair"
    );
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn matrix_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix package roots-loads");
    env
}

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §§1.3, 2. The generic
/// `matrix_identity_left` and `matrix_product_assoc` clients exercise §2;
/// `matrix_vector_product`, `transpose_product`, and `dot_scale` exercise §3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact owned, transparent law identities, selective imports of
/// all twenty-six names and generic clients for ten specified propositions.
/// CLAIMED: the public module exports checked, reusable dimension-safe laws.
/// THE GAP: independent closed seed oracles separately catch wrong values.
#[test]
fn public_operations_and_generic_laws_preserve_provider_trust_set() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        VECTOR,
        "Data.Numeric.Nat.Arithmetic",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("matrix package checks against root-loaded providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "matrix package adds no trusted GlobalIds");
    let expected = [
        "vector_add",
        "vector_scale",
        "dot",
        "matrix_vector",
        "transpose",
        "matrix_product",
        "identity",
        "dot_comm",
        "dot_distrib",
        "transpose_invol",
        "vector_identity",
        "matrix_identity_right",
        "matrix_identity_left",
        "matrix_product_assoc",
        "dot_scale",
        "matrix_vector_product",
        "transpose_product",
        "matrix_add",
        "matrix_product_distrib_left",
        "matrix_product_distrib_right",
        "matrix_vector_distrib",
        "transpose_add",
        "vector_add_comm",
        "vector_add_assoc",
        "matrix_add_comm",
        "matrix_add_assoc",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    let owned_names = env
        .globals
        .iter()
        .filter_map(|(qualified, id)| {
            qualified
                .strip_prefix(&format!("{MATRIX}."))
                .filter(|_| owned.contains(id))
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>();
    assert!(
        expected.is_subset(&owned_names),
        "all twenty-six public names are owned"
    );
    for name in &expected {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    let mut visible = BTreeSet::new();
    for (index, name) in owned_names.iter().enumerate() {
        match env.elaborate_file(&format!("import {MATRIX} ({name} as matrix_probe_{index})")) {
            Ok(_) => {
                visible.insert(name.clone());
            }
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{MATRIX}.{name}"));
            }
            other => panic!("{name}: unrelated selective-import result: {other:?}"),
        }
    }
    assert_eq!(visible, expected, "no private helpers escape as public API");
    env.elaborate_file(
        r#"
import Data.Vector.Vector (Vec, VNil, VCons, map, zip_with, vec_zero_vnil)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, dot, matrix_vector, transpose, matrix_product,
   identity, dot_comm, dot_distrib, transpose_invol, vector_identity,
   matrix_identity_right, matrix_identity_left, matrix_product_assoc,
   dot_scale, matrix_vector_product, transpose_product)
fn client_add (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n =
  vector_add n xs ys
fn client_scale (n : Nat) (f : Nat) (xs : Vec Nat n) : Vec Nat n =
  vector_scale n f xs
fn client_matrix_vector (m : Nat) (n : Nat)
  (rows : Vec (Vec Nat n) m) (v : Vec Nat n) : Vec Nat m =
  matrix_vector m n rows v
fn client_product (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) : Vec (Vec Nat p) m =
  matrix_product m n p a b
theorem client_comm (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) :
  Equal Nat (dot n xs ys) (dot n ys xs) = dot_comm n xs ys
theorem client_distrib (n : Nat) (xs : Vec Nat n)
  (ys : Vec Nat n) (zs : Vec Nat n) :
  Equal Nat (dot n (vector_add n xs ys) zs)
    (add (dot n xs zs) (dot n ys zs)) = dot_distrib n xs ys zs
theorem client_transpose (m : Nat) (n : Nat)
  (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (transpose n m (transpose m n a)) a =
  transpose_invol m n a
theorem client_vector_identity (n : Nat) (v : Vec Nat n) :
  Equal (Vec Nat n) (matrix_vector n n (identity n) v) v =
  vector_identity n v
theorem client_matrix_identity (m : Nat) (n : Nat)
  (a : Vec (Vec Nat n) m) :
  Equal (Vec (Vec Nat n) m) (matrix_product m n n a (identity n)) a =
  matrix_identity_right m n a
theorem client_matrix_identity_left (m : Nat) (n : Nat)
  (a : Vec (Vec Nat m) n) :
  Equal (Vec (Vec Nat m) n) (matrix_product n n m (identity n) a) a =
  matrix_identity_left m n a
theorem client_product_assoc (m : Nat) (n : Nat) (p : Nat) (q : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n)
  (c : Vec (Vec Nat q) p) :
  Equal (Vec (Vec Nat q) m)
    (matrix_product m p q (matrix_product m n p a b) c)
    (matrix_product m n q a (matrix_product n p q b c)) =
  matrix_product_assoc m n p q a b c
theorem client_dot_scale (n : Nat) (x : Nat)
  (u : Vec Nat n) (c : Vec Nat n) :
  Equal Nat (dot n (vector_scale n x u) c) (mul x (dot n u c)) =
  dot_scale n x u c
theorem client_matrix_vector_product (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) (x : Vec Nat p) :
  Equal (Vec Nat m)
    (matrix_vector m p (matrix_product m n p a b) x)
    (matrix_vector m n a (matrix_vector n p b x)) =
  matrix_vector_product m n p a b x
theorem client_transpose_product (m : Nat) (n : Nat) (p : Nat)
  (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat p) n) :
  Equal (Vec (Vec Nat m) p)
    (transpose m p (matrix_product m n p a b))
    (matrix_product p n m (transpose n p b) (transpose m n a)) =
  transpose_product m n p a b
"#,
    )
    .expect("generic clients independently state ten law types");
}

// Sixteen positive, closed oracles from the sixteen-case conformance seed. The
// expected vectors are constructed independently of the operations tested.
const SEED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, dot, matrix_vector,
   transpose, matrix_product, identity)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

fn v2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn v3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat Zero z (VNil Nat)))

const a23 : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one
    (v3 one two Zero)
    (VCons (Vec Nat three) Zero
      (v3 Zero one three) (VNil (Vec Nat three)))
const b32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 two one)
    (VCons (Vec Nat two) one (v2 Zero two)
      (VCons (Vec Nat two) Zero (v2 one Zero)
        (VNil (Vec Nat two))))
const expected22 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 two five)
    (VCons (Vec Nat two) Zero (v2 three two)
      (VNil (Vec Nat two)))
const expected32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 one Zero)
    (VCons (Vec Nat two) one (v2 two one)
      (VCons (Vec Nat two) Zero (v2 Zero three)
        (VNil (Vec Nat two))))
const c22 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 one two)
    (VCons (Vec Nat two) Zero (v2 three Zero) (VNil (Vec Nat two)))

theorem dot_oracle : Equal Nat
  (dot three (v3 one two Zero) (v3 three one two)) five = Proved
theorem identity_two_oracle : Equal (Vec (Vec Nat two) two) (identity two)
  (VCons (Vec Nat two) one (v2 one Zero)
    (VCons (Vec Nat two) Zero (v2 Zero one) (VNil (Vec Nat two)))) = Refl
theorem vector_add_oracle : Equal (Vec Nat three)
  (vector_add three (v3 one two Zero) (v3 two Zero one))
  (v3 three two one) = Refl
theorem vector_scale_oracle : Equal (Vec Nat three)
  (vector_scale three two (v3 one two Zero))
  (v3 two four Zero) = Refl
theorem vector_scale_three_oracle : Equal (Vec Nat three)
  (vector_scale three three (v3 one two Zero))
  (v3 three six Zero) = Refl
theorem vector_scale_empty_oracle : Equal (Vec Nat Zero)
  (vector_scale Zero three (VNil Nat)) (VNil Nat) = Proved
theorem matrix_vector_oracle : Equal (Vec Nat two)
  (matrix_vector two three a23 (v3 two two one))
  (v2 six five) = Refl
theorem product_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two three two a23 b32) expected22 = Refl
theorem right_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two three three a23 (identity three)) a23 = Refl
theorem left_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two two three (identity two) a23) a23 = Refl
theorem product_assoc_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two two two (matrix_product two three two a23 b32) c22)
  (matrix_product two three two a23 (matrix_product three two two b32 c22))
  = Refl
theorem transpose_intermediate_oracle : Equal (Vec (Vec Nat two) three)
  (transpose two three a23) expected32 = Refl
theorem transpose_round_trip_oracle : Equal (Vec (Vec Nat three) two)
  (transpose three two (transpose two three a23)) a23 = Refl
theorem matrix_vector_product_oracle : Equal (Vec Nat two)
  (matrix_vector two two (matrix_product two three two a23 b32) (v2 Zero one))
  (matrix_vector two three a23 (matrix_vector three two b32 (v2 Zero one))) = Refl
theorem transpose_product_oracle : Equal (Vec (Vec Nat two) two)
  (transpose two two (matrix_product two three two a23 b32))
  (matrix_product two three two (transpose three two b32) (transpose two three a23)) = Refl
theorem matrix_vector_product_identity_control : Equal (Vec Nat two)
  (matrix_vector two three
    (matrix_product two three three a23 (identity three)) (v3 Zero one Zero))
  (matrix_vector two three a23
    (matrix_vector three three (identity three) (v3 Zero one Zero))) = Refl
"#;

/// MEASURED: the sixteen closed propositions kernel-check against independent
/// numeral/constructor expectations, except that product association and §3's
/// product relations compare two routes. CLAIMED: the sixteen specified seed
/// cases produce the stated results. THE GAP: closed equality cannot prove
/// a generic law, and the association row could pass if both products erred;
/// the separately fixed product intermediate and generic proofs cover that gap.
#[test]
fn sixteen_seed_cases_check_sixteen_closed_oracles() {
    let mut env = matrix_env();
    env.elaborate_file(SEED)
        .expect("all sixteen seed oracles must kernel-check from public imports");
    for name in [
        "dot_oracle",
        "identity_two_oracle",
        "vector_add_oracle",
        "vector_scale_oracle",
        "vector_scale_three_oracle",
        "vector_scale_empty_oracle",
        "matrix_vector_oracle",
        "product_oracle",
        "right_identity_oracle",
        "left_identity_oracle",
        "product_assoc_oracle",
        "transpose_intermediate_oracle",
        "transpose_round_trip_oracle",
        "matrix_vector_product_oracle",
        "transpose_product_oracle",
        "matrix_vector_product_identity_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-nat-matrix.md §§1–3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shipped positive seed fence roots-loads through the actual
/// catalog and all sixteen closed assertions elaborate, including three §3
/// rows; the §3 theorem declarations have transparent checked bodies.
/// CLAIMED: the authored seed's closed output and product-relation clients
/// agree with the published package on their typed inputs.
/// THE GAP: the seed is finite; arbitrary indexed laws and trust identities
/// are checked independently by the generic client above.
#[test]
fn shipped_seed_positive_clients_check_section_three() {
    let sources = shipped_seed_sources();
    let positive = sources
        .first()
        .expect("shipped seed has a positive program");
    let mut env = matrix_env();
    env.elaborate_file(positive)
        .expect("shipped seed's sixteen positive clients check");
    for name in [
        "matrix_vector_product_oracle",
        "transpose_product_oracle",
        "matrix_vector_product_identity_control",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked shipped seed client"
        );
    }
}

/// SOURCE: conformance/stdlib/linear-algebra/seed-nat-matrix.md §3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: on the same two square inputs, the reversed product relation
/// checks and the unreversed relation refuses as a `Refl` TypeMismatch. Ken
/// structurally verifies that the seed's positive theorem states the reversed
/// relation and its negative the unreversed relation, both derived from the
/// positive control's own product operands. Unrelated false equalities cannot
/// pass by merely normalizing to Bottom.
/// CLAIMED: transpose reverses product factor order.
/// THE GAP: one finite pair does not prove the generic theorem; the exported
/// transparent body and independent arbitrary-index client are checked above.
#[test]
fn unreversed_transpose_product_refuses_after_reversed_control_checks() {
    let sources = shipped_seed_sources();
    let positive = sources.first().expect("shipped seed positive program");
    let contrast = sources.last().expect("shipped seed order contrast");
    let declarations = parse_decls(contrast).expect("order contrast Ken parses");
    let Some(SourceDecl::TheoremDecl { span, .. }) = declarations.last() else {
        panic!("order contrast must end in a theorem");
    };
    let (shared, refused) = contrast.split_at(span.start);
    assert_eq!(
        parse_decls(refused).expect("refused theorem parses").len(),
        1,
        "negative input must contain only its own false theorem"
    );
    assert_paired_order_goals(shared, refused);
    let mut env = matrix_env();
    env.elaborate_file(positive)
        .expect("shipped seed's positive program checks first");
    env.elaborate_file(shared)
        .expect("shipped seed's reversed-order control checks on the same pair");
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal" && span.end <= refused.len() => {}
        other => panic!("shipped unreversed-order claim must refuse at Refl: {other:?}"),
    }
}

/// MEASURED: the exact matching call above checks, while changing only the
/// right input from three rows to two gets a kernel TypeMismatch. CLAIMED:
/// inner dimensions must agree before a product is evaluated. THE GAP: one
/// fixture does not prove other laws; the product's generic type is checked.
#[test]
fn wrong_inner_dimension_rejects_at_the_index_boundary() {
    let mut env = matrix_env();
    let sources = shipped_seed_sources();
    env.elaborate_file(sources.first().expect("seed positives"))
        .expect("matching inner dimensions must check first");
    match env.elaborate_file(sources.get(1).expect("seed wrong-inner client")) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("a two-row right operand cannot satisfy a three-row index: {other:?}"),
    }
}
