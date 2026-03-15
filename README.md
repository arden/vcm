# VCM - Vibe Coding Manager

> One command to manage all your AI coding tools.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**VCM** is a CLI tool manager designed specifically for AI coding assistants. It helps you discover, install, configure, and manage all your AI coding tools in one place.

[中文文档](README_zh.md)

---

## Why VCM?

If you use AI coding tools like Claude Code, Cursor, Gemini CLI, or others, you might have experienced these pain points:

- **Scattered tools**: Different tools have different installation methods
- **Forgotten tools**: Installed tools but forgot what's available
- **Messy configs**: API keys scattered across different config files
- **Unknown status**: Not sure which tools are configured or need updates
- **No comparison**: Hard to compare different tools and their features

**VCM solves all of these problems.**

---

## Features

### Core Capabilities

| Feature | Description |
|---------|-------------|
| **Scan** | Automatically detect all installed AI coding tools |
| **List** | View all known AI coding tools with detailed info |
| **Install** | One-click install with automatic package manager detection |
| **Status** | Check configuration status and API key availability |
| **Config** | Interactive API key configuration wizard |
| **Update** | Keep all tools up to date |
| **Free** | Find tools with free AI models and pro-grade access |

### Supported Tools

VCM supports 30+ AI coding tools including:

| Vendor | Tools |
|--------|-------|
| **Anthropic** | Claude Code |
| **OpenAI** | OpenAI CLI |
| **Google** | Gemini CLI |
| **GitHub** | Copilot CLI |
| **Amazon** | Kiro, CodeWhisperer |
| **Cursor** | Cursor Editor |
| **Codeium** | Windsurf |
| **Open Source** | Aider, OpenCode, Ollama, Kilo Code, Goose |
| **Sourcegraph** | Amp, Cody |
| **Augment** | Augment Code |
| **And more...** | |

### Multi-language Support

- **English** (default)
- **中文** (Chinese)

---

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/arden/vcm.git
cd vcm

# Build and install
cargo build --release
cargo install --path .
```

### Prerequisites

- Rust 1.70+ (for building from source)
- A package manager: npm, pip, cargo, or brew

---

## Quick Start

### 1. Scan Your System

Discover what AI coding tools you already have installed:

```bash
vcm scan
```

Output:
```
🔍 Scanning installed CLI AI tools...

✓ Claude Code    v2.1.19
✓ Cursor         v2.6.19
✓ Kiro           v0.10.78
✓ OpenCode       v1.2.21

Found 4 installed tools
```

### 2. List Available Tools

See all supported AI coding tools:

```bash
vcm list
```

Output:
```
📋 CLI AI Programming Tools (35 total)

Installed (4)
  ✓ claude-code    Anthropic    Terminal AI coding assistant
  ✓ cursor         Cursor Inc.  AI-powered code editor

Recommended
  ○ gemini-cli     Google       Open-source terminal AI agent
  ○ aider          Open Source  AI pair programming tool
  ○ kilo           Kilo         Open-source AI coding agent

Other Tools
  ○ copilot-cli    GitHub       Terminal AI assistant
  ○ opencode       Open Source  Supports 75+ LLM providers
  ...
```

### 3. Check Tool Status

See which tools need configuration:

```bash
vcm status
```

Output:
```
📊 Tool Status

Tool            Version      Status     Note
────────────────────────────────────────────────────────────
Claude Code     2.1.19       ⚠ Warning  Missing: ANTHROPIC_API_KEY
Cursor          2.6.19       ✓ Healthy
OpenCode        1.2.21       ✓ Healthy

Completion: 2/3 (66%)

