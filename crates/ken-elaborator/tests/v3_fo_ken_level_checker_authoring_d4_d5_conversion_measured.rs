//! `V3-FO-KEN-LEVEL-CHECKER-AUTHORING` `D4`+`D5`, the node's final increment:
//! measure **kernel conversion** of the Ken equation `fok_check_cert
//! (fok_embed f) pi = True`, via `Proved` -- not the Rust reference checker
//! (that was `V3-FO-CONVERSION-LOAD-MEASURED`'s corrected, honestly-labelled
//! subject; see `v3_fo_conversion_load_measured.rs`'s own module doc for why
//! `check_cert`/`embed` there are native Rust, not kernel-checked Ken terms).
//! `D0`-`D3` built the Ken-level artifact that measurement needed and did
//! not have; this file takes it.
//!
//! # Why `Proved`, not `Refl`, and what that means for `AC-1`
//!
//! The node's own framing (inherited from `V3-FO-CONVERSION-LOAD-MEASURED`'s
//! original cost model) names the forcing mechanism as `refl True`. **That is
//! wrong for a `Bool`-valued equation in this kernel, and the wrongness is
//! itself informative, not merely a naming slip.**
//!
//! Confirmed empirically: `theorem t : Equal Bool True True = Refl` is
//! **rejected** -- `"Refl expects an Eq-shaped goal"`. The reason is in
//! `ken_kernel::conv::whnf`'s `Term::Eq` arm and `ken_kernel::obs::
//! eq_reduce`: equality at an INDUCTIVE type (`Bool`, `Nat`, and `FokForm`/
//! `FokCert` alike) is an **observational** reduction (`eq_at_inductive`),
//! not primitive `Eq`. `whnf`-ing the goal type `Eq Bool (fok_check_cert
//! ...) True` **already reduces it past the `Eq` shape** -- for two matching
//! nullary constructors (`True`/`True`, once `fok_check_cert (...)` itself
//! reduces to `True`) it reduces all the way to `Top`, which `Refl` cannot
//! inhabit by construction (its own check requires the WHNF'd goal to still
//! be `Term::Eq`). `Proved` (`elab.globals["Proved"] = env.tt_id()`, the
//! canonical `Top` inhabitant) is the correct term, and the catalog's own
//! precedent agrees: `EmptyDec.ken.md`'s `any_proof_decides` proves `Equal
//! Bool True True` with `Proved`, never `Refl`. `Refl` proves equality at a
//! type whose equality stays primitive (a Π/function type, an abstract
//! postulated sort); `Bool`/`Nat`/`FokForm` are not such types.
//!
//! **This is exactly the class of correction this node has produced
//! repeatedly** (the `AC-2` framing error, the `D2` failure-encoding
//! question, the `D3` near-miss gap): a plausible restatement of an earlier
//! frame, carried forward unchecked, and wrong on contact with the actual
//! mechanism.
//!
//! **Why this does not weaken `AC-1`.** The expensive step -- reducing
//! `fok_check_cert (fok_embed f) pi` to WHNF -- happens INSIDE `whnf` on the
//! goal TYPE, via `eq_at_inductive`'s own `whnf(a)`/`whnf(b)` calls, before
//! either `Refl` or `Proved` is even considered. The choice of proof term
//! decides what inhabits the RESULT of that reduction; it does not decide
//! whether the reduction happens. `Proved` forces exactly the same kernel
//! conversion `Refl` would have, had it applied.
//!
//! # Demonstrating `AC-1`: the interval contains conversion and nothing else
//!
//! Two declarations are timed per case, sharing everything except whether
//! the kernel is asked to CONVERT the result to `True`:
//!
//! - **forced**: `theorem ok_<label> : Equal Bool (fok_check_cert (fok_embed
//!   <f>) <pi>) True = Proved` -- the kernel must reduce `fok_check_cert
//!   (fok_embed f) pi` to WHNF to type-check `Proved` against the
//!   (observationally-reduced) goal.
//! - **baseline**: `const ok_<label>_baseline : Bool = fok_check_cert
//!   (fok_embed <f>) <pi>` -- inferring an application's type is a lookup on
//!   the callee's declared return type (`Bool`), never a reduction of the
//!   body, so this needs no conversion at all.
//!
//! Both share parsing (identical `<f>`/`<pi>` source), name resolution, and
//! application-position type-checking; they differ only in whether the
//! kernel is asked to CONVERT the computed value to `True`. The measured
//! table below reports both, and their difference: the baseline is small
//! and near-flat across depth (confirming it does no reduction), while the
//! forced measurement grows with depth -- the growth is the conversion cost,
//! demonstrated by a controlled comparison rather than asserted from a
//! single number.
//!
//! # Corpus: the same real Ken source programs, same provenance
//!
//! Reuses `v3_fo_conversion_load_measured.rs`'s established `imp_chain`/
//! `forall_chain` generators verbatim in shape (duplicated here, not
//! imported -- each integration test file is its own compilation unit, the
//! established convention in this directory; `iform_source`/`form_source`/
//! `cert_source` are similarly duplicated from `V3-FO-KEN-LEVEL-CHECKER-
//! AUTHORING`'s own D0/D3 test file for the same reason). Every program is
//! genuine surface syntax through `elaborate_decl_v1`/`v2_extract`, exactly
//! as the predecessor's corpus was built -- never a hand-built `IForm`.
//!
//! `find_certificate`'s fuel is the fixed production constant, **`200`**
//! (`fo_kripke.rs`, `V3-FO-SEARCH-FUEL-STACK-AGREEMENT`'s measured value; not
//! configurable through the public API, so this measurement runs under it
//! unconditionally -- stated here per that node's own carried non-blocking
//! finding that a measurement's fuel must be written down to be re-takeable).
//!
//! # `D4`/`D5` result: kernel conversion is dramatically more expensive than
//! the Rust reference, and growth is super-linear -- the corpus is capped
//! honestly rather than pushed to the predecessor's depth-64 scale
//!
//! **Measured, debug profile, 1 GiB test thread (`AC-5` -- see below):**
//! wall-clock for `imp_chain` at depths 1/2/4/8 was approximately 3.1s /
//! 4.8s / 9.4s / 30.2s; `forall_chain` at depths 1/2/4 approximately 3s / 5s
//! / 11.8s (hand-probed before authoring the corpus below; the shipped
//! table is this file's own run and is the durable record). Growth
//! accelerates with depth (ratios roughly 1.5x, 2.0x, 3.2x per doubling for
//! `imp_chain`) -- distinctly worse than the Rust reference checker's
//! roughly-quadratic growth in `V3-FO-CONVERSION-LOAD-MEASURED`, and by
//! **four to five orders of magnitude** at matching depths (that node's
//! `imp_chain[8]` was 265 **microseconds**; this file's `imp_chain[8]` is
//! tens of **seconds**).
//!
//! **This node does not push to depth 64.** At the observed growth rate,
//! depth 16 was extrapolated to roughly one to two minutes and depth 32 to
//! plausibly tens of minutes -- a single-test-run cost that would itself
//! violate this repository's `--workspace`-forbidding resource discipline
//! (`COORDINATION §12`) if run routinely. **The corpus below is capped at
//! `imp_chain` depth 8 and `forall_chain` depth 4**, and this cap is the
//! honest `D5` result for reach: kernel conversion of this checker over
//! this Kripke theory is not tractable to measure at the predecessor's
//! scale on this hardware, and that intractability is reported as a finding
//! rather than worked around by silently shrinking the corpus without
//! saying so.
//!
//! **Termination: every case attempted terminated and was accepted. No
//! non-terminating or pathological case was found within the capped
//! corpus** -- the accelerating cost is a genuine, reported result, distinct
//! from non-termination.
//!
//! # `AC-5`: the stack is a harness artifact, demonstrated, not assumed
//!
//! **Confirmed empirically before authoring this corpus:** `imp_chain`
//! depth 2 aborts with a stack overflow on this test binary's DEFAULT 8 MiB
//! thread (`SIGABRT`, `"has overflowed its stack"`), and succeeds cleanly
//! once run on an oversized thread. This is the same class
//! `V3-FO-CONVERSION-LOAD-MEASURED`'s `D4` correction already established
//! for the elaborator/Rust-checker path (`run_with_big_stack`, reused
//! there from four sibling files in this directory) -- but kernel-level
//! reduction is markedly more stack-hungry than that path: the predecessor
//! needed 256 MiB; this file's `run_with_big_stack` uses **1 GiB**, sized
//! empirically against the actual depths measured (256 MiB was not
//! re-tried once 1 GiB was confirmed sufficient across the whole capped
//! corpus; if a future increment needs to know the exact boundary between
//! the two, that is unmeasured here). **The default-thread stack overflow
//! at depth 2 is a harness limit, not a mechanism property** -- every
//! program in the capped corpus terminates and is accepted once given
//! enough native stack, exactly the distinction `AC-5` requires.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{
    fo_kripke::{
        discover_and_quote_fo, find_certificate, Cert, Form, IForm, IVar, QTerm, Rule, Sequent,
    },
    v2_extract, ElabEnv,
};
use ken_kernel::{Level, Term};
use std::time::{Duration, Instant};

