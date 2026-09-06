//! `LANG-MATCH-AS-PATTERN`: consumer controls for `32 §4` and `34 §3.1`.
//!
//! Promise class: durable invariants. These controls assert the binding,
//! coverage, refusal, and trust properties of the as-pattern wrapper. Intended
//! future pattern forms remain free to extend the pattern grammar while keeping
//! these relations and diagnostics intact.

use ken_elaborator::{error::ElabError, ArmDeadCause, ElabEnv};
use ken_kernel::{whnf, Context, GlobalId, Term};

fn elaborate(env: &mut ElabEnv, source: &str) -> GlobalId {
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("elaboration failed: {error}"))
}

fn body(env: &ElabEnv, id: GlobalId) -> Term {
    env.env
        .transparent_body(id)
        .expect("test declaration is transparent")
        .1
}

fn constructor(id: GlobalId, arguments: impl IntoIterator<Item = Term>) -> Term {
    arguments.into_iter().fold(
        Term::Constructor {
            id,
            level_args: Vec::new(),
        },
        Term::app,
    )
}

fn setup_nested_fixture(env: &mut ElabEnv) {
    elaborate(env, "data NatAs = ZeroAs | SuccAs NatAs");
    elaborate(env, "data AliasPair = MkAliasPair NatAs NatAs");
}

#[test]
fn top_level_constructor_alias_binds_the_whole_scrutinee() {
    // MEASURED: the normalized result is the complete two-field constructor,
    // not either field. CLAIMED: a top-level constructor alias denotes the
    // value matched at that position. THE GAP: normalization is discriminating
    // because the two fields and the whole value have different kernel shapes.
    let mut env = ElabEnv::new().expect("base environment");
    setup_nested_fixture(&mut env);
    let trusted_before = env.env.trusted_base();
    let id = elaborate(
        &mut env,
        "let captured : AliasPair = match MkAliasPair (SuccAs ZeroAs) ZeroAs { \
         (MkAliasPair first second) as whole |-> whole }",
    );

    let zero = constructor(env.globals["ZeroAs"], []);
    let succ_zero = constructor(env.globals["SuccAs"], [zero.clone()]);
    let expected = constructor(env.globals["MkAliasPair"], [succ_zero, zero]);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, id)), expected);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn nested_constructor_alias_uses_its_depth_rebased_occurrence() {
    // MEASURED: after an outer PairAs split and an inner recursive SuccAs
    // split, the alias reduces to `SuccAs ZeroAs`, while the inner binder is
    // only `ZeroAs`. CLAIMED: a nested alias receives its own matrix-column
    // occurrence after real and recursive-IH binders are rebased. THE GAP: the
    // result pair is non-degenerate, so substituting the child occurrence for
    // the nested whole occurrence changes the observed constructor.
    let mut env = ElabEnv::new().expect("base environment");
    setup_nested_fixture(&mut env);
    let trusted_before = env.env.trusted_base();
    let id = elaborate(
        &mut env,
        "let nested : NatAs = match MkAliasPair (SuccAs ZeroAs) ZeroAs { \
         MkAliasPair ZeroAs tail |-> tail ; \
         MkAliasPair (SuccAs child as nested_whole) tail |-> nested_whole }",
    );

    let zero = constructor(env.globals["ZeroAs"], []);
    let expected = constructor(env.globals["SuccAs"], [zero]);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, id)), expected);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

fn redundant_cause(source: &str) -> ArmDeadCause {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data RedundantAs = RZero | RSucc RedundantAs");
    match env.elaborate_decl(source) {
        Err(ElabError::ReachabilityError { cause, .. }) => cause,
        other => panic!("expected a reachability error, got {other:?}"),
    }
}

#[test]
fn alias_delegates_to_the_inner_patterns_subsumption_cause() {
    // MEASURED: bare and aliased copies of the same constructor are both
    // classified as Subsumed by the same earlier arm span. CLAIMED: `as` adds
    // no coverage and never converts ordinary redundancy into NoInhabitants.
    // THE GAP: both fixtures retain the same exhaustive arm prefix; only the
    // redundant arm gains the alias wrapper.
    let bare = redundant_cause(
        "let plain : RedundantAs = match RZero { \
         RZero |-> RZero ; RSucc n |-> n ; RSucc m |-> m }",
    );
    let aliased = redundant_cause(
        "let alias : RedundantAs = match RZero { \
         RZero |-> RZero ; RSucc n |-> n ; RSucc m as whole |-> whole }",
    );

    match (bare, aliased) {
        (
            ArmDeadCause::Subsumed {
                first: bare_first,
                rest: bare_rest,
            },
            ArmDeadCause::Subsumed {
                first: alias_first,
                rest: alias_rest,
            },
        ) => {
            assert_eq!(bare_first, alias_first);
            assert_eq!(bare_rest, alias_rest);
        }
        other => panic!("bare and aliased arms must share Subsumed, got {other:?}"),
    }
}

#[test]
fn qualified_module_patterns_preserve_alias_bindings() {
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        "module AliasOwner { \
         pub data AliasToken = MkAliasToken ; \
         pub const make : AliasToken = MkAliasToken ; \
         pub fn keep (value : AliasToken) : AliasToken = \
           match value { MkAliasToken as whole |-> whole } \
         }",
    )
    .expect("qualified owner elaborates its as-pattern");
    env.elaborate_decl("const observed : AliasOwner.AliasToken = AliasOwner.keep AliasOwner.make")
        .expect("client consumes the public function without constructor visibility");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

fn assert_alias_collision(source: &str, collision: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    setup_nested_fixture(&mut env);
    match env.elaborate_decl(source) {
        Err(ElabError::ParseError { msg, .. }) => assert_eq!(
            msg,
            format!("as-pattern alias '{collision}' collides with an existing pattern binder")
        ),
        other => panic!("expected the named alias collision, got {other:?}"),
    }
}

#[test]
fn alias_collision_with_inner_binder_is_rejected_by_name() {
    assert_alias_collision(
        "let bad_inner : NatAs = match MkAliasPair ZeroAs ZeroAs { \
         MkAliasPair ZeroAs tail |-> tail ; \
         MkAliasPair (SuccAs n as n) tail |-> n }",
        "n",
    );
}

#[test]
fn alias_collision_with_sibling_binder_is_rejected_by_name() {
    assert_alias_collision(
        "let bad_sibling : NatAs = match MkAliasPair ZeroAs ZeroAs { \
         MkAliasPair ZeroAs tail |-> tail ; \
         MkAliasPair (SuccAs n as tail) tail |-> n }",
        "tail",
    );
}

fn assert_top_level_refusal(pattern: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data RefusalAs = RefusalCtor");
    let source = format!("let bad : RefusalAs = match RefusalCtor {{ {pattern} |-> RefusalCtor }}");
    match env.elaborate_decl(&source) {
        Err(ElabError::Internal(msg)) => assert_eq!(
            msg,
            "non-constructor pattern in match (wildcard/var not yet supported at top level; \
             use constructor patterns)"
        ),
        other => panic!("top-level non-constructor alias must stay refused, got {other:?}"),
    }
}

// The positive top-level constructor control above proves the wrapper is
// reached; these paired negatives independently isolate each inner-pattern
// boundary that remains fail-closed in this slice.
#[test]
fn top_level_wildcard_alias_remains_fail_closed() {
    assert_top_level_refusal("_ as whole");
}

#[test]
fn top_level_variable_alias_remains_fail_closed() {
    assert_top_level_refusal("value as whole");
}
