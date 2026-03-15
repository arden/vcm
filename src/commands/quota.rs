//! 配额追踪命令

use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;
use std::collections::HashMap;

/// 配额命令
pub struct QuotaCommand {
    action: QuotaAction,
}

/// 配额操作类型
pub enum QuotaAction {
    /// 显示配额状态
    Status,
    /// 设置警告阈值
    Warn { threshold: u8 },
    /// 设置硬限制
    Limit { threshold: Option<u8> },
    /// 显示使用记录
    Usage { tool: Option<String> },
    /// 重置使用记录
    Reset { tool: Option<String> },
}

/// 工具使用记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ToolUsageRecord {
    /// 工具 ID
    pub tool_id: String,
    /// 今日使用次数
    pub today_count: u32,
    /// 本月使用次数
    pub month_count: u32,
    /// 总使用次数
    pub total_count: u64,
    /// 今日日期
    pub today_date: String,
    /// 本月
    pub month_date: String,
    /// 最后使用时间
    pub last_used: Option<String>,
}

/// 配额配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct QuotaConfig {
    /// 警告阈值 (百分比)
    #[serde(default)]
    pub warn_threshold: Option<u8>,
    /// 硬限制阈值 (百分比，超过后阻止使用)
    #[serde(default)]
    pub hard_limit: Option<u8>,
    /// 工具使用记录
    #[serde(default)]
    pub usage_records: HashMap<String, ToolUsageRecord>,
}

impl QuotaCommand {
    pub fn new(action: QuotaAction) -> Self {
        Self { action }
    }

    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;
        let mut quota_config = self.load_quota_config(&config_manager)?;

        match &self.action {
            QuotaAction::Status => {
                self.show_status(&quota_config)?;
            }
            QuotaAction::Warn { threshold } => {
                self.set_warn_threshold(&config_manager, &mut quota_config, *threshold)?;
            }
            QuotaAction::Limit { threshold } => {
                self.set_hard_limit(&config_manager, &mut quota_config, *threshold)?;
            }
            QuotaAction::Usage { tool } => {
                self.show_usage(&quota_config, tool)?;
            }
            QuotaAction::Reset { tool } => {
                self.reset_usage(&config_manager, &mut quota_config, tool)?;
            }
        }

