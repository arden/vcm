//! usage 命令实现 - 显示工具使用统计

use crate::core::{Discovery, Registry};
use crate::models::*;
use anyhow::Result;
use console::style;

/// usage 命令
pub struct UsageCommand;

impl UsageCommand {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self) -> Result<()> {
        println!("{} CLI AI 工具使用统计\n", style("📊").dim());

        let registry = Registry::load()?;
        let discovery = Discovery::new(Registry::load()?);
        let installed = discovery.scan();

        // 统计数据
        let total_tools = registry.len();
        let installed_count = installed.len();
        let configured_count = installed.iter().filter(|t| t.is_configured).count();
        let needs_config_count = installed_count - configured_count;

        // 按供应商分组
        let mut vendors: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for tool in &installed {
            *vendors.entry(tool.tool_id.split('-').next().unwrap_or("other").to_string()).or_default() += 1;
        }

        // 按安装方式分组
        let mut methods: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for tool in &installed {
            let method = tool.install_method.as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            *methods.entry(method).or_default() += 1;
        }

        // 打印统计
        self.print_overview(total_tools, installed_count, configured_count, needs_config_count);
        self.print_vendor_stats(&vendors);
        self.print_method_stats(&methods);
        self.print_installed_list(&installed);
        self.print_recommendations(&registry, &installed);

        Ok(())
    }

    fn print_overview(&self, total: usize, installed: usize, configured: usize, needs_config: usize) {
        println!("{}", style("概览").cyan().bold());
        println!("┌───────────────────────────────────────────────┐");
        println!("│ {:<45} │", format!("注册表工具: {}", style(total).cyan()));
        println!("│ {:<45} │", format!("已安装: {} ({}%)", 
            style(installed).green(), 
            installed * 100 / total.max(1)
        ));
        println!("│ {:<45} │", format!("已配置: {}", style(configured).green()));
        if needs_config > 0 {
            println!("│ {:<45} │", format!("待配置: {}", style(needs_config).yellow()));
        }
        println!("└───────────────────────────────────────────────┘");
        println!();
    }

    fn print_vendor_stats(&self, vendors: &std::collections::HashMap<String, usize>) {
        if vendors.is_empty() {
            return;
        }

        println!("{}", style("按供应商").cyan().bold());
        let mut sorted: Vec<_> = vendors.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (vendor, count) in sorted.iter().take(5) {
            let bar = "█".repeat(**count);
            println!("  {:<12} {} {}", 
                vendor, 
                style(bar).cyan(), 
                style(count).dim()
            );
        }
        println!();
    }

    fn print_method_stats(&self, methods: &std::collections::HashMap<String, usize>) {
        if methods.is_empty() {
            return;
        }

        println!("{}", style("按安装方式").cyan().bold());
        for (method, count) in methods {
            println!("  {} {} 个工具", 
                style(method).yellow(), 
                style(count).cyan()
            );
        }
        println!();
    }

    fn print_installed_list(&self, installed: &[InstalledTool]) {
        if installed.is_empty() {
            return;
        }

        println!("{}", style("已安装工具").cyan().bold());
        println!("{:<15} {:<12} {:<8} {}", 
            style("工具").bold(),
            style("版本").dim(),
            style("配置").dim(),
            style("路径").dim()
        );
        println!("{}", "─".repeat(60));

        for tool in installed {
            let version = tool.version.as_deref().unwrap_or("未知");
            let config = if tool.is_configured {
                style("✓").green()
            } else {
                style("○").yellow()
            };
            
            // 安全截断路径
            let path_chars: Vec<char> = tool.path.chars().collect();
            let path_short = if path_chars.len() > 30 {
                let truncated: String = path_chars[path_chars.len()-27..].iter().collect();
                format!("...{}", truncated)
            } else {
                tool.path.clone()
            };

            println!("{:<15} {:<12} {:<8} {}", 
                style(&tool.tool_name).bold(),
                style(version).dim(),
                config,
                style(&path_short).dim()
            );
        }
        println!();
    }

    fn print_recommendations(&self, registry: &Registry, installed: &[InstalledTool]) {
        let installed_ids: std::collections::HashSet<_> = installed.iter()
            .map(|t| t.tool_id.as_str())
            .collect();

        let featured_not_installed: Vec<_> = registry.tools.iter()
            .filter(|t| t.featured && !installed_ids.contains(t.id.as_str()))
            .take(3)
            .collect();

        if !featured_not_installed.is_empty() {
            println!("{}", style("推荐安装").cyan().bold());
            for tool in featured_not_installed {
                println!("  {} {} - {}", 
                    style("•").dim(),
                    style(&tool.name).bold(),
                    tool.description.lines().next().unwrap_or("")
                );
                println!("    安装: {}", style(&format!("vcm install {}", tool.id)).cyan());
            }
            println!();
        }

        // 配置建议
        let needs_config: Vec<_> = installed.iter()
            .filter(|t| !t.is_configured)
            .collect();

        if !needs_config.is_empty() {
            println!("{}", style("配置建议").yellow().bold());
            for tool in needs_config {
                println!("  {} 需要配置 {} ({})", 
                    style("•").dim(),
                    tool.tool_name,
                    tool.missing_env_vars.join(", ")
                );
            }
            println!("  运行 {} 进行配置", style("vcm config <tool>").cyan());
        }
    }
}
