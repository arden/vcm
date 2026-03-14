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
        Commands::Free { pro } => {
            let cmd = FreeCommand::new(pro);
            cmd.execute()?;
        }
    }

    Ok(())
}
