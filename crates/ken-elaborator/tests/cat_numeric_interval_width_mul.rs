//! Spec 68 §17: width of a product of ordered exact rational intervals.
//! The generic law is checked at open endpoints; closed product values and
//! fresh false rows distinguish its formula and both order hypotheses.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("width multiplication and its checked providers root-load");
    env
}

fn contains_const(term: &Term, needle: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == needle)
        || term
            .children()
            .iter()
            .any(|child| contains_const(child, needle))
}

/// SOURCE: spec 68 §17 and its visibility-only Rational prerequisite.
/// PROMISE CLASS: durable invariant.
/// MEASURED: independent provider-first loads keep the exact trusted
/// GlobalId set unchanged; Rational owns a Transparent right-add cancellation
/// and RationalInterval owns a Transparent product-width theorem. A selective
/// import resolves to the same kernel Const used by rational_sub::add_distrib,
/// and an independent open client applies its published right-cancel type.
/// CLAIMED: both canonical owners publish checked, reusable zero-local-trust
/// laws, and the promoted provider is the one the existing proof consumes.
/// THE GAP: imported providers retain inherited trust; the closed and generic
/// interval clients below check behavior beyond owner and visibility.
#[test]
fn provider_first_zero_trust_owned_transparent_and_cancel_identity() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} provider: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let rational_owned = env
        .elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational checked provider");
    assert_eq!(
        before,
        env.env.trusted_base().into_iter().collect(),
        "public Rational cancellation adds no trust"
    );
    let interval_owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval checked owner");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "product width trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "both catalog changes have zero local trust");
    for (module, name, owned) in [
        (RATIONAL, "rational_add_cancel_r", &rational_owned),
        (INTERVAL, "interval_width_mul", &interval_owned),
    ] {
        let qualified = format!("{module}.{name}");
        let id = *env
            .globals
            .get(&qualified)
            .unwrap_or_else(|| panic!("missing public {qualified}"));
        assert!(owned.contains(&id), "{qualified} is owned by {module}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} has a kernel-checked body"
        );
    }

    let cancel = env.globals["Data.Numeric.Rational.rational_add_cancel_r"];
    let distributive = env.globals["Data.Numeric.Rational.rational_sub::add_distrib"];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(distributive) else {
        panic!("existing checked subtraction distribution body");
    };
    assert!(
        contains_const(body, cancel),
        "imported right-cancel is the existing distribution proof's Const"
    );
    env.elaborate_file("import Data.Numeric.Rational (rational_add_cancel_r)")
        .expect("selective client import of the public provider");
    env.elaborate_file("import Data.Numeric.Rational (Rational, rational_add, rational_equiv)")
        .expect("selectively import the application argument names");
    env.elaborate_decl("theorem cancel_client (x : Rational) (y : Rational) (z : Rational) : rational_equiv (rational_add x z) (rational_add y z) → rational_equiv x y = rational_add_cancel_r x y z")
        .expect("independent open right-cancellation client");
    eprintln!("provider selective import {cancel:?} = existing proof Const {cancel:?} in {distributive:?}");
}

const PREFIX: &str = r#"
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational, rational_add, rational_mul, rational_equiv, rational_leq)
import Algorithm.Numeric.RationalInterval (Interval, MkInterval, interval_mul, interval_width, interval_width_mul)
const r1 : Rational = MkRational (Suc Zero) Zero
const r2 : Rational = MkRational (Suc (Suc Zero)) Zero
const r3 : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
"#;

