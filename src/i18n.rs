//! 国际化支持模块

use std::sync::OnceLock;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    En,
    Zh,
}

impl Language {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Some(Language::En),
            "zh" | "zh-cn" | "chinese" | "中文" => Some(Language::Zh),
            _ => None,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Zh => "zh",
        }
    }

    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Zh => "中文",
        }
    }
}

/// 全局语言设置
static CURRENT_LANG: OnceLock<Language> = OnceLock::new();

/// 获取当前语言
pub fn current_lang() -> Language {
    *CURRENT_LANG.get_or_init(|| {
        // 1. 检查环境变量
        if let Ok(lang) = std::env::var("VCM_LANG") {
            if let Some(l) = Language::from_str(&lang) {
                return l;
            }
        }
        
        // 2. 检查系统语言
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("zh") || lang.starts_with("ZH") {
                return Language::Zh;
            }
        }
        
        // 3. 默认英文
        Language::En
    })
}

/// 设置当前语言
pub fn set_lang(lang: Language) {
    let _ = CURRENT_LANG.set(lang);
}

/// 翻译函数 - 返回 String 以支持动态 key
pub fn translate(key: &str) -> String {
    let lang = current_lang();
    let translations = match lang {
        Language::Zh => get_zh_translations(),
        Language::En => get_en_translations(),
    };
    
    translations.get(key).map(|s| s.to_string()).unwrap_or_else(|| key.to_string())
}

/// 英文翻译
static EN_TRANSLATIONS: OnceLock<std::collections::HashMap<&'static str, &'static str>> = OnceLock::new();

/// 中文翻译
static ZH_TRANSLATIONS: OnceLock<std::collections::HashMap<&'static str, &'static str>> = OnceLock::new();

fn get_en_translations() -> &'static std::collections::HashMap<&'static str, &'static str> {
    EN_TRANSLATIONS.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        
        // 通用
        m.insert("app.name", "VCM - Vibe Coding Manager");
        m.insert("app.description", "CLI AI Programming Tool Manager");
        
        // 命令名称
        m.insert("cmd.scan", "Scan system installed tools");
        m.insert("cmd.list", "List all known tools");
        m.insert("cmd.install", "Install tool");
        m.insert("cmd.update", "Update tool");
        m.insert("cmd.remove", "Uninstall tool");
        m.insert("cmd.config", "Configure tool");
        m.insert("cmd.status", "Check tool status");
        m.insert("cmd.search", "Search tools");
        m.insert("cmd.info", "Show tool details");
        m.insert("cmd.doctor", "System diagnostics");
        m.insert("cmd.update-registry", "Update registry");
        m.insert("cmd.completions", "Generate shell completion script");
        m.insert("cmd.outdated", "Check for updates");
        m.insert("cmd.export", "Export installed tools list");
        m.insert("cmd.import", "Import tools list from file");
        m.insert("cmd.init", "Interactive setup wizard");
        m.insert("cmd.usage", "Show tool usage statistics");
        m.insert("cmd.run", "Launch CLI AI tool");
        m.insert("cmd.default", "Set default tool");
        m.insert("cmd.lang", "Display or set language");
        
        // 通用消息
        m.insert("msg.installed", "Installed");
        m.insert("msg.not_installed", "Not installed");
        m.insert("msg.configured", "Configured");
        m.insert("msg.not_configured", "Not configured");
        m.insert("msg.success", "Success");
        m.insert("msg.error", "Error");
        m.insert("msg.warning", "Warning");
        
        // 状态
        m.insert("status.healthy", "Healthy");
        m.insert("status.warning", "Warning");
        m.insert("status.error", "Error");
        m.insert("status.unknown", "Unknown");
        
        // Scan 命令
        m.insert("scan.title", "Scanning installed CLI AI tools...");
        m.insert("scan.found", "Found {} installed tools");
        m.insert("scan.none", "No installed CLI AI tools found");
        
        // List 命令
        m.insert("list.title", "CLI AI Programming Tools ({} total)");
        m.insert("list.installed", "Installed");
        m.insert("list.recommended", "Recommended");
        m.insert("list.other", "Other Tools");
        m.insert("list.summary", "Installed: {} / {} ({}%)");
        m.insert("list.available", "Available: {} tools");
        
        // Install 命令
        m.insert("install.installing", "Installing {}...");
        m.insert("install.success", "Successfully installed: {}");
        m.insert("install.failed", "Failed to install: {}");
        m.insert("install.already_installed", "{} is already installed");
        
        // Config 命令
        m.insert("config.title", "Configuring {}");
        m.insert("config.api_key_set", "API Key set successfully");
        m.insert("config.missing_key", "Missing API Key: {}");
        
        // Doctor 命令
        m.insert("doctor.title", "System Diagnostics");
        m.insert("doctor.package_managers", "Package Managers");
        m.insert("doctor.api_keys", "API Keys");
        m.insert("doctor.registry", "Tool Registry");
        
        // Run 命令
        m.insert("run.launching", "Launching {}...");
        m.insert("run.not_installed", "Tool {} is not installed. Run `vcm install {}` to install");
        m.insert("run.missing_env", "Missing environment variables:");
        
        // Default 命令
        m.insert("default.set", "Default tool set to: {}");
        m.insert("default.current", "Current default tool: {}");
        m.insert("default.none", "No default tool set");
        m.insert("default.not_cli", "{} is not a CLI tool, cannot set as default");
        
        // 提示
        m.insert("hint.install", "Run `vcm install <tool>` to install a new tool");
        m.insert("hint.config", "Run `vcm config <tool>` to configure");
        m.insert("hint.update", "Run `vcm update <tool>` to update");
        
        m
    })
}

