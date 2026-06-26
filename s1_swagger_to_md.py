#!/usr/bin/env python3
"""
SentinelOne Swagger (OpenAPI 2.0) -> Markdown docs.

Usage:
    python3 s1_swagger_to_md.py swagger_2_1.json [formatted_swagger_2_1.json] [-o docs]

Reads the raw swagger spec (required) and, if given, the portal's
"formatted_swagger" file to enrich each operation with permissions /
best-practice text. Emits one markdown file per category (tag) plus a
README.md index.
"""
import argparse
import json
import os
import re
import sys
from collections import defaultdict

HTTP_METHODS = ("get", "post", "put", "patch", "delete", "head", "options")


def load(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def slug(s):
    s = re.sub(r"[^a-zA-Z0-9]+", "-", (s or "").strip().lower())
    return re.sub(r"-+", "-", s).strip("-") or "misc"


def resolve_ref(spec, ref):
    """Resolve a #/definitions/Foo style local ref to its node."""
    if not ref.startswith("#/"):
        return None
    node = spec
    for part in ref[2:].split("/"):
        part = part.replace("~1", "/").replace("~0", "~")
        if isinstance(node, dict) and part in node:
            node = node[part]
        else:
            return None
    return node


def type_str(schema, spec, depth=0):
    """Compact human type for a schema node."""
    if schema is None:
        return ""
    if isinstance(schema, list):
        # e.g. type: ["string","null"] or a oneOf-ish list of schemas
        parts = [type_str(s, spec, depth + 1) if isinstance(s, dict) else str(s)
                 for s in schema]
        return " | ".join(p for p in parts if p)
    if not isinstance(schema, dict):
        return str(schema)
    if "$ref" in schema:
        name = schema["$ref"].split("/")[-1]
        return f"[{name}](#def-{slug(name)})"
    t = schema.get("type")
    if t == "array":
        items = schema.get("items", {})
        return f"array<{type_str(items, spec, depth+1)}>"
    if schema.get("enum"):
        return f"enum({', '.join(map(str, schema['enum'][:8]))})"
    fmt = schema.get("format")
    return f"{t}({fmt})" if fmt else (t or "object")


def md_escape(s):
    if s is None:
        return ""
    return str(s).replace("|", "\\|").replace("\n", " ").strip()


def build_formatted_index(formatted):
    """
    Map (METHOD, path) -> extra info from the portal's formatted file.
    Structure is unknown/loose, so this walks defensively for anything that
    looks like permissions / notes keyed by method+url.
    """
    idx = {}
    if not formatted:
        return idx

    def visit(node):
        if isinstance(node, dict):
            url = node.get("url") or node.get("path") or node.get("endpoint")
            method = node.get("method") or node.get("verb") or node.get("httpMethod")
            if url and method:
                key = (str(method).upper(), str(url))
                extra = {}
                for k in ("permissions", "permission", "scopes", "roles",
                          "bestPractice", "best_practice", "notes", "note",
                          "description", "summary", "rateLimit", "rate_limit"):
                    if k in node and node[k]:
                        extra[k] = node[k]
                if extra:
                    idx[key] = extra
            for v in node.values():
                visit(v)
        elif isinstance(node, list):
            for v in node:
                visit(v)

    visit(formatted)
    return idx


def params_table(params, spec):
    rows = []
    body_schema = None
    for p in params:
        if "$ref" in p:
            p = resolve_ref(spec, p["$ref"]) or p
        loc = p.get("in", "")
        if loc == "body":
            body_schema = p.get("schema")
            continue
        name = md_escape(p.get("name"))
        req = "yes" if p.get("required") else "no"
        typ = type_str(p.get("schema", p), spec)
        desc = md_escape(p.get("description"))
        rows.append(f"| `{name}` | {loc} | {typ} | {req} | {desc} |")
    out = ""
    if rows:
        out += "| Name | In | Type | Required | Description |\n"
        out += "|------|----|------|----------|-------------|\n"
        out += "\n".join(rows) + "\n\n"
    return out, body_schema


def responses_table(responses):
    if not responses:
        return ""
    rows = []
    for code, r in sorted(responses.items(), key=lambda kv: str(kv[0])):
        desc = md_escape(r.get("description") if isinstance(r, dict) else r)
        rows.append(f"| `{code}` | {desc} |")
    out = "| Code | Description |\n|------|-------------|\n"
    return out + "\n".join(rows) + "\n\n"


def render_operation(method, path, op, spec, fmt_idx):
    lines = []
    summary = op.get("summary") or op.get("operationId") or ""
    lines.append(f"### `{method.upper()}` `{path}`")
    if summary:
        lines.append(f"\n**{md_escape(summary)}**\n")
    desc = op.get("description")
    if desc:
        lines.append(desc.strip() + "\n")

    extra = fmt_idx.get((method.upper(), path))
    if extra:
        for label, keys in (("Permissions", ("permissions", "permission", "scopes", "roles")),
                            ("Rate limit", ("rateLimit", "rate_limit")),
                            ("Best practice", ("bestPractice", "best_practice", "notes", "note"))):
            for k in keys:
                if k in extra:
                    val = extra[k]
                    if isinstance(val, (list, tuple)):
                        val = ", ".join(map(str, val))
                    lines.append(f"> **{label}:** {md_escape(val)}\n")
                    break

    if op.get("deprecated"):
        lines.append("> ⚠️ **Deprecated**\n")

    table, body_schema = params_table(op.get("parameters", []), spec)
    if table:
        lines.append("**Parameters**\n")
        lines.append(table)

    if body_schema:
        lines.append("**Request body**\n")
        lines.append("```\n" + type_str(body_schema, spec) + "\n```\n")

    rt = responses_table(op.get("responses"))
    if rt:
        lines.append("**Responses**\n")
        lines.append(rt)

    return "\n".join(lines) + "\n---\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("swagger", help="raw swagger_*.json")
    ap.add_argument("formatted", nargs="?", help="formatted_swagger_*.json (optional)")
    ap.add_argument("-o", "--out", default="docs", help="output dir (default: docs)")
    args = ap.parse_args()

    spec = load(args.swagger)
    formatted = load(args.formatted) if args.formatted and os.path.exists(args.formatted) else None
    fmt_idx = build_formatted_index(formatted)

    os.makedirs(args.out, exist_ok=True)

    info = spec.get("info", {})
    base = spec.get("basePath", "")
    title = info.get("title", "SentinelOne API")
    version = info.get("version", "")

    tag_desc = {t.get("name"): t.get("description", "") for t in spec.get("tags", [])}

    by_tag = defaultdict(list)
    for path, item in spec.get("paths", {}).items():
        for method, op in item.items():
            if method.lower() not in HTTP_METHODS:
                continue
            tags = op.get("tags") or ["uncategorized"]
            by_tag[tags[0]].append((method, path, op))

    # per-category files
    index_rows = []
    for tag in sorted(by_tag):
        ops = sorted(by_tag[tag], key=lambda x: (x[1], x[0]))
        fname = f"{slug(tag)}.md"
        with open(os.path.join(args.out, fname), "w", encoding="utf-8") as f:
            f.write(f"# {tag}\n\n")
            if tag_desc.get(tag):
                f.write(tag_desc[tag].strip() + "\n\n")
            if base:
                f.write(f"Base path: `{base}`\n\n")
            f.write(f"{len(ops)} endpoints.\n\n")
            # TOC
            for method, path, op in ops:
                anchor = slug(f"{method}-{path}")
                f.write(f"- [`{method.upper()}` {path}](#{anchor})\n")
            f.write("\n")
            for method, path, op in ops:
                f.write(render_operation(method, path, op, spec, fmt_idx))
                f.write("\n")
        index_rows.append((tag, fname, len(ops)))

    # definitions appendix
    defs = spec.get("definitions", {})
    if defs:
        with open(os.path.join(args.out, "definitions.md"), "w", encoding="utf-8") as f:
            f.write("# Definitions / Models\n\n")
            for name in sorted(defs):
                d = defs[name]
                f.write(f"## <a id=\"def-{slug(name)}\"></a>{name}\n\n")
                if d.get("description"):
                    f.write(d["description"].strip() + "\n\n")
                props = d.get("properties", {})
                if props:
                    req = set(d.get("required", []))
                    f.write("| Field | Type | Required | Description |\n")
                    f.write("|-------|------|----------|-------------|\n")
                    for pn, ps in props.items():
                        pdesc = ps.get("description") if isinstance(ps, dict) else None
                        f.write(f"| `{pn}` | {type_str(ps, spec)} | "
                                f"{'yes' if pn in req else 'no'} | {md_escape(pdesc)} |\n")
                    f.write("\n")

    # README index
    total = sum(n for _, _, n in index_rows)
    with open(os.path.join(args.out, "README.md"), "w", encoding="utf-8") as f:
        f.write(f"# {title}\n\n")
        if version:
            f.write(f"Spec version: **{version}**  \n")
        if base:
            f.write(f"Base path: `{base}`  \n")
        f.write(f"\n{len(index_rows)} categories, {total} endpoints total.\n\n")
        f.write("## Categories\n\n")
        f.write("| Category | Endpoints | Doc |\n|----------|-----------|-----|\n")
        for tag, fname, n in sorted(index_rows, key=lambda x: -x[2]):
            f.write(f"| {tag} | {n} | [{fname}]({fname}) |\n")
        if defs:
            f.write(f"\nModels: [definitions.md](definitions.md) ({len(defs)})\n")

    print(f"OK: {len(index_rows)} categories, {total} endpoints -> {args.out}/")
    if not fmt_idx and args.formatted:
        print("NOTE: formatted file gave no matched extras (unexpected structure). "
              "Send me its top-level shape and I'll adjust the enrichment parser.")


if __name__ == "__main__":
    main()