const W_GEN: &str = "theorem open_width_mul (lo1 : Rational) (hi1 : Rational) (lo2 : Rational) (hi2 : Rational) : IsTrue (rational_leq lo1 hi1) → IsTrue (rational_leq lo2 hi2) → rational_equiv (interval_width (interval_mul (MkInterval lo1 hi1) (MkInterval lo2 hi2))) (rational_add (rational_mul hi1 (interval_width (MkInterval lo2 hi2))) (rational_mul lo2 (interval_width (MkInterval lo1 hi1)))) = interval_width_mul lo1 hi1 lo2 hi2";
const W_APPLY: &str = "theorem product_by_law : rational_equiv (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2))) (rational_add (rational_mul r2 (interval_width (MkInterval r1 r2))) (rational_mul r1 (interval_width (MkInterval r1 r2)))) = interval_width_mul r1 r2 r1 r2 Proved Proved";
const W_THREE: &str = "theorem product_width_three : rational_equiv (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2))) r3 = Proved";
const W_FORM: &str = "theorem product_by_value : rational_equiv (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2))) (rational_add (rational_mul r2 (interval_width (MkInterval r1 r2))) (rational_mul r1 (interval_width (MkInterval r1 r2)))) = Proved";
const X_PRODUCT: &str = "theorem product_width_is_widths_product : rational_equiv (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2))) (rational_mul (interval_width (MkInterval r1 r2)) (interval_width (MkInterval r1 r2))) = Proved";
const X_DROP_FIRST: &str = "theorem inverted_first_width : rational_equiv (interval_width (interval_mul (MkInterval r3 r1) (MkInterval r1 r2))) (rational_add (rational_mul r1 (interval_width (MkInterval r1 r2))) (rational_mul r1 (interval_width (MkInterval r3 r1)))) = Proved";
const X_DROP_SECOND: &str = "theorem inverted_second_width : rational_equiv (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r3 r1))) (rational_add (rational_mul r2 (interval_width (MkInterval r3 r1))) (rational_mul r3 (interval_width (MkInterval r1 r2)))) = Proved";

fn closed_prefix() -> ElabEnv {
    let mut env = loaded_interval();
    // Loading LawfulClasses does not bind IsTrue in a client; PREFIX imports
    // its checked identity for the separate in-memory source.
    env.elaborate_file(PREFIX)
        .expect("closed selective imports and one/two/three spellings");
    env
}

/// SOURCE: spec 68 §17's ordered product law and closed [1,2]·[1,2] seed.
/// PROMISE CLASSES: durable invariant for the open application, normative
/// compatibility vector for the stated [1,2] product value and formula.
/// MEASURED: W-GEN accepts at four open endpoints and two ordered premises;
/// W-APPLY applies the export at two checked True premises; independent
/// W-THREE and same-form W-FORM both accept by closed conversion to Proved.
/// CLAIMED: the public law has precisely the selected factor order, and the
/// checked width definition yields three, not two or a product-of-widths.
/// THE GAP: closed values do not establish generic validity; the checked open
/// theorem does. These local clients are not the official conformance seed.
#[test]
fn open_product_law_and_closed_ordered_width_three() {
    let mut env = closed_prefix();
    for (name, source) in [
        ("W-GEN", W_GEN),
        ("W-APPLY", W_APPLY),
        ("W-THREE", W_THREE),
        ("W-FORM", W_FORM),
    ] {
        env.elaborate_decl(source)
            .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
        eprintln!("{name} accepted");
    }
}

/// SOURCE: spec 68 §17, product-width discriminator and necessity of both
/// ordered-endpoint premises.
/// PROMISE CLASS: normative compatibility vector for the specified intervals.
/// MEASURED: each X row on a fresh accepted prefix with W-THREE or W-FORM
/// first checks its same-form positive control, then rejects specifically
/// as KernelRejected(TypeMismatch) at its own theorem including its Proved.
/// CLAIMED: width 3 is not width-product 1; inverted first [3,1] or inverted
/// second [3,1] makes the premise-free product formula false, independently.
/// THE GAP: these are closed witnesses, not the universal theorem; positive
/// open W-GEN and independent W-THREE/W-FORM above cover other boundaries.
#[test]
fn false_product_and_each_missing_order_premise_refuse_at_own_proved() {
    for (label, positive, false_claim) in [
        ("X-PRODUCT", W_THREE, X_PRODUCT),
        ("X-DROP-FIRST", W_FORM, X_DROP_FIRST),
        ("X-DROP-SECOND", W_FORM, X_DROP_SECOND),
    ] {
        let mut env = closed_prefix();
        env.elaborate_decl(positive)
            .unwrap_or_else(|error| panic!("{label} same-form positive: {error:?}"));
        let proof = false_claim
            .rfind("Proved")
            .expect("own explicit proof term");
        match env.elaborate_decl(false_claim) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start <= proof
                && span.end >= proof + "Proved".len()
                && span.end <= false_claim.len() =>
            {
                eprintln!(
                    "{label}: TypeMismatch own {span:?}, Proved {proof}..{}",
                    proof + "Proved".len()
                );
            }
            other => panic!("{label} must reject its own Proved: {other:?}"),
        }
    }
}
