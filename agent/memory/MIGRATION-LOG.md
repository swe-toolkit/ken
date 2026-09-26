# Migration log — private memory to `agent/memory/`

Coverage audit for the migration of the shared Claude Code private memory
store (`/home/node/.claude/projects/-workspaces-ken/memory/*.md`, 165 lesson
files, `MEMORY.md` index excluded as a source) into the checked-in
`agent/memory/` corpus, ahead of the Codex harness switch. Every source file
appears exactly once below.

**Summary:** 135 kept &middot; 18 merged into a kept file &middot; 11 dropped &middot; 1 excluded (personal) &middot; 165 total sources.

| Source (former private memory) | Type | Disposition |
|---|---|---|
| `abstraction-visibility-feature-soundness-gate` | feedback | kept &rarr; `agent/memory/enclave/abstraction-visibility-feature-soundness-gate.md` |
| `architect-gate-can-be-skipped-review-on-main` | feedback | kept &rarr; `agent/memory/roles/architect/architect-gate-can-be-skipped-review-on-main.md` |
| `architect-verify-leader-actor-ids` | feedback | kept &rarr; `agent/memory/roles/architect/architect-verify-leader-actor-ids.md` |
| `assert-specific-error-variant-not-is-err` | feedback | kept &rarr; `agent/memory/build/assert-specific-error-variant-not-is-err.md` |
| `attribute-a-suite-arm-reject-before-calling-it-a-gap` | feedback | kept &rarr; `agent/memory/enclave/attribute-a-suite-arm-reject-before-calling-it-a-gap.md` |
| `bash-cd-main-repo-vs-steward-worktree` | feedback | kept &rarr; `agent/memory/roles/steward/bash-cd-main-repo-vs-steward-worktree.md` |
| `batched-plan-needs-explicit-self-continue-autonomy` | feedback | kept &rarr; `agent/memory/build/leaders/batched-plan-needs-explicit-self-continue-autonomy.md` |
| `buildability-claim-ground-every-axis` | feedback | merged &rarr; `agent/memory/enclave/buildability-ruling-must-ground-every-axis.md` |
| `buildability-classify-every-capability-axis` | feedback | merged &rarr; `agent/memory/enclave/buildability-ruling-must-ground-every-axis.md` |
| `buildability-ruling-must-ground-every-axis` | feedback | merged &rarr; `agent/memory/enclave/buildability-ruling-must-ground-every-axis.md` |
| `builtins-campaign-architect-design-lead` | project | dropped (reason: stale/superseded -- long BUILTINS campaign narrative (WP-status, evt_ ids, per-row verdicts already landed in the merged registry + /spec); its one durable reusable nugget (the eliminator-shadow rule) was extracted into roles/conformance-validator/builtins-tcb-audit-disciplines.md) |
| `builtins-tcb-audit-disciplines` | feedback | kept &rarr; `agent/memory/roles/conformance-validator/builtins-tcb-audit-disciplines.md` |
| `bundled-changes-need-per-mechanism-isolation-flip` | feedback | kept &rarr; `agent/memory/build/bundled-changes-need-per-mechanism-isolation-flip.md` |
| `bundled-frame-doc-goes-stale-when-mechanism-flips` | feedback | kept &rarr; `agent/memory/roles/steward/bundled-frame-doc-goes-stale-when-mechanism-flips.md` |
| `capability-gate-three-state-lifecycle` | feedback | kept &rarr; `agent/memory/enclave/capability-gate-three-state-lifecycle.md` |
| `capitalized-identifiers-never-scope-check` | project | kept &rarr; `agent/memory/teams/language/capitalized-identifiers-never-scope-check.md` |
| `carrier-canonicity-axis-for-lawful-class-laws` | feedback | kept &rarr; `agent/memory/enclave/carrier-canonicity-axis-for-lawful-class-laws.md` |
| `cast-direction-test-at-nondegenerate-endpoints` | feedback | kept &rarr; `agent/memory/teams/kernel/cast-direction-test-at-nondegenerate-endpoints.md` |
| `cat2-five-fork-rulings-wire-attested-abstractg-rowvar` | reference | dropped (reason: in-spec -- CAT-2 design rulings landed in spec chapters ~55-58; reusable reasoning nuggets independently captured in enclave/coexist-over-subsume-when-trust-levels-differ.md, enclave/higher-kinded-class-param-and-funext-definitional.md, teams/language/effect-row-polymorphism-machinery-landed-gap-is-surface.md) |
| `cat3-three-fork-rulings-perm-optic-name` | reference | dropped (reason: in-spec -- CAT-3 design rulings landed in spec chapters ~55-58; reusable nugget independently captured in enclave/proof-relevant-inductive-cannot-be-declared-at-omega.md) |
| `cat4-three-fork-rulings-union-closure-relation` | reference | dropped (reason: in-spec -- CAT-4 design rulings landed in spec chapters ~55-58; reusable nuggets independently captured in enclave/proof-relevant-inductive-cannot-be-declared-at-omega.md and fleet/grep-rust-prelude-emission-for-landedness.md) |
| `cbv-eliminator-method-laziness` | feedback | kept &rarr; `agent/memory/enclave/cbv-eliminator-method-laziness.md` |
| `check-main-via-git-object-store-not-find` | feedback | kept &rarr; `agent/memory/fleet/check-main-via-git-object-store-not-find.md` |
| `class-dict-explicit-vs-implicit-abstract-tyvar` | feedback | kept &rarr; `agent/memory/enclave/class-dict-explicit-vs-implicit-abstract-tyvar.md` |
| `cleanroom-is-role-discipline-not-host` | feedback | kept &rarr; `agent/memory/fleet/cleanroom-is-role-discipline-not-host.md` |
| `coexist-over-subsume-when-trust-levels-differ` | feedback | kept &rarr; `agent/memory/enclave/coexist-over-subsume-when-trust-levels-differ.md` |
| `compact-verify-survey-can-eat-the-compact-command` | feedback | kept &rarr; `agent/memory/fleet/compact-verify-survey-can-eat-the-compact-command.md` |
| `compact-wiped-memory-reflog-first` | feedback | kept &rarr; `agent/memory/fleet/compact-wiped-memory-reflog-first.md` |
| `compaction-is-manual-no-clean-seam` | feedback | kept &rarr; `agent/memory/fleet/compaction-is-manual-no-clean-seam.md` |
| `compaction-render-delay-escape-aborts` | feedback | kept &rarr; `agent/memory/fleet/compaction-render-delay-escape-aborts.md` |
| `composition-wp-real-producer-may-be-deferred-engine` | feedback | kept &rarr; `agent/memory/build/qa/composition-wp-real-producer-may-be-deferred-engine.md` |
| `conformance-assert-at-locked-granularity` | feedback | kept &rarr; `agent/memory/enclave/conformance-assert-at-locked-granularity.md` |
| `conformance-hand-feeds-the-deliverable` | feedback | kept &rarr; `agent/memory/build/qa/conformance-hand-feeds-the-deliverable.md` |
| `conformance-oracle-grounding-fallback` | reference | kept &rarr; `agent/memory/roles/conformance-validator/conformance-oracle-grounding-fallback.md` |
| `conformance-reconcile-inherits-spec-metatheory-bugs` | feedback | kept &rarr; `agent/memory/enclave/conformance-reconcile-inherits-spec-metatheory-bugs.md` |
| `conformance-validator-casts-spec-review-vote` | feedback | kept &rarr; `agent/memory/roles/conformance-validator/conformance-validator-casts-spec-review-vote.md` |
| `contract-spec-defer-spelling-not-concept` | feedback | kept &rarr; `agent/memory/enclave/contract-spec-defer-spelling-not-concept.md` |
| `conv-reduction-arm-gate-needs-termination-stress` | feedback | kept &rarr; `agent/memory/teams/kernel/conv-reduction-arm-gate-needs-termination-stress.md` |
| `convo-mention-id-must-be-grepped-not-typed` | feedback | merged &rarr; `agent/memory/fleet/mention-discipline.md` |
| `correcting-scope-must-sweep-whole-doc` | feedback | kept &rarr; `agent/memory/fleet/correcting-scope-must-sweep-whole-doc.md` |
| `credit-window-reserve-opus-for-t1` | project | dropped (reason: stale -- transient operational/credit-window state from 2026-07-04, not durable) |
| `deceq-on-noncanonical-carrier-inhabits-bottom` | reference | kept &rarr; `agent/memory/enclave/deceq-on-noncanonical-carrier-inhabits-bottom.md` |
| `delivery-contract-op-list-can-overscope` | feedback | kept &rarr; `agent/memory/roles/conformance-validator/delivery-contract-op-list-can-overscope.md` |
| `dependent-match-construction-fails-closed-via-infer-elim` | reference | kept &rarr; `agent/memory/teams/kernel/dependent-match-construction-fails-closed-via-infer-elim.md` |
| `differential-verify-which-mechanism-is-the-net` | feedback | kept &rarr; `agent/memory/enclave/differential-verify-which-mechanism-is-the-net.md` |
| `disclaimed-framing-still-binds-your-own-companion-artifact` | feedback | kept &rarr; `agent/memory/enclave/disclaimed-framing-still-binds-your-own-companion-artifact.md` |
| `discriminating-axis-vacuous-until-capability-lands` | feedback | kept &rarr; `agent/memory/enclave/discriminating-axis-vacuous-until-capability-lands.md` |
| `discriminating-conformance-verdict-must-flip` | feedback | kept &rarr; `agent/memory/enclave/discriminating-conformance-verdict-must-flip.md` |
| `discriminating-flip-must-be-checked-per-test` | feedback | kept &rarr; `agent/memory/build/qa/discriminating-flip-must-be-checked-per-test.md` |
| `discriminator-negative-arm-must-be-expressible-and-reaching` | feedback | kept &rarr; `agent/memory/build/qa/discriminator-negative-arm-must-be-expressible-and-reaching.md` |
| `dont-preempt-technical-fork-with-sequencing` | feedback | kept &rarr; `agent/memory/enclave/dont-preempt-technical-fork-with-sequencing.md` |
| `effect-row-polymorphism-machinery-landed-gap-is-surface` | reference | kept &rarr; `agent/memory/teams/language/effect-row-polymorphism-machinery-landed-gap-is-surface.md` |
| `eliminator-termination-finiteness-not-stuckness` | feedback | kept &rarr; `agent/memory/enclave/eliminator-termination-finiteness-not-stuckness.md` |
| `enclave-does-not-pull-in-build-leads` | feedback | merged &rarr; `agent/memory/enclave/enclave-elaborates-autonomously-no-build-team-pulls.md` |
| `enclave-elaborates-autonomously-no-build-team-pulls` | feedback | merged &rarr; `agent/memory/enclave/enclave-elaborates-autonomously-no-build-team-pulls.md` |
| `enclave-ruling-in-thread-is-not-a-durable-deliverable` | feedback | kept &rarr; `agent/memory/enclave/enclave-ruling-in-thread-is-not-a-durable-deliverable.md` |
| `enriching-opaque-former-kind-is-kernel-clean` | reference | kept &rarr; `agent/memory/teams/kernel/enriching-opaque-former-kind-is-kernel-clean.md` |
| `es4-ac3-omega-elimination-kernel-blocker` | project | dropped (reason: stale -- RESOLVED historical WP narrative (K4), fix landed in kernel + spec; superseded by landed code) |
| `eval-only-harness-cant-detect-phantom-arg-staleness` | feedback | kept &rarr; `agent/memory/build/eval-only-harness-cant-detect-phantom-arg-staleness.md` |
| `eval-store-resync-recurring-trap` | project | kept &rarr; `agent/memory/build/implementers/eval-store-resync-recurring-trap.md` |
| `fleet-model-rollout-stagger-restarts` | feedback | kept &rarr; `agent/memory/roles/steward/fleet-model-rollout-stagger-restarts.md` |
| `fold-in-doc-only-conditions-while-holding-branch` | feedback | kept &rarr; `agent/memory/build/qa/fold-in-doc-only-conditions-while-holding-branch.md` |
| `gate-widening-exposes-latent-bugs-in-newly-reachable-code` | project | kept &rarr; `agent/memory/teams/kernel/gate-widening-exposes-latent-bugs-in-newly-reachable-code.md` |
| `general-fix-can-conflate-similar-shaped-different-cases` | project | kept &rarr; `agent/memory/build/general-fix-can-conflate-similar-shaped-different-cases.md` |
| `git-author-not-per-agent` | feedback | kept &rarr; `agent/memory/fleet/git-author-not-per-agent.md` |
| `green-vs-green-does-not-confirm-a-fix` | project | kept &rarr; `agent/memory/build/green-vs-green-does-not-confirm-a-fix.md` |
| `grep-ken-sources-misses-rust-emitted-prelude` | feedback | merged &rarr; `agent/memory/fleet/grep-rust-prelude-emission-for-landedness.md` |
| `grep-the-producer-not-the-cited-proxy` | feedback | kept &rarr; `agent/memory/fleet/grep-the-producer-not-the-cited-proxy.md` |
| `grounding-a-fabricated-citation-two-failure-modes` | feedback | kept &rarr; `agent/memory/enclave/grounding-a-fabricated-citation-two-failure-modes.md` |
| `hand-built-elim-motive-and-method-gotchas` | project | kept &rarr; `agent/memory/teams/kernel/hand-built-elim-motive-and-method-gotchas.md` |
| `handoff-is-not-done-review-loop-on-my-spec` | feedback | kept &rarr; `agent/memory/roles/spec-author/handoff-is-not-done-review-loop-on-my-spec.md` |
| `handoff-scope-count-must-match-full-thread` | feedback | kept &rarr; `agent/memory/fleet/handoff-scope-count-must-match-full-thread.md` |
| `held-branch-scaffolding-is-load-bearing-evidence` | project | kept &rarr; `agent/memory/fleet/held-branch-scaffolding-is-load-bearing-evidence.md` |
| `higher-kinded-class-param-and-funext-definitional` | reference | kept &rarr; `agent/memory/enclave/higher-kinded-class-param-and-funext-definitional.md` |
| `isolate-executed-vs-present-before-naming-perf-cause` | project | kept &rarr; `agent/memory/build/qa/isolate-executed-vs-present-before-naming-perf-cause.md` |
| `isolate-mechanism-from-orthogonal-fail-closed-gates` | feedback | kept &rarr; `agent/memory/build/qa/isolate-mechanism-from-orthogonal-fail-closed-gates.md` |
| `k5-top-bottom-intro-elim-kernel-gap` | project | merged &rarr; `agent/memory/teams/kernel/kernel-completeness-gap-shapes.md (+ trust-root-reduction-change-needs-full-workspace-gate.md for K7's review-discipline finding)` |
| `k6-conv-struct-eq-congruence-gap` | project | merged &rarr; `agent/memory/teams/kernel/kernel-completeness-gap-shapes.md (+ trust-root-reduction-change-needs-full-workspace-gate.md for K7's review-discipline finding)` |
| `k7-eq-at-inductive-operand-whnf-gap` | project | merged &rarr; `agent/memory/teams/kernel/kernel-completeness-gap-shapes.md (+ trust-root-reduction-change-needs-full-workspace-gate.md for K7's review-discipline finding)` |
| `ken-cargo-build-lock-wedge` | reference | kept &rarr; `agent/memory/build/ken-cargo-build-lock-wedge.md` |
| `kernel-backed-claim-grep-the-emission-not-the-name` | feedback | kept &rarr; `agent/memory/enclave/kernel-backed-claim-grep-the-emission-not-the-name.md` |
| `kernel-backed-obligation-certificate-vs-discrimination` | reference | kept &rarr; `agent/memory/enclave/kernel-backed-obligation-certificate-vs-discrimination.md` |
| `kernel-rejects-is-completeness-fix-is-where-soundness-converts` | feedback | kept &rarr; `agent/memory/enclave/kernel-rejects-is-completeness-fix-is-where-soundness-converts.md` |
| `keyword-purity-agreement-fn-proc` | feedback | dropped (reason: in-spec -- const/fn/proc keyword-purity split is landed in spec/30-surface/{31,32,33,36} (verified via grep)) |
| `laundered-citation-authority` | feedback | kept &rarr; `agent/memory/enclave/laundered-citation-authority.md` |
| `lawful-class-instances-must-carry-law-proofs` | project | dropped (reason: superseded -- long ES4-lawproofs/lawful-classes campaign narrative; durable findings independently distilled and kept in enclave/carrier-canonicity-axis-for-lawful-class-laws.md and enclave/deceq-on-noncanonical-carrier-inhabits-bottom.md) |
| `layer-dependent-pin-at-unconditional-layer` | reference | kept &rarr; `agent/memory/enclave/layer-dependent-pin-at-unconditional-layer.md` |
| `leader-relays-frame-citations-must-reverify-too` | feedback | kept &rarr; `agent/memory/roles/spec-leader/leader-relays-frame-citations-must-reverify-too.md` |
| `leader-since-window-blindness-on-decision-votes` | feedback | kept &rarr; `agent/memory/build/leaders/leader-since-window-blindness-on-decision-votes.md` |
| `live-review-candidate-goes-stale-reanchor-sha` | feedback | kept &rarr; `agent/memory/fleet/live-review-candidate-goes-stale-reanchor-sha.md` |
| `llm-proxy-is-build-tier-only-anthropic-runs-direct` | reference | kept &rarr; `agent/memory/roles/steward/llm-proxy-is-build-tier-only-anthropic-runs-direct.md` |
| `markdown-80col-reflow-gotchas` | feedback | merged &rarr; `agent/memory/fleet/markdown-80col-reflow.md` |
| `mention-iff-question-or-action-no-ack-mentions` | feedback | merged &rarr; `agent/memory/fleet/mention-discipline.md` |
| `mention-only-for-question-or-action` | feedback | merged &rarr; `agent/memory/fleet/mention-discipline.md` |
| `mentions-field-needs-literal-mention-text-too` | feedback | merged &rarr; `agent/memory/fleet/mention-discipline.md` |
| `merge-ready-sent-is-a-race-boundary` | feedback | kept &rarr; `agent/memory/roles/spec-leader/merge-ready-sent-is-a-race-boundary.md` |
| `mootup-posting-from-agent` | reference | kept &rarr; `agent/memory/fleet/mootup-posting-from-agent.md` |
| `multi-piece-erratum-landing-integrity` | feedback | merged &rarr; `agent/memory/roles/integrator/multi-piece-erratum-verify-all-on-main.md` |
| `multi-worktree-cwd-drift-phantom-diff` | feedback | kept &rarr; `agent/memory/fleet/multi-worktree-cwd-drift-phantom-diff.md` |
| `multipiece-erratum-verify-all-on-main` | feedback | merged &rarr; `agent/memory/roles/integrator/multi-piece-erratum-verify-all-on-main.md` |
| `my-own-tracker-capability-landed-line-can-be-stale` | feedback | kept &rarr; `agent/memory/roles/steward/my-own-tracker-capability-landed-line-can-be-stale.md` |
| `named-floor-must-be-grepped-not-assumed` | feedback | kept &rarr; `agent/memory/build/named-floor-must-be-grepped-not-assumed.md` |
| `negative-landed-claim-grep-the-rust-prelude-emission` | feedback | merged &rarr; `agent/memory/fleet/grep-rust-prelude-emission-for-landedness.md` |
| `obligation-must-descend-into-structure` | feedback | kept &rarr; `agent/memory/enclave/obligation-must-descend-into-structure.md` |
| `operator-timezone-pdt` | user | excluded-personal |
| `opus-enclave-authors-shovel-ready-wps` | feedback | dropped (reason: superseded -- T1/T2 tiering + shovel-ready-WP division of labor now documented in CLAUDE.md, agent/MODELS.md, and agent/COORDINATION.md) |
| `orphan-watchdog-timer-record-id` | feedback | kept &rarr; `agent/memory/roles/steward/orphan-watchdog-timer-record-id.md` |
| `orphaned-background-task-loops-leak-cpu` | project | kept &rarr; `agent/memory/fleet/orphaned-background-task-loops-leak-cpu.md` |
| `package-ecosystem-comprehensive-standard-small-contrib` | project | kept &rarr; `agent/memory/enclave/package-ecosystem-comprehensive-standard-small-contrib.md` |
| `pane-suggestion-text-is-not-agent-state` | feedback | kept &rarr; `agent/memory/fleet/pane-suggestion-text-is-not-agent-state.md` |
| `perf-primitive-vs-fix-the-evaluator-fork` | feedback | kept &rarr; `agent/memory/enclave/perf-primitive-vs-fix-the-evaluator-fork.md` |
| `playbooks-state-mechanism-not-intent` | feedback | kept &rarr; `agent/memory/roles/steward/playbooks-state-mechanism-not-intent.md` |
| `probe-recursion-depth-before-writing-the-real-test` | project | kept &rarr; `agent/memory/build/probe-recursion-depth-before-writing-the-real-test.md` |
| `proof-relevant-inductive-cannot-be-declared-at-omega` | feedback | kept &rarr; `agent/memory/enclave/proof-relevant-inductive-cannot-be-declared-at-omega.md` |
| `re-read-latest-events-immediately-before-a-stall-nudge` | feedback | kept &rarr; `agent/memory/fleet/re-read-latest-events-immediately-before-a-stall-nudge.md` |
| `reason-in-agent-team-hours-not-human-days` | feedback | kept &rarr; `agent/memory/fleet/reason-in-agent-team-hours-not-human-days.md` |
| `reconcile-binds-a-co-reviewers-plausible-reading-too` | feedback | kept &rarr; `agent/memory/enclave/reconcile-binds-a-co-reviewers-plausible-reading-too.md` |
| `reconcile-own-over-claim-then-grep-coupled` | feedback | kept &rarr; `agent/memory/enclave/reconcile-own-over-claim-then-grep-coupled.md` |
| `reconcile-proof-rides-elaboration-merge-not-build-phase` | feedback | kept &rarr; `agent/memory/enclave/reconcile-proof-rides-elaboration-merge-not-build-phase.md` |
| `resource-blowup-on-small-code-is-a-checker-bug` | feedback | kept &rarr; `agent/memory/enclave/resource-blowup-on-small-code-is-a-checker-bug.md` |
| `reviewers-review-branches-not-prs` | feedback | kept &rarr; `agent/memory/fleet/reviewers-review-branches-not-prs.md` |
| `safe-reflow-space-substitution-only` | reference | merged &rarr; `agent/memory/fleet/markdown-80col-reflow.md` |
| `scope-review-vote-to-my-lane` | feedback | kept &rarr; `agent/memory/enclave/scope-review-vote-to-my-lane.md` |
| `sct-completeness-fix-conservative-construction` | feedback | kept &rarr; `agent/memory/teams/kernel/sct-completeness-fix-conservative-construction.md` |
| `sct-unapplied-self-reference-over-accepts` | feedback | kept &rarr; `agent/memory/teams/kernel/sct-unapplied-self-reference-over-accepts.md` |
| `sigma-sort-pi-vs-sigma-over-equating` | feedback | kept &rarr; `agent/memory/enclave/sigma-sort-pi-vs-sigma-over-equating.md` |
| `sizing-a-subsume-fix-enumerate-every-piece` | feedback | kept &rarr; `agent/memory/enclave/sizing-a-subsume-fix-enumerate-every-piece.md` |
| `sole-net-prefer-structural-self-evidence-over-positional-scalar` | feedback | kept &rarr; `agent/memory/enclave/sole-net-prefer-structural-self-evidence-over-positional-scalar.md` |
| `soundness-AC-static-vs-runtime-face` | feedback | kept &rarr; `agent/memory/enclave/soundness-AC-static-vs-runtime-face.md` |
| `spec-claim-kernel-admittance-vs-staging` | feedback | kept &rarr; `agent/memory/enclave/spec-claim-kernel-admittance-vs-staging.md` |
| `spec-conv-omega-shortcut-trap` | feedback | kept &rarr; `agent/memory/enclave/spec-conv-omega-shortcut-trap.md` |
| `spec-enclave-always-compact-before-new-work` | feedback | kept &rarr; `agent/memory/enclave/spec-enclave-always-compact-before-new-work.md` |
| `spec-example-must-satisfy-its-own-rules` | feedback | kept &rarr; `agent/memory/enclave/spec-example-must-satisfy-its-own-rules.md` |
| `spelling-currency-sweep-separate-from-vacuity` | feedback | kept &rarr; `agent/memory/enclave/spelling-currency-sweep-separate-from-vacuity.md` |
| `steward-coldstart-infra-checks` | feedback | kept &rarr; `agent/memory/roles/steward/steward-coldstart-infra-checks.md` |
| `steward-must-relay-merges-integrator-notifies-only-steward` | feedback | kept &rarr; `agent/memory/roles/steward/steward-must-relay-merges-integrator-notifies-only-steward.md` |
| `structural-reachability-beats-empirical-probe-for-dead-code-fix` | feedback | kept &rarr; `agent/memory/build/qa/structural-reachability-beats-empirical-probe-for-dead-code-fix.md` |
| `surface-the-seam-need-not-your-preferred-mechanism` | feedback | kept &rarr; `agent/memory/fleet/surface-the-seam-need-not-your-preferred-mechanism.md` |
| `systems-os-kernel-interface-first-party` | project | kept &rarr; `agent/memory/enclave/systems-os-kernel-interface-first-party.md` |
| `taint-axis-orientation-needs-distinguishing-pair` | feedback | kept &rarr; `agent/memory/build/qa/taint-axis-orientation-needs-distinguishing-pair.md` |
| `terminal-gate-resolve-race-resolving-on-cast` | feedback | kept &rarr; `agent/memory/fleet/terminal-gate-resolve-race-resolving-on-cast.md` |
| `tested-not-trusted-posture-needs-reachability-precondition` | feedback | kept &rarr; `agent/memory/enclave/tested-not-trusted-posture-needs-reachability-precondition.md` |
| `thin-flow-directive-dont-cc-the-room` | feedback | kept &rarr; `agent/memory/fleet/thin-flow-directive-dont-cc-the-room.md` |
| `timeout-does-not-kill-grandchild-cargo-test` | project | kept &rarr; `agent/memory/build/timeout-does-not-kill-grandchild-cargo-test.md` |
| `transcription-moves-contract-requires-three-part-reconcile` | feedback | kept &rarr; `agent/memory/enclave/transcription-moves-contract-requires-three-part-reconcile.md` |
| `transport-schema-degenerate-endpoint-trap` | feedback | kept &rarr; `agent/memory/enclave/transport-schema-degenerate-endpoint-trap.md` |
| `trust-level-claim-grep-per-check-both-directions` | feedback | kept &rarr; `agent/memory/enclave/trust-level-claim-grep-per-check-both-directions.md` |
| `trust-level-prose-vs-locked-adr-crosscheck` | feedback | kept &rarr; `agent/memory/enclave/trust-level-prose-vs-locked-adr-crosscheck.md` |
| `trust-root-test-coverage-discipline` | feedback | kept &rarr; `agent/memory/teams/kernel/trust-root-test-coverage-discipline.md` |
| `trusted-by-typing-guarantee-is-not-kernel-proved-Q` | project | kept &rarr; `agent/memory/enclave/trusted-by-typing-guarantee-is-not-kernel-proved-Q.md` |
| `trusted-primitive-refinement-codomain-witness` | feedback | kept &rarr; `agent/memory/enclave/trusted-primitive-refinement-codomain-witness.md` |
| `tt-vs-refl-endpoint-rule-for-inductive-equal-law-bases` | feedback | kept &rarr; `agent/memory/enclave/tt-vs-refl-endpoint-rule-for-inductive-equal-law-bases.md` |
| `two-arm-producer-needs-a-case-per-arm` | feedback | kept &rarr; `agent/memory/build/qa/two-arm-producer-needs-a-case-per-arm.md` |
| `untrusted-layer-backstop-hole-for-omissions` | feedback | kept &rarr; `agent/memory/enclave/untrusted-layer-backstop-hole-for-omissions.md` |
| `use-tier-labels-never-model-names` | feedback | kept &rarr; `agent/memory/fleet/use-tier-labels-never-model-names.md` |
| `verdict-mapping-silence-is-a-latent-conformance-bug` | feedback | kept &rarr; `agent/memory/enclave/verdict-mapping-silence-is-a-latent-conformance-bug.md` |
| `verified-showcase-predicate-must-be-defined-not-postulated` | feedback | kept &rarr; `agent/memory/enclave/verified-showcase-predicate-must-be-defined-not-postulated.md` |
| `verify-field-order-arity-against-declaration-not-prose` | feedback | kept &rarr; `agent/memory/fleet/verify-field-order-arity-against-declaration-not-prose.md` |
| `verify-proposed-fix-excludes-the-counterexample` | feedback | kept &rarr; `agent/memory/enclave/verify-proposed-fix-excludes-the-counterexample.md` |
| `verify-symbol-exposure-not-just-call-site-safety` | feedback | kept &rarr; `agent/memory/enclave/verify-symbol-exposure-not-just-call-site-safety.md` |
| `wp-branch-handoff-deadlock-leader-holds` | project | kept &rarr; `agent/memory/build/leaders/wp-branch-handoff-deadlock-leader-holds.md` |
| `wp-frame-stale-vs-landed-kernel` | feedback | kept &rarr; `agent/memory/enclave/wp-frame-stale-vs-landed-kernel.md` |
| `wp-release-process-steward-spec-build` | project | dropped (reason: superseded -- Steward-frame to spec-leader-elaboration to build-team release pipeline now documented in CLAUDE.md, agent/COORDINATION.md, and agent/playbooks/federation/integrator.md + agent/playbooks/spec/leader.md) |
| `ws-sec-build-owned-by-verify` | project | dropped (reason: superseded (FLAG for operator review) -- 2026-06-30 memory says WS-Sec build routes solely to Team Verify, but current agent/README.md documents security as cross-cutting across Language+Foundation+Kernel with Architect review, Verify owning only the WS-B behavioral seam; org-chart appears to have evolved and the two sources now conflict) |
| `zonk_term-must-be-exhaustive-over-term-variants` | project | kept &rarr; `agent/memory/teams/kernel/exhaustive-term-traversals.md` |

