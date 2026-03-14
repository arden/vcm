//! init 命令实现 - 交互式初始化向导

use crate::core::{Discovery, Registry};
use crate::models::*;
use anyhow::Result;
use console::style;
use dialoguer::{Input, MultiSelect, Select};

/// init 命令
pub struct InitCommand;

impl InitCommand {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self) -> Result<()> {
        self.print_welcome();

        // 步骤1: 扫描已安装工具
        println!("\n{}", style("步骤 1/3: 扫描已安装工具").cyan().bold());
        let registry = Registry::load()?;
        let discovery = Discovery::new(Registry::load()?);
        let installed = discovery.scan();

        if installed.is_empty() {
            println!("{} 未发现已安装的 CLI AI 工具", style("○").yellow());
        } else {
            println!("{} 发现 {} 个已安装工具", 
                style("✓").green(), 
                style(installed.len()).cyan()
            );
            for tool in &installed {
                println!("  {} {} ({})", style("✓").green(), tool.tool_name, tool.tool_id);
            }
        }

        // 步骤2: 选择要安装的工具
        println!("\n{}", style("步骤 2/3: 选择要安装的工具").cyan().bold());
        
        let not_installed: Vec<&Tool> = registry.tools.iter()
            .filter(|t| !t.is_installed())
            .collect();

        if not_installed.is_empty() {
            println!("{} 所有工具都已安装", style("✓").green());
        } else {
            let items: Vec<String> = not_installed.iter()
                .map(|t| format!("{} - {}", t.name, t.description.lines().next().unwrap_or("")))
                .collect();

            let selections = MultiSelect::new()
                .with_prompt("选择要安装的工具 (空格选择，回车确认)")
                .items(&items)
                .interact()?;

            if !selections.is_empty() {
                println!("\n{} 将安装以下工具:", style("📦").dim());
                for idx in &selections {
                    let tool = not_installed[*idx];
                    println!("  {} {}", style("•").dim(), tool.name);
                }

                // 步骤3: 配置 API Key
                println!("\n{}", style("步骤 3/3: 配置 API Key").cyan().bold());
                self.configure_api_keys(&selections, &not_installed)?;
            }
        }

        // 完成
        self.print_complete();

        Ok(())
    }

    fn print_welcome(&self) {
        println!();
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║                                                           ║");
        println!("║   {} - Vibe Coding Manager                    ║", style("VCM").cyan().bold());
        println!("║                                                           ║");
        println!("║   CLI AI 编程工具管理器                                   ║");
        println!("║                                                           ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("欢迎使用 VCM 初始化向导!");
        println!("这个向导将帮助你设置 CLI AI 编程环境。");
    }

    fn configure_api_keys(&self, selections: &[usize], tools: &[&Tool]) -> Result<()> {
        let mut api_keys: Vec<(&str, &str)> = Vec::new();

        for idx in selections {
            let tool = tools[*idx];
            for env_var in &tool.env_vars {
                if env_var.required && !api_keys.iter().any(|(k, _)| *k == env_var.name) {
                    api_keys.push((&env_var.name, &env_var.description));
                }
            }
        }

        if api_keys.is_empty() {
            println!("{} 选中的工具不需要配置 API Key", style("○").dim());
            return Ok(());
        }

        println!("以下 API Key 需要配置:");
        for (_, desc) in &api_keys {
            println!("  {} {}", style("•").dim(), desc);
        }

        let configure = Select::new()
            .with_prompt("是否现在配置 API Key?")
            .items(&["是，现在配置", "稍后配置"])
            .interact()?;

        if configure == 0 {
            for (key_name, _desc) in &api_keys {
                let key: String = Input::new()
                    .with_prompt(&format!("输入 {}", key_name))
                    .interact()?;
                
                if !key.is_empty() {
                    // 显示配置命令
                    let masked = if key.len() > 8 {
                        format!("{}...", &key[..8])
                    } else {
                        format!("{}...", key)
                    };
                    println!("{} 添加到你的 shell 配置文件:", style("提示:").yellow());
                    println!("  export {}=\"{}\"", key_name, masked);
                }
            }
        }

        Ok(())
    }

    fn print_complete(&self) {
        println!();
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║                                                           ║");
        println!("║   {} 初始化完成!                              ║", style("✓").green().bold());
        println!("║                                                           ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("下一步:");
        println!("  {} 安装工具: {}", style("•").dim(), style("vcm install <tool>").cyan());
        println!("  {} 查看状态: {}", style("•").dim(), style("vcm status").cyan());
        println!("  {} 配置工具: {}", style("•").dim(), style("vcm config <tool>").cyan());
        println!("  {} 检查更新: {}", style("•").dim(), style("vcm outdated").cyan());
        println!();
    }
}