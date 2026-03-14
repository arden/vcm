# VCM (Vibe Coding Manager) 开发计划

> CLI AI 编程工具管理器 - 一站式管理你的 Vibe Coding 工具集

---

## 一、项目概述

### 1.1 背景

随着 AI 编程工具的爆发式增长，开发者面临着越来越多的 CLI AI 工具选择：

- Claude Code (Anthropic)
- GitHub Copilot CLI
- Aider
- Gemini CLI
- Cursor
- Windsurf
- ... 更多工具持续涌现

**核心痛点：**

1. **工具分散** - 不同工具有不同的安装方式（npm/pip/cargo/brew）
2. **遗忘安装** - 安装后容易忘记，不知道系统里有哪些工具
3. **配置混乱** - API Key 配置分散在各处，难以统一管理
4. **状态不清** - 不清楚哪些工具已配置、哪些需要更新
5. **缺乏对比** - 难以快速了解各工具的特点和差异

### 1.2 目标

打造一款**专注于 AI 编程工具领域**的 CLI 管理器，提供：

- 🔍 **发现** - 自动扫描系统已安装的 AI 编程工具
- 📦 **管理** - 统一的安装、更新、卸载入口
- ⚙️ **配置** - 集中管理 API Key 和工具配置
- 📊 **状态** - 清晰展示各工具的健康状态
- 🔄 **切换** - 快速切换默认使用的工具

### 1.3 与通用包管理器的区别

| 特性 | Homebrew/apt | asdf/mise | VCM |
|-----|-------------|-----------|-----|
| 通用性 | ✅ 全类型软件 | ✅ 开发工具版本 | 🎯 AI 编程工具专用 |
| API Key 管理 | ❌ | ❌ | ✅ |
| 工具状态检查 | ❌ | ❌ | ✅ |
| 工具对比信息 | ❌ | ❌ | ✅ |
| 配置向导 | ❌ | ❌ | ✅ |

---

## 二、市场调研与竞品分析

### 2.1 通用版本管理器

#### asdf

**优点：**
- 插件系统成熟，支持大量工具
- `.tool-versions` 文件版本锁定
- 社区活跃

**缺点：**
- Bash 编写，性能较差
- 使用 shims 机制，每次调用增加 ~120ms 延迟
- 不支持 Windows
- 插件安全性问题（第三方编写）

#### mise (推荐参考)

**优点：**
- Rust 编写，性能优秀（~5ms 延迟）
- 不使用 shims，直接修改 PATH
- 支持多种后端（asdf/aqua/cargo/npm/go）
- 模糊匹配、命令别名，用户体验好
- 支持 Windows
- 安全性更好（cosign/slsa 验证）

**设计亮点：**
```bash
# 单命令完成插件安装+版本安装+配置
mise use node@20

# 模糊匹配
mise install node@20  # 自动匹配最新 20.x

# 多后端支持
mise use -g cargo:ripgrep@14
mise use -g npm:prettier@3
```

### 2.2 专用版本管理器

| 工具 | 专注领域 | 特点 |
|-----|---------|-----|
| nvm/fnm | Node.js | 项目级版本切换 |
| pyenv | Python | 多版本并行 |
| sdkman | Java 生态 | JDK/Maven/Gradle |
| rustup | Rust | 工具链管理 |

### 2.3 系统包管理器

| 工具 | 平台 | 特点 |
|-----|-----|-----|
| Homebrew | macOS | Formula 系统，社区丰富 |
| Scoop | Windows | 用户级安装，无需管理员 |
| Chocolatey | Windows | 系统级安装 |
| apt/dnf/pacman | Linux | 系统原生 |

### 2.4 关键启示

1. **性能至关重要** - mise 的成功证明了 Rust + 无 shims 的优势
2. **用户体验** - 模糊匹配、命令别名、单命令操作
3. **安全性** - 直接从官方源下载，避免第三方插件
4. **跨平台** - Windows 支持越来越重要
5. **专注领域** - 垂直领域管理器比通用工具更精准

---

## 三、产品定位

### 3.1 目标用户

- **开发者** - 使用多种 AI 编程工具的专业开发者
- **早期采用者** - 喜欢尝试新 AI 工具的技术爱好者
- **团队负责人** - 需要统一管理团队工具链的技术 Leader

### 3.2 核心价值主张

```
"一个命令，管理所有 AI 编程工具"
```

### 3.3 差异化优势

