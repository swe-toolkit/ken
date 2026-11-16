#!/usr/bin/env python3
"""Convert git diff --name-only's newline-terminated paths to JSON strings.

The output is consumed by the Z3, formatter, and doc-only CI classifiers. Do
not use a backslash-n string as the trim character: it leaves the actual LF
in each path, making exact file names fail to match classifier allowlists.
"""
import json
import sys


def paths_from_lines(lines):
    paths = []
    for line in lines:
        if not line.endswith("\n"):
            raise ValueError("git path listing is not newline-terminated")
        path = line[:-1]
        if not path:
            raise ValueError("git path listing contains an empty name")
        paths.append(path)
    return paths


def main():
    try:
        paths = paths_from_lines(sys.stdin)
    except (UnicodeError, ValueError) as error:
        print(f"CI_CHANGED_PATHS {error}", file=sys.stderr)
        return 2
    json.dump(paths, sys.stdout)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
