//! `LANG-FIXITY-DECL-SURFACE` acceptance.
//!
//! Spec sources: `spec/30-surface/32-grammar.md` §6 and
//! `spec/30-surface/33-declarations.md` §6.
//! Promise class: durable invariants. The structural checks distinguish
//! association and precedence without relying on operator evaluation.

use ken_elaborator::resolve::{resolve_decl, RExpr, RInfixOperator};
use ken_elaborator::{BinOp, Decl, ElabEnv, ElabError, Expr};
use ken_kernel::Term;

fn definition_body(env: &ElabEnv, name: &str) -> Term {
    let id = env.globals[name];
    env.env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("{name} must be transparent"))
        .1
        .clone()
}

fn peel_three_lambdas(term: &Term) -> &Term {
    let mut body = term;
    for ordinal in ["first", "second", "third"] {
        let Term::Lam(_, next) = body else {
            panic!("expected {ordinal} lambda, got {body:?}");
        };
        body = next;
    }
    body
}

fn infix_application<'a>(term: &'a Term, operator: ken_kernel::GlobalId) -> (&'a Term, &'a Term) {
    let Term::App(function, rhs) = term else {
        panic!("expected saturated operator application, got {term:?}");
    };
    let Term::App(head, lhs) = function.as_ref() else {
        panic!("expected operator applied to lhs, got {function:?}");
    };
    assert!(
        matches!(head.as_ref(), Term::Const { id, .. } if *id == operator),
        "expected operator {operator:?}, got {head:?}"
    );
    (lhs, rhs)
}

fn assert_left_chain(env: &ElabEnv, consumer: &str, operator_name: &str) {
    let operator = env.globals[operator_name];
    let body = definition_body(env, consumer);
    let body = peel_three_lambdas(&body);
    let (lhs, rhs) = infix_application(body, operator);
    assert!(matches!(rhs, Term::Var(0)), "right leaf must be c: {rhs:?}");
    let (a, b) = infix_application(lhs, operator);
    assert!(matches!(a, Term::Var(2)), "left leaf must be a: {a:?}");
    assert!(matches!(b, Term::Var(1)), "middle leaf must be b: {b:?}");
}

fn assert_right_chain(env: &ElabEnv, consumer: &str, operator_name: &str) {
    let operator = env.globals[operator_name];
    let body = definition_body(env, consumer);
    let body = peel_three_lambdas(&body);
    let (lhs, rhs) = infix_application(body, operator);
    assert!(matches!(lhs, Term::Var(2)), "left leaf must be a: {lhs:?}");
    let (b, c) = infix_application(rhs, operator);
    assert!(matches!(b, Term::Var(1)), "middle leaf must be b: {b:?}");
    assert!(matches!(c, Term::Var(0)), "right leaf must be c: {c:?}");
}

/// MEASURED: unit parsing and lexical resolution both retain one flat spine
/// with two operators and three operands. CLAIMED: reassociation is not an
/// in-parser or name-resolution action. THE GAP: checking only the parsed AST
/// would permit resolution to restore the old hard-wired fold, so both seams
/// are asserted.
#[test]
fn operator_run_stays_flat_through_resolution() {
    let mut decls = ken_elaborator::parser::parse_decls(
        "fn flatUse (a : Nat) (b : Nat) (c : Nat) : Nat = a + b <+> c",
    )
    .expect("unit declaration parses");
    let decl = decls.pop().expect("one declaration");
    let Decl::ViewDecl { body, .. } = &decl else {
        panic!("expected view declaration, got {decl:?}");
    };
    assert!(matches!(
        body,
        Expr::EInfixSpine {
            operands,
            operators,
            ..
        } if operands.len() == 3 && operators.len() == 2
    ));

    let resolved = resolve_decl(&decl).expect("declaration resolves");
    assert!(resolved.contains_infix_spine);
    let mut body = &resolved.body;
    for _ in 0..3 {
        let RExpr::RLam(_, next, _) = body else {
            panic!("expected resolved parameter lambda, got {body:?}");
        };
        body = next;
    }
    assert!(matches!(
        body,
        RExpr::RInfixSpine {
            operands,
            operators,
            ..
        } if operands.len() == 3
            && matches!(operators.as_slice(), [
                RInfixOperator::Builtin(BinOp::Add, _),
                RInfixOperator::User(name, _),
            ] if name == "<+>")
    ));
}

