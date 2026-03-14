# VCM 插件系统技术架构方案

> 文档版本：v1.0
> 创建时间：2026-03-14
> 目标版本：v3.0

---

## 一、概述

### 1.1 目标

将 VCM 从"工具管理器"进化为"Vibe Coding 操作系统"，通过插件系统实现：

- 跨工具协调能力
- 社区生态建设
- 差异化竞争优势

### 1.2 设计原则

1. **安全第一**：插件不能影响主程序稳定性
2. **低门槛**：让尽可能多的开发者能写插件
3. **高性能**：插件调用不能显著拖慢主程序
4. **可扩展**：支持多种插件类型和运行时

---

## 二、技术方案对比

### 2.1 方案一览

| 方案 | 安全性 | 性能 | 开发门槛 | 跨平台 | 生态 | 推荐度 |
|------|--------|------|----------|--------|------|--------|
| **Lua 脚本** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **JS/Deno** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **进程隔离** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **WASM** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Python** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **动态库** | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐ |

### 2.2 详细分析

#### 方案一：Lua 脚本引擎

```rust
use mlua::{Lua, Result};

let lua = Lua::new();
lua.globals().set("vcm", vcm_api)?;

lua.load(r#"
    function on_tool_start(tool, task)
        if task:find("refactor") then
            vcm.switch_tool("aider")
        end
    end
"#).exec()?;
```

**优势**：
- 内置轻量，无需额外依赖
- Lua 语法简单，学习成本低
- 沙箱隔离，安全可控
- 插件体积小（几 KB）

**劣势**：
- 生态较小，库不如 JS/Python 丰富
- 不适合复杂逻辑

**适用场景**：轻量钩子、配置型插件

#### 方案二：JavaScript/TypeScript (Deno)

```rust
use deno_core::*;

let runtime = JsRuntime::new(Default::default());
runtime.execute("plugin.js", r#"
    vcm.onToolStart((tool, task) => {
        if (task.includes("refactor")) {
            vcm.switchTool("aider");
        }
    });
"#)?;
```

**优势**：
- 前端开发者多，生态最广
- npm 海量包可复用
- Deno 原生沙箱，安全性好
- TypeScript 支持类型提示

**劣势**：
- 运行时体积较大（~30MB）
- 启动开销比 Lua 大

**适用场景**：复杂插件、需要 npm 生态

#### 方案三：进程隔离 + RPC

```
┌─────────────────────────────────────────────┐
│                  VCM Core                    │
│  ┌─────────────────────────────────────────┐│
│  │           Plugin Manager                ││
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐ ││
│  │  │ Plugin 1│  │ Plugin 2│  │ Plugin N│ ││
│  │  │ (进程)  │  │ (进程)  │  │ (进程)  │ ││
│  │  └────┬────┘  └────┬────┘  └────┬────┘ ││
│  └───────┼────────────┼────────────┼───────┘│
│          │            │            │        │
│          └────────────┴────────────┘        │
│                 gRPC / Unix Socket          │
└─────────────────────────────────────────────┘
```

```rust
// 插件作为独立进程运行
pub struct PluginProcess {
    name: String,
    child: Child,
    client: PluginClient,  // gRPC/Tonic client
}

// 通信协议
service Plugin {
    rpc OnToolStart(ToolStartRequest) returns (Action);
    rpc OnToolComplete(ToolCompleteRequest) returns (Action);
    rpc OnCostUpdate(CostUpdateRequest) returns (Action);
}
```

**优势**：
- 完全隔离，崩溃不影响主程序
- 任何语言都可以写插件
- 资源限制简单（CPU/内存）
- 调试方便

**劣势**：
- 进程启动开销
- IPC 通信延迟
- 部署复杂度增加

**适用场景**：重型插件、企业级隔离要求

#### 方案四：WASM (WebAssembly)

```rust
use wasmtime::*;

let engine = Engine::default();
let module = Module::from_file(&engine, "plugin.wasm")?;
let instance = Instance::new(&store, &module, &imports)?;
```

**优势**：
- 沙箱隔离，安全性最高
- 跨平台一致性
- 接近原生性能
- 体积小

**劣势**：
- Rust/AssemblyScript 开发门槛高
- WASI 接口受限
- 调试困难
- 生态相对小

**适用场景**：安全要求高、Rust 开发者

#### 方案五：Python (pyo3)

