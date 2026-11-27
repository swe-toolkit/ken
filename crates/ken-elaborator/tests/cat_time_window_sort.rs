//! Spec 69 §24 and the window-sort seed. Promise class: durable invariant.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const TRACE: &str = "Data.Time.EventTrace";
const LAW: &str = "trace_window_sort";
const OPEN: &str = "sorted_window_open";
const CLOSED: &str = "closed_by_law";
const REFUSED: &str = "sorted_window_is_unsorted";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-trace-window-sort-commute.md"
));

fn loaded() -> (ElabEnv, Vec<GlobalId>, GlobalId) {
    let roots = &[PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(roots, "Data.Collections.Derived")
        .expect("checked collections provider roots-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(roots, TRACE)
        .expect("EventTrace proof bodies check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert!(!before.is_empty(), "inherited provider trust is nonempty");
    assert_eq!(before, after, "EventTrace adds no trusted identities");
    let law = env.globals[&format!("{TRACE}.{LAW}")];
    assert!(owned.contains(&law), "EventTrace owns {LAW}");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    (env, owned, law)
}

fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§24 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§24 seed source parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            )
        {
            assert!(
                selected.replace(source).is_none(),
                "duplicate §24 case {theorem_name}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §24 case {theorem_name}"))
}

fn accepted_prefix(env: &mut ElabEnv, source: &str) -> Vec<GlobalId> {
    // Do not pre-import any names into the client's namespace.
    env.elaborate_file(source)
        .expect("complete accepted §24 seed prefix checks")
}

fn cites(body: &Term, law: GlobalId) -> bool {
    matches!(body, Term::Const { id, .. } if *id == law)
        || body.children().into_iter().any(|child| cites(child, law))
}

fn assert_citation(
    env: &ElabEnv,
    declared: &[GlobalId],
    name: &str,
    law: GlobalId,
    expected: bool,
) {
    let id = env.globals[name];
    assert!(declared.contains(&id), "seed client declares {name}");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a checked body");
    };
    assert_eq!(cites(body, law), expected, "{name}: checked law citation");
}

// Rewrite only the parsed closing expression, leaving the seed's actual
// imports, fixtures, binders and statement in place. Reparse the result so
// rejection spans are relative to the program the elaborator receives.
fn with_closing_term(source: &str, name: &str, replacement: &str) -> (String, SourceDecl) {
    fn theorem(source: &str, name: &str) -> SourceDecl {
        parse_decls(source)
            .expect("seed control parses")
            .into_iter()
            .find(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name))
            .unwrap_or_else(|| panic!("missing {name} in accepted seed"))
    }
    let SourceDecl::TheoremDecl { body, .. } = theorem(source, name) else {
        unreachable!("theorem selector")
    };
    let span = body.span();
    let mut control = source.to_owned();
    control.replace_range(span.start..span.end, replacement);
    let claim = theorem(&control, name);
    (control, claim)
}

/// MEASURED: exact trust identities are unchanged. The accepted seed's open
/// and closed checked bodies cite the owned Transparent law. The same closed
/// claim checks with Refl and no law citation. CLAIMED: the closed result is
/// usage evidence only. THE GAP: it cannot establish the generic equality;
/// provider trust persists and this is not the conformance runner.
#[test]
fn seed_checks_against_owned_law_without_new_trust() {
    let accepted = seed_case(OPEN);
    let (mut env, _, law) = loaded();
    let declared = accepted_prefix(&mut env, accepted);
    assert_citation(&env, &declared, OPEN, law, true);
    assert_citation(&env, &declared, CLOSED, law, true);

    let (by_refl, _) = with_closing_term(accepted, CLOSED, "Refl");
    let (mut env, _, law) = loaded();
    let declared = env
        .elaborate_file(&by_refl)
        .expect("closed usage computes without the law");
    assert_citation(&env, &declared, CLOSED, law, false);
}

/// MEASURED: after the original accepted program checks, replacing only its
/// open closing term by Refl or Proved refuses in that same theorem. CLAIMED:
/// the open witness is not a computation-only example. THE GAP: these are
/// two closing terms, not a claim that no other proof exists.
#[test]
fn open_seed_claim_does_not_close_by_computation() {
    let accepted = seed_case(OPEN);
    let (mut env, _, _) = loaded();
    accepted_prefix(&mut env, accepted);
    for replacement in ["Refl", "Proved"] {
        let (control, claim) = with_closing_term(accepted, OPEN, replacement);
        let SourceDecl::TheoremDecl {
            params,
            span: claim,
            body,
            ..
        } = claim
        else {
            unreachable!("theorem selector")
        };
        assert!(!params.is_empty(), "seed witness must be open");
        let (mut env, _, _) = loaded();
        match (replacement, env.elaborate_file(&control)) {
            ("Refl", Err(ElabError::TypeMismatch { span, .. })) if &span == body.span() => {
                eprintln!("{OPEN} by Refl: TypeMismatch at {span:?}");
            }
            (
                "Proved",
                Err(ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    span,
                }),
            ) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
                eprintln!("{OPEN} by Proved: KernelRejected(TypeMismatch) at {span:?}");
            }
            (_, other) => {
                panic!("{OPEN} by {replacement} must refuse at its own theorem: {other:?}")
            }
        }
    }
}

/// MEASURED: the single seed twin, run after a fresh complete accepted
/// prefix, kernel-refuses inside its own declaration. CLAIMED: applying the
/// law does not justify dropping the sort. THE GAP: this pins the failure of
/// this application, not its internal diagnostic wording or span.
#[test]
fn dropping_sort_refuses_after_fresh_accepted_seed() {
    let (mut env, _, _) = loaded();
    accepted_prefix(&mut env, seed_case(OPEN));
    let refused = seed_case(REFUSED);
    let decls = parse_decls(refused).expect("isolated seed refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim,
        ..
    }] = decls.as_slice()
    else {
        panic!("expected one isolated refused theorem");
    };
    assert_eq!(name, REFUSED);
    assert!(!params.is_empty(), "refused twin must be open");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
            eprintln!("{REFUSED}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{REFUSED} must kernel-refuse at its own theorem: {other:?}"),
    }
}

/// MEASURED: all nine helpers belong to the checked package, and named
/// selective imports refuse. CLAIMED: they are not client API. THE GAP:
/// this checks the named helpers, not the entire export population.
#[test]
fn window_sort_proof_helpers_are_private() {
    let (mut env, owned, _) = loaded();
    for helper in [
        "trace_window_sort_cons",
        "filter_insert",
        "filter_insert_cons",
        "filter_insert_later",
        "filter_cons_insert",
        "insert_kept",
        "window_keep_insert",
        "insert_after_head",
        "insert_starts_after",
    ] {
        let qualified = format!("{TRACE}.{helper}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "EventTrace owns {helper}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) if name == qualified => {}
            other => panic!("{helper} must stay unimportable: {other:?}"),
        }
    }
}
