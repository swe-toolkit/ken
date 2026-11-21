//! Numerical intervals §20: a zero-width interval has equivalent members.
//! Promise class: durable invariant. A public open client checks the stated
//! law, while closed cases distinguish a false width premise from a true
//! equivalence conclusion without claiming structural Rational equality.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";
const LAW: &str = "interval_width_zero_equiv";

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
        .expect("checked Rational provider");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval root-loads");
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
import Data.Numeric.Rational (Rational, rational_equiv, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_width, interval_width_zero_equiv)

theorem width_zero_open (i : Interval) (x : Rational) (y : Rational)
    : rational_equiv (interval_width i) rational_zero
      → IsTrue (interval_contains i x)
      → IsTrue (interval_contains i y)
      → rational_equiv x y =
  interval_width_zero_equiv i x y
"#;

const PREFIX: &str = r#"
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational, rational_equiv, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_width, interval_point,
   interval_width_point, interval_width_zero_equiv)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const two_quarters : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const one : Rational = MkRational (Suc Zero) Zero
const pinned : Interval = MkInterval half two_quarters
const loose : Interval = MkInterval half one
const point : Interval = interval_point half
"#;

const CONTROLS: &str = r#"
theorem member_equivalence_is_true : rational_equiv half two_quarters = Proved
theorem pinned_contains_half : IsTrue (interval_contains pinned half) = Proved
theorem pinned_contains_two_quarters : IsTrue (interval_contains pinned two_quarters) = Proved
theorem loose_contains_half : IsTrue (interval_contains loose half) = Proved
theorem loose_contains_two_quarters : IsTrue (interval_contains loose two_quarters) = Proved
theorem pinned_width_zero : rational_equiv (interval_width pinned) rational_zero = Proved
"#;

const CLOSED_LAWS: &str = r#"
theorem pinned_members_equiv : rational_equiv half two_quarters =
  interval_width_zero_equiv pinned half two_quarters Proved Proved Proved

theorem point_width_zero : rational_equiv (interval_width point) rational_zero =
  interval_width_point half

theorem point_contains_half : IsTrue (interval_contains point half) = Proved
theorem point_contains_two_quarters : IsTrue (interval_contains point two_quarters) = Proved
theorem point_members_equiv : rational_equiv half two_quarters =
  interval_width_zero_equiv point half two_quarters
    (interval_width_point half) Proved Proved
"#;

const FALSE_WIDTH: &str = r#"theorem loose_width_zero_claim
    : rational_equiv (interval_width loose) rational_zero = Proved"#;
const REFUSED_LAW: &str = r#"theorem loose_members_equiv : rational_equiv half two_quarters =
  interval_width_zero_equiv loose half two_quarters Proved Proved Proved"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §20, Architect AC-0 proof.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the provider-first pre/post trusted-base inventories are exact
/// GlobalId sets, unchanged through Rational and RationalInterval. The
/// public owner is Transparent, and an open client body contains its Const.
/// CLAIMED: clients can reuse the generic checked law with no additional
/// trust, using its zero-width and two containment premises in that order.
/// THE GAP: inherited provider trust remains, and this is neither an
/// official conformance run nor a whole-workspace no-regression claim.
#[test]
fn provider_first_exact_identity_trust_and_open_law() {
    let mut env = ElabEnv::new().expect("fresh compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(
        !before.is_empty(),
        "the inherited provider baseline is nonempty"
    );
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational provider proof bodies check");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_rational, before, "Rational adds no trust identities");
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval closure proof bodies check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "zero-width trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "RationalInterval adds no trust identities");
    let id = env.globals[&format!("{INTERVAL}.{LAW}")];
    assert!(owned.contains(&id), "the package owns its public theorem");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public theorem has a checked body"
    );
    env.elaborate_file(&checked_is_true::selected_source_with_import(OPEN_CLIENT))
        .expect("explicit IsTrue alias and open ordered-premise law check");
    let (_, body) = env
        .env
        .transparent_body(env.globals["width_zero_open"])
        .expect("open client has a checked body");
    assert!(
        body_contains_const(&body, id),
        "client cites exact law Const"
    );
}