```rust
use pyo3::*;

let gil = Python::acquire_gil();
let py = gil.python();
let plugin = py.import("my_plugin")?;
plugin.call_method1("on_tool_start", ("claude-code", "refactor code"))?;
```

**优势**：
- Python 开发者最多
- PyPI 生态丰富
- 数据科学、ML 库多

**劣势**：
- 需要系统安装 Python
- GIL 限制并发
- 安全性需要额外处理

**适用场景**：数据分析、ML 相关插件

#### 方案六：动态库 (Native)

```rust
use libloading::{Library, Symbol};

let lib = Library::new("./plugin.so")?;
let on_tool_start: Symbol<fn(&str, &str) -> Action> = 
    unsafe { lib.get(b"on_tool_start")? };
```

**优势**：
- 性能最高
- Rust/C++ 直接调用

**劣势**：
- 安全性最差（恶意代码可以干任何事）
- 跨平台需要分别编译
- ABI 兼容性问题

**不推荐**用于社区插件，仅限官方可信插件

---

## 三、推荐方案：混合架构

### 3.1 架构图

```
VCM Plugin System Architecture

┌─────────────────────────────────────────────────────────┐
│                     Plugin Manager                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ Lua Engine  │  │ Deno Runtime│  │  Process    │     │
│  │ (内置轻量)  │  │ (JS 插件)   │  │ (独立进程)  │     │
│  │             │  │             │  │             │     │
│  │ *.lua       │  │ *.js/*.ts   │  │ bin/*       │     │
│  │ ~10KB       │  │ ~100KB      │  │ 任意语言    │     │
│  │ 沙箱隔离    │  │ Deno 沙箱   │  │ 系统隔离    │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│                                                         │
│  ┌─────────────┐                                        │
│  │ WASM Engine │  ← 可选，按需加载                      │
│  │ *.wasm      │                                        │
│  └─────────────┘                                        │
│                                                         │
├─────────────────────────────────────────────────────────┤
│              Common Plugin API (Rust Trait)             │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Hooks: on_tool_start, on_tool_complete, on_cost_update │
│  APIs: vcm.switch_tool, vcm.log, vcm.notify, ...        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 3.2 分层策略

| 插件类型 | 技术栈 | 体积 | 适用场景 |
|----------|--------|------|----------|
| **轻量钩子** | Lua | ~10KB | smart-router, watchdog, notify |
| **复杂逻辑** | JS/TS | ~100KB | cost-guard, workflow-forge, ide-bridge |
| **独立服务** | 进程 | 任意 | memory-vault, team-sync, pr-sentinel |
| **性能敏感** | WASM | ~50KB | 可选，按需支持 |

### 3.3 分层理由

1. **Lua 作为默认运行时**
   - 满足 80% 插件需求
   - 开发者学习成本低（1小时上手）
   - 无额外依赖，体积小

2. **Deno 作为扩展运行时**
   - 吸引前端生态开发者
   - npm 包可直接复用
   - 适合复杂业务逻辑

3. **进程隔离作为企业级方案**
   - 完全隔离，安全性最高
   - 支持任何语言
   - 适合重型插件

---

## 四、插件接口设计

### 4.1 插件声明文件 (vcm-plugin.toml)

```toml
[plugin]
name = "smart-router"
version = "1.0.0"
author = "vcm-team"
description = "Intelligent tool router based on task type"
repository = "https://github.com/vcm-plugins/smart-router"

# 运行时类型: lua | js | process | wasm
runtime = "lua"
entry = "main.lua"

[permissions]
# 工具操作权限
tools = ["read", "switch", "install"]
# 配置权限
config = ["read"]
# 网络权限
network = false
# 文件系统权限
fs = ["read"]
# 执行命令权限
exec = false

[hooks]
# 工具启动前钩子
on_tool_start = true
# 任务完成后钩子
on_task_complete = false
# 成本更新钩子
on_cost_update = true
# 配置变更钩子
on_config_change = false

[config]
# 插件可配置项
enable_auto_switch = true
preferred_tools = ["claude-code", "gemini-cli"]
```

### 4.2 核心插件 API

```rust
/// 插件核心接口
pub trait Plugin: Send + Sync {
    /// 插件元信息
    fn metadata(&self) -> PluginMetadata;
    
