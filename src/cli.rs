//! CLI 命令定义

use clap::{Parser, Subcommand};

/// VCM - Vibe Coding Manager
#[derive(Parser, Debug)]
#[command(name = "vcm")]
#[command(author = "Arden")]
#[command(version)]
#[command(about = "CLI AI Programming Tool Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// JSON output format
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan system for installed tools
    Scan {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// List all known tools
    List {
        /// Show only installed tools
        #[arg(short, long)]
        installed: bool,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Install a tool
    Install {
        /// Tool ID or name
        tool: String,

        /// Specify package manager
        #[arg(short, long)]
        manager: Option<String>,
    },

    /// Update tools
    Update {
        /// Tool ID (optional, updates all if not specified)
        tool: Option<String>,
    },

    /// Remove a tool
    Remove {
        /// Tool ID
        tool: String,
    },

    /// Configure a tool
    Config {
        /// Tool ID
        tool: Option<String>,

        /// Set API Key (format: PROVIDER=KEY)
        #[arg(long)]
        set_key: Option<String>,
    },

    /// Check tool status
    Status,

    /// Search tools
    Search {
        /// Search query
        query: String,
    },

    /// Show tool details
    Info {
        /// Tool ID
        tool: String,
    },

    /// System diagnostics
    Doctor,

    /// Update registry
    UpdateRegistry {
        /// Custom registry URL
        #[arg(short, long)]
        url: Option<String>,
    },

    /// Generate shell completion script
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: String,
    },

    /// Check for tool updates
    Outdated,

    /// Export installed tools list
    Export {
        /// Output file path
        #[arg(short, long, default_value = "vcm-tools.json")]
        output: String,
    },

    /// Import tools list from file
    Import {
        /// Input file path
        #[arg(short, long, default_value = "vcm-tools.json")]
        input: String,
        
        /// Install missing tools
        #[arg(short, long)]
        install: bool,
    },

    /// Interactive setup wizard
    Init,

    /// Show tool usage statistics
    Usage,

    /// Launch a CLI AI tool
    Run {
        /// Tool ID or name
        tool: String,

        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Set default tool
    Default {
        /// Tool ID (shows current default if not specified)
        tool: Option<String>,
    },

    /// Display or set language
    Lang {
        /// Language code (en/zh)
        lang: Option<String>,
    },

    /// Find tools with free AI models
    Free {
        /// Show only tools with free pro-grade models
        #[arg(short, long)]
        pro: bool,

        /// Aggregate all free quotas
        #[arg(long)]
        aggregate: bool,
    },

    /// Manage tool aliases
    Alias {
        /// Alias name (for set/remove) or list to show all
        alias: Option<String>,

        /// Tool ID to map to (for set)
        tool: Option<String>,

        /// Remove an alias
        #[arg(short, long)]
        remove: bool,
    },

    /// Compare multiple tools
    Compare {
        /// Tool IDs to compare (space-separated)
        #[arg(required = true)]
        tools: Vec<String>,
    },

    /// Manage quota tracking and limits
    Quota {
        /// Quota action: status, warn, limit, usage, reset
        #[arg(value_name = "ACTION")]
        action: Option<String>,

        /// Threshold value for warn/limit (0-100)
        #[arg(value_name = "VALUE")]
        value: Option<u8>,

        /// Tool ID for usage/reset
        #[arg(short, long)]
        tool: Option<String>,
    },

    /// Show usage statistics
    Stats,

    /// Show cost estimation
    Cost,

    /// Manage project-level configuration
    Project {
        /// Project action: init, status, use, path
        #[arg(value_name = "ACTION")]
        action: Option<String>,

        /// Tool ID for 'use' action
        #[arg(value_name = "TOOL")]
        tool: Option<String>,

        /// Model for 'use' action
        #[arg(short, long)]
        model: Option<String>,
    },

    /// Manage smart fallback chains
    Fallback {
        /// Fallback action: status, add, remove
        #[arg(value_name = "ACTION")]
        action: Option<String>,

        /// Primary tool ID
        #[arg(value_name = "PRIMARY")]
        primary: Option<String>,

        /// Fallback tool IDs
        #[arg(value_name = "FALLBACKS")]
        fallbacks: Vec<String>,

        /// Enable fallback
        #[arg(long)]
        enable: bool,

        /// Disable fallback
        #[arg(long)]
        disable: bool,
    },

    /// Manage multiple API keys/accounts
    Key {
        /// Key action: list, add, remove, switch, rotate, current
        #[arg(value_name = "ACTION")]
        action: Option<String>,

        /// Tool ID
        #[arg(value_name = "TOOL")]
        tool: Option<String>,

        /// Key name
        #[arg(value_name = "NAME")]
        name: Option<String>,

        /// Key value (for add)
        #[arg(value_name = "KEY")]
        key: Option<String>,

        /// Enable rotation
        #[arg(long)]
        enable: bool,

        /// Disable rotation
        #[arg(long)]
        disable: bool,
    },

    /// Get tool recommendations
    Recommend {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Show trending tools
    Trending,

    /// Show new tools
    New,
}
