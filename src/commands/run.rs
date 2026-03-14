//! run 命令实现 - 启动 CLI AI 工具

use crate::core::Registry;
use crate::models::*;
use crate::i18n::translate;
use anyhow::{bail, Result};
use console::style;
use std::process::{Command, Stdio};

/// run 命令
pub struct RunCommand {
    tool: String,
    args: Vec<String>,
}

impl RunCommand {
    pub fn new(tool: String, args: Vec<String>) -> Self {
        Self { tool, args }
    }

    pub fn execute(&self) -> Result<()> {
        let registry = Registry::load()?;

        // 查找工具
        let tool_def = registry.find_by_id(&self.tool)
            .or_else(|| registry.find_by_name(&self.tool).first().copied());

        let tool_def = match tool_def {
            Some(t) => t,
            None => bail!("{}", translate("tool.not_found").replace("{}", &self.tool)),
        };

        // 检查工具是否已安装
        if !tool_def.is_installed() {
            bail!("{}", translate("run.not_installed").replace("{}", &tool_def.name).replace("{tool}", &tool_def.id));
        }

        // 获取可执行文件路径
        let executable = tool_def.executable_path()
            .ok_or_else(|| anyhow::anyhow!("{}", translate("run.executable_not_found")))?;

        // 检查环境变量配置
        let missing_env: Vec<_> = tool_def.env_vars.iter()
            .filter(|e| e.required && std::env::var(&e.name).is_err())
            .collect();

        if !missing_env.is_empty() {
            println!("{} {}", style("⚠").yellow(), translate("run.missing_env"));
            for env_var in &missing_env {
                println!("  {} - {}", style(&env_var.name).yellow(), env_var.description);
            }
            println!("\n{}", translate("run.configure_env").replace("{}", &style("vcm config <tool>").cyan().to_string()));
            println!();
        }

        // 显示启动信息
        println!("{} {}\n", style("🚀").dim(), translate("run.launching").replace("{}", &style(&tool_def.name).cyan().bold().to_string()));
        if !self.args.is_empty() {
            println!("{}", translate("run.args").replace("{}", &self.args.join(" ")));
            println!();
        }

        // 启动工具
        let mut cmd = Command::new(&executable);
        
        if !self.args.is_empty() {
            cmd.args(&self.args);
        }

        // 继承标准输入输出
        let status = cmd
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
        }

        Ok(())
    }
}
