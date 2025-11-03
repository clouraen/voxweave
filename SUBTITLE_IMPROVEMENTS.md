# Subtitle Display Improvements

## Overview
Implemented two major subtitle enhancements:
1. **Word-by-Word Display Mode**: Shows only the current word being spoken
2. **IPA Transcription Support**: Optional International Phonetic Alphabet transcription for language learning

## Changes Made

### 1. State Management (`abogen-ui/crates/ui/state.rs`)

#### New Enums
- `SubtitleDisplayStyle`: Controls how subtitles appear in videos
  - `WordByWord`: Shows only the current word (clean, focused)
  - `Cumulative`: Shows all previous words + highlighted current word (original behavior)

#### Updated Structs
- `QueuedItem`: Added fields:
  - `subtitle_display_style: SubtitleDisplayStyle`
  - `show_ipa_transcription: bool`
  
- `AppState`: Added state signals:
  - `subtitle_display_style: Signal<SubtitleDisplayStyle>`
  - `show_ipa_transcription: Signal<bool>`

### 2. MLT Video Service (`abogen-ui/crates/ui/services/mlt_video.rs`)

#### Updated `WordSubtitle` Struct
```rust
pub struct WordSubtitle {
    pub word: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub ipa: Option<String>, // New: IPA transcription if available
}
```

#### Enhanced `generate_mlt_xml()` Function
- Added parameters: `display_style: SubtitleDisplayStyle`, `show_ipa: bool`
- Implements two display modes:
  - **Word-by-Word**: Only current word in large cyan text
  - **Cumulative**: All previous words in white + current word highlighted in cyan
- IPA transcription displayed below main text in smaller gray font

#### XML Markup Generation
- Uses Pango markup for rich text formatting
- Proper XML and Pango escaping
- Conditional IPA display with `\n` separator

### 3. Z.AI Video Integration (`abogen-ui/crates/ui/services/zai_video.rs`)

#### Updated `generate_zai_video_with_composition()`
- Added parameters to pass through subtitle display options
- Forwards `subtitle_display_style` and `show_ipa` to MLT merger

### 4. UI Components (`abogen-ui/crates/ui/lib.rs`)

#### New UI Controls
Added two new dropdown selectors after subtitle format:

1. **Subtitle Display Style**
   ```
   📺 Word-by-Word Display (default)
   📜 Cumulative Display
   ```

2. **IPA Transcription Toggle**
   ```
   🔤 IPA Transcription: Enabled
   🔤 IPA Transcription: Disabled (default)
   ```

#### Queue Item Creation
Updated to include new fields when adding items to queue

### 5. TTS Service (`abogen-ui/crates/ui/services/tts_service.rs`)

- Updated Z.AI video generation call to pass new parameters
- Forwards from QueuedItem to video generation function

### 6. Test Stubs (`abogen-ui/crates/ui/services/tts_stub.rs`)

- Updated test fixtures to include new required fields

## Usage

### For Users

1. **Word-by-Word Mode** (Recommended for learning):
   - Select "📺 Word-by-Word Display" from dropdown
   - Only the current spoken word appears on screen
   - Cleaner, more focused viewing experience

2. **Cumulative Mode** (Classic):
   - Select "📜 Cumulative Display"
   - All words spoken so far remain visible
   - Current word highlighted in cyan

3. **IPA Transcription** (for language learners):
   - Enable "🔤 IPA Transcription: Enabled"
   - Phonetic transcription appears below each word
   - Helps with pronunciation learning
   - Note: IPA generation requires external service (future implementation)

### Example Output

**Word-by-Word + IPA:**
```
    HELLO
   [həˈloʊ]
```

**Cumulative + IPA:**
```
  Hello WORLD today
        [wɜrld]
```

## Technical Details

### MLT XML Generation
- Uses Pango markup for text styling
- Subtitles rendered as separate producers with timing
- Semi-transparent black background (#00000080)
- Cyan highlight (#00FFFF) for current word
- Gray IPA text (#AAAAAA) in smaller font

### IPA Integration (Future Work)
Currently, IPA field is `Option<String>` set to `None`. To enable:
1. Integrate IPA generation library (e.g., `espeak-ng --ipa` or ML model)
2. Generate IPA during `generate_word_timing()`
3. Map words to IPA transcriptions based on language

## Testing

Run the desktop app:
```bash
cd /Users/cleitonmouraloura/Documents/voxweave/abogen-ui
cargo run --bin abogen-desktop --features video-generation
```

Test with different subtitle modes:
1. Load a text file
2. Enable video generation
3. Try both Word-by-Word and Cumulative modes
4. Observe subtitle rendering in generated videos

## Future Enhancements

1. **Auto-detect language** for IPA generation
2. **Color-coded IPA** for stress marks and syllables
3. **Multiple subtitle tracks** (original + IPA separate)
4. **Adjustable font sizes** per mode
5. **Custom highlight colors**
6. **Subtitle positioning** (top/bottom/custom)

## Files Modified

- `abogen-ui/crates/ui/state.rs`
- `abogen-ui/crates/ui/services/mlt_video.rs`
- `abogen-ui/crates/ui/services/zai_video.rs`
- `abogen-ui/crates/ui/services/tts_service.rs`
- `abogen-ui/crates/ui/services/tts_stub.rs`
- `abogen-ui/crates/ui/lib.rs`
