#!/usr/bin/env python3
"""Generates the docs site's catalog from the Octane docs and the examples.

Reads ../src/markdown/components/<slug>.mdx (title, description, example
headings and prose) and site/src/examples/<module>.rs (example functions),
then writes:

  site/src/examples.rs           `pub mod <module>;` for every examples file
  site/src/catalog/generated.rs  DOCS, render_example, component_source,
                                 example_file

Run it through scripts/sync.sh.
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent  # topcoat/
SITE = ROOT / "site"
MDX = ROOT.parent / "src" / "markdown" / "components"
EXAMPLES = SITE / "src" / "examples"
COMPONENTS = SITE / "src" / "components"

# Octane descriptions that name libraries the Topcoat versions don't use.
DESCRIPTIONS = {
    "carousel": "A carousel with scroll snapping and previous/next controls.",
    "chart": "Server-rendered SVG charts: bar, line, area and pie, with legends and tooltips.",
    "command": "Fast, composable command menu with filtering and keyboard navigation.",
    "data-table": "Sortable, filterable, paginated tables driven by the URL.",
    "direction": "Sets the reading direction (LTR or RTL) for the components inside it.",
    "form": "Server-validated forms: labels, controls, descriptions and error messages.",
}

FN = re.compile(r"pub async fn (\w+)\s*[(<]")
PREVIEW = re.compile(r'<ComponentPreview[^>]*>')
FILE = re.compile(r"file=<rootDir>/src/examples/ui/([\w/-]+)\.tsrx")
LINK = re.compile(r"\[([^\]]+)\]\([^)]+\)")


def snake(name: str) -> str:
    return name.replace("-", "_")


def lit(text: str) -> str:
    """A Rust string literal (JSON escapes are valid Rust escapes here)."""
    return json.dumps(text, ensure_ascii=False)


def example_for(path: str, slug: str, fns: list[str]) -> str | None:
    """Maps an Octane example file to its Rust example function."""
    stem = path.split("/")[-1]
    if stem in ("index", slug):
        candidates = ["demo"]
    else:
        s = snake(stem)
        candidates = [s, f"{s}_demo", f"{s}_example", f"{snake(slug)}_{s}", f"{s}_{snake(slug)}"]
    for name in candidates:
        if name in fns:
            return name
    loose = [f for f in fns if snake(stem) in f]
    return loose[0] if len(loose) == 1 else None


def parse(slug: str, fns: list[str]):
    text = (MDX / f"{slug}.mdx").read_text()
    front, body = text.split("---", 2)[1:]
    meta = dict(
        (k.strip(), v.strip().strip("\"'"))
        for k, v in (line.split(":", 1) for line in front.strip().splitlines() if ":" in line)
    )
    sections, used = [], set()
    heading, prose, in_code, top = "", [], False, None
    file_fence = False  # inside an empty ```tsx file=...``` fence
    lines = body.splitlines()
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("```tsx file="):
            file_fence = True
            continue
        if stripped.startswith("```"):
            if file_fence:
                file_fence = False
            else:
                in_code = not in_code
            continue
        if in_code:
            continue
        if stripped.startswith("#"):
            heading = stripped.lstrip("#").strip()
            prose = []
            continue
        if PREVIEW.match(stripped):
            window = "\n".join(lines[i : i + 3])
            m = FILE.search(window)
            ex = example_for(m.group(1), slug, fns) if m else None
            if ex is None:
                if m:
                    print(f"  ! {slug}: no example function for {m.group(1)}", file=sys.stderr)
                continue
            if top is None and heading in ("",):
                top = ex
                continue
            if heading in ("Installation", "Usage"):
                continue
            if ex not in used:
                used.add(ex)
                sections.append((heading, " ".join(prose).strip(), ex))
            prose = []
            continue
        if stripped and not stripped.startswith("<") and not stripped.startswith("```"):
            prose.append(LINK.sub(r"\1", stripped))
    first = top or ("demo" if "demo" in fns else (fns[0] if fns else None))
    out = [("", "", first)] if first else []
    out += [s for s in sections if s[2] != first]
    listed = {s[2] for s in out}
    out += [(f.replace("_", " ").capitalize(), "", f) for f in fns if f not in listed]
    return meta.get("title", slug), meta.get("description", ""), out


USES = re.compile(r"use\s+crate::components::([^;]*);", re.S)
PATH_USE = re.compile(r"crate::components::(\w+)::")


def ready(module: str) -> bool:
    """An examples file is listed once every component it uses is installed."""
    source = (EXAMPLES / f"{module}.rs").read_text()
    used = set(PATH_USE.findall(source))
    for path in USES.findall(source):
        # `button::{..}` or `{button::{..}, card::card}`: modules are the
        # names followed by `::` at the top level of the import.
        depth, word = 0, ""
        for i, ch in enumerate(path):
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
            elif depth <= 1 and path[i : i + 2] == "::" and word:
                used.add(word)
            if ch.isalnum() or ch == "_":
                word += ch
            elif ch != ":":
                word = ""
    missing = sorted(c for c in used if not (COMPONENTS / f"{c}.rs").exists())
    if missing:
        print(f"  - skipping {module}: needs {', '.join(missing)}", file=sys.stderr)
    return not missing


def main() -> None:
    modules = sorted(p.stem for p in EXAMPLES.glob("*.rs") if ready(p.stem))
    (SITE / "src" / "examples.rs").write_text(
        "// @generated by scripts/gen_catalog.py; do not edit.\n"
        + "".join(f"pub mod {m};\n" for m in modules)
    )

    docs = []
    for module in modules:
        slug = module.replace("_", "-")
        fns = FN.findall((EXAMPLES / f"{module}.rs").read_text())
        if (MDX / f"{slug}.mdx").exists():
            title, description, sections = parse(slug, fns)
        else:
            title, description = slug.replace("-", " ").title(), ""
            sections = [(f.replace("_", " ").capitalize() if i else "", "", f) for i, f in enumerate(fns)]
        description = DESCRIPTIONS.get(slug, description)
        docs.append((slug, module, title, description, sections, fns))
    docs.sort(key=lambda d: d[2].lower())

    out = [
        "// @generated by scripts/gen_catalog.py; do not edit.",
        "",
        "use topcoat::{",
        "    Result,",
        "    view::{View, ViewExt, component, view},",
        "};",
        "",
        "use super::{Doc, Section};",
        "use crate::examples;",
        "",
        "pub const DOCS: &[Doc] = &[",
    ]
    for slug, module, title, description, sections, _ in docs:
        out.append(f"    Doc {{ slug: {lit(slug)}, module: {lit(module)}, title: {lit(title)}, description: {lit(description)}, sections: &[")
        for heading, body, ex in sections:
            out.append(f"        Section {{ heading: {lit(heading)}, body: {lit(body)}, example: {lit(ex)} }},")
        out.append("    ] },")
    out += [
        "];",
        "",
        "/// Renders one example by module and function name; nothing if unknown.",
        "///",
        "/// Each arm is a different view type, so `.boxed()` gives them one type.",
        "#[component]",
        "pub async fn render_example(module: &str, example: &str) -> Result<impl View> {",
        "    Ok(match (module, example) {",
    ]
    for _, module, _, _, _, fns in docs:
        for f in fns:
            out.append(f"        ({lit(module)}, {lit(f)}) => view! {{ examples::{module}::{f}() }}.boxed(),")
    out += [
        "        _ => view! {}.boxed(),",
        "    })",
        "}",
        "",
        "/// The installed source of a component, if the site has one by that name.",
        "pub fn component_source(module: &str) -> Option<&'static str> {",
        "    match module {",
    ]
    for _, module, *_ in docs:
        if (COMPONENTS / f"{module}.rs").exists():
            out.append(f"        {lit(module)} => Some(include_str!(\"../components/{module}.rs\")),")
    out += [
        "        _ => None,",
        "    }",
        "}",
        "",
        "/// The whole examples file of a module.",
        "pub fn example_file(module: &str) -> &'static str {",
        "    match module {",
    ]
    for _, module, *_ in docs:
        out.append(f"        {lit(module)} => include_str!(\"../examples/{module}.rs\"),")
    out += ["        _ => \"\",", "    }", "}", ""]
    (SITE / "src" / "catalog").mkdir(exist_ok=True)
    (SITE / "src" / "catalog" / "generated.rs").write_text("\n".join(out))
    print(f"catalog: {len(docs)} components, {sum(len(d[5]) for d in docs)} examples")


if __name__ == "__main__":
    main()