| 维度 | VCM 优势 |
|-----|---------|
| **领域专注** | 专注于 AI 编程工具，提供深度信息 |
| **配置集成** | 统一管理 API Key、模型选择等配置 |
| **状态监控** | 实时检查工具健康状态和配置完整性 |
| **智能推荐** | 根据使用场景推荐合适的工具 |
| **轻量快速** | Rust 实现，毫秒级响应 |

---

## 四、功能规划

### 4.1 MVP 核心功能

```
┌─────────────────────────────────────────────────────────────┐
│                      VCM 核心功能                           │
├─────────────────────────────────────────────────────────────┤
│  vcm scan        扫描系统已安装的工具                       │
│  vcm list        列出所有已知工具（支持筛选）               │
│  vcm install     安装工具                                   │
│  vcm status      检查工具状态                               │
│  vcm config      配置工具（API Key 等）                     │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 功能矩阵

| 功能 | 优先级 | 描述 |
|-----|-------|------|
| `scan` 扫描 | P0 | 自动检测已安装工具 |
| `list` 列表 | P0 | 显示所有已知工具 |
| `status` 状态 | P0 | 检查配置和健康状态 |
| `install` 安装 | P0 | 一键安装工具 |
| `config` 配置 | P0 | API Key 配置向导 |
| `update` 更新 | P1 | 更新工具版本 |
| `remove` 卸载 | P1 | 卸载工具 |
| `search` 搜索 | P1 | 模糊搜索工具 |
| `info` 详情 | P1 | 显示工具详细信息 |
| `switch` 切换 | P2 | 设置默认工具 |
| `doctor` 诊断 | P2 | 系统诊断和问题修复 |
| `sync` 同步 | P3 | 同步配置到云端 |

### 4.3 命令详细设计

#### `vcm scan` - 扫描已安装工具

```bash
$ vcm scan
🔍 扫描已安装的 CLI AI 工具...

✓ claude-code    v1.0.0    ~/.claude (已配置)
✓ aider          v0.50.0   未配置 API Key
✓ cursor         v0.45.0   GUI 应用
✓ amp            v0.1.0    ~/.amp
✗ gemini-cli     未安装

共发现 4 个已安装工具

选项:
  --detailed, -d    显示详细信息（路径、安装方式等）
  --json            JSON 格式输出
  --quiet, -q       仅输出工具名
```

#### `vcm list` - 列出所有工具

```bash
$ vcm list
📋 CLI AI 编程工具 (共 15 个)

已安装 (4)
  ✓ claude-code    Anthropic    终端 AI 编程助手
  ✓ aider          Open Source  AI 结对编程工具
  ✓ cursor         Cursor Inc.  AI 代码编辑器
  ✓ amp            Sourcegraph  AI 代码助手

热门推荐
  ○ copilot-cli    GitHub       终端 AI 助手
  ○ gemini-cli     Google       开源终端 AI 代理
  ○ windsurf       Codeium      企业级编辑器
  ○ ollama         Open Source  本地 LLM 运行

其他工具
  ○ v0             Vercel       UI 组件生成
  ○ bolt-new       StackBlitz   快速原型开发
  ...

选项:
  --installed, -i   仅显示已安装
  --tag <tag>       按标签筛选 (ai, cli, gui, opensource)
  --json            JSON 格式输出
```

#### `vcm install` - 安装工具

```bash
$ vcm install gemini-cli
📦 安装 gemini-cli...

检测到可用包管理器: npm, pip, cargo
使用 npm 安装...
[████████████████████] 100%
✓ 安装完成: @google/gemini-cli@latest

下一步:
  1. 获取 API Key: https://ai.google.dev
  2. 配置: vcm config gemini-cli

$ vcm install aider --manager pipx
📦 使用 pipx 安装 aider...
✓ 安装完成

$ vcm install node@20  # 模糊匹配版本
```

#### `vcm status` - 检查状态

```bash
$ vcm status
📊 工具状态概览

工具            版本     状态      配置      API Key      备注
────────────────────────────────────────────────────────────────
claude-code     1.0.0    ✓ 正常    ✓ 已配置  sk-ant-****  
aider           0.50.0   ⚠ 警告    ✗ 未配置  -           缺少 API Key
cursor          0.45.0   ✓ 正常    ✓ 已配置  -           GUI 应用
amp             0.1.0    ✓ 正常    -         -           