/// Kernel-level reduction is markedly more stack-hungry than the
/// interpreter/elaborator path `V3-FO-CONVERSION-LOAD-MEASURED` already
/// guards with `run_with_big_stack` (256 MiB there); confirmed empirically
/// that 1 GiB is sufficient for every depth this file measures (module doc,
/// `AC-5`).
fn run_with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(f)
        .expect("spawn big-stack test thread")
        .join()
        .expect("test thread panicked");
}

fn declare_fo_vocabulary(env: &mut ElabEnv, sort_name: &str, pred_name: &str) {
    env.declare_postulate_raw(sort_name, Term::Type(Level::zero()))
        .unwrap_or_else(|e| panic!("declare {sort_name}: {e:?}"));
    let sort_id = *env.globals.get(sort_name).unwrap();
    let sort_ty = Term::const_(sort_id, vec![]);
    env.declare_postulate_raw(pred_name, Term::pi(sort_ty, Term::Omega(Level::zero())))
        .unwrap_or_else(|e| panic!("declare {pred_name}: {e:?}"));
}

/// `N`-deep implication chain, verbatim in shape from
/// `v3_fo_conversion_load_measured.rs`.
fn imp_chain_source(name: &str, depth: usize) -> String {
    let chain = vec!["P x"; depth + 1].join(" -> ");
    format!("fn {name} (x : A) : A ensures {chain} = x")
}

