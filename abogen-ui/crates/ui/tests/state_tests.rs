#[cfg(test)]
mod state_tests {
    use crate::state::{AppState, FileInfo, SubtitleFormat, SubtitleMode, VoiceFormat};
    
    #[test]
    fn test_file_info_equality() {
        let file1 = FileInfo {
            name: "test.txt".to_string(),
            path: "/test.txt".to_string(),
            size: 100,
        };
        
        let file2 = FileInfo {
            name: "test.txt".to_string(),
            path: "/test.txt".to_string(),
            size: 100,
        };
        
        assert_eq!(file1, file2);
    }
    
    #[test]
    fn test_subtitle_mode_as_str() {
        assert_eq!(SubtitleMode::Sentence.as_str(), "Sentence");
        assert_eq!(SubtitleMode::Paragraph.as_str(), "Paragraph");
        assert_eq!(SubtitleMode::None.as_str(), "None");
    }
    
    #[test]
    fn test_voice_format_as_str() {
        assert_eq!(VoiceFormat::Wav.as_str(), "wav");
        assert_eq!(VoiceFormat::Mp3.as_str(), "mp3");
        assert_eq!(VoiceFormat::Flac.as_str(), "flac");
    }
    
    #[test]
    fn test_subtitle_format_as_str() {
        assert_eq!(SubtitleFormat::Ass.as_str(), "ass");
        assert_eq!(SubtitleFormat::Srt.as_str(), "srt");
        assert_eq!(SubtitleFormat::Vtt.as_str(), "vtt");
    }
}

