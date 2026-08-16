//! `V3-FO-KEN-LEVEL-CHECKER-AUTHORING`, increments 1-2 (`D0`+`D1`, then
//! `D2`).
//!
//! Increment 1: a differential control between the Ken-authored `fok_embed`
//! (`catalog/packages/Tooling/Verification/FoKripke.ken`) and the Rust
//! reference `ken_elaborator::fo_kripke::embed`, on the same quoted `IForm`
//! inputs. The Rust side is the REFERENCE for shape only (`23 §4.3-§4.5`);
//! the Ken source was authored independently and is checked here for
//! agreement, never copied from Rust behavior.
//!
//! Increment 2: `D2`, `fok_check_cert`. No Rust/Ken differential yet (`D3`,
//! a separate increment) -- these controls establish standalone (a) that
//! the checker and its constructors kernel-check, (b) a positional-order
//! round-trip on the two same-typed-adjacent-field constructors outside
//! `embed`'s image (`FokMkSequent`, `FokInit`), which no arity/constructor-
//! count control can see, and (c) that `fok_check_cert` both accepts a
//! genuine derivation and totally rejects a malformed certificate at every
//! guard in `fok_check_tree`/`fok_check_rule` -- individually, not as
//! `D3`'s forthcoming near-miss pairs.

use std::collections::BTreeSet;

use ken_elaborator::fo_kripke::{embed, Form, IForm, IVar, QTerm};
use ken_elaborator::ElabEnv;
use ken_interp::{eval, EvalStore, EvalVal};
use ken_kernel::GlobalId;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

struct FokIds {
    fbottom: GlobalId,
    faccess: GlobalId,
    fdomain_a: GlobalId,
    fforcing_p: GlobalId,
    fand: GlobalId,
    for_: GlobalId,
    fimp: GlobalId,
    fforall_world: GlobalId,
    fforall_obj: GlobalId,
    qbound: GlobalId,
    qparameter: GlobalId,
    zero: GlobalId,
    suc: GlobalId,
    nil: GlobalId,
    cons: GlobalId,
    fmk_sequent: GlobalId,
    fok_init: GlobalId,
    ktrue: GlobalId,
    kfalse: GlobalId,
}

impl FokIds {
    fn resolve(env: &ElabEnv) -> Self {
        let id = |name: &str| {
            *env.globals
                .get(name)
                .unwrap_or_else(|| panic!("missing global {name}"))
        };
        FokIds {
            fbottom: id("FokBottom"),
            faccess: id("FokAccess"),
            fdomain_a: id("FokDomainA"),
            fforcing_p: id("FokForcingP"),
            fand: id("FokAnd"),
            for_: id("FokOr"),
            fimp: id("FokImp"),
            fforall_world: id("FokForallWorld"),
            fforall_obj: id("FokForallObj"),
            qbound: id("FokQBound"),
            qparameter: id("FokQParameter"),
            zero: id("Zero"),
            suc: id("Suc"),
            nil: id("Nil"),
            cons: id("Cons"),
            fmk_sequent: id("FokMkSequent"),
            fok_init: id("FokInit"),
            ktrue: id("True"),
            kfalse: id("False"),
        }
    }
}

fn decode_nat(ids: &FokIds, v: &EvalVal) -> usize {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.zero => {
            assert!(args.is_empty(), "Zero must carry no args");
            0
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.suc => {
            assert_eq!(args.len(), 1, "Suc must carry exactly one arg");
            1 + decode_nat(ids, &args[0])
        }
        other => panic!("expected a Nat Ctor (Zero/Suc), got {other:?}"),
    }
}

