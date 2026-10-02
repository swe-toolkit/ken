//! Inference discovers R once before an indexed method's first IH, and
//! reuses both that leaf and any literal plans already made on the descent.
//! Spec: spec/30-surface/34-data-match.md §3.1–3.2, §4.4.
//! Promise class: durable value and single-elaboration invariants.

use ken_elaborator::{ElabEnv, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{env::PrimReduction, normalize, Context, Decl, Term};

const VEC: &str = "data Vec (a : Type) : Nat → Type where { \
    VNil : Vec a Zero; VCons : (n : Nat) → a → Vec a n → Vec a (Suc n) }";

fn literal_declarations(env: &ElabEnv) -> usize {
    env.env.declarations().iter().filter(|decl| {
        matches!(decl, Decl::Primitive { reduction: PrimReduction::Literal, .. })
    }).count()
}

fn normal(env: &ElabEnv, name: &str) -> Term {
    let id = env.globals[name];
    let body = env.env.transparent_body(id).expect("checked value").1;
    normalize(&env.env, &Context::new(), &body)
}

fn check_literal_result(source: &str, label: &str) -> (ElabEnv, ken_kernel::GlobalId) {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_file(source).unwrap_or_else(|e| panic!("{label}: {e:?}"));
    assert_eq!(env.env.trusted_base(), trusted, "{label}: literal changed trust");
    let Term::Const { id, .. } = normal(&env, "observed") else {
        panic!("{label}: the match did not reduce to its literal leaf")
    };
    (env, id)
}

#[test]
fn vcons_first_infers_string_and_decimal_once_before_ih() {
    // MEASURED: inferred VCons-first values are literal payloads and mint
    // exactly as many literal declarations as an otherwise identical leaf
    // elaborated in a nonrecursive Box bucket. CLAIMED: discovery reuses its
    // first leaf rather than minting it again after R is seeded. THE GAP:
    // the independent Box control shares the RHS spelling and one leaf, but
    // has no IH; only the indexed fixture exercises first-leaf replay.
    for (result_ty, rhs) in [("String", "\"once\""), ("Decimal", "12.5d")] {
        let indexed = format!(
            "{VEC}\nfn choose (n : Nat) (xs : Vec Nat (Suc n)) : {result_ty} = \
             let r = match xs {{ VCons m e tl ↦ {rhs} }} in r\n\
             const observed : {result_ty} = choose Zero \
               (VCons Nat Zero Zero (VNil Nat))"
        );
        let flat = format!(
            "{VEC}\ndata BoxNat : Type where {{ MkBoxNat : Nat → BoxNat }}\n\
             fn choose (b : BoxNat) : {result_ty} = \
               let r = match b {{ MkBoxNat v ↦ {rhs} }} in r\n\
             const observed : {result_ty} = choose (MkBoxNat Zero)"
        );
        let (indexed_env, indexed_id) = check_literal_result(&indexed, "VCons-first");
        let (flat_env, flat_id) = check_literal_result(&flat, "non-IH Box");
        assert_eq!(literal_declarations(&indexed_env), literal_declarations(&flat_env),
            "{result_ty}: replay must not mint a second RHS literal");
        for (label, env, id) in [
            ("VCons-first", &indexed_env, indexed_id),
            ("non-IH Box", &flat_env, flat_id),
        ] {
            match (result_ty, env.num_values.get(&id)) {
                ("String", Some(NumericLitVal::Str(text))) if text.as_str() == "once" => {}
                ("Decimal", Some(NumericLitVal::Decimal { coeff, exp }))
                    if coeff.to_string() == "125" && *exp == -1 => {}
                (_, other) => panic!("{label}: wrong {result_ty} literal payload: {other:?}"),
            }
        }
    }
}

fn eval_nat(env: &ElabEnv, name: &str) -> usize {
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    for (id, value) in &env.num_values {
        let value = match value {
            NumericLitVal::Int(value) => EvalVal::from(value.clone()),
            NumericLitVal::Float(value) => EvalVal::Float(*value),
            NumericLitVal::Float32(value) => EvalVal::Float32(*value),
            NumericLitVal::Decimal { coeff, exp } => ken_interp::decimal_value(
                env.prelude_env.mkdecimalpair_id,
                coeff.clone(),
                *exp,
            ),
            NumericLitVal::Str(value) => EvalVal::Str(value.clone()),
            NumericLitVal::Bytes(value) => EvalVal::Bytes(value.clone()),
        };
        store.num_values.insert(*id, value);
    }
    let id = env.globals[name];
    let body = env.env.transparent_body(id).expect("checked value").1;
    fn count(value: EvalVal, zero: ken_kernel::GlobalId, suc: ken_kernel::GlobalId) -> usize {
        match value {
            EvalVal::Ctor { id, args, .. } if id == zero && args.is_empty() => 0,
            EvalVal::Ctor { id, args, .. } if id == suc && args.len() == 1 => {
                1 + count(args[0].clone(), zero, suc)
            }
            other => panic!("expected Nat result, got {other:?}"),
        }
    }
    count(eval(&[], &body, &env.env, &mut store), env.globals["Zero"], env.globals["Suc"])
}

#[test]
fn vcons_first_string_pattern_reuses_descent_comparator_plan() {
    // MEASURED: literal declarations equal a non-IH bucket with the same
    // pattern and both closed calls normalize to their respective branches.
    // CLAIMED: replay must reuse its already checked comparator plan, never
    // mint the pattern's String a second time. THE GAP: the values alone
    // cannot see duplicate literal identities; the count relation does.
    let indexed = format!(
        "{VEC}\nfn select (n : Nat) (xs : Vec String (Suc n)) : Nat = \
           let r = match xs {{ \
             VCons m \"a\" tl ↦ Suc (Suc (Suc Zero)); \
             VCons m _ tl ↦ Zero \
           }} in r\n\
           const yes : Nat = select Zero \
             (VCons String Zero \"a\" (VNil String))\n\
           const no : Nat = select Zero \
             (VCons String Zero \"b\" (VNil String))\n\
           const expected_yes : Nat = Suc (Suc (Suc Zero))\n\
           const expected_no : Nat = Zero"
    );
    let flat = format!(
        "{VEC}\ndata BoxString : Type where {{ MkBoxString : String → BoxString }}\n\
           fn select (b : BoxString) : Nat = \
             let r = match b {{ \
               MkBoxString \"a\" ↦ Suc (Suc (Suc Zero)); \
               MkBoxString _ ↦ Zero \
             }} in r\n\
           const yes : Nat = select (MkBoxString \"a\")\n\
           const no : Nat = select (MkBoxString \"b\")\n\
           const expected_yes : Nat = Suc (Suc (Suc Zero))\n\
           const expected_no : Nat = Zero"
    );
    let mut indexed_env = ElabEnv::new().expect("prelude");
    indexed_env.elaborate_file(&indexed).expect("indexed comparator");
    let mut flat_env = ElabEnv::new().expect("prelude");
    flat_env.elaborate_file(&flat).expect("non-IH comparator");
    for (label, env) in [("indexed", &indexed_env), ("non-IH", &flat_env)] {
        assert_eq!(eval_nat(env, "yes"), 3, "{label} yes");
        assert_eq!(eval_nat(env, "no"), 0, "{label} no");
    }
    assert_eq!(literal_declarations(&indexed_env), literal_declarations(&flat_env),
        "the pattern comparator must be minted once before IH replay");
}
