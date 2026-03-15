//! 多账号/多 Key 管理命令

use crate::core::{ConfigManager, Registry};
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Key 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeysConfig {
    /// 工具 -> 账号配置
    #[serde(default)]
    pub tools: HashMap<String, ToolKeys>,
}

/// 工具的 Key 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolKeys {
    /// 当前激活的 Key 名称
    #[serde(default)]
    pub active: Option<String>,
    /// Key 列表 (name -> key info)
    #[serde(default)]
    pub keys: HashMap<String, KeyInfo>,
    /// 轮换设置
    #[serde(default)]
    pub rotation: Option<RotationConfig>,
}

/// Key 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Key 值 (加密存储)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 是否是试用
    #[serde(default)]
    pub is_trial: bool,
    /// 过期时间
    #[serde(default)]
    pub expires: Option<String>,
    /// 添加时间
    #[serde(default)]
    pub added_at: String,
}

/// 轮换配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    /// 是否启用
    pub enabled: bool,
    /// 轮换间隔
    #[serde(default)]
    pub interval: RotationInterval,
    /// 上次轮换时间
    #[serde(default)]
    pub last_rotation: Option<String>,
}

/// 轮换间隔
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RotationInterval {
    #[default]
    PerRequest,
    Hourly,
    Daily,
    Weekly,
}

/// Key 命令
pub struct KeyCommand {
    action: KeyAction,
}

/// Key 操作类型
pub enum KeyAction {
    /// 列出所有 Key
    List { tool: Option<String> },
    /// 添加 Key
    Add { tool: String, name: String, key: String },
    /// 删除 Key
    Remove { tool: String, name: String },
    /// 切换 Key
    Switch { tool: String, name: String },
    /// 设置轮换
    Rotate { tool: String, enable: bool },
    /// 显示当前激活的 Key
    Current { tool: String },
}

impl KeyCommand {
    pub fn new(action: KeyAction) -> Self {
        Self { action }
    }

    pub fn execute(&self) -> Result<()> {
        let config_manager = ConfigManager::new()?;

        match &self.action {
            KeyAction::List { tool } => {
                self.list_keys(&config_manager, tool)?;
            }
            KeyAction::Add { tool, name, key } => {
                self.add_key(&config_manager, tool, name, key)?;
            }
            KeyAction::Remove { tool, name } => {
                self.remove_key(&config_manager, tool, name)?;
            }
            KeyAction::Switch { tool, name } => {
                self.switch_key(&config_manager, tool, name)?;
            }
            KeyAction::Rotate { tool, enable } => {
                self.toggle_rotation(&config_manager, tool, *enable)?;
            }
            KeyAction::Current { tool } => {
                self.show_current(&config_manager, tool)?;
            }
        }

        Ok(())
    }

