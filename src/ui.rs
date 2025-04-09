mod about;
mod home;
mod settings;
mod theme;
mod tool;

use eframe::egui;

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
    // 预先收集工具信息，避免在闭包中直接访问app
    let mut categorized_tools = Vec::new();

    let categories: Vec<_> = app.categories.keys().collect();
    for &category in &categories {
        if let Some(tools) = app.categories.get(category) {
            let tool_names: Vec<String> = tools.iter().map(|t| t.name().to_string()).collect();
            categorized_tools.push((category.to_string(), tool_names));
        }
    }

    // 排序分类
    categorized_tools.sort_by(|(a, _), (b, _)| a.cmp(b));

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
                for (category, tool_names) in &categorized_tools {
                    ui.collapsing(format!("{} ({})", category, tool_names.len()), |ui| {
                        for tool_name in tool_names {
                            if ui.button(tool_name).clicked() {
                                app.navigate_to_tool(tool_name);
                            }
                        }
                    });
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
