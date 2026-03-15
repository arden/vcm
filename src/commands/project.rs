//! 项目级配置命令

use crate::core::Registry;
use anyhow::{bail, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    /// 项目名称
    #[serde(default)]
    pub name: Option<String>,
    /// 默认工具
    #[serde(default)]
    pub default_tool: Option<String>,
    /// 工具配置
    #[serde(default)]
    pub tools: std::collections::HashMap<String, ToolProjectConfig>,
}

/// 工具项目配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolProjectConfig {
    /// 使用的模型
    #[serde(default)]
    pub model: Option<String>,
    /// 环境变量
    #[serde(default)]
    pub env_vars: std::collections::HashMap<String, String>,
}

/// 项目命令
pub struct ProjectCommand {
    action: ProjectAction,
}

/// 项目操作类型
pub enum ProjectAction {
    /// 初始化项目配置
    Init,
    /// 显示项目状态
    Status,
    /// 设置默认工具
    Use { tool: String, model: Option<String> },
    /// 显示项目路径
    Path,
}

impl ProjectCommand {
    pub fn new(action: ProjectAction) -> Self {
        Self { action }
    }

    pub fn execute(&self) -> Result<()> {
        match &self.action {
            ProjectAction::Init => {
                self.init_project()?;
            }
            ProjectAction::Status => {
                self.show_status()?;
            }
            ProjectAction::Use { tool, model } => {
                self.set_tool(tool, model)?;
            }
            ProjectAction::Path => {
                self.show_path()?;
            }
        }

        Ok(())
    }

    /// 查找项目根目录
    fn find_project_root(&self) -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;

        loop {
            // 检查是否存在 .vcm 目录
            if current.join(".vcm").exists() {
                return Some(current);
            }

            // 检查是否存在 .git 目录（作为项目根的标志）
            if current.join(".git").exists() {
                return Some(current);
            }

            // 检查常见的项目标志文件
            if current.join("package.json").exists()
                || current.join("Cargo.toml").exists()
                || current.join("pyproject.toml").exists()
                || current.join("go.mod").exists()
            {
                return Some(current);
            }

            // 向上一级
            if !current.pop() {
                break;
            }
        }

