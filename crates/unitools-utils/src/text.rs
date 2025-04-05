use regex::Regex;
use unitools_core::error::ToolError;

/// 按行分割文本
pub fn split_lines(text: &str) -> Vec<String> {
    text.lines().map(|line| line.to_string()).collect()
}

/// 计数文本中的字符数
pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

/// 计数文本中的单词数（以空格分割）
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// 计数文本中的行数
pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}

/// 使用正则表达式查找所有匹配
pub fn regex_find_all(text: &str, pattern: &str) -> Result<Vec<String>, ToolError> {
    let regex = Regex::new(pattern)
        .map_err(|e| ToolError::ParseError(format!("无效的正则表达式: {}", e)))?;

    let matches: Vec<String> = regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect();

    Ok(matches)
}

/// 使用正则表达式替换文本
pub fn regex_replace_all(
    text: &str,
    pattern: &str,
    replacement: &str,
) -> Result<String, ToolError> {
    let regex = Regex::new(pattern)
        .map_err(|e| ToolError::ParseError(format!("无效的正则表达式: {}", e)))?;

    Ok(regex.replace_all(text, replacement).to_string())
}

/// 计算两个字符串之间的编辑距离 (Levenshtein distance)
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    // 初始化第一行和第一列
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    // 填充矩阵
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                matrix[i - 1][j] + 1, // 删除
                std::cmp::min(
                    matrix[i][j - 1] + 1,        // 插入
                    matrix[i - 1][j - 1] + cost, // 替换或匹配
                ),
            );
        }
    }

    matrix[a_len][b_len]
}

/// 检查字符串是否包含子串（区分大小写）
pub fn contains(text: &str, substring: &str) -> bool {
    text.contains(substring)
}

/// 检查字符串是否包含子串（不区分大小写）
pub fn contains_ignore_case(text: &str, substring: &str) -> bool {
    text.to_lowercase().contains(&substring.to_lowercase())
}

/// 转换文本为大写
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// 转换文本为小写
pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}
