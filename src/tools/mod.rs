mod file_tools;
mod text_tools;

use std::sync::Arc;
use unitools_core::tool::Tool;

/// 获取所有内置工具
pub fn get_builtin_tools() -> Vec<Arc<dyn Tool>> {
    let mut tools: Vec<Arc<dyn Tool>> = Vec::new();

    // 添加文本工具
    tools.extend(text_tools::get_text_tools());

    // 添加文件工具
    tools.extend(file_tools::get_file_tools());

    tools
}
