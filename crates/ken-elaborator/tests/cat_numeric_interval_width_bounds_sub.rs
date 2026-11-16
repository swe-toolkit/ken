//! Spec 68 §18: contained values differ by at most interval width under
//! truncated Rational subtraction. Promise class: durable invariant.
//! Open use and independent closed value proofs exercise the public law,
//! including a false y-containment proof and tight endpoint case.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const LAW: &str = "interval_width_bounds_sub";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational provider roots-loads");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval package roots-loads");
    env
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_sub, rational_leq)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_width, interval_width_bounds_sub)

theorem width_bounds_open (i : Interval) (x : Rational) (y : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains i y)
      → IsTrue (rational_leq (rational_sub y x) (interval_width i)) =
  interval_width_bounds_sub i x y
"#;

const ACCEPTED: &str = r#"
import Data.Numeric.Rational
  (Rational, MkRational, rational_sub, rational_leq, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_width, interval_width_bounds_sub)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const five : Rational = MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) Zero
const five_halves : Rational = MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) (Suc Zero)
const wide : Interval = MkInterval half three
const narrow : Interval = MkInterval one two

theorem a1_contains_one : IsTrue (interval_contains wide one) = Proved
theorem a1_contains_five_halves : IsTrue (interval_contains wide five_halves) = Proved
theorem a1_bound_value
    : IsTrue (rational_leq (rational_sub five_halves one) (interval_width wide)) =
  Proved
theorem a1_bound_by_law
    : IsTrue (rational_leq (rational_sub five_halves one) (interval_width wide)) =
  interval_width_bounds_sub wide one five_halves Proved Proved

theorem a2_contains_half : IsTrue (interval_contains wide half) = Proved
theorem a2_contains_three : IsTrue (interval_contains wide three) = Proved
theorem a2_bound_value
    : IsTrue (rational_leq (rational_sub three half) (interval_width wide)) =
  Proved
theorem a2_bound_by_law
    : IsTrue (rational_leq (rational_sub three half) (interval_width wide)) =
  interval_width_bounds_sub wide half three Proved Proved
theorem a2_tight_reverse
    : IsTrue (rational_leq (interval_width wide) (rational_sub three half)) =
  Proved
theorem wide_width_is_five_halves : rational_equiv (interval_width wide) five_halves = Proved

theorem narrow_contains_one : IsTrue (interval_contains narrow one) = Proved
theorem narrow_contains_two : IsTrue (interval_contains narrow two) = Proved
theorem narrow_width_is_one : rational_equiv (interval_width narrow) one = Proved
theorem narrow_bound_value
    : IsTrue (rational_leq (rational_sub two one) (interval_width narrow)) =
  Proved
theorem narrow_bound_by_law
    : IsTrue (rational_leq (rational_sub two one) (interval_width narrow)) =
  interval_width_bounds_sub narrow one two Proved Proved
theorem five_minus_one_is_four : rational_equiv (rational_sub five one) four = Proved
"#;

const FALSE_Y_PROOF: &str = r#"theorem false_y_containment_claim
    : IsTrue (rational_leq (rational_sub five one) (interval_width narrow)) =
  interval_width_bounds_sub narrow one five Proved Proved
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §18, Architect AC-0 law.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after five provider-first loads, exact trusted GlobalId sets
/// agree through Rational and then RationalInterval; the new law is public,
/// owner-local and Transparent. Its separately imported open application
/// inhabits the stated ordered-premise type and its body contains the owner
/// Const. CLAIMED: arbitrary interval clients can reuse a checked law at no
/// new local trust. THE GAP: imported providers retain inherited trust, and
/// closed arithmetic is separately exercised below.
#[test]
fn provider_first_exact_trust_and_open_owned_law() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational provider proof bodies check");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        after_rational, before,
        "Rational adds no trusted identities"
    );
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval proof bodies check");
    let after_interval = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        after_interval, before,
        "width bound adds no trusted identities"
    );
    let qualified = format!("{INTERVAL}.{LAW}");
    let id = *env
        .globals
        .get(&qualified)
        .expect("new law has checked identity");
    assert!(owned.contains(&id), "RationalInterval owns its new law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "new law has a checked proof body"
    );
    env.elaborate_file(&checked_is_true::selected_source_with_import(OPEN_CLIENT))
        .expect("explicitly aliased IsTrue open application checks");
    let (_, body) = env
        .env
        .transparent_body(env.globals["width_bounds_open"])
        .expect("open client has a checked body");
    assert!(
        body_contains_const(&body, id),
        "open client uses checked owner Const"
    );
}

/// SOURCE: spec/50-stdlib/68-numerical.md §18, AC-2 accepted rows and control.
/// PROMISE CLASS: durable invariant with specified closed-value controls.
/// MEASURED: the accepted block checks [1/2,3] with interior x=1,y=5/2
/// and tight endpoints x=1/2,y=3, both by closed Proved and the law. It
/// also accepts [1,2], x=1, y=2 with both witnesses and checks the closed
/// bound. The false-y case keeps the interval and x fixed and changes y to 5.
/// CLAIMED: both ordered-premise applications use a non-vacuous bound and
/// the endpoint bound cannot be sharpened on this interval.
/// THE GAP: these finite examples cannot establish the open law's generality.
#[test]
fn interior_tight_and_same_interval_control_accept_whole() {
    let mut env = loaded_interval();
    env.elaborate_file(&checked_is_true::selected_source_with_import(ACCEPTED))
        .expect("whole accepted block with explicit IsTrue alias checks");
}

/// SOURCE: spec/50-stdlib/68-numerical.md §18, false-y witness case.
/// PROMISE CLASS: durable invariant with a false-premise refusal discriminator.
/// MEASURED: the accepted prefix checks in a fresh environment. The appended
/// theorem supplies `Proved` in y's containment slot for y=5 on [1,2], where
/// the required proposition is false. Kernel TypeMismatch is inside the new
/// theorem and its span encloses the second `Proved` argument.
/// CLAIMED: a false y-containment proof cannot justify the width bound.
/// THE GAP: this false-proof case differs from the seed's separate
/// omitted-argument arity case and does not establish the open law.
#[test]
fn false_y_containment_proof_refuses_after_fresh_accepted_prefix() {
    let mut env = loaded_interval();
    env.elaborate_file(&checked_is_true::selected_source_with_import(ACCEPTED))
        .expect("fresh whole accepted prefix checks before negative");
    let negative = checked_is_true::selected_source(FALSE_Y_PROOF);
    let declarations = parse_decls(&negative).expect("negative parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = declarations.as_slice()
    else {
        panic!("the appended row must contain only its own theorem");
    };
    assert_eq!(name, "false_y_containment_claim");
    let span = match env
        .elaborate_file(&negative)
        .expect_err("false y-containment proof refuses")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } => span,
        other => panic!("false y proof must reject at its own kernel TypeMismatch: {other:?}"),
    };
    assert!(
        claim.start <= span.start && span.start < span.end && span.end <= claim.end,
        "the rejection must start inside its appended theorem: {span:?} vs {claim:?}"
    );
    assert!(
        negative.is_char_boundary(span.start) && negative.is_char_boundary(span.end),
        "diagnostic span must use UTF-8 byte boundaries"
    );
    let terminal = negative
        .rfind("Proved")
        .expect("second y-containment proof");
    assert!(
        span.start <= terminal && terminal + "Proved".len() <= span.end,
        "the failing span must enclose the false y-containment proof: {span:?}"
    );
}
