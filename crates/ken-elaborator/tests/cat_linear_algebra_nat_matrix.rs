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
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const VECTOR: &str = "Data.Vector.Vector";
const ZERO_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-zero-annihilation.md"
));
const SHIPPED_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-nat-matrix.md"
));

const GRAM_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/linear-algebra/seed-gram-trace-zero.md"
));

// The accepted client stands alone under selective public imports. The
// rejected variant keeps its same 2-vector fixtures and proof form and
// changes only the right-hand operand from v12 to v21.
const CANCEL_ACCEPTED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, matrix_add, vector_add_cancel_right, matrix_add_cancel_right)

const one : Nat = Suc Zero
const two : Nat = Suc one
fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
const v12 : Vec Nat two = pair one two
const v21 : Vec Nat two = pair two one
const v00 : Vec Nat two = pair Zero Zero
const m : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one v12
    (VCons (Vec Nat two) Zero v21 (VNil (Vec Nat two)))
const z : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one v00
    (VCons (Vec Nat two) Zero v00 (VNil (Vec Nat two)))
const empty_vector : Vec Nat Zero = VNil Nat
const empty_rows : Vec (Vec Nat two) Zero = VNil (Vec Nat two)
const empty_cols : Vec (Vec Nat Zero) two =
  VCons (Vec Nat Zero) one (VNil Nat)
    (VCons (Vec Nat Zero) Zero (VNil Nat) (VNil (Vec Nat Zero)))

theorem v12_fixture : Equal (Vec Nat two) v12
  (VCons Nat one one (VCons Nat Zero two (VNil Nat))) = Refl
theorem v21_fixture : Equal (Vec Nat two) v21
  (VCons Nat one two (VCons Nat Zero one (VNil Nat))) = Refl
theorem v00_fixture : Equal (Vec Nat two) v00
  (VCons Nat one Zero (VCons Nat Zero Zero (VNil Nat))) = Refl

theorem vector_same_value : Equal (Vec Nat two) (vector_add two v12 v00) v12 = Refl
theorem matrix_same_value : Equal (Vec (Vec Nat two) two) (matrix_add two two m z) m = Refl

theorem vector_cancel_closed : Equal (Vec Nat two) v12 v12 =
  vector_add_cancel_right two v12 v12 v00 Refl
theorem matrix_cancel_closed : Equal (Vec (Vec Nat two) two) m m =
  matrix_add_cancel_right two two m m z Refl

theorem vector_cancel_zero : Equal (Vec Nat Zero) empty_vector empty_vector =
  vector_add_cancel_right Zero empty_vector empty_vector empty_vector Proved
theorem matrix_cancel_zero_rows : Equal (Vec (Vec Nat two) Zero) empty_rows empty_rows =
  matrix_add_cancel_right Zero two empty_rows empty_rows empty_rows Proved
theorem matrix_cancel_zero_cols : Equal (Vec (Vec Nat Zero) two) empty_cols empty_cols =
  matrix_add_cancel_right two Zero empty_cols empty_cols empty_cols Refl

theorem vector_cancel_open
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) (zs : Vec Nat n)
      (sum_eq : Equal (Vec Nat n) (vector_add n xs zs) (vector_add n ys zs))
    : Equal (Vec Nat n) xs ys =
  vector_add_cancel_right n xs ys zs sum_eq

theorem matrix_cancel_open
      (rows : Nat) (cols : Nat)
      (a : Vec (Vec Nat cols) rows)
      (b : Vec (Vec Nat cols) rows)
      (c : Vec (Vec Nat cols) rows)
      (sum_eq : Equal
        (Vec (Vec Nat cols) rows)
        (matrix_add rows cols a c)
        (matrix_add rows cols b c))
    : Equal (Vec (Vec Nat cols) rows) a b =
  matrix_add_cancel_right rows cols a b c sum_eq
"#;

const CANCEL_FALSE_SUM: &str = r#"theorem cancel_false_sum : Equal (Vec Nat two) v12 v21 =
  vector_add_cancel_right two v12 v21 v00 Refl
"#;

// The same fixed operands expose the factor boundary independently of the
// cancellation law: factor two is injective here, factor Zero maps distinct
// inputs to the same zero vector or zero matrix.
const SCALE_ACCEPTED: &str = r#"import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_scale, matrix_scale, vector_scale_cancel, matrix_scale_cancel)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
const v12 : Vec Nat two = pair one two
const v34 : Vec Nat two = pair three four
const m12 : Vec (Vec Nat two) one =
  VCons (Vec Nat two) Zero v12 (VNil (Vec Nat two))
const m34 : Vec (Vec Nat two) one =
  VCons (Vec Nat two) Zero v34 (VNil (Vec Nat two))