## 2026-09-26: the regrown private store, triaged once (R4)

The operator adopted research recommendation R4 (2026-09-26): retire the
private Claude Code store as a second corpus. The July migration above left
it as the single home for nothing, but it had regrown to 1,262 files
(8.1 MB) written by every Claude Code seat on the machine, unreviewed and
invisible to pi and Codex seats. Two changes land together:

- `.claude/settings.json` sets `"autoMemoryEnabled": false`, so Claude Code
  no longer loads or writes the private store for this project.
- Every file in the store was triaged exactly once. The per-file list
  (source, disposition, target or reason) is
  `agent/memory/migration-2026-09-26.tsv`. The store itself was not edited;
  the Steward archives it after this lands.

**Summary:** 719 DUP &middot; 35 PROMOTE &middot; 45 OPERATOR &middot; 463
DROP &middot; 1,262 total sources.

Method. Each non-index file was read in full and matched against the
checked-in corpus by mechanism (`scripts/memory-search`, plus a text
similarity pass), not by filename, because promotion renames a lesson. The
169 files sharing a filename with a checked-in lesson were word-diffed
against it. The 31 other July sources still present kept their July
disposition (19 DUP, 6 DROP), except five operator rulings the July pass
dropped, which are now listed as OPERATOR beside the July-excluded identity
file.

