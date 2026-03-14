//! default 命令实现 - 设置默认工具

use crate::core::{ConfigManager, Discovery, Registry};
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
                    println!("当前默认工具: {}", style(default_tool).cyan().bold());
                    
                    // 显示工具信息
                    let registry = Registry::load()?;
                    if let Some(tool) = registry.find_by_id(default_tool) {
                        println!("  名称: {}", tool.name);
                        println!("  供应商: {}", tool.vendor);
                        println!("  状态: {}", if tool.is_installed() {
                            style("已安装").green()
                        } else {
                            style("未安装").yellow()
                        });
                        
                        // 启动命令提示
                        println!("\n启动命令: {} 或 {}", 
                            style(&format!("vcm run {}", default_tool)).cyan(),
                            style(&format!("vcm run {}", tool.name)).cyan()
                        );
                    }
                } else {
                    println!("未设置默认工具");
                    println!("\n设置默认工具: {}", style("vcm default <tool>").cyan());
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
                    None => bail!("未找到工具: {}", tool_id),
                };

                // 检查是否是 CLI 工具
                if !tool_def.is_cli {
                    bail!("{} 不是 CLI 工具，无法设置为默认", tool_def.name);
                }

                // 更新配置
                let mut config = config_manager.load_config()?;
                config.settings.default_tool = Some(tool_def.id.clone());
                config_manager.save_config(&config)?;

                println!("{} 已设置默认工具: {}", style("✓").green(), style(&tool_def.name).cyan().bold());
                
                if !tool_def.is_installed() {
                    println!("\n{} 工具尚未安装", style("⚠").yellow());
                    println!("运行 {} 安装", style(&format!("vcm install {}", tool_def.id)).cyan());
                }

                println!("\n启动命令:");
                println!("  {} - 直接启动", style("vcm run").cyan());
                println!("  {} {} - 指定工具启动", style("vcm run").cyan(), tool_def.id);
            }
        }

        Ok(())
    }
}