theorem vector_scale_open
      (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
      (h : Equal (Vec Nat n)
        (vector_scale n two xs) (vector_scale n two ys))
    : Equal (Vec Nat n) xs ys = vector_scale_cancel n one xs ys h
theorem matrix_scale_open
      (rows : Nat) (cols : Nat)
      (a : Vec (Vec Nat cols) rows) (b : Vec (Vec Nat cols) rows)
      (h : Equal (Vec (Vec Nat cols) rows)
        (matrix_scale rows cols two a) (matrix_scale rows cols two b))
    : Equal (Vec (Vec Nat cols) rows) a b =
  matrix_scale_cancel rows cols one a b h

theorem vector_scale_two_value : Equal (Vec Nat two)
  (vector_scale two two v12) (pair two four) = Refl
theorem vector_scale_zero_equal : Equal (Vec Nat two)
  (vector_scale two Zero v12) (vector_scale two Zero v34) = Refl
theorem vector_scale_zero_value : Equal (Vec Nat two)
  (vector_scale two Zero v34) (pair Zero Zero) = Refl
theorem matrix_scale_zero_equal : Equal (Vec (Vec Nat two) one)
  (matrix_scale one two Zero m12) (matrix_scale one two Zero m34) = Refl

theorem vector_scale_closed : Equal (Vec Nat two) v12 v12 =
  vector_scale_cancel two one v12 v12 Refl
theorem matrix_scale_closed : Equal (Vec (Vec Nat two) one) m12 m12 =
  matrix_scale_cancel one two one m12 m12 Refl
theorem vector_scale_empty : Equal (Vec Nat Zero) (VNil Nat) (VNil Nat) =
  vector_scale_cancel Zero one (VNil Nat) (VNil Nat) Proved

theorem vector_cancel_at_factor
      (h : Equal (Vec Nat two) (vector_scale two two v12) (vector_scale two two v34))
    : Equal (Vec Nat two) v12 v34 =
  vector_scale_cancel two one v12 v34 h
theorem matrix_cancel_at_factor
      (h : Equal (Vec (Vec Nat two) one)
        (matrix_scale one two two m12) (matrix_scale one two two m34))
    : Equal (Vec (Vec Nat two) one) m12 m34 =
  matrix_scale_cancel one two one m12 m34 h
"#;

const SCALE_VECTOR_ZERO_REFUSED: &str = r#"theorem vector_cancel_at_zero
      (h : Equal (Vec Nat two) (vector_scale two Zero v12) (vector_scale two Zero v34))
    : Equal (Vec Nat two) v12 v34 =
  vector_scale_cancel two one v12 v34 h
"#;

const SCALE_MATRIX_ZERO_REFUSED: &str = r#"theorem matrix_cancel_at_zero
      (h : Equal (Vec (Vec Nat two) one)
        (matrix_scale one two Zero m12) (matrix_scale one two Zero m34))
    : Equal (Vec (Vec Nat two) one) m12 m34 =
  matrix_scale_cancel one two one m12 m34 h
"#;

const SCALE_FALSE_CONCLUSION: &str = "theorem v12_is_v34 : Equal (Vec Nat two) v12 v34 = Refl";

fn term_contains_const(term: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

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

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §§1.3, 2, 14–18, 22–23. The generic
/// `matrix_identity_left` and `matrix_product_assoc` clients exercise §2;
/// `matrix_vector_product`, `transpose_product`, and `dot_scale` exercise §3.
/// PROMISE CLASS: transition sentinel for the exact published API: the next
/// approved public declaration requires a reviewed inventory update. The
/// trust, ownership and generic-law checks are durable invariants.
/// MEASURED: the exact set of seventy-nine public identities is selectively
/// importable, all have transparent bodies, and generic clients instantiate
/// seventeen earlier propositions; the eight §16 laws have their own clients;
/// the §14 law has a separate exact generic client in its named test;
/// the §17, §22 and §23 laws have separate open-dimension clients.
/// CLAIMED: the public module exports checked, reusable dimension-safe laws.
/// Private proof refactoring stays green; missing or extra exports and a
/// changed trusted identity set fail.
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
        "vector_zero",
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
        "dot_distrib_right",
        "dot_scale_right",
        "matrix_vector_product",
        "transpose_product",
        "matrix_add",
        "matrix_zero",
        "matrix_scale",
        "trace",
        "matrix_product_distrib_left",
        "matrix_product_distrib_right",
        "matrix_vector_distrib",
        "transpose_add",
        "vector_add_comm",
        "vector_add_assoc",
        "matrix_vector_additive",
        "matrix_vector_homogeneous",
        "transpose_adjoint",
        "transpose_identity",
        "matrix_scale_distrib",
        "matrix_scale_factor_distrib",
        "matrix_scale_compose",
        "matrix_scale_one",
        "matrix_scale_zero",
        "transpose_scale",
        "matrix_product_scale",
        "matrix_product_scale_right",
        "matrix_vector_scale",
        "trace_add",
        "trace_scale",
        "trace_transpose",
        "trace_identity",
        "trace_zero",
        "trace_product_comm",
        "matrix_zero_of_trace_gram_zero",
        "transpose_gram",
        "matrix_add_comm",
        "matrix_add_assoc",
        "vector_scale_distrib",
        "vector_scale_factor_distrib",
        "vector_scale_compose",
        "vector_scale_one",
        "vector_add_zero_left",
        "vector_add_zero_right",
        "vector_add_cancel_right",
        "vector_add_eq_zero_left",
        "vector_add_eq_zero_right",
        "vector_scale_cancel",
        "vector_scale_zero",
        "dot_zero_left",
        "dot_zero_right",
        "vector_zero_of_dot_self_zero",
        "vector_scale_zero_vector",
        "matrix_vector_zero",
        "matrix_add_zero_left",
        "matrix_add_zero_right",
        "matrix_add_cancel_right",
        "matrix_add_eq_zero_left",
        "matrix_add_eq_zero_right",
        "matrix_scale_cancel",
        "matrix_vector_zero_matrix",
        "matrix_product_zero_left",
        "matrix_product_zero_right",
        "transpose_zero",
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
    assert_eq!(
        visible, expected,
        "the exact public set has seventy-nine checked names and no private helper"
    );
    env.elaborate_file(
        r#"
import Data.Vector.Vector (Vec, VNil, VCons, map, zip_with, vec_zero_vnil)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, vector_zero, dot, matrix_vector, transpose,
   matrix_product, identity, dot_comm, dot_distrib, transpose_invol,
   vector_identity, matrix_identity_right, matrix_identity_left,
   matrix_product_assoc, dot_scale, matrix_vector_product, transpose_product,
   vector_add_zero_left, vector_add_zero_right, vector_scale_zero, dot_zero_left,
   dot_zero_right, vector_scale_zero_vector, matrix_vector_zero)
fn client_add (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n) : Vec Nat n =
  vector_add n xs ys
fn client_scale (n : Nat) (f : Nat) (xs : Vec Nat n) : Vec Nat n =
  vector_scale n f xs
fn client_zero (n : Nat) : Vec Nat n = vector_zero n
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
theorem client_add_zero_left (n : Nat) (u : Vec Nat n) :
  Equal (Vec Nat n) (vector_add n (vector_zero n) u) u =
  vector_add_zero_left n u
theorem client_add_zero_right (n : Nat) (u : Vec Nat n) :
  Equal (Vec Nat n) (vector_add n u (vector_zero n)) u =
  vector_add_zero_right n u
theorem client_scale_zero (n : Nat) (u : Vec Nat n) :
  Equal (Vec Nat n) (vector_scale n Zero u) (vector_zero n) =
  vector_scale_zero n u
theorem client_dot_zero (n : Nat) (u : Vec Nat n) :
  Equal Nat (dot n (vector_zero n) u) Zero =
  dot_zero_left n u
theorem client_dot_zero_right (n : Nat) (u : Vec Nat n) :
  Equal Nat (dot n u (vector_zero n)) Zero =
  dot_zero_right n u
theorem client_scale_zero_vector (n : Nat) (f : Nat) :
  Equal (Vec Nat n) (vector_scale n f (vector_zero n)) (vector_zero n) =
  vector_scale_zero_vector n f
theorem client_matrix_vector_zero (m : Nat) (n : Nat)
  (a : Vec (Vec Nat n) m) :
  Equal (Vec Nat m) (matrix_vector m n a (vector_zero n)) (vector_zero m) =
  matrix_vector_zero m n a
"#,
    )
    .expect("generic clients independently state seventeen law types");
}

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §10. PROMISE CLASS:
/// durable invariant. MEASURED: five independent closed results at n=2 and
/// a same-form scale-one control check, while the sole false scale-zero
/// equality refuses at its own Refl. CLAIMED: the zero-vector operations
/// and four checked laws have the intended small-vector behavior. THE GAP:
/// closed cases cannot establish generic proofs; the open clients above do.
#[test]
fn zero_vector_closed_oracles_accept_and_false_scale_zero_refuses() {
    const ACCEPTED: &str = r#"
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, vector_zero, dot,
   vector_add_zero_left, vector_add_zero_right, vector_scale_zero, dot_zero_left)
