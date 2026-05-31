# Feishu / Lark Bridge

This bridge lets a Feishu or Lark chat control a local `codewhale serve --http`
runtime from a phone. It uses the official Lark/Feishu Node SDK long-connection
mode, so the first version does not need a public webhook URL.

Security model:

- `codewhale serve --http` stays bound to `127.0.0.1`.
- `/v1/*` runtime calls use `CODEWHALE_RUNTIME_TOKEN`; the legacy
  `DEEPSEEK_RUNTIME_TOKEN` alias is still accepted.
- Feishu/Lark chats must be allowlisted unless `CODEWHALE_ALLOW_UNLISTED=true`
  is set for first pairing; `DEEPSEEK_ALLOW_UNLISTED=true` remains a legacy
  alias.
- Direct messages are the intended MVP control surface. Group chat control is
  disabled unless `FEISHU_ALLOW_GROUPS=true`.
- Tool approvals are text commands: `/allow <approval_id>` or `/deny <approval_id>`.

## Setup

```bash
cd /opt/codewhale/bridge
npm install --omit=dev
sudo mkdir -p /etc/codewhale
cp .env.example /etc/codewhale/feishu-bridge.env
sudoedit /etc/codewhale/feishu-bridge.env
node src/index.mjs
```

Validate the env files before starting the service:

```bash
npm run validate:config -- \
  --env /etc/codewhale/feishu-bridge.env \
  --runtime-env /etc/codewhale/runtime.env \
  --workspace-root /opt/whalebro \
  --check-filesystem
```

Existing deployments using `/etc/deepseek/*.env` can keep those paths during
the rename window.

For a Tencent Lighthouse deployment, use:

```bash
sudo systemctl enable --now codewhale-runtime codewhale-feishu-bridge
sudo journalctl -u codewhale-feishu-bridge -f
```

## Commands

- `/status`
- `/threads`
- `/new`
- `/resume <thread_id>`
- `/interrupt`
- `/compact`
- `/allow <approval_id> [remember]`
- `/deny <approval_id>`

Anything else is sent as a prompt. If group control is explicitly enabled,
messages must start with `/ds` by default, for example:

```text
/ds check git status and tell me what is dirty
```
