//! LANG-ELAB-NESTED-FORMER-RECURSION acceptance.
//!
//! Layer-2 of the nested-former-recursion capability (layer-1 kernel former-lift
//! landed as KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE, 486e9f33): the elaborator
//! now SURFACES the nested induction hypothesis the kernel builds for a recursive
//! occurrence nested one positive former deeper than direct (`List (Pair String
//! Self)`), so `recursive result for <field>` reaches it and a total fold over
//! such a value elaborates and executes. Totality is preserved exactly: the fix
//! surfaces an evidence binder that exists ONLY for a positive recursive position
//! (`derive_carrier_shape` is positivity-gated), and a higher-order (Pi / W-style)
//! evidence stays out of scope — see `lang_structural_result_elab.rs`
//! (`d2_wstyle_without_a_trailing_result_binder_is_exactly_out_of_scope`) and the
//! SCT non-subterm rejection, both unchanged by this fix.

use ken_elaborator::{error::ElabError, ElabEnv};
use ken_interp::eval::{eval, EvalStore};
use ken_kernel::{Decl, KernelError, Term};

const ADD: &str =
    "fn add (x : Nat) (y : Nat) : Nat = match x { Zero |-> y ; Suc x2 |-> Suc (add x2 y) }\n";

// A recursive occurrence one positive former deeper than direct: the `NRose`
// inside `NNode` is buried in `Pair String _` inside `List`.
const NESTED_TYPE: &str = "\
data NRose : Type where { NLeaf : NRose ; NNode : List (Pair String NRose) -> NRose }\n";

// The direct-occurrence comparator: `List Self` with no intervening former.
const DIRECT_TYPE: &str = "\
data DRose : Type where { DLeaf : DRose ; DNode : List DRose -> DRose }\n";

// A total fold over the nested value. `recursive result for member`
// (member : Pair String NRose) is the former-nested occurrence this WP admits;
// `recursive result for rest` (rest : List (Pair String NRose)) is the list
// continuation. Counts the `NLeaf` leaves reachable through the object nesting.
const NESTED_FOLD: &str = "\
fn nsize (r : NRose) : Nat = match r {\n\
  NLeaf |-> Suc Zero ;\n\
  NNode members |-> match members {\n\
    Nil |-> Zero ;\n\
    Cons member rest |-> add (recursive result for member) (recursive result for rest)\n\
  }\n\
}\n";

// A concrete two-leaf object value: NNode [ ("x", NLeaf), ("y", NLeaf) ].
const NESTED_VALUE: &str = "\
const two_leaves : NRose =\n\
  NNode (Cons (Pair String NRose)\n\
    (mk_pair String NRose \"x\" NLeaf)\n\
    (Cons (Pair String NRose) (mk_pair String NRose \"y\" NLeaf) (Nil (Pair String NRose))))\n\
const nested_result : Nat = nsize two_leaves\n";

fn nat_count(env: &ElabEnv, value: &ken_interp::eval::EvalVal) -> u64 {
    use ken_interp::eval::EvalVal;
    match value {
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_count(env, &args[0])
        }
        other => panic!("expected Nat, got {other:?}"),
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    use ken_interp::eval::ListCharIds;
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        store.num_values.insert(
            *id,
            match value {
                ken_elaborator::NumericLitVal::Int(n) => ken_interp::eval::EvalVal::from(n.clone()),
                ken_elaborator::NumericLitVal::Str(s) => ken_interp::eval::EvalVal::Str(s.clone()),
                ken_elaborator::NumericLitVal::Bytes(b) => {
                    ken_interp::eval::EvalVal::Bytes(b.clone())
                }
                ken_elaborator::NumericLitVal::Float(f) => ken_interp::eval::EvalVal::Float(*f),
                ken_elaborator::NumericLitVal::Float32(f) => {
                    ken_interp::eval::EvalVal::Float32(*f)
                }
                ken_elaborator::NumericLitVal::Decimal { coeff, exp } => {
                    ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
                }
            },
        );
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_nat(env: &ElabEnv, name: &str) -> u64 {
    let mut store = make_store(env);
    let id = env.globals[name];
    let value = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, &mut store),
        other => panic!("`{name}` must be transparent, got {other:?}"),
    };
    nat_count(env, &value)
}