    /// 生命周期钩子
    fn on_load(&mut self, ctx: &PluginContext) -> Result<()>;
    fn on_unload(&mut self) -> Result<()>;
    
    /// 事件钩子
    fn on_tool_start(&self, event: &ToolStartEvent) -> Option<Action>;
    fn on_tool_complete(&self, event: &ToolCompleteEvent) -> Option<Action>;
    fn on_cost_update(&self, event: &CostUpdateEvent) -> Option<Action>;
}

/// 插件可执行的动作
pub enum Action {
    /// 切换到其他工具
    SwitchTool { tool: String, reason: String },
    /// 停止当前任务
    StopTask { reason: String },
    /// 发送通知
    Notify { message: String, level: LogLevel },
    /// 记录日志
    Log { message: String, level: LogLevel },
    /// 修改配置
    SetConfig { key: String, value: String },
    /// 无动作
    None,
}

/// 插件上下文（暴露给插件的 API）
pub struct PluginContext {
    /// 获取工具列表
    pub fn list_tools(&self) -> Vec<ToolInfo>;
    /// 获取当前工具
    pub fn current_tool(&self) -> Option<String>;
    /// 切换工具
    pub fn switch_tool(&self, tool: &str) -> Result<()>;
    /// 获取配置
    pub fn get_config(&self, key: &str) -> Option<String>;
    /// 设置配置
    pub fn set_config(&self, key: &str, value: &str) -> Result<()>;
    /// 发送通知
    pub fn notify(&self, message: &str, level: LogLevel) -> Result<()>;
    /// 记录日志
    pub fn log(&self, message: &str, level: LogLevel);
    /// 获取成本统计
    pub fn get_cost_stats(&self) -> CostStats;
}
```

### 4.3 Lua 插件 API 示例

```lua
-- main.lua

-- 插件元信息
function metadata()
    return {
        name = "smart-router",
        version = "1.0.0"
    }
end

-- 工具启动前钩子
function on_tool_start(event)
    local tool = event.tool
    local task = event.task
    
    -- 智能路由逻辑
    if string.find(task, "refactor") then
        vcm.switch_tool("aider")
        return {
            action = "switch_tool",
            tool = "aider",
            reason = "Aider is better for refactoring tasks"
        }
    end
    
    if string.find(task, "debug") then
        vcm.switch_tool("claude-code")
        return {
            action = "switch_tool", 
            tool = "claude-code",
            reason = "Claude Code has better debugging capabilities"
        }
    end
    
    -- 检查免费额度
    local stats = vcm.get_cost_stats()
    if stats.gemini_remaining > 0 then
        vcm.log("Using Gemini CLI for free quota", "info")
        return {
            action = "switch_tool",
            tool = "gemini-cli",
            reason = "Using free Gemini quota"
        }
    end
    
    return { action = "none" }
end

-- 成本更新钩子
function on_cost_update(event)
    if event.total_cost > 50 then
        vcm.notify("Monthly cost exceeded $50!", "warning")
    end
end
```

### 4.4 JavaScript 插件 API 示例

```javascript
// main.js

export const metadata = {
    name: "cost-guard",
    version: "1.0.0"
};

export function onToolStart(event) {
    const { tool, task } = event;
    
    // 检查预算
    const stats = vcm.getCostStats();
    if (stats.monthlySpent > stats.monthlyBudget * 0.8) {
        vcm.notify(`Budget warning: ${stats.monthlySpent}/${stats.monthlyBudget}`, "warning");
        
        // 自动切换到免费工具
        return {
            action: "switch_tool",
            tool: "gemini-cli",
            reason: "Budget limit approaching, switching to free tier"
        };
    }
    
    return { action: "none" };
}

export function onCostUpdate(event) {
    const { tool, cost, totalCost } = event;
    
    vcm.log(`${tool} cost: $${cost.toFixed(4)}, total: $${totalCost.toFixed(2)}`, "info");
    
    // 保存成本记录
    vcm.setConfig(`cost_history.${Date.now()}`, JSON.stringify(event));
}
```

### 4.5 进程插件协议 (gRPC)

```protobuf
// plugin.proto

syntax = "proto3";

package vcm.plugin;

service Plugin {
    rpc GetMetadata(Empty) returns (PluginMetadata);
    rpc OnLoad(OnLoadRequest) returns (Empty);
    rpc OnUnload(Empty) returns (Empty);
    rpc OnToolStart(ToolStartEvent) returns (Action);
    rpc OnToolComplete(ToolCompleteEvent) returns (Action);
    rpc OnCostUpdate(CostUpdateEvent) returns (Action);
}