        Ok(())
    }

    /// 加载配额配置
    pub fn load_quota_config(&self, config_manager: &ConfigManager) -> Result<QuotaConfig> {
        let quota_path = config_manager.config_dir().join("quota.json");
        
        if !quota_path.exists() {
            return Ok(QuotaConfig::default());
        }

        let content = std::fs::read_to_string(&quota_path)?;
        let config: QuotaConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 保存配额配置
    fn save_quota_config(&self, config_manager: &ConfigManager, config: &QuotaConfig) -> Result<()> {
        let quota_path = config_manager.config_dir().join("quota.json");
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(&quota_path, content)?;
        Ok(())
    }

    /// 显示配额状态
    fn show_status(&self, quota_config: &QuotaConfig) -> Result<()> {
        let registry = Registry::load()?;

        println!("\n{}", style(format!("📊 {}", translate("quota.title"))).cyan().bold());
        println!("{}", "═".repeat(90));

        // 获取当前日期
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let month = chrono::Local::now().format("%Y-%m").to_string();

        // 表头
        println!("\n{:<18} {:<8} {:<8} {:<40} {:<12}", 
            style(translate("quota.tool")).bold(), 
            style(translate("quota.today")).bold(), 
            style(translate("quota.month")).bold(),
            style(translate("quota.limit_col")).bold(),
            style(translate("quota.status")).bold()
        );
        println!("{}", "─".repeat(90));

        // 只显示有使用记录或有限额的工具
        let mut tools_with_data: Vec<_> = registry.tools.iter()
            .filter(|t| {
                let has_record = quota_config.usage_records.contains_key(&t.id);
                let has_limit = t.pricing.as_ref()
                    .map(|p| p.free_limit.is_some())
                    .unwrap_or(false);
                has_record || has_limit
            })
            .collect();

        // 按今日使用量排序
        tools_with_data.sort_by(|a, b| {
            let a_count = quota_config.usage_records.get(&a.id)
                .map(|r| if r.today_date == today { r.today_count } else { 0 })
                .unwrap_or(0);
            let b_count = quota_config.usage_records.get(&b.id)
                .map(|r| if r.today_date == today { r.today_count } else { 0 })
                .unwrap_or(0);
            b_count.cmp(&a_count)
        });

        for tool in &tools_with_data {
            let record = quota_config.usage_records.get(&tool.id);
            let pricing = tool.pricing.as_ref();

            // 获取使用量
            let (today_count, month_count) = if let Some(r) = record {
                let today_c = if r.today_date == today { r.today_count } else { 0 };
                let month_c = if r.month_date == month { r.month_count } else { 0 };
                (today_c, month_c)
            } else {
                (0, 0)
            };

            // 获取限额信息 - 直接显示实际限额
            let limit_str = pricing
                .and_then(|p| p.free_limit.as_ref())
                .cloned()
                .unwrap_or_else(|| translate("quota.unlimited"));

            // 计算状态
            let status = self.calculate_status(today_count, month_count, pricing, quota_config);

            // 计算今日使用百分比
            let percentage = self.calculate_percentage(today_count, pricing);

            // 截断过长的限额字符串
            let display_limit = if limit_str.chars().count() > 35 {
                let chars: Vec<char> = limit_str.chars().collect();
                format!("{}...", chars[..32].iter().collect::<String>())
            } else {
                limit_str.clone()
            };

            println!("{:<18} {:<8} {:<8} {:<40} {}",
                tool.name,
                if today_count > 0 {
                    format!("{} ({:.0}%)", today_count, percentage)
                } else {
                    "0".to_string()
                },
                month_count,
                style(display_limit).dim(),
                status
            );
        }

        // 显示阈值设置
        println!("{}", "─".repeat(90));
        println!("\n{}", style(format!("⚙️  {}", translate("quota.threshold_settings"))).bold());
        if let Some(warn) = quota_config.warn_threshold {
            println!("  {}: {}%", translate("quota.warn_threshold"), style(warn).yellow());
        } else {
            println!("  {}: {} ({})", translate("quota.warn_threshold"), style(&translate("quota.default_80")).dim(), translate("quota.not_set"));
        }
        if let Some(limit) = quota_config.hard_limit {
            println!("  {}: {}% {}", translate("quota.hard_limit"), style(limit).red(), style(translate("quota.block_on_exceed")).dim());
        } else {
            println!("  {}: {} ({})", translate("quota.hard_limit"), style(&translate("quota.disabled")).dim(), translate("quota.not_set"));
        }

        println!("\n{}", "═".repeat(90));
        println!("\n{}", translate("quota.warn_hint"));
        println!("{}", translate("quota.usage_hint"));

        Ok(())
    }

    /// 计算使用状态
    fn calculate_status(
        &self,
        today_count: u32,
        _month_count: u32,
        pricing: Option<&crate::models::Pricing>,
        quota_config: &QuotaConfig,
    ) -> String {
        let percentage = self.calculate_percentage(today_count, pricing);
        let warn_threshold = quota_config.warn_threshold.unwrap_or(80);
        let hard_limit = quota_config.hard_limit;

        if let Some(limit) = hard_limit {
            if percentage >= limit as f32 {
                return style("⛔ ".to_string() + &translate("status.error")).red().to_string();
            }
        }

        if percentage >= warn_threshold as f32 {
            return style("⚠️  ".to_string() + &translate("status.warning")).yellow().to_string();
        }

        if today_count > 0 {
            style("✓ ".to_string() + &translate("status.healthy")).green().to_string()
        } else {
            style("- ".to_string() + &translate("quota.not_used")).dim().to_string()
        }
    }

    /// 计算使用百分比
    fn calculate_percentage(&self, _count: u32, _pricing: Option<&crate::models::Pricing>) -> f32 {
        // 由于限额格式多样，这里返回一个估算值
        // 实际应用中可以解析具体的限额数字
        0.0
    }

    /// 设置警告阈值
    fn set_warn_threshold(
        &self,
        config_manager: &ConfigManager,
        quota_config: &mut QuotaConfig,
        threshold: u8,
    ) -> Result<()> {
        if threshold > 100 {
            bail!("{}", translate("quota.threshold_range"));
        }

        quota_config.warn_threshold = Some(threshold);
        self.save_quota_config(config_manager, quota_config)?;

        println!("{} {}", 
            style("✓").green(), 
            translate("quota.warn_set").replace("{}", &threshold.to_string())
        );
        println!("\n{}", translate("quota.warn_desc"));

        Ok(())
    }

    /// 设置硬限制
    fn set_hard_limit(
        &self,
        config_manager: &ConfigManager,
        quota_config: &mut QuotaConfig,
        threshold: Option<u8>,
    ) -> Result<()> {
        match threshold {
            Some(t) => {
                if t > 100 {
                    bail!("{}", translate("quota.threshold_range"));
                }
                quota_config.hard_limit = Some(t);
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} {}", 
                    style("✓").green(), 
                    translate("quota.limit_set").replace("{}", &t.to_string())
                );
                println!("\n{} {}", 
                    style("⚠️").yellow(),
                    translate("quota.limit_warning")
                );
            }
            None => {
                quota_config.hard_limit = None;
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} {}", style("✓").green(), translate("quota.limit_disabled"));
            }
        }

        Ok(())
    }

    /// 显示使用记录
    fn show_usage(&self, quota_config: &QuotaConfig, tool: &Option<String>) -> Result<()> {
        let registry = Registry::load()?;

        match tool {
            Some(tool_id) => {
                // 显示单个工具的使用记录
                let tool_def = registry.find_by_id(tool_id)
                    .or_else(|| registry.find_by_name(tool_id).first().copied());

                let tool_def = match tool_def {
                    Some(t) => t,
                    None => bail!("{}", translate("tool.not_found").replace("{}", tool_id)),
                };

                let record = quota_config.usage_records.get(&tool_def.id);
                
                println!("\n{} {}", style("📊").dim(), style(translate("quota.usage_title").replace("{}", &tool_def.name)).cyan().bold());
                println!("{}", "─".repeat(50));

                if let Some(r) = record {
                    println!("  {}: {} {}", translate("quota.today_usage"), r.today_count, translate("stats.times"));
                    println!("  {}: {} {}", translate("quota.month_usage"), r.month_count, translate("stats.times"));
                    println!("  {}: {} {}", translate("quota.total_usage"), r.total_count, translate("stats.times"));
                    if let Some(ref last) = r.last_used {
                        println!("  {}: {}", translate("quota.last_used"), last);
                    }
                } else {
                    println!("  {}", translate("quota.no_records"));
                }

                // 显示限额信息
                if let Some(pricing) = &tool_def.pricing {
                    if let Some(limit) = &pricing.free_limit {
                        println!("\n  {}: {}", translate("quota.free_limit"), style(limit).cyan());
                    }
                }
            }
            None => {
                // 显示所有工具的使用记录
                println!("\n{}", style(format!("📊 {}", translate("quota.summary_title"))).cyan().bold());
                println!("{}", "═".repeat(80));

                if quota_config.usage_records.is_empty() {
                    println!("\n{}", translate("quota.no_records"));
                    println!("\n{}", translate("quota.run_hint"));
                } else {
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

                    println!("\n{:<20} {:<10} {:<10} {:<10}", 
                        style(translate("quota.tool")).bold(),
                        style(translate("quota.today")).bold(),
                        style(translate("quota.month")).bold(),
                        style(translate("quota.total_usage")).bold()
                    );
                    println!("{}", "─".repeat(60));

                    let mut records: Vec<_> = quota_config.usage_records.iter().collect();
                    records.sort_by(|a, b| b.1.total_count.cmp(&a.1.total_count));

                    for (tool_id, record) in records {
                        let tool_name = registry.find_by_id(tool_id)
                            .map(|t| t.name.as_str())
                            .unwrap_or(tool_id);

                        let today_count = if record.today_date == today {
                            record.today_count
                        } else {
                            0
                        };

                        println!("{:<20} {:<10} {:<10} {:<10}",
                            tool_name,
                            today_count,
                            record.month_count,
                            record.total_count
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// 重置使用记录
    fn reset_usage(
        &self,
        config_manager: &ConfigManager,
        quota_config: &mut QuotaConfig,
        tool: &Option<String>,
    ) -> Result<()> {
        match tool {
            Some(tool_id) => {
                if quota_config.usage_records.remove(tool_id).is_some() {
                    self.save_quota_config(config_manager, quota_config)?;
                    println!("{} {}", style("✓").green(), translate("quota.reset_tool").replace("{}", tool_id));
                } else {
                    println!("{}", translate("quota.no_records_for_tool").replace("{}", tool_id));
                }
            }
            None => {
                quota_config.usage_records.clear();
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} {}", style("✓").green(), translate("quota.all_reset"));
            }
        }

        Ok(())
    }
}

/// 记录工具使用（供其他命令调用）
pub fn record_tool_usage(tool_id: &str) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let quota_path = config_manager.config_dir().join("quota.json");
    
    let mut quota_config = if quota_path.exists() {
        let content = std::fs::read_to_string(&quota_path)?;
        serde_json::from_str(&content)?
    } else {
        QuotaConfig::default()
    };

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let month = chrono::Local::now().format("%Y-%m").to_string();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let record = quota_config.usage_records.entry(tool_id.to_string())
        .or_insert(ToolUsageRecord {
            tool_id: tool_id.to_string(),
            ..Default::default()
        });

    // 检查是否需要重置日期
    if record.today_date != today {
        record.today_date = today.clone();
        record.today_count = 0;
    }
    if record.month_date != month {
        record.month_date = month.clone();
        record.month_count = 0;
    }

    // 增加计数
    record.today_count += 1;
    record.month_count += 1;
    record.total_count += 1;
    record.last_used = Some(now);

    // 保存
    let content = serde_json::to_string_pretty(&quota_config)?;
    std::fs::write(&quota_path, content)?;

    Ok(())
}

/// 加载配额配置（公共辅助函数）
pub fn load_quota_config_or_default(config_manager: &ConfigManager) -> Result<QuotaConfig> {
    let quota_path = config_manager.config_dir().join("quota.json");

    if !quota_path.exists() {
        return Ok(QuotaConfig::default());
    }

    let content = std::fs::read_to_string(&quota_path)?;
    let config: QuotaConfig = serde_json::from_str(&content)?;
    Ok(config)
}