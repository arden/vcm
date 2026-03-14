//! info 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use anyhow::{bail, Result};
use console::style;

/// info 命令
pub struct InfoCommand {
    tool: String,
}

impl InfoCommand {
    pub fn new(tool: String) -> Self {
        Self { tool }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;
        let discovery = Discovery::new(registry);

        // 查找工具
        let tool = discovery.registry().find_by_id(&self.tool)
            .or_else(|| discovery.registry().find_by_name(&self.tool).first().copied());

        let tool = match tool {
            Some(t) => t,
            None => bail!("未找到工具: {}", self.tool),
        };

        // 检查安装状态
        let installed = discovery.check_tool_installed(tool);

        self.print_tool_info(tool, installed.as_ref())?;

        Ok(())
    }

    fn print_tool_info(&self, tool: &Tool, installed: Option<&InstalledTool>) -> Result<()> {
        let width = 60;

        // 顶部边框
        println!("┌{}┐", "─".repeat(width));

        // 标题
        let title = format!("{} {:<40} {}",
            tool.name,
            "",
            tool.vendor
        );
        println!("│{:<60}│", title);

        // 分隔线
        println!("├{}┤", "─".repeat(width));

        // 描述
        for line in tool.description.lines() {
            println!("│ {:<59}│", line);
        }
        println!("│ {:<59}│", "");

        // 安装状态
        if let Some(inst) = installed {
            println!("│ {:<59}│", format!("版本: {} (已安装)", 
                inst.version.as_deref().unwrap_or("未知")));
            println!("│ {:<59}│", format!("路径: {}", inst.path));
            
            if let Some(ref method) = inst.install_method {
                println!("│ {:<59}│", format!("安装方式: {}", method));
            }
        } else {
            println!("│ {:<59}│", style("状态: 未安装").yellow());
        }

        // 配置路径
        if !tool.config_paths.is_empty() {
            println!("│ {:<59}│", "配置路径:");
            for path in &tool.config_paths {
                println!("│   {:<57}│", path);
            }
        }

        // 环境变量
        if !tool.env_vars.is_empty() {
            println!("│ {:<59}│", "环境变量:");
            for env_var in &tool.env_vars {
                let configured = std::env::var(&env_var.name).is_ok();
                let status = if configured {
                    style("✓ 已配置").green()
                } else if env_var.required {
                    style("○ 未配置").yellow()
                } else {
                    style("○ 未配置").dim()
                };

                println!("│   {} {:<20} {}│",
                    if env_var.required { "*" } else { " " },
                    env_var.name,
                    status
                );
            }
        }

        // 安装命令
        println!("│ {:<59}│", "");
        println!("│ {:<59}│", "安装方式:");
        for method in &tool.install_methods {
            let cmd = match method.manager {
                PackageManager::Npm => format!("npm install -g {}", method.package),
                PackageManager::Pip => format!("pip install {}", method.package),
                PackageManager::Pipx => format!("pipx install {}", method.package),
                PackageManager::Cargo => format!("cargo install {}", method.package),
                PackageManager::Brew => format!("brew install {}", method.package),
                PackageManager::Go => format!("go install {}", method.package),
                _ => method.package.clone(),
            };
            println!("│   {:<57}│", cmd);
        }

        // 链接
        println!("│ {:<59}│", "");
        if let Some(ref website) = tool.website {
            println!("│   官网: {:<48}│", website);
        }
        if let Some(ref repo) = tool.repository {
            println!("│   仓库: {:<48}│", repo);
        }

        // 标签
        if !tool.tags.is_empty() {
            println!("│ {:<59}│", "");
            println!("│   标签: {}│", tool.tags.join(", "));
        }

        // 底部边框
        println!("└{}┘", "─".repeat(width));

        Ok(())
    }
}
