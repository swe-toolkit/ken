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
ALLOWED: tuple[tuple[str, str, str, str], ...] = (
    # Fixture inputs/configuration, not observed environment IDs.
    ("crates/ken-cli/tests/rt_escape_second_resource_native.rs", "128", "fixture input value", "source-owned resource profile nodes/words"),
    ("crates/ken-cli/tests/rt_escape_second_resource_native.rs", "512", "fixture input value", "source-owned resource profile words"),
    ("crates/ken-cli/tests/rt_escape_second_resource_native.rs", "18446744073709551615", "fixture input value", "Ken source u64::MAX input"),
    # Stored pre-W3 core vectors are not recompiled from the current prelude.
    ("crates/ken-elaborator/src/checked_core.rs", "0x3c35_abb3_d8de_b8ea", "stored-artifact vector", "pre-W3 stored core fixture"),
    ("crates/ken-elaborator/src/checked_core.rs", "0x23d4_d551_775c_12cd", "stored-artifact vector", "pre-W3 stored core fixture"),
    # Algorithmic compatibility vectors and bit patterns.
    ("crates/ken-foundation/src/hash.rs", "0xcbf29ce484222325", "algorithm test vector", "FNV-1a offset basis"),
    ("crates/ken-foundation/src/hash.rs", "0xa430d84680aabd0b", "algorithm test vector", "FNV-1a output for test bytes"),
    ("crates/ken-runtime-support/src/hash.rs", "0xa430d84680aabd0b", "algorithm test vector", "FNV-1a output for test bytes"),
    ("crates/ken-elaborator/tests/lang_lex_hex_float.rs", "0x0010_0000_0000_0000", "algorithm test vector", "IEEE-754 bit vector"),
    ("crates/ken-elaborator/tests/lang_lex_hex_float.rs", "0x0008_0000_0000_0000", "algorithm test vector", "IEEE-754 bit vector"),
    # Input hex limbs and rendered values, not semantic hashes of a prelude.
    ("crates/ken-elaborator/tests/lang_lex_hex_float.rs", "0x100000000000008", "fixture input value", "source hex lexer input"),
    ("crates/ken-elaborator/tests/lang_lex_hex_float.rs", "0x100000000000009", "fixture input value", "source hex lexer input"),
    ("crates/ken-elaborator/tests/lang_lex_hex_float.rs", "0x100000000000018", "fixture input value", "source hex lexer input"),
    ("crates/ken-elaborator/tests/lang_lex_numeric_forms.rs", "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", "fixture input value", "source hex lexer input"),
    ("crates/ken-runtime/src/object_linker_packaging.rs", "0x10000000000000007", "fixture input value", "rendered program integer"),
    ("crates/ken-runtime-support/src/activation_abi.rs", "0x1ffffffffffffffff", "fixture input value", "rendered program integer"),
    ("crates/ken-runtime/src/cranelift_backend/lowering/core/tests/constructors.rs", "0xdead_beef_0000_0001", "fixture input value", "hand-built constructor limb"),
    ("crates/ken-interp/src/eval.rs", "0x7ff8_0000_0000_0001", "fixture input value", "hand-built NaN payload"),
    ("crates/ken-interp/src/eval.rs", "0x7ff8_0000_0000_0002", "fixture input value", "hand-built NaN payload"),
    ("crates/ken-runtime-support/src/activation_abi.rs", "0x8000_0000_0000_0001", "fixture input value", "hand-built activation limb"),
    ("crates/ken-runtime-support/src/activation_abi.rs", "0x8000000000000001", "fixture input value", "hand-built rendered activation limb"),
    ("crates/ken-runtime-support/src/activation_abi.rs", "0x0000_0000_0000_00ab", "fixture input value", "hand-built activation limb"),
    ("crates/ken-runtime-support/src/activation_abi.rs", "0x100000000000000ab", "fixture input value", "hand-built rendered activation limb"),
    ("crates/ken-host/src/effect_wire.rs", "0x1020_3040_5060_7080", "fixture input value", "hand-built effect wire payload"),
    ("crates/ken-host/src/abi_v1.rs", "0xfedc_ba98_7654_3210", "fixture input value", "hand-built trace identity"),
    # The count is part of the explicit spec floor, not an incidental prelude size.
    ("crates/ken-elaborator/tests/lang_prelude_floor_console_char.rs", "32", "spec-anchored count", "spec 30 section 4 floor"),
    # These populations belong to the fixture or local enum, not the prelude.
    ("crates/ken-cli/tests/rt_parity_native.rs", "14", "subject-owned count", "fixture-owned capture entries"),
    ("crates/ken-cli/tests/rt_parity_native.rs", "13", "subject-owned count", "fixture-owned materializations"),
    ("crates/ken-cli/tests/rt_parity_native.rs", "17", "subject-owned count", "fixture-owned settlement segments"),
    ("crates/ken-elaborator/src/r_layer_tests/cat_tier_e_decoder_import.rs", "28", "subject-owned count", "schema module's own public surface"),
    ("crates/ken-runtime/src/cranelift_backend/planning/static_transition/closure.rs", "25", "subject-owned count", "fixture-built disposition rows"),
    ("crates/ken-runtime/src/cranelift_backend/planning/static_transition/closure.rs", "11", "subject-owned count", "fixture-built inactive disposition rows"),
    ("crates/ken-runtime/src/cranelift_backend/planning/static_transition/closure.rs", "12", "subject-owned count", "fixture-built Live disposition rows"),
    ("crates/ken-runtime-support/src/boundary_value.rs", "13", "subject-owned count", "unit fixture's boundary value cases"),
    ("crates/ken-elaborator/tests/px3_abi_scalars.rs", "12", "subject-owned count", "test-named type IDs and PX3 scalar conversion IDs"),
    ("crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs", "22", "subject-owned count", "module-owned LoweredVariant::ALL enum"),
    ("crates/ken-runtime/src/cranelift_backend/lowering/core/tests/effects.rs", "16", "subject-owned count", "test-enumerated host_effect_seat_contract_of byte-span seats"),
    ("crates/ken-runtime/src/boundary_value_clif.rs", "30", "subject-owned count", "module-owned BOUNDARY_LOCAL_HELPERS inventory"),
)