配置完成度: 75% (3/4)
建议: 运行 `vcm config aider` 配置 API Key
```

#### `vcm config` - 配置工具

```bash
$ vcm config claude-code
🔑 配置 claude-code

当前状态: ✓ 已配置
API Key: sk-ant-...abcd (已设置)

操作:
  1. 更新 API Key
  2. 查看配置文件
  3. 测试连接
  4. 清除配置

$ vcm config --set-key ANTHROPIC_API_KEY=sk-ant-xxx
✓ 已设置环境变量 ANTHROPIC_API_KEY

$ vcm config aider
🔑 配置 aider

aider 支持多种 LLM 后端:
  1. OpenAI (需要 OPENAI_API_KEY)
  2. Anthropic Claude (需要 ANTHROPIC_API_KEY)
  3. 本地模型 (Ollama)

选择后端 [1-3]: 2
请输入 ANTHROPIC_API_KEY: ****
✓ 配置完成
```

#### `vcm search` - 搜索工具

```bash
$ vcm search google
🔍 搜索结果: "google"

  gemini-cli     Google       开源终端 AI 代理
                 支持多模态，免费额度大
                 
  gemini-code    Google       Gemini 代码助手
                 VS Code 扩展

$ vcm search cli --tag ai
🔍 搜索结果: "cli" (标签: ai)

  claude-code    Anthropic    终端 AI 编程助手
  copilot-cli    GitHub       终端 AI 助手
  aider          Open Source  AI 结对编程工具
  gemini-cli     Google       开源终端 AI 代理
```

#### `vcm info` - 工具详情

```bash
$ vcm info claude-code
┌─────────────────────────────────────────────────────────────┐
│  Claude Code                                   Anthropic   │
├─────────────────────────────────────────────────────────────┤
│  终端 AI 编程助手，具备强大的推理能力                      │
│                                                             │
│  版本: 1.0.0 (已安装)                                       │
│  路径: /usr/local/bin/claude                                │
│  配置: ~/.claude                                            │
│  状态: ✓ 正常                                               │
│                                                             │
│  安装方式:                                                   │
│    npm install -g @anthropic-ai/claude-code                 │
│                                                             │
│  环境变量:                                                   │
│    ANTHROPIC_API_KEY ✓ 已配置                               │
│                                                             │
│  链接:                                                       │
│    官网: https://claude.ai                                  │
│    文档: https://docs.anthropic.com                         │
│    仓库: https://github.com/anthropics/claude-code          │
│                                                             │
│  标签: ai, coding, cli, llm                                 │
└─────────────────────────────────────────────────────────────┘
```

### 4.4 配置文件设计

#### ~/.vcm/config.toml

```toml
# VCM 配置文件

[settings]
default_tool = "claude-code"
check_updates = true
auto_update_registry = true

[registry]
url = "https://raw.githubusercontent.com/arden/vcm/main/registry/tools.yaml"
local_path = "~/.vcm/registry.yaml"
update_interval = "24h"

