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
    /// 从字符串解析语言
    pub fn parse(s: &str) -> Option<Self> {
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

impl std::str::FromStr for Language {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or(())
    }
}

/// 全局语言设置
static CURRENT_LANG: OnceLock<Language> = OnceLock::new();

/// 获取当前语言
pub fn current_lang() -> Language {
    *CURRENT_LANG.get_or_init(|| {
        // 1. 检查环境变量
        if let Ok(lang) = std::env::var("VCM_LANG") {
            if let Some(l) = Language::parse(&lang) {
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
        m.insert("msg.yes", "Yes");
        m.insert("msg.no", "No");
        
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
        m.insert("list.not_installed", "Not Installed");
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
        m.insert("import.exported_at", "Exported at");
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
        
        // Free 命令
        m.insert("free.title", "Tools with Free AI Models");
        m.insert("free.subtitle", "Find tools that offer free access to pro-grade AI models");
        m.insert("free.pro_grade", "Pro-Grade Models");
        m.insert("free.free_tier", "Free Tier");
        m.insert("free.free_models", "Free Models");
        m.insert("free.limit", "Free Limit");
        m.insert("free.card_required", "Card Required");
        m.insert("free.no_card", "No card");
        m.insert("free.card_needed", "Card needed");
        m.insert("free.pro_models", "Pro Models");
        m.insert("free.aggregate_title", "Free Quota Aggregate Panel");
        m.insert("free.no_free_tools", "No tools with free quota found");
        m.insert("free.aggregate_stats", "Aggregate Statistics");
        m.insert("free.tools_with_free", "Tools with free quota");
        m.insert("free.tools_with_pro", "Tools with pro models");
        m.insert("free.free_pro_models", "Free pro models available");
        m.insert("free.optimal_strategy", "Optimal Free Combination Strategy");
        m.insert("free.recommended_combo", "Recommended combination");
        m.insert("free.compare_hint", "Tip: Use 'vcm compare <tool1> <tool2>...' to compare tools");
        m.insert("free.pro_only_hint", "Use 'vcm free --pro' to show pro models only");
        m.insert("free.install_hint", "Install: vcm install {}");
        m.insert("free.none_found", "No tools with free models found");
        m.insert("free.best_choice", "Best Free Choice!");
        
        // 定价相关
        m.insert("pricing.free", "FREE");
        m.insert("pricing.paid", "Paid");
        m.insert("pricing.free_tier_available", "Free tier available");
        m.insert("pricing.no_free_tier", "No free tier");
        m.insert("pricing.models_available", "Models available");
        m.insert("pricing.pro_grade", "Pro-Grade");
        m.insert("pricing.note", "Note");
        
        // 后端错误消息
        m.insert("backend.npm_install_failed", "npm install failed");
        m.insert("backend.pip_install_failed", "pip install failed");
        m.insert("backend.pipx_install_failed", "pipx install failed");
        m.insert("backend.cargo_install_failed", "cargo install failed");
        m.insert("backend.brew_install_failed", "brew install failed");
        m.insert("backend.go_install_failed", "go install failed");
        m.insert("backend.npm_update_failed", "npm update failed");
        m.insert("backend.pip_update_failed", "pip update failed");
        m.insert("backend.cargo_update_failed", "cargo update failed");
        m.insert("backend.brew_update_failed", "brew update failed");
        m.insert("backend.npm_remove_failed", "npm uninstall failed");
        m.insert("backend.pip_remove_failed", "pip uninstall failed");
        m.insert("backend.cargo_remove_failed", "cargo uninstall failed");
        m.insert("backend.brew_remove_failed", "brew uninstall failed");
        
        // Alias command
        m.insert("alias.title", "Tool Aliases");
        m.insert("alias.none", "No aliases configured");
        m.insert("alias.set", "Alias set: {} -> {}");
        m.insert("alias.removed", "Alias removed: {}");
        m.insert("alias.not_found", "Alias '{}' does not exist");
        m.insert("alias.reserved", "'{}' is a reserved command, cannot be used as alias");
        m.insert("alias.tool_not_found", "Warning: Tool '{}' not found in registry, but alias will still be created");
        m.insert("alias.overwrite", "Alias '{}' already maps to '{}', updating to '{}'");
        m.insert("alias.hint", "Tip: Use 'vcm <alias>' to quickly launch a tool");
        m.insert("alias.hint_example", "Example: 'vcm cc' will launch claude-code (if alias cc is set)");
        m.insert("alias.list_title", "Tool Alias List");
        
        // Compare command
        m.insert("compare.title", "Tool Comparison");
        m.insert("compare.min_tools", "At least two tools are required for comparison");
        m.insert("compare.max_tools", "Maximum 5 tools can be compared at once");
        m.insert("compare.tool_not_found", "Tool '{}' not found");
        m.insert("compare.feature", "Feature");
        m.insert("compare.vendor", "Vendor");
        m.insert("compare.free_quota", "Free Quota");
        m.insert("compare.pro_models", "Pro Models");
        m.insert("compare.card_required", "Card Required");
        m.insert("compare.install_method", "Install Method");
        m.insert("compare.tags", "Tags");
        m.insert("compare.model_details", "Available Model Details");
        m.insert("compare.free_models", "Free Models");
        m.insert("compare.paid_models", "Paid Models");
        m.insert("compare.pro_grade", "Pro-Grade");
        
        // Quota command
        m.insert("quota.title", "Quota Monitoring Panel");
        m.insert("quota.threshold_settings", "Threshold Settings");
        m.insert("quota.threshold_range", "Threshold must be between 0-100");
        m.insert("quota.warn_threshold", "Warning Threshold");
        m.insert("quota.hard_limit", "Hard Limit");
        m.insert("quota.default_80", "Default 80%");
        m.insert("quota.not_set", "Not Set");
        m.insert("quota.disabled", "Disabled");
        m.insert("quota.block_on_exceed", "(Will block on exceed)");
        m.insert("quota.warn_hint", "Tip: Use 'vcm quota warn 80' to set warning threshold");
        m.insert("quota.usage_hint", "      Use 'vcm quota usage <tool>' to view detailed usage records");
        m.insert("quota.warn_set", "Warning threshold set to {}%");
        m.insert("quota.warn_desc", "When usage reaches this threshold, a warning will be displayed");
        m.insert("quota.limit_set", "Hard limit set to {}%");
        m.insert("quota.limit_warning", "When usage reaches this threshold, the system will block further use");
        m.insert("quota.limit_disabled", "Hard limit disabled");
        m.insert("quota.usage_title", "{} Usage Records");
        m.insert("quota.today_usage", "Today usage");
        m.insert("quota.month_usage", "Month usage");
        m.insert("quota.total_usage", "Total usage");
        m.insert("quota.last_used", "Last used");
        m.insert("quota.no_records", "No usage records");
        m.insert("quota.free_limit", "Free Limit");
        m.insert("quota.summary_title", "Usage Records Summary");
        m.insert("quota.run_hint", "Tip: Use 'vcm run <tool>' to automatically record usage when launching tools");
        m.insert("quota.reset_tool", "Usage records for '{}' have been reset");
        m.insert("quota.no_records_for_tool", "Tool '{}' has no usage records");
        m.insert("quota.all_reset", "All usage records have been reset");
        m.insert("quota.tool", "Tool");
        m.insert("quota.today", "Today");
        m.insert("quota.month", "Month");
        m.insert("quota.limit_col", "Limit");
        m.insert("quota.status", "Status");
        m.insert("quota.unlimited", "Limited");
        m.insert("quota.daily_limit", "Daily Limit");
        m.insert("quota.monthly_limit", "Monthly Limit");
        m.insert("quota.not_used", "Not used");
        
        // Stats command
        m.insert("stats.title", "Usage Statistics Panel");
        m.insert("stats.no_records", "No usage records");
        m.insert("stats.run_hint", "Tip: Usage is automatically recorded when launching tools with 'vcm run <tool>'");
        m.insert("stats.today_ranking", "Today's Usage Ranking");
        m.insert("stats.no_today", "No usage today");
        m.insert("stats.month_stats", "Monthly Usage Statistics");
        m.insert("stats.times", "times");
        m.insert("stats.no_month", "No usage this month");
        m.insert("stats.month_total", "Month total usage");
        m.insert("stats.trend", "Usage Trend");
        m.insert("stats.total_calls", "Total calls");
        m.insert("stats.tools_used", "Tools used");
        m.insert("stats.most_used", "Most used tool");
        m.insert("stats.cost_hint", "Tip: Use 'vcm cost' to view cost estimation");
        m.insert("stats.quota_hint", "      Use 'vcm quota usage' to view detailed usage records");
        m.insert("cost.title", "Cost Estimation Report");
        m.insert("cost.no_records", "No usage records, cannot estimate cost");
        m.insert("cost.month_estimate", "Estimated monthly cost");
        m.insert("cost.optimization", "Cost Optimization Suggestions");
        m.insert("cost.register_free", "Register for {} tools' free quotas to save costs");
        m.insert("cost.view_free", "Use 'vcm free --aggregate' to view all free quotas");
        m.insert("cost.compare_hint", "Use 'vcm compare <tool1> <tool2>' to compare tool cost-effectiveness");
        m.insert("cost.all_free", "All Free!");
        m.insert("cost.use_free_quota", "Using free quota");
        m.insert("cost.consider_free_alt", "Consider free alternatives");
        
        // Project command
        m.insert("project.not_found", "Project root not found. Run this command in a project directory, or use 'vcm project init' to initialize.");
        m.insert("project.initialized", "Project initialized");
        m.insert("project.config_dir", "Config directory");
        m.insert("project.next_steps", "Next steps");
        m.insert("project.use_hint", "Use 'vcm project use <tool>' to set default tool");
        m.insert("project.edit_hint", "Edit .vcm/config.toml to configure tool parameters");
        m.insert("project.title", "Project Configuration");
        m.insert("project.name", "Project Name");
        m.insert("project.default_tool", "Default Tool");
        m.insert("project.not_set", "Not set");
        m.insert("project.tool_config", "Tool Configuration");
        m.insert("project.model", "Model");
        m.insert("project.env_vars", "Environment Variables");
        m.insert("project.tool_not_found_warn", "Tool '{}' not found in registry, but configuration will still be saved");
        m.insert("project.default_set", "Project default tool set");
        m.insert("project.config_saved", "Configuration saved to .vcm/config.toml");
        m.insert("project.vcm_dir", ".vcm directory");
        m.insert("project.config_file", "Config file");
        
        // Fallback command
        m.insert("fallback.title", "Smart Fallback Configuration");
        m.insert("fallback.status", "Status");
        m.insert("fallback.enabled", "Enabled");
        m.insert("fallback.disabled", "Disabled");
        m.insert("fallback.default_chain", "Default Fallback Chain");
        m.insert("fallback.custom_chains", "Custom Fallback Chains");
        m.insert("fallback.enable_hint", "Tip: Use 'vcm fallback --enable' to enable smart fallback");
        m.insert("fallback.add_hint", "      Use 'vcm fallback add <primary> <fallback1> [fallback2]...' to add fallback chain");
        m.insert("fallback.primary", "Primary");
        m.insert("fallback.backup", "Backup");
        m.insert("fallback.fallback", "Fallback");
        m.insert("fallback.primary_not_found", "Primary tool '{}' not found, but configuration will still be saved");
        m.insert("fallback.chain_added", "Fallback chain added");
        m.insert("fallback.order", "Fallback order");
        m.insert("fallback.not_enabled", "Smart fallback is not enabled");
        m.insert("fallback.use_enable", "Use 'vcm fallback --enable' to enable");
        m.insert("fallback.chain_removed", "Fallback chain for '{}' removed");
        m.insert("fallback.chain_not_found", "No fallback chain found for '{}'");
        m.insert("fallback.enabled_msg", "Smart fallback enabled");
        m.insert("fallback.auto_switch", "When primary tool is unavailable, the system will automatically switch to backup tools");
        m.insert("fallback.disabled_msg", "Smart fallback disabled");
        m.insert("fallback.default_set", "Default fallback chain set");
        m.insert("fallback.need_one_backup", "At least one backup tool is required");
        m.insert("fallback.need_two_tools", "Default fallback chain needs at least 2 tools");
        
        // Key command
        m.insert("key.title", "Multi-Account Management");
        m.insert("key.no_config", "No saved key configuration");
        m.insert("key.add_hint", "Tip: Use 'vcm key add <tool> <name> <key>' to add an account");
        m.insert("key.not_found", "No key configuration found for '{}'");
        m.insert("key.active", "Active");
        m.insert("key.rotation_mode", "Rotation mode");
        m.insert("key.add_cmd", "Command: vcm key add <tool> <name> <key>   Add account");
        m.insert("key.switch_cmd", "      vcm key switch <tool> <name>       Switch account");
        m.insert("key.remove_cmd", "      vcm key remove <tool> <name>       Remove account");
        m.insert("key.rotate_cmd", "      vcm key rotate <tool> --enable     Enable rotation");
        m.insert("key.name_empty", "Key name cannot be empty");
        m.insert("key.value_empty", "Key value cannot be empty");
        m.insert("key.per_request", "Per request");
        m.insert("key.hourly", "Hourly");
        m.insert("key.daily", "Daily");
        m.insert("key.weekly", "Weekly");
        m.insert("key.exists", "Key '{}' already exists, will be overwritten");
        m.insert("key.added", "Key '{}' added to {}");
        m.insert("key.set_active", "Set as current active account");
        m.insert("key.switched", "Account switched to");
        m.insert("key.restart_hint", "Restart the tool for changes to take effect");
        m.insert("key.removed", "Key removed");
        m.insert("key.not_found_name", "Key not found");
        m.insert("key.rotate_need_two", "Enabling rotation requires at least 2 keys");
        m.insert("key.current_count", "Currently only {} key(s)");
        m.insert("key.rotation_enabled", "Key rotation enabled");
        m.insert("key.rotation_desc", "Each request will use a different key");
        m.insert("key.rotation_disabled", "Key rotation disabled");
        m.insert("key.current_active", "Current active");
        m.insert("key.type_trial", "Trial");
        m.insert("key.expires", "Expires");
        m.insert("key.no_active", "No active account set");
        m.insert("key.no_saved", "No saved keys");
        m.insert("key.name", "Name");
        m.insert("key.status", "Status");
        m.insert("key.type", "Type");
        m.insert("key.note", "Note");
        m.insert("key.official", "Official");
        
        // Recommend command
        m.insert("recommend.title", "Personalized Recommendations");
        m.insert("recommend.based_usage", "Based on your usage habits and needs");
        m.insert("recommend.install_free", "Recommended Installation (Free Pro-Grade Models)");
        m.insert("recommend.hot", "Hot Recommendations");
        m.insert("recommend.installed_overview", "Installed Overview");
        m.insert("recommend.installed_count", "Installed: {} tools");
        m.insert("recommend.configured_count", "Configured: {c} / {t}");
        m.insert("recommend.some_unconfigured", "Some tools not configured, run 'vcm status' for details");
        m.insert("recommend.trending_title", "Trending Tools Ranking");
        m.insert("recommend.rank", "Rank");
        m.insert("recommend.tool_col", "Tool");
        m.insert("recommend.vendor_col", "Vendor");
        m.insert("recommend.free_quota_col", "Free Quota");
        m.insert("recommend.pro_free", "Pro-Grade Free");
        m.insert("recommend.paid", "Paid");
        m.insert("recommend.unknown", "Unknown");
        m.insert("recommend.install_hint", "Use 'vcm install <tool>' to install");
        m.insert("recommend.new_title", "Newly Added Tools");
        m.insert("recommend.free_pro_models", "Free Pro-Grade Models");
        m.insert("recommend.by_tag", "Filter by Tag");
        m.insert("recommend.no_tag_tools", "No tools found with tag '{}'");
        m.insert("recommend.available_tags", "Available tags: ai, coding, cli, llm, google, anthropic, opensource");
        m.insert("recommend.found_tools", "Found {} tools");
        m.insert("recommend.has_free_quota", "Has free quota");
        m.insert("recommend.install", "Install");
        
        // Free --aggregate
        m.insert("free.aggregate_title", "Free Quota Aggregation Panel");
        m.insert("free.no_free_tools", "No tools with free quotas found");
        m.insert("free.aggregate_stats", "Aggregation Statistics");
        m.insert("free.tools_with_free", "Tools with free quotas");
        m.insert("free.tools_with_pro", "Tools with pro-grade models");
        m.insert("free.free_pro_models", "Free pro-grade models available");
        m.insert("free.optimal_strategy", "Optimal Free Combination Strategy");
        m.insert("free.recommended_combo", "Recommended Combination");
        m.insert("free.compare_hint", "Tip: Use 'vcm compare <tool1> <tool2>...' to compare tools");
        m.insert("free.pro_only_hint", "      Use 'vcm free --pro' to show only pro-grade models");
        
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
        m.insert("msg.yes", "是");
        m.insert("msg.no", "否");
        
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
        m.insert("list.not_installed", "未安装");
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
        m.insert("import.exported_at", "导出时间");
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
        
        // Free 命令
        m.insert("free.title", "支持免费模型的工具");
        m.insert("free.subtitle", "查找提供免费专业级 AI 模型访问的工具");
        m.insert("free.pro_grade", "专业级模型");
        m.insert("free.free_tier", "免费额度");
        m.insert("free.free_models", "免费模型");
        m.insert("free.limit", "免费限额");
        m.insert("free.card_required", "需要信用卡");
        m.insert("free.no_card", "无需信用卡");
        m.insert("free.card_needed", "需要信用卡");
        m.insert("free.pro_models", "专业级模型");
        m.insert("free.aggregate_title", "免费额度聚合面板");
        m.insert("free.no_free_tools", "未找到有免费额度的工具");
        m.insert("free.aggregate_stats", "聚合统计");
        m.insert("free.tools_with_free", "有免费额度的工具");
        m.insert("free.tools_with_pro", "提供专业级模型的工具");
        m.insert("free.free_pro_models", "可免费使用的专业级模型");
        m.insert("free.optimal_strategy", "最优免费组合策略");
        m.insert("free.recommended_combo", "推荐组合");
        m.insert("free.compare_hint", "提示: 使用 'vcm compare <tool1> <tool2>...' 对比多个工具");
        m.insert("free.pro_only_hint", "使用 'vcm free --pro' 只显示专业级模型");
        m.insert("free.install_hint", "安装: vcm install {}");
        m.insert("free.none_found", "未找到支持免费模型的工具");
        m.insert("free.best_choice", "最佳免费选择!");
        
        // 定价相关
        m.insert("pricing.free", "免费");
        m.insert("pricing.paid", "付费");
        m.insert("pricing.free_tier_available", "有免费额度");
        m.insert("pricing.no_free_tier", "无免费额度");
        m.insert("pricing.models_available", "可用模型");
        m.insert("pricing.pro_grade", "专业级");
        m.insert("pricing.note", "备注");
        
        // 后端错误消息
        m.insert("backend.npm_install_failed", "npm 安装失败");
        m.insert("backend.pip_install_failed", "pip 安装失败");
        m.insert("backend.pipx_install_failed", "pipx 安装失败");
        m.insert("backend.cargo_install_failed", "cargo 安装失败");
        m.insert("backend.brew_install_failed", "brew 安装失败");
        m.insert("backend.go_install_failed", "go 安装失败");
        m.insert("backend.npm_update_failed", "npm 更新失败");
        m.insert("backend.pip_update_failed", "pip 更新失败");
        m.insert("backend.cargo_update_failed", "cargo 更新失败");
        m.insert("backend.brew_update_failed", "brew 更新失败");
        m.insert("backend.npm_remove_failed", "npm 卸载失败");
        m.insert("backend.pip_remove_failed", "pip 卸载失败");
        m.insert("backend.cargo_remove_failed", "cargo 卸载失败");
        m.insert("backend.brew_remove_failed", "brew 卸载失败");
        
        // Alias 命令
        m.insert("alias.title", "工具别名列表");
        m.insert("alias.none", "未设置任何别名");
        m.insert("alias.set", "已设置别名: {} -> {}");
        m.insert("alias.removed", "已删除别名: {}");
        m.insert("alias.not_found", "别名 '{}' 不存在");
        m.insert("alias.reserved", "'{}' 是保留命令，不能用作别名");
        m.insert("alias.tool_not_found", "警告: 工具 '{}' 未在注册表中找到，但仍会创建别名");
        m.insert("alias.overwrite", "别名 '{}' 已映射到 '{}'，将更新为 '{}'");
        m.insert("alias.hint", "提示: 使用 'vcm <alias>' 快速启动工具");
        m.insert("alias.hint_example", "示例: 'vcm cc' 将启动 claude-code (如果设置了别名 cc)");
        m.insert("alias.list_title", "工具别名列表");
        
        // Compare 命令
        m.insert("compare.title", "工具对比");
        m.insert("compare.min_tools", "至少需要指定两个工具进行对比");
        m.insert("compare.max_tools", "最多支持同时对比 5 个工具");
        m.insert("compare.tool_not_found", "工具 '{}' 未找到");
        m.insert("compare.feature", "特性");
        m.insert("compare.vendor", "供应商");
        m.insert("compare.free_quota", "免费额度");
        m.insert("compare.pro_models", "专业模型");
        m.insert("compare.card_required", "需信用卡");
        m.insert("compare.install_method", "安装方式");
        m.insert("compare.tags", "标签");
        m.insert("compare.model_details", "可用模型详情");
        m.insert("compare.free_models", "免费模型");
        m.insert("compare.paid_models", "付费模型");
        m.insert("compare.pro_grade", "专业级");
        
        // Quota 命令
        m.insert("quota.title", "配额监控面板");
        m.insert("quota.threshold_settings", "阈值设置");
        m.insert("quota.threshold_range", "阈值必须在 0-100 之间");
        m.insert("quota.warn_threshold", "警告阈值");
        m.insert("quota.hard_limit", "硬限制");
        m.insert("quota.default_80", "默认 80%");
        m.insert("quota.not_set", "未设置");
        m.insert("quota.disabled", "禁用");
        m.insert("quota.block_on_exceed", "(超限将阻止使用)");
        m.insert("quota.warn_hint", "提示: 使用 'vcm quota warn 80' 设置警告阈值");
        m.insert("quota.usage_hint", "      使用 'vcm quota usage <tool>' 查看详细使用记录");
        m.insert("quota.warn_set", "警告阈值已设置为 {}%");
        m.insert("quota.warn_desc", "当使用量达到此阈值时，系统将显示警告提示");
        m.insert("quota.limit_set", "硬限制已设置为 {}%");
        m.insert("quota.limit_warning", "当使用量达到此阈值时，系统将阻止继续使用");
        m.insert("quota.limit_disabled", "硬限制已禁用");
        m.insert("quota.usage_title", "{} 使用记录");
        m.insert("quota.today_usage", "今日使用");
        m.insert("quota.month_usage", "本月使用");
        m.insert("quota.total_usage", "总使用量");
        m.insert("quota.last_used", "最后使用");
        m.insert("quota.no_records", "暂无使用记录");
        m.insert("quota.free_limit", "免费限额");
        m.insert("quota.summary_title", "使用记录汇总");
        m.insert("quota.run_hint", "提示: 使用 'vcm run <tool>' 启动工具时会自动记录使用量");
        m.insert("quota.reset_tool", "已重置 '{}' 的使用记录");
        m.insert("quota.no_records_for_tool", "工具 '{}' 没有使用记录");
        m.insert("quota.all_reset", "已重置所有使用记录");
        m.insert("quota.tool", "工具");
        m.insert("quota.today", "今日");
        m.insert("quota.month", "本月");
        m.insert("quota.limit_col", "限额");
        m.insert("quota.status", "状态");
        m.insert("quota.unlimited", "有限额");
        m.insert("quota.daily_limit", "每日限制");
        m.insert("quota.monthly_limit", "每月限制");
        m.insert("quota.not_used", "未使用");
        
        // Stats 命令
        m.insert("stats.title", "使用统计面板");
        m.insert("stats.no_records", "暂无使用记录");
        m.insert("stats.run_hint", "提示: 使用 'vcm run <tool>' 启动工具时会自动记录使用量");
        m.insert("stats.today_ranking", "今日使用排行");
        m.insert("stats.no_today", "今日暂无使用记录");
        m.insert("stats.month_stats", "本月使用统计");
        m.insert("stats.times", "次");
        m.insert("stats.no_month", "本月暂无使用记录");
        m.insert("stats.month_total", "本月总使用");
        m.insert("stats.trend", "使用趋势");
        m.insert("stats.total_calls", "总调用次数");
        m.insert("stats.tools_used", "使用过的工具");
        m.insert("stats.most_used", "最常用工具");
        m.insert("stats.cost_hint", "提示: 使用 'vcm cost' 查看成本估算");
        m.insert("stats.quota_hint", "      使用 'vcm quota usage' 查看详细使用记录");
        m.insert("cost.title", "成本估算报告");
        m.insert("cost.no_records", "暂无使用记录，无法估算成本");
        m.insert("cost.month_estimate", "本月估算总成本");
        m.insert("cost.optimization", "成本优化建议");
        m.insert("cost.register_free", "注册 {} 个工具的免费额度可节省成本");
        m.insert("cost.view_free", "使用 'vcm free --aggregate' 查看所有免费额度");
        m.insert("cost.compare_hint", "使用 'vcm compare <tool1> <tool2>' 对比工具性价比");
        m.insert("cost.all_free", "全部免费!");
        m.insert("cost.use_free_quota", "利用免费额度");
        m.insert("cost.consider_free_alt", "考虑免费替代");
        
        // Project 命令
        m.insert("project.not_found", "未找到项目根目录。请确保在项目目录中运行此命令，或使用 'vcm project init' 初始化项目配置。");
        m.insert("project.initialized", "项目已初始化");
        m.insert("project.config_dir", "配置目录");
        m.insert("project.next_steps", "下一步");
        m.insert("project.use_hint", "使用 'vcm project use <tool>' 设置默认工具");
        m.insert("project.edit_hint", "编辑 .vcm/config.toml 配置工具参数");
        m.insert("project.title", "项目配置");
        m.insert("project.name", "项目名称");
        m.insert("project.default_tool", "默认工具");
        m.insert("project.not_set", "未设置");
        m.insert("project.tool_config", "工具配置");
        m.insert("project.model", "模型");
        m.insert("project.env_vars", "环境变量");
        m.insert("project.tool_not_found_warn", "工具 '{}' 未在注册表中找到，但仍会保存配置");
        m.insert("project.default_set", "已设置项目默认工具");
        m.insert("project.config_saved", "配置已保存到 .vcm/config.toml");
        m.insert("project.vcm_dir", ".vcm 目录");
        m.insert("project.config_file", "配置文件");
        
        // Fallback 命令
        m.insert("fallback.title", "智能降级配置");
        m.insert("fallback.status", "状态");
        m.insert("fallback.enabled", "已启用");
        m.insert("fallback.disabled", "未启用");
        m.insert("fallback.default_chain", "默认降级链");
        m.insert("fallback.custom_chains", "自定义降级链");
        m.insert("fallback.enable_hint", "提示: 使用 'vcm fallback --enable' 启用智能降级");
        m.insert("fallback.add_hint", "      使用 'vcm fallback add <primary> <fallback1> [fallback2]...' 添加降级链");
        m.insert("fallback.primary", "主力");
        m.insert("fallback.backup", "备选");
        m.insert("fallback.fallback", "兜底");
        m.insert("fallback.primary_not_found", "主力工具 '{}' 未找到，但仍会保存配置");
        m.insert("fallback.chain_added", "已添加降级链");
        m.insert("fallback.order", "降级顺序");
        m.insert("fallback.not_enabled", "智能降级当前未启用");
        m.insert("fallback.use_enable", "使用 'vcm fallback --enable' 启用");
        m.insert("fallback.chain_removed", "已移除 '{}' 的降级链");
        m.insert("fallback.chain_not_found", "未找到 '{}' 的降级链");
        m.insert("fallback.enabled_msg", "智能降级已启用");
        m.insert("fallback.auto_switch", "当主力工具不可用时，系统将自动切换到备选工具");
        m.insert("fallback.disabled_msg", "智能降级已禁用");
        m.insert("fallback.default_set", "已设置默认降级链");
        m.insert("fallback.need_one_backup", "至少需要指定一个备选工具");
        m.insert("fallback.need_two_tools", "默认降级链至少需要 2 个工具");
        
        // Key 命令
        m.insert("key.title", "多账号管理");
        m.insert("key.no_config", "暂无保存的 Key 配置");
        m.insert("key.add_hint", "提示: 使用 'vcm key add <tool> <name> <key>' 添加账号");
        m.insert("key.not_found", "未找到 '{}' 的 Key 配置");
        m.insert("key.active", "当前激活");
        m.insert("key.rotation_mode", "轮换模式");
        m.insert("key.add_cmd", "命令: vcm key add <tool> <name> <key>   添加账号");
        m.insert("key.switch_cmd", "      vcm key switch <tool> <name>       切换账号");
        m.insert("key.remove_cmd", "      vcm key remove <tool> <name>       删除账号");
        m.insert("key.rotate_cmd", "      vcm key rotate <tool> --enable     启用轮换");
        m.insert("key.name_empty", "Key 名称不能为空");
        m.insert("key.value_empty", "Key 值不能为空");
        m.insert("key.per_request", "每次请求");
        m.insert("key.hourly", "每小时");
        m.insert("key.daily", "每天");
        m.insert("key.weekly", "每周");
        m.insert("key.exists", "Key '{}' 已存在，将被覆盖");
        m.insert("key.added", "已添加 Key '{}' 到 {}");
        m.insert("key.set_active", "已设为当前激活账号");
        m.insert("key.switched", "已切换到账号");
        m.insert("key.restart_hint", "重启工具后生效");
        m.insert("key.removed", "已删除 Key");
        m.insert("key.not_found_name", "未找到 Key");
        m.insert("key.rotate_need_two", "启用轮换需要至少 2 个 Key");
        m.insert("key.current_count", "当前只有 {} 个 Key");
        m.insert("key.rotation_enabled", "已启用 Key 轮换");
        m.insert("key.rotation_desc", "每次请求将使用不同的 Key");
        m.insert("key.rotation_disabled", "已禁用 Key 轮换");
        m.insert("key.current_active", "当前激活");
        m.insert("key.type_trial", "试用");
        m.insert("key.expires", "过期");
        m.insert("key.no_active", "未设置激活账号");
        m.insert("key.no_saved", "没有保存的 Key");
        m.insert("key.name", "名称");
        m.insert("key.status", "状态");
        m.insert("key.type", "类型");
        m.insert("key.note", "备注");
        m.insert("key.official", "正式");
        
        // Recommend 命令
        m.insert("recommend.title", "个性化推荐");
        m.insert("recommend.based_usage", "基于您的使用习惯和需求推荐");
        m.insert("recommend.install_free", "推荐安装 (免费专业级模型)");
        m.insert("recommend.hot", "热门推荐");
        m.insert("recommend.installed_overview", "已安装概览");
        m.insert("recommend.installed_count", "已安装: {} 个工具");
        m.insert("recommend.configured_count", "已配置: {c} / {t}");
        m.insert("recommend.some_unconfigured", "部分工具未配置，运行 'vcm status' 查看详情");
        m.insert("recommend.trending_title", "热门工具排行");
        m.insert("recommend.rank", "排名");
        m.insert("recommend.tool_col", "工具");
        m.insert("recommend.vendor_col", "供应商");
        m.insert("recommend.free_quota_col", "免费额度");
        m.insert("recommend.pro_free", "专业级免费");
        m.insert("recommend.paid", "付费");
        m.insert("recommend.unknown", "未知");
        m.insert("recommend.install_hint", "使用 'vcm install <tool>' 安装");
        m.insert("recommend.new_title", "新上架工具");
        m.insert("recommend.free_pro_models", "免费专业级模型");
        m.insert("recommend.by_tag", "按标签筛选");
        m.insert("recommend.no_tag_tools", "未找到标签为 '{}' 的工具");
        m.insert("recommend.available_tags", "可用标签: ai, coding, cli, llm, google, anthropic, opensource");
        m.insert("recommend.found_tools", "找到 {} 个工具");
        m.insert("recommend.has_free_quota", "有免费额度");
        m.insert("recommend.install", "安装");
        
        // Free --aggregate
        m.insert("free.aggregate_title", "免费额度聚合面板");
        m.insert("free.no_free_tools", "未找到有免费额度的工具");
        m.insert("free.aggregate_stats", "聚合统计");
        m.insert("free.tools_with_free", "有免费额度的工具");
        m.insert("free.tools_with_pro", "提供专业级模型的工具");
        m.insert("free.free_pro_models", "可免费使用的专业级模型");
        m.insert("free.optimal_strategy", "最优免费组合策略");
        m.insert("free.recommended_combo", "推荐组合");
        m.insert("free.compare_hint", "提示: 使用 'vcm compare <tool1> <tool2>...' 对比多个工具");
        m.insert("free.pro_only_hint", "      使用 'vcm free --pro' 只显示专业级模型");
        
        m
    })
}

/// 初始化翻译
pub fn init() {
    // 预加载翻译
    let _ = get_en_translations();
    let _ = get_zh_translations();
}