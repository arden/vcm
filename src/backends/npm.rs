//! npm 后端

use crate::models::installer::Backend;
use anyhow::{bail, Result};

pub struct NpmBackend;

impl Backend for NpmBackend {
    fn is_available(&self) -> bool {
        which::which("npm").is_ok()
    }

    fn name(&self) -> &str {
        "npm"
    }

    fn install(&self, package: &str, _version: Option<&str>) -> Result<()> {
        let status = std::process::Command::new("npm")
            .args(["install", "-g", package])
            .status()?;

        if !status.success() {
            bail!("npm install 失败");
        }
        Ok(())
    }

    fn update(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("npm")
            .args(["update", "-g", package])
            .status()?;

        if !status.success() {
            bail!("npm update 失败");
        }
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<()> {
        let status = std::process::Command::new("npm")
            .args(["uninstall", "-g", package])
            .status()?;

        if !status.success() {
            bail!("npm uninstall 失败");
        }
        Ok(())
    }

    fn is_installed(&self, package: &str) -> bool {
        let output = std::process::Command::new("npm")
            .args(["list", "-g", "--depth=0"])
            .output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                stdout.contains(package)
            }
            Err(_) => false,
        }
    }

    fn get_version(&self, package: &str) -> Option<String> {
        let output = std::process::Command::new("npm")
            .args(["list", "-g", package, "--depth=0"])
            .output()
            .ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // 解析版本: package@1.0.0
        for line in stdout.lines() {
            if line.contains(package) {
                if let Some(idx) = line.find('@') {
                    let version = &line[idx + 1..];
                    return Some(version.trim().to_string());
                }
            }
        }
        None
    }
}
