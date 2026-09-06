//! `LANG-MATCH-GUARDS`: arm-selection guards for `34 §3.3`, §4.1, and §4.2.
//!
//! Promise class: durable invariants. These controls pin runtime fallthrough,
//! Bool checking in the pattern context, and the two-sided coverage and
//! reachability exceptions. A guard never contributes type refinement.

use ken_elaborator::lossless::parse_lossless;
use ken_elaborator::{error::ElabError, ArmDeadCause, Decl, ElabEnv, Expr};
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

#[test]
fn guard_uses_pattern_binders_and_false_falls_through() {
    // MEASURED: the same constructor reaches its guarded body for True and its
    // later unguarded body for False, with the guard reading the bound flag.
    // CLAIMED: a guard is a runtime arm-selection gate in the pattern context.
    // THE GAP: unequal payloads distinguish real fallthrough from choosing one
    // arm statically or checking the guard without putting it in emitted core.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    elaborate(
        &mut env,
        "data GuardPayload = MkGuardPayload Bool Nat | GuardOther",
    );
    elaborate(
        &mut env,
        "fn guarded_payload (value : GuardPayload) : Nat = match value { \
         MkGuardPayload flag payload if flag |-> payload ; \
         MkGuardPayload _ _ |-> Zero ; GuardOther |-> Zero }",
    );
    let selected_true = elaborate(
        &mut env,
        "const selected_true : Nat = guarded_payload \
         (MkGuardPayload True (Suc Zero))",
    );
    let selected_false = elaborate(
        &mut env,
        "const selected_false : Nat = guarded_payload \
         (MkGuardPayload False (Suc Zero))",
    );
    let zero = constructor(env.globals["Zero"], []);
    let one = constructor(env.globals["Suc"], [zero.clone()]);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected_true)),
        one
    );
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected_false)),
        zero
    );
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn guard_must_be_bool_and_does_not_refine_its_truth() {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data GuardValue = MkGuardValue Bool");
    match env.elaborate_decl(
        "const non_bool_guard : Nat = match MkGuardValue True { \
         MkGuardValue flag if Zero |-> Zero ; MkGuardValue _ |-> Zero }",
    ) {
        Err(ElabError::IfConditionNotBool { .. }) => {}
        other => panic!("non-Bool guard must select the Bool diagnostic, got {other:?}"),
    }

    match env.elaborate_decl(
        "const guard_is_not_a_proof : Nat = match MkGuardValue True { \
         MkGuardValue flag if flag |-> \
           let impossible : IsTrue flag = Proved in Zero ; \
         MkGuardValue _ |-> Zero }",
    ) {
        Err(ElabError::KernelRejected { .. }) => {}
        other => panic!("guard truth must not refine the body goal, got {other:?}"),
    }
}

#[test]
fn guard_and_body_use_the_dependent_constructor_context() {
    // MEASURED: a constructor fixes the family's abstract index to Nat, and
    // both the guard and guarded/fallback bodies elaborate in that refined
    // method context. CLAIMED: guard lowering stays inside the existing
    // dependent constructor method rather than bypassing its refinement.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "data GuardIx : Type -> Type 1 where { \
         GuardNat : Bool -> Nat -> GuardIx Nat ; \
         GuardBool : Bool -> GuardIx Bool }",
    );
    elaborate(
        &mut env,
        "fn guarded_index (carrier : Type) (value : GuardIx carrier) : carrier = \
         match value { GuardNat flag payload if flag |-> Zero ; \
         GuardNat _ payload |-> payload ; GuardBool flag |-> flag }",
    );
    let selected_false = elaborate(
        &mut env,
        "const guarded_index_false : Nat = \
         guarded_index Nat (GuardNat False (Suc Zero))",
    );
    let selected_true = elaborate(
        &mut env,
        "const guarded_index_true : Nat = \
         guarded_index Nat (GuardNat True (Suc Zero))",
    );
    let zero = constructor(env.globals["Zero"], []);
    let one = constructor(env.globals["Suc"], [zero]);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected_false)),
        one
    );
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected_true)),
        constructor(env.globals["Zero"], [])
    );
}

#[test]
fn guard_presence_flips_constructor_coverage() {
    // MEASURED: adding only `if True` changes an otherwise exhaustive Bool
    // match into ExhaustivenessError naming True. CLAIMED: guarded arms never
    // discharge coverage, even for a syntactically true guard. THE GAP: the
    // unguarded twin proves this is the guard boundary, not a malformed family.
    let mut guarded = ElabEnv::new().expect("base environment");
    match guarded.elaborate_decl(
        "const incomplete : Nat = match True { \
         True if True |-> Zero ; False |-> Zero }",
    ) {
        Err(ElabError::ExhaustivenessError { missing, .. }) => {
            assert_eq!(missing.constructor, "True");
            assert_eq!(missing.arity, 0);
        }
        other => panic!("guarded True must remain missing, got {other:?}"),
    }

    let mut unguarded = ElabEnv::new().expect("base environment");
    elaborate(
        &mut unguarded,
        "const complete : Nat = match True { True |-> Zero ; False |-> Zero }",
    );
}