    /// 加载 Key 配置
    fn load_keys_config(&self, config_manager: &ConfigManager) -> Result<KeysConfig> {
        let keys_path = config_manager.config_dir().join("keys.json");

        if !keys_path.exists() {
            return Ok(KeysConfig::default());
        }

        let content = std::fs::read_to_string(&keys_path)?;
        let config: KeysConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 保存 Key 配置
    fn save_keys_config(&self, config_manager: &ConfigManager, config: &KeysConfig) -> Result<()> {
        let keys_path = config_manager.config_dir().join("keys.json");
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(&keys_path, content)?;
        Ok(())
    }

    /// 列出 Key
    fn list_keys(&self, config_manager: &ConfigManager, tool_filter: &Option<String>) -> Result<()> {
        let config = self.load_keys_config(config_manager)?;
        let registry = Registry::load()?;

        println!("\n{}", style(format!("🔑 {}", translate("key.title"))).cyan().bold());
        println!("{}", "═".repeat(70));

        if config.tools.is_empty() {
            println!("\n{}", translate("key.no_config"));
            println!("\n{}", translate("key.add_hint"));
            return Ok(());
        }

        let tools_to_show: Vec<_> = if let Some(tool_id) = tool_filter {
            config.tools.iter()
                .filter(|(id, _)| *id == tool_id)
                .collect()
        } else {
            config.tools.iter().collect()
        };

        if tools_to_show.is_empty() {
            println!("\n{}", translate("key.not_found").replace("{}", tool_filter.as_ref().unwrap()));
            return Ok(());
        }

        for (tool_id, tool_keys) in tools_to_show {
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);

            println!("\n  {} {}", style("▸").cyan(), style(tool_name).bold());
            println!("  {}", "─".repeat(50));

            if let Some(ref active) = tool_keys.active {
                println!("  {}: {}", translate("key.active"), style(active).green().bold());
            }

            if !tool_keys.keys.is_empty() {
                println!("\n  {:<15} {:<10} {:<12} {}",
                    style(translate("key.name")).dim(),
                    style(translate("key.status")).dim(),
                    style(translate("key.type")).dim(),
                    style(translate("key.note")).dim()
                );

                for (name, info) in &tool_keys.keys {
                    let status = if tool_keys.active.as_ref() == Some(name) {
                        style(format!("✓ {}", translate("key.active"))).green().to_string()
                    } else {
                        style("-").dim().to_string()
                    };

                    let key_type = if info.is_trial {
                        style(translate("key.type_trial")).yellow()
                    } else {
                        style(translate("key.official")).cyan()
                    };

                    let note = info.note.as_deref().unwrap_or("-");

                    println!("  {:<15} {:<10} {:<12} {}",
                        style(name).bold(),
                        status,
                        key_type,
                        style(note).dim()
                    );
                }
            }

            // 轮换状态
            if let Some(ref rotation) = tool_keys.rotation {
                if rotation.enabled {
                    let interval = match rotation.interval {
                        RotationInterval::PerRequest => translate("key.per_request"),
                        RotationInterval::Hourly => translate("key.hourly"),
                        RotationInterval::Daily => translate("key.daily"),
                        RotationInterval::Weekly => translate("key.weekly"),
                    };
                    println!("\n  {}: {} ({})", translate("key.rotation_mode"), style(format!("✓ {}", translate("fallback.enabled"))).green(), interval);
                }
            }
        }

        println!("{}", "═".repeat(70));
        println!("\n{}", translate("key.add_cmd"));
        println!("{}", translate("key.switch_cmd"));
        println!("{}", translate("key.remove_cmd"));
        println!("{}", translate("key.rotate_cmd"));

        Ok(())
    }

    /// 添加 Key
    fn add_key(&self, config_manager: &ConfigManager, tool: &str, name: &str, key: &str) -> Result<()> {
        if name.is_empty() {
            bail!("{}", translate("key.name_empty"));
        }
        if key.is_empty() {
            bail!("{}", translate("key.value_empty"));
        }

        let registry = Registry::load()?;
        let tool_def = registry.find_by_id(tool)
            .or_else(|| registry.find_by_name(tool).first().copied());

        let mut config = self.load_keys_config(config_manager)?;

        let tool_keys = config.tools.entry(tool.to_string())
            .or_insert(ToolKeys {
                active: None,
                keys: HashMap::new(),
                rotation: None,
            });

        // 检查是否已存在
        if tool_keys.keys.contains_key(name) {
            println!("{} {}", style("⚠️").yellow(), translate("key.exists").replace("{}", name));
        }

        // 添加 Key
        let key_info = KeyInfo {
            key: Some(self.mask_key(key)),
            note: None,
            is_trial: false,
            expires: None,
            added_at: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
        };

        tool_keys.keys.insert(name.to_string(), key_info);

        // 如果是第一个 Key，设为激活
        if tool_keys.active.is_none() {
            tool_keys.active = Some(name.to_string());
        }

        let is_active = tool_keys.active.as_ref() == Some(&name.to_string());

        self.save_keys_config(config_manager, &config)?;

        let tool_name = tool_def.map(|t| t.name.as_str()).unwrap_or(tool);
        println!("{} {}", style("✓").green(), translate("key.added").replace("{}", name).replace("{tool}", tool_name));

        if is_active {
            println!("  {}", translate("key.set_active"));
        }

        Ok(())
    }

    /// 删除 Key
    fn remove_key(&self, config_manager: &ConfigManager, tool: &str, name: &str) -> Result<()> {
        let mut config = self.load_keys_config(config_manager)?;

        let tool_keys = config.tools.get_mut(tool)
            .ok_or_else(|| anyhow::anyhow!("{}", translate("key.no_saved").replace("{}", tool)))?;

        if tool_keys.keys.remove(name).is_some() {
            // 如果删除的是激活的 Key，切换到其他
            if tool_keys.active.as_ref() == Some(&name.to_string()) {
                tool_keys.active = tool_keys.keys.keys().next().cloned();
                if let Some(ref new_active) = tool_keys.active {
                    println!("{} {}", style("→").yellow(), translate("key.switched").replace("{}", new_active));
                }
            }

            self.save_keys_config(config_manager, &config)?;
            println!("{} {}", style("✓").green(), translate("key.removed"));
        } else {
            println!("{}", translate("key.not_found_name"));
        }

        Ok(())
    }

