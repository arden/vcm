//! 工具发现引擎

use crate::models::*;
use super::registry::Registry;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

/// 工具发现器
pub struct Discovery {
    registry: Registry,
}

impl Discovery {
    /// 创建新的发现器
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    /// 扫描所有已安装的工具
    pub fn scan(&self) -> Vec<InstalledTool> {
        // 使用并行迭代
        self.registry.tools.iter()
            .filter_map(|tool| self.check_tool_installed(tool))
            .collect()
    }

    /// 检查单个工具是否安装
    pub fn check_tool_installed(&self, tool: &Tool) -> Option<InstalledTool> {
        // 遍历可能的可执行文件名
        for executable in &tool.executables {
            if let Ok(path) = which::which(executable) {
                let version = self.get_version_with_timeout(executable);
                let config_exists = self.check_config_exists(tool);
                let (is_configured, missing_env_vars) = self.check_configured(tool);
                let install_method = self.detect_install_method(&path);

                return Some(InstalledTool {
                    tool_id: tool.id.clone(),
                    tool_name: tool.name.clone(),
                    executable: executable.clone(),
                    version,
                    path: path.to_string_lossy().to_string(),
                    install_method,
                    config_exists,
                    is_configured,
                    missing_env_vars,
                });
            }
        }
        None
    }

    /// 带超时的版本检测
    fn get_version_with_timeout(&self, executable: &str) -> Option<String> {
        let version_flags = ["--version", "-V", "version"];

        for flag in &version_flags {
            // 使用 spawn + wait_with_output + timeout
            let result = Command::new(executable)
                .arg(flag)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .ok()?;

            // 等待最多 2 秒
            match result.wait_with_output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    // 尝试从 stdout 或 stderr 提取版本
                    let combined = format!("{}\n{}", stdout, stderr);
                    if let Some(version) = extract_version(&combined) {
                        return Some(version);
                    }
                }
                Err(_) => continue,
            }
        }

        None
    }

    /// 检查配置文件是否存在
    fn check_config_exists(&self, tool: &Tool) -> bool {
        let home = dirs::home_dir().unwrap_or_default();

        for config_path in &tool.config_paths {
            let path = if config_path.starts_with('~') {
                PathBuf::from(&home).join(config_path.strip_prefix('~').unwrap_or(config_path))
            } else {
                PathBuf::from(config_path)
            };

            if path.exists() {
                return true;
            }
        }
        false
    }

    /// 检查是否已配置（检查环境变量）
    fn check_configured(&self, tool: &Tool) -> (bool, Vec<String>) {
        let mut missing = Vec::new();

        for env_var in &tool.env_vars {
            if env_var.required && std::env::var(&env_var.name).is_err() {
                missing.push(env_var.name.clone());
            }
        }

        (missing.is_empty(), missing)
    }

    /// 检测安装方式
    fn detect_install_method(&self, path: &PathBuf) -> Option<PackageManager> {
        let path_str = path.to_string_lossy();

        // 检查 npm
        if path_str.contains("node_modules") || path_str.contains("npm") {
            return Some(PackageManager::Npm);
        }

        // 检查 pip/pipx
        if path_str.contains(".local/bin") || path_str.contains("site-packages") {
            return Some(PackageManager::Pip);
        }

        // 检查 cargo
        if path_str.contains(".cargo/bin") {
            return Some(PackageManager::Cargo);
        }

        // 检查 brew
        if path_str.contains("/homebrew/") || path_str.contains("/brew/") {
            return Some(PackageManager::Brew);
        }

        None
    }

    /// 获取注册表引用
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

/// 从字符串中提取版本号
pub fn extract_version(output: &str) -> Option<String> {
    // 简单版本提取：查找类似 X.Y.Z 的模式
    for line in output.lines().take(5) {
        let line = line.trim();
        
        // 跳过空行
        if line.is_empty() {
            continue;
        }
        
        // 尝试找到版本号模式
        let version = find_version_pattern(line);
        if version.is_some() {
            return version;
        }
    }
    None
}

/// 查找版本号模式
fn find_version_pattern(s: &str) -> Option<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        // 查找数字开始
        if chars[i].is_ascii_digit() {
            let start = i;
            let mut dot_count = 0;
            
            // 继续读取版本号
            while i < chars.len() {
                if chars[i].is_ascii_digit() {
                    i += 1;
                } else if chars[i] == '.' {
                    dot_count += 1;
                    i += 1;
                } else if chars[i] == '-' {
                    // 支持如 1.0.0-beta 格式
                    i += 1;
                    // 继续读取预发布标识
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '.') {
                        i += 1;
                    }
                    break;
                } else {
                    break;
                }
            }
            
            // 至少有一个点才算版本号
            if dot_count >= 1 && i > start + 2 {
                return Some(chars[start..i].iter().collect());
            }
        } else {
            i += 1;
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_version_pattern() {
        assert_eq!(find_version_pattern("v1.0.0"), Some("1.0.0".to_string()));
        assert_eq!(find_version_pattern("version 2.5.3"), Some("2.5.3".to_string()));
        assert_eq!(find_version_pattern("1.0"), Some("1.0".to_string()));
        assert_eq!(find_version_pattern("no version here"), None);
        assert_eq!(find_version_pattern("v1.0.0-beta.1"), Some("1.0.0-beta.1".to_string()));
    }
}