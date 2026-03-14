//! import 命令实现 - 从文件导入工具列表

use crate::commands::export::ExportData;
use crate::core::Registry;
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
        println!("  {}: {}", translate("import.exported_at"), export_data.exported_at);
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
                let mut success = 0;
                let mut failed = 0;
                
                println!("{}...", translate("import.start_install"));
                
                for tool in &to_install {
                    if let Some(tool_def) = registry.find_by_id(&tool.id) {
                        println!("\n{} {}", style("📦").dim(), translate("install.installing").replace("{}", &style(&tool_def.name).cyan().bold().to_string()));
                        
                        // 调用 vcm install 执行实际安装
                        let result = self.install_tool(&tool.id);
                        
                        match result {
                            Ok(_) => {
                                println!("{} {}", style("✓").green(), translate("install.success").replace("{}", &tool_def.name));
                                success += 1;
                            }
                            Err(e) => {
                                println!("{} {} - {}", style("✗").red(), translate("install.failed").replace("{}", &tool_def.name), e);
                                failed += 1;
                            }
                        }
                    }
                }
                
                println!("\n{}: {} {}, {} {}",
                    translate("update.complete"),
                    style(success).green(),
                    translate("msg.success").to_lowercase(),
                    style(failed).red(),
                    translate("msg.failed").to_lowercase()
                );
            } else {
                println!("{}", translate("import.hint").replace("{}", &style("--install").cyan().to_string()));
            }
        } else {
            println!("{} {}", style("✓").green(), translate("import.all_installed"));
        }

        Ok(())
    }
    
    /// 执行工具安装
    fn install_tool(&self, tool_id: &str) -> Result<()> {
        // 获取当前可执行文件路径
        let vcm_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "vcm".to_string());
        
        let status = std::process::Command::new(&vcm_path)
            .args(["install", tool_id])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            bail!("{}", translate("install.failed").replace("{}", tool_id))
        }
    }
}
