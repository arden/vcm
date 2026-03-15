//! 工具推荐命令

use crate::core::{Discovery, Registry};
use crate::i18n::translate;
use crate::models::Tool;
use anyhow::Result;
use console::style;

/// 推荐命令
pub struct RecommendCommand {
    mode: RecommendMode,
}

/// 推荐模式
pub enum RecommendMode {
    /// 基于使用习惯推荐
    Personal,
    /// 热门工具
    Trending,
    /// 新工具
    New,
    /// 按标签推荐
    ByTag(String),
}

impl RecommendCommand {
    pub fn new(mode: RecommendMode) -> Self {
        Self { mode }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;

        match &self.mode {
            RecommendMode::Personal => {
                // 先扫描安装的工具
                let installed = Discovery::new(Registry::load()?).scan();
                self.show_personal_recommendations(&registry, &installed)?;
            }
            RecommendMode::Trending => {
                self.show_trending(&registry)?;
            }
            RecommendMode::New => {
                self.show_new_tools(&registry)?;
            }
            RecommendMode::ByTag(tag) => {
                self.show_by_tag(&registry, tag)?;
            }
        }

        Ok(())
    }

    /// 个人推荐（基于已安装工具和免费额度）
    fn show_personal_recommendations(&self, registry: &Registry, installed: &[crate::models::InstalledTool]) -> Result<()> {
        println!("\n{}", style(format!("🎯 {}", translate("recommend.title"))).cyan().bold());
        println!("{}", "═".repeat(70));
        println!("\n{}", style(translate("recommend.based_usage")).dim());

        // 已安装的工具 ID
        let installed_ids: Vec<_> = installed.iter()
            .map(|t| t.tool_id.as_str())
            .collect();

        // 推荐有免费专业级模型的工具
        let mut free_pro_tools: Vec<&Tool> = registry.tools.iter()
            .filter(|t| {
                !installed_ids.contains(&t.id.as_str()) &&
                t.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false)
            })
            .collect();

        // 按是否有免费专业级模型排序
        free_pro_tools.sort_by(|a, b| {
            let a_score = self.score_tool(a);
            let b_score = self.score_tool(b);
            b_score.cmp(&a_score)
        });

        if !free_pro_tools.is_empty() {
            println!("\n  {}", style(format!("💡 {}:", translate("recommend.install_free"))).bold());
            for tool in free_pro_tools.iter().take(5) {
                self.print_tool_recommendation(tool);
            }
        }

        // 推荐热门但未安装的工具
        let featured_not_installed: Vec<&Tool> = registry.tools.iter()
            .filter(|t| t.featured && !installed_ids.contains(&t.id.as_str()))
            .collect();

        if !featured_not_installed.is_empty() {
            println!("\n  {}", style(format!("⭐ {}:", translate("recommend.hot"))).bold());
            for tool in featured_not_installed.iter().take(3) {
                self.print_tool_recommendation(tool);
            }
        }

        // 显示已安装概览
        println!("\n  {}", style(format!("📊 {}:", translate("recommend.installed_overview"))).bold());
        println!("    {}", translate("recommend.installed_count").replace("{}", &installed.len().to_string()));

        let configured_count = installed.iter().filter(|t| t.is_configured).count();
        println!("    {}", translate("recommend.configured_count")
            .replace("{c}", &configured_count.to_string())
            .replace("{t}", &installed.len().to_string()));

        if configured_count < installed.len() {
            println!("\n    {} {}",
                style("⚠️").yellow(),
                translate("recommend.some_unconfigured"));
        }

        println!("{}", "═".repeat(70));