const one : Nat = Suc Zero
const two : Nat = Suc one
fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
const u : Vec Nat two = pair one two
theorem left_unit : Equal (Vec Nat two)
  (vector_add two (vector_zero two) u) u = vector_add_zero_left two u
theorem right_unit : Equal (Vec Nat two)
  (vector_add two u (vector_zero two)) u = vector_add_zero_right two u
theorem zero_scale : Equal (Vec Nat two)
  (vector_scale two Zero u) (vector_zero two) = vector_scale_zero two u
theorem zero_dot : Equal Nat (dot two (vector_zero two) u) Zero =
  dot_zero_left two u
theorem left_unit_value : Equal (Vec Nat two)
  (vector_add two (vector_zero two) u) (pair one two) = Refl
theorem right_unit_value : Equal (Vec Nat two)
  (vector_add two u (vector_zero two)) (pair one two) = Refl
theorem zero_scale_value : Equal (Vec Nat two)
  (vector_scale two Zero u) (pair Zero Zero) = Refl
theorem zero_vector_value : Equal (Vec Nat two)
  (vector_zero two) (pair Zero Zero) = Refl
theorem zero_dot_value : Equal Nat (dot two (vector_zero two) u) Zero = Proved
theorem scale_one_same_form : Equal (Vec Nat two)
  (vector_scale two (Suc Zero) u) u = Refl
