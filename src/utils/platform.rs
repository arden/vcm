//! 平台检测工具

use std::env;

/// 当前平台
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    /// 获取当前平台
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        {
            Platform::Linux
        }
        #[cfg(target_os = "macos")]
        {
            Platform::Macos
        }
        #[cfg(target_os = "windows")]
        {
            Platform::Windows
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Platform::Linux // 默认
        }
    }

    /// 是否为 Linux
    pub fn is_linux(&self) -> bool {
        matches!(self, Platform::Linux)
    }

    /// 是否为 macOS
    pub fn is_macos(&self) -> bool {
        matches!(self, Platform::Macos)
    }

    /// 是否为 Windows
    pub fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::Macos => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
        }
    }
}

/// 获取 shell 名称
pub fn shell_name() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "bash".to_string())
        .split('/')
        .next_back()
        .unwrap_or("bash")
        .to_string()
}

/// 获取 home 目录
pub fn home_dir() -> std::path::PathBuf {
    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
}

/// 获取配置目录
pub fn config_dir() -> std::path::PathBuf {
    dirs::config_dir().unwrap_or_else(|| home_dir().join(".config"))
}
