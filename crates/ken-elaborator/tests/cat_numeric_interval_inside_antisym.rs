//! Mutual interval inclusion fixes both endpoint values: spec/50-stdlib/68-numerical.md §14.
//! Promise class: durable invariant. An open client checks both public types;
//! closed clients distinguish rational equivalence from structural equality.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser::parse_decls, Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const LAWS: [&str; 2] = ["interval_inside_antisym_lo", "interval_inside_antisym_hi"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("interval package root-loads");
    env
}

const GENERIC: &str = r#"
import Data.Numeric.Rational (Rational, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside, interval_inside_antisym_lo,
   interval_inside_antisym_hi)
theorem g_lo (a : Rational) (b : Rational) (c : Rational) (d : Rational)
    : IsTrue (interval_inside (MkInterval a b) (MkInterval c d))
      → IsTrue (interval_inside (MkInterval c d) (MkInterval a b))
      → rational_equiv a c =
  interval_inside_antisym_lo a b c d
theorem g_hi (a : Rational) (b : Rational) (c : Rational) (d : Rational)
    : IsTrue (interval_inside (MkInterval a b) (MkInterval c d))
      → IsTrue (interval_inside (MkInterval c d) (MkInterval a b))
      → rational_equiv b d =
  interval_inside_antisym_hi a b c d
"#;

/// MEASURED: both public laws belong to the interval package, are checked
/// Transparent declarations, selectively import, and have their full open
/// result types consumed; the exact trusted GlobalId sets agree around load.
/// CLAIMED: mutual inclusion fixes the two endpoint values with no new trust.
/// THE GAP: imported providers retain their inherited trusted identities;
/// closed cases do not substitute for the kernel-checked open proof bodies.
#[test]
fn checked_public_laws_have_generic_clients_without_added_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|e| panic!("{provider} root-loads: {e:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("both checked interval antisymmetry laws load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no new trusted GlobalId identity");
    for law in LAWS {
        let id = env.globals[&format!("{INTERVAL}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} has a checked transparent body"
        );
        env.elaborate_file(&format!("import {INTERVAL} ({law} as client_{law})"))
            .unwrap_or_else(|e| panic!("{law} selectively imports: {e:?}"));
    }
    env.elaborate_file(GENERIC)
        .expect("open client consumes both precise endpoint-value propositions");
}

// MkRational n k represents n/(Suc k). Both intervals have endpoint values
// [1/2, 2], but their endpoint spellings differ.
const ACCEPT_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside, interval_inside_antisym_lo,
   interval_inside_antisym_hi)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const two_quarters : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const four_halves : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) (Suc Zero)

theorem outward_control : Equal Bool
    (interval_inside (MkInterval half two) (MkInterval two_quarters four_halves)) True = Proved
theorem inward_control : Equal Bool
    (interval_inside (MkInterval two_quarters four_halves) (MkInterval half two)) True = Proved

theorem lo_equiv : rational_equiv half two_quarters =
  interval_inside_antisym_lo half two two_quarters four_halves Proved Proved

theorem hi_equiv : rational_equiv two four_halves =
  interval_inside_antisym_hi half two two_quarters four_halves Proved Proved
"#;

/// MEASURED: two independent closed comparisons and both endpoint-law
/// applications accept on distinct rational spellings of equal values.
/// CLAIMED: both endpoint results are value equivalence, not identity.
/// THE GAP: closed examples cannot establish the generic proposition.
#[test]
fn equal_value_endpoint_clients_accept_with_two_direction_controls() {
    interval_env()
        .elaborate_file(ACCEPT_CLIENT)
        .expect("both directions and both value-equivalence clients check");
}

const REFUSE_ONE_WAY_CONTROL: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside, interval_inside_antisym_lo)

const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero

theorem outward_control : Equal Bool
    (interval_inside (MkInterval zero one) (MkInterval zero two)) True = Proved
theorem reverse_false : Equal Bool
    (interval_inside (MkInterval zero two) (MkInterval zero one)) False = Proved
"#;

const REFUSE_ONE_WAY: &str = r#"
theorem lo_equiv_one_way : rational_equiv zero zero =
  interval_inside_antisym_lo zero one zero two Proved Proved
"#;

const REFUSE_EQUAL_FORM: &str = r#"
theorem lo_equal : Equal Rational half two_quarters =
  interval_inside_antisym_lo half two two_quarters four_halves Proved Proved
"#;

fn reject_own_theorem(env: &mut ElabEnv, source: &str) -> KernelError {
    let parsed = parse_decls(source).expect("negative client parses");
    let [SourceDecl::TheoremDecl { span: claim, .. }] = parsed.as_slice() else {
        panic!("negative client must be one attempted theorem");
    };
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: error @ KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => error,
        other => panic!("negative client must TypeMismatch at its own claim: {other:?}"),
    }
}

/// MEASURED: the positive and independently False reverse comparisons check,
/// and a one-way application fails with TypeMismatch inside its own theorem.
/// The same-form equal-value lo client accepts in the preceding test.
/// CLAIMED: the reverse inclusion is indispensable even when lower endpoints
/// already have equal values. THE GAP: the refused theorem is closed; the
/// open law's exact premises are consumed by the generic client above.
#[test]
fn dropped_reverse_inclusion_refuses_at_the_law_application() {
    interval_env()
        .elaborate_file(ACCEPT_CLIENT)
        .expect("same-form two-way lo application accepts");
    let mut env = interval_env();
    env.elaborate_file(REFUSE_ONE_WAY_CONTROL)
        .expect("one-way True and reverse False check independently");
    let error = reject_own_theorem(&mut env, REFUSE_ONE_WAY);
    assert!(matches!(error, KernelError::TypeMismatch { .. }));
}

/// MEASURED: the accepted client's lo application checks, but ascribing the
/// identical application to Equal Rational rejects with TypeMismatch.
/// CLAIMED: the exported conclusion is rational_equiv, not structural Equal.
/// THE GAP: the control does not claim equality is generally impossible;
/// it shows only this public law cannot provide it at these spellings.
#[test]
fn value_equivalence_does_not_ascribe_to_structural_equality() {
    let mut env = interval_env();
    env.elaborate_file(ACCEPT_CLIENT)
        .expect("same-input value-equivalence client checks first");
    let error = reject_own_theorem(&mut env, REFUSE_EQUAL_FORM);
    assert!(matches!(error, KernelError::TypeMismatch { .. }));
}
