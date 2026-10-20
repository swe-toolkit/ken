//! AC-1: a checked subset pair cannot be forged through a type former,
//! a function domain, a local alias, or a split result. A direct introduction
//! remains possible with one explicit proof obligation.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{check, Context, Decl, Term};

fn five_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("const six : Int = 6")
        .expect("ordinary Int");
    env.elaborate_decl_v1("def Five = { x : Int | Equal Int x 5 }")
        .expect("checked subset type");
    env
}

fn typed_refusal(env: &mut ElabEnv, source: &str, name: &str) {
    let before_id = env.env.next_global_id();
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("forgery must refuse");
    assert!(
        matches!(
            error,
            ElabError::TypeMismatch { .. }
                | ElabError::KernelRejected { .. }
                | ElabError::ObligationWithoutChannel { .. }
        ),
        "forgery must be a typed refusal: {error:?}; source={source}"
    );
    assert!(
        !env.globals.contains_key(name),
        "failed declaration became visible"
    );
    assert_eq!(
        env.env.next_global_id(),
        before_id,
        "forgery minted a global"
    );
    assert_eq!(env.env.trusted_base(), before_trust, "forgery minted trust");
}

// Promise class: durable invariant.
// MEASURED: the actual constructor argument and declared List element type
// disagree, across a named, local-alias, and direct type-application route.
// CLAIMED: no coercion is inferred under the List type former.
// THE GAP: the positive Cons Five counterpart must introduce a checked pair;
// this test also checks the successful constructor value at its List type.
#[test]
fn list_subset_and_let_alias_forgeries_refuse_without_new_globals() {
    let mut env = five_env();
    env.elaborate_decl_v1("def L = List Five")
        .expect("named List subset");
    typed_refusal(&mut env, "const bad : L = Cons Int six (Nil Int)", "bad");
    typed_refusal(&mut env,
        "fn forge_alias (u : Int) : List Five = let T = Five; ys : List T = Cons Int six (Nil Int) in ys",
        "forge_alias");
    typed_refusal(&mut env,
        "fn forge_value (u : Int) : List Five = let xs = Cons Int six (Nil Int); ys : List Five = xs in ys",
        "forge_value");
    typed_refusal(&mut env,
        "fn forge_branch (b : Bool) : L = match b { True ↦ Cons Int six (Nil Int); False ↦ Nil Int }",
        "forge_branch");

    let valid = env
        .elaborate_decl_v1("const introduced : L = Cons Five six (Nil Five)")
        .expect("an actual field introduction carries proof");
    let [obligation] = valid.obligations.as_slice() else {
        panic!("one field obligation")
    };
    assert!(matches!(
        obligation.kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(
        env.is_open_hole(obligation.hole_id),
        "Equal Int six 5 is not proved"
    );
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(valid.def_id) else {
        panic!("checked constructor value")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks List Five");
}

// Promise class: durable invariant. MEASURED: a non-scalar Int literal in a
// Char constructor argument produces one visible unsolved proof obligation.
// CLAIMED: a Char is never manufactured without a subset proof component.
// THE GAP: the standalone List Int spelling must not convert to List Char.
#[test]
fn list_char_literal_has_a_visible_scalar_obligation() {
    let mut env = ElabEnv::new().expect("checked prelude");
    typed_refusal(
        &mut env,
        "const forged_chars : List Char = Cons Int 55296 (Nil Int)",
        "forged_chars",
    );
    let result = env
        .elaborate_decl_v1("const checked_chars : List Char = Cons Char 55296 (Nil Char)")
        .expect("an unproved scalar enters only as a checked pair with a hole");
    let [obligation] = result.obligations.as_slice() else {
        panic!("one scalar obligation")
    };
    assert!(matches!(
        obligation.kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(env.is_open_hole(obligation.hole_id));
}

// Promise class: durable invariant. MEASURED: a lambda is paired with a
// checked subset proof even when the transparent alias hides a Pi carrier.
// CLAIMED: checking a lambda against Fn5 cannot silently erase the predicate.
// THE GAP: inspecting the kernel pair and its recorded goal distinguishes a
// real introduction from merely returning an unresolved ordinary function.
#[test]
fn lambda_at_named_function_subset_introduces_one_checked_pair() {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("def Fn5 = { f : Int -> Int | Equal Int (f 0) 5 }")
        .expect("subset over a Pi carrier");
    let bad = env
        .elaborate_decl_v1("const c : Fn5 = \\y. 6")
        .expect("checked lambda plus unsolved proposition");
    let [obligation] = bad.obligations.as_slice() else {
        panic!("one function proof")
    };
    assert!(matches!(
        obligation.kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(env.is_open_hole(obligation.hole_id));
    let Term::App(equal_left, five) = &obligation.goal_closed else {
        panic!("Fn5 proof must compare the lambda result with five")
    };
    let Term::App(equal_carrier, applied) = equal_left.as_ref() else {
        panic!("Fn5 proof must apply the lambda to zero")
    };
    let Term::App(equal, int) = equal_carrier.as_ref() else {
        panic!("Fn5 proof must use Equal at Int")
    };
    assert_eq!(equal.as_ref(), &Term::const_(env.globals["Equal"], vec![]));
    assert_eq!(int.as_ref(), &Term::const_(env.globals["Int"], vec![]));
    assert_eq!(five.as_ref(), &Term::IntLit(5.into()));
    let Term::App(function, zero) = applied.as_ref() else {
        panic!("Fn5 proof must use the checked lambda as a function")
    };
    assert_eq!(zero.as_ref(), &Term::IntLit(0.into()));
    assert!(
        matches!(function.as_ref(), Term::Ascript(lambda, _) if matches!(lambda.as_ref(),
        Term::Lam(_, body) if body.as_ref() == &Term::IntLit(6.into())))
    );
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(bad.def_id) else {
        panic!("checked lambda subset")
    };
    assert!(
        matches!(body, Term::Pair(value, _)
        if matches!(value.as_ref(), Term::Ascript(lambda, function_ty)
            if matches!(lambda.as_ref(), Term::Lam(..))
                && matches!(function_ty.as_ref(), Term::Pi(..)))),
        "a function-valued subset introduces a checked pair, not a bare lambda: {body:?}"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks Fn5 pair");
    let use_it = env
        .elaborate_decl_v1("fn use_checked (f : Fn5) : Int = f 0")
        .expect("application forgets a function-valued subset at the head");
    let Some(Decl::Transparent {
        body: Term::Lam(_, call),
        ..
    }) = env.env.lookup(use_it.def_id)
    else {
        panic!("checked function consumer")
    };
    assert!(
        matches!(call.as_ref(), Term::App(head, _)
        if matches!(head.as_ref(), Term::Proj1(pair)
            if matches!(pair.as_ref(), Term::Ascript(_, _)))),
        "the application head must project the checked carrier: {call:?}"
    );
}

// Promise class: durable invariant. MEASURED: an inductive value whose
// checked type is a subset eliminates through Proj1 of that checked pair.
// CLAIMED: constructor matches forget a subset only at their scrutinee head.
// THE GAP: the accepted match must actually eliminate the projected value,
// not merely omit the proof while keeping the scrutinee's subset type.
#[test]
fn constructor_match_forgets_a_checked_subset_scrutinee() {
    fn projected_elimination(term: &Term) -> bool {
        matches!(term, Term::Elim { scrut, .. }
            if matches!(scrut.as_ref(), Term::Proj1(_)))
            || term.children().into_iter().any(projected_elimination)
    }
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("data Color = Red | Blue")
        .expect("nonrecursive constructor family");
    env.elaborate_decl_v1("def ColorChecked = { c : Color | Top }")
        .expect("subset of an inductive carrier");
    let result = env
        .elaborate_decl_v1(
            "fn inspect (value : ColorChecked) : Bool = \
         match value { Red ↦ True; Blue ↦ False }",
        )
        .expect("checked subset scrutinee forgets at the eliminator");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked eliminator declaration")
    };
    assert!(
        projected_elimination(body),
        "match must eliminate Proj1: {body:?}"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks the eliminator");
}

// Promise class: durable invariant. MEASURED: an Int literal comparator
// consumes the projected carrier as its argument, not a subset pair as Int.
// CLAIMED: a literal-pattern match forgets at its value head.
// THE GAP: it must keep the Char literal comparator on its checked Char
// carrier; the adjacent lang_match_literal_patterns suite pins that path.
#[test]
fn literal_pattern_on_a_checked_int_subset_projects_at_the_head() {
    fn projected_comparator_argument(term: &Term) -> bool {
        matches!(term, Term::App(_, argument)
            if matches!(argument.as_ref(), Term::Proj1(_)))
            || term
                .children()
                .into_iter()
                .any(projected_comparator_argument)
    }
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("def CheckedInt = { x : Int | Top }")
        .expect("checked subset");
    let result = env
        .elaborate_decl_v1("fn is_five (n : CheckedInt) : Bool = match n { 5 ↦ True; _ ↦ False }")
        .expect("literal comparison reads an Int carrier");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked literal match")
    };
    assert!(
        projected_comparator_argument(body),
        "literal comparator must consume Proj1: {body:?}"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks literal match");
}

// Promise class: durable invariant. MEASURED: the if eliminator reads Bool
// from a checked subset only through Proj1; the pair itself stays typed.
// CLAIMED: proof fields cannot be used as Bool conditions by accident.
// THE GAP: inspect the emitted eliminator scrutinee, not just admission.
#[test]
fn if_condition_projects_a_checked_bool_subset() {
    fn projected_elimination(term: &Term) -> bool {
        matches!(term, Term::Elim { scrut, .. }
            if matches!(scrut.as_ref(), Term::Proj1(_)))
            || term.children().into_iter().any(projected_elimination)
    }
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("def CheckedBool = { b : Bool | Top }")
        .expect("checked Bool subset");
    let result = env
        .elaborate_decl_v1("fn choose (b : CheckedBool) : Int = if b then 1 else 0")
        .expect("if reads only the checked Bool carrier");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked conditional")
    };
    assert!(
        projected_elimination(body),
        "condition must project Bool: {body:?}"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks the conditional");
}

// Promise class: durable invariant. MEASURED: binary-operator dispatch
// reads a checked subset's carrier and applies the primitive to Proj1.
// CLAIMED: classification cannot treat a subset Sigma as its carrier.
// THE GAP: the actual emitted operator operand must contain the projection.
#[test]
fn binary_operator_classifies_a_projected_subset_carrier() {
    fn projected_left_operand(term: &Term) -> bool {
        matches!(term, Term::App(function, _)
            if matches!(function.as_ref(), Term::App(_, left)
                if matches!(left.as_ref(), Term::Proj1(_))))
            || term.children().into_iter().any(projected_left_operand)
    }
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("def CheckedInt = { x : Int | Top }")
        .expect("checked Int subset");
    for source in [
        "fn plus (n : CheckedInt) : Int = n + 1",
        "fn equal (n : CheckedInt) : Bool = n == 1",
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .expect("operator dispatch forgets the subset at its operand");
        let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(result.def_id) else {
            panic!("checked binary-operator declaration")
        };
        assert!(
            projected_left_operand(body),
            "operator consumes Proj1: {body:?}"
        );
        check(&env.env, &Context::new(), body, ty).expect("kernel rechecks operator");
    }
}

// Promise class: durable invariant. MEASURED: a refined named record
// projects an actual carrier field, while an ordinary record with an Ω
// field keeps its checked named-field owner instead of being forgotten.
// CLAIMED: `.field` forgets only a subset wrapper, not a record's fields.
// THE GAP: the body must project the subset's outer first component.
#[test]
fn named_projection_forgets_only_an_outer_subset_wrapper() {
    fn forgets_wrapper(term: &Term, wrapper: &Term) -> bool {
        matches!(term, Term::Proj1(base) if matches!(base.as_ref(),
            Term::Ascript(_, ty) if ty.as_ref() == wrapper))
            || term
                .children()
                .into_iter()
                .any(|child| forgets_wrapper(child, wrapper))
    }
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("record Boxed { value : Int, witness : Top }")
        .expect("named record with an Ω field");
    env.elaborate_decl_v1("def CheckedBox = { b : Boxed | Top }")
        .expect("checked record wrapper");
    let record = env
        .elaborate_decl_v1("fn original (b : Boxed) : Int = b.value")
        .expect("named record projection stays available");
    assert!(record.obligations.is_empty());
    let result = env
        .elaborate_decl_v1("fn projected (b : CheckedBox) : Int = b.value")
        .expect("field projection forgets only the subset wrapper");
    let Some(Decl::Transparent {
        body: Term::Lam(_, field),
        ty,
        ..
    }) = env.env.lookup(result.def_id)
    else {
        panic!("checked field projection")
    };
    assert!(
        matches!(field.as_ref(), Term::Proj1(record)
        if matches!(record.as_ref(), Term::Proj1(_))),
        "the field reads the projected record, not the subset proof: {field:?}"
    );
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked projection declaration")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks field projection");
    let matched = env
        .elaborate_decl_v1("fn destructured (b : CheckedBox) : Int = match b { { value = x } ↦ x }")
        .expect("record pattern forgets a subset wrapper, not the record owner");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(matched.def_id) else {
        panic!("checked record pattern")
    };
    let wrapper = Term::const_(env.globals["CheckedBox"], vec![]);
    assert!(
        forgets_wrapper(body, &wrapper),
        "record pattern must project its checked wrapper: {body:?}"
    );
    check(&env.env, &Context::new(), body, ty).expect("kernel rechecks record match");
}

// Promise class: durable invariant. MEASURED: first-class passing of a
// refined-domain function to an Int-domain argument refuses, while a direct
// Int-to-refined call introduces one checked pair and obligation.
// CLAIMED: Π-domain variance is not silently inferred from carrier erasure.
// THE GAP: the two rows share the exact same take5 definition and caller.
#[test]
fn first_class_refined_parameter_refuses_while_direct_call_obligates() {
    let mut env = five_env();
    env.elaborate_decl_v1("fn take5 (x : Five) : Int = x")
        .expect("forget uses Proj1");
    env.elaborate_decl_v1("fn apply (g : Int -> Int) : Int = g six")
        .expect("ordinary first-class consumer");
    typed_refusal(&mut env, "const forged : Int = apply take5", "forged");
    let result = env
        .elaborate_decl_v1("const direct : Int = take5 six")
        .expect("direct call introduces a checked subset");
    let [obligation] = result.obligations.as_slice() else {
        panic!("one call-site proof")
    };
    assert!(matches!(
        obligation.kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(env.is_open_hole(obligation.hole_id));
}
