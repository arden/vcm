//! 工具对比命令

use crate::core::Registry;
use crate::models::Tool;
use anyhow::{bail, Result};
use console::style;

/// 工具对比命令
pub struct CompareCommand {
    tools: Vec<String>,
}

impl CompareCommand {
    pub fn new(tools: Vec<String>) -> Self {
        Self { tools }
    }

    pub fn execute(&self) -> Result<()> {
        if self.tools.len() < 2 {
            bail!("至少需要指定两个工具进行对比");
        }

        if self.tools.len() > 5 {
            bail!("最多支持同时对比 5 个工具");
        }

        let registry = Registry::load()?;
        let mut tools_to_compare: Vec<&Tool> = Vec::new();

        // 查找所有工具
        for tool_id in &self.tools {
            let tool = registry.find_by_id(tool_id)
                .or_else(|| registry.find_by_name(tool_id).first().copied());

            match tool {
                Some(t) => tools_to_compare.push(t),
                None => bail!("工具 '{}' 未找到", tool_id),
            }
        }

        // 打印对比表格
        self.print_comparison(&tools_to_compare);

        Ok(())
    }

    fn print_comparison(&self, tools: &[&Tool]) {
        println!("\n{}", style("工具对比").bold());
        println!("{}", "═".repeat(80));

        // 表头
        let header: String = tools.iter()
            .map(|t| format!("{:<18}", t.name))
            .collect::<Vec<_>>()
            .join("│");
        println!("{:<12}│{}", style("特性").bold(), style(header).bold());
        println!("{}", "─".repeat(80));

        // 供应商
        self.print_row("供应商", tools.iter().map(|t| t.vendor.as_str()).collect());

        // 描述
        let descriptions: Vec<String> = tools.iter()
            .map(|t| {
                let desc = &t.description;
                if desc.len() > 16 {
                    format!("{}...", &desc[..13])
                } else {
                    desc.clone()
                }
            })
            .collect();
        self.print_row("描述", descriptions.iter().map(|s| s.as_str()).collect());

        // 是否 CLI
        self.print_row("CLI", tools.iter().map(|t| if t.is_cli { "✓" } else { "✗" }).collect());

        // 定价信息
        let pricing_info: Vec<String> = tools.iter()
            .map(|t| {
                match &t.pricing {
                    Some(p) => {
                        if p.free_tier {
                            let limit = p.free_limit.as_deref().unwrap_or("有");
                            if limit.len() > 16 {
                                format!("{}...", &limit[..13])
                            } else {
                                limit.to_string()
                            }
                        } else {
                            "付费".to_string()
                        }
                    }
                    None => "未知".to_string()
                }
            })
            .collect();
        self.print_row("免费额度", pricing_info.iter().map(|s| s.as_str()).collect());

        // 免费专业级模型
        let pro_models: Vec<String> = tools.iter()
            .map(|t| {
                match &t.pricing {
                    Some(p) => {
                        let pro: Vec<_> = p.free_models.iter()
                            .filter(|m| m.pro_grade)
                            .map(|m| m.name.as_str())
                            .collect();
                        if pro.is_empty() { "-".to_string() } else { pro.join(", ") }
                    }
                    None => "-".to_string()
                }
            })
            .collect();
        self.print_row("专业模型", pro_models.iter().map(|s| s.as_str()).collect());

        // 需要信用卡
        let cc_required: Vec<&str> = tools.iter()
            .map(|t| {
                match &t.pricing {
                    Some(p) => if p.credit_card_required { "是" } else { "否" }
                    None => "未知"
                }
            })
            .collect();
        self.print_row("需信用卡", cc_required);

        // 安装方式
        let install_methods: Vec<String> = tools.iter()
            .map(|t| {
                if t.install_methods.is_empty() {
                    "-".to_string()
                } else {
                    t.install_methods.iter()
                        .map(|m| format!("{}", m.manager))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            })
            .collect();
        self.print_row("安装方式", install_methods.iter().map(|s| s.as_str()).collect());

        // 标签
        let tags: Vec<String> = tools.iter()
            .map(|t| t.tags.join(", "))
            .collect();
        self.print_row("标签", tags.iter().map(|s| s.as_str()).collect());

        // 详细模型对比
        println!("\n{}", style("可用模型详情").bold());
        println!("{}", "─".repeat(80));

        for tool in tools {
            if let Some(pricing) = &tool.pricing {
                println!("\n{} {}", style("▸").cyan(), style(&tool.name).bold());

                if !pricing.free_models.is_empty() {
                    println!("  {}", style("免费模型:").green());
                    for model in &pricing.free_models {
                        let pro_badge = if model.pro_grade { 
                            style(" [专业级]").yellow().to_string() 
                        } else { 
                            String::new() 
                        };
                        let desc = model.description.as_deref().unwrap_or("");
                        println!("    • {}{} {}", model.name, pro_badge, style(desc).dim());
                    }
                }

                if !pricing.paid_models.is_empty() {
                    println!("  {}", style("付费模型:").yellow());
                    for model in &pricing.paid_models {
                        let pro_badge = if model.pro_grade { 
                            style(" [专业级]").yellow().to_string() 
                        } else { 
                            String::new() 
                        };
                        let desc = model.description.as_deref().unwrap_or("");
                        println!("    • {}{} {}", model.name, pro_badge, style(desc).dim());
                    }
                }
            }
        }

        println!("{}", "═".repeat(80));
    }

    /// 打印表格行
    fn print_row(&self, label: &str, values: Vec<&str>) {
        let values_str: String = values.iter()
            .map(|v| format!("{:<18}", v))
            .collect::<Vec<_>>()
            .join("│");
        println!("{:<12}│{}", style(label).dim(), values_str);
    }
}
