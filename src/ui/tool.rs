use eframe::egui;
use std::collections::HashMap;
use unitools_core::tool::{Tool, ToolContext};

// 用于存储工具参数输入的状态
thread_local! {
    static TOOL_INPUTS: std::cell::RefCell<HashMap<String, ToolInputState>> = std::cell::RefCell::new(HashMap::new());
}

struct ToolInputState {
    input_text: String,
    parameters: HashMap<String, String>,
    output_text: Option<String>,
    has_error: bool,
    error_message: String,
}

/// 渲染工具页面
pub fn render_tool_page(ctx: &egui::Context, tool: &dyn Tool, app: &mut crate::app::UniToolsApp) {
    // 渲染侧边栏
    super::render_sidebar(ctx, app);

    // 渲染工具内容区域
    egui::CentralPanel::default().show(ctx, |ui| {
        // 工具页面标题
        ui.vertical_centered(|ui| {
            ui.heading(tool.name());
            ui.label(tool.description());
        });

        ui.separator();

        // 获取工具的输入状态
        let tool_name = tool.name().to_string();

        TOOL_INPUTS.with(|tool_inputs| {
            let mut tools = tool_inputs.borrow_mut();
            let input_state = tools
                .entry(tool_name.clone())
                .or_insert_with(|| ToolInputState {
                    input_text: String::new(),
                    parameters: HashMap::new(),
                    output_text: None,
                    has_error: false,
                    error_message: String::new(),
                });

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.heading("输入");

                        // 添加工具参数输入
                        let param_descriptions = tool.parameter_descriptions();
                        if !param_descriptions.is_empty() {
                            ui.heading("参数");
                            for (param_name, param_desc) in param_descriptions {
                                ui.horizontal(|ui| {
                                    ui.label(&param_name);
                                    ui.text_edit_singleline(
                                        input_state
                                            .parameters
                                            .entry(param_name)
                                            .or_insert(String::new()),
                                    );
                                    ui.label(param_desc);
                                });
                            }
                            ui.separator();
                        }

                        ui.heading("输入内容");
                        let text_height = ui.available_height() * 0.4;
                        egui::ScrollArea::vertical()
                            .max_height(text_height)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut input_state.input_text)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(10)
                                        .hint_text("输入文本内容..."),
                                );
                            });

                        ui.horizontal(|ui| {
                            if ui.button("执行").clicked() {
                                // 创建工具上下文
                                let mut context = ToolContext::default();

                                // 添加文本输入
                                if !input_state.input_text.is_empty() {
                                    context.input_data =
                                        Some(input_state.input_text.as_bytes().to_vec());
                                }

                                // 添加参数
                                for (name, value) in &input_state.parameters {
                                    if !value.is_empty() {
                                        context.parameters.insert(name.clone(), value.clone());
                                    }
                                }

                                // 执行工具
                                match tool.execute(&context) {
                                    Ok(result) => {
                                        input_state.has_error = false;
                                        if let Some(data) = result {
                                            // 尝试将结果转换为字符串
                                            match String::from_utf8(data) {
                                                Ok(text) => {
                                                    input_state.output_text = Some(text);
                                                }
                                                Err(_) => {
                                                    input_state.output_text =
                                                        Some("[二进制数据]".to_string());
                                                }
                                            }
                                        } else {
                                            input_state.output_text = Some("[无输出]".to_string());
                                        }
                                    }
                                    Err(error) => {
                                        input_state.has_error = true;
                                        input_state.error_message = format!("错误: {}", error);
                                    }
                                }
                            }

                            if ui.button("清除").clicked() {
                                input_state.input_text.clear();
                                input_state.parameters.clear();
                                input_state.output_text = None;
                                input_state.has_error = false;
                            }
                        });
                    });
                });

                // 输出区域
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.heading("输出");
                        let text_height = ui.available_height() * 0.8;
                        egui::ScrollArea::vertical()
                            .max_height(text_height)
                            .show(ui, |ui| {
                                if input_state.has_error {
                                    ui.colored_label(
                                        egui::Color32::RED,
                                        &input_state.error_message,
                                    );
                                } else if let Some(output) = &input_state.output_text {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut output.clone())
                                            .desired_width(f32::INFINITY)
                                            .desired_rows(15)
                                            .interactive(false),
                                    );
                                } else {
                                    ui.weak("运行工具后将在此显示结果...");
                                }
                            });
                    });
                });
            });
        });
    });
}
