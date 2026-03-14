//! update 命令实现

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
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
            None => bail!("{}", translate("tool.not_found").replace("{}", tool_id)),
        };

        // 检查是否已安装
        if !tool.is_installed() {
            bail!("{}", translate("update.not_installed").replace("{}", &tool.name));
        }

        // 检测安装方式
        let installed = discovery.check_tool_installed(tool);
        let install_method = installed.as_ref()
            .and_then(|i| i.install_method.clone());

        println!("{} {}\n", style("📦").dim(), translate("update.updating").replace("{}", &style(&tool.name).cyan().bold().to_string()));

        // 执行更新
        if let Some(method) = install_method {
            self.do_update(&method, &get_package_name(tool, &method))?;
            println!("{} {}", style("✓").green(), translate("update.updating").replace("{}", "").trim());
        } else {
            // 尝试所有安装方法
            for install_method in &tool.install_methods {
                if self.do_update(&install_method.manager, &install_method.package).is_ok() {
                    println!("{} {}", style("✓").green(), translate("update.updating").replace("{}", "").trim());
                    return Ok(());
                }
            }
            bail!("{}", translate("update.cannot_update"));
        }

        Ok(())
    }

    fn update_all_tools(&self, discovery: &Discovery) -> Result<()> {
        let installed = discovery.scan();

        if installed.is_empty() {
            println!("{}", translate("update.no_tools"));
            return Ok(());
        }

        println!("{} {}\n", style("📦").dim(), translate("update.all"));
        println!("{}\n", translate("update.found_tools").replace("{}", &installed.len().to_string()));

        let mut success = 0;
        let mut failed = 0;

        for tool_info in &installed {
            if let Some(tool) = discovery.registry().find_by_id(&tool_info.tool_id) {
                print!("  {}... ", translate("update.updating").replace("{}", &style(&tool.name).bold().to_string()));

                if let Some(ref method) = tool_info.install_method {
                    match self.do_update_silent(method, &get_package_name(tool, method)) {
                        Ok(_) => {
                            println!("{}", style("✓").green());
                            success += 1;
                        }
                        Err(_) => {
                            println!("{} {}", style("✗").red(), translate("msg.failed"));
                            failed += 1;
                        }
                    }
                } else {
                    println!("○ {}", translate("msg.skipped"));
                }
            }
        }

        println!("\n{}", translate("update.complete")
            .replace("{}", &style(success).green().to_string())
            .replace("{failed}", &style(failed).red().to_string())
        );

        Ok(())
    }

    fn do_update(&self, manager: &PackageManager, package: &str) -> Result<()> {
        let progress = ProgressBar::new_spinner();
        progress.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}")?);
        progress.enable_steady_tick(std::time::Duration::from_millis(100));
        progress.set_message(translate("update.updating_progress"));

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
            _ => bail!("{}", translate("msg.unsupported_manager")),
        };

        if !status.success() {
            bail!("{}", translate("msg.failed"));
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
