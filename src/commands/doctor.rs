//! doctor 命令实现

use crate::core::Registry;
use crate::i18n::translate;
use anyhow::Result;
use console::style;

/// doctor 命令
pub struct DoctorCommand;

impl Default for DoctorCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl DoctorCommand {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self) -> Result<()> {
        println!("{} {}\n", style("🏥").dim(), translate("doctor.title"));

        // 检查包管理器
        self.check_package_managers()?;

        // 检查环境变量
        self.check_env_vars()?;

        // 检查注册表
        self.check_registry()?;

        println!("\n{} {}", style("✓").green(), translate("doctor.done"));

        Ok(())
    }

    fn check_package_managers(&self) -> Result<()> {
        println!("{}:", style(translate("doctor.package_managers")).bold());

        let managers = [
            ("npm", "doctor.npm"),
            ("pip", "doctor.pip"),
            ("pipx", "doctor.pipx"),
            ("cargo", "doctor.cargo"),
            ("brew", "doctor.brew"),
            ("go", "doctor.go"),
        ];

        for (cmd, desc_key) in &managers {
            let desc = translate(desc_key);
            let status = if which::which(cmd).is_ok() {
                let version = self.get_version(cmd);
                match version {
                    Some(v) => style(format!("✓ {} ({})", desc, v)).green(),
                    None => style(format!("✓ {}", desc)).green(),
                }
            } else {
                style(format!("○ {} ({})", desc, translate("msg.not_installed"))).dim()
            };
            println!("  {}", status);
        }

        Ok(())
    }

    fn check_env_vars(&self) -> Result<()> {
        println!("\n{}:", style(translate("doctor.api_keys")).bold());

        let env_vars = [
            ("ANTHROPIC_API_KEY", "Anthropic"),
            ("OPENAI_API_KEY", "OpenAI"),
            ("GOOGLE_API_KEY", "Google"),
            ("GITHUB_TOKEN", "GitHub"),
        ];

        for (var, provider) in &env_vars {
            let status = if std::env::var(var).is_ok() {
                style(format!("✓ {} {}", provider, translate("msg.configured"))).green()
            } else {
                style(format!("○ {} {}", provider, translate("msg.not_configured"))).dim()
            };
            println!("  {}", status);
        }

        Ok(())
    }

    fn check_registry(&self) -> Result<()> {
        println!("\n{}:", style(translate("doctor.registry")).bold());

        match Registry::load() {
            Ok(registry) => {
                println!("  {} {} {} {}",
                    style("✓").green(),
                    translate("doctor.registry_loaded"),
                    style(registry.len()).cyan(),
                    translate("doctor.tools")
                );
            }
            Err(e) => {
                println!("  {} {}: {}", style("✗").red(), translate("msg.error"), e);
            }
        }

        Ok(())
    }

    fn get_version(&self, cmd: &str) -> Option<String> {
        let output = std::process::Command::new(cmd)
            .arg("--version")
            .output()
            .ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let first_line = stdout.lines().next()?;
            for word in first_line.split_whitespace() {
                if word.contains('.') && word.chars().any(|c| c.is_ascii_digit()) {
                    return Some(word.to_string());
                }
            }
        }
        None
    }
}