- **DUP (719).** The lesson already exists in `agent/memory/` under some
  name, or is one of the `CHECKS.md` checks (87 files). This includes the
  169 same-name copies, 19 July sources, and 10 files merged into a promoted
  lesson below. Drift that still held a true, general point missing from the
  checked-in copy was folded into it (8 files, listed below); the other 161
  same-name copies differed only by the depersonalizing and formatting edits
  the checked-in copies had already received.
- **PROMOTE (35).** 32 new lessons, and 3 extensions of existing ones.
  Selection kept only mechanisms that no lesson or check covers, that name
  only things that still exist, and that generalize. The first preference was
  lessons already cited by a checked-in file under the private slug, which
  were dangling links. Sibling private files on one mechanism were merged
  into one lesson.
- **OPERATOR (45).** Operator rulings, standing instructions, identity, and
  operator task state. None were placed in the corpus; they were listed for
  the Steward with the operator's words, date, and whether each looks in
  force.
- **DROP (463).** The 15 index files (`MEMORY.md` and 14 sub-indexes);
  campaign, node and incident narratives whose general point is covered
  loosely or not at all; stale live state; and 46 lessons proposed for
  promotion during triage and declined at selection, each with its reason in
  the TSV. One declined lesson was false as stated
  (`git rev-parse <sha>:<bad path>` exits 128; it does not print the commit
  SHA).

