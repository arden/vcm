//! cargo 后端

use crate::models::installer::Backend;
use anyhow::{bail, Result};

pub struct CargoBackend;

impl Backend for CargoBackend {
    fn is_available(&self) -> bool {
        which::which("cargo").is_ok()
    }

    fn name(&self) -> &str {
        "cargo"
    }

    fn install(&self, package: &str, _version: Option<&str>) -> Result<()> {
        let status = std::process::Command::new("cargo")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("cargo install 失败");
        }
        Ok(())
    }

    fn update(&self, package: &str) -> Result<()> {
        // cargo 没有直接的更新命令，需要重新安装
        let status = std::process::Command::new("cargo")
            .args(["install", "--force", package])
            .status()?;

        if !status.success() {
            bail!("cargo update 失败");
        }
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("cargo")
            .args(["uninstall", package])
            .status()?;

        if !status.success() {
            bail!("cargo uninstall 失败");
        }
        Ok(())
    }

    fn is_installed(&self, package: &str) -> bool {
        which::which(package).is_ok()
    }

    fn get_version(&self, package: &str) -> Option<String> {
        let output = std::process::Command::new(package)
            .arg("--version")
            .output()
            .ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // 提取版本号
            for word in stdout.split_whitespace() {
                if word.contains('.') {
                    return Some(word.to_string());
                }
            }
        }
        None
    }
}
