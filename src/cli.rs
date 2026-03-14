//! CLI 命令定义

use clap::{Parser, Subcommand};

/// VCM - Vibe Coding Manager
#[derive(Parser, Debug)]
#[command(name = "vcm")]
#[command(author = "Arden")]
#[command(version)]
#[command(about = "CLI AI编程工具管理器", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// 详细输出
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// JSON 输出格式
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 扫描系统已安装的工具
    Scan {
        /// 显示详细信息
        #[arg(short, long)]
        detailed: bool,
    },

    /// 列出所有已知工具
    List {
        /// 仅显示已安装
        #[arg(short, long)]
        installed: bool,

        /// 按标签筛选
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// 安装工具
    Install {
        /// 工具ID或名称
        tool: String,

        /// 指定包管理器
        #[arg(short, long)]
        manager: Option<String>,
    },

    /// 更新工具
    Update {
        /// 工具ID（可选，不指定则更新全部）
        tool: Option<String>,
    },

    /// 卸载工具
    Remove {
        /// 工具ID
        tool: String,
    },

    /// 配置工具
    Config {
        /// 工具ID
        tool: Option<String>,

        /// 设置API Key (格式: PROVIDER=KEY)
        #[arg(long)]
        set_key: Option<String>,
    },

    /// 检查工具状态
    Status,

    /// 搜索工具
    Search {
        /// 搜索关键词
        query: String,
    },

    /// 显示工具详情
    Info {
        /// 工具ID
        tool: String,
    },

    /// 系统诊断
    Doctor,

    /// 更新注册表
    UpdateRegistry {
        /// 自定义注册表 URL
        #[arg(short, long)]
        url: Option<String>,
    },

    /// 生成 shell 补全脚本
    Completions {
        /// Shell 类型 (bash, zsh, fish, powershell)
        shell: String,
    },

    /// 检查已安装工具是否有更新
    Outdated,

    /// 导出已安装工具列表
    Export {
        /// 输出文件路径
        #[arg(short, long, default_value = "vcm-tools.json")]
        output: String,
    },

    /// 从文件导入工具列表
    Import {
        /// 输入文件路径
        #[arg(short, long, default_value = "vcm-tools.json")]
        input: String,
        
        /// 是否安装缺失的工具
        #[arg(short, long)]
        install: bool,
    },

    /// 交互式初始化向导
    Init,

    /// 显示工具使用统计
    Usage,

    /// 启动 CLI AI 工具
    Run {
        /// 工具ID或名称
        tool: String,

        /// 传递给工具的参数
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// 设置默认工具
    Default {
        /// 工具ID（不指定则显示当前默认工具）
        tool: Option<String>,
    },

    /// 显示或设置语言
    Lang {
        /// 语言代码 (en/zh)
        lang: Option<String>,
    },
}
