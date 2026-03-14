//! VCM - Vibe Coding Manager
//!
//! CLI AI编程工具管理器

pub mod cli;
pub mod commands;
pub mod core;
pub mod models;
pub mod backends;
pub mod utils;
pub mod i18n;

// Re-export commonly used types
pub use cli::{Cli, Commands};
pub use core::registry::Registry;
pub use models::*;
pub use i18n::{current_lang, set_lang, Language, translate};