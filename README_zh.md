# VCM - Vibe Coding Manager

> 一个命令，管理所有 AI 编程工具。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**VCM** 是一款专为 AI 编程助手设计的 CLI 工具管理器。帮助你在一个地方发现、安装、配置和管理所有 AI 编程工具。

[English Documentation](README.md)

---

## 为什么需要 VCM？

如果你使用 Claude Code、Cursor、Gemini CLI 等 AI 编程工具，可能会遇到这些问题：

- **工具分散**：不同工具有不同的安装方式
- **容易遗忘**：安装了很多工具，但忘记系统里有哪些
- **配置混乱**：API Key 分散在不同的配置文件中
- **状态不清**：不清楚哪些工具已配置、哪些需要更新
- **缺乏对比**：难以快速了解各工具的特点和差异

**VCM 解决以上所有问题。**

---

## 功能特性

### 核心能力

| 功能 | 描述 |
|-----|------|
| **扫描** | 自动检测系统已安装的所有 AI 编程工具 |
| **列表** | 查看所有已知 AI 编程工具及详细信息 |
| **安装** | 一键安装，自动检测可用包管理器 |
| **状态** | 检查配置状态和 API Key 是否可用 |
| **配置** | 交互式 API Key 配置向导 |
| **更新** | 保持所有工具为最新版本 |
| **免费模型** | 查找支持免费 AI 模型和专业级模型的工具 |

### 支持的工具

VCM 支持 30+ 款 AI 编程工具，包括：

| 供应商 | 工具 |
|-------|------|
| **Anthropic** | Claude Code |
| **OpenAI** | OpenAI CLI |
| **Google** | Gemini CLI |
| **GitHub** | Copilot CLI |
| **Amazon** | Kiro, CodeWhisperer |
| **Cursor** | Cursor Editor |
| **Codeium** | Windsurf |
| **开源** | Aider, OpenCode, Ollama, Kilo Code, Goose |
| **Sourcegraph** | Amp, Cody |
| **Augment** | Augment Code |
| **更多...** | |

### 多语言支持

- **English** (默认)
- **中文**

---

## 安装

### 从源码安装（推荐）

```bash
# 克隆仓库
git clone https://github.com/arden/vcm.git
cd vcm

# 编译并安装
cargo build --release
cargo install --path .
```

### 前置要求

- Rust 1.70+（从源码编译需要）
- 包管理器：npm、pip、cargo 或 brew 之一

---

## 快速开始

### 1. 扫描系统

发现你已安装的 AI 编程工具：

```bash
vcm scan
```

输出：
```
🔍 扫描已安装的 CLI AI 工具...

✓ Claude Code    v2.1.19
✓ Cursor         v2.6.19
✓ Kiro           v0.10.78
✓ OpenCode       v1.2.21

共发现 4 个已安装工具
```

### 2. 列出可用工具

查看所有支持的 AI 编程工具：

```bash
vcm list
```

输出：
```
📋 CLI AI 编程工具 (共 35 个)

已安装 (4)
  ✓ claude-code    Anthropic    终端 AI 编程助手
  ✓ cursor         Cursor Inc.  AI 代码编辑器

热门推荐
  ○ gemini-cli     Google       开源终端 AI 代理
  ○ aider          Open Source  AI 结对编程工具
  ○ kilo           Kilo         开源 AI 编程代理

其他工具
  ○ copilot-cli    GitHub       终端 AI 助手
  ○ opencode       Open Source  支持 75+ LLM 提供商
  ...
```

### 3. 检查工具状态

查看哪些工具需要配置：

```bash
vcm status
```

输出：
```
📊 工具状态

工具              版本           状态         备注
────────────────────────────────────────────────────────────
Claude Code     2.1.19       ⚠ 警告       缺少: ANTHROPIC_API_KEY
Cursor          2.6.19       ✓ 正常
OpenCode        1.2.21       ✓ 正常

完成度: 2/3 (66%)

建议: 运行 `vcm config <tool>` 配置
```

### 4. 配置 API Key

交互式配置向导：

```bash
vcm config claude-code
```

### 5. 安装新工具

```bash
vcm install gemini-cli
```

### 6. 启动工具

```bash
vcm run claude-code
```

---

## 命令参考

| 命令 | 描述 |
|-----|------|
| `vcm scan` | 扫描已安装的 AI 编程工具 |
| `vcm list` | 列出所有已知工具 |
| `vcm install <tool>` | 安装工具 |
| `vcm remove <tool>` | 卸载工具 |
| `vcm update [tool]` | 更新工具 |
| `vcm config [tool]` | 配置工具（API Key） |
| `vcm status` | 检查工具状态 |
| `vcm info <tool>` | 显示工具详情 |
| `vcm search <query>` | 搜索工具 |
| `vcm run <tool>` | 启动 CLI 工具 |
| `vcm outdated` | 检查更新 |
| `vcm free [--pro]` | 查找支持免费 AI 模型的工具 |
| `vcm doctor` | 系统诊断 |
| `vcm init` | 交互式初始化向导 |
| `vcm usage` | 显示使用统计 |
| `vcm lang [en\|zh]` | 设置语言 |

### 全局选项

```
-v, --verbose    详细输出
-j, --json       JSON 输出格式
```

