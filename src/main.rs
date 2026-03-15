//! VCM - Vibe Coding Manager
//! CLI AI编程工具管理器

use clap::Parser;
use vcm::commands::*;
use vcm::Cli;
use vcm::Commands;

fn main() -> anyhow::Result<()> {
    // 初始化国际化
    vcm::i18n::init();
    
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 执行命令
    match cli.command {
        Commands::Scan { detailed } => {
            let cmd = ScanCommand::new(detailed, cli.json);
            cmd.execute()?;
        }
        Commands::List { installed, tag } => {
            let cmd = ListCommand::new(installed, tag, cli.json);
            cmd.execute()?;
        }
        Commands::Install { tool, manager } => {
            let cmd = InstallCommand::new(tool, manager);
            cmd.execute()?;
        }
        Commands::Update { tool } => {
            let cmd = UpdateCommand::new(tool);
            cmd.execute()?;
        }
        Commands::Remove { tool } => {
            let cmd = RemoveCommand::new(tool, false);
            cmd.execute()?;
        }
        Commands::Config { tool, set_key } => {
            let cmd = ConfigCommand::new(tool, set_key);
            cmd.execute()?;
        }
        Commands::Status => {
            let cmd = StatusCommand::new(cli.json);
            cmd.execute()?;
        }
        Commands::Search { query } => {
            let cmd = SearchCommand::new(query, cli.json);
            cmd.execute()?;
        }
        Commands::Info { tool } => {
            let cmd = InfoCommand::new(tool);
            cmd.execute()?;
        }
        Commands::Doctor => {
            let cmd = DoctorCommand::new();
            cmd.execute()?;
        }
        Commands::UpdateRegistry { url } => {
            let cmd = UpdateRegistryCommand::new(url);
            cmd.execute()?;
        }
        Commands::Completions { shell } => {
            let cmd = CompletionsCommand::new(shell);
            cmd.execute()?;
        }
        Commands::Outdated => {
            let cmd = OutdatedCommand::new(cli.json);
            cmd.execute()?;
        }
        Commands::Export { output } => {
            let cmd = ExportCommand::new(output);
            cmd.execute()?;
        }
        Commands::Import { input, install } => {
            let cmd = ImportCommand::new(input, install);
            cmd.execute()?;
        }
        Commands::Init => {
            let cmd = InitCommand::new();
            cmd.execute()?;
        }
        Commands::Usage => {
            let cmd = UsageCommand::new();
            cmd.execute()?;
        }
        Commands::Run { tool, args } => {
            let cmd = RunCommand::new(tool, args);
            cmd.execute()?;
        }
        Commands::Default { tool } => {
            let cmd = DefaultCommand::new(tool);
            cmd.execute()?;
        }
        Commands::Lang { lang } => {
            let cmd = LangCommand::new(lang);
            cmd.execute()?;
        }
        Commands::Free { pro, aggregate } => {
            let cmd = FreeCommand::new(pro, aggregate);
            cmd.execute()?;
        }
        Commands::Alias { alias, tool, remove } => {
            use vcm::commands::alias::AliasAction;
            let action = if remove {
                AliasAction::Remove { 
                    alias: alias.unwrap_or_default() 
                }
            } else if let Some(a) = alias {
                if let Some(t) = tool {
                    AliasAction::Set { alias: a, tool: t }
                } else {
                    AliasAction::List
                }
            } else {
                AliasAction::List
            };
            let cmd = AliasCommand::new(action);
            cmd.execute()?;
        }
        Commands::Compare { tools } => {
            let cmd = CompareCommand::new(tools);
            cmd.execute()?;
        }
        Commands::Quota { action, value, tool } => {
            use vcm::commands::quota::QuotaAction;
            let quota_action = match action.as_deref() {
                Some("warn") => {
                    QuotaAction::Warn { 
                        threshold: value.unwrap_or(80) 
                    }
                }
                Some("limit") => {
                    QuotaAction::Limit { 
                        threshold: value 
                    }
                }
                Some("usage") => {
                    QuotaAction::Usage { tool }
                }
                Some("reset") => {
                    QuotaAction::Reset { tool }
                }
                Some("status") | None => {
                    QuotaAction::Status
                }
                Some(other) => {
                    anyhow::bail!("未知的配额操作: '{}'\n可用操作: status, warn, limit, usage, reset", other);
                }
            };
            let cmd = QuotaCommand::new(quota_action);
            cmd.execute()?;
        }
        Commands::Stats => {
            let cmd = StatsCommand::new(false, cli.json);
            cmd.execute()?;
        }
        Commands::Cost => {
            let cmd = StatsCommand::new(true, cli.json);
            cmd.execute()?;
        }
        Commands::Project { action, tool, model } => {
            use vcm::commands::project::ProjectAction;
            let project_action = match action.as_deref() {
                Some("init") => ProjectAction::Init,
                Some("use") => {
                    ProjectAction::Use { 
                        tool: tool.unwrap_or_default(),
                        model 
                    }
                }
                Some("path") => ProjectAction::Path,
                Some("status") | None => ProjectAction::Status,
                Some(other) => {
                    anyhow::bail!("未知的项目操作: '{}'\n可用操作: init, status, use, path", other);
                }
            };
            let cmd = ProjectCommand::new(project_action);
            cmd.execute()?;
        }
        Commands::Fallback { action, primary, fallbacks, enable, disable } => {
            use vcm::commands::fallback::FallbackAction;
            let fallback_action = if enable {
                FallbackAction::Toggle { enabled: true }
            } else if disable {
                FallbackAction::Toggle { enabled: false }
            } else {
                match action.as_deref() {
                    Some("add") => {
                        FallbackAction::Add { 
                            primary: primary.unwrap_or_default(),
                            fallbacks 
                        }
                    }
                    Some("remove") => {
                        FallbackAction::Remove { 
                            primary: primary.unwrap_or_default() 
                        }
                    }
                    Some("default") => {
                        let mut tools = vec![primary.unwrap_or_default()];
                        tools.extend(fallbacks);
                        FallbackAction::SetDefault { tools }
                    }
                    Some("status") | None => FallbackAction::Status,
                    Some(other) => {
                        anyhow::bail!("未知的降级操作: '{}'\n可用操作: status, add, remove, default", other);
                    }
                }
            };
            let cmd = FallbackCommand::new(fallback_action);
            cmd.execute()?;
        }
        Commands::Key { action, tool, name, key, enable, disable } => {
            use vcm::commands::key::KeyAction;
            let key_action = if enable || disable {
                KeyAction::Rotate { 
                    tool: tool.unwrap_or_default(),
                    enable: enable 
                }
            } else {
                match action.as_deref() {
                    Some("add") => {
                        KeyAction::Add { 
                            tool: tool.unwrap_or_default(),
                            name: name.unwrap_or_default(),
                            key: key.unwrap_or_default(),
                        }
                    }
                    Some("remove") => {
                        KeyAction::Remove { 
                            tool: tool.unwrap_or_default(),
                            name: name.unwrap_or_default(),
                        }
                    }
                    Some("switch") => {
                        KeyAction::Switch { 
                            tool: tool.unwrap_or_default(),
                            name: name.unwrap_or_default(),
                        }
                    }
                    Some("rotate") => {
                        KeyAction::Rotate { 
                            tool: tool.unwrap_or_default(),
                            enable: true,
                        }
                    }
                    Some("current") => {
                        KeyAction::Current { 
                            tool: tool.unwrap_or_default(),
                        }
                    }
                    Some("list") | None => {
                        KeyAction::List { tool }
                    }
                    Some(other) => {
                        anyhow::bail!("未知的 Key 操作: '{}'\n可用操作: list, add, remove, switch, rotate, current", other);
                    }
                }
            };
            let cmd = KeyCommand::new(key_action);
            cmd.execute()?;
        }
        Commands::Recommend { tag } => {
            use vcm::commands::recommend::RecommendMode;
            let mode = match tag {
                Some(t) => RecommendMode::ByTag(t),
                None => RecommendMode::Personal,
            };
            let cmd = RecommendCommand::new(mode);
            cmd.execute()?;
        }
        Commands::Trending => {
            use vcm::commands::recommend::RecommendMode;
            let cmd = RecommendCommand::new(RecommendMode::Trending);
            cmd.execute()?;
        }
        Commands::New => {
            use vcm::commands::recommend::RecommendMode;
            let cmd = RecommendCommand::new(RecommendMode::New);
            cmd.execute()?;
        }
    }

    Ok(())
}
