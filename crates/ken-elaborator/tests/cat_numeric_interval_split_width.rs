//! Source: `spec/50-stdlib/68-numerical.md` §22 and Architect AC-0.
//! These package clients are separate from the CV-owned conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";

const IMPORTS: &str = r#"
import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue)
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_equiv, rational_leq, rational_of_nat)
import Algorithm.Numeric.RationalInterval (MkInterval, interval_width, interval_split_width)
"#;

const OPEN: &str = r#"
theorem split_width_open_swap (lo : Rational) (hi : Rational) (m : Rational)
    : CatalogIsTrue (rational_leq lo m)
      → CatalogIsTrue (rational_leq m hi)
      → rational_equiv
        (interval_width (MkInterval lo hi))
        (rational_add (interval_width (MkInterval lo m)) (interval_width (MkInterval m hi))) =
  λlower.
    λupper.
      (proof sym for rational_equiv)
        (rational_add (interval_width (MkInterval lo m)) (interval_width (MkInterval m hi)))
        (interval_width (MkInterval lo hi))
        (interval_split_width lo hi m lower upper)
"#;

const VALUES: &str = r#"
const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const five : Rational = MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) Zero
const seven : Rational = MkRational (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))) Zero
"#;

const FOUR: &str = "(Suc (Suc (Suc (Suc Zero))))";
const THREE: &str = "(Suc (Suc (Suc Zero)))";

fn split_row(name: &str, total: &str) -> String {
    format!(
        r#"theorem {name}
    : rational_equiv
        (rational_add (interval_width (MkInterval one two)) (interval_width (MkInterval two five)))
        (rational_of_nat {total}) =
  (proof trans for rational_equiv)
    (rational_add (interval_width (MkInterval one two)) (interval_width (MkInterval two five)))
    (interval_width (MkInterval one five))
    (rational_of_nat {total})
    (interval_split_width one five two Proved Proved)
    Proved"#
    )
}

const ABOVE_HI: &str = r#"theorem split_width_above_hi
    : rational_equiv
        (rational_add (interval_width (MkInterval one seven)) (interval_width (MkInterval seven five)))
        (interval_width (MkInterval one five)) =
  interval_split_width one five seven Proved Proved"#;

const BELOW_LO: &str = r#"theorem split_width_below_lo
    : rational_equiv
        (rational_add (interval_width (MkInterval one zero)) (interval_width (MkInterval zero five)))
        (interval_width (MkInterval one five)) =
  interval_split_width one five zero Proved Proved"#;

const ABOVE_HI_VALUES: &str = r#"
theorem exterior_sum_is_six
    : rational_equiv
        (rational_add (interval_width (MkInterval one seven)) (interval_width (MkInterval seven five)))
        (rational_of_nat (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))) =
  Proved

theorem original_width_is_four
    : rational_equiv (interval_width (MkInterval one five))
        (rational_of_nat (Suc (Suc (Suc (Suc Zero))))) =
  Proved
"#;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn provider_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("fresh compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    env
}

fn body_mentions(body: &Term, id: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id: found, .. } if *found == id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn accepted_prefix() -> ElabEnv {
    let mut env = provider_env();
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational roots-loads");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval roots-loads");
    env.elaborate_file(IMPORTS)
        .expect("selective imports check");
    env.elaborate_file(VALUES).expect("closed values check");
    env.elaborate_file(&split_row("split_width_four", FOUR))
        .expect("four checks via the public split law and transitivity");
    let (_, body) = env
        .env
        .transparent_body(env.globals["split_width_four"])
        .expect("checked closed client body");
    let law = env.globals[&format!("{INTERVAL}.interval_split_width")];
    assert!(
        body_mentions(&body, law),
        "four row cites the exact law Const"
    );
    env
}

fn refuses_at_own_proof(env: &mut ElabEnv, source: &str, label: &str) {
    let declarations = parse_decls(source).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl { span: claim, .. }] = declarations.as_slice() else {
        panic!("{label}: expected one theorem");
    };
    let proof = source.find("Proved").expect("explicit proof input");
    let last_proof = source.rfind("Proved").unwrap() + "Proved".len();
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start
            && span.start <= proof
            && span.end >= last_proof
            && span.end <= claim.end =>
        {
            eprintln!("{label}: kernel TypeMismatch at {span:?}");
        }
        other => panic!("{label} must refuse at its own proof: {other:?}"),
    }
}

