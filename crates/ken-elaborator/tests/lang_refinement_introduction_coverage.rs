use ken_elaborator::{ElabEnv, ElabError, ElabResult, ObligationKind};
use ken_kernel::{Decl, Level, Term};

fn open_refinements(env: &ElabEnv, result: &ElabResult) -> usize {
    result.obligations.iter().filter(|obligation| {
        matches!(obligation.kind, ObligationKind::RefinementIntroduction)
            && env.is_open_hole(obligation.hole_id)
    }).count()
}

fn opaque_predicate(env: &mut ElabEnv) {
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("fixture predicate");
}

#[test]
fn instance_fields_and_expression_entry_share_the_literal_refinement_boundary() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("class Pick (a : Type) { pick : a }")
        .expect("class");
    let instance = env.elaborate_decl_v1("instance Pick Char { pick = 55296 }")
        .expect("named Char field");
    assert_eq!(open_refinements(&env, &instance), 1);
    let literal_instance = env.elaborate_decl_v1(
        "instance Pick Int { pick = (55296 : {c : Int | isScalar c}) }",
    ).expect("literal ascription in an instance field");
    assert_eq!(open_refinements(&env, &literal_instance), 1);
    let ordinary = env.elaborate_decl_v1("const bad_scalar : Char = 55296")
        .expect("reported Char introduction");
    assert_eq!(open_refinements(&env, &ordinary), 1);

    let mut standalone = ElabEnv::new().expect("prelude");
    let literal_error = standalone.elaborate_expr("literal", "(55296 : {c : Int | isScalar c})")
        .expect_err("literal open scalar cannot be reported");
    assert!(matches!(literal_error, ElabError::ObligationWithoutChannel { .. }), "{literal_error:?}");
    let named_error = standalone.elaborate_expr("named", "(55296 : Char)")
        .expect_err("named open scalar cannot be reported");
    assert!(matches!(named_error, ElabError::ObligationWithoutChannel { .. }), "{named_error:?}");
}

#[test]
fn alias_chains_carry_the_root_predicate_and_reuse_is_identity_based() {
    let mut env = ElabEnv::new().expect("prelude");
    opaque_predicate(&mut env);
    env.elaborate_decl("def Five = { x : Int | P x }").expect("refinement");
    env.elaborate_decl("def Five2 = Five").expect("alias");
    env.elaborate_decl("def Five3 = Five2").expect("alias chain");
    for (alias, target) in [("Five2", "Five"), ("Five3", "Five2")] {
        let Some(Decl::Transparent { body: Term::Const { id, .. }, .. }) =
            env.env.lookup(env.globals[alias]) else {
                panic!("{alias} must elaborate to a bare checked target Const")
            };
        assert_eq!(*id, env.globals[target], "{alias} target identity");
    }
    let fresh = env.elaborate_decl_v1("const value : Five3 = 0").expect("new value");
    assert_eq!(open_refinements(&env, &fresh), 1);
    let reuse = env.elaborate_decl_v1("fn reuse (p : Five3) : Five = p")
        .expect("a value at the same refinement root");
    assert_eq!(reuse.obligations.len(), 0, "same-root reuse is free");
    let reverse = env.elaborate_decl_v1("fn reverse (p : Five) : Five2 = p")
        .expect("reverse same-root reuse");
    assert_eq!(reverse.obligations.len(), 0);
    let distinct = env.elaborate_decl_v1("const from_int : Five2 = 1")
        .expect("a carrier entering the alias still incurs the predicate");
    assert_eq!(open_refinements(&env, &distinct), 1);

    env.elaborate_decl("def MyChar = Char").expect("prelude alias");
    let invalid = env.elaborate_decl_v1("const invalid : MyChar = 55296")
        .expect("alias introduction");
    assert_eq!(open_refinements(&env, &invalid), 1);
}

