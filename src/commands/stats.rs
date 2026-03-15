//! 使用统计命令

use crate::commands::quota::{load_quota_config_or_default, QuotaConfig};
use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use crate::models::Pricing;
use anyhow::Result;
use console::style;

/// 统计命令
pub struct StatsCommand {
    /// 是否显示成本估算
    cost_only: bool,
    /// 是否导出为 JSON
    #[allow(dead_code)]
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
        println!("\n{}", style(format!("📊 {}", translate("stats.title"))).cyan().bold());
        println!("{}", "═".repeat(80));

        if quota_config.usage_records.is_empty() {
            println!("\n{}", translate("stats.no_records"));
            println!("\n{}", translate("stats.run_hint"));
            return Ok(());
        }

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let month = chrono::Local::now().format("%Y-%m-%m").to_string();

        // 今日使用排行
        println!("\n{}", style(format!("📈 {}", translate("stats.today_ranking"))).bold());
        println!("{}", "─".repeat(60));

        let mut today_stats: Vec<_> = quota_config.usage_records.iter()
            .filter(|(_, r)| r.today_date == today && r.today_count > 0)
            .collect();
        today_stats.sort_by(|a, b| b.1.today_count.cmp(&a.1.today_count));

        if today_stats.is_empty() {
            println!("  {}", translate("stats.no_today"));
        } else {
            for (i, (tool_id, record)) in today_stats.iter().enumerate() {
                let tool_name = registry.find_by_id(tool_id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(tool_id);
                println!("  {}. {} - {} {}", 
                    i + 1, 
                    style(tool_name).cyan(),
                    style(record.today_count).yellow(),
                    translate("stats.times")
                );
            }
        }

        // 本月使用统计
        println!("\n{}", style(format!("📅 {}", translate("stats.month_stats"))).bold());
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
            println!("  {} - {} {} ({}%)", 
                style(tool_name).cyan(),
                style(record.month_count).yellow(),
                translate("stats.times"),
                style(percentage).dim()
            );
        }

        if month_stats.is_empty() {
            println!("  {}", translate("stats.no_month"));
        } else {
            println!("\n  {}: {} {}", translate("stats.month_total"), style(total_month_count).green().bold(), translate("stats.times"));
        }

        // 使用趋势
        println!("\n{}", style(format!("📊 {}", translate("stats.trend"))).bold());
        println!("{}", "─".repeat(60));

        let total_calls: u64 = quota_config.usage_records.values()
            .map(|r| r.total_count)
            .sum();

        println!("  {}: {}", translate("stats.total_calls"), style(total_calls).cyan().bold());
        println!("  {}: {}", translate("stats.tools_used"), style(quota_config.usage_records.len()).cyan());

        // 最常用工具
        if let Some((tool_id, record)) = quota_config.usage_records.iter()
            .max_by_key(|(_, r)| r.total_count) 
        {
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);
            println!("  {}: {} ({} {})", 
                translate("stats.most_used"),
                style(tool_name).green(),
                style(record.total_count).yellow(),
                translate("stats.times")
            );
        }

        println!("{}", "═".repeat(80));
        println!("\n{}", translate("stats.cost_hint"));
        println!("{}", translate("stats.quota_hint"));

        Ok(())
    }

    /// 显示成本报告
    fn show_cost_report(&self, quota_config: &QuotaConfig, registry: &Registry) -> Result<()> {
        println!("\n{}", style(format!("💰 {}", translate("cost.title"))).cyan().bold());
        println!("{}", "═".repeat(80));

        if quota_config.usage_records.is_empty() {
            println!("\n{}", translate("cost.no_records"));
            return Ok(());
        }

        let month = chrono::Local::now().format("%Y-%m-%m").to_string();

        println!("\n{:<18} {:<12} {:<15} {:<15}", 
            style(translate("label.tool")).bold(),
            style(translate("stats.month_stats")).bold(),
            style(translate("cost.month_estimate")).bold(),
            style(translate("cost.optimization")).bold()
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

            let free_text = translate("pricing.free");
            println!("{:<18} {:<12} {:<15} {:<15}",
                tool_name,
                record.month_count,
                if cost > 0.0 {
                    style(format!("${:.2}", cost)).yellow().to_string()
                } else {
                    style(format!("$0.00 ({})", free_text)).green().to_string()
                },
                style(saving_tip).dim()
            );
        }

        println!("{}", "─".repeat(80));
        let all_free_text = translate("cost.all_free");
        println!("\n📊 {}: {}", 
            translate("cost.month_estimate"),
            if total_cost > 0.0 {
                style(format!("${:.2}", total_cost)).red().bold().to_string()
            } else {
                style(format!("$0.00 ({})", all_free_text)).green().bold().to_string()
            }
        );

        // 成本优化建议
        println!("\n{}", style(format!("💡 {}", translate("cost.optimization"))).bold());
        println!("{}", "─".repeat(60));

        // 检查是否使用免费工具
        let free_tools = registry.tools.iter()
            .filter(|t| {
                t.pricing.as_ref().map(|p| p.free_tier).unwrap_or(false)
            })
            .count();

        println!("  • {}", translate("cost.register_free").replace("{}", &style(free_tools).cyan().to_string()));
        println!("  • {}", translate("cost.view_free"));
        println!("  • {}", translate("cost.compare_hint"));

        println!("{}", "═".repeat(80));

        Ok(())
    }

    /// 估算成本和节省建议
    fn estimate_cost_and_savings(&self, count: u32, pricing: Option<&Pricing>) -> (f64, String) {
        match pricing {
            Some(p) => {
                if p.free_tier {
                    // 有免费额度，假设在免费范围内
                    (0.0, translate("cost.use_free_quota"))
                } else {
                    // 付费工具，粗略估算
                    // 假设每次调用约 $0.01-$0.10
                    let estimated_cost = count as f64 * 0.02;
                    (estimated_cost, translate("cost.consider_free_alt"))
                }
            }
            None => {
                // 未知定价，假设免费
                (0.0, "-".to_string())
            }
        }
    }
}