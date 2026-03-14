//! search 命令实现

use crate::core::Registry;
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// search 命令
pub struct SearchCommand {
    query: String,
    json: bool,
}

impl SearchCommand {
    pub fn new(query: String, json: bool) -> Self {
        Self { query, json }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;

        let results = registry.search(&self.query);

        if results.is_empty() {
            println!("{}", translate("search.no_results").replace("{}", &self.query));
            return Ok(());
        }

        if self.json {
            self.output_json(&results)?;
        } else {
            self.output_human(&results)?;
        }

        Ok(())
    }

    fn output_json(&self, tools: &[&Tool]) -> Result<()> {
        let output: Vec<serde_json::Value> = tools.iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "name": t.name,
                    "vendor": t.vendor,
                    "description": t.description,
                    "tags": t.tags,
                })
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&output)?);
        Ok(())
    }

    fn output_human(&self, tools: &[&Tool]) -> Result<()> {
        println!("{} {}\n",
            style("🔍").dim(),
            translate("search.results")
                .replace("{}", &style(&self.query).cyan().to_string())
                .replace("{count}", &tools.len().to_string())
        );

        for tool in tools {
            println!("{} {}",
                style(&tool.id).bold(),
                style(&tool.vendor).dim()
            );
            println!("  {}", tool.description.lines().next().unwrap_or(""));
            if !tool.tags.is_empty() {
                println!("  {} {}",
                    style(format!("{}:", translate("label.tag"))).dim(),
                    style(tool.tags.join(", ")).dim()
                );
            }
            println!();
        }

        println!("{}", translate("search.install_hint").replace("{}", &style("vcm install <tool>").cyan().to_string()));

        Ok(())
    }
}
