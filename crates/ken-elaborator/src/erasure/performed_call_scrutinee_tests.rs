use super::*;

// Synthetic lowered bodies, not checked-source witnesses. The production queue
// supplies `performing` (members lower_checked_host_declaration lowered) and
// the spine's checked Ret; here both are fixed spellings.
const RET: &str = "ctor:fixture::ITree::Ret";
const VIS: &str = "ctor:fixture::ITree::Vis";
const PERFORMED: &str = "decl:fixture::drain";
const TREE_STYLE: &str = "decl:fixture::helper";

fn owner() -> StableSymbol {
    StableSymbol::declaration("fixture", &[], "main")
}

fn performing() -> BTreeSet<String> {
    BTreeSet::from([PERFORMED.to_string()])
}

fn call(symbol: &str) -> RuntimeExpr {
    RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: symbol.to_string(),
        }),
        args: vec![RuntimeExpr::Var(0)],
    }
}

fn tree_match(scrutinee: RuntimeExpr, vis_body: RuntimeExpr, ret_case: bool) -> RuntimeExpr {
    let mut cases = Vec::new();
    if ret_case {
        cases.push(RuntimeComputationalMatchCase {
            constructor: RET.to_string(),
            argument_binders: 1,
            recursive_positions: Vec::new(),
            body: RuntimeExpr::Var(0),
        });
    }
    cases.push(RuntimeComputationalMatchCase {
        constructor: VIS.to_string(),
        argument_binders: 2,
        recursive_positions: vec![1],
        body: vis_body,
    });
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(scrutinee),
        cases,
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "performed-call fixture default".to_string(),
        },
    }
}

fn scrutinee_of(expression: &RuntimeExpr) -> &RuntimeExpr {
    let RuntimeExpr::ComputationalMatch { scrutinee, .. } = expression else {
        panic!("the fixture is a computational match: {expression:?}");
    };
    scrutinee
}

fn is_call_to(expression: &RuntimeExpr, symbol: &str) -> bool {
    matches!(
        expression,
        RuntimeExpr::Call { callee, .. }
            if matches!(callee.as_ref(), RuntimeExpr::DeclarationRef { symbol: called } if called == symbol)
    )
}

#[test]
fn a_performed_call_scrutinee_is_given_its_ret_tree_at_any_depth() {
    let mut expression = RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Value(RuntimeValue::Int(1.into()))),
        body: Box::new(tree_match(call(PERFORMED), RuntimeExpr::Var(0), true)),
    };
    wrap_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect("the match has a checked Ret case");
    let RuntimeExpr::Let { body, .. } = &expression else {
        panic!("the Let is kept: {expression:?}");
    };
    let RuntimeExpr::Construct { constructor, args } = scrutinee_of(body) else {
        panic!("the performed call must be wrapped: {body:?}");
    };
    assert_eq!(constructor, RET);
    assert!(
        matches!(args.as_slice(), [only] if is_call_to(only, PERFORMED)),
        "Ret carries exactly the original call: {args:?}"
    );
}

#[test]
fn only_a_performed_call_in_scrutinee_position_is_wrapped() {
    // A tree-style member already returns a tree, and a performed call that is
    // a case body is a continuation's result, which is not a scrutinee.
    let mut expression = tree_match(call(TREE_STYLE), call(PERFORMED), true);
    wrap_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect("nothing here needs a Ret case");
    assert!(
        is_call_to(scrutinee_of(&expression), TREE_STYLE),
        "{expression:?}"
    );
    let RuntimeExpr::ComputationalMatch { cases, .. } = &expression else {
        unreachable!()
    };
    assert!(is_call_to(&cases[1].body, PERFORMED), "{expression:?}");
}

#[test]
fn a_performed_call_scrutinee_without_a_checked_ret_case_refuses() {
    let mut expression = tree_match(call(PERFORMED), RuntimeExpr::Var(0), false);
    let error = wrap_performed_call_scrutinees(&owner(), &mut expression, &performing(), RET)
        .expect_err("no Ret case to receive the payload");
    assert!(
        matches!(
            &error,
            ErasureError::ExpressionLowering { lane, .. }
                if *lane == "performed_call_scrutinee_without_ret_case"
        ),
        "{error:?}"
    );
}
