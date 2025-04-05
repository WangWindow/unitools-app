use eframe::egui;

/// 渲染关于页面
pub fn render_about_page(ctx: &egui::Context, app: &mut crate::app::UniToolsApp) {
    // 渲染侧边栏
    super::render_sidebar(ctx, app);

    // 渲染主内容区域
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("关于 UniTools 工具箱");
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("UniTools 工具箱");
            ui.label(format!("版本: {}", env!("CARGO_PKG_VERSION")));
            ui.label(format!("作者: {}", env!("CARGO_PKG_AUTHORS")));
            ui.add_space(10.0);
            ui.label("一个功能强大、可扩展的跨平台工具集合");
            ui.add_space(20.0);
        });

        ui.collapsing("功能亮点", |ui| {
            ui.label("• 多种实用工具集成于一个应用");
            ui.label("• 插件系统支持功能扩展");
            ui.label("• 跨平台支持：Windows, macOS, Linux");
            ui.label("• 轻量级且高性能");
            ui.label("• 直观易用的界面");
        });

        ui.collapsing("技术栈", |ui| {
            ui.label("• Rust 编程语言");
            ui.label("• egui/eframe 用于跨平台GUI");
            ui.label("• 模块化架构设计");
            ui.label("• 插件系统支持动态加载");
        });

        ui.collapsing("开源许可", |ui| {
            ui.label("本软件基于 MIT 许可证开源发布");
            ui.label("Copyright © 2025 weidong");
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            if ui.button("返回主页").clicked() {
                app.navigate_to_page(super::Page::Home);
            }
        });
    });
}
