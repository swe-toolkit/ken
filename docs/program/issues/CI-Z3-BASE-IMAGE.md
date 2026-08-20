---
id: CI-Z3-BASE-IMAGE
title: "Bake z3 into a digest-pinned ghcr base image and run the restored z3-process-adapter job in it, so the flaky apt-get-install-z3 step leaves CI's critical path"
status: active
owner: verify
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Operator directive 2026-08-20 reopening lane 2 toward z3/FO integration: solve the slow/flaky z3 CI job before the lane's z3 work resumes. Steward diagnosis from GitHub run timings (see Fixed inputs). Operator approved the ghcr base-image approach and the first-party build workflow. Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

CI plumbing: a Dockerfile, a build-and-push workflow, and a job restoration with
a container reference and a digest pin. No design judgment beyond pinning
choices; the diagnosis and the approach are settled below. The T2 row.

## Why this exists: the diagnosis, measured not guessed

The `z3-process-adapter` job (test `v3_z3_process_adapter`, the one that
exercises the z3 process integration at runtime) was **removed from CI
2026-08-19** for being pathologically slow. Measured from the GitHub run
timings of the last runs that carried it, the slowness was **not** a container
build and **not** the test:

| step | normal run | observed failure run |
|---|---|---|
| `Install Z3` (`apt-get update && apt-get install --yes z3`) | 3.5 min | **360.9 min** (hung, then cancelled) |
| `Test feature-on adapter` (the actual test) | 0.8 min | — |

⇒ The cost and the instability were the `apt-get install z3` step hanging on the
GitHub runner's package mirror — a transient-infra failure mode, not the test.
Baking z3 into a **pinned image** removes that step from CI's critical path
entirely: no apt, no runtime network dependency for z3.

This job is what re-establishes CI coverage of the z3 path as lane 2 advances
its FO/checker work. The solver-free `z3-emission-control` (`--lib`) job already
stays and is unaffected.

## Fixed inputs, measured at `origin/main` `421f291f7`

- The removed job's exact body is recoverable at `c02125c02^`
  (`.github/workflows/ci.yml`): `runs-on: ubuntu-latest`; steps
  `actions/checkout@v4`, `Install Z3` (apt), `Test feature-on adapter` running
  `cargo test --locked -p ken-elaborator --features z3-process --test
  v3_z3_process_adapter`.
- The repo has **no `rust-toolchain.toml`** — the base image must pin an
  explicit Rust stable version rather than inherit one.
- `build-test` (line ~387 of `ci.yml`) is the REQUIRED aggregate check; its
  `needs:` list and its result-check `for` loop must BOTH name every
  test-running job or a failure reports green. The removed job's comment states
  this explicitly.

## Deliverables

### `D1` — the base image and its first-party build workflow

- Add a `Dockerfile` (repo root or `.github/`): `FROM rust:<pinned-stable>` (a
  specific version tag, e.g. `rust:1.NN-bookworm`, chosen to build the
  workspace), then `RUN apt-get update && apt-get install -y z3 &&
  rm -rf /var/lib/apt/lists/*`. Keep it minimal — z3 plus the toolchain the job
  needs, nothing else.
- Add `.github/workflows/build-ci-base.yml`: triggers on the Dockerfile
  changing and on `workflow_dispatch`; `permissions: { packages: write,
  contents: read }`; logs in to ghcr with the built-in `GITHUB_TOKEN`; builds
  and pushes `ghcr.io/swe-toolkit/ken/ci-z3` with a meaningful tag; **prints the
  pushed image's `@sha256:` digest** in the job log for D2 to pin.
- **PERMISSION GATE.** If the first push returns 403 (org policy blocks
  `GITHUB_TOKEN` package writes), STOP and escalate to the Steward — the
  operator pre-offered the org permission (Actions → ghcr `packages: write`).
  Do not work around it with a PAT.

### `D2` — restore the job, pointed at the image

- Restore the `z3-process-adapter` job in `ci.yml` from `c02125c02^`, but
  **replace** `runs-on: ubuntu-latest` + the `Install Z3` step with
  `container: ghcr.io/swe-toolkit/ken/ci-z3@sha256:<digest-from-D1>`. Keep
  `actions/checkout@v4` and the unchanged test command. The digest pin (not a
  moving tag) matches the repo's SHA-pinning discipline; state the version the
  digest corresponds to in a comment, as the nextest pin does.
- Re-add the job to `build-test`'s `needs:` list AND to its result-check `for`
  loop (both places).
- `D1` and `D2` are separate accepted partials: `D2` cannot pin a digest that
  `D1` has not yet produced. Land `D1`, read the digest from its run, then `D2`.

## Acceptance criteria

- **AC-1 (D1 image exists and is pinnable):** `build-ci-base.yml` has run
  successfully and pushed an image to `ghcr.io/swe-toolkit/ken/ci-z3`; the run
  log shows a concrete `sha256:` digest. Control: without the image, D2's
  `container:` reference cannot resolve and the job errors at setup.
- **AC-2 (D2 job runs in the container, no apt):** the restored job's YAML has
  `container:` with a `@sha256:` digest and **no** `Install Z3` / `apt-get`
  step; `z3 -version` is resolvable inside the container (add one verification
  line). Control: a grep for `apt-get` in the job body returns nothing.
- **AC-3 (required aggregate wired):** the job name appears in `build-test`'s
  `needs:` and in its result loop; a deliberate temporary failure of the job
  would flip `build + test` red (reason through it, do not actually merge red).
- **AC-4 (the test itself passes in the container):** `v3_z3_process_adapter`
  passes on the container image, in roughly the ~0.8 min the test took before,
  with no apt step in the timing.

## Banned scope

- Do **not** reintroduce a dependency cache (`Swatinem/rust-cache` was removed
  deliberately — see the `ci.yml` note). This WP adds a first-party pinned
  image, not a cache.
- Do **not** touch the bare-`PATH` `z3` default in `prover.rs` — that is a
  separate GRADUATION GATE (V3-Z3-PROCESS-ADAPTER successor ledger (a)), an
  operator call, not this WP.
- Do **not** containerize the other CI jobs; they stay on `ubuntu-latest`. Scope
  is the z3 job only.
- Do **not** change the test command or the `z3-process` feature.

## Sequencing

D1 (image + workflow, land, capture digest) strictly before D2 (pin + restore
job). Merge routing: lane-2 candidates come to the Steward, not the lieutenant.
