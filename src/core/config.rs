//! 配置管理

use crate::models::*;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// 配置管理器
pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vcm");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .with_context(|| format!("无法创建配置目录: {:?}", config_dir))?;
        }

        Ok(Self { config_dir })
    }

    /// 配置目录
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    /// 加载配置
    pub fn load_config(&self) -> Result<VcmConfig> {
        let config_path = self.config_dir.join("config.toml");

        if !config_path.exists() {
            return Ok(VcmConfig::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| "无法读取配置文件")?;

        toml::from_str(&content)
            .with_context(|| "解析配置文件失败")
    }

    /// 保存配置
    pub fn save_config(&self, config: &VcmConfig) -> Result<()> {
        let config_path = self.config_dir.join("config.toml");

        let content = toml::to_string_pretty(config)
            .with_context(|| "序列化配置失败")?;

        fs::write(&config_path, content)
            .with_context(|| "无法写入配置文件")?;

        Ok(())
    }

    /// 加载状态
    pub fn load_state(&self) -> Result<VcmState> {
        let state_path = self.config_dir.join("state.json");

        if !state_path.exists() {
            return Ok(VcmState::default());
        }

        let content = fs::read_to_string(&state_path)
            .with_context(|| "无法读取状态文件")?;

        serde_json::from_str(&content)
            .with_context(|| "解析状态文件失败")
    }

    /// 保存状态
    pub fn save_state(&self, state: &VcmState) -> Result<()> {
        let state_path = self.config_dir.join("state.json");

        let content = serde_json::to_string_pretty(state)
            .with_context(|| "序列化状态失败")?;

        fs::write(&state_path, content)
            .with_context(|| "无法写入状态文件")?;

        Ok(())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("无法创建配置管理器")
    }
}