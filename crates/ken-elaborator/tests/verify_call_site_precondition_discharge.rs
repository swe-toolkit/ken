//! VERIFY-CALL-SITE-PRECONDITION-DISCHARGE.
//! Promise class: durable invariant (spec 21 §6.3 and 22 §2.3).

use std::collections::HashSet;

use ken_elaborator::{ElabEnv, ElabResult, ObligationKind};
use ken_interp::{EvalStore, EvalVal, eval};
use ken_kernel::GlobalId;

fn trusted(env: &ElabEnv) -> HashSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn only_requires(result: &ElabResult) {
    assert_eq!(result.obligations.len(), 1);
    assert!(matches!(
        result.obligations[0].kind,
        ObligationKind::Requires
    ));
}

/// AC-1, AC-4, AC-5. MEASURED: P1 reports one Requires hole, its id is the
/// complete trusted-base delta, and interpreter evaluation of the caller
/// returns the carrier despite the open proof argument. CLAIMED: absent caller
/// evidence becomes one call-site obligation and the proof argument is erased
/// from observable execution. THE GAP: the exact hole-set comparison and
/// interpreter result jointly bind the reported hole to the actual call.
#[test]
fn missing_call_premise_is_reported_and_does_not_change_the_result() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let results = env
        .elaborate_file_v1(
            "fn f (n : Int) : Int requires Equal Int n n = n\n\
             fn g (m : Int) : Int = f m",
        )
        .expect("caller with an open precondition obligation elaborates");
    let [f, g] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    assert!(f.obligations.is_empty());
    only_requires(g);

    let reported = g
        .obligations
        .iter()
        .map(|obligation| obligation.hole_id)
        .collect::<HashSet<_>>();
    let after = trusted(&env);
    assert_eq!(
        after.difference(&before).copied().collect::<HashSet<_>>(),
        reported
    );

    let (call, _) = env
        .elaborate_expr("runtime-call", "g 7")
        .expect("the checked caller is callable");
    let mut store = EvalStore::new();
    assert_eq!(eval(&[], &call, &env.env, &mut store), EvalVal::Int(7));
}

/// AC-5. MEASURED: T1 minus T0 is exactly the caller's Requires hole and
/// Ensures hole, with both kinds reported and no PartialPrim hole. CLAIMED:
/// obligation-hole accounting survives adding call-site preconditions to the
/// existing contract lowering. THE GAP: compare the actual id set and kinds,
/// not just counts.
#[test]
fn call_site_and_ensures_holes_are_a_complete_trusted_base_delta() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let results = env
        .elaborate_file_v1(
            "fn f (n : Int) : Int requires Equal Int n n = n\n\
             fn g (m : Int) : Int ensures Equal Int result m = f m",
        )
        .expect("caller has Requires and Ensures obligations");
    let [_, g] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    assert_eq!(g.obligations.len(), 2);
    assert!(g
        .obligations
        .iter()
        .any(|obligation| matches!(obligation.kind, ObligationKind::Requires)));
    assert!(g
        .obligations
        .iter()
        .any(|obligation| matches!(obligation.kind, ObligationKind::Ensures)));
    assert!(!g
        .obligations
        .iter()
        .any(|obligation| matches!(obligation.kind, ObligationKind::PartialPrim)));

    let reported = g
        .obligations
        .iter()
        .map(|obligation| obligation.hole_id)
        .collect::<HashSet<_>>();
    let after = trusted(&env);
    assert_eq!(
        after.difference(&before).copied().collect::<HashSet<_>>(),
        reported
    );
}

/// A non-Const higher-order head retains its explicit proof argument.
/// MEASURED: `g x p` elaborates with no Requires hole when `g` is a local Pi
/// binder. CLAIMED: only a saturated checked `Const` uses the arity table.
/// THE GAP: the local binder's Pi type itself contains an Ω domain.
#[test]
fn non_const_higher_order_head_keeps_explicit_arguments() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let result = env
        .elaborate_decl_v1(
            "fn apply (g : (x : Int) -> Equal Int x x -> Int) \
             (x : Int) (p : Equal Int x x) : Int = g x p",
        )
        .expect("the local higher-order head receives its explicit proof");
    assert!(result.obligations.is_empty());
}