[api_keys]
# 加密存储的 API Keys（可选）
# 实际存储在环境变量或系统 Keychain 中
```

#### ~/.vcm/state.json

```json
{
  "installed_tools": {
    "claude-code": {
      "version": "1.0.0",
      "installed_at": "2025-01-15T10:00:00Z",
      "install_method": "npm"
    }
  },
  "last_scan": "2025-01-20T08:30:00Z",
  "registry_version": "2025-01-19"
}
```

---

## 五、技术架构设计

### 5.1 技术栈

| 层级 | 技术选择 | 理由 |
|-----|---------|------|
| 语言 | Rust 1.75+ | 高性能、内存安全、跨平台 |
| CLI 框架 | clap 4.x | 功能完善、derive 宏易用 |
| 异步运行时 | tokio | 生态成熟、性能优秀 |
| 序列化 | serde + toml/yaml | Rust 标准方案 |
| 终端 UI | dialoguer + console + indicatif | 交互式选择、彩色输出、进度条 |
| HTTP 客户端 | reqwest | 异步、支持 rustls |
| 错误处理 | anyhow + thiserror | 应用层 + 库层分离 |
| 配置目录 | dirs | 跨平台路径 |
| 命令查找 | which | 检测可执行文件 |

### 5.2 项目结构

```
vcm/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs              # 入口点
│   ├── cli.rs               # CLI 命令定义
│   ├── lib.rs               # 库入口
│   │
│   ├── commands/            # 命令实现
│   │   ├── mod.rs
│   │   ├── scan.rs          # 扫描命令
│   │   ├── list.rs          # 列表命令
│   │   ├── install.rs       # 安装命令
│   │   ├── update.rs        # 更新命令
│   │   ├── remove.rs        # 卸载命令
│   │   ├── config.rs        # 配置命令
│   │   ├── status.rs        # 状态命令
│   │   ├── search.rs        # 搜索命令
│   │   └── info.rs          # 详情命令
│   │
│   ├── core/                # 核心逻辑
│   │   ├── mod.rs
│   │   ├── discovery.rs     # 工具发现引擎
│   │   ├── installer.rs     # 多包管理器安装器
│   │   ├── registry.rs      # 工具注册表
│   │   ├── config.rs        # 配置管理
│   │   └── health.rs        # 健康检查
│   │
│   ├── models/              # 数据模型
│   │   ├── mod.rs
│   │   ├── tool.rs          # 工具定义
│   │   ├── installer.rs     # 安装器类型
│   │   └── config.rs        # 配置结构
│   │
│   ├── backends/            # 包管理器后端
│   │   ├── mod.rs
│   │   ├── npm.rs           # npm 后端
│   │   ├── pip.rs           # pip/pipx 后端
│   │   ├── cargo.rs         # cargo 后端
│   │   ├── brew.rs          # Homebrew 后端
│   │   ├── go.rs            # go install 后端
│   │   └── binary.rs        # 二进制下载后端
│   │
│   └── utils/               # 工具函数
│       ├── mod.rs
│       ├── shell.rs         # Shell 命令执行
│       ├── platform.rs      # 平台检测
│       ├── env.rs           # 环境变量
│       └── version.rs       # 版本解析
│
├── registry/                # 内置注册表
│   └── tools.yaml           # 工具定义
│
├── tests/                   # 测试
│   ├── integration/
│   └── fixtures/
│
├── docs/                    # 文档
│   └── architecture.md
│
└── README.md
```

### 5.3 核心模块设计

#### 5.3.1 工具发现引擎 (discovery.rs)

```rust
pub struct Discovery {
    registry: Registry,
    platform: Platform,
}

impl Discovery {
    /// 扫描所有已安装的工具
    pub fn scan(&self) -> Vec<InstalledTool> { ... }
    
    /// 检查单个工具是否安装
    pub fn check_installed(&self, tool: &Tool) -> Option<InstalledTool> { ... }
    
    /// 获取工具版本
    fn get_version(&self, executable: &str) -> Option<String> { ... }
    
    /// 检测安装方式
    fn detect_install_method(&self, tool: &Tool) -> Option<PackageManager> { ... }
    
    /// 检查配置文件是否存在
    fn check_config_exists(&self, tool: &Tool) -> bool { ... }
}
```

#### 5.3.2 多后端安装器 (installer.rs)

```rust
pub struct Installer {
    backends: HashMap<PackageManager, Box<dyn Backend>>,
}

#[async_trait]
pub trait Backend {
    /// 检查后端是否可用
    fn is_available(&self) -> bool;
    
    /// 安装包
    async fn install(&self, package: &str, version: Option<&str>) -> Result<()>;
    
    /// 更新包
    async fn update(&self, package: &str) -> Result<()>;
    
    /// 卸载包
    async fn remove(&self, package: &str) -> Result<()>;
    
    /// 检查是否已安装
    fn is_installed(&self, package: &str) -> bool;
    
    /// 获取已安装版本
    fn get_version(&self, package: &str) -> Option<String>;
}

impl Installer {
    /// 自动选择最佳安装方式
    pub async fn install(&self, tool: &Tool, preferred: Option<PackageManager>) -> Result<()> { ... }
    
    /// 获取可用的后端列表
    pub fn available_backends(&self) -> Vec<PackageManager> { ... }
}
```

#### 5.3.3 工具注册表 (registry.rs)

```rust
pub struct Registry {
    tools: Vec<Tool>,
    index: HashMap<String, usize>,  // id -> index
    by_tag: HashMap<String, Vec<usize>>,  // tag -> indices
}

impl Registry {
    /// 加载注册表
    pub fn load() -> Result<Self> { ... }
    
    /// 从远程更新
    pub async fn update_from_remote(&mut self) -> Result<()> { ... }
    