/// MEASURED: a pure fixed-arithmetic run is an `EBinOp`/`RBinOp` tree and its
/// resolution-side declaration marker is false. CLAIMED: legacy arithmetic
/// never pays for the declaration-dependent reassociation traversal. THE GAP:
/// checking only the final value would allow the expensive neutral spine to
/// return, so both AST seams and the skip marker are asserted.
#[test]
fn pure_fixed_arithmetic_retains_merge_base_shape_and_skips_reassociation() {
    let mut decls = ken_elaborator::parser::parse_decls(
        "fn arithmeticUse (a : Nat) (b : Nat) (c : Nat) : Nat = a + b * c",
    )
    .expect("fixed arithmetic parses");
    let decl = decls.pop().expect("one declaration");
    let Decl::ViewDecl { body, .. } = &decl else {
        panic!("expected view declaration, got {decl:?}");
    };
    assert!(matches!(
        body,
        Expr::EBinOp(BinOp::Add, _, rhs, _)
            if matches!(rhs.as_ref(), Expr::EBinOp(BinOp::Mul, _, _, _))
    ));

    let resolved = resolve_decl(&decl).expect("fixed arithmetic resolves");
    assert!(!resolved.contains_infix_spine);
    let mut body = &resolved.body;
    for _ in 0..3 {
        let RExpr::RLam(_, next, _) = body else {
            panic!("expected resolved parameter lambda, got {body:?}");
        };
        body = next;
    }
    assert!(matches!(
        body,
        RExpr::RBinOp(BinOp::Add, _, rhs, _)
            if matches!(rhs.as_ref(), RExpr::RBinOp(BinOp::Mul, _, _, _))
    ));
}

/// MEASURED: the same three-operand source produces opposite kernel-term
/// nesting under left and right declarations, including when the declaration
/// follows its use. CLAIMED: fixity changes structure and collection is
/// module-wide. THE GAP: evaluating a projection-like operator could collapse
/// both trees, so this inspects the stored terms and all three leaves.
#[test]
fn declared_associativity_changes_structure_with_late_declaration() {
    let mut left = ElabEnv::new().expect("base environment");
    left.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         fn leftUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c\n\
         infixl 5 <+>",
    )
    .expect("left declaration after use");
    assert_left_chain(&left, "leftUse", "<+>");

    let mut right = ElabEnv::new().expect("base environment");
    right
        .elaborate_file(
            "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
             fn rightUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c\n\
             infixr 5 <+>",
        )
        .expect("right declaration after use");
    assert_right_chain(&right, "rightUse", "<+>");
}

/// MEASURED: a caller, its fixity declaration, and the operator definition may
/// appear in that order and still elaborate with the declared right grouping.
/// CLAIMED: whole-module collection composes with dependency-first identity
/// admission. THE GAP: placing the operator definition first would not exercise
/// the forward-identity boundary.
#[test]
fn use_and_fixity_may_precede_the_operator_definition() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn forwardUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c\n\
         infixr 5 <+>\n\
         fn <+> (a : Nat) (b : Nat) : Nat = a",
    )
    .expect("dependency-first forward operator");
    assert_right_chain(&env, "forwardUse", "<+>");
}

/// MEASURED: a declared level 5 user operator becomes the root around level-6
/// addition and level-7 multiplication. CLAIMED: declared levels participate
/// in the arithmetic precedence space. THE GAP: the old parser level always
/// put user operators above arithmetic; this exact shape reds under that bug.
#[test]
fn declared_precedence_compares_with_arithmetic_levels() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn <+> (a : Int) (b : Int) : Int = a\n\
         infixl 5 <+>\n\
         fn usePrecedence (a : Int) (b : Int) (c : Int) (d : Int) : Int = a + b <+> c * d\n\
         fn expectedPrecedence (a : Int) (b : Int) (c : Int) (d : Int) : Int = <+> (a + b) (c * d)",
    )
    .expect("mixed declared precedence");

    let body = definition_body(&env, "usePrecedence");
    let mut body_ref = &body;
    for ordinal in ["first", "second", "third", "fourth"] {
        let Term::Lam(_, next) = body_ref else {
            panic!("expected {ordinal} lambda: {body_ref:?}");
        };
        body_ref = next;
    }
    let body = body_ref;
    let operator = env.globals["<+>"];
    let (lhs, rhs) = infix_application(body, operator);
    assert!(
        matches!(lhs, Term::App(_, _)),
        "addition must be left operand"
    );
    assert!(
        matches!(rhs, Term::App(_, _)),
        "multiplication must be right operand"
    );
    assert_eq!(
        definition_body(&env, "usePrecedence"),
        definition_body(&env, "expectedPrecedence")
    );
}

