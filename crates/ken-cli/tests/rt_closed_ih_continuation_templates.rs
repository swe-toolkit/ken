//! Checked host-Vis IH template census before the native Process gate.
//! Source: RT-NATIVE-IH-CONTINUATION-CLOSED-TEMPLATES AC-1 and Architect
//! CK-0. This observes the prepared, checked metadata; it does not enable
//! native Process operations or claim that a consumer resumes the K.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use ken_elaborator::compiler_driver::{prepare_native_program_sources, CompilerSource};
use ken_runtime::{
    ClosedIhContinuationRefusalV1, ClosedIhContinuationSetV1, OrientedSubcontinuationPlanV1,
    ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER,
};

const ARGV: &str = include_str!("fixtures/px10_argv_env_pipe.ken");
const LIVE: &str = include_str!("fixtures/px10_live_child.ken");

/// Promise class: transition sentinel for these three fixed fixtures. A new
/// checked IH template or a changed eliminator ordinal intentionally reds the
/// census for a fresh closed-set design review (including the later full
/// may-yield summary). MEASURED: the pre-gate prepared plan has six escaping
/// host-Vis method keys and the main#5 relay member. CLAIMED: these fixture
/// relays resolve to a checked K identity on the base branch. THE GAP: a
/// native consumer is not switched by this WP, so no executable parity is
/// inferred from this producer-side census.
// The D0-23/CK-0 preparation of these fixtures passed on a stated 16 MiB
// thread stack, an upper bound on the observed peak; the default 2 MiB test
// stack overflowed this same preparation. Provision 16 MiB plus 8 MiB
// headroom explicitly rather than inheriting RUST_MIN_STACK or ulimit.
const PREPARATION_STACK: usize = 24 * 1024 * 1024;

fn assert_fixture_census(package: &'static str, raw: String, arity_one: usize) {
    std::thread::Builder::new()
        .name(format!("closed-ih-{package}"))
        .stack_size(PREPARATION_STACK)
        .spawn(move || assert_fixture_census_on_thread(package, &raw, arity_one))
        .expect("spawn stated-stack checked preparation")
        .join()
        .expect("checked preparation must not panic");
}

fn assert_fixture_census_on_thread(package: &str, raw: &str, arity_one: usize) {
    let source = raw
        .replace(
            "PX10_CHILD_PROBE_PATH",
            env!("CARGO_BIN_EXE_px10-child-probe"),
        )
        .replace("PX10_READY_FILE", "closed-ih-census-ready");
    let roots =
        vec![std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let preparation = prepare_native_program_sources(
        package,
        vec![CompilerSource::new("main.ken", &source).with_roots(roots)],
    )
    .expect("checked Process preparation precedes the unavailable native gate");
    let plans: Vec<_> = preparation
        .runtime_program()
        .erased_core
        .metadata
        .checked_core
        .metadata
        .values()
        .filter(|bytes| bytes.starts_with(ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER))
        .collect();
    let [bytes] = plans.as_slice() else {
        panic!("expected exactly one oriented plan, found {}", plans.len());
    };
    let plan = OrientedSubcontinuationPlanV1::decode(bytes)
        .expect("prepared checked metadata carries the oriented plan");
    let host_vis = preparation.host_spine_vis();
    let set = ClosedIhContinuationSetV1::enumerate(&plan, &host_vis)
        .expect("each checked IH slot has a unique method key");
    let templates = set.templates().collect::<Vec<_>>();
    assert_eq!(
        templates.len(),
        6,
        "six distinct escaping Vis methods in this fixture"
    );
    let expected_declaration = format!("decl:{package}::main");
    for (ordinal, template) in (1..=6).zip(templates) {
        assert_eq!(template.key.declaration, expected_declaration);
        assert_eq!(template.key.checked_match_ordinal, ordinal);
        assert_eq!(template.key.constructor, host_vis);
        assert_eq!(template.key.method_binder_ordinal, 0);
        assert_eq!(template.recursive_position, 1);
        assert_eq!(template.escaping_calls.len(), 1);
        let call = template.escaping_calls[0];
        assert_eq!(set.arm(call), Ok(template));
        if ordinal == 5 {
            assert_eq!(
                plan.computational_ih_call(call).map(|c| c.arity),
                Some(0),
                "the relay's main#5 checked call escapes without arguments",
            );
        }
    }
    let refused = plan
        .computational_ih_calls
        .iter()
        .filter(|call| call.arity == 1)
        .collect::<Vec<_>>();
    assert_eq!(refused.len(), arity_one);
    assert_eq!(
        plan.computational_ih_calls.len(),
        6 + arity_one,
        "every checked IH call is a member's escaping call or a NotEscaping refusal",
    );
    for call in refused {
        assert_eq!(
            set.arm(call.call_template_id),
            Err(ClosedIhContinuationRefusalV1::NotEscaping),
            "the response-taking IH call is not the escaping K",
        );
    }
}

#[test]
fn argv_has_six_checked_host_vis_ih_templates_before_process_gate() {
    assert_fixture_census("rt_closedk_argv", ARGV.to_owned(), 2);
}

#[test]
fn null_stdout_has_six_checked_host_vis_ih_templates_before_process_gate() {
    let source = ARGV
        .replacen("        ChildPipe)", "        ChildNull)", 1)
        .replacen(
            "import Capability.Process.Child (ChildPipe,",
            "import Capability.Process.Child (ChildNull,",
            1,
        );
    assert!(
        source.contains("        ChildNull)"),
        "variant must change the child's stdout disposition"
    );
    assert_fixture_census("rt_closedk_v_null", source, 2);
}

#[test]
fn live_child_has_six_checked_host_vis_ih_templates_before_process_gate() {
    assert_fixture_census("rt_closedk_live", LIVE.to_owned(), 1);
}
