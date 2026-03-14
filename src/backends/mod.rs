//! 包管理器后端模块

pub mod npm;
pub mod pip;
pub mod cargo;
pub mod brew;

pub use npm::NpmBackend;
pub use pip::PipBackend;
pub use cargo::CargoBackend;
pub use brew::BrewBackend;