        Ok(())
    }

    /// 热门工具
    fn show_trending(&self, registry: &Registry) -> Result<()> {
        println!("\n{}", style(format!("🔥 {}", translate("recommend.trending_title"))).cyan().bold());
        println!("{}", "═".repeat(70));

        // 按 featured 和定价排序
        let mut tools: Vec<&Tool> = registry.tools.iter()
            .filter(|t| t.featured)
            .collect();

        tools.sort_by(|a, b| {
            // 先按是否有免费额度排序
            let a_free = a.pricing.as_ref().map(|p| p.free_tier).unwrap_or(false);
            let b_free = b.pricing.as_ref().map(|p| p.free_tier).unwrap_or(false);
            b_free.cmp(&a_free)
        });

        println!("\n{}  {:<18} {:<14} {}",
            style(translate("recommend.rank")).bold(),
            style(translate("recommend.tool_col")).bold(),
            style(translate("recommend.vendor_col")).bold(),
            style(translate("recommend.free_quota_col")).bold());
        println!("{}", "─".repeat(60));

        for (i, tool) in tools.iter().enumerate().take(10) {
            let rank = style(format!("#{}", i + 1)).yellow();

            let free_status = if let Some(ref pricing) = tool.pricing {
                if pricing.has_free_pro_models() {
                    style(format!("✓ {}", translate("recommend.pro_free"))).green()
                } else if pricing.free_tier {
                    style(format!("✓ {}", translate("recommend.has_free_quota"))).cyan()
                } else {
                    style(format!("- {}", translate("recommend.paid"))).dim()
                }
            } else {
                style(format!("- {}", translate("recommend.unknown"))).dim()
            };

            println!("{}   {:<18} {:<14} {}",
                rank,
                style(&tool.name).bold(),
                style(&tool.vendor).dim(),
                free_status
            );
        }

        println!("{}", "═".repeat(70));
        println!("\n{}", translate("recommend.install_hint"));

        Ok(())
    }

    /// 新工具
    fn show_new_tools(&self, registry: &Registry) -> Result<()> {
        println!("\n{}", style(format!("🆕 {}", translate("recommend.new_title"))).cyan().bold());
        println!("{}", "═".repeat(70));

        // 按 updated 日期排序（如果有）
        let mut tools: Vec<&Tool> = registry.tools.iter().collect();

        // 简单排序：有免费额度的排前面
        tools.sort_by(|a, b| {
            let a_has_free = a.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            let b_has_free = b.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false);
            b_has_free.cmp(&a_has_free)
        });

        // 显示前 5 个作为"新工具"
        for tool in tools.iter().take(5) {
            println!("\n  {} {}", style("▸").cyan(), style(&tool.name).bold());
            println!("    {}", tool.description.lines().next().unwrap_or(""));

            if let Some(ref pricing) = tool.pricing {
                if pricing.has_free_pro_models() {
                    let pro_models: Vec<_> = pricing.free_models.iter()
                        .filter(|m| m.pro_grade)
                        .collect();
                    if !pro_models.is_empty() {
                        println!("    {} {}: {}",
                            style("🎁").yellow(),
                            translate("recommend.free_pro_models"),
                            pro_models.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ")
                        );
                    }
                }
            }

            println!("    {}: vcm install {}", translate("recommend.install"), tool.id);
        }

        println!("\n{}", "═".repeat(70));

        Ok(())
    }

    /// 按标签推荐
    fn show_by_tag(&self, registry: &Registry, tag: &str) -> Result<()> {
        println!("\n{} {}", style(format!("🏷️ {}:", translate("recommend.by_tag"))).cyan().bold(), style(tag).yellow());
        println!("{}", "═".repeat(70));

        let tools = registry.by_tag(tag);

        if tools.is_empty() {
            println!("\n{}", translate("recommend.no_tag_tools").replace("{}", tag));
            println!("\n{}", translate("recommend.available_tags"));
            return Ok(());
        }

        println!("\n{}\n", translate("recommend.found_tools").replace("{}", &tools.len().to_string()));

        for tool in &tools {
            println!("  {} {} - {}",
                style("▸").cyan(),
                style(&tool.name).bold(),
                tool.description.lines().next().unwrap_or("")
            );

            if let Some(ref pricing) = tool.pricing {
                if pricing.free_tier {
                    print!("    {}", style(format!("✓ {}", translate("recommend.has_free_quota"))).green());
                    if let Some(ref limit) = pricing.free_limit {
                        print!(" ({})", limit);
                    }
                    println!();
                }
            }
        }

        println!("\n{}", "═".repeat(70));
        println!("{}", translate("recommend.install_hint"));

        Ok(())
    }

    /// 计算工具评分
    fn score_tool(&self, tool: &Tool) -> u32 {
        let mut score = 0;

        // 有免费专业级模型 +50
        if tool.pricing.as_ref().map(|p| p.has_free_pro_models()).unwrap_or(false) {
            score += 50;
        }

        // 不需要信用卡 +20
        if tool.pricing.as_ref().map(|p| !p.credit_card_required).unwrap_or(false) {
            score += 20;
        }

        // 是 CLI 工具 +10
        if tool.is_cli {
            score += 10;
        }

        // 是 featured +15
        if tool.featured {
            score += 15;
        }

        score
    }

    /// 打印工具推荐
    fn print_tool_recommendation(&self, tool: &Tool) {
        println!("\n    {} {}", style(&tool.name).cyan().bold(), style(&tool.id).dim());
        println!("    {}", tool.description.lines().next().unwrap_or(""));

        if let Some(ref pricing) = tool.pricing {
            if pricing.has_free_pro_models() {
                let pro_models: Vec<_> = pricing.free_models.iter()
                    .filter(|m| m.pro_grade)
                    .map(|m| m.name.as_str())
                    .collect();
                println!("    {}: {}",
                    style(format!("🎁 {}", translate("recommend.free_pro_models"))).yellow(),
                    pro_models.join(", ")
                );
            }
        }

        println!("    {}: vcm install {}", translate("recommend.install"), tool.id);
    }
}