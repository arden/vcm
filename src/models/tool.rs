//! 工具定义模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// 工具ID（唯一标识）
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 供应商
    pub vendor: String,
    /// 描述
    pub description: String,
    /// 官网
    #[serde(default)]
    pub website: Option<String>,
    /// 仓库地址
    #[serde(default)]
    pub repository: Option<String>,
    /// 安装方法列表
    #[serde(default)]
    pub install_methods: Vec<InstallMethod>,
    /// 可执行命令列表（用于检测）
    #[serde(default)]
    pub executables: Vec<String>,
    /// 配置文件路径
    #[serde(default)]
    pub config_paths: Vec<String>,
    /// 需要的环境变量
    #[serde(default)]
    pub env_vars: Vec<EnvVar>,
    /// 标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 是否为 CLI 工具
    #[serde(default = "default_true")]
    pub is_cli: bool,
    /// 是否为 GUI 工具
    #[serde(default)]
    pub is_gui: bool,
    /// 是否为推荐工具
    #[serde(default)]
    pub featured: bool,
    /// 定价信息
    #[serde(default)]
    pub pricing: Option<Pricing>,
}

/// 定价信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    /// 是否有免费额度
    #[serde(default)]
    pub free_tier: bool,
    /// 免费额度描述（如 "100 requests/day"）
    #[serde(default)]
    pub free_limit: Option<String>,
    /// 免费可用的模型列表
    #[serde(default)]
    pub free_models: Vec<ModelInfo>,
    /// 付费可用的模型列表
    #[serde(default)]
    pub paid_models: Vec<ModelInfo>,
    /// 是否需要信用卡
    #[serde(default)]
    pub credit_card_required: bool,
    /// 价格说明
    #[serde(default)]
    pub price_note: Option<String>,
}

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// 模型名称
    pub name: String,
    /// 模型描述（如 SWE-bench 分数）
    #[serde(default)]
    pub description: Option<String>,
    /// 是否为专业级模型（SWE-bench >= 60%）
    #[serde(default)]
    pub pro_grade: bool,
}

impl Pricing {
    /// 是否有免费的专业级模型
    pub fn has_free_pro_models(&self) -> bool {
        self.free_models.iter().any(|m| m.pro_grade)
    }
    
    /// 获取免费的专业级模型列表
    pub fn free_pro_models(&self) -> Vec<&ModelInfo> {
        self.free_models.iter().filter(|m| m.pro_grade).collect()
    }
}

fn default_true() -> bool {
    true
}

impl Tool {
    /// 检查工具是否已安装
    pub fn is_installed(&self) -> bool {
        self.executables.iter().any(|exe| which::which(exe).is_ok())
    }

    /// 获取可执行文件路径
    pub fn executable_path(&self) -> Option<String> {
        self.executables
            .iter()
            .find_map(|exe| which::which(exe).ok())
            .map(|p| p.to_string_lossy().to_string())
    }
}

/// 安装方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMethod {
    /// 包管理器
    pub manager: PackageManager,
    /// 包名
    pub package: String,
    /// 版本约束
    #[serde(default)]
    pub version: Option<String>,
    /// 平台限制
    #[serde(default)]
    pub platforms: Option<Vec<Platform>>,
}

/// 包管理器类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PackageManager {
    Npm,
    Pip,
    Pipx,
    Brew,
    Cargo,
    Go,
    Scoop,
    Chocolatey,
    Apt,
    Dnf,
    Pacman,
    Snap,
    Binary,
    Script,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pip => write!(f, "pip"),
            PackageManager::Pipx => write!(f, "pipx"),
            PackageManager::Brew => write!(f, "brew"),
            PackageManager::Cargo => write!(f, "cargo"),
            PackageManager::Go => write!(f, "go"),
            PackageManager::Scoop => write!(f, "scoop"),
            PackageManager::Chocolatey => write!(f, "choco"),
            PackageManager::Apt => write!(f, "apt"),
            PackageManager::Dnf => write!(f, "dnf"),
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Snap => write!(f, "snap"),
            PackageManager::Binary => write!(f, "binary"),
            PackageManager::Script => write!(f, "script"),
        }
    }
}

/// 环境变量定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    /// 变量名
    pub name: String,
    /// 描述
    #[serde(default)]
    pub description: String,
    /// 是否必需
    #[serde(default)]
    pub required: bool,
    /// 获取地址
    #[serde(default)]
    pub get_url: Option<String>,
}

/// 平台类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    Macos,
    Windows,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "linux"),
            Platform::Macos => write!(f, "macos"),
            Platform::Windows => write!(f, "windows"),
        }
    }
}

/// 已安装工具信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledTool {
    /// 工具ID
    pub tool_id: String,
    /// 工具名称
    pub tool_name: String,
    /// 可执行文件
    pub executable: String,
    /// 版本
    pub version: Option<String>,
    /// 路径
    pub path: String,
    /// 安装方式
    pub install_method: Option<PackageManager>,
    /// 配置是否存在
    pub config_exists: bool,
    /// 是否已配置（API Key等）
    pub is_configured: bool,
    /// 缺失的环境变量
    pub missing_env_vars: Vec<String>,
}

/// 工具状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    /// 已安装的工具信息
    pub tool: InstalledTool,
    /// 健康状态
    pub health: HealthStatus,
    /// 建议
    pub suggestions: Vec<String>,
}

/// 健康状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Error,
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "✓ 正常"),
            HealthStatus::Warning => write!(f, "⚠ 警告"),
            HealthStatus::Error => write!(f, "✗ 错误"),
            HealthStatus::Unknown => write!(f, "? 未知"),
        }
    }
}

/// 工具注册表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegistry {
    /// 版本
    pub version: u32,
    /// 更新时间
    #[serde(default)]
    pub updated: Option<String>,
    /// 来源
    #[serde(default)]
    pub source: Option<String>,
    /// 工具列表
    pub tools: Vec<Tool>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self {
            version: 1,
            updated: None,
            source: None,
            tools: Vec::new(),
        }
    }
}

/// VCM 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcmConfig {
    /// 设置
    #[serde(default)]
    pub settings: Settings,
    /// 注册表配置
    #[serde(default)]
    pub registry: RegistryConfig,
}

impl Default for VcmConfig {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            registry: RegistryConfig::default(),
        }
    }
}

/// 设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// 默认工具
    #[serde(default)]
    pub default_tool: Option<String>,
    /// 语言 (en/zh)
    #[serde(default)]
    pub language: Option<String>,
    /// 检查更新
    #[serde(default = "default_true")]
    pub check_updates: bool,
    /// 自动更新注册表
    #[serde(default = "default_true")]
    pub auto_update_registry: bool,
}

/// 注册表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// 远程URL
    #[serde(default = "default_registry_url")]
    pub url: String,
    /// 本地路径
    #[serde(default)]
    pub local_path: Option<String>,
    /// 更新间隔（小时）
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            url: default_registry_url(),
            local_path: None,
            update_interval: default_update_interval(),
        }
    }
}

fn default_registry_url() -> String {
    "https://raw.githubusercontent.com/arden/vcm/main/registry/tools.yaml".to_string()
}

fn default_update_interval() -> u64 {
    24
}