/// MEASURED: two declared canonical identities at levels 5 and 8 group by
/// their table entries, not by token spelling or declaration order. CLAIMED:
/// precedence lookup occurs independently per GlobalId. THE GAP: one declared
/// operator beside arithmetic would not expose a table that accidentally reused
/// the first declaration for every user operator.
#[test]
fn distinct_user_operator_identities_use_distinct_precedence_entries() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         fn <*> (a : Nat) (b : Nat) : Nat = a\n\
         infixl 5 <+>\n\
         infixl 8 <*>\n\
         fn actual (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <*> c\n\
         fn expected (a : Nat) (b : Nat) (c : Nat) : Nat = <+> a (<*> b c)",
    )
    .expect("two declared precedences");
    assert_eq!(
        definition_body(&env, "actual"),
        definition_body(&env, "expected")
    );
}

/// MEASURED: a re-exported/selectively imported operator retains the provider
/// GlobalId and its right fixity. CLAIMED: fixity propagates by identity rather
/// than surface path. THE GAP: a copied table keyed by the imported spelling
/// could pass one direct import; the facade path and identity equality exclude
/// that substitute.
#[test]
fn fixity_propagates_through_reexported_canonical_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module Provider {\n\
           pub fn <+> (a : Nat) (b : Nat) : Nat = a\n\
           infixr 5 <+>\n\
         }\n\
         module Facade { export Provider (<+>) }\n\
         import Facade (<+>)\n\
         fn importedUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c",
    )
    .expect("re-exported fixity");

    let provider = env.globals["Provider.<+>"];
    assert!(env.fixities.contains_key(&provider));
    assert_right_chain(&env, "importedUse", "Provider.<+>");
}

/// MEASURED: an imported operator remains usable with its provider fixity,
/// while a local fixity declaration targeting that imported spelling is
/// rejected at the ownership boundary. CLAIMED: fixity is not re-scopable at
/// a use site. THE GAP: rejection alone could mean imports are unusable, so the
/// same provider/import path is first admitted without the redeclaration.
#[test]
fn imported_fixity_cannot_be_redeclared_at_the_use_site() {
    let provider = "module Provider {\n\
                      pub fn <+> (a : Nat) (b : Nat) : Nat = a\n\
                      infixr 5 <+>\n\
                    }\n\
                    import Provider (<+>)\n";
    let mut admitted = ElabEnv::new().expect("base environment");
    admitted
        .elaborate_file(&format!(
            "{provider}fn useImported (a : Nat) (b : Nat) : Nat = a <+> b"
        ))
        .expect("imported operator with provider fixity");

    let mut rejected = ElabEnv::new().expect("base environment");
    match rejected.elaborate_file(&format!("{provider}infixl 5 <+>")) {
        Err(ElabError::FixityTargetNotLocal { operator, .. }) => {
            assert_eq!(operator, "<+>");
        }
        other => panic!("expected imported-fixity ownership rejection, got {other:?}"),
    }
}

/// MEASURED: equal repeated declarations admit, while one changed association
/// reports both sites in a dedicated error. CLAIMED: agreement is idempotent
/// and conflict is never last-wins. THE GAP: testing only the rejection would
/// permit a blanket duplicate ban, so the agreeing control is paired here.
#[test]
fn agreeing_fixity_is_idempotent_and_conflict_names_both_sites() {
    let mut agreeing = ElabEnv::new().expect("base environment");
    agreeing
        .elaborate_file(
            "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
             infixl 5 <+>\n\
             infixl 5 <+>\n\
             fn useAgreeing (a : Nat) (b : Nat) : Nat = a <+> b",
        )
        .expect("agreeing declarations are idempotent");

    let mut conflicting = ElabEnv::new().expect("base environment");
    match conflicting.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         infixl 5 <+>\n\
         infixr 5 <+>",
    ) {
        Err(ElabError::ConflictingFixity {
            operator,
            first_span,
            second_span,
            ..
        }) => {
            assert_eq!(operator, "<+>");
            assert_ne!(first_span, second_span);
        }
        other => panic!("expected attributed fixity conflict, got {other:?}"),
    }
}

/// MEASURED: the non-associative declaration reaches its own diagnostic, while
/// explicit parentheses admit both groupings. CLAIMED: only an unparenthesized
/// same-level chain is refused. THE GAP: a blanket two-use rejection would also
/// reject the two positive controls.
#[test]
fn non_associative_chain_requires_parentheses() {
    let mut rejected = ElabEnv::new().expect("base environment");
    match rejected.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         infix 5 <+>\n\
         fn bad (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c",
    ) {
        Err(ElabError::NonAssociativeInfix { operator, .. }) => {
            assert_eq!(operator, "<+>");
        }
        other => panic!("expected non-associative diagnostic, got {other:?}"),
    }

    for (name, body) in [
        ("leftGrouped", "(a <+> b) <+> c"),
        ("rightGrouped", "a <+> (b <+> c)"),
    ] {
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file(&format!(
            "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
             infix 5 <+>\n\
             fn {name} (a : Nat) (b : Nat) (c : Nat) : Nat = {body}"
        ))
        .unwrap_or_else(|error| panic!("parenthesized nonassoc use must elaborate: {error:?}"));
    }
}

