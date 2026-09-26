//! A checked session reference must not depend on its legacy globals spelling.

use ken_elaborator::ElabEnv;
use ken_kernel::{GlobalId, Level, Term};

fn mentions_const(term: &Term, id: GlobalId) -> bool {
    matches!(term, Term::Const { id: found, .. } if *found == id)
        || term
            .children()
            .into_iter()
            .any(|child| mentions_const(child, id))
}

/// Promise class: durable invariant. MEASURED: a checked raw postulate is
/// usable as a type after its flat spelling is removed. CLAIMED: source-unit
/// resolution uses session identity. THE GAP: the matching pre-session base
/// must reject this exact test after the same removal, not merely stay green.
#[test]
fn raw_type_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let ty = env
        .declare_postulate_raw("SessionRawType", Term::ty(Level::Zero))
        .expect("checked raw type");
    assert_eq!(env.globals.remove("SessionRawType"), Some(ty));
    let use_id = env
        .elaborate_decl("fn use_session_raw (x : SessionRawType) : SessionRawType = x")
        .expect("type reference must use the session identity");
    let (_, checked_ty) = env.env.const_type(use_id).expect("checked type");
    assert_eq!(
        checked_ty,
        Term::pi(Term::const_(ty, vec![]), Term::const_(ty, vec![]))
    );
}

/// Promise class: durable invariant. MEASURED: a prior incremental source
/// declaration remains readable to the next unit without its flat globals key.
#[test]
fn prior_source_local_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let first = env
        .elaborate_decl("const session_source : Bool = True")
        .expect("checked earlier source unit");
    assert_eq!(env.globals.remove("session_source"), Some(first));
    let second = env
        .elaborate_decl("const second_session : Bool = session_source")
        .expect("later unit must select the earlier checked identity");
    let (_, body) = env.env.transparent_body(second).expect("checked body");
    assert_eq!(body, Term::const_(first, vec![]));
}

/// Promise class: durable invariant. MEASURED: the REPL expression API reads
/// a checked raw postulate's identity after its flat spelling is removed.
#[test]
fn repl_expression_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let ty = env
        .declare_postulate_raw("SessionRawType", Term::ty(Level::Zero))
        .expect("checked raw type");
    let value = env
        .declare_postulate_raw("session_raw_value", Term::const_(ty, vec![]))
        .expect("checked raw value");
    assert_eq!(env.globals.remove("session_raw_value"), Some(value));
    let (term, actual_ty) = env
        .elaborate_expr("session REPL probe", "session_raw_value")
        .expect("standalone expression must select the session identity");
    assert_eq!(term, Term::const_(value, vec![]));
    assert_eq!(actual_ty, Term::const_(ty, vec![]));
}

/// Promise class: durable invariant. MEASURED: a checked bootstrap attached
/// proof is captured at the seal and selected without its flat spelling.
#[test]
fn prelude_attached_proof_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    let proof = env.globals["write_all_call_bound::termination"];
    assert_eq!(
        env.globals.remove("write_all_call_bound::termination"),
        Some(proof)
    );
    let (term, ty) = env
        .elaborate_expr(
            "prelude attached selector probe",
            "write_all_call_bound::termination",
        )
        .expect("bootstrap checked attached proof is selected in the session");
    assert_eq!(term, Term::const_(proof, vec![]));
    assert_eq!(
        ty,
        env.env.const_type(proof).expect("checked prelude proof").1
    );
}