    /// 按 ID 查找
    pub fn find_by_id(&self, id: &str) -> Option<&Tool> { ... }
    
    /// 按名称模糊查找
    pub fn find_by_name(&self, name: &str) -> Vec<&Tool> { ... }
    
    /// 搜索
    pub fn search(&self, query: &str) -> Vec<&Tool> { ... }
    
    /// 按标签筛选
    pub fn by_tag(&self, tag: &str) -> Vec<&Tool> { ... }
}
```

### 5.4 数据模型

```rust
// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub description: String,
    pub website: Option<String>,
    pub repository: Option<String>,
    pub install_methods: Vec<InstallMethod>,
    pub executables: Vec<String>,
    pub config_paths: Vec<String>,
    pub env_vars: Vec<EnvVar>,
    pub tags: Vec<String>,
    pub is_cli: bool,
    pub is_gui: bool,
}

// 安装方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMethod {
    pub manager: PackageManager,
    pub package: String,
    pub version: Option<String>,
    pub platforms: Option<Vec<Platform>>,
}

// 包管理器类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PackageManager {
    Npm,
    Pip,
    Pipx,
    Brew,
    Cargo,
    Go,
    Scoop,
    Chocolatey,
    Apt,
    Dnf,
    Pacman,
    Snap,
    Binary,
}

// 已安装工具
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledTool {
    pub tool_id: String,
    pub executable: String,
    pub version: Option<String>,
    pub path: String,
    pub install_method: Option<PackageManager>,
    pub config_exists: bool,
    pub is_configured: bool,
}

// 工具状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub tool: InstalledTool,
    pub health: HealthStatus,
    pub missing_env_vars: Vec<String>,
    pub suggestions: Vec<String>,
}
```

---

## 六、开发阶段规划

### Phase 1: 基础框架 (Week 1-2)

**目标：** 搭建项目骨架，实现基础命令

**任务清单：**
- [ ] 初始化 Cargo 项目
- [ ] 实现 CLI 命令定义 (clap)
- [ ] 定义核心数据模型
- [ ] 创建工具注册表 YAML
- [ ] 实现 `scan` 命令（工具发现）
- [ ] 实现 `list` 命令
- [ ] 单元测试框架搭建

**交付物：**
```
vcm scan    # 可扫描已安装工具
vcm list    # 可列出已知工具
```

### Phase 2: 安装功能 (Week 3-4)

**目标：** 实现多后端安装器

**任务清单：**
- [ ] 实现 Backend trait
- [ ] 实现 npm 后端
- [ ] 实现 pip/pipx 后端
- [ ] 实现 cargo 后端
- [ ] 实现 brew 后端
- [ ] 实现 go 后端
- [ ] 实现 `install` 命令
- [ ] 实现版本模糊匹配
- [ ] 集成测试

**交付物：**
```
vcm install <tool>      # 可安装工具
vcm install <tool>@ver  # 支持版本指定
```

### Phase 3: 配置与状态 (Week 5-6)

**目标：** 实现配置管理和状态检查

**任务清单：**
- [ ] 实现配置文件读写
- [ ] 实现 API Key 管理
- [ ] 实现环境变量检测
- [ ] 实现 `config` 命令
- [ ] 实现 `status` 命令
- [ ] 实现健康检查逻辑
- [ ] 实现配置向导（交互式）

**交付物：**
```
vcm config <tool>       # 配置工具
vcm status              # 检查状态
```

### Phase 4: 完善功能 (Week 7-8)

**目标：** 补全剩余功能，优化体验

**任务清单：**
- [ ] 实现 `update` 命令
- [ ] 实现 `remove` 命令
- [ ] 实现 `search` 命令
- [ ] 实现 `info` 命令
- [ ] 实现 `doctor` 命令
- [ ] 远程注册表更新
- [ ] 错误处理优化
- [ ] 性能优化

**交付物：**
```
vcm update <tool>       # 更新工具
vcm remove <tool>       # 卸载工具
vcm search <query>      # 搜索工具
vcm info <tool>         # 工具详情
vcm doctor              # 系统诊断
```

### Phase 5: 发布准备 (Week 9-10)

**目标：** 打包发布，文档完善

**任务清单：**
- [ ] 跨平台编译 (Linux/macOS/Windows)
- [ ] Homebrew Formula
- [ ] 安装脚本
- [ ] CI/CD 配置
- [ ] 使用文档
- [ ] 示例和教程

**交付物：**
- GitHub Release
- Homebrew: `brew install arden/tap/vcm`
- 脚本安装: `curl -fsSL https://vcm.dev/install.sh | sh`

