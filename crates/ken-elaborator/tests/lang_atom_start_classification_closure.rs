//! `LANG-ATOM-START-CLASSIFICATION-CLOSURE` — atom-start classification.
//!
//! AC-9: the type side's two NEGATIVE start conditions, preserved as
//! form-keyed exclusions rather than collapsed into the admitted set.
//!
//! **Every case here is a REFUSAL control.** `can_start_atom_type` feeds two
//! type-application `while` loops, so each exclusion's job is to stop the loop
//! consuming input that belongs to the enclosing construct. A closure that
//! kept every admission and dropped these would be fail-open in type parsing,
//! and a suite that only checked admissions would stay green over it.

use ken_elaborator::parser::parse_decls;

fn parses(src: &str) -> bool {
    parse_decls(src).is_ok()
}

#[test]
fn ac9_the_binder_name_exclusion_admits_separator_free_class_fields() {
    // THE CASE THIS GUARD EXISTS TO REFUSE, found by measurement rather than
    // by reasoning about it.
    //
    // Class fields may be written without a `;` separator. Parsing `f`'s type
    // `Bool` runs a type-application `while` loop, and the next tokens are
    // `g` `:`. Without the exclusion the loop takes `g` as another atom
    // argument of `Bool`, and the parse then dies on the `:` it did not
    // expect -- "expected identifier, found Colon".
    //
    // So the guard is not a tidiness rule: it is what makes separator-free
    // class bodies parse at all.
    assert!(
        parses("class C a { f : Bool g : Bool }"),
        "separator-free class fields must parse"
    );
    // Same body with the optional separator, so a reader can see the guard is
    // about the SEPARATOR-FREE reading specifically.
    assert!(
        parses("class C a { f : Bool ; g : Bool }"),
        "the separated form must keep parsing"
    );
}

#[test]
fn ac9_the_effect_row_exclusion_keeps_visits_out_of_the_return_type() {
    // `visits [FS]` is an effect-row annotation on the signature. Without the
    // exclusion the return type's application loop consumes `visits` as an
    // atom argument of the return type.
    assert!(
        parses("proc p (a : Auth) : Unit visits [FS] = q"),
        "a `visits` row must not be consumed as a return-type argument"
    );
}

#[test]
fn ac9_positive_control_the_excluded_tokens_still_start_atoms_elsewhere() {
    // The exclusions are CONTEXTUAL, not removals from the admitted set. Both
    // tokens must still begin an atom where no exclusion applies -- otherwise
    // the guards would be over-broad and the refusal controls above would pass
    // for the wrong reason.
    assert!(
        parses("fn f (x : T y) : Bool = z"),
        "a lowercase ident IS a valid type argument when not a binder name"
    );
    assert!(
        parses("fn f (x : T visits) : Bool = z"),
        "`visits` not followed by `[` is an ordinary type argument"
    );
}

// ---------------------------------------------------------------------------
// AC-6 — the contextual-form derivation, and the POSITION axis it produced.
//
// The derivation found a third negative start condition, on the pattern side:
// `can_start_atom_pat` excluded `as` inline. The frame's "the pattern rosters
// are clean" is true of `can_start_pattern` -- a flat `matches!`, no guards --
// and `can_start_atom_pat` is a different function that carried one.
//
// That is what makes `applies_in` a MEASURED discriminator rather than
// scaffolding: until now every exclusion governed the one position, so
// `applies_in` was indistinguishable from `|_| true`.
// ---------------------------------------------------------------------------

#[test]
fn ac6_the_as_alias_exclusion_governs_the_pattern_position() {
    // PARSE SUCCESS CANNOT SEE THIS DEFECT, and my first version of this case
    // was vacuous for exactly that reason: with the exclusion removed,
    // `C x as w` still parses -- as `C` applied to THREE patterns (`x`, a
    // variable named `as`, and `w`) instead of an as-pattern over `C x`. Both
    // readings parse; only one has the right shape.
    //
    // ARITY is the behavioural discriminator. `C` takes one argument, so the
    // broken reading is an elaboration error and the correct one is not.
    let mut env = ken_elaborator::ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "data T = C Bool \
         fn f (v : T) : Bool = match v { C x as w ↦ x }",
    )
    .expect("an as-pattern over a one-argument constructor must elaborate");
}

#[test]
fn ac6_and_it_does_not_govern_the_type_position() {
    // THE OTHER HALF OF THE PAIR. `as` is an ordinary identifier in a type
    // argument, so the exclusion must NOT fire here. A single-position test
    // could not tell `applies_in` apart from `|_| true`; this pair can.
    assert!(
        parses("fn f (x : T as) : Bool = y"),
        "`as` is a valid type argument -- the pattern exclusion must not reach here"
    );
}

#[test]
fn ac6_a_constructor_pattern_still_takes_several_arguments() {
    // Positive control for the pattern side: the exclusion is CONTEXTUAL, so
    // ordinary pattern arguments must keep being admitted. Without this, an
    // exclusion that refused every pattern argument would pass both cases
    // above for the wrong reason.
    assert!(
        parses("fn f (v : T) : Bool = match v { C x y ↦ x }"),
        "a constructor pattern must still take several arguments"
    );
}

// ---------------------------------------------------------------------------
// The expression position's two vetoes, previously inline breaks in
// `parse_app_expr`'s loop.
//
// Both discriminate through plain parse success, and I checked that by
// neutering each rather than assuming it -- the as-pattern case above taught
// me that a wrong parse is still a parse.
// ---------------------------------------------------------------------------

#[test]
fn the_match_equation_binder_is_not_a_scrutinee_argument() {
    // Without the veto the application loop takes `eqn` as another argument of
    // the scrutinee and the parse dies on the colon:
    // "expected LBrace, found Colon".
    assert!(
        parses("fn f (v : T) : Bool = match v eqn: h { C ↦ z }"),
        "`match v eqn: h` must parse"
    );
    assert!(
        parses("fn f (v : T) : Bool = match v { C ↦ z }"),
        "the form without the binder must keep parsing"
    );
}

#[test]
fn the_effect_row_veto_governs_the_expression_position_too() {
    // THE ARM I SHIPPED AS UNMEASURED, now measured. The fixture is the
    // Architect's (evt_c7ga5m781kgq); they proposed it without running it and
    // flagged it might be void. It is not.
    //
    // Why my earlier attempt could not see it: BOTH readings of `g visits [x]`
    // fail, so success-versus-failure is blind. They fail at DIFFERENT
    // COORDINATES, and that is the discriminator.
    //
    //   with the exclusion     loop stops at `visits`  -> error spans `visits`
    //   without it             `visits` is taken as an argument, the loop
    //                          stops at `[`            -> error spans `[`
    //
    // The mirror of this node's vacuous-control family: there both readings
    // PARSED, here both readings FAIL. Either way the observable has to be
    // finer than the outcome.
    const SOURCE: &str = "const k : Nat = g visits [x]";
    let visits_at = SOURCE.find("visits").expect("fixture contains `visits`");

    let error = parse_decls(SOURCE).expect_err("`g visits [x]` is not a valid expression");
    let rendered = format!("{error:?}");
    let expected = format!("start: {visits_at}, end: {}", visits_at + "visits".len());

    assert!(
        rendered.contains(&expected),
        "the error must land on `visits` -- the expression loop stopping there is \
         the exclusion firing. Landing on `[` means `visits` was consumed as an \
         argument and the Expression membership is inert. got {rendered}"
    );
}