        None
    }

    /// 获取项目配置目录
    fn get_vcm_dir(&self) -> Result<PathBuf> {
        let project_root = self.find_project_root()
            .ok_or_else(|| anyhow::anyhow!("未找到项目根目录。请确保在项目目录中运行此命令，或使用 'vcm project init' 初始化项目配置。"))?;

        Ok(project_root.join(".vcm"))
    }

    /// 加载项目配置
    fn load_project_config(&self) -> Result<ProjectConfig> {
        let vcm_dir = self.get_vcm_dir()?;
        let config_path = vcm_dir.join("config.toml");

        if !config_path.exists() {
            return Ok(ProjectConfig::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: ProjectConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 保存项目配置
    fn save_project_config(&self, config: &ProjectConfig) -> Result<()> {
        let vcm_dir = self.get_vcm_dir()?;
        
        if !vcm_dir.exists() {
            std::fs::create_dir_all(&vcm_dir)?;
        }

        let config_path = vcm_dir.join("config.toml");
        let content = toml::to_string_pretty(config)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// 初始化项目配置
    fn init_project(&self) -> Result<()> {
        let vcm_dir = self.get_vcm_dir()?;

        if vcm_dir.exists() {
            println!("{} 项目已初始化", style("✓").green());
            println!("\n配置目录: {}", vcm_dir.display());
            return Ok(());
        }

        // 创建 .vcm 目录
        std::fs::create_dir_all(&vcm_dir)?;

        // 创建默认配置
        let project_name = std::env::current_dir()?
            .file_name()
            .map(|n| n.to_string_lossy().to_string());

        let config = ProjectConfig {
            name: project_name,
            ..Default::default()
        };

        self.save_project_config(&config)?;

        // 创建 .gitignore（忽略敏感配置）
        let gitignore_path = vcm_dir.join(".gitignore");
        std::fs::write(&gitignore_path, "# 敏感配置\n*.local.toml\n*.env\n")?;

        println!("{} 项目配置已初始化", style("✓").green());
        println!("\n配置目录: {}", vcm_dir.display());
        println!("\n下一步:");
        println!("  • 使用 'vcm project use <tool>' 设置默认工具");
        println!("  • 编辑 .vcm/config.toml 配置工具参数");

        Ok(())
    }

    /// 显示项目状态
    fn show_status(&self) -> Result<()> {
        let vcm_dir = self.get_vcm_dir()?;
        let config = self.load_project_config()?;
        let registry = Registry::load()?;

        println!("\n{}", style("📁 项目配置").cyan().bold());
        println!("{}", "═".repeat(60));

        // 项目名称
        if let Some(ref name) = config.name {
            println!("  项目名称: {}", style(name).cyan());
        }

        // 配置目录
        println!("  配置目录: {}", style(vcm_dir.display()).dim());

        // 默认工具
        if let Some(ref tool_id) = config.default_tool {
            let tool_name = registry.find_by_id(tool_id)
                .map(|t| t.name.as_str())
                .unwrap_or(tool_id);
            println!("  默认工具: {} ({})", style(tool_name).green(), tool_id);
        } else {
            println!("  默认工具: {}", style("未设置").dim());
        }

        // 工具配置
        if !config.tools.is_empty() {
            println!("\n{}", style("🔧 工具配置").bold());
            println!("{}", "─".repeat(60));

            for (tool_id, tool_config) in &config.tools {
                let tool_name = registry.find_by_id(tool_id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(tool_id);
                println!("  {}:", style(tool_name).cyan());

                if let Some(ref model) = tool_config.model {
                    println!("    模型: {}", style(model).yellow());
                }

                if !tool_config.env_vars.is_empty() {
                    println!("    环境变量:");
                    for (key, _) in tool_config.env_vars.iter() {
                        println!("      - {}", key);
                    }
                }
            }
        }

        println!("{}", "═".repeat(60));
        println!("\n提示: 使用 'vcm project use <tool>' 设置默认工具");

        Ok(())
    }

    /// 设置工具
    fn set_tool(&self, tool: &str, model: &Option<String>) -> Result<()> {
        let registry = Registry::load()?;
        
        // 验证工具
        let tool_def = registry.find_by_id(tool)
            .or_else(|| registry.find_by_name(tool).first().copied());

        let tool_id = match tool_def {
            Some(t) => t.id.clone(),
            None => {
                println!("{} 工具 '{}' 未在注册表中找到，但仍会保存配置", 
                    style("⚠️").yellow(), tool);
                tool.to_string()
            }
        };

        let mut config = self.load_project_config()?;
        config.default_tool = Some(tool_id.clone());

        // 如果指定了模型，更新工具配置
        if let Some(ref m) = model {
            let tool_config = config.tools.entry(tool_id.clone())
                .or_insert(ToolProjectConfig::default());
            tool_config.model = Some(m.clone());
        }

        self.save_project_config(&config)?;

        let tool_name = tool_def.map(|t| t.name.as_str()).unwrap_or(&tool_id);
        println!("{} 已设置项目默认工具: {}", style("✓").green(), style(tool_name).cyan());

        if let Some(ref m) = model {
            println!("    模型: {}", style(m).yellow());
        }

        println!("\n配置已保存到 .vcm/config.toml");

        Ok(())
    }

    /// 显示项目路径
    fn show_path(&self) -> Result<()> {
        let vcm_dir = self.get_vcm_dir()?;
        println!(".vcm 目录: {}", vcm_dir.display());

        let config_path = vcm_dir.join("config.toml");
        if config_path.exists() {
            println!("配置文件: {}", config_path.display());
        }

        Ok(())
    }
}

/// 获取当前项目的默认工具（供其他命令调用）
pub fn get_project_default_tool() -> Option<String> {
    let cmd = ProjectCommand::new(ProjectAction::Status);
    cmd.load_project_config().ok()?.default_tool
}

/// 获取项目的工具配置（供其他命令调用）
pub fn get_project_tool_config(tool_id: &str) -> Option<ToolProjectConfig> {
    let cmd = ProjectCommand::new(ProjectAction::Status);
    cmd.load_project_config().ok()?.tools.get(tool_id).cloned()
}