/// Promise class: durable invariant. MEASURED: a checked attached proof in a
/// prior unit remains referable after removing only its flat selector key.
/// CLAIMED: the selector is a session-selected ID, not a fresh spelling lookup.
/// THE GAP: an earlier checked source without a proof would not exercise this
/// branch, so the positive control names the exact ID in the resulting term.
#[test]
fn prior_attached_proof_survives_disabled_flat_fallthrough() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "fn session_identity (x : Int) : Int = x \
         proof stable for session_identity (x : Int) : Equal Int (session_identity x) x = Refl",
    )
    .expect("checked subject and attached proof");
    let proof = env.globals["session_identity::stable"];
    assert_eq!(env.globals.remove("session_identity::stable"), Some(proof));
    let theorem = env
        .elaborate_decl(
            "theorem follow_session (x : Int) : Equal Int (session_identity x) x = \
             session_identity::stable x",
        )
        .expect("prior checked attached proof must survive missing flat spelling");
    let (_, body) = env
        .env
        .transparent_body(theorem)
        .expect("checked theorem body");
    assert!(
        mentions_const(&body, proof),
        "wrong attached proof selected: {body:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a prior-session proof's
/// subject is validated by its checked ID, even without the subject's flat
/// spelling; other globals keys remain available for the proof declaration.
#[test]
fn new_attached_proof_validates_prior_subject_without_flat_key() {
    let mut env = ElabEnv::new().expect("prelude");
    let subject = env
        .elaborate_decl("fn session_subject (x : Int) : Int = x")
        .expect("checked prior subject");
    assert_eq!(env.globals.remove("session_subject"), Some(subject));
    let proof = env
        .elaborate_decl(
            "proof stable for session_subject (x : Int) : Equal Int (session_subject x) x = Refl",
        )
        .expect("a checked prior subject does not need globals for proof admission");
    let (_, ty) = env.env.const_type(proof).expect("checked proof type");
    assert!(
        mentions_const(&ty, subject),
        "wrong attached subject: {ty:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a same-unit proof selector is
/// available during the ordered declaration pass, before session write-back.
#[test]
fn same_unit_attached_proof_preserves_scc_precedence() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "fn local_identity (x : Int) : Int = x \
         proof stable for local_identity (x : Int) : Equal Int (local_identity x) x = Refl \
         theorem use_local (x : Int) : Equal Int (local_identity x) x = \
         local_identity::stable x",
    )
    .expect("same-unit proof reference elaborates in the SCC run");
    let proof = env.globals["local_identity::stable"];
    let theorem = env.globals["use_local"];
    let (_, body) = env
        .env
        .transparent_body(theorem)
        .expect("checked theorem body");
    assert!(
        mentions_const(&body, proof),
        "same-unit proof changed: {body:?}"
    );
}

/// Promise class: durable invariant. MEASURED: when a harness-installed
/// session selector has the same spelling as an imported proof selector,
/// the imported provider ID wins, even if its flat spelling is removed.
#[test]
fn imported_proof_provider_precedes_session_selector_of_same_spelling() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "fn earlier_identity (x : Int) : Int = x \
         proof stable for earlier_identity (x : Int) : Equal Int (earlier_identity x) x = Refl",
    )
    .expect("first checked attached proof");
    let earlier = env.globals["earlier_identity::stable"];
    env.bind_session_name("id::stable", earlier)
        .expect("harness selector for the prior proof");
    env.elaborate_file(
        "module Provider { \
             pub fn id (x : Int) : Int = x \
             pub proof stable for id (x : Int) : Equal Int (id x) x = Refl \
         } \
         import Provider (id)",
    )
    .expect("selected provider import");
    let imported = env.globals["Provider.id::stable"];
    assert_ne!(earlier, imported);
    assert_eq!(env.globals.remove("Provider.id::stable"), Some(imported));
    let theorem = env
        .elaborate_decl("theorem use_provider (x : Int) : Equal Int (id x) x = id::stable x")
        .expect("imported attached proof selects the provider, not session history");
    let (_, body) = env
        .env
        .transparent_body(theorem)
        .expect("checked theorem body");
    assert!(
        mentions_const(&body, imported),
        "imported proof not selected: {body:?}"
    );
    assert!(
        !mentions_const(&body, earlier),
        "session proof stole imported selector: {body:?}"
    );
}
