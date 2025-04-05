mod app;
mod config;
mod tools;
mod ui;

use clap::Parser;
use eframe::egui;

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

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        ..Default::default()
    };

    // 创建并运行应用
    eframe::run_native(
        "UniTools 工具箱",
        options,
        Box::new(|cc| Box::new(app::UniToolsApp::new(cc, app_config))),
    )
}
