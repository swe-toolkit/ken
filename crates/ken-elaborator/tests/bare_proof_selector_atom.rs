//! Acceptance coverage for the bare attached-proof selector atom.
//!
//! Pins `spec/30-surface/32-grammar.md` and
//! `spec/30-surface/33-declarations.md` section 8.2.

use ken_elaborator::{
    parser::{parse_decls, parse_expr},
    Decl, ElabEnv, Expr,
};

fn selector_bytes(source: &str) -> Vec<u8> {
    match parse_expr(source).unwrap_or_else(|error| panic!("{source:?} should parse: {error}")) {
        Expr::EAttachedProofRef {
            subject,
            proof_name,
            ..
        } => [subject.as_bytes(), b"\0", proof_name.as_bytes()].concat(),
        other => panic!("{source:?} should be an attached-proof selector, got {other:?}"),
    }
}

#[test]
fn bare_grouped_and_canonical_selectors_have_identical_ast_and_elaboration() {
    let bare = selector_bytes("proof p for s");
    let grouped = selector_bytes("(proof p for s)");
    let canonical = selector_bytes("s::p");
    assert_eq!(bare, grouped);
    assert_eq!(bare, canonical);

    let mut env = ElabEnv::new().expect("base env construction failed");
    env.elaborate_file(
        r#"
        fn s (x : Int) : Int = x
        proof p for s (x : Int) : Equal Int (s x) x = Refl
        theorem via_bare (x : Int) : Equal Int (s x) x = proof p for s x
        theorem via_grouped (x : Int) : Equal Int (s x) x = (proof p for s) x
        theorem via_canonical (x : Int) : Equal Int (s x) x = s::p x
        "#,
    )
    .expect("all three selector spellings should elaborate");

    let body = |name: &str| {
        env.env
            .transparent_body(env.globals[name])
            .unwrap_or_else(|| panic!("{name} should be transparent"))
            .1
    };
    assert_eq!(body("via_bare"), body("via_grouped"));
    assert_eq!(body("via_bare"), body("via_canonical"));
}

#[test]
fn application_wraps_outside_the_single_path_selector_subject() {
    let parsed = parse_expr("proof p for Mod.s a b").expect("applied bare selector should parse");
    match parsed {
        Expr::EApp(outer_fun, outer_arg, _) => {
            assert!(matches!(*outer_arg, Expr::EVar(ref name, _) if name == "b"));
            match *outer_fun {
                Expr::EApp(inner_fun, inner_arg, _) => {
                    assert!(matches!(*inner_arg, Expr::EVar(ref name, _) if name == "a"));
                    assert!(matches!(
                        *inner_fun,
                        Expr::EAttachedProofRef {
                            ref subject,
                            ref proof_name,
                            ..
                        } if subject == "Mod.s" && proof_name == "p"
                    ));
                }
                other => panic!("expected the inner application spine, got {other:?}"),
            }
        }
        other => panic!("expected ((proof p for Mod.s) a) b, got {other:?}"),
    }
}

#[test]
fn recursive_attached_proof_can_use_its_bare_selector() {
    let mut env = ElabEnv::new().expect("base env construction failed");
    env.elaborate_file(
        r#"
        fn leq_nat (m : Nat) (n : Nat) : Bool =
          match m {
            Zero |-> True ;
            Suc m2 |-> match n { Zero |-> False ; Suc n2 |-> leq_nat m2 n2 }
          }
        proof refl for leq_nat (x : Nat) : Equal Bool (leq_nat x x) True =
          match x { Zero |-> Proved ; Suc x2 |-> proof refl for leq_nat x2 }
        "#,
    )
    .expect("descending recursive proof through a bare self-selector should elaborate");

    assert!(env.globals.contains_key("leq_nat::refl"));
}