"#;
    const FALSE_SCALE_ZERO: &str = r#"theorem false_scale_zero :
  Equal (Vec Nat two) (vector_scale two Zero u) u = Refl"#;
    let mut accepted = matrix_env();
    accepted
        .elaborate_file(ACCEPTED)
        .expect("generic applications and independent closed results check");
    for name in [
        "left_unit",
        "right_unit",
        "zero_scale",
        "zero_dot",
        "left_unit_value",
        "right_unit_value",
        "zero_scale_value",
        "zero_vector_value",
        "zero_dot_value",
        "scale_one_same_form",
    ] {
        let id = accepted.globals[name];
        assert!(matches!(
            accepted.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let parsed_false = parse_decls(FALSE_SCALE_ZERO).expect("isolated false theorem parses");
    let [SourceDecl::TheoremDecl { span, .. }] = parsed_false.as_slice() else {
        panic!("false scale-zero client must contain exactly one theorem");
    };
    let mut rejecting = matrix_env();
    rejecting
        .elaborate_file(ACCEPTED)
        .expect("fresh same-form accepted control checks before refusal");
    match rejecting.elaborate_file(FALSE_SCALE_ZERO) {
        Err(ElabError::TypeMismatch {
            span: failure,
            reason,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("false scale-zero equality must refuse at Refl: {other:?}"),
    }
}

fn zero_seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(ZERO_SEED)
        .expect("§11 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &ZERO_SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§11 seed Ken parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "duplicate §11 seed case {name}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §11 seed case {name}"))
}

// The seed's own value rows check the two images. This separate client
// pins the zero-input and nonzero-input fixture labels to their constructed
// values, not to computations using the operation under test.
const ZERO_SEED_FIXTURE: &str = r#"
theorem zero_seed_u_fixture : Equal (Vec Nat two) u (pair one two) = Refl
theorem zero_seed_a_fixture : Equal (Vec (Vec Nat two) two) a
  (VCons (Vec Nat two) one (pair one two)
    (VCons (Vec Nat two) Zero (pair two one) (VNil (Vec Nat two)))) = Refl
theorem zero_seed_zero_fixture : Equal (Vec Nat two) zero2 (pair Zero Zero) = Refl
"#;

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §11.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full closed and generic seed checks via roots; its fixed
/// matrix and vector fixtures have independent constructor-valued checks,
/// and its false same-form proposition refuses at its own Refl after a
/// fresh full accepted prefix. CLAIMED: §11's finite, nonzero matrix
/// distinguishes its zero input from its nonzero input. THE GAP: these
/// closed values cannot establish the three generic public laws; the
/// exact roster, generic type clients, and checked bodies are tested above.
#[test]
fn zero_annihilation_seed_checks_and_nonzero_input_refuses_at_refl() {
    let accepted = zero_seed_case("dot_open");
    let refused = zero_seed_case("seed_neg");
    let mut env = matrix_env();
    env.elaborate_file(accepted)
        .expect("complete accepted §11 client and same-form control check");
    env.elaborate_file(ZERO_SEED_FIXTURE)
        .expect("§11 seed's nonzero and zero fixtures retain their values");
    for name in [
        "dot_open",
        "scale_open",
        "matrix_vector_open",
        "dot_two",
        "dot_two_value",
        "scale_two",
        "scale_two_value",
        "matrix_vector_two",
        "matrix_vector_nonzero_value",
        "matrix_vector_zero_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let positive = parse_decls(accepted).expect("§11 accepted client parses");
    let control = positive.iter().find(|decl| {
        matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "matrix_vector_zero_control")
    });
    let Some(SourceDecl::TheoremDecl {
        theorem: control_goal,
        body: Expr::ECon(control_proof, _),
        ..
    }) = control
    else {
        panic!("§11 accepted same-form control must be a closed theorem");
    };
    assert_eq!(control_proof, "Refl");
    assert_eq!(
        type_shape(control_goal),
        proposition_shape("Equal (Vec Nat two) (matrix_vector two two a zero2) zero2"),
        "accepted same-form control uses the zero vector operand"
    );
    let negative = parse_decls(refused).expect("isolated §11 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem,
        body: Expr::ECon(proof, proof_span),
        ..
    }] = negative.as_slice()
    else {
        panic!("§11 refusal is one closed Refl theorem");
    };
    assert_eq!(name, "seed_neg");
    assert_eq!(proof, "Refl");
    let mut rejecting = matrix_env();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh full §11 accepted prefix checks before refusal");
    rejecting
        .elaborate_file(ZERO_SEED_FIXTURE)
        .expect("fresh independent fixture checks before refusal");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= proof_span.start
                && span.end <= proof_span.end
                && span.start < span.end => {}
        other => panic!("§11 false nonzero input must refuse at its Refl: {other:?}"),
    }
    assert_eq!(
        type_shape(theorem),
        proposition_shape("Equal (Vec Nat two) (matrix_vector two two a u) zero2"),
        "refusal keeps the false nonzero matrix-vector operand"
    );
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

/// SOURCE: Architect AC-0 D0 and spec/50-stdlib/65-linear-algebra.md §20
/// (assembled by the Spec owner separately).
/// PROMISE CLASS: durable invariant.
/// MEASURED: the provider-first exact trusted GlobalId set and checked public
/// owner bodies are tested in the package roster above. A separate module
/// imports both laws, proves open applications at variable dimensions, checks
/// independently constructed nonempty values, and applies the laws at a
/// nonempty vector, 2-by-2 matrix, zero vector, zero-row and zero-column matrix.
/// CLAIMED: right cancellation is public, generic, checked, and zero-trust.
/// THE GAP: selected closed values cannot replace generic kernel-checked proofs;
/// no local check attests that the separate Spec/CV children or CI have run.
#[test]
fn vector_and_matrix_add_right_cancel_public_clients_check() {
    let mut env = matrix_env();
    let vector_owner = env.globals[&format!("{MATRIX}.vector_add_cancel_right")];
    let matrix_owner = env.globals[&format!("{MATRIX}.matrix_add_cancel_right")];
    for (name, id) in [
        ("vector_add_cancel_right", vector_owner),
        ("matrix_add_cancel_right", matrix_owner),
    ] {
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked NatMatrix declaration"
        );
    }
    env.elaborate_file(CANCEL_ACCEPTED)
        .expect("all closed and open right-cancellation clients check together");
    for (client, owner) in [
        ("vector_cancel_open", vector_owner),
        ("matrix_cancel_open", matrix_owner),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .unwrap_or_else(|| panic!("{client} has a checked body"));
        assert!(
            term_contains_const(&body, owner),
            "{client} cites its exact provider law owner"
        );
    }
}

/// SOURCE: Architect AC-0 D0 and spec/50-stdlib/65-linear-algebra.md §20.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh complete accepted prefix proves the same-form true
/// premise at (v12,v12,v00); changing only ys to v21 is rejected by precisely
/// ElabError::TypeMismatch at the appended premise's Refl, not at the result.
/// CLAIMED: a false sum equality cannot supply right cancellation.
/// THE GAP: the premise's finite values do not prove all distinct vectors;
/// the kernel-checked generic theorem and open client cover that obligation.
#[test]
fn false_sum_premise_refuses_at_its_own_refl_after_fresh_accepted_prefix() {
    let declarations = parse_decls(CANCEL_FALSE_SUM).expect("isolated refusal Ken parses");
    let [SourceDecl::TheoremDecl { span, .. }] = declarations.as_slice() else {
        panic!("negative client is exactly one theorem");
    };
    let refl = CANCEL_FALSE_SUM
        .rfind("Refl")
        .expect("negative premise uses Refl");
    assert!(refl >= span.start && refl + "Refl".len() <= span.end);
    let mut env = matrix_env();
    env.elaborate_file(CANCEL_ACCEPTED)
        .expect("fresh whole accepted client, including same-form true premise, checks");
    match env.elaborate_file(CANCEL_FALSE_SUM) {
        Err(ElabError::TypeMismatch {
            reason,
            span: failure,
        }) if reason == "Refl expects an `Eq`-shaped goal"
            && failure.start == refl
            && failure.end == refl + "Refl".len() => {}
        other => panic!("false sum premise must reject at its Refl: {other:?}"),
    }
}

const ZERO_SUM_ACCEPTED: &str = r#"import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_zero, matrix_add, matrix_zero,
   vector_add_eq_zero_left, vector_add_eq_zero_right,
   matrix_add_eq_zero_left, matrix_add_eq_zero_right)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1

fn vector2 (x : Nat) (y : Nat) : Vec Nat n2 =
  VCons Nat n1 x (VCons Nat n0 y (VNil Nat))

fn matrix2 (r0 : Vec Nat n2) (r1 : Vec Nat n2) : Vec (Vec Nat n2) n2 =
  VCons (Vec Nat n2) n1 r0 (VCons (Vec Nat n2) n0 r1 (VNil (Vec Nat n2)))

const zero2 : Vec Nat n2 = vector2 n0 n0
const one_zero : Vec Nat n2 = vector2 n1 n0
const zero22 : Vec (Vec Nat n2) n2 = matrix2 zero2 zero2
const one_zero22 : Vec (Vec Nat n2) n2 = matrix2 one_zero zero2

theorem open_vector_left
    (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (sum_zero : Equal (Vec Nat n) (vector_add n xs ys) (vector_zero n))
  : Equal (Vec Nat n) xs (vector_zero n) =
  vector_add_eq_zero_left n xs ys sum_zero

theorem open_vector_right
    (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (sum_zero : Equal (Vec Nat n) (vector_add n xs ys) (vector_zero n))
  : Equal (Vec Nat n) ys (vector_zero n) =
  vector_add_eq_zero_right n xs ys sum_zero

theorem open_matrix_left
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (sum_zero : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_zero m n))
  : Equal (Vec (Vec Nat n) m) a (matrix_zero m n) =
  matrix_add_eq_zero_left m n a b sum_zero

theorem open_matrix_right
    (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (sum_zero : Equal (Vec (Vec Nat n) m) (matrix_add m n a b) (matrix_zero m n))
  : Equal (Vec (Vec Nat n) m) b (matrix_zero m n) =
  matrix_add_eq_zero_right m n a b sum_zero

theorem vector_left_zero_by_law : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_left n2 zero2 zero2 Refl

theorem vector_right_zero_by_law : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_right n2 zero2 zero2 Refl

theorem matrix_left_zero_by_law
  : Equal (Vec (Vec Nat n2) n2) zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_left n2 n2 zero22 zero22 Refl

theorem matrix_right_zero_by_law
  : Equal (Vec (Vec Nat n2) n2) zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_right n2 n2 zero22 zero22 Refl

theorem one_zero_sum_is_not_zero_sum
  : Equal (Vec Nat n2) (vector_add n2 one_zero zero2) one_zero =
  Refl
"#;

const ZERO_SUM_REFUSED_LEFT: &str = r#"
theorem vector_left_zero_by_law_false
  : Equal (Vec Nat n2) one_zero (vector_zero n2) =
  vector_add_eq_zero_left n2 one_zero zero2 Refl
"#;

const ZERO_SUM_REFUSED_RIGHT: &str = r#"
theorem vector_left_zero_by_law_false_premise
  : Equal (Vec Nat n2) zero2 (vector_zero n2) =
  vector_add_eq_zero_left n2 zero2 one_zero Refl
"#;

const ZERO_SUM_REFUSED_MATRIX: &str = r#"
theorem matrix_left_zero_by_law_false
  : Equal (Vec (Vec Nat n2) n2) one_zero22 (matrix_zero n2 n2) =
  matrix_add_eq_zero_left n2 n2 one_zero22 zero22 Refl
"#;

/// SOURCE: Architect AC-0 AC-2 clients; Linear Algebra §23 contract routed
/// separately. PROMISE CLASS: durable invariant.
/// MEASURED: four polymorphic and four closed clients cite their respective
/// NatMatrix-owned Transparent laws; the nonzero vector sum computes [1, 0].
/// CLAIMED: each law takes exactly the zero-sum premise and recovers the
/// indicated summand at open dimensions. Private refactoring is compatible;
/// dropping the premise or swapping the conclusion breaks the open clients.
/// THE GAP: all closed zero-sum rows have two zero operands; those rows alone
/// cannot distinguish a swapped summand. This is not official seed execution.
#[test]
fn zero_sum_laws_check_open_and_closed_summands() {
    let mut env = matrix_env();
    env.elaborate_file(ZERO_SUM_ACCEPTED)
        .expect("complete zero-sum accepted client checks");
    for (owner, open, closed) in [
        (
            "vector_add_eq_zero_left",
            "open_vector_left",
            "vector_left_zero_by_law",
        ),
        (
            "vector_add_eq_zero_right",
            "open_vector_right",
            "vector_right_zero_by_law",
        ),
        (
            "matrix_add_eq_zero_left",
            "open_matrix_left",
            "matrix_left_zero_by_law",
        ),
        (
            "matrix_add_eq_zero_right",
            "open_matrix_right",
            "matrix_right_zero_by_law",
        ),
    ] {
        let owner = env.globals[&format!("{MATRIX}.{owner}")];
        assert!(matches!(
            env.env.lookup(owner),
            Some(Decl::Transparent { .. })
        ));
        for client in [open, closed] {
            let (_, body) = env
                .env
                .transparent_body(env.globals[client])
                .expect("client has a checked body");
            assert!(
                term_contains_const(&body, owner),
                "{client} cites its exact law owner"
            );
        }
    }
}

/// SOURCE: Architect AC-0 AC-2 refusals; Linear Algebra §23.
/// PROMISE CLASS: durable invariant.
/// MEASURED: three distinct appended declarations on fresh accepted prefixes
/// each refuse at their own premise `Refl`. The nonzero right operand row
/// still has a true conclusion because its left operand is zero.
/// CLAIMED: a false zero-sum premise cannot invoke the left law. New laws
/// and proof refactoring are compatible; accepting the false premise is not.
/// THE GAP: these finite refusals locate the premise check, not its proof at
/// arbitrary dimensions. The open clients check the generic signature.
#[test]
fn false_zero_sums_refuse_at_their_own_premise_refl() {
    for (case, suffix) in [
        ("nonzero left vector", ZERO_SUM_REFUSED_LEFT),
        ("nonzero right vector", ZERO_SUM_REFUSED_RIGHT),
        ("nonzero left matrix", ZERO_SUM_REFUSED_MATRIX),
    ] {
        let declarations = parse_decls(suffix).expect("isolated refusal parses");
        let [SourceDecl::TheoremDecl { span, .. }] = declarations.as_slice() else {
            panic!("{case}: suffix must be exactly one theorem");
        };
        let refl = suffix.rfind("Refl").expect("negative premise uses Refl");
        assert!(refl >= span.start && refl + "Refl".len() <= span.end);
        assert!(suffix[refl + "Refl".len()..].trim().is_empty());
        let expected = ZERO_SUM_ACCEPTED.len() + refl;
        let mut env = matrix_env();
        let source = format!("{ZERO_SUM_ACCEPTED}{suffix}");
        match env.elaborate_file(&source) {
            Err(ElabError::TypeMismatch { span: failure, .. })
                if failure.start == expected && failure.end == expected + "Refl".len() =>
            {
                eprintln!("{case}: TypeMismatch at premise Refl {failure:?}");
            }
            other => panic!("{case}: false zero sum must reject at its premise Refl: {other:?}"),
        }
    }
}

/// SOURCE: Architect AC-0 private-helper ruling for Linear Algebra §23.
/// PROMISE CLASS: durable invariant.
/// MEASURED: neither checked helper can be selectively imported by qualified
/// identity. CLAIMED: the cons steps are private, not supported client APIs.
/// Compatible proof rewrites retain privacy; exporting either step breaks
/// the pin. THE GAP: the separate roster test checks the entire export set.
#[test]
fn zero_sum_cons_helpers_stay_private() {
    let mut env = matrix_env();
    for helper in ["vector_add_eq_zero_cons", "matrix_add_eq_zero_cons"] {
        let qualified = format!("{MATRIX}.{helper}");
        let id = env.globals[&qualified];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, qualified),
            other => panic!("{qualified} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: Architect AC-0 §6 and pending spec/50-stdlib/65-linear-algebra.md
/// §21. PROMISE CLASS: durable invariant.
/// MEASURED: the NatMatrix-owned Transparent laws are used by open selective-
/// import clients at arbitrary dimensions; both checked client bodies cite
/// the exact owner GlobalId. Closed factor-two and factor-Zero observations,
/// nonempty law applications, and an empty vector application check together.
/// All three private helpers refuse selective import. The inventory row above
/// independently compares exact provider-first trusted GlobalId identities.
/// CLAIMED: successor scaling cancels on vectors and matrices without new
/// trust or an exported helper. THE GAP: inherited provider trust remains;
/// these finite values do not replace the two generic checked proofs.
#[test]
fn successor_scale_cancel_public_clients_and_opposite_values_check() {
    let mut env = matrix_env();
    env.elaborate_file(SCALE_ACCEPTED)
        .expect("complete accepted scale-cancellation prefix checks");
    for (name, client) in [
        ("vector_scale_cancel", "vector_scale_open"),
        ("matrix_scale_cancel", "matrix_scale_open"),
    ] {
        let id = env.globals[&format!("{MATRIX}.{name}")];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("checked client");
        assert!(
            term_contains_const(&body, id),
            "{client} cites its exact law owner"
        );
    }
    for name in [
        "vector_scale_two_value",
        "vector_scale_zero_equal",
        "vector_scale_zero_value",
        "matrix_scale_zero_equal",
        "vector_scale_closed",
        "matrix_scale_closed",
        "vector_scale_empty",
        "vector_cancel_at_factor",
        "matrix_cancel_at_factor",
    ] {
        assert!(matches!(
            env.env.lookup(env.globals[name]),
            Some(Decl::Transparent { .. })
        ));
    }
    for helper in ["map_injective", "map_view", "mul_cancel_left"] {
        let id = env.globals[&format!("{MATRIX}.{helper}")];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MATRIX}.{helper}"));
            }
            other => panic!("{helper} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: Architect AC-0 §6 and pending spec 65 §21.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each factor-Zero refusal starts after a fresh entire accepted
/// prefix. The same-form factor-two clients check, while the otherwise
/// unchanged vector and matrix applications with Zero rather than two reject
/// as KernelRejected TypeMismatch in their own theorem declarations. Direct
/// closed Refl proves both factor-Zero premises, and a separate Refl refusal
/// establishes that the common v12 = v34 conclusion is false on these inputs.
/// CLAIMED: successor factor is required for cancellation; Zero is not enough.
/// THE GAP: rejection spans cover whole declarations, not just the h token;
/// the independent value controls constrain the reason for the contrast.
#[test]
fn zero_factor_vector_and_matrix_cancel_refuse_after_fresh_controls() {
    for (source, name) in [
        (SCALE_VECTOR_ZERO_REFUSED, "vector_cancel_at_zero"),
        (SCALE_MATRIX_ZERO_REFUSED, "matrix_cancel_at_zero"),
    ] {
        let parsed = parse_decls(source).expect("isolated factor-Zero refusal parses");
        let [SourceDecl::TheoremDecl {
            name: parsed_name,
            span: claim,
            ..
        }] = parsed.as_slice()
        else {
            panic!("exactly one false factor-Zero theorem");
        };
        assert_eq!(parsed_name, name);
        let mut env = matrix_env();
        env.elaborate_file(SCALE_ACCEPTED)
            .expect("fresh complete factor-two and Zero-value controls check");
        match env
            .elaborate_file(source)
            .expect_err("factor-Zero law use rejects")
        {
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            } if span.start >= claim.start && span.end <= claim.end && span.start < span.end => {}
            other => panic!("{name} must reject at its own theorem: {other:?}"),
        }
    }
    let mut env = matrix_env();
    env.elaborate_file(SCALE_ACCEPTED)
        .expect("fresh accepted prefix for independent false conclusion");
    let refl = SCALE_FALSE_CONCLUSION
        .rfind("Refl")
        .expect("false proof token");
    match env
        .elaborate_file(SCALE_FALSE_CONCLUSION)
        .expect_err("distinct vector fixtures are not equal")
    {
        ElabError::TypeMismatch { reason, span }
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start == refl
                && span.end == refl + "Refl".len() => {}
        other => panic!("distinct vector conclusion must fail at Refl: {other:?}"),
    }
}

fn gram_seed_sources() -> (&'static str, &'static str) {
    let mut sources = extract_ken_md(GRAM_SEED)
        .expect("§22 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &GRAM_SEED[fence.body_range]);
    (
        sources.next().expect("accepted §22 client"),
        sources.next().expect("isolated false trace premise client"),
    )
}

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §22 and
/// conformance/stdlib/linear-algebra/seed-gram-trace-zero.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: both open clients cite the root-loaded Transparent law owners;
/// closed square and rectangular applications and independent value checks
/// elaborate, while the internal row proof refuses selective import.
/// CLAIMED: the laws are reusable at independent dimensions, with no public
/// proof helper. THE GAP: the closed cases alone would not prove the laws.
/// Private proof refactors and unrelated exports stay green; changing the law
/// types, closed results, or helper visibility fails.
#[test]
fn gram_laws_check_open_and_closed_clients_and_keep_the_row_helper_private() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let owned = env
        .elaborate_module_from_roots(&[root()], MATRIX)
        .expect("NatMatrix package roots-loads");
    let (accepted, _) = gram_seed_sources();
    env.elaborate_file(accepted)
        .expect("all §22 accepted seed controls check");
    for (law, clients) in [
        (
            "matrix_zero_of_trace_gram_zero",
            &[
                "open_gram_trace_zero",
                "zero_gram_recovers_input",
                "wide_gram_recovers_input",
            ][..],
        ),
        (
            "transpose_gram",
            &["open_gram_symmetry", "skew_gram_symmetric"][..],
        ),
    ] {
        let id = env.globals[&format!("{MATRIX}.{law}")];
        assert!(owned.contains(&id), "NatMatrix owns {law}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        for client in clients {
            let (_, body) = env
                .env
                .transparent_body(env.globals[*client])
                .expect("checked client");
            assert!(term_contains_const(&body, id), "{client} cites {law}");
        }
    }
    for value_check in [
        "zero_gram_trace",
        "one_entry_gram_trace",
        "skew_transpose_value",
        "skew_gram_value",
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[value_check])
            .expect("checked comparison with a literal value");
        for law in ["matrix_zero_of_trace_gram_zero", "transpose_gram"] {
            assert!(
                !term_contains_const(&body, env.globals[&format!("{MATRIX}.{law}")]),
                "{value_check} must check without citing {law}"
            );
        }
    }
    let helper = "row_dot_sum_self_zero";
    match env.elaborate_file(&format!("import {MATRIX} ({helper})")) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, format!("{MATRIX}.{helper}")),
        other => panic!("{helper} must refuse selective import: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/65-linear-algebra.md §22 and
/// conformance/stdlib/linear-algebra/seed-gram-trace-zero.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after a fresh accepted prefix, the false trace-zero proof
/// supplies Top where the kernel expects a type convertible to Bottom.
/// CLAIMED: the zero-trace law cannot be applied with a false trace premise.
/// THE GAP: the span locates the declaration, not the proof token; the
/// accepted one-entry trace of one and the mismatch types identify the premise.
/// Additional accepted controls stay green; admitting the false premise or
/// refusing for an unrelated reason fails.
#[test]
fn gram_seed_refuses_false_trace_premise() {
    let (accepted, source) = gram_seed_sources();
    let parsed = parse_decls(source).expect("isolated §22 refusal parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("expected a single isolated trace-premise theorem");
    };
    assert_eq!(name, "refuse_nonzero_gram_trace");
    let mut env = matrix_env();
    env.elaborate_file(accepted)
        .expect("fresh accepted §22 prefix checks before refusal");
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found },
            span,
        }) if span.start >= claim.start && span.end <= claim.end && span.start < span.end => {
            let ctx = ken_kernel::Context::default();
            let top = Term::const_(env.env.top_id(), vec![]);
            let bottom = Term::const_(env.env.bottom_id(), vec![]);
            assert!(ken_kernel::convert_type(&env.env, &ctx, &found, &top));
            assert!(ken_kernel::convert_type(&env.env, &ctx, &expected, &bottom));
            eprintln!("{name}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{name} must refuse at its own theorem: {other:?}"),
    }
}
