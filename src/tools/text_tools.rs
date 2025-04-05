use std::fmt::Debug;
use std::sync::Arc;
use unitools_core::error::ToolError;
use unitools_core::tool::{Tool, ToolCategory, ToolContext, ToolResult};
use unitools_utils::text;

/// 获取所有文本工具
pub fn get_text_tools() -> Vec<Arc<dyn Tool>> {
    vec![
        Arc::new(TextAnalyzer {}),
        Arc::new(TextCaseConverter {}),
        Arc::new(TextRegexTool {}),
    ]
}

/// 文本分析工具
#[derive(Debug, Clone)]
pub struct TextAnalyzer {}

impl Tool for TextAnalyzer {
    fn name(&self) -> &str {
        "文本分析器"
    }

    fn description(&self) -> &str {
        "分析文本，统计字符数、单词数和行数"
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Text
    }

    fn execute(&self, ctx: &ToolContext) -> ToolResult {
        // 获取输入数据
        let input = match &ctx.input_data {
            Some(data) => match String::from_utf8(data.clone()) {
                Ok(text) => text,
                Err(_) => {
                    return Err(ToolError::FormatError(
                        "输入数据不是有效的UTF-8文本".to_string(),
                    ))
                }
            },
            None => return Err(ToolError::ParameterError("没有提供输入文本".to_string())),
        };

        // 分析文本
        let char_count = text::count_chars(&input);
        let word_count = text::count_words(&input);
        let line_count = text::count_lines(&input);

        // 构建结果
        let result = format!(
            "文本分析结果:\n\n字符数: {}\n单词数: {}\n行数: {}",
            char_count, word_count, line_count
        );

        Ok(Some(result.into_bytes()))
    }
}

/// 文本大小写转换工具
#[derive(Debug, Clone)]
pub struct TextCaseConverter {}

impl Tool for TextCaseConverter {
    fn name(&self) -> &str {
        "大小写转换"
    }

    fn description(&self) -> &str {
        "将文本转换为大写或小写"
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Text
    }

    fn parameter_descriptions(&self) -> Vec<(String, String)> {
        vec![(
            "mode".to_string(),
            "转换模式: upper=大写, lower=小写".to_string(),
        )]
    }

    fn execute(&self, ctx: &ToolContext) -> ToolResult {
        // 获取输入数据
        let input = match &ctx.input_data {
            Some(data) => match String::from_utf8(data.clone()) {
                Ok(text) => text,
                Err(_) => {
                    return Err(ToolError::FormatError(
                        "输入数据不是有效的UTF-8文本".to_string(),
                    ))
                }
            },
            None => return Err(ToolError::ParameterError("没有提供输入文本".to_string())),
        };

        // 获取转换模式参数
        let mode = ctx
            .parameters
            .get("mode")
            .cloned()
            .unwrap_or_else(|| "upper".to_string());

        // 根据模式转换文本
        let result = match mode.to_lowercase().as_str() {
            "upper" => text::to_uppercase(&input),
            "lower" => text::to_lowercase(&input),
            _ => {
                return Err(ToolError::ParameterError(format!(
                    "无效的模式参数: {}",
                    mode
                )))
            }
        };

        Ok(Some(result.into_bytes()))
    }
}

/// 正则表达式工具
#[derive(Debug, Clone)]
pub struct TextRegexTool {}

impl Tool for TextRegexTool {
    fn name(&self) -> &str {
        "正则表达式工具"
    }

    fn description(&self) -> &str {
        "使用正则表达式查找或替换文本"
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Text
    }

    fn parameter_descriptions(&self) -> Vec<(String, String)> {
        vec![
            ("pattern".to_string(), "正则表达式模式".to_string()),
            (
                "replacement".to_string(),
                "替换文本(可选，如果提供则执行替换操作)".to_string(),
            ),
            (
                "mode".to_string(),
                "操作模式: find=查找, replace=替换(默认为find)".to_string(),
            ),
        ]
    }

    fn execute(&self, ctx: &ToolContext) -> ToolResult {
        // 获取输入数据
        let input = match &ctx.input_data {
            Some(data) => match String::from_utf8(data.clone()) {
                Ok(text) => text,
                Err(_) => {
                    return Err(ToolError::FormatError(
                        "输入数据不是有效的UTF-8文本".to_string(),
                    ))
                }
            },
            None => return Err(ToolError::ParameterError("没有提供输入文本".to_string())),
        };

        // 获取正则表达式参数
        let pattern = match ctx.parameters.get("pattern") {
            Some(p) => p,
            None => {
                return Err(ToolError::ParameterError(
                    "没有提供正则表达式模式".to_string(),
                ))
            }
        };

        // 获取操作模式
        let mode = ctx
            .parameters
            .get("mode")
            .cloned()
            .unwrap_or_else(|| "find".to_string());

        // 根据模式执行操作
        let result = match mode.to_lowercase().as_str() {
            "find" => {
                let matches = text::regex_find_all(&input, pattern)?;
                if matches.is_empty() {
                    "未找到匹配项".to_string()
                } else {
                    format!("找到 {} 个匹配项:\n\n{}", matches.len(), matches.join("\n"))
                }
            }
            "replace" => {
                let replacement = ctx
                    .parameters
                    .get("replacement")
                    .cloned()
                    .unwrap_or_else(|| "".to_string());
                text::regex_replace_all(&input, pattern, &replacement)?
            }
            _ => {
                return Err(ToolError::ParameterError(format!(
                    "无效的模式参数: {}",
                    mode
                )))
            }
        };

        Ok(Some(result.into_bytes()))
    }
}