New lessons:

| Lesson | Merged sources |
|---|---|
| `fleet/a-boundary-claim-needs-a-measurement-at-every-position-it-names.md` | |
| `fleet/a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less.md` | |
| `fleet/a-credential-in-your-own-home-directory-can-belong-to-someone-else.md` | |
| `fleet/a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named.md` | `a-resource-exhaustion-failure-may-be-a-deleted-guard-not-new-code` |
| `fleet/a-guard-on-what-the-command-says-is-not-a-guard-on-what-the-command-does.md` | |
| `fleet/a-hedge-does-not-constrain-what-is-done-with-the-claim-it-hedges.md` | `discounting-evidence-weight-is-not-declining-to-assert-it` |
| `fleet/a-measurement-correct-on-one-tree-becomes-a-cross-tree-claim-only-through-a-carry-argument.md` | |
| `fleet/a-mutation-revert-restores-the-last-commit-not-the-pre-mutation-file.md` | `commit-real-fix-before-any-mutation-proof-reset`, `committing-before-a-mutation-campaign-does-not-protect-work-added-during-it` |
| `fleet/a-search-run-after-a-fix-lands-includes-the-fix.md` | |
| `fleet/a-stale-number-that-drifts-into-being-correct-cannot-be-caught-by-checking-it.md` | |
| `fleet/a-waiver-covers-only-the-issuers-own-gates.md` | `a-waiver-reaches-only-as-far-as-what-the-waiver-holder-owns` |
| `fleet/a-withdrawal-is-not-delivered-by-being-posted.md` | |
| `fleet/an-instruction-not-to-recheck-is-what-makes-a-false-negative-durable.md` | |
| `fleet/an-instruction-to-close-is-not-evidence-the-work-behind-it-is-done.md` | |
| `fleet/an-instrument-that-reports-a-verdict-cannot-distinguish-inapplicable-from-false.md` | `zero-executions-and-never-called-are-two-facts-one-number` |
| `fleet/landing-a-config-change-on-main-does-not-apply-it-the-consumer-reads-a-working-tree.md` | |
| `fleet/never-complete-an-abbreviated-sha-cite-rev-parse.md` | |
| `fleet/prefer-the-number-the-producer-already-emits-over-one-you-count-yourself.md` | |
| `fleet/provider-content-refusal-is-a-distinct-stall-class.md` | |
| `fleet/the-case-a-rule-was-written-for-is-the-one-it-is-never-tested-against.md` | |
| `build/cargo-check-and-cargo-test-compile-different-code.md` | `cargo-check-reports-zero-while-cfg-test-sites-remain`, `cfg-test-build-asymmetry-is-bidirectional`, `a-cfg-test-reexport-is-a-production-only-red-the-test-profile-cannot-see` |
| `build/crate-wide-fmt-plus-git-add-A-smuggles-unrelated-files-into-a-scoped-diff.md` | |
| `build/rust-comments-and-attributes-are-not-inert.md` | `inserting-a-test-before-an-existing-one-silently-splits-its-doc-comment`, `a-comment-only-diff-is-not-a-safe-diff-doc-comments-compile`, `a-doc-comment-is-not-inert-an-indented-block-inside-a-module-header-is-a-compiled-doctest` |
| `enclave/reference-derived-corroboration-must-not-become-a-frames-cited-rationale.md` | |
| `roles/adversary/a-mutation-campaign-needs-a-grid-not-a-count.md` | |
| `roles/adversary/a-mutation-that-reddens-does-not-confirm-which-detector-caught-it.md` | |
| `roles/steward/a-fleet-outage-can-be-platform-partitioned-and-only-a-pane-census-sees-it.md` | `a-platform-level-gate-is-not-evaded-by-swapping-model-tier-within-that-platform` |
| `roles/steward/a-pane-status-describing-progress-says-nothing-about-liveness.md` | |
| `roles/steward/a-process-count-matches-your-own-shell-when-the-command-embeds-the-path.md` | |
| `roles/steward/a-route-names-a-sha-but-the-publisher-pushes-a-ref.md` | |
| `roles/steward/filing-a-program-issues-node-requires-regenerating-implementation-progress-in-the-same-pr.md` | |
| `roles/steward/unblocking-one-of-several-blockers-restores-nothing-price-the-remedy-against-the-throughput-not-the-blocker.md` | |

