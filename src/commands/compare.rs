//! 工具对比命令

use crate::core::Registry;
use crate::i18n::translate;
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
            bail!("{}", translate("compare.min_tools"));
        }

        if self.tools.len() > 5 {
            bail!("{}", translate("compare.max_tools"));
        }

        let registry = Registry::load()?;
        let mut tools_to_compare: Vec<&Tool> = Vec::new();

        // 查找所有工具
        for tool_id in &self.tools {
            let tool = registry.find_by_id(tool_id)
                .or_else(|| registry.find_by_name(tool_id).first().copied());

            match tool {
                Some(t) => tools_to_compare.push(t),
                None => bail!("{}", translate("compare.tool_not_found").replace("{}", tool_id)),
            }
        }

        // 打印对比表格
        self.print_comparison(&tools_to_compare);

        Ok(())
    }

    fn print_comparison(&self, tools: &[&Tool]) {
        println!("\n{}", style(translate("compare.title")).bold());
        println!("{}", "═".repeat(80));

        // 表头
        let header: String = tools.iter()
            .map(|t| format!("{:<18}", t.name))
            .collect::<Vec<_>>()
            .join("│");
        println!("{:<12}│{}", style(translate("compare.feature")).bold(), style(header).bold());
        println!("{}", "─".repeat(80));

        // 供应商
        self.print_row(&translate("compare.vendor"), tools.iter().map(|t| t.vendor.as_str()).collect());

        // 描述
        let descriptions: Vec<String> = tools.iter()
            .map(|t| {
                let desc = &t.description;
                let chars: Vec<char> = desc.chars().collect();
                if chars.len() > 16 {
                    format!("{}...", chars[..13].iter().collect::<String>())
                } else {
                    desc.clone()
                }
            })
            .collect();
        self.print_row(&translate("label.note"), descriptions.iter().map(|s| s.as_str()).collect());

        // 是否 CLI
        self.print_row("CLI", tools.iter().map(|t| if t.is_cli { "✓" } else { "✗" }).collect());

        // 定价信息
        let pricing_info: Vec<String> = tools.iter()
            .map(|t| {
                match &t.pricing {
                    Some(p) => {
                        if p.free_tier {
                            let free_default = translate("pricing.free");
                            let limit = p.free_limit.as_deref().unwrap_or(&free_default);
                            let chars: Vec<char> = limit.chars().collect();
                            if chars.len() > 16 {
                                format!("{}...", chars[..13].iter().collect::<String>())
                            } else {
                                limit.to_string()
                            }
                        } else {
                            translate("pricing.paid")
                        }
                    }
                    None => translate("msg.unknown")
                }
            })
            .collect();
        self.print_row(&translate("compare.free_quota"), pricing_info.iter().map(|s| s.as_str()).collect());

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
        self.print_row(&translate("compare.pro_models"), pro_models.iter().map(|s| s.as_str()).collect());

        // 需要信用卡
        let cc_required: Vec<String> = tools.iter()
            .map(|t| {
                match &t.pricing {
                    Some(p) => if p.credit_card_required { translate("msg.yes") } else { translate("msg.no") }
                    None => translate("msg.unknown")
                }
            })
            .collect();
        self.print_row(&translate("compare.card_required"), cc_required.iter().map(|s| s.as_str()).collect());

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
        self.print_row(&translate("compare.install_method"), install_methods.iter().map(|s| s.as_str()).collect());

        // 标签
        let tags: Vec<String> = tools.iter()
            .map(|t| t.tags.join(", "))
            .collect();
        self.print_row(&translate("compare.tags"), tags.iter().map(|s| s.as_str()).collect());

        // 详细模型对比
        println!("\n{}", style(translate("compare.model_details")).bold());
        println!("{}", "─".repeat(80));

        for tool in tools {
            if let Some(pricing) = &tool.pricing {
                println!("\n{} {}", style("▸").cyan(), style(&tool.name).bold());

                if !pricing.free_models.is_empty() {
                    println!("  {}", style(format!("{}:", translate("compare.free_models"))).green());
                    for model in &pricing.free_models {
                        let pro_badge = if model.pro_grade { 
                            format!(" [{}]", translate("compare.pro_grade"))
                        } else { 
                            String::new() 
                        };
                        let desc = model.description.as_deref().unwrap_or("");
                        println!("    • {}{} {}", model.name, style(pro_badge).yellow(), style(desc).dim());
                    }
                }

                if !pricing.paid_models.is_empty() {
                    println!("  {}", style(format!("{}:", translate("compare.paid_models"))).yellow());
                    for model in &pricing.paid_models {
                        let pro_badge = if model.pro_grade { 
                            format!(" [{}]", translate("compare.pro_grade"))
                        } else { 
                            String::new() 
                        };
                        let desc = model.description.as_deref().unwrap_or("");
                        println!("    • {}{} {}", model.name, style(pro_badge).yellow(), style(desc).dim());
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