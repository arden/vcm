//! brew 后端

use crate::models::installer::Backend;
use anyhow::{bail, Result};

pub struct BrewBackend;

impl Backend for BrewBackend {
    fn is_available(&self) -> bool {
        which::which("brew").is_ok()
    }

    fn name(&self) -> &str {
        "brew"
    }

    fn install(&self, package: &str, _version: Option<&str>) -> Result<()> {
        let status = std::process::Command::new("brew")
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("brew install 失败");
        }
        Ok(())
    }

    fn update(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("brew")
            .args(["upgrade", package])
            .status()?;

        if !status.success() {
            bail!("brew upgrade 失败");
        }
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("brew")
            .args(["uninstall", package])
            .status()?;

        if !status.success() {
            bail!("brew uninstall 失败");
        }
        Ok(())
    }

    fn is_installed(&self, package: &str) -> bool {
        let output = std::process::Command::new("brew")
            .args(["list", "--formula"])
            .output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                stdout.lines().any(|line| line.trim() == package)
            }
            Err(_) => false,
        }
    }

    fn get_version(&self, package: &str) -> Option<String> {
        let output = std::process::Command::new("brew")
            .args(["info", "--json=v2", package])
            .output()
            .ok()?;

        if output.status.success() {
            // 简单解析 JSON 获取版本
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(idx) = stdout.find("\"version\":") {
                let rest = &stdout[idx + 10..];
                if let Some(start) = rest.find('"') {
                    let rest = &rest[start + 1..];
                    if let Some(end) = rest.find('"') {
                        return Some(rest[..end].to_string());
                    }
                }
            }
        }
        None
    }
}