ID = re.compile(r"(?<![\w.])(\d{3,})(?:_?u(?:8|16|32|64|size)|_?i(?:32|64))?(?![\w.])")
HEX = re.compile(r"0[xX][0-9a-fA-F_]+(?:_?[iu](?:8|16|32|64|size))?")
HASH = re.compile(r"0x[0-9a-fA-F_]{12,}")
HASH_CONTEXT = re.compile(r"assert|==|!=|expected|contains\(")
ASSERT = re.compile(r"\b(?:debug_)?assert(?:_eq|_ne)?\s*!\s*[({\[]")
COUNT_MACRO = {"assert_eq!", "assert_ne!", "debug_assert_eq!"}
COUNT_LITERAL = re.compile(r"\d[\d_]*\d(?:_?(?:usize|u32|u64))?")
COUNT_EXPR = re.compile(r"\.\s*(?:len|count)\s*\(\s*\)\s*$", re.DOTALL)
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
INLINE_TEST_MODULE = re.compile(
    r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*"
    r"(?:#\s*\[[^\]]*\]\s*)*"
    r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+\w+\s*\{"
)
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
    # The value boundary is an assertion statement, not a line: rustfmt may
    # split either the comparison or its literal across several source lines.
    code, _ = lex(scoped)
    for found in ASSERT.finditer(code):
        opening = found.end() - 1
        matching = {"(": ")", "[": "]", "{": "}"}
        close = [matching[code[opening]]]
        pos = opening + 1
        while pos < len(code) and close:
            if code[pos] in matching:
                close.append(matching[code[pos]])
            elif code[pos] in ")]}":
                if code[pos] != close.pop():
                    raise ValueError(f"mismatched assertion delimiter in {path}")
            pos += 1
        if close:
            raise ValueError(f"unclosed assertion in {path}:{scoped.count(chr(10), 0, opening) + 1}")
        line_number = scoped.count("\n", 0, found.start()) + 1
        body = scoped[opening + 1:pos - 1]
        for match in HASH.finditer(body):
            hit = Hit(path, line_number, match.group(), "hash")
            if hit not in hits:
                hits.append(hit)
        name = re.sub(r"\s+", "", found.group()[:-1])
        if name not in COUNT_MACRO:
            continue
        # Split only on commas at invocation depth zero; commas inside
        # expressions and strings cannot separate the macro arguments.
        commas = []
        bracket_depth = 0
        for index in range(opening + 1, pos - 1):
            if code[index] in "([{":
                bracket_depth += 1
            elif code[index] in ")]}":
                bracket_depth -= 1
            elif code[index] == "," and not bracket_depth:
                commas.append(index)
        if not commas:
            continue
        args = [scoped[a:b].strip() for a, b in zip(
            [opening + 1] + [at + 1 for at in commas], commas + [pos - 1]
        )]
        if len(args) < 2:
            continue
        for actual, expected in ((args[0], args[1]), (args[1], args[0])):
            if COUNT_EXPR.search(actual):
                literal = COUNT_LITERAL.fullmatch(expected)
                if literal:
                    hits.append(Hit(path, line_number, literal.group(), "count"))
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
