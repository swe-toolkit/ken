//! Manhattan metric: spec/50-stdlib/67-geometry.md §2.
//! The two closed conformance refusals and the D0 identity refusal each
//! run after the same checked positive client. Promise class: durable invariant.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, ImportKind, KenMdFenceRole,
};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-manhattan-metric.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

// The isolated compiler base already binds IsTrue. Inspect the parsed Ken
// import, not its whitespace or source line: remove only that redundant name
// and retain every other selective import, including future additions.
fn seed_client_without_redundant_import(source: &str) -> String {
    let mut edits = Vec::new();
    for declaration in parse_decls(source).expect("seed Ken parses") {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        {
            if module != "Core.Classes.LawfulClasses" {
                continue;
            }
            let mut keep = Vec::new();
            let mut removed = false;
            for item in items {
                if item.name == "IsTrue" && item.rename.is_none() {
                    removed = true;
                } else {
                    keep.push(match item.rename {
                        Some(alias) => format!("{} as {alias}", item.name),
                        None => item.name,
                    });
                }
            }
            if removed {
                let replacement = if keep.is_empty() {
                    String::new()
                } else {
                    format!("import {module} ({})", keep.join(", "))
                };
                edits.push((span.start..span.end, replacement));
            }
        }
    }
    let mut client = source.to_owned();
    for (range, replacement) in edits.into_iter().rev() {
        client.replace_range(range, &replacement);
    }
    client
}

fn seed_programs() -> Vec<String> {
    let extraction = extract_ken_md(SEED).expect("conformance Ken fences are well-formed");
    extraction
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| seed_client_without_redundant_import(&SEED[fence.body_range]))
        .collect()
}

fn geometry_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §2 roots-loads with checked providers");
    env
}

// The client states the §2 theorem interfaces with arbitrary point values,
// rather than merely asserting that the closed seed's propositions check.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox
  (Point, manhattan, manhattan_triangle, manhattan_zero_equal)

theorem generic_triangle (p : Point) (q : Point) (r : Point) :
  IsTrue (leq_nat (manhattan p r) (add (manhattan p q) (manhattan q r))) =
  manhattan_triangle p q r

