#!/usr/bin/env python3
"""Controls for scripts/ci-z3-relevant.py: skips only on known non-inputs."""
import copy
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
SCRIPT = os.path.join(HERE, "ci-z3-relevant.py")
spec = importlib.util.spec_from_file_location("ci_z3_relevant", SCRIPT)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

W = "/w"


def pkg(name, deps=()):
    return {
        "id": f"path+file://{W}/crates/{name}#0.0.0",
        "name": name,
        "manifest_path": f"{W}/crates/{name}/Cargo.toml",
        "dependencies": [
            {"name": d, "kind": k, "path": f"{W}/crates/{d}"} for d, k in deps
        ] + [{"name": "serde", "kind": None}],
    }


# Mirrors the shape of the real graph: a normal chain, a dev-dependency that
# depends back on the root, and crates outside the closure.
PACKAGES = [
    pkg("ken-elaborator", [("ken-kernel", None), ("ken-runtime", None), ("ken-interp", "dev")]),
    pkg("ken-kernel"),
    pkg("ken-runtime", [("ken-runtime-support", None), ("ken-codegen", "build"), ("ken-bench", "dev")]),
    pkg("ken-runtime-support"),
    pkg("ken-codegen"),
    pkg("ken-interp", [("ken-elaborator", None), ("ken-cli", "dev")]),
    pkg("ken-bench"),
    pkg("ken-cli", [("ken-elaborator", None)]),
    pkg("ken-verify", [("ken-cli", None)]),
]
METADATA = {
    "workspace_root": W,
    "workspace_members": [p["id"] for p in PACKAGES],
    "packages": PACKAGES,
}


def verdict(paths, metadata=METADATA):
    return m.classify(paths, metadata)[0]


def run_cli(paths_arg, metadata_arg):
    out = subprocess.run([sys.executable, SCRIPT, paths_arg, metadata_arg],
                         check=True, capture_output=True, text=True)
    return out.stdout.strip()


cases = [
    # The operator's controls.
    (["catalog/packages/Core/Logic/Or.ken.md"], True),
    (["crates/ken-cli/src/main.rs"], False),
    # spec/ is read by the lib test kenfmt_b4_splicing, so it is an input.
    (["spec/10-kernel/18a-primitive-registry.md"], True),
    (["Cargo.lock"], True),
    ([".github/workflows/ci.yml"], True),
    ([], True),
    # Closure: normal, transitive, build and root dev edges are followed.
    (["crates/ken-elaborator/src/prover.rs"], True),
    (["crates/ken-kernel/src/lib.rs"], True),
    (["crates/ken-runtime-support/src/lib.rs"], True),
    (["crates/ken-codegen/src/lib.rs"], True),
    (["crates/ken-interp/tests/x.rs"], True),
    # A non-root crate's dev-dependency is not built by this job.
    (["crates/ken-bench/src/lib.rs"], False),
    (["crates/ken-verify/tests/x.rs", "docs/a.md", "scripts/other.py"], False),
    # Any member manifest can break `--locked` workspace resolution.
    (["crates/ken-cli/Cargo.toml"], True),
    (["crates/ken-cli/tests/fixtures/rt_bracket_body_once_buffer.ken"], True),
    (["conformance/stdlib/geometry/seed-box-area.md"], True),
    (["examples/rosetta/x.ken"], True),
    (["library/learn/x.md"], True),
    (["Cargo.toml"], True),
    ([".cargo/config.toml"], True),
    (["rust-toolchain.toml"], True),
    (["Dockerfile"], True),
    (["scripts/ci-resource-monitor.py"], True),
    (["scripts/ci-z3-relevant.py"], True),
    ([".github/CODEOWNERS", "README.md", "agent/a.md"], False),
    # Unknown roots and non-member crate directories fail safe to run.
    (["newroot/x"], True),
    (["crates/ken-new/src/lib.rs"], True),
    (["crates/ken-cli/src/main.rs", "newroot/x"], True),
    # Malformed paths run.
    (["docs/../crates/ken-kernel/src/lib.rs"], True),
    (["docs//a.md"], True),
    (["/docs/a.md"], True),
    ([7], True),
    ("crates/ken-cli/src/main.rs", True),
]
for paths, expected in cases:
    assert verdict(paths) is expected, (paths, m.classify(paths, METADATA))

# Unreadable or inconsistent metadata runs, even for an otherwise-skipped path.
skip = ["crates/ken-cli/src/main.rs"]
assert verdict(skip) is False
for mutate in [
    lambda md: md.pop("packages"),
    lambda md: md.update(workspace_root="w"),
    lambda md: md["packages"][0].update(name="renamed"),
    lambda md: md["packages"][2]["dependencies"].append({"kind": "weird", "path": W + "/crates/x"}),
    lambda md: md["packages"][1]["dependencies"].append({"kind": None, "path": "/elsewhere/crate"}),
    lambda md: md["packages"][0].update(manifest_path=W + "/Cargo.toml"),
    lambda md: md["workspace_members"].append("missing#0.0.0"),
]:
    md = copy.deepcopy(METADATA)
    mutate(md)
    assert verdict(skip, md) is True, md
assert verdict(skip, None) is True

# The command line prints exactly one word and fails safe on unreadable input.
with tempfile.TemporaryDirectory() as tmp:
    meta = os.path.join(tmp, "meta.json")
    json.dump(METADATA, open(meta, "w"))

    def paths_file(content):
        p = os.path.join(tmp, "paths.json")
        with open(p, "w") as f:
            f.write(content)
        return p

    assert run_cli(paths_file(json.dumps(skip)), meta) == "false"
    assert run_cli(paths_file(json.dumps(["Cargo.lock"])), meta) == "true"
    assert run_cli(paths_file("not json"), meta) == "true"
    assert run_cli(paths_file('{"a": 1}'), meta) == "true"
    assert run_cli(os.path.join(tmp, "missing.json"), meta) == "true"
    assert run_cli(paths_file(json.dumps(skip)), os.path.join(tmp, "missing.json")) == "true"
    assert subprocess.run([sys.executable, SCRIPT], capture_output=True, text=True).stdout.strip() == "true"

    # Against this checkout's own workspace, when cargo is available.
    cargo = shutil.which("cargo")
    if cargo:
        root = os.path.dirname(HERE)
        live = subprocess.run([cargo, "metadata", "--format-version", "1", "--no-deps", "--offline"],
                              cwd=root, check=True, capture_output=True, text=True).stdout
        live_meta = os.path.join(tmp, "live.json")
        with open(live_meta, "w") as f:
            f.write(live)
        for paths, expected in [
            (["catalog/packages/Core/Logic/Or.ken.md"], "true"),
            (["crates/ken-cli/src/main.rs"], "false"),
            (["spec/10-kernel/18a-primitive-registry.md"], "true"),
            (["Cargo.lock"], "true"),
            ([".github/workflows/ci.yml"], "true"),
            (["crates/ken-elaborator/src/prover.rs"], "true"),
        ]:
            got = run_cli(paths_file(json.dumps(paths)), live_meta)
            assert got == expected, (paths, got)
        print("live workspace metadata controls passed")
    else:
        print("cargo unavailable: live workspace metadata controls not run")

print(f"ci z3-relevant classifier controls passed ({len(cases)} path cases)")
