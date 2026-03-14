//! Shell 命令执行工具

use anyhow::{Context, Result};
use std::process::{Command, Output};

/// 执行命令并获取输出
pub fn execute_capture(program: &str, args: &[&str]) -> Result<Output> {
    Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("执行命令失败: {} {:?}", program, args))
}

/// 执行命令
pub fn execute(program: &str, args: &[&str]) -> Result<bool> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("执行命令失败: {} {:?}", program, args))?;

    Ok(status.success())
}

/// 检查是否为 npm 包
pub fn is_npm_package(executable: &str) -> bool {
    if let Ok(output) = execute_capture("npm", &["list", "-g", "--depth=0"]) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains(executable);
    }
    false
}

/// 检查是否为 pip 包
pub fn is_pip_package(executable: &str) -> bool {
    if let Ok(output) = execute_capture("pip", &["show", executable]) {
        return output.status.success();
    }
    if let Ok(output) = execute_capture("pip3", &["show", executable]) {
        return output.status.success();
    }
    false
}

/// 检查是否为 cargo 包
pub fn is_cargo_package(executable: &str) -> bool {
    if let Ok(output) = execute_capture("cargo", &["install", "--list"]) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains(executable);
    }
    false
}
