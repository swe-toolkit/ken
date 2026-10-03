//! Measurement-only source variants: f5/f6 originals did not survive.
//! Neither result is an exact-source pin or a law over the catalog Vector.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert, env::Context, term::Term, KernelError};

const F5_RECONSTRUCTED: &str =
    include_str!("fixtures/sibling_goal_refinement/f5_reconstructed.ken.md");
const F5_ASCRIBED_RECONSTRUCTED: &str =
    include_str!("fixtures/sibling_goal_refinement/f5_reconstructed_ascribed.ken.md");
const F6_RECONSTRUCTED: &str =
    include_str!("fixtures/sibling_goal_refinement/f6_reconstructed.ken.md");
const F6_NO_WITNESS_RECONSTRUCTED: &str =
    include_str!("fixtures/sibling_goal_refinement/f6_reconstructed_no_witness.ken.md");

// A checked kernel Equal goal with identical endpoints is convertible by
// the kernel's identity rule, independently of observational rewriting.
fn assert_reflexive_goal(env: &ElabEnv, goal: &Term) {
    let (ty, left, right) = match goal {
        Term::Eq(ty, left, right) => (&**ty, &**left, &**right),
        Term::App(head, right) => match &**head {
            Term::App(head, left) => match &**head {
                Term::App(head, ty)
                    if matches!(&**head, Term::Const { id, .. }
                        if Some(*id) == env.globals.get("Equal").copied()) =>
                {
                    (&**ty, &**left, &**right)
                }
                _ => panic!("expected a prelude Equal goal, found {goal:?}"),
            },
            _ => panic!("expected an applied Equal goal, found {goal:?}"),
        },
        _ => panic!("expected an equality goal, found {goal:?}"),
    };
    assert_eq!(left, right, "the measured goal has nonidentical endpoints");
    // The identity arm of kernel conversion precedes any context lookup, so
    // the empty diagnostic context is sufficient even for open Var operands.
    assert!(convert(&env.env, &Context::new(), ty, left, right));
}

fn source_verdict(source: &str) -> (Result<(), ElabError>, ElabEnv, bool) {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    let result = env.elaborate_ken_md_file(source).map(|_| ());
    let unchanged = env.env.trusted_base() == trusted;
    (result, env, unchanged)
}

#[test]
fn f5_reconstructed_goal_is_reflexive_even_when_bare_refl_loses_eq_origin() {
    // Promise: diagnostic measurement, not a durable refusal. The bare
    // source follows the described Fin goal through i→xs→ys; the ascribed
    // peer checks Refl at that source equality, exposing the kernel goal
    // at the later boundary if the un-ascribed route loses Eq provenance.
    let (bare, _, trusted) = source_verdict(F5_RECONSTRUCTED);
    assert!(trusted, "measurement must not add trusted declarations");
    match bare {
        Ok(()) => println!("f5 reconstructed bare: checks"),
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal" =>
        {
            println!("f5 reconstructed bare: Eq-origin loss at {span:?}");
        }
        Err(error) => panic!("f5 reconstructed: different first rejecting site: {error:?}"),
    }

    let (ascribed, env, trusted) = source_verdict(F5_ASCRIBED_RECONSTRUCTED);
    assert!(
        trusted,
        "ascribed measurement must not add trusted declarations"
    );
    match ascribed {
        Ok(()) => println!("f5 reconstructed ascribed: checks"),
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found },
            span,
        }) => {
            // The Refl ascription has already passed; this later kernel
            // mismatch reports the actual branch target and supplied type.
            assert_reflexive_goal(&env, &expected);
            assert_reflexive_goal(&env, &found);
            println!("f5 reconstructed: reflexive target; later mismatch at {span:?}");
        }
        Err(error) => panic!("f5 ascribed: different first rejecting site: {error:?}"),
    }
}

#[test]
fn f6_reconstructed_premise_consumer_has_reflexive_goal() {
    // This is a source-facing analogue of the premise-consumer seam in
    // sibling_goal_refinement_tests.rs, not the missing literal f6.
    // A let consumes the named equality premise after i→xs→ys. The
    // otherwise identical no-witness source checks, so this premise
    // consumer is causally reached. If its method is rejected later,
    // measure the kernel's expected goal rather than blaming Refl.
    let (control, _, trusted_control) = source_verdict(F6_NO_WITNESS_RECONSTRUCTED);
    assert!(trusted_control, "control must not add trusted declarations");
    control.expect("f6 reconstructed no-witness control must check");
    let (result, env, trusted) = source_verdict(F6_RECONSTRUCTED);
    assert!(trusted, "measurement must not add trusted declarations");
    match result {
        Ok(()) => println!("f6 reconstructed: checks"),
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found },
            span,
        }) => {
            assert_reflexive_goal(&env, &expected);
            assert_reflexive_goal(&env, &found);
            println!("f6 reconstructed: reflexive target; later mismatch at {span:?}");
        }
        Err(error) => panic!("f6 reconstructed: different first rejecting site: {error:?}"),
    }
}