/// Promise class: durable invariant (the capability).
///
/// MEASURED: a total fold over `List (Pair String Self)` (the former-nested
/// occurrence) elaborates and kernel-checks, and the direct `List Self` fold
/// stays admitted. CLAIMED: the elaborator surfaces the nested induction
/// hypothesis so `recursive result for` reaches an occurrence one positive
/// former deeper than direct. THE GAP: runtime consumption is pinned separately
/// by the execution test below.
#[test]
fn ac_nested_former_fold_admitted_direct_stays_green() {
    let mut env = ElabEnv::new().unwrap();
    let trusted_before: std::collections::BTreeSet<_> =
        env.env.trusted_base().into_iter().collect();

    env.elaborate_file(&format!("{ADD}{NESTED_TYPE}{NESTED_FOLD}"))
        .expect("nested-former total fold must elaborate");
    assert!(env.globals.contains_key("nsize"));

    let mut direct = ElabEnv::new().unwrap();
    direct
        .elaborate_file(&format!(
            "{ADD}{DIRECT_TYPE}fn dsize (r : DRose) : Nat = match r {{ \
             DLeaf |-> Suc Zero ; \
             DNode kids |-> match kids {{ \
               Nil |-> Zero ; \
               Cons head tail |-> add (recursive result for head) (recursive result for tail) \
             }} }}\n"
        ))
        .expect("direct List Self fold must stay admitted (no regression)");
    assert!(direct.globals.contains_key("dsize"));

    // AC-NO-TCB-WIDENING: surfacing an already-built IH adds no trusted decl.
    let trusted_after: std::collections::BTreeSet<_> =
        env.env.trusted_base().into_iter().collect();
    assert_eq!(
        trusted_after, trusted_before,
        "surfacing the nested IH must add zero trusted-base declarations"
    );
}

/// Promise class: durable invariant (AC-EXECUTES-AT-RUNTIME, layer-3 pre-empt).
///
/// MEASURED: the admitted nested object fold runs to a concrete Nat via the
/// interpreter, and a mutation that drops the nested-IH consumption produces a
/// different value. CLAIMED: the generated nested IH is actually CONSUMED at
/// runtime, not merely admitted. THE GAP: this drives the interpreter; native
/// agreement is the runtime lane's separate gate.
#[test]
fn ac_executes_at_runtime_and_dropping_nested_ih_changes_the_value() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(&format!("{ADD}{NESTED_TYPE}{NESTED_FOLD}{NESTED_VALUE}"))
        .expect("nested fold + value must elaborate");
    assert_eq!(
        eval_nat(&env, "nested_result"),
        2,
        "the nested object fold must count both leaves through the Pair-nested recursion"
    );

    // Reaching mutation: the same fold that DROPS the nested-IH consumption
    // (`recursive result for member`) can only ever return the list-tail result,
    // so it cannot count the Pair-nested leaves — a different runtime value.
    let mut dropped = ElabEnv::new().unwrap();
    dropped
        .elaborate_file(&format!(
            "{ADD}{NESTED_TYPE}\
             fn nsize (r : NRose) : Nat = match r {{ \
               NLeaf |-> Suc Zero ; \
               NNode members |-> match members {{ \
                 Nil |-> Zero ; \
                 Cons member rest |-> recursive result for rest \
               }} }}\n{NESTED_VALUE}"
        ))
        .expect("the dropped-IH variant still elaborates");
    assert_eq!(
        eval_nat(&dropped, "nested_result"),
        0,
        "dropping the nested-IH consumption must lose the leaf count (0), proving the \
         nested IH is what carries the runtime result"
    );
}

/// Promise class: durable invariant (AC-TOTALITY-PRESERVED — the soundness gate).
///
/// MEASURED: a NEGATIVE-position occurrence one former deeper is rejected at
/// positivity admission, so no evidence binder is ever built for it and no fold
/// over it can manufacture a decrease. CLAIMED: the fix admits ONLY positive
/// former-nested subterms. THE GAP: the higher-order (W-style) positive case
/// stays out of scope and the non-subterm self-loop stays NotTerminating —
/// both are pinned unchanged in `lang_structural_result_elab.rs` and the SCT
/// suites; this pins the negative-position boundary the new admission must not
/// cross.
#[test]
fn ac_totality_preserved_negative_position_is_rejected() {
    let mut env = ElabEnv::new().unwrap();
    // `NRose` in the DOMAIN of a function nested in the Pair is a negative
    // occurrence: it must fail positivity admission, never build an IH.
    let result = env.elaborate_file(
        "data BadRose : Type where { \
         BLeaf : BadRose ; \
         BNode : List (Pair (BadRose -> Bool) String) -> BadRose }\n",
    );
    assert!(
        matches!(
            result,
            Err(ElabError::KernelRejected {
                error: KernelError::PositivityViolation(_),
                ..
            })
        ),
        "a negative-position occurrence nested in a former must be rejected at \
         positivity, got {result:?}"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: the completed nested fold's `nsize` global is a real transparent,
/// kernel-checked declaration whose body is an elimination — i.e. the surface
/// fold lowered to an ordinary eliminator, not a postulate or a stuck term.
/// CLAIMED: the admission is by ordinary structural elaboration. THE GAP: the
/// exact eliminated family identity is an internal detail; this pins only that a
/// real checked elimination was produced.
#[test]
fn ac_nested_fold_lowers_to_a_checked_elimination() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_file(&format!("{ADD}{NESTED_TYPE}{NESTED_FOLD}"))
        .expect("nested fold must elaborate");
    let (_, mut body) = env
        .env
        .transparent_body(env.globals["nsize"])
        .expect("nsize must be transparent");
    while let Term::Lam(_, next) = body {
        body = *next;
    }
    assert!(
        matches!(body, Term::Elim { .. }),
        "nsize must lower to a real match elimination, got {body:?}"
    );
}