---

## 七、工具注册表设计

### 7.1 注册表结构

```yaml
# registry/tools.yaml

version: 1
updated: 2025-01-20
source: https://github.com/arden/vcm

tools:
  # Claude Code
  - id: claude-code
    name: Claude Code
    vendor: Anthropic
    description: |
      终端 AI 编程助手，具备强大的推理能力。
      支持代码编辑、调试、重构等多种编程任务。
    website: https://claude.ai
    repository: https://github.com/anthropics/claude-code
    executables:
      - claude
      - claude-code
    install_methods:
      - manager: npm
        package: "@anthropic-ai/claude-code"
    config_paths:
      - ~/.claude
      - ~/.config/claude
    env_vars:
      - name: ANTHROPIC_API_KEY
        description: Anthropic API Key
        required: true
        get_url: https://console.anthropic.com
    tags:
      - ai
      - coding
      - cli
      - llm
      - anthropic
    is_cli: true
    is_gui: false
    featured: true

  # Aider
  - id: aider
    name: Aider
    vendor: Open Source
    description: |
      开源 AI 结对编程工具，支持多种 LLM 后端。
      可以在终端中与 AI 协作编写和修改代码。
    website: https://aider.chat
    repository: https://github.com/paul-gauthier/aider
    executables:
      - aider
    install_methods:
      - manager: pip
        package: aider-chat
      - manager: pipx
        package: aider-chat
    config_paths:
      - ~/.aider.conf.yml
    env_vars:
      - name: OPENAI_API_KEY
        description: OpenAI API Key
        required: false
      - name: ANTHROPIC_API_KEY
        description: Anthropic API Key
        required: false
    tags:
      - ai
      - coding
      - cli
      - opensource
    is_cli: true
    is_gui: false
    featured: true

  # GitHub Copilot CLI
  - id: copilot-cli
    name: GitHub Copilot CLI
    vendor: GitHub/Microsoft
    description: |
      GitHub 官方终端 AI 编程助手。
      支持 shell 命令生成、代码解释、Git 操作辅助。
    website: https://github.com/features/copilot/cli
    repository: https://github.com/github/copilot-cli
    executables:
      - github-copilot
      - gh copilot
    install_methods:
      - manager: npm
        package: "@github/copilot"
    config_paths:
      - ~/.config/github-copilot
    env_vars:
      - name: GITHUB_TOKEN
        description: GitHub Personal Access Token
        required: true
        get_url: https://github.com/settings/tokens
    tags:
      - ai
      - coding
      - cli
      - github
    is_cli: true
    is_gui: false
    featured: true

  # Gemini CLI
  - id: gemini-cli
    name: Gemini CLI
    vendor: Google
    description: |
      Google 开源终端 AI 代理。
      支持多模态输入，提供免费使用额度。
    website: https://ai.google.dev
    repository: https://github.com/google-gemini/gemini-cli
    executables:
      - gemini
      - gemini-cli
    install_methods:
      - manager: npm
        package: "@google/gemini-cli"
    config_paths:
      - ~/.config/gemini
    env_vars:
      - name: GOOGLE_API_KEY
        description: Google API Key
        required: true
        get_url: https://ai.google.dev/api-key
    tags:
      - ai
      - coding
      - cli
      - google
    is_cli: true
    is_gui: false
    featured: true

  # Cursor
  - id: cursor
    name: Cursor
    vendor: Cursor Inc.
    description: |
      AI 驱动的代码编辑器，基于 VS Code。
      深度集成 AI 能力，支持代码补全和对话。
    website: https://cursor.sh
    executables:
      - cursor
    install_methods:
      - manager: brew
        package: cursor
        platforms: [macos]
      - manager: scoop
        package: cursor
        platforms: [windows]
    config_paths:
      - ~/.cursor
    tags:
      - ai
      - coding
      - editor
    is_cli: false
    is_gui: true
    featured: true

  # Windsurf
  - id: windsurf
    name: Windsurf
    vendor: Codeium
    description: |
      企业级 AI 代码编辑器。
      适合大型项目和团队协作。
    website: https://codeium.com/windsurf
    executables:
      - windsurf
    install_methods:
      - manager: brew
        package: windsurf
        platforms: [macos]
    config_paths:
      - ~/.windsurf
    tags:
      - ai
      - coding
      - editor
      - enterprise
    is_cli: false
    is_gui: true

  # Ollama
  - id: ollama
    name: Ollama
    vendor: Open Source
    description: |
      本地运行大语言模型。
      支持 Llama、Mistral、Qwen 等开源模型。
    website: https://ollama.ai
    repository: https://github.com/ollama/ollama
    executables:
      - ollama
    install_methods:
      - manager: brew
        package: ollama
        platforms: [macos, linux]
      - manager: script
        package: https://ollama.ai/install.sh
        platforms: [linux, macos]
    config_paths:
      - ~/.ollama
    tags:
      - ai
      - llm
      - local
      - opensource
    is_cli: true
    is_gui: false

  # v0
  - id: v0
    name: v0 by Vercel
    vendor: Vercel
    description: |
      AI UI 组件生成器。
      专注于 React/Next.js 组件开发。
    website: https://v0.dev
    executables:
      - v0
    install_methods:
      - manager: npm
        package: v0
    config_paths:
      - ~/.v0
    env_vars:
      - name: VERCEL_API_KEY
        description: Vercel API Key
        required: true
    tags:
      - ai
      - ui
      - react
      - nextjs
    is_cli: false
    is_gui: false
    featured: false

  # Bolt.new
  - id: bolt-new
    name: Bolt.new
    vendor: StackBlitz
    description: |
      快速原型开发工具。
      支持多框架快速迭代。
    website: https://bolt.new
    tags:
      - ai
      - prototype
      - web
    is_cli: false
    is_gui: false

  # 更多工具...
```

