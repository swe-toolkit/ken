---
scope: build
audience: (see scope README)
source: private memory `timeout-does-not-kill-grandchild-cargo-test`
---

# `timeout` doesn't kill a grandchild cargo test binary

On `wp/VAL2-rosetta-pangram` (2026-07-03), `runtime-implementer` flagged that my
`timeout 120 scripts/ken-cargo test ...` calls were leaving the actual test
binary (`target/debug/deps/zzdebug-<hash>`, a
grandchild through `ken-cargo`'s bash wrapper → `cargo test` → the compiled
binary) running well past the 120s bound, burning CPU and RAM on a shared box.

**The orphan does not hold the build lock.** `scripts/ken-cargo` runs cargo as
`cargo "$@" 9>&-`, closing the `flock` descriptor before exec, so no cargo
descendant can inherit the lock; reaping the wrapper frees it whatever
survives below. Do not diagnose a stalled build as a wedged mutex on the
strength of an orphaned test binary.

**Why:** `timeout` only sends its signal to the direct child it spawns (the
`ken-cargo` bash script). That script execs `cargo test`, which in turn spawns
the compiled test binary as ITS OWN child — neither of those further descendants
receives `timeout`'s signal automatically; process groups aren't propagated
through the chain the way a naive read of `timeout`'s behavior suggests.

**How to apply:** when running a scratch/probe test with an expected runtime
bound via `scripts/ken-cargo test ...`, do not trust `timeout N` alone to
enforce the bound. Options, in order of reliability:
1. Run the command with `run_in_background` / `&`, then explicitly
   `pkill -9 -f <test-binary-basename>` (e.g. `pkill -9 -f zzdebug`) if it's
   still alive past your intended bound — verified via
   `ps aux | grep <basename>`.
2. After any suspected timeout/kill, always `ps aux | grep -i "<crate>\|cargo"`
   to confirm nothing of yours is still running before starting the next
   command.
3. Don't chain multiple nested `timeout` calls hoping one will "win" — none of
   them reach the grandchild reliably.

Sibling of fleet model rollout stagger restarts (shared-resource discipline on
the same box) — the CPU and memory analog of not stepping on other agents'
concurrent work.
