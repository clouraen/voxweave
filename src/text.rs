use once_cell::sync::Lazy;
use regex::Regex;

static NON_NEWLINE_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^\S\n]+").expect("valid regex"));
static MULTIPLE_NEWLINES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").expect("valid regex"));

pub fn clean_text(input: &str, replace_single_newlines: bool) -> String {
    let collapsed_lines: Vec<String> = input
        .split('\n')
        .map(|line| collapse_whitespace(line))
        .collect();

    let mut text = collapsed_lines.join("\n");
    text = MULTIPLE_NEWLINES.replace_all(&text, "\n\n").to_string();
    text = text.trim().to_string();

    if replace_single_newlines {
        text = replace_single_newlines_with_spaces(&text);
    }

    text
}

pub fn collapse_whitespace(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    NON_NEWLINE_WHITESPACE.replace_all(trimmed, " ").to_string()
}

fn replace_single_newlines_with_spaces(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut segments = text.split("\n\n").peekable();
    while let Some(segment) = segments.next() {
        if !result.is_empty() {
            result.push_str("\n\n");
        }
        let replaced = segment.replace('\n', " ");
        result.push_str(&replaced);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_collapses_whitespace_per_line() {
        let original = "  Hello   world  \nThis   is   a  test ";
        let result = clean_text(original, false);
        assert_eq!(result, "Hello world\nThis is a test");
    }

    #[test]
    fn reduces_multiple_blank_lines() {
        let original = "Line1\n\n\n\nLine2";
        let result = clean_text(original, false);
        assert_eq!(result, "Line1\n\nLine2");
    }

    #[test]
    fn replaces_single_newlines_when_enabled() {
        let original = "Line1\nLine2\n\nLine3";
        let result = clean_text(original, true);
        assert_eq!(result, "Line1 Line2\n\nLine3");
    }
}