/// Source: §22's generic law and AC-T. Promise class: durable invariant.
/// MEASURED: rooted provider loads preserve the exact trusted ID set;
/// selective import and open application refer to the owned Transparent Const.
/// CLAIMED: the public law checks generically without new provider/package trust.
/// THE GAP: the comparison is relative to the preloaded foundations, and it
/// does not fix their inherited trust, private proof staging or public API size.
/// New checked laws and proof refactors are compatible; extra trust, loss of
/// visibility or an incompatible type change is breaking.
#[test]
fn exact_trust_and_owned_open_split_width() {
    let mut env = provider_env();
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(!before.is_empty(), "inherited provider trust is nonempty");
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational roots-loads");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_rational, before, "Rational adds no trusted GlobalIds");
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "split-width trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "RationalInterval adds no trusted GlobalIds");

    let law = env.globals[&format!("{INTERVAL}.interval_split_width")];
    assert!(owned.contains(&law), "RationalInterval owns the theorem");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(IMPORTS)
        .expect("selective imports check");
    env.elaborate_file(OPEN)
        .expect("open split-width client checks");
    let (_, body) = env
        .env
        .transparent_body(env.globals["split_width_open_swap"])
        .expect("checked generic client body");
    assert!(
        body_mentions(&body, law),
        "client cites the exact law Const"
    );
}

/// Source: §22's four-versus-three control. Promise class: durable invariant.
/// MEASURED: the law-based four row loads; on its accepted prefix the same
/// form claiming three produces a kernel mismatch at the isolated theorem.
/// CLAIMED: a split at two of [1, 5] has total width four, not three.
/// THE GAP: the refusal is a closed-value soundness control, not evidence
/// for the generic law. Equivalent Rational spellings or proof refactors
/// are compatible; a wrong total or a law that cannot supply `trans` breaks.
#[test]
fn law_based_four_checks_and_false_total_refuses() {
    let mut env = accepted_prefix();
    refuses_at_own_proof(
        &mut env,
        &split_row("split_width_test_total", THREE),
        "wrong total",
    );
}

/// Source: §22's ordered split and AC-0's exterior example.
/// Promise class: durable invariant.
/// MEASURED: six and four check by closed computation; the law application
/// with only the upper comparison false refuses at its own declaration.
/// CLAIMED: an exterior split above hi is not a valid input to the law.
/// THE GAP: the whole-declaration span does not select an argument token;
/// the closed fixture fixes the lower comparison as true. Equivalent values
/// and proof refactors are compatible; accepting this false premise breaks.
#[test]
fn split_above_hi_refuses_at_its_own_application() {
    let mut env = accepted_prefix();
    env.elaborate_file(ABOVE_HI_VALUES)
        .expect("exterior halves sum to six and whole width is four");
    env.elaborate_file(
        "theorem lower_comparison_holds : CatalogIsTrue (rational_leq one seven) = Proved",
    )
    .expect("lower comparison remains true");
    refuses_at_own_proof(&mut env, ABOVE_HI, "false upper premise");
}

/// Source: §22's ordered split. Promise class: durable invariant.
/// MEASURED: the upper comparison checks, but applying the law with the
/// false lower comparison refuses at its own declaration on a fresh prefix.
/// CLAIMED: an exterior split below lo is not a valid input to the law.
/// THE GAP: this is a closed control, not a proof of the universal type.
/// Equivalent values and proof refactors are compatible; accepting the false
/// lower premise breaks.
#[test]
fn split_below_lo_refuses_at_its_own_application() {
    let mut env = accepted_prefix();
    env.elaborate_file(
        "theorem upper_comparison_holds : CatalogIsTrue (rational_leq zero five) = Proved",
    )
    .expect("upper comparison remains true");
    refuses_at_own_proof(&mut env, BELOW_LO, "false lower premise");
}
