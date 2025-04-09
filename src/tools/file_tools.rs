use std::fmt::Debug;
use std::sync::Arc;
use unitools_core::error::ToolError;
use unitools_core::tool::{Tool, ToolCategory, ToolContext, ToolResult};
use unitools_utils::file;

/// 获取所有文件工具
pub fn get_file_tools() -> Vec<Arc<dyn Tool>> {
    vec![Arc::new(FileInfoTool {}), Arc::new(FileBrowserTool {})]
}

/// 文件信息工具
#[derive(Debug, Clone)]
pub struct FileInfoTool {}

impl Tool for FileInfoTool {
    fn name(&self) -> &str {
        "文件信息"
    }

    fn description(&self) -> &str {
        "获取文件的基本信息（大小、类型等）"
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::File
    }

    fn parameter_descriptions(&self) -> Vec<(String, String)> {
        vec![("path".to_string(), "文件路径".to_string())]
    }

    fn execute(&self, ctx: &ToolContext) -> ToolResult {
        // 获取文件路径参数
        let path = match ctx.parameters.get("path") {
            Some(p) => p,
            None => return Err(ToolError::ParameterError("没有提供文件路径".to_string())),
        };

        // 检查文件是否存在
        if !file::file_exists(path) {
            return Err(ToolError::NotFoundError(format!("文件不存在: {}", path)));
        }

        // 收集文件信息
        let size = file::file_size(path)?;
        let extension = file::file_extension(path).unwrap_or_else(|| "无".to_string());
        let filename = file::file_name(path).unwrap_or_else(|| "未知".to_string());

        // 显示文件基本信息
        let mut info = String::new();
        info.push_str(&format!("文件名: {}\n", filename));
        info.push_str(&format!("路径: {}\n", path));
        info.push_str(&format!("大小: {} 字节\n", size));
        info.push_str(&format!("扩展名: {}\n", extension));

        // 返回结果
        Ok(Some(info.into_bytes()))
    }
}

/// 文件浏览工具
#[derive(Debug, Clone)]
pub struct FileBrowserTool {}

impl Tool for FileBrowserTool {
    fn name(&self) -> &str {
        "文件浏览器"
    }

    fn description(&self) -> &str {
        "浏览目录，列出文件和子目录"
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::File
    }

    fn parameter_descriptions(&self) -> Vec<(String, String)> {
        vec![("directory".to_string(), "要浏览的目录路径".to_string())]
    }

    fn execute(&self, ctx: &ToolContext) -> ToolResult {
        // 获取目录路径参数
        let dir_path = match ctx.parameters.get("directory") {
            Some(p) => p,
            None => {
                // 如果没有提供目录参数，则使用当前目录
                "."
            }
        };

        // 检查目录是否存在
        if !file::directory_exists(dir_path) {
            return Err(ToolError::NotFoundError(format!(
                "目录不存在: {}",
                dir_path
            )));
        }

        // 列出文件和子目录
        let files = file::list_dir_files(dir_path)?;
        let subdirs = file::list_dir_subdirs(dir_path)?;

        // 构建结果
        let mut result = String::new();
        result.push_str(&format!("目录: {}\n\n", dir_path));

        // 列出子目录
        result.push_str("子目录:\n");
        if subdirs.is_empty() {
            result.push_str("  (无)\n");
        } else {
            for dir in subdirs {
                result.push_str(&format!(
                    "  [DIR] {}\n",
                    dir.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }

        result.push_str("\n文件:\n");
        if files.is_empty() {
            result.push_str("  (无)\n");
        } else {
            for file_path in files {
                let file_size = match file::file_size(&file_path) {
                    Ok(size) => format!("{} 字节", size),
                    Err(_) => "未知大小".to_string(),
                };
                result.push_str(&format!(
                    "  {} ({})\n",
                    file_path.file_name().unwrap_or_default().to_string_lossy(),
                    file_size
                ));
            }
        }

        Ok(Some(result.into_bytes()))
    }
}
