use eframe::egui;
use std::path::Path;
use unitools_core::config::Theme;

/// 设置应用主题
pub fn setup_theme(ctx: &egui::Context, theme: Theme) {
    // 加载字体
    load_fonts(ctx);

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
            let is_dark =
                dark_light::detect().unwrap_or(dark_light::Mode::Light) == dark_light::Mode::Dark;

            if is_dark {
                ctx.set_visuals(egui::Visuals::dark());
            } else {
                ctx.set_visuals(egui::Visuals::light());
            }
        }
    }
}

/// 加载自定义字体
fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 添加思源黑体作为中文字体
    let font_path = Path::new("assets/fonts/SourceHanSansSC-Normal.otf");
    if let Ok(font_data) = std::fs::read(font_path) {
        // 注册字体
        fonts.font_data.insert(
            "source_han_sans".to_string(),
            egui::FontData::from_owned(font_data).into(), // 添加.into()将FontData转换为Arc<FontData>
        );

        // 将字体添加到所有字体族中（默认和等宽）
        for family in &[egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts
                .families
                .entry(family.clone())
                .or_default()
                .insert(0, "source_han_sans".to_string()); // 插入到开头，优先使用
        }

        // 应用字体
        ctx.set_fonts(fonts);
    } else {
        eprintln!("无法加载字体文件: {:?}", font_path);
    }
}