/// `N`-deep forall-nesting, verbatim in shape from
/// `v3_fo_conversion_load_measured.rs`.
fn forall_chain_source(name: &str, depth: usize) -> String {
    let params: Vec<String> = (0..depth).map(|i| format!("(x{i} : A)")).collect();
    format!(
        "fn {name} {} : A ensures P x{} -> P x{} = x0",
        params.join(" "),
        depth - 1,
        depth - 1
    )
}

fn nat_source(n: usize) -> String {
    if n == 0 {
        "Zero".to_string()
    } else {
        format!("(Suc {})", nat_source(n - 1))
    }
}

fn qterm_source(q: &QTerm) -> String {
    match q {
        QTerm::Bound(i) => format!("(FokQBound {})", nat_source(*i)),
        QTerm::Parameter(i) => format!("(FokQParameter {})", nat_source(*i)),
    }
}

fn iform_source(f: &IForm) -> String {
    match f {
        IForm::Bottom => "FokIBottom".to_string(),
        IForm::Atom(IVar(k)) => format!("(FokIAtom (FokMkIVar {}))", nat_source(*k)),
        IForm::Or(p, q) => format!("(FokIOr {} {})", iform_source(p), iform_source(q)),
        IForm::Imp(p, q) => format!("(FokIImp {} {})", iform_source(p), iform_source(q)),
        IForm::Forall(p) => format!("(FokIForall {})", iform_source(p)),
    }
}