---

## 语言设置

VCM 支持中英文切换：

```bash
# 切换到中文
vcm lang zh

# 切换到英文
vcm lang en

# 或使用环境变量
VCM_LANG=zh vcm scan
```

---

## 配置

### 配置目录

VCM 将配置存储在 `~/.config/vcm/` 目录：

```
~/.config/vcm/
├── config.toml      # 用户设置
├── state.json       # 运行时状态
└── registry.yaml    # 工具注册表缓存
```

### 配置文件结构

```toml
# ~/.config/vcm/config.toml

[settings]
language = "zh"
default_tool = "claude-code"
```

---

## 查找免费 AI 模型

VCM 帮助你发现提供免费专业级 AI 模型访问的工具。非常适合想要免费使用强大 AI 编程助手的开发者。

### 列出支持免费模型的工具

```bash
vcm free
```

输出：
```
🎁 支持免费模型的工具

★ Gemini CLI [最佳免费选择!]
  免费限额: 100 requests/day Gemini 2.5 Pro
  免费模型:
    ● Gemini 2.5 Pro [专业级] - 63.2% SWE-bench
    ● Gemini 3 Pro [专业级] - 76.2% SWE-bench (waitlist)
  需要信用卡: 无需信用卡
  备注: Best free tier for pro-grade models!

★ Ollama [最佳免费选择!]
  免费限额: Unlimited - runs locally
  免费模型:
    ● Qwen2.5-Coder-32B [专业级] - Excellent for coding
    ● DeepSeek Coder V2 [专业级] - Great for coding
  需要信用卡: 无需信用卡
  备注: 100% FREE - runs on your hardware!
```

### 仅显示有免费专业级模型的工具

```bash
vcm free --pro
```

### 最佳免费选择

| 工具 | 免费额度 | 专业级模型 | 需要信用卡 |
|-----|---------|-----------|-----------|
| **Gemini CLI** | 100 次/天 | Gemini 2.5 Pro, Gemini 3 Pro | 否 |
| **Ollama** | 无限本地运行 | Qwen2.5-Coder, DeepSeek Coder | 否 |
| **Aider** | BYOK/本地 | Qwen3-Coder (OpenRouter) | 否 |
| **Kiro** | 50 积分/月 | Claude 4 Sonnet | 否 |
| **OpenCode** | BYOK | Gemini 2.5 Pro, Qwen3-Coder | 否 |
| **Kilo Code** | $25 注册积分 | Claude Opus 4.5, GPT-4.1 | 是 |

### 什么是专业级模型？

在 **SWE-bench Verified** 测试中得分 **≥60%** 的模型被认为是专业级模型：

| 模型 | SWE-bench | 供应商 |
|-----|-----------|-------|
| Claude Opus 4.5 | 80.9% | Anthropic |
| GPT-5.1-Codex-Max | 77.9% | OpenAI |
| Gemini 3 Pro | 76.2% | Google |
| Gemini 2.5 Pro | 63.2% | Google |
| Qwen3-Coder-480B | 69.6% | Alibaba |

---

## 高级用法

### 导出已安装工具

导出工具列表用于备份或分享：

```bash
vcm export -o my-tools.json
```

### 导入工具列表

从列表导入并安装工具：

```bash
vcm import my-tools.json --install
```

### 检查更新

查看哪些工具有新版本：

```bash
vcm outdated
```

### 系统诊断

排查配置问题：

```bash
vcm doctor
```

---

## 支持的包管理器

VCM 自动检测并使用可用的包管理器：

| 管理器 | 平台 | 工具类型 |
|-------|------|---------|
| **npm** | 全平台 | Node.js 包 |
| **pip/pipx** | 全平台 | Python 包 |
| **cargo** | 全平台 | Rust 包 |
| **brew** | macOS/Linux | Homebrew 包 |
| **go** | 全平台 | Go 包 |

---

## 添加新工具

想为 VCM 添加新的 AI 编程工具？编辑 `registry/tools.yaml`：

```yaml
- id: my-tool
  name: My Tool
  vendor: MyCompany
  description: |
    一个很棒的 AI 编程助手。
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

## 常见问题

### Q: VCM 如何检测已安装的工具？

VCM 检查可执行文件是否存在于 PATH 中，并通过检查版本号来验证是否为正确的工具。

### Q: API Key 存储在哪里？

VCM 将 API Key 作为环境变量添加到你的 shell 配置文件中（`~/.bashrc`、`~/.zshrc` 等）。

### Q: VCM 可以管理 GUI 工具吗？

可以！VCM 同时追踪 CLI 和 GUI 类型的 AI 编程工具。GUI 工具会在输出中特别标注。

### Q: VCM 支持 Windows 吗？

支持，VCM 支持 Linux、macOS 和 Windows。部分包管理器仅适用于特定平台。

---

## 参与贡献

欢迎贡献代码！请随时提交 Issue 和 Pull Request。

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 提交 Pull Request

---

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## 致谢

VCM 的灵感来源于蓬勃发展的 AI 编程工具生态。特别感谢所有让 AI 辅助编程成为现实的工具开发者们。

---

<p align="center">
  用 ❤️ 为 AI 编程社区打造
</p>
