//! update-registry 命令实现

use crate::core::Registry;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// update-registry 命令
pub struct UpdateRegistryCommand {
    url: Option<String>,
}

impl UpdateRegistryCommand {
    pub fn new(url: Option<String>) -> Self {
        Self { url }
    }

    pub fn execute(&self) -> Result<()> {
        // 显示当前缓存状态
        let (has_cache, cache_age) = Registry::cache_status();
        
        if has_cache {
            if let Some(age) = &cache_age {
                println!("{}: {} ({})", 
                    translate("registry.current_cache").split(':').next().unwrap_or("Current cache"),
                    style(translate("registry.valid")).green(),
                    style(format!("{} {}", age, translate("registry.updated_at").split(' ').next().unwrap_or(""))).dim()
                );
            }
        } else {
            println!("{}: {}", translate("registry.current_cache").split(':').next().unwrap_or("Current cache"), style(translate("registry.none")).dim());
        }

        println!();

        // 更新注册表
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(Registry::update_from_remote(self.url.as_deref()))?;

        // 显示更新后的信息
        let registry = Registry::load()?;
        println!("\n{}", translate("registry.loaded").replace("{}", &style(registry.len()).cyan().bold().to_string()));

        Ok(())
    }
}
