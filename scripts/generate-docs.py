#!/usr/bin/env python3
"""Generate gnotes documentation via cloud LLM (NVIDIA → OpenRouter → DeepSeek).

Loads keys from project .env (gitignored; copy from ORGANIZATION/.env).
Usage: python3 scripts/generate-docs.py [--provider nvidia|openrouter|deepseek]
"""
from __future__ import annotations

import argparse
import json
import os
import ssl
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SECTIONS = {
    "README": {
        "path": "README.md",
        "max_tokens": 2000,
        "prompt": """Write README.md for gnotes (markdown only, no preamble).
~70 lines max. Include: tagline, v0.1 alpha status, bullet list of what works TODAY,
quick start (clone https://github.com/davidthegnomad/gnotes, npm install, cargo tauri dev),
prerequisites (Rust, Node 18+, Tauri CLI v2), tech stack table, links to docs/OVERVIEW.md and AI.txt.
Do NOT mention API keys or .env.""",
    },
    "OVERVIEW": {
        "path": "docs/OVERVIEW.md",
        "max_tokens": 3000,
        "prompt": """Write docs/OVERVIEW.md for gnotes (markdown only, no preamble).
~120 lines. Sections: Product, Architecture diagram (ascii), Multi-window model,
Implemented (v0.1 alpha checklist), Roadmap (planned from PLAN.md), Dev Setup,
Folder Layout (tree), Contributing, License TBD.
Be precise: only mark features implemented if they exist in code.""",
    },
    "AI_TXT": {
        "path": "AI.txt",
        "max_tokens": 4500,
        "prompt": """Write AI.txt for coding agents (plain text only, no preamble).
Use ALL_CAPS section headers. Include: IDENTITY, REPO, PATHS, STACK, MULTI-WINDOW MODEL,
ALL TAURI COMMANDS (with param names), SQLITE SCHEMA, FILE RESPONSIBILITIES,
STATE FLOW (numbered), CONVENTIONS, IMPLEMENTED vs PLANNED, EXTENSION RECIPES,
PITFALLS. Exhaustive and factual from the codebase. No API keys.""",
    },
}


def load_dotenv(path: Path) -> None:
    if not path.is_file():
        return
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        k, _, v = line.partition("=")
        os.environ.setdefault(k.strip(), v.strip().strip('"').strip("'"))


def ssl_ctx():
    try:
        import certifi

        return ssl.create_default_context(cafile=certifi.where())
    except ImportError:
        return ssl.create_default_context()


def chat(
    base_url: str,
    api_key: str,
    model: str,
    system: str,
    user: str,
    max_tokens: int,
    timeout: int,
) -> str:
    body = {
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user},
        ],
        "max_tokens": max_tokens,
        "temperature": 0.2,
    }
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
    }
    if "openrouter" in base_url:
        headers["HTTP-Referer"] = "https://github.com/davidthegnomad/gnotes"
        headers["X-Title"] = "gnotes docgen"
    req = urllib.request.Request(
        f"{base_url.rstrip('/')}/chat/completions",
        data=json.dumps(body).encode(),
        headers=headers,
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=timeout, context=ssl_ctx()) as resp:
        data = json.loads(resp.read())
    msg = data["choices"][0]["message"]
    return (msg.get("content") or msg.get("reasoning_content") or "").strip()


def codebase_context() -> str:
    files = [
        "README.md", "PLAN.md", "package.json", "src-tauri/Cargo.toml",
        "src-tauri/tauri.conf.json", "src-tauri/src/lib.rs",
        "src-tauri/src/commands/notes.rs", "src-tauri/src/db/migrations.rs",
        "src/App.tsx", "src/stores/noteStore.ts", "src/components/NoteWindow.tsx",
        "src/components/Editor.tsx", "src/components/TitleBar.tsx", "src/components/Toolbar.tsx",
    ]
    parts = []
    for f in files:
        p = ROOT / f
        if p.is_file():
            parts.append(f"=== {f} ===\n{p.read_text(errors='replace')[:6000]}")
    return "\n\n".join(parts)


PROVIDERS = [
    {
        "name": "nvidia",
        "base_url": "https://integrate.api.nvidia.com/v1",
        "key_envs": ["NVIDIA_NIM_API_KEY", "NVIDIA_API_KEY"],
        "model": "google/gemma-3n-e2b-it",
        "timeout": 120,
        "note": "gemma-4-31b-it times out on NIM; use 3n e2b for reliable docgen",
    },
    {
        "name": "openrouter",
        "base_url": "https://openrouter.ai/api/v1",
        "key_envs": ["OPENROUTER_API_KEY"],
        "model": "meta-llama/llama-3.3-70b-instruct:free",
        "timeout": 120,
    },
    {
        "name": "deepseek",
        "base_url": "https://api.deepseek.com",
        "key_envs": ["DEEPSEEK_API_KEY"],
        "model": "deepseek-chat",
        "timeout": 120,
    },
]


def resolve_providers(preferred: str | None) -> list[dict]:
    pool = PROVIDERS if not preferred else [p for p in PROVIDERS if p["name"] == preferred]
    out: list[dict] = []
    for p in pool:
        for env in p["key_envs"]:
            if os.environ.get(env):
                out.append({**p, "api_key": os.environ[env]})
                break
    if not out:
        raise SystemExit("No API key found. Copy keys from ORGANIZATION/.env into gnotes/.env")
    return out


def generate_section(providers: list[dict], system: str, ctx: str, spec: dict) -> tuple[str, str, str]:
    user = f"{spec['prompt']}\n\nCODEBASE:\n{ctx[:35000]}"
    errors: list[str] = []
    for p in providers:
        try:
            text = chat(
                p["base_url"],
                p["api_key"],
                p["model"],
                system,
                user,
                spec["max_tokens"],
                p.get("timeout", 120),
            )
            if text:
                return text, p["name"], p["model"]
        except Exception as exc:  # noqa: BLE001
            errors.append(f"{p['name']}: {exc}")
    raise RuntimeError("; ".join(errors))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--provider", choices=["nvidia", "openrouter", "deepseek"])
    args = parser.parse_args()

    load_dotenv(ROOT / ".env")
    providers = resolve_providers(args.provider)
    ctx = codebase_context()
    system = (
        "You are a senior technical writer for open-source desktop apps. "
        "Distinguish IMPLEMENTED features from PLANNED. Output only the requested document."
    )

    log_dir = ROOT / ".agents" / "LOGS"
    log_dir.mkdir(parents=True, exist_ok=True)
    results: dict[str, str] = {}
    meta: list[dict] = []

    for key, spec in SECTIONS.items():
        print(f"Generating {key}...", file=sys.stderr)
        text, provider, model = generate_section(providers, system, ctx, spec)
        results[key] = text
        meta.append({"section": key, "provider": provider, "model": model, "chars": len(text)})
        (log_dir / f"docgen-{key.lower()}.txt").write_text(text)

        rel = spec["path"]
        out_path = ROOT / rel
        if "/" in rel:
            out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(text + "\n")

    (log_dir / "docgen-meta.json").write_text(json.dumps(meta, indent=2))
    print(json.dumps({"ok": True, "sections": list(results.keys()), "meta": meta}))


if __name__ == "__main__":
    main()