/// SOURCE: spec §20, zero-width pair [1/2, 2/4] and point composition.
/// PROMISE CLASS: durable invariant on law use; the closed values are
/// independently specified for this finite Rational fixture.
/// MEASURED: the members are equivalent by their own Proved row; each
/// containment is proved independently; the interval and point widths
/// are zero and the public law applies to both.
/// CLAIMED: the public law is usable on equal-valued but differently
/// spelled members and composes with the existing point-width law.
/// THE GAP: finite values cannot prove the universally quantified body.
#[test]
fn zero_width_pair_and_point_composition_accept() {
    let mut env = loaded_interval();
    env.elaborate_file(&checked_is_true::selected_source_with_import(PREFIX))
        .expect("fresh checked interval and distinct rational spellings");
    env.elaborate_file(&checked_is_true::selected_source(CONTROLS))
        .expect("both members and both containments check independently");
    env.elaborate_file(&checked_is_true::selected_source(CLOSED_LAWS))
        .expect("closed absorption and point composition check");
}

fn assert_own_type_mismatch(env: &mut ElabEnv, source: &str, name: &str, proof: &str) {
    let decls = parse_decls(source).expect("the isolated claim parses");
    let [SourceDecl::TheoremDecl {
        name: actual,
        span: claim,
        ..
    }] = decls.as_slice()
    else {
        panic!("one own theorem, not a rejected prefix");
    };
    assert_eq!(actual, name);
    let span = match env
        .elaborate_file(source)
        .expect_err("false width premise refuses")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } => span,
        other => panic!("{name} must refuse by KernelRejected(TypeMismatch): {other:?}"),
    };
    let terminal = source
        .find(proof)
        .expect("the false premise proof is present");
    assert!(
        claim.start <= span.start
            && span.start < span.end
            && span.end <= claim.end
            && span.start <= terminal
            && terminal + proof.len() <= span.end,
        "{name}: rejection must enclose its own premise proof: {span:?} vs {claim:?}"
    );
}

/// SOURCE: spec §20, positive-width interval [1/2, 1].
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full accepted prefix, true equivalence conclusion, and
/// both loose-interval containment proofs check in fresh environments.
/// A standalone false-width claim and the same-form public-law application
/// each KernelReject at their own theorem and enclose their proof terms.
/// CLAIMED: a true member-equivalence conclusion cannot excuse a false
/// zero-width premise on an interval of positive width.
/// THE GAP: the whole-law diagnostic spans its theorem; the separate
/// premise-only refusal identifies the cause without claiming checker order.
#[test]
fn positive_width_refuses_false_premise_with_true_conclusion() {
    for (name, claim, proof) in [
        ("loose_width_zero_claim", FALSE_WIDTH, "Proved"),
        ("loose_members_equiv", REFUSED_LAW, "Proved"),
    ] {
        let mut env = loaded_interval();
        env.elaborate_file(&checked_is_true::selected_source_with_import(PREFIX))
            .expect("fresh interval prefix accepts");
        env.elaborate_file(&checked_is_true::selected_source(CONTROLS))
            .expect("true conclusion and both containments accept first");
        let selected = checked_is_true::selected_source(claim);
        assert_own_type_mismatch(&mut env, &selected, name, proof);
    }
}

/// SOURCE: spec §20 private truncation-to-order proof chain.
/// PROMISE CLASS: durable invariant.
/// MEASURED: selectively importing each helper from a checked package
/// refuses with UnboundName on the requested helper.
/// CLAIMED: clients use the public equivalence theorem, not private Nat or
/// Rational proof machinery.
/// THE GAP: this names three helpers, not a universal helper inventory.
#[test]
fn zero_width_proof_helpers_are_private() {
    for helper in [
        "nat_sub_zero_leq",
        "rational_sub_zero_numerator",
        "rational_sub_zero_leq",
    ] {
        let mut env = loaded_interval();
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