#[test]
fn declaration_head_and_bare_selector_body_are_parsed_and_elaborated_separately() {
    let source = r#"
        fn s (x : Int) : Int = x
        proof p for s (x : Int) : Equal Int (s x) x = Refl
        proof q for s (x : Int) : Equal Int (s x) x = proof p for s x
    "#;
    let declarations = parse_decls(source).expect("proof declaration with selector body parses");
    let declaration = declarations
        .last()
        .expect("q declaration should be present");
    match declaration {
        Decl::AttachedProofDecl {
            proof_name,
            subject,
            body,
            ..
        } => {
            assert_eq!(proof_name, "q", "declaration-position proof names the head");
            assert_eq!(subject, "s", "declaration-position `for` names the subject");
            assert!(matches!(
                body,
                Expr::EApp(fun, arg, _)
                    if matches!(
                        &**fun,
                        Expr::EAttachedProofRef {
                            subject,
                            proof_name,
                            ..
                        } if subject == "s" && proof_name == "p"
                    ) && matches!(&**arg, Expr::EVar(name, _) if name == "x")
            ));
        }
        other => panic!("expected an attached-proof declaration, got {other:?}"),
    }

    let mut env = ElabEnv::new().expect("base env construction failed");
    env.elaborate_file(source)
        .expect("declaration head and bare-selector body should elaborate without cross-talk");
    assert!(env.globals.contains_key("s::q"));
}

// ---------------------------------------------------------------------
// AC-5: the POSITION axis.
//
// Every test above varies the SPELLING -- bare, grouped, canonical `s::p` --
// and holds the POSITION at the application HEAD. The head call site is
// ungated; the roster gates the argument loop and the operator-prefix tail.
// So the green suite above routes around the predicate under test.
//
// These hold the spelling and vary the position instead.
// ---------------------------------------------------------------------

const PRELUDE: &str = "fn s (x : Int) : Int = x\n\
                       proof p for s (x : Int) : Equal Int (s x) x = Refl\n";

/// AC-5 -- a bare multi-word selector is REFUSED in argument position, and
/// that refusal is CORRECT rather than a gap this node owes.
///
/// `proof` is a declaration keyword, so the complement of the atom-start
/// roster is the DECLARATION SEPARATOR -- Ken has no declaration terminator.
/// A bare `proof p for s` after an applied head therefore ends the expression
/// and starts a new declaration, which then demands its own `:`. Measured
/// here: `expected Colon, found Eof`.
///
/// That is structural for any parser of this grammar, so the row is a
/// NEGATIVE CONTROL THAT STAYS REFUSED -- it is not a target. The two rows
/// below it are its positive control: without them, "refused" would be
/// satisfied by a harness that refuses everything.
#[test]
fn ac5_bare_selector_in_argument_position_is_refused_and_grouping_is_the_escape_hatch() {
    // NEGATIVE: bare, in argument position.
    let bare = parse_decls(&format!("{PRELUDE}const k : Int = f proof p for s"));
    let error = bare.expect_err(
        "a BARE multi-word selector in argument position must stay refused -- \
         `proof` starts a declaration, and this grammar has no declaration \
         terminator",
    );
    assert!(
        matches!(error, ken_elaborator::ElabError::ParseError { .. }),
        "the refusal must be a ParseError, not some later phase: {error:?}"
    );
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains("Colon"),
        "the refusal must carry the DECLARATION-SEPARATOR signature -- the \
         parser read `proof` as a new declaration head and wanted its `:`. If \
         this wording has changed, re-establish that the refusal still comes \
         from declaration separation rather than from an unrelated failure \
         before editing this assertion: {rendered}"
    );

    // POSITIVE CONTROL 1: the same selector, grouped, in the same position.
    // This is what makes the refusal above mean something.
    let grouped = parse_decls(&format!("{PRELUDE}const k : Int = f (proof p for s)"))
        .expect("the GROUPED selector is the sanctioned escape hatch and must parse");
    assert!(
        format!("{grouped:?}").contains("EAttachedProofRef"),
        "the grouped argument must still be a selector, not reparsed as \
         something else"
    );

    // POSITIVE CONTROL 2: the canonical spelling needs no grouping at all,
    // because it is a single path token rather than a multi-word form. This
    // is the row that shows the refusal is about CONTEXTUAL FORMS, not about
    // selectors.
    let canonical = parse_decls(&format!("{PRELUDE}const k : Int = f s::p"))
        .expect("the canonical `s::p` spelling must parse bare in argument position");
    assert!(
        format!("{canonical:?}").contains("EAttachedProofRef"),
        "`s::p` in argument position must still be a selector"
    );
}

