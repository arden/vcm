//! default 命令实现 - 设置默认工具

use crate::core::{ConfigManager, Discovery, Registry};
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;

/// default 命令
pub struct DefaultCommand {
    tool: Option<String>,
}

impl DefaultCommand {
    pub fn new(tool: Option<String>) -> Self {
        Self { tool }
    }

    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;

        match &self.tool {
            None => {
                // 显示当前默认工具
                let config = config_manager.load_config()?;
                
                if let Some(default_tool) = &config.settings.default_tool {
                    println!("{}: {}", translate("default.current").split(':').next().unwrap_or("Current"), style(default_tool).cyan().bold());
                    
                    // 显示工具信息
                    let registry = Registry::load()?;
                    if let Some(tool) = registry.find_by_id(default_tool) {
                        println!("  {}: {}", translate("label.name"), tool.name);
                        println!("  {}: {}", translate("label.vendor"), tool.vendor);
                        println!("  {}: {}", translate("label.status"), if tool.is_installed() {
                            style(translate("msg.installed")).green()
                        } else {
                            style(translate("msg.not_installed")).yellow()
                        });
                        
                        // 启动命令提示
                        println!("\n{}:", translate("default.launch_cmd"));
                        println!("  {} {} {}", style("vcm run").cyan(), translate("label.or"), style(&format!("vcm run {}", tool.name)).cyan());
                    }
                } else {
                    println!("{}", translate("default.none"));
                    println!("\n{}", translate("default.set_prompt").replace("{}", &style("vcm default <tool>").cyan().to_string()));
                }
            }
            Some(tool_id) => {
                // 设置默认工具
                let registry = Registry::load()?;
                
                // 查找工具
                let tool_def = registry.find_by_id(tool_id)
                    .or_else(|| registry.find_by_name(tool_id).first().copied());

                let tool_def = match tool_def {
                    Some(t) => t,
                    None => bail!("{}", translate("tool.not_found").replace("{}", tool_id)),
                };

                // 检查是否是 CLI 工具
                if !tool_def.is_cli {
                    bail!("{}", translate("default.not_cli").replace("{}", &tool_def.name));
                }

                // 更新配置
                let mut config = config_manager.load_config()?;
                config.settings.default_tool = Some(tool_def.id.clone());
                config_manager.save_config(&config)?;

                println!("{} {}", style("✓").green(), translate("default.set").replace("{}", &style(&tool_def.name).cyan().bold().to_string()));
                
                if !tool_def.is_installed() {
                    println!("\n{} {}", style("⚠").yellow(), translate("default.tool_not_installed"));
                    println!("{}", translate("default.run_install").replace("{}", &style(&format!("vcm install {}", tool_def.id)).cyan().to_string()));
                }

                println!("\n{}:", translate("default.launch_cmd"));
                println!("  {} - {}", style("vcm run").cyan(), translate("default.direct_launch"));
                println!("  {} {} - {}", style("vcm run").cyan(), tool_def.id, translate("default.specify_tool"));
            }
        }

        Ok(())
    }
}