/// MEASURED: both endpoints parse, while the first value beyond the range has
/// the dedicated bounds diagnostic. CLAIMED: the chosen declared range is
/// exactly 0..=9. THE GAP: testing only 10 would not show that 0 and 9 remain
/// admitted.
#[test]
fn fixity_precedence_range_is_zero_through_nine() {
    for level in [0, 9] {
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_file(&format!(
            "fn <+> (a : Nat) (b : Nat) : Nat = a\ninfixl {level} <+>"
        ))
        .unwrap_or_else(|error| panic!("boundary level {level} must elaborate: {error:?}"));
    }

    let error = ken_elaborator::parser::parse_decls("infixl 10 <+>")
        .expect_err("level 10 must be rejected");
    assert!(matches!(
        error,
        ElabError::InvalidFixityPrecedence { ref written, .. } if written == "10"
    ));
}

/// MEASURED: the declaration is attached only after the symbolic definition's
/// self identity is pre-admitted, and the recursive infix call then resolves.
/// CLAIMED: singleton recursive bodies use the ruled predeclare-to-body seam.
/// THE GAP: a non-recursive fixture would remain green if reassociation moved
/// back before predeclaration.
#[test]
fn recursive_symbolic_definition_reassociates_after_self_preadmission() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = match a {\n\
           Zero |-> b;\n\
           Suc n |-> n <+> b;\n\
         }\n\
         infixr 5 <+>",
    )
    .expect("recursive symbolic operator");
    let id = env.globals["<+>"];
    assert_eq!(
        env.fixities.get(&id).map(|fixity| fixity.precedence),
        Some(5)
    );
}

/// MEASURED: two symbolic definitions in one genuine recursive SCC both gain
/// their GlobalIds before either flat body is reassociated, even with metadata
/// between the definitions. CLAIMED: fixity declarations do not split the SCC
/// and the mutual predeclare-to-body boundary hosts the sole pass. THE GAP: a
/// non-recursive pair would not exercise the ruled group placement.
#[test]
fn mutual_symbolic_group_reassociates_after_all_members_are_preadmitted() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = match a {\n\
           Zero |-> b;\n\
           Suc n |-> n <*> b;\n\
         }\n\
         infixr 5 <+>\n\
         fn <*> (a : Nat) (b : Nat) : Nat = match a {\n\
           Zero |-> b;\n\
           Suc n |-> n <+> b;\n\
         }\n\
         infixl 6 <*>",
    )
    .expect("mutually recursive symbolic operators");

    for operator in ["<+>", "<*>"] {
        let id = env.globals[operator];
        assert!(env.fixities.contains_key(&id));
        assert!(env.env.transparent_body(id).is_some());
    }
}

/// MEASURED: the layout formatter preserves a fixity declaration and the
/// formatted unit re-elaborates with the same right-associated result.
/// CLAIMED: the new metadata/spine nodes participate in the existing lossless
/// formatting surface. THE GAP: byte preservation alone would not prove the
/// formatted declaration is consumed, so the result shape is checked.
#[test]
fn fixity_declarations_survive_layout_formatting() {
    let source = "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
                  infixr 5 <+>\n\
                  fn formattedUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c";
    let formatted = ken_elaborator::layout::format_ken(source).expect("source formats");
    assert!(formatted.contains("infixr 5 <+>"));
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(&formatted)
        .expect("formatted fixity unit elaborates");
    assert_right_chain(&env, "formattedUse", "<+>");
}

/// MEASURED: an undeclared operator has no side-table entry and its three-use
/// term remains left-associated. CLAIMED: table absence, not a pre-seeded row,
/// selects default infixl 9. THE GAP: the structural assertion alone could pass
/// with a seeded default; the negative table assertion distinguishes it.
#[test]
fn undeclared_operator_uses_default_only_on_table_miss() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "fn <+> (a : Nat) (b : Nat) : Nat = a\n\
         fn defaultUse (a : Nat) (b : Nat) (c : Nat) : Nat = a <+> b <+> c",
    )
    .expect("undeclared default");
    let operator = env.globals["<+>"];
    assert!(!env.fixities.contains_key(&operator));
    assert_left_chain(&env, "defaultUse", "<+>");
}
