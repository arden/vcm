//! init 命令实现 - 交互式初始化向导

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
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
        println!("\n{}", style(translate("init.step1")).cyan().bold());
        let registry = Registry::load()?;
        let discovery = Discovery::new(Registry::load()?);
        let installed = discovery.scan();

        if installed.is_empty() {
            println!("{} {}", style("○").yellow(), translate("scan.none"));
        } else {
            println!("{} {}",
                style("✓").green(),
                translate("init.found_installed").replace("{}", &style(installed.len()).cyan().to_string())
            );
            for tool in &installed {
                println!("  {} {} ({})", style("✓").green(), tool.tool_name, tool.tool_id);
            }
        }

        // 步骤2: 选择要安装的工具
        println!("\n{}", style(translate("init.step2")).cyan().bold());
        
        let not_installed: Vec<&Tool> = registry.tools.iter()
            .filter(|t| !t.is_installed())
            .collect();

        if not_installed.is_empty() {
            println!("{} {}", style("✓").green(), translate("init.all_installed"));
        } else {
            let items: Vec<String> = not_installed.iter()
                .map(|t| format!("{} - {}", t.name, t.description.lines().next().unwrap_or("")))
                .collect();

            let selections = MultiSelect::new()
                .with_prompt(&translate("init.select_tools"))
                .items(&items)
                .interact()?;

            if !selections.is_empty() {
                println!("\n{}:", translate("init.will_install"));
                for idx in &selections {
                    let tool = not_installed[*idx];
                    println!("  {} {}", style("•").dim(), tool.name);
                }

                // 步骤3: 配置 API Key
                println!("\n{}", style(translate("init.step3")).cyan().bold());
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
        println!("║   {}                                                    ║", translate("app.description"));
        println!("║                                                           ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("{}", translate("init.welcome"));
        println!("{}", translate("init.welcome_desc"));
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
            println!("{} {}", style("○").dim(), translate("init.no_api_key"));
            return Ok(());
        }

        println!("{}:", translate("init.need_api_key"));
        for (_, desc) in &api_keys {
            println!("  {} {}", style("•").dim(), desc);
        }

        let options = vec![translate("init.yes_configure"), translate("init.later")];
        let configure = Select::new()
            .with_prompt(&translate("init.configure_now"))
            .items(&options)
            .interact()?;

        if configure == 0 {
            for (key_name, _desc) in &api_keys {
                let key: String = Input::new()
                    .with_prompt(&translate("init.input").replace("{}", key_name))
                    .interact()?;
                
                if !key.is_empty() {
                    // 显示配置命令
                    let masked = if key.len() > 8 {
                        format!("{}...", &key[..8])
                    } else {
                        format!("{}...", key)
                    };
                    println!("{} {}", style(translate("label.hint")).yellow(), translate("init.add_to_shell"));
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
        println!("║   {} {}                                  ║", style("✓").green().bold(), translate("init.complete"));
        println!("║                                                           ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("{}:", translate("init.next_steps"));
        println!("  {} {}", style("•").dim(), translate("init.install_tools").replace("{}", &style("vcm install <tool>").cyan().to_string()));
        println!("  {} {}", style("•").dim(), translate("init.view_status").replace("{}", &style("vcm status").cyan().to_string()));
        println!("  {} {}", style("•").dim(), translate("init.configure_tools").replace("{}", &style("vcm config <tool>").cyan().to_string()));
        println!("  {} {}", style("•").dim(), translate("init.check_updates").replace("{}", &style("vcm outdated").cyan().to_string()));
        println!();
    }
}