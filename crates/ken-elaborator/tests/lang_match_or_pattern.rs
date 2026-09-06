//! `LANG-MATCH-OR-PATTERN`: same-occurrence alternation controls for
//! `32 §4` and `34 §3.1`.
//!
//! Promise class: durable invariants. These controls pin canonical by-name
//! bindings, common-context type agreement, union coverage, whole-arm
//! reachability, and the absence of a new projection or carrier lowering.

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

fn count_nodes(term: &Term) -> (usize, usize) {
    let here = (
        usize::from(matches!(term, Term::Elim { .. })),
        usize::from(matches!(term, Term::Proj1(_) | Term::Proj2(_))),
    );
    term.children().into_iter().fold(here, |counts, child| {
        let child_counts = count_nodes(child);
        (counts.0 + child_counts.0, counts.1 + child_counts.1)
    })
}

#[test]
fn alternatives_feed_one_body_slot_over_the_same_occurrence() {
    // MEASURED: both constructors return their unequal payload through the one
    // source name, raw core uses the family's existing eliminator, and no
    // projection appears. CLAIMED: `|` duplicates a residual row over the same
    // occurrence rather than constructing or projecting a parallel carrier.
    // THE GAP: checking both unequal payloads distinguishes a shared body slot
    // from accidentally retaining only one alternative's occurrence.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    elaborate(&mut env, "data OrChoice = OrLeft Nat | OrRight Nat");
    elaborate(
        &mut env,
        "fn choose_payload (value : OrChoice) : Nat = match value { \
         OrLeft payload | OrRight payload |-> payload }",
    );
    let left = elaborate(
        &mut env,
        "const chosen_left : Nat = choose_payload (OrLeft Zero)",
    );
    let right = elaborate(
        &mut env,
        "const chosen_right : Nat = choose_payload (OrRight (Suc Zero))",
    );

    let zero = constructor(env.globals["Zero"], []);
    let one = constructor(env.globals["Suc"], [zero.clone()]);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, left)), zero);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, right)), one);
    let (eliminators, projections) = count_nodes(&body(&env, env.globals["choose_payload"]));
    assert!(
        eliminators >= 1,
        "or-patterns reuse ordinary data elimination"
    );
    assert_eq!(projections, 0, "or-patterns add no projection carrier");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn three_alternatives_are_flat_and_map_binders_by_name() {
    // MEASURED: the second constructor writes `(flag, payload)` while the first
    // and third write `(payload, flag)`, yet all three feed the canonical first
    // alternative's `payload` slot. CLAIMED: later alternatives map bindings by
    // name, not traversal position, and a chain is one flat union.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "data OrTriple = OrFirst Nat Bool | OrSecond Bool Nat | OrThird Nat Bool",
    );
    elaborate(
        &mut env,
        "fn triple_payload (value : OrTriple) : Nat = match value { \
         OrFirst payload flag | OrSecond flag payload | OrThird payload flag |-> payload }",
    );
    let selected = elaborate(
        &mut env,
        "const triple_selected : Nat = triple_payload (OrSecond False (Suc Zero))",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

fn parse_error(source: &str) -> (String, ken_elaborator::Span) {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrPair = OrPairCtor Nat Nat");
    elaborate(&mut env, "data OrSides = OrSideL Nat | OrSideR Nat");
    match env.elaborate_decl(source) {
        Err(ElabError::ParseError { msg, span }) => (msg, span),
        other => panic!("expected a pattern well-formedness error, got {other:?}"),
    }
}

#[test]
fn binder_sets_and_intra_alternative_duplicates_are_rejected() {
    let mismatch = "const bad_names : Nat = match OrSideL Zero { \
         OrSideL left | OrSideR right |-> left }";
    let (msg, span) = parse_error(mismatch);
    assert_eq!(
        msg,
        "or-pattern alternatives must bind the same names; expected [\"left\"], found [\"right\"]"
    );
    let second = mismatch.rfind("OrSideR right").expect("second alternative");
    assert_eq!(span.start, second);

    let duplicate = "const bad_duplicate : Nat = match OrPairCtor Zero Zero { \
         OrPairCtor repeated repeated | OrPairCtor left right |-> left }";
    let duplicate_start =
        duplicate.find("repeated repeated").expect("duplicate pair") + "repeated ".len();
    let (msg, span) = parse_error(duplicate);
    assert_eq!(
        msg,
        "or-pattern alternative binds 'repeated' more than once"
    );
    assert_eq!(span.start, duplicate_start);
    assert_eq!(span.end, duplicate_start + "repeated".len());
}

#[test]
fn definitionally_equal_common_context_binder_types_are_accepted() {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "def OrNatAlias = Nat");
    elaborate(
        &mut env,
        "data OrDefEq = OrDirect Nat | OrAliased OrNatAlias",
    );
    elaborate(
        &mut env,
        "fn defeq_payload (value : OrDefEq) : Nat = match value { \
         OrDirect payload | OrAliased payload |-> payload }",
    );
    let selected = elaborate(
        &mut env,
        "const defeq_selected : Nat = defeq_payload (OrAliased (Suc Zero))",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn unequal_common_context_binder_types_require_separate_arms() {
    // MEASURED: the same name receives Nat and Bool under distinct alternatives
    // and selects the specific common-context diagnostic. CLAIMED: branch-local
    // elaboration never invents a join for unequal binder types.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrDifferent = OrNat Nat | OrBool Bool");
    match env.elaborate_decl(
        "const bad_types : Nat = match OrNat Zero { \
         OrNat payload | OrBool payload |-> Zero }",
    ) {
        Err(ElabError::TypeMismatch { reason, .. }) => assert_eq!(
            reason,
            "or-pattern binder 'payload' must have definitionally equal types in the common pre-branch context; use separate arms"
        ),
        other => panic!("expected the common-context binder-type diagnostic, got {other:?}"),
    }

    elaborate(
        &mut env,
        "data OrRefined : Type -> Type 1 where { \
         OrKnown : Nat -> OrRefined Nat ; \
         OrGeneral : (carrier : Type) -> carrier -> OrRefined carrier }",
    );
    match env.elaborate_decl(
        "fn bad_refinement_only (value : OrRefined Nat) : Nat = match value { \
         OrKnown payload | OrGeneral _ payload |-> Zero }",
    ) {
        Err(ElabError::TypeMismatch { reason, .. }) => assert_eq!(
            reason,
            "or-pattern binder 'payload' must have definitionally equal types in the common pre-branch context; use separate arms"
        ),
        other => panic!("refinement-only agreement must require separate arms, got {other:?}"),
    }
}

#[test]
fn fully_subsumed_union_is_subsumed_never_no_inhabitants() {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrReach = OrA | OrB | OrC");
    match env.elaborate_decl(
        "const fully_dead : Nat = match OrA { \
         OrA |-> Zero ; OrB |-> Zero ; OrA | OrB |-> Zero ; OrC |-> Zero }",
    ) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { rest, .. },
            ..
        }) => assert_eq!(rest.len(), 1, "both earlier arms cover the dead union"),
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("a fully covered or-arm must not be NoInhabitants"),
        other => panic!("expected union subsumption, got {other:?}"),
    }
}

