//! config 命令实现

use crate::core::{ConfigManager, Registry};
use crate::models::*;
use crate::i18n::translate;
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
            None => bail!("{}", translate("tool.not_found").replace("{}", &tool_id)),
        };

        self.configure_tool(tool)
    }

    fn set_api_key(&self, key_spec: &str) -> Result<()> {
        let parts: Vec<&str> = key_spec.splitn(2, '=').collect();
        if parts.len() != 2 {
            bail!("{}", translate("config.format_error"));
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

        println!("{} {}", style("✓").green(), translate("config.env_var_set").replace("{}", var_name));
        println!("{}", translate("config.apply_changes").replace("{}", &style("source ~/.bashrc").cyan().to_string()));

        Ok(())
    }

    fn show_config_overview(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;
        let config = config_manager.load_config()?;

        println!("{} {}\n", style("⚙️").dim(), translate("config.overview"));

        // API Keys 状态
        println!("{}", style(translate("config.api_key_status")).bold());

        let env_vars = [
            ("ANTHROPIC_API_KEY", "Anthropic"),
            ("OPENAI_API_KEY", "OpenAI"),
            ("GOOGLE_API_KEY", "Google"),
            ("GITHUB_TOKEN", "GitHub"),
        ];

        for (var_name, provider) in &env_vars {
            let status = if std::env::var(var_name).is_ok() {
                style(format!("✓ {}", translate("msg.configured"))).green()
            } else {
                style(format!("○ {}", translate("msg.not_configured"))).dim()
            };
            println!("  {:<20} {} {}", provider, status, style(var_name).dim());
        }

        // 默认工具
        if let Some(ref default) = config.settings.default_tool {
            println!("\n{}: {}", style(translate("config.default_tool")).bold(), default);
        }

        println!("\n{} {}", translate("hint.install").split('`').next().unwrap_or("Use"), style("vcm config <tool>").cyan());

        Ok(())
    }

    fn configure_tool(&self, tool: &Tool) -> Result<()> {
        println!("{} {}\n", style("🔑").dim(), translate("config.title").replace("{}", &style(&tool.name).cyan().bold().to_string()));

        // 检查当前状态
        let env_status: Vec<(String, bool)> = tool.env_vars.iter()
            .map(|e| (e.name.clone(), std::env::var(&e.name).is_ok()))
            .collect();

        let all_configured = env_status.iter().all(|(_, configured)| *configured);

        if all_configured {
            println!("{} {}", style("✓").green(), translate("config.all_configured"));

            let options = vec![translate("config.update_key"), translate("config.view_files"), translate("config.back")];
            let selection = Select::new()
                .with_prompt(translate("config.select_action"))
                .items(&options)
                .interact()?;

            match selection {
                0 => self.prompt_for_keys(tool),
                1 => self.show_config_files(tool),
                _ => Ok(()),
            }
        } else {
            println!("{}:", translate("config.need_config"));
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
                    println!("\n{}: {}", translate("install.get_api_key").replace("{}", ""), style(url).cyan());
                }

                let key: String = Input::new()
                    .with_prompt(translate("config.input_prompt").replace("{}", &env_var.name))
                    .interact()?;

                // 写入到 shell 配置
                let shell_config = self.get_shell_config_path()?;
                let export_line = format!("\nexport {}=\"{}\"", env_var.name, key);

                std::fs::OpenOptions::new()
                    .append(true)
                    .open(&shell_config)?
                    .write_all(export_line.as_bytes())?;

                println!("{} {}", style("✓").green(), translate("config.env_var_set").replace("{}", &env_var.name));
            }
        }

        println!("\n{}", translate("config.apply_changes").replace("{}", &style("source ~/.bashrc").cyan().to_string()));

        Ok(())
    }

    fn show_config_files(&self, tool: &Tool) -> Result<()> {
        println!("\n{}:", translate("config.config_files").replace("{}", &tool.name));

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
