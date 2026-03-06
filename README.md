# Cortex-OS — AI System Control Agent

<p align="center">
  <strong>Control your entire computer with natural language.</strong><br>
  An open-source, privacy-first AI agent that plans, executes, and verifies complex multi-step tasks.
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#security">Security</a>
</p>

---

## Why Cortex-OS?

Unlike simple command runners, Cortex-OS is a **true agentic system** with a Plan → Execute → Verify → Retry pipeline:

1. **Planner** — LLM converts your natural language into a structured multi-step action plan
2. **Executor** — Each action is dispatched to native OS APIs (never GUI automation)
3. **Verifier** — Post-execution verification confirms the action succeeded
4. **Auto-Fix** — If generated code fails, the LLM automatically fixes and retries it
5. **Security** — Five-tier permission system with confirmation gates and rollback support

## 🧪 Tested With 10 Complex Tasks — 80%+ Pass Rate

| Task | Type | Status |
|------|------|--------|
| Web scrape Wikipedia + word frequency analysis | Web + Code | ✅ |
| Background CPU trigger with voice alert | System Monitor | ✅ |
| Screenshot OCR + text reversal + file tree | Vision + Code | ✅ |
| Multi-page web comparison (Python vs JS) | Web + Analysis | ✅ |
| Create project scaffold + run unit tests | File + Code | ✅ |
| REST API fetch + JSON parse + formatted table | API + Code | ✅ |
| CSV data pipeline + financial analysis | Data + Code | ✅ |
| And more... | | ✅ |

## 🖥️ Cross-Platform Support

| Platform | Status |
|----------|--------|
| Windows 10/11 | ✅ Full support |
| Ubuntu / Debian | ✅ Full support |
| macOS | ✅ Full support |
| Fedora / Arch | ✅ Via dnf/pacman |

## ⚡ 50+ Action Types

### File Operations
`file_read` · `file_write` · `file_delete` · `file_move` · `file_copy` · `file_list` · `file_search` · `file_permissions`

### Process Management
`process_list` · `process_kill` · `process_info`

### Shell Execution
`shell_command` · `shell_script` (multi-line bash/powershell/python)

### Code Execution
`code_execute` — Run Python, PowerShell, Bash, or JavaScript with auto-fix on failure

### Browser & Web
`browser_navigate` · `browser_extract` · `browser_extract_table` · `browser_extract_links`

### Screen & Vision
`screenshot` · `screen_ocr` · `screen_analyze`

### Package Management
`package_install` · `package_remove` · `package_update` · `package_search`
Auto-detects: winget, choco, brew, apt, dnf, pacman

### System Information
`system_info` · `cpu_usage` · `memory_usage` · `disk_usage` · `network_info` · `battery_info`

### Window Management
`window_list` · `window_focus` · `window_close` · `window_minimize` · `window_maximize`

### Audio / Volume
`volume_get` · `volume_set` · `volume_mute`

### Display / Screen
`brightness_get` · `brightness_set` · `screenshot`

### Power Management
`power_shutdown` · `power_restart` · `power_sleep` · `power_lock` · `power_logout`

### Network / WiFi
`wifi_list` · `wifi_connect` · `wifi_disconnect`

### Clipboard
`clipboard_read` · `clipboard_write`

### Scheduled Tasks & Triggers
`schedule_create` · `schedule_list` · `schedule_delete` · `trigger_create`

### Environment Variables
`env_get` · `env_set` · `env_list`

### Downloads
`download_file`

### Service Management (Linux)
`service_start` · `service_stop` · `service_restart` · `service_enable` · `service_disable` · `service_status`

### GNOME / Desktop (Linux)
`gnome_setting_read` · `gnome_setting_write` · `dbus_call`

### Windows Registry
`registry_read` · `registry_write`

### Open / Launch / Notify
`open_url` · `open_application` · `notify`

## Architecture

```
cortex-os/
├── daemon/              # Python backend (agent system)
│   └── pilot/
│       ├── agents/      # Planner, Executor, Verifier, Code Sanitizer
│       ├── models/      # Cloud (Gemini/OpenAI/Claude) + Local (Ollama) LLM routing
│       ├── security/    # Encrypted vault, permission system, audit log
│       └── system/      # OS interface modules (files, processes, network, etc.)
├── tauri-app/           # Desktop GUI (Svelte 5 + Tauri v2)
│   ├── ui/              # Frontend (Svelte + Vite)
│   └── src-tauri/       # Rust backend
└── schemas/             # Shared JSON schemas for action validation
```

## Requirements

- **Python 3.11+**
- **Ollama** (for local LLM inference) OR a cloud API key (OpenAI, Claude, Gemini)
- **Rust toolchain** (for building the Tauri desktop app)
- **Node.js 20+** (for building the Svelte frontend)

## Quick Start

### 1. Install the Python daemon

```bash
cd daemon
pip install -e ".[full,dev]"
```

### 2. Choose your LLM

**Option A: Local (Ollama — free, private)**
```bash
ollama pull llama3.1:8b
ollama serve
```

**Option B: Cloud (Gemini / OpenAI / Claude)**
Set your API key through the app UI or config file.

### 3. Run the daemon

```bash
cd daemon
python -m pilot.server
```

### 4. Run the desktop app

```bash
cd tauri-app/ui
npm install
npm run dev
```

Or build the full Tauri desktop app:
```bash
cd tauri-app/src-tauri
cargo tauri dev
```

## Example Commands

```
"Show me my system info"
"Take a screenshot and read the text on screen"
"Go to Wikipedia's page on AI and summarize the first 3 paragraphs"
"Create a Python project with tests and run them"
"Kill the process using the most CPU"
"Monitor my CPU and alert me when it goes above 80%"
"Download a file and show me a tree of the folder"
"List all .py files on my Desktop"
"Set my volume to 50%"
"Create a CSV with sales data and analyze it"
"What's my IP address?"
"Install Firefox"
```

## Security

- All AI outputs pass through structured schema validation before execution
- Five-tier permission system (read-only through root-level)
- Confirmation required for system-modifying and destructive actions
- Snapshot-based rollback via Btrfs or Timeshift (Linux)
- Append-only audit log for all executed actions
- Command whitelist with optional unrestricted mode
- **Encrypted API key storage** via platform keyring (GNOME Keyring / Windows Credential Manager)
- API keys are NEVER logged, included in plans, or sent to local LLMs

### Permission Tiers

| Tier | Level | Auto-Execute | Examples |
|------|-------|-------------|----------|
| 0 - Read Only | 🟢 | Yes | file_read, system_info, clipboard_read |
| 1 - User Write | 🟡 | Yes | file_write, clipboard_write, env_set |
| 2 - System Modify | 🟠 | Needs Confirm | package_install, service_restart, wifi_connect |
| 3 - Destructive | 🔴 | Needs Confirm | file_delete, process_kill, power_shutdown |
| 4 - Root Critical | ⛔ | Needs Confirm | root operations, disk operations |

## Configuration

Config file: `~/.config/pilot/config.toml`

```toml
[model]
provider = "ollama"           # "ollama" | "cloud"
ollama_model = "llama3.1:8b"
cloud_provider = "gemini"     # "gemini" | "openai" | "claude"

[security]
root_enabled = false
confirm_tier2 = true
unrestricted_shell = false
snapshot_on_destructive = true

[server]
host = "127.0.0.1"
port = 8785
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a Pull Request

## License

MIT