fn decode_qterm(ids: &FokIds, v: &EvalVal) -> QTerm {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.qbound => {
            QTerm::Bound(decode_nat(ids, &args[0]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.qparameter => {
            QTerm::Parameter(decode_nat(ids, &args[0]))
        }
        other => panic!("expected a FokQTerm Ctor, got {other:?}"),
    }
}

fn decode_form(ids: &FokIds, v: &EvalVal) -> Form {
    match v {
        EvalVal::Ctor { id, .. } if *id == ids.fbottom => Form::Bottom,
        EvalVal::Ctor { id, args, .. } if *id == ids.faccess => {
            Form::Access(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fdomain_a => {
            Form::DomainA(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fforcing_p => {
            Form::ForcingP(decode_qterm(ids, &args[0]), decode_qterm(ids, &args[1]))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fand => Form::And(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.for_ => Form::Or(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.fimp => Form::Imp(
            Box::new(decode_form(ids, &args[0])),
            Box::new(decode_form(ids, &args[1])),
        ),
        EvalVal::Ctor { id, args, .. } if *id == ids.fforall_world => {
            Form::ForallWorld(Box::new(decode_form(ids, &args[0])))
        }
        EvalVal::Ctor { id, args, .. } if *id == ids.fforall_obj => {
            Form::ForallObj(Box::new(decode_form(ids, &args[0])))
        }
        other => panic!("expected a FokForm Ctor, got {other:?}"),
    }
}

/// Decodes a Ken `List FokForm` value. `Cons`/`Nil` are the prelude's
/// generic constructors -- confirmed empirically that a parametric
/// constructor's `args` carries the type witness FIRST (`args[0]`), so
/// `Cons`'s own fields are `args[1]` (head) and `args[2]` (tail); `Nil`
/// carries only the type witness.
fn decode_list_form(ids: &FokIds, v: &EvalVal) -> Vec<Form> {
    match v {
        EvalVal::Ctor { id, .. } if *id == ids.nil => Vec::new(),
        EvalVal::Ctor { id, args, .. } if *id == ids.cons => {
            let head = decode_form(ids, &args[1]);
            let mut rest = decode_list_form(ids, &args[2]);
            rest.insert(0, head);
            rest
        }
        other => panic!("expected a List FokForm Ctor, got {other:?}"),
    }
}

/// Decodes a `FokSequent` to `(gamma, delta)`, POSITIONALLY: `args[0]` is
/// whatever `FokMkSequent`'s first constructor argument decoded to,
/// `args[1]` the second -- this function does not itself know which is
/// "supposed" to be gamma, so a transposition in the `.ken` source's
/// constructor declaration is exactly what this decoder combined with a
/// fixed caller-side expectation is built to catch.
fn decode_sequent(ids: &FokIds, v: &EvalVal) -> (Vec<Form>, Vec<Form>) {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.fmk_sequent => {
            assert_eq!(args.len(), 2, "FokMkSequent must carry exactly two fields");
            (
                decode_list_form(ids, &args[0]),
                decode_list_form(ids, &args[1]),
            )
        }
        other => panic!("expected a FokSequent Ctor, got {other:?}"),
    }
}

/// Decodes a `FokRule::FokInit` value to `(left, right)`, positionally.
fn decode_init_rule(ids: &FokIds, v: &EvalVal) -> (usize, usize) {
    match v {
        EvalVal::Ctor { id, args, .. } if *id == ids.fok_init => {
            assert_eq!(args.len(), 2, "FokInit must carry exactly two fields");
            (decode_nat(ids, &args[0]), decode_nat(ids, &args[1]))
        }
        other => panic!("expected a FokInit Ctor, got {other:?}"),
    }
}

fn decode_bool(ids: &FokIds, v: &EvalVal) -> bool {
    match v {
        EvalVal::Bool(b) => *b,
        EvalVal::Ctor { id, .. } if *id == ids.ktrue => true,
        EvalVal::Ctor { id, .. } if *id == ids.kfalse => false,
        other => panic!("expected a Bool value, got {other:?}"),
    }
}

/// Elaborates `const {name} : Bool = {expr}` and evaluates/decodes it.
fn eval_bool(env: &mut ElabEnv, ids: &FokIds, name: &str, expr: &str) -> bool {
    let id = env
        .elaborate_decl(&format!("const {name} : Bool = {expr}"))
        .unwrap_or_else(|e| panic!("{name}: elaboration failed: {e}"));
    let (_, body) = env
        .env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("{name}: not a transparent definition"));
    let mut store = EvalStore::new();
    let value = eval(&[], &body, &env.env, &mut store);
    decode_bool(ids, &value)
}

/// Renders `n` as a `Nat` literal, e.g. `nat_source(2)` = `"(Suc (Suc
/// Zero))"`. The result is always a single argument-safe token: bare `Zero`
/// for `0`, fully parenthesized otherwise.
fn nat_source(n: usize) -> String {
    if n == 0 {
        "Zero".to_string()
    } else {
        format!("(Suc {})", nat_source(n - 1))
    }
}

/// Renders an `IForm` as Ken source text over the `FokIForm` constructors,
/// independently of `embed`/`fok_embed` -- this function only serializes the
/// INPUT, never computes the translation under test.
fn iform_source(f: &IForm) -> String {
    match f {
        IForm::Bottom => "FokIBottom".to_string(),
        IForm::Atom(IVar(k)) => format!("(FokIAtom (FokMkIVar {}))", nat_source(*k)),
        IForm::Or(p, q) => format!("(FokIOr {} {})", iform_source(p), iform_source(q)),
        IForm::Imp(p, q) => format!("(FokIImp {} {})", iform_source(p), iform_source(q)),
        IForm::Forall(p) => format!("(FokIForall {})", iform_source(p)),
    }
}

/// Elaborates `f` as a Ken `FokIForm` constant, evaluates `fok_embed` on it,
/// and decodes the result back to the Rust `Form` type for comparison.
fn ken_embed(env: &mut ElabEnv, ids: &FokIds, case: &str, f: &IForm) -> Form {
    let input_name = format!("fok_case_{case}_input");
    let output_name = format!("fok_case_{case}_output");
    env.elaborate_decl(&format!(
        "const {input_name} : FokIForm = {}",
        iform_source(f)
    ))
    .unwrap_or_else(|e| panic!("case {case}: input elaboration failed: {e}"));
    let output_id = env
        .elaborate_decl(&format!(
            "const {output_name} : FokForm = fok_embed {input_name}"
        ))
        .unwrap_or_else(|e| panic!("case {case}: fok_embed elaboration failed: {e}"));
    let (_, body) = env
        .env
        .transparent_body(output_id)
        .unwrap_or_else(|| panic!("case {case}: output is not a transparent definition"));
    let mut store = EvalStore::new();
    let value = eval(&[], &body, &env.env, &mut store);
    decode_form(ids, &value)
}

/// The five `IForm` constructors, individually and combined. `IForm::Atom`
/// is only well-scoped under an enclosing `IForm::Forall` (its `IVar` must
/// name a bound object variable) -- exactly the invariant `quote_iform`
/// enforces on its callers (`fo_kripke.rs`'s doc comment above `IVar`), so
/// every case below wraps its atoms accordingly; a bare top-level
/// `IForm::Atom` is not a valid slice input and `embed` panics on it
/// (confirmed empirically: the Rust reference indexes an empty
/// `object_env`). The two doubly-nested cases exercise `IVar` indices above
/// `0`, so `fok_nth_nat_default`'s `Suc` arm and the world/object shift
/// bookkeeping in both `w_forces` and `fok_w_forces` are exercised under
/// real nesting, not only at the trivial depth-zero index.
fn cases() -> Vec<(&'static str, IForm)> {
    vec![
        ("bottom", IForm::Bottom),
        (
            "atom_under_forall",
            IForm::Forall(Box::new(IForm::Atom(IVar(0)))),
        ),
        (
            "or",
            IForm::Forall(Box::new(IForm::Or(
                Box::new(IForm::Bottom),
                Box::new(IForm::Atom(IVar(0))),
            ))),
        ),
        (
            "imp",
            IForm::Forall(Box::new(IForm::Imp(
                Box::new(IForm::Atom(IVar(0))),
                Box::new(IForm::Bottom),
            ))),
        ),
        (
            "forall_nested_inner_ref",
            IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Atom(IVar(0)))))),
        ),
        (
            "forall_nested_outer_ref",
            IForm::Forall(Box::new(IForm::Forall(Box::new(IForm::Atom(IVar(1)))))),
        ),
        (
            "nested_mixed",
            IForm::Forall(Box::new(IForm::Imp(
                Box::new(IForm::Atom(IVar(0))),
                Box::new(IForm::Or(
                    Box::new(IForm::Atom(IVar(0))),
                    Box::new(IForm::Bottom),
                )),
            ))),
        ),
    ]
}

#[test]
fn fok_embed_agrees_with_rust_embed_on_quoted_inputs() {
    let mut env = ElabEnv::new().expect("prelude construction");
    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_trust, after_trust,
        "loading FoKripke.ken must add no trusted_base entry (AC-2 of the node)"
    );

    let ids = FokIds::resolve(&env);

    for (case, f) in cases() {
        let expected = embed(&f);
        let actual = ken_embed(&mut env, &ids, case, &f);
        assert_eq!(
            actual, expected,
            "case {case}: Ken fok_embed disagrees with Rust embed on {f:?}"
        );
    }
}

#[test]
fn fok_iform_has_exactly_five_slice_constructors() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ind = env
        .env
        .inductive(*env.globals.get("FokIForm").expect("FokIForm registered"))
        .expect("FokIForm is an inductive");
    assert_eq!(
        ind.constructors.len(),
        5,
        "FokIForm must carry exactly the five slice constructors (23 §4.5)"
    );
}

#[test]
fn fok_rule_has_exactly_three_slice_variants() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ind = env
        .env
        .inductive(*env.globals.get("FokRule").expect("FokRule registered"))
        .expect("FokRule is an inductive");
    assert_eq!(
        ind.constructors.len(),
        3,
        "FokRule must carry exactly the three slice variants, against the general \
         23 §4.3 Rule's ~20 (init, imp-right, forall-right only)"
    );
}

