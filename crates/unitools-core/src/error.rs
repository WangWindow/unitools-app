use std::io;
use thiserror::Error;

/// 工具箱错误类型
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("IO错误: {0}")]
    IoError(#[from] io::Error),

    #[error("参数错误: {0}")]
    ParameterError(String),

    #[error("格式错误: {0}")]
    FormatError(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("未找到: {0}")]
    NotFoundError(String),

    #[error("插件错误: {0}")]
    PluginError(String),

    #[error("未实现: {0}")]
    NotImplementedError(String),

    #[error("其他错误: {0}")]
    Other(String),
}
