//! 安装器模型

use serde::{Deserialize, Serialize};

/// 安装器后端 trait
pub trait Backend: Send + Sync {
    /// 检查后端是否可用
    fn is_available(&self) -> bool;

    /// 获取后端名称
    fn name(&self) -> &str;

    /// 安装包
    fn install(&self, package: &str, version: Option<&str>) -> anyhow::Result<()>;

    /// 更新包
    fn update(&self, package: &str) -> anyhow::Result<()>;

    /// 卸载包
    fn remove(&self, package: &str) -> anyhow::Result<()>;

    /// 检查是否已安装
    fn is_installed(&self, package: &str) -> bool;

    /// 获取已安装版本
    fn get_version(&self, package: &str) -> Option<String>;
}

/// 安装结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    /// 是否成功
    pub success: bool,
    /// 包名
    pub package: String,
    /// 版本
    pub version: Option<String>,
    /// 消息
    pub message: String,
}