Suggestion: Run `vcm config <tool>` to configure
```

### 4. Configure API Keys

Interactive configuration wizard:

```bash
vcm config claude-code
```

### 5. Install New Tools

```bash
vcm install gemini-cli
```

### 6. Launch a Tool

```bash
vcm run claude-code
```

---

## Commands Reference

### Basic Commands

| Command | Description |
|---------|-------------|
| `vcm scan` | Scan for installed AI coding tools |
| `vcm list` | List all known tools |
| `vcm install <tool>` | Install a tool |
| `vcm remove <tool>` | Remove a tool |
| `vcm update [tool]` | Update tool(s) |
| `vcm config [tool]` | Configure tool (API keys) |
| `vcm status` | Check tool status |
| `vcm info <tool>` | Show tool details |
| `vcm search <query>` | Search tools |
| `vcm run <tool>` | Launch a CLI tool |
| `vcm outdated` | Check for updates |
| `vcm free [--pro]` | Find tools with free AI models |
| `vcm doctor` | System diagnostics |
| `vcm init` | Interactive setup wizard |
| `vcm usage` | Show usage statistics |
| `vcm lang [en\|zh]` | Set language |

### v2.0 New Commands

| Command | Description |
|---------|-------------|
| `vcm alias <name> <tool>` | Set tool alias for quick launch |
| `vcm compare <t1> <t2>...` | Compare multiple tools side-by-side |
| `vcm free --aggregate` | Aggregate all free quotas |
| `vcm quota status` | Monitor quota usage |
| `vcm stats` | Show usage statistics |
| `vcm cost` | Estimate API costs |
| `vcm project init` | Initialize project-level config |
| `vcm fallback add <primary> <fallback>` | Set up fallback chain |
| `vcm key add <tool> <name> <key>` | Manage multiple API keys |
| `vcm recommend` | Get personalized recommendations |
| `vcm trending` | Show trending tools |
| `vcm new` | Show newly added tools |

### Global Options

```
-v, --verbose    Verbose output
-j, --json       JSON output format
```

---

## Language Settings

VCM supports English and Chinese. Switch languages using:

```bash
# Set to Chinese
vcm lang zh

# Set to English
vcm lang en

# Or use environment variable
VCM_LANG=zh vcm scan
```

---

## Configuration

### Config Directory

VCM stores its configuration in `~/.config/vcm/`:

```
~/.config/vcm/
├── config.toml      # User settings
├── state.json       # Runtime state
└── registry.yaml    # Tool registry cache
```

### Config File Structure

```toml
# ~/.config/vcm/config.toml

[settings]
language = "en"
default_tool = "claude-code"
```

---

## Find Free AI Models

VCM helps you discover tools that offer free access to pro-grade AI models. This is perfect for developers who want to use powerful AI coding assistants without paying for API access.

### List Tools with Free Models

```bash
vcm free
```

Output:
```
🎁 Tools with Free AI Models

★ Gemini CLI [Best Free Choice!]
  Free Limit: 100 requests/day Gemini 2.5 Pro
  Free Models:
    ● Gemini 2.5 Pro [Pro-Grade] - 63.2% SWE-bench
    ● Gemini 3 Pro [Pro-Grade] - 76.2% SWE-bench (waitlist)
  Card Required: No card
  Note: Best free tier for pro-grade models!

★ Ollama [Best Free Choice!]
  Free Limit: Unlimited - runs locally
  Free Models:
    ● Qwen2.5-Coder-32B [Pro-Grade] - Excellent for coding
    ● DeepSeek Coder V2 [Pro-Grade] - Great for coding
  Card Required: No card
  Note: 100% FREE - runs on your hardware!
```

### Show Only Tools with Free Pro-Grade Models

```bash
vcm free --pro
```

### Best Free Options

| Tool | Free Limit | Pro-Grade Models | Card Required |
|------|-----------|------------------|---------------|
| **Gemini CLI** | 100 req/day | Gemini 2.5 Pro, Gemini 3 Pro | No |
| **Ollama** | Unlimited local | Qwen2.5-Coder, DeepSeek Coder | No |
| **Aider** | BYOK/Local | Qwen3-Coder (OpenRouter) | No |
| **Kiro** | 50 credits/mo | Claude 4 Sonnet | No |
| **OpenCode** | BYOK | Gemini 2.5 Pro, Qwen3-Coder | No |
| **Kilo Code** | $25 signup | Claude Opus 4.5, GPT-4.1 | Yes |

### What is a Pro-Grade Model?

Models with **≥60% on SWE-bench Verified** are considered pro-grade for real-world coding tasks:

| Model | SWE-bench | Provider |
|-------|-----------|----------|
| Claude Opus 4.5 | 80.9% | Anthropic |
| GPT-5.1-Codex-Max | 77.9% | OpenAI |
| Gemini 3 Pro | 76.2% | Google |
| Gemini 2.5 Pro | 63.2% | Google |
| Qwen3-Coder-480B | 69.6% | Alibaba |

---

## v2.0 New Features

### Tool Aliases

Set short aliases for quick tool launch:

```bash
# Set alias
vcm alias cc claude-code

# Now you can use
vcm cc
```

### Tool Comparison

Compare multiple tools side-by-side:

```bash
vcm compare gemini-cli claude-code aider
```

Output:
```
工具对比
══════════════════════════════════════════════════════════════════
特性          │Gemini CLI        │Claude Code       │Aider
────────────────────────────────────────────────────────────────────
供应商         │Google            │Anthropic         │Open Source
免费额度        │100 requests/...  │付费               │Unlimited...
专业模型        │Gemini 2.5 Pro... │-                  │Qwen3-Coder...
需信用卡        │否                 │否                 │否
```

### Free Quota Aggregation

See all your free quotas in one place:

```bash
vcm free --aggregate
```

Output:
```
🎁 免费额度聚合面板

