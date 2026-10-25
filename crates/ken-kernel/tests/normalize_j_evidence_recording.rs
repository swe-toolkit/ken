//! Normalization retains J's checked Eq formation even when Eq reduces by type.
//! Contract: spec/10-kernel/15-identity.md §4; 16-observational.md §2.2.

use ken_kernel::env::Context;
use ken_kernel::{declare_postulate, infer, normalize, whnf, GlobalEnv, KernelError, Level, Term};

fn eq(ty: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(left), Box::new(right))
}

fn post(env: &mut GlobalEnv, name: &str, ty: Term) -> Term {
    Term::const_(
        declare_postulate(env, name.into(), vec![], ty).expect("fixture declaration"),
        vec![],
    )
}

struct Fixture {
    env: GlobalEnv,
    ctx: Context,
    carrier: Term,
    left: Term,
    right: Term,
    motive: Term,
    base: Term,
}

impl Fixture {
    fn new() -> Self {
        let mut env = GlobalEnv::new();
        let type0 = Term::Type(Level::zero());
        let carrier = Term::sigma(type0.clone(), type0);
        let left = post(&mut env, "left", carrier.clone());
        let right = post(&mut env, "right", carrier.clone());
        let mut ctx = Context::new();
        ctx.push(eq(carrier.clone(), left.clone(), right.clone()));
        // An Ω-valued motive leaves the checked, non-refl J neutral: the
        // type-equality witness builder handles Type-valued motives only.
        let motive = post(
            &mut env,
            "motive",
            Term::pi(
                carrier.clone(),
                Term::pi(
                    eq(carrier.clone(), left.clone(), Term::var(0)),
                    Term::Omega(Level::zero()),
                ),
            ),
        );
        let base = post(
            &mut env,
            "base",
            Term::app(
                Term::app(motive.clone(), left.clone()),
                Term::Refl(Box::new(left.clone())),
            ),
        );
        Self {
            env,
            ctx,
            carrier,
            left,
            right,
            motive,
            base,
        }
    }

    fn recorded(&self, right: Term) -> Term {
        eq(self.carrier.clone(), self.left.clone(), right)
    }

    fn j(&self, recorded: Term) -> Term {
        Term::J(
            Box::new(self.motive.clone()),
            Box::new(self.base.clone()),
            Box::new(Term::Ascript(Box::new(Term::var(0)), Box::new(recorded))),
        )
    }
}

// Promise class: durable invariant. MEASURED: the normal form of this typed,
// neutral J still carries its Eq recording and `infer` returns the same type.
// CLAIMED: checked J normal forms keep enough formation information to type.
// THE GAP: this pin reaches the Σ-reducing Eq with an Ω motive; it does not
// claim the independent Pi carrier or executable-motive route is complete.
#[test]
fn normalized_j_with_recorded_sigma_evidence_stays_inferable() {
    let f = Fixture::new();
    let recorded = f.recorded(f.right.clone());
    let reduct = whnf(&f.env, &f.ctx, &recorded);
    assert!(
        matches!(reduct, Term::Sigma(..)),
        "Eq at this Σ carrier must reduce before the recording is lost: {reduct:?}"
    );
    let j = f.j(recorded.clone());
    let before = infer(&f.env, &f.ctx, &j).expect("J is typed before normalization");
    let normal = normalize(&f.env, &f.ctx, &j);
    assert_eq!(
        infer(&f.env, &f.ctx, &normal),
        Ok(before),
        "the normalized J must still be inferable"
    );
    let Term::J(_, _, evidence) = &normal else {
        panic!("this pin must reach the neutral J arm: {normal:?}");
    };
    let Term::Ascript(_, kept) = evidence.as_ref() else {
        panic!("normalization lost J's checked Eq recording: {evidence:?}");
    };
    assert_eq!(**kept, recorded, "the Eq formation stays authoritative");
}

// Promise class: durable invariant. MEASURED: changing only the ascribed
// target endpoint makes J normalization retain that bad Eq formation, and
// inference refuses the evidence's mismatched type. CLAIMED: preservation
// never launders an unchecked equality. THE GAP: the fixture distinguishes
// independently declared, nonconvertible endpoints but not every Eq carrier.
#[test]
fn normalized_j_keeps_wrong_endpoint_and_refuses_it() {
    let f = Fixture::new();
    let actual = f.recorded(f.right.clone());
    let wrong = f.recorded(f.left.clone());
    assert_ne!(actual, wrong, "endpoints must be distinct");
    let normal = normalize(&f.env, &f.ctx, &f.j(wrong.clone()));
    let Term::J(_, _, evidence) = &normal else {
        panic!("bad evidence must not reduce J away: {normal:?}");
    };
    let Term::Ascript(_, kept) = evidence.as_ref() else {
        panic!("bad recorded endpoint must remain visible: {evidence:?}");
    };
    assert_eq!(**kept, wrong, "do not repair the bad recording silently");
    assert_eq!(
        infer(&f.env, &f.ctx, &normal),
        Err(KernelError::TypeMismatch {
            expected: Box::new(wrong),
            found: Box::new(actual),
        }),
        "j_endpoints must check the recording before reading it"
    );
}