/// `FokMkSequent (List FokForm) (List FokForm)` and `FokInit Nat Nat` are
/// the two slice constructors with two ADJACENT fields of the SAME type,
/// outside `embed`'s image (`D1`'s differential never constructs either).
/// Elaboration cannot catch a transposed field order (both fields still
/// typecheck), and neither can an arity/constructor-count pin (the count is
/// unchanged either way) -- only a construct-with-distinguishable-contents,
/// decode-back, assert-positionally control can. `language-leader`,
/// `evt_3bb6c5kmphc0n`.
#[test]
fn fok_sequent_and_init_preserve_positional_field_order() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ids = FokIds::resolve(&env);
    let mut store = EvalStore::new();

    let seq_id = env
        .elaborate_decl(
            "const fok_case_positional_sequent : FokSequent = \
             FokMkSequent \
               (Cons FokForm FokBottom (Nil FokForm)) \
               (Cons FokForm (FokAccess (FokQBound Zero) (FokQBound Zero)) (Nil FokForm))",
        )
        .expect("positional sequent case: elaboration failed");
    let (_, seq_body) = env
        .env
        .transparent_body(seq_id)
        .expect("positional sequent case: not a transparent definition");
    let seq_value = eval(&[], &seq_body, &env.env, &mut store);
    let (gamma, delta) = decode_sequent(&ids, &seq_value);
    assert_eq!(
        gamma,
        vec![Form::Bottom],
        "FokMkSequent's FIRST constructor argument must decode as gamma"
    );
    assert_eq!(
        delta,
        vec![Form::Access(QTerm::Bound(0), QTerm::Bound(0))],
        "FokMkSequent's SECOND constructor argument must decode as delta, \
         not gamma's contents"
    );

    let init_id = env
        .elaborate_decl(
            "const fok_case_positional_init : FokRule = \
             FokInit (Suc (Suc (Suc Zero))) (Suc (Suc (Suc (Suc (Suc Zero)))))",
        )
        .expect("positional init case: elaboration failed");
    let (_, init_body) = env
        .env
        .transparent_body(init_id)
        .expect("positional init case: not a transparent definition");
    let init_value = eval(&[], &init_body, &env.env, &mut store);
    let (left, right) = decode_init_rule(&ids, &init_value);
    assert_eq!(
        left, 3,
        "FokInit's FIRST constructor argument must decode as left"
    );
    assert_eq!(
        right, 5,
        "FokInit's SECOND constructor argument must decode as right, not left's value"
    );
}