Extended with a private lesson:

| Checked-in lesson | Source | What was added |
|---|---|---|
| `fleet/a-stuck-required-check-is-not-slow-ci-and-rerunning-it-wedges-the-pr.md` | `a-required-check-stuck-at-in-progress-despite-a-success-conclusion-blocks-the-merge` | the passed-but-frozen state, where a whole-run rerun is the right repair |
| `fleet/scripted-publisher-target-is-head-branch-never-main.md` | `resolve-branch-can-pick-up-a-stale-shared-local-ref-from-another-worktree` | a branch-name target resolves the shared local ref first |
| `fleet/terminal-gate-resolve-race-resolving-on-cast.md` | `an-announced-intent-cannot-deconflict-an-action-already-in-flight` | read `list_decisions` before writing; resolve both duplicates |

Drift folded back into the same-name checked-in lesson:

| Checked-in lesson | What was folded |
|---|---|
| `fleet/compact-verify-survey-can-eat-the-compact-command.md` | the slash menu can fail to open and the model answers `/compact` in prose |
| `build/failed-post-condition-probe-suspect-the-probe-first.md` | sweeping `RUST_MIN_STACK` throttles `rustc` too |
| `fleet/live-review-candidate-goes-stale-reanchor-sha.md` | the author publishes the old-to-new SHA map on every rebase under review |
| `roles/steward/steward-coldstart-infra-checks.md` | a logged-out `gh` is the resting state; the publisher mints its own token |
| `roles/steward/a-queued-delivery-waits-on-a-next-tool-call-an-idle-seat-will-never-make.md` | Escape can discard the queued message; confirm by ctx delta and re-send |
| `fleet/citing-a-private-lesson-to-another-seat-is-the-promotion-signal.md` | a zero-hit slug has three causes; search by subject first |
| `enclave/contract-spec-defer-spelling-not-concept.md` | unspellable is not unclosable: locate the obstruction's layer |
| `build/timeout-does-not-kill-grandchild-cargo-test.md` | correction: the orphaned binary cannot hold the `ken-cargo` lock (fd 9 is closed before exec) |
