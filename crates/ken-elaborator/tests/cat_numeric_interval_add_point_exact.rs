//! Numerical intervals §24: shifting a point and both bounds by one value
//! preserves the Boolean membership decision in both directions.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";
const LAW: &str = "interval_add_point_exact";

fn loaded_interval() -> (ElabEnv, GlobalId) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let roots = &[root];
    let mut env = ElabEnv::new().expect("fresh compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(
        !before.is_empty(),
        "the inherited provider baseline is nonempty"
    );
    env.elaborate_module_from_roots(roots, RATIONAL)
        .expect("Rational provider checks");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(roots, INTERVAL)
        .expect("RationalInterval roots-loads with checked bodies");
    let after_interval = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "point-shift trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_rational.len(),
        after_interval.len()
    );
    assert_eq!(after_rational, before, "Rational adds no trust identities");
    assert_eq!(
        after_interval, after_rational,
        "RationalInterval adds no trust identities"
    );
    let law = env.globals[&format!("{INTERVAL}.{LAW}")];
    assert!(owned.contains(&law), "RationalInterval owns the law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    (env, law)
}

fn cites(body: &Term, law: GlobalId) -> bool {
    matches!(body, Term::Const { id, .. } if *id == law)
        || body.children().into_iter().any(|child| cites(child, law))
}

const ACCEPTED: &str = r#"
import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue)
import Core.Logic.Transport (sym, trans)
import Data.Numeric.Rational (Rational, MkRational, rational_add, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_add, interval_point,
   interval_add_encloses, interval_point_contains, interval_add_point_exact)

theorem add_point_exact_open (i : Interval) (t : Rational) (x : Rational)
    : Equal Bool
        (interval_contains (interval_add i (interval_point t)) (rational_add x t))
        (interval_contains i x) =
  interval_add_point_exact i t x

const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const base : Interval = MkInterval one two
const shifted : Interval = interval_add base (interval_point one)

theorem shifted_member_unshifts (x : Rational)
    : CatalogIsTrue (interval_contains shifted (rational_add x one))
      → CatalogIsTrue (interval_contains base x) =
  λmember.
    trans
      Bool
      (interval_contains base x)
      (interval_contains shifted (rational_add x one))
      True
      (sym
        Bool
        (interval_contains shifted (rational_add x one))
        (interval_contains base x)
        (interval_add_point_exact base one x))
      member

theorem shifted_excludes_one
    : Equal Bool (interval_contains shifted (rational_add rational_zero one)) False =
  trans Bool (interval_contains shifted (rational_add rational_zero one))
    (interval_contains base rational_zero) False
    (interval_add_point_exact base one rational_zero) Proved

theorem shifted_excludes_one_by_computation
    : Equal Bool (interval_contains shifted (rational_add rational_zero one)) False =
  Proved
"#;

const ENCLOSURE_ONLY: &str = r#"theorem unshift_by_enclosure (x : Rational)
    : CatalogIsTrue (interval_contains shifted (rational_add x one))
      → CatalogIsTrue (interval_contains base x) =
  λmember. interval_add_encloses base (interval_point one) x one member (interval_point_contains one)
"#;

const DROPPED_SHIFT_CLOSED: &str = r#"theorem shifted_contains_one
    : CatalogIsTrue (interval_contains shifted one) =
  trans Bool (interval_contains shifted one) (interval_contains base one) True
    (interval_add_point_exact base one one) Proved
"#;

const DROPPED_SHIFT_OPEN: &str = r#"theorem shifted_contains_unshifted (x : Rational)
    : CatalogIsTrue (interval_contains base x) → CatalogIsTrue (interval_contains shifted x) =
  λmember.
    trans Bool (interval_contains shifted x) (interval_contains base x) True
      (interval_add_point_exact base one x) member
"#;

/// SOURCE: numerical intervals §24 and the seed's open converse witness.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact provider trust sets agree through both packages. The
/// owned Transparent proof is cited by checked generic and converse clients.
/// CLAIMED: clients can use the generic law to unshift members without new
/// trust. The closed exclusion also checks without citing the law.
/// THE GAP: the closed row does not distinguish the converse; inherited
/// provider trust remains. This package test is not a conformance-runner run.
#[test]
fn exact_trust_and_owned_open_point_shift() {
    let (mut env, law) = loaded_interval();
    let declared = env
        .elaborate_file(ACCEPTED)
        .expect("generic and seed clients check");
    for (name, by_law) in [
        ("add_point_exact_open", true),
        ("shifted_member_unshifts", true),
        ("shifted_excludes_one", true),
        ("shifted_excludes_one_by_computation", false),
    ] {
        let id = env.globals[name];
        assert!(declared.contains(&id), "the client declares {name}");
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
            panic!("{name} must have a checked body");
        };
        assert_eq!(cites(body, law), by_law, "{name}: checked law citation");
    }
}

fn assert_own_type_mismatch(env: &mut ElabEnv, source: &str, name: &str) {
    let decls = parse_decls(source).expect("isolated claim parses");
    let [SourceDecl::TheoremDecl {
        name: actual,
        span: claim,
        ..
    }] = decls.as_slice()
    else {
        panic!("expected one appended theorem");
    };
    assert_eq!(actual, name);
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
            eprintln!("{name}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{name} must kernel-refuse at its own theorem: {other:?}"),
    }
}

/// SOURCE: numerical intervals §24, the two dropped-shift rows, and the
/// enclosure-only control from Architect AC-0.
/// PROMISE CLASS: durable invariant; [1, 2] and shift 1 are fixture values.
/// MEASURED: fresh accepted prefixes; both dropped-shift clients and the
/// open enclosure-only proof refuse with TypeMismatch in their own theorem.
/// CLAIMED: the law must be used at x + t, and enclosure alone does not type
/// check as this open converse.
/// THE GAP: the enclosure probe rules out one proof term, not all clients;
/// the diagnostics are declaration-level, not a pin on internal proof steps.
#[test]
fn dropped_shifts_and_enclosure_only_refuse_in_the_own_theorem() {
    for (name, source) in [
        ("shifted_contains_one", DROPPED_SHIFT_CLOSED),
        ("shifted_contains_unshifted", DROPPED_SHIFT_OPEN),
        ("unshift_by_enclosure", ENCLOSURE_ONLY),
    ] {
        let (mut env, _) = loaded_interval();
        env.elaborate_file(ACCEPTED)
            .expect("fresh full accepted prefix checks");
        assert_own_type_mismatch(&mut env, source, name);
    }
}

/// SOURCE: numerical intervals §24, private AC-0 cancellation chain.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each selective import from the checked package refuses with
/// UnboundName for the requested helper.
/// CLAIMED: the six proof helpers are not client API.
/// THE GAP: this checks the six named import attempts; the exact export
/// roster separately detects any other public addition.
#[test]
fn point_shift_proof_helpers_are_private() {
    let (mut env, _) = loaded_interval();
    for helper in [
        "rational_add_leq_shift",
        "rational_add_leq_cancel_r",
        "rational_add_sub_cancel",
        "rational_leq_add_left",
        "rational_zero_leq",
        "bool_eq_from_iff",
    ] {
        let source = format!("import {INTERVAL} ({helper})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, span })
                if name == format!("{INTERVAL}.{helper}")
                    && span.start <= "import ".len()
                    && span.end >= source.len() - 1 => {}
            other => panic!("private helper {helper} must refuse its import: {other:?}"),
        }
    }
}
