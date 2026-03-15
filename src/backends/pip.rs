//! pip 后端

use crate::models::installer::Backend;
use crate::i18n::translate;
use anyhow::{bail, Result};

pub struct PipBackend;

impl Backend for PipBackend {
    fn is_available(&self) -> bool {
        which::which("pip").is_ok() || which::which("pip3").is_ok()
    }

    fn name(&self) -> &str {
        "pip"
    }

    fn install(&self, package: &str, _version: Option<&str>) -> Result<()> {
        let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
        
        let status = std::process::Command::new(pip)
            .args(["install", package])
            .status()?;

        if !status.success() {
            bail!("{}", translate("backend.pip_install_failed"));
        }
        Ok(())
    }

    fn update(&self, package: &str) -> Result<()> {
        let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
        
        let status = std::process::Command::new(pip)
            .args(["install", "--upgrade", package])
            .status()?;

        if !status.success() {
            bail!("{}", translate("backend.pip_update_failed"));
        }
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<()> {
        let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
        
        let status = std::process::Command::new(pip)
            .args(["uninstall", "-y", package])
            .status()?;

        if !status.success() {
            bail!("{}", translate("backend.pip_remove_failed"));
        }
        Ok(())
    }

    fn is_installed(&self, package: &str) -> bool {
        let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
        
        let output = std::process::Command::new(pip)
            .args(["show", package])
            .output();

        match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }

    fn get_version(&self, package: &str) -> Option<String> {
        let pip = if which::which("pip3").is_ok() { "pip3" } else { "pip" };
        
        let output = std::process::Command::new(pip)
            .args(["show", package])
            .output()
            .ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(stripped) = line.strip_prefix("Version:") {
                return Some(stripped.trim().to_string());
            }
        }
        None
    }
}
