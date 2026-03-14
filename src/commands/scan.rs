//! scan 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::{style, Emoji};

static SEARCH: Emoji<'_, '_> = Emoji("🔍 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✓ ", "[+] ");

/// scan 命令
pub struct ScanCommand {
    detailed: bool,
    json: bool,
}

impl ScanCommand {
    pub fn new(detailed: bool, json: bool) -> Self {
        Self { detailed, json }
    }

    pub fn execute(&self) -> Result<()> {
        // 加载注册表
        let registry = Registry::load()?;
        let discovery = Discovery::new(registry);

        // 显示扫描提示
        if !self.json {
            println!("{} {}\n", SEARCH, translate("scan.title"));
        }

        // 扫描
        let installed = discovery.scan();

        // 输出结果
        if self.json {
            self.output_json(&installed)?;
        } else {
            self.output_human(&installed)?;
        }

        Ok(())
    }

    fn output_json(&self, tools: &[InstalledTool]) -> Result<()> {
        let output = serde_json::to_string_pretty(tools)?;
        println!("{}", output);
        Ok(())
    }

    fn output_human(&self, tools: &[InstalledTool]) -> Result<()> {
        if tools.is_empty() {
            println!("{}", translate("scan.none"));
            return Ok(());
        }

        for tool in tools {
            self.print_tool(tool);
        }

        println!();
        println!("{}", translate("scan.found")
            .replace("{}", &format!("{}", style(tools.len()).cyan().bold())));

        if self.detailed {
            self.print_summary(tools);
        }

        Ok(())
    }

    fn print_tool(&self, tool: &InstalledTool) {
        let status_icon = if tool.is_configured {
            CHECK.to_string()
        } else if tool.config_exists {
            "⚠ ".to_string()
        } else {
            CHECK.to_string()
        };

        let status_style = if tool.is_configured {
            style(&status_icon).green()
        } else if tool.config_exists {
            style(&status_icon).yellow()
        } else {
            style(&status_icon).green()
        };

        let version = tool.version.as_deref().unwrap_or(&translate("msg.unknown_version"));
        let version_str = style(format!("v{}", version)).dim();

        print!("{}{} ", status_style, style(&tool.tool_name).bold());

        if self.detailed {
            println!("{} {:<15}", version_str, style(&tool.tool_id).dim());
            println!("    {}: {}", translate("label.path"), tool.path);
            if let Some(ref method) = tool.install_method {
                println!("    {}: {}", translate("label.install_method"), method);
            }
            if !tool.missing_env_vars.is_empty() {
                println!("    {}: {}",
                    translate("label.missing_config"),
                    style(tool.missing_env_vars.join(", ")).yellow());
            }
        } else {
            println!("{}", version_str);
        }
    }

    fn print_summary(&self, tools: &[InstalledTool]) {
        println!("\n{}:", style(translate("label.config_summary")).bold());

        let configured = tools.iter().filter(|t| t.is_configured).count();
        let missing_config = tools.len() - configured;

        println!("  {}: {}/{}", translate("label.configured"), style(configured).green(), tools.len());

        if missing_config > 0 {
            println!("  {}: {}", translate("label.needs_config"), style(missing_config).yellow());
        }
    }
}