/// AC-5 -- the loop must CONTINUE past a grouped selector argument.
///
/// The sibling of the truncation suite's AC-2 row: a repair measured only on
/// its rejections going away cannot see what it started swallowing. Here the
/// question is whether `y` survives as a sibling argument.
#[test]
fn ac5_a_grouped_selector_argument_does_not_swallow_the_following_argument() {
    let decls = parse_decls(&format!("{PRELUDE}const k : Int = f (proof p for s) y"))
        .expect("grouped selector followed by another argument must parse");
    let rendered = format!("{decls:?}");
    assert!(
        rendered.contains("EAttachedProofRef"),
        "the selector must survive as a selector"
    );
    assert!(
        rendered.contains("EVar(\"y\""),
        "`y` must survive as a SIBLING argument rather than being swallowed \
         into the grouped selector: {rendered}"
    );

    // And the bare form with a trailing argument stays refused, for the same
    // structural reason as the row above.
    assert!(
        parse_decls(&format!("{PRELUDE}const k : Int = f proof p for s y")).is_err(),
        "the bare form followed by another argument stays refused"
    );
}

/// The YIELD half of `ProofSelector`'s speak-or-yield decision, and it is
/// LOAD-BEARING rather than a default.
///
/// The exclusion layer decides, per member, whether stopping the
/// application-argument loop raises a diagnostic or stops quietly.
/// `IfExpression` speaks; `ProofSelector` yields. **Yielding is not the safe
/// choice — it is the only correct one**, because Ken has no declaration
/// terminator: a declaration's extent ends exactly where its body expression
/// stops, so the atom-start roster's COMPLEMENT is the declaration separator.
/// `proof` opening the next declaration is the ordinary case, not the exotic
/// one.
///
/// **Measured, both outcomes producible (base `7cb535be5`).** Giving
/// `ProofSelector` a distinctly-marked `Some(...)` in `argument_diagnostic`:
///
/// ```text
/// const k : Int = f proof p for s    marker at byte 18   the ARGUMENT case
/// fn s .. = x  /  proof p for s ..   marker at byte 25   THIS case
/// ```
///
/// So the arm is wired — `None` is consulted, not decorative — and flipping it
/// turns the two-declaration program below into a parse error at the `proof`
/// that opens the second declaration. That is this node's opening hazard
/// (*"admitting `proof` makes the declaration sequence lose its separator"*)
/// reproduced from the other side: not by admitting the token to the roster,
/// but by letting the loop speak about it.
#[test]
fn a_declaration_beginning_with_proof_terminates_the_previous_declarations_body() {
    let decls = parse_decls(PRELUDE).expect(
        "a `fn` whose body is a bare identifier, followed by an attached-proof \
         declaration, is an ordinary two-declaration program",
    );
    assert_eq!(
        decls.len(),
        2,
        "the `proof` must START a second declaration rather than be consumed as \
         an argument to the first one's body: {decls:?}"
    );
    assert!(
        matches!(&decls[0], Decl::ViewDecl { name, .. } if name == "s"),
        "the first declaration must be `fn s`, with its body ending at `x`: \
         {:?}",
        decls[0]
    );

    // POSITIVE CONTROL. Without it, "two declarations" is satisfied by a parser
    // that splits everywhere -- the property under test is that the split
    // happens HERE and not inside an application spine.
    let applied = parse_decls("fn s (x : Int) : Int = x\nconst k : Int = s 1 2\n")
        .expect("an applied body must still absorb its arguments");
    assert_eq!(
        applied.len(),
        2,
        "the argument loop must still CONSUME `1` and `2` into one body"
    );
    assert!(
        format!("{applied:?}").matches("EApp").count() >= 2,
        "`s 1 2` must be a two-argument application spine, not three \
         declarations: {applied:?}"
    );
}
