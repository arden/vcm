//! completions 命令实现 - 生成 shell 补全脚本

use anyhow::{bail, Result};
use clap::CommandFactory;
use std::io::Write;

/// completions 命令
pub struct CompletionsCommand {
    shell: String,
}

impl CompletionsCommand {
    pub fn new(shell: String) -> Self {
        Self { shell }
    }

    pub fn execute(&self) -> Result<()> {
        let mut cmd = crate::Cli::command();
        
        let shell_name = self.shell.to_lowercase();
        
        let shell = match shell_name.as_str() {
            "bash" => clap_complete::Shell::Bash,
            "zsh" => clap_complete::Shell::Zsh,
            "fish" => clap_complete::Shell::Fish,
            "elvish" => clap_complete::Shell::Elvish,
            "powershell" | "ps1" => clap_complete::Shell::PowerShell,
            _ => bail!("不支持的 shell: {}\n支持的 shell: bash, zsh, fish, elvish, powershell", self.shell),
        };

        let bin_name = "vcm";
        
        // 生成补全脚本
        clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());

        Ok(())
    }
}
