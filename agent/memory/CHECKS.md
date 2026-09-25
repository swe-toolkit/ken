# Decision checks

The short list every seat reads at startup and applies at the moment of
decision: before posting a ruling, releasing a frame, approving a candidate,
or building against a premise. Each check is a recurring cause of hard stops.
The 2026-09-25 audit of the 20 latest advancing hard stops found a lesson
naming the cause for 16 of them. Ten of those lessons were in the responsible
seat's required reading, and the stop happened anyway.

**The rule is to run the check, not to recall it.** Its trigger names the
moment to run it. A check that passes is silent. One that fails is written
into the ruling, frame or review as a measured result.

**Hard limit: 12 checks, 100 lines.** To add a check, merge or replace one;
never append. A check earns its place by preventing a recurring kind of
stop, not by being true. The pointers lead to the full lessons in the
reference corpus (`README.md`).

1. **Two derivations agree.** Trace both to their inputs before counting
   the agreement as evidence. If one source minted both, the agreement is a
   single value checked against itself.
   (`fleet/a-pin-cannot-disagree-with-its-own-source.md`,
   `fleet/agreement-is-not-corroboration-when-a-premise-was-inherited.md`)
2. **A cited guard, refusal or choke point.** Resolve the arm it sits on.
   Then enumerate every path that reaches the property it protects (the
   fan-in), not only the path in front of you.
   (`fleet/a-pattern-match-is-evidence-about-what-encloses-it.md`)
3. **A consumer list or sweep.** Grep every source root (`catalog/`,
   `crates/*/tests`, `r_layer_tests`, `examples/`, `conformance/`, CLI
   fixtures), not the obvious ones, and sweep by mechanism as well as by
   name. A two-file grep is cheaper than a red CI.
   (`roles/steward/exported-name-migration-needs-whole-harness-consumer-inventory.md`,
   `build/rename-wp-needs-whole-workspace-basename-sweep.md`)
4. **An enabler or dependency assumed available.** Write the one
   declaration, obligation or state the plan needs, in the vocabulary the
   code actually delivers, before ruling or framing on it. This covers a
   syntax form, a reachable fixture state, and a primitive's output type.
   (`fleet/a-dependency-is-met-when-you-can-write-the-obligation.md`,
   `roles/steward/brief-settled-input-enabler-must-be-probed.md`)
5. **A rule keyed on a property.** Confirm that the plane you read actually
   carries that property. A names-only plane cannot classify by type,
   backing or provenance. Reading the property off another plane is a
   different claim. (`enclave/buildability-ruling-must-ground-every-axis.md`)
6. **A population placed inside a mechanism.** Count what actually arrives
   at the seam. Do not infer membership from behaviour, from one-pass
   execution, or from group shape. Apply your own counting demand to your own
   ruling. (`enclave/count-the-population-before-you-place-it-inside-a-mechanism.md`)
7. **A fix ruled against a counterexample.** Re-run that exact
   counterexample against the fix as narrowed and written, not the probe
   that motivated it. Then run the fix's criterion on the next consumer of
   the same kind. (`enclave/verify-proposed-fix-excludes-the-counterexample.md`,
   `fleet/a-fix-that-closes-the-named-counterexample-need-not-close-the-class.md`)
8. **A structural or proxy fact used as a gate or criterion.** Name what
   the failing configuration would produce. If it produces the same
   observation, the check is not evidence. Coverage, closure and "the
   refusal stays" are the usual proxies; a lawful repair can remove an
   incidental refusal.
   (`fleet/an-acceptance-criterion-must-name-an-observation-the-failing-configuration-does-not-also-produce.md`,
   `build/a-check-that-measures-a-proxy-passes-for-the-wrong-reason.md`)
9. **An axis declared controlled.** Every axis you exempt needs its own
   measurement. The axis the code dispatches on (lowering path, route) often
   moves with a visible one and cannot be seen in source.
   (`fleet/a-green-pilot-is-not-evidence-for-a-shape-it-never-produced.md`)
10. **A lookup by spelling.** A name in a mutable or flat global table can be
    overwritten by an unrelated declaration or constructor. Key on the checked
    identity, and apply that to every consumer (globals, classes, instances,
    desugaring), not just the one that failed.
11. **A Ken proof matching on a computed term.** A dependent `match` refines
    only the occurrences of its scrutinee as written, not terms convertible to
    it. A scrutinee reached through delta-unfolding or a helper stays neutral.
    Bind or destructure it first, and carry the other side with `cong`.