/// A contracted function value is not silently coerced to a plain function
/// value. MEASURED: passing `f` to a plain-arrow parameter rejects without a
/// Requires hole. CLAIMED: partial/value positions do not insert proofs. THE
/// GAP: `f`'s actual telescope contains an extra premise binder.
#[test]
fn contracted_function_value_is_refused_at_a_plain_higher_order_type() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    env.elaborate_decl_v1("fn f (n : Int) : Int requires Equal Int n n = n")
        .expect("callee declaration");
    env.elaborate_decl_v1("fn apply (g : Int -> Int) (n : Int) : Int = g n")
        .expect("plain higher-order caller");
    let before = trusted(&env);
    let error = env
        .elaborate_decl_v1("fn caller (n : Int) : Int = apply f n")
        .expect_err("contracted function value does not match a plain arrow");
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    assert_eq!(
        trusted(&env),
        before,
        "function-value rejection minted no hole"
    );
}

/// AC-1 and M4. MEASURED: the matching `requires` premise makes the caller's
/// obligation list empty. CLAIMED: the call consumes the in-scope kernel
/// binder. THE GAP: compare the exact premise instance after parameter
/// substitution, not merely the caller's obligation count.
#[test]
fn matching_caller_premise_supplies_the_proof() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let results = env
        .elaborate_file_v1(
            "fn f (n : Int) : Int requires Equal Int n n = n\n\
             fn g (m : Int) : Int requires Equal Int m m = f m",
        )
        .expect("matching caller premise discharges the call");
    let [_, g] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    assert!(
        g.obligations.is_empty(),
        "matching caller premise left obligations: {:?}",
        g.obligations
    );
    assert_eq!(trusted(&env), before);
}

/// Multi-clause call-site control. MEASURED: reversed caller premise order
/// still supplies both callee proofs by conversion, and omitting one premise
/// emits exactly one Requires hole. CLAIMED: premises are instantiated and
/// discharged independently in callee clause order. THE GAP: the two domains
/// are distinct equations over same-typed parameters.
#[test]
fn multiple_call_premises_match_independently_and_keep_clause_order() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let results = env
        .elaborate_file_v1(
            "fn f (n : Int) (d : Int) : Int \
             requires Equal Int n n requires Equal Int d d = n\n\
             fn reversed (n : Int) (d : Int) : Int \
             requires Equal Int d d requires Equal Int n n = f n d",
        )
        .expect("caller premises discharge in a different order");
    let [_, reversed] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    assert!(reversed.obligations.is_empty());

    let mut env = ElabEnv::new().expect("numeric prelude");
    let results = env
        .elaborate_file_v1(
            "fn f (n : Int) (d : Int) : Int \
             requires Equal Int n n requires Equal Int d d = n\n\
             fn partial_evidence (n : Int) (d : Int) : Int \
             requires Equal Int n n = f n d",
        )
        .expect("the caller's uncovered second premise becomes an obligation");
    let [_, caller] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    only_requires(caller);
}

/// AC-2. MEASURED: structural recursion with a changed argument admits and
/// reports one Requires hole. CLAIMED: a recursive call uses the same
/// call-site discharge as an ordinary call. THE GAP: the fixture decreases a
/// Nat field, and the kernel's admission gate still runs SCT.
#[test]
fn recursive_calls_receive_premises_without_bypassing_sct() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let result = env
        .elaborate_decl_v1(
            "fn r (n : Nat) : Nat requires Equal Nat n n = \
             match n { Zero ↦ Zero; Suc k ↦ r k }",
        )
        .expect("structural recursive call has a call-site obligation");
    only_requires(&result);
    env.env
        .transparent_body(result.def_id)
        .expect("the accepted recursive body is admitted");
}

/// P3. MEASURED: same-argument Int recursion with a matching requires proof
/// reaches SCT and is refused as non-terminating. CLAIMED: staging at the full
/// contract type repairs NotAFunction without weakening termination checking.
/// THE GAP: the recursive call's premise is supplied from the current binder.
#[test]
fn same_argument_int_recursion_reaches_sct_after_premise_insertion() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let error = env
        .elaborate_decl_v1("fn recurse (n : Int) : Int requires Equal Int n n = recurse n")
        .expect_err("same-argument recursion remains non-terminating");
    assert!(format!("{error:?}").contains("NotTerminating"), "{error:?}");
}

/// The admission transaction owns call-site holes created while checking a
/// recursive body. MEASURED: the nondecreasing `bad (Suc n)` call creates a
/// Requires goal before SCT refusal, and the trusted base is identical after
/// refusal. CLAIMED: failed recursive admission leaves neither the placeholder
/// nor its call-site hole. THE GAP: the premise instance differs from the
/// caller's `Equal Nat n n`, so a Requires hole is reached before admission.
#[test]
fn failed_recursive_admission_rolls_back_its_call_site_hole() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let error = env
        .elaborate_decl_v1("fn bad (n : Nat) : Nat requires Equal Nat n n = bad (Suc n)")
        .expect_err("the call increases its argument and must fail SCT");
    assert!(format!("{error:?}").contains("NotTerminating"), "{error:?}");
    assert_eq!(
        trusted(&env),
        before,
        "staged declaration and Requires hole roll back"
    );
    assert!(!env.globals.contains_key("bad"));
}

