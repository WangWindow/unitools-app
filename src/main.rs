mod app;
mod config;
mod tools;
mod ui;

use clap::Parser;
use eframe::egui;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// 启动特定工具
    #[arg(short, long)]
    tool: Option<String>,

    /// 调试模式
    #[arg(short, long)]
    debug: bool,
}

/// 加载应用图标
fn load_icon() -> Option<egui::IconData> {
    let icon_path = Path::new("assets/icon.ico");
    match std::fs::read(icon_path) {
        Ok(icon_data) => {
            let (icon_rgba, icon_width, icon_height) =
                match ico::IconDir::read(std::io::Cursor::new(icon_data)) {
                    Ok(icon_dir) => {
                        if let Some(entry) = icon_dir.entries().get(0) {
                            let icon = entry.decode().unwrap();
                            let width = icon.width() as u32;
                            let height = icon.height() as u32;
                            let rgba = icon.rgba_data().to_vec();
                            (rgba, width, height)
                        } else {
                            eprintln!("图标文件不包含任何图标");
                            return None;
                        }
                    }
                    Err(e) => {
                        eprintln!("无法解析图标文件: {}", e);
                        return None;
                    }
                };

            Some(egui::IconData {
                rgba: icon_rgba,
                width: icon_width,
                height: icon_height,
            })
        }
        Err(e) => {
            eprintln!("无法加载图标文件: {}", e);
            None
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // 解析命令行参数
    let args = Args::parse();

    // 设置日志
    if args.debug {
        env_logger::init();
    }

    // 加载配置
    let config_path = args.config.unwrap_or_else(|| {
        let mut path = dirs::config_dir().unwrap_or_default();
        path.push("unitools");
        path.push("config.json");
        path.to_string_lossy().to_string()
    });

    let app_config = config::load_or_create_config(&config_path);

    // 创建视口构建器
    let mut viewport_builder = egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]);

    // 如果图标加载成功，则添加图标
    if let Some(icon) = load_icon() {
        viewport_builder = viewport_builder.with_icon(std::sync::Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    // 创建并运行应用
    eframe::run_native(
        "UniTools 工具箱",
        options,
        Box::new(|cc| Ok(Box::new(app::UniToolsApp::new(cc, app_config)))),
    )
}
