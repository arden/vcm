//! free 命令实现 - 列出支持免费模型的工具

use crate::core::Registry;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// free 命令
pub struct FreeCommand {
    pro_only: bool,
}

impl FreeCommand {
    pub fn new(pro_only: bool) -> Self {
        Self { pro_only }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;
        
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