    /// 切换 Key
    fn switch_key(&self, config_manager: &ConfigManager, tool: &str, name: &str) -> Result<()> {
        let mut config = self.load_keys_config(config_manager)?;

        let tool_keys = config.tools.get_mut(tool)
            .ok_or_else(|| anyhow::anyhow!("{}", translate("key.no_saved").replace("{}", tool)))?;

        if !tool_keys.keys.contains_key(name) {
            bail!("{}", translate("key.not_found_name"));
        }

        tool_keys.active = Some(name.to_string());
        self.save_keys_config(config_manager, &config)?;

        println!("{} {}: {}", style("✓").green(), translate("key.switched"), style(name).cyan());
        println!("\n{}", translate("key.restart_hint"));

        Ok(())
    }

    /// 切换轮换
    fn toggle_rotation(&self, config_manager: &ConfigManager, tool: &str, enable: bool) -> Result<()> {
        let mut config = self.load_keys_config(config_manager)?;

        let tool_keys = config.tools.entry(tool.to_string())
            .or_insert(ToolKeys {
                active: None,
                keys: HashMap::new(),
                rotation: None,
            });

        if enable && tool_keys.keys.len() < 2 {
            println!("{} {}", style("⚠️").yellow(), translate("key.rotate_need_two"));
            println!("{}", translate("key.current_count").replace("{}", &tool_keys.keys.len().to_string()));
            return Ok(());
        }

        tool_keys.rotation = Some(RotationConfig {
            enabled: enable,
            interval: RotationInterval::PerRequest,
            last_rotation: None,
        });

        self.save_keys_config(config_manager, &config)?;

        if enable {
            println!("{} {}", style("✓").green(), translate("key.rotation_enabled"));
            println!("\n{}", translate("key.rotation_desc"));
        } else {
            println!("{} {}", style("✓").yellow(), translate("key.rotation_disabled"));
        }

        Ok(())
    }

    /// 显示当前激活的 Key
    fn show_current(&self, config_manager: &ConfigManager, tool: &str) -> Result<()> {
        let config = self.load_keys_config(config_manager)?;
        let registry = Registry::load()?;

        let tool_name = registry.find_by_id(tool)
            .map(|t| t.name.as_str())
            .unwrap_or(tool);

        if let Some(tool_keys) = config.tools.get(tool) {
            if let Some(ref active) = tool_keys.active {
                println!("{} {}: {}", style(tool_name).cyan(), translate("key.current_active"), style(active).green().bold());

                if let Some(info) = tool_keys.keys.get(active) {
                    if let Some(ref note) = info.note {
                        println!("  {}: {}", translate("key.note"), note);
                    }
                    if info.is_trial {
                        println!("  {}: {}", translate("key.type"), translate("key.type_trial"));
                        if let Some(ref expires) = info.expires {
                            println!("  {}: {}", translate("key.expires"), expires);
                        }
                    }
                }
            } else {
                println!("{} {}", tool_name, translate("key.no_active"));
            }
        } else {
            println!("{} {}", tool_name, translate("key.no_saved"));
        }

        Ok(())
    }

    /// 遮蔽 Key 显示
    fn mask_key(&self, key: &str) -> String {
        if key.len() <= 8 {
            return "*".repeat(key.len());
        }
        format!("{}...{}", &key[..4], &key[key.len()-4..])
    }
}

/// 获取工具的当前激活 Key（供其他命令调用）
pub fn get_active_key(tool_id: &str) -> Option<String> {
    let config_manager = ConfigManager::new().ok()?;
    let keys_path = config_manager.config_dir().join("keys.json");

    if !keys_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&keys_path).ok()?;
    let config: KeysConfig = serde_json::from_str(&content).ok()?;

    let tool_keys = config.tools.get(tool_id)?;

    // 检查轮换
    if let Some(ref rotation) = tool_keys.rotation {
        if rotation.enabled && tool_keys.keys.len() > 1 {
            // 简单轮换：返回下一个 Key 名称
            let keys: Vec<_> = tool_keys.keys.keys().collect();
            let current_idx = tool_keys.active.as_ref()
                .and_then(|a| keys.iter().position(|k| *k == a))
                .unwrap_or(0);
            let next_idx = (current_idx + 1) % keys.len();
            return keys.get(next_idx).map(|s| (*s).clone());
        }
    }

    tool_keys.active.clone()
}