#[test]
fn fok_check_cert_kernel_checks_with_no_trusted_base_delta() {
    let mut env = ElabEnv::new().expect("prelude construction");
    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_trust, after_trust,
        "loading FoKripke.ken (including D2's fok_check_cert) must add no \
         trusted_base entry (AC-2 of the node)"
    );
    // `fok_check_cert` itself must be an ordinary kernel-checked total
    // function, ordinary enough that a fully-applied Bool constant built
    // from it elaborates/kernel-checks with no error.
    env.elaborate_decl(
        "const fok_check_cert_typechecks : Bool = \
         fok_check_cert FokBottom \
           (FokMkCert (FokMkSequent (Nil FokForm) (Cons FokForm FokBottom (Nil FokForm))) \
             (FokInit Zero Zero) (Nil FokCert))",
    )
    .expect("fok_check_cert must kernel-check when fully applied");
}

/// Two genuine derivations, proving the checker is not vacuously `False`:
/// `Bottom -> Bottom` via `imp-right` then `init`, and `forall w. Bottom ->
/// Bottom` via `forall-right` wrapping the same subderivation.
#[test]
fn fok_check_cert_accepts_genuine_derivations() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ids = FokIds::resolve(&env);

    let imp_bottom_bottom = "(FokImp FokBottom FokBottom)";
    let init_child = "(FokMkCert \
          (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
          (FokInit Zero Zero) (Nil FokCert))";
    let cert1 = format!(
        "(FokMkCert \
            (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
            (FokImpRight Zero) \
            (Cons FokCert {init_child} (Nil FokCert)))"
    );

    assert!(
        eval_bool(
            &mut env,
            &ids,
            "fok_case_accept_imp_right",
            &format!("fok_check_cert {imp_bottom_bottom} {cert1}"),
        ),
        "fok_check_cert must accept a genuine imp-right/init derivation of Bottom -> Bottom"
    );

    let q2 = format!("(FokForallWorld {imp_bottom_bottom})");
    let cert2 = format!(
        "(FokMkCert \
            (FokMkSequent (Nil FokForm) (Cons FokForm {q2} (Nil FokForm))) \
            (FokForallRight Zero (FokQParameter Zero)) \
            (Cons FokCert {cert1} (Nil FokCert)))"
    );
    assert!(
        eval_bool(
            &mut env,
            &ids,
            "fok_case_accept_forall_right",
            &format!("fok_check_cert {q2} {cert2}"),
        ),
        "fok_check_cert must accept a genuine forall-right derivation wrapping the imp-right/init one"
    );
}