fn form_source(f: &Form) -> String {
    match f {
        Form::Bottom => "FokBottom".to_string(),
        Form::Access(a, b) => format!("(FokAccess {} {})", qterm_source(a), qterm_source(b)),
        Form::DomainA(a, b) => format!("(FokDomainA {} {})", qterm_source(a), qterm_source(b)),
        Form::ForcingP(a, b) => format!("(FokForcingP {} {})", qterm_source(a), qterm_source(b)),
        Form::And(p, q) => format!("(FokAnd {} {})", form_source(p), form_source(q)),
        Form::Or(p, q) => format!("(FokOr {} {})", form_source(p), form_source(q)),
        Form::Imp(p, q) => format!("(FokImp {} {})", form_source(p), form_source(q)),
        Form::ForallWorld(b) => format!("(FokForallWorld {})", form_source(b)),
        Form::ForallObj(b) => format!("(FokForallObj {})", form_source(b)),
    }
}

fn form_list_source(fs: &[Form]) -> String {
    match fs.split_first() {
        None => "(Nil FokForm)".to_string(),
        Some((head, rest)) => format!(
            "(Cons FokForm {} {})",
            form_source(head),
            form_list_source(rest)
        ),
    }
}

fn sequent_source(s: &Sequent) -> String {
    format!(
        "(FokMkSequent {} {})",
        form_list_source(&s.gamma),
        form_list_source(&s.delta)
    )
}

fn rule_source(r: &Rule) -> String {
    match r {
        Rule::Init { left, right } => {
            format!("(FokInit {} {})", nat_source(*left), nat_source(*right))
        }
        Rule::ImpRight { right } => format!("(FokImpRight {})", nat_source(*right)),
        Rule::ForallRight { right, eigen } => format!(
            // `D1`: eigen is a parameter INDEX; `FokForallRight` takes `Nat Nat`.
            "(FokForallRight {} {})",
            nat_source(*right),
            nat_source(*eigen)
        ),
    }
}

fn cert_source(c: &Cert) -> String {
    format!(
        "(FokMkCert {} {} {})",
        sequent_source(&c.conclusion),
        rule_source(&c.rule),
        cert_list_source(&c.children)
    )
}

fn cert_list_source(cs: &[Cert]) -> String {
    match cs.split_first() {
        None => "(Nil FokCert)".to_string(),
        Some((head, rest)) => format!(
            "(Cons FokCert {} {})",
            cert_source(head),
            cert_list_source(rest)
        ),
    }
}

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn count_cert_nodes(cert: &Cert) -> usize {
    1 + cert.children.iter().map(count_cert_nodes).sum::<usize>()
}

struct Measurement {
    label: String,
    formula_depth: usize,
    cert_node_count: usize,
    forced_wall_clock: Duration,
    baseline_wall_clock: Duration,
    outcome: &'static str,
}

/// One case, fully through the real pipeline: real Ken source ->
/// `elaborate_decl_v1`/`v2_extract` -> `discover_and_quote_fo` ->
/// `find_certificate` (Rust, fuel 200) -> independently serialize `f`/`pi`
/// to Ken source -> time the FORCED (`Proved`) and BASELINE (`Bool` const)
/// declarations, both through `ElabEnv::elaborate_decl` (real parsing, real
/// elaboration, real kernel type-checking -- the same path any Ken source
/// program goes through).
fn measure_one(env: &mut ElabEnv, label: &str, source: &str, formula_depth: usize) -> Measurement {
    let elab_res = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|e| panic!("{label}: source must elaborate, got {e:?}"));
    let ex = v2_extract(&elab_res);
    assert_eq!(
        ex.obligations.len(),
        1,
        "{label}: exactly one ensures obligation"
    );
    let phi_closed = &ex.obligations[0].goal_closed;

    let (_sig, problem) = discover_and_quote_fo(&env.env, phi_closed)
        .unwrap_or_else(|| panic!("{label}: discovery+quotation must succeed on this obligation"));
    let cert = find_certificate(&problem.f).unwrap_or_else(|| {
        panic!("{label}: this obligation is a tautology, a certificate must exist")
    });
    let cert_node_count = count_cert_nodes(&cert);

    let f_src = iform_source(&problem.f);
    let pi_src = cert_source(&cert);

    let forced_name = format!("fok_case_d4_{label}");
    let start = Instant::now();
    let forced = env.elaborate_decl(&format!(
        "theorem {forced_name} : Equal Bool (fok_check_cert (fok_embed {f_src}) {pi_src}) True = Proved"
    ));
    let forced_wall_clock = start.elapsed();
    forced.unwrap_or_else(|e| {
        panic!("{label}: forced (Proved) declaration must elaborate/kernel-check, got {e:?}")
    });

    let baseline_name = format!("fok_case_d4_{label}_baseline");
    let start2 = Instant::now();
    let baseline = env.elaborate_decl(&format!(
        "const {baseline_name} : Bool = fok_check_cert (fok_embed {f_src}) {pi_src}"
    ));
    let baseline_wall_clock = start2.elapsed();
    baseline.unwrap_or_else(|e| panic!("{label}: baseline declaration must elaborate, got {e:?}"));

    Measurement {
        label: label.to_string(),
        formula_depth,
        cert_node_count,
        forced_wall_clock,
        baseline_wall_clock,
        outcome: "terminated, accepted",
    }
}

