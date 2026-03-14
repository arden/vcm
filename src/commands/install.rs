//! install 命令实现

use crate::core::Registry;
use crate::models::*;
use anyhow::{bail, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

/// install 命令
pub struct InstallCommand {
    tool: String,
    manager: Option<String>,
}

impl InstallCommand {
    pub fn new(tool: String, manager: Option<String>) -> Self {
        Self { tool, manager }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;

        // 查找工具
        let tool = registry.find_by_id(&self.tool)
            .or_else(|| registry.find_by_name(&self.tool).first().copied());

        let tool = match tool {
            Some(t) => t,
            None => bail!("未找到工具: {}", self.tool),
        };

        // 检查是否已安装
        if tool.is_installed() {
            println!("{} 已经安装", style(&tool.name).green());
            return Ok(());
        }

        println!("{} 安装 {}...\n", style("📦").dim(), style(&tool.name).cyan().bold());

        // 选择安装方法
        let method = self.select_install_method(tool)?;

        println!("使用 {} 安装...", style(&method.manager).yellow());

        // 执行安装
        self.do_install(&method)?;

        println!("\n{} 安装完成: {}", style("✓").green(), &method.package);

        // 提示下一步
        if !tool.env_vars.is_empty() {
            println!("\n下一步:");
            for (i, env_var) in tool.env_vars.iter().enumerate() {
                if env_var.required {
                    println!(
                        "  {}. 获取 API Key: {}",
                        i + 1,
                        env_var.get_url.as_deref().unwrap_or("-")
                    );
                    println!(
                        "     export {}=\"your-api-key\"",
                        env_var.name
                    );
                }
            }
            println!("  {}. 配置: vcm config {}", tool.env_vars.iter().filter(|e| e.required).count() + 1, tool.id);
        }

        Ok(())
    }

    fn select_install_method<'a>(&self, tool: &'a Tool) -> Result<&'a InstallMethod> {
        // 检测可用包管理器
        let available = self.detect_available_managers();

        // 如果用户指定了管理器
        if let Some(ref manager_name) = self.manager {
            for method in &tool.install_methods {
                if method.manager.to_string() == *manager_name && available.contains(&method.manager) {
                    return Ok(method);
                }
            }
            bail!("指定的包管理器 {} 不可用或不支持此工具", manager_name);
        }

        // 自动选择第一个可用的方法
        for method in &tool.install_methods {
            if available.contains(&method.manager) {
                return Ok(method);
            }
        }

        bail!("没有找到可用的安装方法。请安装 npm、pip、pipx 或 cargo 后重试。")
    }

    fn detect_available_managers(&self) -> Vec<PackageManager> {
        let mut managers = Vec::new();

        if which::which("npm").is_ok() { managers.push(PackageManager::Npm); }
        if which::which("pip").is_ok() || which::which("pip3").is_ok() {
            managers.push(PackageManager::Pip);
        }
        if which::which("pipx").is_ok() { managers.push(PackageManager::Pipx); }
        if which::which("cargo").is_ok() { managers.push(PackageManager::Cargo); }
        if which::which("brew").is_ok() { managers.push(PackageManager::Brew); }
        if which::which("go").is_ok() { managers.push(PackageManager::Go); }

        managers
    }

    fn do_install(&self, method: &InstallMethod) -> Result<()> {
        let progress = ProgressBar::new_spinner();
        progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")?
        );
        progress.enable_steady_tick(std::time::Duration::from_millis(100));
        progress.set_message("正在安装...");

        let result = match method.manager {
            PackageManager::Npm => self.install_npm(&method.package),
            PackageManager::Pip => self.install_pip(&method.package),
            PackageManager::Pipx => self.install_pipx(&method.package),
            PackageManager::Cargo => self.install_cargo(&method.package),
            PackageManager::Brew => self.install_brew(&method.package),
            PackageManager::Go => self.install_go(&method.package),
            _ => bail!("暂不支持的包管理器"),
        };

        progress.finish();
        result
    }

    fn install_npm(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("npm")
            .args(["install", "-g", package])
            .status()?;

        if !status.success() {
            bail!("npm install 失败");
        }
        Ok(())
    }

    fn install_pip(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("pip")
            .args(["install", package])
            .status()
            .or_else(|_| {
                std::process::Command::new("pip3")
                    .args(["install", package])
                    .status()
            })?;

        if !status.success() {
            bail!("pip install 失败");
        }
        Ok(())
    }

    fn install_pipx(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("pipx")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("pipx install 失败");
        }
        Ok(())
    }

    fn install_cargo(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("cargo")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("cargo install 失败");
        }
        Ok(())
    }

    fn install_brew(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("brew")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("brew install 失败");
        }
        Ok(())
    }

    fn install_go(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("go")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("go install 失败");
        }
        Ok(())
    }
}
