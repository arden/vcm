//! update 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use anyhow::{bail, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

/// update 命令
pub struct UpdateCommand {
    tool: Option<String>,
}

impl UpdateCommand {
    pub fn new(tool: Option<String>) -> Self {
        Self { tool }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;
        let discovery = Discovery::new(registry);

        match &self.tool {
            Some(tool_id) => {
                // 更新单个工具
                self.update_single_tool(&discovery, tool_id)?;
            }
            None => {
                // 更新所有已安装的工具
                self.update_all_tools(&discovery)?;
            }
        }

        Ok(())
    }

    fn update_single_tool(&self, discovery: &Discovery, tool_id: &str) -> Result<()> {
        // 查找工具
        let tool = discovery.registry().find_by_id(tool_id)
            .or_else(|| discovery.registry().find_by_name(tool_id).first().copied());

        let tool = match tool {
            Some(t) => t,
            None => bail!("未找到工具: {}", tool_id),
        };

        // 检查是否已安装
        if !tool.is_installed() {
            bail!("工具 {} 未安装，请先安装", tool.name);
        }

        // 检测安装方式
        let installed = discovery.check_tool_installed(tool);
        let install_method = installed.as_ref()
            .and_then(|i| i.install_method.clone());

        println!("{} 更新 {}...\n", style("📦").dim(), style(&tool.name).cyan().bold());

        // 执行更新
        if let Some(method) = install_method {
            self.do_update(&method, &get_package_name(tool, &method))?;
            println!("{} 更新完成", style("✓").green());
        } else {
            // 尝试所有安装方法
            for install_method in &tool.install_methods {
                if self.do_update(&install_method.manager, &install_method.package).is_ok() {
                    println!("{} 更新完成", style("✓").green());
                    return Ok(());
                }
            }
            bail!("无法确定更新方式，请手动更新");
        }

        Ok(())
    }

    fn update_all_tools(&self, discovery: &Discovery) -> Result<()> {
        let installed = discovery.scan();

        if installed.is_empty() {
            println!("没有已安装的工具需要更新");
            return Ok(());
        }

        println!("{} 更新所有已安装工具...\n", style("📦").dim());
        println!("发现 {} 个已安装工具\n", installed.len());

        let mut success = 0;
        let mut failed = 0;

        for tool_info in &installed {
            if let Some(tool) = discovery.registry().find_by_id(&tool_info.tool_id) {
                print!("  更新 {}... ", style(&tool.name).bold());

                if let Some(ref method) = tool_info.install_method {
                    match self.do_update_silent(method, &get_package_name(tool, method)) {
                        Ok(_) => {
                            println!("{}", style("✓").green());
                            success += 1;
                        }
                        Err(_) => {
                            println!("{}", style("✗ 失败").red());
                            failed += 1;
                        }
                    }
                } else {
                    println!("{}", style("○ 跳过").dim());
                }
            }
        }

        println!("\n更新完成: {} 成功, {} 失败", 
            style(success).green(),
            style(failed).red()
        );

        Ok(())
    }

    fn do_update(&self, manager: &PackageManager, package: &str) -> Result<()> {
        let progress = ProgressBar::new_spinner();
        progress.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}")?);
        progress.enable_steady_tick(std::time::Duration::from_millis(100));
        progress.set_message("正在更新...");

        let result = self.do_update_silent(manager, package);

        progress.finish();
        result
    }

    fn do_update_silent(&self, manager: &PackageManager, package: &str) -> Result<()> {
        let status = match manager {
            PackageManager::Npm => {
                std::process::Command::new("npm")
                    .args(["update", "-g", package])
                    .status()?
            }
            PackageManager::Pip => {
                let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
                std::process::Command::new(pip)
                    .args(["install", "--upgrade", package])
                    .status()?
            }
            PackageManager::Pipx => {
                std::process::Command::new("pipx")
                    .args(["upgrade", package])
                    .status()?
            }
            PackageManager::Cargo => {
                std::process::Command::new("cargo")
                    .args(["install", "--force", package])
                    .status()?
            }
            PackageManager::Brew => {
                std::process::Command::new("brew")
                    .args(["upgrade", package])
                    .status()?
            }
            PackageManager::Go => {
                std::process::Command::new("go")
                    .args(["install", package])
                    .status()?
            }
            _ => bail!("暂不支持的包管理器"),
        };

        if !status.success() {
            bail!("更新失败");
        }
        Ok(())
    }
}

/// 获取包名
fn get_package_name(tool: &Tool, manager: &PackageManager) -> String {
    tool.install_methods.iter()
        .find(|m| m.manager == *manager)
        .map(|m| m.package.clone())
        .unwrap_or_else(|| tool.id.clone())
}
