//! import 命令实现 - 从文件导入工具列表

use crate::commands::export::{ExportData, ExportedTool};
use crate::core::Registry;
use crate::models::*;
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
        println!("{} 从文件导入工具列表...\n", style("📥").dim());

        let path = std::path::Path::new(&self.input);
        if !path.exists() {
            bail!("文件不存在: {}", self.input);
        }

        let content = fs::read_to_string(path)?;
        let export_data: ExportData = serde_json::from_str(&content)?;

        println!("导入文件信息:");
        println!("  版本: {}", export_data.version);
        println!("  导出时间: {}", export_data.exported_at);
        if let Some(ref hostname) = export_data.hostname {
            println!("  来源主机: {}", hostname);
        }
        println!("  工具数量: {}", export_data.tools.len());
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
            println!("{} 已安装 ({} 个):", style("✓").green(), already_installed.len());
            for tool in &already_installed {
                println!("  {} {} ({})", style("✓").green(), tool.name, tool.id);
            }
            println!();
        }

        // 显示待安装工具
        if !to_install.is_empty() {
            println!("{} 待安装 ({} 个):", style("○").yellow(), to_install.len());
            for tool in &to_install {
                println!("  {} {} ({})", style("○").yellow(), tool.name, tool.id);
            }
            println!();

            if self.install {
                println!("开始安装缺失的工具...");
                for tool in &to_install {
                    if let Some(tool_def) = registry.find_by_id(&tool.id) {
                        println!("\n{} 安装 {}...", style("📦").dim(), tool_def.name);
                        // 这里可以调用安装逻辑
                        println!("  运行: vcm install {}", tool.id);
                    }
                }
            } else {
                println!("提示: 使用 {} 自动安装缺失的工具",
                    style("--install").cyan()
                );
            }
        } else {
            println!("{} 所有工具都已安装", style("✓").green());
        }

        Ok(())
    }
}
