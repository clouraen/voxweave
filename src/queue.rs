use crate::tts::VoiceProfile;

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub source_path: String,
    pub output_dir: String,
    pub voice: VoiceProfile,
    pub speed: f32,
    pub subtitle_granularity: SubtitleGranularity,
    pub replace_single_newlines: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleGranularity {
    Disabled,
    Sentence,
    Words(u8),
}

#[derive(Default)]
pub struct ConversionQueue {
    items: Vec<QueueItem>,
}

impl ConversionQueue {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: QueueItem) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<QueueItem> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueueItem> {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tts::VoiceProfile;

    fn sample_item(id: &str) -> QueueItem {
        QueueItem {
            source_path: format!("{id}.txt"),
            output_dir: "out".into(),
            voice: VoiceProfile::builtin("voice", "Voice"),
            speed: 1.0,
            subtitle_granularity: SubtitleGranularity::Sentence,
            replace_single_newlines: false,
        }
    }

    #[test]
    fn queue_preserves_fifo_order() {
        let mut queue = ConversionQueue::new();
        queue.enqueue(sample_item("a"));
        queue.enqueue(sample_item("b"));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.dequeue().unwrap().source_path, "a.txt");
        assert_eq!(queue.dequeue().unwrap().source_path, "b.txt");
        assert!(queue.is_empty());
    }
}
