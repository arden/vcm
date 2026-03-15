//! 智能降级切换命令

use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 降级配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FallbackConfig {
    /// 是否启用智能降级
    #[serde(default)]
    pub enabled: bool,
    /// 降级链配置 (primary -> [fallbacks])
    #[serde(default)]
    pub chains: HashMap<String, FallbackChain>,
    /// 默认降级链
    #[serde(default)]
    pub default_chain: Option<Vec<String>>,
}

/// 降级链
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackChain {
    /// 主力工具
    pub primary: String,
    /// 备选工具列表
    pub fallbacks: Vec<String>,
    /// 触发条件
    #[serde(default)]
    pub triggers: Vec<FallbackTrigger>,
}

/// 降级触发条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackTrigger {
    /// 配额用尽
    QuotaExhausted,
    /// 响应超时
    Timeout,
    /// API 错误
    ApiError,
    /// 成本超限
    CostExceeded,
}

/// 降级命令
pub struct FallbackCommand {
    action: FallbackAction,
}

/// 降级操作类型
pub enum FallbackAction {
    /// 显示降级配置
    Status,
    /// 设置降级链
    Add { primary: String, fallbacks: Vec<String> },
    /// 移除降级链
    Remove { primary: String },
    /// 启用/禁用
    Toggle { enabled: bool },
    /// 设置默认链
    SetDefault { tools: Vec<String> },
}

impl FallbackCommand {
    pub fn new(action: FallbackAction) -> Self {
        Self { action }
    }

    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;

        match &self.action {
            FallbackAction::Status => {
                self.show_status(&config_manager)?;
            }
            FallbackAction::Add { primary, fallbacks } => {
                self.add_chain(&config_manager, primary, fallbacks)?;
            }
            FallbackAction::Remove { primary } => {
                self.remove_chain(&config_manager, primary)?;
            }
            FallbackAction::Toggle { enabled } => {
                self.toggle(&config_manager, *enabled)?;
            }
            FallbackAction::SetDefault { tools } => {
                self.set_default(&config_manager, tools)?;
            }
        }

