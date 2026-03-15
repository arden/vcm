//! 配额追踪命令

use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use crate::models::VcmConfig;
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
    /// 今日日期 (YYYY-MM-DD)
    pub today_date: String,
    /// 本月 (YYYY-MM)
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

        println!("\n{}", style("📊 配额监控面板").cyan().bold());
        println!("{}", "═".repeat(80));

        // 获取当前日期
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let month = chrono::Local::now().format("%Y-%m").to_string();

        // 表头
        println!("\n{:<18} {:<12} {:<12} {:<10} {:<10}", 
            style("工具").bold(), 
            style("今日").bold(), 
            style("本月").bold(),
            style("限额").bold(),
            style("状态").bold()
        );
        println!("{}", "─".repeat(80));

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

            // 获取限额信息
            let limit_str = pricing
                .and_then(|p| p.free_limit.as_ref())
                .map(|l| {
                    // 尝试解析限额数字
                    if l.contains("/day") || l.contains("次/天") {
                        "每日限制"
                    } else if l.contains("/month") || l.contains("次/月") {
                        "每月限制"
                    } else {
                        "有限额"
                    }
                })
                .unwrap_or("无限制");

            // 计算状态
            let status = self.calculate_status(today_count, month_count, pricing, quota_config);

            // 计算今日使用百分比
            let percentage = self.calculate_percentage(today_count, pricing);

            println!("{:<18} {:<12} {:<12} {:<10} {}",
                tool.name,
                if today_count > 0 {
                    format!("{} ({:.0}%)", today_count, percentage)
                } else {
                    "0".to_string()
                },
                month_count,
                style(limit_str).dim(),
                status
            );
        }

        // 显示阈值设置
        println!("{}", "─".repeat(80));
        println!("\n{}", style("⚙️  阈值设置").bold());
        if let Some(warn) = quota_config.warn_threshold {
            println!("  警告阈值: {}%", style(warn).yellow());
        } else {
            println!("  警告阈值: {} (未设置)", style("默认 80%").dim());
        }
        if let Some(limit) = quota_config.hard_limit {
            println!("  硬限制: {}% {}", style(limit).red(), style("(超限将阻止使用)").dim());
        } else {
            println!("  硬限制: {} (未设置)", style("禁用").dim());
        }

        println!("\n{}", "═".repeat(80));
        println!("\n提示: 使用 'vcm quota warn 80' 设置警告阈值");
        println!("      使用 'vcm quota usage <tool>' 查看详细使用记录");

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
                return style("⛔ 超限").red().to_string();
            }
        }

        if percentage >= warn_threshold as f32 {
            return style("⚠️  即将达到限额").yellow().to_string();
        }

        if today_count > 0 {
            style("✓ 正常").green().to_string()
        } else {
            style("- 未使用").dim().to_string()
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
            bail!("阈值必须在 0-100 之间");
        }

        quota_config.warn_threshold = Some(threshold);
        self.save_quota_config(config_manager, quota_config)?;

        println!("{} 警告阈值已设置为 {}%", 
            style("✓").green(), 
            style(threshold).yellow()
        );
        println!("\n当使用量达到此阈值时，系统将显示警告提示");

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
                    bail!("阈值必须在 0-100 之间");
                }
                quota_config.hard_limit = Some(t);
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} 硬限制已设置为 {}%", 
                    style("✓").green(), 
                    style(t).red()
                );
                println!("\n{} 当使用量达到此阈值时，系统将阻止继续使用", 
                    style("⚠️").yellow()
                );
            }
            None => {
                quota_config.hard_limit = None;
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} 硬限制已禁用", style("✓").green());
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
                    None => bail!("工具 '{}' 未找到", tool_id),
                };

                let record = quota_config.usage_records.get(&tool_def.id);
                
                println!("\n{} {}", style("📊").dim(), style(format!("{} 使用记录", tool_def.name)).cyan().bold());
                println!("{}", "─".repeat(50));

                if let Some(r) = record {
                    println!("  今日使用: {} 次", r.today_count);
                    println!("  本月使用: {} 次", r.month_count);
                    println!("  总使用量: {} 次", r.total_count);
                    if let Some(ref last) = r.last_used {
                        println!("  最后使用: {}", last);
                    }
                } else {
                    println!("  暂无使用记录");
                }

                // 显示限额信息
                if let Some(pricing) = &tool_def.pricing {
                    if let Some(limit) = &pricing.free_limit {
                        println!("\n  免费限额: {}", style(limit).cyan());
                    }
                }
            }
            None => {
                // 显示所有工具的使用记录
                println!("\n{}", style("📊 使用记录汇总").cyan().bold());
                println!("{}", "═".repeat(80));

                if quota_config.usage_records.is_empty() {
                    println!("\n暂无使用记录");
                    println!("\n提示: 使用 'vcm run <tool>' 启动工具时会自动记录使用量");
                } else {
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

                    println!("\n{:<20} {:<10} {:<10} {:<10}", 
                        style("工具").bold(),
                        style("今日").bold(),
                        style("本月").bold(),
                        style("总计").bold()
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
                    println!("{} 已重置 '{}' 的使用记录", style("✓").green(), tool_id);
                } else {
                    println!("工具 '{}' 没有使用记录", tool_id);
                }
            }
            None => {
                quota_config.usage_records.clear();
                self.save_quota_config(config_manager, quota_config)?;
                println!("{} 已重置所有使用记录", style("✓").green());
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