工具                   免费额度                      专业级模型                状态
────────────────────────────────────────────────────────────────────────
Gemini CLI           100 requests/day            Gemini 2.5 Pro       ✓ 已安装
Kiro                 50 credits/month            Claude 4 Sonnet      ✓ 已安装
Ollama               Unlimited - runs locally    Qwen2.5-Coder        ✓ 已安装

📊 聚合统计
  • 有免费额度的工具: 9 个
  • 提供专业级模型的工具: 9 个
  • 可免费使用的专业级模型: 19 个
```

### Quota Tracking

Monitor your usage and set alerts:

```bash
# View quota status
vcm quota status

# Set warning threshold (80%)
vcm quota warn 80

# Set hard limit (100%)
vcm quota limit 100
```

### Usage Statistics & Cost Estimation

Track your tool usage:

```bash
# Show usage statistics
vcm stats

# Estimate API costs
vcm cost
```

### Project-Level Configuration

Configure tools per project:

```bash
# Initialize project config
vcm project init

# Set default tool for project
vcm project use claude-code --model claude-sonnet-4

# View project config
vcm project status
```

Creates `.vcm/config.toml` in your project:
```toml
name = "my-project"
default_tool = "claude-code"

[tools.claude-code]
model = "claude-sonnet-4"
```

### Smart Fallback Chain

Automatically switch to backup tools when primary fails:

```bash
# Add fallback chain: claude-code → gemini-cli → ollama
vcm fallback add claude-code gemini-cli ollama

# Enable fallback
vcm fallback --enable
```

### Multiple API Keys

Manage multiple accounts for the same tool:

```bash
# Add keys
vcm key add claude-code personal sk-ant-xxx
vcm key add claude-code work sk-ant-yyy

# List keys
vcm key list

# Switch active key
vcm key switch claude-code work

# Enable key rotation
vcm key rotate claude-code --enable
```

### Tool Recommendations

Discover new tools:

```bash
# Personalized recommendations
vcm recommend

# Trending tools
vcm trending

# New tools
vcm new

# Filter by tag
vcm recommend --tag coding
```

---

## Advanced Usage

### Export Installed Tools

Export your tool list for backup or sharing:

```bash
vcm export -o my-tools.json
```

### Import Tools List

Import and install tools from a list:

```bash
vcm import my-tools.json --install
```

### Check for Updates

See which tools have new versions:

```bash
vcm outdated
```

### System Diagnostics

Troubleshoot your setup:

```bash
vcm doctor
```

---

## Supported Package Managers

VCM automatically detects and uses your available package managers:

| Manager | Platform | Tools |
|---------|----------|-------|
| **npm** | All | Node.js packages |
| **pip/pipx** | All | Python packages |
| **cargo** | All | Rust packages |
| **brew** | macOS/Linux | Homebrew packages |
| **go** | All | Go packages |

---

## Adding New Tools

Want to add a new AI coding tool to VCM? Edit `registry/tools.yaml`:

```yaml
- id: my-tool
  name: My Tool
  vendor: MyCompany
  description: |
    A great AI coding assistant.
  website: https://mytool.ai
  repository: https://github.com/mycompany/my-tool
  executables:
    - my-tool
  install_methods:
    - manager: npm
      package: "@mycompany/my-tool"
  config_paths:
    - ~/.mytool
  env_vars:
    - name: MYTOOL_API_KEY
      description: MyTool API Key
      required: true
      get_url: https://mytool.ai/api-keys
  tags:
    - ai
    - coding
    - cli
  is_cli: true
  is_gui: false
  featured: false
```

---

## FAQ

### Q: How does VCM detect installed tools?

VCM checks if the executable exists in your PATH and verifies it's the correct tool by checking the version.

### Q: Where are API keys stored?

VCM adds API keys to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.) as environment variables.

### Q: Can I use VCM with GUI tools?

Yes! VCM tracks both CLI and GUI AI coding tools. GUI tools are marked differently in the output.

### Q: Does VCM work on Windows?

Yes, VCM supports Linux, macOS, and Windows. Some package managers are platform-specific.

---

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

VCM is inspired by the amazing ecosystem of AI coding tools. Special thanks to all the tool developers making AI-assisted coding a reality.

---

<p align="center">
  Made with ❤️ for the AI coding community
</p>
