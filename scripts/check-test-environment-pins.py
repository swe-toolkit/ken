#!/usr/bin/env python3
"""Fail on test assertions pinned to incidental compiler-environment values.

This is a CI source lint, not a test of Ken behavior. Its scope and allowed
vectors are intentionally explicit so a new pin cannot silently join them.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]

# (path, literal, reason class, why this value belongs to its subject).
# Keys never depend on line numbers: adding an unrelated test does not re-key
# an allowed vector, and deleting a vector leaves a stale entry that fails.
ALLOWED: tuple[tuple[str, str, str, str], ...] = ()

ID = re.compile(r"(?<![\w.])(\d{3,})(?:_?u(?:8|16|32|64|size)|_?i(?:32|64))?(?![\w.])")
HEX = re.compile(r"0[xX][0-9a-fA-F_]+(?:_?[iu](?:8|16|32|64|size))?")
HASH = re.compile(r"0x[0-9a-fA-F_]{12,}")
HASH_CONTEXT = re.compile(r"assert|==|!=|expected|contains\(")
COUNT = re.compile(
    r"assert_eq!\([^;]*?\.(?:len|count)\(\)\s*,\s*(\d{2,})\b"
    r"|assert_eq!\(\s*(\d{2,})\s*,[^;]*?\.(?:len|count)\(\)",
    re.DOTALL,
)
COMPILES_KEN = re.compile(
    r"build_native_program|compile_ken_package_sources|compile_literate|"
    r"compile_source|ElabEnv::new|run_program_effect_observation|"
    r"elaborate_module_from_roots|ken_cli::|compile_program"
)
ORIGIN_PLANE = re.compile(
    r"\b(?:vis_origin|owner_origin|StaticOriginId|producer_call_origin|"
    r"response_origin|operation_root_origin|emission_origin|"
    r"underived_owner_origins|static_response_owners|returned_vis_protocols|"
    r"selected_leaf|eliminating_cm|with_forced_underived_owner)\b"
)
GLOBAL_READ = re.compile(r"\.globals(?:\[|\.get\()")
STACK_LINE = re.compile(
    r"stack_size|1024|KiB|KIB|MiB|Duration|from_millis|from_secs|"
    r"sleep|timeout|with_capacity"
)
INLINE_TEST_MODULE = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*mod\s+\w+\s*\{")
RAW_STRING = re.compile(r'(?:b?r)(#{0,32})"')
CHAR_LITERAL = re.compile(r"(?:b)?'(?:[^'\\\n]|\\.)'")


@dataclass(frozen=True, order=True)
class Hit:
    path: str
    line: int
    value: str
    shape: str


def _blank(text: str) -> str:
    return "".join("\n" if char == "\n" else " " for char in text)


def lex(text: str) -> tuple[str, str]:
    """Return Rust code for brace matching and source with comments removed.

    Braces inside normal/raw strings, chars and comments cannot close an
    inline test module. The second plane preserves string values deliberately:
    a stderr assertion containing an origin ordinal is still a pin.
    """
    code, visible = [], []
    pos = 0
    while pos < len(text):
        start = pos
        if text.startswith("//", pos):
            end = text.find("\n", pos)
            pos = len(text) if end < 0 else end
            segment = text[start:pos]
            code.append(_blank(segment))
            visible.append(_blank(segment))
        elif text.startswith("/*", pos):
            depth = 1
            pos += 2
            while pos < len(text) and depth:
                if text.startswith("/*", pos):
                    depth += 1
                    pos += 2
                elif text.startswith("*/", pos):
                    depth -= 1
                    pos += 2
                else:
                    pos += 1
            if depth:
                raise ValueError("unterminated Rust block comment")
            segment = text[start:pos]
            code.append(_blank(segment))
            visible.append(_blank(segment))
        else:
            raw = RAW_STRING.match(text, pos)
            char = CHAR_LITERAL.match(text, pos)
            if raw:
                end_marker = '"' + raw.group(1)
                end = text.find(end_marker, raw.end())
                if end < 0:
                    raise ValueError("unterminated Rust raw string")
                pos = end + len(end_marker)
            elif char:
                pos = char.end()
            elif text[pos] == '"':
                pos += 1
                while pos < len(text):
                    if text[pos] == "\\":
                        pos += 2
                    elif text[pos] == '"':
                        pos += 1
                        break
                    else:
                        pos += 1
                else:
                    raise ValueError("unterminated Rust string")
            else:
                code.append(text[pos])
                visible.append(text[pos])
                pos += 1
                continue
            segment = text[start:pos]
            code.append(_blank(segment))
            visible.append(segment)
    return "".join(code), "".join(visible)


def test_source(path: str, text: str) -> str:
    """Select test files or brace-matched inline test modules, not prod code."""
    parts = Path(path).parts
    if len(parts) < 4 or parts[0] != "crates" or parts[2] not in ("src", "tests"):
        return ""
    code, visible = lex(text)
    if parts[2] == "tests" or "tests" in parts[3:] or "r_layer_tests" in parts[3:] or parts[-1] == "tests.rs":
        return visible
    if parts[-1].endswith(".rs"):
        pairs: dict[int, int] = {}
        stack: list[int] = []
        for pos, char in enumerate(code):
            if char == "{":
                stack.append(pos)
            elif char == "}":
                if not stack:
                    raise ValueError(f"unbalanced Rust braces in {path}")
                pairs[stack.pop()] = pos
        if stack:
            raise ValueError(f"unbalanced Rust braces in {path}")
        scoped = list(_blank(visible))
        for found in INLINE_TEST_MODULE.finditer(code):
            end = pairs[found.end() - 1] + 1
            scoped[found.start():end] = visible[found.start():end]
        return "".join(scoped)
    return ""


def scan_text(path: str, text: str) -> list[Hit]:
    scoped = test_source(path, text)
    if not scoped:
        return []
    hits: list[Hit] = []
    gate = bool(COMPILES_KEN.search(scoped) and ORIGIN_PLANE.search(scoped))
    for line_num, line in enumerate(scoped.splitlines(), 1):
        without_hex = HEX.sub(lambda match: " " * len(match.group()), line)
        if gate and not STACK_LINE.search(line):
            for found in ID.finditer(without_hex):
                if re.search(r"\bGlobalId\s*\(\s*$", line[: found.start()]):
                    continue  # Fixture-constructed IDs do not observe the environment.
                hits.append(Hit(path, line_num, found.group(1), "origin-id"))
        if GLOBAL_READ.search(line):
            for found in ID.finditer(without_hex):
                if not re.search(r"\bGlobalId\s*\(\s*$", line[: found.start()]):
                    hits.append(Hit(path, line_num, found.group(1), "global-read"))
        if HASH_CONTEXT.search(line):
            for found in HASH.finditer(line):
                hits.append(Hit(path, line_num, found.group(), "hash"))
    for found in COUNT.finditer(scoped):
        start = found.start(1) if found.group(1) is not None else found.start(2)
        value = found.group(1) if found.group(1) is not None else found.group(2)
        hits.append(Hit(path, scoped.count("\n", 0, start) + 1, value, "count"))
    return sorted(hits)


def source_paths(root: Path) -> list[Path]:
    return sorted((root / "crates").glob("*/src/**/*.rs")) + sorted(
        (root / "crates").glob("*/tests/**/*.rs")
    )


def inspect(root: Path = ROOT) -> tuple[list[Hit], set[tuple[str, str, str, str]]]:
    hits = []
    for file in source_paths(root):
        path = file.relative_to(root).as_posix()
        hits.extend(scan_text(path, file.read_text(encoding="utf-8")))
    inventory = {(path, literal): (kind, reason) for path, literal, kind, reason in ALLOWED}
    if len(inventory) != len(ALLOWED):
        raise ValueError("duplicate allowed inventory key")
    seen = {(hit.path, hit.value) for hit in hits}
    unexpected = [hit for hit in hits if (hit.path, hit.value) not in inventory]
    stale = {entry for entry in ALLOWED if entry[:2] not in seen}
    return sorted(unexpected), stale


def main() -> int:
    unexpected, stale = inspect()
    for hit in unexpected:
        print(f"{hit.path}:{hit.line}: {hit.value}: {hit.shape}")
    for path, value, kind, reason in sorted(stale):
        print(f"{path}:stale: {value}: {kind}: {reason}")
    return 1 if unexpected or stale else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError) as error:
        print(f"environment pin lint could not determine its input: {error}", file=sys.stderr)
        sys.exit(1)
