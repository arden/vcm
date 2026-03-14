//! 配置模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// VCM 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcmState {
    /// 已安装工具
    #[serde(default)]
    pub installed_tools: HashMap<String, InstalledToolState>,
    /// 上次扫描时间
    #[serde(default)]
    pub last_scan: Option<String>,
    /// 注册表版本
    #[serde(default)]
    pub registry_version: Option<String>,
}

impl Default for VcmState {
    fn default() -> Self {
        Self {
            installed_tools: HashMap::new(),
            last_scan: None,
            registry_version: None,
        }
    }
}

/// 已安装工具状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledToolState {
    /// 版本
    pub version: Option<String>,
    /// 安装时间
    pub installed_at: Option<String>,
    /// 安装方式
    pub install_method: Option<String>,
}