message PluginMetadata {
    string name = 1;
    string version = 2;
    string author = 3;
    string description = 4;
}

message ToolStartEvent {
    string tool = 1;
    string task = 2;
    map<string, string> context = 3;
}

message Action {
    oneof action {
        SwitchTool switch_tool = 1;
        StopTask stop_task = 2;
        Notify notify = 3;
        None none = 4;
    }
}

message SwitchTool {
    string tool = 1;
    string reason = 2;
}

message StopTask {
    string reason = 1;
}

message Notify {
    string message = 1;
    string level = 2;  // info, warning, error
}

message None {}
```

---

## 五、权限系统设计

### 5.1 权限级别

| 权限 | 说明 | 风险等级 |
|------|------|----------|
| `tools.read` | 读取工具列表和状态 | 低 |
| `tools.switch` | 切换当前工具 | 中 |
| `tools.install` | 安装新工具 | 高 |
| `config.read` | 读取配置 | 低 |
| `config.write` | 修改配置 | 中 |
| `fs.read` | 读取文件系统 | 中 |
| `fs.write` | 写入文件系统 | 高 |
| `network` | 网络访问 | 高 |
| `exec` | 执行系统命令 | 极高 |

### 5.2 权限检查流程

```
插件调用 API
    │
    ▼
┌─────────────────┐
│ Permission Check│
└────────┬────────┘
         │
    ┌────┴────┐
    │ Allowed?│
    └────┬────┘
         │
    ┌────┴────┐
   Yes        No
    │          │
    ▼          ▼
执行API    拒绝访问
           记录日志
```

### 5.3 安全沙箱

```rust
/// Lua 沙箱配置
pub fn create_sandbox(lua: &Lua, permissions: &Permissions) -> Result<()> {
    let globals = lua.globals();
    
    // 创建受限的 vcm 对象
    let vcm = lua.create_table()?;
    
    // 根据权限添加 API
    if permissions.tools.read {
        vcm.set("list_tools", lua.create_function(|_, _| {
            // 实现 list_tools
            Ok(())
        })?)?;
    }
    
    if permissions.tools.switch {
        vcm.set("switch_tool", lua.create_function(|_, tool: String| {
            // 实现 switch_tool
            Ok(())
        })?)?;
    }
    
    // 设置全局 vcm 对象
    globals.set("vcm", vcm)?;
    
    // 禁用危险函数
    globals.set("dofile", lua.create_function(|_, _| {
        Err(mlua::Error::runtime("dofile is disabled"))
    })?)?;
    
    globals.set("loadfile", lua.create_function(|_, _| {
        Err(mlua::Error::runtime("loadfile is disabled"))
    })?)?;
    
    Ok(())
}
```

---

## 六、插件市场设计

### 6.1 插件仓库结构

```
vcm-plugins/
├── official/
│   ├── smart-router/
│   │   ├── vcm-plugin.toml
│   │   ├── main.lua
│   │   └── README.md
│   ├── cost-guard/
│   │   ├── vcm-plugin.toml
│   │   ├── main.js
│   │   └── README.md
│   └── ...
├── community/
│   ├── awesome-theme/
│   ├── productivity-booster/
│   └── ...
└── registry.json
```

### 6.2 插件索引 (registry.json)

```json
{
  "version": "1",
  "updated": "2026-03-14",
  "plugins": [
    {
      "name": "smart-router",
      "version": "1.0.0",
      "author": "vcm-team",
      "runtime": "lua",
      "description": "Intelligent tool router",
      "repository": "https://github.com/vcm-plugins/smart-router",
      "downloads": 15000,
      "rating": 4.8,
      "tags": ["router", "productivity"],
      "verified": true
    },
    {
      "name": "cost-guard",
      "version": "2.1.0",
      "author": "community",
      "runtime": "js",
      "description": "Cost monitoring and budget protection",
      "repository": "https://github.com/vcm-plugins/cost-guard",
      "downloads": 8500,
      "rating": 4.6,
      "tags": ["cost", "budget"],
      "verified": false
    }
  ]
}
```

### 6.3 插件管理命令

```bash
# 搜索插件
vcm plugin search router

# 安装插件
vcm plugin install smart-router

