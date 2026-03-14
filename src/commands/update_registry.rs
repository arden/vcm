//! update-registry 命令实现

use crate::core::Registry;
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
                println!("当前缓存: {} (更新于 {})", 
                    style("有效").green(), 
                    style(age).dim()
                );
            }
        } else {
            println!("当前缓存: {}", style("无").dim());
        }

        println!();

        // 更新注册表
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(Registry::update_from_remote(self.url.as_deref()))?;

        // 显示更新后的信息
        let registry = Registry::load()?;
        println!("\n已加载 {} 个工具", style(registry.len()).cyan().bold());

        Ok(())
    }
}