#[test]
fn partially_subsumed_union_remains_reachable_and_exhaustive() {
    // MEASURED: OrA is already covered, OrB remains live through the union, and
    // OrC closes the family. CLAIMED: reachability is existential over an arm's
    // alternatives while coverage is their union.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrPartial = OrPA | OrPB | OrPC");
    let selected = elaborate(
        &mut env,
        "const partial_live : Nat = match OrPB { \
         OrPA |-> Zero ; OrPA | OrPB |-> Suc Zero ; OrPC |-> Zero }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

fn assert_top_refusal(pattern: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrTop = OrTopA | OrTopB");
    let source = format!("const refused : Nat = match OrTopA {{ {pattern} |-> Zero }}");
    match env.elaborate_decl(&source) {
        Err(ElabError::Internal(msg)) => assert_eq!(
            msg,
            "non-constructor pattern in match (wildcard/var not yet supported at top level; \
             use constructor patterns)"
        ),
        other => panic!("top-level catch-all alternative must stay refused, got {other:?}"),
    }
}

#[test]
fn top_level_wildcard_and_variable_alternatives_remain_refused() {
    assert_top_refusal("_ | OrTopA");
    assert_top_refusal("value | OrTopA as value");
}

#[test]
fn module_rewriting_recurses_through_or_alternatives() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module OrOwner { \
         data LocalOr = LocalLeft Nat | LocalRight Nat ; \
         fn select (value : LocalOr) : Nat = match value { \
           LocalLeft payload | LocalRight payload |-> payload \
         } ; \
         pub const result : Nat = select (LocalRight (Suc Zero)) \
         }",
    )
    .expect("module-local or-pattern constructors rewrite recursively");
    let selected = elaborate(&mut env, "const module_or_result : Nat = OrOwner.result");
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn anonymous_alternatives_do_not_shift_outer_variables() {
    // MEASURED: two alternative-specific wildcard fields bind no source name,
    // and the body still selects the enclosing function parameter. CLAIMED:
    // occurrence-backed alternation slots hide their core binders per residual
    // row rather than shifting ordinary outer de Bruijn references.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "data OrAnon = OrAnonL Nat | OrAnonR Nat");
    elaborate(
        &mut env,
        "fn retain_or_outer (fallback : Nat) (value : OrAnon) : Nat = match value { \
         OrAnonL _ | OrAnonR _ |-> fallback }",
    );
    let selected = elaborate(
        &mut env,
        "const retained_or_outer : Nat = retain_or_outer (Suc Zero) (OrAnonR Zero)",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn or_patterns_compose_inside_tuple_record_constructor_and_as_patterns() {
    // MEASURED: one nested or-pattern traverses all three already-landed inner
    // forms and returns the whole selected Nat occurrence through `as`.
    // CLAIMED: alternation duplicates the existing residual row at its current
    // column instead of adding a special descent path.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "record OrEnvelope { pair : (x : Nat) × Nat }");
    elaborate(&mut env, "data OrBox = OrBoxCtor OrEnvelope");
    elaborate(
        &mut env,
        "const or_box : OrBox = OrBoxCtor { pair = (Suc Zero, Zero) }",
    );
    let selected = elaborate(
        &mut env,
        "const nested_or : Nat = match or_box { \
         OrBoxCtor { pair = ((Zero as whole | Suc _ as whole), other) } |-> whole }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}