# 安装指定版本
vcm plugin install smart-router@1.0.0

# 从 URL 安装
vcm plugin install https://github.com/user/my-plugin

# 列出已安装插件
vcm plugin list

# 更新插件
vcm plugin update smart-router

# 更新所有插件
vcm plugin update --all

# 卸载插件
vcm plugin remove smart-router

# 查看插件信息
vcm plugin info smart-router

# 启用/禁用插件
vcm plugin enable smart-router
vcm plugin disable smart-router
```

---

## 七、实现计划

### 7.1 Phase 1：核心框架 (v3.0)

**目标**：建立插件系统基础

**任务**：
- [ ] 定义插件 trait 和 API
- [ ] 实现 Lua 运行时
- [ ] 实现权限系统
- [ ] 实现插件加载器
- [ ] 实现 `vcm plugin` 命令组

**预计工作量**：5 天

### 7.2 Phase 2：扩展运行时 (v3.1)

**目标**：支持更多运行时

**任务**：
- [ ] 集成 Deno 运行时
- [ ] 实现进程插件支持
- [ ] (可选) 实现 WASM 支持

**预计工作量**：5 天

### 7.3 Phase 3：插件市场 (v3.2)

**目标**：建立插件生态

**任务**：
- [ ] 创建插件仓库
- [ ] 实现插件索引服务
- [ ] 开发 3-5 个官方插件
- [ ] 编写插件开发文档

**预计工作量**：7 天

---

## 八、官方插件规划

### 8.1 第一批官方插件

| 插件 | 运行时 | 功能 | 优先级 |
|------|--------|------|--------|
| **smart-router** | Lua | 智能工具路由 | P0 |
| **cost-guard** | JS | 成本监控保护 | P0 |
| **watchdog** | Lua | 后台任务通知 | P1 |
| **memory-vault** | Process | 会话记忆存储 | P1 |
| **workflow-forge** | JS | 工作流模板 | P2 |

### 8.2 smart-router 实现示例

```lua
-- smart-router/main.lua

local config = {
    enable_auto_switch = true,
    preferred_tools = {
        refactor = "aider",
        debug = "claude-code",
        write = "claude-code",
        explain = "gemini-cli"
    }
}

function on_tool_start(event)
    if not config.enable_auto_switch then
        return { action = "none" }
    end
    
    local task = string.lower(event.task)
    
    for pattern, tool in pairs(config.preferred_tools) do
        if string.find(task, pattern) then
            -- 检查工具是否已安装
            local tools = vcm.list_tools()
            for _, t in ipairs(tools) do
                if t.id == tool and t.installed then
                    vcm.log(string.format("Routing to %s for %s task", tool, pattern), "info")
                    return {
                        action = "switch_tool",
                        tool = tool,
                        reason = string.format("Detected %s task", pattern)
                    }
                end
            end
        end
    end
    
    -- 默认使用免费工具
    local stats = vcm.get_cost_stats()
    if stats.gemini_remaining > 0 then
        return {
            action = "switch_tool",
            tool = "gemini-cli",
            reason = "Using free Gemini quota"
        }
    end
    
    return { action = "none" }
end
```

---

## 九、风险与缓解

### 9.1 安全风险

| 风险 | 缓解措施 |
|------|----------|
| 恶意插件 | 权限沙箱 + 代码审核 + 用户确认 |
| 插件崩溃 | 进程隔离 + 异常捕获 |
| 资源泄露 | 资源限制 + 超时机制 |
| 数据泄露 | 权限控制 + 敏感数据隔离 |

### 9.2 兼容性风险

| 风险 | 缓解措施 |
|------|----------|
| API 变更 | 版本化 API + 废弃警告 |
| 运行时版本 | 锁定运行时版本 + 兼容层 |
| 平台差异 | 跨平台测试 + 平台特定配置 |

---

## 十、总结

### 推荐方案

**混合架构**：Lua (默认) + Deno (扩展) + 进程 (企业)

### 核心优势

1. **安全**：多层沙箱，权限控制
2. **灵活**：支持多种运行时，适应不同场景
3. **易用**：Lua 低门槛，JS 生态丰富
4. **可扩展**：插件市场驱动生态发展

### 下一步

1. 完善本方案细节
2. 开发插件系统原型
3. 实现 2-3 个官方插件验证架构
4. 开放社区贡献