/// AC-3 and M3. MEASURED: a declaration with an explicit Ω-typed parameter
/// requires the caller to supply that parameter and emits no Requires hole.
/// CLAIMED: only checked identities registered as spec'd callees trigger
/// insertion. THE GAP: the fixture's proof domain is Ω-typed but is not a
/// `requires` clause.
#[test]
fn explicit_proof_parameter_remains_explicit() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let results = env
        .elaborate_file_v1(
            "fn contracted (n : Int) : Int requires Equal Int n n = n\n\
             fn h (n : Int) (p : Equal Int n n) : Int = n\n\
             fn caller (m : Int) (p : Equal Int m m) : Int = h m p",
        )
        .expect("caller explicitly supplies the proof parameter");
    let [_, _, caller] = results.as_slice() else {
        panic!("the file contains an unrelated contracted callee, h, and caller")
    };
    assert!(caller.obligations.is_empty());
}

/// The saturated-Const gate does not turn a partial spine into a hidden call.
/// MEASURED: returning a one-argument partial spine for a two-parameter
/// contracted function is rejected with no Requires hole. CLAIMED: fewer than
/// |Delta| explicit arguments receive no insertion. THE GAP: the function
/// type retains its unsupplied parameter and requires telescope.
#[test]
fn partial_spine_gets_no_precondition_insertion() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    env.elaborate_decl_v1("fn f (n : Int) (d : Int) : Int requires Equal Int n n = d")
        .expect("callee declaration");
    let before = trusted(&env);
    let error = env
        .elaborate_decl_v1("fn partial (n : Int) : Int -> Int = f n")
        .expect_err("partial contracted function does not match a plain arrow");
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    assert_eq!(
        trusted(&env),
        before,
        "partial spine minted no Requires hole"
    );
}

/// MEASURED: a zero-parameter contract whose result is a function, applied at
/// a call, reports exactly one Requires hole, and the trusted-base delta is
/// that hole. CLAIMED: probing the call head for its precondition arity raises
/// no obligation of its own. THE GAP: on `02d2610` this caller reported two
/// Requires holes for one premise, because the declined probe had already
/// raised one before generic application raised it again.
#[test]
fn applied_zero_parameter_contract_raises_one_call_site_obligation() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let results = env
        .elaborate_file_v1(
            "const k : Int -> Int requires Equal Int 0 0 = \\m. m\n\
             fn g (x : Int) : Int = k x",
        )
        .expect("caller elaborates with one open precondition");
    let [_, g] = results.as_slice() else {
        panic!("the file contains the callee and caller")
    };
    only_requires(g);
    let reported = g
        .obligations
        .iter()
        .map(|obligation| obligation.hole_id)
        .collect::<HashSet<_>>();
    assert_eq!(
        trusted(&env)
            .difference(&before)
            .copied()
            .collect::<HashSet<_>>(),
        reported
    );
}

/// MEASURED: a checked global imported from a module, applied at a call,
/// reports exactly one Requires hole, and that hole is the trusted-base delta.
/// CLAIMED: probing an imported `RCheckedGlobal` head does not insert the same
/// premise before generic application. THE GAP: the same-unit regression
/// exercises `RCon`; the checked-import route must be independently reached.
#[test]
fn imported_zero_parameter_contract_raises_one_call_site_obligation() {
    let mut env = ElabEnv::new().expect("numeric prelude");
    let before = trusted(&env);
    let results = env
        .elaborate_file_v1(
            "module M { pub const k : Int -> Int requires Equal Int 0 0 = \\m. m }\n\
             import M\n\
             fn caller (x : Int) : Int = M.k x",
        )
        .expect("imported caller elaborates with one open precondition");
    let caller = results
        .last()
        .expect("the file contains the imported caller");
    only_requires(caller);
    let reported = caller
        .obligations
        .iter()
        .map(|obligation| obligation.hole_id)
        .collect::<HashSet<_>>();
    assert_eq!(
        trusted(&env)
            .difference(&before)
            .copied()
            .collect::<HashSet<_>>(),
        reported
    );
}
