use eframe::egui;
use unitools_core::config::Theme;

/// 设置应用主题
pub fn setup_theme(ctx: &egui::Context, theme: Theme) {
    match theme {
        Theme::Light => {
            ctx.set_visuals(egui::Visuals::light());
        }
        Theme::Dark => {
            ctx.set_visuals(egui::Visuals::dark());
        }
        Theme::System => {
            // 尝试从系统获取暗色模式偏好，如果获取失败则默认为浅色模式
            #[cfg(target_arch = "wasm32")]
            let is_dark = match web_sys::window()
                .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            {
                Some(query) => query.matches(),
                None => false,
            };

            #[cfg(not(target_arch = "wasm32"))]
            let is_dark = dark_light::detect() == dark_light::Mode::Dark;

            if is_dark {
                ctx.set_visuals(egui::Visuals::dark());
            } else {
                ctx.set_visuals(egui::Visuals::light());
            }
        }
    }
}
