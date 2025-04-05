mod about;
mod home;
mod settings;
mod theme;
mod tool;

use eframe::egui;
use unitools_core::config::Theme;
use unitools_core::tool::Tool;

pub use about::render_about_page;
pub use home::render_home_page;
pub use settings::render_settings_page;
pub use theme::setup_theme;
pub use tool::render_tool_page;

/// 应用页面枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Tool,
    Settings,
    About,
}

/// 渲染侧边导航栏
pub fn render_sidebar(ctx: &egui::Context, app: &mut crate::app::UniToolsApp) {
    egui::SidePanel::left("sidebar").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("UniTools 工具箱");
        });

        ui.separator();

        if ui.button("主页").clicked() {
            app.navigate_to_page(Page::Home);
        }

        ui.separator();

        // 分类菜单
        egui::CollapsingHeader::new("工具分类")
            .default_open(true)
            .show(ui, |ui| {
                // 获取所有工具分类并排序
                let mut categories: Vec<_> = app.categories.keys().collect();
                categories.sort_by_key(|c| std::mem::discriminant(*c));

                for &category in &categories {
                    if let Some(tools) = app.categories.get(category) {
                        ui.collapsing(format!("{} ({})", category, tools.len()), |ui| {
                            for tool in tools {
                                if ui.button(tool.name()).clicked() {
                                    app.navigate_to_tool(tool.name());
                                }
                            }
                        });
                    }
                }
            });

        ui.separator();

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            if ui.button("关于").clicked() {
                app.navigate_to_page(Page::About);
            }

            if ui.button("设置").clicked() {
                app.navigate_to_page(Page::Settings);
            }
        });
    });
}
