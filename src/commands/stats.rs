//! 使用统计命令

use crate::commands::quota::{load_quota_config_or_default, QuotaConfig};
use crate::core::{ConfigManager, Registry};
use crate::models::Pricing;
use anyhow::Result;
use console::style;
use std::collections::HashMap;

/// 统计命令
pub struct StatsCommand {
    /// 是否显示成本估算
    cost_only: bool,
    /// 是否导出为 JSON
    json_output: bool,
}

impl StatsCommand {
    pub fn new(cost_only: bool, json_output: bool) -> Self {
        Self { cost_only, json_output }
    }

    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;
        let quota_config = load_quota_config_or_default(&config_manager)?;
        let registry = Registry::load()?;

        if self.cost_only {
            self.show_cost_report(&quota_config, &registry)?;
        } else {
            self.show_stats(&quota_config, &registry)?;
        }

        Ok(())
    }

    /// 显示使用统计
    fn show_stats(&self, quota_config: &QuotaConfig, registry: &Registry) -> Result<()> {
        println!("\n{}", style("📊 使用统计面板").cyan().bold());
        println!("{}", "═".repeat(80));

        if quota_config.usage_records.is_empty() {
            println!("\n暂无使用记录");
            println!("\n提示: 使用 'vcm run <tool>' 启动工具时会自动记录使用量");
            return Ok(());
        }

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let month = chrono::Local::now().format("%Y-%m").to_string();

        // 今日使用排行
        println!("\n{}", style("📈 今日使用排行").bold());
        println!("{}", "─".repeat(60));

        let mut today_stats: Vec<_> = quota_config.usage_records.iter()
            .filter(|(_, r)| r.today_date == today && r.today_count > 0)
            .collect();
        today_stats.sort_by(|a, b| b.1.today_count.cmp(&a.1.today_count));

        if today_stats.is_empty() {
            println!("  今日暂无使用记录");
        } else {
            for (i, (tool_id, record)) in today_stats.iter().enumerate() {
                let tool_name = registry.find_by_id(tool_id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(tool_id);
                println!("  {}. {} - {} 次", 
                    i + 1, 
                    style(tool_name).cyan(),
                    style(record.today_count).yellow()
                );
            }
        }

        // 本月使用统计
        println!("\n{}", style("📅 本月使用统计").bold());
        println!("{}", "─".repeat(60));

        let mut month_stats: Vec<_> = quota_config.usage_records.iter()
            .filter(|(_, r)| r.month_date == month && r.month_count > 0)
            .collect();
        month_stats.sort_by(|a, b| b.1.month_count.cmp(&a.1.month_count));

        let mut total_month_count = 0u32;
        for (tool_id, record) in &month_stats {
            total_month_count += record.month_count;
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);
            let percentage = if total_month_count > 0 {
                (record.month_count as f64 / total_month_count as f64 * 100.0) as u8
            } else {
                0
            };
            println!("  {} - {} 次 ({}%)", 
                style(tool_name).cyan(),
                style(record.month_count).yellow(),
                style(percentage).dim()
            );
        }

        if month_stats.is_empty() {
            println!("  本月暂无使用记录");
        } else {
            println!("\n  本月总使用: {} 次", style(total_month_count).green().bold());
        }

        // 使用趋势
        println!("\n{}", style("📊 使用趋势").bold());
        println!("{}", "─".repeat(60));

        let total_calls: u64 = quota_config.usage_records.values()
            .map(|r| r.total_count)
            .sum();

        println!("  总调用次数: {}", style(total_calls).cyan().bold());
        println!("  使用过的工具: {} 个", style(quota_config.usage_records.len()).cyan());

        // 最常用工具
        if let Some((tool_id, record)) = quota_config.usage_records.iter()
            .max_by_key(|(_, r)| r.total_count) 
        {
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);
            println!("  最常用工具: {} ({} 次)", 
                style(tool_name).green(),
                style(record.total_count).yellow()
            );
        }

        println!("{}", "═".repeat(80));
        println!("\n提示: 使用 'vcm cost' 查看成本估算");
        println!("      使用 'vcm quota usage' 查看详细使用记录");

        Ok(())
    }

    /// 显示成本报告
    fn show_cost_report(&self, quota_config: &QuotaConfig, registry: &Registry) -> Result<()> {
        println!("\n{}", style("💰 成本估算报告").cyan().bold());
        println!("{}", "═".repeat(80));

        if quota_config.usage_records.is_empty() {
            println!("\n暂无使用记录，无法估算成本");
            return Ok(());
        }

        let month = chrono::Local::now().format("%Y-%m").to_string();

        println!("\n{:<18} {:<12} {:<15} {:<15}", 
            style("工具").bold(),
            style("本月调用").bold(),
            style("估算成本").bold(),
            style("节省策略").bold()
        );
        println!("{}", "─".repeat(80));

        let mut total_cost = 0.0f64;
        let mut month_records: Vec<_> = quota_config.usage_records.iter()
            .filter(|(_, r)| r.month_date == month && r.month_count > 0)
            .collect();
        month_records.sort_by(|a, b| b.1.month_count.cmp(&a.1.month_count));

        for (tool_id, record) in &month_records {
            let tool = registry.find_by_id(tool_id);
            let tool_name = tool.map(|t| t.name.as_str()).unwrap_or(tool_id);

            // 估算成本（简化估算，实际应基于具体 API 定价）
            let (cost, saving_tip) = self.estimate_cost_and_savings(
                record.month_count,
                tool.and_then(|t| t.pricing.as_ref())
            );

            total_cost += cost;

            println!("{:<18} {:<12} {:<15} {:<15}",
                tool_name,
                record.month_count,
                if cost > 0.0 {
                    style(format!("${:.2}", cost)).yellow().to_string()
                } else {
                    style("$0.00 (免费)").green().to_string()
                },
                style(saving_tip).dim()
            );
        }

        println!("{}", "─".repeat(80));
        println!("\n📊 本月估算总成本: {}", 
            if total_cost > 0.0 {
                style(format!("${:.2}", total_cost)).red().bold().to_string()
            } else {
                style("$0.00 (全部免费!)").green().bold().to_string()
            }
        );

        // 成本优化建议
        println!("\n{}", style("💡 成本优化建议").bold());
        println!("{}", "─".repeat(60));

        // 检查是否使用免费工具
        let free_tools = registry.tools.iter()
            .filter(|t| {
                t.pricing.as_ref().map(|p| p.free_tier).unwrap_or(false)
            })
            .count();

        println!("  • 注册 {} 个工具的免费额度可节省成本", style(free_tools).cyan());
        println!("  • 使用 'vcm free --aggregate' 查看所有免费额度");
        println!("  • 使用 'vcm compare <tool1> <tool2>' 对比工具性价比");

        println!("{}", "═".repeat(80));

        Ok(())
    }

    /// 估算成本和节省建议
    fn estimate_cost_and_savings(&self, count: u32, pricing: Option<&Pricing>) -> (f64, &'static str) {
        match pricing {
            Some(p) => {
                if p.free_tier {
                    // 有免费额度，假设在免费范围内
                    (0.0, "利用免费额度")
                } else {
                    // 付费工具，粗略估算
                    // 假设每次调用约 $0.01-$0.10
                    let estimated_cost = count as f64 * 0.02;
                    (estimated_cost, "考虑免费替代")
                }
            }
            None => {
                // 未知定价，假设免费
                (0.0, "-")
            }
        }
    }
}
