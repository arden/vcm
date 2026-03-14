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
        m.insert("msg.unknown", "Unknown");
        m.insert("msg.unknown_version", "Unknown");
        
        // 标签
        m.insert("label.tool", "Tool");
        m.insert("label.version", "Version");
        m.insert("label.status", "Status");
        m.insert("label.note", "Note");
        m.insert("label.tag", "Tag");
        m.insert("label.hint", "Hint");
        m.insert("label.path", "Path");
        m.insert("label.install_method", "Install Method");
        m.insert("label.missing_config", "Missing Config");
        m.insert("label.config_summary", "Config Summary");
        m.insert("label.configured", "Configured");
        m.insert("label.needs_config", "Needs Config");
        m.insert("label.missing", "Missing");
        m.insert("label.suggestion", "Suggestion");
        
        // 状态
        m.insert("status.healthy", "Healthy");
        m.insert("status.warning", "Warning");
        m.insert("status.error", "Error");
        m.insert("status.unknown", "Unknown");
        m.insert("status.title", "Tool Status");
        m.insert("status.completion", "Completion");
        
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
        m.insert("doctor.done", "Diagnostics complete");
        m.insert("doctor.registry_loaded", "Registry loaded with");
        m.insert("doctor.tools", "tools");
        m.insert("doctor.npm", "npm package manager");
        m.insert("doctor.pip", "pip package manager");
        m.insert("doctor.pipx", "pipx package manager");
        m.insert("doctor.cargo", "Cargo package manager");
        m.insert("doctor.brew", "Homebrew package manager");
        m.insert("doctor.go", "Go package manager");
        
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
        
        // 更多标签
        m.insert("label.name", "Name");
        m.insert("label.vendor", "Vendor");
        m.insert("label.website", "Website");
        m.insert("label.repository", "Repository");
        m.insert("label.args", "Args");
        m.insert("label.or", "or");
        
        // 更多消息
        m.insert("msg.cancelled", "Cancelled");
        m.insert("msg.failed", "Failed");
        m.insert("msg.skipped", "Skipped");
        m.insert("msg.unsupported_manager", "Unsupported package manager");
        
        // 工具相关
        m.insert("tool.not_found", "Tool not found: {}");
        
        // Install 命令扩展
        m.insert("install.using", "Installing with {}...");
        m.insert("install.next_steps", "Next steps:");
        m.insert("install.get_api_key", "Get API Key: {}");
        m.insert("install.configure", "Configure: vcm config {}");
        m.insert("install.no_method", "No available installation method found. Please install npm, pip, pipx or cargo first.");
        
        // Config 命令扩展
        m.insert("config.overview", "Configuration Overview");
        m.insert("config.api_key_status", "API Keys Status:");
        m.insert("config.default_tool", "Default tool");
        m.insert("config.env_var_set", "Environment variable {} set");
        m.insert("config.apply_changes", "Run {} to apply changes");
        m.insert("config.format_error", "Format error. Use: PROVIDER=KEY");
        m.insert("config.all_configured", "All environment variables are configured");
        m.insert("config.update_key", "Update API Key");
        m.insert("config.view_files", "View config files");
        m.insert("config.back", "Back");
        m.insert("config.select_action", "Select action");
        m.insert("config.need_config", "The following environment variables need configuration:");
        m.insert("config.input_prompt", "Enter {}");
        m.insert("config.config_files", "{} config files:");
        
        // Run 命令扩展
        m.insert("run.tool_not_installed", "Tool {} is not installed");
        m.insert("run.executable_not_found", "Cannot find tool executable");
        m.insert("run.configure_env", "Run {} to configure environment variables");
        m.insert("run.args", "Args: {}");
        
        // Default 命令扩展
        m.insert("default.launch_cmd", "Launch command:");
        m.insert("default.direct_launch", "direct launch");
        m.insert("default.specify_tool", "specify tool launch");
        m.insert("default.set_prompt", "Set default tool: {}");
        m.insert("default.tool_not_installed", "Tool is not installed yet");
        m.insert("default.run_install", "Run {} to install");
        
        // Info 命令
        m.insert("info.status_not_installed", "Status: Not installed");
        m.insert("info.config_paths", "Config paths:");
        m.insert("info.env_vars", "Environment variables:");
        m.insert("info.install_commands", "Install commands:");
        
        // Search 命令
        m.insert("search.no_results", "No matching tools found: {}");
        m.insert("search.results", "Search results: \"{}\" ({} found)");
        m.insert("search.install_hint", "Use {} to install a tool");
        
        // Outdated 命令
        m.insert("outdated.checking", "Checking for tool updates...");
        m.insert("outdated.all_latest", "All tools are up to date");
        m.insert("outdated.available", "The following tools have updates available:");
        m.insert("outdated.update_hint", "Run {} to update tools");
        
        // Update 命令
        m.insert("update.all", "Updating all installed tools...");
        m.insert("update.found_tools", "Found {} installed tools");
        m.insert("update.updating", "Updating {}...");
        m.insert("update.complete", "Update complete: {} succeeded, {} failed");
        m.insert("update.not_installed", "Tool {} is not installed. Install it first.");
        m.insert("update.cannot_update", "Cannot determine update method. Please update manually.");
        m.insert("update.updating_progress", "Updating...");
        m.insert("update.no_tools", "No installed tools to update");
        
        // Remove 命令
        m.insert("remove.confirm", "Are you sure you want to uninstall {}?");
        m.insert("remove.removing", "Uninstalling {}...");
        m.insert("remove.removed", "{} has been uninstalled");
        m.insert("remove.cannot_remove", "Cannot determine uninstall method. Please uninstall manually.");
        m.insert("remove.failed", "Uninstall failed");
        m.insert("remove.not_installed", "Tool {} is not installed");
        
        // Export 命令
        m.insert("export.exporting", "Exporting installed tools list...");
        m.insert("export.exported", "Exported {} tools to: {}");
        m.insert("export.tool_list", "Tool list:");
        
        // Import 命令
        m.insert("import.importing", "Importing tools list from file...");
        m.insert("import.file_info", "Import file info:");
        m.insert("import.source_host", "Source host");
        m.insert("import.tool_count", "Tool count");
        m.insert("import.to_install", "To install ({} tools):");
        m.insert("import.start_install", "Starting to install missing tools...");
        m.insert("import.all_installed", "All tools are already installed");
        m.insert("import.hint", "Tip: Use {} to auto-install missing tools");
        
        // Init 命令
        m.insert("init.step1", "Step 1/3: Scan installed tools");
        m.insert("init.step2", "Step 2/3: Select tools to install");
        m.insert("init.step3", "Step 3/3: Configure API Keys");
        m.insert("init.found_installed", "Found {} installed tools");
        m.insert("init.all_installed", "All tools are already installed");
        m.insert("init.select_tools", "Select tools to install (Space to select, Enter to confirm)");
        m.insert("init.will_install", "Will install the following tools:");
        m.insert("init.no_api_key", "Selected tools do not require API Key configuration");
        m.insert("init.need_api_key", "The following API Keys need configuration:");
        m.insert("init.configure_now", "Configure API Keys now?");
        m.insert("init.yes_configure", "Yes, configure now");
        m.insert("init.later", "Configure later");
        m.insert("init.input", "Enter {}");
        m.insert("init.add_to_shell", "Add to your shell config:");
        m.insert("init.complete", "Initialization complete!");
        m.insert("init.next_steps", "Next steps:");
        m.insert("init.install_tools", "Install tools: {}");
        m.insert("init.view_status", "View status: {}");
        m.insert("init.configure_tools", "Configure tools: {}");
        m.insert("init.check_updates", "Check updates: {}");
        m.insert("init.welcome", "Welcome to VCM setup wizard!");
        m.insert("init.welcome_desc", "This wizard will help you set up your CLI AI programming environment.");
        
        // Usage 命令
        m.insert("usage.title", "CLI AI Tool Usage Statistics");
        m.insert("usage.overview", "Overview");
        m.insert("usage.registry_tools", "Registry tools: {}");
        m.insert("usage.needs_config", "Needs config: {}");
        m.insert("usage.by_vendor", "By Vendor");
        m.insert("usage.by_method", "By Install Method");
        m.insert("usage.tools_count", "{} tools");
        m.insert("usage.recommended", "Recommended");
        m.insert("usage.config_suggestion", "Configuration Suggestions");
        
        // Registry 相关
        m.insert("registry.current_cache", "Current cache: {}");
        m.insert("registry.valid", "valid");
        m.insert("registry.none", "none");
        m.insert("registry.loaded", "Loaded {} tools");
        m.insert("registry.updated_at", "Updated at {}");
        
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
        m.insert("msg.unknown", "未知");
        m.insert("msg.unknown_version", "未知");
        
        // 标签
        m.insert("label.tool", "工具");
        m.insert("label.version", "版本");
        m.insert("label.status", "状态");
        m.insert("label.note", "备注");
        m.insert("label.tag", "标签");
        m.insert("label.hint", "提示");
        m.insert("label.path", "路径");
        m.insert("label.install_method", "安装方式");
        m.insert("label.missing_config", "缺少配置");
        m.insert("label.config_summary", "配置摘要");
        m.insert("label.configured", "已配置");
        m.insert("label.needs_config", "需要配置");
        m.insert("label.missing", "缺少");
        m.insert("label.suggestion", "建议");
        
        // 状态
        m.insert("status.healthy", "正常");
        m.insert("status.warning", "警告");
        m.insert("status.error", "错误");
        m.insert("status.unknown", "未知");
        m.insert("status.title", "工具状态");
        m.insert("status.completion", "完成度");
        
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
        m.insert("doctor.done", "诊断完成");
        m.insert("doctor.registry_loaded", "注册表已加载，包含");
        m.insert("doctor.tools", "个工具");
        m.insert("doctor.npm", "npm 包管理器");
        m.insert("doctor.pip", "pip 包管理器");
        m.insert("doctor.pipx", "pipx 包管理器");
        m.insert("doctor.cargo", "Cargo 包管理器");
        m.insert("doctor.brew", "Homebrew 包管理器");
        m.insert("doctor.go", "Go 包管理器");
        
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
        
        // 更多标签
        m.insert("label.name", "名称");
        m.insert("label.vendor", "供应商");
        m.insert("label.website", "官网");
        m.insert("label.repository", "仓库");
        m.insert("label.args", "参数");
        m.insert("label.or", "或");
        
        // 更多消息
        m.insert("msg.cancelled", "已取消");
        m.insert("msg.failed", "失败");
        m.insert("msg.skipped", "跳过");
        m.insert("msg.unsupported_manager", "暂不支持的包管理器");
        
        // 工具相关
        m.insert("tool.not_found", "未找到工具: {}");
        
        // Install 命令扩展
        m.insert("install.using", "使用 {} 安装...");
        m.insert("install.next_steps", "下一步:");
        m.insert("install.get_api_key", "获取 API Key: {}");
        m.insert("install.configure", "配置: vcm config {}");
        m.insert("install.no_method", "没有找到可用的安装方法。请安装 npm、pip、pipx 或 cargo 后重试。");
        
        // Config 命令扩展
        m.insert("config.overview", "配置概览");
        m.insert("config.api_key_status", "API Keys 状态:");
        m.insert("config.default_tool", "默认工具");
        m.insert("config.env_var_set", "已设置环境变量 {}");
        m.insert("config.apply_changes", "运行 {} 使配置生效");
        m.insert("config.format_error", "格式错误，请使用: PROVIDER=KEY");
        m.insert("config.all_configured", "所有环境变量已配置");
        m.insert("config.update_key", "更新 API Key");
        m.insert("config.view_files", "查看配置文件");
        m.insert("config.back", "返回");
        m.insert("config.select_action", "选择操作");
        m.insert("config.need_config", "以下环境变量需要配置:");
        m.insert("config.input_prompt", "请输入 {}");
        m.insert("config.config_files", "{} 配置文件:");
        
        // Run 命令扩展
        m.insert("run.tool_not_installed", "工具 {} 未安装");
        m.insert("run.executable_not_found", "无法找到工具的可执行文件");
        m.insert("run.configure_env", "运行 {} 配置环境变量");
        m.insert("run.args", "参数: {}");
        
        // Default 命令扩展
        m.insert("default.launch_cmd", "启动命令:");
        m.insert("default.direct_launch", "直接启动");
        m.insert("default.specify_tool", "指定工具启动");
        m.insert("default.set_prompt", "设置默认工具: {}");
        m.insert("default.tool_not_installed", "工具尚未安装");
        m.insert("default.run_install", "运行 {} 安装");
        
        // Info 命令
        m.insert("info.status_not_installed", "状态: 未安装");
        m.insert("info.config_paths", "配置路径:");
        m.insert("info.env_vars", "环境变量:");
        m.insert("info.install_commands", "安装命令:");
        
        // Search 命令
        m.insert("search.no_results", "未找到匹配的工具: {}");
        m.insert("search.results", "搜索结果: \"{}\" ({} 个)");
        m.insert("search.install_hint", "使用 {} 安装工具");
        
        // Outdated 命令
        m.insert("outdated.checking", "检查工具更新...");
        m.insert("outdated.all_latest", "所有工具都是最新版本");
        m.insert("outdated.available", "以下工具有更新可用:");
        m.insert("outdated.update_hint", "运行 {} 更新工具");
        
        // Update 命令
        m.insert("update.all", "更新所有已安装工具...");
        m.insert("update.found_tools", "发现 {} 个已安装工具");
        m.insert("update.updating", "更新 {}...");
        m.insert("update.complete", "更新完成: {} 成功, {} 失败");
        m.insert("update.not_installed", "工具 {} 未安装，请先安装");
        m.insert("update.cannot_update", "无法确定更新方式，请手动更新");
        m.insert("update.updating_progress", "正在更新...");
        m.insert("update.no_tools", "没有已安装的工具需要更新");
        
        // Remove 命令
        m.insert("remove.confirm", "确定要卸载 {} 吗?");
        m.insert("remove.removing", "卸载 {}...");
        m.insert("remove.removed", "已卸载 {}");
        m.insert("remove.cannot_remove", "无法确定卸载方式，请手动卸载");
        m.insert("remove.failed", "卸载失败");
        m.insert("remove.not_installed", "工具 {} 未安装");
        
        // Export 命令
        m.insert("export.exporting", "导出已安装工具列表...");
        m.insert("export.exported", "已导出 {} 个工具到: {}");
        m.insert("export.tool_list", "工具列表:");
        
        // Import 命令
        m.insert("import.importing", "从文件导入工具列表...");
        m.insert("import.file_info", "导入文件信息:");
        m.insert("import.source_host", "来源主机");
        m.insert("import.tool_count", "工具数量");
        m.insert("import.to_install", "待安装 ({} 个):");
        m.insert("import.start_install", "开始安装缺失的工具...");
        m.insert("import.all_installed", "所有工具都已安装");
        m.insert("import.hint", "提示: 使用 {} 自动安装缺失的工具");
        
        // Init 命令
        m.insert("init.step1", "步骤 1/3: 扫描已安装工具");
        m.insert("init.step2", "步骤 2/3: 选择要安装的工具");
        m.insert("init.step3", "步骤 3/3: 配置 API Key");
        m.insert("init.found_installed", "发现 {} 个已安装工具");
        m.insert("init.all_installed", "所有工具都已安装");
        m.insert("init.select_tools", "选择要安装的工具 (空格选择，回车确认)");
        m.insert("init.will_install", "将安装以下工具:");
        m.insert("init.no_api_key", "选中的工具不需要配置 API Key");
        m.insert("init.need_api_key", "以下 API Key 需要配置:");
        m.insert("init.configure_now", "是否现在配置 API Key?");
        m.insert("init.yes_configure", "是，现在配置");
        m.insert("init.later", "稍后配置");
        m.insert("init.input", "输入 {}");
        m.insert("init.add_to_shell", "添加到你的 shell 配置文件:");
        m.insert("init.complete", "初始化完成!");
        m.insert("init.next_steps", "下一步:");
        m.insert("init.install_tools", "安装工具: {}");
        m.insert("init.view_status", "查看状态: {}");
        m.insert("init.configure_tools", "配置工具: {}");
        m.insert("init.check_updates", "检查更新: {}");
        m.insert("init.welcome", "欢迎使用 VCM 初始化向导!");
        m.insert("init.welcome_desc", "这个向导将帮助你设置 CLI AI 编程环境。");
        
        // Usage 命令
        m.insert("usage.title", "CLI AI 工具使用统计");
        m.insert("usage.overview", "概览");
        m.insert("usage.registry_tools", "注册表工具: {}");
        m.insert("usage.needs_config", "待配置: {}");
        m.insert("usage.by_vendor", "按供应商");
        m.insert("usage.by_method", "按安装方式");
        m.insert("usage.tools_count", "{} 个工具");
        m.insert("usage.recommended", "推荐安装");
        m.insert("usage.config_suggestion", "配置建议");
        
        // Registry 相关
        m.insert("registry.current_cache", "当前缓存: {}");
        m.insert("registry.valid", "有效");
        m.insert("registry.none", "无");
        m.insert("registry.loaded", "已加载 {} 个工具");
        m.insert("registry.updated_at", "更新于 {}");
        
        m
    })
}

/// 初始化翻译
pub fn init() {
    // 预加载翻译
    let _ = get_en_translations();
    let _ = get_zh_translations();
}