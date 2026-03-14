//! import 命令实现 - 从文件导入工具列表

use crate::commands::export::{ExportData, ExportedTool};
use crate::core::Registry;
use crate::models::*;
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;
use std::fs;

/// import 命令
pub struct ImportCommand {
    input: String,
    install: bool,
}

impl ImportCommand {
    pub fn new(input: String, install: bool) -> Self {
        Self { input, install }
    }

    pub fn execute(&self) -> Result<()> {
        println!("{} {}\n", style("📥").dim(), translate("import.importing"));

        let path = std::path::Path::new(&self.input);
        if !path.exists() {
            bail!("{}: {}", translate("msg.error"), self.input);
        }

        let content = fs::read_to_string(path)?;
        let export_data: ExportData = serde_json::from_str(&content)?;

        println!("{}:", translate("import.file_info"));
        println!("  {}: {}", translate("label.version"), export_data.version);
        println!("  {}: {}", translate("import.exported_at").split(':').next().unwrap_or("Exported at"), export_data.exported_at);
        if let Some(ref hostname) = export_data.hostname {
            println!("  {}: {}", translate("import.source_host"), hostname);
        }
        println!("  {}: {}", translate("import.tool_count"), export_data.tools.len());
        println!();

        // 检查工具安装状态
        let registry = Registry::load()?;
        let mut to_install = Vec::new();
        let mut already_installed = Vec::new();

        for tool in &export_data.tools {
            let installed = registry.find_by_id(&tool.id)
                .map(|t| t.is_installed())
                .unwrap_or(false);

            if installed {
                already_installed.push(tool);
            } else {
                to_install.push(tool);
            }
        }

        // 显示已安装工具
        if !already_installed.is_empty() {
            println!("{} {} ({}):", style("✓").green(), translate("msg.installed"), already_installed.len());
            for tool in &already_installed {
                println!("  {} {} ({})", style("✓").green(), tool.name, tool.id);
            }
            println!();
        }

        // 显示待安装工具
        if !to_install.is_empty() {
            println!("{} {}", style("○").yellow(), translate("import.to_install").replace("{}", &to_install.len().to_string()));
            for tool in &to_install {
                println!("  {} {} ({})", style("○").yellow(), tool.name, tool.id);
            }
            println!();

            if self.install {
                println!("{}...", translate("import.start_install"));
                for tool in &to_install {
                    if let Some(tool_def) = registry.find_by_id(&tool.id) {
                        println!("\n{} {}", style("📦").dim(), translate("install.installing").replace("{}", &tool_def.name));
                        // 这里可以调用安装逻辑
                        println!("  {}: vcm install {}", translate("run.launching").split("...").next().unwrap_or("Run"), tool.id);
                    }
                }
            } else {
                println!("{}", translate("import.hint").replace("{}", &style("--install").cyan().to_string()));
            }
        } else {
            println!("{} {}", style("✓").green(), translate("import.all_installed"));
        }

        Ok(())
    }
}
