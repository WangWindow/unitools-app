use std::fs;
use std::path::{Path, PathBuf};
use unitools_core::config::AppConfig;

/// 加载或创建新的配置
pub fn load_or_create_config<P: AsRef<Path>>(path: P) -> AppConfig {
    let config_path = path.as_ref();

    // 如果配置文件不存在，确保目录存在
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        // 创建默认配置并保存
        let default_config = AppConfig::default();
        let _ = save_config(&default_config, config_path);
        return default_config;
    }

    // 尝试加载配置，如果失败则返回默认值
    match AppConfig::load_from_file(config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("加载配置文件失败: {:?}，将使用默认配置", e);
            AppConfig::default()
        }
    }
}

/// 保存配置到文件
pub fn save_config<P: AsRef<Path>>(config: &AppConfig, path: P) -> Result<(), String> {
    config
        .save_to_file(&path)
        .map_err(|e| format!("保存配置失败: {:?}", e))
}

/// 获取工具配置目录
pub fn get_tools_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("unitools");
    path.push("tools");

    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }

    path
}

/// 获取默认的插件目录
pub fn get_default_plugin_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("unitools");
    path.push("plugins");

    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }

    path
}
