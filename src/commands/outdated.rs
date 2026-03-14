//! outdated 命令实现 - 检查工具是否有更新

use crate::core::{Discovery, Registry};
use crate::models::*;
use crate::i18n::translate;
use anyhow::Result;
use console::style;
use std::process::Command;

/// outdated 命令
pub struct OutdatedCommand {
    json: bool,
}

impl OutdatedCommand {
    pub fn new(json: bool) -> Self {
        Self { json }
    }

    pub fn execute(&self) -> Result<()> {
        println!("{} {}\n", style("🔍").dim(), translate("outdated.checking"));

        let registry = Registry::load()?;
        let discovery = Discovery::new(Registry::load()?);
        let installed = discovery.scan();

        // 获取工具定义的辅助函数
        let get_tool_def = |tool_id: &str| -> Option<(String, Vec<InstallMethod>)> {
            registry.find_by_id(tool_id).map(|t| {
                (t.name.clone(), t.install_methods.clone())
            })
        };

        if installed.is_empty() {
            println!("{}", translate("scan.none"));
            return Ok(());
        }

        let mut outdated = Vec::new();

        for tool in &installed {
            // 获取工具定义
            if let Some((tool_name, install_methods)) = get_tool_def(&tool.tool_id) {
                // 获取最新版本
                if let Some(latest_version) = self.get_latest_version(&install_methods) {
                    if let Some(ref current_version) = tool.version {
                        if self.compare_versions(current_version, &latest_version) {
                            outdated.push(OutdatedTool {
                                tool_id: tool.tool_id.clone(),
                                tool_name,
                                current_version: current_version.clone(),
                                latest_version,
                            });
                        }
                    }
                }
            }
        }

        if outdated.is_empty() {
            println!("{} {}", style("✓").green(), translate("outdated.all_latest"));
        } else {
            println!("{}", style(translate("outdated.available")).yellow().bold());
            println!();

            for tool in &outdated {
                println!(
                    "  {} {} {} → {}",
                    style(&tool.tool_name).bold(),
                    style(&format!("v{}", tool.current_version)).dim(),
                    style("→").dim(),
                    style(&format!("v{}", tool.latest_version)).green()
                );
            }

            println!();
            println!("{}", translate("outdated.update_hint").replace("{}", &style("vcm update <tool>").cyan().to_string()));
        }

        Ok(())
    }

    /// 获取工具的最新版本
    fn get_latest_version(&self, install_methods: &[InstallMethod]) -> Option<String> {
        for method in install_methods {
            let version = match method.manager {
                PackageManager::Npm => self.get_npm_latest_version(&method.package),
                PackageManager::Cargo => self.get_cargo_latest_version(&method.package),
                PackageManager::Pip => self.get_pip_latest_version(&method.package),
                _ => None,
            };

            if version.is_some() {
                return version;
            }
        }
        None
    }

    /// 从 npm 获取最新版本
    fn get_npm_latest_version(&self, package: &str) -> Option<String> {
        let output = Command::new("npm")
            .args(["view", package, "version"])
            .output()
            .ok()?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim().to_string();
            if !version.is_empty() {
                return Some(version);
            }
        }
        None
    }

    /// 从 cargo 获取最新版本
    fn get_cargo_latest_version(&self, package: &str) -> Option<String> {
        let output = Command::new("cargo")
            .args(["search", package, "--limit", "1"])
            .output()
            .ok()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // 格式: package = "version"
            for line in output_str.lines() {
                if line.starts_with(package) {
                    if let Some(start) = line.find('"') {
                        if let Some(end) = line[start + 1..].find('"') {
                            return Some(line[start + 1..start + 1 + end].to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// 从 pip 获取最新版本
    fn get_pip_latest_version(&self, package: &str) -> Option<String> {
        let output = Command::new("pip")
            .args(["index", "versions", package])
            .output()
            .ok()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // 格式: Available versions: x.y.z, ...
            if let Some(start) = output_str.find("Available versions:") {
                let rest = &output_str[start + 20..];
                if let Some(end) = rest.find(',') {
                    return Some(rest[..end].trim().to_string());
                } else if let Some(end) = rest.find('\n') {
                    return Some(rest[..end].trim().to_string());
                }
            }
        }
        None
    }

    /// 比较版本，返回 true 表示 current < latest
    fn compare_versions(&self, current: &str, latest: &str) -> bool {
        let current_parts: Vec<u64> = self.parse_version_parts(current);
        let latest_parts: Vec<u64> = self.parse_version_parts(latest);

        for i in 0..std::cmp::max(current_parts.len(), latest_parts.len()) {
            let current_val = current_parts.get(i).unwrap_or(&0);
            let latest_val = latest_parts.get(i).unwrap_or(&0);

            if current_val < latest_val {
                return true;
            } else if current_val > latest_val {
                return false;
            }
        }
        false
    }

    /// 解析版本号
    fn parse_version_parts(&self, version: &str) -> Vec<u64> {
        version
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .next()
            .unwrap_or("0")
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect()
    }
}

/// 过期工具信息
#[derive(Debug, Clone)]
struct OutdatedTool {
    tool_id: String,
    tool_name: String,
    current_version: String,
    latest_version: String,
}