#[test]
fn guard_presence_flips_later_arm_reachability() {
    // MEASURED: the guarded predecessor leaves the second True arm reachable;
    // removing that guard makes the same arm Subsumed by the first, never
    // NoInhabitants. CLAIMED: guarded arms do not shadow later arms.
    let mut guarded = ElabEnv::new().expect("base environment");
    elaborate(
        &mut guarded,
        "const reachable : Nat = match True { True if False |-> Suc Zero ; \
         True |-> Zero ; False |-> Zero }",
    );

    let mut unguarded = ElabEnv::new().expect("base environment");
    let source = "const dead : Nat = match True { True |-> Suc Zero ; \
         True |-> Zero ; False |-> Zero }";
    match unguarded.elaborate_decl(source) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { first, rest },
            ..
        }) => {
            assert!(rest.is_empty());
            assert_eq!(&source[first.start..first.end], "True |-> Suc Zero");
        }
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("duplicate inhabited constructor must not be NoInhabitants"),
        other => panic!("unguarded predecessor must subsume the second arm, got {other:?}"),
    }
}

#[test]
fn multiple_guards_fall_through_in_source_order() {
    let mut env = ElabEnv::new().expect("base environment");
    let selected = elaborate(
        &mut env,
        "const selected_guard : Nat = match True { \
         True if False |-> Zero ; True if True |-> Suc Zero ; \
         True |-> Suc (Suc Zero) ; False |-> Zero }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );

    let mut dead = ElabEnv::new().expect("base environment");
    match dead.elaborate_decl(
        "const guarded_after_fallback : Nat = match True { \
         True |-> Zero ; True if False |-> Suc Zero ; False |-> Zero }",
    ) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { .. },
            ..
        }) => {}
        other => panic!("an unguarded fallback must subsume later guarded arms, got {other:?}"),
    }
}

#[test]
fn guard_composes_with_or_pattern_occurrence_slots() {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "data GuardOr = GuardLeft Bool Nat | GuardRight Bool Nat",
    );
    elaborate(
        &mut env,
        "fn guarded_or (value : GuardOr) : Nat = match value { \
         GuardLeft flag payload | GuardRight flag payload if flag |-> payload ; \
         GuardLeft _ _ |-> Zero ; GuardRight _ _ |-> Zero }",
    );
    let selected = elaborate(
        &mut env,
        "const guarded_or_selected : Nat = \
         guarded_or (GuardRight False (Suc Zero))",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, selected)), zero);
}

#[test]
fn guard_composes_with_nested_tuple_and_record_patterns() {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "record GuardEnvelope { inner : (flag : Bool) × Nat }",
    );
    elaborate(
        &mut env,
        "const guard_envelope : GuardEnvelope = { inner = (False, Suc Zero) }",
    );
    let selected = elaborate(
        &mut env,
        "const selected_envelope : Nat = match guard_envelope { \
         { inner = (flag, payload) } if flag |-> payload ; \
         { inner = (_, _) } |-> Zero }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, selected)), zero);
}

#[test]
fn fully_guarded_sum_is_incomplete_and_unguarded_fallbacks_close_it() {
    let mut incomplete = ElabEnv::new().expect("base environment");
    match incomplete.elaborate_decl(
        "const all_guarded : Nat = match True { \
         True if True |-> Zero ; False if True |-> Zero }",
    ) {
        Err(ElabError::ExhaustivenessError { .. }) => {}
        other => panic!("fully guarded sum must be non-exhaustive, got {other:?}"),
    }

    let mut complete = ElabEnv::new().expect("base environment");
    elaborate(
        &mut complete,
        "const guarded_with_fallbacks : Nat = match True { \
         True if True |-> Suc Zero ; True |-> Zero ; \
         False if True |-> Suc Zero ; False |-> Zero }",
    );
}

#[test]
fn guard_composes_after_whole_pattern_and_module_rewriting_reaches_it() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module GuardOwner { \
         data LocalGuard = MkLocalGuard Bool Nat | LocalOther ; \
         fn local_id (flag : Bool) : Bool = flag ; \
         pub const selected : Nat = match MkLocalGuard True (Suc Zero) { \
           MkLocalGuard (flag as same) payload as whole if local_id same |-> payload ; \
           MkLocalGuard _ _ |-> Zero ; LocalOther |-> Zero \
         } \
         }",
    )
    .expect("guard resolution and module rewriting descend through the arm");
    let selected = elaborate(&mut env, "const module_guard : Nat = GuardOwner.selected");
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn guarded_top_wildcard_and_variable_remain_refused() {
    for pattern in ["_", "value", "_ | True"] {
        let mut env = ElabEnv::new().expect("base environment");
        let source = format!("const refused : Nat = match True {{ {pattern} if True |-> Zero }}");
        match env.elaborate_decl(&source) {
            Err(ElabError::Internal(message)) => assert!(message.contains("wildcard/var")),
            other => panic!("guard must not lift top catchall refusal, got {other:?}"),
        }
    }
}

#[test]
fn guarded_arm_round_trips_and_owns_guard_trivia_losslessly() {
    let source = "const kept : Nat = match True { True if if False {- guard -} then True else True |-> Zero ; False |-> Zero }\n";
    let parsed = parse_lossless(source).expect("full conditional guard parses losslessly");
    assert_eq!(parsed.reconstruct(), source);
    let condition_span = match &parsed.typed_decls()[0] {
        Decl::ViewDecl {
            body: Expr::EMatch { arms, .. },
            ..
        } => match arms[0].guard.as_ref().expect("first arm has a guard") {
            Expr::EIf { condition, .. } => condition.span().clone(),
            other => panic!("expected a full conditional guard, got {other:?}"),
        },
        other => panic!("expected a match declaration, got {other:?}"),
    };
    let attachment = parsed
        .comment_attachments()
        .iter()
        .find(|attachment| {
            &source[attachment.comment_span.start..attachment.comment_span.end] == "{- guard -}"
        })
        .expect("guard comment is retained");
    assert_eq!(
        attachment.home_span, condition_span,
        "the guard condition must remain in the lossless AST traversal"
    );
}