### 7.2 注册表更新机制

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   GitHub    │────>│   VCM CLI   │────>│  本地缓存   │
│  Registry   │     │  (拉取更新) │     │  (~/.vcm)   │
└─────────────┘     └─────────────┘     └─────────────┘
                          │
                          ▼
                    ┌─────────────┐
                    │  内置注册表  │
                    │ (fallback)  │
                    └─────────────┘
```

---

## 八、风险与挑战

### 8.1 技术风险

| 风险 | 影响 | 缓解措施 |
|-----|-----|---------|
| 多包管理器兼容性 | 安装失败 | 充分测试各平台，提供降级方案 |
| 版本解析复杂 | 版本匹配失败 | 参考 mise 的版本解析实现 |
| 工具可执行文件名变化 | 检测失败 | 维护多名称映射表 |
| API Key 安全存储 | 泄露风险 | 使用系统 Keychain，不存储明文 |

### 8.2 产品风险

| 风险 | 影响 | 缓解措施 |
|-----|-----|---------|
| 工具注册表维护成本 | 信息过时 | 社区贡献 + 自动化检测 |
| 新工具层出不穷 | 覆盖不全 | 支持用户自定义添加 |
| 工具更新频繁 | 兼容性问题 | 版本锁定 + 更新提醒 |

### 8.3 应对策略

1. **渐进式开发** - 先支持最常用的工具，逐步扩展
2. **社区驱动** - 开源注册表，接受 PR
3. **自动化测试** - CI/CD 自动测试各平台安装
4. **错误恢复** - 提供详细的错误诊断和修复建议

---

## 九、成功指标

### 9.1 MVP 成功标准

- [ ] 支持扫描 10+ 主流 AI 编程工具
- [ ] 支持 5 种包管理器后端
- [ ] 安装成功率 > 95%
- [ ] 扫描速度 < 500ms
- [ ] 跨平台支持 (Linux/macOS/Windows)

### 9.2 长期目标

- 注册表覆盖 50+ AI 编程工具
- 周活用户 > 1000
- GitHub Star > 500
- Homebrew 官方收录

---

## 十、附录

### 10.1 参考项目

- [mise](https://github.com/jdx/mise) - 现代工具版本管理器
- [asdf](https://github.com/asdf-vm/asdf) - 通用版本管理器
- [sdkman](https://sdkman.io) - Java 生态 SDK 管理器
- [Homebrew](https://brew.sh) - macOS 包管理器

### 10.2 相关链接

- [clap documentation](https://docs.rs/clap)
- [tokio tutorial](https://tokio.rs/tokio/tutorial)
- [serde documentation](https://serde.rs)

---

*文档版本: 1.0*
*最后更新: 2025-01-20*
*作者: Arden*
