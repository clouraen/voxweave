use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
    Other,
}

impl OperatingSystem {
    #[cfg(target_os = "windows")]
    const DEFAULT: OperatingSystem = OperatingSystem::Windows;
    #[cfg(target_os = "macos")]
    const DEFAULT: OperatingSystem = OperatingSystem::MacOs;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const DEFAULT: OperatingSystem = OperatingSystem::Linux;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "android"
    )))]
    const DEFAULT: OperatingSystem = OperatingSystem::Other;

    pub fn current() -> OperatingSystem {
        Self::DEFAULT
    }
}

pub fn sanitize_name_for_os(name: &str, is_folder: bool) -> String {
    sanitize_name_for(name, OperatingSystem::current(), is_folder)
}

pub fn sanitize_name_for(name: &str, os: OperatingSystem, is_folder: bool) -> String {
    let trimmed = name.trim();
    let mut sanitized = if trimmed.is_empty() {
        Cow::Borrowed("audiobook")
    } else {
        Cow::Owned(trimmed.to_string())
    };

    match os {
        OperatingSystem::Windows => {
            sanitized.to_mut().retain(|c| c != '\0');
            sanitized = Cow::Owned(
                sanitized
                    .chars()
                    .map(|c| match c {
                        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                        c if (c as u32) < 32 => '_',
                        _ => c,
                    })
                    .collect::<String>(),
            );
            while sanitized.ends_with([' ', '.']) {
                sanitized.to_mut().pop();
            }

            let upper = sanitized.to_uppercase();
            let base = upper.split('.').next().unwrap_or_default().to_string();
            let reserved = [
                "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
                "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8",
                "LPT9",
            ];
            if reserved.iter().any(|&item| item == upper || item == base) {
                sanitized = Cow::Owned(format!("_{sanitized}"));
            }
        }
        OperatingSystem::MacOs => {
            sanitized = Cow::Owned(
                sanitized
                    .chars()
                    .map(|c| match c {
                        ':' => '_',
                        c if (c as u32) < 32 => '_',
                        _ => c,
                    })
                    .collect(),
            );

            if is_folder && sanitized.starts_with('.') {
                let mut as_string = sanitized.into_owned();
                as_string.remove(0);
                sanitized = Cow::Owned(format!("_{as_string}"));
            }
        }
        OperatingSystem::Linux | OperatingSystem::Other => {
            sanitized = Cow::Owned(
                sanitized
                    .chars()
                    .map(|c| match c {
                        '/' => '_',
                        c if (c as u32) < 32 => '_',
                        '\0' => '_',
                        _ => c,
                    })
                    .collect(),
            );
            if is_folder && sanitized.starts_with('.') {
                let mut as_string = sanitized.into_owned();
                as_string.remove(0);
                sanitized = Cow::Owned(format!("_{as_string}"));
            }
        }
    }

    if sanitized.trim().is_empty() {
        return "audiobook".to_string();
    }

    if sanitized.len() > 255 {
        let mut slice = sanitized[..255].to_string();
        while slice.ends_with([' ', '.']) {
            slice.pop();
        }
        if slice.is_empty() {
            "audiobook".to_string()
        } else {
            slice
        }
    } else {
        sanitized.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_reserved_names_are_prefixed() {
        let result = sanitize_name_for("AUX.txt", OperatingSystem::Windows, false);
        assert_eq!(result, "_AUX.txt");
    }

    #[test]
    fn windows_invalid_chars_replaced() {
        let result = sanitize_name_for("bad:name?.mp3", OperatingSystem::Windows, false);
        assert_eq!(result, "bad_name_.mp3");
    }

    #[test]
    fn mac_folder_avoids_leading_dot() {
        let result = sanitize_name_for(".hidden", OperatingSystem::MacOs, true);
        assert_eq!(result, "_hidden");
    }

    #[test]
    fn linux_slashes_replaced() {
        let result = sanitize_name_for("chapter/one", OperatingSystem::Linux, false);
        assert_eq!(result, "chapter_one");
    }

    #[test]
    fn empty_input_defaults_to_audiobook() {
        let result = sanitize_name_for("", OperatingSystem::Linux, true);
        assert_eq!(result, "audiobook");
    }
}
