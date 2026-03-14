//! list 命令实现

use crate::core::Registry;
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::style;
use std::collections::HashSet;

/// list 命令
pub struct ListCommand {
    installed_only: bool,
    tag: Option<String>,
    json: bool,
}

impl ListCommand {
    pub fn new(installed_only: bool, tag: Option<String>, json: bool) -> Self {
        Self { installed_only, tag, json }
    }

    pub fn execute(&self) -> Result<()> {
        // 加载注册表
        let registry = Registry::load()?;

        // 获取已安装工具ID
        let installed_ids: HashSet<_> = registry.tools.iter()
            .filter(|t| t.is_installed())
            .map(|t| t.id.as_str())
            .collect();

        // 筛选工具
        let tools: Vec<&Tool> = if self.installed_only {
            registry.tools.iter()
                .filter(|t| installed_ids.contains(t.id.as_str()))
                .collect()
        } else if let Some(ref tag) = self.tag {
            registry.by_tag(tag)
        } else {
            registry.tools.iter().collect()
        };

        // 输出结果
        if self.json {
            self.output_json(&tools, &installed_ids)?;
        } else {
            self.output_human(&tools, &installed_ids)?;
        }

        Ok(())
    }

    fn output_json(&self, tools: &[&Tool], installed_ids: &HashSet<&str>) -> Result<()> {
        let output: Vec<serde_json::Value> = tools.iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "name": t.name,
                    "vendor": t.vendor,
                    "description": t.description,
                    "installed": installed_ids.contains(t.id.as_str()),
                    "tags": t.tags,
                    "featured": t.featured,
                })
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&output)?);
        Ok(())
    }

    fn output_human(&self, tools: &[&Tool], installed_ids: &HashSet<&str>) -> Result<()> {
        let total = tools.len();
        let installed_count = installed_ids.len();

        println!("{} {}\n",
            style("📋").dim(),
            translate("list.title").replace("{}", &format!("{}", style(total).cyan().bold()))
        );

        // 已安装工具
        let installed_tools: Vec<&&Tool> = tools.iter()
            .filter(|t| installed_ids.contains(t.id.as_str()))
            .collect();
        
        if !installed_tools.is_empty() {
            println!("{}", style(translate("list.installed")).green().bold());
            for tool in &installed_tools {
                self.print_tool(tool, true);
            }
            println!();
        }

        // 推荐工具
        let featured: Vec<&&Tool> = tools.iter()
            .filter(|t| t.featured && !installed_ids.contains(t.id.as_str()))
            .collect();

        if !featured.is_empty() {
            println!("{}", style(translate("list.recommended")).cyan().bold());
            for tool in &featured {
                self.print_tool(tool, false);
            }
            println!();
        }

        // 其他工具
        let others: Vec<&&Tool> = tools.iter()
            .filter(|t| !t.featured && !installed_ids.contains(t.id.as_str()))
            .collect();

        if !others.is_empty() && !self.installed_only {
            self.print_grouped_tools(&others);
        }

        // 汇总统计
        self.print_summary(total, installed_count);

        Ok(())
    }

    fn print_grouped_tools(&self, tools: &[&&Tool]) {
        let mut groups: std::collections::HashMap<String, Vec<&&Tool>> = std::collections::HashMap::new();
        
        for tool in tools {
            let primary_tag = tool.tags.first().cloned().unwrap_or_else(|| "other".to_string());
            groups.entry(primary_tag)
                .or_default()
                .push(*tool);
        }

        // 按数量排序
        let mut sorted_groups: Vec<_> = groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (tag, tag_tools) in sorted_groups {
            if tag_tools.len() >= 2 {
                println!("{} {}", style(&format!("{}:", translate("label.tag"))).dim(), style(&tag).yellow().bold());
                for tool in &tag_tools {
                    self.print_tool(tool, false);
                }
                println!();
            }
        }
    }

    fn print_tool(&self, tool: &Tool, is_installed: bool) {
        let status = if is_installed {
            style("✓").green()
        } else {
            style("○").dim()
        };

        let desc = tool.description.lines().next().unwrap_or("");
        let desc_chars: Vec<char> = desc.chars().collect();
        let desc = if desc_chars.len() > 40 {
            let truncated: String = desc_chars[..37].iter().collect();
            format!("{}...", truncated)
        } else {
            desc.to_string()
        };

        println!(
            "  {} {:<15} {:<12} {}",
            status,
            style(&tool.id).bold(),
            style(&tool.vendor).dim(),
            desc
        );
    }

    fn print_summary(&self, total: usize, installed_count: usize) {
        let available = total - installed_count;
        let percentage = if total > 0 {
            installed_count * 100 / total
        } else {
            0
        };

        println!("┌─────────────────────────────────────┐");
        println!("│ {:<35} │", translate("list.summary")
            .replace("{}", &format!("{}", installed_count))
            .replace("{total}", &format!("{}", total))
            .replace("{percent}", &format!("{}", percentage))
        );
        println!("│ {:<35} │", translate("list.available").replace("{}", &format!("{}", available)));
        println!("└─────────────────────────────────────┘");

        if available > 0 {
            println!("\n{}: {}", translate("label.hint"), translate("hint.install"));
        }
    }
}
