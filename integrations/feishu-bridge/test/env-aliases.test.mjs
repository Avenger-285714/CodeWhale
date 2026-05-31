import test from "node:test";
import assert from "node:assert/strict";

import { applyCodewhaleEnvAliases, validateBridgeConfig } from "../src/lib.mjs";

test("CODEWHALE_* names backfill their legacy DEEPSEEK_* equivalents", () => {
  const env = applyCodewhaleEnvAliases({
    CODEWHALE_RUNTIME_TOKEN: "tok",
    CODEWHALE_RUNTIME_URL: "http://127.0.0.1:7878",
    CODEWHALE_WORKSPACE: "/opt/whalebro",
  });
  assert.equal(env.DEEPSEEK_RUNTIME_TOKEN, "tok");
  assert.equal(env.DEEPSEEK_RUNTIME_URL, "http://127.0.0.1:7878");
  assert.equal(env.DEEPSEEK_WORKSPACE, "/opt/whalebro");
});

test("an explicit DEEPSEEK_* value is never overwritten by its alias", () => {
  const env = applyCodewhaleEnvAliases({
    CODEWHALE_RUNTIME_TOKEN: "from-codewhale",
    DEEPSEEK_RUNTIME_TOKEN: "from-deepseek",
  });
  assert.equal(env.DEEPSEEK_RUNTIME_TOKEN, "from-deepseek");
});

test("legacy-only configs are untouched (non-breaking)", () => {
  const env = applyCodewhaleEnvAliases({ DEEPSEEK_RUNTIME_TOKEN: "legacy" });
  assert.equal(env.DEEPSEEK_RUNTIME_TOKEN, "legacy");
  assert.equal(env.CODEWHALE_RUNTIME_TOKEN, undefined);
});

test("the DeepSeek provider key accepts a CODEWHALE_API_KEY convenience alias", () => {
  const env = applyCodewhaleEnvAliases({ CODEWHALE_API_KEY: "sk-deepseek" });
  assert.equal(env.DEEPSEEK_API_KEY, "sk-deepseek");
});

test("validateBridgeConfig accepts a fully CODEWHALE_*-named bridge config", () => {
  const result = validateBridgeConfig({
    FEISHU_APP_ID: "cli_abc",
    FEISHU_APP_SECRET: "secret-value",
    CODEWHALE_RUNTIME_URL: "http://127.0.0.1:7878",
    CODEWHALE_RUNTIME_TOKEN: "a-real-token",
    CODEWHALE_WORKSPACE: "/opt/whalebro",
    FEISHU_THREAD_MAP_PATH: "/var/lib/codewhale-feishu-bridge/thread-map.json",
  });
  // No missing_required errors for the runtime vars supplied via CODEWHALE_*.
  const missing = result.errors.filter((e) => e.code === "missing_required").map((e) => e.message);
  assert.deepEqual(missing, [], `unexpected missing-required: ${missing.join(", ")}`);
});