        Ok(())
    }

    /// 加载降级配置
    fn load_fallback_config(&self, config_manager: &ConfigManager) -> Result<FallbackConfig> {
        let fallback_path = config_manager.config_dir().join("fallback.json");

        if !fallback_path.exists() {
            return Ok(FallbackConfig::default());
        }

        let content = std::fs::read_to_string(&fallback_path)?;
        let config: FallbackConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 保存降级配置
    fn save_fallback_config(&self, config_manager: &ConfigManager, config: &FallbackConfig) -> Result<()> {
        let fallback_path = config_manager.config_dir().join("fallback.json");
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(&fallback_path, content)?;
        Ok(())
    }

    /// 显示降级配置状态
    fn show_status(&self, config_manager: &ConfigManager) -> Result<()> {
        let config = self.load_fallback_config(config_manager)?;
        let registry = Registry::load()?;

        println!("\n{}", style(format!("🔄 {}", translate("fallback.title"))).cyan().bold());
        println!("{}", "═".repeat(70));

        // 全局状态
        let status = if config.enabled {
            style(format!("✓ {}", translate("fallback.enabled"))).green()
        } else {
            style(format!("✗ {}", translate("fallback.disabled"))).dim()
        };
        println!("\n  {}: {}", translate("fallback.status"), status);

        // 默认降级链
        if let Some(ref chain) = config.default_chain {
            println!("\n  {}:", style(translate("fallback.default_chain")).bold());
            self.print_chain(chain, &registry, "    ");
        }

        // 自定义降级链
        if !config.chains.is_empty() {
            println!("\n  {}:", style(translate("fallback.custom_chains")).bold());

            for (primary_id, chain) in &config.chains {
                let primary_name = registry.find_by_id(primary_id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(primary_id);
                println!("\n    {} →", style(primary_name).cyan());

                let mut full_chain = vec![primary_id.clone()];
                full_chain.extend(chain.fallbacks.clone());
                self.print_chain(&full_chain, &registry, "      ");
            }
        }

        println!("{}", "═".repeat(70));

        if !config.enabled {
            println!("\n{}", translate("fallback.enable_hint"));
        }
        println!("{}", translate("fallback.add_hint"));

        Ok(())
    }

    /// 打印降级链
    fn print_chain(&self, tools: &[String], registry: &Registry, indent: &str) {
        for (i, tool_id) in tools.iter().enumerate() {
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);

            let icon = if i == 0 {
                style(translate("fallback.primary")).green()
            } else if i == tools.len() - 1 {
                style(translate("fallback.fallback")).yellow()
            } else {
                style(translate("fallback.backup")).cyan()
            };

            println!("{}{} {} ({})", indent, icon, style(tool_name).bold(), tool_id);

            if i < tools.len() - 1 {
                println!("{}    ↓", indent);
            }
        }
    }

    /// 添加降级链
    fn add_chain(&self, config_manager: &ConfigManager, primary: &str, fallbacks: &[String]) -> Result<()> {
        if fallbacks.is_empty() {
            bail!("{}", translate("fallback.need_one_backup"));
        }

        let registry = Registry::load()?;

        // 验证工具
        let primary_tool = registry.find_by_id(primary)
            .or_else(|| registry.find_by_name(primary).first().copied());

        if primary_tool.is_none() {
            println!("{} {}", style("⚠️").yellow(), translate("fallback.primary_not_found").replace("{}", primary));
        }

        let mut config = self.load_fallback_config(config_manager)?;

        let chain = FallbackChain {
            primary: primary.to_string(),
            fallbacks: fallbacks.to_vec(),
            triggers: vec![
                FallbackTrigger::QuotaExhausted,
                FallbackTrigger::Timeout,
                FallbackTrigger::ApiError,
            ],
        };

        config.chains.insert(primary.to_string(), chain);
        self.save_fallback_config(config_manager, &config)?;

        let primary_name = primary_tool.map(|t| t.name.as_str()).unwrap_or(primary);
        println!("{} {}: {}", style("✓").green(), translate("fallback.chain_added"), style(primary_name).cyan());

        println!("\n  {}:", translate("fallback.order"));
        println!("    {} →", primary_name);
        for fallback in fallbacks {
            let fallback_name = registry.find_by_id(fallback)
                .map(|t| t.name.as_str())
                .unwrap_or(fallback);
            println!("    {} ({})", style(fallback_name).cyan(), fallback);
        }

        if !config.enabled {
            println!("\n{} {}", style("⚠️").yellow(), translate("fallback.not_enabled"));
            println!("  {}", translate("fallback.use_enable"));
        }

        Ok(())
    }

    /// 移除降级链
    fn remove_chain(&self, config_manager: &ConfigManager, primary: &str) -> Result<()> {
        let mut config = self.load_fallback_config(config_manager)?;

        if config.chains.remove(primary).is_some() {
            self.save_fallback_config(config_manager, &config)?;
            println!("{} {}", style("✓").green(), translate("fallback.chain_removed").replace("{}", primary));
        } else {
            println!("{}", translate("fallback.chain_not_found").replace("{}", primary));
        }

        Ok(())
    }

    /// 启用/禁用
    fn toggle(&self, config_manager: &ConfigManager, enabled: bool) -> Result<()> {
        let mut config = self.load_fallback_config(config_manager)?;
        config.enabled = enabled;
        self.save_fallback_config(config_manager, &config)?;

        if enabled {
            println!("{} {}", style("✓").green(), translate("fallback.enabled_msg"));
            println!("\n{}", translate("fallback.auto_switch"));
        } else {
            println!("{} {}", style("✓").yellow(), translate("fallback.disabled_msg"));
        }

        Ok(())
    }

    /// 设置默认链
    fn set_default(&self, config_manager: &ConfigManager, tools: &[String]) -> Result<()> {
        if tools.len() < 2 {
            bail!("{}", translate("fallback.need_two_tools"));
        }

        let mut config = self.load_fallback_config(config_manager)?;
        config.default_chain = Some(tools.to_vec());
        self.save_fallback_config(config_manager, &config)?;

        println!("{} {}", style("✓").green(), translate("fallback.default_set"));
        println!("\n  {}:", translate("fallback.order"));
        for (i, tool) in tools.iter().enumerate() {
            let label = if i == 0 { 
                translate("fallback.primary") 
            } else if i == tools.len() - 1 { 
                translate("fallback.fallback") 
            } else { 
                translate("fallback.backup") 
            };
            println!("    {} {} ({})", label, style(tool).cyan(), tool);
            if i < tools.len() - 1 {
                println!("        ↓");
            }
        }

        Ok(())
    }
}

/// 获取工具的降级备选（供其他命令调用）
pub fn get_fallback_tool(primary_id: &str) -> Option<String> {
    let config_manager = ConfigManager::new().ok()?;
    let fallback_path = config_manager.config_dir().join("fallback.json");

    if !fallback_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&fallback_path).ok()?;
    let config: FallbackConfig = serde_json::from_str(&content).ok()?;

    if !config.enabled {
        return None;
    }

    // 先检查自定义降级链
    if let Some(chain) = config.chains.get(primary_id) {
        return chain.fallbacks.first().cloned();
    }

    // 再检查默认降级链
    if let Some(ref default_chain) = config.default_chain {
        // 找到 primary 在链中的位置，返回下一个
        for (i, tool_id) in default_chain.iter().enumerate() {
            if tool_id == primary_id && i + 1 < default_chain.len() {
                return Some(default_chain[i + 1].clone());
            }
        }
    }

    None
}