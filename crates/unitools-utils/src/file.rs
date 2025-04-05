use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use unitools_core::error::ToolError;

/// 读取文件内容为字符串
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, ToolError> {
    fs::read_to_string(path).map_err(ToolError::IoError)
}

/// 读取文件内容为字节数组
pub fn read_to_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ToolError> {
    fs::read(path).map_err(ToolError::IoError)
}

/// 写入字符串到文件
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> Result<(), ToolError> {
    fs::write(path, content).map_err(ToolError::IoError)
}

/// 写入字节数组到文件
pub fn write_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), ToolError> {
    fs::write(path, content).map_err(ToolError::IoError)
}

/// 检查文件是否存在
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_file()
}

/// 检查目录是否存在
pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_dir()
}

/// 创建目录，如果父目录不存在则递归创建
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ToolError> {
    fs::create_dir_all(path).map_err(ToolError::IoError)
}

/// 获取文件大小（以字节为单位）
pub fn file_size<P: AsRef<Path>>(path: P) -> Result<u64, ToolError> {
    let metadata = fs::metadata(path).map_err(ToolError::IoError)?;
    Ok(metadata.len())
}

/// 获取文件扩展名（无点号）
pub fn file_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .extension()
        .map(|ext| ext.to_string_lossy().to_string())
}

/// 获取文件名（不包含路径）
pub fn file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
}

/// 列出目录中的所有文件
pub fn list_dir_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, ToolError> {
    let entries = fs::read_dir(path).map_err(ToolError::IoError)?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(ToolError::IoError)?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

/// 列出目录中的所有子目录
pub fn list_dir_subdirs<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, ToolError> {
    let entries = fs::read_dir(path).map_err(ToolError::IoError)?;
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.map_err(ToolError::IoError)?;
        let path = entry.path();

        if path.is_dir() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}