#[test]
fn aliases_refuse_erased_nested_predicates_where_literal_function_refuses() {
    let mut env = ElabEnv::new().expect("prelude");
    opaque_predicate(&mut env);
    let expected = "a refinement under a function-valued return type is not supported yet";
    let literal = env.elaborate_decl_v1(
        "const literal : Int -> {x : Int | P x} = \\m. m",
    ).expect_err("literal nested refinement must refuse");
    assert!(matches!(literal, ElabError::TypeMismatch { ref reason, .. } if reason == expected), "{literal:?}");
    let named = env.elaborate_decl_v1("def FiveFn = Int -> {x : Int | P x}")
        .expect_err("the alias itself must refuse before predicate erasure");
    assert!(matches!(named, ElabError::TypeMismatch { ref reason, .. } if reason == expected), "{named:?}");
    let parameter_src = "def G = Int → ({ x : Int | Equal Int x 5 })";
    let parameter = env.elaborate_decl_v1(parameter_src)
        .expect_err("a function-nested refinement must refuse at alias declaration");
    assert!(matches!(parameter, ElabError::TypeMismatch { ref reason, ref span }
        if reason == expected && span.start == 16 && span.end == 45),
        "{parameter:?}");
}

#[test]
fn type_application_cannot_hide_a_refinement_in_an_alias() {
    let mut env = ElabEnv::new().expect("prelude");
    let alias_src = "def L = List ({ x : Int | Equal Int x 5 })";
    let err = env.elaborate_decl_v1(alias_src)
        .expect_err("a nested refinement has no predicate-owning alias root");
    assert!(matches!(err, ElabError::TypeMismatch { ref reason, ref span }
        if reason == "a refinement nested inside a named type alias is not supported yet"
           && span.start == 13 && span.end == 42), "{err:?}");
    // Literal-side diagnostic only: if this accepts without an obligation,
    // that existing limitation belongs to the Architect, not this repair.
    let literal = env.elaborate_decl_v1(
        "const xs : List ({ x : Int | Equal Int x 5 }) = Cons Int 5 (Nil Int)",
    );
    eprintln!("literal nested List refinement outcome: {literal:?}");
}

#[test]
fn space_cells_and_operations_report_named_introductions() {
    let mut env = ElabEnv::new().expect("prelude");
    let rows = env.elaborate_file_v1(
        "space GlyphSpace { \
           mut initial : Char = 55296 \
           proc bad () : Char visits [GlyphSpace] = 55296 \
         }",
    ).expect("space with two reported introduction sites");
    assert_eq!(rows.len(), 2, "state and operation each produce a result");
    assert_eq!(open_refinements(&env, &rows[0]), 1, "cell initial value");
    assert_eq!(open_refinements(&env, &rows[1]), 1, "operation return value");
    let literal = env.elaborate_file_v1(
        "space GlyphLiteral { \
           mut initial : Int = (55296 : {c : Int | isScalar c}) \
           proc bad () : Int visits [GlyphLiteral] = (55296 : {c : Int | isScalar c}) \
         }",
    ).expect("literal ascriptions at the same two space value sites");
    assert_eq!(literal.len(), rows.len());
    assert_eq!(open_refinements(&env, &literal[0]), 1, "literal cell");
    assert_eq!(open_refinements(&env, &literal[1]), 1, "literal operation");
}

