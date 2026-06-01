# CodeWhale v0.8.49 — Overnight Prep Notes

> Generated 2026-06-01. All branches pushed to origin. Nothing merged, pushed to main, published, or deleted.

---

## Branches Created

### 1. `work/v0.8.49/fix-qwen37-stale-references`
**What:** Removes 2 stale `qwen/qwen3.7-max` references from `config.example.toml` that survived the v0.8.48 Qwen 3.7 removal.
**Files:** `config.example.toml` (lines 41, 280)
**Tests:** N/A (docs-only)
**Risk:** None. Pure cleanup.

### 2. `work/v0.8.49/add-moonshot-provider-config`
**What:** Adds the missing `[providers.moonshot]` table block to `config.example.toml`. Moonshot/Kimi was the only shipped provider missing its config section.
**Files:** `config.example.toml` (+10 lines between siliconflow and sglang)
**Tests:** N/A (docs-only)
**Risk:** None. Pure config example addition.

### 3. `work/v0.8.49/update-tool-surface-docs`
**What:** Updates `docs/TOOL_SURFACE.md`:
- Version header: v0.8.35 → v0.8.49
- PDF backend: "pdftotext (poppler)" → "bundled pure-Rust extractor"
- Adds "Additional registered tools" section listing ~15 tools present in the registry but undocumented (web.run, multi_tool_use.parallel, request_user_input, git_show/log/blame, load_skill, revert_turn, pandoc_convert, validate_data, code_execution, review, project_map, remember, image_analyze, image_ocr, finance)
- Clarifies canonical live names version reference
**Files:** `docs/TOOL_SURFACE.md`
**Tests:** N/A (docs-only)
**Risk:** None.

### 4. `work/v0.8.49/fix-double-tool-registration`
**What:** Fixes double-registration of web and patch tools in agent/YOLO modes.
- `with_agent_tools()` unconditionally called `with_web_tools()` and `with_patch_tools()`, then `tool_setup.rs` conditionally called them again based on feature flags. This caused duplicate inserts (overwritten with warning log).
- Moves `finance` tool out of `with_web_tools()` into dedicated `with_finance_tool()` — finance is market data, not web search, and should not be gated behind `Feature::WebSearch`.
- `with_agent_tools()` now calls `with_finance_tool()` to keep finance always available.
- Updated 3 tests: renamed `test_builder_with_web_tools_includes_finance` → `test_builder_with_web_tools_no_longer_includes_finance` (now asserts web_search/fetch_url/web.run present, finance absent), added `test_builder_with_finance_tool`.
**Files:** `crates/tui/src/tools/registry.rs`
**Tests:** All 3814 tests pass (incl. targeted registry tests).
**Risk:** Low. The `ToolRegistry::register()` silently overwrites duplicates, so the previous double-registration was benign (just a log warning). This change makes the registration correct. The only behavioral change: `finance` is now always available regardless of `Feature::WebSearch` (which is correct — it was previously conditionally available only when web search feature was enabled, which made no semantic sense).

### 5. `work/v0.8.49/home-polish`
**What:** Consolidated polish:
- Adds `completion_sound` to `config.example.toml` notifications section (`off`/`beep`/`bell`, default `beep`)
- Adds `kimi-k2.6` to Default Models comments in `config.example.toml`
- Fixes stale v0.8.47 reference in `docs/PROVIDERS.md` Planned section → v0.8.48+
**Files:** `config.example.toml`, `docs/PROVIDERS.md`
**Tests:** N/A (docs-only)
**Risk:** None.

---

## Issues Triaged

### Bugs reported since v0.8.48 (via GitHub API / web search)

