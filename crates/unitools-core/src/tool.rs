use crate::error::ToolError;
use dyn_clone::DynClone;
use std::fmt::{Debug, Display};

/// 工具类别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    File,      // 文件操作工具
    Text,      // 文本处理工具
    Network,   // 网络工具
    Image,     // 图像处理工具
    Converter, // 格式转换工具
    System,    // 系统工具
    Other,     // 其他工具
}

impl Display for ToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCategory::File => write!(f, "文件工具"),
            ToolCategory::Text => write!(f, "文本工具"),
            ToolCategory::Network => write!(f, "网络工具"),
            ToolCategory::Image => write!(f, "图像工具"),
            ToolCategory::Converter => write!(f, "转换工具"),
            ToolCategory::System => write!(f, "系统工具"),
            ToolCategory::Other => write!(f, "其他工具"),
        }
    }
}

/// 工具上下文，包含工具执行时所需的数据
pub struct ToolContext {
    pub input_data: Option<Vec<u8>>,
    pub parameters: std::collections::HashMap<String, String>,
}

impl Default for ToolContext {
    fn default() -> Self {
        Self {
            input_data: None,
            parameters: std::collections::HashMap::new(),
        }
    }
}

/// 工具执行结果
pub type ToolResult = Result<Option<Vec<u8>>, ToolError>;

/// 工具特质/接口
pub trait Tool: Debug + DynClone + Send + Sync {
    /// 返回工具名称
    fn name(&self) -> &str;

    /// 返回工具描述
    fn description(&self) -> &str;

    /// 返回工具类别
    fn category(&self) -> ToolCategory;

    /// 执行工具
    fn execute(&self, ctx: &ToolContext) -> ToolResult;

    /// 返回工具参数说明
    fn parameter_descriptions(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

dyn_clone::clone_trait_object!(Tool);
