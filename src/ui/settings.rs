use eframe::egui;
use std::sync::Mutex;
use unitools_core::config::{AppConfig, Theme};

/// 渲染设置页面
pub fn render_settings_page(ctx: &egui::Context, app: &mut crate::app::UniToolsApp) {
    // 渲染侧边栏
    super::render_sidebar(ctx, app);

    // 渲染主内容区域
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("设置");
        });

        ui.separator();

        // 获取配置的可变引用
        let mut config_guard = app.config.lock().unwrap();

        // 主题设置
        ui.heading("界面设置");
        ui.horizontal(|ui| {
            ui.label("主题: ");
            ui.radio_value(&mut config_guard.theme, Theme::Light, "浅色");
            ui.radio_value(&mut config_guard.theme, Theme::Dark, "深色");
            ui.radio_value(&mut config_guard.theme, Theme::System, "跟随系统");
        });

        // 应用主题变更
        super::setup_theme(ctx, config_guard.theme);

        ui.separator();

        // 用户设置
        ui.heading("用户设置");
        ui.horizontal(|ui| {
            ui.label("用户名: ");
            ui.text_edit_singleline(&mut config_guard.user.username);
        });

        ui.horizontal(|ui| {
            ui.label("默认工作目录: ");
            if let Some(dir) = &config_guard.user.working_directory {
                ui.label(dir.to_string_lossy().to_string());
            } else {
                ui.label("[未设置]");
            }

            if ui.button("浏览...").clicked() {
                // 在实际应用中，这里会打开一个文件选择对话框
                // 但在这个简化版本中，我们只是添加一个占位符
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    config_guard.user.working_directory = Some(path);
                }
            }
        });

        ui.separator();

        // 插件设置
        ui.heading("插件设置");
        ui.horizontal(|ui| {
            ui.label("插件目录: ");
            if let Some(dir) = &config_guard.plugin_directory {
                ui.label(dir.to_string_lossy().to_string());
            } else {
                ui.label("[使用默认目录]");
            }

            if ui.button("浏览...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    config_guard.plugin_directory = Some(path);
                }
            }
        });

        // 已启用的插件列表
        ui.label("启用的插件:");

        if config_guard.enabled_plugins.is_empty() {
            ui.label("[无]");
        } else {
            for plugin_name in &config_guard.enabled_plugins {
                ui.label(plugin_name);
            }
        }

        ui.separator();

        // 底部按钮
        ui.horizontal(|ui| {
            if ui.button("保存设置").clicked() {
                // 保存配置到文件
                if let Err(e) = crate::config::save_config(&config_guard, "../config.json") {
                    eprintln!("保存配置失败: {}", e);
                }
            }

            if ui.button("恢复默认").clicked() {
                *config_guard = AppConfig::default();
            }
        });
    });
}
