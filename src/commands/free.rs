//! free 命令实现 - 列出支持免费模型的工具

use crate::core::Registry;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// free 命令
pub struct FreeCommand {
    pro_only: bool,
    aggregate: bool,
}

impl FreeCommand {
    pub fn new(pro_only: bool, aggregate: bool) -> Self {
        Self { pro_only, aggregate }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;

        if self.aggregate {
            self.print_aggregate_view(&registry)?;
        } else {
            self.print_list_view(&registry)?;
        }

        Ok(())
    }

    /// 列表视图（原有功能）
    fn print_list_view(&self, registry: &Registry) -> Result<()> {
        println!("{} {}\n",
            style("🎁").dim(),
            style(translate("free.title")).cyan().bold()
        );
        println!("{}\n", style(translate("free.subtitle")).dim());

        // 筛选有免费额度的工具
        let free_tools: Vec<_> = registry.tools.iter()
            .filter(|t| {
                if let Some(ref pricing) = t.pricing {
                    if self.pro_only {
                        pricing.has_free_pro_models()
                    } else {
                        pricing.free_tier
                    }
                } else {
                    false
                }
            })
            .collect();

        if free_tools.is_empty() {
            println!("{}", translate("free.none_found"));
            return Ok(());
        }

        // 按是否有专业级免费模型排序
        let mut sorted_tools = free_tools.clone();
        sorted_tools.sort_by(|a, b| {
            let a_has_pro = a.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            let b_has_pro = b.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            b_has_pro.cmp(&a_has_pro)
        });

        for tool in sorted_tools {
            self.print_tool(tool);
        }

        println!("\n{}", style(translate("free.install_hint").replace("{}", "<tool>")).dim());

        Ok(())
    }

    /// 聚合视图（新功能）
    fn print_aggregate_view(&self, registry: &Registry) -> Result<()> {
        println!("\n{}", style("🎁 免费额度聚合面板").cyan().bold());
        println!("{}", "═".repeat(80));

        // 筛选有免费额度的工具
        let free_tools: Vec<_> = registry.tools.iter()
            .filter(|t| {
                if let Some(ref pricing) = t.pricing {
                    pricing.free_tier
                } else {
                    false
                }
            })
            .collect();

        if free_tools.is_empty() {
            println!("未找到有免费额度的工具");
            return Ok(());
        }

        // 按是否有专业级模型排序
        let mut sorted_tools = free_tools.clone();
        sorted_tools.sort_by(|a, b| {
            let a_has_pro = a.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            let b_has_pro = b.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            b_has_pro.cmp(&a_has_pro)
        });

        // 表头
        println!("\n{:<20} {:<25} {:<20} {:<12}", 
            style("工具").bold(), 
            style("免费额度").bold(), 
            style("专业级模型").bold(),
            style("状态").bold()
        );
        println!("{}", "─".repeat(80));

        let mut total_pro_tools = 0;
        let mut total_free_models = 0;

        for tool in &sorted_tools {
            let pricing = tool.pricing.as_ref().unwrap();

            // 免费额度
            let free_limit = pricing.free_limit.as_deref().unwrap_or("有");

            // 专业级模型
            let pro_models: Vec<_> = pricing.free_models.iter()
                .filter(|m| m.pro_grade)
                .collect();

            let pro_models_str = if pro_models.is_empty() {
                "-".to_string()
            } else {
                total_pro_tools += 1;
                total_free_models += pro_models.len();
                pro_models.iter()
                    .map(|m| m.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            // 状态
            let status = if tool.is_installed() {
                style("✓ 已安装").green().to_string()
            } else {
                style("未安装").dim().to_string()
            };

            // 截断过长的模型列表
            let display_models = if pro_models_str.len() > 18 {
                format!("{}...", &pro_models_str[..15])
            } else {
                pro_models_str
            };

            println!("{:<20} {:<25} {:<20} {:<12}", 
                tool.name, 
                free_limit, 
                display_models,
                status
            );
        }

        // 聚合统计
        println!("{}", "═".repeat(80));
        println!("\n{}", style("📊 聚合统计").bold());
        println!("  • 有免费额度的工具: {} 个", sorted_tools.len());
        println!("  • 提供专业级模型的工具: {} 个", style(total_pro_tools).green());
        println!("  • 可免费使用的专业级模型: {} 个", style(total_free_models).green());

        // 优化建议
        println!("\n{}", style("💡 最优免费组合策略").bold());

        // 推荐最佳组合
        let best_free_tools: Vec<_> = sorted_tools.iter()
            .filter(|t| {
                t.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false)
                    && t.pricing.as_ref().map(|p| !p.credit_card_required).unwrap_or(true)
            })
            .take(3)
            .collect();

        if !best_free_tools.is_empty() {
            println!("  推荐组合:");
            for (i, tool) in best_free_tools.iter().enumerate() {
                let pricing = tool.pricing.as_ref().unwrap();
                let pro_model = pricing.free_pro_models().first()
                    .map(|m| m.name.as_str())
                    .unwrap_or("专业模型");
                println!("    {}. {} - {} ({})",
                    i + 1,
                    style(&tool.name).cyan(),
                    style(pro_model).green(),
                    pricing.free_limit.as_deref().unwrap_or("免费")
                );
            }
        }

        println!("\n  提示: 使用 'vcm compare <tool1> <tool2>...' 对比多个工具");
        println!("        使用 'vcm free --pro' 只显示专业级模型\n");

        Ok(())
    }

    fn print_tool(&self, tool: &crate::models::Tool) {
        let pricing = match &tool.pricing {
            Some(p) => p,
            None => return,
        };

        let has_pro = pricing.has_free_pro_models();

        // 工具名称和标记
        if has_pro {
            println!("{} {} {}",
                style("★").yellow(),
                style(&tool.name).bold().green(),
                style(&format!("[{}]", translate("free.best_choice"))).yellow()
            );
        } else {
            println!("{}", style(&tool.name).bold());
        }

        // 免费限额
        if let Some(ref limit) = pricing.free_limit {
            println!("  {}: {}",
                style(translate("free.limit")).dim(),
                style(limit).cyan()
            );
        }

        // 免费模型
        if !pricing.free_models.is_empty() {
            println!("  {}:", translate("free.free_models"));
            for model in &pricing.free_models {
                let pro_badge = if model.pro_grade {
                    format!(" [{}]", translate("pricing.pro_grade"))
                } else {
                    "".to_string()
                };
                let desc = model.description.as_ref()
                    .map(|d| format!(" - {}", d))
                    .unwrap_or_default();

                println!("    {} {}{}{}",
                    if model.pro_grade { style("●").green() } else { style("○").dim() },
                    style(&model.name).yellow(),
                    style(pro_badge).green(),
                    style(desc).dim()
                );
            }
        }

        // 信用卡要求
        let card_status = if pricing.credit_card_required {
            style(translate("free.card_needed")).yellow()
        } else {
            style(translate("free.no_card")).green()
        };
        println!("  {}: {}", translate("free.card_required"), card_status);

        // 备注
        if let Some(ref note) = pricing.price_note {
            println!("  {}: {}", translate("pricing.note"), style(note).dim());
        }

        println!();
    }
}
