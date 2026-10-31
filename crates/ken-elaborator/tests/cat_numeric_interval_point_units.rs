//! Point-unit inclusion: spec/50-stdlib/68-numerical.md §10.
//! Promise class: durable invariant. Open clients check all four left-unit
//! exports and derive four right-unit forms from the public composition laws.
//! Closed controls on one interval distinguish true one-unit inclusion from
//! false zero-unit inclusion using `Proved`, not non-discriminating `Refl`.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const NEW_LAWS: [&str; 4] = [
    "interval_add_zero_l_inside",
    "interval_add_zero_l_rev_inside",
    "interval_mul_one_l_inside",
    "interval_mul_one_l_rev_inside",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§10 interval package root-loads with checked laws");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_zero, rational_one)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_add, interval_mul, interval_point, interval_inside,
   interval_inside_trans, interval_add_comm_inside, interval_mul_comm_inside,
   interval_add_zero_l_inside, interval_add_zero_l_rev_inside,
   interval_mul_one_l_inside, interval_mul_one_l_rev_inside)

theorem add_left (i : Interval)
    : IsTrue (interval_inside (interval_add (interval_point rational_zero) i) i) =
  interval_add_zero_l_inside i
theorem add_left_rev (i : Interval)
    : IsTrue (interval_inside i (interval_add (interval_point rational_zero) i)) =
  interval_add_zero_l_rev_inside i
theorem mul_left (i : Interval)
    : IsTrue (interval_inside (interval_mul (interval_point rational_one) i) i) =
  interval_mul_one_l_inside i
theorem mul_left_rev (i : Interval)
    : IsTrue (interval_inside i (interval_mul (interval_point rational_one) i)) =
  interval_mul_one_l_rev_inside i

theorem add_right (i : Interval)
    : IsTrue (interval_inside (interval_add i (interval_point rational_zero)) i) =
  interval_inside_trans
    (interval_add i (interval_point rational_zero))
    (interval_add (interval_point rational_zero) i)
    i
    (interval_add_comm_inside i (interval_point rational_zero))
    (interval_add_zero_l_inside i)
theorem add_right_rev (i : Interval)
    : IsTrue (interval_inside i (interval_add i (interval_point rational_zero))) =
  interval_inside_trans
    i
    (interval_add (interval_point rational_zero) i)
    (interval_add i (interval_point rational_zero))
    (interval_add_zero_l_rev_inside i)
    (interval_add_comm_inside (interval_point rational_zero) i)
theorem mul_right (i : Interval)
    : IsTrue (interval_inside (interval_mul i (interval_point rational_one)) i) =
  interval_inside_trans
    (interval_mul i (interval_point rational_one))
    (interval_mul (interval_point rational_one) i)
    i
    (interval_mul_comm_inside i (interval_point rational_one))
    (interval_mul_one_l_inside i)
theorem mul_right_rev (i : Interval)
    : IsTrue (interval_inside i (interval_mul i (interval_point rational_one))) =
  interval_inside_trans
    i
    (interval_mul (interval_point rational_one) i)
    (interval_mul i (interval_point rational_one))
    (interval_mul_one_l_rev_inside i)
    (interval_mul_comm_inside (interval_point rational_one) i)
"#;

/// MEASURED: all four RationalInterval-owned transparent exports type-check
/// under open client `i`, their four right forms compose using existing public
/// laws, no local trusted GlobalId is added, and old helpers remain private.
/// CLAIMED: the §10 one-argument inclusions are generic endpoint-value laws,
/// not new structural interval equalities or new trust. THE GAP: inherited
/// provider assumptions persist; the closed controls below cannot prove the
/// generic statements in place of their kernel-checked bodies.
#[test]
fn generic_point_unit_laws_compose_right_forms_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("four generic §10 laws root-load against checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§10 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§10 adds no trusted GlobalId identities");
    for name in NEW_LAWS {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent open client checks four left laws and public-only right forms");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match interval_env().elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} stays private: {other:?}"),
        }
    }
}

const CLOSED_ACCEPTED: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_zero, rational_one)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_mul, interval_point, interval_inside,
   interval_add_zero_l_inside, interval_add_zero_l_rev_inside,
   interval_mul_one_l_inside, interval_mul_one_l_rev_inside)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const i : Interval = MkInterval half one
const inverted : Interval = MkInterval one half

theorem fixture_order : Equal Interval i (MkInterval half one) = Refl
theorem fixture_inverted : Equal Interval inverted (MkInterval one half) = Refl

theorem add_left_closed : IsTrue (interval_inside (interval_add (interval_point rational_zero) i) i) =
  interval_add_zero_l_inside i
theorem add_left_rev_closed : IsTrue (interval_inside i (interval_add (interval_point rational_zero) i)) =
  interval_add_zero_l_rev_inside i
theorem mul_left_closed : IsTrue (interval_inside (interval_mul (interval_point rational_one) i) i) =
  interval_mul_one_l_inside i
theorem mul_left_rev_closed : IsTrue (interval_inside i (interval_mul (interval_point rational_one) i)) =
  interval_mul_one_l_rev_inside i
theorem inverted_add : IsTrue (interval_inside (interval_add (interval_point rational_zero) inverted) inverted) =
  interval_add_zero_l_inside inverted
theorem inverted_mul_rev : IsTrue (interval_inside inverted (interval_mul (interval_point rational_one) inverted)) =
  interval_mul_one_l_rev_inside inverted

theorem true_one_unit : IsTrue (interval_inside (interval_mul (interval_point rational_one) i) i) = Proved
theorem false_zero_value : Equal Bool (interval_inside (interval_mul (interval_point rational_zero) i) i) False = Proved
"#;

const FALSE_ZERO_UNIT: &str = r#"
theorem false_zero_unit : IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) i) = Proved
"#;

/// MEASURED: the checked [1/2,1] fixture accepts one-point product inclusion
/// with `Proved`, separately observes the zero-point product as `False`, and
/// refuses the otherwise identical zero-point inclusion at its own theorem
/// with `KernelRejected TypeMismatch`; inverted endpoints accept two laws.
/// CLAIMED: point one, not point zero, is the multiplicative inclusion unit
/// for this interval. THE GAP: one finite fixture is a discriminator, not a
/// general proof or an execution of the later CV-owned seed.
#[test]
fn one_point_proved_accepts_and_zero_point_proved_refuses_on_same_interval() {
    let mut accepted = interval_env();
    accepted
        .elaborate_file(CLOSED_ACCEPTED)
        .expect("§10 accepted one-unit, false-value and inverted controls check");

    let parsed = parse_decls(FALSE_ZERO_UNIT).expect("isolated false claim parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated false claim must contain one theorem");
    };
    assert_eq!(name, "false_zero_unit");
    assert!(params.is_empty(), "wrong-unit claim is closed");

    let mut rejecting = interval_env();
    rejecting
        .elaborate_file(CLOSED_ACCEPTED)
        .expect("same accepted fixture and one-point control precede refusal");
    match rejecting.elaborate_file(FALSE_ZERO_UNIT) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false zero-unit inclusion must reject at its own claim: {other:?}"),
    }
}
