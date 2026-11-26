//! Source: `spec/50-stdlib/68-numerical.md` §23 and Architect AC-0.
//! These package-local clients do not execute the CV-owned conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";

fn load(env: &mut ElabEnv, module: &str) -> Vec<GlobalId> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    env.elaborate_module_from_roots(&[root], module)
        .unwrap_or_else(|error| panic!("{module} roots-loads: {error:?}"))
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
        "Data.Numeric.Rational",
    ] {
        load(&mut env, provider);
    }
    env
}

fn contains_const(body: &Term, id: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id: found, .. } if *found == id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

/// Source: §23's two inclusion laws and AC-T. Promise class: durable invariant.
/// MEASURED: fresh provider loads followed by RationalInterval leave the exact
/// trusted ID set unchanged. Open clients, both membership derivations and
/// closed applications cite their exact owned Transparent laws. The closed
/// interior conclusion also checks without using either law.
/// CLAIMED: the ordered split hull is inside the whole, while the whole is
/// inside the split hull unconditionally, with no new package trust.
/// THE GAP: the trust check is provider-relative; the closed examples do not
/// pin generic direction or premises, which the open clients exercise.
/// Compatible: proof refactors or new checked laws. Breaking: added trust,
/// wrong ownership, a reversed sound direction or extra complete premises.
#[test]
fn exact_trust_and_owned_open_split_hull() {
    let mut env = provider_env();
    let trusted = |env: &ElabEnv| env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let before = trusted(&env);
    let owned = load(&mut env, INTERVAL);
    let after = trusted(&env);
    eprintln!(
        "split-hull trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "RationalInterval adds no trusted GlobalIds");

    let clients = env
        .elaborate_file(&format!("{PREFIX}{INTERIOR_CONCLUSION}"))
        .expect("open and closed split-hull clients check");
    for (export, names) in [
        (
            "interval_split_hull_inside",
            &[
                "split_hull_inside_open",
                "split_lower_member_in_whole",
                "split_upper_member_in_whole",
                "split_hull_inside_interior",
            ][..],
        ),
        (
            "interval_inside_split_hull",
            &["inside_split_hull_open", "inside_split_hull_exterior"][..],
        ),
    ] {
        let id = env.globals[&format!("{INTERVAL}.{export}")];
        assert!(owned.contains(&id), "{INTERVAL} owns {export}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{export} has a checked body"
        );
        for name in names {
            let client = env.globals[*name];
            assert!(clients.contains(&client), "{name} is a fresh client");
            let (_, body) = env
                .env
                .transparent_body(client)
                .expect("checked client body");
            assert!(contains_const(&body, id), "{name} cites the exact {export}");
        }
    }
}

fn refuses_at_appended_theorem(suffix: &str, label: &str) {
    let source = format!("{PREFIX}{suffix}");
    let decls = parse_decls(&source).expect("refusal fixture parses");
    let Some(SourceDecl::TheoremDecl {
        name,
        span: expected,
        ..
    }) = decls.last()
    else {
        panic!("{label}: expected an appended theorem");
    };
    assert_eq!(name, label);

    let mut env = provider_env();
    load(&mut env, INTERVAL);
    match env.elaborate_file(&source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            assert_eq!(span, *expected, "refusal must reach {label}");
            eprintln!("{label}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{label} must refuse at its own declaration: {other:?}"),
    }
}

/// Source: §23's sound law and AC-0's same-form premise pair.
/// Promise class: durable invariant. MEASURED: the `m = 2` application
/// refuses at its appended declaration; the `m = 1/2` form in the prefix
/// checks. CLAIMED: `2 ≤ 1` cannot supply the upper premise.
/// THE GAP: this tests closed premise evidence, not the generic law.
/// Compatible: equivalent Rational spellings. Breaking: accepting false
/// evidence or refusing at an unrelated declaration.
#[test]
fn exterior_point_refuses_at_its_premise_claim() {
    refuses_at_appended_theorem(REFUSED_PREMISE, "split_hull_inside_exterior");
}

/// Source: §23's sound conclusion and AC-0's independent conclusion pair.
/// Promise class: durable invariant. MEASURED: direct `Proved` checks the
/// interior inclusion, but the same claim at `m = 2` refuses at its own
/// declaration. CLAIMED: `[0, 2]` is not inside `[0, 1]`.
/// THE GAP: this exercises closed endpoints without using the split law.
/// Compatible: equivalent endpoint values or proof refactors. Breaking:
/// accepting the false inclusion, even without a law application.
#[test]
fn exterior_hull_refuses_at_its_inclusion_claim() {
    refuses_at_appended_theorem(REFUSED_CONCLUSION, "split_hull_exterior_conclusion");
}

const PREFIX: &str = r#"import Core.Classes.LawfulClasses (IsTrue)

import Data.Numeric.Rational (Rational, MkRational, rational_leq)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_contains,
    interval_hull,
    interval_inside,
    interval_inside_sound,
    interval_inside_trans,
    interval_inside_hull_left,
    interval_inside_hull_right,
    interval_split_hull_inside,
    interval_inside_split_hull)

