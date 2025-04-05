pub mod config;
pub mod error;
pub mod plugin;
pub mod tool;

pub use config::AppConfig;
pub use error::ToolError;
pub use plugin::{Plugin, PluginManager};
pub use tool::{Tool, ToolCategory, ToolContext, ToolResult};

/// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