fn get_zh_translations() -> &'static std::collections::HashMap<&'static str, &'static str> {
    ZH_TRANSLATIONS.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        
        // 通用
        m.insert("app.name", "VCM - Vibe Coding Manager");
        m.insert("app.description", "CLI AI 编程工具管理器");
        
        // 命令名称
        m.insert("cmd.scan", "扫描系统已安装的工具");
        m.insert("cmd.list", "列出所有已知工具");
        m.insert("cmd.install", "安装工具");
        m.insert("cmd.update", "更新工具");
        m.insert("cmd.remove", "卸载工具");
        m.insert("cmd.config", "配置工具");
        m.insert("cmd.status", "检查工具状态");
        m.insert("cmd.search", "搜索工具");
        m.insert("cmd.info", "显示工具详情");
        m.insert("cmd.doctor", "系统诊断");
        m.insert("cmd.update-registry", "更新注册表");
        m.insert("cmd.completions", "生成 shell 补全脚本");
        m.insert("cmd.outdated", "检查已安装工具是否有更新");
        m.insert("cmd.export", "导出已安装工具列表");
        m.insert("cmd.import", "从文件导入工具列表");
        m.insert("cmd.init", "交互式初始化向导");
        m.insert("cmd.usage", "显示工具使用统计");
        m.insert("cmd.run", "启动 CLI AI 工具");
        m.insert("cmd.default", "设置默认工具");
        m.insert("cmd.lang", "显示或设置语言");
        
        // 通用消息
        m.insert("msg.installed", "已安装");
        m.insert("msg.not_installed", "未安装");
        m.insert("msg.configured", "已配置");
        m.insert("msg.not_configured", "未配置");
        m.insert("msg.success", "成功");
        m.insert("msg.error", "错误");
        m.insert("msg.warning", "警告");
        
        // 状态
        m.insert("status.healthy", "正常");
        m.insert("status.warning", "警告");
        m.insert("status.error", "错误");
        m.insert("status.unknown", "未知");
        
        // Scan 命令
        m.insert("scan.title", "扫描已安装的 CLI AI 工具...");
        m.insert("scan.found", "共发现 {} 个已安装工具");
        m.insert("scan.none", "未发现已安装的 CLI AI 工具");
        
        // List 命令
        m.insert("list.title", "CLI AI 编程工具 (共 {} 个)");
        m.insert("list.installed", "已安装");
        m.insert("list.recommended", "热门推荐");
        m.insert("list.other", "其他工具");
        m.insert("list.summary", "已安装: {} / {} ({}%)");
        m.insert("list.available", "可安装: {} 个工具");
        
        // Install 命令
        m.insert("install.installing", "正在安装 {}...");
        m.insert("install.success", "安装完成: {}");
        m.insert("install.failed", "安装失败: {}");
        m.insert("install.already_installed", "{} 已经安装");
        
        // Config 命令
        m.insert("config.title", "配置 {}");
        m.insert("config.api_key_set", "API Key 设置成功");
        m.insert("config.missing_key", "缺少 API Key: {}");
        
        // Doctor 命令
        m.insert("doctor.title", "系统诊断");
        m.insert("doctor.package_managers", "包管理器");
        m.insert("doctor.api_keys", "API Keys");
        m.insert("doctor.registry", "工具注册表");
        
        // Run 命令
        m.insert("run.launching", "启动 {}...");
        m.insert("run.not_installed", "工具 {} 未安装。运行 `vcm install {}` 安装");
        m.insert("run.missing_env", "以下环境变量未配置:");
        
        // Default 命令
        m.insert("default.set", "已设置默认工具: {}");
        m.insert("default.current", "当前默认工具: {}");
        m.insert("default.none", "未设置默认工具");
        m.insert("default.not_cli", "{} 不是 CLI 工具，无法设置为默认");
        
        // 提示
        m.insert("hint.install", "运行 `vcm install <tool>` 安装新工具");
        m.insert("hint.config", "运行 `vcm config <tool>` 配置");
        m.insert("hint.update", "运行 `vcm update <tool>` 更新");
        
        m
    })
}

/// 初始化翻译
pub fn init() {
    // 预加载翻译
    let _ = get_en_translations();
    let _ = get_zh_translations();
}