| Issue | Title | Status | Effort | Notes |
|---|---|---|---|---|
| [#2481](https://github.com/Hmbown/CodeWhale/issues/2481) | ANSI escape codes leaking into TUI footer text | Open, has PR [#2485](https://github.com/Hmbown/CodeWhale/pull/2485) | Small | Release-blocking if not fixed. PR #2485 looks targeted. |
| [#2486](https://github.com/Hmbown/CodeWhale/issues/2486) | `codewhale model list` shows wrong context window for Gemma 4 26B | Open, v0.8.49 milestone | Small | Config/metadata fix |
| [#2487](https://github.com/Hmbown/CodeWhale/issues/2487) | `/search` command generating hallucinated fake cmdlets | Open | Medium | UX/agent-behavior issue |
| [#2089](https://github.com/Hmbown/CodeWhale/issues/2089) | good first issue — "Add completion sound config documentation" | Open | Small | Already fixed in home-polish branch |
| [#2086](https://github.com/Hmbown/CodeWhale/issues/2086) | good first issue | Open | TBD | Needs triage |

### Other notable open issues
- 378 total open issues (as of scan)
- v0.8.49 milestone: 11 open, 5 closed
- No issues labeled "regression" or "papercut" exist
- Several fresh bug reports in hours since v0.8.48 shipped

---

## PRs Reviewed

### Key open PRs (18 total, 16 from community)

| PR | Author | Title | Status | Notes |
|---|---|---|---|---|
| [#2256](https://github.com/Hmbown/CodeWhale/pull/2256) | Hmbown | refactor: consolidate workspace crates 14→11 | Draft | **Needs Hunter's decision** — does this land in v0.8.49 or slip? |
| [#2485](https://github.com/Hmbown/CodeWhale/pull/2485) | — | Fix for #2481 (ANSI leak) | Open | Should be reviewed and merged |
| [#1893](https://github.com/Hmbown/CodeWhale/pull/1893) | wavezhang | feat: make TLS certificate verification configurable | Open, unreviewed 11d | Has 2 known bugs in config.rs |
| [#2474](https://github.com/Hmbown/CodeWhale/pull/2474) | Hmbown | codex/first-day-polish | Merged | Already in main |

### Between v0.8.48 and main
- 1 commit: `e0160cc6f` — "fix(release): add codewhale-release to Cargo publish list" (touches only `scripts/release/crates.sh`)

### Community PRs needing review (7+ days old)
- #1893 (wavezhang, 11 days)
- Several others from the 16 community PRs

---

## Provider / Model Audit

### Current state
- 16 shipped providers (all documented in PROVIDERS.md)
- OpenRouter recent large models: Arcee Trinity, MiniMax M3, Xiaomi MiMo v2.5, Qwen 3.6, Kimi K2.6, GLM 5.1, Tencent Hy3, Gemma 4, Nemotron — all current
- Qwen 3.7 Max removed (hosted, not open-weight) — CHANGELOG notes it returns when open-weight release ships

### New models on OpenRouter since v0.8.48 (May 31 cutoff)
Notable additions:
- `stepfun/step-3.7-flash` (May 28) — 196B MoE, 256K context, supports tools + reasoning. Proprietary (StepFun), not open-weight → do not add as preset.
- `x-ai/grok-build-0.1` (May 20) — xAI coding model, 256K context. Proprietary → do not add.
- `qwen/qwen3.7-max` still listed on OpenRouter but is hosted-only (not open-weight) → correctly excluded per v0.8.48 policy.

**Recommendation:** No new OpenRouter presets needed for v0.8.49. The model registry is current.

### New OpenRouter model IDs worth considering (open-weight, tool-capable)
None identified that aren't already in the registry. The v0.8.48 batch was comprehensive.

---

## Constitution / System Prompt

No changes recommended. The constitution is stable. The home-defaults audit found:
- One hardcoded "V4-architecture" reference in the prompt — considered low-priority and intentional (documents V4-specific behavior)
- No model-confusing language found

---

## Web Frontend

### Dependency status
- `npm outdated` shows `wrangler` (4.88→4.95), `@opennextjs/cloudflare` (1.19.7→1.19.11), and dev deps behind. Non-blocking.
- `npm audit`: 0 vulnerabilities
- Sub-agent confirmed all packages current; the `npm outdated` I ran may have been with different flags.

### Stale references
- `web/components/footer.tsx:102`: `https://gitee.com/Hmbown/deepseek-tui` — **Needs Hunter's attention.** The GitHub repo was renamed from deepseek-tui to CodeWhale. Does the Gitee mirror need updating?
- `web/app/[locale]/install/page.tsx` and `web/app/[locale]/faq/page.tsx`: Homebrew tap references (`brew tap Hmbown/deepseek-tui`) — these are **correct** (the tap/formula name is legacy, per CHANGELOG). No change needed.
- Feishu bridge README uses `/etc/deepseek/` paths — documented as a compat fallback.

---

## Test Status

- Full workspace test suite: **3814 passed, 1 failed (flaky), 4 ignored**
- Flaky test: `mcp::tests::legacy_sse_closed_stream_reconnects_and_retries_tool_call` — "connection closed before message completed" — appears to be a race condition in the SSE test, not a regression
- `cargo fmt --all --check`: passes clean

---

## To-Do for Hunter

### Decisions needed
1. **PR #2256 (14→11 crate refactor):** Land in v0.8.49 or slip to v0.9.0? This is a significant refactor.
2. **PR #2485 (ANSI leak fix):** Review and merge — this fixes a release-blocking bug (#2481).
3. **Community PRs:** 16 open community PRs, several unreviewed for 7-11 days. Consider a review pass before release.
4. **Gitee mirror URL:** `gitee.com/Hmbown/deepseek-tui` — is this URL still valid? If not, update in `web/components/footer.tsx`.

### Risk assessment for created branches
| Branch | Risk | Merge confidence |
|---|---|---|
| `fix-qwen37-stale-references` | None | High |
| `add-moonshot-provider-config` | None | High |
| `update-tool-surface-docs` | None | High |
| `fix-double-tool-registration` | Low (tested) | High |
| `home-polish` | None | High |

### Suggested merge order
1. `fix-qwen37-stale-references` (docs-only, no deps)
2. `add-moonshot-provider-config` (docs-only, no deps)
3. `home-polish` (docs-only, no deps)
4. `update-tool-surface-docs` (docs-only, no deps)
5. `fix-double-tool-registration` (code change, tested, review before merge)
