use crate::queue::SubtitleGranularity;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct SubtitleEntry {
    pub index: usize,
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

pub fn generate_subtitles(
    text: &str,
    granularity: SubtitleGranularity,
    average_words_per_minute: f32,
) -> Vec<SubtitleEntry> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    match granularity {
        SubtitleGranularity::Disabled => Vec::new(),
        SubtitleGranularity::Sentence => build_entries(
            split_sentences(trimmed),
            normalize_wpm(average_words_per_minute),
        ),
        SubtitleGranularity::Words(count) => {
            let chunk_size = count.max(1) as usize;
            build_entries(
                split_words(trimmed, chunk_size),
                normalize_wpm(average_words_per_minute),
            )
        }
    }
}

pub fn format_srt(entries: &[SubtitleEntry]) -> String {
    let mut output = String::new();
    for entry in entries {
        output.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            entry.index,
            format_timestamp(entry.start_ms),
            format_timestamp(entry.end_ms),
            entry.text
        ));
    }
    output
}

fn normalize_wpm(wpm: f32) -> f32 {
    if wpm <= 0.0 { 150.0 } else { wpm }
}

fn build_entries(chunks: Vec<String>, average_wpm: f32) -> Vec<SubtitleEntry> {
    if chunks.is_empty() {
        return Vec::new();
    }
    let ms_per_word = (60_000.0 / average_wpm).max(1.0);
    let mut entries = Vec::with_capacity(chunks.len());
    let mut cursor: u64 = 0;

    for (idx, chunk) in chunks.iter().enumerate() {
        let word_count = chunk.split_whitespace().count().max(1);
        let duration = (word_count as f32 * ms_per_word).round().max(500.0) as u64;
        let entry = SubtitleEntry {
            index: idx + 1,
            start_ms: cursor,
            end_ms: cursor + duration,
            text: chunk.clone(),
        };
        entries.push(entry);
        cursor += duration;
    }

    entries
}

fn split_sentences(text: &str) -> Vec<String> {
    SENTENCE_REGEX
        .find_iter(text)
        .map(|m| m.as_str().trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn split_words(text: &str, chunk_size: usize) -> Vec<String> {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.join(" "))
        .collect()
}

fn format_timestamp(ms: u64) -> String {
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let millis = ms % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02},{millis:03}")
}

static SENTENCE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)[^.!?]+[.!?]?").expect("valid sentence regex"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::SubtitleGranularity;

    #[test]
    fn disabled_granularity_yields_no_entries() {
        let entries = generate_subtitles("Hello world", SubtitleGranularity::Disabled, 150.0);
        assert!(entries.is_empty());
    }

    #[test]
    fn sentences_are_split_correctly() {
        let text = "Hello world. This is a test!";
        let entries = generate_subtitles(text, SubtitleGranularity::Sentence, 150.0);
        let texts: Vec<_> = entries.iter().map(|entry| entry.text.as_str()).collect();
        assert_eq!(texts, vec!["Hello world.", "This is a test!"]);
    }

    #[test]
    fn words_grouped_by_chunk_size() {
        let text = "one two three four five";
        let entries = generate_subtitles(text, SubtitleGranularity::Words(2), 180.0);
        let texts: Vec<_> = entries.iter().map(|entry| entry.text.as_str()).collect();
        assert_eq!(texts, vec!["one two", "three four", "five"]);
    }

    #[test]
    fn srt_formatting_is_correct() {
        let entries = vec![
            SubtitleEntry {
                index: 1,
                start_ms: 0,
                end_ms: 2000,
                text: "Hello world.".into(),
            },
            SubtitleEntry {
                index: 2,
                start_ms: 2000,
                end_ms: 4000,
                text: "This is a test.".into(),
            },
        ];
        let srt = format_srt(&entries);
        let expected = "1\n00:00:00,000 --> 00:00:02,000\nHello world.\n\n2\n00:00:02,000 --> 00:00:04,000\nThis is a test.\n\n";
        assert_eq!(srt, expected);
    }
}
