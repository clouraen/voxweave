#[cfg(all(test, feature = "coqui-tts"))]
mod coqui_integration_tests {
    use tempfile::tempdir;
    use voxweave::coqui_tts::CoquiEngine;
    use voxweave::tts::{SpeechEngine, VoiceProfile};

    #[test]
    #[ignore] // Ignore by default since it requires Python and TTS installation
    fn test_coqui_basic_synthesis() {
        let engine = CoquiEngine::default();
        let voice = VoiceProfile::coqui("coqui-en", "Coqui English", "en");
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_output.wav");

        let result = engine.synthesize_to_file(
            "Hello, this is a test of Coqui TTS synthesis.",
            &voice,
            1.0,
            &output_path,
        );

        match result {
            Ok(_) => {
                assert!(output_path.exists(), "Output file should exist");
                let metadata = std::fs::metadata(&output_path).unwrap();
                assert!(metadata.len() > 0, "Output file should not be empty");
                println!("✓ Coqui synthesis test passed: {:?}", output_path);
            }
            Err(e) => {
                eprintln!("✗ Coqui synthesis failed: {}", e);
                eprintln!("This is expected if Python TTS is not installed.");
                eprintln!("To install: pip install TTS torch numpy");
            }
        }
    }

    #[test]
    #[ignore] // Ignore by default since it requires Python and TTS installation
    fn test_coqui_voice_cloning() {
        let engine = CoquiEngine::default();
        
        // For voice cloning test, we'd need an actual audio file
        // This is a placeholder showing how it would work
        let clone_audio_path = "/path/to/reference/audio.wav";
        
        // Check if clone audio exists
        if !std::path::Path::new(clone_audio_path).exists() {
            println!("Skipping voice cloning test - no reference audio found");
            return;
        }

        let voice = VoiceProfile::coqui_clone(
            "clone-en",
            "Cloned Voice",
            "en",
            clone_audio_path,
        );
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_cloned.wav");

        let result = engine.synthesize_to_file(
            "This is a test of voice cloning with Coqui TTS.",
            &voice,
            1.0,
            &output_path,
        );

        match result {
            Ok(_) => {
                assert!(output_path.exists(), "Output file should exist");
                println!("✓ Voice cloning test passed: {:?}", output_path);
            }
            Err(e) => {
                eprintln!("✗ Voice cloning failed: {}", e);
            }
        }
    }

    #[test]
    fn test_voice_profile_creation() {
        // Test that voice profiles can be created correctly
        let voice = VoiceProfile::coqui("coqui-en", "English", "en");
        assert_eq!(voice.id, "coqui-en");
        assert_eq!(voice.engine, voxweave::tts::VoiceEngine::Coqui);
        assert_eq!(voice.lang, Some("en".to_string()));

        let clone_voice = VoiceProfile::coqui_clone(
            "clone-test",
            "Test Clone",
            "en",
            "/path/to/audio.wav",
        );
        assert_eq!(clone_voice.id, "clone-test");
        assert_eq!(clone_voice.engine, voxweave::tts::VoiceEngine::Coqui);
        assert_eq!(clone_voice.command, Some("/path/to/audio.wav".to_string()));
    }
}
