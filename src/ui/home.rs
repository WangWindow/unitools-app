use eframe::egui;

/// 渲染主页
pub fn render_home_page(ctx: &egui::Context, app: &mut crate::app::UniToolsApp) {
    // 渲染侧边栏
    super::render_sidebar(ctx, app);

    // 渲染主内容区域
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("欢迎使用 UniTools 工具箱");
            ui.label("一站式解决各种工具需求");
        });

        ui.add_space(20.0);

        // 从app中获取所有工具
        let categories = &app.categories;

        ui.heading("工具分类");
        ui.separator();

        // 使用网格布局展示工具类别
        egui::Grid::new("categories_grid")
            .num_columns(3)
            .spacing([20.0, 20.0])
            .striped(true)
            .show(ui, |ui| {
                let mut categories: Vec<_> = categories.keys().collect();
                // 使用另一种方式进行排序，避免Discriminant<ToolCategory>没有实现Ord的问题
                categories.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

                for (i, &category) in categories.iter().enumerate() {
                    if i > 0 && i % 3 == 0 {
                        ui.end_row();
                    }

                    // 直接使用HashMap的get方法访问值，而不是对Vec进行索引
                    if let Some(tools) = app.categories.get(category) {
                        ui.vertical(|ui| {
                            ui.heading(format!("{}", category));
                            ui.label(format!("{}个工具", tools.len()));

                            if ui.button("查看").clicked() {
                                // 这里假设有一个展开类别的方法
                                // TODO: 添加展开分类的功能
                            }
                        });
                    }
                }
            });

        ui.add_space(20.0);

        // 最近使用的工具
        ui.heading("最近使用的工具");
        ui.separator();
        ui.label("暂无使用记录");

        // 底部状态栏
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("版本: {}", env!("CARGO_PKG_VERSION")));
                ui.separator();
                ui.label(format!("作者: {}", env!("CARGO_PKG_AUTHORS")));
            });
        });
    });
}