#[test]
fn checked_theorem_body_reports_named_and_literal_refinements() {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "const five : Int = 5",
        "def Five = { x : Int | Equal Int x 5 }",
        "theorem proof_five (p : Five) : Equal Int p p = Refl",
        "theorem proof_int (p : Int) : Equal Int p p = Refl",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    let named = env.elaborate_decl_v1("theorem thm_named : Equal Int five five = proof_five five");
    let literal = env.elaborate_decl_v1(
        "theorem thm_lit : Equal Int five five = proof_int (five : { x : Int | Equal Int x 5 })",
    );
    let named = named.expect("named theorem body introduction");
    let literal = literal.expect("literal theorem body introduction");
    let named_refinements = named.obligations.iter().filter(|obligation|
        matches!(obligation.kind, ObligationKind::RefinementIntroduction)).count();
    let literal_refinements = literal.obligations.iter().filter(|obligation|
        matches!(obligation.kind, ObligationKind::RefinementIntroduction)).count();
    assert_eq!((named_refinements, literal_refinements), (1, 1));
    assert_eq!(named.obligations[0].goal_closed, literal.obligations[0].goal_closed,
        "both introductions must instantiate the same predicate at five");
    assert_eq!(open_refinements(&env, &named), open_refinements(&env, &literal));
}

/// F-C durable invariant: a theorem parameter written as a literal refinement
/// and its named twin both emit the same call-site introduction obligation.
#[test]
fn literal_refined_theorem_parameter_matches_named_call_site_obligation() {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "const six : Int = 6",
        "def Five = { x : Int | Equal Int x 5 }",
        "theorem proof_five (p : Five) : Equal Int p p = Refl",
        "theorem proof_lit (p : { x : Int | Equal Int x 5 }) : Equal Int p p = Refl",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    let named = env.elaborate_decl_v1("theorem thm_named : Equal Int six six = proof_five six")
        .expect("named call");
    let literal = env.elaborate_decl_v1("theorem thm_lit : Equal Int six six = proof_lit six")
        .expect("literal call");
    assert_eq!(open_refinements(&env, &named), 1);
    assert_eq!(open_refinements(&env, &literal), 1);
    assert_eq!(named.obligations[0].goal_closed, literal.obligations[0].goal_closed,
        "the literal and named routes must instantiate the same predicate");
}

#[test]
fn prove_law_and_contract_propositions_report_nested_named_introductions() {
    let mut env = ElabEnv::new().expect("prelude");
    opaque_predicate(&mut env);
    env.elaborate_decl("def Five = { x : Int | P x }")
        .expect("alias");
    let proof = env.elaborate_decl_v1(
        "prove has_intro : Equal Int (0 : Five) 0",
    ).expect("prove proposition");
    assert_eq!(open_refinements(&env, &proof), 1);
    assert_eq!(proof.obligations.len(), 2, "introduction and prove obligation");
    let literal_proof = env.elaborate_decl_v1(
        "prove lit_intro : Equal Int (0 : {x : Int | P x}) 0",
    ).expect("literal prove proposition");
    assert_eq!(open_refinements(&env, &literal_proof), 1);
    assert_eq!(literal_proof.obligations.len(), proof.obligations.len());
    let law = env.elaborate_decl_v1(
        "law IntroLaw (M) { field : Equal Int (0 : Five) 0 }",
    ).expect("law-field proposition");
    assert_eq!(open_refinements(&env, &law), 1);
    assert_eq!(law.obligations.len(), 2, "introduction and law obligation");
    let literal_law = env.elaborate_decl_v1(
        "law LiteralLaw (M) { field : Equal Int (0 : {x : Int | P x}) 0 }",
    ).expect("literal law-field proposition");
    assert_eq!(open_refinements(&env, &literal_law), 1);
    assert_eq!(literal_law.obligations.len(), law.obligations.len());
    let contract = env.elaborate_decl_v1(
        "fn post_clause (n : Int) : Int ensures Equal Int (0 : Five) 0 = n",
    ).expect("contract clause proposition");
    assert_eq!(open_refinements(&env, &contract), 1);
    assert_eq!(contract.obligations.len(), 2, "introduction and postcondition");

    let literal = env.elaborate_decl_v1(
        "fn literal_clause (n : Int) : Int ensures Equal Int (0 : {x : Int | P x}) 0 = n",
    ).expect("literal twin");
    assert_eq!(open_refinements(&env, &literal), 1);
    assert_eq!(literal.obligations.len(), contract.obligations.len());
}
