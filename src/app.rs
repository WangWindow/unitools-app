use eframe::{Frame, egui};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use unitools_core::{
    config::AppConfig,
    plugin::PluginManager,
    tool::{Tool, ToolCategory},
};

use crate::ui::{self, Page};

/// 主应用状态
pub struct UniToolsApp {
    config: Arc<Mutex<AppConfig>>,
    plugin_manager: Arc<PluginManager>,
    tools: Vec<Arc<dyn Tool>>,
    current_page: Page,
    current_tool: Option<String>,
    categories: HashMap<ToolCategory, Vec<Arc<dyn Tool>>>,
}

impl UniToolsApp {
    /// 创建新的应用实例
    pub fn new(cc: &eframe::CreationContext<'_>, config: AppConfig) -> Self {
        // 设置默认主题
        if let Some(ctx) = &cc.egui_ctx {
            ui::setup_theme(ctx, config.theme);
        }

        // 创建插件管理器
        let plugin_manager = Arc::new(PluginManager::new());

        // 加载内置工具
        let mut app = Self {
            config: Arc::new(Mutex::new(config)),
            plugin_manager: plugin_manager.clone(),
            tools: Vec::new(),
            current_page: Page::Home,
            current_tool: None,
            categories: HashMap::new(),
        };

        // 加载内置工具
        app.load_builtin_tools();

        // TODO: 加载插件工具

        // 按类别分组工具
        app.categorize_tools();

        app
    }

    /// 加载内置工具
    fn load_builtin_tools(&mut self) {
        // 加载内置工具
        let builtin_tools = crate::tools::get_builtin_tools();
        self.tools.extend(builtin_tools);
    }

    /// 按类别分组工具
    fn categorize_tools(&mut self) {
        for tool in &self.tools {
            let category = tool.category();
            self.categories
                .entry(category)
                .or_insert_with(Vec::new)
                .push(tool.clone());
        }
    }

    /// 获取当前工具
    fn get_current_tool(&self) -> Option<Arc<dyn Tool>> {
        if let Some(tool_name) = &self.current_tool {
            // 先从工具列表中查找
            for tool in &self.tools {
                if tool.name() == tool_name {
                    return Some(tool.clone());
                }
            }

            // 再尝试从插件管理器中获取
            self.plugin_manager.get_tool(tool_name)
        } else {
            None
        }
    }

    /// 切换到工具页面
    pub fn navigate_to_tool(&mut self, tool_name: &str) {
        self.current_tool = Some(tool_name.to_string());
        self.current_page = Page::Tool;
    }

    /// 切换到指定页面
    pub fn navigate_to_page(&mut self, page: Page) {
        self.current_page = page;
    }
}

impl eframe::App for UniToolsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 根据当前页面渲染不同的UI
        match self.current_page {
            Page::Home => ui::render_home_page(ctx, self),
            Page::Tool => {
                if let Some(tool) = self.get_current_tool() {
                    ui::render_tool_page(ctx, tool.as_ref(), self);
                } else {
                    // 无效工具，返回主页
                    self.current_page = Page::Home;
                }
            }
            Page::Settings => ui::render_settings_page(ctx, self),
            Page::About => ui::render_about_page(ctx, self),
        }
    }
}
