//! export 命令实现 - 导出已安装工具列表

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::Path;

/// 导出数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportData {
    /// 导出版本
    pub version: u32,
    /// 导出时间
    pub exported_at: String,
    /// 主机名
    pub hostname: Option<String>,
    /// 已安装工具列表
    pub tools: Vec<ExportedTool>,
}

/// 导出的工具信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportedTool {
    /// 工具ID
    pub id: String,
    /// 工具名称
    pub name: String,
    /// 版本
    pub version: Option<String>,
    /// 是否已配置
    pub configured: bool,
}

/// export 命令
pub struct ExportCommand {
    output: String,
}

impl ExportCommand {
    pub fn new(output: String) -> Self {
        Self { output }
    }

    pub fn execute(&self) -> Result<()> {
        println!("{} {}\n", style("📦").dim(), translate("export.exporting"));

        let registry = Registry::load()?;
        let discovery = Discovery::new(registry);
        let installed = discovery.scan();

        if installed.is_empty() {
            println!("{}", translate("scan.none"));
            return Ok(());
        }

        let hostname = hostname::get()
            .ok()
            .map(|h| h.to_string_lossy().to_string());

        let export_data = ExportData {
            version: 1,
            exported_at: chrono::Local::now().to_rfc3339(),
            hostname,
            tools: installed.iter().map(|t| ExportedTool {
                id: t.tool_id.clone(),
                name: t.tool_name.clone(),
                version: t.version.clone(),
                configured: t.is_configured,
            }).collect(),
        };

        let json = serde_json::to_string_pretty(&export_data)?;
        fs::write(&self.output, &json)?;

        println!("{}",
            translate("export.exported")
                .replace("{}", &style(installed.len()).cyan().bold().to_string())
                .replace("{path}", &style(&self.output).yellow().to_string())
        );

        println!("\n{}:", translate("export.tool_list"));
        for tool in &export_data.tools {
            let status = if tool.configured {
                style("✓").green()
            } else {
                style("○").yellow()
            };
            println!("  {} {} ({})", status, tool.name, tool.id);
        }

        Ok(())
    }
}
