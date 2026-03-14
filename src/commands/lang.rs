//! lang 命令实现 - 显示或设置语言

use crate::i18n::{self, Language};
use crate::core::ConfigManager;
use anyhow::{bail, Result};
use console::style;

/// lang 命令
pub struct LangCommand {
    lang: Option<String>,
}

impl LangCommand {
    pub fn new(lang: Option<String>) -> Self {
        Self { lang }
    }

    pub fn execute(&self) -> Result<()> {
        match &self.lang {
            None => {
                // 显示当前语言
                let current = i18n::current_lang();
                println!("{}", style("Language Settings").cyan().bold());
                println!();
                println!("  Current: {} ({})", style(current.display_name()).green(), current.as_str());
                println!();
                println!("Available languages:");
                println!("  {} en - English", if current == Language::En { style("✓").green() } else { style("○").dim() });
                println!("  {} zh - 中文", if current == Language::Zh { style("✓").green() } else { style("○").dim() });
                println!();
                println!("Change language: {}", style("vcm lang <en|zh>").cyan());
                println!("Set environment: {}", style("export VCM_LANG=zh").dim());
            }
            Some(lang_str) => {
                // 设置语言
                let new_lang = Language::from_str(lang_str)
                    .ok_or_else(|| anyhow::anyhow!("Unknown language: {}. Supported: en, zh", lang_str))?;

                // 保存到配置
                let config_manager = ConfigManager::new()?;
                let mut config = config_manager.load_config()?;
                config.settings.language = Some(new_lang.as_str().to_string());
                config_manager.save_config(&config)?;

                // 更新当前会话
                i18n::set_lang(new_lang);

                println!("{} Language set to: {}", style("✓").green(), style(new_lang.display_name()).cyan());
                println!();
                
                // 用新语言显示确认
                match new_lang {
                    Language::En => println!("Restart your terminal or run `source ~/.bashrc` to apply changes."),
                    Language::Zh => println!("重启终端或运行 `source ~/.bashrc` 使更改生效。"),
                }
            }
        }

        Ok(())
    }
}
