//! Numerical intervals §21 and Architect AC-0: a split preserves containment.
//! Promise class: durable invariant. Changing proof staging or inherited trust
//! does not change the public disjunction or the package-local trust delta.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";
const LAW: &str = "interval_split_covers";
const HELPER: &str = "interval_split_cases";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Or",
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
        .expect("Rational provider roots-loads");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval roots-loads");
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

const IMPORTS: &str = r#"
import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue)
import Core.Logic.Or (Or, Inl, Inr)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval (MkInterval, interval_contains, interval_split_covers)
"#;

const VALUES: &str = r#"
const zero : Rational = MkRational Zero Zero
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const quarter : Rational = MkRational (Suc Zero) (Suc (Suc (Suc Zero)))
const three_quarters : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc (Suc (Suc Zero)))
"#;

const OPEN_CLIENT: &str = r#"
fn split_open_swap (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : CatalogIsTrue (interval_contains (MkInterval lo hi) x)
      → Or
        (CatalogIsTrue (interval_contains (MkInterval m hi) x))
        (CatalogIsTrue (interval_contains (MkInterval lo m) x)) =
  λmember.
    match interval_split_covers lo hi m x member {
      Inl below ↦
        Inr
          (CatalogIsTrue (interval_contains (MkInterval m hi) x))
          (CatalogIsTrue (interval_contains (MkInterval lo m) x))
          below;
      Inr above ↦
        Inl
          (CatalogIsTrue (interval_contains (MkInterval m hi) x))
          (CatalogIsTrue (interval_contains (MkInterval lo m) x))
          above
    }
"#;

const TAG_CLIENT: &str = r#"
fn split_is_lower (lo : Rational) (hi : Rational) (m : Rational) (x : Rational)
    : CatalogIsTrue (interval_contains (MkInterval lo hi) x) → Bool =
  λmember.
    match interval_split_covers lo hi m x member {
      Inl below ↦ True;
      Inr above ↦ False
    }

theorem above_split_uses_upper
    : Equal Bool (split_is_lower zero one half three_quarters Proved) False = Proved
"#;

fn load_closed_fixture(env: &mut ElabEnv) {
    env.elaborate_file(IMPORTS).expect("public imports check");
    env.elaborate_file(VALUES)
        .expect("closed rational values check");
    env.elaborate_file(TAG_CLIENT)
        .expect("above-split law application checks and returns Inr");
}

const CLOSED_CLIENT: &str = r#"
theorem below_split_uses_lower
    : Equal Bool (split_is_lower zero one half quarter Proved) True = Proved

theorem inverted_upper_uses_lower
    : Equal Bool (split_is_lower zero one two half Proved) True = Proved

theorem inverted_lower_uses_upper
    : Equal Bool (split_is_lower half one zero three_quarters Proved) False = Proved

const split_at_endpoint
    : Or
        (CatalogIsTrue (interval_contains (MkInterval zero one) one))
        (CatalogIsTrue (interval_contains (MkInterval one one) one)) =
  interval_split_covers zero one one one Proved

theorem quarter_lower : CatalogIsTrue (interval_contains (MkInterval zero half) quarter) =
  match interval_split_covers zero one half quarter Proved {
    Inl below ↦ below;
    Inr above ↦ Proved
  }

theorem three_quarters_upper : CatalogIsTrue (interval_contains (MkInterval half one) three_quarters) =
  match interval_split_covers zero one half three_quarters Proved {
    Inl below ↦ Proved;
    Inr above ↦ above
  }
"#;

const FALSE_LOWER: &str = r#"theorem three_quarters_lower_by_law
    : CatalogIsTrue (interval_contains (MkInterval zero half) three_quarters) =
  match interval_split_covers zero one half three_quarters Proved {
    Inl below ↦ below;
    Inr above ↦ above
  }"#;

/// MEASURED: actual post-load GlobalId sets equal the provider baseline; the
/// rooted package owns the Transparent declaration, and a generic checked
/// client matches the Or and cites that exact Const.
/// CLAIMED: the package adds no trust and exposes an eliminable split cover.
/// THE GAP: inherited provider trust remains; the open client checks reuse,
/// not a particular algorithm or an official conformance run.
#[test]
fn provider_first_exact_trust_and_open_split() {
    let mut env = ElabEnv::new().expect("fresh compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(!before.is_empty(), "inherited provider trust is nonempty");
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational provider proof bodies check");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_rational, before, "Rational adds no trust identities");
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval proof bodies check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "split trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "RationalInterval adds no trust identities");

    let id = env.globals[&format!("{INTERVAL}.{LAW}")];
    assert!(owned.contains(&id), "the package owns the split law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public split law has a checked body"
    );
    env.elaborate_file(IMPORTS).expect("public imports check");
    env.elaborate_file(OPEN_CLIENT)
        .expect("generic client eliminates both branches of the split cover");
    let (_, body) = env
        .env
        .transparent_body(env.globals["split_open_swap"])
        .expect("open client has a checked body");
    assert!(
        body_contains_const(&body, id),
        "client cites exact law Const"
    );
}

/// MEASURED: exclusive-half rows reduce to opposite branch tags, including
/// cases where the other half is inverted. A boundary split also checks.
/// CLAIMED: the split is usable for either half without requiring interior m.
/// THE GAP: finite rows distinguish the cases; the open type gives the
/// general result. The boundary row promises no tag when both halves hold.
#[test]
fn closed_split_uses_each_half_and_accepts_exterior_points() {
    let mut env = loaded_interval();
    load_closed_fixture(&mut env);
    env.elaborate_file(CLOSED_CLIENT)
        .expect("exclusive branches and endpoint split check");
}

/// MEASURED: the same above-split application produces Inr, but returning
/// either payload as lower evidence fails at the isolated lower claim.
/// CLAIMED: the disjunction is not an unconditional lower-half guarantee.
/// THE GAP: the whole-declaration diagnostic does not name the bad branch;
/// the separate tag check establishes that the law application is valid
/// without assuming the type of the Inr payload.
#[test]
fn above_split_evidence_cannot_prove_lower_membership() {
    let mut env = loaded_interval();
    load_closed_fixture(&mut env);
    let decls = parse_decls(FALSE_LOWER).expect("the isolated lower claim parses");
    let [SourceDecl::TheoremDecl { span: claim, .. }] = decls.as_slice() else {
        panic!("fixture must be a single theorem");
    };
    match env.elaborate_file(FALSE_LOWER) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {}
        other => panic!("false lower claim must TypeMismatch at its own declaration: {other:?}"),
    }
}

/// MEASURED: a selective import of the public law succeeds; importing the
/// helper instead reports UnboundName for its exact qualified name.
/// CLAIMED: proof staging is not part of the public package interface.
/// THE GAP: this covers the split helper, not every private package member.
#[test]
fn split_helper_is_not_importable() {
    let mut env = loaded_interval();
    env.elaborate_file(&format!("import {INTERVAL} ({LAW})"))
        .expect("public law imports from the same module");
    let source = format!("import {INTERVAL} ({HELPER})");
    match env.elaborate_file(&source) {
        Err(ElabError::UnboundName { name, span })
            if name == format!("{INTERVAL}.{HELPER}")
                && span.start <= "import ".len()
                && span.end >= source.len() - 1 => {}
        other => panic!("private split helper must refuse its import: {other:?}"),
    }
}
