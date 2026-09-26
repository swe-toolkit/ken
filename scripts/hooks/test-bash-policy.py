#!/usr/bin/env python3
"""Cases for scripts/hooks/bash-policy: each is (expected exit, command).

Run after editing the policy: `python3 scripts/hooks/test-bash-policy.py`.
A false refusal stops a seat, so every rule needs an allowed neighbour here.
"""

import pathlib
import subprocess
import sys

POLICY = pathlib.Path(__file__).with_name("bash-policy")

CASES = [
    (0, "grep -n -E 'transcript|gh run rerun|moot.toml' agent/COORDINATION.md"),
    (0, 'grep "a|gh run rerun" f'),
    (2, 'gh run rerun 123'),
    (2, 'echo x | gh run rerun 1'),
    (2, 'foo && gh run rerun 1'),
    (2, 'git stash'),
    (0, 'git stash push -m "steward: x"'),
    (2, 'git stash pop'),
    (0, 'cd x; git stash apply stash@{2}'),
    (2, 'git stash apply'),
    (2, 'scripts/ken-cargo test --workspace'),
    (2, 'timeout 600 cargo build --all'),
    (0, 'scripts/ken-cargo test -p ken-cli --test rt_parity_native'),
    (2, 'git checkout -- moot.toml'),
    (2, 'git -C /x restore moot.toml'),
    (0, 'git log --oneline'),
    (0, 'cat > f <<EOF\ngit stash pop\nEOF\necho ok'),
    (0, "cat > f <<'EOF'\ngh run rerun 1\nEOF"),
    (2, 'echo a\ngit stash pop'),
    (2, '(cd x && git stash pop)'),
    (2, 'echo "unbalanced; git stash pop'),
    (0, 'git commit -m "do not git stash pop; ever"'),
    (0, 'echo a # gh run rerun'),
    (2, 'x=1 git stash pop'),
    (0, 'git stash list'),
]


def main():
    failed = 0
    for want, command in CASES:
        rc = subprocess.run([str(POLICY)], input=command, text=True,
                            capture_output=True).returncode
        if rc != want:
            failed += 1
            print(f"FAIL: exit {rc}, want {want}: {command!r}")
    print(f"{len(CASES) - failed}/{len(CASES)} cases pass")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
