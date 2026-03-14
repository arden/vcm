//! status 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// status 命令
pub struct StatusCommand {
    json: bool,
}

impl StatusCommand {
    pub fn new(json: bool) -> Self {
        Self { json }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;
        let discovery = Discovery::new(registry);

        let installed = discovery.scan();

        if self.json {
            self.output_json(&installed)?;
        } else {
            self.output_human(&installed)?;
        }

        Ok(())
    }

    fn output_json(&self, tools: &[InstalledTool]) -> Result<()> {
        let statuses: Vec<ToolStatus> = tools.iter()
            .map(|t| {
                let health = if t.is_configured {
                    HealthStatus::Healthy
                } else if !t.missing_env_vars.is_empty() {
                    HealthStatus::Warning
                } else {
                    HealthStatus::Unknown
                };

                let suggestions = if !t.missing_env_vars.is_empty() {
                    vec![format!("vcm config {}", t.tool_id)]
                } else {
                    vec![]
                };

                ToolStatus {
                    tool: t.clone(),
                    health,
                    suggestions,
                }
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&statuses)?);
        Ok(())
    }

    fn output_human(&self, tools: &[InstalledTool]) -> Result<()> {
        println!("{} {}\n", style("📊").dim(), translate("status.title"));

        if tools.is_empty() {
            println!("{}", translate("scan.none"));
            println!("{}", translate("hint.install"));
            return Ok(());
        }

        // 表头
        println!(
            "{:<15} {:<12} {:<10} {}",
            style(translate("label.tool")).bold(),
            style(translate("label.version")).dim(),
            style(translate("label.status")).dim(),
            style(translate("label.note")).dim()
        );
        println!("{}", "─".repeat(60));

        let mut configured = 0;
        let mut needs_config = 0;

        let unknown_version = translate("msg.unknown");
        let status_healthy = translate("status.healthy");
        let status_warning = translate("status.warning");
        let label_missing = translate("label.missing");

        for tool in tools {
            let version = tool.version.as_deref().unwrap_or(&unknown_version);

            if tool.is_configured {
                configured += 1;
                let status_text = format!("✓ {}", status_healthy);
                let note = if !tool.missing_env_vars.is_empty() {
                    format!("{}: {}", label_missing, tool.missing_env_vars.join(", "))
                } else {
                    "".to_string()
                };
                println!(
                    "{:<15} {:<12} {:<10} {}",
                    style(&tool.tool_name).bold(),
                    style(version).dim(),
                    style(status_text).green(),
                    style(note).yellow()
                );
            } else if !tool.missing_env_vars.is_empty() {
                needs_config += 1;
                let status_text = format!("⚠ {}", status_warning);
                let note = format!("{}: {}", label_missing, tool.missing_env_vars.join(", "));
                println!(
                    "{:<15} {:<12} {:<10} {}",
                    style(&tool.tool_name).bold(),
                    style(version).dim(),
                    style(status_text).yellow(),
                    style(note).yellow()
                );
            } else {
                configured += 1;
                let status_text = format!("✓ {}", status_healthy);
                println!(
                    "{:<15} {:<12} {:<10} {}",
                    style(&tool.tool_name).bold(),
                    style(version).dim(),
                    style(status_text).green(),
                    style("").dim()
                );
            }
        }

        // 汇总
        println!("{}", "─".repeat(60));
        let percentage = configured * 100 / tools.len().max(1);
        println!("{}: {}/{} ({}%)",
            translate("status.completion"),
            style(configured).green().bold(),
            tools.len(),
            percentage
        );

        if needs_config > 0 {
            println!("\n{} {}",
                style(&format!("{}:", translate("label.suggestion"))).yellow().bold(),
                translate("hint.config")
            );
        }

        Ok(())
    }
}