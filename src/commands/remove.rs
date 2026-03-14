//! remove 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use anyhow::{bail, Result};
use console::style;
use dialoguer::Confirm;

/// remove 命令
pub struct RemoveCommand {
    tool: String,
    force: bool,
}

impl RemoveCommand {
    pub fn new(tool: String, force: bool) -> Self {
        Self { tool, force }
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

        // 检查是否已安装
        if !tool.is_installed() {
            bail!("工具 {} 未安装", tool.name);
        }

        // 获取安装信息
        let installed = discovery.check_tool_installed(tool);
        let install_method = installed.as_ref()
            .and_then(|i| i.install_method.clone());

        // 确认卸载
        if !self.force {
            let confirm = Confirm::new()
                .with_prompt(format!("确定要卸载 {} 吗?", style(&tool.name).cyan()))
                .default(false)
                .interact()?;

            if !confirm {
                println!("已取消");
                return Ok(());
            }
        }

        println!("{} 卸载 {}...\n", style("🗑️").dim(), style(&tool.name).cyan().bold());

        // 执行卸载
        if let Some(method) = install_method {
            let package = get_package_name(tool, &method);
            self.do_remove(&method, &package)?;
            println!("{} 已卸载 {}", style("✓").green(), tool.name);
        } else {
            // 尝试所有安装方法
            let mut removed = false;
            for install_method in &tool.install_methods {
                if self.do_remove_silent(&install_method.manager, &install_method.package).is_ok() {
                    println!("{} 已卸载 {}", style("✓").green(), tool.name);
                    removed = true;
                    break;
                }
            }
            if !removed {
                bail!("无法确定卸载方式，请手动卸载");
            }
        }

        Ok(())
    }

    fn do_remove(&self, manager: &PackageManager, package: &str) -> Result<()> {
        let result = self.do_remove_silent(manager, package);
        result
    }

    fn do_remove_silent(&self, manager: &PackageManager, package: &str) -> Result<()> {
        let status = match manager {
            PackageManager::Npm => {
                std::process::Command::new("npm")
                    .args(["uninstall", "-g", package])
                    .status()?
            }
            PackageManager::Pip => {
                let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
                std::process::Command::new(pip)
                    .args(["uninstall", "-y", package])
                    .status()?
            }
            PackageManager::Pipx => {
                std::process::Command::new("pipx")
                    .args(["uninstall", package])
                    .status()?
            }
            PackageManager::Cargo => {
                std::process::Command::new("cargo")
                    .args(["uninstall", package])
                    .status()?
            }
            PackageManager::Brew => {
                std::process::Command::new("brew")
                    .args(["uninstall", package])
                    .status()?
            }
            _ => bail!("暂不支持的包管理器"),
        };

        if !status.success() {
            bail!("卸载失败");
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
