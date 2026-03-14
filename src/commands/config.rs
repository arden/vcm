//! config 命令实现

use crate::core::{ConfigManager, Discovery, Registry};
use crate::models::*;
use anyhow::{bail, Result};
use console::style;
use dialoguer::{Input, Select};
use std::io::Write;

/// config 命令
pub struct ConfigCommand {
    tool: Option<String>,
    set_key: Option<String>,
}

impl ConfigCommand {
    pub fn new(tool: Option<String>, set_key: Option<String>) -> Self {
        Self { tool, set_key }
    }

    pub fn execute(&self) -> Result<()> {
        // 如果指定了 set_key
        if let Some(ref key_spec) = self.set_key {
            return self.set_api_key(key_spec);
        }

        // 如果没有指定工具，显示配置概览
        let tool_id = match &self.tool {
            Some(id) => id.clone(),
            None => return self.show_config_overview(),
        };

        let registry = Registry::load()?;
        let tool = registry.find_by_id(&tool_id)
            .or_else(|| registry.find_by_name(&tool_id).first().copied());

        let tool = match tool {
            Some(t) => t,
            None => bail!("未找到工具: {}", tool_id),
        };

        self.configure_tool(tool)
    }

    fn set_api_key(&self, key_spec: &str) -> Result<()> {
        let parts: Vec<&str> = key_spec.splitn(2, '=').collect();
        if parts.len() != 2 {
            bail!("格式错误，请使用: PROVIDER=KEY");
        }

        let var_name = parts[0];
        let key_value = parts[1];

        // 写入到 shell 配置文件
        let shell_config = self.get_shell_config_path()?;

        let export_line = format!("\nexport {}=\"{}\"", var_name, key_value);

        std::fs::OpenOptions::new()
            .append(true)
            .open(&shell_config)?
            .write_all(export_line.as_bytes())?;

        println!("{} 已设置环境变量 {}", style("✓").green(), var_name);
        println!("运行 {} 使配置生效", style("source ~/.bashrc").cyan());

        Ok(())
    }

    fn show_config_overview(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;
        let config = config_manager.load_config()?;

        println!("{} 配置概览\n", style("⚙️").dim());

        // API Keys 状态
        println!("{}", style("API Keys 状态:").bold());

        let env_vars = [
            ("ANTHROPIC_API_KEY", "Anthropic"),
            ("OPENAI_API_KEY", "OpenAI"),
            ("GOOGLE_API_KEY", "Google"),
            ("GITHUB_TOKEN", "GitHub"),
        ];

        for (var_name, provider) in &env_vars {
            let status = if std::env::var(var_name).is_ok() {
                style("✓ 已配置").green()
            } else {
                style("○ 未配置").dim()
            };
            println!("  {:<20} {} {}", provider, status, style(var_name).dim());
        }

        // 默认工具
        if let Some(ref default) = config.settings.default_tool {
            println!("\n{}: {}", style("默认工具").bold(), default);
        }

        println!("\n使用 {} 配置特定工具", style("vcm config <tool>").cyan());

        Ok(())
    }

    fn configure_tool(&self, tool: &Tool) -> Result<()> {
        println!("{} 配置 {}\n", style("🔑").dim(), style(&tool.name).cyan().bold());

        // 检查当前状态
        let env_status: Vec<(String, bool)> = tool.env_vars.iter()
            .map(|e| (e.name.clone(), std::env::var(&e.name).is_ok()))
            .collect();

        let all_configured = env_status.iter().all(|(_, configured)| *configured);

        if all_configured {
            println!("{} 所有环境变量已配置", style("✓").green());

            let options = vec!["更新 API Key", "查看配置文件", "返回"];
            let selection = Select::new()
                .with_prompt("选择操作")
                .items(&options)
                .interact()?;

            match selection {
                0 => self.prompt_for_keys(tool),
                1 => self.show_config_files(tool),
                _ => Ok(()),
            }
        } else {
            println!("以下环境变量需要配置:");
            for env_var in &tool.env_vars {
                let status = if std::env::var(&env_var.name).is_ok() {
                    style("✓").green()
                } else {
                    style("○").yellow()
                };
                println!(
                    "  {} {} - {}",
                    status,
                    style(&env_var.name).bold(),
                    env_var.description
                );
            }

            self.prompt_for_keys(tool)
        }
    }

    fn prompt_for_keys(&self, tool: &Tool) -> Result<()> {
        for env_var in &tool.env_vars {
            if std::env::var(&env_var.name).is_err() {
                if let Some(ref url) = env_var.get_url {
                    println!("\n获取 API Key: {}", style(url).cyan());
                }

                let key: String = Input::new()
                    .with_prompt(format!("请输入 {}", env_var.name))
                    .interact()?;

                // 写入到 shell 配置
                let shell_config = self.get_shell_config_path()?;
                let export_line = format!("\nexport {}=\"{}\"", env_var.name, key);

                std::fs::OpenOptions::new()
                    .append(true)
                    .open(&shell_config)?
                    .write_all(export_line.as_bytes())?;

                println!("{} 已设置 {}", style("✓").green(), env_var.name);
            }
        }

        println!("\n运行 {} 使配置生效", style("source ~/.bashrc").cyan());

        Ok(())
    }

    fn show_config_files(&self, tool: &Tool) -> Result<()> {
        println!("\n{} 配置文件:", style(&tool.name).bold());

        let home = dirs::home_dir().unwrap_or_default();

        for path in &tool.config_paths {
            let expanded = if path.starts_with('~') {
                format!("{}/{}",
                    home.display(),
                    path.strip_prefix('~').unwrap_or(path)
                )
            } else {
                path.clone()
            };

            let exists = std::path::Path::new(&expanded).exists();
            let status = if exists {
                style("✓").green()
            } else {
                style("○").dim()
            };

            println!("  {} {}", status, expanded);
        }

        Ok(())
    }

    fn get_shell_config_path(&self) -> Result<std::path::PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();

        // 检查使用的 shell
        let shell = std::env::var("SHELL").unwrap_or_default();

        if shell.contains("zsh") {
            Ok(home.join(".zshrc"))
        } else if shell.contains("fish") {
            Ok(home.join(".config/fish/config.fish"))
        } else {
            Ok(home.join(".bashrc"))
        }
    }
}