/// One malformed certificate per guard in `fok_check_tree`/`fok_check_rule`,
/// each confirmed rejected (`False`) INDIVIDUALLY -- establishing total
/// rejection across every arm, without D3's forthcoming near-miss PAIRS
/// (which additionally pair each rejection against a minimally-different
/// ACCEPTING case; that is `D3`'s job, not this increment's).
#[test]
fn fok_check_cert_totally_rejects_malformed_certificates() {
    let mut env = ElabEnv::new().expect("prelude construction");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken failed to elaborate/kernel-check");
    let ids = FokIds::resolve(&env);

    let imp_bottom_bottom = "(FokImp FokBottom FokBottom)";
    let init_child = "(FokMkCert \
          (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
          (FokInit Zero Zero) (Nil FokCert))";
    let cert1 = format!(
        "(FokMkCert \
            (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
            (FokImpRight Zero) \
            (Cons FokCert {init_child} (Nil FokCert)))"
    );
    let placeholder_child =
        "(FokMkCert (FokMkSequent (Nil FokForm) (Nil FokForm)) (FokInit Zero Zero) (Nil FokCert))";

    let cases: Vec<(&str, String)> = vec![
        (
            "root_conclusion_mismatch",
            format!("fok_check_cert FokBottom {cert1}"),
        ),
        (
            "init_nonempty_children",
            format!(
                "fok_check_tree \
                   (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
                   (FokMkCert \
                     (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
                     (FokInit Zero Zero) \
                     (Cons FokCert {init_child} (Nil FokCert)))"
            ),
        ),
        (
            "init_index_out_of_range",
            format!(
                "fok_check_tree \
                   (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
                   (FokMkCert \
                     (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) (Cons FokForm FokBottom (Nil FokForm))) \
                     (FokInit (Suc Zero) Zero) (Nil FokCert))"
            ),
        ),
        (
            "init_unequal_formulas",
            "fok_check_tree \
               (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) \
                 (Cons FokForm (FokAccess (FokQBound Zero) (FokQBound Zero)) (Nil FokForm))) \
               (FokMkCert \
                 (FokMkSequent (Cons FokForm FokBottom (Nil FokForm)) \
                   (Cons FokForm (FokAccess (FokQBound Zero) (FokQBound Zero)) (Nil FokForm))) \
                 (FokInit Zero Zero) (Nil FokCert))"
                .to_string(),
        ),
        (
            "imp_right_target_not_imp",
            "fok_check_tree \
               (FokMkSequent (Nil FokForm) (Cons FokForm FokBottom (Nil FokForm))) \
               (FokMkCert \
                 (FokMkSequent (Nil FokForm) (Cons FokForm FokBottom (Nil FokForm))) \
                 (FokImpRight Zero) (Nil FokCert))"
                .to_string(),
        ),
        (
            "imp_right_zero_children",
            format!(
                "fok_check_tree \
                   (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
                   (FokMkCert \
                     (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
                     (FokImpRight Zero) (Nil FokCert))"
            ),
        ),
        (
            "imp_right_two_children",
            format!(
                "fok_check_tree \
                   (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
                   (FokMkCert \
                     (FokMkSequent (Nil FokForm) (Cons FokForm {imp_bottom_bottom} (Nil FokForm))) \
                     (FokImpRight Zero) \
                     (Cons FokCert {placeholder_child} (Cons FokCert {placeholder_child} (Nil FokCert))))"
            ),
        ),
        (
            "forall_right_target_not_quantifier",
            "fok_check_tree \
               (FokMkSequent (Nil FokForm) (Cons FokForm FokBottom (Nil FokForm))) \
               (FokMkCert \
                 (FokMkSequent (Nil FokForm) (Cons FokForm FokBottom (Nil FokForm))) \
                 (FokForallRight Zero (FokQParameter Zero)) (Nil FokCert))"
                .to_string(),
        ),
        (
            "forall_right_freshness_violation",
            "fok_check_tree \
               (FokMkSequent \
                 (Cons FokForm (FokForcingP (FokQParameter Zero) (FokQParameter Zero)) (Nil FokForm)) \
                 (Cons FokForm (FokForallWorld FokBottom) (Nil FokForm))) \
               (FokMkCert \
                 (FokMkSequent \
                   (Cons FokForm (FokForcingP (FokQParameter Zero) (FokQParameter Zero)) (Nil FokForm)) \
                   (Cons FokForm (FokForallWorld FokBottom) (Nil FokForm))) \
                 (FokForallRight Zero (FokQParameter Zero)) \
                 (Cons FokCert \
                   (FokMkCert \
                     (FokMkSequent \
                       (Cons FokForm (FokForcingP (FokQParameter Zero) (FokQParameter Zero)) (Nil FokForm)) \
                       (Cons FokForm FokBottom (Nil FokForm))) \
                     (FokInit Zero Zero) (Nil FokCert)) \
                   (Nil FokCert)))"
                .to_string(),
        ),
        (
            "forall_right_zero_children",
            "fok_check_tree \
               (FokMkSequent (Nil FokForm) (Cons FokForm (FokForallWorld FokBottom) (Nil FokForm))) \
               (FokMkCert \
                 (FokMkSequent (Nil FokForm) (Cons FokForm (FokForallWorld FokBottom) (Nil FokForm))) \
                 (FokForallRight Zero (FokQParameter Zero)) (Nil FokCert))"
                .to_string(),
        ),
    ];

    for (name, expr) in cases {
        let const_name = format!("fok_case_reject_{name}");
        assert!(
            !eval_bool(&mut env, &ids, &const_name, &expr),
            "fok_check_cert/fok_check_tree must reject case {name}, but accepted it"
        );
    }
}
