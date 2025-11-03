/// Platform-agnostic file picker service
/// Web: uses <input type="file">
/// Desktop/Mobile: uses platform-specific APIs

use crate::state::FileInfo;

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::FileInfo;

    /// Open file picker on web platform
    pub fn open_file_picker(_on_select: impl Fn(Option<FileInfo>) + 'static) {
        // Web implementation would use web_sys or wasm-bindgen
        // For now, return None (to be implemented with actual web APIs)
        log::warn!("Web file picker not yet implemented");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::FileInfo;

    /// Open file picker on native platforms
    pub fn open_file_picker(_on_select: impl Fn(Option<FileInfo>) + 'static) {
        // Native implementation would use Tauri or mobile APIs
        // For now, return None (to be implemented with platform-specific APIs)
        log::warn!("Native file picker not yet implemented");
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