theorem generic_zero_equal (p : Point) (q : Point)
    (h : Equal Nat (manhattan p q) Zero) : Equal Point p q =
  manhattan_zero_equal p q h
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §2.1–2.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after loading the four explicit providers, NatBox adds zero
/// trusted GlobalId identities; both law bodies belong to that module and
/// are transparent, and a separate public client restates their generic
/// types. Four chosen private per-axis imports are refused by the loader.
/// CLAIMED: the two metric laws are proved in the catalog without growing
/// trust or exporting the per-axis implementation to clients.
/// THE GAP: the provider assumptions remain inherited, while privacy of
/// other helpers is source-reviewed rather than established by four probes.
#[test]
fn metric_laws_are_generic_checked_and_trust_set_is_unchanged() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("metric source and both generic proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "metric source adds no trusted GlobalId");

    for law in ["manhattan_triangle", "manhattan_zero_equal"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must belong to NatBox");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must contain a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("public clients can consume both §2 laws at arbitrary points");

    for private in [
        "axis_distance",
        "axis_distance_triangle",
        "axis_distance_zero",
        "sub_triangle",
    ] {
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-manhattan-metric.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the real conformance Ken client checks through public imports;
/// independently authored closed points establish direct=2, detour=8,
/// reverse=False, and distance=2 on distinct points, plus an instance of
/// the triangle law. Thus the contract stays pinned when the seed grows or
/// renames its own declarations without changing these behaviors.
/// CLAIMED: closed arithmetic discriminates the inequality direction and
/// the metric laws remain usable outside their owning module.
/// THE GAP: fixed vectors do not prove universal laws; generic bodies and
/// independent arbitrary-point client types are checked above.
#[test]
fn metric_seed_clients_check_strict_and_tight_detours() {
    let programs = seed_programs();
    let positive = programs.first().expect("conformance has a positive client");
    geometry_env()
        .elaborate_file(positive)
        .expect("positive conformance client imports and checks §2 metrics");
    geometry_env()
        .elaborate_file(CLOSED_METRIC_CLIENT)
        .expect("independent closed metric controls check through public imports");
}

// These closed controls use only spec §2's public values and laws; they do
// not depend on spellings, declaration counts, or fixture names in the seed.
const CLOSED_METRIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, manhattan, manhattan_triangle, manhattan_zero_equal)

const client_zero : Nat = Zero
const client_one : Nat = Suc client_zero
const client_two : Nat = Suc client_one
const client_three : Nat = Suc client_two
const client_eight : Nat = Suc (Suc (Suc (Suc (Suc client_three))))
const client_origin : Point = MkPoint client_zero client_zero
const client_detour : Point = MkPoint client_three client_two
const client_target : Point = MkPoint client_one client_one
const client_distinct_a : Point = MkPoint client_zero client_one
const client_distinct_b : Point = MkPoint client_one client_zero

theorem direct_is_two : Equal Nat (manhattan client_origin client_target) client_two = Proved
theorem detour_is_eight :
  Equal Nat
    (add (manhattan client_origin client_detour) (manhattan client_detour client_target))
    client_eight = Proved
theorem reverse_is_false :
  Equal Bool
    (leq_nat
      (add (manhattan client_origin client_detour) (manhattan client_detour client_target))
      (manhattan client_origin client_target))
    False = Proved
theorem distinct_distance_is_two :
  Equal Nat (manhattan client_distinct_a client_distinct_b) client_two = Proved
theorem triangle_instance :
  IsTrue (leq_nat (manhattan client_origin client_target)
                  (add (manhattan client_origin client_detour)
                       (manhattan client_detour client_target))) =
  manhattan_triangle client_origin client_detour client_target
"#;

const REJECT_REVERSE_CONTRACT: &str = r#"
theorem reject_reverse_contract :
  IsTrue (leq_nat
    (add (manhattan client_origin client_detour) (manhattan client_detour client_target))
    (manhattan client_origin client_target)) = Proved
"#;

const REJECT_ZERO_CONTRACT: &str = r#"
theorem reject_zero_contract :
  Equal Nat (manhattan client_distinct_a client_distinct_b) Zero = Proved
"#;

// The third refusal is the Architect D0 identity-on-distinct case. Its
// premise is impossible on the same independently controlled distinct pair.
const REJECT_IDENTITY_ON_DISTINCT: &str = r#"
theorem reject_identity_on_distinct :
  Equal Point client_distinct_a client_distinct_b =
  manhattan_zero_equal client_distinct_a client_distinct_b Proved
"#;

/// SOURCE: conformance/stdlib/geometry/seed-manhattan-metric.md negatives;
/// Architect D0 evt_1nzebfnfzwq0, third identity-on-distinct refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all seed rejection fences, however many, are compiled after
/// their positive client. Independently, three fixed, source-agnostic
/// negative propositions are compiled after their own positive distance
/// controls. Every rejection is KernelRejected(TypeMismatch) at its theorem.
/// CLAIMED: reverse detours, false zero distances, and identities derived
/// with fabricated zero-distance premises cannot be proved.
/// THE GAP: the controls cover these closed points, not arbitrary points;
/// the generic laws are checked separately and QA owns AC-3's proof mutant.
#[test]
fn false_metric_claims_are_kernel_refused_at_their_own_theorems() {
    let programs = seed_programs();
    let (positive, seed_negatives) = programs.split_first().expect("conformance has a client");
    for negative in seed_negatives {
        let mut env = geometry_env();
        env.elaborate_file(positive)
            .expect("seed's positive client checks before its refusal");
        assert_own_theorem_kernel_refusal(&mut env, negative, "seed refusal");
    }
    for (negative, label) in [
        (REJECT_REVERSE_CONTRACT, "reverse detour"),
        (REJECT_ZERO_CONTRACT, "distinct points at zero"),
        (REJECT_IDENTITY_ON_DISTINCT, "identity on distinct points"),
    ] {
        let mut env = geometry_env();
        env.elaborate_file(CLOSED_METRIC_CLIENT)
            .expect("independent positive controls check before each refusal");
        assert_own_theorem_kernel_refusal(&mut env, negative, label);
    }
}

fn assert_own_theorem_kernel_refusal(env: &mut ElabEnv, negative: &str, label: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            // The parser locates the false declaration's own span; its
            // spelling and whitespace are irrelevant to this diagnosis.
            let in_theorem = parse_decls(negative)
                .expect("negative client is valid Ken")
                .into_iter()
                .any(|decl| {
                    matches!(decl, SourceDecl::TheoremDecl { span: theorem, .. }
                        if span.start >= theorem.start && span.end <= theorem.end && span.start < span.end)
                });
            assert!(
                in_theorem,
                "{label}: K0004 must target its false theorem: {span:?}"
            );
        }
        other => panic!("{label}: expected kernel TypeMismatch, got {other:?}"),
    }
}
