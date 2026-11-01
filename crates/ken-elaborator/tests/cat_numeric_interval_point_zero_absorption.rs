//! Point-zero absorption: spec/50-stdlib/68-numerical.md §11 (pending enclave assembly).
//! Promise class: durable invariant. Open clients use both left-zero inclusions
//! and compose both right-zero forms from public laws; closed clients include
//! ordinary and inverted intervals and a same-product Proved discriminator.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const NEW_LAWS: [&str; 2] = [
    "interval_mul_zero_l_inside",
    "interval_mul_zero_l_rev_inside",
];
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-point-zero-absorption.md"
));

fn seed_source_containing(theorem: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§11 Ken seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§11 source fence parses")
            .iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem))
        {
            assert!(
                selected.replace(source).is_none(),
                "one source fence declares {theorem}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("§11 seed declares {theorem}"))
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§11 interval package root-loads with checked laws");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_mul, interval_point, interval_inside,
   interval_inside_trans, interval_mul_comm_inside,
   interval_mul_zero_l_inside, interval_mul_zero_l_rev_inside)

theorem zero_left (i : Interval)
    : IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) (interval_point rational_zero)) =
  interval_mul_zero_l_inside i
theorem zero_left_rev (i : Interval)
    : IsTrue (interval_inside (interval_point rational_zero) (interval_mul (interval_point rational_zero) i)) =
  interval_mul_zero_l_rev_inside i

theorem zero_right (i : Interval)
    : IsTrue (interval_inside (interval_mul i (interval_point rational_zero)) (interval_point rational_zero)) =
  interval_inside_trans
    (interval_mul i (interval_point rational_zero))
    (interval_mul (interval_point rational_zero) i)
    (interval_point rational_zero)
    (interval_mul_comm_inside i (interval_point rational_zero))
    (interval_mul_zero_l_inside i)
theorem zero_right_rev (i : Interval)
    : IsTrue (interval_inside (interval_point rational_zero) (interval_mul i (interval_point rational_zero))) =
  interval_inside_trans
    (interval_point rational_zero)
    (interval_mul (interval_point rational_zero) i)
    (interval_mul i (interval_point rational_zero))
    (interval_mul_zero_l_rev_inside i)
    (interval_mul_comm_inside (interval_point rational_zero) i)
"#;

/// MEASURED: both public names are owned Transparent declarations, a generic
/// independent client consumes both exact propositions and composes right-zero
/// using public exports only, while trusted GlobalId identities do not grow.
/// CLAIMED: the two point-zero endpoint-inclusion laws hold for arbitrary
/// intervals without added trust or a separately exported right-zero law.
/// THE GAP: inherited rational/logical provider assumptions persist; no finite
/// client establishes a generic law without its checked proof body.
#[test]
fn generic_point_zero_absorption_and_right_composition_add_no_trust() {
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
        .expect("§11 generic laws root-load against checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§11 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§11 adds no trusted GlobalId identities");
    for name in NEW_LAWS {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent client checks both exact left laws and both right compositions");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match interval_env().elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} stays private: {other:?}"),
        }
    }
}

/// MEASURED: the CV seed's complete accepted Ken source checks against the
/// current package before its separate false point-one claim is elaborated;
/// the latter fails with KernelRejected TypeMismatch inside its own theorem.
/// CLAIMED: the independently authored point-zero control accepts whereas
/// the same-product point-one target is refused on its own claim.
/// THE GAP: this is package-test elaboration of source fences, not execution
/// through the official conformance runner.
#[test]
fn cv_seed_prefix_checks_before_point_one_refuses_at_its_own_claim() {
    let accepted_source = seed_source_containing("point_zero_target_control");
    let refused_source = seed_source_containing("reject_zero_product_inside_point_one");
    let mut accepted = interval_env();
    accepted
        .elaborate_file(accepted_source)
        .expect("CV seed's complete accepted source block checks");

    let parsed = parse_decls(refused_source).expect("isolated CV seed refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("CV seed refusal is exactly one theorem");
    };
    assert_eq!(name, "reject_zero_product_inside_point_one");
    assert!(params.is_empty(), "false CV claim is closed");

    let mut rejecting = interval_env();
    rejecting
        .elaborate_file(accepted_source)
        .expect("accepted CV seed prefix checks before isolated refusal");
    match rejecting.elaborate_file(refused_source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("CV seed point-one claim must reject at its own theorem: {other:?}"),
    }
}

const CLOSED_ACCEPTED: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_zero, rational_one)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_mul, interval_point, interval_inside,
   interval_mul_zero_l_inside, interval_mul_zero_l_rev_inside)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const i : Interval = MkInterval half one
const inverted : Interval = MkInterval one half

theorem fixture_order : Equal Interval i (MkInterval half one) = Refl
theorem fixture_inverted : Equal Interval inverted (MkInterval one half) = Refl

theorem left_closed : IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) (interval_point rational_zero)) =
  interval_mul_zero_l_inside i
theorem left_rev_closed : IsTrue (interval_inside (interval_point rational_zero) (interval_mul (interval_point rational_zero) i)) =
  interval_mul_zero_l_rev_inside i
theorem inverted_left : IsTrue (interval_inside (interval_mul (interval_point rational_zero) inverted) (interval_point rational_zero)) =
  interval_mul_zero_l_inside inverted
theorem inverted_left_rev : IsTrue (interval_inside (interval_point rational_zero) (interval_mul (interval_point rational_zero) inverted)) =
  interval_mul_zero_l_rev_inside inverted

theorem true_zero_target : IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) (interval_point rational_zero)) = Proved
"#;

const FALSE_ONE_TARGET: &str = r#"
theorem false_one_target : IsTrue (interval_inside (interval_mul (interval_point rational_zero) i) (interval_point rational_one)) = Proved
"#;

/// MEASURED: both laws apply to [1/2,1] and inverted [1,1/2]; with the
/// identical product and claim form, Proved accepts point zero as target and
/// rejects point one with KernelRejected TypeMismatch at its own theorem.
/// CLAIMED: point zero absorbs left multiplication by endpoint inclusion;
/// point one is not an absorbing target on this non-degenerate interval.
/// THE GAP: closed cases discriminate the claimed boundary, not all generic
/// inputs and not official execution of the independently authored CV seed.
#[test]
fn zero_target_proved_accepts_and_one_target_refuses_on_same_product() {
    let mut accepted = interval_env();
    accepted
        .elaborate_file(CLOSED_ACCEPTED)
        .expect("§11 both laws, inverted controls and true Proved check");

    let parsed = parse_decls(FALSE_ONE_TARGET).expect("isolated false claim parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated false claim must contain one theorem");
    };
    assert_eq!(name, "false_one_target");
    assert!(params.is_empty(), "false claim is closed");

    let mut rejecting = interval_env();
    rejecting
        .elaborate_file(CLOSED_ACCEPTED)
        .expect("accepted same-product point-zero control precedes refusal");
    match rejecting.elaborate_file(FALSE_ONE_TARGET) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("point-one target must reject at its own claim: {other:?}"),
    }
}