const zero : Rational = MkRational Zero Zero

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const one : Rational = MkRational (Suc Zero) Zero

const two : Rational = MkRational (Suc (Suc Zero)) Zero

theorem split_hull_inside_open
      (lo : Rational) (hi : Rational) (m : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue
        (interval_inside (interval_hull (MkInterval lo m) (MkInterval m hi)) (MkInterval lo hi)) =
  λlower. λupper. interval_split_hull_inside lo hi m lower upper

theorem inside_split_hull_open
      (lo : Rational) (hi : Rational) (m : Rational)
    : IsTrue
      (interval_inside (MkInterval lo hi) (interval_hull (MkInterval lo m) (MkInterval m hi))) =
  interval_inside_split_hull lo hi m

theorem split_lower_member_in_whole
      (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue (interval_contains (MkInterval lo m) x)
      → IsTrue (interval_contains (MkInterval lo hi) x) =
  λlower.
    λupper.
      λmember.
        interval_inside_sound
          (MkInterval lo m)
          (MkInterval lo hi)
          x
          (interval_inside_trans
            (MkInterval lo m)
            (interval_hull (MkInterval lo m) (MkInterval m hi))
            (MkInterval lo hi)
            (interval_inside_hull_left (MkInterval lo m) (MkInterval m hi))
            (interval_split_hull_inside lo hi m lower upper))
          member

theorem split_upper_member_in_whole
      (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : IsTrue (rational_leq lo m)
      → IsTrue (rational_leq m hi)
      → IsTrue (interval_contains (MkInterval m hi) x)
      → IsTrue (interval_contains (MkInterval lo hi) x) =
  λlower.
    λupper.
      λmember.
        interval_inside_sound
          (MkInterval m hi)
          (MkInterval lo hi)
          x
          (interval_inside_trans
            (MkInterval m hi)
            (interval_hull (MkInterval lo m) (MkInterval m hi))
            (MkInterval lo hi)
            (interval_inside_hull_right (MkInterval lo m) (MkInterval m hi))
            (interval_split_hull_inside lo hi m lower upper))
          member

theorem split_hull_inside_interior
    : IsTrue
      (interval_inside (interval_hull (MkInterval zero half) (MkInterval half one)) (MkInterval zero one)) =
  interval_split_hull_inside zero one half Proved Proved

theorem inside_split_hull_exterior
    : IsTrue
      (interval_inside (MkInterval zero one) (interval_hull (MkInterval zero two) (MkInterval two one))) =
  interval_inside_split_hull zero one two
"#;

const INTERIOR_CONCLUSION: &str = r#"
theorem split_hull_interior_conclusion
    : IsTrue
      (interval_inside (interval_hull (MkInterval zero half) (MkInterval half one)) (MkInterval zero one)) =
  Proved
"#;

const REFUSED_PREMISE: &str = r#"
theorem split_hull_inside_exterior
    : IsTrue
      (interval_inside (interval_hull (MkInterval zero two) (MkInterval two one)) (MkInterval zero one)) =
  interval_split_hull_inside zero one two Proved Proved
"#;

const REFUSED_CONCLUSION: &str = r#"
theorem split_hull_exterior_conclusion
    : IsTrue
      (interval_inside (interval_hull (MkInterval zero two) (MkInterval two one)) (MkInterval zero one)) =
  Proved
"#;