/// `D4`+`D5`: measure kernel conversion of `fok_check_cert (fok_embed f)
/// pi = True` via `Proved` (module doc: why `Proved`, not `Refl`, and why
/// that substitution still forces the same conversion) on real Ken source
/// programs, capped honestly short of the predecessor's depth-64 scale
/// given the observed super-linear growth (module doc).
#[test]
fn measure_kernel_conversion_load_on_real_source_programs() {
    run_with_big_stack(|| {
        let mut env = ElabEnv::new().expect("base env");
        catalog_or::load_core_logic_or(&mut env);
        env.elaborate_file(FOK_SOURCE)
            .expect("FoKripke.ken failed to elaborate/kernel-check");
        declare_fo_vocabulary(&mut env, "A", "P");

        let mut measurements = Vec::new();

        // Axis 1: implication-chain depth. Capped at 8 -- module doc's D5.
        for depth in [1usize, 2, 4, 8] {
            let name = format!("imp_chain_{depth}");
            let source = imp_chain_source(&name, depth);
            measurements.push(measure_one(
                &mut env,
                &format!("imp_chain_{depth}"),
                &source,
                depth,
            ));
        }

        // Axis 2: forall-nesting depth (independent dimension). Capped at 4.
        for depth in [1usize, 2, 4] {
            let name = format!("forall_chain_{depth}");
            let source = forall_chain_source(&name, depth);
            measurements.push(measure_one(
                &mut env,
                &format!("forall_chain_{depth}"),
                &source,
                depth,
            ));
        }

        eprintln!("\n=== V3-FO-KEN-LEVEL-CHECKER-AUTHORING: D4-D5 report ===");
        eprintln!(
            "measured: KERNEL CONVERSION of fok_check_cert (fok_embed f) pi = True via Proved \
             (Ken-level, not the Rust reference) -- see this file's module doc for the Refl-vs-\
             Proved correction and the AC-1 forced-vs-baseline demonstration"
        );
        eprintln!("fuel: find_certificate's fixed production constant, 200 (not configurable)");
        eprintln!("stack: 1 GiB test thread (run_with_big_stack) -- AC-5, see module doc");
        eprintln!(
            "{:<16} {:>14} {:>12} {:>18} {:>18} {:>20}",
            "label", "formula_depth", "cert_nodes", "forced_us", "baseline_us", "outcome"
        );
        for m in &measurements {
            eprintln!(
                "{:<16} {:>14} {:>12} {:>18} {:>18} {:>20}",
                m.label,
                m.formula_depth,
                m.cert_node_count,
                m.forced_wall_clock.as_micros(),
                m.baseline_wall_clock.as_micros(),
                m.outcome
            );
        }
        let worst = measurements
            .iter()
            .max_by_key(|m| m.forced_wall_clock)
            .unwrap();
        eprintln!(
            "\nworst case (forced/conversion): {} at {} us (formula_depth={}, cert_nodes={})",
            worst.label,
            worst.forced_wall_clock.as_micros(),
            worst.formula_depth,
            worst.cert_node_count
        );
        eprintln!(
            "all {} runs terminated and were accepted; no pathological (non-terminating) case \
             found within the capped corpus (D5) -- the corpus is capped at imp_chain depth 8 / \
             forall_chain depth 4 due to observed super-linear growth, reported as a finding, \
             not silently worked around (see module doc)",
            measurements.len()
        );
    });
}
