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
