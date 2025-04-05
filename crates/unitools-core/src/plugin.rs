use crate::error::ToolError;
use crate::tool::Tool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 插件接口
pub trait Plugin: Send + Sync {
    /// 返回插件名称
    fn name(&self) -> &str;

    /// 返回插件描述
    fn description(&self) -> &str;

    /// 返回插件版本
    fn version(&self) -> &str;

    /// 初始化插件
    fn initialize(&mut self) -> Result<(), ToolError>;

    /// 获取插件提供的工具列表
    fn get_tools(&self) -> Vec<Arc<dyn Tool>>;

    /// 清理插件资源
    fn cleanup(&mut self) -> Result<(), ToolError>;
}

/// 插件管理器
pub struct PluginManager {
    plugins: Mutex<HashMap<String, Box<dyn Plugin>>>,
    tools: Mutex<HashMap<String, Arc<dyn Tool>>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self {
            plugins: Mutex::new(HashMap::new()),
            tools: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册插件
    pub fn register_plugin(&self, plugin: Box<dyn Plugin>) -> Result<(), ToolError> {
        let plugin_name = plugin.name().to_string();
        let mut plugins = self.plugins.lock().unwrap();

        if plugins.contains_key(&plugin_name) {
            return Err(ToolError::PluginError(format!(
                "插件 '{}' 已经注册",
                plugin_name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        // 注册工具
        let tools = plugin.get_tools();
        let mut tools_map = self.tools.lock().unwrap();

        for tool in tools {
            let tool_name = tool.name().to_string();
            if tools_map.contains_key(&tool_name) {
                return Err(ToolError::PluginError(format!(
                    "工具 '{}' 已经被其他插件注册",
                    tool_name
                )));
            }
            tools_map.insert(tool_name, tool);
        }

        plugins.insert(plugin_name, plugin);
        Ok(())
    }

    /// 获取所有工具
    pub fn get_tools(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.lock().unwrap().values().cloned().collect()
    }

    /// 通过名称获取工具
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.lock().unwrap().get(name).cloned()
    }

    /// 卸载插件
    pub fn unload_plugin(&self, name: &str) -> Result<(), ToolError> {
        let mut plugins = self.plugins.lock().unwrap();

        if let Some(mut plugin) = plugins.remove(name) {
            // 清理插件资源
            plugin.cleanup()?;

            // 移除相关工具
            let tools = plugin.get_tools();
            let mut tools_map = self.tools.lock().unwrap();

            for tool in tools {
                tools_map.remove(tool.name());
            }

            Ok(())
        } else {
            Err(ToolError::NotFoundError(format!("未找到插件: '{}'", name)))
        }
    }
}
