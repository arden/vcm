//! 别名命令

use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use crate::models::VcmConfig;
use anyhow::{bail, Result};
use console::{style, Emoji};

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️ ", "");

/// 别名命令
pub struct AliasCommand {
    /// 别名操作
    action: AliasAction,
}

/// 别名操作类型
pub enum AliasAction {
    /// 设置别名
    Set { alias: String, tool: String },
    /// 列出所有别名
    List,
    /// 删除别名
    Remove { alias: String },
}

impl AliasCommand {
    /// 创建新的别名命令
    pub fn new(action: AliasAction) -> Self {
        Self { action }
    }

    /// 执行命令
    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;
        let mut config = config_manager.load_config()?;

        match &self.action {
            AliasAction::Set { alias, tool } => {
                self.set_alias(&config_manager, &mut config, alias, tool)?;
            }
            AliasAction::List => {
                self.list_aliases(&config)?;
            }
            AliasAction::Remove { alias } => {
                self.remove_alias(&config_manager, &mut config, alias)?;
            }
        }

        Ok(())
    }

    /// 设置别名
    fn set_alias(
        &self,
        config_manager: &ConfigManager,
        config: &mut VcmConfig,
        alias: &str,
        tool: &str,
    ) -> Result<()> {
        // 验证别名格式
        if alias.is_empty() {
            bail!("别名不能为空");
        }

        // 检查别名是否是保留字（防止与命令冲突）
        let reserved_words = [
            "scan", "list", "install", "update", "remove", "config", "status",
            "search", "info", "doctor", "completions", "outdated", "export",
            "import", "init", "usage", "run", "default", "lang", "free",
            "alias", "compare", "quota", "stats", "cost", "project", "fallback",
            "key", "recommend", "trending", "local",
        ];

        if reserved_words.contains(&alias) {
            bail!("'{}' 是保留命令，不能用作别名", alias);
        }

        // 验证工具是否存在
        let registry = Registry::load()?;
        let tool_info = registry.find_by_id(tool)
            .or_else(|| registry.find_by_name(tool).first().copied());

        let tool_name = match tool_info {
            Some(tool) => tool.name.clone(),
            None => {
                println!(
                    "{}{}",
                    WARNING,
                    style(format!("警告: 工具 '{}' 未在注册表中找到，但仍会创建别名", tool))
                        .yellow()
                );
                tool.to_string()
            }
        };

        // 检查别名是否已存在
        if let Some(existing) = config.settings.aliases.get(alias) {
            println!(
                "{}别名 '{}' 已映射到 '{}'，将更新为 '{}'",
                WARNING, alias, existing, tool
            );
        }

        // 设置别名
        config.settings.aliases.insert(alias.to_string(), tool.to_string());
        config_manager.save_config(config)?;

        println!(
            "{}已设置别名: {} -> {} ({})",
            SPARKLE,
            style(alias).cyan().bold(),
            style(&tool_name).green(),
            tool
        );

        Ok(())
    }

    /// 列出所有别名
    fn list_aliases(&self, config: &VcmConfig) -> Result<()> {
        if config.settings.aliases.is_empty() {
            println!("{}", translate("alias.none"));
            return Ok(());
        }

        println!("\n{}", style("工具别名列表").bold());
        println!("{}", "─".repeat(50));

        let registry = Registry::load()?;
        let mut aliases: Vec<_> = config.settings.aliases.iter().collect();
        aliases.sort_by_key(|(k, _)| *k);

        for (alias, tool_id) in aliases {
            let tool_name = registry
                .find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);
            println!(
                "  {} -> {} ({})",
                style(alias).cyan().bold(),
                style(tool_name).green(),
                style(tool_id).dim()
            );
        }

        println!("{}", "─".repeat(50));
        println!("\n提示: 使用 'vcm <alias>' 快速启动工具");
        println!("示例: 'vcm cc' 将启动 claude-code (如果设置了别名 cc)");

        Ok(())
    }

    /// 删除别名
    fn remove_alias(
        &self,
        config_manager: &ConfigManager,
        config: &mut VcmConfig,
        alias: &str,
    ) -> Result<()> {
        if config.settings.aliases.remove(alias).is_some() {
            config_manager.save_config(config)?;
            println!("{}已删除别名: {}", SPARKLE, style(alias).cyan());
        } else {
            bail!("别名 '{}' 不存在", alias);
        }

        Ok(())
    }
}
