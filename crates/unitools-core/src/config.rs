use crate::error::ToolError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用主题
    pub theme: Theme,
    /// 界面语言
    pub language: String,
    /// 启用的插件列表
    pub enabled_plugins: Vec<String>,
    /// 个人配置
    pub user: UserConfig,
    /// 工具配置
    pub tool_configs: std::collections::HashMap<String, serde_json::Value>,
    /// 插件目录
    pub plugin_directory: Option<PathBuf>,
}

/// 应用主题
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    System,
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// 用户名
    pub username: String,
    /// 首选工作目录
    pub working_directory: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "zh-CN".to_string(),
            enabled_plugins: Vec::new(),
            user: UserConfig {
                username: "用户".to_string(),
                working_directory: None,
            },
            tool_configs: std::collections::HashMap::new(),
            plugin_directory: None,
        }
    }
}

impl AppConfig {
    /// 从文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ToolError> {
        let content = fs::read_to_string(&path).map_err(|e| ToolError::IoError(e))?;

        serde_json::from_str(&content).map_err(|e| ToolError::ParseError(e.to_string()))
    }

    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ToolError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ToolError::FormatError(e.to_string()))?;

        fs::write(&path, content).map_err(|e| ToolError::IoError(e))
    }

    /// 获取指定工具的配置
    pub fn get_tool_config(&self, tool_name: &str) -> Option<&serde_json::Value> {
        self.tool_configs.get(tool_name)
    }

    /// 设置工具配置
    pub fn set_tool_config(&mut self, tool_name: String, config: serde_json::Value) {
        self.tool_configs.insert(tool_name, config);
    }
}
