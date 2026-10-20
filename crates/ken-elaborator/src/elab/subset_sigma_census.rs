//! Candidate-side AC-0b-3: count emitted introduction pairs on the real
//! catalog dependency closures and independently kernel-check each pair at
//! the exact local context and subset type at its production introduction.

use super::*;
use crate::ElabEnv;
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::path::PathBuf;

thread_local! {
    static RECORD: Cell<bool> = const { Cell::new(false) };
    static PAIRS: RefCell<Vec<PairWitness>> = const { RefCell::new(Vec::new()) };
}

#[derive(Debug)]
struct PairWitness {
    owner: String,
    span: Span,
    proof_uses_trusted: bool,
}

fn uses_trusted(term: &Term, trusted: &BTreeSet<GlobalId>) -> bool {
    matches!(term, Term::Const { id, .. } if trusted.contains(id))
        || term
            .children()
            .into_iter()
            .any(|child| uses_trusted(child, trusted))
}

pub(super) fn record_pair(cx: &ElabCtx<'_>, pair: &Term, target: &Term, span: &Span) {
    if !RECORD.with(Cell::get) {
        return;
    }
    let Term::Pair(_, proof) = pair else {
        panic!("introduction lost its checked pair")
    };
    assert!(
        subset_refinement(cx, target).is_some(),
        "pair target must be a Type-carrier, Omega-predicate subset"
    );
    kernel_check_raw(cx.env, &cx.ctx, pair, target)
        .expect("independent kernel check of emitted subset pair at introduction context");
    let trusted: BTreeSet<_> = cx.env.trusted_base().into_iter().collect();
    PAIRS.with(|pairs| {
        pairs.borrow_mut().push(PairWitness {
            owner: cx.owner_label.clone(),
            span: span.clone(),
            proof_uses_trusted: uses_trusted(proof, &trusted),
        })
    });
}

fn take_pairs() -> Vec<PairWitness> {
    PAIRS.with(|pairs| std::mem::take(&mut *pairs.borrow_mut()))
}

// Promise class: transition sentinel. The frame pins these six dependency
// closures at the candidate base. Changing their catalog dependency graph
// requires re-running the introduction census rather than re-recording 16.
// MEASURED: exact roots-loaded results and the pairs emitted at the checked
// introduction seam, with a second kernel check on each pair.
// CLAIMED: the six baseline closures stay admitted with sixteen independently
// checked subset pairs and no trusted assumption in a proof field.
// THE GAP: this does not census a newly added catalog root; the independent
// 78-root population/parity sweep is a separate acceptance measurement.
#[test]
fn catalog_six_closures_introduce_sixteen_independently_checked_pairs() {
    // Baseline's measured prelude peak was 1,982,464 bytes at 569f52f19.
    // A stated 8 MiB stack provisions more than 6 MiB of headroom for
    // dependency-heavy roots loading; this test does not pin stack depth.
    const CATALOG_STACK_BYTES: usize = 8 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(CATALOG_STACK_BYTES)
        .spawn(|| {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("catalog/packages");
            let closures = [
                ("Application.CommandLine.ArgParse", 3),
                ("Application.Configuration.Decoder", 3),
                ("Application.Input.Schema", 3),
                ("Capability.Console.Text", 1),
                ("Capability.Diagnostics.Render", 3),
                ("Capability.Formatting.Doc", 3),
            ];
            let mut outcomes = Vec::new();
            let mut all_sites = BTreeSet::new();
            let mut total_pairs = 0;
            for (name, expected) in closures {
                let mut env = ElabEnv::new().expect("fresh checked prelude");
                assert!(take_pairs().is_empty(), "prior closure record leaked");
                RECORD.with(|enabled| enabled.set(true));
                let result = env.elaborate_module_from_roots_v1(&[root.clone()], name);
                RECORD.with(|enabled| enabled.set(false));
                let pairs = take_pairs();
                let outcome = match result {
                    Ok(results) => {
                        let introductions: Vec<_> = results.iter()
                            .flat_map(|result| result.obligations.iter())
                            .filter(|o| matches!(o.kind, ObligationKind::RefinementIntroduction))
                            .collect();
                        let open = introductions.iter()
                            .filter(|o| env.is_open_hole(o.hole_id)).count();
                        let trusted = pairs.iter().filter(|pair| pair.proof_uses_trusted).count();
                        for pair in &pairs {
                            all_sites.insert((pair.owner.clone(), pair.span.start, pair.span.end));
                        }
                        total_pairs += pairs.len();
                        format!("{name}: obligations={} pairs={} open={open} trusted={trusted} expected={expected}",
                            introductions.len(), pairs.len())
                    }
                    Err(error) => format!("{name}: refused={error:?} partial_pairs={}", pairs.len()),
                };
                eprintln!("AC0B-PRODUCTION {outcome}");
                outcomes.push(outcome);
            }
            assert!(outcomes.iter().zip(closures).all(|(outcome, (name, expected))|
                outcome == &format!("{name}: obligations={expected} pairs={expected} open=0 trusted=0 expected={expected}")),
                "six-closure admission and independent pair-check boundary: {outcomes:?}");
            assert_eq!(total_pairs, 16, "all sixteen introduction events are paired");
            assert_eq!(all_sites.len(), 4, "three Doc leaves and one Console source site");
        })
        .expect("stated catalog stack")
        .join()
        .expect("catalog census thread");
}
